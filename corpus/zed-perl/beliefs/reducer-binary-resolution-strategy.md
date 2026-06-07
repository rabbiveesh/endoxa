---
id: b_888ef7dddf4e
slug: reducer-binary-resolution-strategy
claim:
  kind: text
  text: >-
    The binary resolution strategy is: (1) check PATH via worktree.which, (2) check in-
    memory cache, (3) download from GitHub releases. Only the download path updates the
    cache.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-03
    turn: 8
  refs:
    - src/perl.rs:15-96
  derived_from:
    - b_55b643e0d3f2
    - b_cd133049e928
    - b_e33cef7c70b9
confidence:
  directness: reduced
  observation_count: 1
  source_weight: 0.88
  asserted: null
edges:
  - kind: derived_from
    target: b_55b643e0d3f2   # path-first-binary-resolution
  - kind: derived_from
    target: b_cd133049e928   # scope-github-release-lsp
  - kind: derived_from
    target: b_e33cef7c70b9   # lsp-started-with-stdio
coord: null
---

Synthesized from [[path-first-binary-resolution]], [[scope-github-release-lsp]], and [[lsp-started-with-stdio]]. The three-step priority is clear from reading src/perl.rs lines 15-96.
