//! memory-mcp — an MCP stdio server exposing two verbs over the belief store:
//!   recall(query, limit?)            -> frontier-resolved beliefs (current only)
//!   remember(claim, refs?, body?)    -> append a new belief to the store
//!
//! This is the dogfood surface: point Claude Code at it and you read/write real memory
//! mid-work. MCP stdio transport = newline-delimited JSON-RPC 2.0, so we hand-roll the
//! loop (one serde_json dep, no SDK). STDOUT is the protocol channel — all logging goes to
//! STDERR.
//!
//! Store dir: $MEMORY_DIR (default ~/.local/share/agentic-memory/beliefs). Point it at a
//! corpus's `beliefs/` to recall over the corpus during dev.

use memory_core::Graph;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SERVER_NAME: &str = "agentic-memory";
const SERVER_VERSION: &str = "0.1.0";
const DEFAULT_PROTOCOL: &str = "2024-11-05";

fn main() {
    let dir = store_dir();
    let _ = std::fs::create_dir_all(&dir);
    eprintln!("[memory-mcp] store = {}", dir.display());

    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[memory-mcp] bad json: {e}");
                continue;
            }
        };
        // Notifications have no `id` and get no response.
        let id = req.get("id").cloned();
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = req.get("params").cloned().unwrap_or(Value::Null);

        let response = match handle(method, &params, &dir) {
            Ok(Some(result)) => id.map(|id| json!({"jsonrpc":"2.0","id":id,"result":result})),
            Ok(None) => None, // a notification we handled silently
            Err(msg) => id.map(|id| {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32603,"message":msg}})
            }),
        };
        if let Some(resp) = response {
            let _ = writeln!(out, "{resp}");
            let _ = out.flush();
        }
    }
}

/// Returns Ok(Some(result)) for a request, Ok(None) for a handled notification.
fn handle(method: &str, params: &Value, dir: &PathBuf) -> Result<Option<Value>, String> {
    match method {
        "initialize" => {
            let protocol = params
                .get("protocolVersion")
                .and_then(|v| v.as_str())
                .unwrap_or(DEFAULT_PROTOCOL);
            Ok(Some(json!({
                "protocolVersion": protocol,
                "capabilities": { "tools": {} },
                "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION }
            })))
        }
        "notifications/initialized" => Ok(None),
        "ping" => Ok(Some(json!({}))),
        "tools/list" => Ok(Some(json!({ "tools": tool_specs() }))),
        "tools/call" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(Value::Null);
            let text = match name {
                "recall" => recall(&args, dir),
                "remember" => remember(&args, dir),
                other => return Err(format!("unknown tool: {other}")),
            };
            Ok(Some(json!({ "content": [ { "type": "text", "text": text } ] })))
        }
        other => Err(format!("method not found: {other}")),
    }
}

fn tool_specs() -> Value {
    json!([
        {
            "name": "recall",
            "description": "Recall what you already know about something from long-term memory. \
                            Returns only CURRENT beliefs — superseded and refuted ones are dropped \
                            (frontier-resolved), so you won't get an out-of-date or reverted answer.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "what you want to recall" },
                    "limit": { "type": "integer", "description": "max beliefs to return (default 10)" }
                },
                "required": ["query"]
            }
        },
        {
            "name": "remember",
            "description": "Save a new belief (a thing you learned) to long-term memory. State a \
                            single clear proposition; optionally cite refs (files/commits). Don't \
                            try to manage how it relates to other memories — the memory layer links it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "claim": { "type": "string", "description": "the proposition to remember" },
                    "refs": { "type": "array", "items": { "type": "string" },
                              "description": "grounding: files, commits, urls" },
                    "body": { "type": "string", "description": "optional extra context / justification" }
                },
                "required": ["claim"]
            }
        }
    ])
}

fn recall(args: &Value, dir: &PathBuf) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    if query.trim().is_empty() {
        return "(no query)".into();
    }
    let g = match Graph::load_dir(dir) {
        Ok(g) => g,
        Err(_) => return "No memories yet.".into(),
    };
    if g.beliefs.is_empty() {
        return "No memories yet.".into();
    }
    let defeated = g.defeated();
    // Lexical stand-in for the semantic lens (embeddings come next). Frontier-filtered.
    let mut hits: Vec<&memory_core::Belief> = g
        .beliefs
        .iter()
        .filter(|b| !defeated.contains(&b.id))
        .filter(|b| b.slug.to_lowercase().contains(&query) || b.claim.to_lowercase().contains(&query))
        .collect();
    hits.sort_by(|a, b| b.source_weight.partial_cmp(&a.source_weight).unwrap_or(std::cmp::Ordering::Equal));
    if hits.is_empty() {
        return format!("Nothing current recalled for \"{query}\".");
    }
    let mut s = format!("Recalled {} current belief(s):\n", hits.len().min(limit));
    for b in hits.iter().take(limit) {
        s.push_str(&format!("\n• [{}] {}", b.slug, b.claim));
    }
    s
}

fn remember(args: &Value, dir: &PathBuf) -> String {
    let claim = args.get("claim").and_then(|v| v.as_str()).unwrap_or("").trim();
    if claim.is_empty() {
        return "remember needs a `claim`.".into();
    }
    let body = args.get("body").and_then(|v| v.as_str()).unwrap_or("");
    let refs: Vec<String> = args
        .get("refs")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let txn = iso_now();
    let id = bid(&format!("{claim}|{txn}"));
    let slug = slugify(claim);
    let one_line = claim.replace('\n', " ");

    let mut fm = String::new();
    fm.push_str("---\n");
    fm.push_str(&format!("id: {id}\n"));
    fm.push_str(&format!("slug: {slug}\n"));
    fm.push_str("claim:\n  kind: text\n  text: >-\n");
    fm.push_str(&format!("    {one_line}\n"));
    fm.push_str("author:\n  kind: agent\n  id: claude\n");
    fm.push_str("provenance:\n");
    fm.push_str(&format!("  txn_time: {txn}\n"));
    fm.push_str("  valid_time: null\n");
    fm.push_str("  source:\n    kind: conversation\n    session: mcp\n    turn: 0\n");
    if refs.is_empty() {
        fm.push_str("  refs: []\n");
    } else {
        fm.push_str("  refs:\n");
        for r in &refs {
            fm.push_str(&format!("    - {r}\n"));
        }
    }
    fm.push_str("  derived_from: []\n");
    fm.push_str("confidence:\n  directness: stated\n  observation_count: 1\n  source_weight: 0.8\n  asserted: null\n");
    fm.push_str("edges: []\n");
    fm.push_str("coord: null\n");
    fm.push_str("---\n\n");
    fm.push_str(if body.is_empty() { "(remembered via MCP)" } else { body });
    fm.push('\n');

    let path = dir.join(format!("{id}.md"));
    match std::fs::write(&path, fm) {
        Ok(_) => format!("Remembered as {id} ([{slug}]). The memory layer will link it."),
        Err(e) => format!("Failed to write memory: {e}"),
    }
}

// --- helpers -------------------------------------------------------------------------

fn store_dir() -> PathBuf {
    if let Ok(d) = std::env::var("MEMORY_DIR") {
        return PathBuf::from(d);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/agentic-memory/beliefs")
}

/// Placeholder content id (low 48 bits of std SipHash). Deterministic per process; the real
/// scheme is sha256(observation)[:12] — fine to reconcile later, ids are not the durable part.
fn bid(seed: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    seed.hash(&mut h);
    format!("b_{:012x}", h.finish() & 0xffff_ffff_ffff)
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

fn iso_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_from_days((secs / 86400) as i64);
    let sod = secs % 86400;
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

/// Howard Hinnant's civil-from-days: days since 1970-01-01 -> (year, month, day).
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}
