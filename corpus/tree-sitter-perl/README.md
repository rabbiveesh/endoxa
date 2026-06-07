# tree-sitter-perl — belief corpus

Seeded from `~/personal/tree-sitter-perl` (the tree-sitter grammar for Perl —
`grammar.js` + a hand-written C external scanner — that `perl-lsp` builds on).
9 beliefs across 2 sessions. This corpus exists mainly as an **adversarial
harvest**: it's small on purpose, and weighted toward *mistaken* beliefs an
underpowered model produced.

## How it was built

A multi-agent workflow (`adversarial-wrong-belief-harvest`) ran an underpowered
model (**claude-haiku-4-5, no tools**) over tricky `grammar.js` / `scanner.c`
snippets, forced confident claims, then a tool-using adjudicator checked each
against the real code. The 3 perl-lsp errors live in that corpus; the 3
tree-sitter-perl errors are here, each authored *as the Haiku model*
(`source_weight 0.4`) and defeated by a verdict that cites the code.

## Contents

**Grounding facts** (verified — they double as the verdicts' ground truth):
- `ts-perl-identity` — grammar + external scanner; perl-lsp builds on it.
- `scanner-heredoc-fifo-bounded` — the heredoc queue is a bounded FIFO; overflow
  overwrites the last slot as *documented, intentional* graceful degradation.
- `grammar-glr-conflicts-not-structural` — ambiguities resolve at parse time via
  declared GLR conflicts; tree-sitter has **no** rule-order tiebreaker.

**Falsified beliefs** (verdict gold labels — see `INDEX.md`):
- `postinc-is-glr-attempted` defeats `haiku-postinc-structurally-impossible`
  (0.95) — right answer, wrong reasoning: postinc *is* GLR-attempted for `++$x`,
  not structurally excluded.
- `no-rule-order-tiebreaker` defeats `haiku-prec-rule-order-tiebreaker` (0.72) —
  invented a "rule order tiebreaker" that doesn't exist (refuting it needed the
  tree-sitter *core* repo).
- `heredoc-overflow-is-intentional` defeats `haiku-heredoc-overflow-is-bug`
  (0.95) — **invented a bug**: cried "FSM invariant violated" while ignoring the
  23-line comment directly above that documents the design.

The sharpest lesson: the model's wrongness clustered where the justification
reached *beyond the snippet* — and in the heredoc case it ignored a correct doc
sitting right there (the mirror image of the wrong-docs thread in the other
corpora). Regenerate with `python3 _seed.py` (shared machinery in `../_belief_lib.py`).
