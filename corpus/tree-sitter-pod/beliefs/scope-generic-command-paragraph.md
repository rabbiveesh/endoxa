---
id: b_8c260ca39d5c
slug: scope-generic-command-paragraph
kind: project-scope
claim:
  kind: text
  text: >-
    Since 2023-09-30 all =cmd paragraphs (other than =pod, =cut, =begin/=end, =for) are
    parsed as generic command_paragraph nodes with a command token matching /=[a-zA-Z]\S*/.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-09-30T12:00:00Z
  valid_time:
    start: 2023-09-30
    end: 2999-01-01
  source:
    kind: conversation
    session: session-generic-cmd-2023-09
    turn: 2
  refs:
    - grammar.js:55-56
    - queries/highlights.scm:10-33
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges: []
coord: null
---

grammar.js line 55-56: command_paragraph uses command = token(/=[a-zA-Z]\S*/), letting any unknown =cmd through. Highlights.scm uses #match? and #not-match? predicates to differentiate =head, =over, =item, =encoding from the generic fallback.
