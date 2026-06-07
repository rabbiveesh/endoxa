---
id: b_6bf04cab77a5
slug: r3-backspace-verdict2
claim:
  kind: text
  text: >-
    The backspace-aborts-empty-prompt change was wrong: the old no-op behaviour was less
    surprising, and Ctrl-C / Esc are sufficient abort paths
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-verdict-session
    turn: 3
  refs:
    - git:07e21a10f065eab5491e1e4a1a7aa12000b47d59
    - helix-term/src/ui/prompt.rs:544
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_f4104e892dd6   # r3-backspace-verdict1
  - kind: attacks
    target: b_f4104e892dd6   # r3-backspace-verdict1
coord: null
---

Commit 07e21a10f065eab5491e1e4a1a7aa12000b47d59 (2024-03-26, PR #10005) explicitly reverts 0dc67ff, citing post-merge discussion in #9828: "The old behavior was less surprising and we have other ways to abort from a prompt, so let's revert the behavior change." The revert removes the four-line guard entirely. This adjudicates [[r3-backspace-verdict1]] and declares it wrong: the first fix introduced surprising behaviour that violated the principle of least surprise, and Esc/Ctrl-C already cover abort. The original assumption [[r3-backspace-original]] is thereby reinstated.
