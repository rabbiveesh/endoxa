---
id: b_c13394f53aa9
slug: tendril-is-smartstring
claim:
  kind: text
  text: >-
    The Tendril type used in ChangeSet Inserts is SmartString<LazyCompact>, not the tendril
    crate. This provides inline small-string optimization.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 2
  refs:
    - helix-core/src/lib.rs:46-49
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supports
    target: b_c49d91e2af75   # changeset-ops
coord: null
---

lib.rs: `// pub use tendril::StrTendril as Tendril;` is commented out; active definition is `pub type Tendril = SmartString<smartstring::LazyCompact>;`. The commented-out line documents the migration away from tendril.
