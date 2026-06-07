---
id: b_e56b981f31aa
slug: r3-corrob-workspace-merged
claim:
  kind: text
  text: >-
    Corroborated by two independent observations (Cargo.toml [workspace] + the helix-*
    directory layout): Helix is a multi-crate Cargo workspace (helix-core / -view / -term /
    -lsp / -tui / ...).
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2024-08-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-corrob-dirs-2024
    turn: 5
  refs: []
  derived_from:
    - b_c995499f12ea
    - b_338733d9242c
confidence:
  directness: reduced
  observation_count: 2
  source_weight: 0.7
  asserted: null
edges:
  - kind: derived_from
    target: b_c995499f12ea   # r3-corrob-workspace-via-cargo
  - kind: derived_from
    target: b_338733d9242c   # r3-corrob-workspace-via-dirs
coord: null
---

CORROBORATION-ACCRETION case (hand-off build-list #3): the same proposition observed two independent ways in two sessions, merged here with observation_count=2 from two `derived_from` inputs (a genuine count, not hand-set on a lone belief). This deliberately FORCES the id-identity fork (standing-fix #2). It is modeled as OBSERVATION-IDENTITY: two distinct beliefs (distinct content-ids only because the wording differs) linked by `supports` and reduced into this consensus. RECOMMENDATION: adopt observation-identity (id = hash of the full BeliefBody incl. provenance, per design doc §3) — N independent observations stay N beliefs that accrete corroboration via edges + a reducer. Under the lib's current proposition-identity (id = sha256(claim_text)), identical wording would silently collapse the two observations into ONE id and erase the second's provenance. This case is the concrete argument to switch before the edge-match scorer relies on belief identity.
