---
id: b_68dec4b4a585
slug: perl-gen-importbase
claim:
  kind: text
  text: >-
    perl-gen/ reads an Import::Base kit's @IMPORT_MODULES / %IMPORT_BUNDLES tables and emits
    a ready-to-commit Rhai plugin, so the LSP understands shop-specific `use Co::Base
    -Class` import boilerplate.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: nav-graph-2026-06-04
    turn: 8
  refs:
    - perl-gen/README.md
    - docs/adr/importbase-plugin-gen.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

Bridges the plugin platform to real Perl shops that centralize imports behind a kit.
