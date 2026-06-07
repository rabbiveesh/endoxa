---
id: b_1dbad2e5b8df
slug: max-fold-runs-in-release
claim:
  kind: text
  text: >-
    VERDICT: FALSE. The fold has a `debug_assert!` (dev-only) AND a separate unconditional
    `if iters >= MAX_FOLD_ITERATIONS { eprintln!("...bailing out..."); break; }` that runs
    in RELEASE too — the code's own comment even calls it 'the all-builds safety net'.
    CLAUDE.md's 'debug-only' contradicts the code and the code's comment.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: wrong-docs-2026-06-04
    turn: 6
  refs:
    - src/builder.rs:10108
    - src/builder.rs:10113
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_080832a9f09a   # doc-max-fold-debug-only
  - kind: attacks
    target: b_080832a9f09a   # doc-max-fold-debug-only
coord: null
---

Defeats [[doc-max-fold-debug-only]]. A stale doc claim about runtime behavior — the kind that would make an agent reason wrongly about whether release builds can bail the fold.
