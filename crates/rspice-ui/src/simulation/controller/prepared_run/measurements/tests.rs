use super::*;
use crate::simulation::config::{AnalysisConfig, TransientAnalysisConfig};
use crate::simulation::controller::SimulationController;
use crate::simulation::engine_bridge::EngineBridge;
use crate::simulation::plan::AnalysisKind;
use crate::state::{SimulationRunIntent, SpecEntry, SpecPointScope};

fn definition(name: &str, card: &str) -> SpecificationDefinition {
    let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: name.into(),
        expression: card.into(),
        min: None,
        max: None,
        unit: "V".into(),
        scope: SpecPointScope::AllPoints,
    });
    definition.define_measurement = true;
    definition
}

#[test]
fn authored_plan_measurements_reach_sealed_source_results_and_saved_projects() {
    let mut state = super::super::tests::runnable_state();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let transient = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .unwrap()
        .id();
    plan.edit(transient, |draft| {
        let AnalysisDraft::Transient(config) = draft else {
            unreachable!()
        };
        config.stop = "1u".into();
        config.step = "100n".into();
        config.max_step = "100n".into();
    })
    .unwrap();
    let plan_id = plan.id();
    let controller = SimulationController::new();
    let baseline = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .unwrap();
    let baseline_generation =
        crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(&state)
            .unwrap();
    let mut definitions = vec![
        definition(
            "average",
            ".MEAS TRAN average AVG V(out) FROM=200n TO=800n GOAL=100 TOL=1m",
        ),
        definition("rms", ".MEAS TRAN rms RMS V(out) FROM=200n TO=800n"),
        definition("area", ".MEAS TRAN area INTEG V(out) FROM=0 TO=1u"),
        definition("at", ".MEAS TRAN at FIND V(out) AT=500n"),
        definition("twice", ".MEAS TRAN twice PARAM='rms*2'"),
    ];
    definitions[2].unit = "V*s".into();
    for definition in &mut definitions {
        definition.producing_analysis = Some(transient);
    }
    state
        .workspace
        .replace_active_specification_definitions(plan_id, definitions.clone());
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .unwrap();
    let generated =
        crate::workbench::menu_bar::build_menu_netlist(&mut state, crate::io::NetlistFormat::Spice)
            .unwrap();
    let inspection = SimulationController::prepare_design_netlist_for_inspection(&state).unwrap();
    assert_ne!(
        baseline_generation,
        crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(&state)
            .unwrap()
    );
    for definition in &definitions {
        assert!(
            snapshot
                .executable_netlist()
                .contains(&definition.expression)
        );
        assert!(generated.contains(&definition.expression));
        assert!(!inspection.contains(&definition.expression));
    }
    assert_ne!(
        baseline.metadata().source_digest,
        snapshot.metadata().source_digest
    );
    let result = EngineBridge::new()
        .run_with_abort(
            &AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 1e-6,
                step_time: 1e-7,
                max_timestep: Some(1e-7),
                ..Default::default()
            }),
            snapshot.executable_netlist(),
            &rspice_core::NoAbort,
        )
        .unwrap();
    for (name, expected) in [
        ("average", 2.5),
        ("rms", 2.5),
        ("area", 2.5e-6),
        ("at", 2.5),
        ("twice", 5.0),
    ] {
        let observed = result.study_measurement(name).unwrap();
        assert!(
            (observed.value.unwrap() - expected).abs() < expected.abs() * 1e-8,
            "{name}: {observed:?}"
        );
        assert_eq!(observed.passed, name != "average");
    }
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let json = crate::io::project_io::serialize_project_file(&project).unwrap();
    let loaded = crate::io::project_io::load_project_text(&json, None).unwrap();
    assert_eq!(
        loaded
            .workspace
            .plan_data(plan_id)
            .unwrap()
            .specification_definitions,
        definitions
    );

    // The same historical text remains descriptive unless explicitly enabled.
    for definition in &mut definitions {
        definition.define_measurement = false;
    }
    let serialized = serde_json::to_string(&definitions).unwrap();
    assert!(!serialized.contains("define_measurement"));
    let legacy: Vec<SpecificationDefinition> = serde_json::from_str(&serialized).unwrap();
    assert!(
        legacy
            .iter()
            .all(|definition| definition.measurement_statement().unwrap().is_none())
    );
    state
        .workspace
        .replace_active_specification_definitions(plan_id, legacy);
    let reference = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .unwrap();
    assert_eq!(
        baseline.metadata().source_digest,
        reference.metadata().source_digest
    );
    assert_ne!(snapshot.digest(), reference.digest());
}

#[test]
fn authored_plan_measurements_validate_context_ownership_and_duplicate_names() {
    let base = AnalysisInstanceId::new();
    let study = AnalysisInstanceId::new();
    let ac = AnalysisDraft::for_kind(AnalysisKind::Ac);
    let mut mc = crate::simulation::dialog::McDialogState::default();
    mc.base_analysis = Some(base);
    let mc = AnalysisDraft::MonteCarlo(mc);
    let drafts = HashMap::from([(base, &ac), (study, &mc)]);
    let source = "Measurement context\n.param freq=1000\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1u\n.end\n";
    let mut measurement = definition("gain", ".MEAS AC gain FIND VM(out) AT={freq}");
    measurement.producing_analysis = Some(study);
    let composed = materialize(source, &[measurement.clone()], &drafts).unwrap();
    let result = EngineBridge::new()
        .run_with_abort(
            &AnalysisConfig::Ac(crate::simulation::config::AcAnalysisConfig {
                start_freq: 1000.0,
                stop_freq: 1000.0,
                num_points: 1,
                sweep_type: crate::simulation::config::AcSweepType::Linear,
                ..Default::default()
            }),
            &composed,
            &rspice_core::NoAbort,
        )
        .unwrap();
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
