//! eval-worlds — the worlds keystone: parallel realities from one belief DAG (V3 → N6).
//!
//! Two passes over every corpus that ships a `worlds.json`:
//!
//! **Deterministic (default, CI-able, no LLM)** — re-proves V3's reachability table in-tree:
//! for each world, suppress-then-refixpoint via `Graph::defeated_with` and check that
//!   1. every non-default world with a suppress set actually DIVERGES from the default frontier;
//!   2. every flipped-to-live belief is defeated on the default frontier (suppression *reinstates*,
//!      it never invents);
//!   3. for every reduction fixture, each pair of worlds with different expected answers sees a
//!      DIFFERENT live subset of the fixture's neighborhood — the frontier makes divergent
//!      consensus *possible* (the necessary substrate half of the demo);
//!   4. each expected world has ≥1 live neighborhood belief (something to reduce).
//! Exit code 1 on any failure.
//!
//! **LLM (`--llm`, needs ollama)** — the sufficiency half, N6's done-when: for each fixture ×
//! world, feed the reducer the world-frontier-filtered neighborhood WITH the world's assumption
//! threaded into the prompt (both halves V3 measured as load-bearing), then grade world-relative
//! divergence by embedding proximity: the answer must sit closer to its OWN world's expected
//! consensus than to every other world's. All cells passing = the reduction_fixtures graduate
//! from TARGET to DEMONSTRATED end-to-end (`ASK_MODEL`/`JUDGE_MODEL` override; qwen2.5:7b @ 0).
//!
//! Usage: cargo run -p memory-cli --bin eval-worlds [-- [--llm] [corpus-dir ...]]

use memory_core::{cosine, Graph};
use std::collections::HashSet;
use std::path::PathBuf;

#[path = "../worlds.rs"]
mod worlds;

const REDUCE_SYSTEM: &str = "You answer a question using ONLY the supplied beliefs. Never invent \
facts that aren't in them. If a working assumption is given, answer AS IF it holds — when several \
beliefs are live, prefer the one the assumption selects. Cite the [slug] of every belief you draw \
on. Be concise. Reply JSON: {\"answer\": \"...\", \"cited\": [\"slug\"]}";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let llm = args.iter().any(|a| a == "--llm");
    let mut dirs: Vec<PathBuf> = args.iter().filter(|a| !a.starts_with("--")).map(PathBuf::from).collect();
    if dirs.is_empty() {
        // every corpus that ships worlds gold
        if let Ok(rd) = std::fs::read_dir("corpus") {
            for e in rd.flatten() {
                if e.path().join("worlds.json").is_file() {
                    dirs.push(e.path());
                }
            }
        }
        dirs.sort();
    }
    if dirs.is_empty() {
        eprintln!("no corpus with a worlds.json found (run from the repo root, or pass dirs)");
        std::process::exit(2);
    }

    let (mut checks, mut failures) = (0usize, 0usize);
    let mut llm_cells: Vec<(String, String, bool)> = Vec::new(); // (corpus/fixture, world, pass)
    let mut llm_errors = 0usize;

    for dir in &dirs {
        let name = dir.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
        let g = match Graph::load_dir(&dir.join("beliefs")) {
            Ok(g) if !g.is_empty() => g,
            _ => { eprintln!("[{name}] no beliefs — skipping"); continue; }
        };
        let Some(wf) = worlds::load(&dir.join("beliefs")) else {
            eprintln!("[{name}] worlds.json unparsable — skipping");
            failures += 1;
            continue;
        };
        let default = wf.default_world().cloned().unwrap_or_default();
        let d_default = g.defeated_in(&default);
        println!("=== {name}: {} worlds, {} fixtures ===", wf.worlds.len(), wf.fixtures.len());

        // 1+2: divergence + reinstatement-only
        for w in &wf.worlds {
            if w.name == default.name || w.suppress.is_empty() {
                continue;
            }
            let d = g.defeated_in(w);
            let flips = g.frontier_flips(&d_default, &d);
            checks += 1;
            if flips.is_empty() {
                println!("  FAIL [{}] suppress set changes nothing on the frontier", w.name);
                failures += 1;
            } else {
                for (b, live_default, live_w) in &flips {
                    let arrow = if *live_w { "reinstated" } else { "re-defeated" };
                    println!("  world {}: [{}] {arrow}", w.name, b.slug);
                    checks += 1;
                    // a flip must trace back to the default frontier having the opposite status —
                    // guaranteed by construction of frontier_flips; the substantive claim is that
                    // *reinstated* beliefs were genuinely defeated by a now-suppressed source.
                    if *live_w && *live_default {
                        println!("  FAIL [{}] flip without a default-side defeat", w.name);
                        failures += 1;
                    }
                }
            }
        }

        // 3+4: fixture-neighborhood reachability (the V3 table, generalized)
        for f in &wf.fixtures {
            println!("  fixture: {}", f.query);
            let mut live_sets: Vec<(String, HashSet<String>)> = Vec::new();
            for (wname, _expected) in &f.expected_by_world {
                let Some(w) = wf.get(wname) else {
                    println!("    FAIL expected world '{wname}' not defined");
                    failures += 1;
                    continue;
                };
                let d = g.defeated_in(w);
                let live: HashSet<String> = f
                    .neighborhood
                    .iter()
                    .filter(|s| g.get(s).map(|b| !d.contains(&b.id)).unwrap_or(false))
                    .cloned()
                    .collect();
                let table: Vec<String> = f
                    .neighborhood
                    .iter()
                    .map(|s| format!("{s}={}", if live.contains(s) { "live" } else { "DEFEAT" }))
                    .collect();
                println!("    {wname}: {}", table.join("  "));
                checks += 1;
                if live.is_empty() {
                    println!("    FAIL [{wname}] no live neighborhood belief — nothing to reduce");
                    failures += 1;
                }
                live_sets.push((wname.clone(), live));
            }
            for i in 0..live_sets.len() {
                for j in (i + 1)..live_sets.len() {
                    checks += 1;
                    if live_sets[i].1 == live_sets[j].1 {
                        println!(
                            "    FAIL identical live neighborhoods for {} vs {} — divergent answers are impossible",
                            live_sets[i].0, live_sets[j].0
                        );
                        failures += 1;
                    }
                }
            }

            // --llm: the sufficiency half — does the reducer actually SELECT world-relatively?
            if llm {
                run_llm_fixture(&name, &g, &wf, f, &mut llm_cells, &mut llm_errors);
            }
        }
        println!();
    }

    println!("deterministic: {} checks, {failures} failures — {}", checks, if failures == 0 { "the frontier substrate for world-relative reduction HOLDS" } else { "BROKEN" });
    if llm {
        let passed = llm_cells.iter().filter(|(_, _, p)| *p).count();
        println!("\nllm divergence: {passed}/{} fixture×world cells select their own world's answer ({llm_errors} error(s))", llm_cells.len());
        for (fx, w, p) in &llm_cells {
            println!("  [{}] {fx} × {w}", if *p { "PASS" } else { "FAIL" });
        }
        if !llm_cells.is_empty() && passed == llm_cells.len() && llm_errors == 0 {
            println!("\n→ world-relative reduction DEMONSTRATED end-to-end (the N6 done-when): the same DAG, reduced under different assumptions, yields each world's gold answer.");
        }
    } else {
        println!("(run with --llm and ollama up for the reducer-divergence half — the N6 graduation gate)");
    }
    if failures > 0 {
        std::process::exit(1);
    }
}

/// One fixture × all its expected worlds through the live reducer, graded by embedding
/// proximity to each world's gold consensus (own-world must win over every other).
fn run_llm_fixture(
    corpus: &str,
    g: &Graph,
    wf: &worlds::WorldFile,
    f: &worlds::Fixture,
    cells: &mut Vec<(String, String, bool)>,
    errors: &mut usize,
) {
    let model = std::env::var("ASK_MODEL")
        .or_else(|_| std::env::var("JUDGE_MODEL"))
        .unwrap_or_else(|_| "qwen2.5:7b".into());
    let oll = memory_embed::Ollama::from_env();

    let mut answers: Vec<(String, String)> = Vec::new(); // (world, answer)
    for (wname, _expected) in &f.expected_by_world {
        let Some(w) = wf.get(wname) else { continue };
        let d = g.defeated_in(w);
        // oracle the neighborhood (isolate the reasoning layer from retrieval noise), then
        // world-frontier-filter it — the reducer must never see a belief this world defeats.
        let mut ctx = String::new();
        if !w.assumption.is_empty() {
            ctx.push_str(&format!("Working assumption (world '{}'): {}\n\n", w.name, w.assumption));
        }
        ctx.push_str(&format!("Question: {}\n\nBeliefs:\n", f.query));
        for slug in &f.neighborhood {
            if let Some(b) = g.get(slug) {
                if !d.contains(&b.id) {
                    ctx.push_str(&format!("- [{slug}] {}\n", b.claim));
                }
            }
        }
        match memory_embed::chat_json(&oll.url, &model, REDUCE_SYSTEM, &ctx) {
            Ok(v) => {
                let ans = v.get("answer").and_then(|x| x.as_str()).unwrap_or("").to_string();
                println!("    {wname} answer: {ans}");
                answers.push((wname.clone(), ans));
            }
            Err(e) => {
                println!("    {wname} reducer error: {e}");
                *errors += 1;
            }
        }
    }

    // grade: embed answers + golds; own-world gold must be the nearest
    let mut texts: Vec<String> = Vec::new();
    for (_, a) in &answers {
        texts.push(format!("search_document: {a}"));
    }
    for (_, exp) in &f.expected_by_world {
        texts.push(format!("search_document: {exp}"));
    }
    let vecs = match oll.embed(&texts) {
        Ok(v) => v,
        Err(e) => {
            println!("    (grading unavailable: {e})");
            *errors += 1;
            return;
        }
    };
    let (ans_vecs, gold_vecs) = vecs.split_at(answers.len());
    for (i, (wname, _)) in answers.iter().enumerate() {
        let own = f.expected_by_world.iter().position(|(w, _)| w == wname);
        let Some(own) = own else { continue };
        let own_sim = cosine(&ans_vecs[i], &gold_vecs[own]);
        let best_other = (0..gold_vecs.len())
            .filter(|&j| j != own)
            .map(|j| cosine(&ans_vecs[i], &gold_vecs[j]))
            .fold(f32::MIN, f32::max);
        let pass = own_sim > best_other;
        println!("    {wname}: own-gold {:.3} vs best-other {:.3} → {}", own_sim, best_other, if pass { "PASS" } else { "FAIL" });
        cells.push((format!("{corpus}: {}", truncate(&f.query, 50)), wname.clone(), pass));
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let t: String = s.chars().take(n).collect();
        format!("{t}…")
    }
}
