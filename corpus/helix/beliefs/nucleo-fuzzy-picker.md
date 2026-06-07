---
id: b_953e9127266e
slug: nucleo-fuzzy-picker
claim:
  kind: text
  text: >-
    The file/symbol picker uses the nucleo crate (v0.5.0) for fuzzy matching, not a home-
    grown algorithm or the older skim library.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 5
  refs:
    - Cargo.toml:42
    - helix-term/src/ui/picker.rs:18-19
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

Cargo.toml workspace.dependencies: `nucleo = "0.5.0"`. helix-term/src/ui/picker.rs imports `use nucleo::{Config, Nucleo}` and constructs `Nucleo::new(...)`. The `Injector<T>` type is used to stream items into the matcher asynchronously.
