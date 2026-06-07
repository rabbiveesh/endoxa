---
id: b_de13663b1e7d
slug: rule-no-special-casing
claim:
  kind: text
  text: >-
    HARD-WON RULE: never special-case for a particular shape — method-name allowlists, base-
    class equality checks, real-vs-synthetic branches, per-name lookup tables in core — you
    are ALWAYS wrong, because the enumerated list is always incomplete. Encode the 'wants
    behavior X' property on the type/value/witness itself so consumers ask the value, never
    the shape.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-15T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: frameworks-2026-03-15
    turn: 4
  refs:
    - CLAUDE.md
    - docs/adr/parametric-types.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges: []
coord: null
---

The load-bearing design discipline of the whole codebase. The special case is always the smallest diff now and never stays cheap. Cited repeatedly across ADRs; the parametric-types work is its canonical application — see [[parametric-types-resultset]].
