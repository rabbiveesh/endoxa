---
id: b_222217a3c839
slug: cra-per-operation
claim:
  kind: text
  text: >-
    CRA stage (Concrete/Representational/Abstract) is tracked independently per `Operation`,
    so a child can be Abstract for addition but still Concrete for division.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 3
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:24
    - robot-buddy-domain/src/learning/learner_profile.rs:44-47
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

LearnerProfile stores `cra_stages: HashMap<Operation, CraStage>`. New profiles initialize all 5 operations (Add, Sub, Multiply, Divide, NumberBond) to Concrete. The reducer advances each operation's CRA independently.
