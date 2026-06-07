---
id: b_322680d5c6b4
slug: aarch64-unsupported
claim:
  kind: text
  text: >-
    ARM64 (aarch64) and 32-bit x86 are explicitly unsupported architectures; the extension
    returns an error for them.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-03
    turn: 2
  refs:
    - src/perl.rs:46-51
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges: []
coord: null
---

The asset_name format block in src/perl.rs returns `Err(format!("unsupported architecture: {arch:?}"))` for both `Architecture::Aarch64` and `Architecture::X86`.
