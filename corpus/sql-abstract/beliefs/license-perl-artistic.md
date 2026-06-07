---
id: b_ec406b4e4171
slug: license-perl-artistic
claim:
  kind: text
  text: >-
    The module is dual-licensed under the same terms as Perl itself (Artistic License or
    GPL), as declared in Makefile.PL.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2007-02-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2007-import
    turn: 1
  refs:
    - Makefile.PL:7
    - lib/SQL/Abstract.pm:4051-4055
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.62
edges: []
coord: null
---

Makefile.PL sets license => 'perl_5'. The POD also confirms 'same terms as perl itself'.
