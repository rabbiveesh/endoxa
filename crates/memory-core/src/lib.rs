//! memory-core — the deterministic heart of the belief-memory system.
//!
//! Pure, no-LLM, no-network: belief model + loader + **frontier resolver** (design P0).
//! The frontier resolver is the keystone the usability study identified — semantic
//! relevance is anti-correlated with currency, so the most lexically-relevant belief is
//! often the *superseded/refuted* one. Resolving the frontier is what stops recall from
//! returning the inverted answer.
//!
//! First cut is deliberately hacky (zero deps, hand-rolled parser, in-memory graph) — but
//! the L0 belief *format* it reads is the durable contract; everything here above L0 is
//! regenerable.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub type Id = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Supports,
    Attacks,
    Supersedes,
    DerivedFrom,
    Refines,
    Adjudicates,
}

impl EdgeKind {
    fn parse(s: &str) -> Option<EdgeKind> {
        Some(match s {
            "supports" => EdgeKind::Supports,
            "attacks" => EdgeKind::Attacks,
            "supersedes" => EdgeKind::Supersedes,
            "derived_from" => EdgeKind::DerivedFrom,
            "refines" => EdgeKind::Refines,
            "adjudicates" => EdgeKind::Adjudicates,
            _ => return None,
        })
    }

    /// Edges that DEFEAT their target on a world's frontier (revision + verdict).
    /// `attacks` alone is a *surfaced* conflict, NOT a defeat — open conflicts keep both
    /// sides live; only an `adjudicates` verdict or a `supersedes` defeats.
    pub fn is_defeating(self) -> bool {
        matches!(self, EdgeKind::Supersedes | EdgeKind::Adjudicates)
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub kind: EdgeKind,
    pub target: Id,
}

#[derive(Debug, Clone, Default)]
pub struct Belief {
    pub id: Id,
    pub slug: String,
    pub claim: String,
    pub project_scope: bool,
    pub author_kind: String,
    pub directness: String,
    pub source_weight: f32,
    pub asserted: Option<f32>,
    pub edges: Vec<Edge>,
}

impl Belief {
    /// Parse one belief markdown file (YAML-ish frontmatter + body). Hand-rolled and
    /// zero-dep: the frontmatter is machine-generated and regular (see
    /// `corpus/_belief_lib.py`), so a small stateful line parser is enough.
    pub fn parse(text: &str) -> Option<Belief> {
        // isolate the frontmatter (between the first two `---` lines)
        let mut fm: Vec<&str> = Vec::new();
        let mut started = false;
        for line in text.lines() {
            if line.trim_end() == "---" {
                if !started {
                    started = true;
                } else {
                    break;
                }
                continue;
            }
            if started {
                fm.push(line);
            }
        }
        if fm.is_empty() {
            return None;
        }

        let mut b = Belief::default();
        let mut section = String::new();
        let mut collecting_claim = false;
        let mut pending_edge_kind: Option<EdgeKind> = None;

        for line in fm {
            let indent = line.len() - line.trim_start().len();
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // folded claim scalar: keep appending the indented continuation lines
            if collecting_claim {
                if indent >= 4 {
                    if !b.claim.is_empty() {
                        b.claim.push(' ');
                    }
                    b.claim.push_str(trimmed);
                    continue;
                }
                collecting_claim = false; // fall through to process this line normally
            }

            if indent == 0 {
                let key = trimmed.split(':').next().unwrap_or("");
                let val = trimmed.splitn(2, ':').nth(1).unwrap_or("").trim();
                match key {
                    "id" => b.id = val.to_string(),
                    "slug" => b.slug = val.to_string(),
                    "kind" => {
                        if val == "project-scope" {
                            b.project_scope = true;
                        }
                    }
                    other => section = other.to_string(),
                }
            } else {
                match section.as_str() {
                    "claim" => {
                        if trimmed.starts_with("text:") {
                            collecting_claim = true;
                            b.claim.clear();
                        }
                    }
                    "author" => {
                        if let Some(v) = trimmed.strip_prefix("kind:") {
                            b.author_kind = v.trim().to_string();
                        }
                    }
                    "confidence" => {
                        if let Some(v) = trimmed.strip_prefix("directness:") {
                            b.directness = v.trim().to_string();
                        } else if let Some(v) = trimmed.strip_prefix("source_weight:") {
                            b.source_weight = v.trim().parse().unwrap_or(0.0);
                        } else if let Some(v) = trimmed.strip_prefix("asserted:") {
                            let v = v.trim();
                            b.asserted = if v == "null" { None } else { v.parse().ok() };
                        }
                    }
                    "edges" => {
                        if let Some(v) = trimmed.strip_prefix("- kind:") {
                            pending_edge_kind = EdgeKind::parse(v.trim());
                        } else if let Some(v) = trimmed.strip_prefix("target:") {
                            if let Some(k) = pending_edge_kind.take() {
                                let target =
                                    v.trim().split_whitespace().next().unwrap_or("").to_string();
                                if !target.is_empty() {
                                    b.edges.push(Edge { kind: k, target });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if b.id.is_empty() {
            return None;
        }
        Some(b)
    }
}

/// An in-memory belief graph for one corpus / world `main`.
pub struct Graph {
    pub beliefs: Vec<Belief>,
    by_id: HashMap<Id, usize>,
    by_slug: HashMap<String, usize>,
}

impl Graph {
    pub fn from_beliefs(beliefs: Vec<Belief>) -> Graph {
        let mut by_id = HashMap::new();
        let mut by_slug = HashMap::new();
        for (i, b) in beliefs.iter().enumerate() {
            by_id.insert(b.id.clone(), i);
            by_slug.insert(b.slug.clone(), i);
        }
        Graph {
            beliefs,
            by_id,
            by_slug,
        }
    }

    /// Load every `*.md` belief from a `corpus/<name>/beliefs/` directory.
    pub fn load_dir(dir: &Path) -> std::io::Result<Graph> {
        let mut beliefs = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let text = fs::read_to_string(&path)?;
                if let Some(b) = Belief::parse(&text) {
                    beliefs.push(b);
                }
            }
        }
        beliefs.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(Graph::from_beliefs(beliefs))
    }

    pub fn get(&self, slug: &str) -> Option<&Belief> {
        self.by_slug.get(slug).map(|&i| &self.beliefs[i])
    }

    /// **Frontier resolution (design P0).** Returns the set of belief ids DEFEATED on the
    /// `main` frontier — i.e. superseded or adjudicated-against by a belief that is itself
    /// not defeated.
    ///
    /// It is frontier-relative and **non-monotonic**: a later verdict that defeats an
    /// earlier verdict thereby REINSTATES the earlier verdict's target (verdict-of-a-
    /// verdict). Computed as an alternating fixpoint — each round recomputes the defeated
    /// set from scratch using only the still-undefeated attackers, so reinstatement falls
    /// out naturally. The corpus's defeat graph is time-acyclic (newer defeats older), so
    /// it converges; the iteration cap is a backstop.
    pub fn defeated(&self) -> HashSet<Id> {
        let mut defeated: HashSet<Id> = HashSet::new();
        for _ in 0..(self.beliefs.len() + 5) {
            let mut next: HashSet<Id> = HashSet::new();
            for b in &self.beliefs {
                if defeated.contains(&b.id) {
                    continue; // a defeated belief defeats nothing
                }
                for e in &b.edges {
                    if e.kind.is_defeating() && self.by_id.contains_key(&e.target) {
                        next.insert(e.target.clone());
                    }
                }
            }
            if next == defeated {
                return defeated;
            }
            defeated = next;
        }
        defeated
    }

    /// Beliefs that live on the current `main` frontier (not defeated).
    pub fn current(&self) -> Vec<&Belief> {
        let d = self.defeated();
        self.beliefs.iter().filter(|b| !d.contains(&b.id)).collect()
    }
}
