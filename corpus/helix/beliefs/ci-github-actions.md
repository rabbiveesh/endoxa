---
id: b_033064aac303
slug: ci-github-actions
claim:
  kind: text
  text: >-
    CI runs on GitHub Actions; workflows are defined in .github/workflows/.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 1
  refs:
    - .github/workflows/build.yml
    - README.md:11
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.65
edges: []
coord: null
---

The .github/workflows/ directory contains build.yml, release.yml, gh-pages.yml, cachix.yml, and a languages.toml validation workflow. The README badge links to the build.yml action.
