---
id: b_6f43a61b1808
slug: scanner-heredoc-fifo-bounded
claim:
  kind: text
  text: >-
    The external scanner queues pending heredocs in a bounded FIFO (HEREDOC_QUEUE_MAX = 8).
    On overflow it deliberately OVERWRITES the last slot rather than dropping — a DOCUMENTED
    graceful degradation: the first MAX-1 bodies parse correctly, the last greedily swallows
    the overflow (wrong but BOUNDED), and code after the block stays in sync. HEREDOC_START
    is primed only when the first heredoc is enqueued (count==1).
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: ts-perl-orient-2026-06-04
    turn: 3
  refs:
    - src/scanner.c:240-262
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The 23-line comment at scanner.c:240 spells out the overflow design. This is the ground truth for [[haiku-heredoc-overflow-is-bug]].
