---
id: b_2af84d18f75b
slug: r3-external-scanner-design
claim:
  kind: text
  text: >-
    POD parsing requires an external scanner (not pure grammar rules) for line-level
    disambiguation of paragraph types
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2023-01-17
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-agent
    turn: 1
  refs:
    - src/scanner.c:132
    - git:e9db1946f771b4571bbd1646501165bcd02c389c
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

Inferred at commit e9db194 (2023-01-17): the very first version of src/scanner.c introduced an external scanner with `TOKEN_EOL`, `TOKEN_START_DIRECTIVE`, and `TOKEN_START_PLAIN`. At write time, this was inferable from the structure of POD itself: paragraph types (plain, verbatim, command) are distinguished by column-zero content and cannot be disambiguated by tree-sitter's purely context-free grammar rules — verbatim paragraphs are identified by indentation, commands by `=` at column 0, and plain paragraphs by the absence of both. The external scanner approach is the only way to encode these positional constraints. This decision has survived all 68 subsequent commits; the external scanner remains the core parsing architecture as of the current HEAD (src/scanner.c still implements the full external scanner API). Mechanically checkable: `grep -c 'tree_sitter_pod_external_scanner_scan' src/scanner.c` returns 1.
