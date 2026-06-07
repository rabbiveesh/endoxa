---
id: b_26f945e5e17f
slug: multi-chevron-intseq
claim:
  kind: text
  text: >-
    Interior sequences support multiple opening chevrons (e.g. C<< inner->arrows >>) where
    the closing delimiter must be the same number of '>' chars.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-03-03T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-chevrons-2023-03
    turn: 1
  refs:
    - src/scanner.c:51-57
    - src/scanner.c:252-262
    - src/scanner.c:270-280
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.92
edges: []
coord: null
---

scanner.c lines 252-262 count consecutive '<' chars when TOKEN_INTSEQ_START is valid and push the count onto a per-nesting stack (chevron_count). Closing at lines 270-280 requires consuming exactly that many '>' chars. The stack holds up to MAX_NESTED_CHEVRONS=8 levels; deeper nesting defaults to 1.
