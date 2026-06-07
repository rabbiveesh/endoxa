---
id: b_ffa22e527e2c
slug: old-cleanup-deletes-stale-versions
claim:
  kind: text
  text: >-
    After downloading a new version, the extension deletes all sibling directories that are
    not the newly downloaded version_dir.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: seed-explore-03
    turn: 7
  refs:
    - src/perl.rs:84-91
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

Lines 84-91 of src/perl.rs read the working directory and call `fs::remove_dir_all` on any entry whose name doesn't match `version_dir`. This is a cleanup to prevent accumulation of old release downloads.
