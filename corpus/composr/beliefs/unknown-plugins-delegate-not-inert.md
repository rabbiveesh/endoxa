---
id: b_06fff9300cd6
slug: unknown-plugins-delegate-not-inert
claim:
  kind: text
  text: >-
    VERDICT: FALSE. The default for UNKNOWN plugins is to DELEGATE each lifecycle event to
    `composer run-script` (so their subscribers fire) — not to install them as inert files.
    The README's own 'Plugin policy' section says exactly this; the parenthetical on the
    `--strict-plugins` line contradicts it and the code (src/install.rs unknown-plugin
    delegation path).
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
    turn: 3
  refs:
    - src/install.rs:155
    - src/install.rs:629
    - README.md (Plugin policy section)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_33263aafe14d   # doc-strict-plugins-default-inert
  - kind: attacks
    target: b_33263aafe14d   # doc-strict-plugins-default-inert
  - kind: supports
    target: b_029dbc660e47   # plugin-policy-three-tier
coord: null
---

Defeats [[doc-strict-plugins-default-inert]] and corroborates [[plugin-policy-three-tier]]. A doc that contradicts ITSELF — the detailed section is right, the flag's parenthetical summary is wrong. 'Inert' is the one thing unknown plugins are NOT.
