//! Xyce Regression Corpus Tests
//!
//! The Xyce corpus is vendored under `tests/xyce`. These tests run the
//! Rust-native RSpice adapter, not the upstream Perl/Bash harness.

use rspice_core::testing::{XyceDeckSection, XyceRunnerConfig, XyceTestRunner};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn get_xyce_tests_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .join("tests")
        .join("xyce")
        .canonicalize()
        .expect("tests/xyce must exist")
}

fn all_circuit_paths(root: &Path) -> BTreeSet<String> {
    fn visit(root: &Path, dir: &Path, paths: &mut BTreeSet<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, paths);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                paths.insert(
                    path.strip_prefix(root)
                        .expect("path under root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut paths = BTreeSet::new();
    visit(root, root, &mut paths);
    paths
}

#[test]
fn test_xyce_corpus_root_is_scoped_under_tests_xyce() {
    let root = get_xyce_tests_dir();
    let normalized = root.to_string_lossy().replace('\\', "/");

    assert!(
        normalized.ends_with("/tests/xyce"),
        "Xyce regression root must be scoped to tests/xyce, got {}",
        root.display()
    );
    assert!(root.join("Netlists").is_dir(), "Netlists directory missing");
    assert!(
        root.join("OutputData").is_dir(),
        "OutputData directory missing"
    );
    assert!(
        root.join("TestScripts").is_dir(),
        "TestScripts directory missing"
    );
    assert!(root.join("COPYING").is_file(), "Xyce COPYING file missing");
    assert!(
        root.join("RSPICE-VENDORING.md").is_file(),
        "RSpice vendoring notes missing"
    );
}

#[test]
fn test_xyce_discovery_covers_every_vendored_circuit() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let discovered = runner
        .discover_tests()
        .into_iter()
        .map(|deck| deck.relative_path)
        .collect::<BTreeSet<_>>();
    let filesystem = all_circuit_paths(&root);

    assert_eq!(
        discovered, filesystem,
        "Every vendored Xyce .cir deck must be discoverable by the harness"
    );
    assert!(
        discovered.iter().any(|path| path.starts_with("Netlists/")),
        "Xyce Netlists corpus must contain simulator decks"
    );
    assert!(
        discovered
            .iter()
            .any(|path| path.starts_with("TestScripts/")),
        "Xyce TestScripts fixtures must remain visible, not silently ignored"
    );
}

#[test]
fn test_xyce_runner_does_not_execute_platform_scripts() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    assert!(
        root.join("TestScripts")
            .join("run_xyce_regression")
            .is_file(),
        "upstream script corpus should remain vendored for provenance"
    );
    assert!(
        !runner.executes_upstream_scripts(),
        "RSpice Xyce tests must not depend on Perl/Bash harness execution"
    );
}

#[test]
fn test_xyce_resistor_static_prn_cases_run() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in ["Netlists/RESISTOR/resistor.cir"] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_unsupported_decks_are_named_results_not_omitted() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let result = runner.run_test(root.join("Netlists/RESISTOR/resistor_neg.cir"));

    assert!(
        result.passed && result.expected_unsupported,
        "unsupported Xyce feature should be a named result, got {result:?}"
    );
    assert!(
        result
            .error
            .as_deref()
            .is_some_and(|error| error.contains("non-positive resistance")),
        "unsupported reason should name the missing feature, got {result:?}"
    );
}

#[test]
fn test_full_xyce_suite_summary_accounts_for_every_deck() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let discovered = runner.discover_tests();
    let netlist_count = discovered
        .iter()
        .filter(|deck| deck.section == XyceDeckSection::Netlists)
        .count();
    let results = runner.run_all();
    XyceTestRunner::print_summary(&results);
    let stats = XyceTestRunner::statistics(&results);

    assert_eq!(
        stats.total,
        discovered.len(),
        "Full Xyce suite must produce exactly one result per discovered deck"
    );
    assert!(netlist_count > 0, "No Xyce Netlists decks discovered");
    assert!(
        stats.executed > 0,
        "At least one Xyce deck must run as a numeric comparison"
    );
    assert_eq!(
        stats.failed, 0,
        "Xyce full suite has {} failing deck(s): {} executed pass, {} expected unsupported",
        stats.failed, stats.passed, stats.expected_unsupported
    );
}
