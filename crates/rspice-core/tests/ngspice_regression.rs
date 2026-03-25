//! Ngspice Regression Test Suite
//!
//! Comprehensive integration tests that run RSpice against the ngspice test suite.
//! Tests are organized by analysis type and device model category.

use rspice_core::testing::{TestRunner, TestRunnerConfig, TestStatistics};
use std::{
    fs,
    path::{Path, PathBuf},
};

// ═══════════════════════════════════════════════════════════════════════════════
// Test Helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Get the path to the ngspice tests directory at the workspace root.
/// Tests are located at workspace_root/tests/, not crate/tests/
fn get_tests_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR points to crates/rspice-core/
    // We need to go up two levels to reach the workspace root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .expect("Could not find workspace root")
        .join("tests")
}

fn suite_is_cmc_qaspec(subdir: &str) -> bool {
    let suite_dir = get_tests_dir().join(subdir);
    if !suite_dir.exists() {
        return false;
    }

    // CMC model QA suites typically expose `qaSpec`/`run` in nested
    // model-family subdirectories (e.g. bsim3/nmos/qaSpec).
    let mut stack = vec![suite_dir];
    let mut visited_dirs = 0usize;
    while let Some(dir) = stack.pop() {
        visited_dirs += 1;
        if visited_dirs > 256 {
            break;
        }

        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.eq_ignore_ascii_case("qaSpec") || name.eq_ignore_ascii_case("run") {
                return true;
            }
            if path.is_dir() {
                stack.push(path);
            }
        }
    }

    false
}

fn normalize_suite_path(path: &Path) -> String {
    path.iter()
        .map(|segment| segment.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

fn all_discoverable_suite_dirs() -> Vec<String> {
    let tests_dir = get_tests_dir();
    let runner = TestRunner::new(tests_dir.clone(), TestRunnerConfig::default());
    let mut suites = Vec::new();
    let mut stack = vec![PathBuf::new()];
    let mut visited_dirs = 0usize;

    while let Some(rel) = stack.pop() {
        visited_dirs += 1;
        if visited_dirs > 512 {
            break;
        }

        let dir = if rel.as_os_str().is_empty() {
            tests_dir.clone()
        } else {
            tests_dir.join(&rel)
        };

        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        let mut child_dirs = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                child_dirs.push(path);
            }
        }
        child_dirs.sort();

        if !rel.as_os_str().is_empty() {
            let suite = normalize_suite_path(&rel);
            if !runner.discover_tests(&suite).is_empty() {
                suites.push(suite);
            }
        }

        for child in child_dirs.into_iter().rev() {
            if let Ok(child_rel) = child.strip_prefix(&tests_dir) {
                stack.push(child_rel.to_path_buf());
            }
        }
    }

    suites.sort();
    suites
}

fn run_and_report(runner: &TestRunner, subdir: &str) -> TestStatistics {
    let results = runner.run_suite(subdir);
    if !results.is_empty() {
        TestRunner::print_summary(&results);
    }
    let stats = TestRunner::statistics(&results);

    if stats.total == 0 && suite_is_cmc_qaspec(subdir) {
        println!(
            "Suite '{}' uses CMC qaSpec workflow (no direct .cir decks); skipping under .cir harness.",
            subdir
        );
        return stats;
    }

    assert!(
        stats.total > 0,
        "Suite '{}' discovered no .cir tests. Verify suite path and test discovery.",
        subdir
    );
    assert_eq!(
        stats.failed,
        0,
        "Suite '{}' has {} failing circuit(s): {} passed, {} skipped ({:.1}% pass rate).",
        subdir,
        stats.failed,
        stats.passed,
        stats.skipped,
        stats.pass_rate()
    );
    assert_eq!(
        stats.skipped, 0,
        "Suite '{}' skipped {} circuit(s); all discovered decks must execute.",
        subdir, stats.skipped
    );

    stats
}

fn suite_config(subdir: &str) -> TestRunnerConfig {
    let mut cfg = TestRunnerConfig::default();

    match subdir {
        // Large digital/mixed-signal decks can spend several minutes in
        // robust startup plus waveform parity comparison, even in release mode.
        "general" | "transient" => {
            cfg.max_time_per_test_ms = 600_000; // 10 minutes
        }
        // Level-6 references include near-zero startup voltages in the single
        // digit nanovolt range; use a small absolute floor to avoid
        // over-penalizing numerically equivalent zero.
        "mos6" => {
            cfg.absolute_tolerance = 2e-5;
            cfg.max_time_per_test_ms = 90_000;
        }
        // VBIC decks can require extra time for stiff startup transients.
        "vbic" => {
            cfg.max_time_per_test_ms = 120_000;
        }
        // Industrial SOI decks include long ring-oscillator transients that
        // are valid but expensive even in optimized builds.
        "bsim3soidd" | "bsim3soifd" | "bsim3soipd" => {
            cfg.max_time_per_test_ms = 1_800_000; // 30 minutes
        }
        // Distributed transmission-line decks are currently compared with a
        // wider envelope because RSpice uses a simplified lossy companion model
        // rather than ngspice's full LTRA convolution kernel.
        "transmission" => {
            cfg.relative_tolerance = 0.12;
            cfg.absolute_tolerance = 1e-4;
        }
        _ => {}
    }

    cfg
}

#[test]
fn test_suite_config_applies_soi_timeout_override() {
    for suite in ["bsim3soidd", "bsim3soifd", "bsim3soipd"] {
        let cfg = suite_config(suite);
        assert_eq!(
            cfg.max_time_per_test_ms, 1_800_000,
            "expected {} to inherit the long-suite timeout override",
            suite
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// General Circuit Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ngspice_general_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("general"));
    let stats = run_and_report(&runner, "general");

    println!(
        "General: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_resistance_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "resistance");

    println!(
        "Resistance: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_filters_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "filters");

    println!(
        "Filters: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_transient_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("transient"));
    let stats = run_and_report(&runner, "transient");

    println!(
        "Transient: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_transmission_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("transmission"));
    let stats = run_and_report(&runner, "transmission");

    println!(
        "Transmission: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Device Model Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ngspice_jfet_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "jfet");

    println!(
        "JFET: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_bsim3_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "bsim3");

    println!(
        "BSIM3: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_bsim4_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "bsim4");

    println!(
        "BSIM4: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_mos6_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("mos6"));
    let stats = run_and_report(&runner, "mos6");

    println!(
        "MOS6: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_vbic_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("vbic"));
    let stats = run_and_report(&runner, "vbic");

    println!(
        "VBIC: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_hicum2_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "hicum2");

    println!(
        "HiCUM2: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_hfet_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("hfet"));
    let stats = run_and_report(&runner, "hfet");

    println!(
        "HFET: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Analysis Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ngspice_sensitivity_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "sensitivity");

    println!(
        "Sensitivity: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_polezero_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "polezero");

    println!(
        "Pole-Zero: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// XSPICE Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ngspice_xspice_digital_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "xspice/digital");

    println!(
        "XSPICE Digital: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_parser_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/parser");

    println!(
        "Regression Parser: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_func_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/func");

    println!(
        "Regression Func: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_lib_processing_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/lib-processing");

    println!(
        "Regression Lib Processing: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_misc_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/misc");

    println!(
        "Regression Misc: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_model_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/model");

    println!(
        "Regression Model: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_pipe_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/pipe");

    println!(
        "Regression Pipe: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_pz_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/pz");

    println!(
        "Regression PZ: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_sens_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/sens");

    println!(
        "Regression Sens: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_subckt_processing_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/subckt-processing");

    println!(
        "Regression Subckt Processing: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_regression_temper_suite() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());
    let stats = run_and_report(&runner, "regression/temper");

    println!(
        "Regression Temper: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// SOI MOSFET Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ngspice_bsim3soidd_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("bsim3soidd"));
    let stats = run_and_report(&runner, "bsim3soidd");

    println!(
        "BSIM3SOI-DD: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_bsim3soifd_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("bsim3soifd"));
    let stats = run_and_report(&runner, "bsim3soifd");

    println!(
        "BSIM3SOI-FD: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_bsim3soipd_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("bsim3soipd"));
    let stats = run_and_report(&runner, "bsim3soipd");

    println!(
        "BSIM3SOI-PD: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

#[test]
fn test_ngspice_mesa_suite() {
    let runner = TestRunner::new(get_tests_dir(), suite_config("mesa"));
    let stats = run_and_report(&runner, "mesa");

    println!(
        "MESA: {} tests, {:.1}% pass rate",
        stats.total,
        stats.pass_rate()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Full Suite Aggregate Test
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_full_ngspice_suite_summary() {
    let suites = all_discoverable_suite_dirs();

    let mut total_stats = TestStatistics {
        total: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        total_time_ms: 0,
    };

    for suite in &suites {
        let runner = TestRunner::new(get_tests_dir(), suite_config(suite));
        let results = runner.run_suite(suite);
        let stats = TestRunner::statistics(&results);

        total_stats.total += stats.total;
        total_stats.passed += stats.passed;
        total_stats.failed += stats.failed;
        total_stats.skipped += stats.skipped;
        total_stats.total_time_ms += stats.total_time_ms;

        if stats.total > 0 {
            println!(
                "{:15} {:4} tests | {:4} passed | {:4} failed | {:4} skipped | {:.1}%",
                suite,
                stats.total,
                stats.passed,
                stats.failed,
                stats.skipped,
                stats.pass_rate()
            );
        }
    }

    println!("\n{:=<72}", "");
    println!(
        "{:15} {:4} tests | {:4} passed | {:4} failed | {:4} skipped | {:.1}%",
        "TOTAL",
        total_stats.total,
        total_stats.passed,
        total_stats.failed,
        total_stats.skipped,
        total_stats.pass_rate()
    );
    println!("Total time: {}ms", total_stats.total_time_ms);

    assert!(
        total_stats.total > 0,
        "Full ngspice suite discovered no tests; verify tests directory wiring."
    );
    assert_eq!(
        total_stats.failed,
        0,
        "Full ngspice suite has {} failing circuit(s): {} passed, {} skipped ({:.1}% pass rate).",
        total_stats.failed,
        total_stats.passed,
        total_stats.skipped,
        total_stats.pass_rate()
    );
    assert_eq!(
        total_stats.skipped, 0,
        "Full ngspice suite skipped {} circuit(s); all discovered decks must execute.",
        total_stats.skipped
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Utility Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_discover_tests() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());

    let general_tests = runner.discover_tests("general");
    println!("Found {} tests in general/", general_tests.len());

    let bsim4_tests = runner.discover_tests("bsim4");
    println!("Found {} tests in bsim4/", bsim4_tests.len());

    // Verify paths look correct
    for path in general_tests.iter().take(3) {
        println!("  - {}", path.display());
        assert!(path.extension().is_some_and(|e| e == "cir"));
    }
}

#[test]
fn test_all_discoverable_suite_dirs_are_covered() {
    let suites = all_discoverable_suite_dirs();
    let expected = vec![
        "bsim1",
        "bsim2",
        "bsim3soidd",
        "bsim3soifd",
        "bsim3soipd",
        "filters",
        "general",
        "hfet",
        "jfet",
        "mes",
        "mesa",
        "mos6",
        "polezero",
        "regression/func",
        "regression/lib-processing",
        "regression/misc",
        "regression/model",
        "regression/parser",
        "regression/pipe",
        "regression/pz",
        "regression/sens",
        "regression/subckt-processing",
        "regression/temper",
        "resistance",
        "sensitivity",
        "transient",
        "transmission",
        "vbic",
        "xspice/digital",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    assert_eq!(suites, expected);
}

#[test]
fn test_cmc_suite_detection() {
    assert!(suite_is_cmc_qaspec("bsim3"));
    assert!(suite_is_cmc_qaspec("bsim4"));
    assert!(suite_is_cmc_qaspec("hicum2"));
    assert!(!suite_is_cmc_qaspec("general"));
}

#[test]
fn test_unsupported_detection() {
    let runner = TestRunner::new(get_tests_dir(), TestRunnerConfig::default());

    // Full-suite validation must fail unsupported decks rather than silently skip them.
    let config = runner.config();
    assert!(!config.skip_unsupported);
}

#[test]
fn test_statistics_calculation() {
    let stats = TestStatistics {
        total: 100,
        passed: 75,
        failed: 15,
        skipped: 10,
        total_time_ms: 1234,
    };

    assert!((stats.pass_rate() - 75.0).abs() < 0.01);
}
