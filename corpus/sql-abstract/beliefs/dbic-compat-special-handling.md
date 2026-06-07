---
id: b_86a81493a62f
slug: dbic-compat-special-handling
claim:
  kind: text
  text: >-
    SQL::Abstract has multiple DBIx::Class-specific compatibility shims: detecting
    `$class->isa('DBIx::Class::SQLMaker')` to set special flags like `warn_once_on_nest`,
    `disable_old_special_ops`, and a custom `select.where` clause renderer.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 6
  refs:
    - lib/SQL/Abstract.pm:248-304
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

__maybe_setup_subclass_overrides at line 298-304 sets DBIC-specific opts when it detects the subclass is DBIx::Class::SQLMaker. The dbic_select_where closure strips leading/trailing whitespace from the WHERE output to match what DBIC expects.
