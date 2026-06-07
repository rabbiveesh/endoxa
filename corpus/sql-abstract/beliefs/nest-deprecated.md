---
id: b_6fc5518fe275
slug: nest-deprecated
claim:
  kind: text
  text: >-
    The `-nest` operator is deprecated and should not be used in new code. It was
    undocumented in 2010 and emits a warning when used.
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
    turn: 3
  refs:
    - lib/SQL/Abstract.pm:1538-1553
    - git:Undocument -nest with extreme prejudice
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_expand_nest at line 1538 checks `$self->{warn_once_on_nest}` (set when DBIC is the calling class) and emits a belch. Commit 48d9f5f (2010-12-21) formally undocumented it. The replacement pattern is `-and => [ \%cond0, \@cond1, ... ]`.
