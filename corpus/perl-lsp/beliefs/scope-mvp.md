---
id: b_72510133d526
slug: scope-mvp
kind: project-scope
claim:
  kind: text
  text: >-
    perl-lsp is a SINGLE-FILE Perl language server on tree-sitter-perl + tower-lsp: scope-
    aware rename, completion, goto-def, hover, hash-key intelligence and signature help, all
    within one open file. No cross-file resolution, no type inference.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-02-20T12:00:00Z
  valid_time:
    start: 2026-02-20
    end: 2026-03-02
  source:
    kind: conversation
    session: mvp-2026-02-20
    turn: 1
  refs:
    - git:831353b
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The MVP scope. tree-sitter parses, an early scope graph powers within-file navigation. Everything later strictly expands this.
