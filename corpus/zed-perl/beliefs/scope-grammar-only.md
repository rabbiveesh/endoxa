---
id: b_68a4e1997fb4
slug: scope-grammar-only
kind: project-scope
claim:
  kind: text
  text: >-
    At project inception (2024-08-03) the extension was purely declarative: grammar config,
    highlight queries, and injection/outline queries — no Rust code or LSP integration.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-08-03T12:00:00Z
  valid_time:
    start: 2024-08-03
    end: 2024-09-08
  source:
    kind: conversation
    session: seed-explore-01
    turn: 2
  refs:
    - git:feat: FIRST COMMIT
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

The FIRST COMMIT (650f370) only adds .gitignore, extension.toml, and files under languages/perl/. Cargo.toml and src/perl.rs did not exist yet.
