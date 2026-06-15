//! memory-core — the deterministic heart of the belief-memory system.
//!
//! Pure, no-LLM, no-network: belief model + loader + **frontier resolver** + the **relation
//! semantics registry** (which edge kinds defeat vs merely annotate) + the **Linker trait**
//! and value types (impls live in `memory-consolidate`; the trait stays here so it pulls no
//! LLM into core). First cut is hacky above L0; the belief *file format* is the durable part.

pub mod confidence;
pub use confidence::StructuralConfidence;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Id = String;

/// Relation/edge kind. Core kinds are named; anything else is a namespaced plugin kind
/// (`Other("myplugin:analogous")`) — the registry decides its semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeKind {
    Supports,
    Attacks,
    Supersedes,
    /// A deliberate retraction ("forget"): defeats its target with NO replacement claim — the
    /// distinction from `supersedes` (which means "a newer version of the same thing") matters,
    /// so `mem forget` doesn't masquerade as an update.
    Retracts,
    DerivedFrom,
    Refines,
    Adjudicates,
    /// JTMS assumption link: this belief is held *only because of* its target(s). It DOES NOT
    /// defeat its target — it points the other way (dependent → justification). A belief with
    /// `depends_on` edges goes OUT when ALL of its (in-force) justifications are defeated (proper
    /// Doyle in/out). Distinct from `supports` on purpose: `supports` is corroboration (V5 found
    /// 95% of sole-`supports` dependents are independently grounded, so withdrawing a supporter
    /// must NOT retract); only a separately-authored `depends_on` carries the justification contract.
    DependsOn,
    Other(String),
}

/// What a relation kind *does* to recall — the registry seam. Frontier-stage semantics
/// change the *current* set; surfacing-stage semantics (Annotate, and later Collapse/Boost)
/// change the ranked/deduped set without changing truth. First cut ships Defeat + Annotate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semantic {
    /// Frontier-stage: an undefeated edge of this kind defeats its target.
    Defeat,
    /// Surfacing-stage default: records a relation, does not change the frontier.
    Annotate,
    /// Surfacing-stage — subject and object are the SAME proposition; recall shows ONE
    /// representative and folds the rest behind it. Does NOT touch the frontier — members
    /// stay current/durable/relivable; only the DISPLAY folds.
    Collapse,
    /// Frontier-stage, JTMS in/out: a `depends_on` edge runs *backwards* of a defeat — the
    /// SOURCE (dependent) belief is defeated when ALL of its in-force justification targets are
    /// defeated. It never defeats its target. See `defeated()`.
    Justify,
}

impl EdgeKind {
    /// Infallible: an unknown string becomes a namespaced `Other`.
    pub fn parse(s: &str) -> EdgeKind {
        match s {
            "supports" => EdgeKind::Supports,
            "attacks" => EdgeKind::Attacks,
            "supersedes" => EdgeKind::Supersedes,
            "retracts" => EdgeKind::Retracts,
            "derived_from" => EdgeKind::DerivedFrom,
            "refines" => EdgeKind::Refines,
            "adjudicates" => EdgeKind::Adjudicates,
            "depends_on" => EdgeKind::DependsOn,
            other => EdgeKind::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            EdgeKind::Supports => "supports",
            EdgeKind::Attacks => "attacks",
            EdgeKind::Supersedes => "supersedes",
            EdgeKind::Retracts => "retracts",
            EdgeKind::DerivedFrom => "derived_from",
            EdgeKind::Refines => "refines",
            EdgeKind::Adjudicates => "adjudicates",
            EdgeKind::DependsOn => "depends_on",
            EdgeKind::Other(s) => s,
        }
    }

    /// THE registry seam. Today a match; later a pluggable `HashMap<String, Semantic>` that
    /// `memory-consolidate` / plugins populate when they register a relation type.
    pub fn semantic(&self) -> Semantic {
        match self {
            EdgeKind::Supersedes | EdgeKind::Adjudicates | EdgeKind::Retracts => Semantic::Defeat,
            // `depends_on` is JTMS: it moves the frontier, but backwards (defeats its SOURCE when
            // its targets all die), so it is its OWN stage — never `Defeat` (it must not defeat its
            // target) and never `Annotate` (it does change the frontier). See `defeated()`.
            EdgeKind::DependsOn => Semantic::Justify,
            // `same-as` is the Reducer's duplicate-fold edge: surfacing-stage, NOT a defeat — a
            // duplicate isn't false, just redundant, so it stays current and only the display folds.
            EdgeKind::Other(s) if s == "same-as" => Semantic::Collapse,
            _ => Semantic::Annotate, // supports/refines/attacks/derived_from/Other(..) annotate
        }
    }

    /// `attacks` alone is a *surfaced* conflict (Annotate), NOT a defeat — only an
    /// `adjudicates` verdict or a `supersedes` defeats.
    pub fn is_defeating(&self) -> bool {
        self.semantic() == Semantic::Defeat
    }

    /// Surfacing-stage duplicate fold: this edge marks subject and object as the SAME
    /// proposition. Recall shows one representative and folds the rest; the frontier is
    /// untouched (a collapsing edge never defeats — see `defeated()`).
    pub fn is_collapsing(&self) -> bool {
        self.semantic() == Semantic::Collapse
    }

    /// JTMS justification link (`depends_on`): frontier-stage, but defeats its SOURCE (not its
    /// target) once all the source's justifications are gone. See `defeated()`.
    pub fn is_justifying(&self) -> bool {
        self.semantic() == Semantic::Justify
    }

    /// Surfacing-stage subsumption: a "generic" relatedness edge (the proximity linker's
    /// `relates-to`, or `analogous`) is hidden at recall when a SPECIFIC edge already connects
    /// the same pair — it only surfaces when it's the sole link. Keeps expand/affordances clean.
    /// Truth/frontier are untouched; this purely dedupes the displayed set.
    pub fn is_generic(&self) -> bool {
        matches!(self, EdgeKind::Other(s) if s == "relates-to" || s == "analogous")
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub kind: EdgeKind,
    pub target: Id,
}

/// A reified relational belief: its claim is a triple `(subject) -kind-> (object)`. Every
/// *assertional* edge reifies into one of these so the relation can be argued/defeated
/// without touching either endpoint; only self-provenance (`derived_from`) stays inline.
#[derive(Debug, Clone)]
pub struct Relation {
    pub kind: EdgeKind,
    pub subject: Id,
    pub object: Id,
}

#[derive(Debug, Clone, Default)]
pub struct Belief {
    pub id: Id,
    pub slug: String,
    pub claim: String,
    pub project_scope: bool,
    pub author_kind: String,
    /// Author identity (e.g. a linker id like `judge@1` / `proximity@1`, or `cli`). Lets the
    /// CLI tell a regenerable machine edge from a human-authored one.
    pub author_id: String,
    pub directness: String,
    pub source_weight: f32,
    pub asserted: Option<f32>,
    pub edges: Vec<Edge>,
    /// Set iff this belief IS a reified relation (an edge-belief). When present, the belief
    /// asserts *about* two other beliefs and is hidden from recall's surfaced set.
    pub relation: Option<Relation>,
    /// Relevance scope: "" / "global" (everywhere), "repo:<id>" (repo canon), or
    /// "repo:<id>@<branch>" (provisional, branch-local). Recall filters by active scopes;
    /// filtering the subgraph BEFORE resolving the frontier gives branch divergence for free.
    pub scope: String,
    /// ISO-8601 transaction time (when recorded). Lexically ordered. Lets the judge only
    /// propose supersedes from the genuinely-newer belief.
    pub txn_time: String,
}

impl Belief {
    /// Parse one belief markdown file (YAML-ish frontmatter). Hand-rolled, zero-dep.
    pub fn parse(text: &str) -> Option<Belief> {
        let mut fm: Vec<&str> = Vec::new();
        let mut started = false;
        for line in text.lines() {
            if line.trim_end() == "---" {
                if !started {
                    started = true;
                } else {
                    break;
                }
                continue;
            }
            if started {
                fm.push(line);
            }
        }
        if fm.is_empty() {
            return None;
        }

        let mut b = Belief::default();
        let mut section = String::new();
        let mut collecting_claim = false;
        let mut pending_edge_kind: Option<EdgeKind> = None;
        let (mut rel_kind, mut rel_subj, mut rel_obj) =
            (None::<EdgeKind>, String::new(), String::new());

        for line in fm {
            let indent = line.len() - line.trim_start().len();
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if collecting_claim {
                if indent >= 4 {
                    if !b.claim.is_empty() {
                        b.claim.push(' ');
                    }
                    b.claim.push_str(trimmed);
                    continue;
                }
                collecting_claim = false;
            }

            if indent == 0 {
                let key = trimmed.split(':').next().unwrap_or("");
                let val = trimmed.splitn(2, ':').nth(1).unwrap_or("").trim();
                match key {
                    "id" => b.id = val.to_string(),
                    "slug" => b.slug = val.to_string(),
                    "scope" => b.scope = val.to_string(),
                    "kind" => {
                        if val == "project-scope" {
                            b.project_scope = true;
                        }
                    }
                    other => section = other.to_string(),
                }
            } else {
                match section.as_str() {
                    "claim" => {
                        if trimmed.starts_with("text:") {
                            collecting_claim = true;
                            b.claim.clear();
                        }
                    }
                    "author" => {
                        if let Some(v) = trimmed.strip_prefix("kind:") {
                            b.author_kind = v.trim().to_string();
                        } else if let Some(v) = trimmed.strip_prefix("id:") {
                            b.author_id = v.trim().to_string();
                        }
                    }
                    "confidence" => {
                        if let Some(v) = trimmed.strip_prefix("directness:") {
                            b.directness = v.trim().to_string();
                        } else if let Some(v) = trimmed.strip_prefix("source_weight:") {
                            b.source_weight = v.trim().parse().unwrap_or(0.0);
                        } else if let Some(v) = trimmed.strip_prefix("asserted:") {
                            let v = v.trim();
                            b.asserted = if v == "null" { None } else { v.parse().ok() };
                        }
                    }
                    "edges" => {
                        if let Some(v) = trimmed.strip_prefix("- kind:") {
                            pending_edge_kind = Some(EdgeKind::parse(v.trim()));
                        } else if let Some(v) = trimmed.strip_prefix("target:") {
                            if let Some(k) = pending_edge_kind.take() {
                                let target =
                                    v.trim().split_whitespace().next().unwrap_or("").to_string();
                                if !target.is_empty() {
                                    b.edges.push(Edge { kind: k, target });
                                }
                            }
                        }
                    }
                    // reified relation (edge-belief): kind/subject/object over two belief-ids
                    "relation" => {
                        if let Some(v) = trimmed.strip_prefix("kind:") {
                            rel_kind = Some(EdgeKind::parse(v.trim()));
                        } else if let Some(v) = trimmed.strip_prefix("subject:") {
                            rel_subj = v.trim().split_whitespace().next().unwrap_or("").to_string();
                        } else if let Some(v) = trimmed.strip_prefix("object:") {
                            rel_obj = v.trim().split_whitespace().next().unwrap_or("").to_string();
                        }
                    }
                    "provenance" => {
                        if let Some(v) = trimmed.strip_prefix("txn_time:") {
                            b.txn_time = v.trim().to_string();
                        }
                    }
                    _ => {}
                }
            }
        }

        if b.id.is_empty() {
            return None;
        }
        if let (Some(kind), false, false) = (rel_kind, rel_subj.is_empty(), rel_obj.is_empty()) {
            b.relation = Some(Relation { kind, subject: rel_subj, object: rel_obj });
        }
        Some(b)
    }
}

/// Cosine similarity between two equal-length vectors; 0.0 on degenerate input.
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let (mut dot, mut na, mut nb) = (0.0f32, 0.0f32, 0.0f32);
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    dot / (na.sqrt() * nb.sqrt())
}

// --- the Linker surface (trait + value types; impls live in memory-consolidate) ----------

/// A single ordinal confidence scale (no float, no taxonomy — add dimensions later if forced).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    Weak,
    Plausible,
    Strong,
}
impl Confidence {
    pub fn weight(self) -> f32 {
        match self {
            Confidence::Weak => 0.4,
            Confidence::Plausible => 0.65,
            Confidence::Strong => 0.9,
        }
    }
}

/// Cost class — the gate for cheaper-model tiering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Cheap,
    Mid,
    Expensive,
}

/// When a linker runs — the "sleep stages".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cadence {
    OnWrite,
    Nrem,
    Rem,
    OnDemand,
}

/// An author's relationship *hint* — a low-trust proposal the Linker consumes (never an edge).
#[derive(Debug, Clone)]
pub struct Hint {
    pub kind: EdgeKind,
    pub target_ref: String, // slug or id
}

/// A drafted edge — what a Linker PROPOSES. The consolidator is the sole writer; linkers
/// never commit, which keeps BYO-linkers safe and lets the agreement layer be one-point-reversible.
#[derive(Debug, Clone)]
pub struct LinkProposal {
    pub kind: EdgeKind,
    pub subject: Id,
    pub object: Id,
    pub confidence: Confidence,
    pub rationale: String,
    pub linker: String, // id@version of the proposing linker
}

/// What a linker gets to look at.
pub struct LinkCtx<'a> {
    pub new: &'a Belief,
    pub graph: &'a Graph,
    pub vectors: &'a HashMap<Id, Vec<f32>>,
    pub hints: &'a [Hint],
}

/// Bring-your-own-linker. Declares its cost/cadence so the orchestrator can schedule + budget;
/// `link` returns drafts, never writes.
pub trait Linker {
    fn id(&self) -> &str;
    fn tier(&self) -> Tier;
    fn cadence(&self) -> Cadence;
    fn link(&self, ctx: &LinkCtx) -> Vec<LinkProposal>;
}

// --- ids + time (shared by the surface so edge-beliefs get stable ids) -------------------

/// Placeholder content id (low 48 bits of std SipHash). Deterministic; real scheme is
/// sha256(observation)[:12]. Stable for a given seed → idempotent edge ids.
pub fn content_id(seed: &str) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    seed.hash(&mut h);
    format!("b_{:012x}", h.finish() & 0xffff_ffff_ffff)
}

pub fn iso_now() -> String {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();
    let z = (secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let (y, m, d) = (y + if m <= 2 { 1 } else { 0 }, m, d);
    let sod = secs % 86400;
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{millis:03}Z", sod / 3600, (sod % 3600) / 60, sod % 60)
}

/// An in-memory belief graph for one corpus / world `main`.
///
/// THE BACKEND SEAM. Fields are private; every consumer goes through the accessors below
/// (`iter`/`content`/`relations`/`by_id`/`current_content`/`adjacency`), so this struct is the
/// only thing that knows beliefs live in a `Vec` loaded from `.md` files. Swapping in an
/// embedded database means reimplementing THIS, not touching the callers. Per the L0 contract
/// (belief files are the durable part, everything above is derived and disposable), a DB
/// backend should be a regenerable INDEX over the files, not a second source of truth.
/// See `docs/design/storage-backends.md` — LadybugDB (`lbug`) is the standing candidate.
pub struct Graph {
    beliefs: Vec<Belief>,
    id_index: HashMap<Id, usize>,
    slug_index: HashMap<String, usize>,
}

impl Graph {
    pub fn from_beliefs(beliefs: Vec<Belief>) -> Graph {
        let mut id_index = HashMap::new();
        let mut slug_index = HashMap::new();
        for (i, b) in beliefs.iter().enumerate() {
            id_index.insert(b.id.clone(), i);
            slug_index.insert(b.slug.clone(), i);
        }
        Graph { beliefs, id_index, slug_index }
    }

    pub fn load_dir(dir: &Path) -> std::io::Result<Graph> {
        let mut beliefs = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let text = fs::read_to_string(&path)?;
                if let Some(b) = Belief::parse(&text) {
                    beliefs.push(b);
                }
            }
        }
        beliefs.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(Graph::from_beliefs(beliefs))
    }

    pub fn get(&self, slug: &str) -> Option<&Belief> {
        self.slug_index.get(slug).map(|&i| &self.beliefs[i])
    }

    /// O(1) lookup by belief id.
    pub fn by_id(&self, id: &str) -> Option<&Belief> {
        self.id_index.get(id).map(|&i| &self.beliefs[i])
    }

    /// id-or-slug → canonical id (prefers id).
    pub fn resolve_ref(&self, r: &str) -> Option<Id> {
        if self.id_index.contains_key(r) {
            Some(r.to_string())
        } else {
            self.slug_index.get(r).map(|&i| self.beliefs[i].id.clone())
        }
    }

    pub fn len(&self) -> usize {
        self.beliefs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.beliefs.is_empty()
    }

    /// Every belief — content AND edge-beliefs.
    pub fn iter(&self) -> impl Iterator<Item = &Belief> {
        self.beliefs.iter()
    }

    /// Content beliefs only (the propositions; edge-beliefs excluded).
    pub fn content(&self) -> impl Iterator<Item = &Belief> {
        self.beliefs.iter().filter(|b| b.relation.is_none())
    }

    /// Edge-beliefs, each with its reified relation.
    pub fn relations(&self) -> impl Iterator<Item = (&Belief, &Relation)> {
        self.beliefs.iter().filter_map(|b| b.relation.as_ref().map(|r| (b, r)))
    }

    /// The surfaced set: undefeated content beliefs. Takes `defeated` so a caller that also
    /// needs the defeat set for adjacency/fold pays for ONE frontier resolution, not three.
    pub fn current_content(&self, defeated: &HashSet<Id>) -> Vec<&Belief> {
        self.content().filter(|b| !defeated.contains(&b.id)).collect()
    }

    /// Index from belief-id → the in-force (undefeated) edge-beliefs touching it (as subject
    /// or object). Deterministic; powers recall affordances + `mem expand`.
    pub fn adjacency(&self, defeated: &HashSet<Id>) -> HashMap<Id, Vec<Relation>> {
        let mut adj: HashMap<Id, Vec<Relation>> = HashMap::new();
        for (b, r) in self.relations() {
            if defeated.contains(&b.id) {
                continue; // a defeated edge is no longer in force
            }
            adj.entry(r.subject.clone()).or_default().push(r.clone());
            if r.object != r.subject {
                adj.entry(r.object.clone()).or_default().push(r.clone()); // avoid double-add on self-anchored edges (forget)
            }
        }
        adj
    }

    /// **Frontier resolution.** Returns the DEFEATED ids on `main`. Honors BOTH inline defeating
    /// edges (corpus back-compat) and reified `Relation` edge-beliefs, via `EdgeKind::semantic()`.
    ///
    /// Two defeat modes with DIFFERENT monotonicity (the eval caught the bug of conflating them):
    ///  - `supersedes` is a VERSION CHAIN → **monotonic**. Once a belief is superseded it stays
    ///    superseded even when its superseder is itself superseded — a newer version (v3) can't
    ///    revive the oldest (v1). So a belief defeated *by supersession* STILL fires its own
    ///    `supersedes` edges (the chain keeps biting forward). Only a *verdict* against the
    ///    superseder un-supersedes it.
    ///  - `adjudicates`/`retracts` are VERDICTS → **non-monotonic**. A defeated verdict bites
    ///    nothing, so a verdict-of-a-verdict (or forgetting a retraction) reinstates the original.
    ///    A belief defeated *by a verdict* fires nothing.
    pub fn defeated(&self) -> HashSet<Id> {
        let mut defeated: HashSet<Id> = HashSet::new(); // all defeated (supersession OR verdict)
        let mut by_verdict: HashSet<Id> = HashSet::new(); // defeated by a non-monotonic kind

        // JTMS justification map (`depends_on`): dependent-id → its justification targets, each
        // tagged with the carrier whose defeat removes that edge (None = inline edge, always in
        // force; Some(eb) = a reified edge-belief, in force only while undefeated). Built once;
        // empty on every corpus/real store today (no `depends_on` edges yet), so the closure below
        // is inert and `defeated()` is unchanged until a Linker authors them. (V5/N4.)
        let mut dep_map: HashMap<Id, Vec<(Id, Option<Id>)>> = HashMap::new();
        for b in &self.beliefs {
            for e in &b.edges {
                if matches!(e.kind, EdgeKind::DependsOn) && self.id_index.contains_key(&e.target) {
                    dep_map.entry(b.id.clone()).or_default().push((e.target.clone(), None));
                }
            }
            if let Some(r) = &b.relation {
                if matches!(r.kind, EdgeKind::DependsOn)
                    && self.id_index.contains_key(&r.subject)
                    && self.id_index.contains_key(&r.object)
                {
                    dep_map
                        .entry(r.subject.clone())
                        .or_default()
                        .push((r.object.clone(), Some(b.id.clone())));
                }
            }
        }

        for _ in 0..(self.beliefs.len() + 5) {
            let (mut next, mut next_v): (HashSet<Id>, HashSet<Id>) = (HashSet::new(), HashSet::new());
            for b in &self.beliefs {
                // `supersedes` (monotonic): fires unless this belief was VERDICT-defeated — a
                // supersession-defeated belief keeps superseding its own targets (chain persists).
                if !by_verdict.contains(&b.id) {
                    for e in &b.edges {
                        if matches!(e.kind, EdgeKind::Supersedes) && self.id_index.contains_key(&e.target) {
                            next.insert(e.target.clone());
                        }
                    }
                    if let Some(r) = &b.relation {
                        if matches!(r.kind, EdgeKind::Supersedes) && self.id_index.contains_key(&r.object) {
                            next.insert(r.object.clone());
                        }
                    }
                }
                // verdicts (`adjudicates`/`retracts`/any other defeating kind): non-monotonic —
                // fire only if this belief is fully current.
                if !defeated.contains(&b.id) {
                    for e in &b.edges {
                        if e.kind.is_defeating() && !matches!(e.kind, EdgeKind::Supersedes)
                            && self.id_index.contains_key(&e.target)
                        {
                            next.insert(e.target.clone());
                            next_v.insert(e.target.clone());
                        }
                    }
                    if let Some(r) = &b.relation {
                        if r.kind.is_defeating() && !matches!(r.kind, EdgeKind::Supersedes)
                            && self.id_index.contains_key(&r.object)
                        {
                            next.insert(r.object.clone());
                            next_v.insert(r.object.clone());
                        }
                    }
                }
            }
            // JTMS closure (`depends_on`): a belief goes OUT when it has ≥1 in-force justification
            // and ALL of them are defeated. Iterate to internal closure so a chain a→b→c collapses
            // fully within this outer step; cascades compose with the native defeat above. A belief
            // whose justification EDGES are all removed (carrier defeated) is left alone — that's a
            // retracted dependency, not a retracted belief (conservative: never over-retract).
            if !dep_map.is_empty() {
                loop {
                    let mut changed = false;
                    for (dependent, js) in &dep_map {
                        if next.contains(dependent) {
                            continue;
                        }
                        let active: Vec<&Id> = js
                            .iter()
                            .filter(|(_, carrier)| carrier.as_ref().map_or(true, |c| !next.contains(c)))
                            .map(|(tgt, _)| tgt)
                            .collect();
                        if !active.is_empty() && active.iter().all(|j| next.contains(*j)) {
                            next.insert(dependent.clone());
                            changed = true;
                        }
                    }
                    if !changed {
                        break;
                    }
                }
            }

            if next == defeated && next_v == by_verdict {
                return defeated;
            }
            defeated = next;
            by_verdict = next_v;
        }
        defeated
    }

    /// Beliefs on the current `main` frontier (not defeated). Includes edge-beliefs; callers
    /// that surface to a human should additionally drop `relation.is_some()`.
    pub fn current(&self) -> Vec<&Belief> {
        let d = self.defeated();
        self.beliefs.iter().filter(|b| !d.contains(&b.id)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn content(id: &str, slug: &str) -> Belief {
        Belief { id: id.into(), slug: slug.into(), claim: format!("claim {slug}"), ..Belief::default() }
    }

    #[test]
    fn same_as_is_a_collapse_semantic() {
        let k = EdgeKind::Other("same-as".into());
        assert_eq!(k.semantic(), Semantic::Collapse);
        assert!(k.is_collapsing());
        // and it is NOT a defeat — a duplicate isn't false, just redundant
        assert!(!k.is_defeating());
    }

    #[test]
    fn depends_on_is_a_justify_semantic_not_a_defeat() {
        let k = EdgeKind::parse("depends_on");
        assert_eq!(k, EdgeKind::DependsOn);
        assert_eq!(k.as_str(), "depends_on");
        assert_eq!(k.semantic(), Semantic::Justify);
        assert!(k.is_justifying());
        assert!(!k.is_defeating(), "depends_on must NOT defeat its target");
    }

    #[test]
    fn depends_on_never_defeats_its_target() {
        // a depends_on b — b is live, so neither goes out (the edge points dependent→justification).
        let mut a = content("a", "a");
        a.edges.push(Edge { kind: EdgeKind::DependsOn, target: "b".into() });
        let g = Graph::from_beliefs(vec![a, content("b", "b")]);
        assert!(g.defeated().is_empty());
    }

    #[test]
    fn depends_on_retracts_dependent_when_sole_justification_dies_transitively() {
        // theorem ⟶depends_on⟶ lemma ⟶depends_on⟶ axiom. Retract the axiom: JTMS takes lemma AND
        // theorem OUT (the cascade). This is the unsound case `supports` alone leaves floating (V5).
        let mut lemma = content("lm", "lemma");
        lemma.edges.push(Edge { kind: EdgeKind::DependsOn, target: "ax".into() });
        let mut theorem = content("th", "theorem");
        theorem.edges.push(Edge { kind: EdgeKind::DependsOn, target: "lm".into() });

        // baseline: nothing retracted → all current
        let g = Graph::from_beliefs(vec![content("ax", "axiom"), lemma.clone(), theorem.clone()]);
        assert!(g.defeated().is_empty(), "no retraction → all current");

        // retract the axiom with a `retracts` verdict
        let mut retractor = content("r", "retractor");
        retractor.edges.push(Edge { kind: EdgeKind::Retracts, target: "ax".into() });
        let g = Graph::from_beliefs(vec![content("ax", "axiom"), lemma, theorem, retractor]);
        let d = g.defeated();
        assert!(d.contains("ax"), "axiom retracted");
        assert!(d.contains("lm"), "lemma depends on the dead axiom → OUT");
        assert!(d.contains("th"), "theorem transitively OUT");
    }

    #[test]
    fn depends_on_survives_while_one_justification_lives() {
        // thm depends on {ax1, ax2}. Retract ax1 only → survives; retract both → OUT. (No over-retraction.)
        let mut th = content("th", "thm");
        th.edges.push(Edge { kind: EdgeKind::DependsOn, target: "a1".into() });
        th.edges.push(Edge { kind: EdgeKind::DependsOn, target: "a2".into() });
        let mut r1 = content("r1", "retract1");
        r1.edges.push(Edge { kind: EdgeKind::Retracts, target: "a1".into() });

        let g = Graph::from_beliefs(vec![th.clone(), content("a1", "ax1"), content("a2", "ax2"), r1.clone()]);
        let d = g.defeated();
        assert!(d.contains("a1"));
        assert!(!d.contains("th"), "one justification still lives → thm survives");

        let mut r2 = content("r2", "retract2");
        r2.edges.push(Edge { kind: EdgeKind::Retracts, target: "a2".into() });
        let g = Graph::from_beliefs(vec![th, content("a1", "ax1"), content("a2", "ax2"), r1, r2]);
        assert!(g.defeated().contains("th"), "all justifications dead → thm OUT");
    }

    #[test]
    fn collapse_does_not_drop_the_folded_member_from_truth() {
        // a `same-as` edge member→rep must NOT defeat its object: both stay current.
        let mut edge = content("e_sameas", "rel-sameas");
        edge.relation = Some(Relation {
            kind: EdgeKind::Other("same-as".into()),
            subject: "b_member".into(),
            object: "b_rep".into(),
        });
        let g = Graph::from_beliefs(vec![content("b_member", "member"), content("b_rep", "rep"), edge]);
        let d = g.defeated();
        assert!(!d.contains("b_rep"), "same-as must not defeat its object (rep)");
        assert!(!d.contains("b_member"), "same-as must not defeat its subject (member)");
    }
}
