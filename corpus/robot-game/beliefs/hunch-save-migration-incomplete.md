---
id: b_bcbc25643f3c
slug: hunch-save-migration-incomplete
claim:
  kind: text
  text: >-
    The `migrate_legacy` method for saves might not handle all old field shapes — the
    `math_band` migration only runs if the saved band differs from 1, which would silently
    fail to upgrade a player who legitimately has math_band=1.
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
    turn: 4
  refs:
    - robot-buddy-game/src/save.rs:53-59
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.45
  asserted: 0.55
edges: []
coord: null
---

save.rs: `if self.profile.math_band == 1 && band != 1 { self.profile.math_band = band; }` — if an old save had math_band:1 explicitly set and profile.math_band defaulted to 1, the migration would be a no-op even if needed. This is a low-confidence architectural hunch.
