---
id: b_49e644876814
slug: subclass-comparison-ne-was-wrong
claim:
  kind: text
  text: >-
    The coderef comparison using `ne` in subclass detection was buggy — commit de47ae7 fixed
    subsequent errors caused by the f8769bf refactor, revealing the ne comparison was
    fragile and led to mistakes.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2024-09-05T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2024-fixes
    turn: 2
  refs:
    - git:fix stupid mistakes in subclass override refactor
    - lib/SQL/Abstract.pm:262-303
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_b6d21565760c   # wrong-hunch-subclass-comparison-was-string-ne
  - kind: attacks
    target: b_b6d21565760c   # wrong-hunch-subclass-comparison-was-string-ne
coord: null
---

Commit de47ae7 (2024-09-05) message: 'fix stupid mistakes in subclass override refactor'. The patch switched from `__PACKAGE__->can($method) ne $class->can($method)` to a `$subclassed` closure using `!=`. This is the safer and correct way to compare coderefs for identity.
