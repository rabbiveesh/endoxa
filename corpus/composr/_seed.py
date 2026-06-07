#!/usr/bin/env python3
"""Seed the composr Layer-0 belief corpus.

composr is young and grew fast (May–Jun 2026): a `composer dump-autoload -o`
speedup that ballooned into a full `composer install` replacement in a day, then
native plugin replication, then a 1.0 crate. Emphasis (per request): the
project's SCOPE at any given time, as a superseding chain of scope beliefs.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from _belief_lib import belief, generate, reset, AGENT, HUMAN, REDUCER

reset()
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "beliefs")

SESSIONS = {
    "C1": ("autoload-dumpr-2026-05-05", "2026-05-05", "the seed: a Rust `composer dump-autoload -o`"),
    "C2": ("installer-2026-05-07",      "2026-05-07", "ballooned into a full `composer install` replacement"),
    "C3": ("hybrid-2026-05-07",         "2026-05-07", "hybrid mode + native Laravel package:discover"),
    "C4": ("plugins-2026-05-10",        "2026-05-10", "native plugin replication (pest, spi) + perf tuning"),
    "C5": ("release-2026-06-02",        "2026-06-02", "1.0.0 to crates.io + publish-on-tag CI"),
    "C6": ("wrong-docs-2026-06-04",     "2026-06-04", "hunting objectively-wrong docs: README claims the code contradicts"),
}

# ---------------------------------------------------------------------------
# SCOPE OVER TIME
# ---------------------------------------------------------------------------
belief(slug="scope-dumpr", session="C1", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-05-05", "2026-05-07"),
    edges=[], refs=["git:5981358", "README.md"],
    claim="composr was born (as 'autoload-dumpr') with a tiny scope: a Rust replacement for ONLY `composer dump-autoload -o` — rewrite vendor/composer/autoload_classmap.php and patch autoload_static.php, byte-equivalent to composer. Nothing else.",
    body="The seed. Even at birth the correctness bar is byte-equivalence; that never changes even as scope explodes.")

belief(slug="scope-installer", session="C2", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-05-07", "2026-05-10"),
    edges=[("supersedes", "scope-dumpr")], refs=["git:9a2f252", "README.md"],
    claim="Scope EXPLODED in a single day (2026-05-07) into a full `composer install` replacement: lock-driven parallel download -> extract -> native autoload bootstrap -> scripts, with path-type packages, hybrid mode, and native Laravel package:discover. Renamed autoload-dumpr -> composr.",
    body="Supersedes [[scope-dumpr]]. The most dramatic single-day scope jump in the corpus — a one-command tool became an end-to-end installer. See [[rename-dumpr-to-composr]].")

belief(slug="scope-plugin-replication", session="C4", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-05-10", "2026-06-02"),
    edges=[("supersedes", "scope-installer")], refs=["git:Native plugin replication"],
    claim="Scope expanded to NATIVE PLUGIN REPLICATION: rather than always delegating composer-plugin hooks, composr ports specific plugins to Rust with byte-equal output (pest-plugin, tbachert/spi), and adds a `composr git-hook` subcommand plus an extraction perf-tuning pass.",
    body="Supersedes [[scope-installer]]. The boundary moves from 'be a fast installer that defers to composer for plugins' to 'replicate the plugins that matter, natively'.")

belief(slug="scope-released", session="C5", turn=1, author=AGENT, scope=True,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-06-02", "2999-01-01"),
    edges=[("supersedes", "scope-plugin-replication")], refs=["git:Prep 1.0.0 release", "git:publish-on-tag CI"],
    claim="Current scope: a PUBLISHED 1.0 crate on crates.io with publish-on-tag CI — a production tool that installs two real production-shape Laravel codebases byte-equivalently to composer, with 0 composer subprocess calls on a cold install.",
    body="Supersedes [[scope-plugin-replication]]. 'Working end-to-end on real apps' is now the headline, not a single command.")

# ---------------------------------------------------------------------------
# Goals / philosophy (stable spine)
# ---------------------------------------------------------------------------
belief(slug="goal-cold-start-path", session="C1", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["README.md"],
    claim="composr deliberately targets the COLD-START install path — the dist-download + autoload-bootstrap + Laravel package:discover that composer spends ~30-70s on — not a general composer reimplementation. It speeds up the slow parts and defers the rest.",
    body="The scoping principle that keeps the project tractable. package:discover alone drops from ~30s to ~5ms.")

belief(slug="goal-byte-equivalence", session="C2", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.95, valid_time=None, edges=[],
    refs=["README.md", "tests/"],
    claim="HARD correctness bar: composr's output (vendor tree, autoload bootstrap, packages.php, plugin artifacts) must be BYTE-EQUIVALENT to composer's, verified by golden tests against a real composer in the integration suite.",
    body="The bar that makes 'replace composer install' safe. It constrains every native reimplementation — discover, autoload, plugin codegen all answer to it.")

belief(slug="hybrid-mode-philosophy", session="C3", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["README.md"],
    claim="Hybrid mode: composr does NOT reimplement composer's EventDispatcher. When a script event contains a class-method handler (Foo\\Bar::baz), it shells out to `composer run-script <event>` for that one event — correctness over coverage.",
    body="The principled boundary of the native reimplementation. Composer-bin resolution order: --composer-bin, COMPOSR_COMPOSER, composer on PATH.")

# delegate-all -> native (an evolution of the post-autoload-dump handling)
belief(slug="delegate-all-post-autoload", session="C2", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.65, asserted=0.7, valid_time=("2026-05-07", "2026-05-07"),
    edges=[], refs=["git:Hybrid mode"],
    claim="The whole post-autoload-dump lifecycle event (including Laravel's package:discover) is handled by delegating to a `composer run-script` subprocess.",
    body="The initial hybrid-mode stance. Quickly narrowed the same day once discover + clearCompiled were handled natively. Kept for audit of the early plan.")

belief(slug="native-post-autoload", session="C3", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-05-07", "2999-01-01"),
    edges=[("supersedes", "delegate-all-post-autoload"), ("refines", "hybrid-mode-philosophy")],
    refs=["git:Native package:discover", "README.md"],
    claim="Two narrowing exceptions handle Laravel natively: ComposerScripts::postAutoloadDump (the known clearCompiled handler) is skipped, and `@php artisan package:discover` is replaced by native `composr discover`. When those fire and the rest are plain shell/php, post-autoload-dump runs with ZERO composer subprocesses.",
    body="Supersedes [[delegate-all-post-autoload]] and refines [[hybrid-mode-philosophy]]. This is how monolith-app reaches '0 composer calls' on a cold install.")

# plugins: delegate-all -> three-tier policy
belief(slug="plugins-all-delegated", session="C2", turn=7, author=AGENT,
    directness="stated", obs=1, weight=0.65, asserted=0.7, valid_time=("2026-05-07", "2026-05-10"),
    edges=[], refs=["git:Hybrid mode"],
    claim="Every composer-plugin is handled by delegating to `composer run-script`; composr natively replicates none of them.",
    body="True for the first installer era; superseded once pest-plugin and spi were ported. Historical.")

belief(slug="plugin-policy-three-tier", session="C4", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=("2026-05-10", "2999-01-01"),
    edges=[("supersedes", "plugins-all-delegated")], refs=["README.md"],
    claim="composr classifies composer-plugins three ways: NATIVELY-REPLICATED (pest-plugin, tbachert/spi — byte-equal codegen ported to Rust, gated on config.allow-plugins), INERT (install files only — php-http/discovery, pest sub-plugins, phpstan/extension-installer), and UNKNOWN (force a per-event `composer run-script` delegation). `--strict-plugins` refuses any plugin in the lock.",
    body="Supersedes [[plugins-all-delegated]]. pest-plugin writes vendor/pest-plugins.json (without it every Pest plugin silently no-ops); spi writes GeneratedServiceProviderData.php. Project-local inert allowlists live in composr.json -> allow-inert-plugins (supports `*` wildcards).")

# ---------------------------------------------------------------------------
# Subsystems / perf
# ---------------------------------------------------------------------------
belief(slug="two-layer-classmap-cache", session="C4", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["README.md", "git:Persistent per-package classmap cache"],
    claim="Two classmap caches stack: a PER-PACKAGE shared cache keyed (package_name, dist.reference) at $XDG_CACHE_HOME/composr/classmap/... that survives `rm -rf vendor`, and a PER-FILE mtime cache (vendor/composer/.classmap-cache.bin) for the root + uncacheable (path/no-reference) packages. Together a changed-one-file install is sub-second.",
    body="A per-package cache hit skips walk + parse + admission entirely and merges cached entries straight into the global map.")

belief(slug="native-autoload-bootstrap", session="C2", turn=9, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["git:Native autoload bootstrap", "README.md"],
    claim="composr generates the full autoload bootstrap natively when missing (autoload.php, ClassLoader.php, autoload_real.php, autoload_static.php, the four data files, platform_check.php, LICENSE), and bundles composer/composer's InstalledVersions.php (MIT). When the bootstrap is already in place it fast-paths and just patches the classmap.",
    body="Killing the cold-start composer call for the autoload step. The fast path is what keeps warm re-runs cheap.")

belief(slug="extraction-is-bottleneck", session="C4", turn=7, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["README.md", "git:Parallelize zip extraction with rayon"],
    claim="After the first run, zip extraction (~14s, parallelized via rayon) DOMINATES cold-cold wall-clock — bound by disk metadata throughput and the largest single archive. Classmap walking is off the critical path once the cache is warm.",
    body="Names the real bottleneck once caching solved the classmap cost. Drove the extraction perf-tuning pass.")

# perf evolution: LPT partition -> single par_iter
belief(slug="lpt-partition-extract", session="C4", turn=9, author=AGENT,
    directness="stated", obs=1, weight=0.6, asserted=0.65, valid_time=("2026-05-07", "2026-05-10"),
    edges=[], refs=["git:Parallelize zip extraction with rayon"],
    claim="Zip extraction partitions archives into huge/small buckets and schedules them separately for balanced parallelism.",
    body="An early parallelism heuristic; measured not worth the complexity and dropped. Historical.")

belief(slug="single-par-iter-extract", session="C4", turn=10, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=("2026-05-10", "2999-01-01"),
    edges=[("supersedes", "lpt-partition-extract")], refs=["git:drop huge/small partition, single LPT par_iter"],
    claim="Extraction dropped the huge/small partition for a single LPT par_iter, and further trims overhead: dedup parent mkdirs, skip chmod for 0o644 entries, async-trash old install dirs in a detached child.",
    body="Supersedes [[lpt-partition-extract]]. Simpler scheduling + per-entry syscall trimming beat the bucketed scheme.")

belief(slug="rename-dumpr-to-composr", session="C2", turn=11, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["git:Rename to composr", "git:Drop autoload-dumpr binary alias, bump to 0.3.0"],
    claim="When it grew past dump-autoload the tool was renamed autoload-dumpr -> composr ('vowels not included'); the bare `composr` command stays an alias for `dump-autoload` for back-compat, and the autoload-dumpr binary alias was dropped at 0.3.0.",
    body="The rename tracks the scope jump in [[scope-installer]] — a name change IS a scope signal.")

# ---------------------------------------------------------------------------
# OBJECTIVELY WRONG beliefs — falsified hunches, defeated by verdicts. No
# valid_time on the wrong belief (never true); only `adjudicates` defeats it.
# ---------------------------------------------------------------------------

# Pair: pest-plugin "is inert" — a falsified classification (silent failure)
belief(slug="pest-plugin-is-inert", session="C2", turn=13, author=AGENT,
    directness="inferred", obs=1, weight=0.6, asserted=0.8, valid_time=None, edges=[],
    refs=["README.md"],
    claim="pest-plugin is an INERT composer-plugin — installing its files is enough; its composer-side install hook has no observable install-time effect, so composr can skip it.",
    body="A confident classification (asserted 0.8): it looked like just another library. The failure mode is silent, which is why the hunch survived until someone checked Pest actually ran.")

belief(slug="pest-plugin-not-inert", session="C4", turn=11, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "pest-plugin-is-inert"), ("attacks", "pest-plugin-is-inert"),
           ("supports", "plugin-policy-three-tier")],
    refs=["README.md"],
    claim="VERDICT: pest-plugin is NOT inert. Without writing vendor/pest-plugins.json, Pest's runtime Loader returns [] and EVERY Pest plugin (Coverage, Bail, Cache, Retry, Snapshot, Parallel, pest-plugin-arch, …) SILENTLY no-ops. composr had to natively replicate the codegen.",
    body="Defeats [[pest-plugin-is-inert]]; this is exactly why the [[plugin-policy-three-tier]] split exists. The worst kind of wrong: a plugin classified inert that silently disables a whole test toolchain with no error.")

# Pair: content hash "is a plain hash" — falsified on first compatibility check
belief(slug="content-hash-is-plain", session="C2", turn=15, author=AGENT,
    directness="inferred", obs=1, weight=0.55, asserted=0.7, valid_time=None, edges=[],
    refs=["git:Add `install` subcommand"],
    claim="The composer.lock content-hash is a straightforward hash of composer.json bytes that composr can recompute directly.",
    body="A reasonable-looking assumption while wiring the lock check. Wrong about composer's actual algorithm.")

belief(slug="content-hash-needs-php-shape", session="C4", turn=13, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.85, valid_time=None,
    edges=[("adjudicates", "content-hash-is-plain"), ("attacks", "content-hash-is-plain")],
    refs=["git:PHP-compatible composer.json content hash"],
    claim="VERDICT: the content hash must match composer's PHP-specific key normalization + serialization to be compatible — a plain byte hash mismatched. composr had to implement a PHP-compatible composer.json content hash.",
    body="Defeats [[content-hash-is-plain]]. Lower stakes (the check is advisory) but the same shape: 'it's obviously just a hash' was an under-specified read of a format owned by another tool.")

# ---------------------------------------------------------------------------
# Reducer consensus over the scope trajectory
# ---------------------------------------------------------------------------
belief(slug="r-composr-trajectory", session="C5", turn=20, author=REDUCER,
    directness="reduced", obs=4, weight=0.6, asserted=None, valid_time=None,
    edges=[("derived_from", "scope-dumpr"), ("derived_from", "scope-installer"),
           ("derived_from", "scope-plugin-replication"), ("derived_from", "goal-byte-equivalence")],
    refs=[],
    claim="composr's trajectory: a one-shot `dump-autoload -o` speedup -> a full lock-driven `composer install` replacement (native autoload + Laravel discover) -> native plugin replication -> a published 1.0 crate. The unchanging throughline: kill cold-start composer subprocess calls while staying byte-equivalent to composer.",
    body="REDUCED over the scope chain plus the [[goal-byte-equivalence]] spine. The property a flat store loses: enormous scope growth (one command -> end-to-end installer in a day) held together by two fixed goals — cold-start speed and byte-equivalence.")

# ---------------------------------------------------------------------------
# OBJECTIVELY WRONG DOCS (C6). README claims the code contradicts — and the
# README even contradicts ITSELF in one case.
# ---------------------------------------------------------------------------
belief(slug="doc-strict-plugins-default-inert", session="C6", turn=2, author=AGENT,
    directness="stated", obs=1, weight=0.8, asserted=0.85, valid_time=None, edges=[],
    refs=["README.md:204"],
    claim="Per the README's flag reference, the default plugin behavior is to 'install plugins as inert files and warn' (and `--strict-plugins` aborts instead).",
    body="Lifted from the README's `--strict-plugins` line. Falsified below — and it even contradicts the README's own plugin-policy section.")

belief(slug="unknown-plugins-delegate-not-inert", session="C6", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.95, asserted=0.95, valid_time=None,
    edges=[("adjudicates", "doc-strict-plugins-default-inert"), ("attacks", "doc-strict-plugins-default-inert"),
           ("supports", "plugin-policy-three-tier")],
    refs=["src/install.rs:155", "src/install.rs:629", "README.md (Plugin policy section)"],
    claim="VERDICT: FALSE. The default for UNKNOWN plugins is to DELEGATE each lifecycle event to `composer run-script` (so their subscribers fire) — not to install them as inert files. The README's own 'Plugin policy' section says exactly this; the parenthetical on the `--strict-plugins` line contradicts it and the code (src/install.rs unknown-plugin delegation path).",
    body="Defeats [[doc-strict-plugins-default-inert]] and corroborates [[plugin-policy-three-tier]]. A doc that contradicts ITSELF — the detailed section is right, the flag's parenthetical summary is wrong. 'Inert' is the one thing unknown plugins are NOT.")

belief(slug="composr-doc-defects", session="C6", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.85, valid_time=None, edges=[],
    refs=["README.md:247", "src/main.rs:325"],
    claim="Smaller objective README defect: the README shows the git-hook templates and says they're 'exactly what install-hooks writes', but the shown templates omit the `# managed by composr install-hooks` marker line that the real constants (src/main.rs, HOOK_MARKER) write as line 2 — the marker is load-bearing (it's how re-running install-hooks recognizes and overwrites its own hooks).",
    body="A TRUE observation about a wrong doc: 'exactly' is falsified by a missing line that matters for idempotent hook installation. Low stakes, ambient doc-rot.")

generate(
    title="composr",
    blurb="A Rust replacement for the slow parts of Composer-based PHP installs, "
          "tracked from a `dump-autoload` speedup to a published byte-equivalent installer.",
    subject_repo="~/personal/composr",
    out_dir=OUT,
    sessions=SESSIONS,
)
