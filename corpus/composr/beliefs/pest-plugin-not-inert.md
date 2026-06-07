---
id: b_462cf8ec96ee
slug: pest-plugin-not-inert
claim:
  kind: text
  text: >-
    VERDICT: pest-plugin is NOT inert. Without writing vendor/pest-plugins.json, Pest's
    runtime Loader returns [] and EVERY Pest plugin (Coverage, Bail, Cache, Retry, Snapshot,
    Parallel, pest-plugin-arch, …) SILENTLY no-ops. composr had to natively replicate the
    codegen.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 11
  refs:
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_d702fc50dd14   # pest-plugin-is-inert
  - kind: attacks
    target: b_d702fc50dd14   # pest-plugin-is-inert
  - kind: supports
    target: b_029dbc660e47   # plugin-policy-three-tier
coord: null
---

Defeats [[pest-plugin-is-inert]]; this is exactly why the [[plugin-policy-three-tier]] split exists. The worst kind of wrong: a plugin classified inert that silently disables a whole test toolchain with no error.
