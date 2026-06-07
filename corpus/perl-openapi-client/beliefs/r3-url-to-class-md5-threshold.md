---
id: b_f2c9d7a2a021
slug: r3-url-to-class-md5-threshold
claim:
  kind: text
  text: >-
    The URL→class-name conversion uses an MD5 hash only for spec URLs longer than 110
    characters; shorter URLs are sanitized directly to Perl identifier form.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 2
  refs:
    - lib/OpenAPI/Client.pm:237
    - git:275bf6b
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.55
  asserted: 0.6
edges: []
coord: null
---

At write time (2017-08-18, commit 275bf6b), the `_url_to_class` sub used `length $package > 110` with a comment 'a bit random'. This heuristic was inferred as deliberate pragmatism — Perl package names have no formal length limit but very long names cause practical problems (file system paths, stack traces). The 110 threshold was already present in the Swagger2::Client ancestor (where the original was 40, also annotated 'a bit random'). The 275bf6b conversion bumped it to 110, suggesting the author consciously revisited it. It has survived all 130 commits unchanged. Checkable: lib/OpenAPI/Client.pm line 237 `if length $package > 110`.
