---
id: b_c2868b874541
slug: event-system-sync-hooks
claim:
  kind: text
  text: >-
    Synchronous hooks in helix-event run immediately on the calling thread and can modify
    editor state in-place. They cannot store their own state. For stateful or async needs,
    AsyncHook is required.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-12-01T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-event-hook-2023
    turn: 2
  refs:
    - helix-event/src/lib.rs:11-18
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges:
  - kind: supports
    target: b_5f80cb3711cd   # scope-async-hook-system
coord: null
---

helix-event/src/lib.rs module docs: 'Hooks run synchronously which can be advantageous since they can modify the current editor state right away... However, they can not contain their own state without locking since they only receive immutable references. For handlers that want to track state... an AsyncHook is preferable.'
