---
id: b_ce83fd9fa293
slug: scope-no-begin-end
kind: project-scope
claim:
  kind: text
  text: >-
    Before March 2026 the grammar had no support for =begin/=end data regions or =for
    paragraphs.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-09-30T12:00:00Z
  valid_time:
    start: 2023-01-17
    end: 2026-03-21
  source:
    kind: conversation
    session: session-generic-cmd-2023-09
    turn: 3
  refs:
    - git:feat: support =begin/=end data regions and =for paragraphs
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.88
  asserted: 0.88
edges:
  - kind: supersedes
    target: b_79df12c5f40b   # scope-begin-end-supported
coord: null
---

PR #13 'support-begin-end-for' merged 2026-03-22 added begin_paragraph, for_paragraph, format_name, and the _data_section external token. The grammar.json was updated in the same batch.
