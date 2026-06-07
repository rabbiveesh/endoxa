---
id: b_d879b9a9448c
slug: extension-version-bump-history
claim:
  kind: text
  text: >-
    The extension version in extension.toml was bumped from 0.1.0 to 0.1.1 in the last
    commit (f0292b8), but Cargo.toml still reads 0.1.0.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2025-02-10T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-05
    turn: 1
  refs:
    - extension.toml:4
    - Cargo.toml:3
    - git:docs: perlnavigator instuctions + bump
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.75
  asserted: 0.8
edges: []
coord: null
---

extension.toml shows `version = '0.1.1'` while Cargo.toml shows `version = "0.1.0"`. The two version numbers are out of sync. Commit f0292b8 only changed README.md and extension.toml.
