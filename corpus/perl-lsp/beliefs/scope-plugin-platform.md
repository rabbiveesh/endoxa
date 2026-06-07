---
id: b_ab7038cc8458
slug: scope-plugin-platform
kind: project-scope
claim:
  kind: text
  text: >-
    Scope SHIFTED from 'an LSP with hardcoded frameworks' to 'an extensible analysis
    PLATFORM': framework intelligence became Rhai plugins (fingerprinted, user-droppable
    into $PERL_LSP_PLUGIN_DIR), and type inference was reorganized around a single canonical
    witness bag.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-04-27T12:00:00Z
  valid_time:
    start: 2026-04-27
    end: 2026-06-01
  source:
    kind: conversation
    session: rhai-plugins-2026-04-27
    turn: 1
  refs:
    - git:feat rhai plugins (#21)
    - docs/adr/plugin-system.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_271c40c6a09a   # scope-frameworks
coord: null
---

Supersedes [[scope-frameworks]]. This is the architectural pivot — extensibility becomes the product, not just the features.
