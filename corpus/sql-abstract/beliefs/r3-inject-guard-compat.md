---
id: b_c583d8d49759
slug: r3-inject-guard-compat
claim:
  kind: text
  text: >-
    Guarding against parentheses in column names inside injection_guard would break many
    existing user hacks and is therefore not safe to add.
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
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:348
    - git:b6251592f2832d5353d37bd05e522091f20ff38f
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: 0.7
edges:
  - kind: attacks
    target: b_9c2c0b359201   # r3-inject-guard-complete
coord: null
---

The same FIXME comment that calls for paren-checking ends with 'but this will break tons of hacks... ideas anyone?' This captures the opposing force: SQLA has a documented tradition of permitting raw expressions as column-name-like values (function calls, aliased expressions, composite expressions) in contexts where quoting is disabled. Adding `()` to the injection guard regex would silently reject such patterns, causing regressions for any user relying on pass-through behavior. As of HEAD (2026), the FIXME remains unresolved and the paren check is absent — neither side has been adjudicated. This directly ATTACKS [[r3-inject-guard-complete]].
