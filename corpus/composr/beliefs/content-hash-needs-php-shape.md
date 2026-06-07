---
id: b_8c5b2cd831e0
slug: content-hash-needs-php-shape
claim:
  kind: text
  text: >-
    VERDICT: the content hash must match composer's PHP-specific key normalization +
    serialization to be compatible — a plain byte hash mismatched. composr had to implement
    a PHP-compatible composer.json content hash.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-10T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: plugins-2026-05-10
    turn: 13
  refs:
    - git:PHP-compatible composer.json content hash
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_7ee008de39f7   # content-hash-is-plain
  - kind: attacks
    target: b_7ee008de39f7   # content-hash-is-plain
coord: null
---

Defeats [[content-hash-is-plain]]. Lower stakes (the check is advisory) but the same shape: 'it's obviously just a hash' was an under-specified read of a format owned by another tool.
