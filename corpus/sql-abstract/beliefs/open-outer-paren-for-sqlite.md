---
id: b_7ebaade789f3
slug: open-outer-paren-for-sqlite
claim:
  kind: text
  text: >-
    When a scalar-ref (literal SQL) is passed to `-in`, the module strips outer parentheses
    before re-wrapping them to avoid double-parens that confuse SQLite (`col IN ( (SELECT
    ...) )` vs `col IN (SELECT ...)`).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2010-12-21T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2010-undoc-nest
    turn: 5
  refs:
    - lib/SQL/Abstract.pm:1800-1829
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

_open_outer_paren at line 1803-1829 strips outer parens from the literal SQL. The comment says 'Some databases (SQLite) treat col IN (1, 2) different from col IN ( (1, 2) )'. Uses Text::Balanced for nested paren detection.
