---
id: b_072089518d32
slug: r3-survivor-lsp-async-subprocess
claim:
  kind: text
  text: >-
    Helix communicates with language servers by spawning each server as a child process and
    driving the JSON-RPC protocol asynchronously over the process stdin/stdout pipes
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2020-10-15
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 4
  refs:
    - helix-lsp/src/lib.rs
    - git:8adcf5083ffc12532ecca7594a2192acd954dd3a
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.55
  asserted: 0.65
edges: []
coord: null
---

Inferable at write-time (2020-10-15): the very first helix-lsp commit (8adcf5083) created a `Client::start(cmd, args)` that called `Command::new(cmd).stdin(Stdio::piped()).stdout(Stdio::piped())` and then wrapped the stdio with an async reader loop. The comment `// TODO: impl drop that kills the process` showed the lifecycle model was already settled as subprocess-owned-by-client. The choice of subprocess-over-stdio was structurally determined: LSP clients universally use this transport because it requires no port negotiation. Has survived 418+ commits to helix-lsp/; mechanically checkable: `grep -n 'Stdio::piped' helix-lsp/src/transport.rs` (or equivalent) still shows subprocess stdio transport.
