---
id: b_da27e2584a0a
slug: r-composr-trajectory
claim:
  kind: text
  text: >-
    composr's trajectory: a one-shot `dump-autoload -o` speedup -> a full lock-driven
    `composer install` replacement (native autoload + Laravel discover) -> native plugin
    replication -> a published 1.0 crate. The unchanging throughline: kill cold-start
    composer subprocess calls while staying byte-equivalent to composer.
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: release-2026-06-02
    turn: 20
  refs: []
  derived_from:
    - b_9adcb230781b
    - b_218b5b2a99c0
    - b_249ebc974aaf
    - b_d0d8020cdaca
confidence:
  directness: reduced
  observation_count: 4
  source_weight: 0.6
  asserted: null
edges:
  - kind: derived_from
    target: b_9adcb230781b   # scope-dumpr
  - kind: derived_from
    target: b_218b5b2a99c0   # scope-installer
  - kind: derived_from
    target: b_249ebc974aaf   # scope-plugin-replication
  - kind: derived_from
    target: b_d0d8020cdaca   # goal-byte-equivalence
coord: null
---

REDUCED over the scope chain plus the [[goal-byte-equivalence]] spine. The property a flat store loses: enormous scope growth (one command -> end-to-end installer in a day) held together by two fixed goals — cold-start speed and byte-equivalence.
