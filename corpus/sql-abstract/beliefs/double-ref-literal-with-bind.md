---
id: b_d40baf013cd2
slug: double-ref-literal-with-bind
claim:
  kind: text
  text: >-
    A reference-to-arrayref (\[\$sql, @bind]) embeds literal SQL with placeholders: `{ col
    => \["= func(?)", $val] }` yields `col = func(?)` with $val bound.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2008-refine
    turn: 5
  refs:
    - lib/SQL/Abstract.pm:72-76
    - lib/SQL/Abstract.pm:3352-3380
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supports
    target: b_8c2c89a51612   # scalarref-literal-sql
coord: null
---

is_literal_value() at line 73-74 checks: `ref $_[0] eq 'REF' and ref ${$_[0]} eq 'ARRAY'`. The bind values must match the current bindtype format — if bindtype is 'columns', they must be [colname, value] pairs. A major gotcha.
