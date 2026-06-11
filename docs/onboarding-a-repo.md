# Onboarding a repo

How to take endoxa to a repository it has never seen — from empty store to a working
belief memory. (For the *design* of the onboarding tiers, see
[design/onboarding.md](design/onboarding.md); this is the operator's guide.)

## 0. Should you onboard at all?

Measured rule of thumb (the "legibility law"): **memory's payoff is inversely
proportional to repo legibility.** On a repo with rich ADRs, written-out commit
bodies, and a maintained backlog, onboarded memory reaches quality *parity* with a
raw agent at ~10–25% fewer tokens — nice, not decisive. On a repo with thin docs and
a messy multi-author history, it's decisive: in our A/Bs the raw arm missed
cross-cutting policy constraints entirely and paid +25% tokens re-mining what memory
already knew. A legible repo has, in effect, already onboarded itself.

So: thin docs → onboard now. Rich docs → consider skipping the harvest and just
wiring the day-to-day loop (§3); the store will accrete as you work.

## 1. Wire the agent

Build and install the CLI (`cargo install --path crates/memory-cli`), then give your
agent two standing instructions (e.g. in `CLAUDE.md`):

```markdown
- Before a task: `mem recall "<what you need>"`.
- When you learn something durable: `mem remember "<the point, in one line>"`
  — one fact per call; detail goes in `--body`. Cross-repo: add `--global`.
- State things as they are now. Don't look up or update old memories — the
  system supersedes outdated beliefs automatically.
```

Scope is automatic: run from inside the repo and beliefs tag `repo:<name>` (or
`repo:<name>@<branch>` on a feature branch). No per-repo setup or config.

## 2. Harvest the history (the tiered pass)

The harvester is an **out-of-band** miner: it looks for the knowledge a fresh reading
of the working tree can't recover — reverts and reinstatements (supersession chains in
the wild), decision rationale in commit bodies, long-surviving HACK/kludge comments
with their forcing constraints.

```sh
# Tier 0 — deterministic, git-only. Emits LEADS (evidence + refs), not beliefs.
mem onboard                       # writes leads.{json,md} to the data dir, outside the repo

# Eyeball pass: read leads.md. This is where you learn what kind of history you have.

# Tier 1 — a cheap local model (Ollama) turns the best leads into claim DRAFTS.
mem onboard --escalate 30         # writes drafts.{json,md} beside the leads

# Review drafts.md; delete bad lines from drafts.json. The gate is yours.

# Commit the kept drafts into repo canon (idempotent per claim):
mem onboard --commit
```

Onboarded beliefs land marked as what they are: authored by the model, directness
`inferred`, conservative source weight. The frontier treats them accordingly.

**Know the failure mode.** In a verified dry run the drafts were ~100% faithful to
their evidence (zero fabrications) but only 68% current — every stale draft traced to
one mid-history rewrite whose consequences the harvester didn't propagate backwards.
Until tier 1 stamps `valid_time` and HEAD-verifies (see the [roadmap](ROADMAP.md)),
**check current-state claims against HEAD during the drafts review**, especially for
claims sourced from commits older than your last big rewrite.

## 3. Link, fold, and settle

```sh
mem consolidate        # the judge draws supersedes/refines/supports/attacks edges
mem reduce --dry-run   # preview duplicate folds, then run without --dry-run
mem dream              # optional: probe far pairs for cross-domain bridges
```

These are deliberate background passes — nothing here runs on the recall hot path.
Then sanity-check the result: `mem recall` something you know the history contains,
and `mem expand` a hit to see its linked context.

## 4. Day-to-day

The harvest is a bootstrap, not the steady state. The store earns its keep by
accreting during work: `remember` when something durable surfaces, `recall` before
tasks, and let contradictions resolve themselves — a new belief that supersedes an
old one defeats it at recall time, no curation required. After a feature branch
merges, `mem promote` lifts its branch-scoped beliefs into repo canon.

## 5. The reverse direction

Once memory has accreted, run the **legibility audit**
(`.claude/skills/legibility-audit/`): it asks what memory knows that the working tree
doesn't record, and proposes ADRs/comments/runbook text to close the gaps. The
systematic blind spots it finds are worth knowing about in advance: *negative
knowledge* (deliberate removals leave no artifact saying "don't reintroduce X") and
ops/release procedure (it accretes in memory but no code change ever carries it
in-tree). The audit doubles as a drift check — beliefs the tree has overtaken get
superseded in-store.
