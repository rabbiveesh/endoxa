---
id: b_5922d9b763ea
slug: r3-lsp-spec-compliance-b
claim:
  kind: text
  text: >-
    Helix does not fully comply with the LSP spec for line terminators because its unicode-
    linebreak feature recognises additional line-break characters beyond \n/\r\n/\r
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
    turn: 2
  refs:
    - helix-lsp/src/lib.rs:164
    - git:7ebcf4e91
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.75
  asserted: null
edges:
  - kind: attacks
    target: b_9e182010691a   # r3-lsp-spec-compliance-a
coord: null
---

The FIXME comment at helix-lsp/src/lib.rs:164 (added in commit 7ebcf4e91, 2023-02-09) documents this directly: "Helix does not fully comply with the LSP spec for line terminators. The LSP standard requires that line terminators are ['\n', '\r\n', '\r']. Without the unicode-linebreak feature disabled, the `\r` terminator is not handled by helix. With the unicode-linebreak feature, helix recognizes multiple extra line break chars which means that positions will be decoded/encoded incorrectly in their presence." Neither path is correct: one omits `\r` support, the other adds extra breaks that break position encoding. This is a genuine open conflict that attacks [[r3-lsp-spec-compliance-a]]: the actual implementation contradicts the normative requirement and no fix has been landed.
