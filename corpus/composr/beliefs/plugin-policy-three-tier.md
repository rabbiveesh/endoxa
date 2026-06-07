---
id: b_029dbc660e47
slug: plugin-policy-three-tier
claim:
  kind: text
  text: >-
    composr classifies composer-plugins three ways: NATIVELY-REPLICATED (pest-plugin,
    tbachert/spi — byte-equal codegen ported to Rust, gated on config.allow-plugins), INERT
    (install files only — php-http/discovery, pest sub-plugins, phpstan/extension-
    installer), and UNKNOWN (force a per-event `composer run-script` delegation). `--strict-
    plugins` refuses any plugin in the lock.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time:
    start: 2026-05-10
    end: 2999-01-01
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 3
  refs:
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_9edb8556eb6e   # plugins-all-delegated
coord: null
---

Supersedes [[plugins-all-delegated]]. pest-plugin writes vendor/pest-plugins.json (without it every Pest plugin silently no-ops); spi writes GeneratedServiceProviderData.php. Project-local inert allowlists live in composr.json -> allow-inert-plugins (supports `*` wildcards).
