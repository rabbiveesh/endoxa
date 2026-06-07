---
id: b_d4d72fcd05dc
slug: hunch-oniguruma-necessary
claim:
  kind: text
  text: >-
    The oniguruma copy hack was probably necessary to make the npm-installed perlnavigator-
    server actually start, since vscode-oniguruma would not resolve in the nested package
    layout.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-02
    turn: 7
  refs:
    - git:feat: setup perlnavigator via npm (needs help tho)
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

The npm approach installed `perlnavigator-server` which depends on `vscode-oniguruma`; it seemed the nested node resolution was failing and the hack was the only fix. But the whole npm approach was abandoned, so this is moot.
