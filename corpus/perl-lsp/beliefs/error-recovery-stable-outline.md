---
id: b_a420c2a2b688
slug: error-recovery-stable-outline
claim:
  kind: text
  text: >-
    perl-lsp degrades gracefully on broken parses: it recovers structural declarations from
    tree-sitter ERROR nodes and keeps a STABLE outline across parse degradation — because
    mid-typing you routinely bork the tree.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: error-recovery-2026-03-29
    turn: 5
  refs:
    - git:recover structural declarations from ERROR nodes
    - git:stable outline
    - docs/adr/error-recovery.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

Robustness era. The stable outline means document symbols don't flicker while you type.
