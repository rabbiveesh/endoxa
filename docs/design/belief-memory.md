# Belief-Based Agentic Memory — Design Draft

Status: **draft v0.2** · 2026-06-03 (holes settled 2026-06-15) · living doc, holes marked `HOLE`.
**Many `HOLE`s are now empirically resolved — see [open-questions-eval.md](open-questions-eval.md)
for the measured verdicts (V1–V6); inline `HOLE→V#` tags point to them.**

## 0. One-paragraph thesis

Don't store *facts*. Store **beliefs** — propositions held with provenance, an
epistemic envelope, and justification edges to other beliefs. The canonical
"fact" is never stored; it is the output of an **agentic reduction** over a
context-resolved neighborhood of beliefs, computed *relative to a world* and
cached lazily. Humans and AIs are symmetric authors. Parallel realities
(experiments, disagreements, what-ifs) are first-class via a git-like belief
DAG. The storage layer is permanent and dumb; every organizing structure above
it is derived and disposable.

This is the rule that keeps the whole thing from becoming Jenga:
**the layout is never the storage.**

---

## 1. Prior work we're standing on

We are not inventing belief revision; we're marrying the classic symbolic
machinery to an LLM reduction engine and a git-backed store. Map of what we
borrow from where:

| Source | What we take |
|---|---|
| **Truth Maintenance Systems** (Doyle 1979, JTMS) | beliefs carry *justifications*; retract a belief when its support is withdrawn. Non-destructive. |
| **Assumption-based TMS** (de Kleer 1986) | maintain **multiple consistent worlds at once**; label each belief with the assumption-sets under which it holds. → our "parallel realities". |
| **AGM belief revision** (Alchourrón/Gärdenfors/Makinson 1985) | expansion / revision / contraction; *minimal change* on new evidence. |
| **Belief bases vs belief sets** (Hansson) | store the finite *explicit* beliefs (the base); the closure/consensus is *derived*, not stored. This is exactly L0 vs reduction. |
| **Defeasible / non-monotonic logic** (Reiter default logic) | beliefs that hold "by default" until defeated → decay & attack edges. |
| **Subjective Logic** (Jøsang) | opinions as ⟨belief, disbelief, uncertainty, base-rate⟩ with *fusion operators* → reduction-as-fusion, structural confidence. |
| **Dempster–Shafer** | combining evidence without committing to a single prior. |
| **Bi-temporal modeling** (Snodgrass) | *valid time* (when the world was so) vs *transaction time* (when we learned it). Both go in provenance. |
| **Generative Agents** (Park et al. 2023) | memory stream + **reflection**; recall scored by recency × importance × relevance → our L2 recall scoring + L3 reduction. |
| **Zep / Graphiti**, **mem0**, **MemGPT/Letta** | production agent memory; temporal knowledge graph, edge invalidation, self-editing memory. Confirms the shape; none keep real epistemic status — that's our gap. |
| **Automerge / Patchwork** (Ink & Switch) | content-addressed history; a snapshot is a **set of heads**. → our "world = a set of heads"; recall-as-of-time = reliving. |
| **Git** | content-addressed Merkle DAG, branches, merges → the substrate model for parallel realities. |

**The gap we fill:** the reflection/consolidation line (Generative Agents,
mem0, Zep) skips the epistemic layer — items are facts with a recency score. The
belief-revision line (TMS/ATMS/AGM) predates LLMs and has no learned reduction.
We put an **LLM reducer on top of an explicitly epistemic, justification-linked,
git-backed belief base.**

**Two findings from the research pass that change the design:**

- **ATMS can be simulated inside AGM** (Dixon & Foo, IJCAI-93): encode
  justifications as epistemic entrenchment and the ATMS behaviour falls out,
  proven correct. This means our justification-edge store (TMS-flavoured) and an
  AGM-style revision discipline are *formally compatible*, not competing choices —
  we can keep edges as the data and use entrenchment as the ordering that drives
  retraction.
- **Merging has concrete, implementable operators** (Konieczny & Pino Pérez, IC
  merging): "which conflicting belief wins" is a solved menu — *majority* operators
  (sum / minisum of distances) vs *arbitration* operators (max / leximax) over a
  profile of bases under integrity constraints. These map straight onto reducer
  verdict strategies (§4a) — we don't have to invent verdict selection.

See §12 for the full annotated reading list and per-citation verification status.

---

## 2. The layers

Dependencies point **downward only**. L0 imports nothing. Everything above L0 is
rebuildable from L0.

```
L5  Surfacing       inject-to-agent · show-to-human · contribute UI
L4  Reduction cache  (world, context-sig, input-heads) → reduced output   [disposable]
L3  Reduction       agentic: neighborhood → consensus belief (recorded as derived)
L2  Neighborhood    (context, world, indices) → belief-set   ← the "lens" layer
L1  Indexing        embeddings · temporal · author · edge-graph   [derivable]
L0  Belief Log      append-only Merkle DAG · provenance · justification   [SOURCE OF TRUTH]
```

Each layer is an independent experiment surface — e.g. "the AI has a desk"
(spatial proximity) is *just an L2 lens* + an optional coordinate on L0; testable
without touching storage or reduction.

### The spine (load-bearing, design first)

The **justification edge** does four jobs with one immutable graph:

1. epistemic support — makes reduction reason well
2. cache dependency — L4 invalidation (input-heads = the edges, read backwards)
3. audit / reliving — *why* did we believe X in world W at time T
4. conflict surfacing — `attacks` edges are the disagreements to show, not hide

Get this graph right or the whole tower wobbles.

### The coupling to forbid (the Jenga trap)

Never persist a neighborhood/cluster. Store **indices** (cheap, derivable);
compute neighborhoods on demand. The moment neighborhoods become stored state,
"different neighbors in different contexts" turns into cache-coherence hell.
Persistent things are exactly two: **L0 beliefs** and **L4 reduction outputs +
their input manifests**. Nothing else.

---

## 3. L0 — the belief record (load-bearing draft)

Content-addressed and immutable. A belief is identified by the hash of its
content; it is never mutated, only superseded by a new belief that `supersedes`
it. The set of beliefs + edges is a Merkle DAG — git for epistemics.

```rust
/// Content address. Hash of the canonical-serialized BeliefBody.
pub struct BeliefId([u8; 32]);

pub struct Belief {
    pub id: BeliefId,           // = hash(body); not stored in body
    pub body: BeliefBody,
}

pub struct BeliefBody {
    pub claim: Claim,           // the proposition itself
    pub author: Author,         // human | agent | reducer — symmetric
    pub provenance: Provenance, // where it came from + bi-temporal stamps
    pub confidence: ConfidenceInputs, // STRUCTURAL inputs, not a float
    pub edges: Vec<Edge>,       // justification graph (see §4)
    pub coord: Option<Coord>,   // optional spatial position ("the desk"); L2 lens reads it
}

pub enum Claim {
    /// Natural-language proposition (the common case for LLM/human authoring).
    Text(String),
    /// Optional structured form for things we want to query/aggregate exactly.
    Triple { subject: String, predicate: String, object: String },
    // HOLE: do we want a typed schema registry, or stay text-first? lean text-first.
}

pub enum Author {
    Human { id: String },
    Agent { id: String, model: String },
    Reducer { id: String, model: String }, // derived beliefs (L3 output)
}

pub struct Provenance {
    pub valid_time: Option<TimeRange>,  // when the claim was true in the world
    pub txn_time: Timestamp,            // when WE recorded it (always known)
    pub source: Source,                 // conversation turn, file, tool output, human input…
    pub derived_from: Vec<BeliefId>,    // for reducer output: exact inputs (= input manifest)
}

pub enum Source {
    Conversation { session: String, turn: u64 },
    Human { channel: String },          // explicit human contribution
    Tool { name: String },
    Reduction,                          // produced by L3
    // HOLE: external doc ingestion shape
}

/// Store BOTH the structural inputs AND whatever confidence the author asserted.
/// Storage is cheap and immutable; what changes is the *interpretation*, which is
/// a pluggable read-time model (see §3a). Earlier draft said "never store a float" —
/// walked back: we store everything, we just don't *trust* the asserted float by
/// default. Confidence representation is itself a thing under test.
pub struct ConfidenceInputs {
    pub directness: Directness,         // Stated | Inferred | Reduced
    pub observation_count: u32,         // how many independent beliefs corroborate
    pub source_weight: f32,             // human > tool-verified > single LLM inference
    pub asserted: Option<f32>,          // what the author/LLM/human *claimed*; kept, not trusted
    // recency is computed at read-time from txn_time, not stored
}
pub enum Directness { Stated, Inferred, Reduced }
```

Notes / decisions:

- **One belief ≈ one file**, frontmatter + body, git-tracked — a direct
  evolution of the current `memory/*.md` + `MEMORY.md` scheme. `MEMORY.md`
  becomes a *generated* reduction view, not hand-maintained.
- **Confidence is stored but its interpretation is pluggable** (see §3a). We keep
  the structural inputs *and* the asserted float; a swappable read-time model
  decides what to do with them. This is a deliberate experimentation surface.
- **Author is just provenance.** Human input is a peer belief, never a
  privileged override — same write path, higher `source_weight`. A human belief
  can `attack` an agent belief and vice versa.

### 3a. Confidence as a pluggable model (experimentation surface)

We are explicitly *unsure* whether confidence should be stored-and-trusted or
derived-from-structure — so we don't bake the answer in. We store everything and
make interpretation a trait:

```rust
/// Maps a belief (+ its context & recency) to an effective weight in [0,1].
/// Multiple impls compete; we benchmark which one predicts correctness best (§11).
pub trait ConfidenceModel {
    fn weight(&self, b: &Belief, ctx: &Context, now: Timestamp) -> f32;
}

// Candidate impls to race against each other:
//   StructuralOnly  — ignores `asserted`; uses directness × log(obs) × source × recency
//   AssertedOnly    — trusts the author's float (baseline; MEASURED miscalibrated, §13B)
//   Blend           — calibrated mix; learn the weights from verdict labels (§4a, §9)
//   BayesianUpdate  — treat each corroborating belief as evidence; posterior
```

**Research verdict (now in, §13B): do not trust an LLM-asserted confidence float.**
The calibration literature finds a measured *mismatch* between verbalized
confidence and a model's underlying uncertainty — so `AssertedOnly` is a
baseline-to-beat, not a default. We still *store* `asserted` (cheap, and a human's
asserted confidence is more meaningful than an LLM's), but the default model
should be `StructuralOnly` or `Blend`. This is no longer a guess; it's the data.

**Representation sub-choice → V1 (RESOLVED):** ship the **scalar** as the frontier driver, and
*additionally* carry a **possibility/necessity pair** (Dubois–Prade) — or the SL triple — as a
**contested-belief affordance**. Measured: possibility separates contested-vs-uncontested *current*
beliefs by **+0.503** where the scalar manages only **+0.105** (≈5×). So the scalar is enough to
rank/drive recall, but the pair earns its keep as the honest "this current belief is under live
attack" signal a scalar can't encode. Candidates were (i) scalar; (ii) possibility/necessity pair
giving a "could be true / must be true" band that encodes ignorance; (iii) Subjective-Logic
⟨belief, disbelief, uncertainty⟩ triple. We adopt these as **graded representations**, never as
soundness — see §13E.

Because the model is swappable and worlds are branchable (§5), we can run
`StructuralOnly` in one world and `Blend` in another over the *same* beliefs and
score both against adjudicated verdicts (§9). Confidence representation becomes a
measured choice, not an architectural commitment.

`HOLE §3a → V1` (RESOLVED): **L1 (per-belief).** L2 query-boost is a tie on the fixtures but
`cosine(defeated) > cosine(current)` in 2/3 conflicted neighborhoods — a proximity boost structurally
points at the stale loser, so L2 must never flip a frontier verdict. Default model = recency-bearing
**StructuralOnly** (best ranker, corpus/real pair-acc 0.93/0.92; recency is non-negotiable — ablation
drops real to 0.17), post-hoc calibrated onto a source-weight prior if a probability is needed.
`AssertedOnly` is the worst ranker (real pair-acc 0.000) — confirms §13B empirically.

### 3b. Deficiency / known-debt axis (R4 finding — orthogonal to confidence)

Harvesting the **kludge / known-debt** flavor (corpus round R4, real-repo grounding:
`FIXME`/`HACK`/ADRs/commit rationale) surfaced a knowledge type the envelope
doesn't represent. A kludge belief — *"`platform_check.php` is shipped as an empty
no-op stub"*, *"GitHub-release download is a stopgap until the next npm version"*,
*"WASM save timestamp hardcoded to 0"* — is **true AND known-deficient at once.**
That is orthogonal to everything we model:

- not **confidence** — the kludge is *certain* (we know exactly that it's there);
- not **refutation** — it's *not wrong*, it's the actual current behaviour;
- not **supersession** — it hasn't been replaced, it's live.

So a belief needs a **deficiency axis** independent of `confidence`: *is this a
compromise, and how bad?* Plus two structured fields the harvest showed are almost
always present and load-bearing:

- **`forcing_constraint`** — *why* we accept the debt (e.g. "tree-sitter has no
  non-assoc primitive", "no native speech backend", "GLIBC < 2.39 on CI").
- **`revisit_when` / `blocked_on`** — the condition under which the debt should be
  reworked ("when miniquad releases the focus fix", "once procedural maps ship").
  This is a *trigger*: when the constraint lifts, the debt should **resurface**.

```rust
pub struct Deficiency {            // None for ordinary beliefs
    pub severity: Severity,        // Low | Medium | High
    pub forcing_constraint: String,
    pub revisit_when: Option<String>, // a condition; resolving it re-raises the debt
}
```

Implications:
- A new **query-type / capability**: *"what's hacky / known-debt around X that I
  shouldn't rely on or should fix?"* — distinct from must-know (warnings),
  hunch (uncertainty), and refutation (wrongness). The §9 usability run didn't see
  it because the corpus had almost no debt beliefs; R4 is collecting them.
- **Entrenchment ⊥ deficiency.** A belief can be maximally entrenched (definitely
  true) *and* maximally deficient (definitely should change). Two independent axes;
  confidence/entrenchment says nothing about *quality*.
- `HOLE §3b → V6 / N5` (RESOLVED + SHIPPED): **edge.** A `blocked_on` edge (Annotate semantics — it
  must NOT defeat, the debt is true-and-live) to the constraint belief; when the constraint is later
  defeated, `mem debt` walks `blocked_on` via `adjacency(&defeated)` and auto-resurfaces the debt
  (`⚠ RESURFACED`). Now implemented end-to-end: `Belief.deficiency` parses; **Tier-2 onboarding**
  (`mem onboard --tier2`) generates deficiency beliefs from debt leads; **`mem debt`** is the
  known-debt query. Verified live (gemma2:9b extraction on endoxa; link→forget→resurface).

> **Intent-dependent frontier (companion R4 finding).** The *design-rationale*
> flavor showed the dual: a "why X over Y" ask needs the **rejected alternative**
> surfaced — which is often a *superseded/reverted* belief. So the frontier filter
> that current-state asks want (drop the loser) is exactly what why-asks want to
> *keep*. **Frontier filtering must be intent-keyed, not a global default** —
> recorded against §6 Recall (`Resolution`/`Filters` need a "include-defeated" mode).

---

## 4. Edge taxonomy (the spine)

```rust
pub struct Edge { pub kind: EdgeKind, pub target: BeliefId }

pub enum EdgeKind {
    Supports,        // this belief is evidence FOR target
    Attacks,         // this belief contradicts/undermines target
    Supersedes,      // this belief replaces target (revision; target stays for audit)
    DerivedFrom,     // reducer output → its inputs (also the cache manifest)
    Refines,         // narrows/specializes target without contradicting it
    Adjudicates,     // a VERDICT: closes a conflict, declaring target defeated (see §4a)
    // HOLE §4 → V5 (RESOLVED): YES — add `DependsOn` (assumption link, JTMS-style) as DISTINCT from
    //   Supports. Measured: endoxa is TMS-unsound (20/20 corpus sole-support dependents float when
    //   their support is retracted); a DependsOn variant (OUT when ALL targets defeated) fixes 20/20,
    //   cascades correctly, never over-retracts. But do NOT auto-promote sole-`supports` to it —
    //   19/20 are `directness: stated` (independently grounded), so that over-retracts 95%. `supports`
    //   is corroboration, not justification. Inert on the real store today (0 supports edges).
}
```

- `Supersedes` is non-destructive — the old belief remains in the DAG so we can
  relive prior worlds. "Current" just means "not superseded along this world's
  frontier."
- `DerivedFrom` edges *are* the L4 cache dependency manifest. One mechanism.
- `Attacks` is surfaced, not auto-resolved — resolution is the reducer's job (or
  a human's), recorded as a new belief.

### 4a. Two kinds of resolution — consensus vs verdict

Your question: *can we store a final resolution — one theory actually being
right?* Yes. We distinguish two:

- **Soft resolution (consensus).** A `Reducer` belief that weighs an `Attacks`
  pair and reports the current best estimate. Derived, revisable, recomputed when
  evidence changes. This is the default and it never claims to be *true*.
- **Hard resolution (verdict).** A belief carrying an `Adjudicates` edge that
  *closes* the conflict: it declares a winner and marks the loser **defeated**
  along this world's frontier. Issued by a human, a high-authority agent, or by
  reality confirming an outcome. "One theory actually being right" = a verdict.

Both are **non-destructive** — the defeated belief stays in the DAG for
audit/reliving; it's just flagged defeated *along this world's frontier*. And
because worlds branch, a verdict is **per-world**: world A can hold the opposite
verdict from world B. That's epistemically correct — "right" is always relative
to a set of assumptions (ATMS). Ground truth, when it exists, is simply a verdict
in the canonical world with maximal source weight.

Why this matters beyond cleanliness: **verdicts are gold labels.** They give the
benchmarks (§9) a supervised signal — did the soft consensus match the eventual
hard verdict? That's directly measurable.

**Verdict selection has a ready-made algorithm menu** (Konieczny & Pino Pérez IC
merging — verified, §12). Treat the conflicting beliefs as a *profile* of bases
and pick an operator:
- **Majority / sum (minisum):** the side with more (weighted) corroborating
  beliefs wins. This is the natural default for "consensus-leaning" reduction.
- **Arbitration / max (leximax):** minimise the worst-off source — fairer when
  one well-supported minority belief shouldn't be steamrolled by volume.
The `source_weight`/`ConfidenceModel` feeds the distance metric. So a `Reducer`'s
verdict strategy is a *choice of merging operator*, and — because it's pluggable —
another knob the benchmarks (§9) can race.

`HOLE §4a → V6` (RESOLVED): **edge is enough**, no distinct `Verdict` claim variant. The code
already ships it (`Adjudicates → Semantic::Defeat` + worlds-suppress + verdict-of-verdict
reinstatement); rationale + cross-world bindingness ride on the edge-belief's body + author
(human = non-regenerable anchor). A separate variant would duplicate the envelope the edge already
carries.

---

## 5. Worlds & git — parallel realities

A **world** is a *set of heads* (frontier of the DAG) — borrowed straight from
Automerge/Patchwork and isomorphic to a git ref + the ATMS notion of an
assumption-context.

```rust
pub struct World {
    pub name: String,          // "main", "experiment/spatial-lens", "alice-disagrees"
    pub heads: Vec<BeliefId>,  // frontier; everything reachable & not-superseded is "in"
}
```

Why this is the right backbone:

- **Experiments are branches.** "Reduce with strategy X" / "what if the user
  actually prefers Y" = a world. The user's entire goal (testing layouts) maps
  to *branching the belief DAG*, recomputing reductions per world, comparing.
- **Disagreement is a branch that may not merge.** A human's contradicting
  beliefs can live in their own world until reconciled.
- **Reduction is world-relative.** The same neighborhood reduces differently in
  different worlds because the frontier differs. L4 cache key includes the world.
- **Merging** = reconcile two frontiers; `attacks` across the merge invoke the
  reducer (or a human) to produce reconciling beliefs. This is the one genuinely
  hard algorithm — `HOLE §5a`.

Open questions:

- `HOLE §5a → V6` (RESOLVED): **real git + a thin custom belief merge driver; defer the driver.**
  Append-only + content-addressed ids ⇒ most merges are conflict-free file unions; real conflicts
  (two `supersedes` of one target across branches) are exactly the `attacks`/adjudicate case the
  reducer already handles, so the driver is a shim, not a bespoke DAG algorithm. (Pressure point is
  embeddings, not the `.md` DAG — per the duckdb spike.)
- `HOLE §5b → V6` (RESOLVED): **same DAG, marked derived.** Bound the cascade by
  **exclusion-by-construction, not a level cap** — the Reducer already skips its own `same-as` output
  so it can't re-enter, which is more robust than an arbitrary depth bound and keeps audit/reliving.

---

## 6. Recall — how an agent or human asks for things

Recall is L2 + L5. **One symmetric request type** for both humans and agents
(same as the contribution path). Recall never mutates; it resolves a
neighborhood and optionally reduces it.

```rust
pub struct Recall {
    pub cue: Cue,               // what you're asking about
    pub lens: Lens,             // which neighborhood strategy
    pub world: WorldRef,        // which reality (default: current)
    pub filters: Filters,       // epistemic / author / recency gates
    pub resolution: Resolution, // raw beliefs | reduced consensus | show-the-conflict
    pub as_of: AsOf,            // Now | Time(t) | Heads(set)  ← reliving
}

pub enum Cue {
    Semantic(String),          // embedding query
    Focal(BeliefId),           // "near this belief"
    Task(String),              // a task description to ground relevance
    Author(Author),            // "what does Alice believe"
}

pub enum Lens {                // pluggable neighborhood functions (L2)
    SemanticKnn { k: usize },
    Temporal { window: Duration },
    Spatial { radius: f32 },   // the "desk"
    GraphWalk { depth: u32 },  // follow justification edges
    Hybrid(Vec<Lens>),
}

pub enum Filters {
    /* min effective confidence, include/exclude derived, author allow/deny,
       recency window, directness floor … */
}

pub enum Resolution {
    Raw,                       // the beliefs themselves
    Reduced,                   // run L3, return consensus (+ its DerivedFrom)
    Conflict,                  // reduced + the surviving Attacks, surfaced
}

pub enum AsOf { Now, Time(Timestamp), Heads(Vec<BeliefId>) }
```

Design intent:

- **Lens is where "different neighbors in different contexts" lives** — and it's
  a *function*, never stored. Same belief participates in many overlapping
  neighborhoods depending on `(cue, lens, world)`.
- **`as_of` is reliving** — recall the belief state as it stood at a time or a
  head-set. Free from the immutable DAG.
- **`Resolution::Conflict`** is a first-class answer. Surfacing disagreement is a
  feature, not an error state.

---

## 6a. Recency is two axes — salience vs truth (and where frecency fits)

Tempting to borrow **frecency** (Mozilla awesomebar: frequency + recency) since
it's proven for human DX. It's useful here — but only once we notice that
"recency" is silently doing **two unrelated jobs**, and frecency only addresses
one:

- **Salience / findability (L2 recall):** among candidate-relevant beliefs, which
  surface first? This *is* the awesomebar problem.
- **Truth / staleness (L3 confidence):** how much do we trust a belief as
  currently correct? A newer assertion supersedes a stale one.

**Frecency is a salience tool. It belongs on the L2 axis and must NOT leak into
L3.** A belief retrieved constantly isn't more *true*, just more *central* —
letting access-frequency touch confidence lets popularity masquerade as truth,
the exact failure an epistemic store exists to prevent. The heuristic "good for
human DX → good here" holds for *findability* and is a category error for *truth*.

**The feedback loop human awesomebars don't punish.** In Firefox the frequency
signal reflects the human's genuine habits — an honest predictor. In an agent
memory, retrieval-frequency is **self-reinforcing**: what we surface gets read,
gets boosted, gets surfaced more (rich-get-richer). A belief retrieved once by
accident can ossify into permanently-hot while a relevant cold belief never
surfaces. The agent reading its own memory has no real-world reason to break the
loop, so naïve retrieval-frecency risks a filter bubble in the agent's own head.

**Better, memory-native cousins** (prefer these to browser frecency on L2):
- **Spaced-repetition decay** (Ebbinghaus / Anki SM-2) — the honest replacement
  for raw recency: strength decays, but each *spaced* reinforcement flattens the
  curve. A belief corroborated repeatedly over time should decay slower. It's a
  retention model, which is what we actually are.
- **Graph centrality** (PageRank over the justification graph) — a *structural*
  salience signal that is **not gameable by access patterns** (topology, not
  behavior). Nearly free given our edge graph; dodges the feedback loop. Reach for
  this before retrieval-frecency.

**The good kind of frequency is already on the right axis:** `observation_count`
(§3) — independent corroborations → more trust — is evidential frequency,
correctly placed in confidence. So "frecency" partly decomposes into something we
already do right.

Placement summary:

| Signal | L2 recall (salience) | L3 confidence (truth) | L4 cache (eviction) |
|---|---|---|---|
| retrieval-frecency | ⚠️ tempting, feedback-loop risk | ❌ category error | ✅ ideal |
| spaced-repetition decay | ✅ honest recency replacement | ✅ (staleness) | — |
| graph centrality | ✅ ungameable salience | — | — |
| corroboration count | — | ✅ already in `ConfidenceInputs` | — |

- **L4 cache eviction** is where frecency wins with no caveats — keeping hot
  reductions warm is a literal cache-replacement problem (LFU/LRU/frecency), no
  truth axis involved.
- `HOLE §6a → V2` (RESOLVED): the cold-belief benchmark was built and run (300-session simulation,
  corpus + 274-belief real store). **Verdict: do NOT put retrieval-frecency on L2 — it ossifies hard**
  (cold-surface rate 5–10%; one belief grabs ~20% of accesses; only 7–18% of beliefs ever surface).
  Keep cosine as the L2 base; use **PageRank** only as a *light* tie-breaking/diversity prior (it's
  ungameable and never ossifies, but a 0.3 blend already costs recall 1.00→0.83). Frecency belongs
  on **L4 cache eviction only**, exactly as this section predicted.

---

## 7. Incrementality (L4) — without a cluster graph

We drop the idea of a canonical clustering entirely. There is no stable cluster
to maintain.

- A reduction output records its **input manifest** = the belief-heads it
  consumed (= its `DerivedFrom` edges) + the `(world, context-sig, lens)` key.
- New belief `b` arrives → cheap index lookup finds cached reductions whose
  manifest *could* include `b` → mark **stale**.
- Recompute **lazily**, only when a context actually asks. No eager cascade; no
  oscillation from re-reducing things nobody reads.

Persistent state is only L0 + (reduction output + manifest). Neighborhoods stay
ephemeral. The dependency chain you worried about *is* the justification graph
read backwards — we don't maintain a second structure.

---

## 8. Language & how the type system encodes the layers

**Recommendation: Rust for the substrate (L0–L2, L4), reducer behind a trait.**

Why Rust fits unusually well here:

- **Immutability by construction.** Beliefs are `Arc<Belief>`, never `&mut`. The
  type system *enforces* that L0 is append-only.
- **Content addressing = `[u8; 32]`** + `serde` canonical serialization; the
  Merkle DAG is natural.
- **Enums nail the taxonomies** — `EdgeKind`, `Author`, `Claim`, `Lens`,
  `Resolution` are all closed sums the compiler checks exhaustively.
- **Layer boundaries become trait boundaries, and the dependency direction is
  encoded by what each trait can even see:**

```rust
pub trait Index            { /* built from &[Belief]; no write access */ }
pub trait Lens   { fn resolve(&self, cue: &Cue, w: &World, ix: &dyn Index) -> Neighborhood; }
pub trait Reducer { async fn reduce(&self, n: &Neighborhood) -> DerivedBelief; }
```

A `Reducer` only ever receives a `Neighborhood` — it *cannot* reach back into
the store or issue writes, so "no upward dependency" is a compile-time fact, not
a convention. Same for `Lens` only seeing an `Index`. The layering is the type
signatures.

**The one tension to be honest about:** L3 (the reducer) is LLM orchestration —
async HTTP, JSON wrangling, fast prompt iteration — where Python/TS ecosystems
are richer. Mitigation: keep `Reducer` a trait so during experimentation it can
be a subprocess/service in *any* language, while the correctness-critical
substrate stays in Rust. We get type-encoded layers without fighting Rust for
LLM glue.

> `HOLE §8a → V6` (RESOLVED): **service (curl-to-ollama).** The spike showed load is dominated by the
> 218 ms embeddings-JSON parse, not embed compute — in-process candle wouldn't move the measured
> bottleneck (a binary vector sidecar would). Keeps core dependency-light + shares the model with the
> LLM linker/reducer. Revisit only for a no-ollama deploy target.

---

## 9. Benchmarks & evaluation (the arch must stay open enough to measure)

You want this as an *experimentation* substrate, so evaluation is a first-class
requirement, not an afterthought. The branchable **world model is the A/B
harness**: run strategy A in world A and strategy B in world B over the *same*
beliefs, score both against gold verdicts. Every pluggable point (`Lens`,
`ConfidenceModel`, `Reducer`) is a knob; worlds are the experiment arms.

Benchmark dimensions we care about:

| # | Dimension | Metric | Depends on |
|---|---|---|---|
| B1 | **Recall quality** | precision / recall / nDCG vs labeled-relevant beliefs | `Lens` |
| B2 | **Reduction correctness** | accuracy of soft consensus vs adjudicated verdict (§4a) | `Reducer` |
| B3 | **Reduction stability** | semantic drift / variance across reruns on identical inputs; *monotonic* change when evidence added (no thrash) | `Reducer` |
| B4 | **Conflict handling** | does it *surface* injected contradictions (not silently pick) and resolve correctly once evidence arrives | edges + `Reducer` |
| B5 | **Confidence calibration** | ECE / Brier of effective weight vs correctness | `ConfidenceModel` |
| B6 | **Retraction soundness** | when a support is withdrawn, do dependent reductions update correctly (TMS consistency) | edges + L4 |
| B7 | **Staleness / temporal** | prefers current beliefs; reliving (`as_of`) reconstructs the right past state | provenance + L4 |
| B8 | **Cost** | tokens & latency per recall/reduction; L4 cache hit-rate | L4 |

Two foundational holes evaluation forces us to confront early:

- `HOLE §9a` **Dataset.** Benchmarks need labeled data: conversations with
  *evolving* facts, *injected* contradictions, and ground-truth *verdicts*. Likely
  synthesize a seed corpus (generate a persona, evolve their preferences over a
  timeline, plant contradictions, stamp verdicts). Without this, B2/B4/B5 are
  unmeasurable.
- `HOLE §9b → V4` (RESOLVED): **τ = mean pairwise-cosine agreement ≥ 0.85 across k reruns.** Measured:
  **qwen2.5:7b @ temp 0 = 0.997** (clears τ wide, ~2.1 s/call); temp 0 dominates default for both
  sizes. **Caveat that gates B2/B5:** stability ≠ trustworthiness — the same cell surfaced an injected
  contradiction only 1/6 times (silently adopted it 5/6). Fix before trusting: frontier-pre-filter the
  neighborhood (drop defeated beliefs) + a dedicated conflict pass.

### 9c. One path to measurability (cost-aware, solo-friendly)

This is **one** operational path, not the chosen evaluation — left deliberately
open because dev cost matters while soloing. The point is to show the dimensions
in §9 *can* be turned into real numbers cheaply, and to order them so the cheapest,
most decision-relevant signal comes first. Pick up tiers only as far as a question
actually needs.

**The trick that gives us ground truth at all:** generate the scenario so we
*authored* the truth. A scenario is a timeline of true-state transitions we define
→ an observation stream derived from it (with controllable noise / contradictions
/ stale-then-updated facts) → a query set with timestamps. Because we wrote the
true-state timeline, every query at time `T` has a *computable* gold answer. The
generator IS the oracle. Difficulty is a knob (inject N contradictions, M stale
updates).

```rust
struct Scenario {
    truth: Vec<(Timestamp, StatePatch)>,   // the authored ground-truth timeline
    stream: Vec<Observation>,              // derived from `truth` + injected noise/conflict
    queries: Vec<(Timestamp, Query, GoldAnswer)>, // gold computed from truth-as-of-T
    knobs: DifficultyKnobs,                // #contradictions, #stale-updates, noise rate
}
```

**Cost-tiered ladder** — stop wherever the current question is answered:

- **Tier 0 — free, NO dataset (do this day one).**
  - *Reducer stability* (B3, the gate): rerun `reduce(N)` k times on a *fixed*
    neighborhood, measure agreement across verdicts (exact-match if structured;
    embedding/entailment agreement if text). Gate downstream metrics on
    agreement ≥ τ — below it, every other number is noise.
  - *Monotonic sensibility*: adding a corroborating belief should ~never flip the
    verdict; measure the flip-rate (target ~0).
  - *Cost* (B8): tokens, latency, cache hit-rate — already free to log.
  - Needs no labels, no generator. If stability fails here, stop and fix L3.

- **Tier 1 — cheap, ONE tiny scenario.** Hand-author or generate a single persona:
  ~3 preference changes, 1 planted contradiction, 1 stale-then-updated fact, ~10
  timestamped queries. Gives a real efficacy number this week:
  - *Reduction correctness* (B2): feed the **oracle neighborhood** → accuracy of
    verdict vs `gold(T)`. (Isolates reasoning from retrieval.)
  - *Conflict handling* (B4): detection rate · resolution correctness · **silent-pick
    rate** (chose a side without surfacing the conflict).
  - *Staleness/reliving* (B7): query@T1.5→old, query@T3→new, `as_of(T1.5)`→old.

- **Tier 2 — more investment, when comparing strategies.**
  - *Recall quality* (B1): nDCG / recall@k of the lens vs the scenario's
    gold-relevant set, **oracle reducer downstream** (isolates retrieval).
  - *Calibration* (B5): bin beliefs by effective confidence → reliability diagram +
    **ECE / Brier** vs verdict labels. This is how the §3a `ConfidenceModel` race is
    *decided by a number*, not a guess.
  - *Comparative deltas via worlds*: run strategy A vs B on the **same fixed
    scenario** (the branch model is the A/B harness). Relative deltas are
    trustworthy even when absolute metrics are fuzzy — usually the most useful
    signal per dollar.

- **Tier 3 — optional, later, don't grade only our own homework.** Cross-check
  against external suites so the synthetic generator (which shares our assumptions)
  can't flatter us: LoCoMo, LongMemEval; belief-dynamics suites BeliefShift
  (BRA / Drift-Coherence / Contradiction-Resolution drop straight in) and STALE.
  *(Both belief-dynamics suites were found-but-unverified in pass v2 — confirm first.)*

**Three rules that keep this measurement, not automated vibing:**
1. **Isolate layers** (oracle the others) or a regression is unattributable —
   bad retrieval vs bad reasoning look identical end-to-end.
2. **Prefer structured gold** so verdict-equivalence is mechanical; when an LLM
   judge is unavoidable, spot-check it against human labels and report agreement.
3. **Lean on comparative deltas** over absolute scores while metrics are immature.

**North star (not a single number):** stability gate passed → reduction
correctness (B2) as headline → ECE to pick the confidence model → world-based A/B
deltas for every change. Tiers 0–1 are affordable solo; 2–3 are opt-in.

---

## 10. Build order (don't build top-down)

The two *risky* bets are L0's edge model and L3's reduction stability.
Everything else is deferrable engineering.

1. **MVP:** L0 (schema + edges, git-backed) → dumb L2 (`SemanticKnn`) → L3 →
   L5. **No L4 cache, no desk** — recompute every reduction. Goal: prove
   beliefs → reduction is *stable and trustworthy*, and that the git/world model
   feels right for experiments.
2. Add **L4 incrementality** once reduction earns caching.
3. Add **L2 lens experiments** (desk / temporal / graph-walk) — cheap because
   the substrate holds.

Don't build the cache or the fancy lenses until reduction earns them.

---

## 11. Open holes (rolling list)

Settled 2026-06-15 — full numbers in [open-questions-eval.md](open-questions-eval.md) (V1–V6).

- `HOLE §3` → **V6**: text-first `Claim`, **no** typed schema registry (0/779 beliefs use Triple).
- `HOLE §4` → **V5**: **add** `DependsOn` (endoxa is TMS-unsound without it); don't auto-promote
  `supports`.
- `HOLE §5a` → **V6**: real git + thin belief merge driver (defer the driver).
- `HOLE §5b` → **V6**: reductions in-DAG; cascade bounded by exclusion-by-construction, not a level cap.
- `HOLE §8a` → **V6**: embeddings via service (curl-to-ollama); bottleneck is JSON parse, not compute.
- `HOLE` reduction **stability** → **V4**: τ = pairwise-cosine ≥ 0.85; qwen2.5:7b @ temp 0 = 0.997.
  (Stable ≠ trustworthy: fix conflict-honesty before the reducer is on the trusted path.)
- `HOLE` confidence function → **V1**: recency-bearing StructuralOnly (recency non-negotiable),
  L1 placement, scalar driver + possibility/necessity affordance; don't rank on `asserted`.
- `HOLE` human contribution UX → **shipped**: the `mem` CLI (`remember`/`forget`/`promote`) is the
  human write path; a human belief is a peer with high `source_weight`, edges authored by the Linker.

**Still open (next experiments):** Linker edge-quality A/B vs the corpus's hand-placed gold edges
(the one big unmeasured thing); the conflict-honest reducer fix; proper isotonic calibration of the
confidence models; validating the §3b `blocked_on` edge once debt beliefs exist.

---

## 12. References — research pass v1 (2026-06-03)

Verification legend: **✓ verified** = independently confirmed 3-0 in the
adversarial pass; **~ found** = source located & fetched but its claims abstained
(a verifier-tooling glitch dropped many cluster 4–7 claims to abstain, *not*
refuted — confirm titles on first read); **✗ refuted** = a specific claim was
voted down (usually a detail like exact postulate numbering, not the paper's
existence).

### Cluster 1 — Truth Maintenance Systems  ✓ verified clean

- **✓** Doyle, *A Truth Maintenance System*, Artificial Intelligence 12(3):231–272,
  1979. DOI 10.1016/0004-3702(79)90008-0 ·
  PDF: https://cse.buffalo.edu/~rapaport/Papers/Papers.by.Others/NONMONOTONIC/doyle79.pdf
  — JTMS: justification edges, incremental retraction, ONE current in/out context,
  dependency-directed backtracking. Our justification-edge spine + retraction.
- **✓** de Kleer, *An Assumption-Based TMS*, Artificial Intelligence 28(2):127–162,
  1986. DOI 10.1016/0004-3702(86)90080-9 ·
  https://www.semanticscholar.org/paper/ed3f9263e936a879092ad7a2bf27e0f94089ccd8
  — **the parallel-realities machinery**: assumption-set *labels* = contexts =
  our worlds/branches; multiple consistent worlds at once; inconsistency tolerated;
  free context switching. This is the formal model behind §5.
- **✓** Reiter & de Kleer, *Foundations of Assumption-Based TMS*, AAAI-87 pp.183–188.
  https://aaai.org/Papers/AAAI/1987/AAAI87-033.pdf
  — prime-implicate label semantics (Horn Clause Mgmt System); minimal support
  *relative to the current clause set* = **branch-relative belief** (our L4 manifest).

### Cluster 2 — Belief revision  ✓ verified

- **✓** Alchourrón, Gärdenfors & Makinson, *On the Logic of Theory Change: Partial
  Meet Contraction and Revision Functions*, J. Symbolic Logic 50(2):510–530, 1985.
  DOI 10.2307/2274239 · https://projecteuclid.org/euclid.jsl/1183741857
  — partial-meet contraction + representation theorem = canonical retraction spec.
- **✓** Dixon & Foo, *Connections Between the ATMS and AGM Belief Revision*, IJCAI-93
  pp.534–539. https://www.researchgate.net/publication/2606235
  — **the bridge**: ATMS simulated inside AGM via epistemic entrenchment, proven
  correct. Justifies using edges as data + entrenchment as the retraction order.
- Stanford Encyclopedia, *Logic of Belief Revision* (secondary, good orientation):
  https://plato.stanford.edu/entries/logic-belief-revision/
- **✗ (detail)** Hansson belief-*base* vs belief-*set* and kernel contraction — the
  *distinction* (finite, non-deductively-closed bases are the implementable variant)
  is real and load-bearing for us, but the pass refuted a specific hierarchy claim.
  Re-source: IJCAI-15 paper https://www.ijcai.org/Proceedings/15/Papers/442.pdf

### Cluster 3 — Belief merging / fusion  ✓ verified (operators), ~ fusion frameworks

- **✓** Konieczny & Pino Pérez, *Merging Information Under Constraints: A Logical
  Framework*, J. Logic & Computation 12(5):773–808, 2002.
  https://academic.oup.com/logcom/article-abstract/12/5/773/1103580
  — IC merging; **majority (sum/minisum) vs arbitration (max/leximax)** operators =
  directly implementable verdict-selection menu (§4a).
- Stanford Encyclopedia, *Belief Merging* (secondary): https://plato.stanford.edu/entries/belief-merging/
- **✗ (detail)** Konieczny, Lang & Marquis, IC merging postulates IC0–IC8 — paper is
  real (Artificial Intelligence / EJOR), only the exact postulate enumeration was
  voted down: https://www.sciencedirect.com/science/article/abs/pii/S0377221703006702
- **~** Jøsang, *Subjective Logic* (fusion operators — cumulative/averaging/weighted/
  consensus&compromise/belief-constraint): book https://books.google.com/books/about/Subjective_Logic.html?id=nqRlDQAAQBAJ ·
  fusion paper https://www.mn.uio.no/ifi/english/people/aca/josang/publications/jwz2017-fusion.pdf ·
  arXiv:1805.01388 — *the survey flagged a fusion-taxonomy claim as unverified; treat
  as a framework to evaluate, not a settled result.*
- **~** Dempster–Shafer evidence combination — Computational Intelligence 28(4), 2012:
  https://onlinelibrary.wiley.com/doi/abs/10.1111/j.1467-8640.2012.00421.x

### Cluster 4 — Confidence / calibration  ~ found (verify titles on read)

- **~** Lin, Hilton & Evans, *Teaching Models to Express Their Uncertainty in Words*,
  arXiv:2205.14334 — verbalized confidence; relevant to whether to trust `asserted`.
- **~** LLM calibration survey, arXiv:2412.14737 (https://arxiv.org/html/2412.14737v2)
  — ECE/Brier, verbalized vs logit confidence.

### Cluster 5 — LLM agent memory  ~ found

- **~** Park et al., *Generative Agents: Interactive Simulacra of Human Behavior*,
  arXiv:2304.03442 — memory stream + reflection + recency×importance×relevance recall.
- **~** Memory-systems sources (confirm which is which on open — likely mem0 / A-MEM /
  Zep-Graphiti): arXiv:2501.13956 · arXiv:2504.19413 · arXiv:2502.12110.
  *Open question the pass did NOT settle: which keep real epistemic status vs a
  recency score. This is exactly our differentiator — worth a targeted read.*

### Cluster 6 — Temporal & provenance  ~ found

- **~** Snodgrass et al., TSQL2 / bitemporal SQL (valid time vs transaction time):
  https://www2.cs.arizona.edu/~rts/sql3.html
- **~** W3C *PROV-DM* (provenance data model): https://www.w3.org/TR/prov-dm/

### Cluster 7 — Knowledge fusion / truth discovery  ~ found

- **~** Dong et al., *Knowledge Vault: A Web-Scale Approach to Probabilistic Knowledge
  Fusion*, KDD 2014: https://www.cs.ubc.ca/~murphyk/papers/kv-kdd14.pdf
- **~** Dong et al., *Knowledge-Based Trust: Estimating Trustworthiness of Web
  Sources*, arXiv:1502.03519 — source-reliability estimation; prior art for the reducer.
- **~** Truth-discovery literature: https://dl.acm.org/doi/10.1145/2588555.2610509 ·
  https://dl.acm.org/doi/10.1145/2897350.2897352

### Synthesis — what to reuse

- **(a) world/branch-relative maintenance & retraction:** ATMS labels (de Kleer)
  for parallel worlds; Reiter–de Kleer minimal-support for branch-relative belief;
  AGM partial-meet + Dixon–Foo entrenchment encoding for the retraction order.
- **(b) selecting a single adjudicated verdict:** Konieczny–Pino Pérez IC merging —
  majority (sum) as default, arbitration (leximax) when minority beliefs are
  well-supported. Truth-discovery / Knowledge-Based-Trust for source-reliability
  weighting inside the distance metric.
- **(c) representing/propagating confidence:** AGM epistemic entrenchment (ordinal)
  as the principled core; Subjective Logic / D-S as richer (evaluate, don't assume);
  LLM-calibration work to decide how much to trust `asserted`.

**Where classic results do NOT transfer:** ATMS/AGM are *propositional and
logically closed* — they assume consistent boolean clauses and deductive closure.
Our beliefs are natural-language and the reducer is a stochastic LLM, so: (1) no
free logical entailment — "support"/"attack" are LLM judgments, not proofs;
(2) consistency is approximate, not guaranteed; (3) the elegant label algebra
needs an embedding/semantic notion of "same proposition" that the symbolic theory
just assumes. Reuse the *structures* (labels, entrenchment, merging operators),
not the *guarantees*.

---

## 13. References — research pass v2 (2026-06-03): the epistemic-status question

This pass targeted "who already does the epistemic layer." Same legend as §12
(✓ verified 3-0 · ~ found-but-unverified · ✗ specific-claim-refuted).

### 13A. LLM memory systems — does anyone keep real epistemic status?  ✓ verified

**Headline: nobody keeps numeric confidence; the field tops out at temporal edge
validity.** Our envelope (confidence + provenance + justification edges +
ATMS-style worlds) is genuinely unoccupied. The closest precedents (Zep, Mem0g)
do *bi-temporal edge-invalidation* — useful, but not confidence, not justification
edges, not parallel worlds.

| System (cite) | Confidence? | Provenance? | Contradiction handling |
|---|---|---|---|
| **Generative Agents** (Park 2023, arXiv:2304.03442) | ✗ — only LLM "importance" poignancy 1–10 | ✗ (timestamps only) | **None** — append-only; recency×importance×relevance recall |
| **MemGPT/Letta** (Packer 2023, arXiv:2310.08560) | ✗ | ✗ | None — OS-style tiered paging; orthogonal to belief |
| **Mem0** (Chhikara 2025, arXiv:2504.19413) | ✗ | partial | LLM ops ADD/UPDATE/DELETE/NOOP; conflict resolved by **recency** |
| **Mem0g** (graph variant, same) | ✗ | partial | explicit conflict detect → mark edge **INVALID** (soft-delete) |
| **A-MEM** (Xu 2025, arXiv:2502.12110) | ✗ | ✗ | None — links are **similarity-based**, not support/attack |
| **Zep / Graphiti** (Rasmussen 2025, arXiv:2501.13956) | ✗ | nascent | **bi-temporal** (valid t_valid/t_invalid + txn time); edge-invalidation on contradiction |

Takeaways folded into the design:
- **Generative Agents = the baseline of what NOT to do** (recency/importance/
  relevance, zero epistemic state). A-MEM confirms **semantic links ≠ justification
  edges** — a contrast case for why our edge taxonomy (§4) matters.
- **Mem0g + Zep validate two of our choices**: soft-delete (≈ our non-destructive
  `Supersedes`/defeat) and bi-temporal validity (≈ `valid_time` vs `txn_time`,
  §3, and `as_of` reliving, §6). We extend them with confidence, justification
  edges, and worlds.
- **`HOLE §13a → V3`** (RESOLVED): tested the cheaper option first — **it fails. ATMS world-labels
  are load-bearing.** Bi-temporal/global-frontier-only made the dissent-world answer *reachable* on
  **0/3** fixtures (a single timeline collapses every world to `main`); suppress-then-refixpoint made
  it **3/3**, and a qwen2.5:7b reducer fed the world's `assumption` flips its answer world-relatively
  3/3 (first reducer-behaves-world-relatively demo). Two things are needed: suppress makes the belief
  *reachable*, the `assumption` lets the reducer *select* it. Live analog: `scope` (`repo@branch`)
  flips canon beliefs in 3/16 real-store branches. Our §5 bet holds.

### 13B. LLM confidence calibration — can we trust an asserted float?  ✓ verified

**Verdict: no.** Verbalized/asserted confidence measurably *mismatches* the
model's underlying uncertainty.

- **✓** Geng et al., *A Survey of Confidence Estimation and Calibration in LLMs*,
  arXiv:2311.08298 — states the verbalized-vs-actual mismatch directly.
- **✓ (corroborating)** *On Verbalized Confidence Scores for LLMs*, arXiv:2412.14737;
  *Calibrating Verbal Uncertainty as a Linear Feature*, arXiv:2503.14477 (mismatch
  predicts hallucination); survey arXiv:2306.13063.
- **✗ (caution)** Lin, Hilton & Evans, *Teaching Models to Express Their Uncertainty
  in Words*, arXiv:2205.14334 — the specific "GPT-3 verbalizes *calibrated*
  confidence" sub-claims were **refuted/unverified** here. Do **not** cite it for
  "verbalized = calibrated." The verdict rests on the survey + 2024–25 papers.
- **~** BeliefShift (arXiv:2603.23848, Mar-2026 preprint, self-defined metrics):
  a **stability–adaptability trade-off** — no architecture both tracks legitimate
  revision *and* resists sycophantic drift; RAG improves *access* (revision +0.09,
  contradiction +0.07) but **not judgment/drift** (+0.02). Motivates **separating
  retrieval from adjudication** — i.e. our reducer/verdict step (§3, §4a) is doing
  work that retrieval alone provably does not. Single preprint; treat as suggestive.

### 13C. Truth discovery / knowledge fusion  ~ found (recognized canonical)

For the reducer's verdict mechanism — joint source-reliability + claim-truth:
- **~** Yin, Han & Yu, *Truth Discovery with Multiple Conflicting Information
  Providers on the Web* (TruthFinder), KDD-07:
  http://hanj.cs.illinois.edu/pdf/kdd07_xyin.pdf — the foundational iterate-to-
  fixpoint "trustworthy sources assert true facts; true facts asserted by
  trustworthy sources" loop. **Directly adaptable** as a reducer that co-estimates
  source weight and verdict.
- **~** Dong et al., *Knowledge Vault*, KDD-14: https://www.cs.ubc.ca/~murphyk/papers/kv-kdd14.pdf
- **~** Dong et al., *Knowledge-Based Trust*, arXiv:1502.03519 — source trust from
  fact correctness; prior art for `source_weight`.
- **~** Truth-discovery survey (Li et al.): https://dl.acm.org/doi/10.1145/2897350.2897352

### 13D. Provenance & temporal  ~ found (recognized canonical)

- **~** Green, Karvounarakis & Tannen, *Provenance Semirings*, PODS-07:
  https://web.cs.ucdavis.edu/~green/papers/pods07.pdf — **how-provenance** via
  semirings; a principled algebra for *derivation tracking* that maps onto our
  `DerivedFrom` edges and reduction lineage. The most reusable of the provenance set.
- **~** Buneman, Khanna & Tan, *Why and Where: A Characterization of Data
  Provenance*, ICDT-01: https://homepages.inf.ed.ac.uk/opb/papers/ICDT2001.pdf
- **~** W3C *PROV-DM*: https://www.w3.org/TR/prov-dm/ · Snodgrass *TSQL2* bitemporal:
  https://www2.cs.arizona.edu/~rts/sql3.html (Zep is the applied precedent).

### 13E. Fuzzy / many-valued / possibility  ~ found + preliminary verdict

- **~** Zadeh, *Fuzzy Sets*, Information and Control 8(3):338–353, 1965:
  https://www.marksmannet.com/RobertMarks/Classes/ENGR5358/Papers/Zadeh1965/ZadehPaper65.pdf
- **~** Dubois & Prade, *Possibility Theory* (encyclopedia article):
  https://www.irit.fr/publis/ADRIA/papersDDUBOIS/possibility-Encyclo.pdf

**Preliminary borrow verdict** (your framing — representation, never guarantee;
the pass didn't verify this cluster, so this is my read, flagged for a check):
- **Borrow:** (1) the **possibility/necessity pair** as a two-number confidence
  that encodes *ignorance* honestly — "nothing rules it out" (possibility) vs "the
  evidence forces it" (necessity) — which a single scalar can't. (2) **t-norms /
  t-conorms** as cheap, associative operators for combining graded support along
  justification edges (min/product for AND-of-support, max for OR). They're just
  monotone combinators — no probability axioms required, which fits "no bounds."
- **Don't borrow:** any claim that these degrees are *truth values* with logical
  guarantees, or fuzzy-control defuzzification machinery — it presumes a calibrated
  membership function we cannot have from a stochastic LLM. Use the shapes as
  bookkeeping, score them empirically (§9 B5), trust nothing a priori.

### Synthesis updates (v2)

1. **The gap is real and unoccupied** — confirmed. Build the envelope; the closest
   precedents only do bi-temporal edge-invalidation.
2. **Reducer verdict mechanism:** adapt **TruthFinder**'s co-estimation of
   source-trust + claim-truth (§13C) on top of the IC-merging operator menu (§4a).
3. **Derivation lineage:** model `DerivedFrom` with **provenance semirings** (§13D)
   — gives the cache manifest (§7) a principled algebra.
4. **Confidence:** asserted-float is measurably untrustworthy (§13B) → default to
   structural/blend; represent as possibility/necessity pair (§13E), score empirically.
```
