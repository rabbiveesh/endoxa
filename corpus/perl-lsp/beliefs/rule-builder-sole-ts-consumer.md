---
id: b_ef333072dce6
slug: rule-builder-sole-ts-consumer
claim:
  kind: text
  text: >-
    HARD RULE: all tree-sitter CST traversal happens inside build() — builder.rs is the ONLY
    tree-sitter consumer. Nothing else walks nodes, calls child_by_field_name, or uses
    TreeCursor; everyone else queries FileAnalysis.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: crossfile-2026-03-02
    turn: 4
  refs:
    - CLAUDE.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges:
  - kind: refines
    target: b_56302f26083a   # arch-four-layers
coord: null
---

Refines [[arch-four-layers]]. file_analysis.rs is the single source of truth; symbols.rs is a thin adapter; cursor_context.rs is the one position-dependent exception (reads a tree but never mutates FileAnalysis).
