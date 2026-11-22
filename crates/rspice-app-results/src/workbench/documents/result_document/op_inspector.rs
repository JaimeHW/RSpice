//! Operating-point source qualification, cache lifetime, schematic navigation and lossless export.

use super::{
    AnalysisPresentationKey,
    frame_work::{self, DatasetWalk},
    operational_state,
};
use crate::state::{
    AnalysisResultPayload, AnalysisType, BusNotations, RunHistoryRevision,
    SchematicAnnotationVisibility, bus_notations,
};
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::framework::requests::ShowSchematicAnnotations;
use egui::Ui;
use rspice_results::operating_point::report::{
    OperatingPointReportFacts, RetainedDeviceDetail as RetainedDetail, retained_detail_allows,
    signal_leaf,
};
use rspice_results_ui::op_inspector::{self as viewer, OpAbsence, OpAction, OpEvidence};
use std::{borrow::Cow, collections::BTreeSet, sync::Arc};
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
fn selected_op_evidence(state: ResultsReads<'_>) -> Option<OpEvidence> {
    state.model.simulation.active_run()?;
    let analysis = state.model.simulation.active_analysis()?;
    if analysis.analysis_type != AnalysisType::DcOp {
        return None;
    }

    let facts = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            temperature_celsius,
            annotation,
            mna_node_names,
            mna_branch_names,
            run_point_index,
            run_point_count,
            run_point_process,
            ..
        }) => Some(OperatingPointReportFacts {
            temperature_celsius: *temperature_celsius,
            process: *run_point_process,
            point_index: *run_point_index,
            point_count: *run_point_count,
            mna_nodes: mna_node_names.len(),
            mna_branches: mna_branch_names.len(),
            annotation: *annotation,
        }),
        _ => None,
    };

    Some(OpEvidence {
        success: analysis.success,
        error: analysis.error_message.clone(),
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
        detail: format!("{rows} operating-point rows"),
        contents: csv.into_string(),
    })
}
fn result_mapping_is_current(state: ResultsReads<'_>) -> bool {
    let Some(run) = state.model.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt().is_some_and(|receipt| {
        receipt.project_revision() == state.model.workspace.content.project.revision()
    }) && state.model.result_mapping_is_current()
}
fn node_target_available(state: ResultsReads<'_>, name: &str) -> bool {
    if !result_mapping_is_current(state) {
        return false;
    }
    let name = signal_leaf(name);
    name != "0" && state.model.drawing_draws_net(name)
}
fn device_target(state: ResultsReads<'_>, name: &str) -> Option<u64> {
    if !result_mapping_is_current(state) {
        return None;
    }
    rspice_design::schematic::component::component_for_engine_device(
        &state.model.schematic.document().components,
        &state.model.workspace.content.occurrence_path(),
        name,
    )
    .map(|component| component.id)
}
pub fn apply_action(state: &mut ResultsParts<'_>, action: OpAction) {
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
            match state.model.select_signal_conductor(&signal) {
                Ok(_) => {
                    state.workbench.outbox.push(ShowSchematicAnnotations(
                        SchematicAnnotationVisibility::OperatingPoint,
                    ));
                    state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
                }
                Err(error) => state.prompt(error.message(&signal)),
            }
        }
        OpAction::Refused(reason) => state.prompt(reason),
        OpAction::LocateDevice(component_id) => {
            let Some(component) = state
                .model
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == component_id)
            else {
                state.prompt("The retained device no longer resolves to the active schematic.");
                return;
            };
            let position = component.pos;
            state.show_component_on_schematic(component_id, position);
            state.workbench.outbox.push(ShowSchematicAnnotations(
                SchematicAnnotationVisibility::OperatingPoint,
            ));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct OpPlanKey {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    filter: String,
    sort: viewer::OpSort,
    collapsed: BTreeSet<String>,
    significant_digits: usize,
    compare: Option<viewer::OpCompare>,
}
impl OpPlanKey {
    fn controls(&self) -> viewer::OpControls<'_> {
        viewer::OpControls {
            filter: &self.filter,
            sort: &self.sort,
            collapsed: &self.collapsed,
            significant_digits: self.significant_digits,
        }
    }
}
#[derive(Debug, Clone)]
pub(super) struct OpPlan {
    key: OpPlanKey,
    display: viewer::OpPlan,
}
fn op_plan(state: &mut ResultsParts<'_>, analysis: AnalysisPresentationKey) -> Option<Arc<OpPlan>> {
    let key = OpPlanKey {
        source: (
            state.model.simulation.retained.runs.revision(),
            state.model.simulation.view.data_version,
        ),
        analysis,
        filter: state.results.session.op_filter.clone(),
        sort: state.results.session.op_sort.clone(),
        collapsed: state.results.session.op_collapsed.clone(),
        significant_digits: usize::from(
            state
                .results_reads()
                .presentation_policy()
                .displayed_significant_digits()
                .get(),
        ),
        compare: state.results.session.op_compare,
    };
    if let Some(plan) = state.results.plans.op.as_ref()
        && plan.key == key
    {
        return Some(Arc::clone(plan));
    }
    let analysis_result = state.model.simulation.active_analysis()?;
    let detail = retained_detail(analysis_result);
    frame_work::note(DatasetWalk::OpPlan);
    let mut display = viewer::OpPlan::new(
        key.controls(),
        analysis_result.dc_op.as_ref(),
        analysis_result.device_op.as_ref(),
        detail.as_ref(),
    );
    if let Some(compare) = key.compare
        && let Some(baseline) = state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .find(|run| run.dataset_id == compare.baseline)
            .and_then(|run| baseline_analysis(run, analysis_result.analysis_type))
    {
        display = display.with_baseline(
            viewer::OpBaseline::new(baseline.dc_op.as_ref(), baseline.device_op.as_ref()),
            compare.delta,
            analysis_result.dc_op.as_ref(),
            analysis_result.device_op.as_ref(),
        );
    }
    let built = Arc::new(OpPlan { key, display });
    state.results.plans.op = Some(Arc::clone(&built));
    Some(built)
}
/// The analysis of `run` whose operating point another of `kind` compares
/// against: a solved one that retained values.
fn baseline_analysis(
    run: &crate::state::SimulationRun,
    kind: AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    run.analyses.iter().find(|analysis| {
        analysis.success
            && analysis.analysis_type == kind
            && (analysis.dc_op.is_some() || analysis.device_op.is_some())
    })
}

/// A run as the bar names it: by its number, and its label unless the label
/// already starts with that number.
fn run_label(run: &crate::state::SimulationRun) -> String {
    let name = format!("Run {}", run.id);
    let label = run.label.trim();
    if label.is_empty() {
        name
    } else if label
        .strip_prefix(&name)
        .is_some_and(|rest| !rest.starts_with(|next: char| next.is_ascii_digit()))
    {
        label.to_owned()
    } else {
        format!("{name} \u{b7} {label}")
    }
}

/// The other runs whose operating point the shown one can be compared with.
fn baseline_candidates(state: ResultsReads<'_>) -> Vec<(crate::product::DatasetId, String)> {
    let simulation = &state.model.simulation;
    let (Some(active), Some(kind)) = (
        simulation.active_run(),
        simulation
            .active_analysis()
            .map(|analysis| analysis.analysis_type),
    ) else {
        return Vec::new();
    };
    simulation
        .retained
        .runs
        .iter()
        .filter(|run| run.dataset_id != active.dataset_id)
        .filter(|run| baseline_analysis(run, kind).is_some())
        .map(|run| (run.dataset_id, run_label(run)))
        .collect()
}

/// The sheet's controls in its bar: the operating point to compare with.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    if selected_op_evidence(state.results_reads()).is_none() {
        return false;
    }
    let candidates = baseline_candidates(state.results_reads());
    viewer::domain_bar(ui, &candidates, &mut state.results.session.op_compare)
}

/// The plan of the analysis on display, built once however many surfaces ask.
fn active_plan(state: &mut ResultsParts<'_>) -> Option<Arc<OpPlan>> {
    let key = state.model.simulation.active_run().and_then(|run| {
        state
            .model
            .simulation
            .active_analysis()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
    });
    key.and_then(|key| op_plan(state, key))
}
struct OpMapping<'a> {
    state: ResultsReads<'a>,
    notations: Option<BusNotations>,
}
impl viewer::SchematicMapping for OpMapping<'_> {
    fn display_node<'a>(&self, name: &'a str) -> Cow<'a, str> {
        self.notations
            .as_ref()
            .map_or(Cow::Borrowed(name), |notations| notations.display(name))
    }
    fn display_element<'a>(&self, name: &'a str) -> Cow<'a, str> {
        self.notations
            .as_ref()
            .map_or(Cow::Borrowed(name), |notations| {
                notations.display_element(name)
            })
    }
    fn node_available(&self, name: &str) -> bool {
        node_target_available(self.state, name)
    }
    fn device_target(&self, name: &str) -> Option<u64> {
        device_target(self.state, name)
    }
}
/// The row the reader picked, kept true to the data under the sheet: moved
/// if a new solve renumbered it, dropped if the row is gone.
fn reconciled_selection(state: &mut ResultsParts<'_>) -> Option<viewer::OpSelection> {
    let selected = state.results.session.op_selected.as_ref()?;
    let analysis = state.model.simulation.active_analysis();
    let resolved = selected.resolved(
        analysis.and_then(|analysis| analysis.dc_op.as_ref()),
        analysis.and_then(|analysis| analysis.device_op.as_ref()),
    );
    if resolved.as_ref() != Some(selected) {
        state.results.session.op_selected = resolved.clone();
    }
    resolved
}
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let Some(evidence) = selected_op_evidence(state.results_reads()) else {
        let absence = match state.model.simulation.active_analysis() {
            Some(_) => OpAbsence::OtherAnalysis,
            None => OpAbsence::Missing,
        };
        viewer::show_absent(ui, absence);
        return;
    };
    let retains_values = state
        .model
        .simulation
        .active_analysis()
        .is_some_and(|analysis| analysis.dc_op.is_some() || analysis.device_op.is_some());
    if !retains_values {
        viewer::show_absent(ui, OpAbsence::NoValues);
        return;
    }

    let plan = active_plan(state);
    let selection = reconciled_selection(state);
    let analysis = state.model.simulation.active_analysis();
    let dc = analysis.and_then(|analysis| analysis.dc_op.as_ref());
    let devices = analysis.and_then(|analysis| analysis.device_op.as_ref());
    let notations = dc
        .filter(|dc| !dc.node_voltages.is_empty())
        .filter(|_| {
            plan.as_ref()
                .is_some_and(|plan| plan.display.node_shown() > 0)
        })
        .map(|_| bus_notations(&state.model.workspace, &state.model.schematic));
    let failure_sites = operational_state::offered_failure_sites(state);
    let mapping = OpMapping {
        state: state.results_reads(),
        notations,
    };
    let response = viewer::show(
        ui,
        plan.as_ref().map(|plan| &plan.display),
        &viewer::OpView {
            evidence: &evidence,
            dc,
            devices,
            selection: selection.as_ref(),
            mapping: &mapping,
            failure_site: failure_sites
                .as_ref()
                .map(operational_state::OfferedFailureSites::control),
        },
    );
    let viewer::OpResponse {
        action,
        clicked_sort,
        selected,
        toggled_section,
        highlight_failure_sites,
    } = response;
    if highlight_failure_sites {
        operational_state::toggle_failure_site_marking(state, failure_sites.as_ref());
    }
    if let Some(action) = action {
        apply_action(state, action);
    }
    if let Some(selected) = selected {
        state.results.session.op_selected = selected;
    }
    if let Some(section) = toggled_section
        && !state.results.session.op_collapsed.remove(&section)
    {
        state.results.session.op_collapsed.insert(section);
    }
    if let Some((section, column)) = clicked_sort {
        let ascending = match state.results.session.op_sort.get(&section) {
            Some((current, ascending)) if current.eq_ignore_ascii_case(&column) => !*ascending,
            // Analog operating-point work generally starts with the largest
            // retained magnitude, so a new quantity sorts descending by
            // absolute value while preserving the retained sign in the table.
            _ => false,
        };
        state
            .results
            .session
            .op_sort
            .insert(section, (column, ascending));
    }
}
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let Some(evidence) = selected_op_evidence(state.results_reads()) else {
        return;
    };
    let plan = active_plan(state);
    let detail = reconciled_selection(state).and_then(|selection| {
        let reads = state.results_reads();
        let analysis = reads.model.simulation.active_analysis();
        viewer::selection_detail(
            &selection,
            analysis.and_then(|analysis| analysis.dc_op.as_ref()),
            analysis.and_then(|analysis| analysis.device_op.as_ref()),
            &OpMapping {
                state: reads,
                notations: Some(bus_notations(
                    &reads.model.workspace,
                    &reads.model.schematic,
                )),
            },
            usize::from(
                reads
                    .presentation_policy()
                    .displayed_significant_digits()
                    .get(),
            ),
        )
    });
    let action = viewer::right_panel(
        ui,
        &evidence,
        result_mapping_is_current(state.results_reads()),
        plan.as_ref().and_then(|plan| plan.display.scope_note()),
        detail.as_ref(),
    );
    if let Some(plan) = plan.as_ref() {
        let baseline = plan.key.compare.and_then(|compare| {
            state
                .model
                .simulation
                .retained
                .runs
                .iter()
                .find(|run| run.dataset_id == compare.baseline)
                .map(run_label)
        });
        viewer::summary_panel(
            ui,
            &plan.display,
            baseline.as_deref(),
            plan.key.significant_digits,
        );
    }
    if let Some(action) = action {
        apply_action(state, action);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::operating_point::DcOpResult;
    fn op_state(nodes: usize, devices: usize) -> ResultsTestHost {
        use crate::state::{AnalysisResult, SimulationRun};

        let (dc, report) = viewer::evidence_fixture(nodes, devices);
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(dc);
        analysis.device_op = Some(report);
        let mut run = SimulationRun::new(3);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state.model.simulation.view.active_analysis_idx = Some(0);
        state
    }
    fn active_key(state: &ResultsTestHost) -> AnalysisPresentationKey {
        let run = state.model.simulation.active_run().expect("retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }
    #[test]
    fn selection_never_falls_back_to_another_analysis() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = ResultsTestHost::default();
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
        state.model.simulation.retained.runs.push(run);
        state.model.simulation.view.active_run_idx = Some(0);

        state.model.simulation.view.active_analysis_idx = Some(1);
        assert!(selected_op_evidence(state.results_reads()).is_none());
        state.model.simulation.view.active_analysis_idx = Some(0);
        assert!(selected_op_evidence(state.results_reads()).is_some());
    }
    #[test]
    fn node_evidence_does_not_depend_on_a_device_report() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = ResultsTestHost::default();
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
        state.model.simulation.retained.runs.push(run);
        state.model.simulation.view.active_run_idx = Some(0);
        state.model.simulation.view.active_analysis_idx = Some(0);

        assert!(selected_op_evidence(state.results_reads()).is_some());
        let dc = state.model.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_ref()
            .expect("retained node voltages");
        assert!(
            state.model.simulation.retained.runs[0].analyses[0]
                .device_op
                .is_none()
        );
        assert_eq!(
            viewer::OpPlan::new(
                viewer::OpControls {
                    filter: "out",
                    sort: &viewer::OpSort::new(),
                    collapsed: &BTreeSet::new(),
                    significant_digits: 7,
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
        let original = op_plan(&mut state.results_parts(), key).unwrap();
        let version = state.model.simulation.view.data_version;
        state.model.simulation.retained.runs[0].analyses[0] =
            op_state(3, 2).model.simulation.retained.runs[0].analyses[0].clone();
        let changed = op_plan(&mut state.results_parts(), key).unwrap();
        assert_eq!(changed.display.node_shown(), 3);
        assert_eq!(changed.display.device_shown(), 2);
        assert_eq!(original.display.node_shown(), 40);
        assert_eq!(original.display.device_shown(), 40);
        assert!(changed.display.node_indices().all(|index| index < 3));
        assert!(changed.display.device_indices().all(|index| index < 2));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state.results_parts());
                right_panel(ui, &mut state.results_parts());
            });
        });
        assert_eq!(state.model.simulation.view.data_version, version);
    }
    #[test]
    fn a_rerun_keeps_the_picked_row_and_a_vanished_row_clears_it() {
        let mut state = op_state(12, 12);
        let picked = state.model.simulation.retained.runs[0].analyses[0]
            .device_op
            .as_ref()
            .expect("a device report")
            .entries[7]
            .name
            .clone();
        state.results.results.session.op_selected = Some(viewer::OpSelection {
            kind: viewer::OpRowKind::Device,
            index: 7,
            name: picked.clone(),
        });

        state.model.simulation.retained.runs[0].analyses[0]
            .device_op
            .as_mut()
            .expect("a device report")
            .entries
            .swap(0, 7);
        let moved = reconciled_selection(&mut state.results_parts()).expect("the row still exists");
        assert_eq!((moved.index, moved.name.as_str()), (0, picked.as_str()));
        assert_eq!(state.results.results.session.op_selected, Some(moved));

        state.model.simulation.retained.runs[0].analyses[0]
            .device_op
            .as_mut()
            .expect("a device report")
            .entries
            .remove(0);
        assert_eq!(reconciled_selection(&mut state.results_parts()), None);
        assert_eq!(
            state.results.results.session.op_selected, None,
            "a row no longer in the data kept the inspector showing it"
        );
    }
    #[test]
    fn the_readers_controls_are_part_of_the_row_plan_key() {
        let mut state = op_state(40, 40);
        let key = active_key(&state);
        let unfiltered = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert!(Arc::ptr_eq(
            &unfiltered,
            &op_plan(&mut state.results_parts(), key).expect("a row plan")
        ));

        state.results.results.session.op_filter = "m1".to_owned();
        let filtered = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert!(filtered.display.device_shown() < unfiltered.display.device_shown());
        assert!(
            filtered.display.device_shown() > 0,
            "the filter matched nothing, so it proves nothing"
        );

        state.results.results.session.op_filter.clear();
        state
            .results
            .results
            .session
            .op_sort
            .insert("MOSFET".to_owned(), ("gm".to_owned(), true));
        let sorted = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert_ne!(
            sorted.display.device_indices().collect::<Vec<_>>(),
            unfiltered.display.device_indices().collect::<Vec<_>>(),
            "a new sort key served the previous ordering"
        );

        state
            .results
            .results
            .session
            .op_collapsed
            .insert("MOSFET".to_owned());
        let folded = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert_eq!(
            folded.display.device_indices().count(),
            0,
            "folding a section served the previous rows"
        );
    }
    #[test]
    fn a_new_dataset_generation_rebuilds_the_row_plan() {
        let mut state = op_state(8, 8);
        let key = active_key(&state);
        let before = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert_eq!(before.display.node_shown(), 8);

        state.model.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("retained node voltages")
            .node_voltages
            .truncate(3);
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);

        let after = op_plan(&mut state.results_parts(), key).expect("a row plan");
        assert_eq!(
            after.display.node_shown(),
            3,
            "the sheet would have indexed rows the new dataset no longer retains"
        );
        let (rows, offsets) = after.display.row_counts();
        assert_eq!(offsets, rows);
    }
}
