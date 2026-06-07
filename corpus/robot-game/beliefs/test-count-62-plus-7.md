---
id: b_7abffedade84
slug: test-count-62-plus-7
claim:
  kind: text
  text: >-
    The test suite has 62 domain unit tests and 7 game integration tests, running as plain
    `cargo test` with no window or browser.
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
    turn: 2
  refs:
    - CLAUDE.md:39
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.72
  asserted: 0.75
edges: []
coord: null
---

CLAUDE.md: 'Tests: cargo test runs 62 domain unit tests + 7 game integration tests (headless story-style — see ADR-002).' ADR-002 describes the Game::step/render split that enables headless testing.
