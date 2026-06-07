---
id: b_fea9d6138d92
slug: undef-becomes-is-null
claim:
  kind: text
  text: >-
    A hashref value of undef generates `IS NULL`: `{ col => undef }` yields `col IS NULL`.
    Similarly `{ col => {'!=', undef} }` yields `col IS NOT NULL`.
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
    turn: 6
  refs:
    - lib/SQL/Abstract.pm:78-83
    - lib/SQL/Abstract.pm:1080-1082
    - lib/SQL/Abstract.pm:1289-1298
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.97
edges: []
coord: null
---

is_undef_value() at line 78-83 detects both bare undef and `{ -value => undef }`. _expand_hashpair_ident at 1080-1082 redirects to _expand_hashpair_cmp. _dwim_op_to_is() maps equality ops to 'is' and inequality to 'is not'.
