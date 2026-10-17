//! A plan's output and measurement expressions read the drawn ground as node 0.
//!
//! A schematic deck writes every ground name a drawing uses as node `0`, so
//! the deck has no node called `GND!`. An output, a measurement or a
//! specification that names the ground the user drew by that name reads node
//! `0`, and one naming a node the deck lacks is still refused.

use crate::simulation::controller::test_execution::{run_generated_batch, run_manual_deck};
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{
    ComponentType, OutputSelectionMode, Point, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming, SimulationRun, SpecEntry, SpecPointScope, SpecificationDefinition,
};
use crate::workbench::app_state::AppState;
use rspice_results::run::SimulationRunLifecycle;
use rspice_results::specification::SpecificationComparison;
use rspice_results::specification_verdict::SpecificationVerdictStatus;

/// V1 = 2 V into R1 = 1k over R2 = 3k, every ground drawn as a `GND!` label
/// and no ground symbol: V(out) = 1.5 V. One transient, nothing else enabled.
fn divider_returned_through_gnd_labels() -> AppState {
    let mut state = AppState::default();
    state.sim_setup.run_set = rspice_simulation_contract::run_set::RunSetState::reference_only();
    state.sim_setup.reference_pvt.temperature_celsius = 27.0;
    for (index, (kind, name, value, nodes)) in [
        (ComponentType::VoltageSource, "V1", "2", ["in", "GND!"]),
        (ComponentType::Resistor, "R1", "1k", ["in", "out"]),
        (ComponentType::Resistor, "R2", "3k", ["out", "GND!"]),
    ]
    .into_iter()
    .enumerate()
    {
        let id = state
            .schematic
            .add_component(kind, Point::new(100 + index as i32 * 200, 100));
        let component = state
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("placed component");
        component.name = name.to_owned();
        component.value = value.to_owned();
        let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
        for (terminal, node) in terminals.into_iter().zip(nodes) {
            let end = Point::new(
                terminal.x,
                terminal.y + if terminal.y < 100 { -20 } else { 20 },
            );
            state.schematic.add_wire(vec![terminal, end]).unwrap();
            state.schematic.add_net_label(end, node.to_owned());
        }
    }
    state.schematic.session.editor.selection.components.clear();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    let (transient, _) = plan.insert(AnalysisKind::Transient).unwrap();
    plan.edit(transient, |draft| {
        let AnalysisDraft::Transient(config) = draft else {
            unreachable!()
        };
        config.stop = "1u".into();
        config.step = "100n".into();
        config.max_step = "100n".into();
    })
    .unwrap();
    state.sim_setup.save_policy.output_selection_mode = OutputSelectionMode::ExplicitOnly;
    state.sim_setup.save_policy.maximum_storage_bytes = u64::MAX;
    state
}

/// A raw probe, kept on the transient's own grid.
fn probe(name: &str, expression: &str) -> SavedOutput {
    output(
        SavedOutputKind::RawVoltageOrCurrent,
        name,
        expression,
        SavedOutputPolicy::SelectedAndFinalPoints,
    )
}

/// A derived expression, evaluated on demand from the retained sources the
/// run bound it to.
fn derived(name: &str, expression: &str) -> SavedOutput {
    output(
        SavedOutputKind::DerivedExpression,
        name,
        expression,
        SavedOutputPolicy::OnDemandFromRetainedState,
    )
}

fn output(
    kind: SavedOutputKind,
    name: &str,
    expression: &str,
    policy: SavedOutputPolicy,
) -> SavedOutput {
    SavedOutput::new(
        kind,
        name,
        expression,
        SavedOutputCompatibility::AllCompatibleAnalyses,
        policy,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap()
}

/// A specification that defines its own `.MEAS` card and bounds it.
fn measured(
    name: &str,
    card: &str,
    comparison: SpecificationComparison,
) -> SpecificationDefinition {
    let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: name.into(),
        expression: card.into(),
        min: None,
        max: None,
        unit: "V".into(),
        scope: SpecPointScope::AllPoints,
    });
    definition.define_measurement = true;
    definition.comparison = comparison;
    definition
}

fn run_with(
    mut state: AppState,
    outputs: Vec<SavedOutput>,
    definitions: Vec<SpecificationDefinition>,
) -> SimulationRun {
    let plan_id = state.sim_setup.analysis_plan.as_ref().unwrap().id();
    state
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, definitions);
    state
        .workspace
        .content
        .plan_data_mut(plan_id)
        .expect("the plan carries a payload")
        .saved_outputs = outputs;
    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

/// Every sample of the output named `name`, materializing it first when the
/// run deferred it.
fn samples(run: &mut SimulationRun, name: &str) -> Vec<f64> {
    let analysis = &mut run.analyses[0];
    let index = analysis
        .saved_output_receipts
        .iter()
        .position(|receipt| receipt.name == name)
        .unwrap_or_else(|| panic!("{name} has a receipt"));
    if matches!(
        analysis.saved_output_receipts[index].status,
        SavedOutputMaterializationStatus::Deferred
    ) {
        crate::simulation::output_contract::materialize_deferred_saved_output(analysis, index)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    let receipt = &analysis.saved_output_receipts[index];
    let materialized = receipt
        .status
        .materialized_waveforms()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    let [waveform] = materialized.as_slice() else {
        panic!("{name}: {:?}", receipt.status);
    };
    analysis
        .waveforms
        .iter()
        .find(|waveform_data| waveform_data.name == *waveform)
        .expect("the materialized waveform is retained")
        .y
        .to_vec()
}

#[test]
fn outputs_measures_and_specifications_read_the_drawn_ground_as_node_zero() {
    let state = divider_returned_through_gnd_labels();
    let range = SpecificationComparison::Range {
        minimum: 1.49,
        maximum: 1.51,
    };
    let mut run = run_with(
        state,
        vec![
            probe("vout", "V(out)"),
            probe("vout_gnd", "V(out,GND!)"),
            derived("vdiff", "V(out)-V(GND!)"),
            probe("vmissing", "V(out,nowhere)"),
        ],
        vec![
            measured(
                "avg_out",
                ".MEAS TRAN avg_out AVG V(out) FROM=200n TO=800n",
                range.clone(),
            ),
            measured(
                "avg_diff",
                ".MEAS TRAN avg_diff AVG {V(out)-V(GND!)} FROM=200n TO=800n",
                range.clone(),
            ),
            measured(
                "at_gnd",
                ".MEAS TRAN at_gnd FIND V(out,GND!) AT=500n",
                range,
            ),
        ],
    );

    // The derived output reads the retained sources at every accepted point
    // rather than on the probes' grid, so each is held to V(out) itself.
    for name in ["vout", "vout_gnd", "vdiff"] {
        let values = samples(&mut run, name);
        assert!(
            !values.is_empty() && values.iter().all(|value| (value - 1.5).abs() < 1e-9),
            "{name}: {values:?}"
        );
    }
    let missing = run.analyses[0]
        .saved_output_receipts
        .iter()
        .find(|receipt| receipt.name == "vmissing")
        .expect("the missing-node output has a receipt");
    assert!(
        matches!(
            missing.status,
            SavedOutputMaterializationStatus::Unavailable { .. }
        ),
        "a node the deck lacks is never read as 0 V: {:?}",
        missing.status
    );

    let verdicts = run
        .specification_verdicts()
        .expect("the run judged its specifications");
    let value_of = |name: &str| {
        let verdict = verdicts
            .iter()
            .find(|verdict| verdict.measurement() == name)
            .unwrap_or_else(|| panic!("{name} is judged: {verdicts:?}"));
        assert_eq!(
            verdict.status(),
            SpecificationVerdictStatus::Pass,
            "{name}: {verdict:?}"
        );
        verdict
            .worst_value()
            .expect("a passing verdict has a value")
    };
    let avg_out = value_of("avg_out");
    assert!((avg_out - 1.5).abs() < 1e-9, "{avg_out}");
    for name in ["avg_diff", "at_gnd"] {
        assert!((value_of(name) - avg_out).abs() < 1e-12, "{name}");
    }
}

/// A measurement over a node the deck lacks is refused rather than measured
/// against 0 V, whatever the drawing calls its ground.
#[test]
fn a_measurement_over_a_node_the_deck_lacks_is_still_refused() {
    let mut state = divider_returned_through_gnd_labels();
    let plan_id = state.sim_setup.analysis_plan.as_ref().unwrap().id();
    state
        .workspace
        .content
        .replace_active_specification_definitions(
            plan_id,
            vec![measured(
                "at_nowhere",
                ".MEAS TRAN at_nowhere FIND V(out,nowhere) AT=500n",
                SpecificationComparison::Tracked,
            )],
        );
    let run = run_generated_batch(state, SimulationRunLifecycle::Failed);
    let [analysis] = run.analyses.as_slice() else {
        panic!("one transient: {:?}", run.analyses);
    };
    let refusal = analysis.error_message.as_deref().unwrap_or_default();
    assert!(
        !analysis.success && refusal.contains(".MEASURE node 'nowhere'"),
        "{refusal}"
    );
}

/// A hand-written deck keeps its dialect's meaning: a deck that defines a
/// real `gnd!` node measures that node, not node 0.
#[test]
fn a_hand_written_deck_that_defines_a_gnd_bang_node_keeps_it() {
    // gnd! sits at 2 V * 1k / (4k + 1k) = 0.4 V.
    let result = run_manual_deck(
        "Hand-written gnd!\nV1 in 0 2\nR1 in out 1k\nR2 out gnd! 3k\nR3 gnd! 0 1k\n\
         .tran 100n 1u\n.MEAS TRAN at_gnd FIND V(gnd!) AT=500n\n\
         .MEAS TRAN out_gnd FIND V(out,gnd!) AT=500n\n.end\n",
    );
    let at_gnd = result.measurement("at_gnd").expect("gnd! is measured");
    assert!((at_gnd - 0.4).abs() < 1e-9, "{at_gnd}");
    let out_gnd = result
        .measurement("out_gnd")
        .expect("out over gnd! is measured");
    assert!((out_gnd - 1.2).abs() < 1e-9, "{out_gnd}");
}
