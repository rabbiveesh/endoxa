---
id: b_82dba2e832b3
slug: rope-starts-with-fixed
claim:
  kind: text
  text: >-
    The correct implementation of `RopeSliceExt::starts_with` slices the first `text.len()`
    bytes (not `len - text.len()`) and compares to the input `text`.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2024-09-21T12:00:00Z
  valid_time:
    start: 2024-09-21
    end: 2999-01-01
  source:
    kind: conversation
    session: s-rope-fix-2024
    turn: 2
  refs:
    - helix-stdx/src/rope.rs:51-55
    - git:Fix Rope.starts_with. (#11739)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_c68122b2bd6c   # rope-starts-with-wrong-slice
  - kind: attacks
    target: b_c68122b2bd6c   # rope-starts-with-wrong-slice
  - kind: supersedes
    target: b_c68122b2bd6c   # rope-starts-with-wrong-slice
coord: null
---

helix-stdx/src/rope.rs after fix: `self.get_byte_slice(..text.len()).map_or(false, |start| start == text)`. The maintainer added unit tests for both `starts_with` and `ends_with` in the same commit.
