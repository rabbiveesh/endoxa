---
id: b_b6d21565760c
slug: wrong-hunch-subclass-comparison-was-string-ne
claim:
  kind: text
  text: >-
    Subclass override detection in `__maybe_setup_subclass_overrides` compared coderefs
    using string `ne`, which is safe because Perl coderef stringification is stable.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 3
  refs:
    - git:refactor subclass override handling
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

Prior to commit f8769bf, the code read `__PACKAGE__->can($method) ne $class->can($method)`. This looks plausible but comparing coderefs with `ne` is actually not guaranteed to work correctly across all Perl versions and can produce wrong results when undef is returned by `can`.
