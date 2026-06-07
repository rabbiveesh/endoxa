---
id: b_2e6b88246b8f
slug: language-config-deny-unknown
claim:
  kind: text
  text: >-
    LanguageConfiguration structs are deserialized with `deny_unknown_fields`, so adding an
    unrecognized key to a `[[language]]` block in languages.toml causes a hard config parse
    error.
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
    turn: 5
  refs:
    - helix-core/src/syntax.rs:93-94
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

syntax.rs line 93: `#[serde(rename_all = "kebab-case", deny_unknown_fields)]` on LanguageConfiguration. This means typos in user-provided language configs fail loudly rather than silently.
