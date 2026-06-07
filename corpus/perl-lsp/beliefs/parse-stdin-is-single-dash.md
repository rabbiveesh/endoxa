---
id: b_127d7f20f7d1
slug: parse-stdin-is-single-dash
claim:
  kind: text
  text: >-
    VERDICT: FALSE. The stdin sentinel is a SINGLE dash: `cli_parse` does `if path == "-"`
    (src/main.rs:1105), and the binary's own usage help even prints "(`-` reads from stdin)"
    (line 150). `--parse --` would try to open a file literally named `--`. CLAUDE.md is
    wrong about its own debugging command.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: wrong-docs-2026-06-04
    turn: 3
  refs:
    - src/main.rs:1105
    - src/main.rs:150
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_a862da50a649   # doc-parse-stdin-double-dash
  - kind: attacks
    target: b_a862da50a649   # doc-parse-stdin-double-dash
coord: null
---

Defeats [[doc-parse-stdin-double-dash]]. Doubly ironic: the doc whose whole job is 'inspect the CST instead of guessing' is itself wrong, and the in-code help string is correct while the prose doc drifted. The trusted doc lost to the code.
