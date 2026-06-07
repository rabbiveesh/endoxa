---
id: b_8decda63cef9
slug: crate-type-cdylib
claim:
  kind: text
  text: >-
    The Rust crate compiles to a cdylib, which Zed loads as a WebAssembly extension.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-02
    turn: 2
  refs:
    - Cargo.toml:8-9
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

Cargo.toml sets `crate-type = ["cdylib"]` with `lib.path = "src/perl.rs"`. The compiled output `extension.wasm` is checked in.
