---
id: b_17d3ff28ce12
slug: scope-released
kind: project-scope
claim:
  kind: text
  text: >-
    Current scope: a PUBLISHED 1.0 crate on crates.io with publish-on-tag CI — a production
    tool that installs two real production-shape Laravel codebases byte-equivalently to
    composer, with 0 composer subprocess calls on a cold install.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-02T12:00:00Z
  valid_time:
    start: 2026-06-02
    end: 2999-01-01
  source:
    kind: conversation
    session: release-2026-06-02
    turn: 1
  refs:
    - git:Prep 1.0.0 release
    - git:publish-on-tag CI
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_249ebc974aaf   # scope-plugin-replication
coord: null
---

Supersedes [[scope-plugin-replication]]. 'Working end-to-end on real apps' is now the headline, not a single command.
