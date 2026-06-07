---
id: b_2defce209835
slug: extraction-is-bottleneck
claim:
  kind: text
  text: >-
    After the first run, zip extraction (~14s, parallelized via rayon) DOMINATES cold-cold
    wall-clock — bound by disk metadata throughput and the largest single archive. Classmap
    walking is off the critical path once the cache is warm.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 7
  refs:
    - README.md
    - git:Parallelize zip extraction with rayon
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

Names the real bottleneck once caching solved the classmap cost. Drove the extraction perf-tuning pass.
