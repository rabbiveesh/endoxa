---
id: b_7e117ad7c848
slug: entry-point-main-rs
claim:
  kind: text
  text: >-
    The game entry point is `robot-buddy-game/src/main.rs`, a thin macroquad shim that
    captures `FrameInput`, calls `Game::step`, then `Game::render`, and awaits `next_frame`.
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
    - robot-buddy-game/src/main.rs:1-29
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.7
edges: []
coord: null
---

main.rs is 29 lines: it seeds the RNG from `macroquad::rand::rand()`, constructs `Game::new(seed)`, then loops `FrameInput::capture → step → render → next_frame`.
