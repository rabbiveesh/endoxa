//! The argument grammar of `mem`. Parsing happens once, up front, in `main` — before any command
//! runs — so `--help`, an unknown flag, a missing value or a bad number can never reach a command
//! that writes to the store or spends LLM calls.
//!
//! Free-text VALUES (`--body`, `--reason`, `--rationale`, `--ref`) accept a leading hyphen, because
//! `--body "- a bullet"` is ordinary prose. Free-text POSITIONALS (claims, queries) don't: a stray
//! flag must not become a belief. To store a claim that starts with a hyphen, use
//! `mem remember -- "--foo is a flag"`.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mem", version, about = "Belief-based agentic memory", arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Store a belief (multiple words join into one claim)
    Remember {
        #[arg(required = true, value_name = "CLAIM")]
        claim: Vec<String>,
        /// The belief this one replaces
        #[arg(long, value_name = "SLUG|ID")]
        supersedes: Option<String>,
        /// Write to the global scope instead of this repo
        #[arg(long)]
        global: bool,
        /// A reference to attach (repeatable)
        #[arg(long = "ref", value_name = "R", allow_hyphen_values = true)]
        refs: Vec<String>,
        /// Longer detail behind the claim
        #[arg(long, value_name = "B", default_value = "", allow_hyphen_values = true)]
        body: String,
    },
    /// Recall current beliefs: frontier-resolved, scope-filtered, semantic
    Recall {
        #[arg(required = true, value_name = "QUERY")]
        query: Vec<String>,
        #[arg(long, value_name = "N", default_value_t = 10)]
        limit: usize,
        /// Don't show the novelty-bridge line
        #[arg(long)]
        no_bridges: bool,
    },
    /// One hop: show a belief's linked context
    Expand {
        #[arg(value_name = "SLUG|ID")]
        reference: String,
    },
    /// LLM-synthesized grounded answer (opt-in; world-relative with --world)
    Ask {
        #[arg(required = true, value_name = "QUESTION")]
        question: Vec<String>,
        #[arg(long, value_name = "N", default_value_t = 8)]
        limit: usize,
        /// Reduce under this world's assumption
        #[arg(long, value_name = "W")]
        world: Option<String>,
    },
    /// Retract a belief: recall drops it (file kept)
    Forget {
        #[arg(value_name = "SLUG|ID")]
        reference: String,
        #[arg(long, value_name = "R", default_value = "", allow_hyphen_values = true)]
        reason: String,
    },
    /// Lift a merged branch's beliefs into repo canon
    Promote {
        /// Defaults to the current branch
        #[arg(value_name = "BRANCH")]
        branch: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Run the LLM judge over recent beliefs (detached by default)
    Consolidate {
        #[arg(long, value_name = "N", default_value_t = 12)]
        limit: usize,
        /// Run inline instead of detached
        #[arg(long)]
        fg: bool,
    },
    /// Find and mark SILENT staleness across the frontier (double-verified supersedes)
    Sweep {
        #[arg(long, value_name = "N", default_value_t = 25)]
        limit: usize,
        /// Similarity floor for candidate pairs
        #[arg(long, value_name = "T", default_value_t = crate::SWEEP_DEFAULT_THRESHOLD)]
        threshold: f32,
        #[arg(long)]
        dry_run: bool,
    },
    /// Collapse duplicate beliefs: recall folds them behind one representative
    Reduce {
        #[arg(long)]
        dry_run: bool,
    },
    /// REM/novelty pass: bridge the most-unrelated pairs
    Dream {
        #[arg(long, value_name = "N", default_value_t = 8)]
        limit: usize,
    },
    /// List edges flagged for frontier review (candidate depends_on)
    Review {
        #[arg(long, value_name = "N", default_value_t = 20)]
        limit: usize,
    },
    /// Author a durable edge (frontier/human adjudication)
    Link {
        #[arg(value_name = "SUBJECT")]
        subject: String,
        /// Edge kind, e.g. depends_on
        #[arg(value_name = "KIND")]
        kind: String,
        #[arg(value_name = "OBJECT")]
        object: String,
        #[arg(long, value_name = "R", default_value = "", allow_hyphen_values = true)]
        rationale: String,
    },
    /// List known-debt beliefs; resurfaces when a blocked_on constraint lifts
    Debt {
        #[arg(value_name = "QUERY")]
        query: Vec<String>,
    },
    /// Lead harvest, then claim drafts, then store
    Onboard {
        #[arg(value_name = "REPO", default_value = ".")]
        repo: PathBuf,
        /// Where to write the lead and draft files
        #[arg(long, value_name = "DIR")]
        out: Option<PathBuf>,
        /// How many leads to report
        #[arg(long, value_name = "N", default_value_t = 25)]
        top: usize,
        /// Draft claims from the best N leads (LLM)
        #[arg(long, value_name = "N", default_value_t = 0)]
        escalate: usize,
        /// Add debt envelopes to kludge leads
        #[arg(long)]
        tier2: bool,
        /// Write the drafts into the store
        #[arg(long)]
        commit: bool,
    },
    /// Parallel realities: per-world frontiers from worlds.json
    World {
        #[command(subcommand)]
        action: Option<WorldCmd>,
    },
    /// Bitemporal replay: the frontier as it stood then, diffed vs now
    Relive {
        /// YYYY-MM-DD (end of that day) or an ISO timestamp
        #[arg(value_name = "AS_OF_TIME", value_parser = parse_as_of)]
        time: String,
        #[arg(long)]
        all: bool,
    },
    /// Background maintenance: show status (--now forces a pass)
    Worker {
        #[arg(long)]
        now: bool,
    },
    /// The detached worker body, spawned by `worker.rs`
    #[command(name = "__worker", hide = true)]
    InternalWorker {
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long)]
        cli: bool,
    },
    /// Show the active scopes here
    Scope,
}

#[derive(Subcommand, Debug)]
pub enum WorldCmd {
    /// List worlds and how each diverges from the default (the default action)
    List,
    /// Show one world's frontier
    Show { name: String },
    /// Beliefs whose current-status flips between two worlds
    Diff { a: String, b: String },
}

/// `as_of` compares timestamps as strings, so anything that isn't a date would "work" and print a
/// confident answer about nothing. Reject it here instead.
fn parse_as_of(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    let digits = |r: std::ops::Range<usize>| b[r].iter().all(u8::is_ascii_digit);
    let two = |i: usize| (b[i] - b'0') * 10 + (b[i + 1] - b'0');
    let ok = b.len() >= 10
        && digits(0..4)
        && b[4] == b'-'
        && digits(5..7)
        && b[7] == b'-'
        && digits(8..10)
        && (1..=12).contains(&two(5))
        && (1..=31).contains(&two(8))
        && (b.len() == 10 || (b[10] == b'T' && b.len() >= 13 && digits(11..13)));
    if ok {
        Ok(s.to_string())
    } else {
        Err("expected YYYY-MM-DD or an ISO timestamp like 2026-03-01T12:00:00Z".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use clap::CommandFactory;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("mem").chain(args.iter().copied()))
    }

    fn kind(args: &[&str]) -> ErrorKind {
        parse(args).err().unwrap_or_else(|| panic!("{args:?} should not parse")).kind()
    }

    #[test]
    fn grammar_is_well_formed() {
        Cli::command().debug_assert();
    }

    /// Walks the real command tree, so a subcommand added later is covered automatically.
    #[test]
    fn every_subcommand_answers_help_without_dispatching() {
        fn walk(cmd: &clap::Command, path: &mut Vec<String>, seen: &mut usize) {
            for sub in cmd.get_subcommands() {
                if sub.get_name() == "help" {
                    continue;
                }
                path.push(sub.get_name().to_string());
                for flag in ["--help", "-h"] {
                    let mut argv: Vec<&str> = path.iter().map(String::as_str).collect();
                    argv.push(flag);
                    assert_eq!(kind(&argv), ErrorKind::DisplayHelp, "{argv:?}");
                    *seen += 1;
                }
                walk(sub, path, seen);
                path.pop();
            }
        }
        let mut seen = 0;
        walk(&Cli::command(), &mut Vec::new(), &mut seen);
        assert!(seen >= 40, "expected the whole tree to be walked, saw {seen}");
    }

    #[test]
    fn stray_flags_never_become_a_claim() {
        assert_eq!(kind(&["remember", "--glboal", "real claim"]), ErrorKind::UnknownArgument);
        assert_eq!(kind(&["remember", "--bogus"]), ErrorKind::UnknownArgument);
        assert_eq!(kind(&["remember", "x", "--supersedes"]), ErrorKind::InvalidValue);
        assert_eq!(kind(&["remember"]), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn bad_values_are_errors_not_defaults() {
        assert_eq!(kind(&["recall", "q", "--limit", "abc"]), ErrorKind::ValueValidation);
        assert_eq!(kind(&["sweep", "--threshold", "high"]), ErrorKind::ValueValidation);
        assert_eq!(kind(&["sweep", "--bogus"]), ErrorKind::UnknownArgument);
    }

    #[test]
    fn words_join_and_flags_can_sit_between_them() {
        let Cmd::Remember { claim, .. } = parse(&["remember", "two", "words"]).unwrap().command else {
            panic!()
        };
        assert_eq!(claim.join(" "), "two words");
        let Cmd::Recall { query, limit, .. } =
            parse(&["recall", "foo", "--limit", "3", "bar"]).unwrap().command
        else {
            panic!()
        };
        assert_eq!((query.join(" ").as_str(), limit), ("foo bar", 3));
    }

    #[test]
    fn hyphens_are_allowed_where_they_are_prose() {
        let Cmd::Remember { claim, .. } = parse(&["remember", "--", "--foo is a flag"]).unwrap().command
        else {
            panic!()
        };
        assert_eq!(claim, ["--foo is a flag"]);
        let Cmd::Remember { body, .. } =
            parse(&["remember", "x", "--body", "- a bullet"]).unwrap().command
        else {
            panic!()
        };
        assert_eq!(body, "- a bullet");
    }

    /// `spawn_detached_args` in worker.rs builds these exact argv shapes; a mismatch would make
    /// the detached worker die silently.
    #[test]
    fn worker_spawn_contract() {
        let worker = |args: &[&str]| match parse(args).unwrap().command {
            Cmd::InternalWorker { limit, cli } => (limit, cli),
            other => panic!("{other:?}"),
        };
        assert_eq!(worker(&["__worker"]), (None, false), "bare = due-check semantics");
        assert_eq!(worker(&["__worker", "--limit", "30", "--cli"]), (Some(30), true));
    }

    #[test]
    fn internal_worker_stays_out_of_help() {
        let help = Cli::command().render_help().to_string();
        assert!(!help.contains("__worker"), "{help}");
        assert!(help.contains("remember"), "{help}");
    }

    #[test]
    fn world_defaults_to_list_and_checks_arity() {
        let Cmd::World { action } = parse(&["world"]).unwrap().command else { panic!() };
        assert!(action.is_none());
        assert_eq!(kind(&["world", "show"]), ErrorKind::MissingRequiredArgument);
        assert_eq!(kind(&["world", "nope"]), ErrorKind::InvalidSubcommand);
    }

    #[test]
    fn relive_accepts_dates_and_rejects_nonsense() {
        for good in ["2026-03-01", "2026-03-01T12:00:00Z", "2026-03-01T12"] {
            assert!(parse(&["relive", good]).is_ok(), "{good}");
        }
        for bad in ["garbage", "2026-13-01", "2026-03-00", "2026-03", "2026-03-01x", "2026-03-01Tx", "מחר-מחר-מחר"] {
            assert_eq!(kind(&["relive", bad]), ErrorKind::ValueValidation, "{bad}");
        }
    }
}
