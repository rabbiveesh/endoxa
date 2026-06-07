---
id: b_ac4c10d9eed1
slug: advance-c-side-effects-c
claim:
  kind: text
  text: >-
    The ADVANCE_C macro both calls lexer->advance and updates the local variable 'c' to
    lexer->lookahead as a side effect.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-09-30T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-generic-cmd-2023-09
    turn: 1
  refs:
    - src/scanner.c:16-29
    - git:Side-effect the c variable inside ADVANCE_C macro
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

src/scanner.c lines 16-29 define ADVANCE_C. The final line inside the do-while is 'c = lexer->lookahead;'. Before commit 18506fc this assignment was done manually after each ADVANCE call; the refactor moved it into the macro. Any code using ADVANCE_C must declare 'int c' in scope before the macro invocation.
