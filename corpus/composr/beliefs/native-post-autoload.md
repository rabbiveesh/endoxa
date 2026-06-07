---
id: b_42dc65f4dc69
slug: native-post-autoload
claim:
  kind: text
  text: >-
    Two narrowing exceptions handle Laravel natively: ComposerScripts::postAutoloadDump (the
    known clearCompiled handler) is skipped, and `@php artisan package:discover` is replaced
    by native `composr discover`. When those fire and the rest are plain shell/php, post-
    autoload-dump runs with ZERO composer subprocesses.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time:
    start: 2026-05-07
    end: 2999-01-01
  source:
    kind: conversation
    session: hybrid-2026-05-07
    turn: 5
  refs:
    - git:Native package:discover
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supersedes
    target: b_5cf8a5c93a05   # delegate-all-post-autoload
  - kind: refines
    target: b_d82752b2b23b   # hybrid-mode-philosophy
coord: null
---

Supersedes [[delegate-all-post-autoload]] and refines [[hybrid-mode-philosophy]]. This is how monolith-app reaches '0 composer calls' on a cold install.
