//! Ollama-backed embeddings + a tiny on-disk vector cache + **pluggable chat providers**.
//! Shells out (curl for ollama, the `claude` CLI for Claude Code) — no HTTP crate.
//! nomic-embed-text is asymmetric: callers prefix docs with `search_document: ` and
//! queries with `search_query: `.
//!
//! **Chat is provider-pluggable through the model ref alone**: every judgment/reduction call
//! goes through [`chat_json`], and every caller takes its model string from env
//! (`ASK_MODEL`/`JUDGE_MODEL`/`TIER2_MODEL`/…), so a provider switch is just
//! `JUDGE_MODEL=claude:sonnet` — no call-site changes anywhere. Embeddings stay ollama-only
//! (Claude Code exposes no embedding endpoint).

use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Ollama {
    pub url: String,
    pub model: String,
}

impl Ollama {
    pub fn from_env() -> Ollama {
        Ollama {
            url: std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into()),
            model: std::env::var("EMBED_MODEL").unwrap_or_else(|_| "nomic-embed-text".into()),
        }
    }

    /// Embed a batch in one call. Errors (ollama down, model not pulled, no curl) are returned
    /// as strings so callers can fall back to lexical.
    pub fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, String> {
        if inputs.is_empty() {
            return Ok(vec![]);
        }
        let body = json!({ "model": self.model, "input": inputs }).to_string();
        let mut child = Command::new("curl")
            .args([
                "-s", "--max-time", "180", "-X", "POST",
                &format!("{}/api/embed", self.url),
                "-H", "Content-Type: application/json", "--data-binary", "@-",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("curl spawn failed (is curl installed?): {e}"))?;
        child
            .stdin
            .take()
            .ok_or("no curl stdin")?
            .write_all(body.as_bytes())
            .map_err(|e| e.to_string())?;
        let out = child.wait_with_output().map_err(|e| e.to_string())?;
        if out.stdout.is_empty() {
            return Err("empty response — is `ollama serve` running on 11434?".into());
        }
        let v: Value =
            serde_json::from_slice(&out.stdout).map_err(|e| format!("bad ollama response: {e}"))?;
        if let Some(err) = v.get("error").and_then(|e| e.as_str()) {
            return Err(format!("ollama error: {err} (try `ollama pull {}`)", self.model));
        }
        let arr = v
            .get("embeddings")
            .and_then(|e| e.as_array())
            .ok_or("ollama response had no `embeddings`")?;
        let mut vecs = Vec::with_capacity(arr.len());
        for row in arr {
            let r = row.as_array().ok_or("embedding row was not an array")?;
            vecs.push(r.iter().map(|x| x.as_f64().unwrap_or(0.0) as f32).collect());
        }
        Ok(vecs)
    }
}

// --- pluggable chat providers ------------------------------------------------------------

/// Where a chat/judgment call goes — THE provider seam. Parsed from the one string every
/// consumer already threads from env, so providers are switchable without touching a call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatBackend {
    /// Ollama over HTTP (curl), `format: json`, temperature 0 — the historical default.
    Ollama { model: String },
    /// The Claude Code CLI (`claude -p`), tools disabled, custom system prompt, JSON envelope.
    /// `None` model = the CLI's configured default.
    ClaudeCode { model: Option<String> },
}

/// Parse a model ref: `<provider>:<model>` with known provider prefixes, else ollama.
///   - `claude` / `claude:<m>` / `claude-code[:<m>]` → Claude Code (`<m>` e.g. sonnet/opus/haiku)
///   - `ollama:<m>` → ollama, explicitly
///   - anything else → ollama with the ref verbatim (bare ollama tags like `qwen2.5:7b`
///     contain `:`, so ONLY the known prefixes above are treated as providers)
pub fn parse_model_ref(model: &str) -> ChatBackend {
    match model.split_once(':') {
        Some(("claude" | "claude-code", m)) => {
            ChatBackend::ClaudeCode { model: (!m.is_empty()).then(|| m.to_string()) }
        }
        Some(("ollama", m)) => ChatBackend::Ollama { model: m.to_string() },
        None if model == "claude" || model == "claude-code" => ChatBackend::ClaudeCode { model: None },
        _ => ChatBackend::Ollama { model: model.to_string() },
    }
}

/// One-shot structured chat, provider-dispatched on the model ref (see [`parse_model_ref`]).
/// Returns the model's parsed JSON object; errors (provider down, model not pulled, non-JSON)
/// come back as strings so callers degrade gracefully. `url` is the ollama base URL — ignored
/// by the Claude Code backend.
pub fn chat_json(url: &str, model: &str, system: &str, user: &str) -> Result<Value, String> {
    match parse_model_ref(model) {
        ChatBackend::ClaudeCode { model } => claude_chat_json(model.as_deref(), system, user),
        ChatBackend::Ollama { model } => ollama_chat_json(url, &model, system, user),
    }
}

/// Chat via the **Claude Code CLI** — the frontier-agent-as-model-provider path (the N4
/// "maximum judge" made ambient). Design choices, each load-bearing:
///   - `--tools ""` + `--max-turns 1`: a pure model call — the judge must not browse the repo;
///   - `--system-prompt` REPLACES the CLI's default agent prompt (and with it the project
///     context), and the child runs from a neutral temp cwd so the surrounding repo's
///     CLAUDE.md can't leak into a judgment;
///   - prompt via stdin (judgment prompts exceed sane argv);
///   - `--output-format json` envelope → `result` text → tolerant JSON extraction (Claude has
///     no `format: json` forcing, so fenced/prefixed JSON is normalized by `extract_json`).
/// `CLAUDE_BIN` overrides the binary; `CLAUDE_CHAT_ARGS` appends raw extra flags (e.g.
/// `--effort low` or `--max-budget-usd 0.5`), whitespace-split.
pub fn claude_chat_json(model: Option<&str>, system: &str, user: &str) -> Result<Value, String> {
    let bin = std::env::var("CLAUDE_BIN").unwrap_or_else(|_| "claude".into());
    let mut cmd = Command::new(&bin);
    cmd.args(["-p", "--output-format", "json", "--max-turns", "1", "--tools", ""]);
    cmd.args(["--system-prompt", system]);
    if let Some(m) = model {
        cmd.args(["--model", m]);
    }
    if let Ok(extra) = std::env::var("CLAUDE_CHAT_ARGS") {
        cmd.args(extra.split_whitespace());
    }
    cmd.current_dir(std::env::temp_dir());
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("`{bin}` spawn failed (is Claude Code installed?): {e}"))?;
    child.stdin.take().ok_or("no claude stdin")?.write_all(user.as_bytes()).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() || out.stdout.is_empty() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("claude CLI failed: {}", err.trim().chars().take(300).collect::<String>()));
    }
    let v: Value = serde_json::from_slice(&out.stdout).map_err(|e| format!("bad claude envelope: {e}"))?;
    if v.get("is_error").and_then(Value::as_bool).unwrap_or(false) {
        return Err(format!("claude error: {}", v.get("result").and_then(Value::as_str).unwrap_or("?")));
    }
    let content = v
        .get("result")
        .and_then(Value::as_str)
        .ok_or("no `result` in claude envelope")?;
    extract_json(content)
}

/// Tolerant "the model was asked for JSON" parsing: direct → fenced (```json … ```) →
/// outermost `{…}` slice. Providers without hard JSON forcing (Claude) need this; harmless
/// for those with it (ollama).
pub fn extract_json(text: &str) -> Result<Value, String> {
    let t = text.trim();
    if let Ok(v) = serde_json::from_str(t) {
        return Ok(v);
    }
    let stripped = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .map(|s| s.strip_suffix("```").unwrap_or(s).trim())
        .unwrap_or(t);
    if let Ok(v) = serde_json::from_str(stripped) {
        return Ok(v);
    }
    if let (Some(a), Some(b)) = (t.find('{'), t.rfind('}')) {
        if b > a {
            if let Ok(v) = serde_json::from_str(&t[a..=b]) {
                return Ok(v);
            }
        }
    }
    Err(format!("model returned non-JSON: {}", t.chars().take(200).collect::<String>()))
}

/// One-shot structured chat against an Ollama generation model (e.g. qwen2.5). Forces JSON
/// output (`format: json`, temperature 0) and returns the parsed object. Used by the judgment
/// linker. Errors (ollama down, model not pulled, non-JSON) come back as strings.
fn ollama_chat_json(url: &str, model: &str, system: &str, user: &str) -> Result<Value, String> {
    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": user }
        ],
        "stream": false,
        "format": "json",
        "options": { "temperature": 0 }
    })
    .to_string();
    let mut child = Command::new("curl")
        .args([
            "-s", "--max-time", "120", "-X", "POST",
            &format!("{url}/api/chat"),
            "-H", "Content-Type: application/json", "--data-binary", "@-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("curl spawn failed: {e}"))?;
    child.stdin.take().ok_or("no curl stdin")?.write_all(body.as_bytes()).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.stdout.is_empty() {
        return Err("empty response — is `ollama serve` running?".into());
    }
    let v: Value = serde_json::from_slice(&out.stdout).map_err(|e| format!("bad ollama response: {e}"))?;
    if let Some(err) = v.get("error").and_then(|e| e.as_str()) {
        return Err(format!("ollama error: {err} (try `ollama pull {model}`)"));
    }
    let content = v
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .ok_or("no message.content in chat response")?;
    serde_json::from_str(content).map_err(|e| format!("judge returned non-JSON ({e}): {content}"))
}

/// Load the vector cache. Fast path is the **binary sidecar** `.embeddings.bin` (ROADMAP
/// scale-path step 1: the measured cold-start bottleneck is the embeddings-JSON *parse*, not
/// embed compute); `.embeddings.json` stays the durable, git-committable form (corpora ship
/// it) and the authority whenever it is newer than the sidecar — in which case the sidecar is
/// transparently rebuilt. Both are derived caches over L0; deleting either is always safe.
pub fn load_cache(dir: &Path, model: &str) -> HashMap<String, Vec<f32>> {
    let json_path = dir.join(".embeddings.json");
    let bin_path = dir.join(".embeddings.bin");
    let mtime = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    if let (Some(bt), jt) = (mtime(&bin_path), mtime(&json_path)) {
        if jt.map_or(true, |jt| bt >= jt) {
            if let Some(map) = load_bin(&bin_path, model) {
                return map;
            }
        }
    }
    let map = load_json(&json_path, model);
    if !map.is_empty() {
        let _ = save_bin(&bin_path, model, &map); // migrate: next load takes the fast path
    }
    map
}

fn load_json(path: &Path, model: &str) -> HashMap<String, Vec<f32>> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return HashMap::new();
    };
    if v.get("model").and_then(|m| m.as_str()) != Some(model) {
        return HashMap::new();
    }
    let mut map = HashMap::new();
    if let Some(obj) = v.get("vectors").and_then(|x| x.as_object()) {
        for (k, val) in obj {
            if let Some(arr) = val.as_array() {
                map.insert(k.clone(), arr.iter().map(|x| x.as_f64().unwrap_or(0.0) as f32).collect());
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_refs_route_to_the_right_provider() {
        // bare refs stay ollama — including tags that CONTAIN ':' (the back-compat trap)
        assert_eq!(parse_model_ref("qwen2.5:7b"), ChatBackend::Ollama { model: "qwen2.5:7b".into() });
        assert_eq!(parse_model_ref("gemma2:9b"), ChatBackend::Ollama { model: "gemma2:9b".into() });
        // explicit ollama prefix unwraps
        assert_eq!(parse_model_ref("ollama:qwen2.5:7b"), ChatBackend::Ollama { model: "qwen2.5:7b".into() });
        // claude, with and without a model
        assert_eq!(parse_model_ref("claude"), ChatBackend::ClaudeCode { model: None });
        assert_eq!(parse_model_ref("claude:sonnet"), ChatBackend::ClaudeCode { model: Some("sonnet".into()) });
        assert_eq!(parse_model_ref("claude-code:opus"), ChatBackend::ClaudeCode { model: Some("opus".into()) });
        assert_eq!(parse_model_ref("claude-code"), ChatBackend::ClaudeCode { model: None });
        // a full model id rides through the claude prefix untouched
        assert_eq!(
            parse_model_ref("claude:claude-haiku-4-5-20251001"),
            ChatBackend::ClaudeCode { model: Some("claude-haiku-4-5-20251001".into()) }
        );
    }

    fn tmpdir(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("memvec-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn sample() -> HashMap<String, Vec<f32>> {
        let mut m = HashMap::new();
        m.insert("b_aaa".to_string(), vec![0.25f32, -1.5, 3.0]);
        m.insert("b_bbb".to_string(), vec![0.0f32; 768]);
        m
    }

    #[test]
    fn sidecar_round_trips_and_is_preferred() {
        let d = tmpdir("roundtrip");
        save_cache(&d, "nomic-embed-text", &sample());
        assert!(d.join(".embeddings.bin").is_file(), "sidecar written");
        // bin path used (delete json to prove it): identical content back
        std::fs::remove_file(d.join(".embeddings.json")).unwrap();
        let back = load_cache(&d, "nomic-embed-text");
        assert_eq!(back, sample(), "binary round-trip is exact (f32 bit-precise)");
        // wrong model → cache miss, never wrong vectors
        assert!(load_cache(&d, "other-model").is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn newer_json_wins_and_rebuilds_the_sidecar() {
        let d = tmpdir("freshness");
        save_cache(&d, "m", &sample());
        // hand-edit the JSON afterwards (the committed corpus-cache workflow) — json is now newer
        std::thread::sleep(std::time::Duration::from_millis(20));
        let doc = json!({ "model": "m", "vectors": { "b_new": [1.0, 2.0] } });
        std::fs::write(d.join(".embeddings.json"), doc.to_string()).unwrap();
        let back = load_cache(&d, "m");
        assert_eq!(back.len(), 1);
        assert_eq!(back["b_new"], vec![1.0f32, 2.0]);
        // and the sidecar was rebuilt from it: json gone → bin still serves the new content
        std::fs::remove_file(d.join(".embeddings.json")).unwrap();
        assert_eq!(load_cache(&d, "m")["b_new"], vec![1.0f32, 2.0]);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn corrupt_sidecar_falls_back_to_json() {
        let d = tmpdir("corrupt");
        save_cache(&d, "m", &sample());
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(d.join(".embeddings.bin"), b"MEMVEC1\0garbage").unwrap(); // truncated
        let back = load_cache(&d, "m");
        assert_eq!(back, sample(), "malformed sidecar → json authority, no panic");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn extract_json_tolerates_provider_quirks() {
        // clean
        assert_eq!(extract_json("{\"k\": 1}").unwrap()["k"], 1);
        // fenced (Claude's usual shape without JSON forcing)
        assert_eq!(extract_json("```json\n{\"kind\": \"attacks\"}\n```").unwrap()["kind"], "attacks");
        assert_eq!(extract_json("```\n{\"k\": true}\n```").unwrap()["k"], true);
        // prose-wrapped
        assert_eq!(extract_json("Here you go:\n{\"a\": [1,2]}\nHope that helps!").unwrap()["a"][0], 1);
        // garbage errors, not panics
        assert!(extract_json("no json here").is_err());
    }
}

/// Save the vector cache: JSON (durable/committable) + the binary sidecar (fast reload).
pub fn save_cache(dir: &Path, model: &str, cache: &HashMap<String, Vec<f32>>) {
    let vectors: serde_json::Map<String, Value> =
        cache.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let doc = json!({ "model": model, "vectors": vectors });
    let _ = std::fs::write(dir.join(".embeddings.json"), doc.to_string());
    let _ = save_bin(&dir.join(".embeddings.bin"), model, cache);
}

// Sidecar format (little-endian, length-prefixed; no alignment games):
//   magic "MEMVEC1\0" · u32 model_len · model utf8 · u32 count ·
//   count × ( u32 id_len · id utf8 · u32 vec_len · vec_len × f32 )
const BIN_MAGIC: &[u8; 8] = b"MEMVEC1\0";

fn save_bin(path: &Path, model: &str, cache: &HashMap<String, Vec<f32>>) -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(64 + cache.len() * (32 + 768 * 4));
    buf.extend_from_slice(BIN_MAGIC);
    buf.extend_from_slice(&(model.len() as u32).to_le_bytes());
    buf.extend_from_slice(model.as_bytes());
    buf.extend_from_slice(&(cache.len() as u32).to_le_bytes());
    for (id, vec) in cache {
        buf.extend_from_slice(&(id.len() as u32).to_le_bytes());
        buf.extend_from_slice(id.as_bytes());
        buf.extend_from_slice(&(vec.len() as u32).to_le_bytes());
        for f in vec {
            buf.extend_from_slice(&f.to_le_bytes());
        }
    }
    // write-then-rename so a killed process never leaves a truncated sidecar in place
    let tmp = path.with_extension("bin.tmp");
    std::fs::write(&tmp, &buf)?;
    std::fs::rename(&tmp, path)
}

/// `None` on any malformation (bad magic, wrong model, truncation) → caller falls back to
/// JSON and rewrites the sidecar. Corruption can never poison the cache, only slow one load.
fn load_bin(path: &Path, model: &str) -> Option<HashMap<String, Vec<f32>>> {
    let buf = std::fs::read(path).ok()?;
    let mut at = 0usize;
    let take = |at: &mut usize, n: usize| -> Option<&[u8]> {
        let s = buf.get(*at..*at + n)?;
        *at += n;
        Some(s)
    };
    let u32_at = |at: &mut usize| -> Option<u32> {
        take(at, 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    if take(&mut at, 8)? != BIN_MAGIC {
        return None;
    }
    let mlen = u32_at(&mut at)? as usize;
    if std::str::from_utf8(take(&mut at, mlen)?).ok()? != model {
        return None;
    }
    let count = u32_at(&mut at)? as usize;
    let mut map = HashMap::with_capacity(count);
    for _ in 0..count {
        let ilen = u32_at(&mut at)? as usize;
        let id = std::str::from_utf8(take(&mut at, ilen)?).ok()?.to_string();
        let vlen = u32_at(&mut at)? as usize;
        let raw = take(&mut at, vlen * 4)?;
        let vec: Vec<f32> = raw
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        map.insert(id, vec);
    }
    Some(map)
}
