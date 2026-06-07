# perl-lsp — belief corpus

Seeded from `~/personal/perl-tree-sitter-lsp` (a Perl LSP on tree-sitter-perl +
tower-lsp). 42 beliefs across 11 sessions spanning the project's real lifetime
(2026-02 → 2026-06), authored as if an agent co-developed it the whole way:
`txn_time` = when each belief was recorded, `valid_time` = when a scope statement
was true in the world.

## The headline: scope over time

Six `project-scope` beliefs form a superseding chain, each with a `valid_time`
window — so `INDEX.md` renders the project's scope at any given date:

```
2026-02-20  single-file LSP (rename/completion/goto-def/hover)      scope-mvp
2026-03-02  + cross-file (resolver thread, SQLite cache, diags)     scope-crossfile
2026-03-08  + type inference (InferredType, cross-file returns)     scope-typed
2026-03-15  + frameworks & inheritance (Moo/Mojo/DBIC), CLI, tokens scope-frameworks
2026-04-27  pivot → extensible Rhai-plugin platform + witness bag   scope-plugin-platform
2026-06-01  → navigation / graph-walking platform                  scope-nav-platform
```

Each `supersedes` the previous: scope expansion modeled as non-destructive
revision. `as_of(2026-03-10)` would relive the "typed but pre-framework" project.

## Decisions that got overturned

- `witness-bag-canonical` **supersedes** `builder-infer-expression-type` — type
  inference moved from build-time `Builder::infer_expression_type` walks to a
  single canonical witness bag (CLAUDE.md: "Builder::infer_expression_type is gone").
- `frameworks-are-rhai-plugins` **supersedes** `frameworks-were-hardcoded` — the
  2026-04-27 pivot from hardcoded Rust framework logic to Rhai plugins.

## Objectively wrong beliefs (verdict gold labels)

Falsified hunches/misreads, each defeated by an `adjudicates` verdict (no
`valid_time` — they were never true):

- `coarse-memo-key-unsound-diamond` defeats `memo-coarse-receiver-key-sound` —
  **the diamond-inheritance caching bug (fixed 2026-06-03).** The cold-start
  type-query memo keyed the receiver slot by `InferredType` variant only, so a
  shared `MethodOnClass{Parent,m}` reached with `ClassName("Foo")` then
  `ClassName("Bar")` served Foo's cached type to Bar — a *silent* wrong type. The
  trap: assuming the cycle-guard's coarse key (safe because collision→None)
  transferred to a result memo (which returns a value). Asserted 0.85, wrong.
- `subprocess-isolation-removed` defeats `subprocess-isolation-needed` — the
  crash-isolation worry never materialized; the IPC overhead was real (removed in #14).
- `honest-miss-over-guessing` defeats `dispatch-always-resolvable` — guessing a
  dispatch target produced wrong go-to-def jumps; NAV now records an honest-miss.

Plus **objectively-wrong docs** (session P10), beliefs sourced from the project's own
docs that the code contradicts — high `source_weight` (docs are trusted) yet false:
`parse-stdin-is-single-dash` defeats `doc-parse-stdin-double-dash` (CLAUDE.md says
`--parse --` reads stdin; the sentinel is `-`, and the in-code help is even correct);
`max-fold-runs-in-release` defeats `doc-max-fold-debug-only`; `doc-test-count-stale`
(README's hardcoded "317 unit tests" has long since drifted).

## The stable spine

What kept scope growth from sprawling, captured as hard-rule beliefs:
`rule-builder-sole-ts-consumer` (one tree-sitter consumer), `witness-bag-canonical`
+ `edges-not-values`, and `rule-no-special-casing` ("you are always wrong" —
encode behavior on the type, not a method-name allowlist; its canonical
application is `parametric-types-resultset`). `r-perl-lsp-trajectory` (reducer)
consolidates the arc.

Grounded in the repo's `CLAUDE.md`, `docs/adr/*`, `README.md`, and git history.
Regenerate with `python3 _seed.py` (shared machinery in `../_belief_lib.py`).
