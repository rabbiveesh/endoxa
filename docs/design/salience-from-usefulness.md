# Salience from usefulness feedback — learn the L2 ranking blend from downstream outcomes

Status: **parked exploration** · 2026-06-22 · provenance: prompted by Wenhan Zhou's essay
*"A Bitter Lesson for Memory"* (2026-06-21) — see [References](#references). One-line verdict:
**steal *usefulness feedback* as an L2 salience signal; refuse it as an L0 currency signal.** Not
scheduled yet — this doc queues the idea and its guardrails so we can return to it.

> **Revisit trigger:** build this once we have *task-outcome traces* to learn from — i.e. after the
> live `mem`/`ask` loop is instrumented to log `(query, world, surfaced-heads, outcome)`, or when a
> coding-agent integration gives us a dense verifiable reward (commit landed / tests passed). Until
> then it is untestable machinery, like N5 was before Tier-2 onboarding. **Measure before modeling.**

---

## The essay, and why it's the sharpest bear case against us

Zhou applies Sutton's Bitter Lesson to AI memory: every handcrafted memory pipeline (knowledge
graphs, summarization, `MEMORY.md`, RAG, hybrid search) encodes a *human-written* retention policy
("be concise", chunk size, schema) that is frozen and can't adapt to outcomes it never saw. The
proposed fix is RL: give the model a multi-turn task, let it compact, reward only the final outcome,
and let the **compression policy emerge from downstream task success**. Two falsifiable predictions:
latent vectors beat text as the compression medium, and APIs grow a non-text "opaque blob" slot
(the OpenAI/xAI `encrypted_content` compaction endpoints).

Read plainly, **endoxa is exactly the handcrafted architecture the essay says will plateau** —
hand-authored edge taxonomy, deterministic frontier resolver, heuristic/LLM linkers. So we treat
this essay as the strongest public statement of the case *against* our approach, and we mine it for
the one idea worth stealing rather than arguing with the thesis head-on (a fight the Bitter Lesson
wins rhetorically).

The idea worth stealing: **the signal for what to surface should come from downstream task utility,
not from topical similarity or a hand-tuned blend.** The essay's own line — *"RAG retrieves by
topical similarity, not task utility… tuned by a human (chunk size, overlap, `alpha`), not by
whether the retrieved context helped the agent succeed"* — is a direct indictment of any
similarity-only L2 lens.

## What we steal, and the one guardrail that makes it endoxa-shaped

**Usefulness is a ranking signal, never a currency signal.** The essay can blur "what to keep" with
"what's useful" because it has no frontier — only keep/drop. We have `defeated()`. A belief can be
*defeated* and still highly useful to surface (the famous-wrong-answer you retrieve precisely to warn
against it). So the placement is hard:

| Layer | Usefulness feedback | Why |
|---|---|---|
| **L2 neighborhood / recall ranking** | ✅ legitimate new signal | which beliefs the lens surfaces, and in what order |
| **L0 / `defeated()` frontier** | ❌ must never touch it | truth stays adjudicated by defeat edges, not voted up by utility |

Collapse the two and we rebuild the exact non-monotonic-degradation pathology the essay dunks on
(popular junk promoted above threshold) — inside the one system designed to resist it. Usefulness
biases salience *downstream of* `defeated()`, never inside it. This composes with the locked-in N2
verdict (no frecency in L2; cosine + optional light PageRank prior) — usefulness is a **utility**
signal distinct from access-frequency frecency, and must clear the same "doesn't ossify" bar.

## Why endoxa does this *better* than RAG or the latent blob

This is the part worth being excited about, and it's the steelman for "RL and provenance are
complementary, not rival":

1. **Usefulness is just another reified, defeasible, provenance-bearing edge.** In RAG it's opaque
   reranker weights; in the essay's world it's baked into a latent blob that "tells you *what* was
   kept, not *why*." In endoxa it's an edge — *"on 2026-06-22, surfacing belief X for query-class Q
   correlated with task success"* — authored by the memory (Consolidator is the sole edge writer),
   append-only, auditable, and **itself defeasible** when the correlation turns out spurious. That is
   the auditable version of RLVR-for-memory, and it directly answers the essay's "the blob can't tell
   you why" critique.
2. **Credit assignment — the problem the essay calls *unsolved* — is what our justification DAG is
   for.** The essay: connecting a failure five turns after compaction back to what was kept/dropped is
   the open problem; "works best where the horizon is short." If X `supports` Y and Y was useful, X
   inherits partial credit *along the edge*. The graph gives a structured propagation path the latent
   vector doesn't have. This is the genuine research bite.
3. **We adopt the Bitter Lesson at exactly one layer** — learn the salience *blend* (embedding-sim vs.
   recency vs. edge-centrality vs. utility) from outcomes instead of hand-tuning it — while the
   substrate stays handcrafted and auditable. That is literally AlexNet's "human-designed
   architecture, learned features."

## MVP (when the revisit trigger fires)

1. **Logging first.** Emit `(query, world, context-sig, surfaced-heads, outcome)` from the recall
   path, keyed on the **L4 reduction-cache key** `(world, context-sig, input-heads)` — the join key
   already exists. *You can't model what you don't measure.*
2. **Reify usefulness** as a new `EdgeKind` (`useful_for` / `helped`, registered in the `Semantic`
   registry, **Annotate semantics — must NOT defeat**, like `blocked_on`), written by the
   Consolidator.
3. **Blend into ranking.** Let recall consume an entrenchment + utility blend (utility as a *bias* on
   top of the N1 `StructuralOnly` prior, not a replacement); weights hand-tuned at first, learned once
   traces exist. A/B in `recall.rs` against the current similarity ranking. Strictly downstream of
   `defeated()`.

## Traps (ordered by how much they'll hurt)

1. **No traces yet → untestable.** Corpora carry gold *edges* and worlds, not task-success labels;
   the live store has currency labels, not usefulness labels. Data collection is step one, modeling is
   step two. (Same shape as the N5 blocker: don't build the lens before the inputs exist.)
2. **"Recalled" ≠ "used."** Rewarding mere co-occurrence is the classic failure; you must capture
   whether the surfaced belief actually influenced the action (cited in output / preceded a passing
   commit). That attribution is the real engineering cost. `mem` runs per-invocation in a repo and
   sees local git state → the coding-agent loop is the dense-reward regime the essay says works
   (compiles? tests pass?). **Start there, not in fuzzy conversational recall.**
3. **Rich-get-richer.** Useful → surfaced more → more usefulness signal → entrench, starving
   rare-but-critical beliefs (the same ossification N2 killed frecency for). Counterweight already
   owned: `dream` (the novelty bridge). Wire usefulness and novelty as **opposing forces**, not one
   signal.
4. **Cold start / sparsity.** Most beliefs never get a signal; fall back to the N1 similarity +
   entrenchment prior (incoming-`supports` count). Utility is a bias, not a replacement.

## Open questions

- Is `useful_for` one edge kind or two (positive utility vs. anti-utility / "surfaced and misled")?
- Does utility attach to a belief, or to a `(belief, query-class)` pair? The latter is more honest
  (a belief useful for Q is noise for Q′) but needs a query-class clustering layer.
- Credit propagation depth along `supports`/`depends_on`: one hop, decayed-k-hop, or full JTMS-style?
- Relationship to N7 calibration — does a *learned* salience weight need a calibrated number, or stay
  scale-invariant like recall ranking does today?

## References

- Zhou, Wenhan. (2026). *"A Bitter Lesson for Memory."*
  <https://personal-website-3bed.onrender.com/blog-viewer.html?slug=A%20Bitter%20Lesson%20for%20Memory>
  (2026-06-21). The prompt for this doc; the source of the usefulness-feedback idea and the
  latent-vs-text / opaque-compaction-API predictions. Companion post worth reading:
  *"Regenerative Engineering"* (2026-03-31, same author) — grow-via-patches → complexity ceiling →
  regenerate-via-compression-preserving-trust, which rhymes with our `consolidate` story.
- Sutton, R. (2019). *"The Bitter Lesson."* <http://www.incompleteideas.net/IncIdeas/BitterLesson.html>
- Zhang, D., Lin, Y., Wu, Z., et al. (2026). *"Useful Memories Become Faulty When Continuously Updated
  by LLMs."* arXiv:2605.12978 — the non-monotonic-degradation result the essay leans on, and endoxa's
  founding motivation (our answer: append-only defeat, never destructive rewrite).

## See also

- [next-experiments.md](next-experiments.md) — N1 (StructuralOnly ranking prior this biases on top
  of), N2 (no-frecency-in-L2 verdict this must respect), N5 (`blocked_on` Annotate-edge precedent).
- [energy-and-argumentation.md](energy-and-argumentation.md) — the other parked exploration; a
  sparse-Hopfield L2 retrieval head is an orthogonal lever on the same recall layer.
