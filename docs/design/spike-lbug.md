# Spike: LadybugDB (`lbug`) as the derived index

Probe of the Kuzu fork (crate `lbug` 0.17.1) as a **regenerable derived index** over the L0
belief files — never a second source of truth. Spike code: `crates/spike-lbug` (407 LOC, one
bin, two modes: `build` rebuilds the index from `corpus/helix` and times all queries; `cold`
opens the existing DB file and answers the adjacency query).

Environment: Ubuntu 22.04, 20 cores. Corpus: `helix` — the largest committed corpus, 56
beliefs / 37 inline edges / 6 defeated. No corpus ships a `.embeddings.json`, so vectors are
deterministic 768-d pseudo-vectors (hash-seeded xorshift); mechanics and latency were the
point, not quality.

## What worked

- **Schema mapping is natural.** `Belief` → node table (`id` PK, slug, claim, scope,
  txn_time, `defeated` bool computed once from `Graph::defeated()`, `is_relation` bool,
  `embedding FLOAT[768]`); edges → one rel table `Relates(kind, edge_id, subj, obj, defeated)`.
  The frontier becomes a plain `WHERE NOT defeated` filter. Note: the committed corpora carry
  **zero reified edge-beliefs** — all 37 edges are inline `edges:` entries, so the rel table is
  built from those (plus the reified path, kept for when the Linker lands). An edge is "in
  force" iff its asserting belief is undefeated.
- **Correctness**: adjacency (a) and current-content (c) results match the memory-core oracle
  exactly (set-equality on (kind, subj, obj) / ids).
- **Native HNSW vector index works** — but only after two non-obvious hurdles (below):
  `INSTALL VECTOR; LOAD EXTENSION VECTOR; CALL CREATE_VECTOR_INDEX(...)`, queried via
  `CALL QUERY_VECTOR_INDEX(...)`. Index build on 56×768d: 69 ms.
- **The dreaded C++ build never happened.** `lbug`'s build.rs downloads a 26.6 MB prebuilt
  static lib and only compiles the cxx bridge. Clean build of the dep + spike: **25.7 s**
  (plus a one-time ~10 s download).

## What didn't (the honest gaps)

1. **Toolchain wall on Ubuntu 22.04.** The prebuilt `lbug.hpp` does `#include <format>` →
   needs libstdc++ ≥ GCC 13. System has GCC 11/12 and no libc++ (clang 14/15/18 don't help —
   the gap is the standard *library*, not the compiler). Built with `nix shell nixpkgs#gcc13`
   (`CXX=g++`). The produced binary then **requires GLIBCXX_3.4.31 at runtime too**
   (`LD_LIBRARY_PATH` to nix's libstdc++). Both 0.17.1 and 0.16.1 hit this. A real adoption
   would pin a toolchain or vendor libstdc++ — this is the biggest operational cost found.
2. **Vector extension is dynamically downloaded** (to `~/.lbdb/extension/0.17.0/...`, 1.1 MB)
   — a network dependency at first `INSTALL VECTOR`. And it dlopens against the host binary,
   which statically embeds lbug — it fails with `undefined symbol: ...IndexAuxInfo...` unless
   the binary is linked with **`-rdynamic`** (`RUSTFLAGS="-C link-arg=-rdynamic"`). Without
   that flag: no vector index (everything else still works).
3. **Insert path is slow via literal Cypher.** Each node CREATE carries a 768-float literal;
   ~28 ms/statement even inside one explicit transaction. Prepared statements with a
   `Value::List`/array param are the untested fix (docs.rs failed to build 0.17.1's docs, so
   the param API for FLOAT[N] was not verified — recorded as a gap, not attempted).
4. **docs.rs is broken for 0.17.1** (last good: 0.16.1) — API had to be inferred from the
   Kuzu lineage. It matched (`Database::new`, `Connection::new`, `conn.query`, `QueryResult`
   as `Iterator<Item = Vec<Value>>`).

## Numbers (best-of-5 unless noted)

| Metric | Value |
|---|---|
| Clean build of `lbug` dep + spike (nix gcc13, 20 cores) | **25.7 s** wall (+ one-time 26.6 MB prebuilt download ~10 s) |
| `target/` growth attributable to spike-lbug | **+146 MB** (84.8 → 230.7 MB); bin alone 25 MB |
| Registry cache cost (`.cache/lbug-prebuilt`) | 106 MB |
| L0 source (corpus/helix/beliefs, 56 files) | 63 KB |
| DB file size (single file, incl. HNSW index) | **1.11 MiB** (~18× the L0 source) |
| Index build (sync) — open+DDL / nodes / rels | 86 ms / 1.55 s / 62 ms = **1.70 s** |
| Vector index build (56 × 768d, cosine HNSW) | 69 ms |
| (a) adjacency of busiest belief (4 edges, matches oracle) | **1.15 ms** |
| (b) 2-hop expand (Cypher, DISTINCT) | **5.7 ms** |
| (c) current content, `WHERE NOT defeated AND NOT is_relation` (50 rows, matches oracle) | **0.80 ms** |
| (d) vector top-10, native HNSW | **29.0 ms** |
| (d') vector top-10, brute-force `array_cosine_similarity` | 26.7 ms (HNSW is a wash at n=56, as expected) |
| (e) cold open: `Database::new`+conn / first query / open→answer | 60 ms / 34 ms / **95 ms** (process total 140 ms, maxRSS 78 MB) |
| Baseline: memory-core `Graph::load_dir` on same corpus | **~2 ms** (frontier resolve sub-ms) |

## Code-shape assessment

The mapping from `Graph` to the schema is genuinely pleasant — beliefs are nodes, the
frontier is a boolean column, edges are rels, and Cypher reads like the domain ("in-force
edges touching X" is a one-liner). 407 LOC total including the oracle mirror, pseudo-vector
generator and both run modes; the DB-facing core is ~120 LOC. The friction is all
operational (toolchain, `-rdynamic`, extension download), not conceptual. One design note:
escaping claims into Cypher literals is fragile; real use needs prepared statements.

## Verdict

**Mechanically viable, operationally premature — and unnecessary at current scale.** The
in-memory `Graph` cold-loads and resolves the frontier in ~2 ms; lbug's *cold-open alone* is
~95 ms, its per-query floor is ~1 ms (the price of an interpreted query engine), and its DB
file is 18× the L0 source. The cold-start win this spike probed **does not exist below
~thousands of beliefs** — the flat files ARE the fast path. What lbug buys is real but
deferred: Cypher for ad-hoc graph queries, native HNSW once corpora outgrow brute-force
cosine (~10k+ vectors), and an index that survives process restarts without re-embedding.
Revisit when (1) corpora reach 4 digits, (2) the toolchain story improves (no `<format>`
wall / no `-rdynamic` folklore), and (3) the prepared-statement param path for FLOAT[N] is
verified. Until then the derived-index slot is better served by something with a zero-cost
open — compare the DuckDB spike side-by-side.
