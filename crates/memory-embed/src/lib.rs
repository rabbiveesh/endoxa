//! Ollama-backed embeddings + a tiny on-disk vector cache. Shells to `curl` (so no HTTP
//! crate). nomic-embed-text is asymmetric: callers prefix docs with `search_document: ` and
//! queries with `search_query: `.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

// --- instrumentation (process-local; every LLM touch flows through this crate) ----------
//
// The worker/CLI snapshot these before and after a pass and persist the DELTA to the metrics
// ledger, so "what did that run cost" is measurable without threading counters through every
// linker. Attempts are counted whether or not the call succeeds — a failed call still burned
// wall-clock and a curl.

static CHAT_CALLS: AtomicU64 = AtomicU64::new(0);
static CHAT_MS: AtomicU64 = AtomicU64::new(0);
static EMBED_CALLS: AtomicU64 = AtomicU64::new(0);
static EMBED_TEXTS: AtomicU64 = AtomicU64::new(0);
static EMBED_MS: AtomicU64 = AtomicU64::new(0);

/// A snapshot of this process's cumulative LLM traffic. Subtract two snapshots for a per-pass
/// delta (`later.delta(&earlier)`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LlmCounters {
    pub chat_calls: u64,
    pub chat_ms: u64,
    pub embed_calls: u64,
    pub embed_texts: u64,
    pub embed_ms: u64,
}

impl LlmCounters {
    pub fn delta(&self, earlier: &LlmCounters) -> LlmCounters {
        LlmCounters {
            chat_calls: self.chat_calls.saturating_sub(earlier.chat_calls),
            chat_ms: self.chat_ms.saturating_sub(earlier.chat_ms),
            embed_calls: self.embed_calls.saturating_sub(earlier.embed_calls),
            embed_texts: self.embed_texts.saturating_sub(earlier.embed_texts),
            embed_ms: self.embed_ms.saturating_sub(earlier.embed_ms),
        }
    }
}

pub fn counters() -> LlmCounters {
    LlmCounters {
        chat_calls: CHAT_CALLS.load(Ordering::Relaxed),
        chat_ms: CHAT_MS.load(Ordering::Relaxed),
        embed_calls: EMBED_CALLS.load(Ordering::Relaxed),
        embed_texts: EMBED_TEXTS.load(Ordering::Relaxed),
        embed_ms: EMBED_MS.load(Ordering::Relaxed),
    }
}

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
            return Ok(vec![]); // no backend touched — not counted
        }
        let t0 = std::time::Instant::now();
        let r = self.embed_uncounted(inputs);
        EMBED_CALLS.fetch_add(1, Ordering::Relaxed);
        EMBED_TEXTS.fetch_add(inputs.len() as u64, Ordering::Relaxed);
        EMBED_MS.fetch_add(t0.elapsed().as_millis() as u64, Ordering::Relaxed);
        r
    }

    fn embed_uncounted(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, String> {
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

/// One-shot structured chat against an Ollama generation model (e.g. qwen2.5). Forces JSON
/// output (`format: json`, temperature 0) and returns the parsed object. Used by the judgment
/// linker. Errors (ollama down, model not pulled, non-JSON) come back as strings.
pub fn chat_json(url: &str, model: &str, system: &str, user: &str) -> Result<Value, String> {
    let t0 = std::time::Instant::now();
    let r = chat_json_uncounted(url, model, system, user);
    CHAT_CALLS.fetch_add(1, Ordering::Relaxed);
    CHAT_MS.fetch_add(t0.elapsed().as_millis() as u64, Ordering::Relaxed);
    r
}

fn chat_json_uncounted(url: &str, model: &str, system: &str, user: &str) -> Result<Value, String> {
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

pub fn save_cache(dir: &Path, model: &str, cache: &HashMap<String, Vec<f32>>) {
    let vectors: serde_json::Map<String, Value> =
        cache.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let doc = json!({ "model": model, "vectors": vectors });
    let _ = std::fs::write(dir.join(".embeddings.json"), doc.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ONE sequential test: the counters are process-global atomics, and two parallel tests
    /// snapshotting deltas would race each other's increments.
    #[test]
    fn counters_track_attempts_and_skip_empty_batches() {
        let oll = Ollama { url: "http://127.0.0.1:9".into(), model: "x".into() };
        let before = counters();
        assert!(oll.embed(&[]).unwrap().is_empty());
        assert_eq!(counters().delta(&before), LlmCounters::default(), "empty batch touches no backend — not counted");
        // a port nothing listens on: fails fast, no network needed — still one counted attempt
        let _ = chat_json("http://127.0.0.1:9", "x", "s", "u");
        let _ = oll.embed(&["a".into(), "b".into()]);
        let d = counters().delta(&before);
        assert_eq!(d.chat_calls, 1, "failed chat still counts as an attempt");
        assert_eq!(d.embed_calls, 1);
        assert_eq!(d.embed_texts, 2, "texts counted per input, not per call");
    }
}
