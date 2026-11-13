//! Event readers must resolve current evidence without losing an unchanged selection.

use super::*;

use crate::state::{SimulationRunLifecycle, SimulationRunProvenance};

fn state_with(analysis: AnalysisResult) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.model.simulation.complete_run();
    state
}

fn paint(state: &mut ResultsTestHost, panel: bool) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            if panel {
                right_panel(ui, &mut state.results_parts());
            } else {
                show(ui, &mut state.results_parts());
            }
        });
    });
}

fn panel_text(state: &mut ResultsTestHost) -> String {
    fn collect(shape: &egui::Shape, text: &mut String) {
        match shape {
            egui::Shape::Text(value) => {
                text.push_str(value.galley.text());
                text.push('\n');
            }
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, text)),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| right_panel(ui, &mut state.results_parts()));
        },
    );
    let mut text = String::new();
    for shape in output.shapes {
        collect(&shape.shape, &mut text);
    }
    text
}

#[test]
fn event_source_unattributed_codes_do_not_claim_native_drive_strength() {
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "events")
            .with_result_payload(viewer::test_support::committed_events(&[(0.0, 1)])),
    );
    pick(&mut state, EventSelectionSource::ExactDigital, "clk", 0);
    let text = panel_text(&mut state);
    assert!(text.contains("Selected event"), "{text}");
    assert!(!text.contains("Drive strength"), "{text}");
    assert!(text.contains("Source not recorded"), "{text}");
    assert!(!text.contains("exact committed"), "{text}");
}

#[test]
fn event_source_import_readout_survives_restoration_and_source_changes() {
    use crate::state::{ResultImportFormat, ResultImportSource};
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "events")
            .with_result_payload(viewer::test_support::committed_events(&[(0.0, 1)])),
    );
    pick(&mut state, EventSelectionSource::ExactDigital, "clk", 0);
    for format in [
        ResultImportFormat::Vcd,
        ResultImportFormat::Fst,
        ResultImportFormat::CsvRfc4180,
    ] {
        state.model.simulation.retained.runs[0].analyses[0].import_source =
            Some(ResultImportSource {
                coordinate: None,
                source_name: "renamed.capture".to_owned(),
                format,
            });
        let restored = crate::io::simulation_state_from_results(
            crate::io::capture_simulation_results(&state.model.simulation),
        )
        .unwrap();
        state.model.simulation = restored;
        let text = panel_text(&mut state);
        assert!(text.contains(format.canonical_id()), "{text}");
        assert!(text.contains("renamed.capture"), "{text}");
        assert!(text.contains("Encoded strength"), "{text}");
        assert!(!text.contains("Drive strength"), "{text}");
        if matches!(format, ResultImportFormat::Vcd | ResultImportFormat::Fst) {
            assert!(
                text.contains("Source drive strength was not retained"),
                "{text}"
            );
        }
        assert!(
            state
                .results
                .results
                .session
                .selected_digital_event
                .is_some()
        );
    }
}

#[test]
fn event_source_native_receipt_keeps_the_retained_drive_strength() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResultProvenance, AnalysisResultSourceDomain, CanonicalAnalysisKind,
        PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
    };
    let id = AnalysisInstanceId::new();
    let digest = ContentDigest::from_bytes([0x41; 32]);
    let revision = ObjectRevision::INITIAL;
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: revision,
        prepared_snapshot_digest: digest,
        source_content_digest: digest,
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                id,
                revision,
                Vec::new(),
                CanonicalAnalysisKind::Transient.tag(),
                digest,
            )
            .unwrap(),
        ],
    })
    .unwrap();
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "events")
            .with_result_payload(viewer::test_support::committed_events(&[(0.0, 4)]))
            .with_provenance(
                AnalysisResultProvenance::new(id, revision, digest, Vec::new()).unwrap(),
            ),
    );
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.model.simulation.complete_run();
    pick(&mut state, EventSelectionSource::ExactDigital, "clk", 0);
    let text = panel_text(&mut state);
    assert!(text.contains("Drive strength"), "{text}");
    assert!(text.contains("resistive"), "{text}");
    // A natively solved event is the normal case and names no source.
    assert!(!text.contains("RSpice prepared execution"), "{text}");
    assert!(!text.contains("canonical strengths"), "{text}");
    state.model.simulation.retained.runs[0].analyses[0].import_source =
        Some(crate::state::ResultImportSource {
            coordinate: None,
            source_name: "inconsistent.vcd".to_owned(),
            format: crate::state::ResultImportFormat::Vcd,
        });
    assert!(
        state.model.simulation.retained.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    let text = panel_text(&mut state);
    assert!(!text.contains("Drive strength"), "{text}");
}

fn pick(
    state: &mut ResultsTestHost,
    source: EventSelectionSource,
    name: &str,
    point: usize,
) -> DigitalEventSelection {
    paint(state, false);
    let analysis = state.model.simulation.active_analysis().unwrap();
    let key = AnalysisPresentationKey::new(
        state.model.simulation.active_run().unwrap().dataset_id,
        analysis,
    );
    let cache = state.results.results.event_order_cache.as_ref().unwrap();
    let selection = cache
        .order
        .rows()
        .iter()
        .find_map(|entry| {
            let row =
                event_row_from_entry(analysis, cache.order.buses(), BusRadix::Binary, *entry, 0)?;
            (row.source() == source && row.trace_name() == name && row.point_index() == point)
                .then(|| row.selection(key, cache.order.buses()))
                .flatten()
        })
        .expect("the event was rendered and can be selected");
    state.results.results.session.selected_digital_event = Some(selection.clone());
    selection
}

#[test]
fn event_source_restoration_refreshes_bus_order_and_raw_codes() {
    let mut state = state_with(AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    });
    paint(&mut state, false);
    assert_eq!(
        state
            .results
            .results
            .event_order_cache
            .as_ref()
            .unwrap()
            .order
            .rows()
            .len(),
        4
    );
    let version = state.model.simulation.view.data_version;
    let mut replacement = state.model.simulation.clone();
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        replacement.retained.runs[0].analyses[0]
            .result_payload
            .as_mut()
    else {
        panic!("events");
    };
    digital_traces[1].points.pop();
    digital_traces[1].points[1].value_code = 4;
    state.model.simulation = crate::io::simulation_state_from_results(
        crate::io::capture_simulation_results(&replacement),
    )
    .unwrap();
    assert_eq!(state.model.simulation.view.data_version, version);
    paint(&mut state, false);
    let cache = state.results.results.event_order_cache.as_ref().unwrap();
    assert_eq!(cache.order.rows().len(), 3);
    assert_eq!(cache.order.buses()[0].events()[1].1, [Some(0), Some(4)]);
}

fn scalar_state(source: EventSelectionSource) -> (ResultsTestHost, &'static str) {
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    let name = match source {
        EventSelectionSource::ExactDigital => {
            analysis.result_payload = Some(viewer::test_support::committed_events(&[
                (0.0, 0),
                (1.0, 1),
            ]));
            "clk"
        }
        EventSelectionSource::ExactReal => {
            analysis.result_payload = Some(AnalysisResultPayload::TransientEvents {
                voltage_impulses: None,
                current_impulses: None,
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
            analysis.waveforms.push(RetainedTrace::new(
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
            let analysis = &mut state.model.simulation.retained.runs[0].analyses[0];
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
                event_selection_block(&mut state.results_parts(), &selection).is_some(),
                "{source:?}, change_time={change_time}"
            );
            paint(&mut state, true);
            assert!(
                state
                    .results
                    .results
                    .session
                    .selected_digital_event
                    .is_none()
            );
        }
    }
}

#[test]
fn event_source_inspector_rejects_invalid_evidence_and_allows_repair() {
    let (mut state, name) = scalar_state(EventSelectionSource::ExactDigital);
    let selection = pick(&mut state, EventSelectionSource::ExactDigital, name, 1);
    state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .push(RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0],
            "#fff",
        ));
    assert!(
        state.model.simulation.retained.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    let block = event_selection_block(&mut state.results_parts(), &selection)
        .expect("invalid evidence cannot be inspected");
    assert!(!block.stale, "repair can restore the selected event");
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );
    state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .clear();
    assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
}

#[test]
fn event_source_bus_selection_detects_changed_members_before_a_frame() {
    let mut state = state_with(AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    });
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 0);
    let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        state.model.simulation.retained.runs[0].analyses[0]
            .result_payload
            .as_mut()
    else {
        unreachable!()
    };
    digital_buses[0].members.reverse();
    assert!(
        state.model.simulation.retained.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_ok()
    );
    assert!(
        event_selection_block(&mut state.results_parts(), &selection).is_some(),
        "the word is still 00, but its bits now name different members"
    );
    paint(&mut state, true);
    assert!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .is_none()
    );
}

#[test]
fn event_source_appended_history_keeps_selection_and_refreshes_inspector_order() {
    let mut state = state_with(AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    });
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        state.model.simulation.retained.runs[0].analyses[0]
            .result_payload
            .as_mut()
    else {
        unreachable!()
    };
    digital_traces[1]
        .points
        .push(crate::state::DigitalEventPointEvidence {
            time_s: 20.0e-9,
            value_code: 0,
        });
    assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
    assert_eq!(
        state
            .results
            .results
            .event_order_cache
            .as_ref()
            .unwrap()
            .order
            .rows()
            .len(),
        5
    );
    state.results.results.session.event_bus_radix = BusRadix::Hex;
    state
        .results
        .results
        .session
        .expanded_event_buses
        .insert("count".to_owned());
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );
}

#[test]
fn event_source_navigation_keeps_a_retained_selection() {
    let mut state = state_with(AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    });
    let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
    let mut other = AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    };
    other.id = 2;
    state.model.simulation.retained.runs[0].add_analysis(other);
    assert!(state.model.simulation.select_analysis(1));
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );
    paint(&mut state, false);
    assert!(state.model.simulation.select_analysis(0));
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );

    let other_run = state_with(AnalysisResult {
        data: viewer::test_support::two_bit_counter(),
    })
    .model
    .simulation
    .retained
    .runs[0]
        .clone();
    state.model.simulation.retained.runs.push(other_run);
    assert!(state.model.simulation.select_run(1));
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );
    paint(&mut state, false);
    assert!(state.model.simulation.select_run(0));
    paint(&mut state, true);
    assert_eq!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .as_ref(),
        Some(&selection)
    );
    state.model.simulation.retained.runs.remove(0);
    assert!(state.model.simulation.select_run(0));
    paint(&mut state, true);
    assert!(
        state
            .results
            .results
            .session
            .selected_digital_event
            .is_none()
    );
}

#[test]
fn event_source_same_time_selection_survives_trace_reordering() {
    let mut payload =
        viewer::test_support::committed_events(&[(0.0, 0), (1.0, 1), (1.0, 0), (1.0, 4)]);
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
        state.model.simulation.retained.runs[0].analyses[0]
            .result_payload
            .as_mut()
    else {
        unreachable!()
    };
    digital_traces.swap(0, 1);
    assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
    let order = event_order(&mut state.results_parts()).unwrap();
    let event = event_row_for_selection(
        state.model.simulation.active_analysis().unwrap(),
        order.buses(),
        BusRadix::Binary,
        &selection,
    )
    .unwrap();
    assert_eq!(event.point_index(), 2);
    assert_eq!(event.time_s(), 1.0);
    assert_eq!(event.value().identity(), 0);
}

#[test]
fn event_source_bus_selection_detects_strength_and_range_changes() {
    for change_range in [false, true] {
        let mut state = state_with(AnalysisResult {
            data: viewer::test_support::two_bit_counter(),
        });
        let selection = pick(&mut state, EventSelectionSource::Bus, "count", 1);
        let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            digital_buses,
            ..
        }) = state.model.simulation.retained.runs[0].analyses[0]
            .result_payload
            .as_mut()
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
            state.model.simulation.retained.runs[0].analyses[0]
                .validate_retained_evidence()
                .is_ok()
        );
        let order = event_order(&mut state.results_parts()).unwrap();
        assert_eq!(
            event_row_at_name(
                state.model.simulation.active_analysis().unwrap(),
                order.buses(),
                BusRadix::Binary,
                EventSelectionSource::Bus,
                &selection.trace_name,
                1
            )
            .unwrap()
            .value()
            .display(),
            "01"
        );
        assert!(
            event_selection_block(&mut state.results_parts(), &selection).is_some(),
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
            .with_result_payload(viewer::test_support::committed_events(&points)),
    );
    let selection = pick(
        &mut state,
        EventSelectionSource::ExactDigital,
        "clk",
        99_999,
    );
    let original = event_order(&mut state.results_parts()).unwrap();
    let mut other = ResultsTestHost::default();
    other.model.simulation = state.model.simulation.clone();
    other.results.results = state.results.results.clone();
    let work = WorkCounts::reset();
    for _ in 0..12 {
        assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
        assert!(event_selection_block(&mut other.results_parts(), &selection).is_none());
        assert!(Arc::ptr_eq(
            &original,
            &event_order(&mut other.results_parts()).unwrap()
        ));
    }
    assert_eq!(work.since().total(), 0);
    other.model.simulation.view.data_version =
        other.model.simulation.view.data_version.wrapping_add(1);
    let work = WorkCounts::reset();
    assert!(event_selection_block(&mut other.results_parts(), &selection).is_none());
    assert_eq!(work.since().get(DatasetWalk::EventOrder), 1);
    assert!(Arc::ptr_eq(
        &original,
        &event_order(&mut state.results_parts()).unwrap()
    ));

    let version = state.model.simulation.view.data_version;
    state.model.simulation = crate::io::simulation_state_from_results(
        crate::io::capture_simulation_results(&state.model.simulation),
    )
    .unwrap();
    assert_eq!(state.model.simulation.view.data_version, version);
    assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
    assert!(!Arc::ptr_eq(
        &original,
        &event_order(&mut state.results_parts()).unwrap()
    ));

    // Retention must release a large event order without drawing EVENTS again.
    let cached = Arc::downgrade(&event_order(&mut state.results_parts()).unwrap());
    state
        .model
        .simulation
        .retained
        .runs
        .push(crate::state::SimulationRun::new(99));
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    state
        .model
        .simulation
        .retained
        .runs
        .retain(|run| run.dataset_id == selection.analysis.dataset_id());
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert!(cached.upgrade().is_some(), "a retained order stays cached");
    state.results.results.session.viewer = rspice_results::result_presentation::ResultViewer::Table;
    state.model.simulation.retained.runs.retain(|_| false);
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert_eq!(state.model.simulation.view.data_version, version);
    assert!(
        cached.upgrade().is_none(),
        "discarding the dataset must release its event order without another event frame"
    );
}

#[test]
fn event_source_failed_analysis_is_unavailable_but_live_partial_can_be_inspected() {
    let payload = viewer::test_support::committed_events(&[(0.0, 0), (1.0, 1)]);
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(payload.clone()),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    state.model.simulation.view.active_analysis_idx = Some(0);
    let selection = pick(&mut state, EventSelectionSource::ExactDigital, "clk", 1);
    assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
    state.model.simulation.retained.runs[0].analyses[0] =
        AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "convergence")
            .with_result_payload(payload);
    assert_eq!(
        event_selection_block(&mut state.results_parts(), &selection)
            .unwrap()
            .reason,
        SelectionAbsence::InvalidEvidence
    );
}

#[test]
fn event_source_projected_idle_reads_do_not_rescan_the_trace() {
    use super::super::frame_work::{DatasetWalk, WorkCounts};

    let mut values = vec![0.0; 100_000];
    values[99_999] = 1.0;
    let mut state = state_with(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(
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
        assert!(active_analysis_is_renderable(state.results_reads()));
        assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
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
            RetainedTrace::new(
                "E(level)",
                (0..100_000).map(f64::from).collect::<Vec<_>>(),
                values,
                "#fff",
            ),
        ]),
    );
    assert!(
        state.model.simulation.retained.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    assert!(!active_analysis_is_renderable(state.results_reads()));
    assert!(event_order(&mut state.results_parts()).is_none());
    assert!(
        crate::io::simulation_state_from_results(crate::io::capture_simulation_results(
            &state.model.simulation
        ))
        .is_err()
    );
    state.model.simulation.retained.runs[0].analyses[0].waveforms = vec![RetainedTrace::new(
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
    assert!(selection.initial());
    assert_eq!(selection.time_bits(), 99_999.0_f64.to_bits());
    let work = WorkCounts::reset();
    for _ in 0..12 {
        assert!(event_selection_block(&mut state.results_parts(), &selection).is_none());
    }
    assert_eq!(work.since().total(), 0);
}

/// A transient run's events as a mixed-signal block leaves them: a clock
/// edge every few nanoseconds, a real-valued control level, and charge
/// impulses on a supply and a transistor lead.
fn mixed_event_state() -> ResultsTestHost {
    let mut history = crate::state::current_impulse_history_fixture();
    history.stop_time_s = 100.0e-9;
    history.traces[0].points[0].time = 12.345_678_901_234_567e-9;
    history.traces[0].points[0].charge_coulombs = -2.5e-15;
    let mut lead = history.traces[0].clone();
    lead.owner = rspice_core::CurrentImpulseOwner::DeviceLead {
        device_name: "Q1".into(),
        parameter: "ic".into(),
    };
    lead.points[0].time = 40.0e-9;
    lead.points[0].charge_coulombs = 3.3e-12;
    history.traces.push(lead);
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    analysis.result_payload = Some(AnalysisResultPayload::TransientEvents {
        voltage_impulses: None,
        current_impulses: Some(history),
        digital_traces: vec![crate::state::DigitalEventTraceEvidence {
            node_name: "clk".to_owned(),
            points: [
                (0.0, 0),
                (1.234_567_890_123_456_7e-9, 1),
                (5.0e-9, 0),
                (7.5e-9, 1),
            ]
            .iter()
            .map(
                |(time_s, value_code)| crate::state::DigitalEventPointEvidence {
                    time_s: *time_s,
                    value_code: *value_code,
                },
            )
            .collect(),
        }],
        digital_buses: vec![],
        real_traces: vec![crate::state::RealEventTraceEvidence {
            node_name: "vctl".to_owned(),
            points: vec![
                crate::state::RealEventPointEvidence {
                    time_s: 0.0,
                    value: 0.0,
                },
                crate::state::RealEventPointEvidence {
                    time_s: 2.0e-9,
                    value: 0.123_456_789_012_345_67,
                },
            ],
        }],
    });
    state_with(analysis)
}

/// One frame of the event sheet at 1200 by 760.
fn event_frame(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 760.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        },
    )
}

/// Every run of text a frame painted, with where it was laid out.
fn painted(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// The event and current-impulse tables state times, charges and real
/// values at the reader's significant digits, with their units, as every
/// other result sheet does — not as seventeen-digit exponents. The exact
/// value is a right-click away: Copy exact row writes it in the notation
/// Copied values chose, as the cursor readout's copy does.
#[test]
fn the_event_tables_state_their_numbers_at_the_readers_digits() {
    let mut state = mixed_event_state();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    for _ in 0..2 {
        event_frame(&ctx, &mut state, Vec::new());
    }
    let output = event_frame(&ctx, &mut state, Vec::new());
    let texts = painted(&output);
    let has = |wanted: &str| texts.iter().any(|(text, _)| text == wanted);
    for wanted in [
        "Retained interval: 0 s to 100 ns",
        "12.34568 ns",
        "-2.5 fC",
        "40 ns",
        "3.3 pC",
        "1.234568 ns",
        "7.5 ns",
        "123.4568 m",
        "Time",
        "Signed charge",
    ] {
        assert!(
            has(wanted),
            "the sheet does not state {wanted:?}: {:?}",
            texts.iter().map(|(text, _)| text).collect::<Vec<_>>()
        );
    }
    let raw_exponent = |text: &str| {
        text.as_bytes().windows(3).any(|window| {
            window[0].is_ascii_digit() && window[1] == b'e' && matches!(window[2], b'-' | b'+')
        })
    };
    assert!(
        !texts
            .iter()
            .any(|(text, _)| raw_exponent(text) || text.contains("000000")),
        "a raw exponent is still printed: {texts:?}"
    );

    // Right-click the clock edge's row and copy it exactly.
    let row = texts
        .iter()
        .find(|(text, _)| text == "1.234568 ns")
        .map(|(_, rect)| rect.center())
        .expect("the clock edge's time is drawn");
    let press = |at, button, pressed| egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    event_frame(&ctx, &mut state, vec![egui::Event::PointerMoved(row)]);
    event_frame(
        &ctx,
        &mut state,
        vec![press(row, egui::PointerButton::Secondary, true)],
    );
    event_frame(
        &ctx,
        &mut state,
        vec![press(row, egui::PointerButton::Secondary, false)],
    );
    let menu = event_frame(&ctx, &mut state, Vec::new());
    let item = painted(&menu)
        .into_iter()
        .find(|(text, _)| text == "Copy exact row")
        .map(|(_, rect)| rect.center())
        .expect("the row offers Copy exact row");
    event_frame(&ctx, &mut state, vec![egui::Event::PointerMoved(item)]);
    event_frame(
        &ctx,
        &mut state,
        vec![press(item, egui::PointerButton::Primary, true)],
    );
    let copied = event_frame(
        &ctx,
        &mut state,
        vec![press(item, egui::PointerButton::Primary, false)],
    )
    .platform_output
    .commands
    .into_iter()
    .find_map(|command| match command {
        egui::OutputCommand::CopyText(text) => Some(text),
        _ => None,
    })
    .expect("Copy exact row copies the row");
    assert!(
        copied.contains("1.234567890123456") && copied.contains("clk"),
        "the copy is not exact: {copied:?}"
    );
}

/// The event sheet for review, written under `RSPICE_PICTURES` when it is
/// set.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_event_sheet_picture() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut state = mixed_event_state();
    let canvas = rspice_ui_kit::raster::render_themed(
        crate::ui::Theme::default(),
        egui::vec2(1200.0, 760.0),
        |ui, _| show(ui, &mut state.results_parts()),
    );
    std::fs::write(
        format!("{directory}/event-sheet.png"),
        canvas.png(canvas.height()),
    )
    .expect("the picture is written");
}
