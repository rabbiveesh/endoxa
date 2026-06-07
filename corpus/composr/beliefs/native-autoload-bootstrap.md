---
id: b_3764f835f5b6
slug: native-autoload-bootstrap
claim:
  kind: text
  text: >-
    composr generates the full autoload bootstrap natively when missing (autoload.php,
    ClassLoader.php, autoload_real.php, autoload_static.php, the four data files,
    platform_check.php, LICENSE), and bundles composer/composer's InstalledVersions.php
    (MIT). When the bootstrap is already in place it fast-paths and just patches the
    classmap.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 9
  refs:
    - git:Native autoload bootstrap
    - README.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

Killing the cold-start composer call for the autoload step. The fast path is what keeps warm re-runs cheap.
