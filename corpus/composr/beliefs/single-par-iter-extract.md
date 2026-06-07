---
id: b_8758a772d3d2
slug: single-par-iter-extract
claim:
  kind: text
  text: >-
    Extraction dropped the huge/small partition for a single LPT par_iter, and further trims
    overhead: dedup parent mkdirs, skip chmod for 0o644 entries, async-trash old install
    dirs in a detached child.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time:
    start: 2026-05-10
    end: 2999-01-01
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 10
  refs:
    - git:drop huge/small partition, single LPT par_iter
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges:
  - kind: supersedes
    target: b_e1fc6b70d4e0   # lpt-partition-extract
coord: null
---

Supersedes [[lpt-partition-extract]]. Simpler scheduling + per-entry syscall trimming beat the bucketed scheme.
