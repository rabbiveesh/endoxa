---
id: b_d0d8020cdaca
slug: goal-byte-equivalence
claim:
  kind: text
  text: >-
    HARD correctness bar: composr's output (vendor tree, autoload bootstrap, packages.php,
    plugin artifacts) must be BYTE-EQUIVALENT to composer's, verified by golden tests
    against a real composer in the integration suite.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 3
  refs:
    - README.md
    - tests/
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges: []
coord: null
---

The bar that makes 'replace composer install' safe. It constrains every native reimplementation — discover, autoload, plugin codegen all answer to it.
