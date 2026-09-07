//! Event readers must resolve current evidence without losing an unchanged selection.

use super::*;
use crate::io::project_io::ProjectSimulationResults;
use crate::state::{SimulationRunLifecycle, SimulationRunProvenance};

fn state_with(analysis: AnalysisResult) -> AppState {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.simulation.complete_run();
    state
}

fn paint(state: &mut AppState, panel: bool) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            if panel {
                right_panel(ui, state);
            } else {
                show(ui, state);
            }
        });
    });
}

fn pick(
    state: &mut AppState,
    source: EventSelectionSource,
    name: &str,
    point: usize,
) -> DigitalEventSelection {
    paint(state, false);
    let analysis = state.simulation.active_analysis().unwrap();
    let key =
        AnalysisPresentationKey::new(state.simulation.active_run().unwrap().dataset_id, analysis);
    let cache = state.ui.results.event_order_cache.as_ref().unwrap();
    let selection = cache
        .order
        .rows
        .iter()
        .find_map(|entry| {
            let row =
                event_row_from_entry(analysis, &cache.order.buses, BusRadix::Binary, *entry, 0)?;
            (row.source == source && row.trace_name == name && row.point_index == point)
                .then(|| row.selection(key, &cache.order.buses))
                .flatten()
        })
        .expect("the event was rendered and can be selected");
    state.ui.results.selected_digital_event = Some(selection.clone());
    selection
}

#[test]
fn event_source_restoration_refreshes_bus_order_and_raw_codes() {
    let mut state = state_with(super::bus_tests::two_bit_counter());
    paint(&mut state, false);
    assert_eq!(
        state
            .ui
            .results
            .event_order_cache
            .as_ref()
            .unwrap()
            .order
            .rows
            .len(),
        4
    );
    let version = state.simulation.data_version;
    let mut replacement = state.simulation.clone();
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        replacement.runs[0].analyses[0].result_payload.as_mut()
    else {
        panic!("events");
    };
    digital_traces[1].points.pop();
    digital_traces[1].points[1].value_code = 4;
    state.simulation = ProjectSimulationResults::from_state(&replacement)
        .into_simulation_state()
        .unwrap();
    assert_eq!(state.simulation.data_version, version);
    paint(&mut state, false);
    let cache = state.ui.results.event_order_cache.as_ref().unwrap();
    assert_eq!(cache.order.rows.len(), 3);
    assert_eq!(cache.order.buses[0].events[1].1, [Some(0), Some(4)]);
}

fn scalar_state(source: EventSelectionSource) -> (AppState, &'static str) {
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    let name = match source {
        EventSelectionSource::ExactDigital => {
            analysis.result_payload = Some(super::tests::committed_events(&[(0.0, 0), (1.0, 1)]));
            "clk"
        }
        EventSelectionSource::ExactReal => {
            analysis.result_payload = Some(AnalysisResultPayload::TransientEvents {
                digital_traces: vec![],
                digital_buses: vec![],
                real_traces: vec![crate::state::RealEventTraceEvidence {
                    node_name: "level".to_owned(),
                    points: vec![
                        crate::state::RealEventPointEvidence {
                            time_s: 0.0,
                            value: 0.0,
                        },
                        crate::state::RealEventPointEvidence {
                            time_s: 1.0,
                            value: 0.5,
                        },
                    ],
                }],
            });
            "level"
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let name = if source == EventSelectionSource::ProjectedDigital {
                "D(clk)"
            } else {
                "E(level)"
            };
            analysis.waveforms.push(WaveformData::new(
                name,
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#fff",
            ));
            name
        }
        EventSelectionSource::Bus => unreachable!(),
    };
    (state_with(analysis), name)
}

#[test]
fn event_source_scalar_selection_rejects_changed_time_value_or_strength() {
    for source in [
        EventSelectionSource::ExactDigital,
        EventSelectionSource::ExactReal,
        EventSelectionSource::ProjectedDigital,
        EventSelectionSource::ProjectedReal,
    ] {
        for change_time in [false, true] {
            let (mut state, name) = scalar_state(source);
            let selection = pick(&mut state, source, name, 1);
            let analysis = &mut state.simulation.runs[0].analyses[0];
            match source {
                EventSelectionSource::ExactDigital => {
                    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
                        analysis.result_payload.as_mut()
                    else {
                        unreachable!()
                    };
                    if change_time {
                        digital_traces[0].points[1].time_s = 2.0;
                    } else {
                        digital_traces[0].points[1].value_code = 4;
                    }
                }
                EventSelectionSource::ExactReal => {
                    let Some(AnalysisResultPayload::TransientEvents { real_traces, .. }) =
                        analysis.result_payload.as_mut()
                    else {
                        unreachable!()
                    };
                    if change_time {
                        real_traces[0].points[1].time_s = 2.0;
                    } else {
                        real_traces[0].points[1].value = 0.75;
                    }
                }
                _ => {
                    if change_time {
                        std::sync::Arc::make_mut(&mut analysis.waveforms[0].x)[1] = 2.0;
                    } else {
                        std::sync::Arc::make_mut(&mut analysis.waveforms[0].y)[1] = 4.0;
                    }
                }
            }
            assert!(
                event_selection_block(&mut state, &selection).is_some(),
                "{source:?}, change_time={change_time}"
            );
            paint(&mut state, true);
            assert!(state.ui.results.selected_digital_event.is_none());
        }
    }
}

#[test]
fn event_source_inspector_rejects_invalid_evidence_and_allows_repair() {
    let (mut state, name) = scalar_state(EventSelectionSource::ExactDigital);
    let selection = pick(&mut state, EventSelectionSource::ExactDigital, name, 1);
    state.simulation.runs[0].analyses[0]
        .waveforms
        .push(WaveformData::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0],
            "#fff",
        ));
    assert!(
        state.simulation.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    let block = event_selection_block(&mut state, &selection)
        .expect("invalid evidence cannot be inspected");
    assert!(!block.stale, "repair can restore the selected event");
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );
    state.simulation.runs[0].analyses[0].waveforms.clear();
    assert!(event_selection_block(&mut state, &selection).is_none());
}

#[test]
fn event_source_bus_selection_detects_changed_members_before_a_frame() {
    let mut state = state_with(super::bus_tests::two_bit_counter());
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 0);
    let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        state.simulation.runs[0].analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    digital_buses[0].members.reverse();
    assert!(
        state.simulation.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_ok()
    );
    assert!(
        event_selection_block(&mut state, &selection).is_some(),
        "the word is still 00, but its bits now name different members"
    );
    paint(&mut state, true);
    assert!(state.ui.results.selected_digital_event.is_none());
}

#[test]
fn event_source_appended_history_keeps_selection_and_refreshes_inspector_order() {
    let mut state = state_with(super::bus_tests::two_bit_counter());
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        state.simulation.runs[0].analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    digital_traces[1]
        .points
        .push(crate::state::DigitalEventPointEvidence {
            time_s: 20.0e-9,
            value_code: 0,
        });
    assert!(event_selection_block(&mut state, &selection).is_none());
    assert_eq!(
        state
            .ui
            .results
            .event_order_cache
            .as_ref()
            .unwrap()
            .order
            .rows
            .len(),
        5
    );
    state.ui.results.event_bus_radix = BusRadix::Hex;
    state
        .ui
        .results
        .expanded_event_buses
        .insert("count".to_owned());
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );
}

#[test]
fn event_source_navigation_keeps_a_retained_selection() {
    let mut state = state_with(super::bus_tests::two_bit_counter());
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
    let mut other = super::bus_tests::two_bit_counter();
    other.id = 2;
    state.simulation.runs[0].add_analysis(other);
    assert!(state.simulation.select_analysis(1));
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );
    paint(&mut state, false);
    assert!(state.simulation.select_analysis(0));
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );

    let other_run = state_with(super::bus_tests::two_bit_counter())
        .simulation
        .runs[0]
        .clone();
    state.simulation.runs.push(other_run);
    assert!(state.simulation.select_run(1));
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );
    paint(&mut state, false);
    assert!(state.simulation.select_run(0));
    paint(&mut state, true);
    assert_eq!(
        state.ui.results.selected_digital_event.as_ref(),
        Some(&selection)
    );
    state.simulation.runs.remove(0);
    assert!(state.simulation.select_run(0));
    paint(&mut state, true);
    assert!(state.ui.results.selected_digital_event.is_none());
}

#[test]
fn event_source_same_time_selection_survives_trace_reordering() {
    let mut payload = super::tests::committed_events(&[(0.0, 0), (1.0, 1), (1.0, 0), (1.0, 4)]);
    let AnalysisResultPayload::TransientEvents { digital_traces, .. } = &mut payload else {
        unreachable!()
    };
    let mut other = digital_traces[0].clone();
    other.node_name = "other".to_owned();
    digital_traces.push(other);
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(payload),
    );
    let selection = pick(&mut state, EventSelectionSource::ExactDigital, "clk", 2);
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        state.simulation.runs[0].analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    digital_traces.swap(0, 1);
    assert!(event_selection_block(&mut state, &selection).is_none());
    let order = event_order(&mut state).unwrap();
    let event = event_row_for_selection(
        state.simulation.active_analysis().unwrap(),
        &order.buses,
        BusRadix::Binary,
        &selection,
    )
    .unwrap();
    assert_eq!(event.point_index, 2);
    assert_eq!(event.time_s, 1.0);
    assert_eq!(event.value.identity(), 0);
}

#[test]
fn event_source_bus_selection_detects_strength_and_range_changes() {
    for change_range in [false, true] {
        let mut state = state_with(super::bus_tests::two_bit_counter());
        let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
        let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            digital_buses,
            ..
        }) = state.simulation.runs[0].analyses[0].result_payload.as_mut()
        else {
            unreachable!()
        };
        if change_range {
            digital_buses[0].msb = 0;
            digital_buses[0].lsb = 1;
        } else {
            digital_traces[1].points[1].value_code = 4;
        }
        assert!(
            state.simulation.runs[0].analyses[0]
                .validate_retained_evidence()
                .is_ok()
        );
        let order = event_order(&mut state).unwrap();
        assert_eq!(
            bus_word(&order.buses[0].events[1].1, BusRadix::Binary).text,
            "01"
        );
        assert!(
            event_selection_block(&mut state, &selection).is_some(),
            "equal binary text is not equal retained identity"
        );
    }
}

#[test]
fn event_source_large_history_reuses_clones_and_survives_unchanged_restoration() {
    use super::super::frame_work::{DatasetWalk, WorkCounts};

    let points = (0..100_000)
        .map(|index| (f64::from(index), (index % 2) as u8))
        .collect::<Vec<_>>();
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(super::tests::committed_events(&points)),
    );
    let selection = pick(
        &mut state,
        EventSelectionSource::ExactDigital,
        "clk",
        99_999,
    );
    let original = event_order(&mut state).unwrap();
    let mut other = AppState::default();
    other.simulation = state.simulation.clone();
    other.ui.results = state.ui.results.clone();
    let work = WorkCounts::reset();
    for _ in 0..12 {
        assert!(event_selection_block(&mut state, &selection).is_none());
        assert!(event_selection_block(&mut other, &selection).is_none());
        assert!(Arc::ptr_eq(&original, &event_order(&mut other).unwrap()));
    }
    assert_eq!(work.since().total(), 0);
    other.simulation.data_version = other.simulation.data_version.wrapping_add(1);
    let work = WorkCounts::reset();
    assert!(event_selection_block(&mut other, &selection).is_none());
    assert_eq!(work.since().get(DatasetWalk::EventOrder), 1);
    assert!(Arc::ptr_eq(&original, &event_order(&mut state).unwrap()));

    let version = state.simulation.data_version;
    state.simulation = ProjectSimulationResults::from_state(&state.simulation)
        .into_simulation_state()
        .unwrap();
    assert_eq!(state.simulation.data_version, version);
    assert!(event_selection_block(&mut state, &selection).is_none());
    assert!(!Arc::ptr_eq(&original, &event_order(&mut state).unwrap()));
}

#[test]
fn event_source_failed_analysis_is_unavailable_but_live_partial_can_be_inspected() {
    let payload = super::tests::committed_events(&[(0.0, 0), (1.0, 1)]);
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(payload.clone()),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    state.simulation.active_analysis_idx = Some(0);
    let selection = pick(&mut state, EventSelectionSource::ExactDigital, "clk", 1);
    assert!(event_selection_block(&mut state, &selection).is_none());
    state.simulation.runs[0].analyses[0] =
        AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "convergence")
            .with_result_payload(payload);
    assert_eq!(
        event_selection_block(&mut state, &selection).unwrap().note,
        EVENT_SELECTION_INVALID_EVIDENCE
    );
}

#[test]
fn event_source_projected_idle_reads_do_not_rescan_the_trace() {
    use super::super::frame_work::{DatasetWalk, WorkCounts};

    let mut values = vec![0.0; 100_000];
    values[99_999] = 1.0;
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "D(clk)",
                (0..100_000).map(f64::from).collect::<Vec<_>>(),
                values,
                "#fff",
            ),
        ]),
    );
    let work = WorkCounts::reset();
    let selection = pick(
        &mut state,
        EventSelectionSource::ProjectedDigital,
        "D(clk)",
        99_999,
    );
    assert!(work.since().get(DatasetWalk::EventProjectionScan) > 0);
    let work = WorkCounts::reset();
    for _ in 0..12 {
        assert!(active_analysis_is_renderable(&state));
        assert!(event_selection_block(&mut state, &selection).is_none());
        paint(&mut state, false);
        paint(&mut state, true);
    }
    assert_eq!(work.since().total(), 0);
}

#[test]
fn event_source_real_projection_rejects_nan_and_preserves_a_late_initial_value() {
    use super::super::frame_work::WorkCounts;

    let mut values = vec![f64::NAN; 100_000];
    values[99_999] = 0.5;
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "E(level)",
                (0..100_000).map(f64::from).collect::<Vec<_>>(),
                values,
                "#fff",
            ),
        ]),
    );
    assert!(
        state.simulation.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    assert!(!active_analysis_is_renderable(&state));
    assert!(event_order(&mut state).is_none());
    assert!(
        ProjectSimulationResults::from_state(&state.simulation)
            .into_simulation_state()
            .is_err()
    );
    state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
        "E(level)",
        vec![99_999.0],
        vec![0.5],
        "#fff",
    )];
    let selection = pick(
        &mut state,
        EventSelectionSource::ProjectedReal,
        "E(level)",
        0,
    );
    assert!(selection.initial);
    assert_eq!(selection.time_bits, 99_999.0_f64.to_bits());
    let work = WorkCounts::reset();
    for _ in 0..12 {
        assert!(event_selection_block(&mut state, &selection).is_none());
    }
    assert_eq!(work.since().total(), 0);
}
