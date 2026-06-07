---
id: b_5db8a5828b9d
slug: project-scope-v3-pure-rust
kind: project-scope
claim:
  kind: text
  text: >-
    From 2026-04-26 onward, the project is pure Rust: domain crate + macroquad game crate,
    single WASM binary, no JS, no adapter, no JSON boundary at runtime.
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
    - git:Macroquad migration: rewrite presentation in Rust, delete all JS (#15)
    - CLAUDE.md:37
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_059fd705a763   # project-scope-v2-rust-wasm-hybrid
coord: null
---

Commit ac48738 ('Macroquad migration: rewrite presentation in Rust, delete all JS') landed on 2026-04-26. CLAUDE.md confirms 'One language: Rust'. The build produces one .wasm file served with macroquad's mq_js_bundle.js loader.
