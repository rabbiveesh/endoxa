---
id: b_00655d6cd4ac
slug: r3-baseurl-doc-verdict1
claim:
  kind: text
  text: >-
    PR #33 (commit 0d648f7) corrected the POD by replacing the mutation-only example with a
    constructor-first pattern: my $client = OpenAPI::Client->new(...);
    $client->base_url(Mojo::URL->new(...));
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-vov-session
    turn: 2
  refs:
    - git:0d648f7
    - git:175b86a
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_29dc556eec80   # r3-baseurl-doc-original
  - kind: attacks
    target: b_29dc556eec80   # r3-baseurl-doc-original
coord: null
---

Contributor Clive Holloway submitted PR #33 asserting that the prior documentation was misleading — it showed mutation of `base_url` fields in isolation without instantiation context. The fix added a `my $client = OpenAPI::Client->new(...)` line and used `$client->base_url(Mojo::URL->new(...))` to replace the URL. This is the first verdict: [[r3-baseurl-doc-original]] was wrong and the POD needed a constructor-first form. However, the fix introduced only a partial picture — it showed how to replace the whole URL via the accessor, but still omitted the string-form constructor argument that actually existed.
