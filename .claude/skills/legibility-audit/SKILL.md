---
name: legibility-audit
description: Audit what the belief-memory knows that the working tree doesn't record, then propose ADRs/comments/CLAUDE.md lines to close the gaps. Use when the user asks for a legibility audit, asks "what does memory know that the repo doesn't", or wants to convert memory into in-tree documentation. Run from inside the target repo.
---

# Legibility audit

Memory's value is inversely proportional to repo legibility (measured: docs/design/onboarding.md
§"The emerging law"). This skill inverts that: find the beliefs the tree does NOT record and
propose how to make them legible. A legible repo has, in effect, onboarded itself — this skill
is how a repo gets there.

The whole audit is read-only with respect to the target repo. Never write into it; the
deliverable is proposed text the user applies.

## Procedure

**0. Resolve the inputs.** Target repo = the current working directory's git toplevel; its
repo id is the toplevel's directory name. Belief store = `$MEMORY_DIR`, else
`$XDG_DATA_HOME/agentic-memory/beliefs`, else `~/.local/share/agentic-memory/beliefs`.

**1. Enumerate the beliefs** (one markdown file per belief, YAML frontmatter):
- every belief with `scope: repo:<repo-id>` (any branch suffix too),
- plus `scope: global` beliefs whose text names this repo,
- EXCLUDE edge-beliefs (frontmatter has a `relation:` block) and retracted beliefs.

Grep the store dir, then read each hit. The claim is `claim.text`; refs in `provenance.refs`;
the body carries why/evidence. If the set is large (>60), audit the repo-scoped set fully and
sample the globals.

**2. Audit each belief against the WORKING TREE.** Where does the tree record this knowledge?
Check in order: docs/ (ADRs, design docs, roadmap/open-problems), CLAUDE.md, README, code
comments at/near the belief's refs, then structural self-evidence (a type or test that enforces
it so clearly no prose is needed). Git history does NOT count as legible — that's the point.

Verdict per belief:
- **LEGIBLE** — stated in docs or a comment at the right place (cite file:line)
- **PARTIAL** — enforced or hinted (test pins the behavior but the WHY is unstated; or stated
  far from where a reader needs it)
- **GAP** — recorded nowhere in the tree

**3. Propose remediation for the gaps.** Cluster related gaps; rank by blast radius (would an
engineer or agent plausibly decide wrongly without this?). Match the repo's own conventions —
read an existing ADR/doc first and copy its format. Three remediation shapes:
- **ADR stub** (title + 5–15 line draft) for standing decisions/constraints,
- **code comment** (exact file:line + 1–3 line text) for site-local knowledge,
- **CLAUDE.md line** for working conventions.

**4. Drift check.** Note any belief the tree now CONTRADICTS — that's a stale belief to
supersede (`mem remember "<correction>" --supersedes <slug>`), not a doc gap.

## Deliverable

1. Headline: N audited → L legible / P partial / G gap (the L:G ratio IS the repo's
   legibility score; record it for comparison across runs).
2. Verdict table, one line per belief.
3. Ranked remediations: full draft text for the top 5–8 gaps, one-liners for the rest.
4. Drift paragraph (or "none found").

Offer to apply accepted remediations as a follow-up, and to supersede drifted beliefs.

## Scale

Fan out only when the belief set is large: one auditor per ~30 beliefs, partitioned by belief
slug, then merge tables. For <50 beliefs a single pass is cheaper and keeps verdicts consistent.
