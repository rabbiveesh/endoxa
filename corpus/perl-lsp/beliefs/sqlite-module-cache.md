---
id: b_69cc0aaa419d
slug: sqlite-module-cache
claim:
  kind: text
  text: >-
    Cross-file resolution is backed by a per-project SQLite cache at ~/.cache/perl-
    lsp/<hash>/modules.db (bincode+zstd FileAnalysis blobs, schema v9). An EXTRACT_VERSION
    bump triggers priority re-resolution without dropping the table; nuke it via `perl-lsp
    --clear-cache [<root>]`, never rm -rf.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: crossfile-2026-03-02
    turn: 6
  refs:
    - CLAUDE.md
    - module_cache.rs
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

A plugin-fingerprint mismatch on startup hard-clears the modules table so QA isn't served stale blobs.
