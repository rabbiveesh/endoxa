---
id: b_4108e6d5e8d5
slug: ci-github-actions
claim:
  kind: text
  text: >-
    CI runs via GitHub Actions on ubuntu-latest and windows-latest for Perl 5.16, 5.26, and
    5.32.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2022-06-03T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s4-inheritance-fix
    turn: 1
  refs:
    - .github/workflows/ci.yml:1-5
    - git:Add github workflow for testing
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.68
edges: []
coord: null
---

The workflow file at .github/workflows/ci.yml (added in commit 8b667ca) defines a matrix strategy over those OS/Perl combinations.
