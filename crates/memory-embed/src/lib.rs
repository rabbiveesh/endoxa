//! Ollama-backed embeddings + a tiny on-disk vector cache. Shells to `curl` (so no HTTP
//! crate). nomic-embed-text is asymmetric: callers prefix docs with `search_document: ` and
//! queries with `search_query: `.

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
