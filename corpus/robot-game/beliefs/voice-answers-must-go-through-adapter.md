---
id: b_1eea6144c17d
slug: voice-answers-must-go-through-adapter
claim:
  kind: text
  text: >-
    Voice answer submission must go through the adapter reducer (`_submitVoiceAnswer`), not
    `recordResult` directly, so the adaptive system sees voice attempts as enriched events.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 4
  refs:
    - git:Fix voice input: route through adapter reducer + add confirmation tier
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.93
  asserted: null
edges:
  - kind: adjudicates
    target: b_bc815199181c   # voice-answers-bypassed-reducer
  - kind: attacks
    target: b_bc815199181c   # voice-answers-bypassed-reducer
coord: null
---

Commit 5e9acc0 established this rule after the bypass bug was found. The fix also added a three-tier confidence system: >0.8 auto-submit, 0.5-0.8 confirmation prompt, <0.5 retry.
