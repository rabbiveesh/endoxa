---
id: b_e041937192c0
slug: entry-point-is-sqla-pm
claim:
  kind: text
  text: >-
    The sole public API entry point is lib/SQL/Abstract.pm; all other files under lib/ are
    helpers, formatters, or test utilities.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2007-02-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2007-import
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:1
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.65
edges: []
coord: null
---

lib/SQL/Abstract.pm is 4057 lines and contains all four public SQL-generating methods plus the extension system. Sibling files are SQL::Abstract::Formatter, ::Parts, ::Test, ::Tree, ::Reference, ::Role/*, ::Plugin/*.
