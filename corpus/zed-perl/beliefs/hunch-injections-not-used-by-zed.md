---
id: b_793fe9cb0850
slug: hunch-injections-not-used-by-zed
claim:
  kind: text
  text: >-
    The injections.scm file may not be actively used by Zed since it still carries an nvim-
    treesitter comment, and Zed may require different injection syntax.
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
    turn: 6
  refs:
    - languages/perl/injections.scm:1
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.55
edges: []
coord: null
---

The file header says '`; an scm file for nvim-treesitter`' and injects `comment` and `pod` languages. It's unclear whether Zed's injection engine supports the same directives. Low confidence — no evidence either way in the git log.
