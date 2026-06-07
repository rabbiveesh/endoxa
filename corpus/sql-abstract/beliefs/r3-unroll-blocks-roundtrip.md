---
id: b_d560ea414d91
slug: r3-unroll-blocks-roundtrip
claim:
  kind: text
  text: >-
    Unconditional parenthesis unrolling in unparse() makes exact roundtrip testing
    impossible without monkey-patching, representing accumulated technical debt.
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
    turn: 2
  refs:
    - t/14roundtrippin.t:37
    - lib/SQL/Abstract/Tree.pm:569
    - git:87abf9bc3395870cef72a3f00b7d4cfe18abd980
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.8
edges:
  - kind: attacks
    target: b_1343f9d4b890   # r3-unroll-useful
coord: null
---

Commit 87abf9b (2013-06-02) introduced t/14roundtrippin.t with a prominently triple-FIXME block: 'The formatter/unparser accumulated a ton of technical debt... format() does an implicit parenthesis unroll for prettyness which makes it hard to do exact comparisons'. The test works around this by monkey-patching `_parenthesis_unroll` to a no-op for roundtrip verification (see t/14roundtrippin.t:58). This is a structural acknowledgment that [[r3-unroll-useful]] and exact-roundtrip correctness are in unresolved tension: as of HEAD, both the unroll-by-default behavior and the FIXME note requesting a config switch remain in place, with no adjudication.
