---
id: b_726384402b24
slug: scope-initial-head-paragraphs
kind: project-scope
claim:
  kind: text
  text: >-
    In January 2023 the grammar only handled specific named paragraphs (=head1..=head6,
    =over, =item, =back, =encoding) as distinct typed nodes; any other =cmd was unparsed.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-01-17T12:00:00Z
  valid_time:
    start: 2023-01-17
    end: 2023-09-29
  source:
    kind: conversation
    session: session-init-2023-01
    turn: 2
  refs:
    - git:Recognise any kind of Pod command, not just specifically-coded ones
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges:
  - kind: supersedes
    target: b_8c260ca39d5c   # scope-generic-command-paragraph
coord: null
---

The initial grammar had head_paragraph, over_paragraph, item_paragraph, back_paragraph, encoding_paragraph as separate rules. The commit 'Recognise any kind of Pod command' (cc1411d, 2023-09-30) replaced them all with a single generic command_paragraph + command token.
