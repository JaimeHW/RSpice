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
        "Netlists/PARAM_REFACTOR/paramDep2.cir",
        "Netlists/PARAM_REFACTOR/paramDep3.cir",
        "Netlists/PARAM_REFACTOR/paramDep4.cir",
        "Netlists/PARAM_REFACTOR/paramStep1.cir",
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
    let relative = "Netlists/RESISTOR_TD/temp_dep.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a native Xyce resistor value-plus-model temperature comparison, got {result:?}"
    );
    assert!(
        result.mismatches.is_empty(),
        "{relative} should match the checked-in Xyce .prn oracle"
    );
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

    for relative in ["Netlists/Output/DC/dc-raw-override.cir"] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} should stay named unsupported until its removed wrapper's full output contract is implemented, got {result:?}"
        );
    }
}

#[test]
fn test_xyce_output_dc_gnuplot_splot_wrapper_case_runs_natively() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Output/DC/dc-gnuplot.cir";

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
        result.contract, "wrapper_gnuplot_splot_prn_dc",
        "{relative} should report the native GNUPLOT/SPLOT wrapper contract"
    );
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

    let unsupported_device_deck =
        "Netlists/Certification_Tests/BUG_812_SON/global_accessor_interference.cir";
    let result = runner.run_test(root.join(unsupported_device_deck));
    assert!(
        result.passed && result.expected_unsupported,
        "{unsupported_device_deck} should remain named unsupported until BSIM3 VERSION=3.1 is fully implemented, got {result:?}"
    );
    assert!(
        result
            .error
            .as_deref()
            .is_some_and(|error| error.contains("BSIM3 pre-3.3 LEVEL=49 VERSION=3.1")),
        "unsupported reason should name the BSIM3 3.1 capability boundary, got {result:?}"
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
fn test_xyce_ekv3_nmos150_static_terminal_currents_run() {
    let _xyce_runner_guard = xyce_runner_lock().lock().expect("Xyce runner mutex");
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1353/150nm_nmos.cir";

    let result = runner.run_test(root.join(relative));

    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should run as a numeric Xyce EKV3 NMOS150 terminal-current comparison, got {result:?}"
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
            .is_some_and(|error| error.contains("netlist parser")),
        "unsupported reason should name the parser capability boundary, got {result:?}"
    );

    let relative = "Netlists/Certification_Tests/BUG_1353/150nm_pmos.cir";
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && result.expected_unsupported,
        "{relative} should stay named unsupported until EKV3 LEVEL=301 PMOS terminal currents are implemented, got {result:?}"
    );
    assert!(
        result
            .error
            .as_deref()
            .is_some_and(|error| error.contains("EKV3 LEVEL=301")),
        "unsupported reason should name the EKV3 capability boundary, got {result:?}"
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
