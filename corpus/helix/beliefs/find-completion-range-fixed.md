---
id: b_a04954163f2e
slug: find-completion-range-fixed
claim:
  kind: text
  text: >-
    The correct `find_completion_range` in replace mode extends `end` by counting word chars
    starting from the cursor position without any skip or +1 adjustment.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time:
    start: 2024-07-24
    end: 2999-01-01
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 4
  refs:
    - helix-lsp/src/lib.rs:276-291
    - git:fix(lsp): `find_completion_range` off-by-one (#11266)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_b827c9d7f6ab   # find-completion-range-off-by-one-old
  - kind: attacks
    target: b_b827c9d7f6ab   # find-completion-range-off-by-one-old
  - kind: supersedes
    target: b_b827c9d7f6ab   # find-completion-range-off-by-one-old
coord: null
---

Commit 7c5e5f4e4 (2024-07-24) simplified the replace-mode branch to `end += text.chars_at(cursor).take_while(|ch| chars::char_is_word(*ch)).count();` — no skip, no +1. This is the current implementation.
