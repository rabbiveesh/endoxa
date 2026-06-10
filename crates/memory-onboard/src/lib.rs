//! memory-onboard — the tiered onboarding harness for an existing repo.
//!
//! TIER 0 ships here: a deterministic, git-only harvest of **leads** — candidate beliefs
//! mined from where out-of-band knowledge actually leaks into a repo. No LLM, no
//! embeddings, no writes to the belief store. A lead is deliberately NOT a belief:
//! tier 0 has no judgment, so it must not author claims — it surfaces evidence for a
//! later tier (cheap model / frontier agent / the human) to turn into beliefs.
//!
//! Why these four sources: the harness exists for knowledge a static doc can't hold,
//! and git history hands us the system's unique strength — temporal defeat structure:
//!   - `Revert`     — a supersession chain with provenance ("we tried X, backed it out").
//!   - `Reinstate`  — a revert-of-a-revert / reapply: verdict-of-a-verdict, the frontier
//!                    resolver's keystone case, found in the wild.
//!   - `Rationale`  — commit messages carrying "because / instead of / turns out":
//!                    design rationale that exists nowhere in the working tree.
//!   - `Debt`       — FIXME/HACK/kludge comments aged via blame: a long-surviving HACK
//!                    is a known-deficient-but-true belief with a forcing constraint.
//!   - `Doc`        — ADR/CHANGELOG/docs pointers, cheap escalation targets for tier 1+.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

// --- model ------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeadKind {
    Revert,
    Reinstate,
    Rationale,
    Debt,
    Doc,
}

impl LeadKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            LeadKind::Revert => "revert",
            LeadKind::Reinstate => "reinstate",
            LeadKind::Rationale => "rationale",
            LeadKind::Debt => "debt",
            LeadKind::Doc => "doc",
        }
    }
}

/// One candidate belief: verbatim evidence + refs, ranked within its kind.
#[derive(Debug, Clone)]
pub struct Lead {
    pub kind: LeadKind,
    pub title: String,
    /// Verbatim excerpt (commit message / comment line) — evidence, not a claim.
    pub evidence: String,
    /// `git:<sha12>` and/or `<path>:<line>` — same ref shapes corpus beliefs use.
    pub refs: Vec<String>,
    /// ISO author date (commit) or blame date (debt line).
    pub date: String,
    pub score: f32,
}

pub struct Harvest {
    pub repo_id: String,
    pub commits_scanned: usize,
    pub leads: Vec<Lead>,
}

impl Harvest {
    pub fn count(&self, k: LeadKind) -> usize {
        self.leads.iter().filter(|l| l.kind == k).count()
    }
}

// --- git plumbing -----------------------------------------------------------------------

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git").arg("-C").arg(repo).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).to_string();
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub sha: String,
    pub date: String,
    pub subject: String,
    pub body: String,
}

/// Full history, first-parent included merges and all; \x1f field / \x1e record separators
/// keep multi-line bodies intact.
fn log_all(repo: &Path) -> Vec<Commit> {
    let Some(raw) = git(repo, &["log", "--format=%H%x1f%aI%x1f%s%x1f%b%x1e"]) else {
        return Vec::new();
    };
    raw.split('\x1e')
        .filter_map(|rec| {
            let mut f = rec.trim_start_matches(['\n', ' ']).split('\x1f');
            let sha = f.next()?.trim().to_string();
            if sha.len() < 40 {
                return None;
            }
            Some(Commit {
                sha,
                date: f.next()?.to_string(),
                subject: f.next()?.to_string(),
                body: f.next().unwrap_or("").trim().to_string(),
            })
        })
        .collect()
}

fn short(sha: &str) -> &str {
    &sha[..sha.len().min(12)]
}

// --- reverts & reinstatements (pure parsing, testable) -----------------------------------

/// The sha named by git's stock "This reverts commit <sha>." body line, if any.
pub fn reverted_sha(body: &str) -> Option<String> {
    let idx = body.find("This reverts commit ")?;
    let rest = &body[idx + "This reverts commit ".len()..];
    let sha: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if sha.len() >= 7 {
        Some(sha)
    } else {
        None
    }
}

pub fn is_revert_subject(subject: &str) -> bool {
    subject.starts_with("Revert ") || subject.starts_with("revert:") || subject.starts_with("revert(")
}

pub fn is_reapply_subject(subject: &str) -> bool {
    subject.starts_with("Reapply ") || subject.starts_with("reapply:")
}

fn harvest_reverts(commits: &[Commit], by_sha: &HashMap<&str, &Commit>) -> Vec<Lead> {
    let mut leads = Vec::new();
    for c in commits {
        let target = reverted_sha(&c.body).and_then(|s| {
            // body shas are full 40-hex; prefix-match tolerates abbreviated ones
            by_sha.get(s.as_str()).copied().or_else(|| {
                by_sha.iter().find(|(k, _)| k.starts_with(s.as_str())).map(|(_, v)| *v)
            })
        });
        // body-only evidence must RESOLVE: squashed PR bodies quote inner "This reverts
        // commit <sha>" lines whose shas left history — those are not reverts of this commit
        let is_revert = is_revert_subject(&c.subject) || target.is_some();
        let is_reapply = is_reapply_subject(&c.subject);
        if !is_revert && !is_reapply {
            continue;
        }
        // a revert whose TARGET is itself a revert reinstates the original: verdict-of-a-verdict
        let reinstates =
            is_reapply || target.map_or(false, |t| is_revert_subject(&t.subject));
        let (kind, score) = if reinstates { (LeadKind::Reinstate, 1.2) } else { (LeadKind::Revert, 1.0) };
        let title = match target {
            Some(t) if reinstates => format!("Reinstated after revert: \"{}\"", t.subject),
            Some(t) => format!("Tried and reverted: \"{}\"", t.subject),
            None if reinstates => format!("Reinstated: \"{}\"", c.subject),
            None => format!("Reverted: \"{}\"", c.subject),
        };
        let mut refs = vec![format!("git:{}", short(&c.sha))];
        if let Some(t) = target {
            refs.push(format!("git:{}", short(&t.sha)));
        }
        leads.push(Lead {
            kind,
            title,
            evidence: excerpt(&c.subject, &c.body),
            refs,
            date: c.date.clone(),
            score,
        });
    }
    leads
}

// --- rationale commits (pure scoring, testable) -------------------------------------------

/// Marker phrases that signal a commit message carries rationale, not just a change summary.
/// Body hits weigh full; subject hits weigh 0.6 (subjects are terse, bodies explain).
const RATIONALE_MARKERS: &[&str] = &[
    "because",
    "instead of",
    "rather than",
    "turns out",
    "turned out",
    "no longer",
    "due to",
    "switch from",
    "switched from",
    "the reason",
    "didn't work",
    "doesn't work",
    "regression",
    "broke ",
    "tradeoff",
    "trade-off",
    "workaround",
    "decided",
    "b/c",
    "cuz ",
    "so that",
];

pub fn rationale_score(subject: &str, body: &str) -> f32 {
    let (s, b) = (subject.to_lowercase(), body.to_lowercase());
    let mut score = 0.0;
    for m in RATIONALE_MARKERS {
        if b.contains(m) {
            score += 1.0;
        } else if s.contains(m) {
            score += 0.6;
        }
    }
    // umbrella branch-merges (bodies stitching many sub-PRs) rack up markers by sheer
    // length without carrying one coherent decision — downweight, don't drop
    if umbrella_refs(body) >= 3 {
        score *= 0.3;
    }
    score
}

/// How many "(#123)" PR references the body carries.
pub fn umbrella_refs(body: &str) -> usize {
    body.match_indices("(#")
        .filter(|(i, _)| {
            body[i + 2..].chars().next().map_or(false, |c| c.is_ascii_digit())
        })
        .count()
}

fn harvest_rationale(commits: &[Commit]) -> Vec<Lead> {
    commits
        .iter()
        .filter(|c| {
            !c.subject.starts_with("Merge ")
                && !is_revert_subject(&c.subject)
                && !is_reapply_subject(&c.subject)
        })
        .filter_map(|c| {
            let score = rationale_score(&c.subject, &c.body);
            if score < 1.0 {
                return None;
            }
            Some(Lead {
                kind: LeadKind::Rationale,
                title: c.subject.clone(),
                evidence: excerpt(&c.subject, &c.body),
                refs: vec![format!("git:{}", short(&c.sha))],
                date: c.date.clone(),
                score,
            })
        })
        .collect()
}

fn excerpt(subject: &str, body: &str) -> String {
    let mut e = subject.to_string();
    if !body.is_empty() {
        e.push('\n');
        e.push_str(truncated(body, 400));
    }
    e
}

fn truncated(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// --- debt comments (git grep + blame age) -------------------------------------------------

/// (tag, weight): a long-surviving HACK/kludge is the prize; TODO is mostly aspiration noise.
const DEBT_TAGS: &[(&str, f32)] = &[
    ("HACK", 1.0),
    ("kludge", 1.0),
    ("FIXME", 0.9),
    ("workaround", 0.7),
    ("XXX", 0.6),
    ("TODO", 0.3),
];

pub fn debt_tag(line: &str) -> Option<(&'static str, f32)> {
    let lower = line.to_lowercase();
    DEBT_TAGS
        .iter()
        .filter(|(t, _)| {
            if t.chars().all(|c| c.is_ascii_uppercase()) {
                // tag conventions are case-sensitive AND word-bounded: `XXXXXX` tempfile
                // templates and base64 blobs contain XXX/TODO-shaped runs that aren't tags
                contains_bounded(line, t)
            } else {
                lower.contains(t)
            }
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .copied()
}

fn contains_bounded(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    let mut from = 0;
    while let Some(pos) = haystack[from..].find(needle) {
        let start = from + pos;
        let end = start + needle.len();
        let pre_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
        let post_ok = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if pre_ok && post_ok {
            return true;
        }
        from = end;
    }
    false
}

const DEBT_HITS_CAP: usize = 1500;
const DEBT_PER_FILE_CAP: usize = 20;

fn harvest_debt(repo: &Path, now_epoch: i64) -> Vec<Lead> {
    let grep = git(
        repo,
        &[
            "grep",
            "-nI",
            "--no-color",
            "-E",
            "FIXME|HACK|XXX|TODO|[Kk]ludge|[Ww]orkaround",
            "--",
            ".",
            ":(exclude)vendored",
            ":(exclude)vendor",
            ":(exclude)node_modules",
            ":(exclude)third_party",
            ":(exclude)*.lock",
            ":(exclude)package-lock.json",
            ":(exclude)pnpm-lock.yaml",
            ":(exclude)*.sum",
            ":(exclude)*.snapshot",
            ":(exclude)*.min.*",
            ":(exclude)*.jsonl",
        ],
    );
    let Some(grep) = grep else { return Vec::new() };

    // group hits per file so each file costs ONE blame call (multiple -L ranges)
    let mut by_file: HashMap<String, Vec<(u32, String)>> = HashMap::new();
    for line in grep.lines().take(DEBT_HITS_CAP) {
        let mut parts = line.splitn(3, ':');
        let (Some(file), Some(ln), Some(text)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        let Ok(ln) = ln.parse::<u32>() else { continue };
        let hits = by_file.entry(file.to_string()).or_default();
        if hits.len() < DEBT_PER_FILE_CAP {
            hits.push((ln, text.trim().to_string()));
        }
    }

    let mut leads = Vec::new();
    for (file, hits) in &by_file {
        let dates = blame_dates(repo, file, &hits.iter().map(|(l, _)| *l).collect::<Vec<_>>());
        for (ln, text) in hits {
            let Some((tag, weight)) = debt_tag(text) else { continue };
            let (date, epoch) = dates.get(ln).cloned().unwrap_or_default();
            let age_years = ((now_epoch - epoch).max(0) as f32) / (365.25 * 86400.0);
            leads.push(Lead {
                kind: LeadKind::Debt,
                title: format!("{tag} aged {age_years:.1}y: {file}:{ln}"),
                evidence: truncated(text, 300).to_string(),
                refs: vec![format!("{file}:{ln}")],
                date,
                score: weight * (1.0 + age_years),
            });
        }
    }
    leads
}

/// Blame dates for specific lines of one file: line → (ISO date, epoch).
fn blame_dates(repo: &Path, file: &str, lines: &[u32]) -> HashMap<u32, (String, i64)> {
    let mut args: Vec<String> = vec!["blame".into(), "--line-porcelain".into()];
    for l in lines {
        args.push("-L".into());
        args.push(format!("{l},{l}"));
    }
    args.push("--".into());
    args.push(file.into());
    let argrefs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let Some(out) = git(repo, &argrefs) else { return HashMap::new() };

    let mut dates = HashMap::new();
    let mut current_line: Option<u32> = None;
    for l in out.lines() {
        // header: "<40-hex sha> <orig> <final> [<num>]"
        let mut f = l.split(' ');
        if let (Some(sha), Some(_), Some(fin)) = (f.next(), f.next(), f.next()) {
            if sha.len() == 40 && sha.chars().all(|c| c.is_ascii_hexdigit()) {
                current_line = fin.parse().ok();
                continue;
            }
        }
        if let Some(epoch) = l.strip_prefix("author-time ").and_then(|e| e.parse::<i64>().ok()) {
            if let Some(ln) = current_line {
                dates.insert(ln, (iso_date(epoch), epoch));
            }
        }
    }
    dates
}

/// Civil date from a unix epoch (UTC, day precision is plenty for lead ranking).
fn iso_date(epoch: i64) -> String {
    let days = epoch.div_euclid(86400);
    // Howard Hinnant's civil_from_days
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

// --- doc pointers --------------------------------------------------------------------------

const DOCS_CAP: usize = 60;

fn harvest_docs(repo: &Path) -> Vec<Lead> {
    let Some(files) = git(repo, &["ls-files"]) else { return Vec::new() };
    let mut leads = Vec::new();
    for f in files.lines() {
        let lower = f.to_lowercase();
        let is_doc = lower.starts_with("docs/") && lower.ends_with(".md")
            || lower.contains("adr")
            || lower.starts_with("changelog")
            || lower.contains("/changelog");
        if !is_doc {
            continue;
        }
        let date = git(repo, &["log", "-1", "--format=%aI", "--", f])
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        leads.push(Lead {
            kind: LeadKind::Doc,
            title: format!("Doc: {f}"),
            evidence: String::new(),
            refs: vec![f.to_string()],
            date,
            score: 0.1,
        });
        if leads.len() >= DOCS_CAP {
            break;
        }
    }
    leads
}

// --- the harvest ----------------------------------------------------------------------------

pub fn harvest(repo: &Path, now_epoch: i64) -> Result<Harvest, String> {
    let repo_id = git(repo, &["rev-parse", "--show-toplevel"])
        .map(|r| r.trim().rsplit('/').next().unwrap_or("repo").to_string())
        .ok_or_else(|| format!("not a git repo: {}", repo.display()))?;
    let commits = log_all(repo);
    let by_sha: HashMap<&str, &Commit> = commits.iter().map(|c| (c.sha.as_str(), c)).collect();

    let mut leads = harvest_reverts(&commits, &by_sha);
    leads.extend(harvest_rationale(&commits));
    leads.extend(harvest_debt(repo, now_epoch));
    leads.extend(harvest_docs(repo));
    leads.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.date.cmp(&a.date))
    });
    Ok(Harvest { repo_id, commits_scanned: commits.len(), leads })
}

// --- rendering -------------------------------------------------------------------------------

fn jesc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\t' => o.push_str("\\t"),
            '\r' => {}
            c if (c as u32) < 0x20 => o.push(' '),
            c => o.push(c),
        }
    }
    o
}

/// Complete machine-readable harvest, one lead per line (greppable like the novelty ledger).
pub fn leads_json(h: &Harvest) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "{{\"schema\":\"onboard-leads/v0\",\"repo\":\"{}\",\"commits_scanned\":{},\"leads\":[\n",
        jesc(&h.repo_id),
        h.commits_scanned
    ));
    for (i, l) in h.leads.iter().enumerate() {
        let refs: Vec<String> = l.refs.iter().map(|r| format!("\"{}\"", jesc(r))).collect();
        s.push_str(&format!(
            "{{\"kind\":\"{}\",\"score\":{:.2},\"date\":\"{}\",\"title\":\"{}\",\"refs\":[{}],\"evidence\":\"{}\"}}{}\n",
            l.kind.as_str(),
            l.score,
            jesc(&l.date),
            jesc(&l.title),
            refs.join(","),
            jesc(&l.evidence),
            if i + 1 < h.leads.len() { "," } else { "" }
        ));
    }
    s.push_str("]}\n");
    s
}

/// Human report for the eyeball pass: grouped by kind, top `per_kind` each, score-ordered.
pub fn leads_md(h: &Harvest, per_kind: usize) -> String {
    let mut s = format!("# Onboard leads — {} (tier 0)\n\n", h.repo_id);
    s.push_str(&format!(
        "{} commits scanned · {} reverts · {} reinstatements · {} rationale · {} debt · {} docs\n",
        h.commits_scanned,
        h.count(LeadKind::Revert),
        h.count(LeadKind::Reinstate),
        h.count(LeadKind::Rationale),
        h.count(LeadKind::Debt),
        h.count(LeadKind::Doc),
    ));
    let sections: &[(LeadKind, &str)] = &[
        (LeadKind::Reinstate, "Reinstatements (verdict-of-a-verdict, in the wild)"),
        (LeadKind::Revert, "Reverts (supersession chains)"),
        (LeadKind::Rationale, "Rationale commits (design decisions, out-of-band)"),
        (LeadKind::Debt, "Aged debt comments (known-deficient-but-true)"),
        (LeadKind::Doc, "Doc pointers (tier-1+ escalation targets)"),
    ];
    for (kind, heading) in sections {
        let group: Vec<&Lead> = h.leads.iter().filter(|l| l.kind == *kind).collect();
        if group.is_empty() {
            continue;
        }
        s.push_str(&format!("\n## {heading} — {}\n\n", group.len()));
        for l in group.iter().take(per_kind) {
            let day = truncated(&l.date, 10);
            s.push_str(&format!("- **{}**  `{}`  ({day}, score {:.1})\n", l.title, l.refs.join(" "), l.score));
            if !l.evidence.is_empty() && l.evidence != l.title {
                for line in l.evidence.lines().take(6) {
                    s.push_str(&format!("  > {line}\n"));
                }
            }
        }
        if group.len() > per_kind {
            s.push_str(&format!("  …and {} more (see leads.json)\n", group.len() - per_kind));
        }
    }
    s
}

// === TIER 1: cheap-model escalation — lead → draft belief ====================================
//
// Tier 1 adds the judgment tier 0 refused to have: a local model (JUDGE_MODEL, same harness as
// the judgment linker) reads each selected lead PLUS richer deterministic context (the full
// commit message; the code around a debt comment) and either drafts a crisp, standalone claim
// or rejects the lead as noise. Drafts are still NOT beliefs — they go to drafts.{json,md} for
// the eyeball pass; committing survivors into the store is a separate, later step.

#[derive(Debug, Clone)]
pub struct Draft {
    pub lead: Lead,
    pub keep: bool,
    pub claim: String,
    /// One line on what in the evidence supports the claim (becomes the belief body later).
    pub why: String,
    /// decision | supersession | debt | episode — the model's read of the claim's shape.
    pub shape: String,
    pub asserted: f32,
}

/// Pick `limit` leads worth an LLM call: doc pointers excluded (they're tier-2 reading
/// targets, not single-claim evidence), debt deduped to ONE lead per file (a kludge cluster
/// is one belief, not five), and an equal quota per kind-group so high-scoring debt can't
/// starve rationale (leftover capacity spills over by score).
pub fn select_for_escalation(leads: &[Lead], limit: usize) -> Vec<Lead> {
    let mut groups: [Vec<&Lead>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut debt_seen_files = std::collections::HashSet::new();
    for l in leads {
        match l.kind {
            LeadKind::Revert | LeadKind::Reinstate => groups[0].push(l),
            LeadKind::Rationale => groups[1].push(l),
            LeadKind::Debt => {
                // TODO leads never escalate: a TODO is an aspiration, and the 7B judge reliably
                // rephrases it as a normative claim ("the script must...") no matter the prompt.
                // The deficiency tags (HACK/kludge/workaround/FIXME) are where real debt lives.
                if l.title.starts_with("TODO") {
                    continue;
                }
                let file = l.refs.first().map(|r| r.rsplit_once(':').map_or(r.as_str(), |(f, _)| f));
                if debt_seen_files.insert(file.unwrap_or("").to_string()) {
                    groups[2].push(l);
                }
            }
            LeadKind::Doc => {}
        }
    }
    // harvest() already sorted by score desc — group order inherits it
    let quota = limit.div_ceil(3);
    let mut picked: Vec<Lead> = Vec::new();
    let mut leftovers: Vec<&Lead> = Vec::new();
    for g in &groups {
        picked.extend(g.iter().take(quota).map(|l| (*l).clone()));
        leftovers.extend(g.iter().skip(quota));
    }
    leftovers.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    for l in leftovers {
        if picked.len() >= limit {
            break;
        }
        picked.push(l.clone());
    }
    picked.truncate(limit);
    picked
}

/// Richer deterministic context for one lead: the full commit message (git refs) and/or
/// the tracked code around a debt comment. Capped — qwen reads it all in one call.
pub fn lead_context(repo: &Path, lead: &Lead) -> String {
    let mut ctx = String::new();
    for r in &lead.refs {
        if let Some(sha) = r.strip_prefix("git:") {
            if let Some(msg) = git(repo, &["show", "-s", "--format=%B", sha]) {
                ctx.push_str(&format!("--- full message of {r} ---\n{}\n", truncated(msg.trim(), 2500)));
            }
        } else if let Some((file, ln)) = r.rsplit_once(':') {
            if let (Ok(ln), Some(content)) = (ln.parse::<usize>(), git(repo, &["show", &format!("HEAD:{file}")])) {
                let lines: Vec<&str> = content.lines().collect();
                let lo = ln.saturating_sub(9);
                let hi = (ln + 8).min(lines.len());
                ctx.push_str(&format!("--- {file} lines {}..{hi} ---\n", lo + 1));
                for (i, line) in lines[lo..hi].iter().enumerate() {
                    ctx.push_str(&format!("{:>5} {}\n", lo + i + 1, truncated(line, 160)));
                }
            }
        }
    }
    ctx
}

const ESCALATE_SYSTEM: &str = "You escalate a TIER-0 LEAD mined from a repo's git history into \
a candidate BELIEF for an engineer's long-term memory. The evidence is usually a change \
description (commit message, code comment) — your job is to EXTRACT the durable knowledge it \
reveals: a design decision and its rationale, a constraint discovered the hard way, a known \
kludge and what forces it, or a supersession (X replaced Y because Z). Write the claim as ONE \
crisp, STANDALONE proposition (1-2 sentences) a future engineer must know — not a change \
summary ('Added feature X'), but the decision/constraint behind it. Rules: ground the claim \
ONLY in the evidence, never invent specifics; name the module/system (never 'this commit'); \
carry the WHY inside the claim when the evidence states one; present tense for current state, \
past tense for episodes. If the evidence is a multi-bullet squash message, extract the SINGLE \
most durable decision and pair it with ITS OWN why — never stitch a claim from one bullet to a \
why from another. A TODO describing unimplemented work is an aspiration, not knowledge: \
keep=false unless it reveals a real present-day constraint or wrongness (e.g. 'X is incorrect \
because Y'). keep=true whenever something durable can be extracted — most rationale-bearing \
evidence qualifies. keep=false ONLY for genuine noise: meta-mentions of TODO/HACK as mere \
strings, test fixtures asserting on such strings, vendored third-party code, or content-free \
messages. Reply ONLY JSON: {\"keep\":bool,\"claim\":\"1-2 sentences\",\
\"why\":\"one line on what in the evidence supports it\",\"kind\":\"decision|supersession|\
debt|episode\",\"confidence\":0.0-1.0}";

/// Kind-specific extraction hint appended to the user prompt — a 7B model does noticeably
/// better told WHAT SHAPE of claim this lead usually yields.
fn shape_hint(kind: LeadKind) -> &'static str {
    match kind {
        LeadKind::Revert => "hint: state what was tried and backed out, and why if stated.",
        LeadKind::Reinstate => "hint: state what was un-reverted/reinstated and what that settled.",
        LeadKind::Rationale => "hint: state the decision AND its why (the 'because/instead of/b\\c' part).",
        LeadKind::Debt => "hint: state the kludge/limitation, where it lives, and the constraint forcing it.",
        LeadKind::Doc => "hint: state what this document governs.",
    }
}

/// One lead → one model call → one draft. Errors are strings (ollama down, non-JSON).
pub fn escalate_lead(repo: &Path, lead: &Lead, url: &str, model: &str) -> Result<Draft, String> {
    let user = format!(
        "LEAD kind={} date={} refs={}\ntitle: {}\n{}\nevidence:\n{}\n\ncontext:\n{}",
        lead.kind.as_str(),
        lead.date,
        lead.refs.join(" "),
        lead.title,
        shape_hint(lead.kind),
        lead.evidence,
        lead_context(repo, lead),
    );
    let v = memory_embed::chat_json(url, model, ESCALATE_SYSTEM, &user)?;
    let claim = v.get("claim").and_then(|c| c.as_str()).unwrap_or("").trim().to_string();
    // degenerate-output gate: a truncated/fragment "claim" is worse than a rejection
    let keep = v.get("keep").and_then(|k| k.as_bool()).unwrap_or(false) && claim.len() >= 30;
    Ok(Draft {
        lead: lead.clone(),
        keep,
        claim,
        why: v.get("why").and_then(|w| w.as_str()).unwrap_or("").trim().to_string(),
        shape: v.get("kind").and_then(|k| k.as_str()).unwrap_or("decision").to_string(),
        asserted: v.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.5) as f32,
    })
}

pub fn drafts_json(repo_id: &str, model: &str, drafts: &[Draft]) -> String {
    let mut s = format!(
        "{{\"schema\":\"onboard-drafts/v0\",\"repo\":\"{}\",\"model\":\"{}\",\"drafts\":[\n",
        jesc(repo_id),
        jesc(model)
    );
    for (i, d) in drafts.iter().enumerate() {
        let refs: Vec<String> = d.lead.refs.iter().map(|r| format!("\"{}\"", jesc(r))).collect();
        s.push_str(&format!(
            "{{\"keep\":{},\"shape\":\"{}\",\"confidence\":{:.2},\"claim\":\"{}\",\"why\":\"{}\",\"lead_kind\":\"{}\",\"lead_title\":\"{}\",\"refs\":[{}],\"date\":\"{}\",\"evidence\":\"{}\"}}{}\n",
            d.keep,
            jesc(&d.shape),
            d.asserted,
            jesc(&d.claim),
            jesc(&d.why),
            d.lead.kind.as_str(),
            jesc(&d.lead.title),
            refs.join(","),
            jesc(&d.lead.date),
            jesc(&d.lead.evidence),
            if i + 1 < drafts.len() { "," } else { "" }
        ));
    }
    s.push_str("]}\n");
    s
}

/// Read drafts.json back (the commit step runs OFF the reviewed file, so the eyeball pass can
/// delete bad drafts before anything reaches the store). Tolerates older files without
/// `evidence`. Returns kept drafts only.
pub fn load_kept_drafts(path: &Path) -> Result<Vec<Draft>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let v: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("bad drafts.json: {e}"))?;
    let arr = v.get("drafts").and_then(|d| d.as_array()).ok_or("drafts.json has no `drafts`")?;
    let s = |o: &serde_json::Value, k: &str| o.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let mut out = Vec::new();
    for d in arr {
        if !d.get("keep").and_then(|k| k.as_bool()).unwrap_or(false) {
            continue;
        }
        let claim = s(d, "claim");
        if claim.is_empty() {
            continue;
        }
        let kind = match s(d, "lead_kind").as_str() {
            "revert" => LeadKind::Revert,
            "reinstate" => LeadKind::Reinstate,
            "debt" => LeadKind::Debt,
            "doc" => LeadKind::Doc,
            _ => LeadKind::Rationale,
        };
        out.push(Draft {
            lead: Lead {
                kind,
                title: s(d, "lead_title"),
                evidence: s(d, "evidence"),
                refs: d
                    .get("refs")
                    .and_then(|r| r.as_array())
                    .map(|r| r.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                    .unwrap_or_default(),
                date: s(d, "date"),
                score: 0.0,
            },
            keep: true,
            claim,
            why: s(d, "why"),
            shape: s(d, "shape"),
            asserted: d.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.5) as f32,
        });
    }
    Ok(out)
}

pub fn drafts_md(repo_id: &str, model: &str, drafts: &[Draft]) -> String {
    let kept: Vec<&Draft> = drafts.iter().filter(|d| d.keep && !d.claim.is_empty()).collect();
    let skipped: Vec<&Draft> = drafts.iter().filter(|d| !d.keep || d.claim.is_empty()).collect();
    let mut s = format!("# Onboard drafts — {repo_id} (tier 1, {model})\n\n");
    s.push_str(&format!("{} escalated · {} kept · {} skipped\n\n## Kept\n\n", drafts.len(), kept.len(), skipped.len()));
    for d in kept {
        s.push_str(&format!("- **{}**\n  ({}, conf {:.1}) `{}`\n  why: {}\n", d.claim, d.shape, d.asserted, d.lead.refs.join(" "), d.why));
    }
    s.push_str("\n## Skipped\n\n");
    for d in skipped {
        s.push_str(&format!("- {} `{}`\n", d.lead.title, d.lead.refs.join(" ")));
    }
    s
}

// --- tests -----------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverted_sha_parses_stock_git_body() {
        let body = "Bad idea.\n\nThis reverts commit 1234567890abcdef1234567890abcdef12345678.";
        assert_eq!(reverted_sha(body).as_deref(), Some("1234567890abcdef1234567890abcdef12345678"));
        assert_eq!(reverted_sha("no revert here"), None);
        assert_eq!(reverted_sha("This reverts commit 12345."), None); // too short to trust
    }

    #[test]
    fn reinstatement_is_detected_via_revert_of_a_revert() {
        let orig = Commit {
            sha: "a".repeat(40),
            date: "2026-01-01T00:00:00+00:00".into(),
            subject: "feat: use polling".into(),
            body: String::new(),
        };
        let revert = Commit {
            sha: "b".repeat(40),
            date: "2026-01-02T00:00:00+00:00".into(),
            subject: "Revert \"feat: use polling\"".into(),
            body: format!("This reverts commit {}.", orig.sha),
        };
        let rerevert = Commit {
            sha: "c".repeat(40),
            date: "2026-01-03T00:00:00+00:00".into(),
            subject: "Revert \"Revert \"feat: use polling\"\"".into(),
            body: format!("This reverts commit {}.", revert.sha),
        };
        let commits = vec![orig.clone(), revert.clone(), rerevert.clone()];
        let by_sha: HashMap<&str, &Commit> = commits.iter().map(|c| (c.sha.as_str(), c)).collect();
        let leads = harvest_reverts(&commits, &by_sha);
        assert_eq!(leads.len(), 2);
        let re = leads.iter().find(|l| l.kind == LeadKind::Reinstate).expect("reinstate lead");
        assert!(re.refs.contains(&format!("git:{}", &"c".repeat(12))));
        assert!(leads.iter().any(|l| l.kind == LeadKind::Revert));
    }

    #[test]
    fn rationale_scoring_prefers_bodies_and_gates_noise() {
        assert!(rationale_score("fix typo", "") < 1.0);
        assert!(rationale_score("fix: handle nulls", "because the driver returns undef") >= 1.0);
        // subject-only marker is sub-threshold on its own
        assert!(rationale_score("switch from polling because reasons", "") < 1.3);
        // local idiom counts too
        assert!(rationale_score("feat: window repeats", "use last instead of lag b/c lag uses the whole partition") >= 2.0);
    }

    #[test]
    fn umbrella_merges_are_downweighted() {
        let umbrella = "* sub one (#101)\nbecause x\n* sub two (#102)\nturns out y\n* three (#103)";
        let focused = "because x, turns out y";
        assert_eq!(umbrella_refs(umbrella), 3);
        assert!(rationale_score("branch/merge (#200)", umbrella) < rationale_score("fix: one thing", focused));
    }

    #[test]
    fn debt_tag_picks_heaviest_and_respects_case() {
        assert_eq!(debt_tag("// TODO: HACK around the cache").map(|t| t.0), Some("HACK"));
        assert_eq!(debt_tag("# todo lowercase is not a tag"), None);
        assert_eq!(debt_tag("a Kludge for the ages").map(|t| t.0), Some("kludge"));
    }

    #[test]
    fn debt_tag_requires_word_boundaries_for_uppercase() {
        assert_eq!(debt_tag("tempfile('corpus-batch-XXXXXX')"), None);
        assert_eq!(debt_tag("sha512-YZoXXXevb5dJI"), None);
        assert_eq!(debt_tag("// XXX: revisit"), Some(("XXX", 0.6)));
    }

    #[test]
    fn escalation_selection_quotas_and_debt_file_dedup() {
        let mk = |kind, score: f32, file: &str| Lead {
            kind,
            title: format!("{file}"),
            evidence: String::new(),
            refs: vec![format!("{file}:1")],
            date: "2026-01-01".into(),
            score,
        };
        let mut leads = vec![
            mk(LeadKind::Debt, 9.0, "a.rs"),
            mk(LeadKind::Debt, 8.0, "a.rs"), // same file → deduped
            mk(LeadKind::Debt, 7.0, "b.rs"),
            mk(LeadKind::Rationale, 3.0, "r1"),
            mk(LeadKind::Rationale, 2.0, "r2"),
            mk(LeadKind::Revert, 1.0, "v1"),
            mk(LeadKind::Doc, 0.1, "docs/x.md"), // never escalated
        ];
        leads.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        leads.push(mk(LeadKind::Debt, 6.0, "c.rs"));
        leads.last_mut().unwrap().title = "TODO aged 6.0y: c.rs:1".into();
        let picked = select_for_escalation(&leads, 4);
        assert_eq!(picked.len(), 4);
        assert!(picked.iter().all(|l| l.kind != LeadKind::Doc));
        assert!(picked.iter().all(|l| !l.title.starts_with("TODO")));
        assert_eq!(picked.iter().filter(|l| l.title == "a.rs").count(), 1);
        // every group is represented despite debt's high scores
        assert!(picked.iter().any(|l| l.kind == LeadKind::Revert));
        assert!(picked.iter().any(|l| l.kind == LeadKind::Rationale));
    }

    #[test]
    fn squashed_pr_body_mentioning_a_gone_sha_is_not_a_revert() {
        let feat = Commit {
            sha: "d".repeat(40),
            date: "2026-01-04T00:00:00+00:00".into(),
            subject: "feat: big squash (#21)".into(),
            body: format!("* Revert \"inner try\"\n\nThis reverts commit {}.", "9".repeat(40)),
        };
        let commits = vec![feat];
        let by_sha: HashMap<&str, &Commit> = commits.iter().map(|c| (c.sha.as_str(), c)).collect();
        assert!(harvest_reverts(&commits, &by_sha).is_empty());
    }

    #[test]
    fn json_escapes_quotes_and_newlines() {
        assert_eq!(jesc("a \"b\"\nc\\d"), "a \\\"b\\\"\\nc\\\\d");
    }

    #[test]
    fn iso_date_civil_conversion() {
        assert_eq!(iso_date(0), "1970-01-01");
        assert_eq!(iso_date(1_700_000_000), "2023-11-14");
    }
}
