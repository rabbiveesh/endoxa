---
id: b_272e373c55f8
slug: subprocess-isolation-needed
claim:
  kind: text
  text: >-
    Module parsing MUST run in isolated subprocesses so a parser crash or a pathological
    file can't take down the language server.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: crossfile-2026-03-02
    turn: 8
  refs:
    - git:subprocess isolation for module parsing
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.8
edges: []
coord: null
---

A confident architecture hunch (asserted 0.8): crash-isolation felt necessary for robustness. It shipped first, then got falsified.
