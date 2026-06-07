#!/usr/bin/env python3
"""Seed the tree-sitter-perl Layer-0 belief corpus.

tree-sitter-perl is the Perl grammar (grammar.js + a hand-written C external
scanner) that perl-lsp is built on. This corpus is small and seeded mainly by
an ADVERSARIAL HARVEST: a workflow ran an underpowered model (claude-haiku-4-5,
NO tools) over tricky grammar/scanner snippets, forced confident claims, and a
tool-using adjudicator checked each against the real code. We keep the ones it
got WRONG (authored as the weak model) plus the grounding facts that refute
them. See ../../docs/design/belief-memory.md.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from _belief_lib import belief, generate, reset, AGENT, REDUCER

reset()
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "beliefs")
HAIKU = {"kind": "agent", "id": "claude-haiku-4-5", "model": "claude-haiku-4-5"}

SESSIONS = {
    "T1": ("ts-perl-orient-2026-06-04", "2026-06-04", "orienting on the grammar + the hand-written external scanner"),
    "T2": ("adversarial-haiku-2026-06-04", "2026-06-04", "adversarial harvest: underpowered model (no tools) on grammar.js + scanner.c"),
}

# ---------------------------------------------------------------------------
# Grounding facts (verified). These double as the ground truth the verdicts cite.
# ---------------------------------------------------------------------------
belief(slug="ts-perl-identity", session="T1", turn=1, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["README.md", "grammar.js", "src/scanner.c"],
    claim="tree-sitter-perl is the tree-sitter grammar for Perl: a generated parser from grammar.js plus a hand-written C external scanner (src/scanner.c) for the context-sensitive bits (heredocs, POD, quote-like, regex). perl-lsp is built on it (via the ts-parser-perl crate).",
    body="The scanner is where Perl's lexer-hostile constructs live; the grammar.js handles the context-free structure with GLR conflicts for the genuinely ambiguous parts.")

belief(slug="scanner-heredoc-fifo-bounded", session="T1", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["src/scanner.c:240-262"],
    claim="The external scanner queues pending heredocs in a bounded FIFO (HEREDOC_QUEUE_MAX = 8). On overflow it deliberately OVERWRITES the last slot rather than dropping — a DOCUMENTED graceful degradation: the first MAX-1 bodies parse correctly, the last greedily swallows the overflow (wrong but BOUNDED), and code after the block stays in sync. HEREDOC_START is primed only when the first heredoc is enqueued (count==1).",
    body="The 23-line comment at scanner.c:240 spells out the overflow design. This is the ground truth for [[haiku-heredoc-overflow-is-bug]].")

belief(slug="grammar-glr-conflicts-not-structural", session="T1", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.85, asserted=0.9, valid_time=None, edges=[],
    refs=["grammar.js:196-197", "grammar.js:510-519", "grammar.js:669-672"],
    claim="Genuine grammar ambiguities (e.g. preinc vs postinc, both prec 24) are resolved at PARSE TIME via declared GLR conflicts, not by structural exclusion: because `_term` includes `preinc_expression`, the parser DOES attempt `++$x` as a postinc whose operand is the preinc, and that branch just fails when no trailing `++` arrives. tree-sitter has NO rule-order tiebreaker — a same-precedence conflict with conflicting associativity is a build error, not a silently-ordered choice.",
    body="Ground truth for [[haiku-postinc-structurally-impossible]] and [[haiku-prec-rule-order-tiebreaker]]. The associativity of an alternative resolves a conflict only when it's the sole active reduce at that state.")

# ---------------------------------------------------------------------------
# ADVERSARIAL HARVEST (T2) — what the underpowered model got WRONG.
# ---------------------------------------------------------------------------
belief(slug="haiku-postinc-structurally-impossible", session="T2", turn=2, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.95, valid_time=None, edges=[],
    refs=["grammar.js:669-672"],
    claim="When parsing `++$x`, the parser picks preinc_expression because the prefix-operator-before-term structure matches only that rule; `++` cannot follow a term here, so postinc_expression is STRUCTURALLY IMPOSSIBLE and the declared conflict is essentially redundant.",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.95. Conclusion (preinc chosen) is right; the reasoning is wrong.")

belief(slug="postinc-is-glr-attempted", session="T2", turn=3, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "haiku-postinc-structurally-impossible"),
           ("attacks", "haiku-postinc-structurally-impossible"),
           ("supports", "grammar-glr-conflicts-not-structural")],
    refs=["grammar.js:510-519", "grammar.js:196-197"],
    claim="VERDICT: the conclusion is right but the reasoning is FALSE. postinc_expression is NOT structurally impossible for `++$x`: since `_term` includes `preinc_expression`, the GLR parser actively pursues a postinc whose operand is the whole preinc `++$x`, waiting for a trailing `++`/`--`; it fails only because none arrives. The conflict declaration at line 197 exists precisely because the ambiguity is real and must be resolved at runtime.",
    body="Defeats [[haiku-postinc-structurally-impossible]]; supports [[grammar-glr-conflicts-not-structural]]. Right-answer-wrong-reason at 0.95 — the model reasoned about token shape and never modeled GLR's parallel-stack exploration (out-of-snippet knowledge).")

belief(slug="haiku-prec-rule-order-tiebreaker", session="T2", turn=4, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.72, valid_time=None, edges=[],
    refs=["grammar.js:641-653"],
    claim="Mixing prec.left and prec.right at the same numeric level inside one choice() is fine: the active alternative's associativity resolves the conflict, with RULE ORDER as the tiebreaker when both are active.",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.72 — notably its LOWEST-confidence claim in the run, and still wrong on the tiebreaker part. (A small calibration win: it hedged most where it was on the shakiest ground.)")

belief(slug="no-rule-order-tiebreaker", session="T2", turn=5, author=AGENT,
    directness="stated", obs=1, weight=0.9, asserted=0.9, valid_time=None,
    edges=[("adjudicates", "haiku-prec-rule-order-tiebreaker"),
           ("attacks", "haiku-prec-rule-order-tiebreaker"),
           ("supports", "grammar-glr-conflicts-not-structural")],
    refs=["~/personal/tree-sitter/crates/generate/src/build_tables/build_parse_table.rs:786-800"],
    claim="VERDICT: FALSE on the tiebreaker. tree-sitter has NO rule-order tiebreaker anywhere. A single active reduce's associativity resolves a shift/reduce cleanly, but if two same-precedence alternatives with conflicting associativity are both active reduces, the `_ => {}` arm in build_parse_table.rs fires (does nothing) and the conflict is left UNRESOLVED — a grammar build error, not an order-decided pick.",
    body="Defeats [[haiku-prec-rule-order-tiebreaker]]; supports [[grammar-glr-conflicts-not-structural]]. Refuting this required reading the tree-sitter CORE repo (the adjudicator cross-repo'd to build_parse_table.rs) — the weak model couldn't have known it from grammar.js alone.")

belief(slug="haiku-heredoc-overflow-is-bug", session="T2", turn=6, author=HAIKU,
    directness="inferred", obs=1, weight=0.4, asserted=0.95, valid_time=None, edges=[],
    refs=["src/scanner.c:252-262"],
    claim="The heredoc overflow path is a BUG: when the queue is full it overwrites the last slot without re-arming HEREDOC_START, violating the FSM initialization invariant.",
    body="HARVESTED from claude-haiku-4-5 (no tools), asserted 0.95. An invented bug — it read the overflow branch and missed the comment directly above it.")

belief(slug="heredoc-overflow-is-intentional", session="T2", turn=7, author=AGENT,
    directness="stated", obs=1, weight=0.92, asserted=0.92, valid_time=None,
    edges=[("adjudicates", "haiku-heredoc-overflow-is-bug"),
           ("attacks", "haiku-heredoc-overflow-is-bug"),
           ("supports", "scanner-heredoc-fifo-bounded")],
    refs=["src/scanner.c:240-262"],
    claim="VERDICT: FALSE — not a bug. The 23-line comment at scanner.c:240 documents the overwrite as INTENTIONAL graceful degradation. HEREDOC_START needs no re-arming because the FSM is already in heredoc mode (START was set when count first hit 1); overflow just retargets the final terminator. Bounded-but-wrong by design, no desync.",
    body="Defeats [[haiku-heredoc-overflow-is-bug]]; supports [[scanner-heredoc-fifo-bounded]]. The sharpest lesson of the harvest: the model cried 'bug' while IGNORING the doc-comment that explained the design sitting directly above the code — the mirror image of the wrong-docs thread (there a trusted doc was wrong; here a correct doc was ignored). Confident-wrong at 0.95.")

generate(
    title="tree-sitter-perl",
    blurb="The tree-sitter grammar for Perl (grammar.js + a hand-written C external scanner) that "
          "perl-lsp builds on; seeded mainly by an adversarial harvest of what an underpowered model "
          "got wrong reading the grammar and scanner.",
    subject_repo="~/personal/tree-sitter-perl",
    out_dir=OUT,
    sessions=SESSIONS,
)
