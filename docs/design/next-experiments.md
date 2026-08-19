# Next steps — the action queue after the 2026-06-15 verdicts

Status: **active backlog** · 2026-06-15 · derived from the measured verdicts in
[open-questions-eval.md](open-questions-eval.md) (V1–V6). This is the *concrete, ordered* work the
experiments now force — implement the locked-in winners, close the validated gaps, run the one big
experiment still missing. Higher-level direction lives in [../ROADMAP.md](../ROADMAP.md); where an
item there overlaps, it's cross-linked.

Ordering principle: **leverage × forced-by-evidence**. Do the things the data already decided before
the things the data only suggested. Each item states *why* (the verdict), the *first concrete step*,
rough *effort*, and a *done-when* gate.

> **Implementation status (2026-06-15, branch `feat/verdict-winners`):** P0 is **done** — N1
> `a4e1044`→`08cabac` (StructuralConfidence + recall wiring), N2 (standing no-op decision), N3
> `17c4f67` (deterministic conflict pass), N4 `a4e1044` (DependsOn JTMS edge). All on green tests;
> `defeated()` is byte-identical on the corpus (additive/inert by construction). **P1/P2 are
> blocked on decisions or data** — see "Blockers / open questions" at the bottom.

---

## P0 — Locked-in winners to implement (the data decided; just build it)

### ✅ N1. Confidence as a `ConfidenceModel` trait, default = recency-bearing `StructuralOnly` (V1)
- **Why:** StructuralOnly is the best ranker on both stores (pair-acc 0.93/0.92); recency is
  non-negotiable (ablation → 0.17 on real); `asserted` must not drive ranking.
- **First step:** lift the experiment's `StructuralOnly` (directness × source_weight × (1+log1p(corrob))
  × recency_decay(txn_time), corrob = incoming live `supports`) from branch `exp/conf` into
  `memory-core` as a `ConfidenceModel` trait with the impl as default; wire it where recall currently
  uses the ad-hoc `support_boost` in `memory-cli`.
- **Effort:** S–M. **Done when:** recall ranking uses the trait; `eval`-style pair-acc regression
  pinned ≥ 0.90 on the corpus.
- **Carry, don't drive:** store a possibility/necessity pair as a *contested-belief affordance* (the
  recall `⚠ contested` line already exists — back it with `necessity`/`possibility`, not just a flag).

### ✅ N2. Kill any L2 frecency temptation; keep cosine + optional light PageRank prior (V2)
- **Why:** frecency ossifies (cold-surface 5–10%). PageRank is ungameable but must stay a *light* prior.
- **First step:** confirm no access-frequency signal leaks into L2 ranking (it doesn't today — good);
  if/when a diversity prior is wanted, add PageRank-over-the-support-graph at blend ≤ 0.15, behind a
  config flag, **off by default** (a 0.3 blend already cost recall 1.00→0.83).
- **Effort:** S. **Done when:** documented as a closed decision; frecency reserved for L4 cache eviction.

### ✅ N3. Frontier pre-filter on the reduction path + a conflict pass (V4) — *gates the trusted reducer*
- **Why:** qwen2.5:7b @ temp 0 is stable (0.997) but silently adopts contradictions 5/6. The reducer
  is only safe if it never sees the defeated loser **and** flags genuine live conflict.
- **First step:** ensure `mem ask` / any L3 reducer feeds `current_content(&defeated)` (frontier-
  filtered), never raw top-k; add a dedicated conflict-detection pass over surviving `attacks` pairs
  *before* synthesis (don't rely on the LLM to volunteer `conflict:true`).
- **Effort:** M. **Done when:** re-run the V4 silent-pick metric with pre-filtering → conflict surfaced
  on injected contradictions ≥ 5/6; reducer answers never echo a defeated belief.
- Pin **reducer = qwen2.5:7b @ temperature 0** as the standing default (it isn't, everywhere).

### ✅ N4. Add the `DependsOn` edge kind (V5) — add now, inert until linkers emit it
- **Why:** endoxa is TMS-unsound for sole-`supports` (20/20). `DependsOn` (OUT when *all* targets
  defeated) fixes it; but `supports` ≠ justification, so **no auto-promotion** (would over-retract 95%).
- **First step:** add `EdgeKind::DependsOn` with JTMS defeat-propagation in `Graph::defeated` (the
  semantics are on branch `exp/retract-dependson`); register it (Defeat-on-all-targets-gone, distinct
  from the existing any-source-defeats kinds). Do **not** synthesize it from `supports`.
- **Effort:** M. **Done when:** the constructed axiom→lemma→theorem chain retracts correctly; the
  corpus regression shows zero behavior change (no DependsOn edges exist yet → inert, as intended).

---

## P1 — Validated-gap closers (small, the verdict named the exact fix)

### ✅ N5. `blocked_on` deficiency edge (V6 / §3b) — DONE (2026-06-15)
> **Shipped.** The deficiency axis is complete end-to-end: `Belief.deficiency` (severity/
> forcing_constraint/revisit_when) parses; **Tier-2 onboarding** (`mem onboard --tier2`, model via
> `TIER2_MODEL`) extracts deficiency structure from debt leads (verified live on endoxa's history
> with gemma2:9b); **`mem debt [<query>]`** is the known-debt query; and a `blocked_on` edge (authored
> via `mem link … blocked_on …`) **auto-resurfaces** the debt with `⚠ RESURFACED` when its forcing-
> constraint belief is later defeated. Verified end-to-end. The original entry follows.
- **Why:** the deficiency axis needs auto-resurface; an `Annotate` `blocked_on` edge to the constraint
  belief composes with `adjacency(&defeated)` for free.
- **First step:** add the edge kind (Annotate semantics — must NOT defeat); a recall lens that walks
  incoming `blocked_on` and re-raises a debt when its constraint belief gets defeated.
- **Effort:** S, but **gated**: 0 deficiency beliefs exist in either store. Pair with onboarding
  **Tier 2** (ROADMAP "Onboarding tiers 2–3" → kludge → deficiency beliefs) to generate the inputs.
- **Done when:** a harvested debt belief auto-resurfaces in recall after its forcing-constraint belief
  is superseded.

### N6. World `assumption` reaches the reducer (V3) — ✅ BUILT (2026-08-02); LLM gate pending one ollama run
> **Shipped end-to-end** (the "most ambitious thing" pass): the design fork was resolved by making
> the live surface the corpus format, verbatim — a `worlds.json` in (or beside) the store, so a
> corpus dir already IS a store. Core grew the worlds machinery (`World`,
> `defeated_with`/`defeated_in` = suppress-then-refixpoint, `frontier_flips`), the CLI grew
> **`mem world [list|show|diff]`**, **`mem relive <as-of-time>`** (bitemporal replay, the ROADMAP
> "worlds and reliving" surface), and **`mem ask --world <w>`** — which does BOTH V3 halves:
> world-frontier recompute (reachable) + assumption threaded into the reducer prompt (selectable).
> **`eval-worlds`** re-proves the V3 reachability table deterministically over every worlds.json
> corpus (helix + composr: 15 checks, 0 failures, in CI reach — no LLM), and its `--llm` mode is
> the graduation gate below. Branch scopes stay untouched as the other live world-analog.
> **GRADUATED (2026-08-03): the `--llm` pass is recorded — 6/6 fixture×world cells select their
> own world's gold answer.** Run: `ASK_MODEL=claude:sonnet cargo run -p memory-cli --bin
> eval-worlds -- --llm` (composr 2 fixtures × 2 worlds + helix 1 × 2; deterministic substrate
> 15/15 alongside). Reducer = Claude Sonnet via the new pluggable chat-provider seam (the
> Claude Code CLI backend); grading = the *blind* letter-labeled LLM judge fallback (ollama was
> absent, so embedding-proximity grading couldn't run — the printed verbatim answers are the
> primary evidence, same standard V3 used). Caveats, recorded not hidden: (1) this is not the
> pinned qwen2.5:7b@0 reducer — the CLI exposes no temperature; an ollama re-run
> (`eval-worlds --llm` with embeddings up) would make the demo model-diverse and
> embedding-graded; (2) LLM-judge grading was V3's weak spot with qwen — here the judge is
> blind (never sees world names) and every cell also passes by inspection. The original entry
> follows.
- **Why:** suppress makes the dissent belief *reachable*; the world's `assumption` is what lets the
  reducer *select* it (3/3). Today the reducer doesn't see the assumption.
- **First step:** thread `world.assumption` into the L3 reduction prompt when reducing under a non-
  default world/scope; the world-frontier recompute (suppress→refixpoint) is on branch `exp/worlds`.
- **Effort:** M. **Done when:** a world-relative `mem ask` reproduces the helix/composr fixture
  divergence end-to-end (graduates the `reduction_fixtures` from TARGET to DEMONSTRATED).

### N7. Calibrate the confidence models properly (V1 caveat)
- **Why:** pair-acc verdicts are scale-invariant and stand, but "Bayesian calibrates best" is soft
  because each model used an arbitrary squash. ECE/Brier comparison isn't trustworthy until calibrated.
- **First step:** fit isotonic/Platt per model on a common scale (label = current vs defeated), then
  re-read ECE/Brier. Only matters if/when a *probability* (not a ranking) is needed downstream.
- **Effort:** S. **Done when:** a calibrated reliability diagram exists; decide if any consumer needs
  a calibrated number at all (recall ranking does not).

---

## P2 — The one big experiment still missing

### ✅ N8. Linker edge-quality A/B vs the corpus's hand-placed gold edges — DONE (2026-06-15) → V7
> **Result: the auto-Linker is a WEAK from-scratch edge proposer (best F1 supersedes 0.19; adjudicates/
> supports/refines 0.00), and that validates the frontier-review design — high-stakes defeating edges
> must be authored via `mem review`/`mem link`, not auto-drawn.** Keep qwen2.5:7b (gemma is no safer:
> more false defeating edges, a flood of junk `refines`). Limit is structural (newer→older gate + a
> missing `adjudicates` vocabulary option), not just the model. Full numbers in
> [open-questions-eval.md](open-questions-eval.md) V7. Optional low-priority follow-ons below.
- **Why:** every V6 verdict pushes structure into the reified edge graph, and the whole edge layer is
  **regenerable by the Linker** — but the Linker's edge quality has **never been scored**. "Invest in
  the Linker" is a direction, not a proven win. This is the highest-uncertainty load-bearing claim left.
- **Pre-req (N8a):** the corpus emitter must normalize inline frontmatter edges → linker-authored
  edge-beliefs (V6/Q7), so the corpus's 285 hand-placed edges become a valid **entrenchment reference**.
- **First step:** run the Linker (proximity + qwen `JudgmentLinker`) from scratch over a corpus's
  claims with edges stripped; score the proposed edges (precision/recall by kind, esp. supersedes/
  attacks) against the gold edges. Then A/B a v_n vs v_{n+1} Linker the same way (the §4a/edge-
  assignment "upgrade the agent → re-link" lever).
- **Effort:** L. **Done when:** a per-edge-kind precision/recall table exists for the Linker on ≥ 3
  corpora, and we know whether the LLM linker beats the proximity heuristic on the subtle kinds.

### N9. Scale eval on a too-big-for-a-doc corpus (cross-link ROADMAP "Evals at scale")
- **Why:** every A/B so far fit a curated CLAUDE.md — the regime static docs are *supposed* to win.
  The decisive test needs a knowledge base too large for a tidy doc, full of conflicts/supersessions.
- **First step:** the apparatus exists (eval-qa + corpus rounds); the missing piece is the big-corpus
  substrate. Grow one corpus (or the real store) past the point a single doc can hold it.
- **Effort:** L. **Done when:** frontier-resolved retrieval beats a flat curated doc on a corpus no
  human would maintain by hand.

---

## Blockers / open questions (the implementation stop point, 2026-06-15)

P0 shipped without a single decision needed — the data forced every choice. Everything past it forks
on something only a human can resolve, so the autonomous pass stops here. The forks:

- **N5 (`blocked_on` resurface) — blocked on DATA + a UX call.** The edge already works as an
  `Annotate` kind (any unknown kind is Annotate; `blocked_on` resurfaces nothing only because the
  resurface *lens* isn't built). But there are **0 deficiency beliefs** in either store, so the lens
  would be untestable machinery. It also needs the `Deficiency` fields (`severity`,
  `forcing_constraint`, `revisit_when`) parsed into `Belief` and a UX decision: is a resurfaced debt
  a recall *filter*, a banner on normal recall, or a dedicated `mem debt` command? **Needs onboarding
  Tier 2 to generate debt beliefs first.**
- **N6 (world-relative reducer) — UNBLOCKED (2026-08-02).** The fork was resolved in favor of
  "the corpus format IS the live format": a `worlds.json` in/beside the store defines named
  assumptions + suppress sets; `mem world`/`mem ask --world`/`mem relive` are the surface;
  `eval-worlds` is the keystone. Branch scopes remain the scope-derived world-analog (no assumption
  text) — a branch world that wants reducer-selectable identity gets a worlds.json entry. The
  `eval-worlds --llm` pass is recorded (2026-08-03, 6/6 — see the N6 entry): fixtures GRADUATED.
- **N4 follow-on (make `DependsOn` non-inert) — IMPLEMENTED + a model-limit finding (2026-06-15).**
  The `JudgmentLinker` can now emit `depends_on`, high-precision by design: a `depends_on` is admitted
  only if (1) the dependent's `directness` is `inferred`/`reduced` (never an independently-grounded
  `stated` fact — V5), AND (2) an adversarial binary verify confirms A rests *entirely* on B;
  otherwise it downgrades to plain `supports` (same direction, drops the JTMS contract). The full
  proposal → reified edge-belief → `defeated()` JTMS-retraction path is proven by an integration test.
  **Finding:** qwen2.5:7b will **not** propose or confirm `depends_on` even for a clean derived
  conclusion — it stably lands on `refines`/`supports` (n-way) and `depends=false` (binary verify),
  across temp-0 reruns and prompt variants (definitions, deletion-test, few-shot). This is the V5
  difficulty made concrete and is the **safe** direction (a false `depends_on` wrongly retracts; a
  missed one is just the status quo), so we keep the high-precision gate and do **not** prompt-hack the
  model toward false positives. **Net:** `depends_on` is effectively *human-authored or
  stronger-judge-authored* until measured otherwise — the open question is whether to wire a stronger
  `JUDGE_MODEL` for this one judgment, or leave emission to humans. Recall of the qwen path ≈ 0; its
  precision is the point.

  **Resolved (2026-06-15) — the frontier agent is the MAXIMUM judge.** Rather than chase a stronger
  local model, the tool now *flags* candidate justifications for on-demand adjudication by the
  consuming agent (a Claude Code session), which authors a **durable** edge. Tiered escalation:
  (1) cheap qwen judge emits `supports`/`refines`; (2) optional local `DEPENDS_MODEL` escalation
  (binary "rests entirely on?" verify → upgrade) — off by default, plumbed; (3) **`mem review`** —
  a derived (unstored) list of admissible `supports`/`refines` from a derivation (`inferred`/`reduced`
  subject) that *might* be a `depends_on`; **`mem link <s> depends_on <o>`** lets the frontier agent
  author it (author `frontier@1` → durable, never re-linked away). `mem recall`/`mem consolidate`
  print a `⚑ N edge(s) for review` nudge. Verified end-to-end: consolidate→flag→review→link→JTMS
  retraction. This is the standing answer — no stronger local model required.
- **N7 (calibration) — no consumer.** Pure analysis; recall ranking is scale-invariant and doesn't
  need a calibrated probability. Run it only if a downstream consumer of a calibrated number appears.
- **N8 (Linker A/B) — the real next experiment**, but it's an L-effort experiment (needs N8a corpus
  normalization), not a locked-in winner to wire — it belongs in a workflow pass, not this one.

## Adjacent proposals (not verdict-driven)

- **[salience-from-usefulness.md](salience-from-usefulness.md)** — parked: learn the L2 ranking blend
  from downstream task-utility feedback (reified as a defeasible `useful_for` Annotate edge), strictly
  downstream of `defeated()`. Stolen from Zhou's *"A Bitter Lesson for Memory."* Gated on having
  task-outcome traces to learn from (same shape as the N5 data blocker).
- **[lazy-background-work.md](lazy-background-work.md)** — ✅ SHIPPED (2026-07-30): daemon-less
  opportunistic consolidation. Writes (`remember`/`promote`/`onboard --commit`) trip a due-check
  that kicks a detached `mem __worker` (pidfile lock, consolidate bounded by pending writes, weekly
  `dream` piggyback); `recall`/`ask` surface a one-line summary; `mem worker [--now]` is the manual
  surface. The delivery mechanism for the existing NREM/REM linkers — see the doc's "As built"
  section for the open-question decisions (scope = triggering invocation's active scope, etc.).

## Housekeeping

- **Scratch branches** hold the experiment code: `exp/conf` (`worktree-wf_…-1`), `exp/salience-l2`,
  `exp/worlds` (`…-3`), `exp/stability` (`…-4`), `exp/retract-dependson`. Harvest the impls (N1, N3,
  N4, N6) from them, then prune the workflow worktrees (`git worktree remove .claude/worktrees/wf_*`).
- The 2026-06-15 doc edits (CLAUDE.md, this doc, open-questions-eval.md, the `HOLE→V#` tags) are
  uncommitted on `master` pending review — branch + commit when ready.

## Dependency sketch

```
N8a (normalize corpus edges) ──► N8 (Linker A/B) ──► N9 (scale eval)
Onboarding Tier 2 (ROADMAP) ───► N5 (blocked_on edge validation)
N3 (frontier pre-filter) ──────► N6 (world-assumption reducer)  [both need the reduction path solid]
N1, N2, N4  — independent, do first (P0)
```
