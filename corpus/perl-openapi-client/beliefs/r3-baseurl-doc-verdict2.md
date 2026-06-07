---
id: b_9d586da27b7f
slug: r3-baseurl-doc-verdict2
claim:
  kind: text
  text: >-
    Commit 590d721 reverted PR #33's change outright, then commit 200c2d5 re-wrote the docs
    to show all three patterns (Mojo::URL constructor arg, string constructor arg, and post-
    construction mutation) and actually wired up string-to-Mojo::URL coercion in new().
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-vov-session
    turn: 3
  refs:
    - git:590d7218563d5908cc78d642b79a7ef12b829669
    - git:200c2d5
    - lib/OpenAPI/Client.pm:44-45
    - t/base-url.t:1
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_00655d6cd4ac   # r3-baseurl-doc-verdict1
  - kind: attacks
    target: b_00655d6cd4ac   # r3-baseurl-doc-verdict1
coord: null
---

Maintainer Jan Henning Thorsen reverted the PR #33 doc fix (commit 590d721, 2021-09-25) and immediately replaced it with a more complete treatment (commit 200c2d5, 2021-09-26). This second verdict defeats [[r3-baseurl-doc-verdict1]]: the PR #33 fix was insufficient because (a) it didn't show the string constructor form, and (b) the string constructor form didn't actually work yet — `new()` didn't coerce plain strings to Mojo::URL objects. The re-write added `Scalar::Util::blessed` import, coercion logic in `new()`, three documented code examples, and a new `t/base-url.t` test covering all three patterns.
