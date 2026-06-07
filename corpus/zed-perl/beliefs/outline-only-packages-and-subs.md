---
id: b_6948aaa4f086
slug: outline-only-packages-and-subs
claim:
  kind: text
  text: >-
    The outline.scm only surfaces package declarations and subroutine declarations as
    outline items; no methods, classes, or variables.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-08-03T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-01
    turn: 3
  refs:
    - languages/perl/outline.scm:1-12
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

outline.scm contains exactly two item patterns: `(package_statement ...)` and `(subroutine_declaration_statement ...)`. No method_declaration_statement or class_statement is included.
