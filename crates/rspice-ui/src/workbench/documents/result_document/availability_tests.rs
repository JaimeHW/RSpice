//! Sheet-availability, projection-key and result-state lifecycle tests for
//! the Results workspace.

use super::*;
use crate::product::{ContentDigest, ObjectRevision};
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisResultProvenance, AnalysisResultSourceDomain,
    AnalysisType, SensitivityResultMode, SensitivityResultRow, SimulationRun, WaveformData,
};
use crate::workbench::app_state::SpecializedViewerCacheProvenance;

fn state_with_analysis(analysis: AnalysisResult) -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));
    state
}

#[test]
fn frame_boundary_repairs_a_selector_that_drifted_from_the_open_dataset() {
    let mut first = SimulationRun::new(1);
    first.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Displayed").with_waveforms(vec![
            WaveformData::new("V(displayed)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    let displayed_dataset = first.dataset_id;
    let mut background = SimulationRun::new(2);
    background.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "Background").with_waveforms(vec![
            WaveformData::new("V(background)", vec![0.0, 1.0], vec![2.0, 3.0], "#ff00aa"),
        ]),
    );

    let mut state = AppState::default();
    state.simulation.runs = vec![first, background];
    assert!(state.simulation.select_run(1));
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(displayed_dataset));

    assert!(synchronize_quick_view_dataset_authority(&mut state));
    assert_eq!(state.simulation.active_run_idx, Some(0));
    assert_eq!(
        state.simulation.active_run().map(|run| run.dataset_id),
        Some(displayed_dataset)
    );
    assert!(
        !synchronize_quick_view_dataset_authority(&mut state),
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
        WaveformData::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
    ]);
    let second = AnalysisResult::new(202, AnalysisType::Transient, "TRAN B").with_waveforms(vec![
        WaveformData::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
    ]);
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(first);
    run.add_analysis(second);
    let dataset_id = run.dataset_id;
    let first_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[0]);
    let second_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[1]);
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    state.ui.results.hidden_strips.insert(first_key);
    state.ui.results.maximized_strip = Some(second_key);
    state.ui.results.analysis_exprs.insert(
        first_key,
        vec![ExprTrace {
            text: "V(a) * 2".to_owned(),
            visible: true,
        }],
    );
    state
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, first_key, 0)
        .x = Some((0.25, 0.75));
    state.ui.results.table.analysis = Some(second_key);

    state.simulation.runs[0].analyses.swap(0, 1);
    let reordered = state.simulation.active_run().expect("active retained run");
    assert_eq!(
        first_key.resolve(reordered).map(|(index, _)| index),
        Some(1)
    );
    assert_eq!(
        second_key.resolve(reordered).map(|(index, _)| index),
        Some(0)
    );
    assert!(state.ui.results.hidden_strips.contains(&first_key));
    assert_eq!(state.ui.results.maximized_strip, Some(second_key));
    assert_eq!(
        state.ui.results.analysis_exprs[&first_key][0].text,
        "V(a) * 2"
    );
    let expression = ResultExpressionPresentationKey::new(first_key, "V(a) * 2");
    state
        .ui
        .results
        .toggle_expression_visibility_by_key(&state.simulation, &expression)
        .expect("stable expression identity resolves after analysis reorder");
    assert!(!state.ui.results.analysis_exprs[&first_key][0].visible);
    assert_eq!(
        state
            .ui
            .results
            .analysis_plot_view_pane(ResultViewer::Waves, first_key, 0)
            .x,
        Some((0.25, 0.75))
    );
    assert_eq!(state.ui.results.table.analysis, Some(second_key));
}

#[test]
fn table_and_marker_waveform_identity_survive_waveform_reorder() {
    let analysis = AnalysisResult::new(101, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
        WaveformData::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
    ]);
    let mut state = state_with_analysis(analysis);
    let run = state.simulation.active_run().expect("active retained run");
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
    state.ui.results.table.analysis = Some(analysis_key);
    state.ui.results.table.columns = vec![trace];
    state
        .ui
        .results
        .add_marker(analysis_key, waveform.clone(), "V(a)".to_owned(), 0.5);

    state.simulation.runs[0].analyses[0].waveforms.swap(0, 1);
    assert_eq!(
        state.ui.results.table.columns[0].source_name, "V(a)",
        "the selected table column must not become the new waveform at slot zero"
    );
    assert_eq!(state.ui.results.markers[0].anchor, waveform);
    assert_eq!(
        state.ui.results.markers[0].anchor.trace.source_name, "V(a)",
        "the marker must remain attached to its source signal"
    );
}

#[test]
fn selected_trace_identity_fails_closed_after_active_dataset_changes() {
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
    ]);
    let mut state = state_with_analysis(analysis.clone());
    state.ui.results.selected_trace = Some(
        SelectedResultTrace::from_run_indices(
            state.simulation.active_run().expect("active retained run"),
            0,
            0,
        )
        .expect("selected retained trace"),
    );
    assert!(
        state
            .ui
            .results
            .valid_selected_trace(&state.simulation)
            .is_some()
    );

    state.simulation.start_run().add_analysis(analysis);

    assert!(
        state
            .ui
            .results
            .valid_selected_trace(&state.simulation)
            .is_none()
    );
}

#[test]
fn waveform_data_does_not_enable_incompatible_result_viewers() {
    let state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#00aaff",
            ),
        ]),
    );

    assert!(viewer_availability(&state, ResultViewer::Waves).available);
    assert!(!viewer_availability(&state, ResultViewer::Bode).available);
    assert!(!viewer_availability(&state, ResultViewer::Hist).available);
    assert!(!viewer_availability(&state, ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(&state, ResultViewer::Contribution).available);
}

#[test]
fn dc_sweep_is_a_distinct_mockup_viewer_and_waveform_projection() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(time)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.1, 0.9], "#ffbd2e"),
        ]),
    );
    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    assert!(viewer_availability(&state, ResultViewer::Waves).available);
    assert!(viewer_availability(&state, ResultViewer::DcSweep).available);

    state.ui.results.viewer = ResultViewer::Waves;
    let presentation = state.ui.preferences.result_presentation_policy();
    let waves = waves::cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(waves.len(), 1);
    assert_eq!(waves[0].analysis_type(), AnalysisType::Transient);

    state.ui.results.viewer = ResultViewer::DcSweep;
    let dc = waves::cached_models(
        &state.simulation,
        &mut state.ui.results,
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
            WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#00aaff"),
        ]),
    );

    assert!(viewer_availability(&state, ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(&state, ResultViewer::Bode).available);
}

#[test]
fn periodic_and_stability_responses_enable_bode_but_pstb_modes_do_not() {
    for analysis_type in [AnalysisType::Pac, AnalysisType::Pxf] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label()).with_waveforms(
                vec![
                    WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![10.0, 1.0], "#00aaff"),
                    WaveformData::new(
                        "phase(V(out))",
                        vec![1.0, 10.0],
                        vec![-90.0, -135.0],
                        "#ffbd2e",
                    ),
                ],
            ),
        );
        assert!(
            viewer_availability(&state, ResultViewer::Bode).available,
            "{} must feed Bode",
            analysis_type.short_label()
        );
    }

    let stb = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Stb, "STB").with_waveforms(vec![
            WaveformData::new(
                "Loop Gain (dB)",
                vec![1.0, 10.0],
                vec![40.0, 0.0],
                "#00aaff",
            )
            .with_unit("dB"),
            WaveformData::new(
                "Loop Phase (deg)",
                vec![1.0, 10.0],
                vec![-90.0, -135.0],
                "#ffbd2e",
            )
            .with_unit("°"),
        ]),
    );
    assert!(viewer_availability(&stb, ResultViewer::Bode).available);

    let pstb = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Pstb, "PSTB").with_waveforms(vec![
            WaveformData::new(
                "Stability Margin (dB)",
                vec![0.0, 1.0],
                vec![12.0, 4.0],
                "#00aaff",
            )
            .with_unit("dB"),
        ]),
    );
    assert!(!viewer_availability(&pstb, ResultViewer::Bode).available);
    assert!(viewer_availability(&pstb, ResultViewer::Table).available);
}

#[test]
fn pss_and_envelope_feed_waves_while_disto_feeds_unit_aware_frequency_curves() {
    for analysis_type in [AnalysisType::Pss, AnalysisType::Envelope] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label()).with_waveforms(
                vec![WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0e-6],
                    vec![0.0, 1.0],
                    "#00aaff",
                )],
            ),
        );
        assert!(viewer_availability(&state, ResultViewer::Waves).available);
    }

    let disto = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_waveforms(vec![
            WaveformData::new(
                "V(out) HD3(dBc)",
                vec![1.0e3, 1.0e4],
                vec![-80.0, -60.0],
                "#00aaff",
            )
            .with_unit("dBc"),
        ]),
    );
    assert!(viewer_availability(&disto, ResultViewer::Bode).available);
}

#[test]
fn fourier_coefficients_feed_spectrum_and_periodic_sparameters_feed_smith() {
    let fourier = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Fourier, "FOURIER").with_waveforms(vec![
            WaveformData::new(
                "|V(out) Spectrum|",
                vec![1.0e3, 2.0e3],
                vec![1.0, 0.1],
                "#00aaff",
            )
            .with_complex_components("V(out) Spectrum", vec![1.0, 0.1], vec![0.0, 0.0]),
        ]),
    );
    assert!(viewer_availability(&fourier, ResultViewer::HarmonicBalance).available);

    for analysis_type in [
        AnalysisType::SParameter,
        AnalysisType::Psp,
        AnalysisType::Hbsp,
    ] {
        let state = state_with_analysis(
            AnalysisResult::new(1, analysis_type, analysis_type.short_label())
                .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                    reference_impedances_ohm: vec![75.0, 100.0],
                })
                .with_waveforms(vec![
                    WaveformData::new("|S11|", vec![1.0e6, 2.0e6], vec![0.5, 0.25], "#00aaff")
                        .with_complex_components("S11", vec![0.5, 0.25], vec![0.0, -0.1]),
                ]),
        );
        assert!(viewer_availability(&state, ResultViewer::Smith).available);
        assert!(viewer_availability(&state, ResultViewer::Table).available);
    }
}

#[test]
fn hbnoise_uses_the_noise_density_projection_and_never_impersonates_ac() {
    let analysis = AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE").with_waveforms(vec![
        WaveformData::new(
            "onoise",
            vec![1.0e3, 1.0e4],
            vec![1.0e-18, 2.0e-18],
            "#00aaff",
        ),
    ]);
    let state = state_with_analysis(analysis.clone());

    assert!(viewer_availability(&state, ResultViewer::NoiseContrib).available);
    assert!(!viewer_availability(&state, ResultViewer::Bode).available);
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
    let mut state = AppState::default();
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    assert!(state.simulation.select_analysis(1));
    assert!(!viewer_availability(&state, ResultViewer::Op).available);
    assert!(state.simulation.select_analysis(0));
    assert!(viewer_availability(&state, ResultViewer::Op).available);
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
                .simulation
                .active_analysis()
                .expect("the fixture selects an analysis")
        )
        .is_none(),
        "the sheet's own projection refuses a transient, so the tab must too"
    );
    assert!(
        !viewer_availability(&transient, ResultViewer::Op).available,
        "a transient's retained bias solution lit a sheet that refuses to draw it"
    );

    let operating_point =
        state_with_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(dc));
    assert!(viewer_availability(&operating_point, ResultViewer::Op).available);
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
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );

    let mut state = AppState::default();
    state.simulation.runs = vec![measured, unmeasured];
    assert!(state.workspace.specs.is_empty());

    assert!(state.simulation.select_run(0));
    assert!(
        viewer_availability(&state, ResultViewer::Specs).available,
        "the run the sheet reads holds a measurement"
    );

    assert!(state.simulation.select_run(1));
    assert!(
        !viewer_availability(&state, ResultViewer::Specs).available,
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
            raw: 0.25,
            normalized: 0.5,
        }],
    };
    let state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(payload.clone()),
    );
    assert!(viewer_availability(&state, ResultViewer::Contribution).available);

    let mut wrong = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    wrong.result_payload = Some(payload);
    let wrong_analysis = state_with_analysis(wrong);
    assert!(!viewer_availability(&wrong_analysis, ResultViewer::Contribution).available);
}

#[test]
fn incompatible_active_viewer_falls_back_to_compatible_view() {
    let mut state = state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#00aaff",
            ),
        ]),
    );
    state.ui.results.viewer = ResultViewer::Hist;

    reconcile_active_viewer(&mut state);

    assert_eq!(state.ui.results.viewer, ResultViewer::Waves);
}

#[test]
fn result_shell_matches_mockup_bar_geometry() {
    let mut tokens = Tokens::default();
    tokens.metrics.ctl_h = 28.0;
    let fine = ResultBarMetrics::resolve(&tokens);
    assert_eq!(fine.viewer_tabs, 41.0);
    assert_eq!(fine.viewer_tab, 30.0);
    assert_eq!(fine.sheet_bar, 31.0);
    assert_eq!(fine.structured_strip, 40.0);
    assert_eq!(fine.instrument_control, 23.0);
}

/// Under a coarse pointer the shell raises every control to a 44 px
/// target, and these rows have to make room for the controls they hold.
///
/// The old fixed geometry meant a 44 px chip was laid out inside a 31 px
/// band on a tablet — taller than the row containing it — while the title
/// bar, drawers, navigator and console all grew correctly around it. The
/// assertion is the containment, not the numbers: a row that merely got
/// bigger is no use if it is still shorter than its own controls.
#[test]
fn result_bars_make_room_for_a_touch_target() {
    let mut tokens = Tokens::default();
    tokens.metrics.ctl_h = 44.0;
    let touch = ResultBarMetrics::resolve(&tokens);

    assert!(touch.viewer_tab >= 44.0, "{}", touch.viewer_tab);
    assert!(
        touch.instrument_control >= 44.0,
        "{}",
        touch.instrument_control
    );
    assert!(
        touch.viewer_tabs >= touch.viewer_tab,
        "the strip must contain its own tab: {} < {}",
        touch.viewer_tabs,
        touch.viewer_tab
    );
    assert!(
        touch.sheet_bar >= touch.instrument_control,
        "the instrument bar must contain its own controls: {} < {}",
        touch.sheet_bar,
        touch.instrument_control
    );
    assert!(
        touch.structured_strip >= touch.instrument_control,
        "the structured strip must contain its own controls: {} < {}",
        touch.structured_strip,
        touch.instrument_control
    );
}

#[test]
fn result_tabs_follow_the_upgraded_mockup_mode_order() {
    assert_eq!(
        ResultViewer::PRIMARY,
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
            ResultViewer::Reliability,
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

    let mut state = AppState::default();
    state.ui.results.viewer = ResultViewer::Specs;
    specs::open_editor(&mut state);
    assert!(result_stage_bar_visible(&state));
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
    let empty = AppState::default();
    assert_eq!(empty.ui.results.viewer, ResultViewer::Waves);
    assert!(!result_stage_bar_visible(&empty));

    let mut allocated = AppState::default();
    allocated.simulation.start_run();
    assert!(!result_stage_bar_visible(&allocated));

    let retained = state_with_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    assert!(result_stage_bar_visible(&retained));
}

#[test]
fn hidden_wave_strips_make_the_instrument_restore_control_reachable() {
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    let mut state = state_with_analysis(analysis);
    let run = state.simulation.active_run().expect("active retained run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.ui.results.hidden_strips.insert(key);

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
    state.simulation.runs.push(other_run);
    state.ui.results.hidden_strips.insert(other_key);

    assert_eq!(hidden_wave_strip_count(&state), 1);
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
        for viewer in ResultViewer::every().filter(|viewer| viewer.viewer_document_id().is_none()) {
            assert!(family_allows_viewer(family, viewer), "{family} {viewer:?}");
        }
    }
}

/// `viewer_document_id` and `from_viewer_document_id` are one map read in
/// two directions, so every answer either gives must agree with the other.
/// Three sheets share `viewer-table`, which is exactly where the second
/// copy of the inverse had drifted onto a different one.
#[test]
fn the_viewer_document_map_agrees_with_itself_in_both_directions() {
    use crate::results::viewer_catalog::VIEWER_DOCUMENTS;

    for viewer in ResultViewer::every() {
        let Some(document_id) = viewer.viewer_document_id() else {
            assert_eq!(
                ResultViewer::from_viewer_document_id(viewer.label()),
                None,
                "{viewer:?} is dataset-native and must not answer to a document id"
            );
            continue;
        };
        assert!(
            VIEWER_DOCUMENTS
                .iter()
                .any(|document| document.id == document_id),
            "{viewer:?} claims {document_id}, which the catalog does not publish"
        );
        let drawn_by = ResultViewer::from_viewer_document_id(document_id)
            .unwrap_or_else(|| panic!("{document_id} has no sheet"));
        assert_eq!(
            drawn_by.viewer_document_id(),
            Some(document_id),
            "{document_id} resolves to {drawn_by:?}, which renders something else"
        );
    }
}

/// Pages carry their family only as their title, so a renamed or imported
/// page resolves to no family at all. It must fall back to offering every
/// sheet the dataset can feed, never to hiding all of them.
#[test]
fn a_page_outside_the_mockup_families_keeps_every_sheet_reachable() {
    for viewer in ResultViewer::every() {
        assert!(family_allows_viewer("Transient review", viewer));
        assert!(family_allows_viewer("Waveform worksheet · 01", viewer));
    }
}

fn soa_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, SoaEvaluationEvidence, SoaParameterEvidence,
        SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    };
    let time = vec![0.0, 1.0e-9];
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
        .with_waveforms(vec![
            WaveformData::new(
                "SOA_VIOLATION_COUNT",
                time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            // The stress history the producer now retains, named exactly as
            // the sheet addresses it.
            WaveformData::new(
                crate::services::safety::soa_stress_waveform_name(
                    "M1",
                    crate::services::safety::SoAParameter::Vds,
                ),
                time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![SoaEvaluationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                // 3.0 V against a 3.3 V limit is inside the warning band,
                // which the payload validator derives rather than trusts.
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            // A non-passing rule must carry the exact event at its worst
            // point; the validator refuses a verdict with nothing behind it.
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        })
}

fn reliability_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
        ReliabilityShiftEvidence, ReliabilityStressEvidence,
    };
    AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0],
        })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: ReliabilityStressEvidence {
                    average_gate_stress_v: 1.1,
                    average_drain_stress_v: 1.8,
                    average_temperature_k: 358.0,
                    duration_s: 1.0e-6,
                },
                checkpoints: vec![
                    ReliabilityCheckpointEvidence {
                        years: 1.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 1.0e-3,
                            mobility_shift: -1.0e-4,
                            drain_source_resistance_shift: 2.0e-3,
                        },
                    },
                    ReliabilityCheckpointEvidence {
                        years: 5.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 4.0e-3,
                            mobility_shift: -4.0e-4,
                            drain_source_resistance_shift: 8.0e-3,
                        },
                    },
                ],
            }],
        })
}

fn optimization_analysis() -> AnalysisResult {
    use crate::state::AnalysisResultFamilyMetadata;
    AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        // The names the optimization runner emits: one OPT_COST trace over
        // the iteration axis, plus OPT_<variable> per design variable.
        .with_waveforms(vec![
            WaveformData::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            WaveformData::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ])
}

/// A committed event history with all three kinds of row the Events sheet
/// draws: a scalar conductor, a real-valued event node, and two declared
/// buses over their own member traces.
///
/// The buses are here rather than in a second fixture because every gate that
/// reads this one is a gate about the sheet — the fit gate below measures the
/// radix bar and the per-row disclosures, which a history declaring no bus
/// does not paint at all.
fn events_analysis() -> AnalysisResult {
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    };
    let trace = |name: &str, points: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence {
                time_s: *time_s,
                value_code: *value_code,
            })
            .collect(),
    };
    let bus = |name: &str| DigitalBusEvidence {
        name: name.to_owned(),
        msb: 1,
        lsb: 0,
        members: vec![format!("{name}[1]"), format!("{name}[0]")],
        source: DigitalBusSourceEvidence::Import,
    };
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
        AnalysisResultPayload::TransientEvents {
            digital_traces: vec![
                trace("clk", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 12)]),
                trace("addr[1]", &[(0.0, 0), (1.0e-9, 1)]),
                trace("addr[0]", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 0)]),
                // One bit the run never states before its first change, so a
                // word the sheet cannot spell as a number is on screen too.
                trace("data[1]", &[(5.0e-10, 2)]),
                trace("data[0]", &[(0.0, 1), (1.0e-9, 12)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2.5e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![bus("addr"), bus("data")],
        },
    )
}

/// Every campaign analysis the Simulate catalog can launch must land on a
/// sheet that draws it. These four sheets shipped in the repository for
/// months with no `mod` declaration, so SOA, ageing, optimizer and event
/// runs produced results the Results workspace could not show at all.
#[test]
fn every_campaign_analysis_reaches_a_sheet_that_can_draw_it() {
    for (analysis, viewer) in [
        (soa_analysis(), ResultViewer::Soa),
        (reliability_analysis(), ResultViewer::Reliability),
        (optimization_analysis(), ResultViewer::Optimization),
        (events_analysis(), ResultViewer::Events),
    ] {
        assert!(
            analysis.validate_retained_evidence().is_ok(),
            "{viewer:?} fixture is not valid evidence"
        );
        let state = state_with_analysis(analysis);
        assert!(
            viewer_availability(&state, viewer).available,
            "{viewer:?} is unreachable"
        );
        // The sheet's own evidence gate must agree with the tab that
        // offered it, or the tab opens onto a refusal.
        let key = active_analysis_key(&state);
        assert!(retained_evidence_is_valid(&state, key), "{viewer:?}");
    }
}

/// A transient with no event nodes must not grow an empty event payload:
/// the tab would light up on a deck that has nothing to show.
#[test]
fn a_transient_without_event_nodes_offers_no_event_sheet() {
    let state = transient_state();
    assert!(!viewer_availability(&state, ResultViewer::Events).available);
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
        (reliability_analysis(), "Ageing"),
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

/// Draw each newly reachable sheet through a real frame and tessellate it.
///
/// Availability only proves the tab lights up. This proves the sheet
/// behind it lays out, paints and meshes — the failure mode these viewers
/// were most exposed to, having never been compiled at all.
#[test]
fn every_newly_reachable_sheet_draws_and_meshes() {
    for (analysis, viewer) in [
        (soa_analysis(), ResultViewer::Soa),
        (reliability_analysis(), ResultViewer::Reliability),
        (optimization_analysis(), ResultViewer::Optimization),
        (events_analysis(), ResultViewer::Events),
    ] {
        let mut state = state_with_analysis(analysis);
        state.ui.results.viewer = viewer;
        if viewer == ResultViewer::Soa {
            // The stress trace is the sheet's only plot; a card that never
            // opens is a card this test never covers.
            state.ui.results.soa_stress_trace_open = true;
            state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
                analysis: active_analysis_key(&state),
                device_id: "M1".to_owned(),
                parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
            });
        }
        let mut app = RSpiceApp::test_instance();
        app.state = state;
        draw_sheet_and_tessellate(&mut app, viewer);
    }
}

/// Every sheet, painted on evidence that sheet declares itself able to
/// draw.
///
/// The fixture has to satisfy the sheet's own `viewer_availability`
/// contract, so this cannot pass by painting an empty state and calling it
/// coverage. Fourteen of the twenty-two sheets had never been through a
/// real frame in any test; their module tests derive and format values
/// without ever painting them, and a value that escapes an axis mapping
/// does not fail an assert — it becomes a vertex at infinity.
#[test]
fn every_available_sheet_draws_the_evidence_it_claims() {
    for viewer in ResultViewer::every() {
        let mut app = app_showing(viewer);
        assert!(
            viewer_availability(&app.state, viewer).available,
            "the {viewer:?} fixture does not satisfy the sheet's own contract: {}",
            viewer_availability(&app.state, viewer).reason
        );
        draw_sheet_and_tessellate(&mut app, viewer);
    }
}

/// A dataset each sheet reports itself able to draw. Built from retained
/// evidence and, where a sheet reads a derived cache, through the same
/// derivation the controller runs after a completed analysis — never by
/// asserting a capability the data does not support.
fn app_showing(viewer: ResultViewer) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    // The production viewer classifies retained data outside an open
    // project as `no-project`; this fixture represents an open Results
    // workspace and must establish that authority explicitly.
    app.state.project_lifecycle.project_open = true;
    let analysis = match viewer {
        ResultViewer::Waves
        | ResultViewer::Table
        | ResultViewer::Manifest
        | ResultViewer::Eye
        | ResultViewer::Fft => transient_analysis(),
        ResultViewer::DcSweep => AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 0.4, 0.9, 1.2],
                "#00aaff",
            )]),
        ResultViewer::Bode | ResultViewer::Nyquist => ac_analysis(),
        ResultViewer::Smith | ResultViewer::Polar => sparameter_analysis(),
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            // The distribution sheets need a bounded measurement to normalize
            // against, and the box's default grouping draws nothing without
            // one.
            app.state.workspace.specs.push(crate::state::SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: String::new(),
                min: Some(39.5),
                max: None,
                unit: "dB".to_owned(),
                scope: crate::state::SpecPointScope::AllPoints,
            });
            monte_carlo_population_analysis()
        }
        ResultViewer::HarmonicBalance => {
            AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0e9, 2.0e9],
                    vec![1.0, 0.1, 0.01],
                    "#00aaff",
                )
                .with_complex_components(
                    "V(out)",
                    vec![1.0, 0.1, 0.01],
                    vec![0.0, 0.02, 0.001],
                ),
            ])
        }
        ResultViewer::PhaseNoise => AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
            .with_waveforms(vec![WaveformData::new(
                "phase_noise",
                vec![1.0e3, 1.0e4, 1.0e5],
                vec![-80.0, -100.0, -120.0],
                "#00aaff",
            )]),
        ResultViewer::Hist => monte_carlo_analysis(),
        ResultViewer::Op => AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
            rspice_core::circuit::DeviceOpReport {
                entries: vec![rspice_core::circuit::DeviceOpEntry {
                    name: "M1".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: Vec::new(),
                }],
            },
        ),
        ResultViewer::NoiseContrib => AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![WaveformData::new(
                "inoise",
                vec![1.0, 10.0, 100.0],
                vec![1.0e-9, 2.0e-9, 4.0e-9],
                "#00aaff",
            )]),
        ResultViewer::Contribution => AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![
                    SensitivityResultRow {
                        parameter: "r1".to_owned(),
                        raw: 0.25,
                        normalized: 0.5,
                    },
                    SensitivityResultRow {
                        parameter: "r2".to_owned(),
                        raw: -0.125,
                        normalized: -0.25,
                    },
                ],
            }),
        ResultViewer::TransferFunction => transfer_function_analysis(),
        ResultViewer::Specs => {
            app.state.workspace.specs.push(crate::state::SpecEntry {
                measurement: "V(out)".to_owned(),
                expression: String::new(),
                min: Some(-2.0),
                max: Some(5.0),
                unit: "V".to_owned(),
                scope: crate::state::SpecPointScope::AllPoints,
            });
            transient_analysis()
        }
        ResultViewer::PoleZero => AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![crate::state::ComplexResultValue {
                    real: -1.0e3,
                    imaginary: 2.0e3,
                }],
                zeros: vec![crate::state::ComplexResultValue {
                    real: -5.0e3,
                    imaginary: 0.0,
                }],
                pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(10.0),
            }),
        ResultViewer::Events => events_analysis(),
        ResultViewer::Soa => soa_analysis(),
        ResultViewer::Reliability => reliability_analysis(),
        ResultViewer::Optimization => optimization_analysis(),
    };

    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    run.lifecycle = crate::state::SimulationRunLifecycle::Completed;
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));

    // Three sheets read a cache the controller fills after a run rather
    // than the retained analysis itself. Derive it exactly as the
    // controller's post-processing does, so the fixture cannot claim a
    // capability the data would not actually produce.
    match viewer {
        ResultViewer::Nyquist => derive_nyquist(&mut app),
        ResultViewer::Smith => {
            assert!(smith::synchronize_active_analysis(&mut app.state));
        }
        ResultViewer::Hist => derive_histogram(&mut app),
        _ => {}
    }

    if viewer == ResultViewer::Soa {
        // The stress trace is the sheet's only plot; a card that never
        // opens is a card this test never covers.
        app.state.ui.results.soa_stress_trace_open = true;
        app.state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
            analysis: active_analysis_key(&app.state),
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        });
    }
    app
}

fn transient_analysis() -> AnalysisResult {
    let time: Vec<f64> = (0..64).map(|index| index as f64 * 1.0e-9).collect();
    let values: Vec<f64> = time
        .iter()
        .map(|t| (t * 1.0e9 * std::f64::consts::TAU / 8.0).sin())
        .collect();
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![WaveformData::new("V(out)", time, values, "#00aaff")])
}

/// A resonant second-order response, swept densely enough to decimate.
///
/// Four points was not a sweep: it sat under every reduction threshold, so
/// the Bode, Nyquist and Smith fixtures exercised only the raw-stroke
/// path and never the one a real AC run takes. The resonance also makes
/// the complex locus genuinely non-monotone in its real part, so the
/// precondition assertion in `decimate_minmax` fires if either locus
/// sheet ever loses its `parametric` declaration.
const AC_FIXTURE_POINTS: usize = 8_192;

fn ac_analysis() -> AnalysisResult {
    let (mut frequency, mut real, mut imaginary) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    let (mut magnitude, mut phase) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    // H(jω) = 1 / (1 − (ω/ω0)² + j·ω/(Q·ω0)), log-swept through resonance.
    let (natural, quality) = (1.0e5_f64, 6.0_f64);
    for index in 0..AC_FIXTURE_POINTS {
        let decade = 3.0 + 4.0 * index as f64 / (AC_FIXTURE_POINTS - 1) as f64;
        let f = 10.0_f64.powf(decade);
        let ratio = f / natural;
        let (denominator_real, denominator_imaginary) = (1.0 - ratio * ratio, ratio / quality);
        let square =
            denominator_real * denominator_real + denominator_imaginary * denominator_imaginary;
        let (re, im) = (denominator_real / square, -denominator_imaginary / square);
        frequency.push(f);
        real.push(re);
        imaginary.push(im);
        magnitude.push(re.hypot(im));
        phase.push(im.atan2(re).to_degrees());
    }
    AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
        WaveformData::new("|V(out)|", frequency.clone(), magnitude, "#00aaff")
            .with_complex_components("V(out)", real, imaginary),
        WaveformData::new("phase(V(out))", frequency, phase, "#ffaa00"),
    ])
}

fn sparameter_analysis() -> AnalysisResult {
    let mut analysis = ac_analysis();
    analysis.analysis_type = AnalysisType::SParameter;
    analysis.label = "SP".to_owned();
    analysis.waveforms.truncate(1);
    analysis.waveforms[0].name = "|S11|".to_owned();
    analysis.waveforms[0]
        .complex
        .as_mut()
        .expect("AC fixture complex components")
        .source_name = "S11".to_owned();
    analysis.with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm: vec![75.0, 100.0],
    })
}

/// A Monte Carlo that retained both halves of its evidence: the sampled
/// variable and what every trial measured, indexed the same way.
///
/// The histogram fixture beside this one deliberately retains neither — it
/// exercises the path where a distribution comes from the derived histogram
/// cache — so the correlation and distribution sheets need their own.
fn monte_carlo_population_analysis() -> AnalysisResult {
    use crate::state::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
        MonteCarloVariableMetadata,
    };

    const TRIALS: usize = 101;
    let samples: Vec<f64> = (0..TRIALS)
        .map(|index| (index as f64 - 50.0) / 50.0)
        .collect();
    let members = samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: 0x73a4 + index as u64,
                },
                vec![
                    FamilyMeasurementEvidence {
                        name: "gain_dc".to_owned(),
                        value: Some(40.0 + 2.0 * sample),
                        passed: true,
                        error: None,
                    },
                    FamilyMeasurementEvidence {
                        name: "vos".to_owned(),
                        value: Some(60.0e-6 * sample),
                        passed: true,
                        error: None,
                    },
                ],
            )
        })
        .collect::<Vec<_>>();
    let mean = samples.iter().sum::<f64>() / TRIALS as f64;
    let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (TRIALS - 1) as f64;
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
        crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 0x73a4,
            runs_requested: TRIALS,
            runs_completed: TRIALS,
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                name: "XBRIDGE.dR".to_owned(),
                mean,
                std_dev: variance.sqrt(),
                min: -1.0,
                max: 1.0,
                samples,
            }],
            member_measurements: members,
        },
    )
}

fn monte_carlo_analysis() -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_waveforms(vec![WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.95, 1.02, 0.98, 1.05],
        "#00aaff",
    )])
}

fn transfer_function_analysis() -> AnalysisResult {
    use crate::state::{
        TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
        TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
    };
    AnalysisResult::new(1, AnalysisType::Tf, "XF").with_result_payload(
        AnalysisResultPayload::TransferFunction {
            input_source: "vin".to_owned(),
            output_expression: "V(out)".to_owned(),
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TransferFunctionNormalizationEvidence::None,
            accuracy: TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(TransferFunctionScalarEvidence::Finite(-12.5)),
            input_resistance: Some(TransferFunctionScalarEvidence::Finite(1.0e6)),
            output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
            nominal_input: None,
            nominal_output: None,
        },
    )
}

fn in_flight_cache_provenance(app: &RSpiceApp) -> SpecializedViewerCacheProvenance {
    let run = app
        .state
        .simulation
        .active_run()
        .expect("the fixture retained one run");
    let dataset_id = run.dataset_id;
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    SpecializedViewerCacheProvenance::for_analysis(dataset_id, analysis)
}

fn ac_complex_trace(app: &RSpiceApp) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.complex.is_some())
        .expect("the AC fixture retains one complex trace");
    let complex = waveform
        .complex
        .as_ref()
        .expect("the trace was selected for its complex components");
    (
        waveform.x.to_vec(),
        complex.real.to_vec(),
        complex.imag.to_vec(),
    )
}

fn derive_nyquist(app: &mut RSpiceApp) {
    let (frequency, real, imaginary) = ac_complex_trace(app);
    let provenance = in_flight_cache_provenance(app);
    app.state.analysis.nyquist_state.load_data(
        crate::analysis::nyquist::data::NyquistData::from_arrays(
            "V(out)", &frequency, &real, &imaginary,
        ),
    );
    app.state
        .bind_specialized_viewer_cache(ActiveViewer::Nyquist, provenance);
}

fn derive_histogram(app: &mut RSpiceApp) {
    use crate::analysis::histogram::data::{Histogram, HistogramBin};

    let counts = [2_usize, 5, 9, 4];
    let edges = [0.90_f64, 0.95, 1.00, 1.05, 1.10];
    let bins: Vec<HistogramBin> = counts
        .iter()
        .enumerate()
        .map(|(index, count)| HistogramBin {
            lower: edges[index],
            upper: edges[index + 1],
            count: *count,
            weight: *count as f64,
        })
        .collect();
    let total_count: usize = counts.iter().sum();
    let provenance = in_flight_cache_provenance(app);
    app.state
        .analysis
        .histogram_state
        .load_histogram(Histogram {
            name: "V(out)".to_owned(),
            bins,
            total_count,
            total_weight: total_count as f64,
            underflow: 0,
            overflow: 0,
            data_min: edges[0],
            data_max: edges[edges.len() - 1],
        });
    app.state
        .bind_specialized_viewer_cache(ActiveViewer::Histogram, provenance);
}

/// Every sheet, painted with no retained dataset at all.
///
/// That is the workspace's state before the first run and the one a user
/// meets first, and it is where a plot divides by a zero-width axis span or
/// reads the first of an empty series. Fourteen of the twenty-two sheets had
/// never been through a real frame in any test — their module tests derive
/// and format values without ever painting them.
#[test]
fn every_sheet_paints_an_empty_dataset() {
    for viewer in ResultViewer::every() {
        let mut app = RSpiceApp::test_instance();
        draw_sheet_and_tessellate(&mut app, viewer);
        assert_eq!(
            app.state.ui.results.viewer, viewer,
            "{viewer:?} did not stay selected across its own frame"
        );
    }
}

/// Run one sheet plus its inspector panel through a real egui frame and
/// assert every mesh vertex is finite. A value that escapes an axis
/// mapping does not panic — it becomes a vertex at infinity and
/// degenerates the whole draw call.
///
/// Painted through `show_persistent_pane_viewer`, the same dispatch the
/// workbench itself paints through, so no sheet can pass here by a route
/// the product never takes.
fn draw_sheet_and_tessellate(app: &mut RSpiceApp, viewer: ResultViewer) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    // Twice: the first frame builds the caches, the second is the path a
    // reader actually spends their time on.
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::Panel::right("inspector").show(ctx, |ui| right_panel(ui, &mut app.state));
            egui::CentralPanel::default()
                .show(ctx, |ui| show_persistent_pane_viewer(ui, app, viewer));
        });
        for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
            let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                continue;
            };
            assert!(
                mesh.vertices
                    .iter()
                    .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                "{viewer:?} put a non-finite vertex in the mesh",
            );
        }
    }
}

/// The two gates a Results sheet is looked at through: the small window the
/// display scaling of a real workstation produces, and a wide one.
const SHEET_FIT_GATES: [(f32, f32); 2] = [(1_024.0, 640.0), (1_680.0, 1_020.0)];

/// Every control of the four sheets that own a domain bar is inside the window
/// it is drawn in, at both gates.
///
/// Measured through AccessKit rather than through painted shapes, for the
/// reason the Studio's own gate states: a decorative shape may legitimately be
/// clipped and a control may not. A control past the right edge is a control
/// behind a horizontal scroll, which is the thing these sheets' single bar
/// exists to avoid; one past the bottom is a register the reader cannot reach
/// without scrolling a document well that does not scroll.
///
/// The Events sheet joined the list when it gained a bar of its own: its bus
/// radix, and the per-row disclosure that opens a word to its member rows,
/// are controls that a reader has to be able to reach.
#[test]
fn the_catalog_sheets_fit_their_window_at_both_gates() {
    // Sub-pixel: a control resting exactly on the edge is inside it.
    const TOLERANCE: f64 = 0.5;

    let mut offenders = Vec::new();
    let mut measured = 0_usize;
    // Counted separately: a disclosure that names the bus it opens is the one
    // control here whose absence would look exactly like a sheet that simply
    // declared no bus.
    let mut buses_measured = 0_usize;
    for viewer in [
        ResultViewer::Polar,
        ResultViewer::Scatter,
        ResultViewer::BoxViolin,
        ResultViewer::Events,
    ] {
        for (width, height) in SHEET_FIT_GATES {
            let mut app = app_showing(viewer);
            app.state.ui.results.viewer = viewer;
            for (_, node) in sheet_route_nodes(app, width, height) {
                if !matches!(
                    node.role(),
                    egui::accesskit::Role::Button
                        | egui::accesskit::Role::CheckBox
                        | egui::accesskit::Role::ComboBox
                        | egui::accesskit::Role::Link
                        | egui::accesskit::Role::RadioButton
                        | egui::accesskit::Role::TextInput
                ) {
                    continue;
                }
                let Some(bounds) = node.bounds() else {
                    continue;
                };
                measured += 1;
                if node
                    .label()
                    .is_some_and(|label| label.starts_with("Bits of "))
                {
                    buses_measured += 1;
                }
                if bounds.x1 > f64::from(width) + TOLERANCE
                    || bounds.x0 < -TOLERANCE
                    || bounds.y1 > f64::from(height) + TOLERANCE
                    || bounds.y0 < -TOLERANCE
                {
                    offenders.push(format!(
                        "{width:.0}x{height:.0} {viewer:?}: {:?} {:?} spans x {:.1}..{:.1}, y {:.1}..{:.1}",
                        node.role(),
                        node.label().unwrap_or_default(),
                        bounds.x0,
                        bounds.x1,
                        bounds.y0,
                        bounds.y1
                    ));
                }
            }
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "controls drawn outside the window they belong to:
{}",
        offenders.join(
            "
"
        )
    );
    assert!(
        measured >= 4 * SHEET_FIT_GATES.len(),
        "the gate measured {measured} controls, so it is not looking at the sheets' bars"
    );
    assert!(
        buses_measured >= 2 * SHEET_FIT_GATES.len(),
        "the gate measured {buses_measured} bus disclosures, so the Events fixture's two \
         declarations are not reaching the sheet"
    );
}

/// The Results workspace at one window size, as its controls published
/// themselves. Two passes: the first settles the layout the second is
/// measured on.
fn sheet_route_nodes(
    mut app: RSpiceApp,
    width: f32,
    height: f32,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, height),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::Panel::right("inspector").show(ctx, |ui| right_panel(ui, &mut app.state));
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show(ui, &mut app));
            },
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes
}

/// Paint one sheet and count the primitives it actually emits.
fn drawn_shape_count(app: &mut RSpiceApp, viewer: ResultViewer) -> usize {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut shapes = 0;
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| show_persistent_pane_viewer(ui, app, viewer));
        });
        shapes = output.shapes.len();
    }
    shapes
}

fn operating_point_app(devices: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
        rspice_core::circuit::DeviceOpReport {
            entries: (0..devices)
                .map(|index| rspice_core::circuit::DeviceOpEntry {
                    // Spread across scopes so the group headers and the
                    // per-scope column sets are exercised too.
                    name: format!("X{}.M{index}", index % 8),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: Vec::new(),
                })
                .collect(),
        },
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app
}

fn sensitivity_app(parameters: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
        AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: (0..parameters)
                .map(|index| SensitivityResultRow {
                    // Zero-padded: the payload requires strictly sorted
                    // parameter names, and "p10" sorts before "p2".
                    parameter: format!("p{index:06}"),
                    raw: index as f64 * 1.0e-3,
                    normalized: (index as f64 * 1.0e-3).tanh(),
                })
                .collect(),
        },
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app
}

fn manifest_app(analyses: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    for index in 0..analyses {
        run.add_analysis(AnalysisResult::new(
            index as u64 + 1,
            AnalysisType::Transient,
            format!("TRAN {index}"),
        ));
    }
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app
}

/// The evidence tables must cost their viewport, not their dataset.
///
/// `save_device_op` on a real block emits one row per device, a swept
/// design ranks one sensitivity row per parameter, and a save-all run
/// retains one manifest row per task. All three laid out, sensed and
/// painted every retained row on every frame, so the sheet's cost grew
/// without bound while the window stayed the same size. A row count is
/// not the assertion — the drawn primitive count is.
#[test]
fn the_evidence_tables_cost_their_viewport_not_their_dataset() {
    for (viewer, small, large) in [
        (
            ResultViewer::Op,
            drawn_shape_count(&mut operating_point_app(64), ResultViewer::Op),
            drawn_shape_count(&mut operating_point_app(20_000), ResultViewer::Op),
        ),
        (
            ResultViewer::Contribution,
            drawn_shape_count(&mut sensitivity_app(64), ResultViewer::Contribution),
            drawn_shape_count(&mut sensitivity_app(20_000), ResultViewer::Contribution),
        ),
        (
            ResultViewer::Manifest,
            drawn_shape_count(&mut manifest_app(64), ResultViewer::Manifest),
            drawn_shape_count(&mut manifest_app(4_000), ResultViewer::Manifest),
        ),
    ] {
        assert!(
            large < small * 2,
            "{viewer:?} drew {large} primitives for the large dataset against {small} \
             for the small one — it is still laying out rows it cannot show"
        );
    }
}

/// A run that stopped early must not draw like one that finished.
///
/// The waveform sheets deliberately still plot it — where a transient
/// stopped converging is what the plot is for — but for a long time they
/// plotted it with no mark at all, while every typed-evidence sheet
/// refused outright. A curve that ends early is indistinguishable from a
/// sweep specified to end there.
#[test]
fn a_run_that_did_not_complete_says_so_on_the_sheet_that_draws_it() {
    let mut app = app_showing(ResultViewer::Waves);
    assert_eq!(
        active_incomplete_evidence_reason(&app.state),
        None,
        "a converged fixture must not claim a caution"
    );
    let clean_purpose = sheet_purpose(&app.state);

    app.state.simulation.runs[0].analyses[0].success = false;

    // Still drawable: refusing the partial samples would take away the
    // one view that answers "where did it go wrong?".
    assert!(
        viewer_availability(&app.state, ResultViewer::Waves).available,
        "partial transient samples must stay drawable"
    );
    let purpose = sheet_purpose(&app.state);
    assert_ne!(purpose, clean_purpose);
    assert!(
        purpose.contains("did not complete"),
        "the sheet bar has to say it in words: {purpose}"
    );

    // And the strip that draws it carries the mark, so a stack whose
    // other analyses converged cannot pass the failed one off as clean.
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| waves::show(ui, &mut app.state));
        },
    );
    let labels: Vec<String> = output
        .platform_output
        .accesskit_update
        .expect("the waveform stack publishes an accessibility tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    assert!(
        labels
            .iter()
            .any(|label| label.contains("stop where it failed")),
        "no pane published the incomplete-evidence reason: {labels:?}"
    );
}

/// An explicit interval has to land in the store the sheet reads.
///
/// The waveform stack keys its viewports by analysis and every other
/// sheet by plot ordinal. A previous fit command wrote to the ordinal
/// store for all of them, so Fit was a silent no-op on the four sheets
/// people actually use. The same trap is waiting for anything that
/// writes a viewport, so this asserts through the reader, not the map.
#[test]
fn an_explicit_interval_lands_in_the_store_its_own_sheet_reads() {
    let tokens = Tokens::default();
    let mut app = app_showing(ResultViewer::Waves);
    let analysis = active_analysis_key(&app.state);
    app.state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    });

    assert!(set_active_axis_range(
        &tokens,
        &mut app.state,
        PaneAxis::X,
        Some((1.0e-6, 4.0e-6))
    ));
    assert_eq!(
        app.state
            .ui
            .results
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 0)
            .x,
        Some((1.0e-6, 4.0e-6)),
        "the waveform stack must read its own analysis-keyed viewport"
    );
    assert!(active_axis_is_pinned(&app.state, PaneAxis::X));

    set_active_axis_range(&tokens, &mut app.state, PaneAxis::X, None);
    assert!(!active_axis_is_pinned(&app.state, PaneAxis::X));

    let mut app = app_showing(ResultViewer::Smith);
    app.state.ui.results.viewer = ResultViewer::Smith;
    assert!(set_active_axis_range(
        &tokens,
        &mut app.state,
        PaneAxis::Y,
        Some((-0.5, 0.5))
    ));
    assert_eq!(
        app.state.ui.results.plot_view(ResultViewer::Smith, 0).y,
        Some((-0.5, 0.5)),
        "a single-canvas sheet keeps its viewport under the plot ordinal"
    );
    assert!(active_axis_is_pinned(&app.state, PaneAxis::Y));
}

/// The editor has to open on the interval the reader can see, including
/// on a sheet that is fitting its data rather than pinned.
#[test]
fn the_axis_editor_opens_on_what_the_sheet_actually_drew() {
    let mut app = app_showing(ResultViewer::Smith);
    app.state.ui.results.viewer = ResultViewer::Smith;
    draw_sheet_and_tessellate(&mut app, ResultViewer::Smith);

    let facts = ActivePaneFacts {
        unit: None,
        analysis: None,
        traces: None,
        runs: None,
        scale: None,
        limit_mask: "none bound",
        x_viewport: None,
        y_viewport: None,
        x_extent: None,
        y_extent: None,
        pinned: None,
    };
    let x = active_axis_range(&app.state, &facts, PaneAxis::X)
        .expect("the Smith sheet recorded the interval it drew");
    assert!(
        x.0 < x.1 && x.0.is_finite() && x.1.is_finite(),
        "recorded a degenerate interval: {x:?}"
    );
    assert!(
        !active_axis_is_pinned(&app.state, PaneAxis::X),
        "a fitted sheet must not report itself pinned just because it drew"
    );
}

/// Every sheet, drawn at the touch composition the tablet build uses.
///
/// The shell has raised its own rows to a 44 px target for a long time
/// while this workspace kept fixed workstation heights, so its chips and
/// icon buttons grew past the bands holding them. A frame is what proves
/// the composition survives, not the constants alone.
#[test]
fn every_sheet_draws_at_a_touch_composition() {
    for viewer in ResultViewer::every() {
        let mut app = app_showing(viewer);
        app.state.ui.results.viewer = viewer;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::Theme::default()
            .apply_responsive_metrics_with_target(&ctx, Some(crate::ui::tokens::TOUCH_TARGET));
        assert!(
            Tokens::get(&ctx).metrics.is_touch(),
            "the fixture did not reach a touch composition"
        );
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_024.0, 1_366.0),
            )),
            ..Default::default()
        };
        for _ in 0..2 {
            let output = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| show_persistent_pane_viewer(ui, &mut app, viewer));
            });
            for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                    continue;
                };
                assert!(
                    mesh.vertices
                        .iter()
                        .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                    "{viewer:?} put a non-finite vertex in the mesh at a touch composition",
                );
            }
        }
    }
}

#[test]
fn stat_columns_are_disjoint_at_phone_panel_width() {
    let width = 240.0;
    let (name, value) = stat_column_widths(width);
    assert!(name > 0.0);
    assert!(value > name);
    assert!((name + value + 24.0 + 8.0 - width).abs() < f32::EPSILON * width);
}

fn transient_state() -> AppState {
    state_with_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    )
}

fn active_analysis_key(state: &AppState) -> AnalysisPresentationKey {
    let run = state.simulation.active_run().expect("active retained run");
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
        .with_waveforms(vec![WaveformData::new(
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
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, first, 0)
        .x = Some((0.25, 0.75));

    // The same authored analysis, solved again into a new dataset.
    let mut rerun = SimulationRun::new(2);
    rerun.add_analysis(authored);
    state.simulation.runs.insert(0, rerun);
    assert!(state.simulation.select_run(0));
    let second = active_analysis_key(&state);
    assert_ne!(
        first.dataset_id(),
        second.dataset_id(),
        "a re-run mints a new dataset identity"
    );

    assert_eq!(
        state
            .ui
            .results
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
            WaveformData::new("V(gone)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    );
    let discarded =
        AnalysisPresentationKey::new(discarded_run.dataset_id, &discarded_run.analyses[0]);
    state.simulation.runs.push(discarded_run);
    state
        .ui
        .results
        .reconcile_retained_datasets(&state.simulation);

    for (analysis, name) in [(kept, "V(out)"), (discarded, "V(gone)")] {
        state.ui.results.add_marker(
            analysis,
            marker_anchor_for(analysis, name),
            name.to_owned(),
            0.5,
        );
        state
            .ui
            .results
            .log_y_panes
            .insert(WavePanePresentationKey {
                analysis,
                unit: "V".to_owned(),
            });
        state.ui.results.hidden_strips.insert(analysis);
        state
            .ui
            .results
            .favorite_signals
            .insert(SourceWaveformPresentationKey::new(analysis, name));
        state.ui.results.analysis_exprs.insert(analysis, Vec::new());
    }

    // Retention discards the second run.
    state
        .simulation
        .runs
        .retain(|run| run.dataset_id == kept.dataset_id());
    state
        .ui
        .results
        .reconcile_retained_datasets(&state.simulation);

    assert_eq!(state.ui.results.markers.len(), 1);
    assert_eq!(state.ui.results.markers[0].analysis, kept);
    assert_eq!(state.ui.results.log_y_panes.len(), 1);
    assert_eq!(state.ui.results.hidden_strips.len(), 1);
    assert_eq!(state.ui.results.favorite_signals.len(), 1);
    assert_eq!(state.ui.results.analysis_exprs.len(), 1);
    assert!(state.ui.results.analysis_exprs.contains_key(&kept));
    assert!(
        state
            .ui
            .results
            .project_expression_groups(&state.simulation)
            .iter()
            .all(|group| group.analysis == kept),
        "a save must not carry a group about a dataset the project dropped"
    );
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
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
        .y = Some((0.0, 1.0));
    state
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 1)
        .x = Some((0.0, 0.5));
    state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis: key,
        unit: "V".to_owned(),
    });

    waves::fit_active_strip(&mut state);

    for ordinal in 0..2 {
        assert!(
            !state
                .ui
                .results
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
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
        .y = Some((0.0, 1.0));
    state.ui.results.active_wave_pane = None;

    waves::fit_active_strip(&mut state);

    assert!(
        !state
            .ui
            .results
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
    state.ui.results.viewer = ResultViewer::Waves;
    assert!(zoom_gesture_available(&state));
    assert!(fit_gesture_available(&state));

    state.ui.results.viewer = ResultViewer::Fft;
    assert!(!zoom_gesture_available(&state));
    assert!(
        fit_gesture_available(&state),
        "a single-canvas plot still has a viewport to release"
    );

    state.ui.results.viewer = ResultViewer::Table;
    assert!(!fit_gesture_available(&state));
    assert!(!zoom_gesture_available(&state));
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

    restore_markers(&mut state, vec![marker]);

    assert_eq!(state.ui.results.markers.len(), 1);
    assert_eq!(state.ui.results.markers[0].note, "settling");
    let next = state.ui.results.add_marker(
        key,
        state.ui.results.markers[0].anchor.clone(),
        "V(out)".to_owned(),
        0.75,
    );
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

    restore_markers(&mut state, vec![kept, dropped]);

    assert_eq!(state.ui.results.markers.len(), 1);
    assert_eq!(state.ui.results.markers[0].id, 1);
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
    let mut magnitude = WaveformData::new("|V(out)|", frequency.clone(), vec![40.0, -40.0], "#fff");
    magnitude.visible = true;
    let mut phase = WaveformData::new("phase(V(out))", frequency, vec![0.0, -135.0], "#fff");
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

