---
id: b_641d6775a012
slug: workspace-two-crates
kind: project-scope
claim:
  kind: text
  text: >-
    The Cargo workspace has exactly two members: `robot-buddy-domain` (pure Rust library)
    and `robot-buddy-game` (macroquad binary).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-04-26T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s4-macroquad
    turn: 1
  refs:
    - Cargo.toml:1-2
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.72
edges: []
coord: null
---

Cargo.toml at the workspace root lists `members = ["robot-buddy-domain", "robot-buddy-game"]`. Domain has no macroquad dependency; game depends on domain.
