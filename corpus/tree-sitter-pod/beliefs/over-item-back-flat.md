---
id: b_2825070ceb37
slug: over-item-back-flat
claim:
  kind: text
  text: >-
    =over/=item/=back are NOT nested into a list sub-tree; they appear as sibling
    command_paragraph nodes at the top level of the pod tree.
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
    turn: 5
  refs:
    - test/corpus/commands:28-50
    - git:Don't attempt to nest over/item/back in a sub-tree
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

Commit 4b7f3ba (2023-01-18) removed the nested list structure 'because it might be intentionally split across multiple sections when embedded in a Perl file'. The test in test/corpus/commands shows =over/=item/=back as flat command_paragraph children of pod.
