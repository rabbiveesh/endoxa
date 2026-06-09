//! memory-eval — v0 efficacy harness (the comparative delta, cheapest fully-grounded form).
//!
//! Compares frontier-RELATIVE refutation (`Graph::defeated()`, the alternating fixpoint) against
//! the NAIVE flat refuted-list ("anything a defeating edge points at, regardless of whether that
//! edge's own source is defeated"). The disagreements ARE the keystone cases the R3 corpus round
//! seeded on purpose:
//!   * REINSTATEMENT (verdict-of-a-verdict): naive says refuted, frontier says CURRENT.
//!   * OPEN CONFLICT: a belief under a live `attacks` that the frontier keeps current — a naive
//!     "an attack refutes its target" system would drop it.
//! Both are "inverted answer" cases the keystone claims to prevent. Deterministic, no LLM.
//! Run: `cargo run -p memory-core --bin eval`.

use memory_core::{EdgeKind, Graph};
use std::collections::HashSet;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let mut corpora: Vec<String> = std::fs::read_dir(&root)
        .expect("corpus dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("beliefs").is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    corpora.sort();

    println!("{:<22} {:>8} {:>9} {:>11} {:>10}", "corpus", "content", "defeated", "reinstated", "open-conf");
    println!("{:-<62}", "");

    let (mut t_content, mut t_def, mut t_reinst, mut t_open) = (0usize, 0usize, 0usize, 0usize);
    let mut wins: Vec<(String, String)> = Vec::new();

    for name in &corpora {
        let Ok(g) = Graph::load_dir(&root.join(name).join("beliefs")) else { continue };
        let ids: HashSet<&str> = g.beliefs.iter().map(|b| b.id.as_str()).collect();
        let frontier = g.defeated();
        let naive = naive_refuted(&g, &ids);
        let slug = |id: &str| g.beliefs.iter().find(|b| b.id == id).map(|b| b.slug.clone()).unwrap_or_else(|| id.into());

        // REINSTATEMENT: naive marks it refuted, the frontier keeps it current.
        let mut reinstated = 0usize;
        for id in &naive {
            if !frontier.contains(id) {
                reinstated += 1;
                wins.push((name.clone(), slug(id)));
            }
        }

        // OPEN CONFLICT: target of a LIVE `attacks` edge that the frontier keeps current.
        let mut open: HashSet<String> = HashSet::new();
        for b in &g.beliefs {
            if frontier.contains(&b.id) {
                continue; // the attacking edge must itself be in force
            }
            let target: Option<&str> = match &b.relation {
                Some(r) if r.kind == EdgeKind::Attacks => Some(r.object.as_str()),
                _ => b.edges.iter().find(|e| e.kind == EdgeKind::Attacks).map(|e| e.target.as_str()),
            };
            if let Some(t) = target {
                if ids.contains(t) && !frontier.contains(t) {
                    open.insert(t.to_string());
                }
            }
        }

        let n_content = g.beliefs.iter().filter(|b| b.relation.is_none()).count();
        println!("{:<22} {:>8} {:>9} {:>11} {:>10}", name, n_content, frontier.len(), reinstated, open.len());
        t_content += n_content;
        t_def += frontier.len();
        t_reinst += reinstated;
        t_open += open.len();
    }

    println!("{:-<62}", "");
    println!("{:<22} {:>8} {:>9} {:>11} {:>10}", "TOTAL", t_content, t_def, t_reinst, t_open);
    println!();
    println!(
        "Keystone delta: frontier resolution flips {} belief(s) a naive refuted-list gets backwards",
        t_reinst + t_open
    );
    println!("  ({t_reinst} verdict-of-a-verdict reinstatement(s) + {t_open} open-conflict belief(s) kept live)");
    if !wins.is_empty() {
        println!("\nReinstatements (naive says refuted, frontier says CURRENT):");
        for (c, s) in &wins {
            println!("  ↩ [{c}] {s}");
        }
    }
}

/// The naive flat refuted-list: every belief a defeating edge (inline or reified) points at —
/// WITHOUT the fixpoint guard that ignores edges whose own source is defeated.
fn naive_refuted(g: &Graph, ids: &HashSet<&str>) -> HashSet<String> {
    let mut r = HashSet::new();
    for b in &g.beliefs {
        for e in &b.edges {
            if e.kind.is_defeating() && ids.contains(e.target.as_str()) {
                r.insert(e.target.clone());
            }
        }
        if let Some(rel) = &b.relation {
            if rel.kind.is_defeating() && ids.contains(rel.object.as_str()) {
                r.insert(rel.object.clone());
            }
        }
    }
    r
}
