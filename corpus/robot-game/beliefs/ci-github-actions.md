---
id: b_d486af78d2f9
slug: ci-github-actions
claim:
  kind: text
  text: >-
    CI runs on GitHub Actions: `cargo test`, WASM build, and deploy to GitHub Pages. No
    Node, no npm in CI.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-24T12:00:00Z
  valid_time:
    start: 2026-03-24
    end: 2999-01-01
  source:
    kind: conversation
    session: s1-init
    turn: 1
  refs:
    - .github/workflows/deploy.yml:1-50
    - CLAUDE.md:41
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.63
edges: []
coord: null
---

The workflow at .github/workflows/*.yml uses `dtolnay/rust-toolchain@stable` with `wasm32-unknown-unknown`, runs `cargo test`, then `./build-wasm.sh`, and deploys the assembled `_site/` directory to GitHub Pages.
