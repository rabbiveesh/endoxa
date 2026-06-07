---
id: b_fefedeb38ae0
slug: r3-backspace-original
claim:
  kind: text
  text: >-
    Backspace on an empty command-prompt line should only delete (no-op at empty), never
    abort the prompt
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2020-12-13
    end: 2024-03-09
  source:
    kind: conversation
    session: r3-verdict-session
    turn: 1
  refs:
    - helix-term/src/ui/prompt.rs
    - git:07801b60bccd0f084eae925e0290c24322de575f
    - git:0dc67ff8852ce99d40ad4464062ebe212b0b03a1
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: 0.75
edges: []
coord: null
---

Before commit 0dc67ff8852ce99d40ad4464062ebe212b0b03a1 landed on 2024-03-09, pressing Backspace in an empty Helix command prompt had no special behaviour: the handler fell through to `self.delete_char_backwards(cx.editor)` which is a no-op on empty input. The existing abort paths were Ctrl-C / Esc. This was the ambient assumption the PR #9828 explicitly targeted.
