---
id: b_70f6a1cdb789
slug: rename-dumpr-to-composr
claim:
  kind: text
  text: >-
    When it grew past dump-autoload the tool was renamed autoload-dumpr -> composr ('vowels
    not included'); the bare `composr` command stays an alias for `dump-autoload` for back-
    compat, and the autoload-dumpr binary alias was dropped at 0.3.0.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 11
  refs:
    - git:Rename to composr
    - git:Drop autoload-dumpr binary alias, bump to 0.3.0
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

The rename tracks the scope jump in [[scope-installer]] — a name change IS a scope signal.
