# Open questions — empirical settlement

Status: **verdicts in** · 2026-06-15 · settles the `HOLE §N` markers in
[belief-memory.md](belief-memory.md) (and the open `Q` in [edge-assignment.md](edge-assignment.md))
by building every option and measuring it over BOTH the committed corpus (~442 gold-edged beliefs,
10 corpora) AND the user's real fact store (330 live beliefs). The branch model is the A/B harness.

This doc is the index of *what was asked, how it was decided, and the number that decided it*. Read
it before re-opening any HOLE.

## Classification of the open questions

| HOLE / Q | Question | Kind | How settled |
|---|---|---|---|
| §3a, §3-conf, §11 | which `ConfidenceModel` (Structural/Asserted/Blend/Bayesian); scalar vs poss-nec vs SL-triple; L1 vs L2 | **empirical** | EXP-CONF — ECE/Brier + verdict-pair ranking vs `defeated()` gold |
| §6a | which L2 salience signal (semantic/frecency/spaced-rep/PageRank); does frecency ossify | **empirical** | EXP-SALIENCE — recall@k + ossification simulation (Gini, cold-surface rate) |
| §13a, §5 | bi-temporal soft-delete ALONE vs explicit ATMS world-labels | **empirical** | EXP-WORLDS — can it reconstruct the dissent-world `reduction_fixtures` |
| §9b, B3 | define "stable enough"; is the qwen reducer stable enough to trust | **empirical** | EXP-STABILITY — k-rerun agreement τ + monotonic flip-rate |
| §4, B6 | need a distinct `DependsOn` assumption edge for sound retraction | **empirical** | EXP-RETRACT — TMS retraction soundness over corpus + synthetic chains |
| §3 | text-first `Claim` vs typed `Triple` schema registry | judgment | EXP-DESIGN |
| §3b | `revisit_when` free-text vs edge-to-constraint (auto-resurface) | judgment | EXP-DESIGN |
| §4a | `Adjudicates` edge vs distinct `Verdict` claim variant | judgment | EXP-DESIGN (code already ships the edge form) |
| §5a | merge semantics: real-git + driver vs custom DAG merge | judgment | EXP-DESIGN |
| §5b | reductions in-DAG vs side store; cascade bound | judgment | EXP-DESIGN |
| §8a | embeddings in-process (candle) vs service (ollama) | judgment | EXP-DESIGN (spikes already measured) |
| edge-assignment Q2–Q7 | Linker sync/async, tiers, cost, edge-on-edge depth, derived_from inline | judgment | EXP-DESIGN |

The five **empirical** experiments each run on their own scratch branch (`exp/<id>`), build all
options, and measure over corpus + real store. The **judgment** calls are reasoned verdicts that cite
the codebase and the spikes (no empirical race would move them).

## Method notes (so the numbers are trustworthy)

- **Gold = `defeated()`**, the frontier-relative fixpoint — never the naive incoming-defeating-edge
  list (wrong for verdict chains). Winner/loser pairs derived programmatically.
- **Isolate layers**: confidence/retraction evals are deterministic (no LLM); reducer evals oracle
  the neighborhood so retrieval noise can't be mistaken for reasoning noise.
- **Corpus vs real store differ**: the corpus has a rich envelope (asserted floats, source weights,
  dense gold edges); the real store is sparse (mostly `asserted: null`), so it tests currency-based
  signals, not asserted-confidence calibration. A verdict that holds on BOTH is the one to trust.
- **Comparative deltas over absolute scores** while metrics are immature.

## Verdicts

Settled by fan-out workflow `wf_1b2d0a19` (5 worktree experiments + design synthesis; ~937 s,
413k subagent tokens). Each experiment's code is committed on the branch noted. Numbers below are
measured, not asserted. Headline metric: **pair-acc** = fraction of verdict winner/loser pairs the
signal orders correctly; **cold-surface rate** = does a relevant-but-rarely-accessed belief still
surface after heavy use.

### V1 — Confidence model + representation + placement (§3a, §3-conf, §11) · branch `exp/conf` (wf-…-1) · *medium*

**Default `ConfidenceModel` = recency-bearing structural ranker, post-hoc calibrated onto a
source-weight/Bayesian prior.** Not pure-anything. The split is CALIBRATION vs RANKING and it
breaks by store:

| model | corpus pair-acc | corpus ECE | real pair-acc | real ECE |
|---|---|---|---|---|
| AssertedOnly (baseline) | 0.552 | 0.203 | **0.000** | 0.347 |
| **StructuralOnly** (w/ recency) | **0.931** | 0.584 | **0.917** | 0.804 |
| Structural − recency (ablation) | 0.897 | 0.563 | **0.167** | 0.733 |
| Blend(.7s+.3a) | 0.741 | 0.421 | 0.750 | 0.738 |
| BayesianUpdate | 0.879 | **0.232** | 0.167 | 0.346 |

- **Recency is non-negotiable.** The ablation collapses real-store pair-acc 0.917 → 0.167 — on a
  live store (zero corroboration, uniform source_weight) recency is the *only* thing separating a
  version-chain winner from its predecessor.
- **AssertedOnly is the worst ranker on real data (0.000)** — empirically confirms §13B "don't trust
  an LLM-asserted float." Keep `asserted` stored, don't rank on it.
- **Pure Bayesian calibrates best on the corpus but collapses on the real store** (no corroboration
  to update on). So: keep the structural ranker (scale-invariant, robust), and *if* you need a
  calibrated probability, fit isotonic/Platt onto a source-weight prior post-hoc.
- **Representation:** ship the **scalar** as the frontier driver, but carry a **possibility/necessity
  pair** (or SL triple) as a *contested-belief affordance* — possibility separates contested-vs-
  uncontested *current* beliefs by **+0.503** vs the scalar's **+0.105** (≈5×). Cheap, and it's the
  honest "this current belief is under live attack" signal a scalar can't encode.
- **Placement = L1 (per-belief), not L2.** L2 query-boost was a 3/3 tie on the fixtures but
  `cosine(defeated) > cosine(current)` in 2/3 neighborhoods — the stale belief is the *closer*
  semantic match, so an L2 proximity boost structurally points at the loser. **L2 must never be
  allowed to flip a frontier verdict.**

### V2 — L2 salience signal (§6a) · branch `exp/salience-l2` · *high*

**Keep cosine semantic relevance as the L2 base. Do NOT use retrieval-frecency as an L2 signal.**
The §6a feedback-loop hypothesis is confirmed with numbers — frecency catastrophically ossifies:

| store | signal | % beliefs ever seen (300 sessions) | cold-surface rate |
|---|---|---|---|
| helix | semantic | 100% | 100% (by construction) |
| helix | **frecency** | **18%** | **10%** |
| helix | pagerank | 100% | 100% |
| real-store (n=274) | **frecency** | **7.3%** | **5%** |
| real-store | pagerank | 25% | 100% |

- Frecency lets one belief grab ~20% of all accesses and **buries 90–95% of on-topic beliefs** behind
  whatever was hot early. Identical direction on corpus and the live 274-belief store.
- **PageRank** is ungameable (access-independent) and never ossifies (100% cold-surface) — but it's a
  relevance-blind prior, so even a 0.3 blend already costs recall (helix recall@5 1.00→0.83). Use it
  only as a **light tie-breaking/diversity prior**, not a co-equal ranker.
- **Spaced-rep**'s age prior actively hurts recall when relevant beliefs are old (helix 1.00→0.33).
- Net: **frecency belongs only on L4 cache eviction** (as §6a predicted); on L2 it's a filter bubble.

### V3 — Worlds: bi-temporal vs ATMS labels (§13a, §5) · branch `exp/worlds` (wf-…-3) · *high*

**ATMS world-labels are load-bearing — bi-temporal soft-delete alone cannot reconstruct a single
dissent world.** §13a's "test the cheaper option first" is answered: the cheaper option fails.

| | bi-temporal only | ATMS suppress-labels |
|---|---|---|
| dissent-world gold belief REACHABLE on frontier | **0/3** | **3/3** |
| L3 reducer produces the OWN-world answer (qwen2.5:7b, temp 0) | 0/3 | **3/3** |

- A single global frontier collapses every "world" to `main` — the dissent answer's belief is
  *defeated and unreachable*. Suppress-then-refixpoint reinstates it (3/3).
- **Bonus:** this is the first **reducer-behaves-world-relatively** demonstration (the corpus marked
  it NOT-YET-DEMONSTRATED). Fed the world's `assumption`, the qwen reducer flips its answer
  world-relatively in 3/3 dissent cases (verbatim text stable across two temp-0 reruns; the automated
  LLM judge was unreliable on shared-vocabulary gold pairs, so the evidence is the deterministic
  reachability table + the verbatim outputs).
- **Two things are required, not one:** the suppress-set makes the dissent belief *reachable*; the
  world's `assumption` is the *tie-breaker* the reducer needs to *select* it among co-live beliefs.
- **Real store:** `scope` (`repo:<id>@<branch>`) is the live analog of a world — **3 of 16 branch
  scopes actually flip canon beliefs** when layered in. Bi-temporal's one timeline can't express this.

### V4 — Reducer stability gate (§9b, B3) · branch `exp/stability` (wf-…-4) · *medium*

**Reducer = qwen2.5:7b @ temperature 0. Stability gate τ = mean pairwise-cosine agreement ≥ 0.85.**

| cell | mean agreement | ms/call | clears τ? |
|---|---|---|---|
| **qwen2.5:7b @ 0** | **0.997** | 2143 | YES (wide) |
| qwen2.5:3b @ 0 | 1.000 | 779 | YES (deterministic, lower fidelity) |
| qwen2.5:7b @ 0.8 | 0.956 | 2060 | YES |
| qwen2.5:3b @ 0.8 | 0.850 | 677 | borderline FAIL (worst 0.726) |

- **temp 0, full stop** — dominates default temp on stability for both sizes with no upside for a
  reducer. 7b over 3b for fidelity (3b's stability is partly determinism-by-poverty).
- **Stability ≠ trustworthiness.** The same cell FAILS conflict-honesty: on an injected contradiction
  it set `conflict:true` only **1/6** times and **silently adopted the contradicting claim 5/6**. This
  is the silent-pick failure B4 targets. Caveat: the experiment fed *raw* neighborhoods; the real
  pipeline pre-filters defeated beliefs via the frontier *before* reduction — so the fix is
  **frontier-pre-filter the neighborhood + a dedicated conflict pass**, and the reducer is *not* a
  safety net if the frontier mislabels. **Gate downstream B2/B5 on this being fixed.**
- Monotonicity: 2/6 borderline "flips" at cos 0.836/0.845 — likely paraphrase-drift measurement
  artifacts (an entailment judge, skipped for budget, would probably reclassify them as stable).

### V5 — Retraction soundness / `DependsOn` (§4, B6) · branch `exp/retract-dependson` · *medium*

**Add `DependsOn` as a distinct edge kind, but do NOT auto-promote sole-`supports` to it.**

- endoxa is confirmed **TMS-unsound**: retracting a belief's sole justification leaves it floating in
  **20/20** corpus cases (`supports` is corroboration, not justification — withdrawing a supporter
  doesn't retract the supported belief).
- A `DependsOn` JTMS variant (belief goes OUT when *all* its DependsOn targets are defeated) fixes
  **20/20**, cascades transitive chains correctly (2 cases, e.g. helix `ts-match-limit →
  ts-parse-timeout-500ms → ts-parser-thread-local`), and **never over-retracts** (multi-supported
  beliefs survive a single withdrawal).
- **But the cheap variant (treat sole-`supports` AS DependsOn) over-retracts 95%:** 19/20 sole-support
  dependents are `directness: stated` (independently grounded), so they must NOT die with their
  supporter. Only a *separately authored* `DependsOn` edge (from a Linker/judge) carries the real
  justification contract.
- **Real store: 0 `supports` edges → `DependsOn` is INERT today.** It's "add the edge kind now;
  load-bearing once linkers emit derived/assumption beliefs at scale." Optionally gate any future
  auto-promotion on `directness != stated`.

### V6 — Design-judgment holes (§3, §3b, §4a, §5a, §5b, §8a, edge Q2–Q7) · *high*

Read-only probes + code reading. The recurring axis is **structure-in-claims vs structure-in-edges**,
and the data is one-sided: **0 of 779 beliefs (corpus + real) use a `Triple` claim — 100% are text** —
while *all* epistemic structure already lives in a reified, defeasible, provenanced edge graph the
code implements (50 edge-beliefs live; a working `Semantic` registry; a tiered/cadenced Linker).

| Hole | Verdict | Grounding |
|---|---|---|
| §3 text vs typed Triple registry | **Text-first; no claim schema registry** | 0/779 Triple; the only structured shape that pays (the relation triple) already exists as `Relation` |
| §3b `revisit_when` free-text vs edge | **`blocked_on` edge (Annotate) to the constraint belief** | composes with `adjacency(&defeated)` to auto-resurface when the constraint is defeated; must NOT defeat (debt is true-and-live). Unvalidated (0 debt beliefs yet) |
| §4a Adjudicates edge vs Verdict variant | **Edge is enough** | code already ships it: `Adjudicates → Semantic::Defeat` + worlds-suppress; rationale rides the edge-belief body/author |
| §5a merge semantics | **Real git + thin belief merge driver; defer the driver** | append-only + content-addressed ids ⇒ most merges are conflict-free unions; real conflicts are the `attacks`/adjudicate case the reducer already handles |
| §5b reductions in-DAG vs side store | **In-DAG; bound cascade by exclusion-by-construction, not a level cap** | Reducer already skips its own `same-as` output, so it can't re-enter — more robust than a depth cap; keeps audit/reliving |
| §8a embeddings in-proc vs service | **Keep curl-to-ollama** | spike shows the load bottleneck is the 218 ms embeddings-JSON parse, not embed compute — candle wouldn't move it; a binary vector sidecar would. Keeps core dependency-light |
| eda Q2 sync/async | **Hints sync (OnWrite/Cheap), enrichment async** | already implemented (SupersedeHintLinker OnWrite; Proximity/Judgment Nrem/Rem) |
| eda Q3 tiers/escalation | **3 tiers; cheap=candidate-gen, mid-LLM=judgment; escalate only high-stakes kinds** | code: kNN+rules generate, qwen `JudgmentLinker` judges |
| eda Q4 cost | **Per-write incremental on the new belief's neighborhood; deep LLM batched at REM** | mirrors the reduction-cache discipline; LLM off the write hot path |
| eda Q5 edge-on-edge depth | **No hard cap; rely on frontier in-force + generic-edge subsumption** | `is_generic()` + undefeated-on-frontier prune naturally; real depth ≈ 1 |
| eda Q6 `derived_from` inline | **Inline — confirmed** | self-provenance, not a claim about two other beliefs; argue it by arguing the reduction |
| eda Q7 corpus bootstrapping | **Normalize inline → edge-beliefs at ingestion, linker-authored** | corpus has 285 inline edges; live store already reifies — the harness must match to make the Linker A/B reference valid |

## Round 2 (2026-06-15) — N8 + shipped-work validation (workflow `wf_e69a961e`)

After implementing V1–V6, a second workflow ran the **N8 Linker A/B** (the one big unmeasured claim)
and re-measured the *shipped* N1/N3 against their original verdicts. All four are *high* confidence.

### V7 — Linker edge-quality A/B (N8): the auto-Linker is a WEAK from-scratch edge proposer · branch `exp/n8` (wf-…-1)

**"Invest in the Linker" is NOT supported — and that validates the frontier-review design.** Run from
scratch over 3 edge-rich corpora (inline edges stripped), the Linker recovers almost none of the gold:

| kind | best F1 (arm) | gold | note |
|---|---|---|---|
| supersedes | 0.19 (gemma) / 0.11 (qwen) | 11–12 | only kind above noise |
| attacks | 0.15 (qwen) | 17 | gemma worse: 0 TP |
| adjudicates | **0.00** | 13 | the judge prompt has **no `adjudicates` option** — a code gap, never proposed |
| supports / refines | 0.00 | 17 / 1 | gemma floods 21 bogus `refines` |

- **Keep qwen2.5:7b as the judge; do NOT switch to gemma.** On the shared corpora gemma buys marginally
  higher supersedes recall (0.18 vs 0.09) at lower precision (0.20 vs 0.33), is ~2–3× slower, emits
  more junk `refines` (21 vs 8), and is **not safer on defeating kinds** — it produced *more* false
  defeating edges (10 vs 8). Of qwen's 17 proposed defeating edges only **3 are gold (14 false)** — a
  false defeating edge silently retracts a belief.
- **The dominant limit is STRUCTURAL, not the model.** The JudgmentLinker only proposes newer→older,
  but the corpora's gold edges mostly point older→newer or share identical txn_times → recall *ceiling*
  0.11 (helix) / 0.12 (sql-abstract) / 0.60 (composr). Two code gaps: the directional gate, and the
  missing `adjudicates` vocabulary. (Caveat: the txn_time clustering is partly a hand-authored-fixture
  artifact; a real append-only store gives distinct write-times, so the ceiling would bite less — but
  composr, with good temporal spread, still caps at F1 ≤ 0.29, so model quality is genuinely binding.)
- **This is the empirical case for the architecture we built.** The auto-Linker cannot be trusted to
  draw high-stakes *defeating* edges (P ≈ 0.18, mostly false). So those must be authored by the
  **frontier agent** (`mem review` → `mem link`) — exactly the path now shipped. The Linker's honest
  jobs are corroboration/relatedness (proximity) and the high-precision *author-hinted* supersedes,
  not autonomous verdict-drawing.

### Shipped-work validations (all confirmed the verdicts held in production code)

- **N3 (conflict pass) — CONFIRMED, strongly.** Over 7 real open-conflict neighborhoods the
  deterministic pass caught **7/7 (100%)** in 3 runs with **0 false-flags** (across 7 conflict + 45
  control neighborhoods); the LLM-only path named the conflict only **~19%** and silent-picked **~81%**
  (reproducing V4's 5/6). New finding: silent-pick is *worse* with realistic neighborhoods (3 cosine
  neighbors dilute the contradiction → 81%) than with the bare pair (57% named) — the deterministic
  pass is noise-invariant. The shipped N3 catches ~5.7 conflicts/run the LLM buries.
- **N1 (confidence boost) — CONFIRMED net-positive, zero regressions.** Over 29 gold-slug queries:
  recall@5 identical at 0.966 across cosine-only / shipped-confidence / old-supports-only (the 12% cap
  working — never moves the top-5), but shipped confidence lifts nDCG@5 0.899→0.912 and MRR 0.883→0.900,
  moving the gold rank in 1/29 (an improvement). The **old supports-only boost it replaced regressed
  2/29** (floated an older well-corroborated belief over the newer correct one) — validating the
  replacement. The cap is right.
- **`mem review` heuristic — NOT noisy; the opposite.** Only 3/337 corpus candidates (0.9%) and **0**
  on the real store — the directness gate (inferred/reduced subject) is very selective, so the queue is
  cheap to review and can't auto-corrupt the frontier. And the **deficiency axis is confirmed empty**
  (0 structured `forcing_constraint`/`revisit_when`/`severity` keys anywhere) → **Tier-2 onboarding is
  needed** to exercise §3b/N5 — now built and validated (see next-experiments.md N5).

## Round 3 (2026-08-19) — V8: the real-store audit + the Sweeper (frontier-agent session, live box)

The first analysis pass over the LIVE store (588 files: 488 content + 100 edge-beliefs) since the
verdicts. Three findings, one refutation, one shipped organ. Analysis code: throwaway crate
(not committed); all numbers below are aggregates over the private store.

### V8a — The store's dominant failure mode is SILENT rot, not marked-conflict resolution
- **484 unlinked near-duplicate pairs** (cos ≥ 0.80, both current, distinct txn_times) among 474
  current content beliefs — vs **13 marked supersedes edges in the entire store**.
- Judging the top 60 by similarity (strict claude:sonnet proposer, "default INDEPENDENT"):
  **35 proposed stale → 30 survived a hostile refuter (86% precision)**. The refuter was told
  retracting a true belief destroys information and killed 5 real over-reaches.
- So the frontier was operating on ~2.9% of the store (14 defeated) while ~2.3× more verified
  staleness sat undefeated in just the top-60 candidates. **The bottleneck is not `defeated()` —
  it's that nothing ever LOOKED for supersession between beliefs from different sessions.**

### V8b — "Relevance is anti-correlated with currency" is REFUTED on the real store
The corpus finding (`cosine(defeated) > cosine(current)` in 2/3 conflicted neighborhoods) does
not transfer: on all 13 real supersession pairs, the WINNER ranks #1 by cosine and the loser
never outranks it (**0/13 inversions**; loser in naive top-5 12/13). Real re-observations are
phrased like their successors, and newer text wins ties. Cross-cutting lesson 3 should be
scoped to hand-authored corpora until re-measured.

### V8c — The frontier's read-path value is real but SMALL on today's store (n=13, controlled)
A controlled A/B (fixed question set, deterministic cosine grading, qwen2.5:7b @ 0, k ∈ {5,8,12}):
frontier-filtering the injected context reduces stale answers in **6/6 arm comparisons, but only
by 1–2 of 13**, mostly converting confidently-stale into unclear. Two method cautions, learned
the hard way: (1) regenerating questions per run + LLM grading produced swings (8/2 → 5/6 on the
SAME arm) larger than the effect — V4's dramatic silent-pick numbers deserve the same controls;
(2) frontier-filtering with backfill admits a new belief the naive arm never saw — a confound.

### V8 verdict → the Sweeper (`mem sweep`, shipped this round)
V7 ("the auto-Linker is weak, invest in review/link") is **task-shape- and model-bound, not
fundamental**: constrain the judgment to "near-duplicate pair, one newer — does the older now
mislead?", use a frontier-grade judge (provider seam: `claude:sonnet`), and add an adversarial
refuter, and edge quality jumps from F1 ≤ 0.19 to 86% precision. `mem sweep`:
- deterministic candidate gen (cos ≥ 0.80, newer→older, unlinked, both current, scope-filtered);
- strict proposer + hostile refuter — only double-verified pairs become `supersedes` (newer→older,
  author `sweep@1`, Confidence::Strong, rationale carries both whys);
- **history is never lost**: ordinary defeasible edge-beliefs via the Consolidator; one
  `mem forget <edge-slug>` reinstates; the verdict LEDGER (`.sweep-ledger.json`) makes the veto
  durable (a judged pair is never re-judged → never re-drawn) and makes `--dry-run` verdicts
  replayable for free; `--limit` caps LLM spend per run.
- **Live result (endoxa+global scope): 59 pairs judged → 10 confirmed (edges drawn), 47
  independent, 2 saved by the refuter, 0 errors.** Recall now reports the dropped stale side;
  supersession chains up to depth 3 emerged across runs; the candidate backlog shrinks
  superlinearly as defeats remove pairs. Sweeping other scopes = run `mem sweep` in those repos.
- `sweep@1` is deliberately NOT a regenerable-edge author, so human vetoes stick.
- **Cost model ("isn't a sweep quadratic?"): quadratic only in the cheap part, once.** Candidate
  gen is n²/2 dot products of CPU float math — measured ~0.1 s at n≈500; ~1.5 s projected at
  n=2k; the knee (~10k, ~40 s scalar) is the SAME knee where storage-backends.md already hands
  vectors to ANN/DuckDB+vss, so sweep inherits that scale path. The LLM part is NOT quadratic:
  only ~0.4% of pairs clear cos ≥ 0.80 on real data (484/112k), each judged ONCE ever (ledger),
  and a **scan watermark** (immutable beliefs + content-keyed embeddings ⇒ a pair wholly older
  than the last completed scan can never become new) makes steady-state enumeration O(new × n).
  Measured steady state on the live store: **66 ms, zero LLM calls.** And unlike `dream`, sweep
  judges via the provider seam (API), not local ollama inference — the backfill was the one-time
  payment for months of rot; per-write marginal cost is the new belief's 0–3 near-duplicate pairs.

Open, in order: (1) sweep the remaining scopes from their repos; (2) an independent-model refuter
(proposer and refuter currently share sonnet's blind spots); (3) `same-as`-aware sweeping (some
"independent" pairs are duplicates wanting `mem reduce`, not supersession); (4) re-run V4 under
V8c's controls.

## Cross-cutting lessons

1. **The corpus and the real store stress different things and you need both.** The corpus has a rich
   envelope (asserted floats, dense gold edges) and rewards corroboration; the real store is sparse
   (0 supports edges, uniform source_weight, mostly `asserted: null`) and is carried almost entirely
   by **recency + supersession chains**. Every verdict that survived both (recency-bearing structural;
   no-frecency; world-labels) is the one to trust; a corpus-only winner (pure Bayesian) was a trap.
2. **Stability and trustworthiness are orthogonal.** A reducer can be perfectly reproducible *and*
   reproducibly wrong (V4): 0.997 agreement, 1/6 conflict-honesty. Never read low variance as correct.
3. **Relevance is anti-correlated with currency** (V1 + V3 + the existing eval-qa keystone):
   `cosine(defeated) > cosine(current)` in most conflicted neighborhoods. Any signal that boosts on
   raw proximity (L2 frecency, L2 query-boost) structurally points at the stale loser — the frontier
   must win over similarity, always.
4. **Push structure into edges, not claims.** 0/779 typed claims; all the leverage is the reified,
   defeasible, frontier-composing edge graph. The open frontier is **Linker edge-quality** (built,
   not yet A/B-scored against the corpus's hand-placed gold edges).

## What's still open (next experiments)

The concrete, ordered action queue — implement the locked-in winners (N1–N4), close the validated
gaps (N5–N7), run the one big missing experiment (N8 Linker A/B) — lives in
**[next-experiments.md](next-experiments.md)**. The four loose ends in brief:

- **Linker A/B** against the corpus's hand-placed gold edges (the entrenchment reference) — the one
  big unmeasured thing; "invest in the Linker" is a direction, not yet a proven win. → N8.
- **Conflict-honest reducer** (V4 fix): frontier-pre-filter + a dedicated conflict pass, then re-run
  the B4 silent-pick metric. → N3.
- **Calibration done right** (V1 caveat): isotonic/Platt on each model on a common scale before
  re-reading ECE/Brier — pair-acc verdicts are scale-invariant and stand, but "Bayesian calibrates
  best" is soft until then. → N7.
- **Deficiency axis (§3b)** stays unvalidated until onboarding Tier 2 harvests debt beliefs to
  exercise `blocked_on`. → N5.
