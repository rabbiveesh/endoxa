//! memory-consolidate — the impure consolidation tier (sibling to a future Reducer).
//!
//! Linkers PROPOSE edges; the **Consolidator** is the sole writer (commits immediately as
//! reified edge-beliefs; risk is resolved at recall, not by a staging gate that could loop).
//! The **Orchestrator** runs linkers by cadence. Two linkers ship:
//!   - `SupersedeHintLinker`  — Cheap/OnWrite, no embeddings: author hint → `supersedes` edge.
//!   - `ProximityLinker`      — Cheap/NREM, embeddings: nearest beliefs → `relates-to` edge.
//! Adding a third (BYO) linker is: implement `Linker`, push it into the orchestrator.

use memory_core::{
    content_id, cosine, iso_now, Belief, Cadence, Confidence, EdgeKind, Graph, LinkCtx,
    LinkProposal, Linker, Tier,
};
use std::collections::HashMap;
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

// --- the Reducer: deterministic duplicate-collapse -------------------------------------
//
// A SURFACING-stage pass, NOT a frontier defeat: a duplicate isn't false, just redundant, so the
// `same-as` edge it emits is `Semantic::Collapse` (recall folds the display) and never drops a
// belief from truth. It does NOT run a fresh all-pairs embedding clustering — it REUSES the
// Linker's existing edges, re-gating each on `sim >= dup_sim` so a loosened linker can't widen
// clusters. FEEDBACK GUARD: reads only L0 content beliefs and explicitly skips any `same-as` edge,
// so it never re-enters its own output.

pub struct Reducer {
    /// Re-gate threshold: union two edge endpoints only when their similarity meets this bar.
    pub dup_sim: f32,
}

impl Default for Reducer {
    fn default() -> Self {
        Reducer { dup_sim: 0.94 }
    }
}

impl Reducer {
    /// Cluster L0 content beliefs that are the SAME proposition and emit a `same-as` fold per
    /// non-representative member (member → rep). `sim(a, b)` returns the pair's similarity (None
    /// when an embedding is missing — treated as "not similar enough" so the edge is dropped).
    pub fn reduce(&self, graph: &Graph, sim: &dyn Fn(&str, &str) -> Option<f32>) -> Vec<LinkProposal> {
        // L0 content beliefs only — never read edge-beliefs (feedback guard).
        let content: Vec<&Belief> = graph.content().collect();
        let mut uf = UnionFind::new(content.iter().map(|b| b.id.clone()));

        // Signal (a): byte-identical trimmed claims merge directly (no similarity gate needed —
        // identical text IS the same proposition).
        let mut by_claim: HashMap<&str, &str> = HashMap::new();
        for b in &content {
            let claim = b.claim.trim();
            match by_claim.get(claim) {
                Some(first) => uf.union(first, &b.id),
                None => {
                    by_claim.insert(claim, &b.id);
                }
            }
        }

        // Signal (b): REUSE the Linker's existing edges. A `Supersedes`/`Adjudicates` edge or a
        // generic relatedness edge (`is_generic`) suggests "same proposition" — but only union when
        // re-gated by `sim >= dup_sim` (a loosened linker can't widen a duplicate cluster). Skip
        // `same-as` edges entirely so the Reducer never re-enters its own output.
        for (_b, r) in graph.relations() {
            if r.kind.is_collapsing() {
                continue; // feedback guard: don't read our own `same-as` edges
            }
            let mergeable = matches!(r.kind, EdgeKind::Supersedes | EdgeKind::Adjudicates) || r.kind.is_generic();
            if !mergeable {
                continue;
            }
            // both endpoints must be L0 content beliefs we're clustering
            if !uf.contains(&r.subject) || !uf.contains(&r.object) {
                continue;
            }
            if sim(&r.subject, &r.object).map(|s| s >= self.dup_sim).unwrap_or(false) {
                uf.union(&r.subject, &r.object);
            }
        }

        // Group members by cluster root; representative = lexically-smallest id (deterministic,
        // idempotent). Emit member → rep folds for every non-rep member.
        let mut clusters: HashMap<String, Vec<String>> = HashMap::new();
        for b in &content {
            let root = uf.find(&b.id);
            clusters.entry(root).or_default().push(b.id.clone());
        }
        let mut out = Vec::new();
        for members in clusters.values() {
            if members.len() < 2 {
                continue;
            }
            let rep = members.iter().min().unwrap().clone();
            let rep_slug = graph.by_id(&rep).map(|b| b.slug.clone()).unwrap_or_else(|| rep.clone());
            for m in members {
                if *m == rep {
                    continue;
                }
                out.push(LinkProposal {
                    kind: EdgeKind::Other("same-as".into()),
                    subject: m.clone(), // member → rep
                    object: rep.clone(),
                    confidence: Confidence::Strong,
                    rationale: format!("reducer: same proposition as [{rep_slug}]"),
                    linker: "reducer@1".into(),
                });
            }
        }
        // Deterministic ordering for stable output / idempotent commits.
        out.sort_by(|a, b| a.subject.cmp(&b.subject).then(a.object.cmp(&b.object)));
        out
    }
}

/// Tiny deterministic union-find over belief ids (path-compression-free; sets are small).
struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    fn new(ids: impl Iterator<Item = String>) -> UnionFind {
        let parent = ids.map(|id| (id.clone(), id)).collect();
        UnionFind { parent }
    }

    fn contains(&self, id: &str) -> bool {
        self.parent.contains_key(id)
    }

    fn find(&self, id: &str) -> String {
        let mut cur = id.to_string();
        while let Some(p) = self.parent.get(&cur) {
            if p == &cur {
                break;
            }
            cur = p.clone();
        }
        cur
    }

    fn union(&mut self, a: &str, b: &str) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        // point the lexically-larger root at the smaller — keeps the smallest id the root, which
        // makes `reduce`'s representative choice consistent with the union-find structure.
        let (root, child) = if ra < rb { (ra, rb) } else { (rb, ra) };
        self.parent.insert(child, root);
    }
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
            .content()
            .filter(|b| b.id != ctx.new.id)
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
    /// Optional STRONGER model for the high-stakes `depends_on` judgment (env `DEPENDS_MODEL`).
    /// qwen2.5:7b won't author `depends_on` at the n-way stage (measured: it says refines/supports),
    /// so when this is set we ESCALATE admissible supports/refines through a focused binary verify on
    /// this model and upgrade to `depends_on` on confirmation. Unset → no escalation, zero extra cost.
    pub depends_model: Option<String>,
    pub k: usize,
    pub min_sim: f32,
}

impl JudgmentLinker {
    pub fn from_env() -> JudgmentLinker {
        JudgmentLinker {
            url: std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into()),
            model: std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into()),
            depends_model: std::env::var("DEPENDS_MODEL").ok().filter(|s| !s.trim().is_empty()),
            k: 3,
            min_sim: 0.55,
        }
    }

    /// The model that adjudicates `depends_on` (the stronger one if configured, else the base judge).
    fn depends_judge(&self) -> &str {
        self.depends_model.as_deref().unwrap_or(&self.model)
    }

    /// Focused binary: does A rest ENTIRELY on B (JTMS dependency)? Used both to verify an n-way
    /// `depends_on` and to escalate a supports/refines into one. Runs on `depends_judge()`.
    fn rests_entirely_on(&self, a: &str, b: &str) -> bool {
        let vu = format!("A: {a}\nB: {b}");
        memory_embed::chat_json(&self.url, self.depends_judge(), DEPENDS_VERIFY_SYSTEM, &vu)
            .ok()
            .and_then(|x| x.get("depends").and_then(|o| o.as_bool()))
            .unwrap_or(false)
    }
}

const JUDGE_SYSTEM: &str = "You judge the directed relation from belief A to belief B in a \
developer's memory, where A is the NEWER belief. Choose exactly one relation: \
\"supersedes\" = A and B are about the SAME specific point and A is its updated version, so B is \
now out of date (e.g. A says \"we now use X\" and B says \"we use Y\" for the same thing) — a NEW \
or additional fact on a related topic is NOT supersedes; \"refines\" = A adds detail to or narrows B, both \
still true; \"supports\" = A is INDEPENDENT evidence for B: A stands on its own and would still be assertable if \
B were DELETED, and it happens to corroborate B; \"depends_on\" = A is a CONCLUSION DERIVED from B — \
apply the deletion test: if B were DELETED, A would no longer make sense or be justified, because A's \
ONLY grounding is B (often signalled by A saying \"because/therefore/so/this means\" about B's \
content). The deletion test is decisive: A still stands without B → supports; A collapses without B → \
depends_on; \"attacks\" = A claims B is factually \
WRONG (a genuine contradiction, not merely a newer state); \"none\" = unrelated, or merely \
similar with no logical relation. A change of decision over time is supersedes, NOT attacks. \
Default to none unless the relation is clear. Reply ONLY with JSON shaped like \
{\"relation\":\"none\",\"confidence\":\"plausible\",\"rationale\":\"<one short sentence>\"} \
where confidence is weak, plausible, or strong.";

/// Adversarial second pass for proposed supersedes (high stakes — it drops a belief). Also
/// extracts the CARRY-OVER: the load-bearing detail B states that A omits, which would otherwise
/// be lost once B is hidden behind the supersede. The carry-over rides the edge body so `mem
/// expand` shows it next to the winner — the displaced point survives without un-defeating B.
const VERIFY_SYSTEM: &str = "You verify whether belief B is made OBSOLETE by belief A. Answer \
outdated=true ONLY if A states an updated value for the SAME thing B is about, making B wrong \
to show now. If they are about different aspects, or both are still true, answer false. When \
outdated=true, also extract carry_over: the specific load-bearing detail B states that A does NOT \
contain — the concrete mechanism, value, reason, or step (e.g. WHAT was done or WHY it worked), \
quoted from B's own words. Do NOT return a title, label, or status phrase like \"step 1\" or \
\"confirmed working\"; return the substance it refers to. Use an empty string only if A already \
carries every concrete detail B has. Reply JSON {\"outdated\": true|false, \"carry_over\": \"<the \
concrete detail, or empty>\"}";

/// Adversarial pass for proposed attacks. A false conflict flag is noise, so the judge must
/// clear a pointed second call before we record a contradiction; otherwise we downgrade to a
/// plain (non-defeating) relation.
const ATTACK_VERIFY_SYSTEM: &str = "You verify whether belief A genuinely claims belief B is \
FACTUALLY WRONG — a real contradiction where both cannot be true at once. A being merely newer, \
about a different aspect, a stronger or critical opinion, or a design choice is NOT a \
contradiction. Reply JSON {\"contradicts\": true|false}";

/// Adversarial pass for proposed depends_on. A `depends_on` is JTMS: it can RETRACT A when B dies,
/// so a false one is dangerous (V5: 95% of sole-`supports` dependents are independently grounded).
/// The verifier must agree A would be UNJUSTIFIED without B; otherwise we downgrade to `supports`.
const DEPENDS_VERIFY_SYSTEM: &str = "You verify whether belief A rests ENTIRELY on belief B: would A \
become unjustified — no longer something you could assert — if B were proven false or withdrawn? \
Answer depends=true ONLY if A is a conclusion DERIVED from B with no independent grounding of its \
own. If A has any independent basis (it's a directly observed fact, or stands without B), answer \
false. Reply JSON {\"depends\": true|false}";

/// Frontier-review aid (safe, never commits): does `model` think belief A rests ENTIRELY on belief B
/// — a JTMS dependency? `mem review` uses this to ANNOTATE candidates so the frontier agent can
/// prioritize. Returns None if the model is unavailable. A high-recall model (e.g. gemma2:9b) is a
/// good candidate flagger here precisely because its false positives become review items a human
/// rejects, not committed edges.
pub fn model_thinks_depends(url: &str, model: &str, a: &str, b: &str) -> Option<bool> {
    let vu = format!("A: {a}\nB: {b}");
    memory_embed::chat_json(url, model, DEPENDS_VERIFY_SYSTEM, &vu)
        .ok()
        .and_then(|x| x.get("depends").and_then(|o| o.as_bool()))
}

/// V5 discipline: admit a `depends_on` ONLY when the dependent A is itself a derivation —
/// `directness` inferred or reduced. A `stated` belief is a direct observation (independently
/// grounded), so a judge's depends_on on it is downgraded to `supports`. Empty/unknown directness is
/// treated conservatively as not-admissible (corroboration, not justification).
fn depends_on_admissible(new_directness: &str) -> bool {
    matches!(new_directness, "inferred" | "reduced")
}

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
            .content()
            .filter(|b| b.id != ctx.new.id)
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
            let judged = v.get("relation").and_then(|x| x.as_str()).unwrap_or("none").to_string();
            let mut kind = match judged.as_str() {
                "supersedes" => EdgeKind::Supersedes,
                "refines" => EdgeKind::Refines,
                "supports" => EdgeKind::Supports,
                "depends_on" => EdgeKind::DependsOn,
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
            let mut carry_over = String::new();
            if matches!(kind, EdgeKind::Supersedes) {
                let vu = format!("A: {}\nB: {}", ctx.new.claim, b.claim);
                let verdict = memory_embed::chat_json(&self.url, &self.model, VERIFY_SYSTEM, &vu).ok();
                let outdated = verdict
                    .as_ref()
                    .and_then(|x| x.get("outdated").and_then(|o| o.as_bool()))
                    .unwrap_or(false);
                if !outdated {
                    continue; // verification rejected the supersession
                }
                // Capture the displaced detail so the winner's edge carries it (see VERIFY_SYSTEM).
                carry_over = verdict
                    .as_ref()
                    .and_then(|x| x.get("carry_over").and_then(|o| o.as_str()))
                    .unwrap_or("")
                    .trim()
                    .to_string();
            }
            // Adversarial verify for attacks: a false "conflict" is noise. A second pointed call
            // must agree A genuinely claims B is WRONG; otherwise DOWNGRADE to a non-defeating
            // relates-to — keep the relation, drop the bogus conflict flag.
            let mut downgrade_to: Option<&str> = None;
            if matches!(kind, EdgeKind::Attacks) {
                let vu = format!("A: {}\nB: {}", ctx.new.claim, b.claim);
                let contradicts = memory_embed::chat_json(&self.url, &self.model, ATTACK_VERIFY_SYSTEM, &vu)
                    .ok()
                    .and_then(|x| x.get("contradicts").and_then(|o| o.as_bool()))
                    .unwrap_or(false);
                if !contradicts {
                    kind = EdgeKind::Other("relates-to".into());
                    downgrade_to = Some("no real contradiction");
                }
            }
            // depends_on (JTMS, V5/N4): high-stakes — it can RETRACT A when its justification B dies,
            // so it must clear TWO guards or it falls back to plain `supports` (same direction, drops
            // the justification contract): (1) directness — A must itself be a derivation
            // (inferred/reduced), never an independently-grounded `stated` fact; (2) an adversarial
            // verify that A truly rests entirely on B. confidence must not be weak.
            // depends_on (JTMS, V5/N4): high-stakes — it can RETRACT A when its justification B dies, so
            // the local judge NEVER auto-commits it. If the n-way judge proposes depends_on directly, it
            // is admitted only when A is a derivation (directness gate), confidence isn't weak, AND a
            // focused binary verify confirms A rests entirely on B; otherwise it downgrades to plain
            // supports. We do NOT escalate supports/refines INTO depends_on from a local model: the A/B
            // (2026-06-15) measured qwen at 0/4 recall and gemma2:9b at 1/4 specificity (it calls
            // CI→GitHub a dependency) — no 8GB-class model is both high-recall and safe, and a false
            // depends_on wrongly retracts an independently-grounded belief. depends_on is authored by the
            // FRONTIER agent via `mem review` + `mem link`; a local DEPENDS_MODEL only ANNOTATES review.
            if matches!(kind, EdgeKind::DependsOn) {
                let admit = depends_on_admissible(&ctx.new.directness)
                    && conf != Confidence::Weak
                    && self.rests_entirely_on(&ctx.new.claim, &b.claim);
                if !admit {
                    kind = EdgeKind::Supports;
                    downgrade_to = Some("A is independently grounded — corroboration, not justification");
                }
            }
            let rationale = v.get("rationale").and_then(|x| x.as_str()).unwrap_or("");
            let mut rationale = match downgrade_to {
                Some(why) => format!("judge: downgraded {judged}→{} ({why}): {rationale}", kind.as_str()),
                None => format!("judge: {rationale}"),
            };
            // Fold the displaced detail onto the supersede edge so the winner carries it forward.
            if !carry_over.is_empty() {
                rationale.push_str(&format!("\ncarries: {carry_over}"));
            }
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
        for (_b, r) in graph.relations() {
            connected.insert(novelty_pair_key(&r.subject, &r.object));
        }
        // `seen` = pairs already probed (the persisted ledger PLUS anything probed earlier this
        // run), so a pair is never probed twice — including from the other endpoint's target.
        let mut seen = probed.clone();
        let (mut proposals, mut records) = (Vec::new(), Vec::new());
        for t in targets {
            let Some(tv) = vectors.get(&t.id) else { continue };
            let mut cands: Vec<(&Belief, f32)> = graph
                .content()
                .filter(|b| b.id != t.id)
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
    fn depends_on_proposal_round_trips_to_a_reified_edge_and_drives_jtms() {
        // The end-to-end wiring (independent of the LLM's recall): a `depends_on` LinkProposal →
        // Consolidator writes a reified edge-belief → it parses back → `defeated()` JTMS-retracts the
        // dependent when its justification is retracted. Proves the emission path the judge feeds into.
        use memory_core::Edge;
        let dir = std::env::temp_dir().join(format!("mc-dep-{}", content_id("dep-rt")));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let p = LinkProposal {
            kind: EdgeKind::DependsOn,
            subject: "b_concl".into(), // the derived belief
            object: "b_ground".into(), // its justification
            confidence: Confidence::Strong,
            rationale: "judge: A is derived from B".into(),
            linker: "judge@1".into(),
        };
        assert_eq!(Consolidator::commit(&dir, &[p], "global"), 1, "depends_on edge written");

        // content beliefs + a retractor of the ground + the committed reified depends_on edge-belief
        let mut beliefs = vec![content("b_ground", "ground"), content("b_concl", "concl")];
        let mut retractor = content("b_ret", "ret");
        retractor.edges.push(Edge { kind: EdgeKind::Retracts, target: "b_ground".into() });
        beliefs.push(retractor);
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                beliefs.push(Belief::parse(&std::fs::read_to_string(path).unwrap()).unwrap());
            }
        }
        let g = Graph::from_beliefs(beliefs);
        let d = g.defeated();
        assert!(d.contains("b_ground"), "ground is retracted");
        assert!(d.contains("b_concl"), "conclusion JTMS-retracted via the reified depends_on edge");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn depends_on_admissible_only_for_derivations() {
        // V5: a `stated` (directly observed) belief is independently grounded → not a justification
        // dependent; only inferred/reduced derivations may carry depends_on. Empty/unknown → no.
        assert!(depends_on_admissible("inferred"));
        assert!(depends_on_admissible("reduced"));
        assert!(!depends_on_admissible("stated"));
        assert!(!depends_on_admissible(""));
        assert!(!depends_on_admissible("linked"));
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
        let new = g.by_id("b_new").unwrap();
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
    fn reducer_folds_byte_identical_claims_member_into_smaller_id_rep() {
        // two beliefs, same trimmed claim, different ids → one `same-as` fold, member→rep, rep =
        // the lexically-smaller id. No edges and no embeddings needed (signal (a) alone fires).
        let mut a = content("b_zzz", "dup-a");
        let mut b = content("b_aaa", "dup-b");
        a.claim = "  the same proposition  ".into(); // leading/trailing space → trim must match
        b.claim = "the same proposition".into();
        let g = Graph::from_beliefs(vec![a, b]);
        let sim = |_: &str, _: &str| None; // no similarity needed for identical-claim merge

        let props = Reducer::default().reduce(&g, &sim);
        assert_eq!(props.len(), 1, "one fold for the duplicate pair");
        assert_eq!(props[0].kind.as_str(), "same-as");
        assert!(props[0].kind.is_collapsing(), "the fold edge is a Collapse semantic");
        assert!(!props[0].kind.is_defeating(), "collapse must never defeat");
        assert_eq!(props[0].object, "b_aaa", "rep is the lexically-smaller id");
        assert_eq!(props[0].subject, "b_zzz", "the other is the folded member (member→rep)");
        assert_eq!(props[0].linker, "reducer@1");
    }

    #[test]
    fn reducer_re_gates_linker_edges_on_dup_sim() {
        // a generic `relates-to` edge joins two DISTINCT-claim beliefs; it only collapses them when
        // the re-gate similarity clears dup_sim — a loosened linker can't widen the cluster.
        let mut rel = content("e_rel", "rel-relates");
        rel.relation = Some(memory_core::Relation {
            kind: EdgeKind::Other("relates-to".into()),
            subject: "b_one".into(),
            object: "b_two".into(),
        });
        let g = Graph::from_beliefs(vec![content("b_one", "one"), content("b_two", "two"), rel]);

        // below dup_sim → no fold
        let low = |_: &str, _: &str| Some(0.90f32);
        assert!(Reducer::default().reduce(&g, &low).is_empty(), "below dup_sim → no collapse");

        // at/above dup_sim → one fold (member→rep, rep = smaller id b_one)
        let high = |_: &str, _: &str| Some(0.96f32);
        let props = Reducer::default().reduce(&g, &high);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].object, "b_one");
        assert_eq!(props[0].subject, "b_two");
    }

    #[test]
    fn reducer_ignores_its_own_same_as_edges() {
        // feedback guard: a pre-existing `same-as` edge must not feed back into clustering, and the
        // folded member must not re-emit. Distinct claims, no qualifying edge → no proposals.
        let mut same = content("e_same", "rel-same");
        same.relation = Some(memory_core::Relation {
            kind: EdgeKind::Other("same-as".into()),
            subject: "b_member".into(),
            object: "b_rep".into(),
        });
        let g = Graph::from_beliefs(vec![content("b_member", "member"), content("b_rep", "rep"), same]);
        let sim = |_: &str, _: &str| Some(1.0f32);
        assert!(Reducer::default().reduce(&g, &sim).is_empty(), "reducer must not read its own same-as edges");
    }

    #[test]
    fn proximity_proposes_relates_to_for_near_beliefs() {
        let g = Graph::from_beliefs(vec![content("b_new", "new"), content("b_near", "near"), content("b_far", "far")]);
        let mut vectors = HashMap::new();
        vectors.insert("b_new".to_string(), vec![1.0, 0.0, 0.0]);
        vectors.insert("b_near".to_string(), vec![0.95, 0.05, 0.0]); // ~0.99 cosine
        vectors.insert("b_far".to_string(), vec![0.0, 1.0, 0.0]); // 0.0 cosine
        let new = g.by_id("b_new").unwrap();
        let ctx = LinkCtx { new, graph: &g, vectors: &vectors, hints: &[] };

        let props = ProximityLinker::default().link(&ctx);
        assert_eq!(props.len(), 1, "only the near belief is above threshold");
        assert_eq!(props[0].object, "b_near");
        assert_eq!(props[0].kind.as_str(), "relates-to");
        assert!(!props[0].kind.is_defeating(), "relates-to must not affect the frontier");
    }
}
