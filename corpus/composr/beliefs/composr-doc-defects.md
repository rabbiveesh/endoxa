---
id: b_fe451b10a9f9
slug: composr-doc-defects
claim:
  kind: text
  text: >-
    Smaller objective README defect: the README shows the git-hook templates and says
    they're 'exactly what install-hooks writes', but the shown templates omit the `# managed
    by composr install-hooks` marker line that the real constants (src/main.rs, HOOK_MARKER)
    write as line 2 — the marker is load-bearing (it's how re-running install-hooks
    recognizes and overwrites its own hooks).
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: wrong-docs-2026-06-04
    turn: 5
  refs:
    - README.md:247
    - src/main.rs:325
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.85
edges: []
coord: null
---

A TRUE observation about a wrong doc: 'exactly' is falsified by a missing line that matters for idempotent hook installation. Low stakes, ambient doc-rot.
