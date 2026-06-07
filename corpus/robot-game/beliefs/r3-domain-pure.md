---
id: b_583bdb9e575b
slug: r3-domain-pure
claim:
  kind: text
  text: >-
    The domain crate is and will remain browser-agnostic: pure reducers, no IO, no
    macroquad, no extern C
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2026-03-25
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 1
  refs:
    - docs/architecture-spec.md
    - robot-buddy-domain/src/learning/learner_profile.rs:177
    - git:454ee9fddfe234cd8efa971b0dae67e967a0e8fb
    - git:ac487382bc87cca9b6b6ad34dad86716f5819947
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

At write time (commit 454ee9f, 2026-03-25) the architecture spec locked in 'Immutable state + reducer pattern' and the rationale section noted event-log reprocessing and confounder resilience as reasons. The spec explicitly separated domain from presentation ('Domain unit tests are what matters'). This was inferable from the DDD layering decision (cb1183c) and the explicit choice to make the domain a pure Rust library crate. The constraint is structurally enforced: robot-buddy-domain/src/ has no #[cfg(target_arch = 'wasm32')] blocks, no macroquad imports, and no extern 'C' declarations. learner_reducer takes ownership and returns a new value. Survived through 150 commits including the Macroquad migration (ac48738, 2026-04-26) which deleted all JS but left the domain crate untouched.
