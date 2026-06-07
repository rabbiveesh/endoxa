---
id: b_9e182010691a
slug: r3-lsp-spec-compliance-a
claim:
  kind: text
  text: >-
    Helix should fully comply with the LSP spec for line terminators (\n, \r\n, \r only),
    mapping positions correctly for all LSP servers
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-session
    turn: 1
  refs:
    - helix-lsp/src/lib.rs:164
    - git:7ebcf4e91
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: null
edges: []
coord: null
---

The LSP specification requires that line terminators are exactly `\n`, `\r\n`, and `\r`. Any editor that implements LSP position encoding must correctly handle exactly these line-end sequences and no others, so that character offsets sent to and received from language servers are correct. This is the normative requirement the code comment in helix-lsp/src/lib.rs:164 acknowledges helix does not fully satisfy.
