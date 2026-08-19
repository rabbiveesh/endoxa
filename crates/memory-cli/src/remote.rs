//! ☁ Remote store — an S3-compatible bucket as the shared L0 transport.
//!
//! WHY OBJECT STORAGE (and not git): the store's write pattern is append-only,
//! content-addressed, never-overwrite. That is *exactly* PUT-of-a-new-key semantics: distinct
//! keys are fully independent, so N concurrent sessions writing beliefs never compete — no
//! branch head to race on, no pull-before-write, no merge. Concurrent supersedes of the same
//! belief are two independent edge objects, and `defeated()` (not the transport) arbitrates.
//!
//! The bucket holds the SAME L0 `.md` files as the local dir — it is the flat-file store with
//! a network path, not a second representation. Local dirs demote to caches of the bucket.
//! Doctrine intact: the layout is never the storage, and L0 files remain the only truth.
//!
//! Mechanics (all zero-dep, on-brand):
//!  - transport: shell to `curl` (same as memory-embed) — no HTTP crate.
//!  - auth: AWS Signature V4, hand-rolled on `memory_core::hash::sha256` (HMAC-SHA256 is
//!    ~15 lines on top of the SHA-256 we already carry for content ids).
//!  - sync: LIST (one call at n≈600) → set-diff against the local dir → GET remote-only,
//!    PUT local-only with `If-None-Match: *` (write-once; 412 = already there = fine).
//!  - embeddings: `.embeddings.json` has a SINGLE WRITER (the ollama box, `remote.role =
//!    "writer"`); everyone else pulls when the remote ETag moves. No conflict is possible
//!    by construction. Cloud-born beliefs lack vectors until the writer next embeds them —
//!    lexical fallback covers the gap.
//!  - failure: every remote error is a one-line benign skip. Offline-first; the local cache
//!    keeps working with no remote at all.
//!
//! Config (`[remote]` in config.toml, or env — single-word keys only, see load_settings):
//!  - `remote.url` / `MEM_REMOTE_URL`: `https://<host>/<bucket>[/<prefix>]` (path-style).
//!  - `remote.role` / `MEM_REMOTE_ROLE`: `reader` (default) | `writer` (embeddings authority).
//!  - `remote.staleness` / `MEM_REMOTE_STALENESS`: minutes between implicit pulls (default 10).
//!  - creds: standard `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` (+ optional
//!    `AWS_SESSION_TOKEN`), region `AWS_REGION` (default `auto`, which is what R2 wants).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use memory_core::hash::{sha256, sha256_hex};

// --- config ------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Remote {
    /// `https` in real life; `http` allowed so a loopback fake-S3 can wire-test the client.
    scheme: String,
    /// Host (authority) part of the endpoint, e.g. `acct.r2.cloudflarestorage.com`.
    host: String,
    /// Bucket name (first path segment of `remote.url`).
    bucket: String,
    /// Optional key prefix inside the bucket (rest of the path), no leading/trailing `/`.
    prefix: String,
    region: String,
    access_key: String,
    secret_key: String,
    session_token: Option<String>,
    pub role_writer: bool,
}

impl Remote {
    /// None when the remote is unconfigured (no url or no creds) — every caller treats that
    /// as "local-only mode", silently.
    pub fn from_settings(r: &crate::RemoteSettings) -> Option<Remote> {
        let url = r.url.clone()?;
        let access_key = std::env::var("AWS_ACCESS_KEY_ID").ok()?;
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").ok()?;
        let (scheme, rest) = if let Some(r) = url.strip_prefix("https://") {
            ("https", r)
        } else if let Some(r) = url.strip_prefix("http://") {
            ("http", r)
        } else {
            return None;
        };
        let (host, path) = match rest.find('/') {
            Some(i) => (&rest[..i], rest[i + 1..].trim_matches('/')),
            None => (rest, ""),
        };
        if path.is_empty() {
            return None; // a bucket is required; refusing beats guessing
        }
        let (bucket, prefix) = match path.find('/') {
            Some(i) => (&path[..i], &path[i + 1..]),
            None => (path, ""),
        };
        Some(Remote {
            scheme: scheme.to_string(),
            host: host.to_string(),
            bucket: bucket.to_string(),
            prefix: prefix.to_string(),
            region: std::env::var("AWS_REGION").unwrap_or_else(|_| "auto".into()),
            access_key,
            secret_key,
            session_token: std::env::var("AWS_SESSION_TOKEN").ok(),
            role_writer: r.role == "writer",
        })
    }

    /// Full object key for a store-dir file name.
    fn key(&self, name: &str) -> String {
        if self.prefix.is_empty() { name.to_string() } else { format!("{}/{}", self.prefix, name) }
    }
}

// --- HMAC-SHA256 + SigV4 (FIPS 198-1 on top of core's FIPS 180-4) ------------------------

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut k = [0u8; 64];
    if key.len() > 64 {
        k[..32].copy_from_slice(&sha256(key));
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let ipad: Vec<u8> = k.iter().map(|b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|b| b ^ 0x5c).collect();
    let mut inner = ipad;
    inner.extend_from_slice(msg);
    let mut outer = opad;
    outer.extend_from_slice(&sha256(&inner));
    sha256(&outer)
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// AWS uri-encode: unreserved chars pass, everything else %XX (uppercase hex). `/` passes
/// only when `keep_slash` (path encoding); query encoding escapes it.
fn uri_encode(s: &str, keep_slash: bool) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => out.push(b as char),
            b'/' if keep_slash => out.push('/'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// `20260819T120000Z` derived from core's `iso_now` (single source of date math).
fn amz_date_now() -> String {
    let iso = memory_core::iso_now(); // 2026-08-19T12:34:56.789Z
    let compact: String = iso.chars().filter(|c| c.is_ascii_digit() || *c == 'T').take(15).collect();
    format!("{compact}Z")
}

struct Signed {
    url: String,
    headers: Vec<(String, String)>,
}

/// Build the signed request: URL + the headers curl must send. `query` must be the FINAL
/// query pairs (they are encoded + sorted here and the same string goes into the URL, so the
/// wire request always matches the signature).
fn sign(
    r: &Remote,
    method: &str,
    key: Option<&str>,
    query: &[(String, String)],
    payload_hash: &str,
    amz_date: &str,
) -> Signed {
    let date = &amz_date[..8];
    let path = match key {
        Some(k) => format!("/{}/{}", r.bucket, uri_encode(k, true)),
        None => format!("/{}", r.bucket),
    };
    let mut q: Vec<String> = query
        .iter()
        .map(|(k, v)| format!("{}={}", uri_encode(k, false), uri_encode(v, false)))
        .collect();
    q.sort();
    let canonical_query = q.join("&");

    let mut headers: Vec<(String, String)> = vec![
        ("host".into(), r.host.clone()),
        ("x-amz-content-sha256".into(), payload_hash.to_string()),
        ("x-amz-date".into(), amz_date.to_string()),
    ];
    if let Some(t) = &r.session_token {
        headers.push(("x-amz-security-token".into(), t.clone()));
    }
    // already alphabetical: host < x-amz-content-sha256 < x-amz-date < x-amz-security-token
    let signed_headers: String = headers.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join(";");
    let canonical_headers: String = headers.iter().map(|(k, v)| format!("{k}:{v}\n")).collect();
    let canonical_request = format!("{method}\n{path}\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{payload_hash}");

    let scope = format!("{date}/{}/s3/aws4_request", r.region);
    let string_to_sign = format!("AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}", sha256_hex(canonical_request.as_bytes()));
    let k_date = hmac_sha256(format!("AWS4{}", r.secret_key).as_bytes(), date.as_bytes());
    let k_region = hmac_sha256(&k_date, r.region.as_bytes());
    let k_service = hmac_sha256(&k_region, b"s3");
    let k_signing = hmac_sha256(&k_service, b"aws4_request");
    let signature = hex(&hmac_sha256(&k_signing, string_to_sign.as_bytes()));

    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={}/{scope}, SignedHeaders={signed_headers}, Signature={signature}",
        r.access_key
    );
    let url = if canonical_query.is_empty() {
        format!("{}://{}{path}", r.scheme, r.host)
    } else {
        format!("{}://{}{path}?{canonical_query}", r.scheme, r.host)
    };
    let mut send: Vec<(String, String)> = headers.into_iter().filter(|(k, _)| k != "host").collect();
    send.push(("Authorization".into(), authorization));
    Signed { url, headers: send }
}

// --- curl transport ----------------------------------------------------------------------

const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/// One S3 request via curl. Returns (http_status, body) — body is empty when `out` captured
/// it to a file. Any transport failure is Err (callers treat every Err as a benign skip).
fn s3(
    r: &Remote,
    method: &str,
    key: Option<&str>,
    query: &[(String, String)],
    upload: Option<&Path>,
    out: Option<&Path>,
    extra_headers: &[(&str, &str)],
) -> Result<(u16, String), String> {
    let payload_hash = match upload {
        Some(p) => {
            let bytes = std::fs::read(p).map_err(|e| format!("read {}: {e}", p.display()))?;
            sha256_hex(&bytes)
        }
        None => EMPTY_SHA256.to_string(),
    };
    let s = sign(r, method, key, query, &payload_hash, &amz_date_now());

    let mut cmd = Command::new("curl");
    cmd.arg("-sS").arg("--max-time").arg("120");
    for (k, v) in &s.headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    for (k, v) in extra_headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    match (method, upload) {
        ("PUT", Some(p)) => {
            cmd.arg("-T").arg(p); // -T implies PUT; never combine with -X
        }
        ("PUT", None) => {
            cmd.arg("-X").arg("PUT");
        }
        _ => {} // GET is curl's default
    }
    let body_tmp; // keep alive until after read
    match out {
        Some(p) => {
            cmd.arg("-o").arg(p);
            body_tmp = None;
        }
        None => {
            let t = std::env::temp_dir().join(format!("mem-sync-{}-{}", std::process::id(), sha256_hex(s.url.as_bytes())[..8].to_string()));
            cmd.arg("-o").arg(&t);
            body_tmp = Some(t);
        }
    }
    cmd.arg("-w").arg("%{http_code}").arg(&s.url);
    let outp = cmd.output().map_err(|e| format!("curl spawn failed (is curl installed?): {e}"))?;
    if !outp.status.success() {
        return Err(format!("curl: {}", String::from_utf8_lossy(&outp.stderr).trim()));
    }
    let code: u16 = String::from_utf8_lossy(&outp.stdout).trim().parse().unwrap_or(0);
    let body = match &body_tmp {
        Some(t) => {
            let b = std::fs::read_to_string(t).unwrap_or_default();
            let _ = std::fs::remove_file(t);
            b
        }
        None => String::new(),
    };
    Ok((code, body))
}

// --- ListObjectsV2 (minimal hand-rolled XML scan; keys here are our own plain filenames) --

#[derive(Debug, Default)]
pub struct RemoteListing {
    /// name (prefix-stripped) → etag. Only top-level names (no `/` remainder).
    pub objects: BTreeMap<String, String>,
}

fn xml_field(chunk: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let s = chunk.find(&open)? + open.len();
    let e = chunk[s..].find(&close)? + s;
    Some(xml_unescape(&chunk[s..e]))
}

fn xml_unescape(s: &str) -> String {
    s.replace("&quot;", "\"").replace("&lt;", "<").replace("&gt;", ">").replace("&#39;", "'").replace("&amp;", "&")
}

/// Parse one ListObjectsV2 page → (entries, continuation token if truncated).
fn parse_list_page(xml: &str) -> (Vec<(String, String)>, Option<String>) {
    let mut entries = Vec::new();
    for chunk in xml.split("<Contents>").skip(1) {
        if let Some(key) = xml_field(chunk, "Key") {
            let etag = xml_field(chunk, "ETag").unwrap_or_default();
            entries.push((key, etag.trim_matches('"').to_string()));
        }
    }
    let token = if xml_field(xml, "IsTruncated").as_deref() == Some("true") {
        xml_field(xml, "NextContinuationToken")
    } else {
        None
    };
    (entries, token)
}

fn list_remote(r: &Remote) -> Result<RemoteListing, String> {
    let mut listing = RemoteListing::default();
    let mut token: Option<String> = None;
    loop {
        let mut query: Vec<(String, String)> = vec![("list-type".into(), "2".into())];
        if !r.prefix.is_empty() {
            query.push(("prefix".into(), format!("{}/", r.prefix)));
        }
        if let Some(t) = &token {
            query.push(("continuation-token".into(), t.clone()));
        }
        let (code, body) = s3(r, "GET", None, &query, None, None, &[])?;
        if code != 200 {
            return Err(format!("list HTTP {code}: {}", body.chars().take(200).collect::<String>()));
        }
        let (entries, next) = parse_list_page(&body);
        for (key, etag) in entries {
            let name = if r.prefix.is_empty() {
                key
            } else {
                match key.strip_prefix(&format!("{}/", r.prefix)) {
                    Some(n) => n.to_string(),
                    None => continue,
                }
            };
            if name.is_empty() || name.contains('/') {
                continue; // not ours / nested
            }
            listing.objects.insert(name, etag);
        }
        match next {
            Some(t) => token = Some(t),
            None => break,
        }
    }
    Ok(listing)
}

// --- sync plan (pure; unit-tested) -------------------------------------------------------

#[derive(Debug, Default, PartialEq)]
pub struct SyncPlan {
    pub pull: Vec<String>,
    pub push: Vec<String>,
}

/// Append-only set reconciliation: no mtimes, no merges, no conflicts — a belief file either
/// exists on a side or it doesn't. Only `*.md` participates (beliefs + edge-beliefs).
pub fn plan_sync(local: &[String], remote: &[String]) -> SyncPlan {
    let l: std::collections::BTreeSet<&str> =
        local.iter().map(|s| s.as_str()).filter(|n| n.ends_with(".md") && !n.starts_with('.')).collect();
    let r: std::collections::BTreeSet<&str> =
        remote.iter().map(|s| s.as_str()).filter(|n| n.ends_with(".md") && !n.starts_with('.')).collect();
    SyncPlan {
        pull: r.difference(&l).map(|s| s.to_string()).collect(),
        push: l.difference(&r).map(|s| s.to_string()).collect(),
    }
}

// --- sync state (.sync-state.json — local, disposable) -----------------------------------

#[derive(Debug, Default)]
struct SyncState {
    last_pull: u64,
    embeddings_etag: String,
    embeddings_pushed_sha: String,
}

fn state_path(dir: &Path) -> PathBuf {
    dir.join(".sync-state.json")
}

fn load_state(dir: &Path) -> SyncState {
    let v: serde_json::Value = std::fs::read_to_string(state_path(dir))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(serde_json::Value::Null);
    SyncState {
        last_pull: v.get("last_pull").and_then(|x| x.as_u64()).unwrap_or(0),
        embeddings_etag: v.get("embeddings_etag").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        embeddings_pushed_sha: v.get("embeddings_pushed_sha").and_then(|x| x.as_str()).unwrap_or("").to_string(),
    }
}

fn save_state(dir: &Path, s: &SyncState) {
    let v = serde_json::json!({
        "last_pull": s.last_pull,
        "embeddings_etag": s.embeddings_etag,
        "embeddings_pushed_sha": s.embeddings_pushed_sha,
    });
    let _ = std::fs::write(state_path(dir), v.to_string());
}

fn epoch_now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

// --- the sync passes ---------------------------------------------------------------------

pub const EMBEDDINGS_FILE: &str = ".embeddings.json";

#[derive(Debug, Default)]
pub struct SyncReport {
    pub pulled: usize,
    pub pushed: usize,
    pub already: usize, // pushes that 412'd: someone else got there first (fine, append-only)
    pub embeddings: Option<&'static str>, // "pulled" | "pushed"
    pub errors: usize,
}

impl SyncReport {
    pub fn line(&self) -> String {
        let mut parts = Vec::new();
        if self.pulled > 0 {
            parts.push(format!("pulled {}", self.pulled));
        }
        if self.pushed > 0 {
            parts.push(format!("pushed {}", self.pushed));
        }
        if self.already > 0 {
            parts.push(format!("{} already there", self.already));
        }
        if let Some(e) = self.embeddings {
            parts.push(format!("embeddings {e}"));
        }
        if self.errors > 0 {
            parts.push(format!("{} errors", self.errors));
        }
        if parts.is_empty() {
            "in sync".into()
        } else {
            parts.join(", ")
        }
    }
}

fn local_md_names(dir: &Path) -> Vec<String> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".md") && !name.starts_with('.') && e.path().is_file() {
                v.push(name);
            }
        }
    }
    v
}

/// One sync pass. `do_pull`/`do_push` select direction; both true = `mem sync`.
/// Never panics, never leaves partial files (pull writes tmp→rename).
pub fn sync_pass(dir: &Path, r: &Remote, do_pull: bool, do_push: bool, dry: bool) -> Result<SyncReport, String> {
    let listing = list_remote(r)?; // the one call everything hinges on; Err → caller skips
    let local = local_md_names(dir);
    let remote_names: Vec<String> = listing.objects.keys().cloned().collect();
    let plan = plan_sync(&local, &remote_names);
    let mut rep = SyncReport::default();
    let mut state = load_state(dir);

    if do_pull {
        for name in &plan.pull {
            if dry {
                println!("  would pull {name}");
                rep.pulled += 1;
                continue;
            }
            let tmp = dir.join(format!(".pull-tmp-{name}"));
            match s3(r, "GET", Some(&r.key(name)), &[], None, Some(&tmp), &[]) {
                Ok((200, _)) => {
                    if std::fs::rename(&tmp, dir.join(name)).is_ok() {
                        rep.pulled += 1;
                    } else {
                        rep.errors += 1;
                    }
                }
                Ok((code, _)) => {
                    let _ = std::fs::remove_file(&tmp);
                    eprintln!("☁ pull {name}: HTTP {code}");
                    rep.errors += 1;
                }
                Err(e) => {
                    let _ = std::fs::remove_file(&tmp);
                    eprintln!("☁ pull {name}: {e}");
                    rep.errors += 1;
                }
            }
        }
        if !dry {
            state.last_pull = epoch_now();
        }
    }

    if do_push {
        for name in &plan.push {
            if dry {
                println!("  would push {name}");
                rep.pushed += 1;
                continue;
            }
            let path = dir.join(name);
            // Write-once: If-None-Match:* → a concurrent identical push 412s, which is
            // success for an append-only store. Providers without conditional-write support
            // (501/400 on the header) get one unconditional retry — identical content
            // anyway, since ids are content-addressed.
            match s3(r, "PUT", Some(&r.key(name)), &[], Some(&path), None, &[("If-None-Match", "*")]) {
                Ok((200, _)) => rep.pushed += 1,
                Ok((412, _)) => rep.already += 1,
                Ok((code, _)) if code == 501 || code == 400 => {
                    match s3(r, "PUT", Some(&r.key(name)), &[], Some(&path), None, &[]) {
                        Ok((200, _)) => rep.pushed += 1,
                        Ok((c2, _)) => {
                            eprintln!("☁ push {name}: HTTP {c2}");
                            rep.errors += 1;
                        }
                        Err(e) => {
                            eprintln!("☁ push {name}: {e}");
                            rep.errors += 1;
                        }
                    }
                }
                Ok((code, _)) => {
                    eprintln!("☁ push {name}: HTTP {code}");
                    rep.errors += 1;
                }
                Err(e) => {
                    eprintln!("☁ push {name}: {e}");
                    rep.errors += 1;
                }
            }
        }
    }

    // Embeddings: single-writer discipline. Writer pushes when the local file changed since
    // the last push; readers pull when the remote ETag moved. ETag is an opaque change
    // token — we never interpret it.
    let emb_local = dir.join(EMBEDDINGS_FILE);
    if r.role_writer && do_push {
        if let Ok(bytes) = std::fs::read(&emb_local) {
            let sha = sha256_hex(&bytes);
            if sha != state.embeddings_pushed_sha {
                if dry {
                    println!("  would push {EMBEDDINGS_FILE}");
                    rep.embeddings = Some("pushed");
                } else {
                    match s3(r, "PUT", Some(&r.key(EMBEDDINGS_FILE)), &[], Some(&emb_local), None, &[]) {
                        Ok((200, _)) => {
                            state.embeddings_pushed_sha = sha;
                            rep.embeddings = Some("pushed");
                        }
                        Ok((code, _)) => {
                            eprintln!("☁ push embeddings: HTTP {code}");
                            rep.errors += 1;
                        }
                        Err(e) => {
                            eprintln!("☁ push embeddings: {e}");
                            rep.errors += 1;
                        }
                    }
                }
            }
        }
    } else if !r.role_writer && do_pull {
        if let Some(etag) = listing.objects.get(EMBEDDINGS_FILE) {
            if *etag != state.embeddings_etag {
                if dry {
                    println!("  would pull {EMBEDDINGS_FILE}");
                    rep.embeddings = Some("pulled");
                } else {
                    let tmp = dir.join(".pull-tmp-embeddings.json");
                    match s3(r, "GET", Some(&r.key(EMBEDDINGS_FILE)), &[], None, Some(&tmp), &[]) {
                        Ok((200, _)) => {
                            if std::fs::rename(&tmp, &emb_local).is_ok() {
                                state.embeddings_etag = etag.clone();
                                rep.embeddings = Some("pulled");
                            } else {
                                rep.errors += 1;
                            }
                        }
                        Ok((code, _)) => {
                            let _ = std::fs::remove_file(&tmp);
                            eprintln!("☁ pull embeddings: HTTP {code}");
                            rep.errors += 1;
                        }
                        Err(e) => {
                            let _ = std::fs::remove_file(&tmp);
                            eprintln!("☁ pull embeddings: {e}");
                            rep.errors += 1;
                        }
                    }
                }
            }
        }
    }

    if !dry {
        save_state(dir, &state);
    }
    Ok(rep)
}

// --- surface hooks (the three call sites) ------------------------------------------------

/// After `remember` (and its on-write consolidation): push local-only files up. Also
/// self-healing — anything a previously-failed push left behind goes now. Quiet on no-op.
pub fn push_after_write(dir: &Path) {
    let settings = crate::load_settings().remote;
    let Some(r) = Remote::from_settings(&settings) else { return };
    match sync_pass(dir, &r, false, true, false) {
        Ok(rep) if rep.pushed + rep.already > 0 || rep.embeddings.is_some() => {
            println!("☁ {}", rep.line());
        }
        Ok(_) => {}
        Err(e) => eprintln!("☁ push skipped: {e}"),
    }
}

/// Before `recall`/`ask`: pull if the last pull is older than `remote.staleness` minutes.
/// Quiet on no-op; one line when something arrived.
pub fn pull_if_stale(dir: &Path) {
    let settings = crate::load_settings().remote;
    let Some(r) = Remote::from_settings(&settings) else { return };
    let state = load_state(dir);
    let now = epoch_now();
    if now.saturating_sub(state.last_pull) < settings.staleness * 60 {
        return;
    }
    match sync_pass(dir, &r, true, false, false) {
        Ok(rep) if rep.pulled > 0 || rep.embeddings.is_some() => println!("☁ {}", rep.line()),
        Ok(_) => {}
        Err(e) => eprintln!("☁ pull skipped: {e}"),
    }
}

/// `mem sync` — the explicit, two-way, chatty form.
pub fn cmd_sync(args: &[String]) {
    let mut dry = false;
    let mut status = false;
    for a in args {
        match a.as_str() {
            "--dry-run" => dry = true,
            "--status" => status = true,
            _ => {}
        }
    }
    let dir = crate::store_dir();
    let settings = crate::load_settings().remote;
    let Some(r) = Remote::from_settings(&settings) else {
        eprintln!("remote not configured.");
        eprintln!("  1. create an S3-compatible bucket (Cloudflare R2 free tier works)");
        eprintln!("  2. config.toml: [remote] url = \"https://<host>/<bucket>[/<prefix>]\"  (or MEM_REMOTE_URL)");
        eprintln!("     on the ollama box also: role = \"writer\"   # embeddings authority");
        eprintln!("  3. env: AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY");
        std::process::exit(2);
    };
    if status {
        let s = load_state(&dir);
        println!("remote: {}://{}/{}{}", r.scheme, r.host, r.bucket, if r.prefix.is_empty() { String::new() } else { format!("/{}", r.prefix) });
        println!("role: {}", if r.role_writer { "writer (embeddings authority)" } else { "reader" });
        println!("last pull: {}", if s.last_pull == 0 { "never".into() } else { format!("{}s ago", epoch_now().saturating_sub(s.last_pull)) });
        return;
    }
    match sync_pass(&dir, &r, true, true, dry) {
        Ok(rep) => {
            println!("☁ {}{}", if dry { "dry-run: " } else { "" }, rep.line());
            if rep.errors > 0 {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("☁ sync failed: {e}");
            std::process::exit(1);
        }
    }
}

// --- tests -------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4231 test case 2 ("Jefe" / "what do ya want for nothing?")
    #[test]
    fn hmac_rfc4231_case2() {
        let mac = hmac_sha256(b"Jefe", b"what do ya want for nothing?");
        assert_eq!(hex(&mac), "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
    }

    // RFC 4231 test case 1 (20×0x0b key, "Hi There")
    #[test]
    fn hmac_rfc4231_case1() {
        let mac = hmac_sha256(&[0x0b; 20], b"Hi There");
        assert_eq!(hex(&mac), "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");
    }

    // RFC 4231 test case 6 (131-byte key → key gets hashed first)
    #[test]
    fn hmac_rfc4231_long_key() {
        let mac = hmac_sha256(&[0xaa; 131], b"Test Using Larger Than Block-Size Key - Hash Key First");
        assert_eq!(hex(&mac), "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54");
    }

    fn test_remote() -> Remote {
        Remote {
            scheme: "https".into(),
            host: "examplebucket.s3.amazonaws.com".into(),
            bucket: "examplebucket".into(),
            prefix: String::new(),
            region: "us-east-1".into(),
            access_key: "AKIAIOSFODNN7EXAMPLE".into(),
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".into(),
            session_token: None,
            role_writer: false,
        }
    }

    fn signature_of(s: &Signed) -> String {
        let auth = &s.headers.iter().find(|(k, _)| k == "Authorization").unwrap().1;
        auth.split("Signature=").nth(1).unwrap().to_string()
    }

    /// Signatures pinned against an independent Python (hashlib/hmac) implementation of the
    /// SigV4 spec — pins the whole chain: canonical request → string-to-sign → signing key.
    #[test]
    fn sigv4_get_signature_pinned() {
        let r = test_remote();
        let s = sign(&r, "GET", Some("test.md"), &[], EMPTY_SHA256, "20260819T120000Z");
        assert_eq!(s.url, "https://examplebucket.s3.amazonaws.com/examplebucket/test.md");
        let auth = &s.headers.iter().find(|(k, _)| k == "Authorization").unwrap().1;
        assert!(auth.starts_with("AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20260819/us-east-1/s3/aws4_request, SignedHeaders=host;x-amz-content-sha256;x-amz-date, Signature="));
        assert_eq!(signature_of(&s), "46b311c5ed752f3e7b24b0f7f0309706fc10ceb2f94c9e5e910ce61398a60f6e");
    }

    #[test]
    fn sigv4_list_signature_pinned() {
        let r = test_remote();
        let s = sign(
            &r,
            "GET",
            None,
            &[("prefix".into(), "mem/".into()), ("list-type".into(), "2".into())],
            EMPTY_SHA256,
            "20260819T120000Z",
        );
        assert_eq!(signature_of(&s), "a873533d00ede91408825697c15403568c8bd44122553544d9c8d54dc1502c8b");
    }

    #[test]
    fn sigv4_put_signature_pinned() {
        let r = test_remote();
        // payload = b"hello\n"
        let ph = "5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03";
        let s = sign(&r, "PUT", Some("b_abc.md"), &[], ph, "20260819T120000Z");
        assert_eq!(signature_of(&s), "f20fa3e5795eca046badedcb1e61a84dec3fe4d8822f512ea71d3924e8e7e607");
    }

    #[test]
    fn sigv4_query_is_sorted_and_encoded() {
        let r = test_remote();
        let s = sign(
            &r,
            "GET",
            None,
            &[("prefix".into(), "mem/".into()), ("list-type".into(), "2".into())],
            EMPTY_SHA256,
            "20260819T120000Z",
        );
        // sorted (list-type < prefix), slash %2F-encoded in query position
        assert!(s.url.ends_with("/examplebucket?list-type=2&prefix=mem%2F"));
    }

    #[test]
    fn uri_encode_matrix() {
        assert_eq!(uri_encode("b_1a2b3c.md", true), "b_1a2b3c.md");
        assert_eq!(uri_encode("a/b", true), "a/b");
        assert_eq!(uri_encode("a/b", false), "a%2Fb");
        assert_eq!(uri_encode("a b+c=d", false), "a%20b%2Bc%3Dd");
        assert_eq!(uri_encode(".embeddings.json", true), ".embeddings.json");
    }

    #[test]
    fn plan_is_append_only_set_diff() {
        let local = vec!["a.md".into(), "b.md".into(), ".sync-state.json".into(), "notes.txt".into()];
        let remote = vec!["b.md".into(), "c.md".into(), ".embeddings.json".into()];
        let p = plan_sync(&local, &remote);
        assert_eq!(p.pull, vec!["c.md".to_string()]); // remote-only md
        assert_eq!(p.push, vec!["a.md".to_string()]); // local-only md
        // dotfiles and non-md never travel through the md plan
    }

    #[test]
    fn list_page_parse_with_pagination() {
        let xml = r#"<?xml version="1.0"?><ListBucketResult>
            <IsTruncated>true</IsTruncated>
            <NextContinuationToken>abc+def=</NextContinuationToken>
            <Contents><Key>mem/b_1.md</Key><ETag>&quot;d41d8cd9&quot;</ETag><Size>10</Size></Contents>
            <Contents><Key>mem/.embeddings.json</Key><ETag>&quot;aabbcc&quot;</ETag></Contents>
        </ListBucketResult>"#;
        let (entries, token) = parse_list_page(xml);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], ("mem/b_1.md".to_string(), "d41d8cd9".to_string()));
        assert_eq!(entries[1].0, "mem/.embeddings.json");
        assert_eq!(token.as_deref(), Some("abc+def="));
    }

    #[test]
    fn remote_from_url_parses_bucket_and_prefix() {
        std::env::set_var("AWS_ACCESS_KEY_ID", "k");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "s");
        let rs = crate::RemoteSettings {
            url: Some("https://acct.r2.cloudflarestorage.com/mem-store/beliefs".into()),
            role: "writer".into(),
            staleness: 10,
        };
        let r = Remote::from_settings(&rs).unwrap();
        assert_eq!(r.host, "acct.r2.cloudflarestorage.com");
        assert_eq!(r.bucket, "mem-store");
        assert_eq!(r.prefix, "beliefs");
        assert!(r.role_writer);
        assert_eq!(r.key("x.md"), "beliefs/x.md");
        // bucketless url refuses
        let bad = crate::RemoteSettings { url: Some("https://host.only".into()), role: "reader".into(), staleness: 10 };
        assert!(Remote::from_settings(&bad).is_none());
    }
}
