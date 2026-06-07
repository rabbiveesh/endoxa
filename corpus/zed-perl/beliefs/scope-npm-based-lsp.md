---
id: b_b65968462786
slug: scope-npm-based-lsp
kind: project-scope
claim:
  kind: text
  text: >-
    From 2024-09-08, the extension added a Rust extension that installed PerlNavigator via
    npm (`perlnavigator-server` package), including a hack to relocate vscode-oniguruma.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time:
    start: 2024-09-08
    end: 2024-09-08
  source:
    kind: conversation
    session: seed-explore-02
    turn: 8
  refs:
    - git:feat: setup perlnavigator via npm (needs help tho)
    - git:fix: remove evil oniguruma hack
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_68a4e1997fb4   # scope-grammar-only
coord: null
---

Commits f845a03 and bc94c30 show the npm phase: `PACKAGE_NAME = "perlnavigator-server"`, `SERVER_PATH = "node_modules/.bin/perlnavigator"`. The oniguruma hack existed briefly before removal.
