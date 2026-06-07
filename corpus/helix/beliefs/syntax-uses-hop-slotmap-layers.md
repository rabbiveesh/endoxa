---
id: b_adb55f6443eb
slug: syntax-uses-hop-slotmap-layers
claim:
  kind: text
  text: >-
    The Syntax struct stores injection layers in a HopSlotMap keyed by LayerId. The root
    layer is always present; child injection layers are added for embedded languages.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 5
  refs:
    - helix-core/src/syntax.rs:1075-1079
    - helix-core/src/syntax.rs:1542-1551
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

helix-core/src/syntax.rs: `pub struct Syntax { layers: HopSlotMap<LayerId, LanguageLayer>, root: LayerId, loader: Arc<ArcSwap<Loader>> }`. Each LanguageLayer holds a tree-sitter Tree, depth, parent LayerId, and a set of byte ranges.
