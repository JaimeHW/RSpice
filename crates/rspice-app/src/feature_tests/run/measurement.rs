//! Measurement references that survive a project save.

use std::collections::HashMap;

use super::session_of;
use crate::simulation::config::AnalysisConfig;
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::prepared_run::measurement_tests::definition;
use crate::simulation::controller::prepared_run::tests::runnable_state;
use crate::simulation::controller::test_execution::run_resolved_task;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{SimulationRunIntent, SpecificationDefinition};

#[test]
fn studio_measurement_reference_survives_project_preparation_and_dispatch() {
    use crate::simulation::execution::PreparedRunAuthorization;
    use crate::state::workspace::MeasurementReferenceSource;
    let mut state = session_of(runnable_state());
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
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
    let mut spec = definition(
        "fit",
        ".MEAS TRAN fit ERROR V(out) FILE=studio-reference.csv COMP_FUNCTION=INFNORM INDEPVARCOL=0 DEPVARCOL=1",
    );
    spec.producing_analysis = Some(transient);
    spec.measurement_reference = Some(MeasurementReferenceSource {
        logical_path: "studio-reference.csv".into(),
        contents: "TIME,V(out)\n0,2.5\n0.000001,2.5\n".into(),
    });
    state
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, vec![spec.clone()]);
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .unwrap();
    let digest = snapshot.digest();
    let generation =
        crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(&state)
            .unwrap();
    let source = snapshot.executable_netlist().to_owned();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let loaded = crate::io::project_io::load_project_text(
        &crate::io::project_io::serialize_project_file(&project).unwrap(),
        None,
    )
    .unwrap();
    assert_eq!(
        loaded
            .file
            .workspace
            .plan_data(plan_id)
            .unwrap()
            .specification_definitions,
        [spec.clone()]
    );

    // Reference content is execution identity even when the netlist is unchanged.
    spec.measurement_reference.as_mut().unwrap().contents = "TIME,V(out)\n0,0\n0.000001,0\n".into();
    state
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, vec![spec.clone()]);
    let changed =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .unwrap();
    assert_eq!(changed.executable_netlist(), source);
    assert_ne!(changed.digest(), digest);
    assert_ne!(
        crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(&state)
            .unwrap(),
        generation
    );

    // The original snapshot still executes its captured table after editing.
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap();
    let mut measured = false;
    for task in dispatch.into_tasks() {
        if matches!(task.config(), Some(AnalysisConfig::Transient(_))) {
            let result =
                run_resolved_task(task.resolve_dependency_artifacts(&HashMap::new()).unwrap())
                    .unwrap();
            assert!(result.measurement("fit").unwrap().abs() < 1e-8);
            measured = true;
        }
    }
    assert!(measured);

    // Manual decks own their cards but may use a plan's retained reference.
    spec.define_measurement = false;
    spec.expression = "Manual comparison".into();
    state
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, vec![spec.clone()]);
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some("Manual reference\nV1 out 0 2.5\nR1 out 0 1k\n.tran 100n 1u\n.MEAS TRAN fit ERROR V(out) FILE=studio-reference.csv COMP_FUNCTION=INFNORM INDEPVARCOL=0 DEPVARCOL=1\n.end\n".into());
    let manual =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .unwrap();
    assert_eq!(manual.executable_netlist().matches(".MEAS").count(), 1);
    for reference in [
        None,
        Some(MeasurementReferenceSource {
            logical_path: "wrong.csv".into(),
            contents: "TIME,V(out)\n0,0\n1,0\n".into(),
        }),
    ] {
        spec.measurement_reference = reference;
        state
            .model
            .workspace
            .content
            .replace_active_specification_definitions(plan_id, vec![spec.clone()]);
        assert!(
            SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
                .is_err()
        );
    }
}

#[test]
fn authored_plan_measurements_reach_sealed_source_results_and_saved_projects() {
    let mut state = session_of(runnable_state());
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
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
    let baseline =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, definitions.clone());
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .unwrap();
    let generated =
        crate::workbench::menu_bar::build_menu_netlist(&state, crate::io::NetlistFormat::Spice)
            .unwrap()
            .source;
    let inspection =
        SimulationController::prepare_design_netlist_for_inspection(&state.model).unwrap();
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
    let snapshot_digest = snapshot.digest();
    let dispatch = crate::simulation::execution::PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap();
    let task = dispatch
        .into_tasks()
        .into_iter()
        .find(|task| matches!(task.config(), Some(AnalysisConfig::Transient(_))))
        .expect("prepared transient task");
    let result =
        run_resolved_task(task.resolve_dependency_artifacts(&HashMap::new()).unwrap()).unwrap();
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
            .file
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
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, legacy);
    let reference =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .unwrap();
    assert_eq!(
        baseline.metadata().source_digest,
        reference.metadata().source_digest
    );
    assert_ne!(snapshot_digest, reference.digest());
}
