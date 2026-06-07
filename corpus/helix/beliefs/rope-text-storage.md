---
id: b_f9febdae5c18
slug: rope-text-storage
claim:
  kind: text
  text: >-
    Text content is stored as a ropey::Rope with SIMD acceleration enabled. helix-core re-
    exports it directly: `pub use ropey::{Rope, RopeBuilder, RopeSlice}`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 2
  refs:
    - helix-core/Cargo.toml:22
    - helix-core/src/lib.rs:44
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.93
edges: []
coord: null
---

helix-core/Cargo.toml: `ropey = { version = "1.6.1", default-features = false, features = ["simd"] }`. helix-core/src/lib.rs line 44: `pub use ropey::{self, str_utils, Rope, RopeBuilder, RopeSlice}`.
