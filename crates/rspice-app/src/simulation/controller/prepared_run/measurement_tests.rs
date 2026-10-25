//! Measurement references must survive preparation, dispatch, retention, and project saves.

use super::fixtures::definition;
use crate::product::AnalysisInstanceId;
use crate::simulation::config::AnalysisConfig;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use rspice_simulation::netlist_preparation::measurements::materialize;
use std::collections::HashMap;

#[test]
fn authored_plan_measurements_validate_context_ownership_and_duplicate_names() {
    let base = AnalysisInstanceId::new();
    let study = AnalysisInstanceId::new();
    let ac = AnalysisDraft::for_kind(AnalysisKind::Ac);
    let mut mc = rspice_simulation_contract::mc_draft::McDialogState::default();
    mc.base_analysis = Some(base);
    let mc = AnalysisDraft::MonteCarlo(mc);
    let drafts = HashMap::from([(base, &ac), (study, &mc)]);
    let source = "Measurement context\n.param freq=1000\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1u\n.end\n";
    let mut measurement = definition("gain", ".MEAS AC gain FIND VM(out) AT={freq}");
    measurement.producing_analysis = Some(study);
    let composed = materialize(source, &[measurement.clone()], &drafts).unwrap();
    let config = AnalysisConfig::Ac(crate::simulation::config::AcAnalysisConfig {
        start_freq: 1000.0,
        stop_freq: 1000.0,
        num_points: 1,
        sweep_type: rspice_simulation_contract::config::FrequencySweep::Linear,
    });
    let executable = rspice_simulation::netlist_preparation::splice_before_terminal_end_card(
        &composed,
        &config.to_spice(),
    );
    let result = crate::simulation::controller::test_execution::run_manual_deck(&executable);
    let expected = 1.0 / 1.0_f64.hypot(std::f64::consts::TAU);
    assert!((result.measurement("gain").unwrap() - expected).abs() < 1e-9);
    let duplicate = composed.replace(".MEAS AC gain", ".MEAS AC GAIN");
    assert!(
        materialize(&duplicate, &[measurement.clone()], &drafts)
            .unwrap_err()
            .to_string()
            .contains("already defined")
    );
    for expression in [
        ".MEAS AC wrong MAX VM(out)",
        ".MEAS TRAN gain AVG V(out)",
        ".MEAS AC gain NOT_A_MEASURE V(out)",
        ".MEAS AC gain FIND VM(out) AT={missing}",
        ".MEAS AC gain AVG V(out)\n.end",
    ] {
        measurement.expression = expression.into();
        assert!(
            materialize(source, &[measurement.clone()], &drafts).is_err(),
            "{expression}"
        );
    }
    measurement.expression = ".MEAS AC gain FIND VM(out) AT={freq}".into();
    measurement.producing_analysis = Some(AnalysisInstanceId::new());
    assert!(materialize(source, &[measurement], &drafts).is_err());
}
