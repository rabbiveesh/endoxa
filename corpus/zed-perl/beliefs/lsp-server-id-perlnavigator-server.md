---
id: b_efcd6bf9fe0f
slug: lsp-server-id-perlnavigator-server
claim:
  kind: text
  text: >-
    The language server is registered under the id 'perlnavigator-server' in both
    extension.toml and the Zed JSON settings key.
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
    turn: 3
  refs:
    - extension.toml:13-16
    - README.md:18-30
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges: []
coord: null
---

extension.toml defines `[language_servers.perlnavigator-server]`. The README JSON snippet shows `"lsp": { "perlnavigator-server": { ... } }` as the settings key users must use.
