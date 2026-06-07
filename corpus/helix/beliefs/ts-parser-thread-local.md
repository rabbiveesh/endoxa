---
id: b_8fd2517f605a
slug: ts-parser-thread-local
claim:
  kind: text
  text: >-
    The tree-sitter Parser is stored as a thread-local RefCell (PARSER), not in any shared
    state. This avoids locks but means the parser is single-use per thread.
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
    - helix-core/src/syntax.rs:1061-1072
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges:
  - kind: supports
    target: b_adb55f6443eb   # syntax-uses-hop-slotmap-layers
coord: null
---

syntax.rs: `thread_local! { pub static PARSER: RefCell<TsParser> = RefCell::new(TsParser { parser: Parser::new(), cursors: Vec::new() }) }`. Both `Syntax::update` and `highlight_iter` borrow this thread-local parser.
