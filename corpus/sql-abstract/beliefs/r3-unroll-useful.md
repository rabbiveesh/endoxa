---
id: b_1343f9d4b890
slug: r3-unroll-useful
claim:
  kind: text
  text: >-
    Automatic parenthesis unrolling in SQL::Abstract::Tree's unparse() is necessary for
    pretty-printing and should run unconditionally.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-s3
    turn: 1
  refs:
    - lib/SQL/Abstract/Tree.pm:569
    - git:007f08535b6449047aa3bbc02d88a41b70d2e74c
    - git:bb54fcba9c26
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

The `_unparse` method in lib/SQL/Abstract/Tree.pm calls `$self->_parenthesis_unroll($tree)` at depth-0 before rendering, with a FIXME noting 'needs a config switch to disable'. The unroller removes redundant parentheses to produce cleaner SQL output. The design history shows this was the deliberate default: commit 007f085 (2011-08-01) added the FIXME while fixing overly-eager unrolling, but kept it enabled by default. Commit bb54fcb migrated the unroller from SQL::Abstract::Test to Tree, cementing it in the public formatter API.
