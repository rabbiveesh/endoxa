# Lazy background work — daemon-less opportunistic consolidation

Status: **SHIPPED (first cut)** · 2026-07-30 · originally a proposal + design spec (2026-06-15):
"don't run a worker daemon; let the CLI notice when maintenance is due and kick it off detached."
The mechanism below is implemented in `memory-cli/src/worker.rs`; deltas from the spec and the
open-question decisions are recorded at the bottom (§ "As built").

## The idea (one paragraph)

The worker tier — `consolidate` (LLM linker), `dream` (novelty), `reduce` (duplicate fold) — is
background, agentic "sleep-stage" work. Instead of an always-running daemon, **each `mem`
invocation, after finishing its foreground job, checks whether a maintenance task is due and, if so,
spawns it as a detached background process** that runs without blocking the command the user
actually ran. The next invocation surfaces a one-line "what the background did." No server, no cron,
no install step — the CLI is its own scheduler.

## My assessment — do it; it's the right shape for endoxa

**Strong yes, with a tight first cut.** Three reasons it fits this system specifically:

1. **It preserves the load-bearing invariant.** endoxa is deliberately a *per-invocation CLI that
   reads LOCAL git state to derive scope* (that's why it's `mem`, not a server). A daemon breaks
   that: which process holds the git context, for which repo, with which branch scope? The
   CLI-triggers-its-own-maintenance pattern keeps "no server, runs per-invocation" intact — the
   trigger is just the user already being in the right place.
2. **It matches the metaphor the design already commits to.** Consolidation runs at `Cadence::Nrem`
   / `Rem` — "sleep stages." Real memory consolidates during rest *that follows activity*. A CLI
   invocation IS the moment of activity; using it to opportunistically trigger a sleep-stage pass is
   conceptually exact, not a hack.
3. **It's battle-tested prior art.** `git gc --auto` (runs after commits once loose-object
   thresholds trip), package-manager cache GC, editors' idle tasks. The pattern works; we're not
   inventing scheduling, just borrowing `gc --auto`'s "check a cheap threshold, fork if due, lock so
   you don't pile up."

The one thing to respect: the worker tier here calls an **LLM on a shared GPU**, which `git gc`
doesn't. That makes contention and "surprise heavy work" the real risks (see below), and is why the
first cut should trigger conservatively. But the architecture is right.

## Mechanism (the pipeline)

```
mem remember "<claim>"           # foreground: write + on-write hints, returns immediately
        │
        └─(after foreground)─► maybe_kick_worker():
              1. due-check     read .worker-state.json: writes-since ≥ N AND elapsed ≥ min_interval?
              2. lock          flock(.worker.lock) non-blocking — already running? → skip
              3. detach        double-fork / setsid; stdio → .worker.log; survive parent exit
              4. run           `mem __worker`: consolidate (budgeted, idempotent) → update state
        ▼
mem recall "<q>" (next time)     # surfaces: "🛠 background consolidate ran 3m ago — drew 4 edges"
```

1. **Trigger point — writes, not reads.** Kick the check after `remember` (new unlinked beliefs are
   exactly the work consolidation exists to do). `recall`/`ask` should *surface* the last run but not
   *trigger* one (keeps the read path latency-free and avoids fork storms from rapid recalls).
2. **Due-check — cheap, deterministic, work-first.** A tiny state file
   `$MEMORY_DIR/.worker-state.json`: `{ last_run, writes_since_last, last_summary }`. A task is due
   when `writes_since_last ≥ consolidate_after_writes` **and** `now − last_run ≥ min_interval`, with a
   `max_interval` ceiling so a low-write store still consolidates eventually. Work-based is primary
   (never run on an unchanged store); time bounds are the floor and the safety ceiling.
3. **Lock — serialize, never pile up.** Non-blocking `flock` on `$MEMORY_DIR/.worker.lock`. Held →
   another worker is running → skip silently. This is what makes rapid invocations safe.
4. **Detach — survive and stay quiet.** Re-exec `current_exe()` as `mem __worker` (hidden
   subcommand) with `stdin=null`, `stdout=stderr=.worker.log`, in a new session/process-group
   (`setsid`) so it outlives the parent CLI and never touches the user's terminal.
5. **Run — idempotent + budgeted.** The worker runs the due tasks (start with `consolidate` only),
   bounded by the existing per-write incrementality (re-link only new beliefs' neighborhoods), and
   updates `.worker-state.json` **only on clean completion** (a killed run re-triggers; append-only +
   content-addressed commits mean no corruption and a free resume).
6. **Surface — observability without a daemon.** On the next foreground command, if a run completed
   since last seen, print one line: `🛠 background consolidate ran 3m ago — drew 4 edge(s)` (read
   from `last_summary`). The existing `⚑ N edge(s) for review` nudge composes with this.

## Knobs (config in `agentic-memory/config.toml`)

| Key | Default | Meaning |
|---|---|---|
| `worker.enabled` | `true` | master switch; CI/quiet → `false` |
| `worker.consolidate_after_writes` | `5` | work threshold |
| `worker.min_interval` | `10m` | don't run more often than this |
| `worker.max_interval` | `24h` | run at least this often if any writes pending |
| `worker.tasks` | `["consolidate"]` | which tasks are opportunistic (add `dream`/`reduce` later) |
| env `MEM_NO_BG=1` | — | per-invocation escape hatch |

## Risks & mitigations

1. **GPU/Ollama contention** — a background `consolidate` racing a foreground `mem ask` for the GPU.
   *Mitigate:* the store-global lock means at most one LLM worker; additionally, **don't kick the
   worker when the foreground command itself uses the LLM** (`ask`, `consolidate`, `dream`), and the
   worker skips gracefully if Ollama is down/busy.
2. **Surprise heavy work / battery** — an unexpected LLM job on a laptop. *Mitigate:* trigger on
   writes only (an intent signal), `min_interval` throttle, the one-line surfacing makes it
   non-surprising, and `worker.enabled=false` / `MEM_NO_BG` opt out.
3. **Partial/killed run** — process killed mid-consolidate. *Safe by construction:* append-only L0 +
   content-addressed idempotent edge commits + the `dream` probe ledger ⇒ a re-run resumes, no
   corruption. State timestamp advances only on clean exit, so a killed run simply re-triggers.
4. **Fork storms** — many quick invocations. *Mitigate:* writes-only trigger + lock + `min_interval`.
5. **Self-exe / detach portability** — `current_exe()` for the re-exec; `setsid` is Unix (fine —
   target is Linux; note it if Windows ever matters).

## First-cut implementation sketch (Rust, `memory-cli`)

- Hidden subcommand `__worker` → runs `cmd_consolidate`-equivalent over the due scope, logging to
  `.worker.log`, updating `.worker-state.json`.
- `fn maybe_kick_worker(dir)`: due-check → `flock` try-lock → if acquired, `Command::new(current_exe)
  .arg("__worker").stdin(Stdio::null()).stdout(log).stderr(log)` + `pre_exec(setsid)` → spawn, drop
  (don't wait). Call it at the end of `cmd_remember` (guarded by config + `MEM_NO_BG`).
- `fn surface_last_run(dir)`: at the top of `cmd_recall`/`cmd_ask`, read `last_summary` and print the
  one-liner if newer than last surfaced.
- State + lock + log files live in `$MEMORY_DIR` and are **disposable runtime state, not L0** — fully
  consistent with "the layout is never the storage."

## Open questions

- **Scope of a background run:** store-wide consolidation of all unlinked beliefs, or only the
  trigger invocation's active scope? Leaning store-wide (consolidation is global maintenance; the
  per-write incrementality already bounds cost), but it interacts with branch scopes — decide.
- **Which tasks beyond `consolidate`:** `dream`/`reduce` at rarer cadences (their own thresholds), or
  keep them manual? Leaning: `consolidate` opportunistic first; `dream` opportunistic at a much
  longer `max_interval`; `reduce` manual.
- **Budget cap per run:** a hard ceiling on LLM calls per background pass (beyond the neighborhood
  incrementality), so a big import can't trigger a marathon. Tie to a `worker.max_llm_calls`.
- **Manual control surface:** a `mem worker --status` / `--now` for debugging and explicit kicks.

## Relationship to the rest

- This is the *delivery mechanism* for the `Cadence::Nrem`/`Rem` linkers that already exist — it
  doesn't change what they do, only *when/how* they're triggered.
- Composes with the frontier-review surface (`mem review`): a background `consolidate` draws
  `supports`/`refines`, and the `⚑ N for review` nudge tells the agent there are candidate
  `depends_on` to adjudicate — so opportunistic linking feeds on-demand frontier adjudication.

## As built (2026-07-30, `memory-cli/src/worker.rs`)

The shipped first cut follows the mechanism above with these deltas and decisions:

- **Trigger points:** every store WRITE counts — `remember` (+1), `promote` (+promoted count),
  `onboard --commit` (+committed count) — via `note_writes_and_kick`. Reads never trigger, per spec.
- **Scope of a background run (spec open question → DECIDED):** the worker inherits the triggering
  invocation's cwd and derives the SAME active scope, not store-wide. This is the spec's own reason
  №1 taken seriously — "the trigger is the user already being in the right place" — and it prevents
  a global pass from drawing edges between unrelated branch scopes. The pending-write counter is
  drained only by a pass that actually consolidated targets, so a pass that finds nothing in scope
  leaves the backlog pending and repo A's writes re-trigger when you're next active in A. Known
  imprecision: the counter is store-global, so a pass in repo B that DOES have targets drains it
  even when the pending writes were A's (harmless — consolidation is idempotent and newest-first;
  per-scope counters are a later refinement if it ever bites).
- **Tasks (spec open question → DECIDED):** `consolidate` opportunistic (bounded by the pending
  write count, capped at `worker.max_targets`); `dream` piggybacks on a healthy pass at its own much
  longer cadence (`worker.dream_interval_mins`, default weekly, `--limit 4`); `reduce` stays manual.
- **Budget cap (spec open question → DECIDED):** per-pass targets = `min(pending_writes,
  max_targets)` — a big onboard import consolidates a chunk per pass and catches up across passes,
  never a marathon.
- **Manual control surface (spec open question → DECIDED):** `mem worker` (status: pending count,
  last run, lock holder, due-now, log path) and `mem worker --now` (forced detached pass, still
  lock-serialized).
- **Lock:** a **pidfile** (O_EXCL create, holder pid inside, dropped on exit), not `flock` — zero
  new dependencies, in the shell-out-to-curl spirit. Stale locks (holder dead per `/proc`, or older
  than 6 h) are stolen; the theoretical double-steal race is accepted because edge commits are
  idempotent (a rare double worker wastes LLM budget, corrupts nothing).
- **Detach:** re-exec `current_exe()` as hidden `mem __worker` with stdio → `.worker.log` and
  `process_group(0)` (std, no libc `setsid` needed) so the terminal's Ctrl-C never reaches it.
- **State:** `.worker-state.json` — epoch-second timestamps (exact u64 parse; the f32 ledger parser
  would corrupt epochs), atomic tmp+rename replace, `writes_since` drained by subtraction so writes
  landing mid-run stay pending. Every read-modify-write (foreground count, surfacing, the worker's
  final update) is serialized by a `.worker-state.lock` micro-lock (µs hold; a contender spins
  ~250 ms then proceeds unlocked rather than ever hanging the foreground). A FAILED pass advances
  `last_run` (min-interval throttles retries; the failure is surfaced honestly) but does NOT drain
  the backlog, so the work retries; only a killed run re-triggers immediately. `.worker.log`
  rotates once past ~1 MB (one old generation kept).
- **First-write anchor:** creating the state file stamps `last_run`/`last_dream` = now, so a
  brand-new store doesn't fork a surprise LLM pass on write №1; intervals count from first use.
- **Knob names:** `worker.enabled`, `worker.consolidate_after_writes`, `worker.min_interval_mins`,
  `worker.max_interval_mins`, `worker.dream_interval_mins`, `worker.max_targets` (integer minutes,
  not duration strings — the hand-rolled config layer stays dumb), plus env `MEM_NO_BG=1`. The
  multi-word knobs are config.toml-only: the `MEM_*` env layer's `_` separator can't address them
  (`MEM_WORKER_MAX_TARGETS` → `worker.max.targets`, silently ignored).
- **Not built (yet):** the "don't kick when the foreground command itself uses the LLM" refinement
  (risk №1) — moot today because none of the WRITE commands that kick are LLM-on-read, and the lock
  already serializes workers; revisit if `ask` ever becomes a trigger point.
