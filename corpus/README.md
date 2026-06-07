# Belief corpora

Real-ish Layer-0 belief stores seeded from actual codebases, used to exercise
the belief-memory design in [`../docs/design/belief-memory.md`](../docs/design/belief-memory.md)
against data with realistic shape (accreted across many sessions, with revision,
contradiction, supersession, and scope drift) rather than toy data.

**442 beliefs across 10 corpora** (~52 `adjudicates` verdicts). Note: there is **no
gold** — a verdict is just a higher-`source_weight` belief and is itself revisable
(see the hand-off's "no gold, only entrenchment"); the eval scores against the
most-entrenched frontier (reproducibility > time-survival > convergence), reporting
its own label-uncertainty.

| Corpus | Subject repo | Beliefs | Source | In git? |
|---|---|--:|---|---|
| `private-crm/`          | private third-party code | 105 | inlined `_seed.py` | **no — gitignored** |
| `helix/`              | `~/personal/helix` (OSS) | 56 | `beliefs.json` | yes |
| `robot-game/`         | `~/personal/robot-game` | 53 | `beliefs.json` | yes |
| `sql-abstract/`       | `~/personal/sql-abstract` (OSS) | 50 | `beliefs.json` | yes |
| `perl-lsp/`           | `~/personal/perl-tree-sitter-lsp` | 42 | `_seed.py` (lib) | yes |
| `perl-openapi-client/`| `~/personal/perl-openapi-client` | 41 | `beliefs.json` | yes |
| `tree-sitter-pod/`    | `~/personal/tree-sitter-pod` | 35 | `beliefs.json` | yes |
| `zed-perl/`           | `~/personal/zed-perl` | 26 | `beliefs.json` | yes |
| `composr/`            | `~/personal/composr` | 25 | `_seed.py` (lib) | yes |
| `tree-sitter-perl/`   | `~/personal/tree-sitter-perl` | 9 | `_seed.py` (lib) | yes |

`private-crm/` is derived from private third-party code and is gitignored (see
`../.gitignore`) until we have rights to it. Everything else is committable.

## Layout (each corpus)

```
<corpus>/
  beliefs/*.md   one markdown-file-per-belief = the L0 log (source of truth)
  INDEX.md       generated reduction view (do not hand-edit)
  beliefs.json   the authoring source for JSON-sourced corpora (round-2 harvest)
  _seed.py       the authoring source for the three hand-written corpora
  worlds.json    OPTIONAL: parallel-world definitions (design §5; composr + helix only)
  README.md      what this corpus demonstrates
```

### Parallel worlds (§5 substrate — composr + helix)

A **world is a set of heads** over the *shared* belief DAG (no belief is copied). A
non-`main` world is "the frontier under assumption A". `corpus/<repo>/worlds.json`
authors worlds as a diff from `main` (`suppress: [<superseder-slug>]` drops a
supersession/adjudication relation so the belief it defeated stays live; `assumption`
is the world's identity). `python3 corpus/_worlds.py corpus/<repo>` compiles each world
to its frontier — **frontier-relative and non-monotonic** (a later verdict reinstates
its target), which is also the *correct* "currently-refuted" computation (the INDEX's
naive refuted-list is wrong for verdict-of-a-verdict chains; use `_worlds.py::frontier`).

Verified divergence: composr `delegate-all-post-autoload` is **defeated on `main`, live
on `conservative`**; helix `verdict2` reinstates `original` on `main`, while
`shell-muscle-memory` keeps `verdict1`. This proves the *representation* is coherent and
builds the `reduction_fixtures` (two divergent **target** consensuses per query). It does
**NOT** prove an L3 reducer behaves world-relatively — that needs the reducer to run over
the frontiers. Necessary substrate, not sufficient proof; merge (§5a) is untouched.

Three emit paths, **one schema contract**:
- `perl-lsp/`, `composr/`, `tree-sitter-perl/` — Python `_seed.py` over the shared
  `_belief_lib.py` (emit + index machinery).
- `private-crm/` — **intentionally isolated**: inlines its own copy of the machinery
  and stays uncommitted (private code). Do **not** converge it onto the shared lib.
- `robot-game/`, `helix/`, `sql-abstract/`, `perl-openapi-client/`,
  `tree-sitter-pod/`, `zed-perl/` — a `beliefs.json` emitted by `_gen.py`, which
  **validates the frontmatter contract** (author/directness/edge enums, edge
  targets resolve, sessions exist) before writing. That validator is the
  "one schema contract" guard from the eval-readiness hand-off — the isolated
  private-corpus emitter can't silently drift into a shape the JSON corpora's parser rejects.

Regenerate one: `python3 corpus/_gen.py corpus/<repo>` (JSON) or
`python3 corpus/<repo>/_seed.py` (Python).

> **Resolved (id-identity, hand-off standing-fix #2): OBSERVATION identity, edges
> excluded.** `id = "b_" + sha256(observation)[:12]` where `observation` is the
> write-time BeliefBody **minus edges and minus `observation_count`**: claim +
> author + provenance(txn_time, valid_time, source, refs) + confidence(directness,
> source_weight, asserted) + coord + kind. Consequences, now baked into both emitters:
> - Two independent observations of the same claim get **different ids** (their
>   provenance differs) and are linked by edges; identical observations dedup to one id.
> - Edges are excluded so a later `adjudicates`/`supersedes` doesn't churn the id.
> - `observation_count` is **excluded because it is derived** — corroboration is a
>   read-time clustering count over linked observations, *not* baked into identity.
>   The `obs` integers in the corpora are therefore **placeholders for what the
>   reducer should compute**, not authority.
> - "Same proposition" is deliberately **not** an L0 identity (string-hash can't
>   capture it); it is an L2/L3 semantic-clustering judgment. **Recall is a clustering
>   problem by design** — harder, but honest; the eval's edge-match scorer matches
>   observations exactly by id and propositions by semantic similarity, reporting
>   label-uncertainty on the fuzzy matches.

## What each one stresses

- **private-crm** (gitignored; not documented in detail here) — breadth across many
  subsystems: revision, a human verdict adjudicating a conflict, cross-session
  corroboration → reducer consensus, live un-adjudicated conflicts, and a bi-temporal
  architecture shift.
- **perl-lsp** & **composr** — **project scope over time**: a superseding chain
  of `project-scope` beliefs, each with a `valid_time` window, so the INDEX can
  show "what the project *was* at any given date." Plus design decisions that got
  overturned (witness-bag replacing builder-time inference; hardcoded frameworks
  → Rhai plugins; composr's delegate-all → native handling).
- **round-2 corpora** (robot-game, helix, sql-abstract, perl-openapi-client,
  tree-sitter-pod, zed-perl) — **realistic noise + breadth + organic wrongness.**
  Harvested by the `corpus-harvest-round-2` workflow (one agent per repo). Each
  deliberately includes a handful of **dumb/trivial beliefs** (license, binary
  name, config path — the low-value stuff real memory accretes; it stresses recall
  precision, B1), plus git-grounded scope chains, supersession arcs, and
  git-evidenced falsifications. They also carry **organic Haiku-wrong beliefs**: a
  no-tool underpowered model probed tricky snippets and a tool-using model verified
  each against the code — e.g. robot-game `boredom-min-2-entries` (off-by-one),
  sql-abstract `empty-ne-is-true` (sign-flip) and `open-paren-strips` (read "open"
  as "add parens", it strips them), zed-perl `injection-regex-expressibility`
  (right answer, *confabulated* reason). The harvest ran ~83% correct overall;
  these are the kept wrongs, authored as the Haiku model and defeated by verdicts.
- **round-3 conceptual additions** (the `corpus-round-3-conceptual` workflow + a
  hand-authored corroboration case) — closes the hand-off build-list holes, all
  grounded in real git history, **no new repos** (targeted, not sprawl):
  - **Verdict-of-a-verdict** in all 5 mined public corpora — a verdict that is
    itself later overturned (`Vn` *adjudicates* `Vn-1`). Real arcs: helix
    backspace-out-of-prompt add→revert, sql-abstract `-nulls` add→revert ("not
    supported by enough databases"), tree-sitter-pod `indented_block` reversed in
    27h, robot-game hints added→disabled→re-implemented, perl-openapi-client
    base_url PR#33→revert→rewrite. Tests verdict *revision*; gives time-split
    prediction a label that flips. Refuted verdicts carry **no `valid_time`**.
  - **Open, un-adjudicated conflicts** in public corpora (7) — `attacks` with no
    `adjudicates`, so B4 ("surface, don't auto-resolve") is now testable off the private corpus
    (e.g. sql-abstract injection_guard FIXME, robot-game seed-stable vs -fragile).
  - **Inferred-but-correct survivors** (19) — `directness=inferred`, `valid_time`
    open, no defeating edge, body justified by *write-time* knowledge (not by the
    fact they survived). Fixes the B5 circularity: `inferred` is no longer a clean
    proxy for eventually-refuted.
  - **Corroboration-accretion** (helix `r3-corrob-workspace-*`) — one proposition
    observed two independent ways across two sessions, merged by a reducer
    (`obs=2`). Modeled as **observation-identity** (two linked beliefs), which is
    the recommendation for standing-fix #2 (see the id-identity note above).

### Refuted beliefs (the most important category)

Every corpus has a **"Refuted beliefs (calibration + entrenchment references)"**
section in its INDEX: beliefs a later verdict declared *wrong* — a misread of the
code, or a hunch since refuted — each defeated by an `adjudicates` edge (and a
verdict is itself just a revisable belief, not gold).
These are distinct from supersessions: the wrong belief was **never true**, so it
carries **no `valid_time`**; only the verdict defeats it. Each wrong belief keeps
its (high) `asserted` confidence next to its (low) structural confidence
(`directness=inferred`, `obs=1`) — the gap *is* the calibration gold label
(§3a / §9 B5): confident-and-wrong, with the structural envelope as the honest
predictor. Examples:

- **perl-lsp:** the cold-start type-query memo keyed the receiver by `InferredType`
  *variant* only → under **diamond inheritance** it served one class's cached type
  to another (silent wrong type), caught by soundness review 2026-06-03.
- **composr:** pest-plugin assumed *inert* → without native codegen every Pest
  plugin silently no-ops.

Verdicts are mostly human-authored and post-incident at near-max `source_weight`
(ground truth, §4a) — they're the supervised signal benchmarks need (§9 B2/B5).

#### How the wrong beliefs were *harvested* (not hand-authored)

The most recent additions are **organic** wrong beliefs, gathered two adversarial ways:

1. **Underpowered-model harvest.** Intentionally weak models (claude-haiku-4-5) were run on
   tricky real snippets and asked to commit to confident behavioral claims, each authored
   *as the Haiku model* (`source_weight 0.4`) and defeated by a verdict. Run as a
   **multi-agent workflow** (`adversarial-wrong-belief-harvest`) over
   perl-lsp and tree-sitter-perl: tool-enabled extractors found 18 gotcha-prone snippets → one
   **no-tool Haiku** probe per snippet forced confident claims → a tool-using adjudicator verified
   each against the real code. Result: **36 claims, 6 wrong (~83% correct)**, living in `perl-lsp/`
   (session P11) and `tree-sitter-perl/` (T2), each Haiku-authored with a verdict. Failure modes:
   *right-answer-wrong-mechanism* (a 0.99 claim about which line ends a loop), *under-counting work*
   (a re-scan in another phase), and **two invented bugs that don't exist** (`dedup_by` "broken"; a
   documented heredoc-overflow degradation called a "bug"). The false-positive-bug mode is why §9
   wants adversarial verification, not acceptance on asserted confidence — and the errors all
   clustered where the justification reached *beyond the snippet* (a normalization in another fn, a
   phase skip elsewhere, tree-sitter's GLR conflict semantics in the core repo).
2. **Objectively-wrong docs.** A sweep found documentation the code flatly contradicts, each
   verified by reading both sides. These become wrong beliefs *sourced from a trusted doc*
   (`source_weight` 0.8) — the calibration point being that **high source-weight did not make
   them true**:
   - perl-lsp: CLAUDE.md says `--parse --` reads stdin, but the sentinel is `-` (the in-code
     help is even correct — only the prose drifted); "MAX_FOLD_ITERATIONS debug-only" runs in
     release too.
   - composr: README's `--strict-plugins` parenthetical says unknown plugins default to "inert",
     contradicting its own policy section and the code (they're *delegated*).

The doc-vs-code sweep also gives a **retraction-soundness** demo (§9 B6): where a reducer
derives from a belief later falsified, its conclusion can still *stand* if the other
corroborating inputs were real — consensus over independent sources survives one false
premise. That's the payoff of `observation_count > 1`.

## Regenerating

```bash
python3 corpus/private-crm/_seed.py
python3 corpus/perl-lsp/_seed.py
python3 corpus/composr/_seed.py
```
