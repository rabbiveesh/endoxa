---
id: b_9c7e4efe3333
slug: helix-lsp-types-extracted
claim:
  kind: text
  text: >-
    Since July 2024 helix-lsp uses its own helix-lsp-types crate instead of the third-party
    lsp-types crate, giving the project control over LSP type definitions.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time:
    start: 2024-07-27
    end: 2999-01-01
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 2
  refs:
    - git:Replace lsp-types in helix-lsp with helix-lsp-types
    - helix-lsp/src/lib.rs:11
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges: []
coord: null
---

Commit e21e4eb82 (2024-07-27) 'Replace lsp-types in helix-lsp with helix-lsp-types'. The parent PR (3fcf168c3, 2024-07-31) merged the helix-lsp-types crate. helix-lsp/src/lib.rs now has `pub use helix_lsp_types as lsp;`.
