---
id: b_367449c2213d
slug: asset-name-no-zip-suffix-then-searched-with-zip
claim:
  kind: text
  text: >-
    The `asset_name` variable is constructed WITHOUT the `.zip` suffix, but the asset lookup
    searches for `{asset_name}.zip`; the binary path inside the zip uses the bare asset_name
    as a subdirectory.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-03
    turn: 4
  refs:
    - src/perl.rs:38-58
    - git:feat(perlnavigator): use locally installed version or else download from github
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

This was a bug in commit 88471b7 where `asset_name` included `.zip` and the find query was exact equality — the path also wrongly used `perlnavigator` at root of version_dir. Fixed in 011deca: asset_name drops the suffix, find appends `.zip`, and binary_path includes `{asset_name}/` subdirectory.
