//! Operating-point source qualification, cache lifetime, schematic navigation and lossless export.

use super::{
    AnalysisPresentationKey,
    frame_work::{self, DatasetWalk},
};
use crate::{
    schematic::{BusNotations, bus_notations},
    state::{
        AnalysisResultPayload, AnalysisType, RunHistoryRevision, SchematicAnnotationVisibility,
    },
    workbench::AppState,
};
use egui::Ui;
use rspice_results::operating_point::report::{
    OperatingPointReportFacts, RetainedDeviceDetail as RetainedDetail, retained_detail_allows,
    signal_leaf,
};
use rspice_results_ui::op_inspector::{self as viewer, OpAbsence, OpAction, OpEvidence};
use std::{borrow::Cow, sync::Arc};
fn retained_detail(analysis: &crate::state::AnalysisResult) -> Option<RetainedDetail> {
    match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            device_detail,
            selected_devices,
            violation_devices,
            ..
        }) => Some(RetainedDetail {
            policy: *device_detail,
            selected: selected_devices.clone(),
            violations: violation_devices.clone(),
        }),
        _ => None,
    }
}
fn selected_op_evidence(state: &AppState) -> Option<OpEvidence> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    if analysis.analysis_type != AnalysisType::DcOp {
        return None;
    }

    let (detail_policy, facts) = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            temperature_celsius,
            annotation,
            device_detail,
            mna_node_names,
            mna_branch_names,
            run_point_index,
            run_point_count,
            run_point_process,
            ..
        }) => (
            Some(*device_detail),
            Some(OperatingPointReportFacts {
                temperature_celsius: *temperature_celsius,
                process: *run_point_process,
                point_index: *run_point_index,
                point_count: *run_point_count,
                mna_nodes: mna_node_names.len(),
                mna_branches: mna_branch_names.len(),
                annotation: *annotation,
            }),
        ),
        _ => (None, None),
    };

    Some(OpEvidence {
        run_id: run.id,
        label: analysis.label.clone(),
        success: analysis.success,
        error: analysis.error_message.clone(),
        detail_policy,
        node_count: analysis
            .dc_op
            .as_ref()
            .map_or(0, |dc| dc.node_voltages.len()),
        branch_count: analysis
            .dc_op
            .as_ref()
            .map_or(0, |dc| dc.branch_currents.len()),
        facts,
    })
}
pub(crate) fn export_csv(analysis: &crate::state::AnalysisResult) -> Option<super::ResultSheetCsv> {
    if !analysis.success || analysis.analysis_type != AnalysisType::DcOp {
        return None;
    }
    let (detail, facts) = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            temperature_celsius,
            annotation,
            device_detail,
            selected_devices,
            violation_devices,
            mna_node_names,
            mna_branch_names,
            run_point_index,
            run_point_count,
            run_point_process,
            ..
        }) => (
            Some(RetainedDetail {
                policy: *device_detail,
                selected: selected_devices.clone(),
                violations: violation_devices.clone(),
            }),
            Some(OperatingPointReportFacts {
                temperature_celsius: *temperature_celsius,
                process: *run_point_process,
                point_index: *run_point_index,
                point_count: *run_point_count,
                mna_nodes: mna_node_names.len(),
                mna_branches: mna_branch_names.len(),
                annotation: *annotation,
            }),
        ),
        _ => (None, None),
    };
    let dc = analysis.dc_op.as_ref();
    let devices = analysis.device_op.as_ref();
    if dc.is_none() && devices.is_none() && facts.is_none() {
        return None;
    }

    let mut csv = rspice_formats::result_csv::OperatingPointReportCsv::new();
    if let Some(facts) = facts.as_ref() {
        csv.append_solve_facts(facts);
    }
    if let Some(dc) = dc {
        csv.append_dc_values(dc);
    }
    if let Some(devices) = devices {
        for entry in devices
            .entries
            .iter()
            .filter(|entry| retained_detail_allows(&entry.name, detail.as_ref()))
        {
            csv.append_device(entry);
        }
    }
    let rows = csv.row_count();

    Some(super::ResultSheetCsv {
        default_name: "rspice-operating-point.csv",
        detail: format!("{rows} operating-point evidence rows"),
        contents: csv.into_string(),
    })
}
fn result_mapping_is_current(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt().is_some_and(|receipt| {
        receipt.project_revision() == state.workspace.content.project.revision()
    }) && state.simulation.source.cross_probe.is_current_for(
        &state.workspace.content.active_view,
        state.schematic.topology_version(),
    )
}
fn node_target_available(state: &AppState, name: &str) -> bool {
    if !result_mapping_is_current(state) {
        return false;
    }
    let name = signal_leaf(name);
    name != "0"
        && state
            .simulation
            .source
            .cross_probe
            .net_to_points
            .iter()
            .any(|(candidate, points)| candidate.eq_ignore_ascii_case(name) && !points.is_empty())
}
fn device_target(state: &AppState, name: &str) -> Option<u64> {
    if !result_mapping_is_current(state) {
        return None;
    }
    state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.spice_instance_name().eq_ignore_ascii_case(name))
        .map(|component| component.id)
}
fn apply_action(ui: &Ui, state: &mut AppState, action: OpAction) {
    match action {
        OpAction::LocateNode(name) => {
            let signal = if name
                .trim_start()
                .chars()
                .next()
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(&'V'))
                && name.contains('(')
            {
                name
            } else {
                format!("V({})", signal_leaf(&name))
            };
            match crate::schematic::view::select_signal_conductor(state, &signal) {
                Ok(_) => {
                    state.ui.schematic_visibility.annotations =
                        SchematicAnnotationVisibility::OperatingPoint;
                    state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
                }
                Err(error) => state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Cannot locate node",
                    error.message(&signal),
                ),
            }
        }
        OpAction::LocateDevice(component_id) => {
            let Some(component) = state
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == component_id)
            else {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Cannot locate device",
                    "The retained device no longer resolves to the active schematic.",
                );
                return;
            };
            let position = component.pos;
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(component_id);
            state.schematic.session.editor.center_request = Some(position);
            state.ui.schematic_visibility.annotations =
                SchematicAnnotationVisibility::OperatingPoint;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Design);
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct OpPlanKey {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    filter: String,
    sort: Option<(String, bool)>,
    root: String,
}
impl OpPlanKey {
    fn controls(&self) -> viewer::OpControls<'_> {
        viewer::OpControls {
            filter: &self.filter,
            sort: self.sort.as_ref(),
            root: &self.root,
        }
    }
}
#[derive(Debug, Clone)]
pub(super) struct OpPlan {
    key: OpPlanKey,
    display: viewer::OpPlan,
}
fn op_plan(state: &mut AppState, analysis: AnalysisPresentationKey) -> Option<Arc<OpPlan>> {
    let key = OpPlanKey {
        source: (
            state.simulation.retained.runs.revision(),
            state.simulation.view.data_version,
        ),
        analysis,
        filter: state.ui.results.session.op_filter.clone(),
        sort: state.ui.results.session.op_sort.clone(),
        root: state.workspace.content.simulation_root_reference().cell,
    };
    if let Some(plan) = state.ui.results.plans.op.as_ref()
        && plan.key == key
    {
        return Some(Arc::clone(plan));
    }
    let analysis_result = state.simulation.active_analysis()?;
    let detail = retained_detail(analysis_result);
    frame_work::note(DatasetWalk::OpPlan);
    let display = viewer::OpPlan::new(
        key.controls(),
        analysis_result.dc_op.as_ref(),
        analysis_result.device_op.as_ref(),
        detail.as_ref(),
    );
    let built = Arc::new(OpPlan { key, display });
    state.ui.results.plans.op = Some(Arc::clone(&built));
    Some(built)
}
struct OpMapping<'a> {
    state: &'a AppState,
    notations: Option<BusNotations>,
}
impl viewer::SchematicMapping for OpMapping<'_> {
    fn display_node<'a>(&self, name: &'a str) -> Cow<'a, str> {
        self.notations
            .as_ref()
            .map_or(Cow::Borrowed(name), |notations| notations.display(name))
    }
    fn node_available(&self, name: &str) -> bool {
        node_target_available(self.state, name)
    }
    fn device_target(&self, name: &str) -> Option<u64> {
        device_target(self.state, name)
    }
}
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(evidence) = selected_op_evidence(state) else {
        let absence = match state.simulation.active_analysis() {
            Some(_) => OpAbsence::OtherAnalysis,
            None => OpAbsence::Missing,
        };
        viewer::show_absent(ui, absence);
        return;
    };
    let retains_values = state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| analysis.dc_op.is_some() || analysis.device_op.is_some());
    if !retains_values {
        viewer::show_absent(ui, OpAbsence::NoValues);
        return;
    }

    let analysis_key = state.simulation.active_run().and_then(|run| {
        state
            .simulation
            .active_analysis()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
    });
    let plan = analysis_key.and_then(|key| op_plan(state, key));
    let analysis = state.simulation.active_analysis();
    let dc = analysis.and_then(|analysis| analysis.dc_op.as_ref());
    let devices = analysis.and_then(|analysis| analysis.device_op.as_ref());
    let notations = dc
        .filter(|dc| !dc.node_voltages.is_empty())
        .filter(|_| {
            plan.as_ref()
                .is_some_and(|plan| plan.display.node_shown() > 0)
        })
        .map(|_| bus_notations(&state.workspace, &state.schematic));
    let mapping = OpMapping { state, notations };
    let controls = plan
        .as_ref()
        .map(|plan| plan.key.controls())
        .unwrap_or(viewer::OpControls {
            filter: "",
            sort: None,
            root: "",
        });
    let response = viewer::show(
        ui,
        plan.as_ref().map(|plan| &plan.display),
        &viewer::OpView {
            evidence: &evidence,
            dc,
            devices,
            controls,
            mapping: &mapping,
        },
    );
    let viewer::OpResponse {
        action,
        clicked_sort,
    } = response;
    if let Some(action) = action {
        apply_action(ui, state, action);
    }
    if let Some(key) = clicked_sort {
        state.ui.results.session.op_sort = match state.ui.results.session.op_sort.as_ref() {
            Some((current, ascending)) if current.eq_ignore_ascii_case(&key) => {
                Some((key, !*ascending))
            }
            // Analog operating-point work generally starts with the largest
            // retained magnitude, so a new quantity sorts descending by
            // absolute value while preserving the retained sign in the table.
            _ => Some((key, false)),
        };
    }
}
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(evidence) = selected_op_evidence(state) else {
        return;
    };
    let analysis_key = state.simulation.active_run().and_then(|run| {
        state
            .simulation
            .active_analysis()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
    });
    let device_count = analysis_key
        .and_then(|key| op_plan(state, key))
        .map_or(0, |plan| plan.display.device_in_scope());
    viewer::right_panel(
        ui,
        &evidence,
        device_count,
        result_mapping_is_current(state),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::operating_point::DcOpResult;
    fn op_state(nodes: usize, devices: usize) -> AppState {
        use crate::state::{AnalysisResult, SimulationRun};

        let (dc, report) = viewer::evidence_fixture(nodes, devices);
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(dc);
        analysis.device_op = Some(report);
        let mut run = SimulationRun::new(3);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state.simulation.view.active_analysis_idx = Some(0);
        state
    }
    fn active_key(state: &AppState) -> AnalysisPresentationKey {
        let run = state.simulation.active_run().expect("retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }
    #[test]
    fn selection_never_falls_back_to_another_analysis() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = AppState::default();
        let mut run = SimulationRun::new(9);
        let mut first = AnalysisResult::new(1, AnalysisType::DcOp, "OP 1");
        first.dc_op = Some(DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(out)".to_owned(),
                value: 1.0,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        });
        run.add_analysis(first);
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "TRAN"));
        state.simulation.retained.runs.push(run);
        state.simulation.view.active_run_idx = Some(0);

        state.simulation.view.active_analysis_idx = Some(1);
        assert!(selected_op_evidence(&state).is_none());
        state.simulation.view.active_analysis_idx = Some(0);
        let evidence = selected_op_evidence(&state).expect("selected OP evidence");
        assert_eq!(evidence.run_id, 9);
        assert_eq!(evidence.label, "OP 1");
        assert_eq!(evidence.node_count, 1);
    }
    #[test]
    fn node_evidence_does_not_depend_on_a_device_report() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = AppState::default();
        let mut run = SimulationRun::new(4);
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(top.out)".to_owned(),
                value: 2.5,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        });
        run.add_analysis(analysis);
        state.simulation.retained.runs.push(run);
        state.simulation.view.active_run_idx = Some(0);
        state.simulation.view.active_analysis_idx = Some(0);

        let evidence = selected_op_evidence(&state).expect("retained OP evidence");
        assert_eq!(evidence.node_count, 1);
        let dc = state.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_ref()
            .expect("retained node voltages");
        assert!(
            state.simulation.retained.runs[0].analyses[0]
                .device_op
                .is_none()
        );
        assert_eq!(
            viewer::OpPlan::new(
                viewer::OpControls {
                    filter: "out",
                    sort: None,
                    root: "amplifier"
                },
                Some(dc),
                None,
                None
            )
            .node_shown(),
            1
        );
    }
    #[test]
    fn retained_view_source_operating_point_rebuilds_node_and_device_indices() {
        let mut state = op_state(40, 40);
        let key = active_key(&state);
        let original = op_plan(&mut state, key).unwrap();
        let version = state.simulation.view.data_version;
        state.simulation.retained.runs[0].analyses[0] =
            op_state(3, 2).simulation.retained.runs[0].analyses[0].clone();
        let changed = op_plan(&mut state, key).unwrap();
        assert_eq!(changed.display.node_shown(), 3);
        assert_eq!(changed.display.device_shown(), 2);
        assert_eq!(changed.display.device_in_scope(), 2);
        assert_eq!(original.display.node_shown(), 40);
        assert_eq!(original.display.device_shown(), 40);
        assert!(changed.display.node_indices().all(|index| index < 3));
        assert!(changed.display.device_indices().all(|index| index < 2));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state);
                right_panel(ui, &mut state);
            });
        });
        assert_eq!(state.simulation.view.data_version, version);
    }
    #[test]
    fn the_readers_controls_are_part_of_the_row_plan_key() {
        let mut state = op_state(40, 40);
        let key = active_key(&state);
        let unfiltered = op_plan(&mut state, key).expect("a row plan");
        assert!(Arc::ptr_eq(
            &unfiltered,
            &op_plan(&mut state, key).expect("a row plan")
        ));

        state.ui.results.session.op_filter = "m1".to_owned();
        let filtered = op_plan(&mut state, key).expect("a row plan");
        assert!(filtered.display.device_shown() < unfiltered.display.device_shown());
        assert!(
            filtered.display.device_shown() > 0,
            "the filter matched nothing, so it proves nothing"
        );

        state.ui.results.session.op_filter.clear();
        state.ui.results.session.op_sort = Some(("gm".to_owned(), true));
        let sorted = op_plan(&mut state, key).expect("a row plan");
        assert_ne!(
            sorted.display.device_indices().collect::<Vec<_>>(),
            unfiltered.display.device_indices().collect::<Vec<_>>(),
            "a new sort key served the previous ordering"
        );
    }
    #[test]
    fn a_new_dataset_generation_rebuilds_the_row_plan() {
        let mut state = op_state(8, 8);
        let key = active_key(&state);
        let before = op_plan(&mut state, key).expect("a row plan");
        assert_eq!(before.display.node_shown(), 8);

        state.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("retained node voltages")
            .node_voltages
            .truncate(3);
        state.simulation.view.data_version = state.simulation.view.data_version.wrapping_add(1);

        let after = op_plan(&mut state, key).expect("a row plan");
        assert_eq!(
            after.display.node_shown(),
            3,
            "the sheet would have indexed rows the new dataset no longer retains"
        );
        assert_eq!(
            after.display.node_offset_count(),
            after.display.node_row_count()
        );
    }
}
