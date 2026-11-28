//! A SPICE directive note on the sheet joins the run's netlist: its
//! `.param` sets a value the run reads, its `.ic` sets where the transient
//! starts, and a card in it the run cannot take stops the run on the note.

use super::test_execution::run_generated_batch;
use super::test_host::RunTestHost;
use super::{SimulationController, SimulationRunIntent};
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{ComponentType, DesignNote, DesignNoteKind, Point, SimulationRun};
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation_contract::drafts::TranSetup;

/// Wire each terminal of component `id` out to a label naming its node.
fn connect(state: &mut RunTestHost, id: u64, nodes: &[&str]) {
    let schematic = &mut state.model.schematic;
    let component = schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the placed part");
    let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
    assert_eq!(terminals.len(), nodes.len());
    for (terminal, node) in terminals.into_iter().zip(nodes) {
        let end = Point::new(terminal.x, terminal.y + 20);
        schematic.add_wire(vec![terminal, end]).expect("a wire");
        schematic.add_net_label(end, (*node).to_owned());
    }
}

fn place(state: &mut RunTestHost, kind: ComponentType, value: &str, nodes: &[&str]) {
    let at = Point::new(
        100 + 200 * state.model.schematic.document().components.len() as i32,
        100,
    );
    let id = state.model.schematic.add_component(kind, at);
    let component = state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed part");
    component.value = value.to_owned();
    connect(state, id, nodes);
}

/// A 1 V source driving `top` into `bottom` to ground, the junction named
/// `out`, with a directive note holding `directive` and `analysis` the only
/// analysis of the plan.
fn divider(
    top: &str,
    bottom: (ComponentType, &str),
    directive: &str,
    analysis: AnalysisDraft,
) -> RunTestHost {
    let mut state = RunTestHost::default();
    state.model.sim_setup.run_set =
        rspice_simulation_contract::run_set::RunSetState::reference_only();
    place(&mut state, ComponentType::VoltageSource, "1", &["in", "0"]);
    place(&mut state, ComponentType::Resistor, top, &["in", "out"]);
    place(&mut state, bottom.0, bottom.1, &["out", "0"]);
    place(&mut state, ComponentType::Ground, "", &["0"]);
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            DesignNote::new(
                9_001,
                Point::new(100, 400),
                DesignNoteKind::SpiceDirective,
                directive,
            )
            .expect("a directive the netlist takes"),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .components
        .clear();
    let plan = state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a plan");
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false)
            .expect("disable the default analysis");
    }
    let (id, _) = plan.insert(analysis.kind()).expect("the analysis");
    plan.edit(id, |target| *target = analysis)
        .expect("its settings");
    state
}

/// V(out) at the run's first sample.
fn first_out(run: &SimulationRun) -> f64 {
    let wave = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| wave.name.eq_ignore_ascii_case("V(out)"))
        .unwrap_or_else(|| panic!("no V(out) in the run"));
    wave.y.sample(0).expect("a first sample")
}

fn operating_point() -> AnalysisDraft {
    AnalysisDraft::for_kind(AnalysisKind::OperatingPoint)
}

#[test]
fn a_param_on_a_directive_note_sets_the_value_the_run_reads() {
    let half = run_generated_batch(
        divider(
            "{rtop}",
            (ComponentType::Resistor, "1k"),
            ".param rtop=1k",
            operating_point(),
        ),
        SimulationRunLifecycle::Completed,
    );
    let quarter = run_generated_batch(
        divider(
            "{rtop}",
            (ComponentType::Resistor, "1k"),
            ".param rtop=3k",
            operating_point(),
        ),
        SimulationRunLifecycle::Completed,
    );
    assert!(
        (first_out(&half) - 0.5).abs() < 1e-6,
        "{}",
        first_out(&half)
    );
    assert!(
        (first_out(&quarter) - 0.25).abs() < 1e-6,
        "{}",
        first_out(&quarter)
    );
}

#[test]
fn an_ic_on_a_directive_note_sets_where_the_transient_starts() {
    let transient = || {
        AnalysisDraft::Transient(TranSetup {
            stop: "1u".to_owned(),
            step: "10n".to_owned(),
            ..TranSetup::default()
        })
    };
    let settled = run_generated_batch(
        divider(
            "1k",
            (ComponentType::Capacitor, "1u"),
            "* no start condition",
            transient(),
        ),
        SimulationRunLifecycle::Completed,
    );
    let held = run_generated_batch(
        divider(
            "1k",
            (ComponentType::Capacitor, "1u"),
            ".ic v(out)=0.3",
            transient(),
        ),
        SimulationRunLifecycle::Completed,
    );
    assert!(
        (first_out(&settled) - 1.0).abs() < 1e-6,
        "{}",
        first_out(&settled)
    );
    assert!(
        (first_out(&held) - 0.3).abs() < 1e-6,
        "{}",
        first_out(&held)
    );
}

/// A card the run cannot take stops it on the note's line, and the finding
/// leads to the note rather than into the deck.
fn refusal_route(state: &mut RunTestHost) -> crate::state::PreflightRemediation {
    state.model.simulation.execution.run_intent = SimulationRunIntent::SimulateRunSet;
    state.sync_active_schematic_to_workspace();
    let error = SimulationController::new()
        .preflight_run_set(state)
        .expect_err("the run is refused");
    let issue = crate::workbench::preflight::preparation_issue(state, &error);
    assert!(issue.observed.contains("SPICE directive on"), "{error:?}");
    issue.remediation
}

#[test]
fn a_param_the_plan_also_defines_stops_the_run_on_the_notes_line() {
    let mut state = divider(
        "{rtop}",
        (ComponentType::Resistor, "1k"),
        "* bias\n.param rtop=1k",
        operating_point(),
    );
    let plan = state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("a plan")
        .id();
    let content = &mut state.model.workspace.content;
    content.migrate_active_plan_data(plan);
    let payload = content.plan_data_mut(plan).expect("the plan's payload");
    payload.design_variables.push(
        rspice_simulation_contract::design_variable::DesignVariable::new(
            "rtop".to_owned(),
            "2k".to_owned(),
            rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity::Dimensionless,
            String::new(),
            None,
            rspice_simulation_contract::design_variable::DesignVariableSweepEligibility::NestedSweepAndOptimization,
            rspice_simulation_contract::design_variable::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .expect("a design variable"),
    );
    let reference = state.model.workspace.content.active_schematic_reference();
    assert_eq!(
        refusal_route(&mut state),
        crate::state::PreflightRemediation::Drawing {
            cell_view: reference,
            object: crate::state::DrawnObject::Note { id: 9_001, line: 2 },
        }
    );
}

#[test]
fn a_card_the_engine_refuses_on_a_note_leads_to_the_notes_line() {
    let mut state = divider(
        "1k",
        (ComponentType::Resistor, "1k"),
        ".param rtop=1k\n.model = d",
        operating_point(),
    );
    let reference = state.model.workspace.content.active_schematic_reference();
    assert_eq!(
        refusal_route(&mut state),
        crate::state::PreflightRemediation::Drawing {
            cell_view: reference,
            object: crate::state::DrawnObject::Note { id: 9_001, line: 2 },
        }
    );
}
