//! Integration gate for the oracle-free corpora.
//!
//! One test per corpus. Each runs every discovered deck, judges it against
//! `execution-manifest.tsv`, and fails on any deck that does not meet its
//! contract — in either direction. A deck that regresses fails; a deck whose
//! manifest entry has gone stale because the gap closed fails too, which is
//! what stops the manifest becoming a permanent excuse list.
//!
//! These corpora carry no reference data, so a green run means every deck
//! survived, not that any number was right. See
//! [`rspice_conformance::suites::execution`] for why that is still worth
//! gating on.
//!
//! # Cost
//!
//! Decks the manifest marks `!extended` are skipped by default: the
//! parasitic-annotated ISCAS85 circuits reach ~90k netlist lines and run for
//! minutes each, which does not belong in a per-commit gate. Set
//! `RSPICE_EXECUTION_EXTENDED=1` to run the whole corpus.

use rspice_conformance::suites::execution::{
    ExecutionConfig, ExecutionCorpus, ExecutionResult, ExecutionRunner, ExecutionStatistics,
};
use std::path::PathBuf;

#[test]
fn iscas85_corpus_meets_its_execution_contracts() {
    run_corpus(ExecutionCorpus::Iscas85);
}

#[test]
fn paranoia_corpus_meets_its_execution_contracts() {
    run_corpus(ExecutionCorpus::Paranoia);
}

fn run_corpus(corpus: ExecutionCorpus) {
    let runner = ExecutionRunner::new(corpus, &tests_dir(), config());

    let decks = runner.discover();
    assert!(
        !decks.is_empty(),
        "{} corpus is empty at {} — the vendored corpus is missing, and an \
         empty suite passes vacuously",
        corpus.label(),
        runner.root().display()
    );

    // A manifest row with no deck behind it keeps asserting something about a
    // file that is gone, which is how a re-vendoring silently drops coverage.
    let orphans = runner.orphaned_manifest_entries();
    assert!(
        orphans.is_empty(),
        "{} manifest names decks that do not exist: {}",
        corpus.label(),
        orphans.join(", ")
    );

    let results = runner.run_corpus();
    let stats = ExecutionStatistics::collect(&results);
    report(corpus, &stats, &results, decks.len());

    let failures: Vec<&ExecutionResult> = results.iter().filter(|result| !result.passed).collect();
    assert!(
        failures.is_empty(),
        "{} corpus: {} of {} decks did not meet their contract\n{}",
        corpus.label(),
        failures.len(),
        stats.total,
        failures
            .iter()
            .map(|result| format!(
                "  {} [{}] expected {}, got {}",
                result.name,
                result.contract.token(),
                match &result.failure {
                    Some(rspice_conformance::suites::execution::ExecutionFailure::Unmet {
                        expected,
                    })
                    | Some(
                        rspice_conformance::suites::execution::ExecutionFailure::ContractStale {
                            expected,
                        },
                    ) => *expected,
                    None => "unreachable",
                },
                result.outcome.summary()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn config() -> ExecutionConfig {
    let defaults = ExecutionConfig::default();
    ExecutionConfig {
        skip_extended: !matches!(
            std::env::var("RSPICE_EXECUTION_EXTENDED").as_deref(),
            Ok("1")
        ),
        verbose: matches!(
            std::env::var("RSPICE_EXECUTION_VERBOSE").as_deref(),
            Ok("1")
        ),
        // Overridable because the budget is the only knob that separates
        // "slow" from "hung" on these corpora, and the right value depends on
        // the machine running them. Lowering it is also how a deck's cost
        // tier gets measured before it is written into the manifest.
        max_time_per_deck_ms: std::env::var("RSPICE_EXECUTION_DECK_TIMEOUT_MS")
            .ok()
            .and_then(|raw| raw.parse().ok())
            .unwrap_or(defaults.max_time_per_deck_ms),
    }
}

fn report(
    corpus: ExecutionCorpus,
    stats: &ExecutionStatistics,
    results: &[ExecutionResult],
    discovered: usize,
) {
    // The skipped count is printed even when it is zero. ISCAS85 is entirely
    // `!extended`, so a routine run reports no decks at all — and a bare
    // "0 decks" reads as a missing corpus rather than a deliberately deferred
    // one. Saying what was skipped, and how to run it, is the difference
    // between a suite that looks broken and one that looks bounded.
    let skipped = discovered.saturating_sub(stats.total);
    println!(
        "\n{} — {discovered} decks discovered, {} run ({skipped} deferred as extended), \
         {} completed, {} known-unsupported, {} adjudicated-refusal, {} include fragments, \
         {:.1}% meeting contract in {:.1}s",
        corpus.label(),
        stats.total,
        stats.completed,
        stats.expected_unsupported,
        stats.expected_refusal,
        stats.library_fragments,
        stats.pass_rate(),
        stats.total_time_ms as f64 / 1000.0,
    );
    if skipped > 0 {
        println!(
            "  set RSPICE_EXECUTION_EXTENDED=1 to run the deferred decks \
             (raise RSPICE_EXECUTION_DECK_TIMEOUT_MS with it)"
        );
    }

    // The slowest decks are the ones that decide whether this corpus can stay
    // in a per-commit gate, so they are worth printing even on a green run.
    let mut by_duration: Vec<&ExecutionResult> = results.iter().collect();
    by_duration.sort_by_key(|result| std::cmp::Reverse(result.duration_ms));
    for result in by_duration.iter().take(5) {
        println!("  {:>8} ms  {}", result.duration_ms, result.name);
    }
}

/// The workspace `tests/` root, where corpora are vendored.
fn tests_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
        .parent() // crates/
        .and_then(|path| path.parent()) // workspace root
        .expect("workspace root")
        .join("tests")
}
