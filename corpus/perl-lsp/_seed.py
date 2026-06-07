#!/usr/bin/env python3
"""Seed the perl-lsp (perl-tree-sitter-lsp) Layer-0 belief corpus.

Models beliefs an agent co-developing this project formed ACROSS its lifetime
(Feb–Jun 2026): txn_time tracks when each was recorded, valid_time pins when
project-scope statements were true. Emphasis (per request): the project's SCOPE
at any given time, captured as a superseding chain of scope beliefs.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from _belief_lib import belief, generate, reset, AGENT, HUMAN, REDUCER

reset()
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "beliefs")

SESSIONS = {
    "P1": ("mvp-2026-02-20",            "2026-02-20", "the MVP: a single-file Perl LSP on tree-sitter"),
    "P2": ("crossfile-2026-03-02",      "2026-03-02", "cross-file resolution: resolver thread, cache, diagnostics"),
    "P3": ("typeinfer-2026-03-08",      "2026-03-08", "type inference: InferredType, operators, cross-file returns"),
    "P4": ("frameworks-2026-03-15",     "2026-03-15", "framework intelligence + inheritance (Moo/Mojo/DBIC)"),
    "P5": ("cli-tokens-2026-03-24",     "2026-03-24", "CLI analysis tools, rich semantic tokens, workspace index"),
    "P6": ("error-recovery-2026-03-29", "2026-03-29", "robustness: recover declarations from ERROR nodes"),
    "P7": ("rhai-plugins-2026-04-27",   "2026-04-27", "the pivot: Rhai-scripted plugins + witness-bag inference"),
    "P8": ("parametric-2026-05-08",     "2026-05-08", "parametric types: ResultSet RowOf, coderef return edges"),
    "P9": ("nav-graph-2026-06-04",      "2026-06-04", "NAV: unified FileStore/resolve, dispatch-target edges, exports"),
    "P10": ("wrong-docs-2026-06-04",    "2026-06-04", "hunting objectively-wrong docs: CLAUDE.md/README claims the code contradicts"),
    "P11": ("adversarial-haiku-2026-06-04", "2026-06-04", "adversarial harvest: an underpowered model (haiku, no tools) read tricky src and we kept what it got wrong"),
}

HAIKU = {"kind": "agent", "id": "claude-haiku-4-5", "model": "claude-haiku-4-5"}

# ---------------------------------------------------------------------------
# SCOPE OVER TIME — the headline. Each supersedes the previous as scope grew.
# ---------------------------------------------------------------------------
belief(slug="scope-mvp", session="P1", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-02-20", "2026-03-02"),
    edges=[], refs=["git:831353b", "README.md"],
    claim="perl-lsp is a SINGLE-FILE Perl language server on tree-sitter-perl + tower-lsp: scope-aware rename, completion, goto-def, hover, hash-key intelligence and signature help, all within one open file. No cross-file resolution, no type inference.",
    body="The MVP scope. tree-sitter parses, an early scope graph powers within-file navigation. Everything later strictly expands this.")

belief(slug="scope-crossfile", session="P2", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-03-02", "2026-03-08"),
    edges=[("supersedes", "scope-mvp")], refs=["git:feat cross-file", "README.md"],
    claim="Scope expanded to CROSS-FILE: module resolution via a background resolver thread (@INC + cpanfile), a per-project SQLite cache, unresolved-function diagnostics and auto-import code actions. Still no real type inference.",
    body="Supersedes [[scope-mvp]]. The project stops being a single-buffer tool and becomes project-aware.")

belief(slug="scope-typed", session="P3", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-03-08", "2026-03-15"),
    edges=[("supersedes", "scope-crossfile")], refs=["README.md"],
    claim="Scope expanded to TYPE INFERENCE: an InferredType lattice from literals/operators/constructors/return values, cross-file return-type and hash-key propagation, hover + inlay type display. No annotations needed — types are inferred from use.",
    body="Supersedes [[scope-crossfile]]. 'Deep semantic intelligence' starts here.")

belief(slug="scope-frameworks", session="P4", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-03-15", "2026-04-27"),
    edges=[("supersedes", "scope-typed")], refs=["README.md"],
    claim="Scope expanded to FRAMEWORK INTELLIGENCE + INHERITANCE: Moo/Moose/Mojo::Base/DBIC/perl-5.38-`class` accessor synthesis, inheritance/role/mixin/component chain walking, constant folding + dynamic dispatch, POD via tree-sitter-pod, workspace indexing, CLI analysis tools (`--check`), 10-type semantic tokens. Frameworks are hardcoded in Rust.",
    body="Supersedes [[scope-typed]]. The 'hardcoded frameworks' detail is what the next era overturns.")

belief(slug="scope-plugin-platform", session="P7", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-04-27", "2026-06-01"),
    edges=[("supersedes", "scope-frameworks")], refs=["git:feat rhai plugins (#21)", "docs/adr/plugin-system.md"],
    claim="Scope SHIFTED from 'an LSP with hardcoded frameworks' to 'an extensible analysis PLATFORM': framework intelligence became Rhai plugins (fingerprinted, user-droppable into $PERL_LSP_PLUGIN_DIR), and type inference was reorganized around a single canonical witness bag.",
    body="Supersedes [[scope-frameworks]]. This is the architectural pivot — extensibility becomes the product, not just the features.")

belief(slug="scope-nav-platform", session="P9", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-06-01", "2999-01-01"),
    edges=[("supersedes", "scope-plugin-platform")], refs=["CLAUDE.md", "docs/prompt-graph-walking.md"],
    claim="Current scope: a NAVIGATION / graph-walking platform — unified role-tagged FileStore + `refs_to` resolution, stored dispatch-target edges on method calls, export-surface folding (%EXPORT_TAGS), and perl-gen for Import::Base kit plugins — on top of the CLI analysis toolkit and editor distribution (VS Code auto-download).",
    body="Supersedes [[scope-plugin-platform]]. The frontier (`docs/prompt-graph-walking.md`) is graph walking over the resolved-target edge.")

# ---------------------------------------------------------------------------
# Architecture + hard-won rules (some evolved)
# ---------------------------------------------------------------------------
belief(slug="arch-four-layers", session="P2", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["CLAUDE.md"],
    claim="perl-lsp is four layers, data flows DOWN only: LSP adapter (symbols.rs/backend.rs) -> cross-file (module_index/_resolver/_cache) -> builder (builder.rs) -> data model (file_analysis.rs).",
    body="The dependency direction is the invariant. Higher layers query, never reach around.")

belief(slug="rule-builder-sole-ts-consumer", session="P2", turn=4, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.95, valid_time=None,
    edges=[("refines", "arch-four-layers")], refs=["CLAUDE.md"],
    claim="HARD RULE: all tree-sitter CST traversal happens inside build() — builder.rs is the ONLY tree-sitter consumer. Nothing else walks nodes, calls child_by_field_name, or uses TreeCursor; everyone else queries FileAnalysis.",
    body="Refines [[arch-four-layers]]. file_analysis.rs is the single source of truth; symbols.rs is a thin adapter; cursor_context.rs is the one position-dependent exception (reads a tree but never mutates FileAnalysis).")

belief(slug="rule-no-special-casing", session="P4", turn=4, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.95, valid_time=None, edges=[],
    refs=["CLAUDE.md", "docs/adr/parametric-types.md"],
    claim="HARD-WON RULE: never special-case for a particular shape — method-name allowlists, base-class equality checks, real-vs-synthetic branches, per-name lookup tables in core — you are ALWAYS wrong, because the enumerated list is always incomplete. Encode the 'wants behavior X' property on the type/value/witness itself so consumers ask the value, never the shape.",
    body="The load-bearing design discipline of the whole codebase. The special case is always the smallest diff now and never stays cheap. Cited repeatedly across ADRs; the parametric-types work is its canonical application — see [[parametric-types-resultset]].")

belief(slug="rule-comment-why-not-history", session="P4", turn=6, author=HUMAN,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None, edges=[],
    refs=["CLAUDE.md"],
    claim="Comment discipline (Veesh's, in CLAUDE.md): comments explain WHY — invariants, ordering constraints, trade-offs, ADR pointers — and NEVER narrate history ('used to be X', 'D3 added', 'the staircase deleted'). Git remembers; describe what is, not what changed.",
    body="A human-authored working-style belief about contributing to this repo. When a comment grows past a few lines, move the rationale to an ADR with a one-line pointer.")

# witness bag: the canonical type-inference store (supersedes the old approach)
belief(slug="builder-infer-expression-type", session="P3", turn=4, author=AGENT,
    directness="stated", obs=1, weight=0.7, asserted=0.8, valid_time=("2026-03-08", "2026-04-27"),
    edges=[], refs=["git:operator-based type inference"],
    claim="Type inference is performed by Builder::infer_expression_type plus Builder.resolved_returns, walking expressions at build time and storing materialized InferredTypes.",
    body="The original type-inference design. Correct for its era; entirely replaced by the witness bag — CLAUDE.md now reads 'Builder::infer_expression_type is gone', 'Builder.resolved_returns is gone'. Kept for reliving the pre-bag architecture.")

belief(slug="witness-bag-canonical", session="P7", turn=4, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.95, valid_time=("2026-04-27", "2999-01-01"),
    edges=[("supersedes", "builder-infer-expression-type")], refs=["CLAUDE.md", "docs/adr/bag-canonical.md"],
    claim="Type inference has ONE source of truth: the witness bag. Production is bag.push(...); consumption is a ReducerRegistry query. There is no second source. Adding type behavior = adding a reducer; never write InferredType directly or build a parallel query helper.",
    body="Supersedes [[builder-infer-expression-type]]. Two strict phases: collect in populate_witness_bag(), reduce via the ReducerRegistry (reducers claim attachment shapes in registration order).")

belief(slug="edges-not-values", session="P9", turn=4, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None,
    edges=[("refines", "witness-bag-canonical")], refs=["CLAUDE.md", "docs/adr/bag-canonical.md"],
    claim="Bag invariant: mirror facts between attachments via Edge(target), NEVER re-push a materialized InferredType onto an edge-reachable attachment as a 'cache' — the registry's edge-chase IS the canonical flow; a parallel materialized store drifts.",
    body="Refines [[witness-bag-canonical]]. Witnesses are monotone (append-only); termination follows from the finite InferredType lattice + a snapshot check. Re-emittable passes clear-and-emit by source tag.")

# frameworks: hardcoded -> rhai plugins
belief(slug="frameworks-were-hardcoded", session="P4", turn=8, author=AGENT,
    directness="stated", obs=1, weight=0.7, asserted=0.8, valid_time=("2026-03-15", "2026-04-27"),
    edges=[], refs=["git:framework accessor synthesis (#8)"],
    claim="Framework intelligence (Moo/Moose/Mojo::Base/DBIC accessor synthesis) is implemented as hardcoded Rust logic inside the builder.",
    body="True through the frameworks era; superseded when frameworks moved to Rhai. Historical.")

belief(slug="frameworks-are-rhai-plugins", session="P7", turn=6, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-04-27", "2999-01-01"),
    edges=[("supersedes", "frameworks-were-hardcoded")], refs=["docs/adr/plugin-system.md", "README.md"],
    claim="Framework intelligence ships as bundled Rhai plugins with EMIT hooks (on_use / on_function_call / on_method_call — parse-time, declarative, return Vec<EmitAction>) and QUERY hooks (on_signature_help / on_completion — cursor-time, imperative). Plugin sources are fingerprinted, so editing one invalidates the cross-file cache.",
    body="Supersedes [[frameworks-were-hardcoded]]. 'Silent'/'exclusive' answers let a plugin suppress native paths it knows will mishandle a slot.")

belief(slug="plugin-namespace-ownership", session="P7", turn=8, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None,
    edges=[("refines", "frameworks-are-rhai-plugins")], refs=["docs/adr/plugin-system.md", "CLAUDE.md"],
    claim="Plugin-synthesized content is owned by PluginNamespace, NOT Perl classes; cross-file lookup goes through ModuleIndex::for_each_entity_bridged_to(class, ...) — no parallel reverse indexes (the retired class_content_index / modules_with_class_content were exactly that mistake).",
    body="Refines [[frameworks-are-rhai-plugins]]. An instance of [[rule-no-special-casing]]: the bridge is generic, not per-plugin.")

# ---------------------------------------------------------------------------
# Subsystems / capabilities
# ---------------------------------------------------------------------------
belief(slug="sqlite-module-cache", session="P2", turn=6, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["CLAUDE.md", "module_cache.rs"],
    claim="Cross-file resolution is backed by a per-project SQLite cache at ~/.cache/perl-lsp/<hash>/modules.db (bincode+zstd FileAnalysis blobs, schema v9). An EXTRACT_VERSION bump triggers priority re-resolution without dropping the table; nuke it via `perl-lsp --clear-cache [<root>]`, never rm -rf.",
    body="A plugin-fingerprint mismatch on startup hard-clears the modules table so QA isn't served stale blobs.")

belief(slug="parse-cli-debug", session="P6", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["CLAUDE.md"],
    claim="Standing rule for tree-sitter-perl work: inspect the CST with `perl-lsp --parse <file|-->` (reads stdin with --) instead of guessing node kinds/fields.",
    body="A workflow belief — the cheapest way to avoid wrong assumptions about tree-sitter-perl's grammar.")

belief(slug="error-recovery-stable-outline", session="P6", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["git:recover structural declarations from ERROR nodes", "git:stable outline", "docs/adr/error-recovery.md"],
    claim="perl-lsp degrades gracefully on broken parses: it recovers structural declarations from tree-sitter ERROR nodes and keeps a STABLE outline across parse degradation — because mid-typing you routinely bork the tree.",
    body="Robustness era. The stable outline means document symbols don't flicker while you type.")

belief(slug="parametric-types-resultset", session="P8", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None,
    edges=[("refines", "rule-no-special-casing")], refs=["docs/adr/parametric-types.md", "git:feat parametric-resultset (#35)"],
    claim="DBIC ResultSet column completion resolves through resultset_class discovery + RowOf projection emission — and the rule lives on InferredType::hash_key_class() (a property of the type), NOT a method-name allowlist like {search, find}.",
    body="The canonical application of [[rule-no-special-casing]]: push the 'parametric arg' behavior onto the type so consumers ask the value, never branch on the method name.")

belief(slug="coderef-return-edge", session="P8", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["git:CodeRef carries return_edge", "git:refgen + coderef-call typing (#36)"],
    claim="CodeRef carries a return_edge so sub-literal callable types survive variable rebinding; `\\&foo` and `$cb->()` are first-class typed expressions.",
    body="Part of pushing type inference into corners that need real edges rather than materialized values — see [[edges-not-values]].")

belief(slug="nav-dispatch-target-edge", session="P9", turn=6, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["git:85d8a75", "git:335d89c"],
    claim="Navigation stores a resolved dispatch-target edge on MethodCall refs (invocant->target) and records an HONEST-MISS when it cannot resolve — it does not guess a target.",
    body="The honest-miss is itself the anti-special-casing discipline: don't fabricate an answer to look complete.")

belief(slug="perl-gen-importbase", session="P9", turn=8, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["perl-gen/README.md", "docs/adr/importbase-plugin-gen.md"],
    claim="perl-gen/ reads an Import::Base kit's @IMPORT_MODULES / %IMPORT_BUNDLES tables and emits a ready-to-commit Rhai plugin, so the LSP understands shop-specific `use Co::Base -Class` import boilerplate.",
    body="Bridges the plugin platform to real Perl shops that centralize imports behind a kit.")

# ---------------------------------------------------------------------------
# OBJECTIVELY WRONG beliefs — misreads + falsified hunches, each defeated by a
# verdict. The wrong belief carries NO valid_time (it was never true); only the
# `adjudicates` edge defeats it. High `asserted` + wrong = the calibration label.
# ---------------------------------------------------------------------------

# Pair: subprocess isolation — a falsified safety/perf hunch
belief(slug="subprocess-isolation-needed", session="P2", turn=8, author=AGENT,
    directness="inferred", obs=1, weight=0.6, asserted=0.8, valid_time=None, edges=[],
    refs=["git:subprocess isolation for module parsing"],
    claim="Module parsing MUST run in isolated subprocesses so a parser crash or a pathological file can't take down the language server.",
    body="A confident architecture hunch (asserted 0.8): crash-isolation felt necessary for robustness. It shipped first, then got falsified.")

belief(slug="subprocess-isolation-removed", session="P5", turn=8, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "subprocess-isolation-needed"), ("attacks", "subprocess-isolation-needed")],
    refs=["git:workspace indexing + subprocess removal (#14)"],
    claim="VERDICT: subprocess isolation was REMOVED (#14). The IPC + serialization overhead was real and the crash-isolation worry never materialized; in-process parsing with Rayon (274-file Mojolicious in 204ms) is both faster and simpler.",
    body="Defeats [[subprocess-isolation-needed]]. The robustness fear cost measurable latency and bought nothing observable — a hunch that traded real cost for an imagined risk.")

# Pair: dispatch always resolvable — falsified, leads to honest-miss
belief(slug="dispatch-always-resolvable", session="P8", turn=7, author=AGENT,
    directness="inferred", obs=1, weight=0.6, asserted=0.75, valid_time=None, edges=[],
    refs=["git:MethodCallBinding"],
    claim="Given an invocant type, a method call's dispatch target can essentially always be resolved; when unsure, a best-effort guess at the target is fine.",
    body="A navigation hunch: better to jump somewhere than nowhere. Wrong — see the verdict.")

belief(slug="honest-miss-over-guessing", session="P9", turn=10, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "dispatch-always-resolvable"), ("refines", "nav-dispatch-target-edge")],
    refs=["git:85d8a75"],
    claim="VERDICT: guessing a dispatch target when the invocant is unresolved produced WRONG go-to-definition jumps. NAV now records an HONEST-MISS instead — a wrong jump is worse than no jump, so 'best-effort guess' was actively harmful.",
    body="Defeats [[dispatch-always-resolvable]]; refines [[nav-dispatch-target-edge]]. The corrected principle: surface the miss, never fabricate a target to look complete (an instance of [[rule-no-special-casing]] applied to navigation).")

# Pair: the diamond-inheritance caching unsoundness (caught 2026-06-03)
belief(slug="memo-coarse-receiver-key-sound", session="P9", turn=12, author=AGENT,
    directness="inferred", obs=1, weight=0.6, asserted=0.85, valid_time=None, edges=[],
    refs=["git:a159008", "git:cold-start perf — QueryState memoization"],
    claim="The cold-start type-query memo can key its receiver slot by InferredType VARIANT only (collapsing every ClassName(_) to one tag) and stay sound — the existing cycle-guard already shares that coarse key safely, so the result memo can reuse it.",
    body="A perf optimization (2026-06-03) to memoize away the inheritance-diamond re-chase blowup. Asserted 0.85 — confident BECAUSE the cycle-guard precedent looked authoritative. That precedent was the trap.")

belief(slug="coarse-memo-key-unsound-diamond", session="P9", turn=13, author=AGENT,
    directness="stated", obs=1, weight=0.92, asserted=0.95, valid_time=None,
    edges=[("adjudicates", "memo-coarse-receiver-key-sound"), ("attacks", "memo-coarse-receiver-key-sound"),
           ("supports", "edges-not-values")],
    refs=["git:580b72a", "git:b4a1911"],
    claim="VERDICT (soundness review, fixed 2026-06-03): the coarse memo key was UNSOUND. ReturnExpr::Receiver substitutes the FULL receiver, so one shared MethodOnClass{Parent, m} attachment reached with ClassName('Foo') then ClassName('Bar') served Foo's memoized answer to Bar — a SILENT wrong type. The cycle-guard could share the coarse key (collision→None, conservative) but a memo returning a substantive value on it could not. Fix: key the receiver on full structural identity.",
    body="Defeats [[memo-coarse-receiver-key-sound]]. The exact error class the user flagged: we were wrong about caching because of diamond inheritance. The subtlety: the real inheritance diamond the memo EXISTS to tame is same-receiver (q.receiver constant), so it still hashes to one key and perf held (cold crm --check ~8s) — the bug was *cross-receiver* collision. Lesson: 'the cycle-guard shares this key safely' did NOT transfer to 'the memo may return a cached value on this key' — same key shape, different soundness contract. A live case of the [[edges-not-values]] warning: a materialized cache that drifts from the canonical chase.")

# ---------------------------------------------------------------------------
# Reducer consensus over the scope trajectory
# ---------------------------------------------------------------------------
belief(slug="r-perl-lsp-trajectory", session="P9", turn=20, author=REDUCER,
    directness="reduced", obs=4, weight=0.6, asserted=None, valid_time=None,
    edges=[("derived_from", "scope-mvp"), ("derived_from", "scope-frameworks"),
           ("derived_from", "scope-plugin-platform"), ("derived_from", "rule-no-special-casing")],
    refs=[],
    claim="perl-lsp's trajectory: single-file LSP -> cross-file -> typed -> framework-aware -> extensible Rhai-plugin platform -> navigation/graph-walking platform. Each era strictly expanded scope, while a few hard rules (single tree-sitter consumer, witness-bag-canonical, no special-casing) kept the core from sprawling as features piled on.",
    body="REDUCED over the scope chain plus the governing discipline. The interesting property a flat fact-store would lose: scope is a moving target with a stable spine, and the spine is WHY the expansion didn't collapse into special cases.")

# ---------------------------------------------------------------------------
# OBJECTIVELY WRONG DOCS (P10). Beliefs sourced from the project's own docs that
# the code contradicts — and the doc is the trusted, high source_weight source.
# ---------------------------------------------------------------------------
belief(slug="doc-parse-stdin-double-dash", session="P10", turn=2, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["CLAUDE.md:71", "CLAUDE.md:176"],
    claim="To pipe a snippet to the parser you run `perl-lsp --parse --` — per CLAUDE.md, `--` reads from stdin (`echo '...' | perl-lsp --parse --`).",
    body="Straight from CLAUDE.md, the doc that teaches the canonical CST-inspection workflow. Falsified below.")

belief(slug="parse-stdin-is-single-dash", session="P10", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.95, asserted=0.95, valid_time=None,
    edges=[("adjudicates", "doc-parse-stdin-double-dash"), ("attacks", "doc-parse-stdin-double-dash")],
    refs=["src/main.rs:1105", "src/main.rs:150"],
    claim="VERDICT: FALSE. The stdin sentinel is a SINGLE dash: `cli_parse` does `if path == \"-\"` (src/main.rs:1105), and the binary's own usage help even prints \"(`-` reads from stdin)\" (line 150). `--parse --` would try to open a file literally named `--`. CLAUDE.md is wrong about its own debugging command.",
    body="Defeats [[doc-parse-stdin-double-dash]]. Doubly ironic: the doc whose whole job is 'inspect the CST instead of guessing' is itself wrong, and the in-code help string is correct while the prose doc drifted. The trusted doc lost to the code.")

belief(slug="doc-max-fold-debug-only", session="P10", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.8, valid_time=None, edges=[],
    refs=["CLAUDE.md:148"],
    claim="Per CLAUDE.md, `MAX_FOLD_ITERATIONS = 64` is a DEBUG-ONLY safety net for the type-inference fold.",
    body="A CLAUDE.md claim about the fold driver. Falsified below.")

belief(slug="max-fold-runs-in-release", session="P10", turn=6, author=AGENT,
    directness="stated", obs=1, weight=0.95, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "doc-max-fold-debug-only"), ("attacks", "doc-max-fold-debug-only")],
    refs=["src/builder.rs:10108", "src/builder.rs:10113"],
    claim="VERDICT: FALSE. The fold has a `debug_assert!` (dev-only) AND a separate unconditional `if iters >= MAX_FOLD_ITERATIONS { eprintln!(\"...bailing out...\"); break; }` that runs in RELEASE too — the code's own comment even calls it 'the all-builds safety net'. CLAUDE.md's 'debug-only' contradicts the code and the code's comment.",
    body="Defeats [[doc-max-fold-debug-only]]. A stale doc claim about runtime behavior — the kind that would make an agent reason wrongly about whether release builds can bail the fold.")

belief(slug="doc-test-count-stale", session="P10", turn=8, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.85, valid_time=None, edges=[],
    refs=["README.md:223"],
    claim="README.md states `cargo test  # 317 unit tests`, but the figure is stale: `cargo test -- --list` enumerates far more (~801 reported by a sweep). A hardcoded count in prose drifts the moment a test is added — a TRUE observation that the documented number is wrong.",
    body="A self-verifying class of wrong doc: any literal count baked into prose rots. Recorded as a true observation (the doc IS wrong), with the exact current number left soft (agent-reported ~801, confirm with `cargo test -- --list`).")

# ---------------------------------------------------------------------------
# ADVERSARIAL HARVEST (P11). Organic mistaken beliefs: a workflow ran an
# underpowered model (claude-haiku-4-5, NO tools) on tricky src/ snippets and
# forced confident claims; a tool-using sonnet adjudicator checked each against
# the real code. Across 18 snippets / 36 claims the weak model was right ~83%;
# these are the 3 perl-lsp claims it got WRONG. Wrong beliefs are authored AS
# the weak model (source_weight 0.4); verdicts cite the adjudicator's evidence.
# ---------------------------------------------------------------------------
belief(slug="haiku-transitive-parents-break-guard", session="P11", turn=2, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.99, valid_time=None, edges=[],
    refs=["src/builder.rs:2288"],
    claim="In transitive_parents, with 21 direct parents and no grandparents, exactly 21 parents appear and the loop terminates via the `if depth > 20 { break; }` guard firing on iteration 22.",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.99 — its single most confident claim in the run, and wrong on mechanism.")

belief(slug="transitive-parents-ends-on-empty-stack", session="P11", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "haiku-transitive-parents-break-guard"), ("attacks", "haiku-transitive-parents-break-guard")],
    refs=["src/builder.rs:2288"],
    claim="VERDICT: partly wrong — the COUNT is right (21 out, depth=21) but the MECHANISM is wrong: with no grandparents the stack empties after 21 pops, so `while let Some(p) = stack.pop()` ends normally; the `if depth > 20 { break; }` guard NEVER fires here. Right answer, wrong reason — at 0.99 confidence.",
    body="Defeats [[haiku-transitive-parents-break-guard]]. The instructive bit: a belief can be output-correct and mechanism-wrong, and asserted confidence flags neither. A reducer scoring only the final answer would mark this 'correct' and miss that the model didn't understand the loop.")

belief(slug="haiku-refs-nonopen-single-scan", session="P11", turn=4, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.95, valid_time=None, edges=[],
    refs=["src/resolve.rs:191-208"],
    claim="In refs_to with RoleMask::VISIBLE, a workspace module that is NOT open has collect_from_analysis called exactly once (the DEPENDENCY phase only).",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.95. Wrong about the phase interaction.")

belief(slug="refs-nonopen-scans-twice", session="P11", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "haiku-refs-nonopen-single-scan"), ("attacks", "haiku-refs-nonopen-single-scan")],
    refs=["src/resolve.rs:191-208"],
    claim="VERDICT: FALSE. A not-open workspace module is scanned TWICE — once in the WORKSPACE phase (covered_paths only excludes OPEN files) and again in the DEPENDENCY phase, which has no covered_paths skip. Both phases run collect_from_analysis independently.",
    body="Defeats [[haiku-refs-nonopen-single-scan]]. Bonus: the verdict surfaces a real (benign) inefficiency — the same analysis is walked twice and only deduped afterward. The weak model under-counted the work because the skip logic lives in a different phase than the one it was looking at (out-of-snippet knowledge again).")

belief(slug="haiku-dedup-fails-mixed-key", session="P11", turn=6, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.98, valid_time=None, edges=[],
    refs=["src/resolve.rs:223-232"],
    claim="refs_to's dedup_by does NOT remove duplicates when the same file appears as both FileKey::Url and FileKey::Path, because key_for_sort differentiates them so they sort non-adjacently and dedup_by only drops consecutive equals — i.e. duplicate refs survive (a bug).",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.98. A confidently-INVENTED bug.")

belief(slug="dedup-normalizes-keys", session="P11", turn=7, author=AGENT,
    directness="stated", obs=1, weight=0.92, asserted=0.92, valid_time=None,
    edges=[("adjudicates", "haiku-dedup-fails-mixed-key"), ("attacks", "haiku-dedup-fails-mixed-key")],
    refs=["src/resolve.rs:223-232"],
    claim="VERDICT: FALSE — no such bug. key_for_sort normalizes BOTH FileKey::Path(p) and FileKey::Url(u) to the same PathBuf (u.to_file_path(), falling back to PathBuf::from(u.as_str())), so Url/Path for one file sort ADJACENTLY and file_key_eq (also via key_for_sort) treats them equal; dedup_by removes them correctly.",
    body="Defeats [[haiku-dedup-fails-mixed-key]]. A distinct and dangerous failure mode: the weak model INVENTED a plausible bug from a partial read (it saw two FileKey variants and assumed they'd compare unequal, never checking key_for_sort's normalization). A false-positive bug report asserted at 0.98 — exactly the kind of confident-wrong an epistemic store must be able to defeat with a verdict.")

# meta finding over the cross-repo harvest
belief(slug="r-adversarial-harvest-finding", session="P11", turn=20, author=REDUCER,
    directness="reduced", obs=3, weight=0.6, asserted=None, valid_time=None,
    edges=[("derived_from", "haiku-transitive-parents-break-guard"),
           ("derived_from", "haiku-refs-nonopen-single-scan"),
           ("derived_from", "haiku-dedup-fails-mixed-key")],
    refs=[],
    claim="Adversarial-harvest finding (18 snippets, 36 claims, both perl-lsp and tree-sitter-perl): the underpowered model was CORRECT ~83% even on deliberately tricky code. Its errors clustered not in local reasoning but where the trap needed knowledge OUTSIDE the snippet — a skip in a different phase, a normalization in a different fn, tree-sitter's GLR conflict semantics. Failure modes seen: right-answer-wrong-mechanism (0.99 conf), under-counting work, and TWO invented-bugs-that-don't-exist (0.98/0.95 conf).",
    body="REDUCED over the harvested wrong beliefs. The design implication: confident-wrong beliefs are most likely exactly where a belief's justification reaches beyond what was in view — which is precisely what the justification-edge graph (§4) and cross-file provenance are for. And the 'invented bug' mode argues for adversarial verification (§9): a plausible bug claim should be refuted against code before it's trusted, never accepted on asserted confidence.")

generate(
    title="perl-lsp (perl-tree-sitter-lsp)",
    blurb="A Perl language server with deep semantic intelligence (tree-sitter-perl + tower-lsp), "
          "tracked as it grew from a single-file MVP to an extensible Rhai-plugin navigation platform.",
    subject_repo="~/personal/perl-tree-sitter-lsp",
    out_dir=OUT,
    sessions=SESSIONS,
)
