---
id: b_218b5b2a99c0
slug: scope-installer
kind: project-scope
claim:
  kind: text
  text: >-
    Scope EXPLODED in a single day (2026-05-07) into a full `composer install` replacement:
    lock-driven parallel download -> extract -> native autoload bootstrap -> scripts, with
    path-type packages, hybrid mode, and native Laravel package:discover. Renamed autoload-
    dumpr -> composr.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time:
    start: 2026-05-07
    end: 2026-05-10
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 1
  refs:
    - git:9a2f252
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_9adcb230781b   # scope-dumpr
coord: null
---

Supersedes [[scope-dumpr]]. The most dramatic single-day scope jump in the corpus — a one-command tool became an end-to-end installer. See [[rename-dumpr-to-composr]].
