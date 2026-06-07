---
id: b_7eb3d02941fe
slug: r3-blank-line-lax
claim:
  kind: text
  text: >-
    The scanner deliberately accepts commands that are not preceded by a blank line because
    so many real-world POD documents omit the blank line
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-agent
    turn: 2
  refs:
    - src/scanner.c:321
    - src/scanner.c:322
    - src/scanner.c:323
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.7
  asserted: null
edges:
  - kind: attacks
    target: b_e955139941e0   # r3-blank-line-spec
coord: null
---

The scanner comment at src/scanner.c:322-323 directly surfaces the conflict: '/* Technically there should be a blank line before the next command. * But so many people omit it. We'll allow this here */'. The code then falls through to `TOKEN(TOKEN_CONTENT_PLAIN)` and ends the current plain paragraph when it sees a line beginning with `=` even without a preceding blank line. This contradicts [[r3-blank-line-spec]] and is un-adjudicated: no later commit has resolved the tension or chosen one side; the lax behavior coexists with the acknowledged spec violation in an open comment. There is no bug report closed or issue resolved that changes this behavior.
