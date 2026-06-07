"""
Shared authoring library for Layer-0 belief corpora.

A corpus generator imports this, declares its sessions + beliefs via belief(),
then calls generate(). Emits one markdown-file-per-belief (YAML frontmatter =
BeliefBody) plus a generated INDEX.md. See ../docs/design/belief-memory.md.

Fidelity notes:
  - content-addressed id = "b_" + sha256(observation)[:12], where `observation`
    is the write-time BeliefBody MINUS edges and MINUS the derived
    observation_count: claim + author + provenance(txn_time, valid_time, source,
    refs) + confidence(directness, source_weight, asserted) + coord + kind. This
    is OBSERVATION identity (design §3), not proposition identity: two independent
    observations of the same claim get DIFFERENT ids (their provenance differs) and
    are linked by edges; "same proposition" is a derived L2/L3 clustering judgment,
    never an L0 string-equality. Edges are excluded so later adjudication doesn't
    churn the id; `observation_count` is excluded because it is DERIVED (corroboration
    is counted at read time over linked observations, not baked into identity).
  - authors are symmetric: AGENT | HUMAN | REDUCER.
  - confidence stored structurally + the asserted float (kept, not trusted).
  - edges are the spine: supports/attacks/supersedes/derived_from/refines/adjudicates.
  - `scope=True` marks a belief that states the PROJECT'S SCOPE at a point in
    time; the index renders these as a scope timeline ordered by valid_time.
    Scope expansion is modeled as a later scope belief `supersedes` the earlier.
"""

import hashlib
import json
import os
import textwrap

AGENT = {"kind": "agent", "id": "claude-opus-4-8", "model": "claude-opus-4-8"}
HUMAN = {"kind": "human", "id": "veesh"}
REDUCER = {"kind": "reducer", "id": "reducer-opus", "model": "claude-opus-4-8"}

_B = []

def reset():
    _B.clear()

def belief(*, slug, claim, author, session, turn, directness, obs, weight,
           asserted, valid_time=None, edges=None, refs=None, body="", scope=False):
    _B.append(dict(slug=slug, claim=claim, author=author, session=session, turn=turn,
                   directness=directness, obs=obs, weight=weight, asserted=asserted,
                   valid_time=valid_time, edges=edges or [], refs=refs or [],
                   body=body, scope=scope))

def _obs_id(b, sessions):
    """Observation identity: hash the write-time belief body, MINUS edges and the
    derived observation_count. Same claim + different provenance => different id."""
    sess_id, sess_date, _ = sessions[b["session"]]
    a = b["author"]
    key = {
        "claim": b["claim"],
        "author": {"kind": a["kind"], "id": a["id"], "model": a.get("model")},
        "provenance": {
            "txn_time": f"{sess_date}T12:00:00Z",
            "valid_time": list(b["valid_time"]) if b["valid_time"] else None,
            "source": {"kind": "conversation", "session": sess_id, "turn": b["turn"]},
            "refs": list(b["refs"]),
        },
        "confidence": {
            "directness": b["directness"],
            "source_weight": b["weight"],
            "asserted": b["asserted"],
        },
        "coord": None,
        "kind": "project-scope" if b["scope"] else "belief",
    }
    canon = json.dumps(key, sort_keys=True, ensure_ascii=False, separators=(",", ":"))
    return "b_" + hashlib.sha256(canon.encode("utf-8")).hexdigest()[:12]

def _wrap(text, indent):
    return "\n".join(indent + ln for ln in textwrap.wrap(text, width=92 - len(indent)))

def _author_yaml(a):
    lines = [f"  kind: {a['kind']}", f"  id: {a['id']}"]
    if "model" in a:
        lines.append(f"  model: {a['model']}")
    return "\n".join(lines)

def _emit(b, slug2id, sessions, out):
    bid = slug2id[b["slug"]]
    sess_id, sess_date, _ = sessions[b["session"]]
    fm = ["---", f"id: {bid}", f"slug: {b['slug']}"]
    if b["scope"]:
        fm.append("kind: project-scope")
    fm += ["claim:", "  kind: text", "  text: >-", _wrap(b["claim"], "    "),
           "author:", _author_yaml(b["author"]), "provenance:",
           f"  txn_time: {sess_date}T12:00:00Z"]
    if b["valid_time"] is None:
        fm.append("  valid_time: null")
    else:
        s, e = b["valid_time"]
        fm += ["  valid_time:", f"    start: {s}", f"    end: {e}"]
    fm += ["  source:", "    kind: conversation", f"    session: {sess_id}",
           f"    turn: {b['turn']}"]
    if b["refs"]:
        fm.append("  refs:")
        fm += [f"    - {r}" for r in b["refs"]]
    else:
        fm.append("  refs: []")
    dfrom = [slug2id[t] for (k, t) in b["edges"] if k == "derived_from"]
    if dfrom:
        fm.append("  derived_from:")
        fm += [f"    - {d}" for d in dfrom]
    else:
        fm.append("  derived_from: []")
    fm += ["confidence:", f"  directness: {b['directness']}",
           f"  observation_count: {b['obs']}", f"  source_weight: {b['weight']}",
           f"  asserted: {b['asserted'] if b['asserted'] is not None else 'null'}"]
    if b["edges"]:
        fm.append("edges:")
        for (k, t) in b["edges"]:
            fm += [f"  - kind: {k}", f"    target: {slug2id[t]}   # {t}"]
    else:
        fm.append("edges: []")
    fm += ["coord: null", "---", "", b["body"], ""]
    with open(os.path.join(out, b["slug"] + ".md"), "w") as f:
        f.write("\n".join(fm))

def generate(*, title, blurb, subject_repo, out_dir, sessions, generator="_seed.py"):
    os.makedirs(out_dir, exist_ok=True)
    # self-cleaning: drop belief files from a previous run (e.g. renamed slugs)
    import glob
    keep = {b["slug"] + ".md" for b in _B}
    for f in glob.glob(os.path.join(out_dir, "*.md")):
        if os.path.basename(f) not in keep:
            os.remove(f)
    slug2id = {b["slug"]: _obs_id(b, sessions) for b in _B}
    for b in _B:
        _emit(b, slug2id, sessions, out_dir)

    mark = {"agent": "🤖", "human": "🧑", "reducer": "⚗️"}
    idx = [f"# {title} — belief corpus INDEX (generated)", "",
           f"> {blurb}", ">",
           f"> Generated by `{generator}` (do not hand-edit). The MEMORY.md / reduction-view",
           "> analog over the L0 belief log; not the source of truth.", "",
           f"Subject repo: `{subject_repo}`. "
           f"**{len(_B)} beliefs** across **{len(sessions)} sessions**, single world `main`.",
           "", "Author legend: 🤖 agent · 🧑 human · ⚗️ reducer (derived).", ""]

    # --- scope timeline (the "scope at any given time" view) ---
    scoped = [b for b in _B if b["scope"]]
    if scoped:
        scoped.sort(key=lambda b: (b["valid_time"][0] if b["valid_time"] else "0000"))
        idx += ["## Project scope over time", "",
                "_Each row is the project's scope as believed during that window "
                "(`valid_time`). A later scope belief `supersedes` the earlier — scope "
                "expansion is non-destructive revision._", ""]
        for b in scoped:
            if b["valid_time"]:
                s, e = b["valid_time"]
                window = f"{s} → {'now' if e.startswith('2999') else e}"
            else:
                window = "(no window)"
            idx.append(f"- **{window}** — `{b['slug']}`: {b['claim']}")
        idx.append("")

    # --- per-session listing (chronological) ---
    by_sess = {}
    for b in _B:
        by_sess.setdefault(b["session"], []).append(b)
    for skey in sorted(sessions, key=lambda k: sessions[k][1]):
        sid, sdate, desc = sessions[skey]
        if skey not in by_sess:
            continue
        idx += [f"## {skey} · {sdate} · {sid}", f"_{desc}_", ""]
        for b in by_sess[skey]:
            edge_str = ""
            if b["edges"]:
                edge_str = " — " + ", ".join(f"{k}→{t}" for (k, t) in b["edges"])
            sc = " 🔭" if b["scope"] else ""
            idx.append(f"- {mark[b['author']['kind']]} `{slug2id[b['slug']]}` "
                       f"**[[{b['slug']}]]**{sc} (d={b['directness']}, obs={b['obs']}, "
                       f"w={b['weight']}){edge_str}")
        idx.append("")

    # --- falsified beliefs: verdict gold labels (the calibration signal) ---
    id2b = {slug2id[b["slug"]]: b for b in _B}
    verdicts = [(b, t) for b in _B for (k, t) in b["edges"] if k == "adjudicates"]
    if verdicts:
        idx += ["## Refuted beliefs (calibration + entrenchment references)", "",
                "_Beliefs a later verdict declared WRONG — a misread of the code, or a hunch "
                "since refuted. Each was held at some `asserted` confidence and then defeated; "
                "that (asserted vs structural-envelope vs outcome) is the B5 calibration datum "
                "(§3a): confident-and-wrong is the signal an epistemic store must keep. There is "
                "no gold here — a verdict is itself a revisable belief (some are later overturned; "
                "see verdict-of-a-verdict). These are NOT supersessions — the refuted belief was "
                "never true, so it carries no valid_time; only the `adjudicates` edge defeats it._", ""]
        for v, t in verdicts:
            w = id2b[slug2id[t]]
            idx.append(f"- ❌ `{w['slug']}` (asserted **{w['asserted']}**, d={w['directness']}) "
                       f"— adjudicated false by `{v['slug']}`: {v['claim']}")
        idx.append("")

    # --- notable structure: supersessions + open conflicts ---
    sup = [(b["slug"], t) for b in _B for (k, t) in b["edges"] if k == "supersedes"]
    adj_targets = {slug2id[t] for b in _B for (k, t) in b["edges"] if k == "adjudicates"}
    open_conf = [(b["slug"], t) for b in _B for (k, t) in b["edges"]
                 if k == "attacks" and slug2id[t] not in adj_targets]
    idx += ["## Notable epistemic structure", ""]
    if sup:
        idx.append("**Supersessions (revision over time):** "
                   + "; ".join(f"`{a}` ⇒ `{b}`" for a, b in sup))
    if open_conf:
        idx.append("")
        idx.append("**Open (un-adjudicated) conflicts:** "
                   + "; ".join(f"`{a}` attacks `{b}`" for a, b in open_conf))
    idx.append("")
    with open(os.path.join(os.path.dirname(out_dir.rstrip("/")), "INDEX.md"), "w") as f:
        f.write("\n".join(idx) + "\n")

    print(f"wrote {len(_B)} beliefs to {out_dir}")
    print(f"wrote {os.path.join(os.path.dirname(out_dir.rstrip('/')), 'INDEX.md')}")
