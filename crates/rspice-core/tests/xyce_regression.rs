//! Xyce Regression Corpus Tests
//!
//! The Xyce corpus is vendored under `tests/xyce`. These tests run the
//! Rust-native RSpice adapter, not the upstream Perl/Bash harness. Upstream
//! platform scripts are intentionally trimmed from this corpus.

use rspice_core::testing::{XyceDeckSection, XyceRunnerConfig, XyceTestRunner};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

fn xyce_runner_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn lock_xyce_runner() -> MutexGuard<'static, ()> {
    // Assertions run while the serialization guard is held. Recover after a
    // failed assertion so one fixture reports one failure instead of poisoning
    // every later Xyce test in the process.
    xyce_runner_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

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
fn test_xyce_hard_coded_deck_paths_use_exact_vendored_case() {
    let root = get_xyce_tests_dir();
    let filesystem = all_circuit_paths(&root);
    let referenced = include_str!("xyce_regression.rs")
        .split('"')
        .filter(|fragment| fragment.starts_with("Netlists/") && fragment.ends_with(".cir"))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let missing = referenced.difference(&filesystem).collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "hard-coded Xyce deck paths must match the vendored path and case exactly: {missing:#?}"
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
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/RESISTOR/resistor.cir",
        "Netlists/RESISTOR/resistor_neg.cir",
        "Netlists/ABM_EXPLN/exp_const.cir",
        "Netlists/ABM_SCT/sct.cir",
        "Netlists/BJT_PSPICE_NK/bjt_test_nk.cir",
        "Netlists/BINARYOPS/binary_funcs.cir",
        "Netlists/Certification_Tests/BUG_302/DC_comma.cir",
        "Netlists/Certification_Tests/BUG_302/DC_defaults.cir",
        "Netlists/Certification_Tests/BUG_302/DC_delimiter_invalid.cir",
        "Netlists/Certification_Tests/BUG_302/DC_tab.cir",
        "Netlists/Certification_Tests/BUG_28_SON/bug_28_son2.cir",
        "Netlists/Certification_Tests/BUG_1456/bug_1456.cir",
        "Netlists/Certification_Tests/BUG_1602/vbic_3T_et_cf.cir",
        "Netlists/Certification_Tests/BUG_1602/vbic_3T_et_cf_TNOM27.cir",
        "Netlists/Certification_Tests/BUG_1807/vbic_pnp_default.cir",
        "Netlists/EXPPRINT/expPrint.cir",
        "Netlists/MOS6/nmos6_dc.cir",
        "Netlists/MOS6/pmos6_dc.cir",
        "Netlists/NMESFET/nmesfet.cir",
        "Netlists/NJFET_DC/njfet-2109.cir",
        "Netlists/NMOS1_DC/nmos1.cir",
        "Netlists/NMOS3_DC/mos3_dc.cir",
        "Netlists/NPN_DC/npn1.cir",
        "Netlists/PARAMS2/Params_A0.cir",
        "Netlists/PARAMS2/Params_A1.cir",
        "Netlists/PARAMS2/Params_A2.cir",
        "Netlists/PARAMS2/Params_A3.cir",
        "Netlists/PARAMS2/Params_A4.cir",
        "Netlists/PARAMS2/Params_A5.cir",
        "Netlists/PARAMS2/Params_A5_hs.cir",
        "Netlists/PARAMS2/Params_A6.cir",
        "Netlists/PARAMS2/Params_A6_hs.cir",
        "Netlists/PARAMS2/Params_A7.cir",
        "Netlists/PARAMS2/Params_A7_hs.cir",
        "Netlists/PARAMS2/Params_A8.cir",
        "Netlists/PARAMS2/Params_A8_hs.cir",
        "Netlists/PARAMS2/Params_B0.cir",
        "Netlists/PARAMS2/Params_B1.cir",
        "Netlists/PARAMS2/Params_B2.cir",
        "Netlists/PARAMS2/Params_B3.cir",
        "Netlists/PARAMS2/Params_B4.cir",
        "Netlists/PARAMS2/Params_B5.cir",
        "Netlists/PARAMS2/Params_B5_hs.cir",
        "Netlists/PARAMS2/Params_B6.cir",
        "Netlists/PARAMS2/Params_B6_hs.cir",
        "Netlists/PARAMS2/Params_B7.cir",
        "Netlists/PARAMS2/Params_B7_hs.cir",
        "Netlists/PARAMS2/Params_C0.cir",
        "Netlists/PARAMS2/Params_C1.cir",
        "Netlists/PARAMS2/Params_C2.cir",
        "Netlists/PARAMS2/Params_C3.cir",
        "Netlists/PARAMS2/Params_C3_hs.cir",
        "Netlists/PARAMS2/Params_C4.cir",
        "Netlists/PARAMS2/Params_C4_hs.cir",
        "Netlists/PARAMS2/Params_C5.cir",
        "Netlists/PARAMS2/Params_C5_hs.cir",
        "Netlists/PARAMS2/Params_C6.cir",
        "Netlists/PARAMS2/Params_C7.cir",
        "Netlists/PARAMS2/Params_C8.cir",
        "Netlists/PARAMS2/Params_C9.cir",
        "Netlists/PARAMS2/Params_D0.cir",
        "Netlists/PARAMS2/Params_D1.cir",
        "Netlists/PARAMS2/Params_D2.cir",
        "Netlists/PARAMS2/Params_D3.cir",
        "Netlists/PARAMS2/Params_D3_hs.cir",
        "Netlists/PARAMS2/Params_D4.cir",
        "Netlists/PARAMS2/Params_D4_hs.cir",
        "Netlists/PARAMS2/Params_D5.cir",
        "Netlists/PARAMS2/Params_D5_hs.cir",
        "Netlists/PARAMS2/Params_E1.cir",
        "Netlists/PARAMS2/Params_G1.cir",
        "Netlists/PJFET_DC/pjfet-2108.cir",
        "Netlists/PMOS1_DC/pmos1.cir",
        "Netlists/PNP_DC/pnp1.cir",
        "Netlists/POLY/oneVarFourthOrd.cir",
        "Netlists/POLY/poly.cir",
        "Netlists/POLY/twoVarThirdOrd.cir",
        "Netlists/REDUND_REMOVE/exprGnd.cir",
        "Netlists/RESISTOR_TD/temp_dep_2.cir",
        "Netlists/SUBCKT/subckt_h0.cir",
        "Netlists/SUBCKT/subckt_h0_hs.cir",
        "Netlists/SUBCKT/subckt_i1.cir",
        "Netlists/SUBCKT/subckt_i2.cir",
        "Netlists/SUBCKT/subckt_i3.cir",
        "Netlists/SUBCKT/subckt_i3_hs.cir",
        "Netlists/SUPERNODE/supernode2.cir",
        "Netlists/SUPERNODE/supernode3.cir",
        "Netlists/SUPERNODE/supernode4.cir",
        "Netlists/SUPERNODE/supernode5.cir",
        "Netlists/UNARYOPS/unary_funcs.cir",
        "Netlists/VDMOS_DC/irf130.cir",
        "Netlists/VDMOS_DC/irhc110.cir",
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
fn test_xyce_level1_npn_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MCNC_BJT_LATCH/latch.cir",
        "Netlists/MCNC_BJT_RCA/rca.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run inside the validated native Level-1 NPN transient envelope, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_static_ac_fd_prn_wrapper_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_DB/dbTest.cir",
        "Netlists/ACtests/RC_simple.cir",
        "Netlists/ACtests/lowpass_old.cir",
        "Netlists/ACtests/reg0.cir",
        "Netlists/Output/AC/ac-gnuplot.cir",
        "Netlists/LEAD_CURRENTS/lead_min_ac.cir",
        "Netlists/LEAD_CURRENTS/lead_bsrc_ac.cir",
        "Netlists/COMPLEX_NUM/test1.cir",
        "Netlists/COMPLEX_NUM/test3.cir",
        "Netlists/Certification_Tests/BUG_401_SON/bug_401.cir",
        "Netlists/Certification_Tests/BUG_407_SON/bug_407_ac.cir",
        "Netlists/Certification_Tests/BUG_1035_SON/RC_AC_data_exprAlone.cir",
        "Netlists/Certification_Tests/BUG_1035_SON/RC_simple.cir",
        "Netlists/Certification_Tests/BUG_1212_SON/bug1212.cir",
        "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir",
        "Netlists/Certification_Tests/BUG_701_SON/ac_files.cir",
        "Netlists/Output/AC/ac-phase-in-radians.cir",
        "Netlists/Output/AC/ac-prn.cir",
        "Netlists/Output/AC/ac-prn-diff.cir",
        "Netlists/Output/AC/ac-touchstone-defaults-to-prn.cir",
        "Netlists/Output/AC/op-print-line-order.cir",
        "Netlists/Output/AC/op-prn.cir",
        "Netlists/Output/Dasho/ac.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin Xyce AC .FD.prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .FD.prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_fd_prn_ac",
            "{relative} should report the wrapper-origin AC .FD.prn contract"
        );
    }
}

#[test]
fn test_xyce_static_ac_fd_csv_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_701_SON/ac_files_csv.cir",
        "Netlists/Output/AC/ac-csv.cir",
        "Netlists/Output/AC/op-csv.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin Xyce AC .FD.csv comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .FD.csv oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_fd_csv_ac",
            "{relative} should report the wrapper-origin AC .FD.csv contract"
        );
    }
}

#[test]
fn test_xyce_static_ac_ic_td_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Output/AC/op-td-prn.cir";
    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin Xyce AC_IC .TD.prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .TD.prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_td_prn_ac_ic",
        "{relative} should report the wrapper-origin AC_IC .TD.prn contract"
    );
}

#[test]
fn test_xyce_static_ac_probe_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Output/AC/ac-probe.cir";
    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin Xyce AC PROBE/CSDF comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .csd oracle"
    );
    assert_eq!(
        result.contract, "wrapper_csd_ac",
        "{relative} should report the wrapper-origin AC CSDF contract"
    );
}

#[test]
fn test_xyce_static_ac_step_fd_prn_wrapper_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Output/AC/ac-step-gnuplot.cir",
        "Netlists/Output/AC/ac-stepnum-col.cir",
        "Netlists/Output/AC/ac-step-prn.cir",
        "Netlists/Output/AC/op-step-prn.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed stepped AC PRN wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin stepped Xyce AC .FD.prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in stepped Xyce .FD.prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_fd_prn_step_ac",
            "{relative} should report the wrapper-origin stepped AC .FD.prn contract"
        );
    }
}

#[test]
fn test_xyce_static_ac_step_probe_wrapper_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Output/AC/ac-step-probe.cir",
        "Netlists/Output/AC/op-step-probe.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed stepped AC PROBE wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin stepped Xyce AC PROBE/CSDF comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in stepped Xyce .csd oracle"
        );
        assert_eq!(
            result.contract, "wrapper_csd_step_ac",
            "{relative} should report the wrapper-origin stepped AC CSDF contract"
        );
    }
}

#[test]
fn test_xyce_transient_delimiter_option_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_302/transient_comma.cir",
        "Netlists/Certification_Tests/BUG_302/transient_defaults.cir",
        "Netlists/Certification_Tests/BUG_302/transient_tab.cir",
        "Netlists/Certification_Tests/BUG_302/transient_delimiter_invalid.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce transient delimiter .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_hyperbolic_abm_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_254/atanh_tanh.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce hyperbolic ABM transient .prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_307_311_wrapper_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_307/bug_307_d.cir",
        "Netlists/Certification_Tests/BUG_311/bug_311_a.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin Xyce transient .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_tran",
            "{relative} should report the wrapper-origin transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_bug_307_311_native_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_307/bug_307_e.cir",
        "Netlists/Certification_Tests/BUG_307/bug_307_f.cir",
        "Netlists/Certification_Tests/BUG_307/bug_307_g.cir",
        "Netlists/Certification_Tests/BUG_307/bug_307_h.cir",
        "Netlists/Certification_Tests/BUG_307/bug_307_i.cir",
        "Netlists/Certification_Tests/BUG_311/bug_311_b.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce transient .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_bug_318_scoped_model_family_runs_exactly() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_318/bug_318.cir",
            "scoped_model_family_wrapper",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_318/bug_318_noscope.cir",
            "scoped_model_family_baseline",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass the exact scoped-model relational contract, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should have bit-exact time/probe/value parity with its sibling representation"
        );
        assert_eq!(result.contract, expected_contract);
    }
}

#[test]
fn test_xyce_bjt_external_node_family_runs_exact_dc_parity() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/BJT_EXTNODE/npn.cir",
            "bjt_external_node_family_wrapper",
            true,
        ),
        (
            "Netlists/BJT_EXTNODE/npn1.cir",
            "bjt_external_node_family_baseline",
            false,
        ),
        (
            "Netlists/BJT_EXTNODE/npn2.cir",
            "bjt_external_node_family_wrapper",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass exact omitted/explicit grounded-substrate DC parity, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should have bit-exact sweep/probe/value parity with its sibling representation"
        );
        assert_eq!(result.contract, expected_contract);
    }
}

#[test]
fn test_xyce_sin_expression_family_runs_exact_transient_parity() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_791_SON/bug791.cir",
            "sin_expression_family_wrapper",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_791_SON/bug791_vsrc.cir",
            "sin_expression_family_baseline",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_791_SON/bug791_expr.cir",
            "sin_expression_family_wrapper",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass exact independent-SIN/behavioral-SPICE_SIN parity, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should have bit-exact time/probe/value parity with its sibling representation"
        );
        assert_eq!(result.contract, expected_contract);
    }
}

#[test]
fn test_xyce_bug_1806_parameter_expression_family_runs_canonical_transient_parity() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_1806/bug_1806.cir",
            "param_expression_family_wrapper",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_1806/bug_1806_2.cir",
            "param_expression_family_baseline",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1806/bug_1806_2a.cir",
            "param_expression_family_wrapper",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass strict parameter/literal expression qualification and the canonical xyce_verify transient oracle, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy the Release 7.10 interpolated RMS comparison"
        );
        assert_eq!(result.contract, expected_contract);
    }
}

#[test]
fn test_xyce_bug_374_passive_primary_value_composite_runs_exact_prn_parity() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/BUG_374/bug_374.cir",
            "passive_primary_value_composite_wrapper",
            true,
        ),
        (
            "Netlists/BUG_374/capacitor.cir",
            "passive_primary_value_capacitor_tran_baseline",
            false,
        ),
        (
            "Netlists/BUG_374/capacitor-bug.cir",
            "passive_primary_value_capacitor_tran_wrapper",
            false,
        ),
        (
            "Netlists/BUG_374/resistor.cir",
            "passive_primary_value_resistor_dc_baseline",
            false,
        ),
        (
            "Netlists/BUG_374/resistor-bug.cir",
            "passive_primary_value_resistor_dc_wrapper",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass exact named/positional passive primary-value PRN parity, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy the Release 7.10 byte-exact default-PRN comparison"
        );
        assert_eq!(result.contract, expected_contract);
    }
}

#[test]
fn test_xyce_capacitor_analytic_first_order_rc_wrapper_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/CAPACITOR/capacitor.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed analytic-wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass the generated Release 7.10 analytic RC oracle, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should satisfy the default xyce_verify integrated transient comparison"
    );
    assert_eq!(result.contract, "analytic_first_order_rc_tran_wrapper");
}

#[test]
fn test_xyce_capacitor_analytic_newlte_sibling_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/CAPACITOR/capacitor3.cir";

    assert!(runner.requires_upstream_wrapper(relative));
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its generated Release 7.10 NEWLTE=2 analytic RC oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
    assert_eq!(result.contract, "analytic_first_order_rc_tran_wrapper");
}

#[test]
fn test_xyce_analytic_sinusoidal_rc_wrappers_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/CAPACITOR/rc_osc.cir",
        "Netlists/TIA/TRAP/CAPACITOR/rc_osc.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed analytic-wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass the generated Release 7.10 sinusoidal RC oracle, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy its custom xyce_verify integrated transient comparison"
        );
        assert_eq!(
            result.contract,
            "analytic_sinusoidal_first_order_rc_tran_wrapper"
        );
    }
}

#[test]
fn test_xyce_bare_level9_comparator_uses_release_710_integrated_verifier() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/COMPARATOR/comparator.cir";

    assert!(
        !runner.requires_upstream_wrapper(relative),
        "{relative} is an ordinary checked-PRN regression, not a wrapper-origin contract"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass the Release 7.10 integrated transient verifier, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should remain inside the authoritative normalized-RMS bound"
    );
    assert_eq!(result.contract, "static_xyce_verify_prn_tran");
}

#[test]
fn test_xyce_mid_certification_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_263/bug_263.cir",
            "wrapper_static_prn_tran",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_338_SON/bug_338.cir",
            "wrapper_static_prn_tran",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_427_SON/bug_427.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_718_SON/voltageDiff.cir",
            "wrapper_static_prn_tran",
            true,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper manifest provenance should match the promoted contract"
        );

        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce certification transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_bug_229_native_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_229_SON/bug229son.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce certification transient .prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_629_power_alias_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_629_SON/bug_629.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce transient P/W power-probe comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_1301_wrapper_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/BUG_1301/rlc_tranline.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin RLC transmission-line transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_tran",
        "{relative} should report the wrapper-origin transient .prn contract"
    );
}

#[test]
fn test_xyce_prf_parameter_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1210_SON/bug1210.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin PRF parameter transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_tran",
        "{relative} should report the wrapper-origin transient .prn contract"
    );
}

#[test]
fn test_xyce_tl1x_mpi_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/TL1X/TL1X_mpi.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce TL1X transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_61_constant_step_transient_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_61/capacitor.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin constant-step transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_tran",
        "{relative} should report the wrapper-origin transient .prn contract"
    );
}

#[test]
fn test_xyce_mixed_signal_python_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/MIXED_SIGNAL/Python/runACircuitWithBackwardStep.cir",
            "wrapper_static_prn_tran",
            true,
        ),
        (
            "Netlists/MIXED_SIGNAL/Python/runMultipleSims1.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/MIXED_SIGNAL/Python/runMultipleSims2.cir",
            "static_prn_tran",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper manifest provenance should match the promoted contract"
        );

        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce mixed-signal Python transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_nonlinear_convergence_wrapper_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Nonlinear/ConvergenceTests/Nox1Tran0.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin Xyce nonlinear convergence transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_tran",
        "{relative} should report the wrapper-origin transient .prn contract"
    );
}

#[test]
fn test_xyce_zero_resistance_branch_current_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/RESISTOR/resistor_lv3.cir",
        "Netlists/RESISTOR/resistor_lv3_2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce zero-resistance branch-current comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_resistor_default_value_warning_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/RESISTOR/DefaultValueWarning.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream warning-wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce resistor-default warning and .prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_resistor_default_prn_dc",
        "{relative} should report the native resistor-default wrapper contract"
    );
}

#[test]
fn test_xyce_diode_sidewall_cd_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/DIODE/diode_with_sidewall.cir",
        "Netlists/DIODE/diode_with_sidewall_fcs.cir",
        "Netlists/DIODE/diode_with_sidewall_mjsw.cir",
        "Netlists/DIODE/diode_with_sidewall_nbv.cir",
        "Netlists/DIODE/diode_with_sidewall_ns.cir",
        "Netlists/DIODE/diode_with_sidewall_php.cir",
        "Netlists/DIODE/diode_with_sidewall_temp.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce diode sidewall Cd comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_include_alias_and_path_resolution_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_1131_SON/bug_1131.cir",
        "Netlists/Certification_Tests/BUG_1325_SON/inc_lib_file_relative_path.cir",
        "Netlists/Certification_Tests/BUG_1325_SON/Win/drive_letter_no_slash_path.cir",
        "Netlists/Certification_Tests/BUG_1325_SON/Win/inc_lib_file_relative_path.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce include-path comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_top_level_execution_dir_include_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1325_SON/top_level_file_path.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream top-level execution-directory wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce include execution-directory comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_top_level_execution_dir_prn_dc",
        "{relative} should report the native top-level execution-directory wrapper contract"
    );
}

#[test]
fn test_xyce_absolute_include_library_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_1325_SON/inc_lib_file_absolute_path.cir",
        "Netlists/Certification_Tests/BUG_1325_SON/Win/inc_lib_file_absolute_path.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream absolute include/library wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce absolute include/library comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_dc",
            "{relative} should report the native wrapper-origin static .prn DC contract"
        );
    }
}

#[test]
fn test_xyce_vpwl_delay_repeat_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in ["Netlists/VPWL/vpwl.cir", "Netlists/VPWL/vpwl_filebased.cir"] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce PWL delay/repeat comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_vpwl_step_delay_repeat_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/VPWL/vpwl_repeat_step.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream .STEP TRAN wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce stepped PWL transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_step_tran",
        "{relative} should report the native wrapper-origin stepped transient .prn contract"
    );
}

#[test]
fn test_xyce_vpwl_repeat_error_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let primary = "Netlists/Certification_Tests/BUG_630_SON/vpwl.cir";
    assert!(
        runner.requires_upstream_wrapper(primary),
        "{primary} should retain its removed upstream VPWL error-wrapper provenance"
    );
    let primary_result = runner.run_test(root.join(primary));
    assert!(
        primary_result.passed && !primary_result.expected_unsupported,
        "{primary} should run as a native VPWL transient comparison with sibling expected-error validation, got {primary_result:?}"
    );
    assert!(
        primary_result.mismatches.is_empty(),
        "{primary} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        primary_result.contract, "wrapper_static_prn_tran_expected_error",
        "{primary} should report the native wrapper-origin expected-error contract"
    );

    let repeat_fail = "Netlists/Certification_Tests/BUG_630_SON/vpwlRepeatFail.cir";
    let repeat_fail_result = runner.run_test(root.join(repeat_fail));
    assert!(
        repeat_fail_result.passed && !repeat_fail_result.expected_unsupported,
        "{repeat_fail} should run as a native Xyce expected-error PWL repeat validation, got {repeat_fail_result:?}"
    );
    assert_eq!(
        repeat_fail_result.contract, "expected_error_pwl_repeat_value",
        "{repeat_fail} should report the native expected-error contract"
    );

    let bad_source = "Netlists/Certification_Tests/BUG_657_SON/Bad_PWL_Source.cir";
    assert!(
        runner.requires_upstream_wrapper(bad_source),
        "{bad_source} should retain its removed upstream VPWL error-wrapper provenance"
    );
    let bad_source_result = runner.run_test(root.join(bad_source));
    assert!(
        bad_source_result.passed && !bad_source_result.expected_unsupported,
        "{bad_source} should run as a native Xyce expected-error PWL repeat validation, got {bad_source_result:?}"
    );
    assert_eq!(
        bad_source_result.contract, "expected_error_pwl_repeat_value",
        "{bad_source} should report the native expected-error contract"
    );
}

#[test]
fn test_xyce_pat_pattern_source_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in ["Netlists/VPAT/vpat.cir", "Netlists/IPAT/ipat.cir"] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce PAT transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should use the standard native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_independent_voltage_source_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/VSIN/vsin.cir",
        "Netlists/VSIN/bug510.cir",
        "Netlists/VSIN/bug1679.cir",
        "Netlists/VPULSE/vpulse.cir",
        "Netlists/VEXP/vexp.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce independent voltage-source transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should use the standard native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_source_waveform_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/VSFFM/vsffm.cir",
        "Netlists/SOURCES/sources.cir",
        "Netlists/SOURCES/ebh.cir",
        "Netlists/SOURCES/ebh_sub.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce source-waveform transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_global_node_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/GLOBALNODE/global2.cir",
        "Netlists/GLOBALNODE/global3.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce global-node transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_current_source_waveform_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/IEXP/iexp.cir",
        "Netlists/IPULSE/ipulse.cir",
        "Netlists/IPWL/ipwl.cir",
        "Netlists/ISFFM/isffm.cir",
        "Netlists/ISIN/isin.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce current-source waveform transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_pat_pattern_source_step_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in ["Netlists/VPAT/vpat_step.cir", "Netlists/IPAT/ipat_step.cir"] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream .STEP TRAN wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native stepped Xyce PAT transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_step_tran",
            "{relative} should report the native wrapper-origin stepped transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_sin_source_step_transient_output_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Output/TRAN/tran-step-prn.cir",
        "Netlists/Output/TRAN/tran-step-gnuplot.cir",
        "Netlists/Output/TRAN/tran-stepnum-col.cir",
        "Netlists/Output/TRAN/tran-step-tecplot.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream .STEP TRAN wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native stepped Xyce SIN transient output comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_step_tran",
            "{relative} should report the native wrapper-origin stepped transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_default_prn_transient_output_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Output/Other/blankPrint.cir",
        "Netlists/Output/TRAN/tran-gnuplot.cir",
        "Netlists/Output/TRAN/tran-prn.cir",
        "Netlists/Output/TRAN/tran-prn-comma.cir",
        "Netlists/Output/TRAN/tran-prn-filter.cir",
        "Netlists/Output/TRAN/tran-prn-noindex.cir",
        "Netlists/Output/TRAN/tran-prn-precision.cir",
        "Netlists/Output/TRAN/tran-prn-timescalefactor.cir",
        "Netlists/Output/TRAN/tran-prn-width.cir",
        "Netlists/Output/TRAN/tran-splot.cir",
        "Netlists/Output/TRAN/tran-touchstone-defaults-to-prn.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream transient PRN wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin transient PRN comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_tran",
            "{relative} should report the native wrapper-origin transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_probe_transient_output_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Output/TRAN/tran-probe.cir";
    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream transient PROBE wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin transient PROBE/CSDF comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .csd oracle"
    );
    assert_eq!(
        result.contract, "wrapper_csd_tran",
        "{relative} should report the native wrapper-origin transient CSDF contract"
    );
}

#[test]
fn test_xyce_csv_transient_output_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Output/TRAN/tran-csv.cir",
        "Netlists/Output/TRAN/tran-csv-snapshots.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream transient CSV wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin transient CSV comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .csv oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_csv_tran",
            "{relative} should report the native wrapper-origin transient CSV contract"
        );
    }
}

#[test]
fn test_xyce_output_initial_interval_transient_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_133/bug_133.cir",
        "Netlists/Certification_Tests/BUG_256/bug_256.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream output-interval wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native output-interval transient PRN comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_tran",
            "{relative} should report the native wrapper-origin transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_hierarchical_passive_transient_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_1962/ic.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native hierarchical passive transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_scoped_ic_hierarchical_passive_transient_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_1962/ic2.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native scoped-IC hierarchical passive transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_hierarchical_print_alias_transient_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_1962/print.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native hierarchical print-alias transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_nodeset_hierarchical_behavioral_transient_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_1962/nodeset.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native hierarchical NODESET behavioral transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_mixed_device_param_source_step_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_466_SON/bug_466.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native mixed device/parameter stepped Xyce SIN transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_step_tran",
        "{relative} should report the native stepped transient .prn contract"
    );
}

#[test]
fn test_xyce_inductor_ic_transient_operating_point_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BUG_174/nlrcs10.cir",
        "Netlists/Certification_Tests/BUG_201_SON/bug201a.cir",
        "Netlists/Certification_Tests/BUG_201_SON/bug201b.cir",
        "Netlists/MULTIPLICITY_FACTOR/inductor_ic.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce inductor IC transient operating-point comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_midrange_certification_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_574/bug_574_mpi.cir",
        "Netlists/Certification_Tests/BUG_575_SON/bug575son.cir",
        "Netlists/Certification_Tests/BUG_794_SON/test1.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce certification transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_noise_cancel_wrapper_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_534_SON/noise_cancel_test1.cir",
        "Netlists/Certification_Tests/BUG_534_SON/noise_cancel_test1_rf.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin Xyce noise-cancel transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_tran",
            "{relative} should report the wrapper-origin transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_dc_upgrade_sweep_modes_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/DC_UPGRADE/dcList.cir",
        "Netlists/DC_UPGRADE/dcDec.cir",
        "Netlists/DC_UPGRADE/dcOct.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce DC sweep-mode comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_issue_405_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/ISSUE_405/test1.cir",
            "static_prn_dc",
        ),
        (
            "Netlists/Certification_Tests/ISSUE_405/test2.cir",
            "static_prn_step_dc",
        ),
        (
            "Netlists/Certification_Tests/ISSUE_405/test3.cir",
            "static_prn_dc",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce ISSUE 405 static comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected static .prn contract"
        );
    }
}

#[test]
fn test_xyce_step_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_606_SON/resistor.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params_step.cir",
        "Netlists/Certification_Tests/BUG_654_SON/bug_654.cir",
        "Netlists/Certification_Tests/BUG_1203_SON/dot_step.cir",
        "Netlists/Certification_Tests/BUG_1144_SON/test3.cir",
        "Netlists/GLOBALPAR/gp2.cir",
        "Netlists/NJFET_DC/njfet.cir",
        "Netlists/PARAM_REFACTOR/paramDep2.cir",
        "Netlists/PARAM_REFACTOR/paramDep3.cir",
        "Netlists/PARAM_REFACTOR/paramDep4.cir",
        "Netlists/PARAM_REFACTOR/paramStep1.cir",
        "Netlists/PJFET_DC/pjfet.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce .STEP DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_step_dc",
            "{relative} should run through the native stepped static DC contract"
        );
    }
}

#[test]
fn test_xyce_time_dependent_global_parameter_transient_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/GLOBALPAR/gp3.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native time-dependent global-parameter transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(result.contract, "static_prn_tran");
}

#[test]
fn test_xyce_global_parameter_function_step_transient_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/GLOBALPAR/gp_func.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native stepped transient with a global parameter captured by a function, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match every checked-in stepped Xyce .prn oracle block"
    );
    assert_eq!(result.contract, "static_prn_step_tran");
}

#[test]
fn test_xyce_bug_616_wrapper_step_dc_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_616/bug_616.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin Xyce .STEP DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_step_dc",
        "{relative} should report the native wrapper-origin stepped .prn contract"
    );
}

#[test]
fn test_xyce_param_refactor_dependency_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/PARAM_REFACTOR/paramDep1.cir", "static_prn_dc"),
        ("Netlists/PARAM_REFACTOR/paramDep5.cir", "static_prn_tran"),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce parameter-dependency comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected native .prn contract"
        );
    }
}

#[test]
fn test_xyce_step_data_static_dc_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARAM_REFACTOR/paramStep2.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream .STEP DATA wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce .STEP DATA comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_step_dc",
        "{relative} should report the native wrapper-origin stepped .prn contract"
    );
}

#[test]
fn test_xyce_repeated_dc_cards_form_one_sweep_vector() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_695/bug695.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as one Xyce DC sweep vector across repeated .DC cards, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_dc",
        "{relative} should use the standard static .prn DC comparison once the repeated .DC cards are normalized"
    );
}

#[test]
fn test_xyce_solution_dependent_resistor_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/SOLN_DEP_RES/solDepRes.cir",
        "Netlists/SOLN_DEP_RES/solDepRes2.cir",
        "Netlists/SOLN_DEP_RES/solDepRes3.cir",
        "Netlists/SOLN_DEP_RES/multSolDepRes.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce solution-dependent resistor comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_semiconductor_resistor_step_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/SEMIC_RESISTOR/semic_resistor_step.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin semiconductor resistor .STEP comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_step_dc",
        "{relative} should report the native wrapper-origin stepped .prn contract"
    );
}

#[test]
fn test_xyce_resistor_family_native_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/SEMIC_RESISTOR/semic_resistor.cir",
            "static_prn_dc",
        ),
        ("Netlists/NL_RESISTOR/nlrcs10.cir", "static_prn_tran"),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce resistor-family comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected native .prn contract"
        );
    }
}

#[test]
fn test_xyce_deep_function_parameter_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_1222_SON/bug_1222_son_1.cir",
        "Netlists/Certification_Tests/BUG_1222_SON/bug_1222_son_2.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce deep .FUNC parameter comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_param_function_syntax_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/PARSER/paramFunc1.cir",
        "Netlists/PARSER/paramFunc2.cir",
        "Netlists/PARSER/paramFunc3.cir",
        "Netlists/PARSER/paramFunc4.cir",
        "Netlists/PARSER/paramFunc5.cir",
        "Netlists/PARSER/paramFunc6.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce .PARAM function-syntax comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_parser_certification_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/PARSER/bug_203.cir",
        "Netlists/PARSER/bug_250.cir",
        "Netlists/Certification_Tests/BUG_28_SON/bug_28_son1.cir",
        "Netlists/Certification_Tests/BUG_138/bug_138_1.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce parser/certification static DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_inline_comment_static_dc_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/BUG_139/in_line_comment.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce inline-comment static DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_dc",
        "{relative} should report the native static DC .prn contract"
    );
}

#[test]
fn test_xyce_legacy_test_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/TEST1/test1.cir",
        "Netlists/TEST2/test2.cir",
        "Netlists/TEST3/test3.cir",
        "Netlists/TEST4/test4.cir",
        "Netlists/TEST5/test5.cir",
        "Netlists/TEST6/test6.cir",
        "Netlists/TEST7/test7.cir",
        "Netlists/TEST8/test8.cir",
        "Netlists/TEST9/test9.cir",
        "Netlists/TEST10/test10.cir",
        "Netlists/TEST11/test11.cir",
        "Netlists/TEST12/test12.cir",
        "Netlists/TEST13/test13.cir",
        "Netlists/TEST14/test14.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce legacy TEST static DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_load_static_dc_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/LOAD/load.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce load static DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_dc",
        "{relative} should report the native static DC .prn contract"
    );
}

#[test]
fn test_xyce_sandler_op_amp_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/SANDLER23/sandler23.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce Sandler op-amp transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_1302_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1302/bug_1302.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce certification transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bug_1173_1176_certification_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_1173_1176_SON/bug1173son.cir",
        "Netlists/Certification_Tests/BUG_1173_1176_SON/bug1176son.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce certification transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_post_1770_certification_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_1794/onedevice.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1803/bug_1803a.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1803/bug_1803b.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1803/bug_1803c.cir",
            "static_prn_tran",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1847/bug1847.cir",
            "wrapper_static_prn_tran",
            true,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper manifest provenance should match the promoted contract"
        );

        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce certification transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_certification_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_21_SON/func_loop.cir",
        "Netlists/Certification_Tests/BUG_21_SON/func_param.cir",
        "Netlists/Certification_Tests/BUG_250/bug_250.cir",
        "Netlists/Certification_Tests/BUG_264_SON/bug_264.cir",
        "Netlists/Certification_Tests/BUG_428/bug428.cir",
        "Netlists/Certification_Tests/BUG_525_SON/bug_525.cir",
        "Netlists/Certification_Tests/BUG_584_SON/bug_584.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params_dev_options.cir",
        "Netlists/Certification_Tests/BUG_913_SON/bug913son.cir",
        "Netlists/Certification_Tests/BUG_1113_SON/bug_1113_SON.cir",
        "Netlists/Certification_Tests/BUG_1203_SON/default_temp.cir",
        "Netlists/Certification_Tests/BUG_1203_SON/device_options.cir",
        "Netlists/Certification_Tests/BUG_1377/test1.X.cir",
        "Netlists/Certification_Tests/BUG_1460/bug_1460.cir",
        "Netlists/Certification_Tests/BUG_159/bug_159_1.cir",
        "Netlists/Certification_Tests/BUG_159/bug_159_2.cir",
        "Netlists/Certification_Tests/BUG_1770/HBT_IV_nosweep.cir",
        "Netlists/Certification_Tests/ISSUE_235/issue235.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce certification static DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_meter_unit_suffix_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARSER/meterUnit.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce prefixed-meter unit suffix comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_transient_x_scale_suffix_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARSER/scalingFactors.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce transient X-scale suffix comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_transient_branch_current_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_343/bug_343.cir",
        "Netlists/ISFFM/isffm.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce transient branch-current comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_transient_resistor_branch_current_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/TIA/Interpolation_TIA/interpolation_gear.cir",
        "Netlists/TIA/Interpolation_TIA/interpolationtrap.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce transient resistor branch-current comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_resistor_model_static_dc_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_524_SON/rmod_resmod.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce resistor-model static DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_dc",
        "{relative} should report the native static DC .prn contract"
    );
}

#[test]
fn test_xyce_inductor_model_static_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_524_SON/lmod_indmod.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce inductor-model transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_transient_capacitor_branch_current_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_524_SON/cmod_capmod.cir",
        "Netlists/MULTIPLICITY_FACTOR/capacitor_ic.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce transient capacitor branch-current comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_capacitor_multiplicity_step_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MULTIPLICITY_FACTOR/capacitor.cir",
        "Netlists/MULTIPLICITY_FACTOR/semic_capacitor.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce stepped capacitor multiplicity transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_step_tran",
            "{relative} should report the native stepped transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_linear_coupled_inductor_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MINDUCTORS/InductorICBug.cir",
        "Netlists/MINDUCTORS/MINDUCTORS.cir",
        "Netlists/MINDUCTORS/MINDUCTORS_IC.cir",
        "Netlists/MINDUCTORS/cpldLMIs.cir",
        "Netlists/MINDUCTORS/cpldLMIs_ic.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce linear coupled-inductor transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_zero_step_coupled_inductor_parameter_print_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract) in [
        ("Netlists/MINDUCTORS/mutIndPrint1.cir", "static_prn_tran"),
        ("Netlists/MINDUCTORS/mutIndPrint2.cir", "static_prn_tran"),
        ("Netlists/MINDUCTORS/mutIndStep.cir", "static_prn_step_tran"),
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce zero-step coupled-inductor parameter-print comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, contract,
            "{relative} should report the expected transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_semiconductor_capacitor_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/SEMIC_CAPACITOR/semicap.cir",
        "Netlists/SEMIC_CAPACITOR/semicap_subc.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce semiconductor capacitor transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_classic_jfet_switch_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PJFET_SWITCH/pjfet_tran.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce classic JFET transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_polyg_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/POLYG/polyg.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce POLY G-source transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_behavioral_source_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_ATAN_TAN/atan_tan.cir",
        "Netlists/ABM_BREAK/break.cir",
        "Netlists/ABM_BREAK/fastRisePulse.cir",
        "Netlists/ABM_BREAK/fastTable.cir",
        "Netlists/ABM_EXPLN/exp_ln.cir",
        "Netlists/ABM_FUNC/func.cir",
        "Netlists/ABM_LOG/log.cir",
        "Netlists/BREAK/break.cir",
        "Netlists/Certification_Tests/ISSUE_229/GlobalInExpressionBug.cir",
        "Netlists/Certification_Tests/ISSUE_310/issue310.cir",
        "Netlists/Certification_Tests/BUG_794_SON/test2.cir",
        "Netlists/Certification_Tests/BUG_86_SON/bug86.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce behavioral-source transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_abm_math_function_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/ABM_ACOS_ASIN/acos_asin.cir", "static_prn_dc"),
        ("Netlists/ABM_NINT_FMOD/nint.cir", "static_prn_tran"),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce ABM math-function comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected native .prn contract"
        );
    }
}

#[test]
fn test_xyce_braced_print_expression_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/AMPMOD/amp_mod.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce transient braced .PRINT expression comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bsource_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in ["Netlists/BSRC/Bsrc_A1.cir", "Netlists/BSRC/Bsrc_A2.cir"] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce B-source static DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_bsource_table_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/BSRC/Bsrc_C1.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce B-source TABLE transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_bsource_table_digitizer_transition_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_332/bug_332.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce B-source TABLE transition comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_controlled_source_table_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1012/negtable.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce controlled-source TABLE transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_behavioral_file_table_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/fasttable.cir",
        "Netlists/ABM_SPLINES/fasttable2.cir",
        "Netlists/ABM_SPLINES/table.cir",
        "Netlists/ABM_SPLINES/table2.cir",
        "Netlists/ABM_SPLINES/table3.cir",
        "Netlists/ABM_SPLINES/table4.cir",
        "Netlists/ABM_SPLINES/table5.cir",
        "Netlists/ABM_SPLINES/table6.cir",
        "Netlists/ABM_SPLINES/table7.cir",
        "Netlists/ABM_SPLINES/table8.cir",
        "Netlists/ABM_SPLINES/tableOutOfOrder.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce behavioral file-table transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_static_capacitor_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_1145_SON/tranRc.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce static-capacitor transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_static_inductor_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BUG_458/bug458.cir",
        "Netlists/INDUCTOR/inductor.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce static-inductor transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_temperature_dependent_passive_model_value_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/TEMPCAP/tempcap.cir",
        "Netlists/TEMPIND/tempind.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce temperature-dependent passive transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_param_type_meter_suffix_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARAM_TYPES/par_types.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce typed-parameter and meter-suffix comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_naked_conditional_parameter_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/PARSER/nakedIf.cir",
        "Netlists/PARSER/nakedTernary.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce naked conditional .PARAM comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_subckt_bare_resistor_parameter_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/PARAMS2/Params_B8.cir",
        "Netlists/PARAMS2/Params_B8_hs.cir",
        "Netlists/PARAMS2/Params_B9.cir",
        "Netlists/PARAMS2/Params_B9_hs.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce subcircuit resistor-parameter comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_resistor_value_model_temperature_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract) in [
        ("Netlists/RESISTOR_TD/temp_dep.cir", "static_prn_dc"),
        (
            "Netlists/RESISTOR_TD/exp_temp_dep.cir",
            "wrapper_static_prn_step_dc",
        ),
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce resistor value-plus-model temperature comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, contract,
            "{relative} should report the expected DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_static_device_parameter_probe_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_971_SON/bug971.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce static device-parameter probe comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_model_parameter_probe_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_601_SON/bug_601.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce model-parameter probe comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_parameter_dc_sweep_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARAM_REFACTOR/paramDc1.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce scalar-parameter DC sweep comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_resistor_model_suffix_value_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/ISSUE_206/issue206.cir",
        "Netlists/Certification_Tests/ISSUE_206/issue206binning1.cir",
        "Netlists/Certification_Tests/ISSUE_206/issue206binning2.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce modeled-resistor value-suffix comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_special_character_function_name_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/INVALID_CHARS/valid_chars_func_names.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce special-character .FUNC name comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_standalone_punctuation_node_name_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for relative in [
        "Netlists/INVALID_CHARS/colon_in_node_name.cir",
        "Netlists/INVALID_CHARS/colon_in_node_name2.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a wrapper-origin Xyce punctuation-node DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_dc",
            "{relative} should report the wrapper-origin static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_period_hierarchy_node_name_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/INVALID_CHARS/period_in_node_name.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a wrapper-origin Xyce period-hierarchy node DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_dc",
        "{relative} should report the wrapper-origin static DC .prn contract"
    );
}

#[test]
fn test_xyce_punctuation_node_ac_expression_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/INVALID_CHARS/valid_chars_ac_expressions.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a wrapper-origin Xyce punctuation-node AC expression comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .FD.prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_fd_prn_ac",
        "{relative} should report the wrapper-origin static AC .FD.prn contract"
    );
}

#[test]
fn test_xyce_naked_random_parameter_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/PARSER/nakedRandom.cir",
        "Netlists/PARSER/nakedRandomGlobal.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce nominal random-parameter comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_hspice_random_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARSER/random.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce nominal random-operator wrapper comparison, got {result:?}"
    );
    assert_eq!(result.contract, "wrapper_static_prn_dc");
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_subcircuit_qualified_node_probe_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_792/bug_792.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce subcircuit-qualified node probe comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_dc",
        "{relative} should use the native wrapper-origin static .prn DC contract"
    );
}

#[test]
fn test_xyce_subckt_wrapper_family_members_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    // The inventory metric is per vendored .cir record. The zero-byte anchor
    // represents the upstream logical wrapper, while each member record must
    // independently exercise its baseline-relative route without a vacuous
    // success or a redundant checked-in gold dependency.
    for (relative, expected_contract) in [
        ("Netlists/SUBCKT/subckt_a.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_a1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a1_dup.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a2.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a2_dup.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a3.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a3_hs.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a4.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a4_hs.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a5.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_a5_hs.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_b1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b2.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b2_hs.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b3.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b3_hs.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_e.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_e0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_e1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_e2.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_e3.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_f.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_f0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_f1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_g.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_g0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_g1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_k.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_k0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_k1.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_k2.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_k3.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_l.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_l0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_l1.cir", "subckt_family_wrapper"),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run through the native SUBCKT family contract, got {result:?}"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should not fall back to standalone static .prn comparison"
        );
    }
}

#[test]
fn test_xyce_subckt_j_family_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/SUBCKT/subckt_j1.cir",
        "Netlists/SUBCKT/subckt_j1_dup.cir",
        "Netlists/SUBCKT/subckt_j1_hs.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce SUBCKT transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_supernode_wrapper_family_members_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    assert!(
        runner.requires_upstream_wrapper("Netlists/SUPERNODE/supernode1.cir"),
        "supernode1.cir should retain its removed upstream wrapper provenance"
    );

    for relative in [
        "Netlists/SUPERNODE/supernode1.cir",
        "Netlists/SUPERNODE/supernode1a.cir",
        "Netlists/SUPERNODE/supernode1b.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run through the native SUPERNODE family contract, got {result:?}"
        );
        assert_eq!(
            result.contract, "supernode_family_wrapper",
            "{relative} should not fall back to standalone static .prn comparison"
        );
    }
}

#[test]
fn test_xyce_output_dc_default_prn_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/Output/DC/dc-prn.cir", "wrapper_static_prn_dc"),
        ("Netlists/Output/DC/op-prn.cir", "wrapper_static_prn_dc"),
        (
            "Netlists/Output/DC/op-prn-nodc.cir",
            "wrapper_static_prn_dc",
        ),
        (
            "Netlists/Output/DC/dc-step-prn.cir",
            "wrapper_static_prn_step_dc",
        ),
        (
            "Netlists/Output/DC/op-step-prn-nodc.cir",
            "wrapper_static_prn_step_dc",
        ),
        (
            "Netlists/Output/DC/dc-step-tecplot.cir",
            "wrapper_static_prn_step_dc",
        ),
        (
            "Netlists/Output/DC/dc-stepnum-col.cir",
            "wrapper_static_prn_step_dc",
        ),
        (
            "Netlists/Output/DC/dc-touchstone-defaults-to-prn.cir",
            "wrapper_static_prn_dc",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin default .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the native wrapper-origin .prn contract"
        );
    }
}

#[test]
fn test_xyce_output_dc_csv_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Output/DC/dc-csv.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin CSV comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .csv oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_csv_dc",
        "{relative} should report the native wrapper-origin CSV contract"
    );
}

#[test]
fn test_xyce_output_dc_probe_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/Output/DC/dc-probe.cir", "wrapper_csd_dc"),
        (
            "Netlists/Output/DC/dc-step-probe.cir",
            "wrapper_csd_step_dc",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin PROBE/CSDF comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .csd oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the native wrapper-origin CSDF contract"
        );
    }
}

#[test]
fn test_xyce_output_dc_file_only_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Output/DC/dc-multiprn.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native wrapper-origin file-output comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce side-output .prn oracles"
    );
    assert_eq!(
        result.contract, "wrapper_file_prn_dc",
        "{relative} should report the native wrapper-origin file-output contract"
    );
}

#[test]
fn test_xyce_plain_static_dc_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/XDM/HSPICE/CONTROLLED_SOURCES/voltage_controlled_sources_w_extraneous_identifiers.cir",
        "Netlists/XDM/HSPICE/CONTROLLED_SOURCES/current_controlled_sources_w_extraneous_identifiers.cir",
        "Netlists/XDM/HSPICE/OTHER_PARSING/mixed_param_and_func.cir",
        "Netlists/XDM/HSPICE/OTHER_PARSING/library_parsing.cir",
        "Netlists/XDM/HSPICE/MODELS/correct_instance_parameters_translation_inside_subckt.cir",
        "Netlists/Certification_Tests/BUG_204/bug204.cir",
        "Netlists/XDM/HSPICE/TEMPERATURE/tnom_default_setting.cir",
        "Netlists/XDM/HSPICE/TEMPERATURE/tnom_option_setting.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native plain static DC wrapper comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_dc",
            "{relative} should report the native wrapper-origin static .prn DC contract"
        );
    }
}

#[test]
fn test_xyce_model_binning_static_dc_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MODEL_BINNING/dcModelBinning.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native model-binning static DC wrapper comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_dc",
        "{relative} should report the native wrapper-origin static .prn DC contract"
    );
}

#[test]
fn test_xyce_plain_static_dc_wrapper_guardrails_stay_unsupported() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/XDM/HSPICE/OTHER_PARSING/ternary_operator.cir",
        "Netlists/XDM/HSPICE/MODELS/correct_instance_parameters.cir",
        "Netlists/XDM/PSPICE/OTHER_PARSING/probe_dc_lines.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} should stay unsupported until its broader wrapper semantics are implemented, got {result:?}"
        );
        assert_eq!(
            result.contract, "unsupported_xyce_contract",
            "{relative} should not be promoted by the plain static DC wrapper contract"
        );
    }
}

#[test]
fn test_xyce_output_dc_raw_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/Output/DC/dc-raw.cir", "wrapper_raw_dc"),
        ("Netlists/Output/DC/dc-raw-ascii.cir", "wrapper_raw_dc"),
        ("Netlists/Output/DC/dc-raw-override.cir", "wrapper_raw_dc"),
        (
            "Netlists/Output/DC/dc-raw-override-ascii.cir",
            "wrapper_raw_dc",
        ),
        (
            "Netlists/Output/DC/dc-step-raw-override.cir",
            "wrapper_raw_step_dc",
        ),
        (
            "Netlists/Output/DC/dc-step-raw-override-ascii.cir",
            "wrapper_raw_step_dc",
        ),
        ("Netlists/Output/DC/op-raw-override.cir", "wrapper_raw_dc"),
        (
            "Netlists/Output/DC/op-raw-override-ascii.cir",
            "wrapper_raw_dc",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream RAW wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native wrapper-origin RAW comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .raw oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the native RAW wrapper contract"
        );
    }
}

#[test]
fn test_xyce_output_dc_gnuplot_splot_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Output/DC/dc-gnuplot.cir",
            "wrapper_gnuplot_splot_prn_dc",
        ),
        (
            "Netlists/Output/DC/dc-step-gnuplot.cir",
            "wrapper_gnuplot_splot_prn_step_dc",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native GNUPLOT/SPLOT .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the native GNUPLOT/SPLOT wrapper contract"
        );
    }
}

#[test]
fn test_xyce_hspice_math_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARSER/hspiceMath.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream HSPICE-extension wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native HSPICE math .prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_hspice_math_prn_dc",
        "{relative} should report the native HSPICE math wrapper contract"
    );

    let invalid_cli_wrapper = "Netlists/PARSER/bad-hspice-ext.cir";
    let result = runner.run_test(root.join(invalid_cli_wrapper));
    assert!(
        result.passed && result.expected_unsupported,
        "{invalid_cli_wrapper} should stay named unsupported until RSpice has an equivalent Xyce command-line diagnostic contract, got {result:?}"
    );
}

#[test]
fn test_xyce_voltage_accessor_wrapper_case_runs_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_407_SON/bug_407_dc.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce voltage-accessor comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_voltage_accessor_prn_dc",
        "{relative} should report the native voltage-accessor wrapper contract"
    );
}

#[test]
fn test_xyce_bsim_gm_device_operating_point_probes_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BSIM3_GM/nmosGm1.cir",
        "Netlists/BSIM3_GM/nmosGm1_rev.cir",
        "Netlists/BSIM3_GM/pmosGm1.cir",
        "Netlists/BSIM3_GM/pmosGm1_rev.cir",
        "Netlists/BSIM4_GM/nmosGm1.cir",
        "Netlists/BSIM4_GM/nmosGm1_rev.cir",
        "Netlists/BSIM4_GM/pmosGm1.cir",
        "Netlists/BSIM4_GM/pmosGm1_rev.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce device operating-point comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_bsim4_static_dc_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BSIM4/test1.X.cir",
        "Netlists/BSIM4/test2.X.cir",
        "Netlists/BSIM4/test3.X.cir",
        "Netlists/BSIM4/test4.X.cir",
        "Netlists/BSIM4/test5.X.cir",
        "Netlists/BSIM4/test6.X.cir",
        "Netlists/BSIM4/test7.X.cir",
        "Netlists/BSIM4/test8.X.cir",
        "Netlists/BSIM4/test9.X.cir",
        "Netlists/BSIM4/test10.X.cir",
        "Netlists/BSIM4/test11.X.cir",
        "Netlists/BSIM4/test12.X.cir",
        "Netlists/BSIM4/test13.X.cir",
        "Netlists/BSIM4/test14.X.cir",
        "Netlists/BSIM4_rbodymod1/test1.X.cir",
        "Netlists/BSIM4_rbodymod1/test2.X.cir",
        "Netlists/BSIM4_rbodymod1/test3.X.cir",
        "Netlists/BSIM4_rbodymod1/test4.X.cir",
        "Netlists/BSIM4_rbodymod1/test5.X.cir",
        "Netlists/BSIM4_rbodymod1/test6.X.cir",
        "Netlists/BSIM4_rbodymod1/test7.X.cir",
        "Netlists/BSIM4_rbodymod1/test8.X.cir",
        "Netlists/BSIM4_rbodymod1/test9.X.cir",
        "Netlists/BSIM4_rbodymod1/test10.X.cir",
        "Netlists/BSIM4_rbodymod1/test11.X.cir",
        "Netlists/BSIM4_rbodymod1/test12.X.cir",
        "Netlists/BSIM4_rbodymod1/test13.X.cir",
        "Netlists/BSIM4_rbodymod1/test14.X.cir",
        "Netlists/BSIM4_rbodymod2/test1.X.cir",
        "Netlists/BSIM4_rbodymod2/test2.X.cir",
        "Netlists/BSIM4_rbodymod2/test3.X.cir",
        "Netlists/BSIM4_rbodymod2/test4.X.cir",
        "Netlists/BSIM4_rbodymod2/test5.X.cir",
        "Netlists/BSIM4_rbodymod2/test6.X.cir",
        "Netlists/BSIM4_rbodymod2/test7.X.cir",
        "Netlists/BSIM4_rbodymod2/test8.X.cir",
        "Netlists/BSIM4_rbodymod2/test9.X.cir",
        "Netlists/BSIM4_rbodymod2/test10.X.cir",
        "Netlists/BSIM4_rbodymod2/test11.X.cir",
        "Netlists/BSIM4_rbodymod2/test12.X.cir",
        "Netlists/BSIM4_rbodymod2/test13.X.cir",
        "Netlists/BSIM4_rbodymod2/test14.X.cir",
        "Netlists/BSIM4_v4p7/test1.cir",
        "Netlists/BSIM4_v4p7/test2.cir",
        "Netlists/BSIM4_v4p7/test3.cir",
        "Netlists/BSIM4_v4p7/test4.cir",
        "Netlists/BSIM4_v4p7/test5.cir",
        "Netlists/BSIM4_v4p7/test6.cir",
        "Netlists/BSIM4_v4p7/test7.cir",
        "Netlists/BSIM4_v4p7/test8.cir",
        "Netlists/BSIM4_v4p7/test9.cir",
        "Netlists/BSIM4_v4p7/test10.cir",
        "Netlists/BSIM4_v4p7/test11.cir",
        "Netlists/BSIM4_v4p7/test12.cir",
        "Netlists/BSIM4_v4p7/test13.cir",
        "Netlists/BSIM4_v4p7/test14.cir",
        "Netlists/BSIM4_v4p82/test1.cir",
        "Netlists/BSIM4_v4p82/test2.cir",
        "Netlists/BSIM4_v4p82/test3.cir",
        "Netlists/BSIM4_v4p82/test4.cir",
        "Netlists/BSIM4_v4p82/test5.cir",
        "Netlists/BSIM4_v4p82/test6.cir",
        "Netlists/BSIM4_v4p82/test7.cir",
        "Netlists/BSIM4_v4p82/test8.cir",
        "Netlists/BSIM4_v4p82/test9.cir",
        "Netlists/BSIM4_v4p82/test10.cir",
        "Netlists/BSIM4_v4p82/test11.cir",
        "Netlists/BSIM4_v4p82/test12.cir",
        "Netlists/BSIM4_v4p82/test13.cir",
        "Netlists/BSIM4_v4p82/test14.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce BSIM4 static DC comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_bsimsoi3_gmin_scaling_dc_sweep_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BSIMSOI3/dcSweep.cir",
        "Netlists/BSIMSOI3/dcSweepNoGminScaling.cir",
        "Netlists/BSIMSOI3/dcSweepNoVoltLim.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce BSIMSOI3 DC sweep comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_dc",
            "{relative} should report the native static DC .prn contract"
        );
    }
}

#[test]
fn test_xyce_bsimsoi3_default_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/BSIMSOI3/b3soiTranDefaults.cir",
        "Netlists/BSIMSOI3/b3soiTranDefaultsNoGminScaling.cir",
        "Netlists/BSIMSOI3/b3soiTranDefaultsNoVoltLim.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce BSIMSOI3 transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce transient .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_lead_current_probe_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/EKV26/test_ekv26_nmos.cir",
        "Netlists/EKV26/test_ekv26_pmos.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce lead-current probe comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }

    let bsim3_legacy_version_deck =
        "Netlists/Certification_Tests/BUG_812_SON/global_accessor_interference.cir";
    let result = runner.run_test(root.join(bsim3_legacy_version_deck));
    assert!(
        result.passed && !result.expected_unsupported,
        "{bsim3_legacy_version_deck} should run as a numeric BSIM3 VERSION=3.1 comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{bsim3_legacy_version_deck} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_dc",
        "{bsim3_legacy_version_deck} should report the native static .prn DC contract"
    );
}

#[test]
fn test_xyce_empty_wildcard_lead_current_probe_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_983_SON/noThreeTerminal.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain its removed upstream wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native empty-wildcard lead-current comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_dc",
        "{relative} should report the native wrapper-origin .prn contract"
    );
}

#[test]
fn test_xyce_current_source_probe_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ISWITCH/iswitch.cir",
        "Netlists/ISWITCH/iswitch_spice.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce current-source probe comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_generic_switch_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/GSWITCH/gswitch.cir",
        "Netlists/GSWITCH/gswitchHyst1.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce generic switch transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_switch_on_off_family_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ISWITCH/iswitchHyst1.cir",
        "Netlists/VSWITCH/Vswitch_A1.cir",
        "Netlists/VSWITCH/Vswitch_B1.cir",
        "Netlists/VSWITCH/VswitchHyst1.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce ON/OFF-family switch transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_voltage_controlled_source_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/VCCS/vccs.cir", "static_prn_dc"),
        ("Netlists/VCCS/vccs_tran.cir", "static_prn_tran"),
        ("Netlists/VCVS/vcvs.cir", "static_prn_dc"),
        ("Netlists/VCVS/vcvs_trans.cir", "static_prn_tran"),
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} should be a native Xyce deck without removed wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce voltage-controlled source comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, expected_contract,
            "{relative} should report the expected native .prn contract"
        );
    }
}

#[test]
fn test_xyce_cccs_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/CCCS/ftest.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native current-controlled current-source transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_subckt_controlled_source_parameter_gain_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1020_SON/subcparam.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native subcircuit controlled-source gain-parameter comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_controlled_source_branch_current_probe_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/BSRC/Bsrc_D1.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce controlled-source branch-current comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_multiplicity_factor_resistor_wrapper_cases_run_natively() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MULTIPLICITY_FACTOR/resistor.cir",
        "Netlists/MULTIPLICITY_FACTOR/semic_resistor.cir",
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} should retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native multiplicity-factor resistor .prn comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "wrapper_static_prn_step_dc",
            "{relative} should report the native wrapper-origin stepped .prn contract"
        );
    }
}

#[test]
fn test_xyce_multiplicity_factor_inductor_step_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MULTIPLICITY_FACTOR/inductor.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native stepped inductor multiplicity transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_step_tran",
        "{relative} should report the native stepped transient .prn contract"
    );
}

#[test]
fn test_xyce_rf_port_static_dc_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Output/SPARAMS/RCladderSPdc.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce RF-port static DC comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_rf_port_transient_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Output/SPARAMS/RCladderSPtran.cir";

    assert!(
        !runner.requires_upstream_wrapper(relative),
        "{relative} should be a native Xyce deck without removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce RF-port transient comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_ekv3_150nm_static_terminal_currents_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_1353/150nm_nmos.cir",
        "Netlists/Certification_Tests/BUG_1353/150nm_pmos.cir",
    ] {
        let result = runner.run_test(root.join(relative));

        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a numeric Xyce EKV3 150 nm terminal-current comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
    }
}

#[test]
fn test_xyce_mesfet_model_parameter_step_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_647_SON/mesfet.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce MESFET model-parameter .STEP comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_complex_param_re_img_print_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/COMPLEX_NUM/test2.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce complex-parameter print comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_complex_param_dc_real_default_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/COMPLEX_NUM/test4.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a wrapper-origin Xyce complex DC .prn comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "wrapper_static_prn_dc",
        "{relative} should report the wrapper-origin static DC .prn contract"
    );
}

#[test]
fn test_xyce_behavioral_cubic_file_spline_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/cubic.cir",
        "Netlists/ABM_SPLINES/cubic2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce natural-cubic transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_behavioral_akima_file_spline_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/akima.cir",
        "Netlists/ABM_SPLINES/akima2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce Akima transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_behavioral_wodicka_file_spline_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/wodicka.cir",
        "Netlists/ABM_SPLINES/wodicka2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce Wodicka transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_behavioral_barycentric_file_interpolation_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/barycentric.cir",
        "Netlists/ABM_SPLINES/barycentric2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as a native Xyce barycentric transient comparison, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_inline_library_definition_and_selected_section_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/LIB/lib.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run with native inline .LIB filtering and inherited external section selection, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(
        result.contract, "static_prn_tran",
        "{relative} should report the native transient .prn contract"
    );
}

#[test]
fn test_xyce_stateful_sdt_transient_cases_run() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_963_SON/nonzeroInitialValue.cir",
        "Netlists/Certification_Tests/BUG_963_SON/sdtWithFunc.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run with rollback-safe stateful SDT integration, got {result:?}"
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should match the checked-in Xyce .prn oracle"
        );
        assert_eq!(
            result.contract, "static_prn_tran",
            "{relative} should report the native transient .prn contract"
        );
    }
}

#[test]
fn test_xyce_static_random_global_parameters_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/PARSER/nakedRandomGlobal.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should use one-time static statistical .GLOBAL_PARAM projections, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
    assert_eq!(result.contract, "static_prn_dc");
}

#[test]
fn test_xyce_complex_param_harmonic_balance_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/COMPLEX_NUM/test5.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a three-oracle harmonic-balance comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in HB.FD, HB.TD, and hb_ic oracles"
    );
    assert_eq!(result.contract, "wrapper_static_prn_hb");
}

#[test]
fn test_xyce_connectivity_warning_wrapper_case_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/CONNECTIVITY/connect.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "{relative} should retain removed wrapper provenance"
    );
    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a topology-diagnostic comparison, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
    assert_eq!(result.contract, "wrapper_expected_topology_warnings");
}

#[test]
fn test_xyce_subcircuit_parameter_precedence_pair_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let cases = [
        (
            "Netlists/Certification_Tests/BUGS_ISSUE_54/multiSubcktPar.cir",
            "subckt_parameter_precedence_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUGS_ISSUE_54/multiSubcktParRef.cir",
            "subckt_parameter_precedence_baseline",
        ),
    ];

    for (relative, contract) in cases {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should run as an exact sibling-reference DC comparison, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
        assert_eq!(result.contract, contract);
    }
}

#[test]
fn test_xyce_stepped_initial_condition_reference_family_runs() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let cases = [
        (
            "Netlists/Certification_Tests/BUG_1007_SON/ic_cap_step.cir",
            "stepped_ic_reference_wrapper",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_1007_SON/ic_cap0.cir",
            "stepped_ic_reference_baseline",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1007_SON/ic_cap1.cir",
            "stepped_ic_reference_baseline",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1007_SON/ic_cap2.cir",
            "stepped_ic_reference_baseline",
            false,
        ),
    ];

    for (relative, contract, requires_wrapper) in cases {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            requires_wrapper,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy independent-run parity through the Release 7.10 interpolated-RMS oracle, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
        assert_eq!(result.contract, contract);
    }
}

#[test]
fn test_xyce_unsupported_decks_are_named_results_not_omitted() {
    let _xyce_runner_guard = lock_xyce_runner();
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
            .is_some_and(|error| error.contains("B4SOI 4.7")
                && error.contains("generated Verilog-A builtin")),
        "unsupported reason should name the B4SOI generated-model capability boundary, got {result:?}"
    );

    for relative in [
        "Netlists/Certification_Tests/BUG_1775/HBT_IV.cir",
        "Netlists/VBIC13/HBT_IV.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} should stay named unsupported until native VBIC nested-sweep current-source branch probes are production-ready, got {result:?}"
        );
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("native VBIC nested DC sweep")),
            "unsupported reason should name the native VBIC nested-sweep capability boundary, got {result:?}"
        );
    }
}

#[test]
fn test_xyce_continuous_equation_measure_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE/EquationEvalTest.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its live PARAM/EQN waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_continuous_equation_measure_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE/STEP/EquationEvalTest.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset live equation state for every step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_lotka_volterra_stepped_behavioral_transient_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1145_SON/lotka_volterra.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should rebuild solution-dependent behavioral sources and UIC state for every parameter step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_file_lookup_gradient_downsampling_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_SPLINES/downsample1.cir",
        "Netlists/ABM_SPLINES/downsample2.cir",
        "Netlists/ABM_SPLINES/downsample3.cir",
        "Netlists/ABM_SPLINES/downsample4.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass canonical Xyce file-table downsampling, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_lossless_transmission_line_tiny_delay_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1370/bug1370.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass the native lossless transmission-line transient oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_lossless_transmission_line_frequency_default_length_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig {
        max_time_per_test_ms: 120_000,
        ..Default::default()
    };
    let runner = XyceTestRunner::new(&root, config);
    let relative = "Netlists/TRANSLINE/transline.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should apply Xyce's NL=0.25 default to F-specified lossless lines, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_lossless_transmission_line_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig {
        max_time_per_test_ms: 180_000,
        ..Default::default()
    };
    let runner = XyceTestRunner::new(&root, config);

    for relative in [
        "Netlists/TRANSLINE/transline_step.cir",
        "Netlists/Certification_Tests/BUG_568/bug_568.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should rebuild lossless-line parameters and delay history per step, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_mutual_inductor_device_parameter_print_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MINDUCTORS/mutIndPrint1.cir",
        "Netlists/MINDUCTORS/mutIndPrint2.cir",
        "Netlists/MINDUCTORS/mutIndStep.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should route device-parameter expressions through the generic transient evaluator, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_dc_continuous_equation_measure_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/EquationEvalTestDC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass ordered live DC equation-measure traces, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_continuous_equation_measure_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/EquationEvalTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass ordered live AC equation-measure traces, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_single_point_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/oneSweepValue.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its ordered scalar .ma0 measurement artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_derivative_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/DerivTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reject constant-signal WHEN events while preserving AC derivative artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_single_point_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/oneSweepValue.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its ordered scalar .ms0 measurement artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/AvgTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain its validated inactive .FFT and pass the DC average artifact oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_nested_dc_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/AvgTest2DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain AVG state across its nested DC sweep artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/MaxMinPPTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its DC MIN/MAX/PP measurement artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_nested_dc_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/MaxMinPPTest2DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should filter extrema windows across every nested DC sweep cycle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/IntegTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve DC INTEG direction in its measurement artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/RMSTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its ordered DC RMS measurement artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_lead_current_power_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/LeadCurrentAndPowerTestDC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass resistor lead-current, power, and internal branch measurement artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_derivative_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/DerivTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass direction-aware derivative event artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_derivative_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/DerivTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass descending stepped derivative event artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/FindWhenTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass FIND/WHEN and live equation measurement artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/FindWhenTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain exact FIND/WHEN artifacts across expanded parameter steps, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_nested_dc_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/FindWhenTest2DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should skip synthetic interpolation intervals at nested DC sweep restarts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/TrigTargTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass typed AC trigger/target event artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/TrigTargTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain AC trigger/target artifacts for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/ErrorTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should interpolate AC waveforms onto PRN, CSV, and comma-PRN reference frequencies, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/ErrorTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset AC file-error interpolation for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/TrigTargTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass typed AT/WHEN, moving-target, occurrence, and TD artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_nested_dc_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/TrigTargTest2DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve occurrence counts while reseeding event history at nested DC restarts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/TrigTargTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain descending trigger/target event artifacts for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/ErrorFuncTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass ERR/ERR1/ERR2 norms, filters, expressions, currents, and windows, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/ErrorTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve exact PRN, CSV, comma-PRN, and CSDF file-error norms, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_op_only_dc_measurements_fail_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/DotOpOnly.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should report every DC measurement as FAILED when no .DC analysis is declared, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_measfail_zero_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/MeasfailOptionZero.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should emit numeric zero while preserving failed measurement status, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_nested_dc_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/ErrorTest2DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare the flattened nested DC accepted-point stream positionally, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/ErrorTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset file-error samples and retain every expanded step artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_op_plus_dc_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/DotOpDotDC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve DC file-error artifacts when the deck also requests an operating point, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_op_only_dc_measurement_failure_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/DotOpOnly.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain Xyce's failed DC measurement artifacts when only an operating-point analysis is requested, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_op_only_dc_measfail_zero_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/MeasfailOptionZero.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should serialize the effective Xyce default instead of FAILED when MEASFAIL is disabled, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_data_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/DotData.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate DC DATA measurements on Xyce's one-based table-row axis, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_data_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/DotData.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate and reset the one-based DC DATA row axis for every outer step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/AvgTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE averages over canonical complex small-signal projections, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/AvgTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE averages for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/ErrorFuncTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE ERR-family norms over complex projections and expressions, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/ErrorFuncTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE ERR-family norms for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/ErrorTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE FILE error measurements with canonical interpolation and extrapolation, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_file_error_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/ErrorTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve and evaluate NOISE FILE error inputs for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/TrigTargTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE trigger/target events, occurrences, delays, windows, and expressions, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_trigger_target_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/TrigTargTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE trigger/target events for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_continuous_measurement_sidecar_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/USE_CONT_FILES/Noise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare every scalar NOISE result and typed NOISE_CONT sidecar record, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_continuous_measurement_sidecar_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/USE_CONT_FILES/AC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare every scalar AC result and typed AC_CONT sidecar record, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_continuous_measurement_sidecar_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/USE_CONT_FILES/DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare every scalar DC result and typed DC_CONT sidecar record, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/TrigTargTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare declaration-ordered AC_CONT trigger/target records and metadata in the aggregate artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/TrigTargTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare declaration-ordered DC_CONT trigger/target records and metadata in the aggregate artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/TrigTargTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset AC_CONT trigger/target state and compare every stepped aggregate artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/TrigTargTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset DC_CONT trigger/target state and compare every stepped aggregate artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_continuous_find_when_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/FindWhenTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset AC_CONT FIND/WHEN state and compare every stepped mixed artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_continuous_derivative_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/DerivTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset AC_CONT derivative state and compare every stepped mixed artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_continuous_find_when_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/FindWhenTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset DC_CONT FIND/WHEN state and compare every stepped mixed artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_continuous_derivative_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/DerivTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset DC_CONT derivative state and compare every stepped mixed artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_continuous_find_when_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/FindWhenTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare declaration-ordered scalar and NOISE_CONT FIND/WHEN records in the mixed artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/TrigTargTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare declaration-ordered scalar and NOISE_CONT trigger/target records with metadata, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_dc_continuous_mixed_artifact_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relatives = [
        "Netlists/MEASURE_CONT/STEP/DerivTest1DC.cir",
        "Netlists/MEASURE_CONT/STEP/DerivTestAC.cir",
        "Netlists/MEASURE_CONT/STEP/FindWhenTest1DC.cir",
        "Netlists/MEASURE_CONT/STEP/FindWhenTestAC.cir",
        "Netlists/MEASURE_CONT/STEP/TrigTargTest1DC.cir",
        "Netlists/MEASURE_CONT/STEP/TrigTargTestAC.cir",
    ];

    for relative in relatives {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should compare every stepped scalar and continuous measurement row in declaration order, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_noise_continuous_derivative_mixed_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/DerivTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should compare mixed scalar/NOISE_CONT derivative records and live occurrence-state waveform traces, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_continuous_find_when_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/FindWhenTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and compare mixed scalar and NOISE_CONT FIND/WHEN records for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_continuous_trigger_target_mixed_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_CONT/STEP/TrigTargTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and compare mixed scalar and NOISE_CONT trigger/target metadata for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_cascode_bjt_noise_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/NOISE/cascodeBjtNOISE.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should match the complete native Xyce cascode BJT NOISE waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bjt_differential_pair_linear_ac_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/ACtests/diffpair_spiceManLIN.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should match the complete native Xyce differential-pair linear AC waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_level2_mos_gain_stage_ac_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/ACtests/mos/gain-stage2.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should match the complete native Xyce LEVEL=2 MOS gain-stage AC waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_level6_mos_gain_stage_ac_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/ACtests/mos/gain-stage6.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should match the complete native Xyce LEVEL=6 MOS gain-stage AC waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_dc_measfail_one_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/MeasfailOptionOne.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve Xyce MEASFAIL=1 failed-measurement artifact semantics, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/IntegTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should integrate NOISE projections, expressions, windows, and branch currents, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/IntegTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and integrate NOISE projections and expressions for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/MaxMinPPTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE extrema, peak-to-peak values, windows, expressions, and branch currents, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/MaxMinPPTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE extrema and peak-to-peak values for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_single_point_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/oneSweepValue.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve Xyce's exact single-point NOISE measurement success and failure semantics, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/RMSTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE RMS integrals, windows, expressions, branch currents, and one-point failures, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/RMSTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE RMS integrals and failures for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_equation_measurement_and_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/EquationEvalTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should match Xyce NOISE equations, complex projections, and power spectral density waveforms, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_noise_equation_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/EquationEvalTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE continuous equations for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_derivative_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/DerivTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate NOISE derivatives, interpolated events, windows, expressions, and branch currents, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_noise_complex_components_and_measure_consumers() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/LEAD_CURRENTS/lead_ind_noise.cir",
        "Netlists/MEASURE_NOISE/FindWhenTestNoise.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy the native NOISE component/measurement oracle, got {result:?}"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_stepped_noise_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_NOISE/STEP/FindWhenTestNoise.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate NOISE FIND/WHEN events and equation consumers for every materialized step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_unimplemented_noise_surfaces_fail_closed() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, reason) in [
        (
            "Netlists/MEASURE_CONT/DerivTestNoise.cir",
            "sidecar artifact",
        ),
        (
            "Netlists/MEASURE_NOISE/STEP/DerivTestNoise.cir",
            ".STEP NOISE DERIV",
        ),
        ("Netlists/NOISE/commonEmitterBjt.cir", "DNO/DNI"),
        ("Netlists/VANOISE/commonEmitterBjt_vbic13.cir", "DNO/DNI"),
        ("Netlists/VANOISE/ekv_150nm_nmos_noise.cir", "DNO/DNI"),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} must remain a named unsupported contract, got {result:?}"
        );
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains(reason)),
            "{relative} should name the unsupported boundary '{reason}', got {result:?}"
        );
    }
}

#[test]
fn test_xyce_ac_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/ErrorFuncTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate AC ERR-family norms over complex projections and expressions, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/FindWhenTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate AC FIND-AT, FIND-WHEN, WHEN, equation references, expressions, and windows, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_derivative_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/DerivTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate and reset AC derivative measurements for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_find_when_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/FindWhenTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and evaluate AC FIND/WHEN measurements and equation references for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/ErrorFuncTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and retain AC ERR-family accumulators for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_error_function_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/ErrorFuncTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should reset and retain exact ERR-family accumulators for every expanded step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/AvgTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pair each expanded DC step with its ordered average artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/RMSTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pair descending-sweep RMS results with each step artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/IntegTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve directional INTEG results in each step artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/MaxMinPPTest1DC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should filter descending extrema windows in every step artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_dc_equation_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_DC/STEP/EquationEvalTestDC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should orient continuous equation windows in each descending DC step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_equation_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/EquationEvalTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pair each expanded AC step with its ordered .maN artifact, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_expression_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/AvgTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should evaluate braced signal expressions pointwise in every AC step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/MaxMinPPTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass stepped extrema and peak-to-peak artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/RMSTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass stepped RMS expression artifacts, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_stepped_ac_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/STEP/IntegTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass stepped integration artifacts with their typed tolerance, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_rms_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/RMSTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its AC RMS measurement artifact oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_integration_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/IntegTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its AC integration measurement artifact oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_extrema_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/MaxMinPPTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should pass its AC extrema measurement artifact oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_ac_average_measurement_artifact_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/MEASURE_AC/AvgTestAC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should retain its validated inactive .FFT and pass the AC average artifact oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_initial_condition_inverter_waveform_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/INIT_CONDS/inv1xIC.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should preserve its explicit initial condition and match the transient waveform oracle, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

// The aggregate intentionally replays every retained deck and therefore has
// release-profile runtime requirements. Individual supported contracts remain
// in the normal test tier above, while nightly release CI runs this full census.
#[cfg_attr(
    debug_assertions,
    ignore = "release-only full Xyce corpus; run with `cargo test --release -p rspice-core --test xyce_regression test_full_xyce_suite_summary_accounts_for_every_deck`"
)]
#[test]
fn test_full_xyce_suite_summary_accounts_for_every_deck() {
    let _xyce_runner_guard = lock_xyce_runner();
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
