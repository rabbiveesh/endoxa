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

### Verified dry run (robot-game, 2026-06-10): the failure mode is staleness, not hallucination

First fully-verified tier-0+1 pass (175 commits → 54 leads → 31 drafts, every claim
checked against its evidence AND against HEAD):

- **Faithfulness ~100%**: 31/31 claims trace to their evidence; zero fabrications.
  The full-commit-message context fetch is what makes the 7B reliable (e.g. it pulled
  "stops as soon as placement converges" from the body, beyond the lead's excerpt).
- **Currency 68%**: 21/31 commit-worthy; 10/31 stale — and ALL ten share one cause:
  a mid-history rewrite (the Macroquad migration deleted every JS-era mechanism the
  March commits describe). The harvester even drafted the rewrite itself as a claim
  and still didn't apply it to the older claims it invalidates.
- Tier-0 report noise is structural, not content: doc-pointer leads were 43% of the
  report; rationale leads were clean.

Implication: tier 1's missing piece is exactly the system's own machinery. Staleness
shouldn't drop claims — claims from pre-rewrite commits should land with `valid_time`
ended and a `supersedes` edge from the rewrite-event belief, letting the frontier do
its job. Cheapest interim: a HEAD-verification gate before `--commit`.

## Eval hook

The harness is the apparatus for the decisive experiment: onboarded memory vs an
`/init`-style CLAUDE.md vs raw, on tasks requiring the out-of-band facts. Tier-0
leads escalated through tier 1–2 produce the onboarded corpus; the existing 3-way
methodology does the rest.

### First A/B (2026-06-10, private-crm, n=1)

36 tier-1 beliefs committed into repo canon. Task (same prompt both arms, read-only
design exercise): a by-day repeat-purchase dashboard report over 6 months with
per-customer streaks — booby-trapped with harvested canon: the 6-month ask collides
with a learned 3-month by-day cap; the streak SQL invites a window-function bug the
team already paid for (`lag` reads the whole partition; use `last ... exclude current
row`). Arm M = multi-query `mem recall`; arm R = identical but barred from `mem`.

| rubric item | M (memory) | R (raw) |
|---|---|---|
| 3-month by-day cap flagged | **yes** — recall hit → found the enforcing `ReasonableTimePeriod` type in code | **missed** |
| `lag` trap avoided | yes (recall + reuse advice) | yes — re-mined from git, citing the same commit |
| refund/discount-drift reconciliation | yes (recall) | no |
| metrics rollups can't serve it (no customer dim, once-daily) | yes (recall) | not addressed |
| codebase grounding | excellent | excellent, deeper in places (buyer-hash identity, tz cast) |
| cost | 49k tok, 40 tools, 8.2 min | 61k tok, 58 tools, 12.6 min |

Findings beyond the score:
1. **Tier-0 knowledge is technically in-band** — it all lives in git history, and a
   diligent raw agent re-mined the code-local lesson (the `lag` commit) the expensive
   way (+25% tokens, +50% wall-clock). Memory's edge is *direction*, not exclusivity.
2. **The miss pattern matches the prior 3-ways**: R recovered pitfalls adjacent to
   the code it read, but missed the cross-cutting *policy* constraint (the 3-month
   cap lives in a Types module far from the report code). Zero-footprint /
   policy-shaped knowledge is where memory keeps winning.
3. **Memory acts as an index into the code**: the recall hit ("backend reports are
   limited to three months") is what sent M to the enforcing type — the belief
   didn't replace code reading, it aimed it.

### Second A/B (2026-06-10, perl-tree-sitter-lsp, 2 tasks, un-gamed)

Same protocol, but tasks were picked by an independent agent from the repo's OWN
backlog (`docs/qa-design-items.md` D1: arbitrary-depth inheritance resolution;
B2/B3: export-surface model) without reading git history or the memory. 36 tier-1
beliefs committed to repo canon first. Fairness notes: the backlog doc (readable by
both arms) states the recommended direction for both items, and arm M's top recall
on D1 was a pre-existing global-scoped lesson (frozen-edge/query-time), not an
onboarded belief.

Result: **quality parity, modest cost edge to memory.**

| | task 1 M | task 1 R | task 2 M | task 2 R |
|---|---|---|---|---|
| tokens | 61k | 65k | 40k | 51k |
| tool calls | 18 | 22 | 16 | 22 |
| verdict | parity — complementary strengths | | parity — complementary strengths | |

Per-task texture: on D1, M's unique contribution was recalled scar tissue (don't
memoize a `None` computed while a hop was unresolved — query-memo poisoning; keep
synthesized parent edges transient, never pushed during the build fold); R's was an
integrated fix for the non-primary-package cache hole plus a measure-before-deleting
migration. On B2/B3 both arms discovered the surface model already substantially
exists; M caught two subtle semantic interactions (selector negation × bare-`use`
`export_ok` leniency; "lists a name ≠ defines the sub" origin verification), R had
the broader honest-gaps inventory. Both R arms recovered most canon from the repo's
ADRs, design docs, and dense commit bodies.

### The emerging law

Memory's delta tracks how out-of-band the needed knowledge is — i.e. it is
inversely proportional to repo legibility:

- **private-crm** (sparse docs, messy multi-author history): memory decisive — the
  raw arm missed the policy constraint entirely and paid +25% tokens.
- **perl-tree-sitter-lsp** (rich ADRs, backlog docs with recommendations, written-out
  commit bodies): parity on quality; memory worth ~10–25% fewer tokens and ~25%
  fewer tool calls, plus occasional scar-tissue assists nothing in-tree records.

This is the original hypothesis, now measured from both sides: the onboarding
harness earns its keep exactly where documentation is thin. A legible repo has, in
effect, already onboarded itself — its ADRs are hand-authored beliefs in-tree.

## The legibility audit (tier 2, inverted)

The law inverts into a product: instead of extracting more knowledge into memory,
audit **what memory knows that the tree doesn't record** and propose ADRs/comments
to close the gaps. Shipped as the `legibility-audit` skill
(`.claude/skills/legibility-audit/SKILL.md`); the audit is read-only, the deliverable
is proposed text plus a drift check (beliefs the tree now contradicts → supersede).

First run (perl-tree-sitter-lsp, 55 beliefs, 2026-06-10): **45 legible / 6 partial /
4 gap** — quantifying the same legibility the A/B measured behaviorally. The finding:
all four gaps form ONE cluster — release & publishing operations ("the repo has zero
words about how to ship itself"). Ops knowledge accretes in memory across sessions
but never lands in-tree, because no code change carries it there. Remediations
drafted (RELEASING.md runbook; an ADR promoting the per-instance query-time-cross-file
rule to a stated principle). Drift check earned its keep immediately: caught one
belief the tree had overtaken (SUPER::X typing, fixed the day after the belief was
recorded) — superseded in-store, branch-scoped, promoting at merge. Report:
`<data-dir>/onboard/perl-tree-sitter-lsp/legibility-audit.md`.

Second run (private-crm, 36 beliefs, 2026-06-10): **18 legible / 11 partial / 5 gap /
2 drift** — score 18:5 vs 45:4, the law now quantified from both ends (~11:1 vs
~3.6:1). Three findings that generalize:

1. **Negative knowledge is the systematic blind spot.** The gaps clumped in (a) the
   newest subsystem and (b) *deliberate removals* — a thing destroyed on purpose
   leaves no artifact to read, so "don't reintroduce X" is illegible in every repo.
   Mechanisms get documented; absences don't.
2. **The "actively misleading" verdict class exists**: one gap had code whose surface
   reading implies the WRONG storage location (the out-of-band belief says why it
   moved elsewhere mid-flow). Worse than undocumented — counter-documented.
3. **Drift exposed a tier-1 design flaw**: both drifted beliefs were onboarded from
   historical commits and stated as current-state, then overtaken by later history
   the harvester also saw but didn't connect. Tier 1 should (a) stamp `valid_time`
   from the source commit date instead of leaving it null, and (b) verify
   current-state claims against HEAD before commit — or phrase them as dated
   episodes. The audit doubles as the verification pass, but it shouldn't have to.
