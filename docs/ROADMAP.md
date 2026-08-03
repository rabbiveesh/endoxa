# Roadmap

Direction, not promises. Ordered roughly by how much each item is already forced by
evidence we've collected. The invariant underneath all of it: the L0 belief-file
format is the durable contract; everything listed here is built (and rebuildable)
above it.

## Onboarding currency (forced by the verified dry run)

Tier-1 drafts are ~100% faithful but only ~68% current, and every stale draft shares
one cause: claims harvested from pre-rewrite commits get stated as current-state with
`valid_time: null`. The fix is the system's own machinery, not better prompting:

- stamp `valid_time` from the source commit date (or phrase drafts as dated episodes);
- HEAD-verify current-state claims before `--commit`;
- let a rewrite-event belief `supersede` the older claims it invalidates, so the
  frontier does the dropping.

## Onboarding tiers 2–3 (designed, unbuilt)

- **Tier 2 (frontier agent):** design-rationale extraction over tier-0/1 leads
  ("why X over Y"), verdict structuring, kludge → deficiency beliefs with their
  forcing constraints.
- **Tier 3 (the human):** a guided interview seeded by tier-0–2 findings ("this HACK
  survived 4.2 years — why?"). Humans are bad at volunteering out-of-band knowledge
  cold and good at answering pointed questions; this is also the human-contribution UX.

## Bitemporality — parsing + the relive lens SHIPPED (2026-08-03)

`valid_time {start,end}` now parses into `Belief.valid_from/valid_until` and
`Belief::valid_at(date)` is the second axis ("was it true OF THE WORLD then",
vs `as_of`'s "did we BELIEVE it then"). `mem relive` separates them: a
believed-then belief whose window excludes the replay date is flagged as
describing a different era, not as wrong. Still open: the onboarding currency
fix above (stamp `valid_time` from source-commit dates) — the first *writer*
of validity windows outside hand-authored corpora.

## Pluggable relation semantics

`EdgeKind::semantic()` is a hard-coded match today. The design seam is a registry
that linkers/plugins populate when they register a relation kind (Defeat / Annotate /
Collapse, and later Boost), so bring-your-own-linker extends to
bring-your-own-semantics.

## Real content ids — SHIPPED (2026-08-03)

`content_id` is now `b_` + `sha256(observation)[:12]` (hand-rolled FIPS 180-4 in
core, NIST-vector pinned; zero deps). The id hashes the observation, not the
proposition, so re-observing the same fact is a new belief and dedup stays a
recall-time clustering problem (the Reducer), not an ingest-time identity
problem. Old SipHash ids coexist — ids are opaque and the store is append-only.

## Scale path (measured, waiting on its trigger)

From the 2026-06 backend spikes ([storage-backends.md](design/storage-backends.md)):
flat files beat embedded databases until ~1–3k beliefs, and the real cold-start
bottleneck at scale is the `.embeddings.json` parse, not belief loading. So, in order:

1. ~~**binary vector sidecar** for the embedding cache~~ — SHIPPED (2026-08-03):
   `.embeddings.bin` (length-prefixed LE, magic MEMVEC1) is preferred on load when
   at least as new as the JSON; the JSON stays the durable committable authority
   and transparently rebuilds the sidecar when hand-edited. Zero new dependencies.
2. **DuckDB + vss as a derived, disposable index** (graph adjacency via recursive
   CTEs, HNSW for vectors) — only past a few thousand beliefs, and never as a second
   source of truth.

## Worlds and reliving — surface SHIPPED (2026-08-02)

Branch worlds already fall out of scope-filtering before frontier resolution. The
fuller story now has its surface: `mem world [list|show|diff]` resolves named
worlds (`worlds.json` in/beside the store — the corpus format verbatim) via
suppress-then-refixpoint in core (`Graph::defeated_in`); `mem relive <as-of-time>`
replays the belief state as-of a transaction time and diffs it against now
("what did we believe when we shipped 1.0?" — including reliving a defeated line
of reasoning, since later defeats don't exist yet in the replay); and
`mem ask --world <w>` reduces world-relatively (assumption threaded into the
prompt). `eval-worlds` keeps the substrate honest deterministically; its `--llm`
mode passed 6/6 fixture×world cells on 2026-08-03 (claude:sonnet reducer via
the pluggable chat-provider seam — see next-experiments.md N6 for caveats).
Remaining: `valid_time` (world-as-of-a-date, distinct from txn replay) per
Bitemporality above.

## Evals at scale

Every A/B so far used knowledge small enough to fit a curated CLAUDE.md — the regime
where static docs are *supposed* to win, and the system still reached parity-or-better.
The decisive experiment needs a knowledge base too large for a tidy doc and full of
conflicting/superseded facts, where frontier resolution and on-demand retrieval are
the only honest options. The eval-qa harness and the corpus rounds are the apparatus;
the big-corpus substrate is the missing piece.
