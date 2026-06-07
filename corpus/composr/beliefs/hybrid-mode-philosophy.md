---
id: b_d82752b2b23b
slug: hybrid-mode-philosophy
claim:
  kind: text
  text: >-
    Hybrid mode: composr does NOT reimplement composer's EventDispatcher. When a script
    event contains a class-method handler (Foo\Bar::baz), it shells out to `composer run-
    script <event>` for that one event — correctness over coverage.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: hybrid-2026-05-07
    turn: 3
  refs:
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The principled boundary of the native reimplementation. Composer-bin resolution order: --composer-bin, COMPOSR_COMPOSER, composer on PATH.
