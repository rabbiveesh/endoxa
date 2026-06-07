---
id: b_9adcb230781b
slug: scope-dumpr
kind: project-scope
claim:
  kind: text
  text: >-
    composr was born (as 'autoload-dumpr') with a tiny scope: a Rust replacement for ONLY
    `composer dump-autoload -o` — rewrite vendor/composer/autoload_classmap.php and patch
    autoload_static.php, byte-equivalent to composer. Nothing else.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-05T12:00:00Z
  valid_time:
    start: 2026-05-05
    end: 2026-05-07
  source:
    kind: conversation
    session: autoload-dumpr-2026-05-05
    turn: 1
  refs:
    - git:5981358
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The seed. Even at birth the correctness bar is byte-equivalence; that never changes even as scope explodes.
