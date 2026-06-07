---
id: b_25a58b73e6a2
slug: project-scope-v2-aqt-arch
kind: project-scope
claim:
  kind: text
  text: >-
    From 2019 onward, SQL::Abstract adopted an internal expr->AQT (Abstract Query Tree) two-
    phase architecture: expand_expr() produces a normalized AQT, render_aqt() turns it into
    SQL+bind.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time:
    start: 2019-01-01
    end: 2999-01-01
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:922-944
    - lib/SQL/Abstract/Reference.pm:1-25
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supports
    target: b_70282753eb70   # expander-renderer-extension
coord: null
---

The v2 rewrite (committed across 2018-2021) introduced expand_clause, render_clause, expander/renderer registration. SQL::Abstract::Reference documents the AQT node types. The public methods still work but now delegate through this two-phase pipeline.
