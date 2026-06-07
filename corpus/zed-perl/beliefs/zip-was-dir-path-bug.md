---
id: b_91f956a392c9
slug: zip-was-dir-path-bug
claim:
  kind: text
  text: >-
    VERDICT: wrong about the bug. Pre-011deca, asset_name DID embed '.zip' and was compared
    with `== asset_name`, so the asset SEARCH was correct (no '.zip.zip'). The real bug:
    that same '.zip'-bearing asset_name was reused to build the binary_path directory, so it
    looked for a literal 'perlnavigator-linux-x86_64.zip/' subdir in the archive. The fix
    split the extension out and appended it only in the find call.
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
    turn: 3
  refs:
    - src/perl.rs (git:011deca)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_1b37eb48310b   # haiku-zip-double-ext
  - kind: attacks
    target: b_1b37eb48310b   # haiku-zip-double-ext
coord: null
---

Defeats [[haiku-zip-double-ext]]. Plausible bug guessed; the actual prior bug was a different one (extension leaking into a directory path).
