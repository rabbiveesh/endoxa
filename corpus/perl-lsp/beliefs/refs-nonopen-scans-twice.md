---
id: b_a3cf3f353d77
slug: refs-nonopen-scans-twice
claim:
  kind: text
  text: >-
    VERDICT: FALSE. A not-open workspace module is scanned TWICE — once in the WORKSPACE
    phase (covered_paths only excludes OPEN files) and again in the DEPENDENCY phase, which
    has no covered_paths skip. Both phases run collect_from_analysis independently.
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
    turn: 5
  refs:
    - src/resolve.rs:191-208
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_57eaa985ff0a   # haiku-refs-nonopen-single-scan
  - kind: attacks
    target: b_57eaa985ff0a   # haiku-refs-nonopen-single-scan
coord: null
---

Defeats [[haiku-refs-nonopen-single-scan]]. Bonus: the verdict surfaces a real (benign) inefficiency — the same analysis is walked twice and only deduped afterward. The weak model under-counted the work because the skip logic lives in a different phase than the one it was looking at (out-of-snippet knowledge again).
