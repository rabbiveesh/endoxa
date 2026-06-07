---
id: b_0fc578dac9b2
slug: windows-exe-extension
claim:
  kind: text
  text: >-
    On Windows the binary path appends `.exe`; on macOS and Linux no extension is appended.
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
    turn: 3
  refs:
    - src/perl.rs:63-66
    - git:grumble: windows support
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

The `binary_path` format string uses a match on platform: `Mac | Linux => ""`, `Windows => ".exe"`. Added in commit 4a173b0 with the comment 'grumble: windows support'.
