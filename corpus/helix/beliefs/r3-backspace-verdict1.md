---
id: b_f4104e892dd6
slug: r3-backspace-verdict1
claim:
  kind: text
  text: >-
    Backspace on an empty prompt should abort the prompt, matching muscle-memory for shell-
    like inputs
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-verdict-session
    turn: 2
  refs:
    - git:0dc67ff8852ce99d40ad4464062ebe212b0b03a1
    - helix-term/src/ui/prompt.rs
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_fefedeb38ae0   # r3-backspace-original
  - kind: attacks
    target: b_fefedeb38ae0   # r3-backspace-original
coord: null
---

PR #9828 (commit 0dc67ff8852ce99d40ad4464062ebe212b0b03a1, 2024-03-09) added a guard: if the prompt line is empty when Backspace is pressed, invoke `PromptEvent::Abort` and close the prompt. The rationale was that this matches shell readline behaviour where backspace at an empty line exits the input mode. Attacks [[r3-backspace-original]] by replacing the silent no-op with an abort action.
