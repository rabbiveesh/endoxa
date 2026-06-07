---
id: b_5f80cb3711cd
slug: scope-async-hook-system
kind: project-scope
claim:
  kind: text
  text: >-
    Since December 2023 the project has a formal async hook/event system in the helix-event
    crate, replacing ad-hoc polling patterns for LSP completion, signature help, etc.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-12-01T12:00:00Z
  valid_time:
    start: 2023-12-01
    end: 2999-01-01
  source:
    kind: conversation
    session: s-event-hook-2023
    turn: 1
  refs:
    - git:Add hook/event system
    - helix-event/src/lib.rs:1-33
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_dbb55470267e   # scope-lsp-compositor-added
coord: null
---

Commit 13ed4f6c4 (2023-12-01) 'Add hook/event system' introduced helix-event/src/hook.rs, registry.rs, runtime.rs, debounce.rs and a 1000+ line diff touching helix-term and helix-view. The module docs in helix-event/src/lib.rs explain synchronous hooks and AsyncHook types.
