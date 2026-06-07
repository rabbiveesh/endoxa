---
id: b_338733d9242c
slug: r3-corrob-workspace-via-dirs
claim:
  kind: text
  text: >-
    Helix's code is split across multiple crates: the helix-* directories at the repo root
    (helix-core, helix-view, helix-term, helix-lsp, helix-tui, ...) are the workspace's
    member crates.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2024-08-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-corrob-dirs-2024
    turn: 2
  refs:
    - helix-core/
    - helix-view/
    - helix-term/
    - helix-lsp/
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges:
  - kind: supports
    target: b_c995499f12ea   # r3-corrob-workspace-via-cargo
coord: null
---

Observation #2 — derived independently (different session, different method: the directory layout, not Cargo.toml) of the same fact as [[r3-corrob-workspace-via-cargo]]. Links to it via `supports`; merged in [[r3-corrob-workspace-merged]].
