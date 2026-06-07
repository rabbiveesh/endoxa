---
id: b_25a48ee6ca91
slug: config-dir-location
claim:
  kind: text
  text: >-
    User configuration is loaded from a platform-specific config directory via
    `helix_loader::initialize_config_file`; there is no hard-coded path like
    `~/.config/helix` in main.rs.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 2
  refs:
    - helix-term/src/main.rs:81-82
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.7
edges: []
coord: null
---

main.rs calls `helix_loader::initialize_config_file(args.config_file.clone())` and `helix_loader::initialize_log_file`. The actual path resolution is delegated to helix-loader, which uses OS conventions.
