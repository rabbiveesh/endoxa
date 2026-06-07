---
id: b_313798d03e8e
slug: r3-generated-class-cached
claim:
  kind: text
  text: >-
    Generated subclasses are cached: calling new() with the same spec URL twice reuses the
    already-generated class rather than regenerating it.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 4
  refs:
    - lib/OpenAPI/Client.pm:42
    - git:275bf6b
    - git:810f532
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: 0.7
edges: []
coord: null
---

At write time (2017-08-18, commit 275bf6b), the `unless $class->isa($parent)` guard (originally `unless $class->isa($BASE)`) makes the caching design structurally explicit — the class is only generated the first time a given URL is seen; subsequent calls find the class already exists in Perl's symbol table and skip `_generate_class`. This was inferable as a performance/correctness decision: re-patching methods onto an existing class could cause duplicate method registration. The guard has survived the inheritance refactor (810f532) which changed `$BASE` to `$parent` without altering the caching semantics. Checkable: lib/OpenAPI/Client.pm line 42 `unless $class->isa($parent)`.
