---
id: b_fd4e8a0ec72b
slug: scope-initial-single-file
kind: project-scope
claim:
  kind: text
  text: >-
    At inception (2020-05-20) the project was a single-crate prototype with a CodeMirror-
    inspired ChangeSet core and crossterm-based terminal I/O.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time:
    start: 2020-05-20
    end: 2020-10-17
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 1
  refs:
    - git:Initial import.
    - git:Implement a new core based on CodeMirror.
    - git:Start swapping from termwiz to crossterm + async.
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The initial commit (240e5f4e3) is a single Rust crate. The second commit (44ff4d3c1) reads 'Implement a new core based on CodeMirror', and an early commit (6905ff03c) reads 'Start swapping from termwiz to crossterm + async'. LSP and helix-view did not yet exist.
