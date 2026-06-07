---
id: b_308c05b538b0
slug: quote-char-array-for-different-delimiters
claim:
  kind: text
  text: >-
    The `quote_char` option accepts either a single character (used on both sides) or an
    arrayref of two characters [left, right] for databases like SQL Server that use
    `[column]` style quoting.
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
    turn: 6
  refs:
    - lib/SQL/Abstract.pm:1976-1982
    - lib/SQL/Abstract.pm:2575-2583
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_quote at line 1976-1980 handles both: `!$qref ? ($q, $q) : ($qref eq 'ARRAY') ? @{$_[0]->{quote_char}}`. The POD at line 2575-2583 gives the SQL Server example `['[',']']`.
