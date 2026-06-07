# Hand-off: make the belief corpora eval-ready

Standalone brief for the agent working on `corpus/` and the evaluation harness.
Context lives in [`../design/belief-memory.md`](../design/belief-memory.md) — §3/§3a
(belief body + confidence), §4/§4a (edges + verdicts), §6 (recall/`as_of`), §9/§9c
(benchmarks + the cost-tiered eval path). Read those section tags; they're referenced below.

## What exists

**Ten** Layer-0 belief corpora (~442 beliefs) seeded from real codebases, one
markdown file per belief (frontmatter = `BeliefBody`, body = justification), plus
a generated `INDEX.md` per corpus:

| Corpus | Subject | Beliefs | Status |
|---|--:|---|---|
| `private-crm/`          | private third-party CRM | 105 | **private, gitignored, local-only** |
| `helix/`              | OSS Rust editor | 56 | committable |
| `robot-game/`         | author's Rust game | 53 | committable |
| `sql-abstract/`       | OSS Perl SQL DSL | 50 | committable |
| `perl-lsp/`           | public Perl LSP | 42 | committable |
| `perl-openapi-client/`| author's Perl client | 41 | committable |
| `tree-sitter-pod/`    | author's POD grammar | 35 | committable |
| `zed-perl/`           | author's Zed ext | 26 | committable |
| `composr/`            | public Rust/PHP tool | 25 | committable |
| `tree-sitter-perl/`   | public Perl grammar | 9 | committable |

Counts are from each generated `INDEX.md`; the prose READMEs are kept in sync.

Three emit paths, **one validated schema** (this resolves standing-fix #1):
`perl-lsp`/`composr`/`tree-sitter-perl` use Python `_seed.py` over the shared
`corpus/_belief_lib.py`; the six round-2/3 corpora use a `beliefs.json` emitted by
`corpus/_gen.py`, which **validates the frontmatter contract** (author/directness/
edge enums, edge-target resolution, session existence) before writing. `private-crm`
**intentionally** inlines its own copy of the machinery and stays isolated — private
code, kept self-contained and uncommitted; **do not converge it onto the shared lib**
(the `_gen.py` validator is the drift guard instead).

## There is no gold — only entrenchment

This brief previously used "gold labels." **Discard that framing.** There is no
ground truth in this system and the eval must not pretend otherwise:

- A **verdict is just a belief with higher `source_weight`.** A commit can encode
  a bug, be reverted, or its message can misdescribe the code. Adjudication is
  never terminal — a verdict can itself be defeated later.
- Even **reproducibility** — the strongest signal we have — **is not gold.**
  Re-running the code still requires *interpreting what you observed*, can be
  *confounded* (right output for the wrong reason; a green repro on a misconfigured
  box; an environment-specific result), and presumes the harness/observer is
  itself correct. "I reproduced it" is a defeasible belief too.

What we actually have is Popperian: not verified truths but **conjectures that
have survived refutation attempts**. Entrenchment = how much refutation a belief
has withstood, and it is *always revisable*. Labels are **graded by how hard they
are to disprove, never certain.** This is exactly AGM epistemic entrenchment +
ATMS world-relativity from the research (§12 of the design doc).

**Entrenchment proxies** — cheap, oracle-free, all themselves defeasible, strongest first:

1. **Reproducibility / checkability** — can the claim be mechanically re-derived
   (`run --parse -`, watch stdin; run Pest, watch plugins silently no-op)? Strong,
   but interpretation-laden and confoundable — *still not gold.*
2. **Time-survival** — has the belief's `valid_time` stayed open across many later
   commits? Survival of refutation *over time* is the most honest proxy and needs
   no oracle.
3. **Independent convergence** — multiple sessions / sources / methods arriving at
   the same claim (a *genuine* `observation_count`, not a hand-set one).
4. **Human post-incident confirmation** — high `source_weight`, but still a belief,
   still revisable.

The eval scores the system against the **current most-entrenched frontier**,
weighted by these proxies, and **reports its own label-uncertainty**. It never
claims accuracy-against-truth.

## What's already right — DO NOT UNDO

1. **Falsified beliefs kept non-destructively.** Beliefs a later verdict declared
   wrong (a misread, or a hunch production refuted) are retained, defeated only by
   an `adjudicates` edge, carrying **no `valid_time`** (they were never true) —
   distinct from `supersedes` (once-true, has a window). Keep this distinction.
2. **The confident-and-wrong gap is the calibration signal.** A refuted belief
   keeps its high `asserted` next to its low structural envelope
   (`directness=inferred`, `obs=1`). The *gap* is the B5 datum (§3a). Do not "tidy"
   wrong beliefs out — their wrongness is the data. (Caveat under B5 below.)
3. **Project-scope timelines.** `kind: project-scope` beliefs forming a
   `supersedes` chain with tiling `valid_time` windows → the `as_of` reliving view
   (§6). Verified the windows tile cleanly (each end = next start).
4. **The "wrong-docs" and "adversarial-haiku" sessions are excellent** — doc-vs-code
   contradictions (the doc is the refuted belief, the code the refutation) and
   deliberately-harvested confident-and-wrong reads from a weak model (asserted
   0.99 / structural w=0.4). Manufactured refutation data, on purpose. Keep.

## Eval roles are derivable — no new field, and they are GRADED not binary

Classify each belief in the harness from existing data, but treat the classes as
*entrenchment bands of a reference frontier*, not gold-vs-silver:

- **High-entrenchment reference:** `adjudicates` verdicts that are **reproducible
  and/or time-survived** (the system is scored *toward* these, weighted by
  entrenchment, never as certain truth).
- **Reliving reference:** `kind: project-scope` beliefs + `valid_time` chains →
  `as_of` / staleness (B7).
- **Reproduction targets (low confidence in the label):** `agent`-authored
  `attacks`/`supersedes`/`refines` edges and `reducer` outputs — hand-authored
  hypotheses; score as **agreement**, weighted *down* by how little they've been
  independently corroborated.
- **Observation:** primary grounded beliefs.

Earlier versions of this brief said gold = "human-authored verdicts," then "verdicts
anchored to an external truth." **Both were wrong** — the first because the public
corpora's verdicts are agent-authored, the second because a commit/repro is not
truth. The right axis is **entrenchment (reproducibility + time-survival +
convergence)**, author- and source-agnostic.

The `INDEX.md` generator already extracts verdicts (`adjudicates`) and open conflicts
(un-adjudicated `attacks`) from the edge graph; reuse that, but attach an
entrenchment weight rather than a gold flag.

## Eval method: dynamics, not truth

Because there is no oracle, **stop scoring "is X true." Score "did the system track
what actually happened to the belief over time."** That is observable from the
corpus timeline with no truth claim.

1. **Masked timeline replay (teacher-forcing).** Walk beliefs in `txn_time` order
   (tiebreak on `session, turn` — many share a day at `T12:00:00Z`). At each
   reproduction-target belief, mask it and ask the system to produce it from only
   what preceded; then **reveal the real belief and continue**, so downstream
   reductions/verdicts still have their inputs. No naive deletion; the dependency
   DAG stays intact.
2. **Time-split prediction (the oracle-free core).** Feed beliefs up to time `T`;
   ask the system which current beliefs will be **defeated** (superseded/refuted)
   by `T+k`. The later beliefs reveal it. The label is "did it survive in the
   actual history" — a fact about *dynamics*, not metaphysics. A system that
   anticipates its own revisions is better-calibrated regardless of whether the
   eventual belief was true.
3. **Lean on comparative deltas.** Strategy A vs B on the same data (§9c) needs
   only a *consistent reference*, not absolute truth — given no oracle, this is the
   most trustworthy signal per dollar. Prefer it over absolute scores.

Score reproductions/predictions against the entrenchment-weighted reference, and
**propagate label-uncertainty into the metric** (a prediction that disagrees with a
low-entrenchment reference is barely penalized).

## Refutation is frontier-relative — do NOT consume the INDEX refuted-list as labels

Spot-check (2026-06) found a live trap. The generated INDEX's "Refuted beliefs"
list is computed naively — it flags **any** belief with an incoming `adjudicates`
edge. But verdicts are revisable (verdict-of-a-verdict), so that is *wrong* for
chained verdicts. In helix's backspace chain `verdict1` adjudicates `original`,
then `verdict2` adjudicates `verdict1` — which **reinstates** `original` (the belief
body says so). Yet the INDEX lists *both* `original` and `verdict1` as refuted.

Requirement: "what is currently refuted" is **frontier-relative**. Walk each
adjudication chain to its live tip; a belief is refuted **iff its defeating verdict
is itself still undefeated**. Defeat is **non-monotonic** — a later verdict can
resurrect an earlier belief.

- The harness must **compute the refuted set itself; do not use the INDEX list as
  the label set** — it mislabels reinstated beliefs as wrong. (This is the "no gold"
  lesson one level up: even the generated gold-list is a defeasible belief.)
- **The correct computation now exists:** `corpus/_worlds.py::frontier()` resolves the
  live/defeated set frontier-relative + non-monotonic (a later verdict reinstates its
  target). Verified on helix's backspace chain (`main`: `verdict2` live → `verdict1`
  defeated → `original` reinstated). Use it (or port its fixpoint) as the label source.
- This is the §4a *defeat-along-the-frontier* semantics — exactly the non-monotonic
  case a flat fact-store can't represent, which the corpus now exercises on purpose.

## B5 calibration — and a circularity risk to fix

Extract `(asserted, structural-envelope, refutation-outcome)` triples from the
**currently-refuted** beliefs — computed frontier-relative (above), **not** the
INDEX list; exclude reinstated beliefs and any with `asserted: null` (e.g.
`r3-nulls-original`). That tuple is the calibration dataset for the §3a
`ConfidenceModel` race (StructuralOnly vs AssertedOnly vs Blend), report ECE/Brier.

**Risk:** in the public corpora, `directness=inferred` correlates almost perfectly
with *eventually-refuted*, and **inferred-but-correct survivors are sparse**. A
`StructuralOnly` model can then "win" B5 by learning "inferred → distrust," which is
partly a hindsight artifact of how the corpus was authored, not a real predictive
win. Two mitigations:
- Add **inferred beliefs that turned out right** (so structural confidence isn't a
  clean proxy for correctness).
- Weight toward the **stated-and-wrong** cases (the doc-misled refutations:
  `directness=stated`, high asserted, still wrong) — those *defeat* a structural
  model and are the honest hard cases. They already exist; emphasize them.

Also: structural confidence on refuted beliefs must reflect **what was knowable at
write time**, not the later refutation — otherwise B5 leaks hindsight.

## private-crm: private, local-only

- Richest reference source (real incident refutations); never leaves the machine —
  lean on it for local eval, guilt-free.
- Any harness output / shared report embeds private-crm **numbers only, never its
  claims or `refs`**.
- No scrubbed/redacted export path is needed or wanted — it is simply not published.
- The publishable benchmark (if ever wanted) is `perl-lsp` + `composr` only, which
  are reference-thin (~2–3 verdicts each). Soloing with no publish intent, that's a
  non-issue — only enrich them if someone else must reproduce the numbers.

## Standing fixes

1. ✅ **Schema contract — resolved.** `_gen.py` validates every JSON-sourced corpus
   against the frontmatter contract (enums, session + edge-target resolution,
   dup-slug, `asserted`-presence). **private-crm's inlined emitter remains the one
   un-validated path** — acceptable while private/local, but it has no drift guard;
   diff-check it if its schema ever feeds the shared harness.
2. ✅ **Observation-identity — resolved & implemented.** `id = "b_" +
   sha256(observation)[:12]` where `observation` = write-time body **minus edges
   and minus `observation_count`** (claim + author + provenance + confidence +
   coord + kind). Both emitters (`_belief_lib._obs_id` and the private corpus' inlined `bid`)
   now do this. Verified: same claim + different provenance → distinct ids;
   byte-identical observation → dedup; 0 collisions across 442 beliefs. Edges are
   excluded so adjudication doesn't churn ids; `observation_count` is excluded
   because it is **derived** (the `obs` integers are placeholders for a read-time
   clustering count). "Same proposition" is now an L2/L3 semantic-cluster judgment —
   **recall is a clustering problem by design**; the edge-match scorer matches
   observations by id and propositions by similarity, reporting label-uncertainty.
3. ✅ **INDEX provenance string — fixed.** `generate()` takes a `generator` arg;
   JSON corpora now read "Generated by `corpus/_gen.py (from beliefs.json)`",
   `_seed.py` corpora read "_seed.py".

## Corpus build list (targeted — don't sprawl)

Most valuable additions, in order. Each closes a *conceptual* hole, not just coverage.
**Round-3 status (2026-06): items 1–4 DONE in public corpora, grounded in real git history.**

1. ✅ **A defeated verdict (verdict-of-a-verdict).** DONE in all 5 mined public corpora
   (`r3-*-verdict1` is adjudicated by `r3-*-verdict2`): helix backspace add→revert,
   sql-abstract `-nulls` add→revert, tree-sitter-pod `indented_block` reversed in 27h,
   robot-game hints added→disabled→re-implemented, perl-openapi-client base_url
   PR#33→revert→rewrite. Refuted verdicts carry no `valid_time`.
2. ✅ **Open, un-adjudicated conflict in a public corpus.** DONE — 7 across the public
   corpora (`attacks` with no `adjudicates`): e.g. sql-abstract `injection_guard`
   FIXME, robot-game seed-stable vs -fragile, helix LSP line-terminator + per-view
   diagnostics. B4 is now testable off the private corpus.
3. ✅ **Corroboration-accretion.** DONE — helix `r3-corrob-workspace-*`: one proposition
   observed two independent ways (Cargo.toml + dir layout) across two sessions, merged
   by a reducer (`obs=2`). Modeled as **observation-identity** (two linked beliefs) —
   which is the **recommended resolution for the id-identity fork (#2 above)**; the lib
   still uses proposition-identity, so this needs ratifying before the edge-match scorer
   relies on belief identity.
4. ✅ **Inferred-but-correct survivors.** DONE — 19 across the public corpora
   (`directness=inferred`, `valid_time` open, no defeating edge, justified by write-time
   knowledge not hindsight). `inferred` is no longer a clean proxy for eventually-refuted.
5. 🟡 **Second world/branch — substrate BUILT (2026-06), reducer-behavior still unproven.**
   `corpus/composr/worlds.json` (`main` vs `conservative`) and
   `corpus/helix/worlds.json` (`main` vs `shell-muscle-memory`, grounded in the real
   backspace verdict chain — the corpus has no kanagawa belief). `corpus/_worlds.py`
   resolves each world's frontier from the shared `beliefs/*.md`, frontier-relative +
   non-monotonic; verified divergence (composr: `delegate-all-post-autoload` defeated on
   `main`, live on `conservative`; helix: `verdict2` reinstates `original` on `main`, the
   dissent world keeps `verdict1`). This proves the **representation is coherent** and
   builds the **reduction fixtures** (`worlds.json -> reduction_fixtures`, two divergent
   target consensuses per query). It does **NOT** prove an L3 reducer behaves
   world-relatively — that lands when the reducer runs over these frontiers. Merge (§5a)
   not attempted (its own research).

## Coverage caveat — don't over-read a pass

These corpora exercise L0–L3 (beliefs, edges, confidence, reduction, reliving). §5
parallel worlds now has a **representation substrate** (composr + helix `worlds.json`
+ `_worlds.py`, with divergent frontiers verified and reduction fixtures authored) —
but the worlds *thesis* (does an L3 reducer actually produce divergent consensus when
only the frontier changes?) is **still unproven** until the reducer runs over the
fixtures, and **merge (§5a) is untouched**. "Corpora + worlds substrate pass" validates
shape and that worlds are representable; it does not validate world-relative reduction
or merge — the riskiest bets.

---

# Parallel-worlds demonstrator (build spec)

**Build the minimal composr + helix worlds demonstrator.** Green-lit. This is the
highest-value tangible next step — not just because it's the first pressure on the §5
worlds thesis, but because **worlds are the substrate the whole experimentation program
needs**: the §9 A/B harness, the §3a ConfidenceModel race, and the Linker-version
benchmark are all "run variant A in world A, B in world B, compare deltas." No second
world ⇒ none of those can even be set up. It also makes **"no gold, only entrenchment"
literally true by construction** — the same belief defeated in one world and live in
another.

## What a world is (for the builder)

A world is **a set of heads** — a named pointer to a frontier of the one shared,
immutable belief DAG (Automerge/Patchwork heads ≅ git ref ≅ ATMS assumption-set label).
Everything reachable-and-not-defeated along that frontier is "in" the world. Two worlds
share ~all beliefs and differ only where frontiers diverge. A non-`main` world is **"the
frontier under assumption A"** — the assumption is the world's *identity*, not decoration.

## Representation — two decisions, resolved

1. **`worlds.json` per corpus, expressed as a diff from `main`.** This is the *authoring*
   representation; it **compiles to** the design's `World{heads}` at load time (head-set is
   derived). Not a deviation from the design — just the maintainable, readable way to
   author a world. The `assumption` string is mandatory (it's the world's identity).

2. **Divergence = head-selection at EDGE granularity.** A world diverges by which
   **edge-beliefs** (supersedes / adjudicates relations) are on its frontier — *not* by
   suppressing whole propositions. (Given edges are being reified into edge-beliefs, this
   subsumes "world-scoped edges" into head-selection — same thing, no separate world-tag.)
   So in `conservative` you keep the "handle natively" *proposition* but drop the
   *supersession relation* that makes it current.

   *Pragmatic stand-in for today:* the corpus edges are still inline in frontmatter, so a
   `suppress: [<superseder-slug>]` list is an acceptable shorthand — it yields the right
   frontier for these demos. **Frame the suppress-list as suppressing the supersession**
   (the relation), so it ports cleanly when edges become edge-beliefs.

```jsonc
// corpus/composr/worlds.json
{
  "main": { "default": true },
  "conservative": {
    "assumption": "a reachable composer binary / backward-compat outranks cold-start speed",
    "suppress": ["native-post-autoload", "plugin-policy-three-tier"]   // = suppress their supersessions
  }
}
```
On `conservative`, `delegate-all-post-autoload` and `plugins-all-delegated` are never
superseded → still live. Same belief files, opposite "current."

## What to demonstrate (composr, confirmed real slugs)

- **World-relative reduction FIXTURE.** "How does composr run post-autoload-dump?" →
  `main`: natively, 0 composer subprocess calls; `conservative`: delegate to
  `composer run-script`. Identical beliefs, different frontier, target-different consensus.
- **Per-world verdict / no-gold made literal.** `native-post-autoload`'s
  supersedes/adjudicates is "true" on `main` and simply *absent* on `conservative` —
  neither world is the truth.
- **Reliving × worlds.** `as_of(T)` gives the time axis; `in_world(W)` gives the assumption
  axis; together: "what did `conservative` believe as of date T."

Add a second, different-flavor world on helix: **`kanagawa-fans`** keeping a reverted
colour change live — a *preference* dissent, vs composr's *constraint* dissent. Shows
worlds come in flavors.

## The honesty caveat — bake this in

A corpus-only artifact proves the **representation is coherent** and builds the
**fixture**. It does **NOT** prove the reducer *behaves* world-relatively — that needs the
L3 reducer actually running over the two frontiers, which doesn't exist yet. So author the
two divergent consensuses as the **target** ("the fixture for world-relative reduction"),
**not** as "world-relative reduction demonstrated." The real §5 proof — *does an actual
reducer produce divergence when only the frontier changes?* — lands when L3 runs against
this fixture. Necessary substrate, not sufficient proof. Don't bank the overclaim.

## Staging

1. ✅ **DONE (2026-06):** coexistence + world-relative-reduction fixture + per-world
   verdict — `composr/worlds.json` (`main`/`conservative`) and `helix/worlds.json`
   (`main`/`shell-muscle-memory`, on the real backspace chain — no kanagawa belief
   exists), resolved by `corpus/_worlds.py`. Representation proven coherent; A/B harness
   unblocked. (Reducer behaving world-relatively over the fixtures = still TODO.)
2. **Later, as its own research:** merge (§5a) — reconciling two frontiers, where `attacks`
   across the merge invoke the reducer/human to mint reconciling beliefs. This is where
   cache-coherence hell lives (world-relative reduction means the **L4 cache key gains a
   world axis**) and where "reuse the structures, not the guarantees" bites hardest (ATMS
   is propositional+closed; ours is NL+stochastic). Do **not** attempt in this pass.
