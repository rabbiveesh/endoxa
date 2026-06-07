---
id: b_55b643e0d3f2
slug: path-first-binary-resolution
claim:
  kind: text
  text: >-
    Before downloading PerlNavigator from GitHub, the extension checks if `perlnavigator` is
    already on the user's PATH via `worktree.which()`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time:
    start: 2024-09-19
    end: 2999-01-01
  source:
    kind: conversation
    session: seed-explore-03
    turn: 1
  refs:
    - src/perl.rs:15-17
    - git:feat(perlnavigator): use locally installed version or else download from github
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges:
  - kind: supersedes
    target: b_8416f5315053   # old-always-downloads
coord: null
---

Lines 15-17 of src/perl.rs call `worktree.which("perlnavigator")` and return immediately if found. This was added in commit 011deca, which previously always attempted to download.
