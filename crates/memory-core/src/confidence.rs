//! Structural confidence — the effective-weight model the 2026-06-15 race picked (V1).
//!
//! `StructuralOnly` was the best RANKER on both the corpus and the real store (verdict-pair
//! accuracy 0.93 / 0.92), and **recency is non-negotiable** — dropping it collapsed real-store
//! pair-acc to 0.17 (a live store has no corroboration and uniform source_weight, so recency is the
//! only thing separating a version-chain winner from its predecessor). We do NOT rank on the
//! asserted float (it was the worst ranker, real pair-acc 0.000 — the calibration literature, now
//! measured here). The scalar `weight` drives recall ordering; `possibility_necessity` is carried
//! as a *contested-belief affordance* (possibility separates contested vs uncontested current
//! beliefs ~5× better than the scalar), never as a frontier driver. See
//! `docs/design/open-questions-eval.md` V1.

use crate::{Belief, EdgeKind, Graph, Id};
use std::collections::{HashMap, HashSet};

/// Effective-weight context for one resolved graph: precomputes the corroboration and contest
/// signals (incoming live `supports` / `attacks`) and the recency normalization window, so each
/// per-belief `weight` call is O(1). Build once per recall.
pub struct StructuralConfidence {
    /// incoming live `supports` count per belief id (the system's real entrenchment signal)
    corrob: HashMap<Id, u32>,
    /// incoming live `attacks` count per belief id (the contest signal)
    attacked: HashMap<Id, u32>,
    /// oldest / newest parsed txn_time, as an epoch-ordinal in seconds (for recency normalization)
    oldest_secs: f64,
    newest_secs: f64,
}

impl StructuralConfidence {
    /// Build from a graph and its resolved `defeated` set. Corroboration/contest only count edges
    /// whose SOURCE is undefeated (a defeated source doesn't entrench). Honors both inline edges
    /// (corpus) and reified relation edge-beliefs (live store).
    pub fn build(g: &Graph, defeated: &HashSet<Id>) -> StructuralConfidence {
        let mut corrob: HashMap<Id, u32> = HashMap::new();
        let mut attacked: HashMap<Id, u32> = HashMap::new();
        let (mut oldest, mut newest) = (f64::MAX, f64::MIN);

        for b in g.content() {
            if let Some(t) = iso_to_secs(&b.txn_time) {
                oldest = oldest.min(t);
                newest = newest.max(t);
            }
        }
        for b in g.iter() {
            if defeated.contains(&b.id) {
                continue; // a defeated source's edges don't entrench / contest
            }
            for e in &b.edges {
                bump(&mut corrob, &mut attacked, &e.kind, &e.target);
            }
            if let Some(r) = &b.relation {
                bump(&mut corrob, &mut attacked, &r.kind, &r.object);
            }
        }
        if newest == f64::MIN {
            oldest = 0.0;
            newest = 0.0;
        }
        StructuralConfidence { corrob, attacked, oldest_secs: oldest, newest_secs: newest }
    }

    /// Incoming live `supports` count (entrenchment).
    pub fn corroboration(&self, id: &str) -> u32 {
        self.corrob.get(id).copied().unwrap_or(0)
    }
    /// Incoming live `attacks` count (this belief is under live dispute).
    pub fn contest(&self, id: &str) -> u32 {
        self.attacked.get(id).copied().unwrap_or(0)
    }
    /// True iff the belief is currently contested by a live `attacks`.
    pub fn is_contested(&self, id: &str) -> bool {
        self.contest(id) > 0
    }

    /// Recency in [0,1]: 1.0 = newest belief, decaying toward 0 for the oldest. Linear over the
    /// store's time span (robust to absolute epoch); 1.0 if there's no spread, 0.5 if unparseable.
    fn recency(&self, b: &Belief) -> f64 {
        let span = self.newest_secs - self.oldest_secs;
        if span <= 0.0 {
            return 1.0;
        }
        match iso_to_secs(&b.txn_time) {
            Some(t) => ((t - self.oldest_secs) / span).clamp(0.0, 1.0),
            None => 0.5,
        }
    }

    /// The effective weight in [0,1] — the V1 default `StructuralOnly`:
    /// `directness × source_weight × (1 + ln(1+corroboration)) × recency`, squashed to [0,1].
    /// Recency modulates within [0.5,1.0] so an old-but-solid belief isn't crushed.
    pub fn weight(&self, b: &Belief) -> f32 {
        let corr = self.corroboration(&b.id) as f64;
        let rec = self.recency(b);
        let raw = directness_weight(&b.directness)
            * (b.source_weight as f64)
            * (1.0 + (1.0 + corr).ln())
            * (0.5 + 0.5 * rec);
        // (1+ln(1+corr)) reaches ~2.8 by corr≈5; the /3.0 envelope keeps a well-supported, recent,
        // directly-stated belief near 1.0 while leaving headroom.
        (raw / 3.0).clamp(0.0, 1.0) as f32
    }

    /// Dubois–Prade ⟨possibility, necessity⟩ pair — the contested-belief affordance (carry, don't
    /// drive). `necessity` = the structural floor (how entrenched); `possibility` = `1 − disbelief`,
    /// where disbelief grows with live `attacks`. A contested belief has lower possibility while its
    /// necessity is unchanged — the honest "this current belief is under live attack" signal a single
    /// scalar can't encode. `possibility ≥ necessity` always.
    pub fn possibility_necessity(&self, b: &Belief) -> (f32, f32) {
        let nec = self.weight(b) as f64;
        let atk = self.contest(&b.id) as f64;
        let disbelief = 1.0 - (-0.7 * atk).exp(); // 0 attacks → 0, grows toward 1
        let poss = (1.0 - disbelief).clamp(nec, 1.0);
        (poss as f32, nec as f32)
    }
}

fn bump(
    corrob: &mut HashMap<Id, u32>,
    attacked: &mut HashMap<Id, u32>,
    kind: &EdgeKind,
    target: &str,
) {
    match kind {
        EdgeKind::Supports => *corrob.entry(target.to_string()).or_default() += 1,
        EdgeKind::Attacks => *attacked.entry(target.to_string()).or_default() += 1,
        _ => {}
    }
}

/// directness → multiplier. `linked` is the proximity-linker provenance on the live store; treat it
/// like `inferred`. Unknown directness gets a neutral 0.7.
fn directness_weight(d: &str) -> f64 {
    match d {
        "stated" => 1.0,
        "inferred" | "linked" => 0.6,
        "reduced" => 0.5,
        _ => 0.7,
    }
}

/// Parse an ISO-8601 `YYYY-MM-DD[THH:MM:SS]` prefix into an epoch-ordinal in seconds. We only need a
/// monotone mapping for recency, so this is days-since-civil-epoch × 86400 (+ HMS when present).
/// Hinnant's `days_from_civil`. Returns `None` on a too-short / unparseable stamp.
fn iso_to_secs(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.len() < 10 {
        return None;
    }
    let mut it = s[..10].split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let mo: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    let yy = if mo <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let doy = (153 * (if mo > 2 { mo - 3 } else { mo + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    let mut secs = days as f64 * 86400.0;
    if s.len() >= 19 {
        if let (Ok(h), Ok(mi), Ok(se)) =
            (s[11..13].parse::<f64>(), s[14..16].parse::<f64>(), s[17..19].parse::<f64>())
        {
            secs += h * 3600.0 + mi * 60.0 + se;
        }
    }
    Some(secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Edge;

    fn b(id: &str, directness: &str, sw: f32, txn: &str) -> Belief {
        Belief {
            id: id.into(),
            slug: id.into(),
            claim: id.into(),
            directness: directness.into(),
            source_weight: sw,
            txn_time: txn.into(),
            ..Belief::default()
        }
    }

    #[test]
    fn weight_is_bounded_and_rewards_directness() {
        let g = Graph::from_beliefs(vec![
            b("stated", "stated", 0.9, "2026-06-10T00:00:00Z"),
            b("inferred", "inferred", 0.9, "2026-06-10T00:00:00Z"),
        ]);
        let c = StructuralConfidence::build(&g, &g.defeated());
        let ws = c.weight(g.get("stated").unwrap());
        let wi = c.weight(g.get("inferred").unwrap());
        assert!((0.0..=1.0).contains(&ws) && (0.0..=1.0).contains(&wi));
        assert!(ws > wi, "a stated belief should outweigh an inferred one, all else equal");
    }

    #[test]
    fn recency_separates_a_version_chain_winner() {
        // older belief, superseded by a newer one. Among the *current* set the newer wins on recency.
        let old = b("old", "stated", 0.9, "2024-01-01T00:00:00Z");
        let mut new = b("new", "stated", 0.9, "2026-06-10T00:00:00Z");
        new.edges.push(Edge { kind: EdgeKind::Supersedes, target: "old".into() });
        let g = Graph::from_beliefs(vec![old, new]);
        let c = StructuralConfidence::build(&g, &g.defeated());
        assert!(
            c.weight(g.get("new").unwrap()) > c.weight(g.get("old").unwrap()),
            "the newer belief must outweigh the older one (recency is the live-store separator)"
        );
    }

    #[test]
    fn corroboration_lifts_weight_and_contest_lowers_possibility() {
        let mut core = b("core", "stated", 0.8, "2026-06-10T00:00:00Z");
        let _ = &mut core;
        let mut supporter = b("s1", "stated", 0.8, "2026-06-10T00:00:00Z");
        supporter.edges.push(Edge { kind: EdgeKind::Supports, target: "core".into() });
        let mut attacker = b("a1", "stated", 0.8, "2026-06-10T00:00:00Z");
        attacker.edges.push(Edge { kind: EdgeKind::Attacks, target: "core".into() });

        let g_plain = Graph::from_beliefs(vec![core.clone()]);
        let c_plain = StructuralConfidence::build(&g_plain, &g_plain.defeated());
        let w_plain = c_plain.weight(g_plain.get("core").unwrap());

        let g_sup = Graph::from_beliefs(vec![core.clone(), supporter]);
        let c_sup = StructuralConfidence::build(&g_sup, &g_sup.defeated());
        assert!(c_sup.weight(g_sup.get("core").unwrap()) > w_plain, "corroboration lifts weight");

        let g_atk = Graph::from_beliefs(vec![core, attacker]);
        let c_atk = StructuralConfidence::build(&g_atk, &g_atk.defeated());
        let (poss, nec) = c_atk.possibility_necessity(g_atk.get("core").unwrap());
        let (poss0, _) = c_plain.possibility_necessity(g_plain.get("core").unwrap());
        assert!(poss < poss0, "a live attack lowers possibility");
        assert!(poss >= nec, "possibility >= necessity by construction");
    }
}
