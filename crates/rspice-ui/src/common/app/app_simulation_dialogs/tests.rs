use std::collections::HashSet;

use super::defaults::{set_default_if_blank, toggle_enabled_analysis};
use super::options_dialog::{
    apply_options_preset, commit_validated_options, parse_and_validate_options,
    revert_options_dialog_state, SimulationOptionsPreset,
};
use super::*;

fn make_test_app() -> RSpiceApp {
    RSpiceApp::new_for_tests(super::super::AppState::default())
}

fn assert_approx_eq(actual: f64, expected: f64) {
    let tolerance = expected.abs().max(1.0) * 1e-9;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {}, got {} (tol {})",
        expected,
        actual,
        tolerance
    );
}

#[test]
fn test_set_default_if_blank_updates_only_blank_values() {
    let mut value = "   ".to_string();
    set_default_if_blank(&mut value, "default");
    assert_eq!(value, "default");

    let mut existing = "keep".to_string();
    set_default_if_blank(&mut existing, "default");
    assert_eq!(existing, "keep");
}

#[test]
fn test_toggle_enabled_analysis_toggles_membership() {
    let mut enabled = HashSet::new();
    toggle_enabled_analysis(&mut enabled, 7);
    assert!(enabled.contains(&7));
    toggle_enabled_analysis(&mut enabled, 7);
    assert!(!enabled.contains(&7));
}

#[test]
fn test_simulation_analysis_categories_have_unique_indices() {
    let mut seen = HashSet::new();
    for (category_name, analyses) in SIMULATION_ANALYSIS_CATEGORIES {
        assert!(!category_name.trim().is_empty());
        assert!(!analyses.is_empty());
        for &(index, name) in *analyses {
            assert!(!name.trim().is_empty());
            assert!(
                seen.insert(index),
                "duplicate analysis index found in categories: {}",
                index
            );
        }
    }
}

#[test]
fn test_ensure_simulation_setup_defaults_fills_each_field_independently() {
    let mut app = make_test_app();
    app.state.dialogs.tran_stop = "5m".to_string();
    app.state.dialogs.ac_points = "501".to_string();
    app.state.dialogs.tran_step.clear();
    app.state.dialogs.noise_input = "   ".to_string();
    app.state.dialogs.temp_start.clear();

    app.ensure_simulation_setup_defaults();

    assert_eq!(app.state.dialogs.tran_stop, "5m");
    assert_eq!(app.state.dialogs.ac_points, "501");
    assert_eq!(app.state.dialogs.tran_step, "10n");
    assert_eq!(app.state.dialogs.noise_input, "V1");
    assert_eq!(app.state.dialogs.temp_start, "-40");
}

#[test]
fn test_apply_options_preset_updates_dialog_state_from_preset() {
    let mut dialogs = super::super::DialogState::default();
    dialogs
        .simulation_options_errors
        .push("stale parse error".to_string());

    apply_options_preset(&mut dialogs, SimulationOptionsPreset::Fast);

    let parsed = dialogs.simulation_options_state.to_options().unwrap();
    assert_approx_eq(parsed.reltol, 1e-2);
    assert_approx_eq(parsed.abstol, 1e-9);
    assert_eq!(parsed.gmin_stepping, false);
    assert_eq!(parsed.source_stepping, false);
    assert_eq!(parsed.bypass_enabled, true);
    assert!(dialogs.simulation_options_errors.is_empty());
}

#[test]
fn test_revert_options_dialog_state_restores_current_config() {
    let mut dialogs = super::super::DialogState::default();
    dialogs.simulation_options_config = crate::simulation::dialog::SimulationOptions::accurate();
    apply_options_preset(&mut dialogs, SimulationOptionsPreset::Fast);

    revert_options_dialog_state(&mut dialogs);

    let reverted = dialogs.simulation_options_state.to_options().unwrap();
    assert_approx_eq(reverted.reltol, dialogs.simulation_options_config.reltol);
    assert_approx_eq(reverted.vntol, dialogs.simulation_options_config.vntol);
    assert_approx_eq(reverted.abstol, dialogs.simulation_options_config.abstol);
}

#[test]
fn test_parse_and_validate_options_reports_parse_errors() {
    let mut dialogs = super::super::DialogState::default();
    dialogs.simulation_options_state.reltol = "not-a-number".to_string();

    let parsed = parse_and_validate_options(&mut dialogs);

    assert!(parsed.is_none());
    assert!(dialogs
        .simulation_options_errors
        .iter()
        .any(|error| error.contains("reltol")));
}

#[test]
fn test_parse_and_validate_options_reports_validation_errors() {
    let mut dialogs = super::super::DialogState::default();
    dialogs.simulation_options_state.reltol = "-1".to_string();

    let parsed = parse_and_validate_options(&mut dialogs);

    assert!(parsed.is_none());
    assert!(dialogs
        .simulation_options_errors
        .iter()
        .any(|error| error.contains("reltol must be positive")));
}

#[test]
fn test_commit_validated_options_updates_state_and_clears_errors() {
    let mut dialogs = super::super::DialogState::default();
    dialogs
        .simulation_options_errors
        .push("previous validation issue".to_string());
    let options = crate::simulation::dialog::SimulationOptions::robust();

    commit_validated_options(&mut dialogs, &options);

    assert!(dialogs.simulation_options_errors.is_empty());
    assert_approx_eq(dialogs.simulation_options_config.reltol, options.reltol);
    assert_approx_eq(dialogs.simulation_options_config.gmin, options.gmin);
    assert_eq!(
        dialogs.simulation_options_config.gmin_stepping,
        options.gmin_stepping
    );

    let ui_state_roundtrip = dialogs.simulation_options_state.to_options().unwrap();
    assert_approx_eq(ui_state_roundtrip.reltol, options.reltol);
    assert_approx_eq(ui_state_roundtrip.gmin, options.gmin);
    assert_eq!(ui_state_roundtrip.gmin_stepping, options.gmin_stepping);
}
