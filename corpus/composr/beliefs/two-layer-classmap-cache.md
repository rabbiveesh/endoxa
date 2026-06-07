---
id: b_67baa8cf92f8
slug: two-layer-classmap-cache
claim:
  kind: text
  text: >-
    Two classmap caches stack: a PER-PACKAGE shared cache keyed (package_name,
    dist.reference) at $XDG_CACHE_HOME/composr/classmap/... that survives `rm -rf vendor`,
    and a PER-FILE mtime cache (vendor/composer/.classmap-cache.bin) for the root +
    uncacheable (path/no-reference) packages. Together a changed-one-file install is sub-
    second.
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
    turn: 5
  refs:
    - README.md
    - git:Persistent per-package classmap cache
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

A per-package cache hit skips walk + parse + admission entirely and merges cached entries straight into the global map.
