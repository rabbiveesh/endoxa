---
id: b_9057b4854332
slug: dynamic-subclass-per-spec
claim:
  kind: text
  text: >-
    Each distinct specification URL causes a unique dynamically-generated subclass of
    OpenAPI::Client to be created via eval and Mojo::Util::monkey_patch; these subclasses
    are cached so repeated new() calls with the same URL reuse them.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2017-08-18T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s1-initial-import
    turn: 3
  refs:
    - lib/OpenAPI/Client.pm:37-54
    - lib/OpenAPI/Client.pm:232-240
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supports
    target: b_c0f3b76dc397   # validator-is-class-global
coord: null
---

_url_to_class() converts the spec URL into a valid package name (md5 if >110 chars). _generate_class() only runs if the class doesn't already isa the parent. Methods are monkey-patched onto the class, not re-generated each instantiation.
