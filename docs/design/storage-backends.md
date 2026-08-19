# Storage backends — the seam, and the LadybugDB note

Status: note for the future (2026-06). No DB backend exists or is needed yet; this records the
decision shape so we don't re-derive it. Both candidates were SPIKED on 2026-06-11 — see
[spike-lbug.md](spike-lbug.md) and [spike-duckdb.md](spike-duckdb.md), and the verdict at the
bottom of this doc. The spike code itself was discarded (regenerable from the reports).

## The seam (done)

`memory_core::Graph` is the backend boundary. Its fields are private; every consumer — the CLI,
the linkers, the reducer, the eval bins — goes through accessors:

- `iter()` / `content()` / `relations()` — enumeration (all / propositions / edge-beliefs)
- `by_id(id)` / `get(slug)` / `resolve_ref(ref)` — point lookups
- `defeated()` / `current_content(&defeated)` — frontier resolution + the surfaced set
- `adjacency(&defeated)` — the in-force edge index (recall affordances, `mem expand`)

Nothing outside `Graph` knows beliefs live in a `Vec` parsed from `.md` files. Swapping a backend
means reimplementing `Graph` (or putting a trait behind these exact methods), not touching callers.

## The decision: a DB is a derived index, never the store of record

Per the L0 contract ("the belief *file format* is the durable part"; "every organizing structure
above the storage layer is derived and disposable"): if/when a database comes in, it is a
**regenerable index over the belief files** — sync on load or in `consolidate`/`dream`, keyed by
belief id, deletable and rebuildable at any time. It must NOT become a second source of truth;
that would buy us sync/export machinery and a second durable format for no gain.

## Candidate: LadybugDB

[LadybugDB](https://ladybugdb.com/) is the active continuation of Kuzu (Kuzu's OSS development
stopped after the Apple acquisition). Embedded, serverless, columnar property graph, Cypher,
**native vector index + full-text search**. Rust crate: [`lbug`](https://crates.io/crates/lbug)
(MIT; statically compiles ~200k lines of C++ via cmake — a heavy build dep, the main cost).

Fit: excellent. Reified edge-beliefs `(kind, subject, object)` are literally a property graph;
frontier resolution, `adjacency()`, and multi-hop `expand` map onto Cypher; the vector index could
subsume `.embeddings.json` entirely.

Alternative worth a look at decision time: DuckDB + the DuckPGQ community extension (SQL/PGQ
graph patterns) + the VSS extension (HNSW vectors) — one embedded analytical engine covering
graph + vector + FTS.

## When to actually do it

Not at the current scale (~hundreds of beliefs; full parse + frontier fold is tens of ms per
command). The trigger is when per-command O(corpus) load stops being cheap — thousands of
beliefs, or recall latency anyone can feel. Until then the seam above is the whole investment.

## Spike verdict (2026-06-11)

Both candidates were built and measured against the helix corpus (56 beliefs / 37 edges; DuckDB
also at a 5,600-belief synthetic x100). Full numbers in the spike reports. Both are mechanically
viable — schema mapping from `Graph` is natural, query results match the core oracle exactly —
and both LOSE to flat files today: cold open→first answer is 95 ms (lbug) / 7–9 ms (DuckDB) vs
~2 ms for parse-everything. Crossover is ~1–3k beliefs, and the decisive win is only the vector
path.

**If the trigger fires: DuckDB + vss, drop duckpgq.** Rationale:
- lbug's wall is the toolchain: its prebuilt header needs `<format>` (libstdc++ ≥ GCC 13 — not
  on this system; needed at RUNTIME too), plus `-rdynamic` and a runtime-downloaded vector
  extension. Fast clean build (26 s, prebuilt static lib), but the environment cost travels.
- DuckDB's wall is extension fragility: duckpgq community builds are pin-locked behind core
  (`=1.5.0`), crashed once with a DB-invalidating internal error — and recursive CTEs beat PGQ
  MATCH 3–5x anyway, so duckpgq buys syntax, not speed. vss HNSW is solid (real index scan,
  4.8 ms top-10 at 5.6k vs 13.4 ms brute) though persistence is experimental-flagged.
- Build cost: DuckDB 3m22s wall / +858 MiB target vs lbug 26 s / +146 MB.

**The reframing find:** at 5.6k beliefs the flat-file bottleneck is NOT belief parsing
(27–58 ms) — it's the 95 MB `.embeddings.json` parse (218 ms). A binary sidecar for the vector
cache closes most of the cold-start gap with zero new dependencies. Roadmap: binary vector
sidecar FIRST; DuckDB+vss only past a few thousand beliefs when real HNSW starts paying.

Side-finding: the committed corpora have zero reified edge-beliefs (all edges inline), so
`g.adjacency()` is empty on them — only live-store beliefs get reified edges via the
Consolidator. Anything traversing corpus relations must synthesize from inline `edges:`.

## Cloud addendum (2026-08-19)

The REMOTE question got its own decision: an S3-compatible bucket as the shared L0 transport
(`docs/design/remote-store.md`). It does not change this file's verdict — the bucket is the
same flat files with a network path; local dirs become caches; a future DuckDB index would sit
UNDER a synced local dir exactly as it would today.
