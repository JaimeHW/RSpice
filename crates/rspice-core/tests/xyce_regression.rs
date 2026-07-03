//! Xyce Regression Corpus Tests
//!
//! The Xyce corpus is vendored under `tests/xyce`. These tests run the
//! Rust-native RSpice adapter, not the upstream Perl/Bash harness. Upstream
//! platform scripts are intentionally trimmed from this corpus.

use rspice_core::testing::{XyceDeckSection, XyceRunnerConfig, XyceTestRunner};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

fn xyce_runner_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
        "Netlists/EXPPRINT/expprint.cir",
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
        "Netlists/RESISTOR_TD/temp_dep_2.cir",
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
fn test_xyce_static_ac_fd_prn_wrapper_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/ABM_DB/dbTest.cir",
        "Netlists/ACtests/RC_simple.cir",
        "Netlists/ACtests/lowpass_old.cir",
        "Netlists/ACtests/reg0.cir",
        "Netlists/LEAD_CURRENTS/lead_min_ac.cir",
        "Netlists/LEAD_CURRENTS/lead_bsrc_ac.cir",
        "Netlists/COMPLEX_NUM/test1.cir",
        "Netlists/COMPLEX_NUM/test3.cir",
        "Netlists/Certification_Tests/BUG_401_SON/bug_401.cir",
        "Netlists/Certification_Tests/BUG_407_SON/bug_407_ac.cir",
        "Netlists/Output/AC/ac-phase-in-radians.cir",
        "Netlists/Output/AC/ac-prn.cir",
        "Netlists/Output/AC/ac-prn-diff.cir",
        "Netlists/Output/AC/op-prn.cir",
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Output/AC/op-csv.cir";
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

#[test]
fn test_xyce_transient_delimiter_option_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_bug_229_native_transient_case_runs() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_generic_wrapper_transient_guardrails_stay_unsupported() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_61/capacitor.cir",
        "Netlists/Output/TRAN/tran-prn.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} should stay named unsupported until its wrapper-origin transient contract is implemented, got {result:?}"
        );
        assert_eq!(
            result.contract, "unsupported_xyce_contract",
            "{relative} should not be promoted by the generic transient wrapper contract"
        );
    }
}

#[test]
fn test_xyce_zero_resistance_branch_current_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_vpwl_delay_repeat_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
}

#[test]
fn test_xyce_pat_pattern_source_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/VSFFM/vsffm.cir",
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
fn test_xyce_current_source_waveform_transient_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Output/TRAN/tran-prn-noindex.cir";
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

#[test]
fn test_xyce_output_initial_interval_transient_wrapper_case_runs_natively() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let relative = "Netlists/Certification_Tests/BUG_256/bug_256.cir";
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

#[test]
fn test_xyce_hierarchical_passive_transient_case_runs_natively() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_dc_upgrade_sweep_modes_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_step_static_dc_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_606_SON/resistor.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params_step.cir",
        "Netlists/Certification_Tests/BUG_1203_SON/dot_step.cir",
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
fn test_xyce_param_refactor_dependency_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_certification_static_dc_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_21_SON/func_loop.cir",
        "Netlists/Certification_Tests/BUG_21_SON/func_param.cir",
        "Netlists/Certification_Tests/BUG_250/bug_250.cir",
        "Netlists/Certification_Tests/BUG_264_SON/bug_264.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params.cir",
        "Netlists/Certification_Tests/BUG_606_SON/global_params_dev_options.cir",
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_transient_capacitor_branch_current_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_behavioral_file_table_transient_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_param_type_meter_suffix_case_runs() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/ISSUE_206/issue206.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce modeled-resistor value-suffix comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_special_character_function_name_case_runs() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_naked_random_parameter_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/SUBCKT/subckt_b.cir", "subckt_family_wrapper"),
        ("Netlists/SUBCKT/subckt_b0.cir", "subckt_family_baseline"),
        ("Netlists/SUBCKT/subckt_b1.cir", "subckt_family_wrapper"),
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
fn test_xyce_supernode_wrapper_family_members_run_natively() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_plain_static_dc_wrapper_cases_run_natively() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/XDM/HSPICE/CONTROLLED_SOURCES/voltage_controlled_sources_w_extraneous_identifiers.cir",
        "Netlists/XDM/HSPICE/CONTROLLED_SOURCES/current_controlled_sources_w_extraneous_identifiers.cir",
        "Netlists/XDM/HSPICE/OTHER_PARSING/mixed_param_and_func.cir",
        "Netlists/XDM/HSPICE/MODELS/correct_instance_parameters_translation_inside_subckt.cir",
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
fn test_xyce_plain_static_dc_wrapper_guardrails_stay_unsupported() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/XDM/HSPICE/OTHER_PARSING/ternary_operator.cir",
        "Netlists/XDM/HSPICE/OTHER_PARSING/library_parsing.cir",
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_bsimsoi3_gmin_scaling_dc_sweep_runs() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/BSIMSOI3/dcSweepNoGminScaling.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce BSIMSOI3 GMIN comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
}

#[test]
fn test_xyce_bsimsoi3_default_transient_cases_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_ekv3_150nm_static_terminal_currents_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_xyce_unsupported_decks_are_named_results_not_omitted() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
fn test_full_xyce_suite_summary_accounts_for_every_deck() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
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
