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

use memory_consolidate::{Consolidator, Orchestrator};
use memory_core::{content_id, cosine, iso_now, Belief, Cadence, EdgeKind, Graph, Hint, LinkCtx};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("remember") => cmd_remember(&args[1..]),
        Some("recall") => cmd_recall(&args[1..]),
        Some("scope") => println!("active scopes: {}", active_scopes().join(", ")),
        _ => {
            eprintln!("usage:");
            eprintln!("  mem remember \"<claim>\" [--supersedes <slug|id>] [--global] [--ref R] [--body B]");
            eprintln!("  mem recall \"<query>\" [--limit N]");
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
}

fn cmd_recall(args: &[String]) {
    let mut query = String::new();
    let mut limit = 10usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => { limit = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(10); i += 1; }
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
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    // scope-filter BEFORE resolving the frontier (gives branch divergence for free)
    let sg = scoped_graph(&g, &scopes);
    if sg.beliefs.is_empty() {
        println!("No memories in scope ({}).", scopes.join(", "));
        return;
    }
    let defeated = sg.defeated();
    let current: Vec<&Belief> = sg
        .beliefs
        .iter()
        .filter(|b| !defeated.contains(&b.id) && b.relation.is_none())
        .collect();

    let dropped = sg.beliefs.iter().filter(|b| b.relation.is_none() && defeated.contains(&b.id)).count();
    let (mut hits, mode) = match rank(&dir, &current, &query) {
        Ok(r) => (r, format!("semantic; scopes: {}", scopes.join("+"))),
        Err(reason) => (lexical(&current, &query), format!("lexical fallback ({reason})")),
    };
    if hits.is_empty() {
        println!("Nothing current recalled for \"{query}\".");
        return;
    }
    hits.truncate(limit);
    println!(
        "Recalled {} current belief(s) [{mode}; {dropped} superseded/refuted dropped]:\n",
        hits.len()
    );
    for (slug, claim, score) in &hits {
        match score {
            Some(s) => println!("• ({s:.2}) [{slug}] {claim}"),
            None => println!("• [{slug}] {claim}"),
        }
    }
}

// --- consolidation (on write) ----------------------------------------------------------

fn consolidate(dir: &PathBuf, new_id: &str, scope: &str, hints: &[Hint]) -> usize {
    let g = match Graph::load_dir(dir) {
        Ok(g) => g,
        Err(_) => return 0,
    };
    let sg = scoped_graph(&g, &active_scopes());
    let Some(new) = sg.beliefs.iter().find(|b| b.id == new_id) else {
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
    let proposals = Orchestrator::with_defaults().run(&ctx, &[Cadence::OnWrite, Cadence::Nrem]);
    Consolidator::commit(dir, &proposals, scope)
}

// --- ranking ---------------------------------------------------------------------------

type Hit = (String, String, Option<f32>);

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
        .filter_map(|b| cache.get(&b.id).map(|v| (b.slug.clone(), b.claim.clone(), Some(cosine(&qv, v)))))
        .collect();
    scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    Ok(scored)
}

fn lexical(current: &[&Belief], query: &str) -> Vec<Hit> {
    let ql = query.to_lowercase();
    let mut hits: Vec<Hit> = current
        .iter()
        .filter(|b| b.slug.to_lowercase().contains(&ql) || b.claim.to_lowercase().contains(&ql))
        .map(|b| (b.slug.clone(), b.claim.clone(), None))
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0));
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
        .beliefs
        .iter()
        .filter(|b| scopes.iter().any(|s| s == belief_scope(b)))
        .cloned()
        .collect();
    Graph::from_beliefs(sub)
}

// --- belief writing --------------------------------------------------------------------

fn store_dir() -> PathBuf {
    let dir = std::env::var("MEMORY_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".local/share/agentic-memory/beliefs")
    });
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
