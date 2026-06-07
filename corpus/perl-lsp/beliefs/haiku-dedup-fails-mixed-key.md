---
id: b_be1be8159cce
slug: haiku-dedup-fails-mixed-key
claim:
  kind: text
  text: >-
    refs_to's dedup_by does NOT remove duplicates when the same file appears as both
    FileKey::Url and FileKey::Path, because key_for_sort differentiates them so they sort
    non-adjacently and dedup_by only drops consecutive equals — i.e. duplicate refs survive
    (a bug).
author:
  kind: agent
  id: claude-haiku-4-5
  model: claude-haiku-4-5
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 6
  refs:
    - src/resolve.rs:223-232
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.98
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.98. A confidently-INVENTED bug.
