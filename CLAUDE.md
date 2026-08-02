# endoxa — repo guide for Claude

**endoxa** is a *belief*-based agentic memory: it stores **beliefs** (propositions held with
provenance, an epistemic envelope, and justification edges) rather than facts. The canonical "fact"
is never stored — it is the output of an **agentic reduction** over a context-resolved neighborhood
of beliefs, computed *relative to a world* (a git-like frontier of the belief DAG). Public repo:
`github.com/rabbiveesh/endoxa` (renamed from `agentic-memory`; the CLI/crates/store still use the
old name on disk). Dual-licensed MIT OR Apache-2.0.

The load-bearing rule: **the layout is never the storage.** L0 belief files are the only source of
truth; every index/neighborhood/reduction above them is derived and disposable.

## Layer model (deps point downward only; L0 imports nothing)

```
L5 Surfacing       inject-to-agent · show-to-human
L4 Reduction cache (world, context-sig, input-heads) → reduced output   [disposable]
L3 Reduction       agentic: neighborhood → consensus belief (LLM reducer)
L2 Neighborhood    (context, world, indices) → belief-set   ← the "lens" layer
L1 Indexing        embeddings · temporal · author · edge-graph   [derivable]
L0 Belief Log      append-only Merkle DAG · provenance · justification  [SOURCE OF TRUTH]
```

## Where things live

- **`crates/memory-core`** — the deterministic, no-LLM heart. `Graph` (THE backend seam; fields
  private, all access via `iter/content/relations/by_id/get/current_content/adjacency`),
  `Belief::parse` (hand-rolled zero-dep frontmatter parser), `EdgeKind` + `Semantic` registry, and
  **`defeated()`** — the frontier resolver (alternating fixpoint; two defeat modes: `supersedes` is
  monotonic version-chain, `adjudicates`/`retracts` are non-monotonic verdicts → verdict-of-a-verdict
  reinstates). **Worlds machinery**: `World` + `defeated_with(suppress)`/`defeated_in(world)`
  (suppress-then-refixpoint — V3's load-bearing op), `as_of(txn_time)` (bitemporal replay),
  `frontier_flips` (world/relive diff). The `Linker` trait + value types live here (impls in
  `memory-consolidate`).
  - `src/bin/eval.rs` — deterministic keystone eval: frontier vs naive refuted-list (no LLM).
  - `src/bin/recall.rs` — recall harness.
- **`crates/memory-cli`** (`mem`) — the user surface. Subcommands: `remember`, `recall`, `expand`,
  `ask` (the one LLM-on-read path), `forget` (reified `retracts` edge, non-destructive), `promote`
  (lift a branch's beliefs into repo canon), `consolidate` (LLM judge draws edges), `reduce`
  (duplicate `same-as` fold), `dream` (REM/novelty bridge pass), `review` (frontier-adjudication queue
  for candidate `depends_on`), `link <s> <kind> <o>` (author a durable `frontier@1` edge), `debt`
  (known-debt query + `blocked_on` auto-resurface), `onboard [--tier2]`, **`world [list|show|diff]`**
  (parallel realities from `worlds.json` in/beside the store — corpus format verbatim), **`relive
  <as-of-time>`** (bitemporal replay diffed vs now), **`ask --world <w>`** (world-relative reduction:
  suppress→refixpoint + the world's assumption threaded into the prompt). Runs per-invocation in a repo
  so it sees LOCAL git state → derives the active scope. Commands print the next step (legible without
  docs). `src/bin/eval-qa.rs` is the **template harness** for any corpus+LLM evaluation.
  `src/bin/eval-worlds.rs` is the **worlds keystone**: deterministic V3-reachability re-proof over
  every worlds.json corpus (CI-able, no LLM), plus `--llm` for the N6 fixture-divergence graduation gate.
- **`crates/memory-embed`** — ollama-backed embeddings (shells to `curl`, no HTTP crate) + on-disk
  vector cache. `Ollama::from_env()`, `.embed()`, `chat_json(url,model,system,user)`,
  `load_cache(dir,model)` / `save_cache`. Embed model `nomic-embed-text` (asymmetric: docs prefixed
  `search_document: `, queries `search_query: `).
- **`crates/memory-consolidate`** — Linker impls (proximity heuristic + LLM judge), Reducer
  (duplicate fold), NoveltyDreamer, the `Consolidator` (SOLE edge writer — linkers only *propose*).
- **`crates/memory-onboard`** — deterministic git-history harvest → lead files → judge-drafted claims.

## Data (for evaluation)

- **Corpora** `corpus/<name>/beliefs/*.md` — hand-authored, gold-edged belief sets for: composr (25),
  helix (56), perl-lsp (42), perl-openapi-client (41), private-crm (105, **gitignored** — private),
  robot-game (53), sql-abstract (50), tree-sitter-perl (9), tree-sitter-pod (35), zed-perl (26).
  ~442 committable content beliefs. Per-corpus embedding cache at `corpus/<name>/.embeddings.json`.
  - **`worlds.json`** (helix + composr only): `{ worlds:{name:{default?,assumption,suppress?:[slug]}},
    reduction_fixtures:[{query,neighborhood:[slug],expected_by_world:{world:text},status}] }`. A
    world suppresses defeating edges from `suppress` slugs and recomputes the frontier → parallel
    realities. `reduction_fixtures` are the GOLD divergent answers. The deterministic substrate
    (dissent beliefs reachable per world) is verified in-tree by `eval-worlds`; the LLM half
    (reducer *selects* world-relatively) runs via `eval-worlds --llm` (needs ollama) — fixtures
    stay TARGET until that pass is recorded. Reference resolver: `corpus/_worlds.py` (NB: it is
    simplified — no `retracts`, and non-monotonic supersedes; core's `defeated()` is the truth).
  - Corpora are append-only: never patch a belief, append a correction that defeats it.
- **Real fact store** `~/.local/share/agentic-memory/beliefs/` (= `$MEMORY_DIR`, default) — the
  user's LIVE 330 beliefs + `.embeddings.json`. **READ-ONLY in experiments.** Real beliefs are
  sparser-enveloped than the corpus (mostly `asserted: null`, `source_weight≈0.8`,
  `directness: stated`), so **currency (defeated-vs-current) is the usable label there**, not asserted.

## Gold / oracle

- `g.defeated()` is the label source: a belief in it LOST a verdict / was superseded (loser); the
  current source of a defeating edge is the winner. Derive winner/loser pairs programmatically.
- **Do NOT use the naive refuted-list** (any incoming `adjudicates`) — it's wrong for chains. Use
  `defeated()`.
- `observation_count` is in frontmatter but **not parsed into `Belief`** — use incoming-`supports`
  count (the system's real entrenchment signal) or extend the parser.

## Build / run / LLM

- `cargo build --workspace` ; `cargo run -q -p memory-core --bin eval` (deterministic, fast).
- LLM evals need **ollama** (confirmed available): `qwen2.5:7b` (better, ~seconds/call on CPU),
  `qwen2.5:3b` (faster), `nomic-embed-text` (embeddings). `JUDGE_MODEL`/`ASK_MODEL` env override.
- CI: `cargo test` on push/PR (`.github/workflows/ci.yml`).

## Design docs & open questions

- **`docs/design/belief-memory.md`** is the main design doc (the thesis, layers, edge taxonomy,
  worlds, recall, confidence, benchmarks). Open questions are marked `HOLE §N`.
- `docs/design/edge-assignment.md` — the Linker mechanism (author-local, memory-maintains-global;
  every assertional edge reifies into its own defeasible edge-belief; edges are a regenerable derived
  layer, human edges are durable anchors).
- `docs/design/storage-backends.md` + `spike-{lbug,duckdb}.md` — DB-as-derived-index decision
  (verdict: flat files win today; binary vector sidecar first, DuckDB+vss past a few thousand beliefs).
- `docs/design/open-questions-eval.md` — the empirical settlement of the HOLEs (the experiment map +
  verdicts V1–V6). Read this before re-opening any HOLE.
- `docs/design/next-experiments.md` — the ordered action queue the verdicts force (implement winners
  N1–N4, close gaps N5–N7, run the Linker A/B N8). The "what to do next" doc.

## Conventions

- Append-only by construction: beliefs are never mutated or deleted, only superseded/retracted by a
  new belief (non-destructive — prior worlds stay relivable).
- Edges reify: an assertional edge is its own defeasible belief authored by a *Linker* (not the
  proposition's author). Only `derived_from` (self-provenance) stays inline.
- Frontier-relative everything: "current" means "undefeated along this world's frontier."
