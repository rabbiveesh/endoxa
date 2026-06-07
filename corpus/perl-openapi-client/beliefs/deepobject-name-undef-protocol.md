---
id: b_e0aab0b3044c
slug: deepobject-name-undef-protocol
claim:
  kind: text
  text: >-
    JSON::Validator passes an undefined $name to the query callback when it wants the entire
    params hash to reconstruct a deepObject or form/explode object parameter; failing to
    handle this caused dropped query params and an uninitialized-value warning.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s6-deepobject-fix
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:177-191
    - t/style-explode.t:19-29
    - git:fix: support deepObject explode in openAPIv3 specs
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

The fix in faede03 adds a guard: 'unless (defined $name)' inside the query callback. For deepObject, it filters keys matching /^{name}[/ ; for form/explode it filters keys present in $param->{schema}{properties}. Without this, $url->query->param(undef => ...) was called, silently dropping the params.
