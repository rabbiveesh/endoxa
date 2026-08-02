//! `worlds.json` — the live worlds surface (design §5, V3, N6).
//!
//! A world is a NAMED SET OF HEADS over the shared belief DAG: an `assumption` (its identity)
//! plus a `suppress` list (belief refs whose defeating edges are dropped before frontier
//! resolution). The file format is the corpus format, verbatim, so a corpus dir works as a
//! store unmodified:
//!
//! ```json
//! { "worlds": { "main": { "default": true, "assumption": "..." },
//!               "dissent": { "assumption": "...", "suppress": ["slug"] } },
//!   "reduction_fixtures": [ { "query": "...", "neighborhood": ["slug"],
//!                             "expected_by_world": { "main": "...", "dissent": "..." } } ] }
//! ```
//!
//! Location: `<store>/worlds.json`, else `<store>/../worlds.json` (the corpus layout, where
//! beliefs live in `beliefs/` and worlds.json sits beside it). No file → only `main` exists.

use memory_core::World;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// A gold reduction fixture: the corpus's divergent-answer target for the L3 reducer.
#[allow(dead_code)] // consumed by `eval-worlds`; `mem` compiles the module too but doesn't read it
pub struct Fixture {
    pub query: String,
    pub neighborhood: Vec<String>,
    /// (world name, expected consensus text) — order follows the JSON object.
    pub expected_by_world: Vec<(String, String)>,
}

pub struct WorldFile {
    // this module is compiled into BOTH `mem` and `eval-worlds`; each uses a different subset
    #[allow(dead_code)]
    pub path: PathBuf,
    /// Default world first, then alphabetical.
    pub worlds: Vec<World>,
    #[allow(dead_code)]
    pub fixtures: Vec<Fixture>,
}

impl WorldFile {
    pub fn get(&self, name: &str) -> Option<&World> {
        self.worlds.iter().find(|w| w.name == name)
    }

    /// The default world ("main" wins absent an explicit `default: true`).
    pub fn default_world(&self) -> Option<&World> {
        self.worlds
            .iter()
            .find(|w| w.is_default)
            .or_else(|| self.get("main"))
            .or_else(|| self.worlds.first())
    }
}

fn find_file(store: &Path) -> Option<PathBuf> {
    let own = store.join("worlds.json");
    if own.is_file() {
        return Some(own);
    }
    let beside = store.parent()?.join("worlds.json");
    beside.is_file().then_some(beside)
}

/// Load the store's worlds file, if any. `None` = no worlds authored (main-only reality).
pub fn load(store: &Path) -> Option<WorldFile> {
    let path = find_file(store)?;
    let text = std::fs::read_to_string(&path).ok()?;
    let (worlds, fixtures) = parse(&text)?;
    Some(WorldFile { path, worlds, fixtures })
}

pub fn parse(text: &str) -> Option<(Vec<World>, Vec<Fixture>)> {
    let v: Value = serde_json::from_str(text).ok()?;
    let mut worlds: Vec<World> = v
        .get("worlds")?
        .as_object()?
        .iter()
        .map(|(name, cfg)| World {
            name: name.clone(),
            assumption: cfg.get("assumption").and_then(Value::as_str).unwrap_or("").to_string(),
            suppress: cfg
                .get("suppress")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect())
                .unwrap_or_default(),
            is_default: cfg.get("default").and_then(Value::as_bool).unwrap_or(false),
        })
        .collect();
    worlds.sort_by(|a, b| b.is_default.cmp(&a.is_default).then(a.name.cmp(&b.name)));

    let fixtures = v
        .get("reduction_fixtures")
        .and_then(Value::as_array)
        .map(|fs| {
            fs.iter()
                .filter_map(|f| {
                    Some(Fixture {
                        query: f.get("query")?.as_str()?.to_string(),
                        neighborhood: f
                            .get("neighborhood")
                            .and_then(Value::as_array)
                            .map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect())
                            .unwrap_or_default(),
                        expected_by_world: f
                            .get("expected_by_world")
                            .and_then(Value::as_object)
                            .map(|m| {
                                m.iter()
                                    .filter_map(|(w, t)| Some((w.clone(), t.as_str()?.to_string())))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    Some((worlds, fixtures))
}
