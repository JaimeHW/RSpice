//! Xyce Regression Corpus Tests
//!
//! The Xyce corpus is vendored under `tests/xyce`. These tests run the
//! Rust-native RSpice adapter, not the upstream Perl/Bash harness. Upstream
//! platform scripts are intentionally trimmed from this corpus.

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

fn removed_upstream_harness_artifact_paths(root: &Path) -> Vec<String> {
    fn visit(root: &Path, dir: &Path, paths: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, paths);
                continue;
            }

            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let is_removed_artifact = matches!(
                extension.to_ascii_lowercase().as_str(),
                "sh" | "pl" | "pm" | "py" | "tags"
            ) || name.eq_ignore_ascii_case("tags")
                || name.eq_ignore_ascii_case("exclude")
                || name.eq_ignore_ascii_case("run")
                || name.eq_ignore_ascii_case("run_xyce_regression")
                || name.eq_ignore_ascii_case("run_xyce_regressionMP")
                || (name.starts_with("Manifest") && extension.eq_ignore_ascii_case("txt"));

            if is_removed_artifact {
                paths.push(
                    path.strip_prefix(root)
                        .expect("path under root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut paths = Vec::new();
    visit(root, root, &mut paths);
    paths.sort();
    paths
}

fn harness_manifest_entries(root: &Path) -> BTreeSet<String> {
    let path = root.join("RSPICE-HARNESS-MANIFEST.tsv");
    let content = fs::read_to_string(&path).expect("read RSpice Xyce harness manifest");
    let mut entries = BTreeSet::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (relative_path, contract) = line
            .split_once('\t')
            .expect("manifest rows use tab-separated path and contract");
        assert_eq!(
            contract, "requires_upstream_wrapper",
            "unexpected Xyce harness manifest contract in {line:?}"
        );
        assert!(
            entries.insert(relative_path.to_string()),
            "duplicate Xyce harness manifest path {relative_path}"
        );
    }
    entries
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
        !root.join("TestScripts").exists(),
        "upstream TestScripts runner directory must not be vendored into the RSpice corpus"
    );
    assert!(root.join("COPYING").is_file(), "Xyce COPYING file missing");
    assert!(
        root.join("RSPICE-HARNESS-MANIFEST.tsv").is_file(),
        "RSpice Xyce harness manifest missing"
    );
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
        discovered.iter().all(|path| path.starts_with("Netlists/")),
        "Only runtime simulator decks should remain in the Xyce .cir corpus"
    );
}

#[test]
fn test_xyce_corpus_omits_upstream_platform_harness_artifacts() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let artifacts = removed_upstream_harness_artifact_paths(&root);

    assert!(
        artifacts.is_empty(),
        "Xyce corpus contains upstream platform harness artifacts that RSpice does not execute: {artifacts:#?}"
    );
    assert!(
        !runner.executes_upstream_scripts(),
        "RSpice Xyce tests must not depend on Perl/Bash harness execution"
    );
}

#[test]
fn test_xyce_wrapper_manifest_covers_trimmed_sidecar_contracts() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let entries = harness_manifest_entries(&root);

    assert!(
        entries.len() > 1000,
        "wrapper manifest should preserve the removed Xyce .cir.sh sidecar contracts"
    );
    for relative_path in &entries {
        assert!(
            relative_path.starts_with("Netlists/") && relative_path.ends_with(".cir"),
            "wrapper manifest path must name a retained Netlists .cir deck: {relative_path}"
        );
        let deck_path = root.join(relative_path);
        assert!(
            deck_path.is_file(),
            "wrapper manifest path points at a missing deck: {relative_path}"
        );
        let file_name = deck_path
            .file_name()
            .and_then(|value| value.to_str())
            .expect("deck has file name");
        assert!(
            !deck_path.with_file_name(format!("{file_name}.sh")).exists(),
            "wrapper sidecar should be trimmed, not retained: {relative_path}.sh"
        );
        assert!(
            runner.requires_upstream_wrapper(relative_path),
            "runner did not load wrapper manifest entry for {relative_path}"
        );
    }
    assert!(
        !runner.requires_upstream_wrapper("Netlists/RESISTOR/resistor.cir"),
        "plain resistor smoke deck should not require removed wrapper semantics"
    );
}

#[test]
fn test_xyce_static_prn_cases_run() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/RESISTOR/resistor.cir",
        "Netlists/RESISTOR/resistor_neg.cir",
        "Netlists/ABM_EXPLN/exp_const.cir",
        "Netlists/BJT_PSPICE_NK/bjt_test_nk.cir",
        "Netlists/Certification_Tests/BUG_302/DC_comma.cir",
        "Netlists/Certification_Tests/BUG_28_SON/bug_28_son2.cir",
        "Netlists/Certification_Tests/BUG_1456/bug_1456.cir",
        "Netlists/Certification_Tests/BUG_1602/vbic_3T_et_cf.cir",
        "Netlists/Certification_Tests/BUG_1602/vbic_3T_et_cf_TNOM27.cir",
        "Netlists/Certification_Tests/BUG_1807/vbic_pnp_default.cir",
        "Netlists/MOS6/nmos6_dc.cir",
        "Netlists/NMESFET/nmesfet.cir",
        "Netlists/NMOS1_DC/nmos1.cir",
        "Netlists/NPN_DC/npn1.cir",
        "Netlists/PARAMS2/Params_A3.cir",
        "Netlists/PARAMS2/Params_A4.cir",
        "Netlists/PARAMS2/Params_A8.cir",
        "Netlists/PARAMS2/Params_A8_hs.cir",
        "Netlists/PARAMS2/Params_B4.cir",
        "Netlists/PARAMS2/Params_B7.cir",
        "Netlists/PARAMS2/Params_B7_hs.cir",
        "Netlists/PARAMS2/Params_C2.cir",
        "Netlists/PARAMS2/Params_C3.cir",
        "Netlists/PARAMS2/Params_C3_hs.cir",
        "Netlists/PARAMS2/Params_C4.cir",
        "Netlists/PARAMS2/Params_C4_hs.cir",
        "Netlists/PARAMS2/Params_C5.cir",
        "Netlists/PARAMS2/Params_C5_hs.cir",
        "Netlists/PARAMS2/Params_D2.cir",
        "Netlists/PARAMS2/Params_D3.cir",
        "Netlists/PARAMS2/Params_D3_hs.cir",
        "Netlists/PARAMS2/Params_D4.cir",
        "Netlists/PARAMS2/Params_D4_hs.cir",
        "Netlists/PARAMS2/Params_D5.cir",
        "Netlists/PARAMS2/Params_D5_hs.cir",
        "Netlists/POLY/poly.cir",
        "Netlists/POLY/twoVarThirdOrd.cir",
        "Netlists/SUBCKT/subckt_h0.cir",
        "Netlists/SUBCKT/subckt_h0_hs.cir",
        "Netlists/SUBCKT/subckt_i2.cir",
        "Netlists/SUBCKT/subckt_i3.cir",
        "Netlists/SUBCKT/subckt_i3_hs.cir",
        "Netlists/SUPERNODE/supernode2.cir",
        "Netlists/SUPERNODE/supernode3.cir",
        "Netlists/SUPERNODE/supernode4.cir",
        "Netlists/SUPERNODE/supernode5.cir",
        "Netlists/UNARYOPS/unary_funcs.cir",
        "Netlists/VDMOS_DC/mtb60p06v.cir",
    ] {
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

    let result = runner.run_test(root.join("Netlists/B4SOI/test1.cir"));

    assert!(
        result.passed && result.expected_unsupported,
        "unsupported Xyce feature should be a named result, got {result:?}"
    );
    assert!(
        result
            .error
            .as_deref()
            .is_some_and(|error| error.contains(".STEP")),
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
