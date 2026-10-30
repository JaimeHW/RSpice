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
    state
        .results
        .results
        .session
        .add_marker(analysis_key, waveform.clone(), "V(a)".to_owned(), 0.5)
        .unwrap();

    state.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .swap(0, 1);
    assert_eq!(
        state.results.results.session.table.columns[0].source_name, "V(a)",
        "the selected table column must not become the new waveform at slot zero"
    );
    assert_eq!(state.results.results.session.markers[0].anchor, waveform);
    assert_eq!(
        state.results.results.session.markers[0]
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

    assert!(viewer_availability(state.results_reads(), ResultViewer::Waves).available);
    assert!(viewer_availability(state.results_reads(), ResultViewer::DcSweep).available);

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
            RetainedTrace::new(
                "Loop Gain (dB)",
                vec![1.0, 10.0],
                vec![40.0, 0.0],
                "#00aaff",
            )
            .with_unit("dB"),
            RetainedTrace::new(
                "Loop Phase (deg)",
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

/// The specs tab speaks for the run the sheet reads, which is the active one.
///
/// `specs::show`, its right panel and the hardcopy projection all resolve
/// `active_run()`. Offering the tab because *some* retained run measured
/// something put the reader on a sheet that then reported no measured
/// results, with the evidence sitting in a run they are not looking at.
#[test]
fn the_specs_tab_speaks_for_the_active_run_and_not_the_history() {
    let mut measured = SimulationRun::new(1);
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    analysis.measurements = vec![rspice_core::MeasureResult::success("trise", 1.0e-9)];
    measured.add_analysis(analysis);

    let mut unmeasured = SimulationRun::new(2);
    unmeasured.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );

    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![measured, unmeasured].into();
    assert!(state.model.workspace.content.specs.is_empty());

    assert!(state.model.simulation.select_run(0));
    assert!(
        viewer_availability(state.results_reads(), ResultViewer::Specs).available,
        "the run the sheet reads holds a measurement"
    );

    assert!(state.model.simulation.select_run(1));
    assert!(
        !viewer_availability(state.results_reads(), ResultViewer::Specs).available,
        "the tab was offered for a measurement in a run the sheet never reads"
    );
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

#[test]
fn result_tabs_follow_the_upgraded_mockup_mode_order() {
    assert_eq!(
        ResultViewer::all()
            .filter(|viewer| viewer.viewer_document_id().is_some())
            .collect::<Vec<_>>(),
        [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::NoiseContrib,
            ResultViewer::Nyquist,
            ResultViewer::Fft,
            ResultViewer::HarmonicBalance,
            ResultViewer::PhaseNoise,
            ResultViewer::Smith,
            ResultViewer::Polar,
            ResultViewer::NetworkMatrix,
            ResultViewer::TransferFunction,
            ResultViewer::Contribution,
            ResultViewer::Op,
            ResultViewer::Specs,
            ResultViewer::Table,
            ResultViewer::Hist,
            ResultViewer::Scatter,
            ResultViewer::BoxViolin,
            ResultViewer::Eye,
            ResultViewer::PoleZero,
            ResultViewer::Events,
            ResultViewer::Soa,
            ResultViewer::Optimization,
        ]
    );
}

#[test]
fn specification_authoring_is_reachable_before_the_first_dataset() {
    assert!(!viewer_requires_retained_results(ResultViewer::Specs));
    assert!(!viewer_requires_retained_results(ResultViewer::Manifest));
    assert!(viewer_requires_retained_results(ResultViewer::Waves));
    assert!(viewer_requires_retained_results(ResultViewer::Op));

    let mut state = ResultsTestHost::default();
    state.results.results.session.viewer = ResultViewer::Specs;
    specs::open_editor(&mut state.results_parts());
    assert!(result_stage_bar_visible(state.results_reads()));
}

#[test]
fn structured_result_controls_have_a_reachable_40_px_strip() {
    for viewer in [ResultViewer::Op, ResultViewer::Specs, ResultViewer::Table] {
        assert!(viewer_has_structured_strip(viewer), "{viewer:?}");
        assert!(viewer_has_sheet_bar(viewer), "{viewer:?}");
    }
    for viewer in [ResultViewer::TransferFunction, ResultViewer::Manifest] {
        assert!(!viewer_has_structured_strip(viewer), "{viewer:?}");
        assert!(!viewer_has_sheet_bar(viewer), "{viewer:?}");
    }
    for viewer in [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::NoiseContrib,
        ResultViewer::Contribution,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::Polar,
        ResultViewer::PoleZero,
        ResultViewer::Scatter,
        ResultViewer::BoxViolin,
    ] {
        assert!(!viewer_has_structured_strip(viewer), "{viewer:?}");
        assert!(viewer_has_sheet_bar(viewer), "{viewer:?}");
    }
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

const MOCKUP_FAMILIES: [&str; 8] = [
    "Waveform worksheet",
    "Frequency & stability",
    "RF & network",
    "Statistics & yield",
    "Digital & AMS events",
    "Fields & physical",
    "Photonics",
    "Report page",
];

#[test]
fn persistent_document_families_scope_to_their_own_plot_types() {
    assert!(family_allows_viewer(
        "Waveform worksheet",
        ResultViewer::Waves
    ));
    assert!(family_allows_viewer(
        "Waveform worksheet",
        ResultViewer::DcSweep
    ));
    assert!(family_allows_viewer(
        "Waveform worksheet",
        ResultViewer::Eye
    ));
    assert!(!family_allows_viewer(
        "Waveform worksheet",
        ResultViewer::Smith
    ));

    assert!(family_allows_viewer(
        "Frequency & stability",
        ResultViewer::Bode
    ));
    assert!(family_allows_viewer(
        "Frequency & stability",
        ResultViewer::PoleZero
    ));
    assert!(!family_allows_viewer(
        "Frequency & stability",
        ResultViewer::Hist
    ));

    assert!(family_allows_viewer(
        "Statistics & yield",
        ResultViewer::Contribution
    ));
    assert!(!family_allows_viewer(
        "Statistics & yield",
        ResultViewer::Nyquist
    ));

    assert!(!family_allows_viewer(
        "Fields & physical",
        ResultViewer::Waves
    ));
    assert!(!family_allows_viewer("Photonics", ResultViewer::Waves));
}

/// Dataset-native sheets carry evidence the bound dataset either has or has
/// not; they are not one family's plot mode, and availability is their only
/// gate. A family that hid them would drop evidence the same dataset shows
/// in the workspace the moment the user promoted the view to a document.
#[test]
fn every_family_offers_the_dataset_native_sheets() {
    for family in MOCKUP_FAMILIES {
        for viewer in ResultViewer::all().filter(|viewer| viewer.viewer_document_id().is_none()) {
            assert!(family_allows_viewer(family, viewer), "{family} {viewer:?}");
        }
    }
}

/// Pages carry their family only as their title, so a renamed or imported
/// page resolves to no family at all. It must fall back to offering every
/// sheet the dataset can feed, never to hiding all of them.
#[test]
fn a_page_outside_the_mockup_families_keeps_every_sheet_reachable() {
    for viewer in ResultViewer::all() {
        assert!(family_allows_viewer("Transient review", viewer));
        assert!(family_allows_viewer("Waveform worksheet · 01", viewer));
    }
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

fn transient_state() -> ResultsTestHost {
    state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    )
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
            .add_marker(
                analysis,
                marker_anchor_for(analysis, name),
                name.to_owned(),
                0.5,
            )
            .unwrap();
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
            .favorite_signals
            .insert(SourceWaveformPresentationKey::new(analysis, name));
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

    assert_eq!(state.results.results.session.markers.len(), 1);
    assert_eq!(state.results.results.session.markers[0].analysis, kept);
    assert_eq!(state.results.results.session.log_y_panes.len(), 1);
    assert_eq!(state.results.results.session.hidden_strips.len(), 1);
    assert_eq!(state.results.results.session.favorite_signals.len(), 1);
    assert_eq!(state.results.results.session.analysis_exprs.len(), 1);
    assert!(
        state
            .results
            .results
            .session
            .analysis_exprs
            .contains_key(&kept)
    );
    assert!(
        state
            .results
            .results
            .project_expression_groups(&state.model.simulation)
            .iter()
            .all(|group| group.analysis == kept),
        "a save must not carry a group about a dataset the project dropped"
    );
}

#[test]
fn retained_history_reconciliation_skips_reads_and_tracks_edits_and_replacement() {
    use super::frame_work::{DatasetWalk, WorkCounts};

    let mut state = transient_state();
    let analysis = active_analysis_key(&state);
    state
        .results
        .results
        .session
        .add_marker(
            analysis,
            marker_anchor_for(analysis, "V(out)"),
            "V(out)".to_owned(),
            0.5,
        )
        .unwrap();

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
    assert_eq!(state.results.results.session.markers.len(), 1);

    // Same run count and sequence, different dataset: no old annotation survives.
    state.model.simulation.retained.runs = transient_state().model.simulation.retained.runs;
    let baseline = WorkCounts::reset();
    state
        .results
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    assert_eq!(baseline.since().get(DatasetWalk::RetainedHistoryScan), 1);
    assert!(state.results.results.session.markers.is_empty());
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

#[test]
fn restored_markers_keep_their_labels_and_advance_the_id_allocator() {
    let mut state = transient_state();
    let key = active_analysis_key(&state);
    let anchor = WaveformPresentationKey {
        analysis: key,
        trace: TracePresentationKey {
            source_name: "V(out)".to_owned(),
            kind: 0,
            family_group: 0,
        },
    };
    let marker = ResultMarker {
        id: 7,
        analysis: key,
        anchor,
        trace_name: "V(out)".to_owned(),
        x: 0.5,
        kind: MarkerKind::Peak,
        note: "settling".to_owned(),
    };

    restore_markers(&mut state.results_parts(), vec![marker], None);

    assert_eq!(state.results.results.session.markers.len(), 1);
    assert_eq!(state.results.results.session.markers[0].note, "settling");
    let next = state
        .results
        .results
        .session
        .add_marker(
            key,
            state.results.results.session.markers[0].anchor.clone(),
            "V(out)".to_owned(),
            0.75,
        )
        .unwrap();
    assert!(
        next > 7,
        "a restored label must not be handed out a second time"
    );
}

/// A marker naming a dataset the reopened project no longer retains has
/// nothing to draw on, and must not be adopted as if it did.
#[test]
fn markers_for_absent_datasets_are_dropped_on_restore() {
    let mut state = transient_state();
    let key = active_analysis_key(&state);
    let foreign = AnalysisPresentationKey::new(
        crate::product::DatasetId::new(),
        &AnalysisResult::new(9, AnalysisType::Transient, "TRAN"),
    );
    let anchor = WaveformPresentationKey {
        analysis: foreign,
        trace: TracePresentationKey {
            source_name: "V(gone)".to_owned(),
            kind: 0,
            family_group: 0,
        },
    };
    let kept = ResultMarker {
        id: 1,
        analysis: key,
        anchor: anchor.clone(),
        trace_name: "V(out)".to_owned(),
        x: 0.1,
        kind: MarkerKind::Note,
        note: String::new(),
    };
    let dropped = ResultMarker {
        id: 2,
        analysis: foreign,
        anchor,
        trace_name: "V(gone)".to_owned(),
        x: 0.1,
        kind: MarkerKind::Note,
        note: String::new(),
    };

    restore_markers(&mut state.results_parts(), vec![kept, dropped], None);

    assert_eq!(state.results.results.session.markers.len(), 1);
    assert_eq!(state.results.results.session.markers[0].id, 1);
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
