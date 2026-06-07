---
id: b_5a5399360d05
slug: r3-diagnostics-global-handler-a
claim:
  kind: text
  text: >-
    Diagnostics display should use a single global handler; non-focused views should simply
    not apply cursor-line-specific rendering
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-session
    turn: 3
  refs:
    - helix-view/src/view.rs:150
    - git:7283ef881
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: null
edges: []
coord: null
---

The HACKS comment at helix-view/src/view.rs:150 (introduced in commit 7283ef881, 2024-04-05) states: "there should really only be a global diagnostics handler (the non-focused views should just not have different handling for the cursor line)." This describes the architecturally correct target state: one handler, globally accessible via the Editor, with views simply rendering without per-view state.
