---
id: b_adacbf898036
slug: heredoc-overflow-is-intentional
claim:
  kind: text
  text: >-
    VERDICT: FALSE — not a bug. The 23-line comment at scanner.c:240 documents the overwrite
    as INTENTIONAL graceful degradation. HEREDOC_START needs no re-arming because the FSM is
    already in heredoc mode (START was set when count first hit 1); overflow just retargets
    the final terminator. Bounded-but-wrong by design, no desync.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 7
  refs:
    - src/scanner.c:240-262
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.92
edges:
  - kind: adjudicates
    target: b_92a593f775a2   # haiku-heredoc-overflow-is-bug
  - kind: attacks
    target: b_92a593f775a2   # haiku-heredoc-overflow-is-bug
  - kind: supports
    target: b_6f43a61b1808   # scanner-heredoc-fifo-bounded
coord: null
---

Defeats [[haiku-heredoc-overflow-is-bug]]; supports [[scanner-heredoc-fifo-bounded]]. The sharpest lesson of the harvest: the model cried 'bug' while IGNORING the doc-comment that explained the design sitting directly above the code — the mirror image of the wrong-docs thread (there a trusted doc was wrong; here a correct doc was ignored). Confident-wrong at 0.95.
