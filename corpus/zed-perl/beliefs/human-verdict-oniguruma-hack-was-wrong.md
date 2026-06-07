---
id: b_2db2f059bbcf
slug: human-verdict-oniguruma-hack-was-wrong
claim:
  kind: text
  text: >-
    The vscode-oniguruma directory-copy hack (move_oniguruma_super_evil) was an incorrect
    workaround that the maintainer explicitly labeled evil and removed.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-02
    turn: 6
  refs:
    - git:fix: remove evil oniguruma hack
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_d4d72fcd05dc   # hunch-oniguruma-necessary
  - kind: attacks
    target: b_d4d72fcd05dc   # hunch-oniguruma-necessary
coord: null
---

Commit bc94c30 ('fix: remove evil oniguruma hack') deleted the `copy_dir_all` helper and `move_oniguruma_super_evil` function. The npm-based approach tried to fix a dependency resolution issue by physically copying vscode-oniguruma into perlnavigator-server's nested node_modules — a side-effect hack that was abandoned when the approach switched to GitHub binary releases.
