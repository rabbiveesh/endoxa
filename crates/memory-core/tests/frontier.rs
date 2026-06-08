//! Frontier-resolution tests against the real corpus — the first green checks that turn
//! the thesis from a doc into a running thing.

use memory_core::{Belief, EdgeKind, Graph, Relation};
use std::path::Path;

fn belief(id: &str, slug: &str) -> Belief {
    Belief { id: id.into(), slug: slug.into(), ..Default::default() }
}

fn edge(id: &str, kind: EdgeKind, subject: &str, object: &str) -> Belief {
    Belief {
        id: id.into(),
        slug: format!("rel-{id}"),
        relation: Some(Relation { kind, subject: subject.into(), object: object.into() }),
        ..Default::default()
    }
}

/// `mem forget` writes a self-anchored `retracts` edge; it must defeat its target so recall
/// drops it (while the file stays on disk — that's the CLI's job, not the resolver's).
#[test]
fn retracts_edge_defeats_its_target() {
    let g = Graph::from_beliefs(vec![
        belief("t", "target"),
        edge("r", EdgeKind::Retracts, "t", "t"),
    ]);
    assert!(g.defeated().contains("t"), "a retracts edge must defeat its target");
    let current: Vec<&str> = g.current().iter().map(|b| b.slug.as_str()).collect();
    assert!(!current.contains(&"target"), "a forgotten belief must not be current");
}

/// Forget is reversible at the resolver level: defeat the retraction itself and the target
/// reinstates (the same non-monotonic property supersession relies on).
#[test]
fn defeating_a_retraction_reinstates_the_target() {
    let g = Graph::from_beliefs(vec![
        belief("t", "target"),
        edge("r", EdgeKind::Retracts, "t", "t"),
        edge("u", EdgeKind::Supersedes, "u", "r"), // supersede the retraction edge-belief
    ]);
    let defeated = g.defeated();
    assert!(defeated.contains("r"), "the retraction is itself defeated");
    assert!(!defeated.contains("t"), "the target reinstates once its retraction is defeated");
}

fn load(corpus: &str) -> Graph {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../corpus")
        .join(corpus)
        .join("beliefs");
    Graph::load_dir(&dir).unwrap_or_else(|e| panic!("load {corpus}: {e}"))
}

/// THE first green check (from the usability study): a current-state query must NOT surface
/// the superseded belief, even though it is lexically the strongest match.
#[test]
fn composr_current_state_drops_the_superseded_belief() {
    let g = load("composr");
    let defeated = g.defeated();

    let native = g.get("native-post-autoload").expect("native belief present");
    let delegate = g
        .get("delegate-all-post-autoload")
        .expect("delegate belief present");

    // the superseded belief is defeated on the current frontier...
    assert!(
        defeated.contains(&delegate.id),
        "delegate-all-post-autoload should be defeated (superseded by native-post-autoload)"
    );
    // ...and the superseding belief is current.
    assert!(
        !defeated.contains(&native.id),
        "native-post-autoload should be on the current frontier"
    );

    let current: Vec<&str> = g.current().iter().map(|b| b.slug.as_str()).collect();
    assert!(current.contains(&"native-post-autoload"));
    assert!(
        !current.contains(&"delegate-all-post-autoload"),
        "recall must not surface the superseded belief as current"
    );
}

/// The hard case the whole design hinges on: a verdict that defeats an earlier verdict
/// REINSTATES the original (non-monotonic, frontier-relative refutation).
#[test]
fn helix_verdict_of_a_verdict_reinstates_the_original() {
    let g = load("helix");
    let defeated = g.defeated();

    let original = g.get("r3-backspace-original").expect("original");
    let v1 = g.get("r3-backspace-verdict1").expect("verdict1");
    let v2 = g.get("r3-backspace-verdict2").expect("verdict2");

    assert!(
        defeated.contains(&v1.id),
        "verdict1 should be defeated by verdict2"
    );
    assert!(!defeated.contains(&v2.id), "verdict2 stands");
    assert!(
        !defeated.contains(&original.id),
        "original is REINSTATED — the verdict that defeated it is itself defeated"
    );
}

/// An open (un-adjudicated) `attacks` is a *surfaced conflict*, not a defeat: both sides
/// stay current.
#[test]
fn open_conflicts_keep_both_sides_live() {
    let g = load("sql-abstract");
    let defeated = g.defeated();
    for slug in ["r3-inject-guard-complete", "r3-inject-guard-compat"] {
        let b = g.get(slug).unwrap_or_else(|| panic!("{slug} present"));
        assert!(
            !defeated.contains(&b.id),
            "{slug} is in an OPEN conflict (attacks, no adjudicates) and must stay live"
        );
    }
}
