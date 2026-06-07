---
id: b_8c2c89a51612
slug: scalarref-literal-sql
claim:
  kind: text
  text: >-
    A scalar reference (\$sql) in a value position injects literal SQL without placeholders:
    `{ col => \"NOW()" }` yields `col = NOW()`.
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
    turn: 4
  refs:
    - lib/SQL/Abstract.pm:72-76
    - lib/SQL/Abstract.pm:1112-1126
    - lib/SQL/Abstract.pm:3326-3350
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.93
edges: []
coord: null
---

is_literal_value() checks for SCALAR ref. _expand_hashpair_ident at line 1112 handles the literal ref path. The double-ref syntax \[\$sql, @bind] adds bind values. This is a CAVEAT: untrusted input must never be used this way.
