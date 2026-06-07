---
id: b_dd235454f35f
slug: dedup-normalizes-keys
claim:
  kind: text
  text: >-
    VERDICT: FALSE — no such bug. key_for_sort normalizes BOTH FileKey::Path(p) and
    FileKey::Url(u) to the same PathBuf (u.to_file_path(), falling back to
    PathBuf::from(u.as_str())), so Url/Path for one file sort ADJACENTLY and file_key_eq
    (also via key_for_sort) treats them equal; dedup_by removes them correctly.
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
    turn: 7
  refs:
    - src/resolve.rs:223-232
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.92
edges:
  - kind: adjudicates
    target: b_be1be8159cce   # haiku-dedup-fails-mixed-key
  - kind: attacks
    target: b_be1be8159cce   # haiku-dedup-fails-mixed-key
coord: null
---

Defeats [[haiku-dedup-fails-mixed-key]]. A distinct and dangerous failure mode: the weak model INVENTED a plausible bug from a partial read (it saw two FileKey variants and assumed they'd compare unequal, never checking key_for_sort's normalization). A false-positive bug report asserted at 0.98 — exactly the kind of confident-wrong an epistemic store must be able to defeat with a verdict.
