---
id: b_4b51b4e1859f
slug: hunch-end-command-peek-destructive
claim:
  kind: text
  text: >-
    at_end_command() peeks forward using lexer->advance without calling mark_end first,
    which means the consumed characters are not rolled back; callers must have already
    called mark_end if they care about the token boundary.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-22T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-begin-end-2026-03
    turn: 5
  refs:
    - src/scanner.c:114-130
    - src/scanner.c:183-189
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.6
edges: []
coord: null
---

scanner.c lines 117-130: at_end_command calls lexer->advance multiple times with no mark_end inside it. The caller at line 184 calls mark_end before invoking at_end_command, so position is preserved for the data token. But at_end_command's own advances effectively discard those characters from the result if TOKEN_DATA_SECTION is then emitted at the earlier mark.
