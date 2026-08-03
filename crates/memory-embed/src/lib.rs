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

pub fn load_cache(dir: &Path, model: &str) -> HashMap<String, Vec<f32>> {
    let path = dir.join(".embeddings.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
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

pub fn save_cache(dir: &Path, model: &str, cache: &HashMap<String, Vec<f32>>) {
    let vectors: serde_json::Map<String, Value> =
        cache.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let doc = json!({ "model": model, "vectors": vectors });
    let _ = std::fs::write(dir.join(".embeddings.json"), doc.to_string());
}
