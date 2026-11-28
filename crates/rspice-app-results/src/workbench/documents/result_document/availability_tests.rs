//! Sheet-availability, projection-key and result-state lifecycle tests for
//! the Results workspace.

use super::*;
use crate::product::{ContentDigest, ObjectRevision};
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisResultProvenance, AnalysisResultSourceDomain,
    AnalysisType, RetainedTrace, SensitivityResultMode, SensitivityResultRow, SimulationRun,
};
use crate::workbench::documents::result_document::test_fixtures::{
    events_analysis, optimization_analysis, soa_analysis,
};

mod markers;

fn state_with_analysis(analysis: AnalysisResult) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    state
}

#[test]
fn frame_boundary_repairs_a_selector_that_drifted_from_the_open_dataset() {
    let mut first = SimulationRun::new(1);
    first.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Displayed").with_waveforms(vec![
            RetainedTrace::new("V(displayed)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    let displayed_dataset = first.dataset_id;
    let mut background = SimulationRun::new(2);
    background.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "Background").with_waveforms(vec![
            RetainedTrace::new("V(background)", vec![0.0, 1.0], vec![2.0, 3.0], "#ff00aa"),
        ]),
    );

    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![first, background].into();
    assert!(state.model.simulation.select_run(1));
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(displayed_dataset));

    assert!(synchronize_quick_view_dataset_authority(
        &mut state.results_parts()
    ));
    assert_eq!(state.model.simulation.view.active_run_idx, Some(0));
    assert_eq!(
        state
            .model
            .simulation
            .active_run()
            .map(|run| run.dataset_id),
        Some(displayed_dataset)
    );
    assert!(
        !synchronize_quick_view_dataset_authority(&mut state.results_parts()),
        "a stable frame must not churn the selector"
    );
}

#[test]
fn authored_source_matching_accepts_a_distinct_expanded_execution_identity() {
    let authored_source_id = AnalysisInstanceId::new();
    let expanded_execution_id = AnalysisInstanceId::new();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "PVT-expanded TRAN")
        .with_provenance(
            AnalysisResultProvenance::new_with_authored_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                expanded_execution_id,
                authored_source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x57; 32]),
                Vec::new(),
            )
            .expect("expanded provenance is valid"),
        );

    assert_ne!(
        analysis
            .provenance()
            .expect("provenance")
            .source_instance_id(),
        authored_source_id
    );
    assert!(analysis_matches_authored_source(
        &analysis,
        authored_source_id
    ));
}

#[test]
fn presentation_state_follows_analysis_identity_after_reorder() {
    let first = AnalysisResult::new(101, AnalysisType::Transient, "TRAN A").with_waveforms(vec![
        RetainedTrace::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
    ]);
    let second = AnalysisResult::new(202, AnalysisType::Transient, "TRAN B").with_waveforms(vec![
        RetainedTrace::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
    ]);
    let mut state = ResultsTestHost::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(first);
    run.add_analysis(second);
    let dataset_id = run.dataset_id;
    let first_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[0]);
    let second_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[1]);
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    state
        .results
        .results
        .session
        .hidden_strips
        .insert(first_key);
    state.results.results.session.maximized_strip = Some(second_key);
    state.results.results.session.analysis_exprs.insert(
        first_key,
        vec![ExprTrace {
            text: "V(a) * 2".to_owned(),
            complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
            visible: true,
        }],
    );
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, first_key, 0)
        .x = Some((0.25, 0.75));
    state.results.results.session.table.analysis = Some(second_key);

    state.model.simulation.retained.runs[0].analyses.swap(0, 1);
    let reordered = state
        .model
        .simulation
        .active_run()
        .expect("active retained run");
    assert_eq!(
        first_key.resolve(reordered).map(|(index, _)| index),
        Some(1)
    );
    assert_eq!(
        second_key.resolve(reordered).map(|(index, _)| index),
        Some(0)
    );
    assert!(
        state
            .results
            .results
            .session
            .hidden_strips
            .contains(&first_key)
    );
    assert_eq!(
        state.results.results.session.maximized_strip,
        Some(second_key)
    );
    assert_eq!(
        state.results.results.session.analysis_exprs[&first_key][0].text,
        "V(a) * 2"
    );
    let expression = ResultExpressionPresentationKey::new(first_key, "V(a) * 2");
    state
        .results
        .results
        .toggle_expression_visibility_by_key(&state.model.simulation, &expression)
        .expect("stable expression identity resolves after analysis reorder");
    assert!(!state.results.results.session.analysis_exprs[&first_key][0].visible);
    assert_eq!(
        state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, first_key, 0)
            .x,
        Some((0.25, 0.75))
    );
    assert_eq!(
        state.results.results.session.table.analysis,
        Some(second_key)
    );
}

#[test]
fn table_and_marker_waveform_identity_survive_waveform_reorder() {
    let analysis = AnalysisResult::new(101, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
        RetainedTrace::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
    ]);
    let mut state = state_with_analysis(analysis);
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active retained run");
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let trace = TracePresentationKey {
        source_name: "V(a)".to_owned(),
        kind: 0,
        family_group: 0,
    };
    let waveform = WaveformPresentationKey {
        analysis: analysis_key,
        trace: trace.clone(),
    };
    state.results.results.session.table.analysis = Some(analysis_key);
    state.results.results.session.table.columns = vec![trace];
    state.quick_frame();
    state.place_marker("V(a)", 0.5).expect("a marker");

    state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .swap(0, 1);
    state.quick_frame();
    assert_eq!(
        state.results.results.session.table.columns[0].source_name, "V(a)",
        "the selected table column must not become the new waveform at slot zero"
    );
    assert_eq!(
        state.results.results.session.document_markers[0].anchor,
        waveform
    );
    assert_eq!(
        state.results.results.session.document_markers[0]
            .anchor
            .trace
            .source_name,
        "V(a)",
        "the marker must remain attached to its source signal"
    );
}

#[test]
fn selected_trace_identity_fails_closed_after_active_dataset_changes() {
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
    ]);
    let mut state = state_with_analysis(analysis.clone());
    state.results.results.session.selected_trace = Some(
        SelectedResultTrace::from_run_indices(
            state
                .model
                .simulation
                .active_run()
                .expect("active retained run"),
            0,
            0,
        )
        .expect("selected retained trace"),
    );
    assert!(
        state
            .results
            .results
            .valid_selected_trace(&state.model.simulation)
            .is_some()
    );

    state.model.simulation.start_run().add_analysis(analysis);

    assert!(
        state
            .results
            .results
            .valid_selected_trace(&state.model.simulation)
            .is_none()
    );
}

#[test]
fn waveform_data_does_not_enable_incompatible_result_viewers() {
    let state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#00aaff",
            ),
        ]),
    );

    assert!(viewer_availability(state.results_reads(), ResultViewer::Waves).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Bode).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Hist).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Contribution).available);
}

#[test]
fn dc_sweep_is_a_distinct_mockup_viewer_and_waveform_projection() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(time)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.1, 0.9], "#ffbd2e"),
        ]),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    // Each is a view of its own analysis, offered while the browser shows it.
    assert!(viewer_availability(state.results_reads(), ResultViewer::Waves).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::DcSweep).available);
    assert!(state.model.simulation.select_analysis(1));
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Waves).available);
    assert!(viewer_availability(state.results_reads(), ResultViewer::DcSweep).available);
    assert!(state.model.simulation.select_analysis(0));

    state.results.results.session.viewer = ResultViewer::Waves;
    let presentation = state.model.preferences.result_presentation_policy();
    let waves = waves::cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(waves.len(), 1);
    assert_eq!(waves[0].analysis_type(), AnalysisType::Transient);

    state.results.results.session.viewer = ResultViewer::DcSweep;
    let dc = waves::cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(dc.len(), 1);
    assert_eq!(dc[0].analysis_type(), AnalysisType::DcSweep);
}

#[test]
fn ordinary_noise_enables_noise_but_never_substitutes_for_bode() {
    let state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "NOISE").with_waveforms(vec![
            RetainedTrace::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#00aaff"),
        ]),
    );

    assert!(viewer_availability(state.results_reads(), ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Bode).available);
}

#[test]
fn periodic_and_stability_responses_enable_bode_but_pstb_modes_do_not() {
    for analysis_type in [AnalysisType::Pac, AnalysisType::Pxf] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label()).with_waveforms(
                vec![
                    RetainedTrace::new("|V(out)|", vec![1.0, 10.0], vec![10.0, 1.0], "#00aaff"),
                    RetainedTrace::new(
                        "phase(V(out))",
                        vec![1.0, 10.0],
                        vec![-90.0, -135.0],
                        "#ffbd2e",
                    ),
                ],
            ),
        );
        assert!(
            viewer_availability(state.results_reads(), ResultViewer::Bode).available,
            "{} must feed Bode",
            analysis_type.short_label()
        );
    }

    let stb = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Stb, "STB").with_waveforms(vec![
            RetainedTrace::new("Loop Gain", vec![1.0, 10.0], vec![40.0, 0.0], "#00aaff")
                .with_unit("dB"),
            RetainedTrace::new(
                "Loop Phase",
                vec![1.0, 10.0],
                vec![-90.0, -135.0],
                "#ffbd2e",
            )
            .with_unit("°"),
        ]),
    );
    assert!(viewer_availability(stb.results_reads(), ResultViewer::Bode).available);

    let pstb = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Pstb, "PSTB").with_waveforms(vec![
            RetainedTrace::new(
                "Stability Margin (dB)",
                vec![0.0, 1.0],
                vec![12.0, 4.0],
                "#00aaff",
            )
            .with_unit("dB"),
        ]),
    );
    assert!(!viewer_availability(pstb.results_reads(), ResultViewer::Bode).available);
    assert!(viewer_availability(pstb.results_reads(), ResultViewer::Table).available);
}

#[test]
fn pss_and_envelope_feed_waves_while_disto_feeds_unit_aware_frequency_curves() {
    for analysis_type in [AnalysisType::Pss, AnalysisType::Envelope] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label()).with_waveforms(
                vec![RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 1.0e-6],
                    vec![0.0, 1.0],
                    "#00aaff",
                )],
            ),
        );
        assert!(viewer_availability(state.results_reads(), ResultViewer::Waves).available);
    }

    let disto = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_waveforms(vec![
            RetainedTrace::new(
                "V(out) HD3(dBc)",
                vec![1.0e3, 1.0e4],
                vec![-80.0, -60.0],
                "#00aaff",
            )
            .with_unit("dBc"),
        ]),
    );
    assert!(viewer_availability(disto.results_reads(), ResultViewer::Bode).available);
}

#[test]
fn fourier_coefficients_feed_spectrum_and_periodic_sparameters_feed_smith() {
    let fourier = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Fourier, "FOURIER").with_waveforms(vec![
            RetainedTrace::new(
                "|V(out) Spectrum|",
                vec![1.0e3, 2.0e3],
                vec![1.0, 0.1],
                "#00aaff",
            )
            .with_complex_components("V(out) Spectrum", vec![1.0, 0.1], vec![0.0, 0.0]),
        ]),
    );
    assert!(viewer_availability(fourier.results_reads(), ResultViewer::HarmonicBalance).available);

    for analysis_type in [
        AnalysisType::SParameter,
        AnalysisType::Psp,
        AnalysisType::Hbsp,
    ] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label())
                .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                    noise_reference_temperature_kelvin: None,
                    reference_impedances_ohm: vec![75.0, 100.0],
                })
                .with_waveforms(vec![
                    RetainedTrace::new("|S11|", vec![1.0e6, 2.0e6], vec![0.5, 0.25], "#00aaff")
                        .with_complex_components("S11", vec![0.5, 0.25], vec![0.0, -0.1]),
                ]),
        );
        assert!(viewer_availability(state.results_reads(), ResultViewer::Smith).available);
        assert!(viewer_availability(state.results_reads(), ResultViewer::Table).available);
    }
}

#[test]
fn hbnoise_uses_the_noise_density_projection_and_never_impersonates_ac() {
    let analysis = AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE").with_waveforms(vec![
        RetainedTrace::new(
            "onoise",
            vec![1.0e3, 1.0e4],
            vec![1.0e-18, 2.0e-18],
            "#00aaff",
        ),
    ]);
    let state = state_with_analysis(analysis.clone());

    assert!(viewer_availability(state.results_reads(), ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Bode).available);
    assert_eq!(
        project_viewer_for_analysis(ResultViewer::Bode, &analysis),
        ResultViewer::NoiseContrib
    );
}

#[test]
fn op_viewer_requires_the_selected_analysis_device_report() {
    let report = rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: "M1".to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: Vec::new(),
            module: None,
        }],
    };
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP with report").with_device_op(report),
    );
    run.add_analysis(AnalysisResult::new(
        2,
        AnalysisType::DcOp,
        "OP without report",
    ));
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    assert!(state.model.simulation.select_analysis(1));
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Op).available);
    assert!(state.model.simulation.select_analysis(0));
    assert!(viewer_availability(state.results_reads(), ResultViewer::Op).available);
}

/// The OP tab and the OP sheet answer the same question.
///
/// A transient retains its bias solution in `dc_op`, which lit the tab while
/// `op_inspector::selected_op_evidence` refused the analysis outright — an
/// offered sheet that opens onto "not a DC operating-point result". The gate
/// and the sheet now share the predicate, and so does the export refusal.
#[test]
fn the_op_tab_is_offered_only_where_the_op_sheet_renders() {
    let dc = crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.8,
            unit: "V".to_owned(),
        }],
        ..crate::state::DcOpResult::default()
    };

    let transient = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_dc_op(dc.clone()),
    );
    assert!(
        op_inspector::export_csv(
            transient
                .model
                .simulation
                .active_analysis()
                .expect("the fixture selects an analysis")
        )
        .is_none(),
        "the sheet's own projection refuses a transient, so the tab must too"
    );
    assert!(
        !viewer_availability(transient.results_reads(), ResultViewer::Op).available,
        "a transient's retained bias solution lit a sheet that refuses to draw it"
    );

    let operating_point =
        state_with_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(dc));
    assert!(viewer_availability(operating_point.results_reads(), ResultViewer::Op).available);
}

#[test]
fn contribution_viewer_requires_the_active_valid_sensitivity_payload() {
    let payload = AnalysisResultPayload::Sensitivity {
        output: "V(out)".to_owned(),
        result_mode: SensitivityResultMode::Dc,
        rows: vec![SensitivityResultRow {
            parameter: "r1".to_owned(),
            raw: (0.25).into(),
            normalized: (0.5).into(),
        }],
    };
    let state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(payload.clone()),
    );
    assert!(viewer_availability(state.results_reads(), ResultViewer::Contribution).available);

    let mut wrong = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    wrong.result_payload = Some(payload);
    let wrong_analysis = state_with_analysis(wrong);
    assert!(
        !viewer_availability(wrong_analysis.results_reads(), ResultViewer::Contribution).available
    );
}

#[test]
fn incompatible_active_viewer_falls_back_to_compatible_view() {
    let mut state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#00aaff",
            ),
        ]),
    );
    state.results.results.session.viewer = ResultViewer::Hist;

    reconcile_active_viewer(&mut state.results_parts());

    assert_eq!(state.results.results.session.viewer, ResultViewer::Waves);
}

/// A sheet the run cannot fill gives way to the sheet the analysis kind
/// opens on, as the one table of default sheets names it, not to whichever
/// sheet comes first in tab order (Table used to win for every one of these).
#[test]
fn each_analysis_kind_opens_on_the_sheet_the_table_names() {
    let sensitivity = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
        .with_result_payload(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![SensitivityResultRow {
                parameter: "r1".to_owned(),
                raw: (0.25).into(),
                normalized: (0.5).into(),
            }],
        });
    let dc = crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.8,
            unit: "V".to_owned(),
        }],
        ..crate::state::DcOpResult::default()
    };
    let operating_point = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(dc);
    for (analysis, expected) in [
        (sensitivity, ResultViewer::Contribution),
        (operating_point, ResultViewer::Op),
        (soa_analysis(), ResultViewer::Soa),
        (optimization_analysis(), ResultViewer::Optimization),
    ] {
        let kind = analysis.analysis_type;
        assert_eq!(ResultViewer::default_sheet(kind), expected, "{kind:?}");
        let mut state = state_with_analysis(analysis);
        state.results.results.session.viewer = ResultViewer::Bode;

        reconcile_active_viewer(&mut state.results_parts());

        assert_eq!(state.results.results.session.viewer, expected, "{kind:?}");
    }
}

/// The sheet bar names the sheet and what it reads, never the gate's own
/// "is available" wording; an analysis of a finished run that none of its
/// sheets can draw says in one sentence what is missing, while a run still
/// solving says nothing yet.
#[test]
fn the_sheet_bar_names_its_subject_and_what_an_unfilled_sheet_needs() {
    let mut state = transient_state();
    state.results.results.session.viewer = ResultViewer::Waves;
    let purpose = sheet_purpose(state.results_reads());
    assert!(purpose.starts_with("Waves"), "{purpose}");
    assert!(
        !purpose.contains("available") && !purpose.contains("present"),
        "{purpose}"
    );

    // An AC analysis that kept no usable response.
    let mut state = state_with_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC1"));
    state.model.simulation.retained.runs[0].lifecycle =
        crate::state::SimulationRunLifecycle::Completed;
    state.results.results.session.viewer = ResultViewer::Table;
    let purpose = sheet_purpose(state.results_reads());
    assert!(purpose.starts_with("Table · AC1"), "{purpose}");
    assert!(
        purpose.contains(&format!(
            "Bode: {}",
            viewer_availability(state.results_reads(), ResultViewer::Bode).reason
        )),
        "{purpose}"
    );
    state.model.simulation.retained.runs[0].lifecycle =
        crate::state::SimulationRunLifecycle::Running;
    assert!(!sheet_purpose(state.results_reads()).contains("Bode"));
}

/// A running run that has written nothing yet keeps the reader on the sheet its results
/// arrive on, so a transient's live plot appears where the reader is looking, and a run
/// that ended with nothing to show stays there too: the sheet says what happened, and the
/// run's inventory is in Run details.
#[test]
fn a_run_that_has_written_nothing_yet_keeps_the_readers_sheet() {
    let mut state = ResultsTestHost::default();
    let sequence = state.model.simulation.start_run().id;
    let set_lifecycle = |state: &mut ResultsTestHost, lifecycle| {
        state
            .model
            .simulation
            .retained
            .run_by_sequence_mut(sequence)
            .expect("the run is retained")
            .lifecycle = lifecycle;
    };

    set_lifecycle(&mut state, crate::state::SimulationRunLifecycle::Running);
    reconcile_active_viewer(&mut state.results_parts());
    assert_eq!(state.results.results.session.viewer, ResultViewer::Waves);

    set_lifecycle(&mut state, crate::state::SimulationRunLifecycle::Failed);
    reconcile_active_viewer(&mut state.results_parts());
    assert_eq!(state.results.results.session.viewer, ResultViewer::Waves);
}

#[test]
fn stage_bar_stands_down_without_a_retained_dataset() {
    let empty = ResultsTestHost::default();
    assert_eq!(empty.results.results.session.viewer, ResultViewer::Waves);
    assert!(!result_stage_bar_visible(empty.results_reads()));

    let mut allocated = ResultsTestHost::default();
    allocated.model.simulation.start_run();
    assert!(!result_stage_bar_visible(allocated.results_reads()));

    let retained = state_with_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    assert!(result_stage_bar_visible(retained.results_reads()));
}

#[test]
fn hidden_wave_strips_make_the_instrument_restore_control_reachable() {
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    let mut state = state_with_analysis(analysis);
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active retained run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.results.results.session.hidden_strips.insert(key);

    let mut other_run = SimulationRun::new(2);
    other_run.add_analysis(AnalysisResult::new(
        2,
        AnalysisType::Transient,
        "Other TRAN",
    ));
    let other_key = AnalysisPresentationKey::new(
        other_run.dataset_id,
        other_run.analyses.first().expect("other analysis"),
    );
    state.model.simulation.retained.runs.push(other_run);
    state
        .results
        .results
        .session
        .hidden_strips
        .insert(other_key);

    assert_eq!(hidden_wave_strip_count(state.results_reads()), 1);
}

/// Every campaign analysis the Simulate catalog can launch must land on a
/// sheet that draws it. These four sheets shipped in the repository for
/// months with no `mod` declaration, so SOA, optimizer and event
/// runs produced results the Results workspace could not show at all.
#[test]
fn every_campaign_analysis_reaches_a_sheet_that_can_draw_it() {
    for (analysis, viewer) in [
        (soa_analysis(), ResultViewer::Soa),
        (optimization_analysis(), ResultViewer::Optimization),
        (events_analysis(), ResultViewer::Events),
    ] {
        assert!(
            analysis.validate_retained_evidence().is_ok(),
            "{viewer:?} fixture is not valid evidence"
        );
        let state = state_with_analysis(analysis);
        assert!(
            viewer_availability(state.results_reads(), viewer).available,
            "{viewer:?} is unreachable"
        );
        // The sheet's own evidence gate must agree with the tab that
        // offered it, or the tab opens onto a refusal.
        let key = active_analysis_key(&state);
        assert!(
            retained_evidence_is_valid(state.results_reads(), key),
            "{viewer:?}"
        );
    }
}

/// A transient with no event nodes must not grow an empty event payload:
/// the tab would light up on a deck that has nothing to show.
#[test]
fn a_transient_without_event_nodes_offers_no_event_sheet() {
    let state = transient_state();
    assert!(!viewer_availability(state.results_reads(), ResultViewer::Events).available);
}

/// Render the real tab strip and read the accessibility tree it publishes.
///
/// The contract is that a sheet gets a tab exactly when the dataset can
/// feed it — so each new sheet has to appear on its own evidence and stay
/// out of the strip on everyone else's. Availability alone would not catch
/// a viewer that is gated correctly but never drawn.
fn rendered_tab_labels(analysis: AnalysisResult) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = state_with_analysis(analysis);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| viewer_tabs(ui, &mut state));
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("the viewer tab strip publishes an accessibility tree")
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect()
}

#[test]
fn each_new_sheet_gets_a_tab_on_its_own_evidence_and_only_then() {
    let cases = [
        (soa_analysis(), "SOA"),
        (optimization_analysis(), "Optimization"),
        (events_analysis(), "Events"),
    ];
    for (analysis, expected) in &cases {
        let labels = rendered_tab_labels(analysis.clone());
        assert!(
            labels.iter().any(|label| label == expected),
            "{expected} has no tab on its own dataset; strip is {labels:?}"
        );
        for (_, other) in cases.iter().filter(|(_, other)| other != expected) {
            assert!(
                !labels.iter().any(|label| label == other),
                "{other} offered itself on the {expected} dataset; strip is {labels:?}"
            );
        }
    }
}

/// Every tab the strip draws opens; the strip filters the rest out rather
/// than painting them disabled. A tab that reported itself unavailable
/// would mean `viewer_tab` had grown a state its only caller excludes.
#[test]
fn every_tab_the_strip_draws_can_be_opened() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = state_with_analysis(soa_analysis());
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| viewer_tabs(ui, &mut state));
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("the viewer tab strip publishes an accessibility tree")
        .nodes;
    let tabs = nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
        .collect::<Vec<_>>();
    assert!(!tabs.is_empty(), "the strip drew no tabs at all");
    for (_, node) in tabs {
        assert!(
            !node.is_disabled(),
            "the strip drew a disabled tab: {:?}",
            node.label()
        );
    }
}

/// The accessibility nodes the quick view publishes, on the frame after the
/// one that settles its sheet.
fn quick_view_nodes(state: &mut ResultsTestHost) -> Vec<egui::accesskit::Node> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut controller = SimulationController::new();
    let mut output = None;
    for _ in 0..2 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_600.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(ui, &mut state.results_parts(), &mut controller)
                });
            },
        ));
    }
    output
        .and_then(|output| output.platform_output.accesskit_update)
        .expect("the quick view publishes an accessibility tree")
        .nodes
        .into_iter()
        .map(|(_, node)| node)
        .collect()
}

/// The labels the quick view's chrome publishes, each with whether it is a
/// tab.
fn quick_view_labels(state: &mut ResultsTestHost) -> Vec<(bool, String)> {
    quick_view_nodes(state)
        .iter()
        .filter_map(|node| {
            let tab = node.role() == egui::accesskit::Role::Tab;
            node.label().map(|label| (tab, label.to_owned()))
        })
        .collect()
}

/// A pole-zero run, whose one view is the pole-zero plane.
fn pole_zero_state() -> ResultsTestHost {
    use crate::state::{ComplexResultValue, PoleZeroRootSetEvidence, PoleZeroSpectrumCertificate};
    let evidence = |count: u64| PoleZeroRootSetEvidence::Qualified {
        certificate: PoleZeroSpectrumCertificate {
            asymptotically_stable: None,
            problem_order: count,
            infinite_count: 0,
            max_backward_error: 1.0e-14,
            qualification_tolerance:
                PoleZeroSpectrumCertificate::canonical_qualification_tolerance(count).unwrap(),
        },
    };
    let root = |real: f64| ComplexResultValue {
        real,
        imaginary: 0.0,
    };
    state_with_analysis(
        AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
            AnalysisResultPayload::PoleZero {
                poles: vec![root(-1.0e3)],
                zeros: vec![root(-4.0e3)],
                pole_evidence: evidence(1),
                zero_evidence: evidence(1),
                gain: Some(2.0),
            },
        ),
    )
}

/// An operating point of two nodes, whose one view is its sheet.
fn operating_point_state() -> ResultsTestHost {
    let value = |name: &str, value: f64| crate::state::OperatingPointValue {
        name: name.to_owned(),
        value,
        unit: "V".to_owned(),
    };
    let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
    analysis.dc_op = Some(crate::state::DcOpResult {
        node_voltages: vec![value("V(in)", 0.72), value("V(out)", 0.9125)],
        branch_currents: Vec::new(),
        power_dissipation: Vec::new(),
    });
    state_with_analysis(analysis)
}

/// Every control of the view strip, and of the sheet bar a strip of one tab
/// folded into, stands centred above the row's rule: its painted rect leaves
/// a whole, equal gap above and below it, as the accessibility tree places
/// it.
#[test]
fn the_strip_and_the_folded_bar_centre_every_control_above_their_rule() {
    use egui::accesskit::Role;
    for (scene, mut state, bar, folded) in [
        ("pole-zero", pole_zero_state(), "Result sheet controls", true),
        (
            "operating point",
            operating_point_state(),
            "Structured result controls",
            true,
        ),
        ("transient", transient_state(), "Result views", false),
    ] {
        let nodes = quick_view_nodes(&mut state);
        let tabs = nodes.iter().filter(|node| node.role() == Role::Tab).count();
        assert_eq!(tabs == 0, folded, "{scene}: the strip folds only for one view");
        let rows: Vec<_> = nodes
            .iter()
            .filter(|node| node.role() == Role::Toolbar && node.label() == Some(bar))
            .collect();
        assert_eq!(rows.len(), 1, "{scene}: one {bar}");
        let row = rows[0].bounds().expect("the row has bounds");
        let controls: Vec<_> = nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.role(),
                    Role::Button
                        | Role::Tab
                        | Role::RadioButton
                        | Role::CheckBox
                        | Role::TextInput
                        | Role::ComboBox
                )
            })
            .filter_map(|node| Some((node.label().unwrap_or_default(), node.bounds()?)))
            .filter(|(_, bounds)| {
                bounds.x0 >= row.x0
                    && bounds.x1 <= row.x1
                    && bounds.y0 >= row.y0
                    && bounds.y1 <= row.y1
            })
            .collect();
        assert!(controls.len() >= 2, "{scene}: {controls:?}");
        for (label, bounds) in controls {
            // The row's last point is its rule.
            let above = bounds.y0 - row.y0;
            let below = row.y1 - 1.0 - bounds.y1;
            assert!(
                above == below && above.fract() == 0.0 && above >= 1.0,
                "{scene}: {label:?} stands {above} below the top and {below} above the rule"
            );
        }
    }
}

/// A dataset with one view to offer draws no strip of one tab: the sheet's
/// one bar carries the document's actions, the row goes to the sheet, and
/// the view is named by its pane header alone. A dataset that feeds two
/// views keeps its strip, with the actions on it once.
#[test]
fn one_view_draws_no_tab_strip_and_its_bar_carries_the_document() {
    let mut state = pole_zero_state();
    let count = |labels: &[(bool, String)], wanted: &str| {
        labels.iter().filter(|(_, label)| label == wanted).count()
    };
    let labels = quick_view_labels(&mut state);
    assert_eq!(state.results.results.session.viewer, ResultViewer::PoleZero);
    assert!(!labels.iter().any(|(tab, _)| *tab), "{labels:?}");
    assert_eq!(
        count(&labels, "PZ"),
        0,
        "the pane header names the view; the bar adds no name: {labels:?}"
    );
    assert_eq!(count(&labels, "Plot properties"), 1, "{labels:?}");
    assert_eq!(
        count(&labels, "New result document…"),
        0,
        "a quick view makes its document from Results' tabs: {labels:?}"
    );

    let mut state = transient_state();
    let labels = quick_view_labels(&mut state);
    let tabs = labels.iter().filter(|(tab, _)| *tab).count();
    assert!(tabs > 1, "two views keep their strip: {labels:?}");
    assert_eq!(count(&labels, "Plot properties"), 1, "{labels:?}");
    assert_eq!(
        count(&labels, "New result document…"),
        0,
        "a quick view makes its document from Results' tabs: {labels:?}"
    );
}

fn transient_state() -> ResultsTestHost {
    state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    )
}

/// An imported run's working view goes with the run: once the history no
/// longer keeps it, nothing can draw the view again.
#[test]
fn an_imported_runs_view_goes_with_the_run() {
    let mut state = transient_state();
    state.quick_frame();
    assert!(state.working_view().is_some());
    let mut second = SimulationRun::new(2);
    second.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![1.0, 0.0], "#ffbd2e"),
        ]),
    );
    state.model.simulation.retained.runs = vec![second].into();
    assert!(state.model.simulation.select_run(0));

    state.quick_frame();

    let views = &state.model.workspace.content.visualization_documents;
    assert_eq!(views.len(), 1, "the removed run's view is forgotten");
    assert_eq!(
        views[0].working_view(),
        Some(
            crate::results::visualization_document::WorkingViewSource::Imported {
                dataset_id: state.model.simulation.retained.runs[0].dataset_id,
            }
        )
    );
}

/// A colour the reader gives a trace is drawn from the frame it is chosen,
/// kept by the view that draws it, and drawn from the view again on the next
/// frame; the run keeps the colour it was solved with.
#[test]
fn a_chosen_trace_colour_is_the_views_and_the_run_keeps_its_own() {
    use crate::results::visualization_document::TraceColor;
    let mut state = transient_state();
    state.quick_frame();
    let analysis = active_analysis_key(&state);
    let orange = TraceColor::from_rgb(0xff, 0x88, 0x00);
    let drawn = |state: &mut ResultsTestHost| {
        let presentation = state.model.preferences.result_presentation_policy();
        let models = waves::cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        models[0]
            .traces
            .iter()
            .find(|trace| trace.name == "V(out)" && !trace.overlay)
            .map(|trace| trace.color)
            .expect("V(out) is drawn")
    };
    let solved = drawn(&mut state);

    set_trace_color(&mut state.results_parts(), analysis, "V(out)", Some(orange))
        .expect("the working view keeps the trace");

    assert_eq!(drawn(&mut state), egui::Color32::from_rgb(0xff, 0x88, 0x00));
    let view = state.working_view().expect("the run's view").clone();
    assert!(
        view.traces()
            .iter()
            .any(|trace| trace.label == "V(out)" && trace.color == Some(orange)),
        "the view keeps the colour"
    );
    assert_eq!(
        state.model.simulation.retained.runs[0].analyses[0].waveforms[0].color, "#ffbd2e",
        "the run keeps the colour it was solved with"
    );
    state.results.results.session.waveform_colors.clear();
    state.quick_frame();
    assert_eq!(
        drawn(&mut state),
        egui::Color32::from_rgb(0xff, 0x88, 0x00),
        "the next frame draws the view's colour"
    );

    set_trace_color(&mut state.results_parts(), analysis, "V(out)", None)
        .expect("the colour is given back");
    assert_eq!(drawn(&mut state), solved);
    assert!(
        set_trace_color(
            &mut state.results_parts(),
            analysis,
            "V(nope)",
            Some(orange)
        )
        .is_err(),
        "a signal the view has no trace of keeps its colour"
    );
}

/// A style and a name the reader gives a trace are drawn from the frame
/// they are given, kept by the view, drawn from it again on the next frame,
/// and stepped back by undo.
#[test]
fn a_trace_style_and_name_are_the_views_and_undo_takes_them_back() {
    use crate::results::visualization_document::{TraceDraw, TraceStyle, TraceWidth};
    let mut state = transient_state();
    state.quick_frame();
    let analysis = active_analysis_key(&state);
    let drawn = |state: &mut ResultsTestHost| {
        let presentation = state.model.preferences.result_presentation_policy();
        let models = waves::cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        models[0]
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name == "V(out)" && !trace.overlay)
            .map(|trace| (trace.name.clone(), trace.style))
            .expect("V(out) is drawn")
    };
    let thick_steps = TraceStyle {
        width: Some(TraceWidth::Thick),
        draw: TraceDraw::Steps,
        ..TraceStyle::default()
    };
    set_trace_style(&mut state.results_parts(), analysis, "V(out)", thick_steps)
        .expect("the working view keeps the trace");
    rename_trace(&mut state.results_parts(), analysis, "V(out)", "Output")
        .expect("the trace is renamed");
    assert_eq!(drawn(&mut state), ("Output".to_owned(), thick_steps));
    state.results.results.session.waveform_looks.clear();
    state.quick_frame();
    assert_eq!(
        drawn(&mut state),
        ("Output".to_owned(), thick_steps),
        "the next frame draws the view's style and name"
    );

    let view = state.working_view().expect("the run's view").id();
    view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
        .expect("the rename is undone");
    state.quick_frame();
    assert_eq!(drawn(&mut state), ("V(out)".to_owned(), thick_steps));
    view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
        .expect("the style is undone");
    state.quick_frame();
    assert_eq!(
        drawn(&mut state),
        ("V(out)".to_owned(), TraceStyle::default())
    );
}

/// The signals the run's view plots, and whether each is drawn.
fn view_plots(state: &ResultsTestHost) -> Vec<(String, bool)> {
    let mut plots: Vec<_> = state
        .working_view()
        .expect("the run's view")
        .traces()
        .iter()
        .map(|trace| (trace.label.clone(), trace.visible))
        .collect();
    plots.sort();
    plots
}

/// A run plots the outputs its plan marks Plot and leaves the others in the
/// browser: on the first run, on a re-run that adds one, and for the outputs
/// a run had not yet when it was first shown while running. An output the
/// reader took off stays off.
#[test]
fn a_run_plots_the_outputs_its_plan_marks_and_a_rerun_adds_new_ones() {
    let output = |name: &str, plot: bool| {
        let mut waveform = RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e");
        waveform.visible = plot;
        waveform
    };
    let mut state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![output("V(out)", true), output("V(dbg)", false)]),
    );
    state.model.simulation.retained.runs[0].lifecycle =
        crate::state::SimulationRunLifecycle::Running;
    state.quick_frame();
    assert_eq!(view_plots(&state), [("V(out)".to_owned(), true)]);

    // The rest of the run arrives, and it finishes.
    let run = state.model.simulation.active_run_mut().expect("the run");
    run.analyses[0].waveforms.push(output("V(late)", true));
    run.lifecycle = crate::state::SimulationRunLifecycle::Completed;
    state.quick_frame();
    assert_eq!(
        view_plots(&state),
        [("V(late)".to_owned(), true), ("V(out)".to_owned(), true)],
        "an output that arrived later is plotted too"
    );
    let run = state.model.simulation.active_run().expect("the run");
    let digest = retained_dataset_digest(state.results_reads(), run);
    assert_eq!(
        state.working_view().unwrap().datasets()[0]
            .binding()
            .content_digest,
        digest,
        "the view reads the finished run"
    );

    let analysis = active_analysis_key(&state);
    state.results.results.session.waveform_visibility.insert(
        SourceWaveformPresentationKey::new(analysis, "V(late)"),
        false,
    );
    state.quick_frame();
    assert_eq!(
        view_plots(&state),
        [("V(late)".to_owned(), false), ("V(out)".to_owned(), true)]
    );
}

fn active_analysis_key(state: &ResultsTestHost) -> AnalysisPresentationKey {
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active retained run");
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

/// Keeping the zoomed window across a re-run is what the viewport store
/// exists for — it is how an engineer compares one parameter tweak with
/// the last. The ordinal-keyed single-canvas sheets always did; the
/// waveform strips keyed themselves by the dataset a run produced, so
/// every re-run minted a new key and threw the reader's window away.
#[test]
fn a_re_run_keeps_the_window_the_reader_zoomed_the_wave_stack_to() {
    let authored = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffbd2e",
        )])
        .with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                crate::product::ObjectRevision::INITIAL,
                crate::product::ContentDigest::from_bytes([7; 32]),
                Vec::new(),
            )
            .expect("analysis provenance"),
        );
    let mut state = state_with_analysis(authored.clone());
    let first = active_analysis_key(&state);
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, first, 0)
        .x = Some((0.25, 0.75));

    // The same authored analysis, solved again into a new dataset.
    let mut rerun = SimulationRun::new(2);
    rerun.add_analysis(authored);
    state.model.simulation.retained.runs.insert(0, rerun);
    assert!(state.model.simulation.select_run(0));
    let second = active_analysis_key(&state);
    assert_ne!(
        first.dataset_id(),
        second.dataset_id(),
        "a re-run mints a new dataset identity"
    );

    assert_eq!(
        state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, second, 0)
            .x,
        Some((0.25, 0.75)),
        "the window follows the analysis, not the one solve of it"
    );
}

/// Retention discards a dataset; every decision the reader made about it
/// is then a statement about nothing. Left in place they accumulate for
/// the session and are written into the project file.
#[test]
fn pruning_a_run_drops_the_presentation_state_that_named_its_dataset() {
    let mut state = transient_state();
    let kept = active_analysis_key(&state);
    let mut discarded_run = SimulationRun::new(2);
    discarded_run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(gone)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    );
    let discarded =
        AnalysisPresentationKey::new(discarded_run.dataset_id, &discarded_run.analyses[0]);
    state.model.simulation.retained.runs.push(discarded_run);
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);

    for (analysis, name) in [(kept, "V(out)"), (discarded, "V(gone)")] {
        state
            .results
            .results
            .session
            .log_y_panes
            .insert(WavePanePresentationKey {
                analysis,
                unit: "V".to_owned(),
            });
        state.results.results.session.hidden_strips.insert(analysis);
        state
            .results
            .results
            .session
            .note_recent_signal(SourceWaveformPresentationKey::new(analysis, name));
        state
            .results
            .results
            .session
            .analysis_exprs
            .insert(analysis, Vec::new());
    }

    // Retention discards the second run.
    state
        .model
        .simulation
        .retained
        .runs
        .retain(|run| run.dataset_id == kept.dataset_id());
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);

    assert_eq!(state.results.results.session.log_y_panes.len(), 1);
    assert_eq!(state.results.results.session.hidden_strips.len(), 1);
    assert_eq!(state.results.results.session.recent_signals.len(), 1);
    assert_eq!(state.results.results.session.analysis_exprs.len(), 1);
    assert!(
        state
            .results
            .results
            .session
            .analysis_exprs
            .contains_key(&kept)
    );
}

#[test]
fn retained_history_reconciliation_skips_reads_and_tracks_edits_and_replacement() {
    use super::frame_work::{DatasetWalk, WorkCounts};

    let mut state = transient_state();
    let analysis = active_analysis_key(&state);
    state.results.results.session.hidden_strips.insert(analysis);

    let baseline = WorkCounts::reset();
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert_eq!(baseline.since().get(DatasetWalk::RetainedHistoryScan), 1);
    let baseline = WorkCounts::reset();
    for _ in 0..12 {
        state
            .results
            .results
            .reconcile_retained_datasets(&state.model.simulation);
    }
    let unchanged = state.model.simulation.clone();
    state
        .results
        .results
        .reconcile_retained_datasets(&unchanged);
    assert_eq!(baseline.since().get(DatasetWalk::RetainedHistoryScan), 0);

    // No manual data-version bump: mutable access to a nested run is enough.
    state.model.simulation.retained.runs[0].analyses[0]
        .label
        .push_str(" edited");
    let baseline = WorkCounts::reset();
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert_eq!(baseline.since().get(DatasetWalk::RetainedHistoryScan), 1);
    assert_eq!(state.results.results.session.hidden_strips.len(), 1);

    // Same run count and sequence, different dataset: no old annotation survives.
    state.model.simulation.retained.runs = transient_state().model.simulation.retained.runs;
    let baseline = WorkCounts::reset();
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert_eq!(baseline.since().get(DatasetWalk::RetainedHistoryScan), 1);
    assert!(state.results.results.session.hidden_strips.is_empty());
}

/// The defect this pins: the waveform sheets key their viewports by
/// analysis, so a fit written against the plot-ordinal store cleared
/// nothing. The pin writes through the production path deliberately —
/// a test that writes the ordinal store proves only that the store works.
#[test]
fn fitting_the_wave_stack_clears_the_viewport_the_sheet_actually_reads() {
    let mut state = transient_state();
    let key = active_analysis_key(&state);
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
        .y = Some((0.0, 1.0));
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 1)
        .x = Some((0.0, 0.5));
    state.results.results.session.active_wave_pane = Some(WavePanePresentationKey {
        analysis: key,
        unit: "V".to_owned(),
    });

    waves::fit_active_strip(&mut state.results_parts());

    for ordinal in 0..2 {
        assert!(
            !state
                .results
                .results
                .session
                .analysis_plot_view_pane(ResultViewer::Waves, key, ordinal)
                .is_zoomed(),
            "pane {ordinal} kept a pinned viewport after the strip was fitted"
        );
    }
}

#[test]
fn fitting_without_an_active_pane_releases_every_strip() {
    let mut state = transient_state();
    let key = active_analysis_key(&state);
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
        .y = Some((0.0, 1.0));
    state.results.results.session.active_wave_pane = None;

    waves::fit_active_strip(&mut state.results_parts());

    assert!(
        !state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, key, 0)
            .is_zoomed()
    );
}

/// Magnifying needs the sheet's retained extents, which only the unit-pane
/// stack exposes. Offering it elsewhere would be an enabled control that
/// cannot act.
#[test]
fn zoom_is_offered_only_where_it_can_be_carried_out() {
    let mut state = transient_state();
    state.results.results.session.viewer = ResultViewer::Waves;
    assert!(zoom_gesture_available(state.results_reads()));
    assert!(fit_gesture_available(state.results_reads()));

    state.results.results.session.viewer = ResultViewer::Fft;
    assert!(!zoom_gesture_available(state.results_reads()));
    assert!(
        fit_gesture_available(state.results_reads()),
        "a single-canvas plot still has a viewport to release"
    );

    state.results.results.session.viewer = ResultViewer::Table;
    assert!(!fit_gesture_available(state.results_reads()));
    assert!(!zoom_gesture_available(state.results_reads()));
}

/// A signal whose name declares no accessor reads in its analysis' own
/// quantity. The calculator used to call every such signal a voltage.
#[test]
fn an_unqualified_signal_reads_in_its_analysis_quantity() {
    assert_eq!(analysis_default_unit(AnalysisType::Noise), "V^2/Hz");
    assert_eq!(analysis_default_unit(AnalysisType::Ac), "dB");
    assert_eq!(analysis_default_unit(AnalysisType::Transient), "V");
    assert_eq!(
        browser_signal_unit(
            "onoise_spectrum",
            None,
            analysis_default_unit(AnalysisType::Noise)
        ),
        "V^2/Hz"
    );
    assert_eq!(
        browser_signal_unit("I(VDD)", None, analysis_default_unit(AnalysisType::Noise)),
        "A",
        "an explicit accessor still wins over the analysis default"
    );
}

/// The offering gate has to fail closed on both of its branches.
///
/// A Bode summary resolves off the retained magnitude and phase vectors
/// alone, so a failed AC solve still produced one — and the raw-curve branch
/// beside it checks `success` while the summary branch did not. The sheet
/// itself refuses a failed solve; the tab strip offered it anyway.
#[test]
fn a_failed_frequency_response_is_not_offered_as_a_bode_sheet() {
    let frequency = vec![1.0, 1.0e6];
    let mut magnitude =
        RetainedTrace::new("|V(out)|", frequency.clone(), vec![40.0, -40.0], "#fff");
    magnitude.visible = true;
    let mut phase = RetainedTrace::new("phase(V(out))", frequency, vec![0.0, -135.0], "#fff");
    phase.visible = true;

    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![magnitude, phase]);
    assert!(
        bode_analysis_is_renderable(&analysis),
        "the fixture has to be renderable before the gate can withhold it"
    );

    analysis.success = false;
    assert!(
        !bode_analysis_is_renderable(&analysis),
        "a failed solve was offered a frequency-response sheet"
    );
}

#[test]
fn transfer_function_result_resolves_the_canonical_xf_viewer_contract() {
    use crate::results::viewer_catalog::{
        ViewerCapabilities, ViewerCompatibility, viewer_compatibility,
    };

    let analysis_ids = [create_document::analysis_manifest_id(AnalysisType::Tf)];
    assert_eq!(analysis_ids, ["xf"]);
    assert_eq!(
        viewer_compatibility(
            "viewer-transfer-function",
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        ),
        ViewerCompatibility::Compatible
    );
}

/// What "feature complete" means for the Results workspace, stated as a
/// gate rather than a claim: a sheet answers for every view the product
/// manifest puts in this release's scope. Publish a release-target row with
/// no sheet behind it and this fails.
#[test]
fn every_release_target_viewer_document_ships_a_renderer() {
    use crate::results::viewer_catalog::{VIEWER_DOCUMENTS, ViewerReleaseClass};

    let mut in_scope = 0;
    for definition in VIEWER_DOCUMENTS {
        if definition.release != ViewerReleaseClass::ReleaseTarget {
            continue;
        }
        in_scope += 1;
        assert!(
            ResultViewer::from_viewer_document_id(definition.id).is_some(),
            "{} is in this release's scope but no sheet draws it",
            definition.id
        );
    }
    assert!(
        in_scope > 0,
        "the catalog declares no view in release scope"
    );
}
