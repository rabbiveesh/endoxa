---
id: b_059fd705a763
slug: project-scope-v2-rust-wasm-hybrid
kind: project-scope
claim:
  kind: text
  text: >-
    From 2026-03-29 through 2026-04-25, the project was a Rust+WASM hybrid: domain logic in
    Rust compiled to WASM, presentation still in JS via an adapter bridge.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time:
    start: 2026-03-29
    end: 2026-04-26
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 1
  refs:
    - git:Begin Rust+WASM migration — rolling_window and operation_stats
    - git:Delete JS domain — Rust+WASM is the only domain now
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_7a93dc431b27   # project-scope-v1-js-prototype
coord: null
---

Commit b38c635 began porting domain to Rust on 2026-03-29. Commit c693786 deleted the JS domain. The adapter.js bridge serialized events as JSON across the WASM boundary. This phase ended when macroquad migration (ac48738) deleted all JS on 2026-04-26.
