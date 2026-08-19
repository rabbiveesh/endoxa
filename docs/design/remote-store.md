# Remote store — the bucket is the shared L0

**Status: SHIPPED (2026-08-19).** `mem sync` + implicit hooks; `crates/memory-cli/src/remote.rs`.

## Decision

The store's cloud transport is an **S3-compatible object bucket** (Cloudflare R2 free tier is
the reference target), NOT git and NOT a database/server.

- **Why not git:** git's write is really pull→merge→push — N concurrent sessions serialize on
  the branch head and compete. Our write pattern (append-only, content-addressed, never
  overwrite) gets nothing from git's merge machinery and pays all of its costs.
- **Why object storage fits exactly:** distinct keys are independent; a PUT of a new key needs
  no coordination, no lock, no read-before-write. Two sessions superseding the same belief
  concurrently produce two independent edge objects — and `defeated()` (not the transport)
  arbitrates, which is the frontier doing its designed job.
- **Doctrine intact:** the bucket holds the SAME L0 `.md` files. It is the flat-file store
  with a network path, not a second representation. Local dirs demote to caches of the bucket.

## Mechanics (`remote.rs`, zero new deps)

- **Auth:** hand-rolled AWS SigV4 (HMAC-SHA256 on core's FIPS 180-4 `sha256`), signatures
  pinned in tests against an independent Python implementation. Transport: shell to `curl`,
  same as memory-embed.
- **Sync = set reconciliation:** LIST (one call at n≈600, paginated) → set-diff vs the local
  dir → GET remote-only `*.md`, PUT local-only `*.md` with `If-None-Match: *` (write-once;
  412 = someone else already pushed it = success for an append-only store).
- **Embeddings, single-writer:** `.embeddings.json` is authored by the ONE box with ollama
  (`remote.role = "writer"`); everyone else pulls when the remote ETag moves. No conflict is
  possible by construction. Cloud-born beliefs lack vectors until the writer next embeds
  them; lexical fallback covers the gap.
- **Hooks:** `remember` pushes through after its write (+ self-heals earlier failed pushes);
  `recall`/`ask` pull when the last pull is older than `remote.staleness` minutes (default
  10); the worker pushes background-drawn edges at the end of each pass; `mem sync` is the
  explicit two-way form (`--dry-run`, `--status`).
- **Offline-first:** every remote failure is a one-line benign skip. No remote configured =
  exactly the old local behavior.

## Config

```toml
# config.toml
[remote]
url = "https://<account>.r2.cloudflarestorage.com/<bucket>[/<prefix>]"  # or MEM_REMOTE_URL
role = "writer"    # ONLY on the ollama box; default "reader"           # or MEM_REMOTE_ROLE
staleness = 10     # minutes between implicit pulls                     # or MEM_REMOTE_STALENESS
```

Creds: standard `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` (+ `AWS_SESSION_TOKEN`),
region `AWS_REGION` (default `auto` — what R2 wants). Cloud-session setup is: install the
static `mem` binary, export three env vars, done.

## What deliberately does NOT sync (v1)

- `.sweep-ledger.json` — worst case a cloud box re-judges a pair the laptop already vetoed
  (one wasted LLM call). Follow-up: per-judgment objects (`ledger/<pair_key>.json`) are
  append-only and write-once → durable vetoes everywhere.
- `worlds.json` — hand-edited and mutable; syncing it needs a policy, not a set-diff.
- worker state/metrics/locks — per-machine by design.

## Team memory (the tease)

A team store is a bucket prefix with shared creds. Everyone PUTs beliefs; provenance already
names the author; every reader's frontier resolves the union. Disagreement lives in the graph
as adjudicable edges — not in a merge conflict.
