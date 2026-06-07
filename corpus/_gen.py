#!/usr/bin/env python3
"""Generic corpus generator + schema validator.

Reads a `beliefs.json` (written by a harvest agent) for one repo, VALIDATES it
against the shared frontmatter contract, and emits the markdown belief files via
_belief_lib. This is the "one schema contract, two emitters" guard from the
eval-readiness hand-off: every JSON-sourced corpus passes through this validator,
so an agent's hand-written JSON cannot drift into a shape the parser misreads.

Usage:  python3 corpus/_gen.py <corpus-dir>
        (expects <corpus-dir>/beliefs.json; writes <corpus-dir>/beliefs/*.md + INDEX.md)

beliefs.json shape:
{
  "title": str, "blurb": str, "subject_repo": str,
  "sessions": { "<key>": ["<session-id>", "<YYYY-MM-DD>", "<desc>"], ... },
  "beliefs": [ {
     "slug": str, "claim": str, "body": str,
     "author": {"kind":"agent|human|reducer", "id":str, "model":str?},
     "session": "<key>", "turn": int,
     "directness": "stated|inferred|reduced",
     "obs": int, "weight": float, "asserted": float|null,
     "valid_time": null | ["<start>","<end>"],
     "edges": [ ["supports|attacks|supersedes|derived_from|refines|adjudicates", "<target-slug>"], ... ],
     "refs": [str, ...],
     "scope": bool?            # default false; true => kind: project-scope
  }, ... ]
}
"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from _belief_lib import belief, generate, reset

EDGE_KINDS = {"supports", "attacks", "supersedes", "derived_from", "refines", "adjudicates"}
AUTHOR_KINDS = {"agent", "human", "reducer"}
DIRECTNESS = {"stated", "inferred", "reduced"}

def validate(data, path):
    errs = []
    for k in ("title", "blurb", "subject_repo", "sessions", "beliefs"):
        if k not in data:
            errs.append(f"missing top-level key: {k}")
    if errs:
        raise SystemExit(f"[{path}] " + "; ".join(errs))
    sess_keys = set(data["sessions"])
    for key, v in data["sessions"].items():
        if not (isinstance(v, list) and len(v) == 3):
            errs.append(f"session {key} must be [id, date, desc]")
    slugs = [b.get("slug") for b in data["beliefs"]]
    slugset = set(slugs)
    if len(slugs) != len(slugset):
        dupes = {s for s in slugs if slugs.count(s) > 1}
        errs.append(f"duplicate slugs: {sorted(dupes)}")
    for b in data["beliefs"]:
        s = b.get("slug", "<?>")
        for k in ("slug", "claim", "author", "session", "turn", "directness", "obs", "weight", "edges", "refs"):
            if k not in b:
                errs.append(f"{s}: missing field {k}")
        a = b.get("author", {})
        if a.get("kind") not in AUTHOR_KINDS:
            errs.append(f"{s}: bad author.kind {a.get('kind')!r}")
        if b.get("directness") not in DIRECTNESS:
            errs.append(f"{s}: bad directness {b.get('directness')!r}")
        if b.get("session") not in sess_keys:
            errs.append(f"{s}: unknown session {b.get('session')!r}")
        if "asserted" not in b:
            errs.append(f"{s}: missing field asserted (use null if none)")
        vt = b.get("valid_time", None)
        if vt is not None and not (isinstance(vt, list) and len(vt) == 2):
            errs.append(f"{s}: valid_time must be null or [start,end]")
        for e in b.get("edges", []):
            if not (isinstance(e, list) and len(e) == 2):
                errs.append(f"{s}: edge must be [kind, target]: {e!r}"); continue
            if e[0] not in EDGE_KINDS:
                errs.append(f"{s}: bad edge kind {e[0]!r}")
            if e[1] not in slugset:
                errs.append(f"{s}: edge target {e[1]!r} not a known slug")
    if errs:
        raise SystemExit(f"[{path}] schema errors:\n  - " + "\n  - ".join(errs))

def main(corpus_dir):
    path = os.path.join(corpus_dir, "beliefs.json")
    data = json.load(open(path))
    validate(data, path)
    reset()
    for b in data["beliefs"]:
        belief(
            slug=b["slug"], claim=b["claim"], body=b.get("body", ""),
            author=b["author"], session=b["session"], turn=b["turn"],
            directness=b["directness"], obs=b["obs"], weight=b["weight"],
            asserted=b.get("asserted"),
            valid_time=tuple(b["valid_time"]) if b.get("valid_time") else None,
            edges=[tuple(e) for e in b.get("edges", [])],
            refs=b.get("refs", []), scope=bool(b.get("scope", False)),
        )
    generate(
        title=data["title"], blurb=data["blurb"], subject_repo=data["subject_repo"],
        out_dir=os.path.join(corpus_dir, "beliefs"),
        sessions={k: tuple(v) for k, v in data["sessions"].items()},
        generator="corpus/_gen.py (from beliefs.json)",
    )

if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: python3 corpus/_gen.py <corpus-dir>")
    main(os.path.abspath(sys.argv[1]))
