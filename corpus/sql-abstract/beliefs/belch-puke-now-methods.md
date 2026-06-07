---
id: b_d78b3510101a
slug: belch-puke-now-methods
claim:
  kind: text
  text: >-
    Since August 2024 (commit be2ee60), `belch` and `puke` are method-style calls
    (`$self->belch(...)`) not plain function calls. Old code calling them as `belch(...)`
    still works due to the blessed-object check at the start.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-05T12:00:00Z
  valid_time:
    start: 2024-08-11
    end: 2999-01-01
  source:
    kind: conversation
    session: s-2024-fixes
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:60-70
    - git:modify belch and puke to operate as method calls
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

Commit be2ee60 added `Scalar::Util::blessed($_[0]) and $_[0]->isa(__PACKAGE__) and shift;` to both belch and puke. This was backported from SQL::Abstract::Classic. Allows subclasses to override error reporting.
