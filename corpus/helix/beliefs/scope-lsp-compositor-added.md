---
id: b_dbb55470267e
slug: scope-lsp-compositor-added
kind: project-scope
claim:
  kind: text
  text: >-
    By December 2020 the project had gained a Compositor abstraction for layered UI, basic
    LSP lifecycle support, and the Document/Editor split.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time:
    start: 2020-12-07
    end: 2023-11-30
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 1
  refs:
    - git:Merge pull request #5 from helix-editor/lsp
    - git:wip: Compositor
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_fd4e8a0ec72b   # scope-initial-single-file
coord: null
---

Commits b2b3083a6 (2020-10-19) 'Support multiple open views', b7a3e525e (2020-12-03) 'Merge pull request #5 from helix-editor/lsp', and 83f2c2411 (2020-12-06) 'wip: Compositor' mark these additions. The compositor commit (b12a6dc83) landed 2020-12-13.
