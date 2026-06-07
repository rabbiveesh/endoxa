---
id: b_4f3adb97ba34
slug: coarse-memo-key-unsound-diamond
claim:
  kind: text
  text: >-
    VERDICT (soundness review, fixed 2026-06-03): the coarse memo key was UNSOUND.
    ReturnExpr::Receiver substitutes the FULL receiver, so one shared MethodOnClass{Parent,
    m} attachment reached with ClassName('Foo') then ClassName('Bar') served Foo's memoized
    answer to Bar — a SILENT wrong type. The cycle-guard could share the coarse key
    (collision→None, conservative) but a memo returning a substantive value on it could not.
    Fix: key the receiver on full structural identity.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: nav-graph-2026-06-04
    turn: 13
  refs:
    - git:580b72a
    - git:b4a1911
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.95
edges:
  - kind: adjudicates
    target: b_761169c5ecfe   # memo-coarse-receiver-key-sound
  - kind: attacks
    target: b_761169c5ecfe   # memo-coarse-receiver-key-sound
  - kind: supports
    target: b_977f6b2eed5d   # edges-not-values
coord: null
---

Defeats [[memo-coarse-receiver-key-sound]]. The exact error class the user flagged: we were wrong about caching because of diamond inheritance. The subtlety: the real inheritance diamond the memo EXISTS to tame is same-receiver (q.receiver constant), so it still hashes to one key and perf held (cold crm --check ~8s) — the bug was *cross-receiver* collision. Lesson: 'the cycle-guard shares this key safely' did NOT transfer to 'the memo may return a cached value on this key' — same key shape, different soundness contract. A live case of the [[edges-not-values]] warning: a materialized cache that drifts from the canonical chase.
