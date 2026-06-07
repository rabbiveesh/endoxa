---
id: b_ef0b8a28dcb1
slug: ts-match-limit
claim:
  kind: text
  text: >-
    Query cursors are limited to 256 matches (`TREE_SITTER_MATCH_LIMIT = 256`) to prevent
    catastrophic backtracking in complex queries on large files.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 5
  refs:
    - helix-core/src/syntax.rs:640-657
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supports
    target: b_64194dcbc531   # ts-parse-timeout-500ms
coord: null
---

syntax.rs line 657: `const TREE_SITTER_MATCH_LIMIT: u32 = 256;`. The comment above explains this guards against performance issues on medium-to-large files.
