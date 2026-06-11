# Spike: DuckDB (+ DuckPGQ + VSS) as the derived index

Status: spike result (2026-06). Code in `crates/spike-duckdb` (disposable; 696 LOC bin + 18
LOC manifest). Companion to `docs/design/storage-backends.md` (the decision: a DB is a
regenerable index over the L0 belief files, never a second source of truth) and to the
LadybugDB spike for side-by-side comparison.

## Setup

- `duckdb` Rust crate, `bundled` feature (statically compiles DuckDB's C++); pinned
  `=1.10500.0` → core **v1.5.0** (see "extension story" for why not latest).
- Corpus: `helix` (largest committed corpus, 56 beliefs / 37 edges, 6 defeated), loaded via
  `Graph::load_dir`; real 768-dim `nomic-embed-text` vectors from
  `corpus/helix/.embeddings.json`, hash-seeded pseudo-vectors elsewhere. A `scale` knob
  replicates the corpus into disjoint copies (x100 → 5,600 beliefs / 3,700 edges) to probe
  latency at the scale where a DB would actually matter.
- Schema: `beliefs(id PK, slug, claim, scope, txn_time, is_relation, defeated, embedding FLOAT[768])`,
  `relations(id, kind, subject, object, defeated)`, `relations_inforce` (see PGQ notes), and
  a tiny `meta` table so the cold path needs zero L0 access.
- The `defeated` flag is COMPUTED BY `Graph::defeated()` at sync time. Defeat semantics
  (monotonic supersession vs non-monotonic verdicts) stay in memory-core; the DB stores the
  resolved frontier. Fits the derived-index contract exactly — and means the index is stale
  the moment a belief lands, until the next sync; sync cost is the write-path number.
- Caveat found while building: the nine committed corpora contain **zero reified
  edge-beliefs** — every edge is inline (`edges:` frontmatter), so `g.relations()` and
  `g.adjacency()` are empty over them. The spike synthesizes relation rows from inline edges
  (in force iff the owning belief is undefeated) and verifies SQL results against a Rust
  reference built the same way. Live `mem` writes do create edge-beliefs; corpora predate
  reification.

## The extension story (the big unknown — answered)

TL;DR: **both extensions work from the bundled Rust build, but only after a version-pinning
dance, and duckpgq crashed once at runtime.**

- Extensions are version-built: a binary must exist for your exact core version + platform,
  fetched over the network at `INSTALL` time (one-time, cached in
  `~/.duckdb/extensions/<ver>/<platform>/`; air-gapped CI needs pre-seeding).
- Latest crate 1.10503.1 (core v1.5.3): `vss` exists, **`duckpgq` does not** (404 on
  community-extensions.duckdb.org). Community builds lag core releases.
- Core v1.5.1: a duckpgq binary EXISTS but is **broken** — `LOAD` throws
  `Initialization function "duckpgq_duckdb_cpp_init" ... threw an exception: "basic_string::_M_create"`.
  "The file is there" ≠ "it loads"; you find out at runtime.
- Core v1.5.0: `vss` + `duckpgq` both INSTALL and LOAD cleanly — from the official CLI *and*
  from the bundled duckdb-rs build (no signature/platform issues).
- **duckpgq flakiness**: on one run, re-executing the identical `GRAPH_TABLE` MATCH on the
  same connection died with `INTERNAL Error: Attempted to dereference unique_ptr that is
  NULL` inside the extension — and a DuckDB INTERNAL error **invalidates the whole database
  instance** (every later query on the process fails). Not reproducible since (~300
  subsequent executions across orders and sessions, including a dedicated `stress` mode),
  but a crash that poisons the connection existing at all is the reliability tier you're
  buying. Related: the v1.5.0 CLI crashed on close (`checkpoint ... dereference shared_ptr
  that is NULL`) after a failed PGQ bind. duckpgq is community-tier software.
- VSS HNSW persistence sits behind `SET hnsw_enable_experimental_persistence = true` (the
  docs warn of corruption on unclean shutdown — the WAL does not cover custom indexes). A DB
  file with a persisted HNSW index reopens fine without an explicit `LOAD vss` (official
  extensions autoload). Verified after an actual process panic: file reopened complete.
- duckpgq property graph DDL **is persisted** in the DB file (a fresh connection can MATCH
  after just `LOAD duckpgq`) — undocumented but observed.

### duckpgq mapping notes

Reified edge-beliefs `(kind, subject, object)` are literally a PGQ edge table; the mapping
is the most natural part of the whole spike:

```sql
CREATE PROPERTY GRAPH bg
  VERTEX TABLES (beliefs)
  EDGE TABLES (relations_inforce SOURCE KEY (subject) REFERENCES beliefs (id)
                                 DESTINATION KEY (object) REFERENCES beliefs (id)
               LABEL relates);

SELECT DISTINCT oid FROM GRAPH_TABLE (bg
  MATCH (a:beliefs WHERE a.id = ?)-[r:relates]-{1,2}(b:beliefs)
  COLUMNS (b.id AS oid)) WHERE oid <> ?;          -- quantified undirected: works, reads well
```

BUT an edge predicate inside a quantified pattern does not bind
(`-[r:relates WHERE NOT r.defeated]-{1,2}` → `Binder Error: Referenced table "r" not found`),
so the in-force filter must be materialized as its own edge table (`relations_inforce`,
frontier resolved in Rust, dangling endpoints dropped). One more derived table to keep in
sync.

## Numbers

Machine: linux x86_64, i9-13900H (20 threads). All query latencies best-of-N (N=10–20),
release build.

**Build cost (the price of admission)**
- Clean-tree `cargo build --release -p spike-duckdb` (`cargo clean` first, crates cached):
  **3m22s wall / 51m24s user CPU** on an i9-13900H — the bundled DuckDB C++ is the long pole
  and parallelizes very well (~15x). On a few-core CI box expect 15–30+ min.
- target/ growth attributable to the duckdb dep: **1.6 MiB → 858 MiB**.
- arrow et al. come along as mandatory deps of duckdb-rs; extension cache in `~/.duckdb` adds
  62 MiB (duckpgq + vss, ~31 MiB each).

**Index size + sync (helix; x1 = 56 beliefs / 37 edges, x100 = 5,600 / 3,700)**

| metric | x1 | x100 |
|---|---|---|
| DB file | 2.01 MiB | 33.3 MiB |
| sync (tables + inserts + in-force table) | 38 ms | 1.41 s |
| HNSW index build | 18 ms | 4.96 s |

- Sync first cut (embeddings as inline SQL literals) was **11.9 s** at x100 — ~88% of it
  SQL-parsing 38 MB of float literals. Switching embeddings to a temp TSV +
  `read_csv`/vectorized `UPDATE` got 1.41 s. Appender/Arrow would cut it further; the naive
  path is a trap.
- DB file is ~12x the L0 source at x1 (beliefs/*.md ≈ 170 KiB) but the embeddings dominate:
  5,600×768 f32 ≈ 16.4 MiB raw, so 33 MiB total is ~2x raw payload. Fine.

**Query latencies (warm connection)**

| query | x1 (56) | x100 (5,600) | correct? |
|---|---|---|---|
| (a) adjacency of probe (degree 4) | 0.23 ms | 0.31 ms | = Rust adjacency ✓ |
| (b) 2-hop expand, recursive CTE | 1.13 ms | 1.31 ms | = Rust BFS (7 ids) ✓ |
| (b) 2-hop expand, PGQ MATCH | 3.2 ms | 6.5 ms | = Rust BFS ✓ |
| (c) current content (`NOT defeated`) | 0.14 ms | 0.72 ms | = current_content×scale ✓ |
| (d) vector top-10 brute (`array_cosine_similarity`) | 4.1 ms | 13.4 ms | top-1 = self ✓ |
| (d) vector top-10 HNSW (plan shows HNSW_INDEX_SCAN) | 4.6 ms | 4.8 ms | top-1 = self ✓ |

- The plain recursive CTE **beats PGQ MATCH 3–5x** at this scale (PGQ builds a CSR per
  query). PGQ buys syntax, not speed, until graphs get much larger.
- The ~4 ms floor on vector queries is mostly parsing the 768-float query literal
  (~7 KB of SQL); proper param binding for ARRAY would cut it.
- HNSW only starts paying at thousands of rows (13.4 → 4.8 ms at 5,600; nothing at 56).

**(e) Cold open (the probed win) vs the L0 baseline**

| | x1 (56) | x100 (5,600) |
|---|---|---|
| DuckDB: open file | 5–8 ms | 6–8 ms |
| DuckDB: meta + adjacency query | ~1.1 ms | ~1.2 ms |
| **DuckDB cold → adjacency answer** | **~7–9 ms** | **~8–9 ms** |
| L0: `Graph::load_dir` + `defeated()` | 0.3–2.7 ms | 27–58 ms |
| L0: embeddings JSON parse (serde) | 1.7 ms (380 KB) | **218 ms** (95 MB) |
| L0: brute cosine top-10 in Rust | 0.02 ms | 3.0 ms |

- Cold vector query through the DB: 1.7–2 ms at x1, 18–21 ms at x100 (the query vector came
  from a subquery, which defeats the HNSW constant-fold — with a literal/bound vector it's
  the warm ~5 ms).
- **At today's scale (56–500 beliefs) the DB loses**: parsing the entire corpus is faster
  than opening the DuckDB file. Crossover for the graph-only path is ~1–3k beliefs, and even
  at 5,600 it's only ~4x. The decisive win is the **vector path**: .embeddings.json is
  95 MB / 218 ms of JSON parse at 5,600 beliefs and grows linearly, vs a flat ~12 ms
  open+HNSW. (Equally honest alternative: store vectors in any binary format and the L0 gap
  mostly closes — the JSON is the problem, not the absence of a database.)

## Code shape

- Mapping `Graph` → tables is trivial and natural: beliefs are a row, reified relations are
  an edge row, `defeated()` becomes a boolean column, `current_content` is a WHERE clause,
  `adjacency` is one indexed-ish SELECT, 2-hop is a CTE or a genuinely pretty PGQ MATCH.
  Nothing fought the model; the relational shape fit on the first try.
- The spike is 696 LOC, of which the honest core (schema + sync + queries) is ~250; the rest
  is benchmarking, correctness checks against `Graph`, scale replication, and
  extension-failure handling. A production backend behind the `Graph` seam would be small.
- Frictions, all recorded above: extension version pinning; embeddings need a bulk path (not
  literals); in-force edges need their own table for PGQ; quantified-pattern predicates
  don't bind; one INTERNAL-error crash that poisons the connection; HNSW persistence is
  officially experimental.

## Verdict

**Viable mechanically, wrong cost profile for this system today.**

- Everything the derived index needs *works*: regenerable on-disk index, correct frontier
  filtering, graph traversal, real HNSW vector search, ~8 ms cold opens, all verified
  against `Graph`.
- But the price is a ~1 GiB / tens-of-CPU-minutes build dependency, a version-pinning dance
  to keep duckpgq loadable (currently stuck at core v1.5.0, three releases behind), and a
  community-tier graph extension that has crashed once in this spike and whose MATCH is
  slower than a recursive CTE at our scale. DuckPGQ specifically adds nothing we need: drop
  it and plain DuckDB + vss covers the spike, since 2-hop CTEs are faster anyway.
- At the current corpus scale (tens to hundreds of beliefs) the L0 files beat the DB cold;
  the trigger from storage-backends.md ("thousands of beliefs, or recall latency anyone can
  feel") has not fired. The first real pressure point will be the *embeddings JSON*, and the
  cheapest fix for that is a binary vector sidecar, not a database.
- If/when the trigger fires: DuckDB + vss (no duckpgq, CTEs for traversal) is a credible
  backend; weigh it against the LadybugDB spike's numbers, especially build cost vs Cypher
  ergonomics and whether its vector index is production-tier rather than
  experimental-persistence-tier.
