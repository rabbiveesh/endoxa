---
id: b_249ebc974aaf
slug: scope-plugin-replication
kind: project-scope
claim:
  kind: text
  text: >-
    Scope expanded to NATIVE PLUGIN REPLICATION: rather than always delegating composer-
    plugin hooks, composr ports specific plugins to Rust with byte-equal output (pest-
    plugin, tbachert/spi), and adds a `composr git-hook` subcommand plus an extraction perf-
    tuning pass.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time:
    start: 2026-05-10
    end: 2026-06-02
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 1
  refs:
    - git:Native plugin replication
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_218b5b2a99c0   # scope-installer
coord: null
---

Supersedes [[scope-installer]]. The boundary moves from 'be a fast installer that defers to composer for plugins' to 'replicate the plugins that matter, natively'.
