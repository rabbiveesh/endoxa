//! End-to-end guard for the `mem remember --help` class of bug: asking for help, or mistyping a
//! flag, must never touch the store. The in-crate tests in `src/cli.rs` walk the whole command
//! tree at the parser level; these run the real binary and look at the disk.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

const SUBCOMMANDS: &[&str] = &[
    "remember", "recall", "expand", "ask", "forget", "promote", "consolidate", "sweep", "reduce",
    "dream", "review", "link", "debt", "onboard", "world", "relive", "worker", "__worker", "scope",
];

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("cli-args-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir_all(root.join("store")).unwrap();
        std::fs::create_dir_all(root.join("cwd")).unwrap();
        Sandbox { root }
    }

    /// Fully isolated from the real store and config, with embeddings pointed at a closed port.
    fn mem(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mem"))
            .args(args)
            .current_dir(self.root.join("cwd"))
            .env("MEMORY_DIR", self.root.join("store"))
            .env("XDG_CONFIG_HOME", self.root.join("config"))
            .env("MEM_NO_BG", "1")
            .env("OLLAMA_URL", "http://127.0.0.1:9")
            .output()
            .unwrap()
    }

    fn store_files(&self) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(self.root.join("store"))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    fn beliefs(&self) -> Vec<String> {
        self.store_files()
            .into_iter()
            .filter(|n| n.ends_with(".md"))
            .map(|n| std::fs::read_to_string(self.root.join("store").join(n)).unwrap())
            .collect()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn help_exits_zero_and_leaves_the_store_untouched() {
    for sub in SUBCOMMANDS {
        for flag in ["--help", "-h"] {
            let sb = Sandbox::new();
            let o = sb.mem(&[sub, flag]);
            assert_eq!(o.status.code(), Some(0), "mem {sub} {flag}: {}", stderr(&o));
            assert!(stdout(&o).contains("Usage:"), "mem {sub} {flag}: {}", stdout(&o));
            assert!(sb.store_files().is_empty(), "mem {sub} {flag} wrote {:?}", sb.store_files());
        }
    }
}

#[test]
fn help_after_a_valid_claim_still_writes_nothing() {
    let sb = Sandbox::new();
    let o = sb.mem(&["remember", "a real claim", "--help"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(sb.store_files().is_empty(), "wrote {:?}", sb.store_files());
}

#[test]
fn top_level_help_goes_to_stdout_and_no_args_is_an_error() {
    let sb = Sandbox::new();
    let o = sb.mem(&["--help"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(stdout(&o).contains("remember"));
    assert_eq!(sb.mem(&[]).status.code(), Some(2));
    assert_eq!(sb.mem(&["nonsense"]).status.code(), Some(2));
}

#[test]
fn bad_arguments_exit_two_and_write_nothing() {
    for args in [
        &["remember", "--bogus"][..],
        &["remember", "--glboal", "real claim"],
        &["remember", "--looks like a flag"],
        &["remember", "x", "--supersedes"],
        &["remember"],
        &["sweep", "--bogus"],
        &["reduce", "--bogus"],
        &["promote", "--bogus"],
        &["consolidate", "--limit", "abc"],
        &["dream", "--bogus"],
        &["relive", "garbage"],
        &["link", "a", "depends_on"],
    ] {
        let sb = Sandbox::new();
        let o = sb.mem(args);
        assert_eq!(o.status.code(), Some(2), "mem {args:?}: {}", stdout(&o));
        assert!(sb.store_files().is_empty(), "mem {args:?} wrote {:?}", sb.store_files());
    }
}

#[test]
fn claims_are_still_stored_whole() {
    let sb = Sandbox::new();
    let o = sb.mem(&["remember", "two", "words", "--body", "- a bullet"]);
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    let beliefs = sb.beliefs();
    assert_eq!(beliefs.len(), 1);
    assert!(beliefs[0].contains("two words"), "{}", beliefs[0]);
    assert!(beliefs[0].contains("- a bullet"), "{}", beliefs[0]);

    let sb = Sandbox::new();
    let o = sb.mem(&["remember", "--", "--help is a flag"]);
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert!(sb.beliefs()[0].contains("--help is a flag"), "{}", sb.beliefs()[0]);
}
