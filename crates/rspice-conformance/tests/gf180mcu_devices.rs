//! Integration gate for the GF180MCU device corpus.
//!
//! Runs every vendored case and requires it to reproduce its checked-in
//! reference curve within tolerance. Both sides evaluate the same model card
//! from a released GlobalFoundries PDK, so the bound is tight and the suite
//! is correspondingly sensitive to defects in parameter binding, temperature
//! scaling, and corner selection.
//!
//! See [`rspice_conformance::suites::gf180mcu`] for what the reference is and
//! — importantly — what it is not.

use rspice_conformance::suites::gf180mcu::{
    DeviceConfig, DeviceResult, DeviceRunner, DeviceStatistics,
};
use std::path::PathBuf;

#[test]
fn gf180mcu_devices_reproduce_their_reference_curves() {
    let runner = DeviceRunner::new(&tests_dir(), config());

    let cases = runner.discover();
    assert!(
        !cases.is_empty(),
        "GF180MCU corpus is empty at {} — an empty suite passes vacuously",
        runner.root().display()
    );

    let orphans = runner.orphaned_manifest_entries();
    assert!(
        orphans.is_empty(),
        "GF180MCU manifest names cases that do not exist: {}",
        orphans.join(", ")
    );

    let results = runner.run_corpus();
    let stats = DeviceStatistics::collect(&results);
    report(&stats, &results);

    let failures: Vec<&DeviceResult> = results.iter().filter(|result| !result.passed).collect();
    assert!(
        failures.is_empty(),
        "GF180MCU: {} of {} cases did not meet their contract\n{}",
        failures.len(),
        stats.total,
        failures
            .iter()
            .map(|result| {
                let detail = result.worst.map_or_else(
                    || String::from("no comparable point"),
                    |worst| {
                        format!(
                            "at V={:.4}: reference {:.6e}, RSpice {:.6e}",
                            worst.x, worst.expected, worst.actual
                        )
                    },
                );
                format!(
                    "  {} [{}] {} — {}",
                    result.name,
                    result.contract.token(),
                    result.error.as_deref().unwrap_or("unmet"),
                    detail
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn config() -> DeviceConfig {
    let defaults = DeviceConfig::default();
    DeviceConfig {
        verbose: matches!(std::env::var("RSPICE_GF180_VERBOSE").as_deref(), Ok("1")),
        // Overridable so a tolerance investigation does not need a rebuild.
        minimum_tolerance_pct: std::env::var("RSPICE_GF180_TOLERANCE_PCT")
            .ok()
            .and_then(|raw| raw.parse().ok())
            .unwrap_or(defaults.minimum_tolerance_pct),
        ..defaults
    }
}

fn report(stats: &DeviceStatistics, results: &[DeviceResult]) {
    println!(
        "\nGF180MCU devices — {} cases, {} passed, {} reference-only, \
         worst deviation {:.3}%, {:.1}% meeting contract in {:.1}s",
        stats.total,
        stats.passed,
        stats.reference_only,
        stats.worst_error_pct,
        stats.pass_rate(),
        stats.total_time_ms as f64 / 1000.0,
    );

    // The largest deviations are printed on a green run too: this suite's
    // value is the margin it holds, and a corpus quietly drifting from 0.01%
    // to 0.9% is worth seeing before the bound catches it rather than after.
    let mut ranked: Vec<&DeviceResult> = results.iter().collect();
    ranked.sort_by(|a, b| {
        b.worst_error_pct
            .partial_cmp(&a.worst_error_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for result in ranked.iter().take(5) {
        println!("  {:>8.4}%  {}", result.worst_error_pct, result.name);
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
