# composr — belief corpus

Seeded from `~/personal/composr` (a Rust replacement for the slow parts of
Composer-based PHP installs). 25 beliefs across 6 sessions spanning its short,
fast life (2026-05-05 → 2026-06-02). `txn_time` = recorded-when; `valid_time` =
when a scope statement held.

## The headline: scope over time

Four `project-scope` beliefs, each with a `valid_time` window (`INDEX.md` renders
the timeline):

```
2026-05-05  Rust `composer dump-autoload -o` only (as "autoload-dumpr")  scope-dumpr
2026-05-07  EXPLODED → full `composer install` replacement, renamed       scope-installer
2026-05-10  + native plugin replication (pest, spi) + git-hook + perf     scope-plugin-replication
2026-06-02  published 1.0 crate; installs real Laravel apps byte-equal    scope-released
```

The most dramatic jump in any corpus is `scope-dumpr ⇒ scope-installer`: a
one-command tool became an end-to-end installer **in a single day**. The
`rename-dumpr-to-composr` belief records that a name change *is* a scope signal.

## Decisions that got overturned

- `native-post-autoload` **supersedes** `delegate-all-post-autoload` — from
  shelling out for the whole post-autoload-dump event to handling Laravel's
  `package:discover` + clearCompiled natively (→ "0 composer subprocess calls").
- `plugin-policy-three-tier` **supersedes** `plugins-all-delegated` — from
  delegating every composer-plugin to natively replicating specific ones
  (pest-plugin, tbachert/spi) with byte-equal output.
- `single-par-iter-extract` **supersedes** `lpt-partition-extract` — a perf
  heuristic (huge/small bucket scheduling) measured not worth it and dropped.

## Objectively wrong beliefs (verdict gold labels)

Falsified hunches, each defeated by an `adjudicates` verdict (no `valid_time`):

- `pest-plugin-not-inert` defeats `pest-plugin-is-inert` — classifying pest-plugin
  as inert was wrong: without natively writing `vendor/pest-plugins.json`, Pest's
  Loader returns `[]` and every Pest plugin *silently* no-ops. The silent failure
  is why the wrong classification survived.
- `content-hash-needs-php-shape` defeats `content-hash-is-plain` — the lock
  content-hash isn't a plain byte hash; it needs composer's PHP-specific
  normalization to match.

(See also `lpt-partition-extract`, a measured-wrong perf heuristic — modeled as a
supersession rather than a verdict, since it was a deliberate try, not a misread.)

## The stable spine

Two goals never moved while scope ballooned: `goal-cold-start-path` (only attack
the slow cold-start path, defer the rest) and `goal-byte-equivalence` (output
must be byte-identical to composer, golden-tested). `hybrid-mode-philosophy`
(don't reimplement composer's EventDispatcher) is the principled boundary.
`r-composr-trajectory` (reducer) consolidates the arc.

Grounded in the repo's `README.md` and git history. Regenerate with
`python3 _seed.py` (shared machinery in `../_belief_lib.py`).
