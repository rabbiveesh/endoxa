---
id: b_f02a24adddc4
slug: r3-max-nested-chevrons-8
claim:
  kind: text
  text: >-
    The maximum tracked nesting depth for multiple-chevron interior sequences is exactly 8,
    stored as a fixed-size stack in LexerState
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2023-03-03
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-agent
    turn: 3
  refs:
    - src/scanner.c:51
    - src/scanner.c:54
    - git:ca6882882ec43e05b4def2cac42392c7f34a6841
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.58
  asserted: 0.63
edges: []
coord: null
---

Inferred at commit ca68828 (2023-03-03): `#define MAX_NESTED_CHEVRONS 8` with `unsigned char chevron_count[MAX_NESTED_CHEVRONS]` introduced the multiple-chevron stack. At write time this was inferable as a reasonable upper bound: POD sequences with `<<`, `<<<`, etc. are rare in practice and more than 8 levels of distinct chevron depths in a single document would be pathological. The comment in the struct confirms the design: 'stores at most MAX count; always the true number, further are implied =1 if beyond MAX' — graceful degradation for overflow. The value 8 has never been changed across the 5 subsequent scanner.c commits (d4e6745, c4f8662, 11f0bca, 6902fce, fff3b6a). Mechanically checkable: `grep 'MAX_NESTED_CHEVRONS' src/scanner.c | head -1` returns `#define MAX_NESTED_CHEVRONS 8`.
