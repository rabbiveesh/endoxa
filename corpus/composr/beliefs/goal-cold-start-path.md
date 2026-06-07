---
id: b_9ab49e504c89
slug: goal-cold-start-path
claim:
  kind: text
  text: >-
    composr deliberately targets the COLD-START install path — the dist-download + autoload-
    bootstrap + Laravel package:discover that composer spends ~30-70s on — not a general
    composer reimplementation. It speeds up the slow parts and defers the rest.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-05T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: autoload-dumpr-2026-05-05
    turn: 3
  refs:
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

The scoping principle that keeps the project tractable. package:discover alone drops from ~30s to ~5ms.
