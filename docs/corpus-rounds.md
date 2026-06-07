# Corpus build rounds — record of what each round went to fetch

A log of the corpus-construction rounds: what angle each round targeted, what it
fetched/grounded against, and the artifact it produced. Keeps the belief-building
process auditable (and tells us which facets are already mined vs thin).

| Round | Target angle | Grounded against | Artifact |
|---|---|---|---|
| **R1 — breadth** | descriptive + episodic beliefs across subsystems; project-scope chains | repo docs, schema, git history (per corpus) | the base `beliefs/` per corpus (e.g. the private corpus S1–S13) |
| **R2 — adversarial + wrong-docs** | manufactured confident-and-wrong (weak-model harvest); doc-vs-code contradictions | tricky source snippets; CLAUDE.md/README vs code | `adversarial-haiku-*`, `wrong-docs-*` sessions |
| **R3 — epistemic patterns** | defeated verdicts (verdict-of-a-verdict), open conflicts, corroboration-accretion, inferred-but-correct survivors | real commits/reverts, ADRs, dir layout | `r3-*` sessions (closed the hand-off coverage gaps) |
| **R4 — grounding: rationale + kludge** *(in progress)* | (A) design-rationale "why X over Y"; (B) kludge / known-debt / constraint-driven compromise ("this needs fixing because X") | **the real repos** — `FIXME/HACK/TODO/XXX`, ADRs, design docs, commit rationale | grounded question bank → belief-building targets *(see fetch record below, filled on workflow completion)* |

## Facet coverage (running)

- **Well-mined:** descriptive facts, episodic (bugs/incidents), project-scope-over-time, the epistemic patterns (verdicts/conflicts/corroboration/survivors).
- **Partial:** architecture, testing-ethic (`tests-use-fixtures-not-mocks`, `database-first-no-db-mocks`), prescriptive "spine" rules (`rule-no-special-casing`).
- **Thin / R4 targets:** **design-rationale** (why X over Y), **kludge / known-debt** (constraint-driven compromise), code-style preferences, naming/glossary, scaling.

## R4 fetch record (per corpus)

Workflow `corpus-grounding-harvest` (run `wf_b3bd4d59-a83`). Bank →
[`research/grounded-question-bank.md`](research/grounded-question-bank.md).

| Corpus | repo avail | what it went to fetch |
|---|---|---|
| robot-game | ✅ | CLAUDE.md, ADR-001/002, save.rs, audio/tts.rs, specs; `rg FIXME\|HACK\|for now\|kludge`, `migrate_legacy`, git log -50 + spike bodies |
| perl-openapi-client | ✅ | Client.pm (full+POD), Changes, README; `rg -i FIXME\|HACK\|for now\|wrong\|sigh`; git show ×8 commits |
| tree-sitter-pod | ✅ | grammar.js, scanner.c (full), README, beliefs; `rg -i FIXME\|HACK\|nasty\|ick`; git show ×4 |
| zed-perl | ✅ | perl.rs, injections.scm, README; `rg -i evil\|super_evil` (signals only in git history); git show ×6 |
| composr | ✅ | bootstrap.rs, scripts.rs, install.rs, spi.rs, php_json.rs; `rg not support\|naive\|best-effort\|conservativ`; git log -50 + ×7 bodies |
| tree-sitter-perl | ✅ | CLAUDE.md, grammar.js, scanner.c; `rg _ctrl_z_hack`, `binop.nonassoc`, `we tried`; git show ×5 (valgrind/EOF/revert) |
| private-crm *(private)* · sql-abstract · helix · perl-lsp | ⏸ rate-limited | not harvested this round — **resumable** via `Workflow({scriptPath, resumeFromRunId: "wf_b3bd4d59-a83"})` (6 cached) |

**Retrieve (loop second-half) — what these flavors demanded of search:**
1. **Design-rationale** mostly maps to the existing *Why* query-type — but with a twist: a "why X over Y" ask needs the **rejected/abandoned alternative surfaced**, which is often a *superseded or reverted* belief. So the frontier filter that current-state asks want (drop the loser) is the **opposite** of what why-asks want (keep the loser). → **frontier filtering is intent-dependent, not a global default.**
2. **Kludge / known-debt is a genuinely new dimension** — see design-doc §3b. A kludge belief is *true AND known-deficient*, orthogonal to both confidence (it's not uncertain) and refutation (it's not wrong). It needs a deficiency axis + a `revisit_when`/`blocked_on` link to its forcing constraint, and it spawns a new query-type ("what's hacky / known-debt here").
