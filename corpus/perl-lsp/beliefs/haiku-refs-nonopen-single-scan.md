---
id: b_57eaa985ff0a
slug: haiku-refs-nonopen-single-scan
claim:
  kind: text
  text: >-
    In refs_to with RoleMask::VISIBLE, a workspace module that is NOT open has
    collect_from_analysis called exactly once (the DEPENDENCY phase only).
author:
  kind: agent
  id: claude-haiku-4-5
  model: claude-haiku-4-5
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 4
  refs:
    - src/resolve.rs:191-208
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.95
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.95. Wrong about the phase interaction.
