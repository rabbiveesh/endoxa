---
id: b_64194dcbc531
slug: ts-parse-timeout-500ms
claim:
  kind: text
  text: >-
    Tree-sitter parses are capped at 500ms (500,000 microseconds). If the timeout fires, the
    parse aborts and the syntax tree is left in a partial state.
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
    - helix-core/src/syntax.rs:1251
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges:
  - kind: supports
    target: b_8fd2517f605a   # ts-parser-thread-local
coord: null
---

In Syntax::update: `ts_parser.parser.set_timeout_micros(1000 * 500); // half a second is pretty generours`. The comment even has a typo ('generours'). A failed parse logs an error and returns None.
