---
id: b_bc815199181c
slug: voice-answers-bypassed-reducer
claim:
  kind: text
  text: >-
    An agent initially implemented voice answer submission by calling `recordResult`
    directly, bypassing the adapter's reducer and making voice practice invisible to the
    adaptive system.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 3
  refs:
    - git:Fix voice input: route through adapter reducer + add confirmation tier
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

Commit 5e9acc0 ('Fix voice input: route through adapter reducer') describes exactly this bug: 'Voice answers were calling recordResult directly, bypassing the adapter's reducer.' The fix exposed `window._submitVoiceAnswer` through the adapter.
