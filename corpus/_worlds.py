#!/usr/bin/env python3
"""Parallel-worlds frontier resolver + demonstrator (design §5).

A world is a SET OF HEADS over the one shared, immutable belief DAG. This script
loads a corpus's emitted beliefs (`beliefs/*.md`) and its `worlds.json`, then for
each world computes the live frontier: which beliefs are *current* vs *defeated*.

It is FRONTIER-RELATIVE and NON-MONOTONIC: a belief is defeated iff some
`supersedes`/`adjudicates` edge into it comes from a source that is itself live on
this world's frontier and whose supersession is not `suppress`ed. A later verdict
that defeats an earlier verdict therefore REINSTATES that verdict's target (the
verdict-of-a-verdict case). This is the correct "what is currently refuted"
computation — the naive INDEX refuted-list (any incoming `adjudicates`) is wrong
for chains; use THIS, not that list, as a label source.

What this PROVES: the representation is coherent — the same shared beliefs yield
different frontiers under different assumptions. What it does NOT prove: that an L3
reducer *behaves* world-relatively. The two divergent consensuses in
`worlds.json -> reduction_fixtures` are TARGETS for when the reducer runs, not a
demonstration. Necessary substrate, not sufficient proof.

Usage:  python3 corpus/_worlds.py <corpus-dir>
"""
import json, os, re, sys

DEFEATING = {"supersedes", "adjudicates"}

def load_beliefs(corpus_dir):
    """Parse beliefs/*.md → {slug: {claim, edges:[(kind, target_slug)]}}."""
    beliefs = {}
    bdir = os.path.join(corpus_dir, "beliefs")
    for fn in sorted(os.listdir(bdir)):
        if not fn.endswith(".md"):
            continue
        text = open(os.path.join(bdir, fn)).read()
        slug = re.search(r"^slug:\s*(\S+)", text, re.M).group(1)
        cm = re.search(r"text:\s*>-\s*\n((?:\s{4}.*\n)+)", text)
        claim = " ".join(l.strip() for l in cm.group(1).splitlines()) if cm else ""
        edges = []
        for m in re.finditer(r"-\s*kind:\s*(\w+)\s*\n\s*target:\s*\S+\s*#\s*(\S+)", text):
            edges.append((m.group(1), m.group(2)))
        beliefs[slug] = {"claim": claim, "edges": edges}
    return beliefs

def frontier(beliefs, suppress):
    """Live set under `suppress` (slugs whose defeating edges are dropped).
    Fixpoint over the acyclic adjudication/supersession chains."""
    suppress = set(suppress or [])
    # incoming defeating edges: target -> [source]
    incoming = {s: [] for s in beliefs}
    for src, b in beliefs.items():
        for kind, tgt in b["edges"]:
            if kind in DEFEATING and tgt in beliefs:
                incoming[tgt].append(src)
    live = set(beliefs)
    for _ in range(len(beliefs) + 5):  # converges for DAGs
        defeated = {t for t, srcs in incoming.items()
                    if any(s in live and s not in suppress for s in srcs)}
        nl = set(beliefs) - defeated
        if nl == live:
            break
        live = nl
    return live, defeated

def main(corpus_dir):
    beliefs = load_beliefs(corpus_dir)
    wf = os.path.join(corpus_dir, "worlds.json")
    if not os.path.exists(wf):
        raise SystemExit(f"no worlds.json in {corpus_dir}")
    spec = json.load(open(wf))
    worlds = spec["worlds"]
    name = os.path.basename(corpus_dir.rstrip("/"))

    # resolve every world's frontier
    frontiers = {}
    for w, cfg in worlds.items():
        live, defeated = frontier(beliefs, cfg.get("suppress"))
        frontiers[w] = (live, defeated)

    # focus = beliefs whose live-status DIFFERS across worlds (+ any suppressed)
    focus = set()
    for s in beliefs:
        states = {w: (s in frontiers[w][0]) for w in worlds}
        if len(set(states.values())) > 1:
            focus.add(s)
    for cfg in worlds.values():
        focus.update(cfg.get("suppress", []))

    print(f"\n=== {name}: frontier by world ===")
    for w, cfg in worlds.items():
        live, defeated = frontiers[w]
        tag = " (default)" if cfg.get("default") else ""
        print(f"\n• world `{w}`{tag}")
        print(f"  assumption: {cfg.get('assumption','—')}")
        if cfg.get("suppress"):
            print(f"  suppress (supersession dropped): {', '.join(cfg['suppress'])}")
        print(f"  {len(live)}/{len(beliefs)} beliefs live on this frontier")
        for s in sorted(focus):
            status = "live  " if s in live else "DEFEAT"
            print(f"    [{status}] {s}")
    # divergence summary
    print(f"\n=== divergence (beliefs whose current-status flips between worlds) ===")
    for s in sorted(focus):
        row = {w: ("live" if s in frontiers[w][0] else "defeated") for w in worlds}
        if len(set(row.values())) > 1:
            cells = "  ".join(f"{w}={st}" for w, st in row.items())
            print(f"  {s}:  {cells}")
    print(f"\n(reduction_fixtures in worlds.json are TARGETS for the L3 reducer, not demonstrated here.)")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: python3 corpus/_worlds.py <corpus-dir>")
    main(os.path.abspath(sys.argv[1]))
