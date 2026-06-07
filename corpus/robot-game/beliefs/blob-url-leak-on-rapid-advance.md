---
id: b_6993771ed2dd
slug: blob-url-leak-on-rapid-advance
claim:
  kind: text
  text: >-
    In the JS prototype, ElevenLabs audio blob URLs were only revoked in `onended`, not when
    audio was interrupted by rapid dialogue advancement, causing a memory leak over long
    sessions.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-03-24T12:00:00Z
  valid_time:
    start: 2026-03-24
    end: 2026-04-26
  source:
    kind: conversation
    session: s1-init
    turn: 4
  refs:
    - git:Fix ElevenLabs blob URL leak on rapid dialogue advancement
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: null
edges: []
coord: null
---

Commit 1187800 ('Fix ElevenLabs blob URL leak on rapid dialogue advancement') fixed this by revoking the blob URL immediately on cleanup (pause + revoke) rather than waiting for natural playback end. This was a production bug on the deployed JS version.
