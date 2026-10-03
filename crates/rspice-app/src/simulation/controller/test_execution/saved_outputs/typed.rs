//! Typed DC retrace and noise controls that do not round-trip through SPICE cards.

use super::{AnalysisSpec, OutputSelectionMode, SimulationRun};
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{ComponentType, Point};
use rspice_core::netlist::{ElementKind, SourceSpec};
use rspice_simulation_contract::drafts::{DcSetup, NoiseDraft};

pub(super) fn run(deck: &str, spec: &AnalysisSpec) -> SimulationRun {
    let parsed = rspice_core::Netlist::parse(deck).unwrap();
    assert!(parsed.subcircuits.is_empty());
    let mut state = super::super::AppState::default();
    state.sim_setup.run_set = rspice_simulation_contract::run_set::RunSetState::reference_only();
    state.sim_setup.reference_pvt.temperature_celsius = 27.0;
    for (index, element) in parsed.elements.iter().enumerate() {
        let (kind, value, params) = match &element.kind {
            ElementKind::Resistor {
                value,
                model,
                instance_params,
                deferred_params,
                ..
            } => {
                assert!(
                    model.is_none() && instance_params.is_empty() && deferred_params.is_empty()
                );
                (ComponentType::Resistor, value.to_string(), String::new())
            }
            ElementKind::VoltageSource(SourceSpec::Dc(value)) => (
                ComponentType::VoltageSource,
                value.to_string(),
                String::new(),
            ),
            ElementKind::VoltageSource(SourceSpec::Ac { magnitude, phase }) => (
                ComponentType::VoltageSourceAc,
                magnitude.to_string(),
                format!("phase={phase}"),
            ),
            ElementKind::VoltageSource(SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            }) => (
                ComponentType::VoltageSourceAc,
                ac_magnitude.to_string(),
                format!("dc={dc_value} phase={ac_phase}"),
            ),
            other => panic!("unexpected typed output fixture element: {other:?}"),
        };
        let id = state
            .schematic
            .add_component(kind, Point::new(100 + index as i32 * 200, 100));
        let component = state
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap();
        component.name = element.name.clone();
        component.value = value;
        component.params = params;
        connect(&mut state.schematic, id, &element.nodes);
    }
    let ground = state
        .schematic
        .add_component(ComponentType::Ground, Point::new(-100, 100));
    connect(&mut state.schematic, ground, &["0".into()]);
    state.schematic.session.editor.selection.components.clear();
    let draft = match spec {
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            hysteresis,
            modes,
            ..
        } => {
            assert!(*hysteresis && source2.is_none());
            assert_eq!(modes, &Default::default());
            AnalysisDraft::DcSweep(DcSetup {
                source: source_name.clone(),
                start: start.to_string(),
                stop: stop.to_string(),
                step: step.to_string(),
                hysteresis: true,
                ..Default::default()
            })
        }
        AnalysisSpec::Noise {
            output_node,
            reference_node,
            input_source,
            start_freq,
            stop_freq,
            points_per_decade,
            sweep,
            explicit_frequencies,
            data_table_name,
            contribution_detail,
            integration_mode,
            temperature,
        } => {
            assert!(data_table_name.is_none());
            state.sim_setup.reference_pvt.temperature_celsius = temperature - 273.15;
            AnalysisDraft::Noise(NoiseDraft {
                output: output_node.clone(),
                reference: reference_node.clone(),
                input: input_source.clone(),
                fstart: start_freq.to_string(),
                fstop: stop_freq.to_string(),
                points: points_per_decade.to_string(),
                sweep: *sweep,
                explicit_frequencies: explicit_frequencies
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .map(f64::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                contribution_detail: *contribution_detail,
                integration_mode: *integration_mode,
            })
        }
        _ => unreachable!("only controls without a lossless SPICE card use this fixture"),
    };
    let lowered = rspice_simulation::analysis_preparation::analysis_draft_spec(
        &super::super::super::analysis_spec_build::analysis_inputs(&state),
        &draft,
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(lowered).unwrap(),
        serde_json::to_value(spec).unwrap()
    );
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    let op = matches!(spec, AnalysisSpec::Noise { .. })
        .then(|| plan.insert(AnalysisKind::OperatingPoint).unwrap().0);
    let (id, _) = plan.insert(draft.kind()).unwrap();
    plan.edit(id, |target| *target = draft).unwrap();
    if let Some(op) = op {
        plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
            .unwrap();
    }
    state.sim_setup.save_policy.output_selection_mode = OutputSelectionMode::SaveAll;
    state.sim_setup.save_policy.maximum_storage_bytes = u64::MAX;
    super::super::run_generated_batch(
        state,
        rspice_results::run::SimulationRunLifecycle::Completed,
    )
}

fn connect(schematic: &mut crate::state::SchematicState, id: u64, nodes: &[String]) {
    let component = schematic
        .document()
        .components
        .iter()
        .find(|c| c.id == id)
        .unwrap();
    let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
    assert_eq!(terminals.len(), nodes.len());
    for (terminal, node) in terminals.into_iter().zip(nodes) {
        let end = Point::new(
            terminal.x,
            terminal.y + if terminal.y < 100 { -20 } else { 20 },
        );
        schematic.add_wire(vec![terminal, end]).unwrap();
        schematic.add_net_label(end, node.clone());
    }
}
