---
id: b_e33cef7c70b9
slug: lsp-started-with-stdio
claim:
  kind: text
  text: >-
    PerlNavigator is always started with a single `--stdio` argument and no special
    environment variables.
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
    turn: 4
  refs:
    - src/perl.rs:111-116
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.65
  asserted: 0.75
edges: []
coord: null
---

`language_server_command` returns `zed::Command { command: <path>, args: vec!["--stdio".to_string()], env: Default::default() }`. No env injection occurs.
