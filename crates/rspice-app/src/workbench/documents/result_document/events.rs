//! Event source validation, history cache lifetime and selection recovery.

use super::{AnalysisPresentationKey, SheetContext};
use crate::{
    state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, RunHistoryRevision, WaveformData,
    },
    workbench::AppState,
};
use egui::Ui;
use rspice_results::events::projection::EventOrder;
#[cfg(test)]
use rspice_results::events::projection::{
    BusRadix, EventRow, EventSelectionSource, event_row_at_name, event_row_from_entry,
};
use rspice_results_ui::events::{
    self as viewer, DigitalEventSelection, EventAbsence, EventOrigin, SelectionAbsence,
    event_row_for_selection,
};
use std::sync::Arc;
#[cfg(test)]
use viewer::EventRowSelection;
#[cfg(test)]
mod source_tests;
fn active_event_origin(state: &AppState) -> EventOrigin<'_> {
    if let Some(source) = state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.import_source.as_ref())
    {
        EventOrigin::Imported(source)
    } else if state
        .simulation
        .active_analysis()
        .and_then(AnalysisResult::provenance)
        .is_some()
        && state
            .simulation
            .active_run()
            .is_some_and(|run| run.prepared_receipt().is_some())
    {
        EventOrigin::Native
    } else {
        EventOrigin::Unrecorded
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EventOrderCache {
    pub analysis: AnalysisPresentationKey,
    source: (RunHistoryRevision, u64),
    expanded: std::collections::BTreeSet<String>,
    order: Arc<EventOrder>,
}
fn event_order(state: &mut AppState) -> Option<Arc<EventOrder>> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let source = (
        state.simulation.retained.runs.revision(),
        state.simulation.view.data_version,
    );
    let expanded = &state.ui.results.session.expanded_event_buses;
    if let Some(cache) = &state.ui.results.event_order_cache
        && cache.analysis == key
        && cache.source == source
        && &cache.expanded == expanded
    {
        return Some(Arc::clone(&cache.order));
    }
    if !analysis_is_renderable(analysis) || !super::retained_evidence_is_valid(state, key) {
        return None;
    }
    let order = Arc::new(build_event_order(analysis, expanded));
    state.ui.results.event_order_cache = Some(EventOrderCache {
        analysis: key,
        source,
        expanded: expanded.clone(),
        order: Arc::clone(&order),
    });
    Some(order)
}
fn waveform_is_event(waveform: &WaveformData) -> bool {
    rspice_results::events::projection::waveform_is_event(waveform, || {
        super::frame_work::note(super::frame_work::DatasetWalk::EventProjectionScan);
    })
}
pub(super) fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    (analysis.success || analysis.is_live_partial())
        && analysis.analysis_type == AnalysisType::Transient
        && (matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                current_impulses,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty() || current_impulses.is_some()
        ) || analysis.waveforms.iter().any(waveform_is_event))
}
pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    super::analysis_answers_structural_gate(
        state,
        run.dataset_id,
        analysis,
        super::StructuralGate::EventHistory,
    ) && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
}
fn build_event_order(
    analysis: &AnalysisResult,
    expanded: &std::collections::BTreeSet<String>,
) -> EventOrder {
    super::frame_work::note(super::frame_work::DatasetWalk::EventOrder);
    rspice_results::events::projection::build_event_order(analysis, expanded, || {
        super::frame_work::note(super::frame_work::DatasetWalk::EventProjectionScan);
    })
}
#[cfg(test)]
fn event_rows(analysis: &AnalysisResult) -> Vec<EventRow<'_>> {
    build_event_order(analysis, &std::collections::BTreeSet::new())
        .rows()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, entry)| {
            event_row_from_entry(analysis, &[], BusRadix::Binary, entry, index + 1)
        })
        .collect()
}
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some((analysis_key, structurally_renderable)) =
        state.simulation.active_run().and_then(|run| {
            state.simulation.active_analysis().map(|analysis| {
                (
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    super::analysis_answers_structural_gate(
                        state,
                        run.dataset_id,
                        analysis,
                        super::StructuralGate::EventHistory,
                    ),
                )
            })
        })
    else {
        viewer::show_absent(ui, EventAbsence::NoDataset);
        return;
    };
    if !structurally_renderable {
        viewer::show_absent(ui, EventAbsence::NoEvents);
        return;
    }
    if !super::retained_evidence_is_valid(state, analysis_key) {
        viewer::show_absent(ui, EventAbsence::InvalidEvidence);
        return;
    }
    let Some(cache) = event_order(state) else {
        return;
    };
    let analysis = state
        .simulation
        .active_analysis()
        .expect("active analysis resolved above");
    let response = viewer::show(
        ui,
        &viewer::EventView {
            analysis_key,
            analysis: &analysis.data,
            order: &cache,
            origin: active_event_origin(state),
            radix: state.ui.results.session.event_bus_radix,
            expanded: &state.ui.results.session.expanded_event_buses,
            selected: state.ui.results.session.selected_digital_event.as_ref(),
        },
    );
    let viewer::EventResponse {
        requested,
        toggled_bus,
    } = response;
    if let Some(selection) = requested {
        state.ui.results.session.selected_digital_event = Some(selection);
    }
    if let Some(bus) = toggled_bus
        && !state.ui.results.session.expanded_event_buses.remove(&bus)
    {
        state.ui.results.session.expanded_event_buses.insert(bus);
    }
}
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let declares_bus = context
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.result_payload.as_ref())
        .is_some_and(|payload| {
            matches!(
                payload,
                AnalysisResultPayload::TransientEvents { digital_buses, .. }
                    if !digital_buses.is_empty()
            )
        });
    if !declares_bus {
        return false;
    }
    viewer::domain_bar(ui, &mut context.results.session.event_bus_radix);
    true
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventSelectionBlock {
    reason: SelectionAbsence,
    /// Whether the selection itself is unrecoverable and should be dropped.
    /// A dataset that is merely not open right now is not: it can come back,
    /// and dropping the selection would lose the reader's place for a
    /// condition that is about the workspace rather than about the evidence.
    stale: bool,
}
fn event_selection_block(
    state: &mut AppState,
    selection: &DigitalEventSelection,
) -> Option<EventSelectionBlock> {
    let block = |reason, stale| Some(EventSelectionBlock { reason, stale });
    let Some(run) = state.simulation.active_run() else {
        return block(SelectionAbsence::NoDataset, false);
    };
    let Some((analysis_index, _)) = selection.analysis.resolve(run) else {
        if state
            .simulation
            .retained
            .runs
            .iter()
            .any(|run| selection.analysis.resolve(run).is_some())
        {
            return block(SelectionAbsence::OtherDataset, false);
        }
        return block(SelectionAbsence::UnretainedAnalysis, true);
    };
    if state.simulation.view.active_analysis_idx != Some(analysis_index) {
        return block(SelectionAbsence::OtherAnalysis, false);
    }
    let Some(order) = event_order(state) else {
        return block(SelectionAbsence::InvalidEvidence, false);
    };
    let analysis = state.simulation.active_analysis()?;
    if event_row_for_selection(
        analysis,
        order.buses(),
        state.ui.results.session.event_bus_radix,
        selection,
    )
    .is_none()
    {
        return block(SelectionAbsence::UnretainedRow, true);
    }
    None
}
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.session.selected_digital_event.clone() else {
        viewer::selection_absent(ui, SelectionAbsence::Unselected);
        return;
    };
    if let Some(block) = event_selection_block(state, &selection) {
        if block.stale {
            state.ui.results.session.selected_digital_event = None;
        }
        viewer::selection_absent(ui, block.reason);
        return;
    }
    let radix = state.ui.results.session.event_bus_radix;
    let Some(order) = event_order(state) else {
        return;
    };
    let buses = order.buses();
    let Some(event) = state
        .simulation
        .active_run()
        .and_then(|run| selection.analysis.resolve(run))
        .and_then(|(_, analysis)| event_row_for_selection(analysis, buses, radix, &selection))
    else {
        // Unreachable: the block above resolved this exact row over the same
        // unchanged state. Stated rather than asserted, because a panel that
        // says nothing is the defect this function is fixing.
        viewer::selection_absent(ui, SelectionAbsence::UnretainedRow);
        return;
    };

    viewer::right_panel(
        ui,
        &event,
        &selection,
        buses,
        radix,
        active_event_origin(state),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    use viewer::test_support::committed_events;
    #[test]
    fn current_impulses_offer_an_exact_event_sheet_without_digital_nodes() {
        let mut history = crate::state::current_impulse_history_fixture();
        let mut second = history.traces[0].clone();
        second.owner = rspice_core::CurrentImpulseOwner::DeviceLead {
            device_name: "Q1".into(),
            parameter: "ic".into(),
        };
        second.points[0].time = 0.2;
        history.traces.push(second);
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                voltage_impulses: None,
                digital_traces: vec![],
                real_traces: vec![],
                digital_buses: vec![],
                current_impulses: Some(history),
            },
        );
        assert!(analysis_is_renderable(&analysis));
        let order = build_event_order(&analysis, &Default::default());
        assert!(order.exact());
        assert_eq!(order.current_names(), ["I(V1)", "@Q1[ic]"]);
        assert_eq!(order.current_rows(), [(1, 0), (0, 0)]);
        assert!(order.rows().is_empty());
    }
    #[test]
    fn a_running_analysis_offers_the_events_it_has_already_committed() {
        let running = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            running.is_live_partial(),
            "the fixture must be the provisional result the controller publishes"
        );
        assert!(
            analysis_is_renderable(&running),
            "a run still accepting points has committed events worth showing"
        );

        let failed = AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "converge")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            !analysis_is_renderable(&failed),
            "only a live partial joins successful results; a failure stays out"
        );
    }
    #[test]
    fn an_unresolvable_event_selection_is_explained_rather_than_drawn_empty() {
        use crate::state::SimulationRun;

        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let other = AnalysisResult::new(2, AnalysisType::Transient, "TRAN2").with_waveforms(vec![
            WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
        ]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis.clone());
        run.add_analysis(other);
        let dataset_id = run.dataset_id;

        let mut state = AppState::default();

        // No dataset at all: explained, and the selection is kept because the
        // dataset can come back.
        let selection = event_rows(&analysis)[1]
            .selection(AnalysisPresentationKey::new(dataset_id, &analysis), &[])
            .unwrap();
        assert_eq!(
            event_selection_block(&mut state, &selection),
            Some(EventSelectionBlock {
                reason: SelectionAbsence::NoDataset,
                stale: false,
            })
        );

        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        // The row resolves: nothing blocks the inspector.
        assert_eq!(event_selection_block(&mut state, &selection), None);

        // A selection naming an analysis this dataset never retained.
        let mut unretained = selection.clone();
        unretained.analysis = AnalysisPresentationKey::new(
            dataset_id,
            &AnalysisResult::new(99, AnalysisType::Transient, "GONE"),
        );
        assert_eq!(
            event_selection_block(&mut state, &unretained),
            Some(EventSelectionBlock {
                reason: SelectionAbsence::UnretainedAnalysis,
                stale: true,
            })
        );

        // A selection whose trace no longer carries that sample index.
        let mut past_the_end = selection.clone();
        past_the_end.point_index = 97;
        assert_eq!(
            event_selection_block(&mut state, &past_the_end),
            Some(EventSelectionBlock {
                reason: SelectionAbsence::UnretainedRow,
                stale: true,
            })
        );

        // A selection on a retained analysis that is not the active one.
        assert!(state.simulation.select_analysis(1));
        assert_eq!(
            event_selection_block(&mut state, &selection),
            Some(EventSelectionBlock {
                reason: SelectionAbsence::OtherAnalysis,
                stale: false,
            })
        );
    }
}
#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::state::SimulationRun;

    /// Availability fails closed on evidence nobody has validated yet.
    ///
    /// The gate read the workspace validity memo and treated a miss as a
    /// pass, so an analysis whose retained evidence has never been checked —
    /// which is every analysis on the frame it first appears — was offered
    /// regardless of what validation would have said. Its siblings (SOA,
    /// Smith, Optimization) resolve the memo instead of peeking at it, and
    /// therefore close.
    #[test]
    fn never_validated_evidence_is_not_offered_as_an_events_sheet() {
        let payload = AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![crate::state::DigitalEventPointEvidence {
                    time_s: 0.25,
                    value_code: 0,
                }],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        };
        // A waveform with more coordinates than values is exactly what
        // `validate_retained_evidence` exists to refuse.
        let mut corrupt = WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
        corrupt.y = std::sync::Arc::new(vec![0.0]);
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![corrupt])
            .with_result_payload(payload);
        assert!(
            analysis.validate_retained_evidence().is_err(),
            "the fixture has to be invalid for the gate to have anything to refuse"
        );
        assert!(
            analysis_is_renderable(&analysis),
            "the fixture has to be renderable so only validity can close the gate"
        );

        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(
            !active_analysis_is_renderable(&state),
            "an unvalidated, invalid analysis was offered its sheet"
        );
    }
}
