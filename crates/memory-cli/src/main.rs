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
use memory_core::{content_id, cosine, iso_now, Belief, Cadence, EdgeKind, Graph, Hint, LinkCtx, Relation};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("remember") => cmd_remember(&args[1..]),
        Some("recall") => cmd_recall(&args[1..]),
        Some("expand") => cmd_expand(&args[1..]),
        Some("ask") => cmd_ask(&args[1..]),
        Some("consolidate") => cmd_consolidate(&args[1..]),
        Some("scope") => println!("active scopes: {}", active_scopes().join(", ")),
        _ => {
            eprintln!("usage:");
            eprintln!("  mem remember \"<claim>\" [--supersedes <slug|id>] [--global] [--ref R] [--body B]");
            eprintln!("  mem recall \"<query>\" [--limit N]");
            eprintln!("  mem expand <slug|id>          # one-hop: show a belief's linked context");
            eprintln!("  mem ask \"<question>\" [--limit N]  # LLM-synthesized grounded answer (opt-in)");
            eprintln!("  mem consolidate [--limit N]   # run the LLM judge over recent beliefs");
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
    // Affordances: annotate each hit with its current edge-counts so the robot KNOWS what's
    // expandable (and can probe with `mem expand`) — we don't walk the graph for it here.
    let adj = relation_adjacency(&sg);
    let mut any_expandable = false;
    for (id, slug, claim, score) in &hits {
        let raw = adj.get(id).map(|v| v.as_slice()).unwrap_or(&[]);
        let edges = collapse(id, raw);
        let (aff, contested) = affordance_str(&edges);
        if !aff.is_empty() {
            any_expandable = true;
        }
        let tag = if aff.is_empty() { String::new() } else { format!("   ({aff} → mem expand)") };
        let warn = if contested { "  ⚠ contested" } else { "" };
        match score {
            Some(s) => println!("• ({s:.2}) [{slug}] {claim}{tag}{warn}"),
            None => println!("• [{slug}] {claim}{tag}{warn}"),
        }
    }
    if any_expandable {
        println!("\ndrill into linked context: mem expand <slug>");
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
    let target = sg.beliefs.iter().find(|b| b.id == id).unwrap();
    println!("[{}] {}\n", target.slug, target.claim);

    let defeated = sg.defeated();
    let adj = relation_adjacency(&sg);
    let raw = adj.get(&id).map(|v| v.as_slice()).unwrap_or(&[]);
    let rels = collapse(&id, raw);
    let mut printed = 0;
    for r in &rels {
        let outgoing = r.subject == id;
        let other_id = if outgoing { &r.object } else { &r.subject };
        let Some(nb) = sg.beliefs.iter().find(|b| &b.id == other_id) else { continue };
        let label = if outgoing {
            format!("{} →", r.kind.as_str())
        } else {
            format!("← {}", r.kind.as_str())
        };
        let mark = if defeated.contains(&nb.id) { "  [superseded]" } else { "" };
        println!("  {label:>14}  [{}] {}{}", nb.slug, nb.claim, mark);
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
    let scopes = active_scopes();
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let defeated = sg.defeated();
    let current: Vec<&Belief> = sg
        .beliefs
        .iter()
        .filter(|b| !defeated.contains(&b.id) && b.relation.is_none())
        .collect();
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

    let mut ctx = format!("Question: {question}\n\nBeliefs:\n");
    for (_id, slug, claim, _s) in &hits {
        ctx.push_str(&format!("- [{slug}] {claim}\n"));
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

/// Index from belief-id → the CURRENT (undefeated) edge-beliefs touching it (as subject or
/// object). Deterministic; powers recall affordances + `mem expand`.
fn relation_adjacency(g: &Graph) -> std::collections::HashMap<String, Vec<Relation>> {
    let defeated = g.defeated();
    let mut adj: std::collections::HashMap<String, Vec<Relation>> = std::collections::HashMap::new();
    for b in &g.beliefs {
        let Some(r) = &b.relation else { continue };
        if defeated.contains(&b.id) {
            continue; // a defeated edge is no longer in force
        }
        adj.entry(r.subject.clone()).or_default().push(r.clone());
        adj.entry(r.object.clone()).or_default().push(r.clone());
    }
    adj
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
    let g = match Graph::load_dir(&dir) {
        Ok(g) => g,
        Err(_) => { println!("No memories yet."); return; }
    };
    let sg = scoped_graph(&g, &scopes);
    let content: Vec<&Belief> = sg.beliefs.iter().filter(|b| b.relation.is_none()).collect();
    if content.is_empty() {
        println!("Nothing to consolidate in scope ({}).", scopes.join(", "));
        return;
    }

    // make sure every in-scope content belief has an embedding (candidate generation needs it)
    let oll = memory_embed::Ollama::from_env();
    let mut vectors = memory_embed::load_cache(&dir, &oll.model);
    let need: Vec<&Belief> = content.iter().copied().filter(|b| !vectors.contains_key(&b.id)).collect();
    if !need.is_empty() {
        let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
        match oll.embed(&docs) {
            Ok(vs) => {
                for (b, v) in need.iter().zip(vs) {
                    vectors.insert(b.id.clone(), v);
                }
                memory_embed::save_cache(&dir, &oll.model, &vectors);
            }
            Err(e) => { eprintln!("embedding failed ({e}); can't run candidate generation"); return; }
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
        total += Consolidator::commit(&dir, &props, belief_scope(t));
    }
    println!("consolidated {} belief(s); drew {} new edge(s)", targets.len(), total);
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
        .beliefs
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
    fn collapse_handles_incoming_direction() {
        // anchor is the OBJECT of the specific edge; generic to same neighbor still collapses
        let rels = vec![rel("refines", "B", "A"), rel("relates-to", "B", "A")];
        let out = collapse("A", &rels);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind.as_str(), "refines");
    }
}
