---
id: b_c84fc1ac6b26
slug: wrong-capture-names-initially
claim:
  kind: text
  text: >-
    The initial highlights.scm used Neovim/generic tree-sitter capture names like
    `@include`, `@conditional`, `@repeat` rather than Zed's namespaced forms.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time:
    start: 2024-08-03
    end: 2024-09-19
  source:
    kind: conversation
    session: seed-explore-03
    turn: 5
  refs:
    - git:fix: use more appropriate captures for zed
    - languages/perl/injections.scm:1
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.85
edges:
  - kind: supersedes
    target: b_68a4e1997fb4   # scope-grammar-only
coord: null
---

Commit 4eef601 renamed captures: `@include` -> `@keyword.include`, `@conditional` -> `@keyword.conditional`, `@conditional.ternary` -> `@operator.conditional`, `@repeat` -> `@keyword.repeat`, `@exception` -> `@keyword.exception`. The injections.scm still carries a comment '`; an scm file for nvim-treesitter`' suggesting copy-paste origin.
