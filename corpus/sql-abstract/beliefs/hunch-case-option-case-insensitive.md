---
id: b_444a78ee5665
slug: hunch-case-option-case-insensitive
claim:
  kind: text
  text: >-
    The `case => 'lower'` constructor option likely causes ALL SQL keywords to be
    lowercased, not just a subset.
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
    turn: 12
  refs:
    - lib/SQL/Abstract.pm:2046-2050
    - lib/SQL/Abstract.pm:314-315
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.6
edges: []
coord: null
---

_sqlcase at line 2046-2050 does: `return $_[0]->{case} ? $_[1] : uc($_[1])`. When case is set (to 'lower'), it passes the value through unchanged. But values are passed in lowercase already (e.g., 'insert into'). So setting case => 'lower' effectively outputs lowercase keywords.
