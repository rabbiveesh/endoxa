//! eval-qa — v1 efficacy harness (behavioral QA).
//!
//! Asks current-state questions whose answer has a supersession/verdict HISTORY, under two recall
//! configs — FRONTIER-resolved (drop the defeated) vs RAW semantic top-k (no frontier) — then has
//! qwen answer from the recalled beliefs and qwen-judge the answer against a reference. Measures
//! the keystone BEHAVIORALLY: semantic relevance is anti-correlated with currency, so raw recall
//! surfaces the stale-but-close belief; does frontier resolution fix the actual answer?
//!
//! Run: `cargo run -p memory-cli --bin eval-qa` (needs Ollama).

use memory_core::{cosine, Belief, Graph};
use std::path::Path;

struct Q {
    corpus: &'static str,
    question: &'static str,
    expected: &'static str,
}

// Each question's answer flipped over time (verdict-of-a-verdict, or a supersession tip), so the
// CURRENT answer is only right if the superseded/refuted belief is dropped.
const QUESTIONS: &[Q] = &[
    Q {
        corpus: "helix",
        question: "On an empty command-prompt line in Helix, what should the Backspace key do?",
        expected: "Only delete / act as a no-op when empty — it must NOT abort or close the prompt.",
    },
    Q {
        corpus: "sql-abstract",
        question: "Does SQL::Abstract's ORDER BY support NULLS FIRST / NULLS LAST placement control?",
        expected: "No — ORDER BY only supports -asc/-desc; there is no NULLS placement control.",
    },
    Q {
        corpus: "robot-game",
        question: "Are dot-counting visuals (shown on the 2-wrong trigger) sufficient to teach addition/subtraction across difficulty bands?",
        expected: "Yes — dot-counting visuals are sufficient to teach it at any band.",
    },
    Q {
        corpus: "helix",
        question: "Does Helix currently use ad-hoc polling or a formal async hook/event system for LSP completion and signature help?",
        expected: "A formal async hook/event system (the helix-event crate) that replaced the earlier ad-hoc polling.",
    },
    // Pure relevance-vs-currency: the superseded belief is a STRONG semantic match and has no
    // explicit current corrective — only a newer version supersedes it.
    Q {
        corpus: "helix",
        question: "Is Helix a single-crate prototype, or a mature multi-crate editor with a formal architecture?",
        expected: "A mature multi-crate editor (Compositor, Document/Editor split, async hook system) — no longer the early single-crate/single-file prototype.",
    },
    Q {
        corpus: "sql-abstract",
        question: "Does SQL::Abstract build SQL by direct string concatenation, or via an internal abstract query tree (AQT)?",
        expected: "Via an internal abstract query tree (expr -> AQT), adopted from 2019 onward — not direct string concatenation.",
    },
];

const ANSWER_SYS: &str = "Answer the question in ONE sentence using ONLY the supplied beliefs. If \
the beliefs disagree, prefer the one representing the CURRENT state. Reply JSON {\"answer\":\"...\"}.";

const JUDGE_SYS: &str = "Grade whether the candidate answer matches the reference answer IN \
SUBSTANCE — same conclusion, wording may differ; an opposite or contradictory conclusion is NO. \
Reply JSON {\"match\": true|false}.";

fn main() {
    let oll = memory_embed::Ollama::from_env();
    let model = std::env::var("JUDGE_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let k = 5usize;

    println!("{:<13} {:<58} {:>8} {:>4}", "corpus", "question", "frontier", "raw");
    println!("{:-<87}", "");
    let (mut f_ok, mut r_ok) = (0usize, 0usize);

    for q in QUESTIONS {
        let g = Graph::load_dir(&root.join(q.corpus).join("beliefs")).expect("load corpus");
        let content: Vec<&Belief> = g.beliefs.iter().filter(|b| b.relation.is_none()).collect();

        // embeddings, cached per corpus
        let dir = root.join(q.corpus);
        let mut vectors = memory_embed::load_cache(&dir, &oll.model);
        let need: Vec<&Belief> = content.iter().copied().filter(|b| !vectors.contains_key(&b.id)).collect();
        if !need.is_empty() {
            let docs: Vec<String> = need.iter().map(|b| format!("search_document: {}", b.claim)).collect();
            let vs = oll.embed(&docs).expect("embed corpus");
            for (b, v) in need.iter().zip(vs) {
                vectors.insert(b.id.clone(), v);
            }
            memory_embed::save_cache(&dir, &oll.model, &vectors);
        }
        let qv = oll.embed(&[format!("search_query: {}", q.question)]).expect("embed query").remove(0);
        let defeated = g.defeated();

        let verbose = std::env::var("QA_VERBOSE").is_ok();
        // Answer under one config: rank the (optionally frontier-filtered) content by cosine, take
        // top-k, have qwen answer from them, then qwen-judge vs the reference. Returns (correct, answer).
        let run = |frontier: bool| -> (bool, String) {
            let mut cands: Vec<(&Belief, f32)> = content
                .iter()
                .copied()
                .filter(|b| !frontier || !defeated.contains(&b.id))
                .filter_map(|b| vectors.get(&b.id).map(|v| (b, cosine(&qv, v))))
                .collect();
            cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            cands.truncate(k);
            if verbose {
                println!("  [{}] top-{k}:", if frontier { "frontier" } else { "raw" });
                for (b, s) in &cands {
                    println!("     ({s:.2}) {} — {:.80}", b.slug, b.claim);
                }
            }
            let ctx = cands.iter().map(|(b, _)| format!("- {}", b.claim)).collect::<Vec<_>>().join("\n");
            let user = format!("Question: {}\n\nBeliefs:\n{ctx}", q.question);
            let ans = memory_embed::chat_json(&oll.url, &model, ANSWER_SYS, &user)
                .ok()
                .and_then(|v| v.get("answer").and_then(|a| a.as_str()).map(String::from))
                .unwrap_or_default();
            let judge = format!("Question: {}\nReference: {}\nCandidate: {ans}", q.question, q.expected);
            let ok = memory_embed::chat_json(&oll.url, &model, JUDGE_SYS, &judge)
                .ok()
                .and_then(|v| v.get("match").and_then(|m| m.as_bool()))
                .unwrap_or(false);
            (ok, ans)
        };

        if verbose {
            println!("\n## [{}] {}", q.corpus, q.question);
        }
        let (f, fa) = run(true);
        let (r, ra) = run(false);
        if f { f_ok += 1; }
        if r { r_ok += 1; }
        if verbose {
            println!("  frontier {} → {fa}", mark(f));
            println!("  raw      {} → {ra}", mark(r));
            println!("  expected → {}", q.expected);
        } else {
            println!("{:<13} {:<58.58} {:>8} {:>4}", q.corpus, q.question, mark(f), mark(r));
        }
    }

    let n = QUESTIONS.len();
    println!("{:-<87}", "");
    println!("FRONTIER recall: {f_ok}/{n} correct    ·    RAW recall: {r_ok}/{n} correct");
    println!(
        "→ frontier resolution corrected {} answer(s) that raw semantic recall got wrong (stale-but-close).",
        f_ok.saturating_sub(r_ok)
    );
}

fn mark(ok: bool) -> &'static str {
    if ok { "✓" } else { "✗" }
}
