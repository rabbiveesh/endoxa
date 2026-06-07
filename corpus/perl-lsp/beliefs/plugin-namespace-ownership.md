---
id: b_d190c79522b6
slug: plugin-namespace-ownership
claim:
  kind: text
  text: >-
    Plugin-synthesized content is owned by PluginNamespace, NOT Perl classes; cross-file
    lookup goes through ModuleIndex::for_each_entity_bridged_to(class, ...) — no parallel
    reverse indexes (the retired class_content_index / modules_with_class_content were
    exactly that mistake).
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-04-27T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: rhai-plugins-2026-04-27
    turn: 8
  refs:
    - docs/adr/plugin-system.md
    - CLAUDE.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges:
  - kind: refines
    target: b_429dadb7e809   # frameworks-are-rhai-plugins
coord: null
---

Refines [[frameworks-are-rhai-plugins]]. An instance of [[rule-no-special-casing]]: the bridge is generic, not per-plugin.
