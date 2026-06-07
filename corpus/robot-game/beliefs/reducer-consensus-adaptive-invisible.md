---
id: b_deb9321898a2
slug: reducer-consensus-adaptive-invisible
kind: project-scope
claim:
  kind: text
  text: >-
    The adaptive learning system is architecturally invisible to the child: assessment
    happens through gameplay, difficulty adjusts silently, and nothing in the UI exposes
    bands, CRA stages, or frustration levels to the player.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-25T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s2-adaptive
    turn: 5
  refs:
    - CLAUDE.md:18-36
  derived_from:
    - b_d343c6e6c462
    - b_b261502ec67e
    - b_add813fdf745
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.92
  asserted: null
edges:
  - kind: derived_from
    target: b_d343c6e6c462   # no-labels-shown-to-kids
  - kind: derived_from
    target: b_b261502ec67e   # no-timer-on-challenges
  - kind: derived_from
    target: b_add813fdf745   # wrong-answers-natural-consequence
coord: null
---

Converged from: [[no-labels-shown-to-kids]], [[no-timer-on-challenges]], and [[wrong-answers-natural-consequence]]. Three independent architectural invariants all point to the same design principle: stealth assessment. The parent dashboard is the only sanctioned visibility surface.
