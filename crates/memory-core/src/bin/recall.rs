//! `recall` — a tiny demo of frontier-aware recall over a belief corpus.
//!
//! Stands in for the real L2 lens (a lexical substring match instead of embeddings) so the
//! *frontier* effect is visible in isolation: it prints what a naive store returns vs. what
//! recall should return once superseded/refuted beliefs are dropped.
//!
//!   cargo run -p memory-core --bin recall -- corpus/composr post-autoload

use memory_core::{Belief, Graph};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: recall <corpus-dir> <query-substring>");
        eprintln!("  e.g. recall corpus/composr post-autoload");
        std::process::exit(2);
    }
    let corpus = &args[1];
    let query = args[2].to_lowercase();
    let beliefs_dir = Path::new(corpus).join("beliefs");

    let g = Graph::load_dir(&beliefs_dir).unwrap_or_else(|e| {
        eprintln!("failed to load {}: {}", beliefs_dir.display(), e);
        std::process::exit(1);
    });
    let defeated = g.defeated();
    let matches = |b: &Belief| {
        b.slug.to_lowercase().contains(&query) || b.claim.to_lowercase().contains(&query)
    };

    println!(
        "loaded {} beliefs from {} ({} defeated on the main frontier)\n",
        g.len(),
        beliefs_dir.display(),
        defeated.len()
    );

    println!("NAIVE (lexical match, no frontier) — what a vector-ish store would surface:");
    for b in g.iter().filter(|b| matches(b)) {
        let tag = if defeated.contains(&b.id) {
            "   <-- DEFEATED (superseded/refuted), but lexically central"
        } else {
            ""
        };
        println!("  - {}{}", b.slug, tag);
    }

    println!("\nRESOLVED (current frontier only) — what recall returns:");
    let mut dropped = Vec::new();
    for b in g.iter().filter(|b| matches(b)) {
        if defeated.contains(&b.id) {
            dropped.push(b.slug.as_str());
        } else {
            println!("  + {}", b.slug);
        }
    }
    if !dropped.is_empty() {
        println!("\n  dropped as superseded/refuted (kept for reliving, not surfaced as current):");
        for s in dropped {
            println!("    x {}", s);
        }
    }
}
