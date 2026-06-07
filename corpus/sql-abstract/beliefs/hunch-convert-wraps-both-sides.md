---
id: b_abe26f1b5515
slug: hunch-convert-wraps-both-sides
claim:
  kind: text
  text: >-
    The `convert` option wraps both the column identifier AND the bind value in the
    specified SQL function, enabling symmetric case-insensitive comparisons.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 5
  refs:
    - lib/SQL/Abstract.pm:789-831
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.65
edges: []
coord: null
---

The `convert` option is applied to both sides via `_expand_select_clause_where` at line 789-831, which clones the SQLA object and wraps both `bind` and `ident`/`value` expanders with the conversion function. This means `convert => 'upper'` generates `WHERE UPPER(col) = UPPER(?)`, not just `UPPER(col) = ?`.
