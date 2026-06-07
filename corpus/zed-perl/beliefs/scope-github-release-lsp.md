---
id: b_cd133049e928
slug: scope-github-release-lsp
kind: project-scope
claim:
  kind: text
  text: >-
    From 2024-09-08 (88471b7) onward, the extension downloads PerlNavigator from GitHub
    binary releases instead of npm, using
    `zed::latest_github_release("bscan/PerlNavigator")`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time:
    start: 2024-09-08
    end: 2999-01-01
  source:
    kind: conversation
    session: seed-explore-02
    turn: 9
  refs:
    - git:feat: switch to github release until the next npm version is out
    - src/perl.rs:29-35
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_b65968462786   # scope-npm-based-lsp
coord: null
---

Commit 88471b7 replaced the npm approach entirely with GitHub release download logic. The version directory and cached binary path pattern used today was established here.
