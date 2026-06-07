---
id: b_9c2c0b359201
slug: r3-inject-guard-complete
claim:
  kind: text
  text: >-
    The injection_guard should detect parentheses in column names to prevent SQL injection
    via crafted column-name strings.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-s2
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:348
    - git:b6251592f2832d5353d37bd05e522091f20ff38f
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.65
  asserted: 0.7
edges: []
coord: null
---

In commit b625159 (ribasushi, 2010-12-21), which introduced `injection_guard`, the author added a FIXME comment directly in the implementation: '# FIXME / # need to guard against ()'s in column names too'. The comment acknowledges that unquoted column names could contain parentheses and that this opens an injection surface. The guard regex in lib/SQL/Abstract.pm currently only checks for `;` and leading `GO`, leaving `()` uncovered. From a security completeness standpoint this is a gap.
