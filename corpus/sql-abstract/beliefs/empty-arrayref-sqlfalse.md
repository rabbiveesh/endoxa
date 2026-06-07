---
id: b_08212d3f3b67
slug: empty-arrayref-sqlfalse
claim:
  kind: text
  text: >-
    An empty arrayref value `{ col => [] }` generates the SQL false literal (default `0=1`),
    not broken SQL or an error.
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
    turn: 3
  refs:
    - lib/SQL/Abstract.pm:1099
    - git:fix for key => [] + tests + cleanup of 02where.t
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

Code at lib/SQL/Abstract.pm:1099 returns `$self->sqlfalse unless @$v`. Before commit 8a68b5b (2008-06-12) this generated broken SQL. The fix changed it to DTRT. sqltrue/sqlfalse defaults are '1=1' and '0=1' but are configurable in new().
