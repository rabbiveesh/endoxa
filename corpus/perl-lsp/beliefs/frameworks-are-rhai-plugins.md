---
id: b_429dadb7e809
slug: frameworks-are-rhai-plugins
claim:
  kind: text
  text: >-
    Framework intelligence ships as bundled Rhai plugins with EMIT hooks (on_use /
    on_function_call / on_method_call — parse-time, declarative, return Vec<EmitAction>) and
    QUERY hooks (on_signature_help / on_completion — cursor-time, imperative). Plugin
    sources are fingerprinted, so editing one invalidates the cross-file cache.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-04-27T12:00:00Z
  valid_time:
    start: 2026-04-27
    end: 2999-01-01
  source:
    kind: conversation
    session: rhai-plugins-2026-04-27
    turn: 6
  refs:
    - docs/adr/plugin-system.md
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_35104ff7112c   # frameworks-were-hardcoded
coord: null
---

Supersedes [[frameworks-were-hardcoded]]. 'Silent'/'exclusive' answers let a plugin suppress native paths it knows will mishandle a slot.
