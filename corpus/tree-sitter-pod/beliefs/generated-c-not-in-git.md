---
id: b_e80b6b3978ca
slug: generated-c-not-in-git
claim:
  kind: text
  text: >-
    The generated C parser (src/parser.c and src/grammar.json) is .gitignored; developers
    must run 'npx tree-sitter generate' after grammar changes.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-01-17T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-init-2023-01
    turn: 3
  refs:
    - README.md:17-22
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.7
edges: []
coord: null
---

README.md explicitly states 'the generated C source code (stored in the src directory) is .gitignored' and instructs running 'npx tree-sitter generate'. The src/ directory visible in the repo contains scanner.c (handwritten) plus the generated files committed separately for release.
