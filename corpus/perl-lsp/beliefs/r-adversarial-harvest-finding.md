---
id: b_1b94e45d626d
slug: r-adversarial-harvest-finding
claim:
  kind: text
  text: >-
    Adversarial-harvest finding (18 snippets, 36 claims, both perl-lsp and tree-sitter-
    perl): the underpowered model was CORRECT ~83% even on deliberately tricky code. Its
    errors clustered not in local reasoning but where the trap needed knowledge OUTSIDE the
    snippet — a skip in a different phase, a normalization in a different fn, tree-sitter's
    GLR conflict semantics. Failure modes seen: right-answer-wrong-mechanism (0.99 conf),
    under-counting work, and TWO invented-bugs-that-don't-exist (0.98/0.95 conf).
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 20
  refs: []
  derived_from:
    - b_4a940bb9f90d
    - b_57eaa985ff0a
    - b_be1be8159cce
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.6
  asserted: null
edges:
  - kind: derived_from
    target: b_4a940bb9f90d   # haiku-transitive-parents-break-guard
  - kind: derived_from
    target: b_57eaa985ff0a   # haiku-refs-nonopen-single-scan
  - kind: derived_from
    target: b_be1be8159cce   # haiku-dedup-fails-mixed-key
coord: null
---

REDUCED over the harvested wrong beliefs. The design implication: confident-wrong beliefs are most likely exactly where a belief's justification reaches beyond what was in view — which is precisely what the justification-edge graph (§4) and cross-file provenance are for. And the 'invented bug' mode argues for adversarial verification (§9): a plausible bug claim should be refuted against code before it's trusted, never accepted on asserted confidence.
