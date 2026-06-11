# endoxa

> **ἔνδοξα** — Aristotle's term for reputable opinions: the beliefs that survive scrutiny.

A long-term memory for coding agents that stores **beliefs, not facts**.

Most agent memory is a pile of extracted facts in a vector store. Nothing in the pile can
ever become *false*, so it silently goes stale — and semantic search makes it worse, because
the stale belief is usually the closest match to your query. endoxa treats every remembered
claim as a **belief**: something that can be superseded, retracted, attacked, or adjudicated
by later beliefs. Recall surfaces only the **frontier** — what's still standing.

## The ideas

- **Append-only, always.** A belief file is never edited or deleted. A correction is a new
  belief that *defeats* the old one; `mem forget` writes a retraction (itself defeasible —
  forgetting a retraction reinstates). The full history stays on disk for reliving.
- **Frontier resolution.** A deterministic fixpoint computes the current set. Supersession
  chains are monotonic (v3 can't revive v1); verdicts are non-monotonic (a defeated verdict
  reinstates its target). Naive refuted-lists get both backwards — run
  `cargo run -p memory-core --bin eval` to see the cases where it matters.
- **Edges are beliefs too.** Every relation (`supersedes`, `attacks`, `supports`, …) is
  reified as its own belief, so the *relation* can be argued and defeated without touching
  either endpoint.
- **Scopes from git.** Beliefs are tagged `global`, `repo:<id>`, or `repo:<id>@<branch>` —
  derived from where you run `mem`. Branch beliefs stay branch-local until `mem promote`
  lifts them into repo canon after merge.
- **Surfacing ≠ truth.** Duplicates fold behind one representative (`same-as`), corroborated
  beliefs get a capped boost (entrenchment, not self-rated confidence), and contested ones
  are flagged with what contests them. None of this touches the frontier.
- **The LLM is off the read path.** `recall` is deterministic (embeddings + frontier).
  LLM work — the linking judge, duplicate reduction, novelty "dreaming" — runs in deliberate
  background passes (`mem consolidate` / `reduce` / `dream`) against local Ollama, and
  everything above the belief files is regenerable. Markdown files are the only contract.

## The CLI

```
mem remember "<claim>" [--supersedes <slug|id>] [--global] [--ref R] [--body B]
mem recall "<query>" [--limit N]      # frontier-resolved, scope-filtered, semantic
mem expand <slug|id>                  # one hop: a belief's linked context
mem ask "<question>"                  # opt-in LLM synthesis, grounded + cited
mem forget <slug|id> [--reason R]     # retract (file kept; recall drops it)
mem consolidate | reduce | dream      # background passes: link, dedup, bridge
mem onboard [<repo>]                  # harvest git history into draft beliefs
mem promote [<branch>]                # lift merged-branch beliefs into repo canon
mem scope
```

Build with `cargo build --release` (the binary is `mem`). The store lives in
`$MEMORY_DIR` (default `~/.local/share/agentic-memory/beliefs`). Embeddings and the
judge use Ollama if present; without it, recall degrades gracefully to lexical match.

Wiring it into an agent is two lines of system prompt:

> Before a task: `mem recall "<what you need>"`. When you learn something durable:
> `mem remember "<the point>"`. State things as they are now — the system supersedes
> outdated beliefs automatically.

## What's here

- `crates/memory-core` — the deterministic heart: belief model, loader, frontier resolver,
  relation-semantics registry. No LLM, no network.
- `crates/memory-consolidate` — linkers (hint, proximity, LLM judge), the reducer, the
  novelty dreamer. Proposes edges; the Consolidator is the sole writer.
- `crates/memory-cli` — `mem` itself, plus the behavioral eval (`eval-qa`).
- `crates/memory-onboard` — tiered git-history harvest: deterministic leads → LLM-drafted
  claims → reviewed commits into the store.
- `corpus/` — nine hand-built belief corpora seeded from real codebases, designed around
  the failure modes that matter: supersession chains, verdicts-of-verdicts, open conflicts,
  confident-but-wrong beliefs.
- `docs/design/` — the design (`belief-memory.md`), edge-assignment semantics, onboarding
  tiers, and storage-backend notes (including spikes of LadybugDB and DuckDB as derived
  indexes — verdict: flat files win until a few thousand beliefs).

## Status

Research substrate under active development. The belief-file format (L0) is the durable
contract; indices, linkers, and prompts above it are deliberately disposable. Expect churn
everywhere except the files.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.
