---
id: b_d702fc50dd14
slug: pest-plugin-is-inert
claim:
  kind: text
  text: >-
    pest-plugin is an INERT composer-plugin — installing its files is enough; its composer-
    side install hook has no observable install-time effect, so composr can skip it.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 13
  refs:
    - README.md
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.8
edges: []
coord: null
---

A confident classification (asserted 0.8): it looked like just another library. The failure mode is silent, which is why the hunch survived until someone checked Pest actually ran.
