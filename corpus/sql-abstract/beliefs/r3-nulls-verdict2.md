---
id: b_d7ced4649dbc
slug: r3-nulls-verdict2
claim:
  kind: text
  text: >-
    Despite being ISO/ANSI SQL, NULLS FIRST/LAST is not supported by enough databases to
    include in the default SQLA behavior; the feature should be reverted.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-s1
    turn: 3
  refs:
    - lib/SQL/Abstract.pm:1150
    - git:5e4361304e44378efe29ad97c4430cd5f5c0f1ba
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: adjudicates
    target: b_61a82c08adb5   # r3-nulls-verdict1
  - kind: attacks
    target: b_61a82c08adb5   # r3-nulls-verdict1
coord: null
---

Commit 5e43613 (Dagfinn Ilmari Mannsåker, 2013-04-14) reverted both 2266ca5 and b137b07 with the explicit message: 'Despite being an ISO/ANSI SQL feature, it's apparently not supported by enough databases to have by default.' This overturns [[r3-nulls-verdict1]]: the correctness of a standard does not mandate inclusion in a portability-focused abstraction. The revert restores `_order_by_chunks` to its pre-feature state and removes all `-nulls` tests. As of the current HEAD, NULLS FIRST/LAST remains absent from the codebase.
