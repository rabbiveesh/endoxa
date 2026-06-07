---
id: b_5162889d1f5a
slug: ts-perl-identity
claim:
  kind: text
  text: >-
    tree-sitter-perl is the tree-sitter grammar for Perl: a generated parser from grammar.js
    plus a hand-written C external scanner (src/scanner.c) for the context-sensitive bits
    (heredocs, POD, quote-like, regex). perl-lsp is built on it (via the ts-parser-perl
    crate).
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: ts-perl-orient-2026-06-04
    turn: 1
  refs:
    - README.md
    - grammar.js
    - src/scanner.c
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The scanner is where Perl's lexer-hostile constructs live; the grammar.js handles the context-free structure with GLR conflicts for the genuinely ambiguous parts.
