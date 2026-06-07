---
id: b_6efd38640989
slug: scope-v2-and-v3
kind: project-scope
claim:
  kind: text
  text: >-
    The project supports both OpenAPI v2 and v3 via JSON::Validator::Schema::OpenAPIv2 and
    ::OpenAPIv3; the validator() method returns the appropriate subclass.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2021-02-23T12:00:00Z
  valid_time:
    start: 2021-02-23
    end: 2999-01-01
  source:
    kind: conversation
    session: s3-v3-api
    turn: 1
  refs:
    - Changes:43-46
    - lib/OpenAPI/Client.pm:56-91
    - git:Updated to use new JSON::Validator schema API
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_c8771348ea24   # scope-v2-only-api
coord: null
---

Version 1.00 (2021-02-23) rewrote _generate_class() to use the new JSON::Validator schema API with $jv->schema()->schema, and added support for OpenAPIv3. The validator->routes API is the uniform interface for both versions.
