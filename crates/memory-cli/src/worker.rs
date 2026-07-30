//! Lazy background work — daemon-less opportunistic consolidation
//! (docs/design/lazy-background-work.md).
//!
//! No server, no cron, no install step: each `mem` WRITE (remember / promote / onboard --commit)
//! counts toward a cheap due-check; when the threshold trips, the CLI re-execs itself as a
//! detached hidden `mem __worker` that runs the due sleep-stage passes — `consolidate` every time,
//! `dream` at a much longer cadence — without blocking the command the user actually ran. The next
//! foreground read (`recall`/`ask`) surfaces a one-line "what the background did". `git gc --auto`
//! is the prior art: check a cheap threshold, fork if due, lock so runs never pile up.
//!
//! State, lock, and log are sidecars in `$MEMORY_DIR` and are DISPOSABLE runtime state, not L0 —
//! "the layout is never the storage."

use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

const STATE_FILE: &str = ".worker-state.json";
const STATE_LOCK_FILE: &str = ".worker-state.lock";
const LOCK_FILE: &str = ".worker.lock";
const LOG_FILE: &str = ".worker.log";

/// Rotate `.worker.log` once it grows past this (one old generation kept).
const LOG_ROTATE_BYTES: u64 = 1_000_000;

/// A lock this old is presumed abandoned regardless of its pid (guards against pid reuse making a
/// long-dead holder look alive). Generous: a real pass is minutes, not hours.
const STALE_LOCK_SECS: u64 = 6 * 3600;

/// How many pairs a background dream probes (deliberately smaller than `mem dream`'s default 8 —
/// opportunistic passes stay cheap; the deliberate command is for going deep).
const DREAM_LIMIT: usize = 4;

// --- settings (a `[worker]` section in agentic-memory/config.toml) ----------------------

/// Knobs for the lazy worker. Work-based trigger is primary (never run on an unchanged store);
/// the time bounds are the throttle floor and the safety ceiling. `MEM_NO_BG` (env, any value)
/// and `worker.enabled=false` both switch the whole tier off.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct WorkerSettings {
    /// Master switch; set false in CI or for a quiet store.
    pub enabled: bool,
    /// Kick a consolidate once this many writes have accumulated ...
    pub consolidate_after_writes: u64,
    /// ... but never more often than this (minutes).
    pub min_interval_mins: u64,
    /// With ANY write pending, run at least this often (minutes) even below the write threshold.
    pub max_interval_mins: u64,
    /// Dream piggybacks on a due consolidate at most this often (minutes). Default: weekly.
    pub dream_interval_mins: u64,
    /// Hard cap on beliefs consolidated per background pass (bounds LLM calls — a big onboard
    /// import must not trigger a marathon). The tail beyond the cap is NOT revisited by later
    /// passes (selection is newest-first, no cursor); the run summary surfaces the
    /// `mem consolidate --limit N` that covers it.
    pub max_targets: u64,
}

impl Default for WorkerSettings {
    fn default() -> Self {
        WorkerSettings {
            enabled: true,
            consolidate_after_writes: 5,
            min_interval_mins: 10,
            max_interval_mins: 24 * 60,
            dream_interval_mins: 7 * 24 * 60,
            max_targets: 12,
        }
    }
}

fn worker_allowed(w: &WorkerSettings) -> bool {
    w.enabled && std::env::var_os("MEM_NO_BG").is_none()
}

// --- state (one JSON line, epoch-seconds; atomic replace) -------------------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkerState {
    /// Epoch secs of the last COMPLETED worker run (0 = never). A killed run doesn't update it,
    /// so it re-triggers — safe by construction (append-only L0 + idempotent edge commits).
    pub last_run: u64,
    /// Epoch secs of the last completed dream visit (its cadence is much longer).
    pub last_dream: u64,
    /// Store writes since `last_run` — the work signal the due-check reads.
    pub writes_since: u64,
    /// One line describing what the last run did (what `recall` surfaces).
    pub last_summary: String,
    /// When `last_summary` was written / last shown — surfacing shows each summary exactly once.
    pub summary_at: u64,
    pub surfaced_at: u64,
}

fn state_path(dir: &Path) -> PathBuf {
    dir.join(STATE_FILE)
}

pub fn load_state(dir: &Path) -> WorkerState {
    let Ok(text) = std::fs::read_to_string(state_path(dir)) else {
        return WorkerState::default();
    };
    WorkerState {
        last_run: jget_u64(&text, "\"last_run\":").unwrap_or(0),
        last_dream: jget_u64(&text, "\"last_dream\":").unwrap_or(0),
        writes_since: jget_u64(&text, "\"writes_since\":").unwrap_or(0),
        last_summary: crate::jget(&text, "\"last_summary\":\"").unwrap_or_default(),
        summary_at: jget_u64(&text, "\"summary_at\":").unwrap_or(0),
        surfaced_at: jget_u64(&text, "\"surfaced_at\":").unwrap_or(0),
    }
}

/// Atomic replace (write tmp, rename) so a concurrent reader never sees a torn file. Every
/// read-modify-write of the state goes through `lock_state` — the whole line is rewritten, so a
/// lost update would lose more than a counter blip (a run's summary, the `last_run` anchor).
pub fn save_state(dir: &Path, s: &WorkerState) {
    let json = format!(
        "{{\"last_run\":{},\"last_dream\":{},\"writes_since\":{},\"summary_at\":{},\"surfaced_at\":{},\"last_summary\":\"{}\"}}\n",
        s.last_run, s.last_dream, s.writes_since, s.summary_at, s.surfaced_at, sanitize(&s.last_summary)
    );
    let tmp = dir.join(format!("{STATE_FILE}.{}.tmp", std::process::id()));
    if std::fs::write(&tmp, json).is_ok() {
        let _ = std::fs::rename(&tmp, state_path(dir));
    }
}

/// Keep the summary safe inside our one-line hand-rolled JSON (same discipline as the novelty
/// ledger): no quotes, backslashes, or newlines survive.
fn sanitize(s: &str) -> String {
    s.replace('\\', " ").replace('"', "'").replace('\n', " ")
}

/// Read a bare (unquoted) u64 after `marker`. `crate::jget_num` is f32 — not enough mantissa for
/// epoch seconds, so timestamps get their own exact parser.
fn jget_u64(line: &str, marker: &str) -> Option<u64> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().parse().ok()
}

fn epoch_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Serializes state-file read-modify-writes across foreground commands AND the worker (the
/// `WorkerLock` below only serializes workers). Held for microseconds; a contender spins briefly,
/// steals an abandoned lock (≥10 s old — three orders of magnitude past a real hold), and after
/// ~250 ms proceeds UNLOCKED rather than ever hanging a foreground `mem` — a rare lost update
/// beats a blocked prompt.
struct StateLock {
    path: PathBuf,
}

fn lock_state(dir: &Path) -> Option<StateLock> {
    let path = dir.join(STATE_LOCK_FILE);
    for _ in 0..50 {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(_) => return Some(StateLock { path }),
            Err(_) => {
                let abandoned = std::fs::metadata(&path)
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .map(|t| match t.elapsed() {
                        Ok(d) => d.as_secs() >= 10,
                        // future mtime (clock stepped back): a µs-hold lock can't legitimately be
                        // from the future — steal it rather than paying the spin for hours. A
                        // wrong steal only re-opens the accepted proceed-unlocked mode.
                        Err(_) => true,
                    })
                    .unwrap_or(false);
                if abandoned {
                    let _ = std::fs::remove_file(&path);
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
    }
    None
}

impl Drop for StateLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

// --- due-check (cheap, deterministic, work-first) ---------------------------------------

/// Is a consolidate pass due? Work-based primary (threshold writes AND the min-interval throttle),
/// with a max-interval ceiling so a slow trickle of writes still consolidates eventually. Zero
/// pending writes → never due (an unchanged store needs no maintenance).
pub fn consolidate_due(s: &WorkerState, w: &WorkerSettings, now: u64) -> bool {
    if s.writes_since == 0 {
        return false;
    }
    let elapsed = now.saturating_sub(s.last_run);
    (s.writes_since >= w.consolidate_after_writes && elapsed >= w.min_interval_mins * 60)
        || elapsed >= w.max_interval_mins * 60
}

/// Does the last run have a summary the user hasn't seen yet?
fn unsurfaced(s: &WorkerState) -> bool {
    s.summary_at > s.surfaced_at && !s.last_summary.is_empty()
}

// --- foreground hooks -------------------------------------------------------------------

/// Count `n` fresh store writes and, if the sleep-stage threshold trips, kick a detached worker.
/// Called at the end of the write commands; MUST never block or break the foreground (all
/// failures are swallowed). On the very first write ever, intervals start counting from now —
/// a brand-new store doesn't fork a surprise LLM pass on write #1.
pub fn note_writes_and_kick(dir: &Path, n: usize) {
    if n == 0 {
        return;
    }
    let s = {
        let _guard = lock_state(dir);
        let existed = state_path(dir).exists();
        let mut s = load_state(dir);
        if !existed {
            let now = epoch_now();
            s.last_run = now;
            s.last_dream = now;
        }
        s.writes_since += n as u64;
        save_state(dir, &s);
        s
    }; // lock released before the (slower) spawn
    let w = crate::load_settings().worker;
    if worker_allowed(&w) && consolidate_due(&s, &w, epoch_now()) {
        let _ = spawn_detached(dir);
    }
}

/// Print the one-line "what the background did" (at the top of `recall`/`ask`) — each summary
/// exactly once. Reads/updates state; a running worker is NOT reported here (see `mem worker`).
pub fn surface_last_run(dir: &Path) {
    if !unsurfaced(&load_state(dir)) {
        return; // cheap unlocked pre-check: the common no-news path takes no lock
    }
    let _guard = lock_state(dir);
    let mut s = load_state(dir);
    if !unsurfaced(&s) {
        return; // someone else surfaced it between the pre-check and the lock
    }
    println!("🛠 background {} — {} ago\n", s.last_summary, ago(epoch_now().saturating_sub(s.summary_at)));
    s.surfaced_at = s.summary_at;
    save_state(dir, &s);
}

fn ago(secs: u64) -> String {
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86_400 {
        format!("{}h", secs / 3600)
    } else {
        format!("{}d", secs / 86_400)
    }
}

// --- detach -----------------------------------------------------------------------------

/// Re-exec ourselves as `mem __worker`, detached: stdio → `.worker.log`, own process group (so
/// the terminal's Ctrl-C never reaches it), never waited on — it outlives this CLI invocation.
/// Cwd is inherited ON PURPOSE: the worker derives the SAME active scope as the invocation that
/// triggered it ("the trigger is the user already being in the right place").
fn spawn_detached(dir: &Path) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let log_path = dir.join(LOG_FILE);
    // Bounded log: rotate once, keep one old generation. A racing worker's open fd follows the
    // rename harmlessly (it keeps appending to `.worker.log.1`).
    if std::fs::metadata(&log_path).map(|m| m.len() > LOG_ROTATE_BYTES).unwrap_or(false) {
        let _ = std::fs::rename(&log_path, dir.join(format!("{LOG_FILE}.1")));
    }
    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| e.to_string())?;
    let log_err = log.try_clone().map_err(|e| e.to_string())?;
    let mut cmd = std::process::Command::new(exe);
    cmd.arg("__worker")
        .stdin(std::process::Stdio::null())
        .stdout(log)
        .stderr(log_err);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    cmd.spawn().map(|_| ()).map_err(|e| e.to_string())
}

// --- lock (serialize workers; never pile up) --------------------------------------------

/// A pidfile lock (zero-dep, in the repo's shell-out-to-curl spirit — no libc/flock). Created
/// with O_EXCL; holds our pid; removed on drop. A lock whose holder is dead (or which is
/// impossibly old) is stolen. The steal has a tiny theoretical race (two stealers), which is
/// accepted: edge commits are idempotent, so a rare double-worker wastes LLM budget, corrupts
/// nothing.
pub struct WorkerLock {
    path: PathBuf,
}

impl WorkerLock {
    pub fn try_acquire(dir: &Path) -> Option<WorkerLock> {
        let path = dir.join(LOCK_FILE);
        for _ in 0..2 {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut f) => {
                    let _ = write!(f, "{}", std::process::id());
                    return Some(WorkerLock { path });
                }
                Err(_) => {
                    if !lock_is_stale(&path) {
                        return None; // genuinely held — skip silently
                    }
                    let _ = std::fs::remove_file(&path); // abandoned — take over, retry once
                }
            }
        }
        None
    }
}

impl Drop for WorkerLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn lock_is_stale(path: &Path) -> bool {
    // Age is one signal, not a gate: `elapsed()` errors when the mtime is in the FUTURE (clock
    // stepped back, restored VM snapshot) — fall through to the pid check rather than letting a
    // dead holder's lock block all background work until the wall clock catches up.
    let age = std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.elapsed().ok())
        .map(|d| d.as_secs());
    if matches!(age, Some(a) if a >= STALE_LOCK_SECS) {
        return true; // impossibly old (guards pid reuse)
    }
    match lock_holder(path) {
        Some(pid) => !pid_alive(pid),
        None => false, // vanished, or no pid recorded yet (holder mid-create) — contended
    }
}

fn lock_holder(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path).ok().and_then(|s| s.trim().parse().ok())
}

#[cfg(target_os = "linux")]
fn pid_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

/// Without /proc we can't probe liveness cheaply; claim alive and let the age ceiling decide.
#[cfg(not(target_os = "linux"))]
fn pid_alive(_pid: u32) -> bool {
    true
}

// --- the worker itself (`mem __worker`, hidden) -----------------------------------------

/// The detached body: lock, snapshot the pending-write count, run the due passes, then update
/// state. The claimed writes are DRAINED only when the pass actually consolidated something —
/// a failed pass (backend down) or an empty-scope pass keeps the backlog pending so it retries /
/// catches up later; `last_run` still advances, so the min-interval throttles those retries.
/// Only a KILLED run leaves state fully untouched and re-triggers immediately.
pub fn run_worker() {
    let dir = crate::store_dir();
    let w = crate::load_settings().worker;
    let Some(_lock) = WorkerLock::try_acquire(&dir) else {
        println!("[{}] worker: lock held — another worker is running; exiting", memory_core::iso_now());
        return;
    };
    let now = epoch_now();
    let state = load_state(&dir);
    let claimed = state.writes_since;
    let scopes = crate::active_scopes();
    println!(
        "[{}] worker: start (scopes: {}; {claimed} pending write(s))",
        memory_core::iso_now(),
        scopes.join("+")
    );

    let limit = pass_limit(claimed, w.max_targets);
    let mut parts: Vec<String> = Vec::new();
    let mut healthy = true;
    let mut did_work = false;
    match crate::consolidate_pass(&dir, &scopes, limit) {
        Ok((0, _, _)) => parts.push("consolidate: nothing in scope".into()),
        Ok((n, edges, review)) => {
            did_work = true;
            let mut p = format!("consolidate: {n} belief(s) → {edges} new edge(s)");
            if review > 0 {
                p.push_str(&format!(", ⚑{review} for review"));
            }
            // A capped pass leaves a tail the worker won't revisit (selection is newest-first,
            // no cursor) — surface the deliberate command that covers the whole window instead
            // of pretending later passes catch up.
            if claimed > n as u64 {
                p.push_str(&format!(" — capped at {n}; cover the rest: mem consolidate --limit {claimed}"));
            }
            parts.push(p);
        }
        Err(e) => {
            healthy = false;
            parts.push(format!("consolidate failed: {e}"));
        }
    }

    // Dream piggybacks on a healthy pass at its own (much longer) cadence — no extra forks.
    let mut dreamed = false;
    if healthy && now.saturating_sub(state.last_dream) >= w.dream_interval_mins * 60 {
        match crate::dream_pass(&dir, &scopes, DREAM_LIMIT) {
            Ok(o) => {
                dreamed = true; // even "too few beliefs" counts as a visit — don't re-check hourly
                if o.targets > 0 {
                    parts.push(format!("dream: {} bridge(s) / {} probe(s)", o.drawn, o.recorded));
                }
            }
            Err(e) => parts.push(format!("dream failed: {e}")),
        }
    }

    let summary = parts.join(" · ");
    {
        let _guard = lock_state(&dir);
        // Re-read state: the foreground may have recorded more writes while we ran — preserve them.
        let mut fresh = load_state(&dir);
        if did_work {
            // Drain only what a REAL pass covered; a failed or nothing-in-scope pass keeps the
            // backlog so it retries (or catches up when the user is next active in its scope).
            fresh.writes_since = fresh.writes_since.saturating_sub(claimed);
        }
        fresh.last_run = now;
        if dreamed {
            fresh.last_dream = now;
        }
        fresh.last_summary = sanitize(&summary);
        fresh.summary_at = now;
        save_state(&dir, &fresh);
    }
    println!("[{}] worker: done — {summary}", memory_core::iso_now());
}

/// Beliefs to consolidate this pass: the claimed backlog, at least 1 (a forced `--now` pass with
/// nothing pending still visits the newest belief), capped at `max_targets`. A configured
/// `max_targets` of 0 is treated as 1 — `clamp` on the inverted range would panic.
fn pass_limit(claimed: u64, max_targets: u64) -> usize {
    claimed.max(1).min(max_targets.max(1)) as usize
}

// --- the manual control surface (`mem worker [--now]`) ----------------------------------

pub fn cmd_worker(args: &[String]) {
    let dir = crate::store_dir();
    let w = crate::load_settings().worker;

    if args.iter().any(|a| a == "--now") {
        if !worker_allowed(&w) {
            println!("worker is disabled (worker.enabled=false or MEM_NO_BG set) — not kicking.");
            return;
        }
        match spawn_detached(&dir) {
            Ok(()) => println!(
                "worker kicked (detached). Watch it:  tail -f {}",
                dir.join(LOG_FILE).display()
            ),
            Err(e) => eprintln!("could not spawn worker: {e}"),
        }
        return;
    }

    // status
    let s = load_state(&dir);
    let now = epoch_now();
    println!(
        "worker: {}",
        if worker_allowed(&w) { "enabled" } else { "DISABLED (worker.enabled=false or MEM_NO_BG)" }
    );
    println!(
        "pending: {} write(s) (kicks at {} — or any pending write after {})",
        s.writes_since,
        w.consolidate_after_writes,
        ago(w.max_interval_mins * 60)
    );
    if s.last_run > 0 {
        let what = if s.last_summary.is_empty() { "(no summary)" } else { s.last_summary.as_str() };
        println!("last run: {} ago — {what}", ago(now.saturating_sub(s.last_run)));
    } else {
        println!("last run: never");
    }
    let lock_path = dir.join(LOCK_FILE);
    if lock_path.exists() {
        match lock_holder(&lock_path) {
            Some(pid) if pid_alive(pid) => println!("lock: HELD by pid {pid} (a worker is running now)"),
            _ => println!("lock: stale (holder gone — the next kick reclaims it)"),
        }
    } else {
        println!("lock: free");
    }
    println!("due now: {}", if consolidate_due(&s, &w, now) { "yes" } else { "no" });
    println!("log: {}", dir.join(LOG_FILE).display());
    println!("\nforce a pass: mem worker --now   ·   switch off: MEM_NO_BG=1 (or worker.enabled=false in config.toml)");
}

// --- tests ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mem-worker-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn state_round_trips_through_the_sidecar_file() {
        let dir = tmp("state");
        let s = WorkerState {
            last_run: 1_753_900_000, // > 2^24: would corrupt through an f32 parse
            last_dream: 1_753_000_000,
            writes_since: 7,
            last_summary: "consolidate: 3 belief(s) → 2 new edge(s)".into(),
            summary_at: 1_753_900_001,
            surfaced_at: 1_753_899_000,
        };
        save_state(&dir, &s);
        assert_eq!(load_state(&dir), s, "exact round-trip (epochs must not lose precision)");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_state_file_is_the_default() {
        let dir = tmp("nostate");
        assert_eq!(load_state(&dir), WorkerState::default());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn summary_with_json_breaking_chars_is_sanitized_not_corrupting() {
        let dir = tmp("sanitize");
        let s = WorkerState {
            last_summary: "drew \"edge\" a\\b\nnext".into(),
            summary_at: 5,
            ..WorkerState::default()
        };
        save_state(&dir, &s);
        let back = load_state(&dir);
        assert_eq!(back.last_summary, "drew 'edge' a b next");
        assert_eq!(back.summary_at, 5, "the rest of the state survives the odd summary");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn due_check_is_work_first_with_time_bounds() {
        let w = WorkerSettings::default(); // 5 writes, 10m floor, 24h ceiling
        let base = 1_000_000u64;
        let s = |writes, last_run| WorkerState { writes_since: writes, last_run, ..WorkerState::default() };

        // no pending writes → never due, even after the ceiling
        assert!(!consolidate_due(&s(0, base), &w, base + 48 * 3600));
        // threshold met but inside the min-interval throttle → not due
        assert!(!consolidate_due(&s(5, base), &w, base + 60));
        // threshold met and past the throttle → due
        assert!(consolidate_due(&s(5, base), &w, base + 601));
        // below threshold and below ceiling → not due
        assert!(!consolidate_due(&s(1, base), &w, base + 3600));
        // below threshold but past the max-interval ceiling → due (slow trickle still consolidates)
        assert!(consolidate_due(&s(1, base), &w, base + 25 * 3600));
    }

    #[test]
    fn first_write_initializes_intervals_no_surprise_first_fork() {
        // On a brand-new store the very first write must set last_run=now (not 0), so the
        // max-interval ceiling can't make write #1 immediately due.
        let dir = tmp("firstwrite");
        std::env::set_var("MEM_NO_BG", "1"); // belt: never spawn from a unit test
        note_writes_and_kick(&dir, 1);
        let s = load_state(&dir);
        assert_eq!(s.writes_since, 1);
        assert!(s.last_run > 0, "intervals start from first use");
        assert!(s.last_dream > 0);
        assert!(!consolidate_due(&s, &WorkerSettings::default(), epoch_now()));

        // subsequent writes accumulate without resetting the anchor
        note_writes_and_kick(&dir, 3);
        let s2 = load_state(&dir);
        assert_eq!(s2.writes_since, 4);
        assert_eq!(s2.last_run, s.last_run);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unsurfaced_only_when_a_fresh_summary_exists() {
        let mut s = WorkerState::default();
        assert!(!unsurfaced(&s), "empty state has nothing to show");
        s.last_summary = "consolidate: 1 belief(s) → 0 new edge(s)".into();
        s.summary_at = 100;
        s.surfaced_at = 0;
        assert!(unsurfaced(&s));
        s.surfaced_at = 100; // shown once → never again
        assert!(!unsurfaced(&s));
        s.summary_at = 0;
        s.surfaced_at = 0;
        assert!(!unsurfaced(&s), "a summary with no timestamp is not fresh");
    }

    #[test]
    fn lock_excludes_second_holder_and_frees_on_drop() {
        let dir = tmp("lock");
        let l1 = WorkerLock::try_acquire(&dir).expect("first acquire");
        assert!(WorkerLock::try_acquire(&dir).is_none(), "held by a live pid (ours) — refused");
        drop(l1);
        let l2 = WorkerLock::try_acquire(&dir).expect("freed on drop");
        drop(l2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn dead_holders_lock_is_stolen() {
        let dir = tmp("stale");
        // a real, dead pid: spawn a no-op and wait for it
        let child = std::process::Command::new("true").spawn().expect("spawn true");
        let dead_pid = child.id();
        let mut child = child;
        child.wait().unwrap();
        std::fs::write(dir.join(LOCK_FILE), dead_pid.to_string()).unwrap();
        let l = WorkerLock::try_acquire(&dir);
        assert!(l.is_some(), "a lock whose holder is dead is stolen");
        drop(l);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pass_limit_never_panics_and_bounds_the_backlog() {
        assert_eq!(pass_limit(0, 12), 1, "forced pass with nothing pending still visits 1");
        assert_eq!(pass_limit(5, 12), 5, "backlog below the cap passes through");
        assert_eq!(pass_limit(40, 12), 12, "a big import is capped, catches up across passes");
        assert_eq!(pass_limit(5, 0), 1, "max_targets=0 must not panic (inverted clamp range)");
        assert_eq!(pass_limit(0, 0), 1);
    }

    #[test]
    fn state_lock_excludes_then_frees() {
        let dir = tmp("statelock");
        let l1 = lock_state(&dir).expect("first state lock");
        // a live contender spins its ~250ms and then gives up (proceed-unlocked policy)
        assert!(lock_state(&dir).is_none(), "held → contender must not acquire");
        drop(l1);
        assert!(lock_state(&dir).is_some(), "freed on drop");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn state_lock_steals_an_abandoned_lock() {
        let dir = tmp("statelock-stale");
        let path = dir.join(STATE_LOCK_FILE);
        std::fs::write(&path, "").unwrap();
        // age the lock past the 10s abandonment bar (µs holds never get near it)
        assert!(std::process::Command::new("touch")
            .args(["-d", "1 hour ago"])
            .arg(&path)
            .status()
            .map(|s| s.success())
            .unwrap_or(false));
        assert!(lock_state(&dir).is_some(), "an abandoned state lock is stolen, not spun on");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ago_renders_coarse_human_units() {
        assert_eq!(ago(30), "30s");
        assert_eq!(ago(180), "3m");
        assert_eq!(ago(7200), "2h");
        assert_eq!(ago(3 * 86_400), "3d");
    }
}
