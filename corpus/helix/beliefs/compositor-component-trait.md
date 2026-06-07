---
id: b_77ded93c94a8
slug: compositor-component-trait
claim:
  kind: text
  text: >-
    The Compositor is a layered UI system where each UI element implements the Component
    trait (handle_event, render, cursor, required_size). EventResult can carry a deferred
    Callback closure.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 3
  refs:
    - helix-term/src/compositor.rs:9-67
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges: []
coord: null
---

helix-term/src/compositor.rs: `pub trait Component: Any + AnyComponent` with methods `handle_event`, `render`, `cursor`, `required_size`. `EventResult` is `Ignored(Option<Callback>) | Consumed(Option<Callback>)`. `Callback = Box<dyn FnOnce(&mut Compositor, &mut Context)>`.
