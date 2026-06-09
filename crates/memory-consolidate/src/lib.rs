//! memory-consolidate — the impure consolidation tier (sibling to a future Reducer).
//!
//! Linkers PROPOSE edges; the **Consolidator** is the sole writer (commits immediately as
//! reified edge-beliefs; risk is resolved at recall, not by a staging gate that could loop).
//! The **Orchestrator** runs linkers by cadence. Two linkers ship:
//!   - `SupersedeHintLinker`  — Cheap/OnWrite, no embeddings: author hint → `supersedes` edge.
//!   - `ProximityLinker`      — Cheap/NREM, embeddings: nearest beliefs → `relates-to` edge.
//! Adding a third (BYO) linker is: implement `Linker`, push it into the orchestrator.

use memory_core::{
    content_id, cosine, iso_now, Belief, Cadence, Confidence, EdgeKind, LinkCtx, LinkProposal,
    Linker, Tier,
};
use std::path::Path;

// --- the Consolidator: the only thing that writes edges --------------------------------

pub struct Consolidator;

impl Consolidator {
    /// Commit proposals as reified edge-beliefs in `scope`. Idempotent: an edge's id is a
    /// hash of `(kind, subject, object)`, so re-proposing the same relation is a no-op.
    /// Returns the number of NEW edge-beliefs written.
    pub fn commit(dir: &Path, proposals: &[LinkProposal], scope: &str) -> usize {
        let mut written = 0;
        for p in proposals {
            let edge_id =
                content_id(&format!("{}|{}|{}", p.kind.as_str(), p.subject, p.object));
            let path = dir.join(format!("{edge_id}.md"));
            if path.exists() {
                continue; // already linked — dedup / idempotent
            }
            if std::fs::write(&path, relation_belief_md(&edge_id, p, scope)).is_ok() {
                written += 1;
            }
        }
        written
    }
}

fn relation_belief_md(edge_id: &str, p: &LinkProposal, scope: &str) -> String {
    let kind = p.kind.as_str();
    let mut s = String::new();
    s.push_str("---\n");
    s.push_str(&format!("id: {edge_id}\n"));
    s.push_str(&format!("slug: rel-{}\n", &edge_id[2..]));
    s.push_str(&format!("scope: {scope}\n"));
    s.push_str("claim:\n  kind: text\n  text: >-\n");
    s.push_str(&format!("    [{}] {kind} [{}]\n", p.subject, p.object));
    s.push_str(&format!("author:\n  kind: linker\n  id: {}\n", p.linker));
    s.push_str("provenance:\n");
    s.push_str(&format!("  txn_time: {}\n", iso_now()));
    s.push_str("  valid_time: null\n");
    s.push_str("  source:\n    kind: linker\n    session: consolidate\n    turn: 0\n");
    s.push_str(&format!("  refs:\n    - hinted_by:{}\n", p.subject));
    s.push_str("  derived_from: []\n");
    s.push_str("confidence:\n  directness: linked\n  observation_count: 1\n");
    s.push_str(&format!("  source_weight: {}\n  asserted: null\n", p.confidence.weight()));
    s.push_str(&format!("relation:\n  kind: {kind}\n  subject: {}\n  object: {}\n", p.subject, p.object));
    s.push_str("edges: []\ncoord: null\n---\n\n");
    s.push_str(&p.rationale);
    s.push('\n');
    s
}

// --- the Orchestrator: runs linkers by cadence -----------------------------------------

pub struct Orchestrator {
    pub linkers: Vec<Box<dyn Linker>>,
}

impl Orchestrator {
    /// Write-time registry: cheap, no LLM generation (hint + proximity).
    pub fn with_defaults() -> Orchestrator {
        Orchestrator {
            linkers: vec![
                Box::new(SupersedeHintLinker),
                Box::new(ProximityLinker::default()),
            ],
        }
    }

    /// Deep registry for `mem consolidate` (the REM pass): proximity + the LLM judge.
    pub fn deep() -> Orchestrator {
        Orchestrator {
            linkers: vec![
                Box::new(ProximityLinker::default()),
                Box::new(JudgmentLinker::from_env()),
            ],
        }
    }

    /// Run every linker whose cadence is in `cadences`, collecting their proposals.
    pub fn run(&self, ctx: &LinkCtx, cadences: &[Cadence]) -> Vec<LinkProposal> {
        let mut props = Vec::new();
        for l in &self.linkers {
            if cadences.contains(&l.cadence()) {
                props.extend(l.link(ctx));
            }
        }
        props
    }
}

// --- Linker 1: supersede-from-hint (Cheap, OnWrite, no LLM) -----------------------------

pub struct SupersedeHintLinker;

impl Linker for SupersedeHintLinker {
    fn id(&self) -> &str {
        "supersede-hint@1"
    }
    fn tier(&self) -> Tier {
        Tier::Cheap
    }
    fn cadence(&self) -> Cadence {
        Cadence::OnWrite
    }
    fn link(&self, ctx: &LinkCtx) -> Vec<LinkProposal> {
        let mut out = Vec::new();
        for h in ctx.hints {
            if matches!(h.kind, EdgeKind::Supersedes) {
                if let Some(old) = ctx.graph.resolve_ref(&h.target_ref) {
                    if old != ctx.new.id {
                        out.push(LinkProposal {
                            kind: EdgeKind::Supersedes,
                            subject: ctx.new.id.clone(),
                            object: old,
                            confidence: Confidence::Strong,
                            rationale: "author hint: this belief supersedes the target".into(),
                            linker: self.id().into(),
                        });
                    }
                }
            }
        }
        out
    }
}

// --- Linker 2: embedding proximity (Cheap, NREM) ---------------------------------------

pub struct ProximityLinker {
    pub k: usize,
    pub strong: f32,
    pub plausible: f32,
}

impl Default for ProximityLinker {
    fn default() -> Self {
        // High threshold + small k: only near-duplicate-strength relations (useful for the
        // future collapse/dedup semantic). Avoids a dense relates-to mat in coherent stores.
        ProximityLinker { k: 3, strong: 0.88, plausible: 0.80 }
    }
}

impl Linker for ProximityLinker {
    fn id(&self) -> &str {
        "proximity@1"
    }
    fn tier(&self) -> Tier {
        Tier::Cheap
    }
    fn cadence(&self) -> Cadence {
        Cadence::Nrem
    }
    fn link(&self, ctx: &LinkCtx) -> Vec<LinkProposal> {
        let Some(nv) = ctx.vectors.get(&ctx.new.id) else {
            return Vec::new(); // no embedding for the new belief — skip gracefully
        };
        let mut scored: Vec<(&Belief, f32)> = ctx
            .graph
            .beliefs
            .iter()
            .filter(|b| b.id != ctx.new.id && b.relation.is_none())
            .filter_map(|b| ctx.vectors.get(&b.id).map(|v| (b, cosine(nv, v))))
            .filter(|(_, s)| *s >= self.plausible)
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored
            .into_iter()
            .take(self.k)
            .map(|(b, s)| LinkProposal {
                // a non-defeating annotation kind — never touches the frontier
                kind: EdgeKind::Other("relates-to".into()),
                subject: ctx.new.id.clone(),
                object: b.id.clone(),
                confidence: if s >= self.strong {
                    Confidence::Strong
                } else {
                    Confidence::Plausible
                },
                rationale: format!("embedding similarity {s:.2}"),
                linker: self.id().into(),
            })
            .collect()
    }
}

// --- Linker 3: LLM judgment (Mid tier, REM cadence) ------------------------------------

/// Candidate-generation → judgment: embedding-kNN finds candidate neighbors, then an LLM
/// (qwen2.5) classifies the directed relation A→B. Off the write hot path (REM cadence).
pub struct JudgmentLinker {
    pub url: String,
    pub model: String,
    pub k: usize,
    pub min_sim: f32,
}

impl JudgmentLinker {
    pub fn from_env() -> JudgmentLinker {
        JudgmentLinker {
            url: std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into()),
            model: std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into()),
            k: 3,
            min_sim: 0.55,
        }
    }
}

const JUDGE_SYSTEM: &str = "You judge the directed relation from belief A to belief B in a \
developer's memory, where A is the NEWER belief. Choose exactly one relation: \
\"supersedes\" = A and B are about the SAME specific point and A is its updated version, so B is \
now out of date (e.g. A says \"we now use X\" and B says \"we use Y\" for the same thing) — a NEW \
or additional fact on a related topic is NOT supersedes; \"refines\" = A adds detail to or narrows B, both \
still true; \"supports\" = A is independent evidence for B; \"attacks\" = A claims B is factually \
WRONG (a genuine contradiction, not merely a newer state); \"none\" = unrelated, or merely \
similar with no logical relation. A change of decision over time is supersedes, NOT attacks. \
Default to none unless the relation is clear. Reply ONLY with JSON shaped like \
{\"relation\":\"none\",\"confidence\":\"plausible\",\"rationale\":\"<one short sentence>\"} \
where confidence is weak, plausible, or strong.";

/// Adversarial second pass for proposed supersedes (high stakes — it drops a belief).
const VERIFY_SYSTEM: &str = "You verify whether belief B is made OBSOLETE by belief A. Answer \
outdated=true ONLY if A states an updated value for the SAME thing B is about, making B wrong \
to show now. If they are about different aspects, or both are still true, answer false. Reply \
JSON {\"outdated\": true|false}";

/// Adversarial pass for proposed attacks. A false conflict flag is noise, so the judge must
/// clear a pointed second call before we record a contradiction; otherwise we downgrade to a
/// plain (non-defeating) relation.
const ATTACK_VERIFY_SYSTEM: &str = "You verify whether belief A genuinely claims belief B is \
FACTUALLY WRONG — a real contradiction where both cannot be true at once. A being merely newer, \
about a different aspect, a stronger or critical opinion, or a design choice is NOT a \
contradiction. Reply JSON {\"contradicts\": true|false}";

impl Linker for JudgmentLinker {
    fn id(&self) -> &str {
        "judge@1"
    }
    fn tier(&self) -> Tier {
        Tier::Mid
    }
    fn cadence(&self) -> Cadence {
        Cadence::Rem
    }
    fn link(&self, ctx: &LinkCtx) -> Vec<LinkProposal> {
        let Some(nv) = ctx.vectors.get(&ctx.new.id) else {
            return Vec::new();
        };
        let mut cands: Vec<(&Belief, f32)> = ctx
            .graph
            .beliefs
            .iter()
            .filter(|b| b.id != ctx.new.id && b.relation.is_none())
            // A (the new/target belief) must be genuinely NEWER than B, so supersedes points
            // the right way — the judge can't infer recency from text.
            .filter(|b| b.txn_time < ctx.new.txn_time)
            .filter_map(|b| ctx.vectors.get(&b.id).map(|v| (b, cosine(nv, v))))
            .filter(|(_, s)| *s >= self.min_sim)
            .collect();
        cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        cands.truncate(self.k);

        let mut out = Vec::new();
        for (b, sim) in cands {
            let user = format!("A: {}\nB: {}", ctx.new.claim, b.claim);
            let v = match memory_embed::chat_json(&self.url, &self.model, JUDGE_SYSTEM, &user) {
                Ok(v) => v,
                Err(_) => continue, // judge unavailable for this pair → skip
            };
            let mut kind = match v.get("relation").and_then(|x| x.as_str()).unwrap_or("none") {
                "supersedes" => EdgeKind::Supersedes,
                "refines" => EdgeKind::Refines,
                "supports" => EdgeKind::Supports,
                "attacks" => EdgeKind::Attacks,
                _ => continue, // "none" or unknown → no edge
            };
            let conf = match v.get("confidence").and_then(|x| x.as_str()).unwrap_or("plausible") {
                "strong" => Confidence::Strong,
                "weak" => Confidence::Weak,
                _ => Confidence::Plausible,
            };
            // Stakes gate: a DEFEATING edge (supersedes) drops a belief from recall, so it must
            // clear a high bar — strong confidence AND high embedding similarity (a genuine
            // supersession is the SAME thing, restated; loosely-related newer beliefs are not).
            // Non-defeating edges (refines/supports/attacks) are harmless and pass freely.
            if matches!(kind, EdgeKind::Supersedes) && (conf != Confidence::Strong || sim < 0.78) {
                continue;
            }
            // Adversarial verify: a second pointed call must agree B is now obsolete. Catches
            // the over-eager "same topic, different aspect" supersessions the single judgment misses.
            if matches!(kind, EdgeKind::Supersedes) {
                let vu = format!("A: {}\nB: {}", ctx.new.claim, b.claim);
                let outdated = memory_embed::chat_json(&self.url, &self.model, VERIFY_SYSTEM, &vu)
                    .ok()
                    .and_then(|x| x.get("outdated").and_then(|o| o.as_bool()))
                    .unwrap_or(false);
                if !outdated {
                    continue; // verification rejected the supersession
                }
            }
            // Adversarial verify for attacks: a false "conflict" is noise. A second pointed call
            // must agree A genuinely claims B is WRONG; otherwise DOWNGRADE to a non-defeating
            // relates-to — keep the relation, drop the bogus conflict flag.
            let mut downgraded = false;
            if matches!(kind, EdgeKind::Attacks) {
                let vu = format!("A: {}\nB: {}", ctx.new.claim, b.claim);
                let contradicts = memory_embed::chat_json(&self.url, &self.model, ATTACK_VERIFY_SYSTEM, &vu)
                    .ok()
                    .and_then(|x| x.get("contradicts").and_then(|o| o.as_bool()))
                    .unwrap_or(false);
                if !contradicts {
                    kind = EdgeKind::Other("relates-to".into());
                    downgraded = true;
                }
            }
            let rationale = v.get("rationale").and_then(|x| x.as_str()).unwrap_or("");
            let rationale = if downgraded {
                format!("judge: downgraded attacks→relates-to (no real contradiction): {rationale}")
            } else {
                format!("judge: {rationale}")
            };
            out.push(LinkProposal {
                kind,
                subject: ctx.new.id.clone(),
                object: b.id.clone(),
                confidence: conf,
                rationale,
                linker: self.id().into(),
            });
        }
        out
    }
}

// === REM / novelty "dream" pass — an OBSERVABILITY ARTIFACT, deliberately NOT in the Orchestrator.
// Probes the MOST UNRELATED pairs (farthest cosine, no edge path) for a non-obvious bridge, and
// records EVERY probe — including the non-results ("earned unrelatedness") — so budget isn't
// re-burned. A rare bridge becomes a non-defeating `analogous` edge; the headline is the BRIDGE
// RATE (how integrated the knowledge is). Lives behind `mem dream`, not `mem consolidate`.

/// One recorded probe, positive OR negative. The cache of negative results is the whole point.
pub struct ProbeRecord {
    pub a: String,
    pub b: String,
    pub sim: f32,
    pub bridged: bool,
    pub insight: String,
    pub at: String,
}

/// Order-independent pair key, so a probe of (A,B) also covers (B,A).
pub fn novelty_pair_key(a: &str, b: &str) -> String {
    if a <= b { format!("{a}|{b}") } else { format!("{b}|{a}") }
}

const NOVELTY_SYSTEM: &str = "Two beliefs A and B from a developer's memory look UNRELATED. Find a \
NON-OBVIOUS, useful connection: a shared deep principle, a transferable technique, or an analogy \
that would actually inform one when working on the other. Reject shallow links (same language, \
both code, both 'about systems'). MOST pairs have NO real connection — say so. Reply ONLY JSON \
{\"bridge\":true|false,\"insight\":\"<one sentence naming the specific connection, else empty>\"}.";

/// The REM/novelty pass. Probes far pairs; the CLI owns the ledger (negative cache + metric) and
/// commits the rare bridges through the Consolidator — so this stays a pure proposer.
pub struct NoveltyDreamer {
    pub url: String,
    pub model: String,
    /// Skip pairs below this cosine — orthogonal embedding junk isn't "deliberately distant".
    pub floor: f32,
    /// Probe the K most-distant eligible peers per target belief.
    pub probes_per_target: usize,
}

impl NoveltyDreamer {
    pub fn from_env() -> NoveltyDreamer {
        NoveltyDreamer {
            url: std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into()),
            model: std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into()),
            floor: 0.05,
            probes_per_target: 3,
        }
    }

    /// For each target, probe its most-unrelated peers that are NOT already edge-connected and
    /// NOT already probed. Returns (bridge proposals, ALL probe records incl. non-results).
    pub fn dream(
        &self,
        targets: &[&Belief],
        graph: &memory_core::Graph,
        vectors: &std::collections::HashMap<String, Vec<f32>>,
        probed: &std::collections::HashSet<String>,
    ) -> (Vec<LinkProposal>, Vec<ProbeRecord>) {
        // pairs already joined by any reified edge — a bridged pair isn't "unrelated".
        let mut connected: std::collections::HashSet<String> = std::collections::HashSet::new();
        for b in &graph.beliefs {
            if let Some(r) = &b.relation {
                connected.insert(novelty_pair_key(&r.subject, &r.object));
            }
        }
        // `seen` = pairs already probed (the persisted ledger PLUS anything probed earlier this
        // run), so a pair is never probed twice — including from the other endpoint's target.
        let mut seen = probed.clone();
        let (mut proposals, mut records) = (Vec::new(), Vec::new());
        for t in targets {
            let Some(tv) = vectors.get(&t.id) else { continue };
            let mut cands: Vec<(&Belief, f32)> = graph
                .beliefs
                .iter()
                .filter(|b| b.id != t.id && b.relation.is_none())
                .filter(|b| !connected.contains(&novelty_pair_key(&t.id, &b.id)))
                .filter(|b| !seen.contains(&novelty_pair_key(&t.id, &b.id)))
                .filter_map(|b| vectors.get(&b.id).map(|v| (b, cosine(tv, v))))
                .filter(|(_, s)| *s >= self.floor)
                .collect();
            // ascending cosine → the MOST unrelated probed first (highest-surprise bridges).
            cands.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            cands.truncate(self.probes_per_target);
            for (b, sim) in cands {
                seen.insert(novelty_pair_key(&t.id, &b.id)); // claim the pair before probing it
                let user = format!("A: {}\nB: {}", t.claim, b.claim);
                let v = match memory_embed::chat_json(&self.url, &self.model, NOVELTY_SYSTEM, &user) {
                    Ok(v) => v,
                    Err(_) => continue, // judge down for this pair → not recorded; retry next run
                };
                let bridged = v.get("bridge").and_then(|x| x.as_bool()).unwrap_or(false);
                let insight =
                    v.get("insight").and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
                let real = bridged && !insight.is_empty();
                records.push(ProbeRecord {
                    a: t.id.clone(), b: b.id.clone(), sim, bridged: real, insight: insight.clone(), at: iso_now(),
                });
                if real {
                    proposals.push(LinkProposal {
                        kind: EdgeKind::Other("analogous".into()), // generic + Annotate: never defeats
                        subject: t.id.clone(),
                        object: b.id.clone(),
                        confidence: Confidence::Plausible, // speculative by construction
                        rationale: format!("novelty bridge (sim {sim:.2}): {insight}"),
                        linker: "novelty@1".into(),
                    });
                }
            }
        }
        (proposals, records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memory_core::{Graph, Hint};
    use std::collections::HashMap;

    fn content(id: &str, slug: &str) -> Belief {
        Belief { id: id.into(), slug: slug.into(), claim: format!("claim {slug}"), ..Belief::default() }
    }

    #[test]
    fn novelty_pair_key_is_order_independent() {
        assert_eq!(novelty_pair_key("a", "b"), novelty_pair_key("b", "a"));
        assert_ne!(novelty_pair_key("a", "b"), novelty_pair_key("a", "c"));
    }

    #[test]
    fn supersede_hint_links_and_defeats() {
        let dir = std::env::temp_dir().join(format!("mc-test-{}", content_id("sup")));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let g = Graph::from_beliefs(vec![content("b_old", "old"), content("b_new", "new")]);
        let vectors = HashMap::new();
        let new = g.beliefs.iter().find(|b| b.id == "b_new").unwrap();
        let hints = vec![Hint { kind: EdgeKind::Supersedes, target_ref: "old".into() }];
        let ctx = LinkCtx { new, graph: &g, vectors: &vectors, hints: &hints };

        let props = Orchestrator::with_defaults().run(&ctx, &[Cadence::OnWrite, Cadence::Nrem]);
        assert_eq!(props.len(), 1, "one supersede proposal");
        assert_eq!(Consolidator::commit(&dir, &props, "global"), 1);
        assert_eq!(Consolidator::commit(&dir, &props, "global"), 0, "idempotent re-commit");

        // reload with the content beliefs + the new edge-belief; old must be defeated
        let mut beliefs = vec![content("b_old", "old"), content("b_new", "new")];
        let edge_path = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .next()
            .unwrap();
        beliefs.push(Belief::parse(&std::fs::read_to_string(edge_path).unwrap()).unwrap());
        let g2 = Graph::from_beliefs(beliefs);
        assert!(g2.defeated().contains("b_old"), "old defeated via reified edge-belief");
        assert!(!g2.defeated().contains("b_new"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn proximity_proposes_relates_to_for_near_beliefs() {
        let g = Graph::from_beliefs(vec![content("b_new", "new"), content("b_near", "near"), content("b_far", "far")]);
        let mut vectors = HashMap::new();
        vectors.insert("b_new".to_string(), vec![1.0, 0.0, 0.0]);
        vectors.insert("b_near".to_string(), vec![0.95, 0.05, 0.0]); // ~0.99 cosine
        vectors.insert("b_far".to_string(), vec![0.0, 1.0, 0.0]); // 0.0 cosine
        let new = g.beliefs.iter().find(|b| b.id == "b_new").unwrap();
        let ctx = LinkCtx { new, graph: &g, vectors: &vectors, hints: &[] };

        let props = ProximityLinker::default().link(&ctx);
        assert_eq!(props.len(), 1, "only the near belief is above threshold");
        assert_eq!(props[0].object, "b_near");
        assert_eq!(props[0].kind.as_str(), "relates-to");
        assert!(!props[0].kind.is_defeating(), "relates-to must not affect the frontier");
    }
}
