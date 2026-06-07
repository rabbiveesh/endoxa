---
id: b_e5df132cd5b8
slug: r3-diagnostics-per-view-handler-b
claim:
  kind: text
  text: >-
    Each View currently carries its own DiagnosticsHandler because a global handler would
    require an entity-component refactor of View and Document that is too large to do now
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-session
    turn: 4
  refs:
    - helix-view/src/view.rs:150
    - git:7283ef881
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.75
  asserted: null
edges:
  - kind: attacks
    target: b_5a5399360d05   # r3-diagnostics-global-handler-a
coord: null
---

The same HACKS block (helix-view/src/view.rs:150-156) explains why the global approach has not been implemented: "For that we would need access to editor everywhere (we want to use the positioning code) so this can only happen by refactoring View and Document into entity component like structure. That is a huge refactor left to future work. For now we treat all views as focused and give them each their own handler." The per-view `diagnostics_handler` field therefore exists as an acknowledged wrong approach. This attacks [[r3-diagnostics-global-handler-a]] but is not adjudicated — the conflict is open and unresolved.
