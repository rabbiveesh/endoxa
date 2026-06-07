---
id: b_b827c9d7f6ab
slug: find-completion-range-off-by-one-old
claim:
  kind: text
  text: >-
    Before July 2024, `find_completion_range` in helix-lsp overcounted the end position in
    replace mode by one: it did `.skip(1)` before `.take_while(char_is_word)` and then added
    +1, producing a range end that was one character too far.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time:
    start: None
    end: 2024-07-24
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 3
  refs:
    - git:fix(lsp): `find_completion_range` off-by-one (#11266)
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

The bug was present in the code before commit 7c5e5f4e4 (2024-07-24). The old logic: `end += text.chars_at(cursor).skip(1).take_while(...).count() + 1;` - skipping the first char under cursor then adding 1 back double-counted.
