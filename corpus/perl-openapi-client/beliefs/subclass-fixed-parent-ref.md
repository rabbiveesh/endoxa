---
id: b_7c038cf16cf1
slug: subclass-fixed-parent-ref
claim:
  kind: text
  text: >-
    Since 1.04, new() receives $parent (the calling class) and passes it to
    _generate_class(); generated packages inherit from $parent, enabling proper subclassing
    and Moo/Moose role application.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2022-06-03T12:00:00Z
  valid_time:
    start: 2022-06-03
    end: 2999-01-01
  source:
    kind: conversation
    session: s4-inheritance-fix
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:38-54
    - lib/OpenAPI/Client.pm:232-240
    - git:Allow inheritance and roles to be applied before new() #35, #37
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.93
edges:
  - kind: supersedes
    target: b_26e21c30dc62   # subclass-hardwired-to-base
coord: null
---

Commit 810f532 renamed $class to $parent in new(), changed _generate_class() to accept $parent as first arg, and generates 'use Mojo::Base $parent'. _url_to_class() now returns "$self\::$package" using the actual caller.
