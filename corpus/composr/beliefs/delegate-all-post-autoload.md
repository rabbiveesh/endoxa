---
id: b_5cf8a5c93a05
slug: delegate-all-post-autoload
claim:
  kind: text
  text: >-
    The whole post-autoload-dump lifecycle event (including Laravel's package:discover) is
    handled by delegating to a `composer run-script` subprocess.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time:
    start: 2026-05-07
    end: 2026-05-07
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 5
  refs:
    - git:Hybrid mode
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.65
  asserted: 0.7
edges: []
coord: null
---

The initial hybrid-mode stance. Quickly narrowed the same day once discover + clearCompiled were handled natively. Kept for audit of the early plan.
