---
id: b_3648b2535805
slug: lsp-default-offset-utf16
claim:
  kind: text
  text: >-
    The default LSP offset encoding is UTF-16, as mandated by the LSP specification base.
    The OffsetEncoding enum defaults to Utf16.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 5
  refs:
    - helix-lsp/src/lib.rs:56-65
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges: []
coord: null
---

helix-lsp/src/lib.rs: `pub enum OffsetEncoding { Utf8, Utf32, #[default] Utf16 }`. The `#[default]` attribute on the Utf16 variant makes this the fallback when a server does not specify position encoding capability.
