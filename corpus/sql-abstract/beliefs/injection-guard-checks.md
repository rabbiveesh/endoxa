---
id: b_401e4beb83d2
slug: injection-guard-checks
claim:
  kind: text
  text: >-
    SQL::Abstract has an injection_guard check that throws an exception if a function name
    or unquoted column name matches a regex containing semicolons or `GO` keywords.
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
    turn: 4
  refs:
    - lib/SQL/Abstract.pm:507-514
    - lib/SQL/Abstract.pm:351-355
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_assert_pass_injection_guard at line 507-514 checks `$_[1] =~ $_[0]->{injection_guard}`. The default regex checks for `;` and `^\s*go\s`. This is NOT applied to literal SQL (\$sql or \[$sql, @bind]).
