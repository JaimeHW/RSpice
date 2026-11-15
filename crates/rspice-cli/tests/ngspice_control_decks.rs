//! Upstream decks whose checks live in their `.control` script, run the way
//! a user runs them: `rspice run` in the deck's own directory, in the ngspice
//! dialect the decks were written for and check themselves against, and in
//! the default dialect users run them in.
//!
//! The conformance runners judge a deck's declarative analyses; only this
//! test executes its script through the command users run, so a script that
//! stops on a command, or whose own checks fail (the regression decks `quit
//! 1` on a wrong value), fails here. The decks are the ngspice suite's
//! `scripted_control` rows and the paranoia rows marked `!script`; a row
//! marked `!extended` runs only with `RSPICE_EXECUTION_EXTENDED=1`.

mod common;

use common::test_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

/// Marks a paranoia row whose control script this test runs.
const SCRIPT_MARKER: &str = "!script";
/// Marks a deck too expensive for a routine run.
const EXTENDED_MARKER: &str = "!extended";

/// The dialects every deck runs in: ngspice's, and the default one, which
/// `rspice run` uses without `--spice-dialect`.
const DIALECTS: [Option<&str>; 2] = [Some("ngspice"), None];

/// What the default dialect runs a deck with beyond its own arguments, and
/// the documented difference of that dialect the deck's check does not
/// allow for. The default dialect's TMAX is TSTOP/1024
/// (`resolve_dialect_transient_maximum_step`), ngspice's TSTEP.
const DEFAULT_DIALECT_ARGUMENTS: &[(&str, &[&str], &str)] = &[(
    "regression/misc/resume-1.cir",
    &["--max-step", "0.2u"],
    "the deck's 2e-6 limit holds at ngspice's TMAX, its TSTEP of 0.2 us; at the default \
     dialect's 1.95 us, trapezoidal integration of the 0.5 ms time constant after the stop \
     errs 2.98e-6 against the closed form, as ngspice-46 itself does at that TMAX (2.80e-6)",
)];

fn corpus(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests")
        .join(name)
}

/// The decks of `manifest` (under `tests/<corpus>`) whose row `selects`,
/// with whether the row marks them extended.
fn rows(
    corpus_name: &str,
    manifest: &str,
    selects: impl Fn(&str, &str) -> bool,
) -> Vec<(PathBuf, bool)> {
    let root = corpus(corpus_name);
    let text = std::fs::read_to_string(root.join(manifest))
        .unwrap_or_else(|error| panic!("read {corpus_name}/{manifest}: {error}"));
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.splitn(3, '\t');
            let path = fields.next()?;
            let contract = fields.next().unwrap_or_default();
            let note = fields.next().unwrap_or_default();
            selects(contract, note).then(|| (root.join(path), note.contains(EXTENDED_MARKER)))
        })
        .collect()
}

/// Run `deck` in `dialect` (the default one for `None`) in a copy of its
/// directory, so a file its script writes stays out of the corpus, and
/// report how it failed, if it did.
fn run_script(deck: &Path, dialect: Option<&str>) -> Result<(), String> {
    let name = deck.file_name().unwrap().to_string_lossy().into_owned();
    let dir = test_dir(&name.replace('.', "_"));
    for entry in std::fs::read_dir(deck.parent().unwrap()).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), dir.join(entry.file_name())).unwrap();
        }
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command
        .current_dir(&*dir)
        .args(["--quiet", "run", &name, "-o", "result.csv", "-f", "csv"])
        .env(
            "RSPICE_MODELS_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/spice"),
        );
    let mut reason = None;
    match dialect {
        Some(dialect) => {
            command.args(["--spice-dialect", dialect]);
        }
        None => {
            if let Some((_, arguments, why)) = DEFAULT_DIALECT_ARGUMENTS
                .iter()
                .find(|(path, _, _)| deck.ends_with(path))
            {
                command.args(*arguments);
                reason = Some(format!("with {} because {why}", arguments.join(" ")));
            }
        }
    }
    let output = command.output().expect("execute public CLI");
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "{} in the {} dialect{} exited with {}\nstdout:\n{}\nstderr:\n{}",
        deck.display(),
        dialect.unwrap_or("default"),
        reason.map_or_else(String::new, |reason| format!(" ({reason})")),
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

/// Run every selected deck in every dialect, several at a time, and fail
/// with every failure.
fn run_all(decks: Vec<(PathBuf, bool)>) {
    assert!(
        !decks.is_empty(),
        "no deck selected: the manifest moved or lost its rows"
    );
    for (path, _, _) in DEFAULT_DIALECT_ARGUMENTS {
        assert!(
            corpus("ngspice").join(path).is_file(),
            "{path}: no such deck for its default-dialect arguments"
        );
    }
    let extended = std::env::var("RSPICE_EXECUTION_EXTENDED").as_deref() == Ok("1");
    let pending = Mutex::new(
        decks
            .into_iter()
            .filter(|(_, costly)| extended || !costly)
            .flat_map(|(deck, _)| DIALECTS.map(|dialect| (deck.clone(), dialect)))
            .collect::<Vec<_>>(),
    );
    let failures = Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism().map_or(1, |count| count.get().min(4));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                while let Some((deck, dialect)) = pending.lock().unwrap().pop() {
                    if let Err(failure) = run_script(&deck, dialect) {
                        failures.lock().unwrap().push(failure);
                    }
                }
            });
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn ngspice_scripted_decks_pass_their_own_checks() {
    run_all(rows("ngspice", "validation-manifest.tsv", |contract, _| {
        contract == "scripted_control"
    }));
}

#[test]
fn paranoia_scripted_decks_run_their_scripts() {
    run_all(rows("paranoia", "execution-manifest.tsv", |_, note| {
        note.contains(SCRIPT_MARKER)
    }));
}
