---
id: b_56ed4bbd9d3e
slug: reduced-literal-sql-forms
claim:
  kind: text
  text: >-
    There are two forms of literal SQL: bare scalarref `\$sql` (no bind, no injection
    protection) and ref-to-arrayref `\[$sql, @bind]` (with bind values). Both bypass the
    injection_guard. The `-value` and `-ident` operators are the safe alternatives for
    column comparisons.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2008-refine
    turn: 14
  refs:
    - lib/SQL/Abstract.pm:3344-3350
  derived_from:
    - b_8c2c89a51612
    - b_d40baf013cd2
    - b_401e4beb83d2
confidence:
  directness: reduced
  observation_count: 2
  source_weight: 0.93
  asserted: null
edges:
  - kind: derived_from
    target: b_8c2c89a51612   # scalarref-literal-sql
  - kind: derived_from
    target: b_d40baf013cd2   # double-ref-literal-with-bind
  - kind: derived_from
    target: b_401e4beb83d2   # injection-guard-checks
coord: null
---

Combining scalarref-literal-sql and double-ref-literal-with-bind observations with the injection_guard belief. The CAVEAT in POD at line 3344-3350 explicitly warns about untrusted input.
