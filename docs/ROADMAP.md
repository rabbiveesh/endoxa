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

## Bitemporality

`txn_time` exists; `valid_time` is parsed but null everywhere. Beliefs about the
world-as-of-a-date ("the API cap was 3 months *until March*") need it, and the
onboarding currency fix above is the first consumer.

## Pluggable relation semantics

`EdgeKind::semantic()` is a hard-coded match today. The design seam is a registry
that linkers/plugins populate when they register a relation kind (Defeat / Annotate /
Collapse, and later Boost), so bring-your-own-linker extends to
bring-your-own-semantics.

## Real content ids

`content_id` is a placeholder (SipHash48). The real scheme is
`sha256(observation)[:12]` — the id hashes the observation, not the proposition, so
re-observing the same fact is a new belief and dedup stays a recall-time clustering
problem (the Reducer), not an ingest-time identity problem.

## Scale path (measured, waiting on its trigger)

From the 2026-06 backend spikes ([storage-backends.md](design/storage-backends.md)):
flat files beat embedded databases until ~1–3k beliefs, and the real cold-start
bottleneck at scale is the `.embeddings.json` parse, not belief loading. So, in order:

1. **binary vector sidecar** for the embedding cache (closes most of the gap, zero
   new dependencies);
2. **DuckDB + vss as a derived, disposable index** (graph adjacency via recursive
   CTEs, HNSW for vectors) — only past a few thousand beliefs, and never as a second
   source of truth.

## Worlds and reliving

Branch worlds already fall out of scope-filtering before frontier resolution. The
fuller story — replay the belief state as-of a time ("what did we believe when we
shipped 1.0?"), diff two worlds, relive a defeated line of reasoning — is designed
into the append-only substrate but has no surface yet.

## Evals at scale

Every A/B so far used knowledge small enough to fit a curated CLAUDE.md — the regime
where static docs are *supposed* to win, and the system still reached parity-or-better.
The decisive experiment needs a knowledge base too large for a tidy doc and full of
conflicting/superseded facts, where frontier resolution and on-demand retrieval are
the only honest options. The eval-qa harness and the corpus rounds are the apparatus;
the big-corpus substrate is the missing piece.
