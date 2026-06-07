---
id: b_b163acd32af2
slug: transactor-fix-verdict
claim:
  kind: text
  text: >-
    The correct method name is add_generator() (singular); the earlier doc used
    add_generators() (plural) which was wrong and fixed in commit ce23aa2.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2017-08-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s2-mojo-command
    turn: 2
  refs:
    - git:fix the method name called on the ua transactor
    - lib/Mojolicious/Command/openapi.pm:1
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_7dd83209e6f8   # wrong-transactor-method-name
  - kind: attacks
    target: b_7dd83209e6f8   # wrong-transactor-method-name
coord: null
---

Reneeb submitted PR fixing documentation: -  $client->ua->transactor->add_generators(xml => ...) was corrected to + add_generator(xml => ...). This was a pure documentation bug in the POD.
