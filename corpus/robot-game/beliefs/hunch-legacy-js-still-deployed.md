---
id: b_525181cffd18
slug: hunch-legacy-js-still-deployed
claim:
  kind: text
  text: >-
    The vanilla-JS prototype is likely still deployed at rabbiveesh.github.io/robot-game
    even after the macroquad migration, since the CI workflow deploys from `main` and the
    migration is on a separate branch.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-04-26T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s4-macroquad
    turn: 1
  refs:
    - CLAUDE.md:7-8
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.6
edges: []
coord: null
---

CLAUDE.md states 'main branch: Vanilla-JS prototype (still deployed at https://rabbiveesh.github.io/robot-game/ until macroquad migration lands)'. The macroquad migration appears to be a branch, not yet merged to main. This is a hunch based on the branch naming in CLAUDE.md.
