---
id: b_c23cd38d4411
slug: calver-versioning
claim:
  kind: text
  text: >-
    The project uses CalVer (YY.MM.0) since at least version 23.03. The workspace version at
    HEAD is 24.7.0.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 1
  refs:
    - Cargo.toml:37
    - git:Bump the version to 23.05
    - git:Add changelog notes for 24.07 (#10731)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.63
  asserted: 0.72
edges: []
coord: null
---

Cargo.toml `[workspace.package]` declares `version = "24.7.0"`. Git log shows commits like 'Bump the version to 23.05' (2023-04-05) and 'Add changelog notes for 24.07' (2024-07-14), confirming the CalVer convention.
