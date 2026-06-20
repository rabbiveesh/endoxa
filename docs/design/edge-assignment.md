# Edge assignment — the IRL mechanism (design notes)

Status: **exploratory notes** · 2026-06-07 · companion to `belief-memory.md` §4 (edges)
and §3 (`Claim::Triple`). Open questions flagged `Q`.

## The question that started this

Who actually assigns edges in the running system? An authoring agent shouldn't be
expected to *know the graph* — it sees its local task, not the whole memory. So the
**memory layer owns the graph**; the agent contributes propositions. This note works
out what that implies.

It also corrects an earlier position (belief-memory §4 draft): "born-with edges can
stay inline and inherit the host's provenance." That fails the test **"how do you
argue ONLY on that edge?"** — an inline edge is entangled with the proposition, so
you can't dispute the relation without disputing the claim.

## Principle 1 — author local, memory layer maintains global

The write path is **propositions in, graph maintained by the system**:

```
agent → CLI/MCP:  remember(claim, refs?, confidence_hint?, edge_hints?) → belief_id
```

- The agent submits a **proposition** + its grounding. It is *not* required (or
  trusted) to know how it relates to everything already stored.
- It MAY pass **edge hints** ("I think this replaces the old billing path"). Hints
  are low-trust *proposals*, not authority — see Principle 3.
- Mirrors human memory: you encode an experience; consolidation/linking happens
  later, offline ("sleep"). Same split as the Reducer (you don't compute consensus
  at perception time).

## Principle 2 — every *assertional* edge is its own belief

The dividing line is **proposition vs relational claim**, not born-with vs discovered:

- A belief asserts a **proposition** ("new billing bills usage as invoice items").
- Any **relational claim** it makes ("…and this *supersedes* the old approach") is a
  *separately falsifiable* assertion → its **own edge-belief**, even when the same
  author states both in the same breath. The proposition can be right while the
  relation is wrong (they might coexist, not supersede).

So an assertional edge is reified as a relational belief (`Claim::Triple` from §3),
carrying the **full envelope** for free — author, time, evidence, confidence — and is
itself defeasible (you can `attacks`/`supersedes`/`adjudicates` an edge-belief):

```
EdgeBelief = Belief { claim: Triple { subject: A, predicate: supersedes, object: B },
                      author: <linker or hinter>, provenance, confidence, ... }
```

**Exception — self-provenance stays inline.** `derived_from` (a reducer's "these are
my inputs") is the host's own composition, not a claim about two *other* beliefs;
you argue it by arguing the reduction. It stays attached. Everything assertional
(supersedes / attacks / supports / refines / adjudicates) reifies.

This is what answers "argue ONLY on that edge": mint a relational belief that
`attacks`/`adjudicates` the edge-belief; the endpoints A and B are untouched.

"Inline edges" therefore demote to **authoring sugar** — a convenient wire format the
backend normalizes into separate, provenanced edge-beliefs on ingestion.

## Principle 3 — the Linker: a consolidation service, sibling to the Reducer

Consolidation produces two kinds of derived belief:

| Service | Produces | Over |
|---|---|---|
| **Reducer** (L3) | *node* beliefs (consensus) | a neighborhood of corroborating beliefs |
| **Linker** (new) | *edge* beliefs (relations) | a new belief × its candidate-related neighborhood |

Both are background, agentic, and emit **provenanced, defeasible** beliefs.

The Linker, on a new belief landing:
1. retrieves candidate-related existing beliefs (semantic + structural neighborhood),
2. proposes edges (supersedes / attacks / supports / refines / corroborates),
3. materializes each as an **edge-belief whose author is the linker** — *not* the
   proposition's author. **This is the core reason edges need provenance: their
   author is usually not the belief's author.**

Properties:

- **Tiered linkers.** A cheap heuristic linker (embedding-kNN + rules) for obvious
  edges (corroboration, near-duplicate); an expensive LLM linker for subtle ones
  (supersedes/attacks need understanding); the **human** as a high-weight linker.
- **Edges get entrenchment for free.** Same triple asserted by several independent
  linkers = several corroborating edge-beliefs (observation-identity, per the id
  decision) → the edge is *entrenched* exactly like any belief. A fresh single-linker
  `attacks` is a low-confidence *proposal*; a human-confirmed, survived-many-commits
  edge is hard. Retrieval can filter by **edge entrenchment**.
- **Sync hints + async enrichment.** The author's edge *hints* are edge-beliefs
  available immediately at write; the Linker enriches / corroborates / **contests**
  them in the background. Eventual-consistency on the graph, but author-critical
  edges are never missing.
- **Edge revision is just belief revision.** A new belief can reveal that an old
  `supersedes` was wrong → the Linker mints a relational belief that defeats the old
  edge-belief. The graph is continuously reconciled, never "done."
- **Incremental.** Re-link only the new belief's neighborhood, not the whole store —
  same discipline as the reduction cache (§7).

## Consequences

- **The justification graph is now reified (a hypergraph).** Some nodes are
  object-beliefs, some are edge-beliefs (which themselves have edges). Traversal must
  handle "an edge is also a node." More power (attack an edge), more traversal cost —
  the price of independent arguability. `Q`: do we cap edge-on-edge depth?
- **An edge is "in force"** iff its source edge-belief is on the current frontier and
  undefeated (frontier-relative, like everything — composes with §4a defeat and the
  R4 intent-dependent frontier).
- **Identity:** edge-beliefs use observation-identity (same triple, different linker =
  two corroborating edge-beliefs), consistent with the standing id decision.

## The upgrade lever — edges as a regenerable derived layer (resolves Q1)

**Decision:** the agent **may hint** edges, but a **specific versioned Linker is the
sole author of machine edges.** A hint is recorded as *provenance pointing back to the
originating claim* and used as evidence; the Linker decides whether to commit it. So:

- The edge graph is a **derived projection over the immutable claim log**, not
  hand-placed permanent facts. The claims + hints are the durable source; edges are
  recomputed. (This is the "the layout is never the storage" principle, applied to
  edges.)
- **Improve the Linker → re-link.** A new Linker generation (v_{n+1}) mints edge-beliefs
  that supersede the old; the old (v_n) persist for reliving ("what did Linker-v3
  think?"). Non-destructive, frontier-relative, incremental (find stale-version edges
  via provenance, re-link lazily over their neighborhood).
- **Provenance must carry the agent version** (Linker-v3 / model) — that's what makes a
  generation findable and re-derivable.

**Generalizes past edges.** The Reducer (consensus *nodes*) and the Linker (relational
*edges*) are the same shape: *a versioned consolidation agent authoring regenerable
derived beliefs from the claim log.* "Upgrade the agent" is a general lever — re-reduce
and re-link both apply.

**The human is the exception (load-bearing).** A human-asserted edge (a verdict) cannot
be regenerated by a better agent — it's an **anchor**, not a derivation. So
*agent-Linker edges are regenerable; human edges are durable.* The re-link pass links
*around* human edges and never overwrites them.

**Immediacy is preserved** by a **fast Linker tier** that promotes the author's hints at
write time (still Linker-authored, still versioned, still regenerable), while a deep
tier enriches/contests async. The author *never* authors an edge — even instant ones
come from the Linker.

**The Linker is therefore benchmarkable.** Run v3 vs v4 over the same claims, score the
edge graphs against the entrenchment reference (the corpus's hand-placed edges *are*
that reference, with the no-gold caveats). "We improve the Linker" becomes a measurable
A/B, like the §3a ConfidenceModel race.

## Open questions

- `Q1` ✅ **Resolved** (above): agent hints; versioned Linker is sole author of machine
  edges; hint → provenance to originating claim; edges are a regenerable derived layer.
Q2–Q7 settled 2026-06-15 by the design-judgment pass (V6 in
[open-questions-eval.md](open-questions-eval.md)) — mostly *confirming what the code already does*:

- `Q2` ✅ **Hints sync, enrichment async.** Implemented: `SupersedeHintLinker` is Cheap/OnWrite
  (promotes the author hint to a `supersedes` edge at write, so currency is never missing);
  Proximity/Judgment run at Nrem/Rem. Freshness contract: a recall right after a write sees the
  hint-edges, not the deep edges.
- `Q3` ✅ **3 tiers, gated by stakes×budget; candidate-gen vs judgment split.** Cheap linkers
  (kNN + rules) *generate candidates*; the Mid `JudgmentLinker` (qwen2.5) only *judges* relations on
  them; escalate big judges only for high-stakes kinds (supersedes/attacks). Expensive tier reserved.
- `Q4` ✅ **Per-write incremental over the new belief's neighborhood; deep LLM pass batched at REM.**
  Mirrors the reduction-cache discipline; LLM cost stays off the write hot path.
- `Q5` ✅ **No hard edge-on-edge depth cap.** Rely on frontier-relative in-force (a defeated
  edge-belief drops out) + generic-edge subsumption (`is_generic()` hides `relates-to`/`analogous`
  when a specific edge exists). Real-store edge-on-edge depth ≈ 1; a cap is premature.
- `Q6` ✅ **`derived_from` stays inline.** It's self-provenance (a reducer's own inputs), not a claim
  about two *other* beliefs — you argue it by arguing the reduction. Confirmed.
- `Q7` ✅ **Normalize inline → edge-beliefs at ingestion, linker-authored.** The corpora encode 285
  inline edges in frontmatter; the live store already reifies. The eval/handoff harness must emit
  linker-authored edge-beliefs to match the live shape — this is what makes the corpus's hand-placed
  edges a valid **entrenchment reference** for the Linker A/B (the one big still-open experiment).
```
