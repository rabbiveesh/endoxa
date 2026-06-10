# The tiered onboarding harness

Status: tiers 0–1 shipped (`mem onboard`, crate `memory-onboard`). Tiers 2–3 are design.

## Why this exists (and what it is NOT)

The empirical baseline (supersede task, 4-way A/B, 2026-06-07) showed that on small,
static, curated knowledge a hand-distilled CLAUDE.md matches our best recall run at
lower token cost. The system's value lives in knowledge a static doc can't hold:
large, dynamic, contested, or **out-of-band** — not derivable from the working tree.

So the harness is an *out-of-band harvester*, not a codebase ingester. Walking the
code and emitting descriptive beliefs would manufacture exactly the knowledge class
where a static doc wins. Instead it automates the corpus-authoring playbook
(R1–R4 rounds, see `docs/corpus-authoring.md`), tiered by cost:

| Tier | Cost | What | Status |
|---|---|---|---|
| 0 | deterministic, git-only | leads: reverts/reinstatements, rationale commits, aged debt comments, doc pointers | **shipped** |
| 1 | cheap local model | turn leads into crisp claim drafts (shipped: `--escalate N`); doc-vs-code contradiction probes; weak-model confident-and-wrong harvest (pre-defeated inoculation beliefs) | **partly shipped** |
| 2 | frontier agent | design-rationale extraction over tier-0/1 leads ("why X over Y"), verdict structuring, kludge → Deficiency beliefs | design |
| 3 | the human | guided interview seeded by tier-0–2 findings ("this HACK survived 4.2y — why?"); the only source of head-only knowledge | design |

Tier 3 doubles as the human-contribution UX (design-doc HOLE): humans are bad at
volunteering out-of-band knowledge cold, good at answering pointed questions about it.

## Tier 0: leads, deliberately not beliefs

Tier 0 has no judgment, so it must not author claims. It emits **leads** — verbatim
evidence + refs, ranked within kind — for a later tier (or the human eyeball pass) to
turn into beliefs. Nothing touches the belief store.

Sources, chosen for where out-of-band knowledge leaks into git:

- **Revert** — a supersession chain with provenance ("we tried X, backed it out").
- **Reinstate** — revert-of-a-revert / reapply: verdict-of-a-verdict, the frontier
  resolver's keystone case, found in the wild.
- **Rationale** — commit messages carrying decision markers ("because / instead of /
  turns out / b/c"); bodies weigh more than subjects; umbrella branch-merges (bodies
  stitching many sub-PRs) are downweighted, not dropped.
- **Debt** — FIXME/HACK/kludge/workaround comments aged via blame; score =
  tag-weight × (1 + age-years). A long-surviving kludge is a known-deficient-but-true
  belief carrying its forcing constraint.
- **Doc** — ADR/CHANGELOG/docs pointers: cheap escalation targets for tier 1+.

Output: `leads.json` (complete, one lead per line, greppable) + `leads.md` (grouped
report for the eyeball pass), written to the **data dir** by default
(`~/.local/share/agentic-memory/onboard/<repo>/`) — outside any repo, so
private-repo leads can't be committed by accident.

```
mem onboard [<repo>] [--out DIR] [--top N]
```

## First results (2026-06-10)

| repo | commits | leads | notes |
|---|---|---|---|
| perl-tree-sitter-lsp | 465 | 179 | rationale-dense (109): solo repo with written-out commit bodies; ~0 reverts — solo history is too clean for supersession chains |
| private-crm (gitignored) | 4,158 | 398 | debt-dense (230, kludges up to 4.2y old); rationale catch doubled once the local "b/c" idiom joined the markers |

Two early lessons:
1. **Rationale richness tracks commit-message culture**, including per-repo idiom —
   marker lists may need a per-repo extension knob.
2. **Reverts are rare in solo/squash-merge histories**; the supersession-chain
   harvest will earn its keep on multi-author repos with messier histories.

## Tier 1: drafts, still not beliefs

`mem onboard <repo> --escalate N` adds the judgment tier 0 refused to have. Selection
first (`select_for_escalation`): doc pointers excluded, TODO debt excluded (see
below), remaining debt deduped to one lead per file (a kludge cluster is one belief,
not five), equal quota per kind-group so high-scoring debt can't starve rationale.
Each picked lead gets richer deterministic
context (full commit message; the tracked code around a debt comment), then one
`chat_json` call (JUDGE_MODEL, default qwen2.5:7b — same harness as the judgment
linker) returns `{keep, claim, why, kind, confidence}`. A degenerate-output gate
(claim < 30 chars → rejected) catches truncated 7B fragments.

Output: `drafts.{json,md}` beside the leads. Committing kept drafts into the store
(author kind=agent id=<model>, directness: inferred, low source_weight) is a
deliberate separate step — the eyeball pass owns the gate for now.

Prompt lesson (first smoke test): telling the model a claim "is NOT a change summary"
made it *reject leads whose evidence is a change summary* — which is all of them.
The fix is framing the job as **extraction** ("the evidence is usually a change
description; extract the durable knowledge it reveals") plus a per-kind shape hint
("state the kludge, where it lives, and the constraint forcing it"). Rejection rate
went from 4/6 (including the two best leads) to 0/6, with grounded claims.

Two failure modes survived prompting — the 7B ceiling:
1. **TODO leads → normative junk.** The judge rephrases every TODO as a prescriptive
   claim ("the script must..."), even when explicitly told a TODO is an aspiration.
   Fix is deterministic, not prompted: TODO leads never escalate (selection skips
   them); HACK/kludge/workaround/FIXME are where real debt lives. TODOs remain in
   leads.json for human browsing.
2. **Claim/why mismatch on multi-bullet squash bodies.** The model occasionally pairs
   one bullet's claim with another bullet's why. Prompting reduced but did not
   eliminate it. This is tier-2 territory (frontier model per umbrella commit) —
   the asserted confidence is untrusted anyway (StructuralOnly default), and the
   eyeball pass catches the residue.

Division of labor that emerges: **tier 0 owns recall, tier 1 owns translation, the
gate stays human (or tier 2)** — precision comes from selection (what gets escalated),
not from prompting a small judge harder.

## Eval hook

The harness is the apparatus for the decisive experiment: onboarded memory vs an
`/init`-style CLAUDE.md vs raw, on tasks requiring the out-of-band facts. Tier-0
leads escalated through tier 1–2 produce the onboarded corpus; the existing 3-way
methodology does the rest.
