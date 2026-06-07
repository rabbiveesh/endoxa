---
id: b_e955139941e0
slug: r3-blank-line-spec
claim:
  kind: text
  text: >-
    Per the POD specification, a blank line is required before a command paragraph (=head1,
    =over, etc.) that follows a plain paragraph
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-agent
    turn: 1
  refs:
    - src/scanner.c:322
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: null
edges: []
coord: null
---

The POD spec (perlpod) states that command paragraphs must be preceded by a blank line to separate them from the preceding paragraph. The scanner comment at src/scanner.c:322 directly acknowledges this spec requirement: '/* Technically there should be a blank line before the next command.' This is the spec-side of the tension: strict POD requires the blank line separator, and a compliant parser would not permit its omission.
