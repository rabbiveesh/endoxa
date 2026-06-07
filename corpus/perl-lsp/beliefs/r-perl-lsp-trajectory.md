---
id: b_ab578d405997
slug: r-perl-lsp-trajectory
claim:
  kind: text
  text: >-
    perl-lsp's trajectory: single-file LSP -> cross-file -> typed -> framework-aware ->
    extensible Rhai-plugin platform -> navigation/graph-walking platform. Each era strictly
    expanded scope, while a few hard rules (single tree-sitter consumer, witness-bag-
    canonical, no special-casing) kept the core from sprawling as features piled on.
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: nav-graph-2026-06-04
    turn: 20
  refs: []
  derived_from:
    - b_72510133d526
    - b_271c40c6a09a
    - b_ab7038cc8458
    - b_de13663b1e7d
confidence:
  directness: reduced
  observation_count: 4
  source_weight: 0.6
  asserted: null
edges:
  - kind: derived_from
    target: b_72510133d526   # scope-mvp
  - kind: derived_from
    target: b_271c40c6a09a   # scope-frameworks
  - kind: derived_from
    target: b_ab7038cc8458   # scope-plugin-platform
  - kind: derived_from
    target: b_de13663b1e7d   # rule-no-special-casing
coord: null
---

REDUCED over the scope chain plus the governing discipline. The interesting property a flat fact-store would lose: scope is a moving target with a stable spine, and the spine is WHY the expansion didn't collapse into special cases.
