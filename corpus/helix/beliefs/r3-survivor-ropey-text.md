---
id: b_284fd835ea59
slug: r3-survivor-ropey-text
claim:
  kind: text
  text: >-
    Helix uses the ropey crate as its sole text-storage data structure; text is never stored
    as a plain String or byte Vec during editing
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2020-06-15
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 2
  refs:
    - helix-core/Cargo.toml
    - git:073fe61264d5154eb0bd37be575ccc91e17b74d7
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

Inferable at write-time (2020-06-15): commit 073fe61264 switched the project to ropey 1.2.0 and the `helix-core/Cargo.toml` listed `ropey` as a non-optional dependency. The `Rope` type was exported from `helix-core::lib.rs` as the canonical document type. The `Transaction::apply` API took a `&mut Rope`, and the `Syntax` wrapper accepted `RopeSlice` for highlighting — no String pathway existed at that point. Inferable from: (1) ropey is the only text-container in Cargo.toml, (2) `pub use ropey::Rope` re-exported from helix-core, (3) all API surfaces that touch document text accept `Rope`/`RopeSlice`. Has survived 8000+ commits through ropey version bumps (1.2 → 1.6.1); mechanically checkable: `grep 'ropey' helix-core/Cargo.toml` still shows ropey as sole text dep.
