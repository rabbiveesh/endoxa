---
id: b_5e2149519455
slug: r3-body-param-key-wins
claim:
  kind: text
  text: >-
    When a body parameter name matches a key in $params, that value wins unconditionally and
    is promoted to $content{json}, even if the caller also passes an explicit json=> or
    body=> in %content.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-session
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:139-153
    - lib/OpenAPI/Client.pm:208
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: null
edges: []
coord: null
---

In `_build_tx`, the `body` sub (lib/OpenAPI/Client.pm lines 139-153) checks `if (exists $params->{$name})` first. If true, it does `$content{json} = $params->{$name}` before validation runs. Then at line 208, the tx is built with `defined $content{body} ? $content{body} : %content`. This means: if a caller passes both `{body => $data}` in `$params` AND `body => $raw_string` in `%content`, then `$content{json}` is set from `$params` at line 143, but `$content{body}` is NOT set — so the tx gets the structured JSON from `$params`. There is no conflict warning. But if the caller passes both `$params->{body}` AND `json => $override` in `%content`, the `$content{json}` will be silently overwritten at line 143. This is the implicit assumption: $params->{name} always takes precedence over %content.
