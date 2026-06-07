---
id: b_8f9aa9e10a6d
slug: race-condition-portal-entry
claim:
  kind: text
  text: >-
    The JS prototype had a race condition on secret area entry: `GAME.state` was only set to
    `DIALOGUE` inside a 300ms `setTimeout`, leaving a window where the player could move and
    exit the map before the speech box appeared.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-03-24T12:00:00Z
  valid_time:
    start: 2026-03-24
    end: 2026-03-29
  source:
    kind: conversation
    session: s1-init
    turn: 3
  refs:
    - git:Fix race condition: lock movement immediately on secret area entry
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.93
  asserted: null
edges: []
coord: null
---

Commit 53fca25 ('Fix race condition: lock movement immediately on secret area entry') fixes this by setting `GAME.state = DIALOGUE` immediately on portal entry, before the delay. The 300ms delay was for visual effect; the state lock was accidentally inside it.
