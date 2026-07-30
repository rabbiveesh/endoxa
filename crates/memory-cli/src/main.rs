//! `mem` — the CLI surface for the belief-memory (replaces the MCP server).
//!
//! Runs in the repo per-invocation, so it sees LOCAL (unpushed) git state and derives the
//! active scope from it. Degrades gracefully outside a git repo (→ global scope only).
//!
//!   mem remember "<claim>" [--supersedes <slug|id>] [--global] [--ref R]... [--body B]
//!   mem recall   "<query>" [--limit N]
//!   mem scope                         # show the active scopes here
//!
//! Store: $MEMORY_DIR (default ~/.local/share/agentic-memory/beliefs).

use memory_consolidate::{novelty_pair_key, Consolidator, NoveltyDreamer, Orchestrator, ProbeRecord, Reducer};
use memory_core::{
    content_id, cosine, iso_now, Belief, Cadence, Confidence, EdgeKind, Graph, Hint, LinkCtx,
    LinkProposal, Relation, StructuralConfidence,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

mod worker;

// --- settings (memory-cli ONLY; layered defaults < file < env via the `config` crate) ---

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Settings {
    recall: RecallSettings,
    worker: worker::WorkerSettings,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct RecallSettings {
    /// L1 serendipity: push a cross-domain bridge line after recall hits.
    bridges: bool,
}

impl Default for RecallSettings {
    fn default() -> Self {
        RecallSettings { bridges: true }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings { recall: RecallSettings::default(), worker: worker::WorkerSettings::default() }
    }
}

/// Layered load (precedence: defaults < file < env), done OFF-THE-SHELF by the `config` crate.
///  - file: `$XDG_CONFIG_HOME/agentic-memory/config.toml` (fallback `$HOME/.config/...`), optional.
///  - env: `MEM_RECALL_BRIDGES=false` (prefix MEM, `_` separator, parsed → bool). CAVEAT: the `_`
///    separator means only SINGLE-word keys are env-addressable (`MEM_WORKER_ENABLED` works;
///    `MEM_WORKER_MAX_TARGETS` parses as `worker.max.targets` and is silently ignored) — multi-word
///    worker knobs are config.toml-only; the worker kill switch is the separate `MEM_NO_BG=1`.
/// On ANY error we degrade to Settings::default().
fn load_settings() -> Settings {
    let cfg_path = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("agentic-memory/config.toml")
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".config/agentic-memory/config.toml")
    };
    let built = config::Config::builder()
        .add_source(config::File::from(cfg_path).required(false))
        .add_source(
            config::Environment::with_prefix("MEM")
                .separator("_")
                .try_parsing(true),
        )
        .build();
    match built.and_then(|c| c.try_deserialize::<Settings>()) {
        Ok(s) => s,
        Err(_) => Settings::default(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("remember") => cmd_remember(&args[1..]),
        Some("recall") => cmd_recall(&args[1..]),
        Some("expand") => cmd_expand(&args[1..]),
        Some("ask") => cmd_ask(&args[1..]),
        Some("forget") => cmd_forget(&args[1..]),
        Some("promote") => cmd_promote(&args[1..]),
        Some("consolidate") => cmd_consolidate(&args[1..]),
        Some("reduce") => cmd_reduce(&args[1..]),
        Some("dream") => cmd_dream(&args[1..]),
        Some("review") => cmd_review(&args[1..]),
        Some("link") => cmd_link(&args[1..]),
        Some("debt") => cmd_debt(&args[1..]),
        Some("onboard") => cmd_onboard(&args[1..]),
        Some("worker") => worker::cmd_worker(&args[1..]),
        Some("__worker") => worker::run_worker(),
        Some("scope") => println!("active scopes: {}", active_scopes().join(", ")),
        _ => {
            eprintln!("usage:");
            eprintln!("  mem remember \"<claim>\" [--supersedes <slug|id>] [--global] [--ref R] [--body B]");
            eprintln!("  mem recall \"<query>\" [--limit N] [--no-bridges]");
            eprintln!("  mem expand <slug|id>          # one-hop: show a belief's linked context");
            eprintln!("  mem ask \"<question>\" [--limit N]  # LLM-synthesized grounded answer (opt-in)");
            eprintln!("  mem forget <slug|id> [--reason R]  # retract a belief: recall drops it (file kept)");
            eprintln!("  mem promote [<branch>] [--dry-run]  # lift a merged branch's beliefs into repo canon");
            eprintln!("  mem consolidate [--limit N]   # run the LLM judge over recent beliefs");
            eprintln!("  mem reduce [--dry-run]        # collapse duplicate beliefs: recall folds them behind one rep");
            eprintln!("  mem dream [--limit N]         # REM/novelty pass: bridge the most-unrelated pairs");
            eprintln!("  mem review [--limit N]        # list edges flagged for frontier review (candidate depends_on)");
            eprintln!("  mem link <subj> <kind> <obj> [--rationale R]  # author a durable edge (frontier/human adjudication)");
            eprintln!("  mem debt [<query>]            # list known-debt (deficiency) beliefs; ⚠ resurfaces when a blocked_on constraint lifts");
            eprintln!("  mem onboard [<repo>] [--out DIR] [--top N] [--escalate N] [--tier2] [--commit]  # lead harvest → claim drafts (+§3b debt envelope) → store");
            eprintln!("  mem worker [--now]            # background maintenance: show status (--now forces a pass)");
            eprintln!("  mem scope");
            std::process::exit(2);
        }
    }
}

// --- commands --------------------------------------------------------------------------

fn cmd_remember(args: &[String]) {
    let mut claim = String::new();
    let mut refs: Vec<String> = Vec::new();
    let mut body = String::new();
    let mut supersedes: Option<String> = None;
    let mut global = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--global" => global = true,
            "--supersedes" => { supersedes = args.get(i + 1).cloned(); i += 1; }
            "--ref" => { if let Some(r) = args.get(i + 1) { refs.push(r.clone()); } i += 1; }
            "--body" => { body = args.get(i + 1).cloned().unwrap_or_default(); i += 1; }
            s if claim.is_empty() => claim = s.to_string(),
            _ => {}
        }
        i += 1;
    }
    if claim.trim().is_empty() {
        eprintln!("remember needs a claim");
        std::process::exit(2);
    }

    let dir = store_dir();
    let scope = write_scope(global);
    let txn = iso_now();
    let id = content_id(&format!("{}|{txn}", claim.trim()));
    let slug = slugify(claim.trim());
    let md = belief_md(&id, &slug, &scope, claim.trim(), &refs, &body, &txn);
    if let Err(e) = std::fs::write(dir.join(format!("{id}.md")), md) {
        eprintln!("failed to write: {e}");
        std::process::exit(1);
    }

    let hints: Vec<Hint> = supersedes
        .iter()
        .map(|r| Hint { kind: EdgeKind::Supersedes, target_ref: r.clone() })
        .collect();
    let linked = consolidate(&dir, &id, &scope, &hints);
    let note = if linked > 0 { format!(" — Linker drew {linked} edge(s)") } else { String::new() };
    println!("remembered {id} [{slug}] in scope={scope}{note}");
    // Lazy background work: writes are the intent signal. Count this one; if the sleep-stage
    // threshold trips, kick a detached `mem __worker` (never blocks or breaks the foreground).
    worker::note_writes_and_kick(&dir, 1);
}

fn cmd_recall(args: &[String]) {
    let mut query = String::new();
    let mut limit = 10usize;
    let mut no_bridges = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => { limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(10); i += 1; }
            "--no-bridges" => no_bridges = true,
            s if query.is_empty() => query = s.to_string(),
            s => { query.push(' '); query.push_str(s); }
        }
        i += 1;
    }
    let query = query.trim().to_string();
    if query.is_empty() {
        eprintln!("recall needs a query");
        std::process::exit(2);
    }

    let dir = store_dir();
    worker::surface_last_run(&dir); // one-line "what the background did", at most once per run
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet. Add one:  mem remember \"<a thing you learned>\""); return; }
    };
    // scope-filter BEFORE resolving the frontier (gives branch divergence for free)
    let sg = scoped_graph(&g, &scopes);
    if sg.is_empty() {
        println!(
            "Nothing in scope ({}). Add one:  mem remember \"<claim>\"   (or onboard a repo:  mem onboard <repo>)",
            scopes.join(", ")
        );
        return;
    }
    let defeated = sg.defeated();
    let current = sg.current_content(&defeated);

    let dropped = sg.content().filter(|b| defeated.contains(&b.id)).count();
    let (mut hits, mode) = match rank(&dir, &current, &query) {
        Ok(r) => (r, format!("semantic; scopes: {}", scopes.join("+"))),
        Err(reason) => (lexical(&current, &query), format!("lexical fallback ({reason})")),
    };
    if hits.is_empty() {
        if dropped > 0 {
            println!("Nothing CURRENT for \"{query}\" — but {dropped} matching belief(s) here were superseded/retracted (the answer changed over time).");
        } else {
            println!("Nothing recalled for \"{query}\". Try fewer/broader words, or add it:  mem remember \"<claim>\"");
        }
        return;
    }
    // Current-edge index (drives the affordances below).
    let adj = sg.adjacency(&defeated);
    // CONFIDENCE BOOST (surfacing-stage, V1): gently re-rank current hits by their STRUCTURAL
    // confidence weight — directness × source_weight × corroboration × recency — not corroboration
    // alone. V1 measured recency + directness as load-bearing (the supports-only boost missed them).
    // Capped + multiplicative so a higher-confidence belief pulls a near-tie into the top-N but never
    // leapfrogs a clearly-nearer hit (semantic similarity stays dominant). Re-rank BEFORE truncate.
    // Lexical fallback (None scores) is left untouched. Truth/frontier are unchanged.
    let conf = StructuralConfidence::build(&sg, &defeated);
    for (id, _slug, _claim, score) in hits.iter_mut() {
        if let Some(s) = score {
            if let Some(b) = sg.by_id(id) {
                *s *= confidence_boost(conf.weight(b));
            }
        }
    }
    hits.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    // DUPLICATE FOLD (surfacing-stage, AFTER ranking, BEFORE truncate). A `same-as` cluster takes
    // ONE display slot: rewrite each member-hit onto its representative (surface the rep's
    // slug+claim, keep the BEST score seen for the cluster), then dedup so the cluster occupies a
    // single row. Frontier/defeated logic is untouched — every member stays current on disk.
    let (fold, absorbed) = collapse_map(&sg, &defeated);
    if !fold.is_empty() {
        let rep_of = |id: &str| -> String { fold.get(id).cloned().unwrap_or_else(|| id.to_string()) };
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut folded: Vec<Hit> = Vec::with_capacity(hits.len());
        for (id, slug, claim, score) in hits.into_iter() {
            let rep = rep_of(&id);
            if !seen.insert(rep.clone()) {
                continue; // this cluster already has its one slot (best score wins, kept first)
            }
            // surface the representative's slug + claim (fall back to the hit if rep not found)
            let (rslug, rclaim) = sg
                .by_id(&rep)
                .map(|b| (b.slug.clone(), b.claim.clone()))
                .unwrap_or((slug, claim));
            folded.push((rep, rslug, rclaim, score));
        }
        hits = folded;
    }

    hits.truncate(limit);
    println!(
        "Recalled {} current belief(s) [{mode}; {dropped} superseded/refuted dropped]:\n",
        hits.len()
    );
    // Affordances: annotate each hit with its current edge-counts so the robot KNOWS what's
    // expandable (and can probe with `mem expand`) — we don't walk the graph for it here.
    let slug_of = |id: &str| sg.by_id(id).map(|b| b.slug.clone());
    let mut any_expandable = false;
    for (id, slug, claim, score) in &hits {
        let raw = adj.get(id).map(|v| v.as_slice()).unwrap_or(&[]);
        let edges = collapse(id, raw);
        let (aff, contested) = affordance_str(&edges);
        if !aff.is_empty() {
            any_expandable = true;
        }
        let tag = if aff.is_empty() { String::new() } else { format!("   ({aff} → mem expand)") };
        // Name what it conflicts with (the other endpoint of the attacks edge), falling back to a
        // bare flag if we somehow can't resolve a slug. Carry the Dubois–Prade ⟨necessity,possibility⟩
        // band (V1 affordance, carry-don't-drive): a contested CURRENT belief keeps its necessity but
        // shows a depressed possibility — the honest "still current, but under live attack" signal.
        let warn = if contested {
            let named = contest_str(id, &edges, &slug_of);
            let band = sg
                .by_id(id)
                .map(|b| {
                    let (poss, nec) = conf.possibility_necessity(b);
                    format!(" [nec {nec:.2}·poss {poss:.2}]")
                })
                .unwrap_or_default();
            if named.is_empty() {
                format!("  ⚠ contested{band}")
            } else {
                format!("  ⚠ {named}{band}")
            }
        } else {
            String::new()
        };
        // Duplicate-fold tag: how many members this representative absorbed (surfacing-stage only).
        let dup = match absorbed.get(id) {
            Some(n) if *n > 0 => format!("  ⊕{n} dup"),
            _ => String::new(),
        };
        match score {
            Some(s) => println!("• ({s:.2}) [{slug}] {claim}{tag}{dup}{warn}"),
            None => println!("• [{slug}] {claim}{tag}{dup}{warn}"),
        }
    }
    if any_expandable {
        println!("\ndrill into linked context: mem expand <slug>");
    }

    // SERENDIPITY L1 (surfacing-stage, deterministic — no LLM, no resolver touch). Push at most
    // ONE cross-domain bridge, anchored to a top hit, reading the insight `mem dream` already
    // authored into the novelty ledger. Walk the top-3 hits in rank order; for the first anchor
    // with a current `analogous` edge to a still-current belief that HAS a ledger insight, print
    // the MOST surprising (lowest sim) such bridge. Gated by config + the per-call --no-bridges.
    let show_bridge = load_settings().recall.bridges && !no_bridges;
    if show_bridge {
        let insights = load_bridge_insights(&dir);
        if !insights.is_empty() {
            'anchors: for (anchor_id, anchor_slug, _claim, _score) in hits.iter().take(3) {
                let raw = adj.get(anchor_id).map(|v| v.as_slice()).unwrap_or(&[]);
                let mut best: Option<(&str, &str, f32)> = None; // (other_slug, insight, sim)
                for r in raw {
                    if r.kind != EdgeKind::Other("analogous".into()) {
                        continue;
                    }
                    let other = if &r.subject == anchor_id { &r.object } else { &r.subject };
                    if other == anchor_id {
                        continue;
                    }
                    let Some(other_b) = sg.by_id(other).filter(|b| b.relation.is_none()) else {
                        continue; // other endpoint must be a still-current content belief
                    };
                    let key = novelty_pair_key(anchor_id, other);
                    let Some((insight, sim)) = insights.get(&key) else { continue };
                    if best.map(|(_, _, s)| *sim < s).unwrap_or(true) {
                        best = Some((other_b.slug.as_str(), insight.as_str(), *sim));
                    }
                }
                if let Some((other_slug, insight, _sim)) = best {
                    println!("\n↯ bridge: [{anchor_slug}] ↔ [{other_slug}] — {insight}");
                    break 'anchors; // exactly ONE bridge total
                }
            }
        }
    }

    // FRONTIER-REVIEW NUDGE (surfacing-stage): if the cheap judge left candidate justifications the
    // tool can't safely adjudicate, tell the consuming agent — it's the MAXIMUM judge (see `mem review`).
    let n_review = depends_on_candidates(&sg, &defeated).len();
    if n_review > 0 {
        println!("\n⚑ {n_review} edge(s) may be justifications (depends_on) — run `mem review` to adjudicate.");
    }
}

/// `mem expand <slug|id>` — one hop. Shows a belief and its CURRENT linked neighbors, grouped by
/// relation kind + direction. Deterministic (no LLM); the robot drives the graph-walk explicitly,
/// so the whole exploration stays a replayable command trail (human-reproducible).
fn cmd_expand(args: &[String]) {
    let Some(reference) = args.first() else {
        eprintln!("expand needs a <slug|id>");
        std::process::exit(2);
    };
    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let Some(id) = sg.resolve_ref(reference) else {
        println!("No belief '{reference}' in scope ({}).", scopes.join(", "));
        return;
    };
    let target = sg.by_id(&id).unwrap();
    println!("[{}] {}\n", target.slug, target.claim);

    let defeated = sg.defeated();
    let adj = sg.adjacency(&defeated);
    let raw = adj.get(&id).map(|v| v.as_slice()).unwrap_or(&[]);
    let rels = collapse(&id, raw);
    let mut printed = 0;
    for r in &rels {
        let outgoing = r.subject == id;
        let other_id = if outgoing { &r.object } else { &r.subject };
        let Some(nb) = sg.by_id(other_id) else { continue };
        let label = if outgoing {
            format!("{} →", r.kind.as_str())
        } else {
            format!("← {}", r.kind.as_str())
        };
        let mark = if defeated.contains(&nb.id) { "  [superseded]" } else { "" };
        println!("  {label:>14}  [{}] {}{}", nb.slug, nb.claim, mark);
        // Surface the reified edge's rationale/carry-over note (Linker authored it). This is where
        // a defeating edge carries the displaced point, so an agent sees WHY without re-expanding
        // the loser. The edge-belief id is deterministic from (kind, subject, object).
        let edge_id = content_id(&format!("{}|{}|{}", r.kind.as_str(), r.subject, r.object));
        if let Some(note) = sg.by_id(&edge_id).map(|e| e.body.trim()).filter(|s| !s.is_empty()) {
            for (i, ln) in note.lines().enumerate() {
                println!("                  {} {ln}", if i == 0 { "↳" } else { " " });
            }
        }
        printed += 1;
    }
    if printed == 0 {
        println!("(no linked context)");
    }
}

/// The grounded-synthesis contract for `mem ask`. Faithfulness over fluency — a robot ACTS on
/// this, so no invention, conflicts surfaced (not smoothed), gaps stated, beliefs cited.
const ASK_SYSTEM: &str = "You answer a question using ONLY the supplied beliefs. Never invent \
facts that aren't in them. Cite the [slug] of every belief you draw on. If the beliefs conflict, \
surface the conflict — do not silently pick a side. If they don't cover the question, say what's \
missing in `gaps`. Be concise. Reply JSON: \
{\"answer\": \"...\", \"cited\": [\"slug\"], \"conflicts\": [\"...\"], \"gaps\": \"...\"}";

/// `mem ask "<question>"` — the OPT-IN LLM reducer. Gathers the frontier-resolved, scope-filtered
/// top-k beliefs and has qwen synthesize a grounded answer (cited, conflict-honest, gap-explicit).
/// The ONE place an LLM sits on the read path; `recall` stays deterministic. Degrades to raw
/// beliefs if Ollama is down — the robot can read them itself.
fn cmd_ask(args: &[String]) {
    let mut question = String::new();
    let mut limit = 8usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => { limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(8); i += 1; }
            s if question.is_empty() => question = s.to_string(),
            s => { question.push(' '); question.push_str(s); }
        }
        i += 1;
    }
    let question = question.trim().to_string();
    if question.is_empty() {
        eprintln!("ask needs a question");
        std::process::exit(2);
    }

    let dir = store_dir();
    worker::surface_last_run(&dir); // one-line "what the background did", at most once per run
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let defeated = sg.defeated();
    let current = sg.current_content(&defeated);
    if current.is_empty() {
        println!("No memory in scope ({}).", scopes.join(", "));
        return;
    }

    let mut hits = match rank(&dir, &current, &question) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("(no embeddings: {e}; falling back to lexical match)");
            lexical(&current, &question)
        }
    };
    hits.truncate(limit);
    if hits.is_empty() {
        println!("No memory on \"{question}\".");
        return;
    }

    // DETERMINISTIC CONFLICT PASS (V4/N3): the reducer is frontier-pre-filtered (we feed only
    // `current` beliefs), but V4 showed the LLM silently adopts a contradiction ~5/6 of the time
    // rather than surfacing it. So we detect live `attacks` BETWEEN the recalled beliefs ourselves —
    // genuine open conflicts (both endpoints current) — tell the model about them, and surface them
    // unconditionally afterward regardless of what the LLM volunteers. This is the conflict half of
    // the trusted-reducer gate; the frontier pre-filter is the other half.
    let detected_conflicts = live_conflicts_among_hits(&sg, &defeated, &hits);

    let mut ctx = format!("Question: {question}\n\nBeliefs:\n");
    for (_id, slug, claim, _s) in &hits {
        ctx.push_str(&format!("- [{slug}] {claim}\n"));
    }
    if !detected_conflicts.is_empty() {
        ctx.push_str(
            "\nKNOWN CONFLICTS — these recalled beliefs directly attack each other and are BOTH \
             current. Do NOT silently pick a side; report the disagreement in `conflicts`:\n",
        );
        for (a, b) in &detected_conflicts {
            ctx.push_str(&format!("- [{a}] vs [{b}]\n"));
        }
    }

    let model = std::env::var("ASK_MODEL")
        .or_else(|_| std::env::var("JUDGE_MODEL"))
        .unwrap_or_else(|_| "qwen2.5:7b".into());
    let oll = memory_embed::Ollama::from_env();
    match memory_embed::chat_json(&oll.url, &model, ASK_SYSTEM, &ctx) {
        Ok(v) => {
            let answer = v.get("answer").and_then(|x| x.as_str()).unwrap_or("(no answer)");
            println!("{answer}\n");
            if let Some(cited) = v.get("cited").and_then(|x| x.as_array()) {
                let slugs: Vec<String> =
                    cited.iter().filter_map(|c| c.as_str()).map(|s| format!("[{}]", s.trim_matches(|c| c == '[' || c == ']'))).collect();
                if !slugs.is_empty() {
                    println!("cited: {}", slugs.join(" "));
                }
            }
            if let Some(conf) = v.get("conflicts").and_then(|x| x.as_array()) {
                for c in conf.iter().filter_map(|c| c.as_str()).filter(|s| !s.trim().is_empty()) {
                    println!("⚠ conflict: {c}");
                }
            }
            let gaps = v.get("gaps").and_then(|x| x.as_str()).unwrap_or("").trim();
            if !gaps.is_empty() && gaps.to_lowercase() != "none" {
                println!("gaps: {gaps}");
            }
        }
        Err(e) => {
            eprintln!("(synthesis unavailable: {e}; showing the grounded beliefs)");
            for (_id, slug, claim, _s) in &hits {
                println!("• [{slug}] {claim}");
            }
        }
    }

    // Surface the edge-grounded conflicts unconditionally (after synthesis, even if Ollama was
    // down): these are ground truth from the graph, not the LLM's discretion. The model's free-text
    // `conflicts` above can ADD semantic conflicts the edges don't encode; these are the floor.
    for (a, b) in &detected_conflicts {
        println!("⚠ conflict (live attack): [{a}] vs [{b}] — both current, surfaced not resolved");
    }
}

/// Detect genuine open conflicts among the recalled hits: unordered pairs `(slugA, slugB)` joined by
/// a CURRENT `attacks` edge where BOTH endpoints are in the hit set. Deterministic, dedup'd. This is
/// the conflict half of the trusted-reducer gate (V4/N3) — the reducer can't be relied on to
/// volunteer a contradiction, so the graph surfaces it.
fn live_conflicts_among_hits(g: &Graph, defeated: &std::collections::HashSet<String>, hits: &[Hit]) -> Vec<(String, String)> {
    let hit_ids: std::collections::HashSet<&str> = hits.iter().map(|(id, _, _, _)| id.as_str()).collect();
    let adj = g.adjacency(defeated);
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for (id, _slug, _claim, _score) in hits {
        for r in adj.get(id).map(|v| v.as_slice()).unwrap_or(&[]) {
            if r.kind != EdgeKind::Attacks {
                continue;
            }
            let other = if &r.subject == id { &r.object } else { &r.subject };
            if other == id || !hit_ids.contains(other.as_str()) {
                continue;
            }
            let (lo, hi) = if id.as_str() < other.as_str() {
                (id.clone(), other.clone())
            } else {
                (other.clone(), id.clone())
            };
            if seen.insert((lo.clone(), hi.clone())) {
                let sa = g.by_id(&lo).map(|x| x.slug.clone()).unwrap_or(lo);
                let sb = g.by_id(&hi).map(|x| x.slug.clone()).unwrap_or(hi);
                out.push((sa, sb));
            }
        }
    }
    out
}

/// Machine-inferred edge authors whose edges are a REGENERABLE layer (redrawn by an unattended
/// `mem consolidate`). Hand-vetoing these is fragile under a from-scratch re-link, so `forget`
/// refuses them; human-authored edges (everything else) are safe to forget.
fn is_regenerable_edge(author_id: &str) -> bool {
    matches!(author_id, "judge@1" | "proximity@1")
}

/// `mem forget <slug|id> [--reason R]` — retract a belief so recall stops surfacing it.
///
/// Append-only by construction: we NEVER delete the file. We commit a reified `retracts`
/// edge-belief (a defeating kind, self-anchored: subject == object == target) through the
/// Consolidator (the sole edge writer). Frontier resolution then drops the target from the
/// current set — recall/ask no longer surface it — while the belief stays on disk for reliving,
/// and the retraction is itself defeasible (a later supersedes-of-the-retraction reinstates it).
/// Written in the target's OWN scope, so a branch-local forget stays branch-local.
fn cmd_forget(args: &[String]) {
    let mut reference = String::new();
    let mut reason = String::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--reason" => { reason = args.get(i + 1).cloned().unwrap_or_default(); i += 1; }
            s if reference.is_empty() => reference = s.to_string(),
            _ => {}
        }
        i += 1;
    }
    if reference.trim().is_empty() {
        eprintln!("forget needs a <slug|id>");
        std::process::exit(2);
    }

    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let Some(id) = sg.resolve_ref(reference.trim()) else {
        println!("No belief '{}' in scope ({}).", reference.trim(), scopes.join(", "));
        return;
    };
    let target = sg.by_id(&id).unwrap();

    // Forgetting an edge-belief is the clean UNDO path (forget a retraction = un-forget; forget a
    // supersedes = un-supersede). But machine-inferred edges (judge/proximity) are a REGENERABLE
    // layer — hand-vetoing one is fragile (a from-scratch re-link would redraw it), so refuse those
    // and point at the cause. Human-authored edges (forget-retractions, --supersedes hints,
    // promotions) are never redrawn unattended, so forgetting them is safe.
    if target.relation.is_some() && is_regenerable_edge(&target.author_id) {
        eprintln!(
            "'{}' is a machine-inferred edge ({}) — a regenerable layer. Fix the cause (re-link or the content), don't hand-veto it.",
            target.slug, target.author_id
        );
        std::process::exit(2);
    }
    let slug = target.slug.clone();
    let scope = belief_scope(target).to_string();

    // Already off the frontier? Report instead of stacking a redundant retraction.
    if sg.defeated().contains(&id) {
        println!("[{slug}] is already retracted (not currently surfaced).");
        return;
    }

    let proposal = LinkProposal {
        kind: EdgeKind::Retracts,
        subject: id.clone(),
        object: id.clone(),
        confidence: Confidence::Strong,
        rationale: if reason.trim().is_empty() {
            "mem forget: user retraction (kept on disk, dropped from the frontier)".into()
        } else {
            format!("mem forget: {}", reason.trim())
        },
        linker: "mem-forget@1".into(),
    };
    match Consolidator::commit(&dir, std::slice::from_ref(&proposal), &scope) {
        0 => println!("[{slug}] was already forgotten (retraction exists)."),
        _ => println!("forgot [{slug}] in scope={scope} — recall will no longer surface it (file kept for reliving)"),
    }
}

/// `mem promote [<branch>] [--dry-run]` — lift a feature branch's beliefs into repo canon.
///
/// Run client-side AFTER the branch merges (it reads LOCAL git state — the reason `mem` is a CLI,
/// not a server). Policy (decided earlier, recalled from the store):
///  - NON-DESTRUCTIVE: the original `@branch` beliefs are left on disk untouched, so the branch
///    world stays relivable. Promotion writes a COPY into canon, it does not move/relabel.
///  - Each branch content-belief gets a canon copy with a fresh, DETERMINISTIC id
///    (`content_id("promote|<orig>|<canon-scope>")`) so re-running is idempotent, and a
///    `derived_from` pointer back to the branch original (provenance / reliving).
///  - EDGES COME ALONG: each branch edge-belief is re-emitted into canon through the Consolidator
///    (the sole edge writer) with its endpoints REMAPPED to the promoted copies; an endpoint
///    already in canon is kept; an unresolvable endpoint drops that edge (a canon edge must point
///    at canon beliefs).
///  - CONFLICTS: naive-merge-now, reconcile-later. No conflict gating here — copies land in canon
///    and frontier resolution + a later `mem consolidate`/adjudication settle any clash.
///  - The duplicate claim (branch original + canon copy) is expected to be collapsed by the
///    reduction/caching layer; on `main` only the canon copy is in scope anyway.
fn cmd_promote(args: &[String]) {
    let mut branch_arg: Option<String> = None;
    let mut dry_run = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => dry_run = true,
            s if branch_arg.is_none() && !s.starts_with("--") => branch_arg = Some(s.to_string()),
            _ => {}
        }
        i += 1;
    }

    let Some(id) = repo_id() else {
        eprintln!("not in a git repo — nothing to promote");
        std::process::exit(2);
    };
    let branch = match branch_arg.or_else(current_branch) {
        Some(b) => b,
        None => { eprintln!("could not determine a branch to promote"); std::process::exit(2); }
    };
    if is_default_branch(&branch) {
        eprintln!("'{branch}' is the default branch — it IS repo canon; nothing to promote");
        std::process::exit(2);
    }
    let from_scope = format!("repo:{id}@{branch}");
    let canon_scope = format!("repo:{id}");

    let dir = store_dir();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };

    let branch_beliefs: Vec<&Belief> =
        g.iter().filter(|b| belief_scope(b) == from_scope).collect();
    if branch_beliefs.is_empty() {
        println!("No branch-local beliefs in scope ({from_scope}) — nothing to promote.");
        return;
    }

    // Deterministic id remap: branch-original id → promoted-canon id (idempotent on re-run).
    let content: Vec<&Belief> =
        branch_beliefs.iter().copied().filter(|b| b.relation.is_none()).collect();
    let mut remap: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for b in &content {
        remap.insert(b.id.clone(), content_id(&format!("promote|{}|{canon_scope}", b.id)));
    }
    let existing: std::collections::HashSet<&str> =
        g.iter().map(|b| b.id.as_str()).collect();

    let mut promoted_content = 0usize;
    for b in &content {
        let new_id = remap[&b.id].clone();
        if existing.contains(new_id.as_str()) {
            continue; // already promoted — idempotent
        }
        if dry_run {
            println!("would promote [{}] {} → {canon_scope}", b.slug, truncate(&b.claim, 70));
            promoted_content += 1;
            continue;
        }
        let txn = iso_now();
        let body = format!("(promoted from {from_scope} — original belief {})", b.id);
        let md = promoted_belief_md(&new_id, &b.slug, &canon_scope, &b.claim, &body, &txn, &b.id);
        if let Err(e) = std::fs::write(dir.join(format!("{new_id}.md")), md) {
            eprintln!("failed to write promoted belief: {e}");
            std::process::exit(1);
        }
        promoted_content += 1;
    }

    // Carry the edges: remap endpoints into canon, re-emit through the Consolidator.
    let resolve = |endpoint: &str| -> Option<String> {
        if let Some(p) = remap.get(endpoint) {
            Some(p.clone())
        } else if g.by_id(endpoint).map(|x| belief_scope(x) == canon_scope).unwrap_or(false) {
            Some(endpoint.to_string()) // already canon — point straight at it
        } else {
            None
        }
    };
    let mut proposals: Vec<LinkProposal> = Vec::new();
    let mut dropped_edges = 0usize;
    for e in branch_beliefs.iter().copied().filter(|b| b.relation.is_some()) {
        let r = e.relation.as_ref().unwrap();
        let (Some(subject), Some(object)) = (resolve(&r.subject), resolve(&r.object)) else {
            dropped_edges += 1;
            continue;
        };
        proposals.push(LinkProposal {
            kind: r.kind.clone(),
            subject,
            object,
            confidence: Confidence::Strong,
            rationale: format!("mem promote: carried from {from_scope}"),
            linker: "mem-promote@1".into(),
        });
    }

    if dry_run {
        println!(
            "dry-run: {promoted_content} belief(s) + {} edge(s) would promote into {canon_scope} ({dropped_edges} unresolvable edge(s) dropped)",
            proposals.len()
        );
        return;
    }
    let edges_drawn = Consolidator::commit(&dir, &proposals, &canon_scope);
    println!(
        "promoted {promoted_content} belief(s) and {edges_drawn} edge(s) from {from_scope} into {canon_scope} \
         (originals kept; {dropped_edges} edge(s) dropped as unresolvable). Conflicts settle at recall — run `mem consolidate` to adjudicate."
    );
    worker::note_writes_and_kick(&dir, promoted_content);
}

/// Like `belief_md`, but stamps `derived_from: [<origin>]` so a promoted canon belief points back
/// at its branch original (provenance / reliving). Format otherwise matches `remember`.
fn promoted_belief_md(id: &str, slug: &str, scope: &str, claim: &str, body: &str, txn: &str, origin: &str) -> String {
    let base = belief_md(id, slug, scope, claim, &[], body, txn);
    base.replace("  derived_from: []\n", &format!("  derived_from:\n    - {origin}\n"))
}

fn truncate(s: &str, n: usize) -> String {
    let s = s.replace('\n', " ");
    if s.chars().count() <= n { s } else { format!("{}…", s.chars().take(n).collect::<String>()) }
}

/// Surfacing-stage collapse: drop a generic `relates-to`/`analogous` edge from `anchor` to a
/// neighbor when a SPECIFIC edge already connects them. Order-preserving (the adjacency is
/// slug-sorted), so the output stays deterministic / human-reproducible.
fn collapse(anchor: &str, rels: &[Relation]) -> Vec<Relation> {
    let neighbor = |r: &Relation| if r.subject == anchor { r.object.clone() } else { r.subject.clone() };
    let specific: std::collections::HashSet<String> =
        rels.iter().filter(|r| !r.kind.is_generic()).map(neighbor).collect();
    rels.iter()
        .filter(|r| !(r.kind.is_generic() && specific.contains(&neighbor(r))))
        .cloned()
        .collect()
}

/// Surfacing-stage duplicate fold map: member-id → representative-id, built from CURRENT
/// (undefeated) `same-as` edges (member→rep). Transitive with a hop guard, so a chain
/// m → x → rep resolves all the way to the final representative. Truth/frontier are untouched —
/// this only governs which of a duplicate cluster takes the one display slot at recall. Returns
/// (map, absorbed_counts) where absorbed_counts[rep] = how many members fold behind it.
fn collapse_map(
    g: &Graph,
    defeated: &std::collections::HashSet<String>,
) -> (HashMap<String, String>, HashMap<String, usize>) {
    // direct member → rep links from in-force `same-as` edges
    let mut direct: HashMap<String, String> = HashMap::new();
    for (b, r) in g.relations() {
        if defeated.contains(&b.id) || !r.kind.is_collapsing() {
            continue; // only CURRENT collapse edges fold
        }
        if r.subject == r.object {
            continue; // self-fold is meaningless
        }
        direct.insert(r.subject.clone(), r.object.clone()); // member → rep
    }
    // resolve each member to its FINAL rep (follow the chain, guarding against cycles / runaway).
    let mut map: HashMap<String, String> = HashMap::new();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for member in direct.keys() {
        let mut cur = member.clone();
        for _ in 0..(direct.len() + 1) {
            match direct.get(&cur) {
                Some(next) if next != &cur => cur = next.clone(),
                _ => break,
            }
        }
        if &cur != member {
            map.insert(member.clone(), cur.clone());
            *counts.entry(cur).or_default() += 1;
        }
    }
    (map, counts)
}

/// Compact affordance string for a belief's edges, e.g. "refines 1 · supports 2", plus a
/// `contested` flag if any `attacks` touches it.
fn affordance_str(edges: &[Relation]) -> (String, bool) {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut contested = false;
    for r in edges {
        let k = r.kind.as_str();
        if k == "attacks" {
            contested = true;
        }
        *counts.entry(k).or_default() += 1;
    }
    let parts: Vec<String> = counts.into_iter().map(|(k, n)| format!("{k} {n}")).collect();
    (parts.join(" · "), contested)
}

/// SURFACING-STAGE confidence nudge for the cosine score (V1). `w` is a belief's structural
/// confidence weight in [0,1] (`StructuralConfidence::weight` — directness × source_weight ×
/// corroboration × recency). Returns a multiplicative factor in [1.0, 1.0+CONFIDENCE_BOOST_CAP],
/// linear in `w`: a higher-confidence belief floats up on a near-tie but, capped at ≤12%, never lets
/// a far belief leapfrog a clearly-nearer one — semantic similarity stays dominant. Replaces the old
/// supports-only boost (V1: recency + directness are load-bearing, not just corroboration count).
fn confidence_boost(w: f32) -> f32 {
    const CONFIDENCE_BOOST_CAP: f32 = 0.12; // ≤12% — a gentle tie-breaker, not a re-ranking
    1.0 + CONFIDENCE_BOOST_CAP * w.clamp(0.0, 1.0)
}

/// Name what a hit conflicts with: the slug(s) on the other endpoint of any current `attacks` edge
/// touching it. Deterministic. Returns e.g. "contested by [other-slug]" (or "" if uncontested).
fn contest_str(anchor: &str, edges: &[Relation], slug_of: &dyn Fn(&str) -> Option<String>) -> String {
    let mut others: Vec<String> = Vec::new();
    for r in edges {
        if r.kind != EdgeKind::Attacks {
            continue;
        }
        let other = if r.subject == anchor { &r.object } else { &r.subject };
        if other == anchor {
            continue; // self-anchored, nothing to name
        }
        if let Some(s) = slug_of(other) {
            if !others.contains(&s) {
                others.push(s);
            }
        }
    }
    if others.is_empty() {
        return String::new();
    }
    let named: Vec<String> = others.iter().map(|s| format!("[{s}]")).collect();
    format!("contested by {}", named.join(", "))
}

// --- consolidation (on write) ----------------------------------------------------------

fn consolidate(dir: &PathBuf, new_id: &str, scope: &str, hints: &[Hint]) -> usize {
    let g = match Graph::load_dir(dir) {
        Ok(g) => g,
        Err(_) => return 0,
    };
    let sg = scoped_graph(&g, &active_scopes());
    let Some(new) = sg.by_id(new_id) else {
        return 0;
    };
    let oll = memory_embed::Ollama::from_env();
    let mut vectors = memory_embed::load_cache(dir, &oll.model);
    if !vectors.contains_key(new_id) {
        if let Ok(vs) = oll.embed(&[format!("search_document: {}", new.claim)]) {
            if let Some(v) = vs.into_iter().next() {
                vectors.insert(new_id.to_string(), v);
                memory_embed::save_cache(dir, &oll.model, &vectors);
            }
        }
    }
    let ctx = LinkCtx { new, graph: &sg, vectors: &vectors, hints };
    // write-time = cheap only (explicit-supersede hint). The LLM judge + proximity run in the
    // deliberate `mem consolidate` pass, off the write hot path.
    let proposals = Orchestrator::with_defaults().run(&ctx, &[Cadence::OnWrite]);
    Consolidator::commit(dir, &proposals, scope)
}

/// `mem consolidate [--limit N]` — the REM pass: embed in-scope beliefs, run proximity + the
/// LLM judge over the most recent N, and commit the edges they propose.
fn cmd_consolidate(args: &[String]) {
    let mut limit = 12usize;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--limit" {
            limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(12);
            i += 1;
        }
        i += 1;
    }
    let dir = store_dir();
    let scopes = active_scopes();
    match consolidate_pass(&dir, &scopes, limit) {
        Err(e) => eprintln!("{e}; can't run candidate generation"),
        // 0 targets with a real limit = empty scope; `--limit 0` falls through to the normal
        // report (0 consolidated + the review nudge), as before the pass extraction.
        Ok((0, _, _)) if limit > 0 => println!("Nothing to consolidate in scope ({}).", scopes.join(", ")),
        Ok((n, total, n_review)) => {
            println!("consolidated {n} belief(s); drew {total} new edge(s)");
            // Frontier-review nudge: the judge emits supports/refines but can't safely tell a
            // justification (depends_on) from corroboration — hand candidates to the frontier agent.
            if n_review > 0 {
                println!("⚑ {n_review} edge(s) flagged for frontier review — run `mem review` to adjudicate (candidate depends_on).");
            }
        }
    }
}

/// The consolidation pass shared by `mem consolidate` and the background worker (`mem __worker`):
/// ensure embeddings, run proximity + the LLM judge over the most recent `limit` in-scope beliefs,
/// commit what they propose. `Ok((targets, new_edges, review_candidates))`; `Err` only on a hard
/// failure (the embedding backend is down). Progress goes to stderr — the worker's log.
fn consolidate_pass(dir: &PathBuf, scopes: &[String], limit: usize) -> Result<(usize, usize, usize), String> {
    let g = match Graph::load_dir(dir) {
        Ok(g) => g,
        Err(_) => return Ok((0, 0, 0)),
    };
    let sg = scoped_graph(&g, scopes);
    let content: Vec<&Belief> = sg.content().collect();
    if content.is_empty() {
        return Ok((0, 0, 0));
    }

    // make sure every in-scope content belief has an embedding (candidate generation needs it)
    let oll = memory_embed::Ollama::from_env();
    let mut vectors = memory_embed::load_cache(dir, &oll.model);
    let need: Vec<&Belief> = content.iter().copied().filter(|b| !vectors.contains_key(&b.id)).collect();
    if !need.is_empty() {
        let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
        match oll.embed(&docs) {
            Ok(vs) => {
                for (b, v) in need.iter().zip(vs) {
                    vectors.insert(b.id.clone(), v);
                }
                memory_embed::save_cache(dir, &oll.model, &vectors);
            }
            Err(e) => return Err(format!("embedding failed ({e})")),
        }
    }

    let judge = std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
    // newest-first: consolidate the most recently remembered beliefs (they're the unlinked ones)
    let mut targets: Vec<&Belief> = content.iter().copied().collect();
    targets.sort_by(|a, b| b.txn_time.cmp(&a.txn_time));
    targets.truncate(limit);
    eprintln!("consolidating {} belief(s) with judge={judge} ...", targets.len());
    let orch = Orchestrator::deep();
    let mut total = 0;
    for t in &targets {
        let ctx = LinkCtx { new: t, graph: &sg, vectors: &vectors, hints: &[] };
        let props = orch.run(&ctx, &[Cadence::Nrem, Cadence::Rem]);
        total += Consolidator::commit(dir, &props, belief_scope(t));
    }
    // Reload to count depends_on candidates (the edges just drawn may have created some).
    let n_review = match Graph::load_dir(dir) {
        Ok(g2) => {
            let sg2 = scoped_graph(&g2, scopes);
            depends_on_candidates(&sg2, &sg2.defeated()).len()
        }
        Err(_) => 0,
    };
    Ok((targets.len(), total, n_review))
}

/// `mem reduce [--dry-run]` — the deterministic duplicate-collapse pass. A SURFACING-stage HIDE,
/// NOT a frontier defeat: a duplicate isn't false, just redundant, so the `same-as` edges it
/// commits fold the DISPLAY at recall while every member stays current/durable/relivable. Reuses
/// the Linker's existing edges (re-gated on embedding similarity) rather than re-clustering; each
/// fold is committed in the MEMBER's own scope (like forget/promote) so a branch-local duplicate
/// stays branch-local.
fn cmd_reduce(args: &[String]) {
    let mut dry_run = false;
    for a in args {
        if a == "--dry-run" {
            dry_run = true;
        }
    }
    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let current = sg.current_content(&sg.defeated());
    if current.len() < 2 {
        println!("Need at least 2 current beliefs in scope to reduce.");
        return;
    }

    // Ensure every current content belief has an embedding to back the re-gate `sim` closure
    // (the byte-identical-claim signal needs none, but the edge-reuse signal does).
    let oll = memory_embed::Ollama::from_env();
    let mut vectors = memory_embed::load_cache(&dir, &oll.model);
    let need: Vec<&Belief> = current.iter().copied().filter(|b| !vectors.contains_key(&b.id)).collect();
    if !need.is_empty() {
        let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
        match oll.embed(&docs) {
            Ok(vs) => {
                for (b, v) in need.iter().zip(vs) {
                    vectors.insert(b.id.clone(), v);
                }
                memory_embed::save_cache(&dir, &oll.model, &vectors);
            }
            // Degrade gracefully: without embeddings the edge-reuse signal can't re-gate, but the
            // byte-identical-claim signal still collapses exact duplicates.
            Err(e) => eprintln!("(no embeddings: {e}; reducing on identical claims only)"),
        }
    }
    let sim = |a: &str, b: &str| -> Option<f32> {
        match (vectors.get(a), vectors.get(b)) {
            (Some(va), Some(vb)) => Some(cosine(va, vb)),
            _ => None,
        }
    };

    let proposals = Reducer::default().reduce(&sg, &sim);
    if proposals.is_empty() {
        println!("No duplicates to collapse in scope ({}).", scopes.join(", "));
        return;
    }
    let slug_of = |id: &str| sg.by_id(id).map(|b| b.slug.clone());

    if dry_run {
        for p in &proposals {
            let m = slug_of(&p.subject).unwrap_or_else(|| p.subject.clone());
            let r = slug_of(&p.object).unwrap_or_else(|| p.object.clone());
            println!("would collapse [{m}] → [{r}]");
        }
        println!("dry-run: {} fold(s) would collapse into their representatives.", proposals.len());
        return;
    }

    // Commit each fold in the MEMBER's (subject's) own scope, like forget/promote.
    let mut folded = 0usize;
    for p in &proposals {
        let scope = sg
            .by_id(&p.subject)
            .map(|b| belief_scope(b).to_string())
            .unwrap_or_else(|| "global".into());
        folded += Consolidator::commit(&dir, std::slice::from_ref(p), &scope);
    }
    println!(
        "collapsed {folded} duplicate(s) into their representative(s) — recall now folds them (files kept; frontier untouched)."
    );
}

/// `mem dream [--limit N]` — the REM/novelty pass (an observability artifact, separate from
/// `mem consolidate`). Probes the most-unrelated pairs for non-obvious bridges, caches every
/// probe — including non-results — in a JSONL ledger so budget isn't re-burned, commits the rare
/// bridges as non-defeating `analogous` edges, and reports the BRIDGE RATE.
fn cmd_dream(args: &[String]) {
    let mut limit = 8usize;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--limit" {
            limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(8);
            i += 1;
        }
        i += 1;
    }
    let dir = store_dir();
    let scopes = active_scopes();
    match dream_pass(&dir, &scopes, limit) {
        Err(e) => eprintln!("{e}; can't dream"),
        // 0 targets with a real limit = too few beliefs; `--limit 0` still reports the rate.
        Ok(o) if o.targets == 0 && limit > 0 => println!("Need at least 2 beliefs in scope to dream."),
        Ok(o) => {
            let rate = if o.attempts > 0 { o.bridges as f32 / o.attempts as f32 * 100.0 } else { 0.0 };
            println!("bridge rate: {rate:.1}% ({}/{} far pairs ever bridged)", o.bridges, o.attempts);
            if o.drawn > 0 {
                println!("\n{} new cross-domain bridge(s):", o.drawn);
                for p in &o.props {
                    println!("  • {}", p.rationale);
                }
            } else {
                println!("no new bridges this pass ({} probe(s) recorded as earned-unrelated).", o.recorded);
            }
        }
    }
}

/// What one dream pass did — the CLI prints the full report, the worker a one-line summary.
struct DreamOutcome {
    targets: usize,
    drawn: usize,
    recorded: usize,
    attempts: u64,
    bridges: u64,
    props: Vec<LinkProposal>,
}

/// The REM/novelty pass shared by `mem dream` and the background worker: probe the most-unrelated
/// pairs for bridges, persist every probe to the ledger, commit the rare bridges. `targets == 0`
/// means the scope held fewer than 2 beliefs OR the caller passed `limit == 0` (callers that care
/// distinguish on their own limit); `Err` only when the embedding backend is down.
fn dream_pass(dir: &PathBuf, scopes: &[String], limit: usize) -> Result<DreamOutcome, String> {
    let none = |targets| DreamOutcome { targets, drawn: 0, recorded: 0, attempts: 0, bridges: 0, props: Vec::new() };
    let g = match Graph::load_dir(dir) {
        Ok(g) => g,
        Err(_) => return Ok(none(0)),
    };
    let sg = scoped_graph(&g, scopes);
    let content: Vec<&Belief> = sg.content().collect();
    if content.len() < 2 {
        return Ok(none(0));
    }

    // novelty needs an embedding for every in-scope belief (it's distance-driven).
    let oll = memory_embed::Ollama::from_env();
    let mut vectors = memory_embed::load_cache(dir, &oll.model);
    let need: Vec<&Belief> = content.iter().copied().filter(|b| !vectors.contains_key(&b.id)).collect();
    if !need.is_empty() {
        let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
        match oll.embed(&docs) {
            Ok(vs) => {
                for (b, v) in need.iter().zip(vs) {
                    vectors.insert(b.id.clone(), v);
                }
                memory_embed::save_cache(dir, &oll.model, &vectors);
            }
            Err(e) => return Err(format!("embedding failed ({e})")),
        }
    }

    let (probed, attempts0, bridges0) = load_probes(dir);
    // spread coverage: oldest beliefs first (newest get linked by NREM/REM consolidate already).
    let mut targets: Vec<&Belief> = content.clone();
    targets.sort_by(|a, b| a.txn_time.cmp(&b.txn_time));
    targets.truncate(limit);

    eprintln!("dreaming over {} belief(s) ...", targets.len());
    let dreamer = NoveltyDreamer::from_env();
    let (props, records) = dreamer.dream(&targets, &sg, &vectors, &probed);
    append_probes(dir, &records);
    let attempts = attempts0 + records.len() as u64;
    let bridges = bridges0 + records.iter().filter(|r| r.bridged).count() as u64;
    let drawn = Consolidator::commit(dir, &props, &write_scope(false));
    Ok(DreamOutcome { targets: targets.len(), drawn, recorded: records.len(), attempts, bridges, props })
}

/// A candidate justification the cheap judge couldn't safely adjudicate: an in-force
/// `supports`/`refines` edge from a DERIVATION (subject directness inferred/reduced) to an older
/// belief — it MIGHT be a true JTMS `depends_on`, but only a frontier agent should make that call
/// (V5: a false depends_on wrongly retracts; qwen-7b won't author it). Surfaced by `mem review`.
struct ReviewCandidate {
    subject: String,
    object: String,
    kind: String,
}

/// Derived (not stored): the depends_on candidates on the current frontier. A pair already carrying
/// an in-force `depends_on` is skipped (already adjudicated). "the layout is never the storage."
fn depends_on_candidates(g: &Graph, defeated: &std::collections::HashSet<String>) -> Vec<ReviewCandidate> {
    use std::collections::HashSet;
    // pairs already adjudicated as depends_on → skip
    let mut adjudicated: HashSet<(String, String)> = HashSet::new();
    for (b, r) in g.relations() {
        if !defeated.contains(&b.id) && matches!(r.kind, EdgeKind::DependsOn) {
            adjudicated.insert((r.subject.clone(), r.object.clone()));
        }
    }
    let mut out = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    for (b, r) in g.relations() {
        if defeated.contains(&b.id) {
            continue; // edge must be in force
        }
        if !matches!(r.kind, EdgeKind::Supports | EdgeKind::Refines) {
            continue;
        }
        // subject = the dependent; it must be a current DERIVATION (inferred/reduced), not a
        // directly-observed `stated` fact (those are independently grounded — V5).
        let Some(subj) = g.by_id(&r.subject) else { continue };
        if defeated.contains(&subj.id) || !matches!(subj.directness.as_str(), "inferred" | "reduced") {
            continue;
        }
        if g.by_id(&r.object).map_or(true, |o| defeated.contains(&o.id)) {
            continue; // object (the ground) must be current
        }
        let key = (r.subject.clone(), r.object.clone());
        if adjudicated.contains(&key) || !seen.insert(key) {
            continue;
        }
        out.push(ReviewCandidate { subject: r.subject.clone(), object: r.object.clone(), kind: r.kind.as_str().to_string() });
    }
    out
}

/// `mem review [--limit N]` — surface edges flagged for FRONTIER adjudication. The cheap local judge
/// (qwen) emits `supports`/`refines`; it cannot reliably tell a JTMS justification (`depends_on`) from
/// corroboration (measured). So this lists the candidate justifications and hands them to the
/// consumer of the tool — a Claude Code session is the MAXIMUM judge, adjudicating on demand and
/// authoring a durable edge with `mem link`. Pure derived view; nothing is written.
fn cmd_review(args: &[String]) {
    let mut limit = 20usize;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--limit" {
            limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(20);
            i += 1;
        }
        i += 1;
    }
    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let defeated = sg.defeated();
    let mut cands = depends_on_candidates(&sg, &defeated);
    if cands.is_empty() {
        println!("Nothing flagged for review in scope ({}).", scopes.join(", "));
        return;
    }
    let total = cands.len();
    cands.truncate(limit);
    // Optional local-model annotation (safe — never commits): if DEPENDS_MODEL is set, ask it whether
    // each candidate is a true dependency so the frontier agent can prioritize. A high-recall model
    // (gemma2:9b) is the right tool here BECAUSE its false positives are just review items a human
    // rejects — the opposite of using it to auto-author edges (measured unsafe: 1/4 specificity).
    let depends_model = std::env::var("DEPENDS_MODEL").ok().filter(|s| !s.trim().is_empty());
    let oll = memory_embed::Ollama::from_env();
    if depends_model.is_some() {
        eprintln!("annotating {} candidate(s) with {} ...", cands.len(), depends_model.as_deref().unwrap());
    }
    println!(
        "{total} edge(s) flagged for frontier review — candidate justifications (is this a true depends_on?):\n"
    );
    for c in &cands {
        let (subj, obj) = (sg.by_id(&c.subject), sg.by_id(&c.object));
        let (ss, sc, sd) = subj.map(|b| (b.slug.as_str(), b.claim.as_str(), b.directness.as_str())).unwrap_or(("?", "?", "?"));
        let (os, oc) = obj.map(|b| (b.slug.as_str(), b.claim.as_str())).unwrap_or(("?", "?"));
        let hint = match &depends_model {
            Some(m) => match memory_consolidate::model_thinks_depends(&oll.url, m, sc, oc) {
                Some(true) => "   🤖 model: LIKELY depends — prioritize",
                Some(false) => "   🤖 model: probably corroboration",
                None => "",
            },
            None => "",
        };
        println!("• [{ss}] --{}--> [{os}]{hint}", c.kind);
        println!("    derivation (d={sd}): {}", truncate(sc, 100));
        println!("    ground:           {}", truncate(oc, 100));
        println!("    if [{ss}] holds ONLY because [{os}] is true (it collapses if [{os}] is withdrawn), author it:");
        println!("      mem link {ss} depends_on {os} --rationale \"<why it depends>\"");
        println!();
    }
    if total > cands.len() {
        println!("… {} more (raise --limit).", total - cands.len());
    }
    println!(
        "Adjudicate as a frontier agent: confirm a true JTMS dependency with `mem link … depends_on …`; \
         leave plain corroboration as-is. Authored edges are DURABLE (a re-link never overwrites them)."
    );
}

/// `mem link <subject-ref> <kind> <object-ref> [--rationale R]` — author a DURABLE edge by hand
/// (a frontier agent or human adjudicating). Routed through the Consolidator (the sole edge writer),
/// authored as `frontier@1` so it is NOT a regenerable machine edge — an unattended re-link links
/// AROUND it and never overwrites it (human/frontier edges are anchors, §edge-assignment). The
/// primary use is closing a `mem review` candidate into a real `depends_on`.
fn cmd_link(args: &[String]) {
    let mut rationale = String::new();
    let mut positionals: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--rationale" => { rationale = args.get(i + 1).cloned().unwrap_or_default(); i += 1; }
            s if !s.starts_with("--") => positionals.push(s.to_string()),
            _ => {}
        }
        i += 1;
    }
    if positionals.len() < 3 {
        eprintln!("usage: mem link <subject> <kind> <object> [--rationale R]   (kind e.g. depends_on)");
        std::process::exit(2);
    }
    let (subject_ref, kind_str, object_ref) =
        (positionals[0].clone(), positionals[1].clone(), positionals[2].clone());

    let kind = EdgeKind::parse(&kind_str);
    // self-provenance never reifies; a hand-authored derived_from makes no sense here.
    if matches!(kind, EdgeKind::DerivedFrom) {
        eprintln!("derived_from is inline self-provenance — not authorable as an edge");
        std::process::exit(2);
    }

    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let (Some(subject), Some(object)) = (sg.resolve_ref(&subject_ref), sg.resolve_ref(&object_ref)) else {
        eprintln!(
            "could not resolve {}{} in scope ({})",
            if sg.resolve_ref(&subject_ref).is_none() { format!("subject '{subject_ref}'") } else { String::new() },
            if sg.resolve_ref(&object_ref).is_none() { format!(" object '{object_ref}'") } else { String::new() },
            scopes.join(", ")
        );
        std::process::exit(2);
    };
    if subject == object {
        eprintln!("subject and object are the same belief — nothing to link");
        std::process::exit(2);
    }

    let scope = sg.by_id(&subject).map(|b| belief_scope(b).to_string()).unwrap_or_else(|| "global".into());
    let proposal = LinkProposal {
        kind: kind.clone(),
        subject,
        object,
        confidence: Confidence::Strong,
        rationale: if rationale.trim().is_empty() {
            "frontier adjudication".into()
        } else {
            format!("frontier adjudication: {}", rationale.trim())
        },
        linker: "frontier@1".into(), // durable: not a regenerable machine edge
    };
    match Consolidator::commit(&dir, std::slice::from_ref(&proposal), &scope) {
        0 => println!("[{subject_ref}] --{}--> [{object_ref}] already linked.", kind.as_str()),
        _ => {
            println!(
                "linked [{subject_ref}] --{}--> [{object_ref}] in scope={scope} (durable; authored by frontier@1).",
                kind.as_str()
            );
            // Explain the consequence + point onward, so the loop is legible without docs.
            match kind {
                EdgeKind::DependsOn => println!(
                    "→ [{subject_ref}] now retracts automatically if [{object_ref}] is withdrawn (JTMS). Check: mem recall \"{subject_ref}\""
                ),
                EdgeKind::Other(ref s) if s == "blocked_on" => println!(
                    "→ if [{object_ref}] is later retracted (the constraint lifts), [{subject_ref}] resurfaces in: mem debt"
                ),
                _ => {}
            }
        }
    }
}

/// `mem debt [<query>]` — the known-debt query (§3b): "what's hacky / known-deficient around here
/// that I shouldn't rely on or should fix?" Lists CURRENT beliefs carrying a deficiency envelope,
/// highest severity first; an optional <query> filters semantically. RESURFACE (N5): a debt whose
/// `blocked_on` edge points at a now-DEFEATED belief — its forcing constraint was lifted/retracted —
/// is flagged ⚠, the structural trigger to rework it. Deterministic; no LLM.
fn cmd_debt(args: &[String]) {
    let query: String =
        args.iter().filter(|a| !a.starts_with("--")).cloned().collect::<Vec<_>>().join(" ");
    let dir = store_dir();
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let defeated = sg.defeated();
    let mut debts: Vec<&Belief> =
        sg.current_content(&defeated).into_iter().filter(|b| b.deficiency.is_some()).collect();
    if debts.is_empty() {
        println!("No known-debt beliefs in scope ({}).", scopes.join(", "));
        return;
    }
    // optional semantic filter (graceful: if embeddings are missing, show everything)
    if !query.trim().is_empty() {
        if let Ok(hits) = rank(&dir, &debts, query.trim()) {
            let order: Vec<String> = hits.into_iter().map(|(id, _, _, _)| id).collect();
            debts.retain(|b| order.contains(&b.id));
            debts.sort_by_key(|b| order.iter().position(|id| id == &b.id).unwrap_or(usize::MAX));
            debts.truncate(8);
        }
    }
    // severity-first (high → medium → low), then most-recent.
    let sev_rank = |b: &Belief| match b.deficiency.as_ref().map(|d| d.severity.as_str()) {
        Some("high") => 0,
        Some("medium") => 1,
        _ => 2,
    };
    if query.trim().is_empty() {
        debts.sort_by(|a, b| sev_rank(a).cmp(&sev_rank(b)).then(b.txn_time.cmp(&a.txn_time)));
    }
    let adj = sg.adjacency(&defeated);
    println!("{} known-debt belief(s) [{}]:\n", debts.len(), scopes.join("+"));
    let mut any_resurfaced = false;
    for b in &debts {
        let d = b.deficiency.as_ref().unwrap();
        // resurface: an outgoing `blocked_on` edge whose target (the constraint) is now defeated.
        let resurfaced: Vec<String> = adj
            .get(&b.id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter(|r| r.kind == EdgeKind::Other("blocked_on".into()) && r.subject == b.id)
            .filter(|r| defeated.contains(&r.object))
            .filter_map(|r| sg.by_id(&r.object).map(|x| x.slug.clone()))
            .collect();
        println!("• [{}] [{}] {}", d.severity, b.slug, truncate(&b.claim, 90));
        println!("    forcing: {}", d.forcing_constraint);
        if let Some(rw) = &d.revisit_when {
            println!("    revisit: {rw}");
        }
        if !resurfaced.is_empty() {
            any_resurfaced = true;
            println!("    ⚠ RESURFACED — constraint lifted ([{}] retracted); time to rework", resurfaced.join("], ["));
        }
    }
    if any_resurfaced {
        println!(
            "\n⚠ a RESURFACED debt's forcing constraint has lifted. After reworking the code, retire the debt:\n  mem remember \"<the new, fixed state>\" --supersedes <slug>     (or  mem forget <slug>  if simply gone)"
        );
    } else {
        println!("\n(drill into any debt: mem expand <slug>)");
    }
}

/// Tier-0 onboarding: deterministic git-history harvest → lead files for the eyeball pass.
/// Writes OUTSIDE any repo by default (the data dir), so private-repo leads can't end up
/// committed by accident. Leads are candidates, not beliefs — nothing touches the store.
fn cmd_onboard(args: &[String]) {
    let mut repo = PathBuf::from(".");
    let mut out: Option<PathBuf> = None;
    let mut top = 25usize;
    let mut escalate = 0usize;
    let mut commit = false;
    let mut tier2 = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).map(PathBuf::from);
                i += 1;
            }
            "--top" => {
                top = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(25);
                i += 1;
            }
            "--escalate" => {
                escalate = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(30);
                i += 1;
            }
            "--tier2" => tier2 = true,
            "--commit" => commit = true,
            a if !a.starts_with("--") => repo = PathBuf::from(a),
            _ => {}
        }
        i += 1;
    }
    let now_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let h = match memory_onboard::harvest(&repo, now_epoch) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("onboard failed: {e}");
            std::process::exit(1);
        }
    };
    let out_dir = out.unwrap_or_else(|| {
        store_dir().parent().map(|p| p.to_path_buf()).unwrap_or_else(store_dir).join("onboard").join(&h.repo_id)
    });
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("can't create {}: {e}", out_dir.display());
        std::process::exit(1);
    }
    let json_path = out_dir.join("leads.json");
    let md_path = out_dir.join("leads.md");
    let _ = std::fs::write(&json_path, memory_onboard::leads_json(&h));
    let _ = std::fs::write(&md_path, memory_onboard::leads_md(&h, top));

    use memory_onboard::LeadKind;
    println!(
        "{}: {} commits → {} leads ({} reinstate, {} revert, {} rationale, {} debt, {} doc)",
        h.repo_id,
        h.commits_scanned,
        h.leads.len(),
        h.count(LeadKind::Reinstate),
        h.count(LeadKind::Revert),
        h.count(LeadKind::Rationale),
        h.count(LeadKind::Debt),
        h.count(LeadKind::Doc),
    );
    println!("report: {}", md_path.display());
    println!("full:   {}", json_path.display());
    if escalate == 0 && !commit {
        println!(
            "\nNext → skim the report, then draft beliefs from the leads:\n  mem onboard {} --escalate 30 --tier2   (--tier2 adds §3b debt envelopes to kludge leads)",
            repo.display()
        );
    }

    // tier 1 (opt-in): the judge model drafts a claim per selected lead, or rejects it as noise
    if escalate > 0 {
        let url = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into());
        let model = std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
        let picked = memory_onboard::select_for_escalation(&h.leads, escalate);
        eprintln!("escalating {} lead(s) through {model} ...", picked.len());
        // Tier-2 model: a stronger/extractive model for the deficiency structure (gemma2:9b is a
        // good fit — extraction wants recall, and this is NOT the dangerous depends_on judgment).
        let tier2_model = std::env::var("TIER2_MODEL").unwrap_or_else(|_| model.clone());
        let mut drafts = Vec::new();
        let mut failed = 0;
        let mut enriched = 0;
        for (i, lead) in picked.iter().enumerate() {
            match memory_onboard::escalate_lead(&repo, lead, &url, &model) {
                Ok(mut d) => {
                    // TIER 2: for kept DEBT-shaped drafts, extract the deficiency envelope (§3b).
                    if tier2 && d.keep && d.shape == "debt" {
                        memory_onboard::enrich_deficiency(&repo, &mut d, &url, &tier2_model);
                        if !d.def_forcing.is_empty() {
                            enriched += 1;
                        }
                    }
                    drafts.push(d);
                }
                Err(e) => {
                    failed += 1;
                    eprintln!("  lead {} failed: {e}", i + 1);
                }
            }
            eprint!("\r  {}/{}", i + 1, picked.len());
        }
        eprintln!();
        if tier2 {
            eprintln!("tier 2: {enriched} debt draft(s) given a deficiency envelope ({tier2_model})");
        }
        let kept = drafts.iter().filter(|d| d.keep && !d.claim.is_empty()).count();
        let dj = out_dir.join("drafts.json");
        let dm = out_dir.join("drafts.md");
        let _ = std::fs::write(&dj, memory_onboard::drafts_json(&h.repo_id, &model, &drafts));
        let _ = std::fs::write(&dm, memory_onboard::drafts_md(&h.repo_id, &model, &drafts));
        println!(
            "tier 1: {} drafted, {} kept, {} rejected as noise{}",
            drafts.len(),
            kept,
            drafts.len() - kept,
            if failed > 0 { format!(", {failed} failed") } else { String::new() }
        );
        println!("drafts: {}", dm.display());
        println!(
            "\nNext → review {} and DELETE any junk drafts from {} (commit reads the reviewed JSON), then store the keepers:\n  mem onboard {} --commit",
            dm.display(),
            dj.display(),
            repo.display()
        );
    }

    // commit: kept drafts (from the REVIEWED drafts.json — delete bad lines first) become
    // beliefs in repo canon. Low source_weight + directness:inferred mark them as a cheap
    // model's reading of evidence, not gospel; the frontier treats them accordingly.
    if commit {
        let drafts_path = out_dir.join("drafts.json");
        let kept = match memory_onboard::load_kept_drafts(&drafts_path) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("commit failed: {e} (run with --escalate first)");
                std::process::exit(1);
            }
        };
        let model = std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
        let dir = store_dir();
        let scope = format!("repo:{}", h.repo_id); // onboarded knowledge is repo canon
        let txn = iso_now();
        let (mut written, mut skipped) = (0, 0);
        for d in &kept {
            // deterministic id (no txn): re-running --commit is a no-op per claim
            let id = content_id(&format!("onboard|{scope}|{}", d.claim));
            let path = dir.join(format!("{id}.md"));
            if path.exists() {
                skipped += 1;
                continue;
            }
            let md = onboard_belief_md(&id, &scope, d, &model, &txn);
            if std::fs::write(&path, md).is_ok() {
                written += 1;
            }
        }
        println!("committed {written} onboarded belief(s) into scope={scope} ({skipped} already present)");
        if written > 0 {
            println!(
                "\nNext → use them:  mem recall \"<topic>\"   ·   mem debt   (known-debt)   ·   mem consolidate   (link them into the graph)"
            );
        }
        worker::note_writes_and_kick(&dir, written);
    }
}

/// Belief file for one kept draft. Differs from `belief_md` where it must: the author is the
/// judge MODEL (not the cli), directness is `inferred` (a model's reading of evidence, not a
/// stated fact), source_weight is conservative, and the lead's refs + evidence ride along.
fn onboard_belief_md(id: &str, scope: &str, d: &memory_onboard::Draft, model: &str, txn: &str) -> String {
    let one_line = d.claim.replace('\n', " ");
    let mut fm = String::new();
    fm.push_str("---\n");
    fm.push_str(&format!("id: {id}\n"));
    fm.push_str(&format!("slug: {}\n", slugify(&d.claim)));
    fm.push_str(&format!("scope: {scope}\n"));
    fm.push_str("claim:\n  kind: text\n  text: >-\n");
    fm.push_str(&format!("    {one_line}\n"));
    fm.push_str(&format!("author:\n  kind: agent\n  id: {model}\n"));
    fm.push_str(&format!("provenance:\n  txn_time: {txn}\n  valid_time: null\n"));
    fm.push_str("  source:\n    kind: onboard\n    session: tier1\n    turn: 0\n");
    if d.lead.refs.is_empty() {
        fm.push_str("  refs: []\n");
    } else {
        fm.push_str("  refs:\n");
        for r in &d.lead.refs {
            fm.push_str(&format!("    - {r}\n"));
        }
    }
    fm.push_str("  derived_from: []\n");
    fm.push_str(&format!(
        "confidence:\n  directness: inferred\n  observation_count: 1\n  source_weight: 0.5\n  asserted: {:.2}\n",
        d.asserted
    ));
    // TIER-2 deficiency envelope (§3b): emitted only for debt drafts that yielded a forcing
    // constraint. Orthogonal to confidence — this belief is true AND a known compromise.
    if !d.def_forcing.is_empty() {
        let sev = if d.def_severity.is_empty() { "medium" } else { &d.def_severity };
        fm.push_str(&format!("deficiency:\n  severity: {sev}\n  forcing_constraint: {}\n", d.def_forcing.replace('\n', " ")));
        match &d.def_revisit {
            Some(rw) => fm.push_str(&format!("  revisit_when: {}\n", rw.replace('\n', " "))),
            None => fm.push_str("  revisit_when: null\n"),
        }
    }
    fm.push_str("edges: []\ncoord: null\n---\n\n");
    fm.push_str(&format!("{}\n\nLead: {} ({})\n", d.why, d.lead.title, d.lead.date));
    if !d.lead.evidence.is_empty() {
        fm.push_str(&format!("\nEvidence:\n{}\n", d.lead.evidence));
    }
    fm
}

// --- novelty ledger (the negative-result cache + bridge-rate counters), JSONL sidecar ----

fn novelty_ledger(dir: &PathBuf) -> PathBuf {
    dir.join("novelty-probes.jsonl")
}

/// Load the probed-pair set (for skipping) + cumulative (attempts, bridges) for the metric.
fn load_probes(dir: &PathBuf) -> (std::collections::HashSet<String>, u64, u64) {
    let mut set = std::collections::HashSet::new();
    let (mut attempts, mut bridges) = (0u64, 0u64);
    if let Ok(text) = std::fs::read_to_string(novelty_ledger(dir)) {
        for line in text.lines() {
            if let (Some(a), Some(b)) = (jget(line, "\"a\":\""), jget(line, "\"b\":\"")) {
                set.insert(novelty_pair_key(&a, &b));
                attempts += 1;
                if line.contains("\"bridged\":true") {
                    bridges += 1;
                }
            }
        }
    }
    (set, attempts, bridges)
}

fn append_probes(dir: &PathBuf, records: &[ProbeRecord]) {
    if records.is_empty() {
        return;
    }
    let mut buf = String::new();
    for r in records {
        let ins = r.insight.replace('\\', " ").replace('"', "'").replace('\n', " ");
        buf.push_str(&format!(
            "{{\"a\":\"{}\",\"b\":\"{}\",\"sim\":{:.3},\"bridged\":{},\"insight\":\"{ins}\",\"at\":\"{}\"}}\n",
            r.a, r.b, r.sim, r.bridged, r.at
        ));
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(novelty_ledger(dir)) {
        let _ = f.write_all(buf.as_bytes());
    }
}

fn jget(line: &str, marker: &str) -> Option<String> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Read a BARE (unquoted) JSON number after `marker` (e.g. `"sim":` → 0.595). `jget` only reads
/// quoted strings; this stops at the first `,` or `}` and parses the value.
fn jget_num(line: &str, marker: &str) -> Option<f32> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().parse().ok()
}

/// Bridge insights authored by `mem dream`: pairkey → (insight, sim), only for `bridged:true`
/// probes that carry a non-empty insight. Keyed by `novelty_pair_key` (belief-id pair).
fn load_bridge_insights(dir: &PathBuf) -> HashMap<String, (String, f32)> {
    let mut out = HashMap::new();
    if let Ok(text) = std::fs::read_to_string(novelty_ledger(dir)) {
        for line in text.lines() {
            if !line.contains("\"bridged\":true") {
                continue;
            }
            let (Some(a), Some(b)) = (jget(line, "\"a\":\""), jget(line, "\"b\":\"")) else { continue };
            let insight = jget(line, "\"insight\":\"").unwrap_or_default();
            if insight.trim().is_empty() {
                continue;
            }
            let sim = jget_num(line, "\"sim\":").unwrap_or(1.0);
            out.insert(novelty_pair_key(&a, &b), (insight, sim));
        }
    }
    out
}

// --- ranking ---------------------------------------------------------------------------

type Hit = (String, String, String, Option<f32>); // (id, slug, claim, score)

/// Semantic rank via embeddings; Err → caller falls back to lexical.
fn rank(dir: &PathBuf, current: &[&Belief], query: &str) -> Result<Vec<Hit>, String> {
    let oll = memory_embed::Ollama::from_env();
    let mut cache = memory_embed::load_cache(dir, &oll.model);
    let need: Vec<&Belief> = current.iter().copied().filter(|b| !cache.contains_key(&b.id)).collect();
    if !need.is_empty() {
        let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
        let vecs = oll.embed(&docs)?;
        for (b, v) in need.iter().zip(vecs) {
            cache.insert(b.id.clone(), v);
        }
        memory_embed::save_cache(dir, &oll.model, &cache);
    }
    let qv = oll
        .embed(&[format!("search_query: {query}")])?
        .into_iter()
        .next()
        .ok_or("no query embedding")?;
    let mut scored: Vec<Hit> = current
        .iter()
        .filter_map(|b| {
            cache.get(&b.id).map(|v| (b.id.clone(), b.slug.clone(), b.claim.clone(), Some(cosine(&qv, v))))
        })
        .collect();
    scored.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    Ok(scored)
}

fn lexical(current: &[&Belief], query: &str) -> Vec<Hit> {
    let ql = query.to_lowercase();
    let mut hits: Vec<Hit> = current
        .iter()
        .filter(|b| b.slug.to_lowercase().contains(&ql) || b.claim.to_lowercase().contains(&ql))
        .map(|b| (b.id.clone(), b.slug.clone(), b.claim.clone(), None))
        .collect();
    hits.sort_by(|a, b| b.1.cmp(&a.1));
    hits
}

// --- scope (git-derived) ---------------------------------------------------------------

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn repo_id() -> Option<String> {
    let root = git(&["rev-parse", "--show-toplevel"])?;
    Some(root.rsplit('/').next().unwrap_or(&root).to_string())
}

fn current_branch() -> Option<String> {
    git(&["branch", "--show-current"])
}

fn is_default_branch(b: &str) -> bool {
    b == "main" || b == "master"
}

/// Scopes visible for recall here: global, + repo canon, + this feature branch (if any).
fn active_scopes() -> Vec<String> {
    let mut s = vec!["global".to_string()];
    if let Some(id) = repo_id() {
        s.push(format!("repo:{id}"));
        if let Some(b) = current_branch() {
            if !is_default_branch(&b) {
                s.push(format!("repo:{id}@{b}"));
            }
        }
    }
    s
}

/// Scope to TAG a new belief: most-specific active scope (or global outside a repo / --global).
fn write_scope(global: bool) -> String {
    if global {
        return "global".into();
    }
    match repo_id() {
        None => "global".into(),
        Some(id) => match current_branch() {
            Some(b) if !is_default_branch(&b) => format!("repo:{id}@{b}"),
            _ => format!("repo:{id}"),
        },
    }
}

fn belief_scope(b: &Belief) -> &str {
    if b.scope.is_empty() {
        "global"
    } else {
        &b.scope
    }
}

/// Clone the in-scope beliefs (content + edge-beliefs) into a sub-graph, so frontier
/// resolution is scope-relative — branch-scoped supersedes only bite when that branch is active.
fn scoped_graph(g: &Graph, scopes: &[String]) -> Graph {
    let sub: Vec<Belief> = g
        .iter()
        .filter(|b| scopes.iter().any(|s| s == belief_scope(b)))
        .cloned()
        .collect();
    Graph::from_beliefs(sub)
}

// --- belief writing --------------------------------------------------------------------

fn store_dir() -> PathBuf {
    // MEMORY_DIR > $XDG_DATA_HOME/agentic-memory/beliefs > ~/.local/share/agentic-memory/beliefs
    let dir = if let Ok(d) = std::env::var("MEMORY_DIR") {
        PathBuf::from(d)
    } else if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg).join("agentic-memory/beliefs")
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".local/share/agentic-memory/beliefs")
    };
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn slugify(claim: &str) -> String {
    let mut s = String::new();
    let mut dash = false;
    for c in claim.chars() {
        if c.is_ascii_alphanumeric() {
            s.push(c.to_ascii_lowercase());
            dash = false;
        } else if !s.is_empty() && !dash {
            s.push('-');
            dash = true;
        }
    }
    let words: Vec<&str> = s.trim_matches('-').split('-').filter(|w| !w.is_empty()).take(6).collect();
    if words.is_empty() { "belief".into() } else { words.join("-") }
}

fn belief_md(id: &str, slug: &str, scope: &str, claim: &str, refs: &[String], body: &str, txn: &str) -> String {
    let one_line = claim.replace('\n', " ");
    let mut fm = String::new();
    fm.push_str("---\n");
    fm.push_str(&format!("id: {id}\n"));
    fm.push_str(&format!("slug: {slug}\n"));
    fm.push_str(&format!("scope: {scope}\n"));
    fm.push_str("claim:\n  kind: text\n  text: >-\n");
    fm.push_str(&format!("    {one_line}\n"));
    fm.push_str("author:\n  kind: agent\n  id: cli\n");
    fm.push_str(&format!("provenance:\n  txn_time: {txn}\n  valid_time: null\n"));
    fm.push_str("  source:\n    kind: cli\n    session: mem\n    turn: 0\n");
    if refs.is_empty() {
        fm.push_str("  refs: []\n");
    } else {
        fm.push_str("  refs:\n");
        for r in refs {
            fm.push_str(&format!("    - {r}\n"));
        }
    }
    fm.push_str("  derived_from: []\n");
    fm.push_str("confidence:\n  directness: stated\n  observation_count: 1\n  source_weight: 0.8\n  asserted: null\n");
    fm.push_str("edges: []\ncoord: null\n---\n\n");
    fm.push_str(if body.is_empty() { "(remembered via mem CLI)" } else { body });
    fm.push('\n');
    fm
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(kind: &str, s: &str, o: &str) -> Relation {
        Relation { kind: EdgeKind::parse(kind), subject: s.into(), object: o.into() }
    }

    fn content_belief(id: &str, slug: &str, claim: &str) -> Belief {
        Belief { id: id.into(), slug: slug.into(), claim: claim.into(), ..Belief::default() }
    }

    fn same_as_edge(id: &str, member: &str, rep: &str) -> Belief {
        Belief {
            id: id.into(),
            slug: format!("rel-{id}"),
            relation: Some(Relation {
                kind: EdgeKind::Other("same-as".into()),
                subject: member.into(),
                object: rep.into(),
            }),
            ..Belief::default()
        }
    }

    #[test]
    fn collapse_map_folds_members_onto_rep_with_counts() {
        // two members fold onto one rep via current `same-as` edges
        let g = Graph::from_beliefs(vec![
            content_belief("b_rep", "rep", "the claim"),
            content_belief("b_m1", "m1", "the claim"),
            content_belief("b_m2", "m2", "the claim"),
            same_as_edge("e1", "b_m1", "b_rep"),
            same_as_edge("e2", "b_m2", "b_rep"),
        ]);
        let (map, counts) = collapse_map(&g, &g.defeated());
        assert_eq!(map.get("b_m1"), Some(&"b_rep".to_string()));
        assert_eq!(map.get("b_m2"), Some(&"b_rep".to_string()));
        assert_eq!(map.get("b_rep"), None, "the rep is not folded onto anything");
        assert_eq!(counts.get("b_rep"), Some(&2), "rep absorbed two members");
    }

    #[test]
    fn collapse_map_resolves_transitive_chain() {
        // m → x → rep must resolve m all the way to rep (hop guard, no runaway)
        let g = Graph::from_beliefs(vec![
            content_belief("b_rep", "rep", "c"),
            content_belief("b_x", "x", "c"),
            content_belief("b_m", "m", "c"),
            same_as_edge("e1", "b_x", "b_rep"),
            same_as_edge("e2", "b_m", "b_x"),
        ]);
        let (map, _counts) = collapse_map(&g, &g.defeated());
        assert_eq!(map.get("b_m"), Some(&"b_rep".to_string()), "chain resolves to the final rep");
        assert_eq!(map.get("b_x"), Some(&"b_rep".to_string()));
    }

    #[test]
    fn collapse_drops_generic_when_specific_links_same_pair() {
        let rels = vec![rel("refines", "A", "B"), rel("relates-to", "A", "B")];
        let out = collapse("A", &rels);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind.as_str(), "refines");
    }

    #[test]
    fn collapse_keeps_generic_when_it_is_the_sole_link() {
        let rels = vec![rel("relates-to", "A", "C")];
        assert_eq!(collapse("A", &rels).len(), 1);
    }

    #[test]
    fn collapse_is_per_neighbor_not_global() {
        // relates-to→B is subsumed (refines exists); relates-to→C survives (sole link to C)
        let rels = vec![rel("refines", "A", "B"), rel("relates-to", "A", "B"), rel("relates-to", "A", "C")];
        let out = collapse("A", &rels);
        let kinds: Vec<(&str, String)> = out
            .iter()
            .map(|r| (r.kind.as_str(), if r.subject == "A" { r.object.clone() } else { r.subject.clone() }))
            .collect();
        assert_eq!(kinds.len(), 2);
        assert!(kinds.contains(&("refines", "B".into())));
        assert!(kinds.contains(&("relates-to", "C".into())));
    }

    #[test]
    fn promoted_md_stamps_derived_from_provenance() {
        let md = promoted_belief_md(
            "b_new", "branch-fact", "repo:proj", "the fact", "body", "2026-01-01T00:00:00.000Z", "b_orig",
        );
        assert!(md.contains("derived_from:\n    - b_orig"), "promoted belief must point back at its origin");
        assert!(!md.contains("derived_from: []"), "the empty derived_from must be replaced");
        let b = Belief::parse(&md).expect("promoted md parses");
        assert_eq!(b.scope, "repo:proj");
        assert_eq!(b.id, "b_new");
    }

    #[test]
    fn collapse_handles_incoming_direction() {
        // anchor is the OBJECT of the specific edge; generic to same neighbor still collapses
        let rels = vec![rel("refines", "B", "A"), rel("relates-to", "B", "A")];
        let out = collapse("A", &rels);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind.as_str(), "refines");
    }

    #[test]
    fn confidence_boost_is_monotone_and_capped() {
        // factor grows with structural weight, bounded in [1.0, 1.12].
        let b0 = confidence_boost(0.0);
        let bmid = confidence_boost(0.5);
        let bmax = confidence_boost(1.0);
        assert_eq!(b0, 1.0, "zero-confidence → no boost");
        assert!(bmid > b0 && bmax > bmid, "higher confidence ⇒ higher boost");
        assert!(bmax <= 1.12 + 1e-6, "boost is capped at ≤12%");
        assert_eq!(confidence_boost(5.0), bmax, "weight is clamped to 1.0");
    }

    #[test]
    fn nearer_hit_still_outranks_far_but_confident_one() {
        // A clearly-nearer belief (cosine 0.80, low confidence) must beat a far one (0.62) even when
        // the far one is maximally confident — similarity stays dominant.
        let near = 0.80f32 * confidence_boost(0.0);
        let far_confident = 0.62f32 * confidence_boost(1.0);
        assert!(near > far_confident, "boost must not let a far belief leapfrog a clearly nearer one");

        // But it DOES break a near-tie: 0.70 high-confidence beats a bare 0.70 low-confidence.
        let tie_a = 0.70f32 * confidence_boost(0.9);
        let tie_b = 0.70f32 * confidence_boost(0.1);
        assert!(tie_a > tie_b, "confidence breaks a near-tie");
    }

    #[test]
    fn contest_str_names_the_other_endpoint() {
        let slug_of = |id: &str| match id {
            "B" => Some("rival-belief".to_string()),
            "C" => Some("other-rival".to_string()),
            _ => None,
        };
        // anchor A is attacked-by B (A is object) and attacks C (A is subject); both name the other end
        let edges = vec![rel("attacks", "B", "A"), rel("attacks", "A", "C")];
        let s = contest_str("A", &edges, &slug_of);
        assert!(s.contains("[rival-belief]"), "names the attacker: {s}");
        assert!(s.contains("[other-rival]"), "names the attacked: {s}");
        assert!(s.starts_with("contested by "), "{s}");

        // uncontested → empty
        assert_eq!(contest_str("A", &[rel("supports", "B", "A")], &slug_of), "");
    }

    #[test]
    fn jget_num_parses_bare_sim_from_a_ledger_line() {
        let line = r#"{"a":"b_4fa17525cfed","b":"b_d7ccca88b5b1","sim":0.595,"bridged":true,"insight":"Both curate initial conditions.","at":"2026-06-09T06:50:05.662Z"}"#;
        assert_eq!(jget_num(line, "\"sim\":"), Some(0.595));
        // quoted-string jget still reads the string fields
        assert_eq!(jget(line, "\"a\":\""), Some("b_4fa17525cfed".to_string()));
        // a sim that ends the object (before `}`) still parses
        let tail = r#"{"x":1,"sim":0.872}"#;
        assert_eq!(jget_num(tail, "\"sim\":"), Some(0.872));
        // missing marker → None
        assert_eq!(jget_num(line, "\"nope\":"), None);
    }

    /// Mirror the show_bridge resolution in cmd_recall (settings.bridges && !no_bridges) so the
    /// toggle precedence is pinned: default true; --no-bridges or MEM_RECALL_BRIDGES=false → false.
    fn show_bridge(bridges_setting: bool, no_bridges_flag: bool) -> bool {
        bridges_setting && !no_bridges_flag
    }

    #[test]
    fn bridge_toggle_defaults_true() {
        // the default RecallSettings enables bridges, and no flag is set
        assert!(RecallSettings::default().bridges);
        assert!(show_bridge(RecallSettings::default().bridges, false));
    }

    #[test]
    fn bridge_toggle_suppressed_by_flag_or_env() {
        // --no-bridges suppresses even when the setting is on
        assert!(!show_bridge(true, true));
        // MEM_RECALL_BRIDGES=false (parsed into the setting) suppresses with no flag
        assert!(!show_bridge(false, false));
        // both off → off
        assert!(!show_bridge(false, true));
    }
}
