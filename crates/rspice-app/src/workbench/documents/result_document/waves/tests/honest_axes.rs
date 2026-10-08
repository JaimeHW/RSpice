//! What a strip says about itself, and whether it is entitled to say it.
//!
//! Each of these is the same failure wearing a different hat: a surface that
//! states something it did not read. An axis that calls every DC sweep volts,
//! a legend chip whose colour is a different arithmetic from the curve it
//! names, a row that does not say which run it came from, and a painted table
//! that says nothing at all to a reader who cannot see it.

use super::*;

use crate::state::{ExecutedDeck, ExecutedDeckPoint};

#[test]
fn qpac_plot_keeps_signed_probe_offsets_on_a_named_linear_axis() {
    let result = crate::simulation::results::qpac_retained_test_fixture();
    assert!(result.success);
    assert!(crate::state::ac_bode_summary_for_analysis(&result, 0).is_none());
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(result);
    state.ui.results.session.viewer = super::super::super::ResultViewer::Bode;
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].x_scale, XScale::Linear);
    assert_eq!(models[0].x_label, "Probe offset");
    assert_eq!(models[0].x_unit, "Hz");
    assert_eq!(models[0].x_dimension_key, "qpac-probe-offset");
    let (min, max) = models[0]
        .x_range
        .expect("signed offsets have a visible range");
    assert!(min <= -37.0 && max >= 127.0);
}

#[test]
fn qpxf_plot_keeps_signed_output_frequencies_on_a_named_linear_axis() {
    let result = crate::simulation::results::qpxf_retained_test_fixture();
    assert!(result.success);
    assert!(crate::state::ac_bode_summary_for_analysis(&result, 0).is_none());
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(result);
    state.ui.results.session.viewer = super::super::super::ResultViewer::Bode;
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].x_scale, XScale::Linear);
    assert_eq!(models[0].x_label, "Output frequency");
    assert_eq!(models[0].x_unit, "Hz");
    assert_eq!(models[0].x_dimension_key, "qpxf-output-frequency");
    let (min, max) = models[0]
        .x_range
        .expect("signed offsets have a visible range");
    assert!(min <= -37.0 && max >= 127.0);
}

/// A DC sweep of a current source, with the deck the run actually executed.
fn swept_current_source() -> AppState {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0e-3], vec![0.0, 5.0], "#fff"),
        ]),
    );
    let run_id = state.simulation.retained.runs[0].id;
    state
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id,
            points: vec![ExecutedDeckPoint {
                label: "DC".to_owned(),
                deck: "* bias sweep\nIbias 0 in DC 0\n.dc Ibias 0 1m 10u\n.end\n".into(),
                model_sources: Vec::new(),
            }],
        });
    state.ui.results.session.viewer = super::super::super::ResultViewer::DcSweep;
    state
}

fn model_axis(state: &mut AppState) -> (String, String) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    (models[0].x_label().to_owned(), models[0].x_unit.clone())
}

#[test]
fn imported_dc_axes_use_retained_name_and_unit_instead_of_voltage_defaults() {
    use rspice_results::result_import::{
        ResultImportCoordinate, ResultImportFormat, ResultImportSource,
    };
    for unit in [Some("A"), Some("K"), Some("1"), None] {
        let mut state = swept_current_source();
        state.simulation.retained.runs[0].analyses[0].import_source = Some(ResultImportSource {
            source_name: "axis.raw".into(),
            format: ResultImportFormat::SpiceRaw,
            coordinate: Some(ResultImportCoordinate {
                name: "source-axis".into(),
                unit: unit.map(str::to_owned),
            }),
        });
        assert_eq!(
            model_axis(&mut state),
            ("source-axis".into(), unit.unwrap_or("").to_owned())
        );
    }
}

#[test]
fn imported_overlays_require_compatible_coordinate_units() {
    use rspice_results::result_import::{
        ResultImportCoordinate, ResultImportFormat, ResultImportSource,
    };
    for (overlay_unit, expected_overlay) in [(Some("A"), true), (Some("V"), false), (None, false)] {
        let mut runs = Vec::new();
        for (id, unit) in [(2, Some("A")), (1, overlay_unit)] {
            let mut run = SimulationRun::new(id);
            let mut analysis =
                AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
                    WaveformData::new("V(out)", vec![0.0, 1.0], vec![1.0, 2.0], "#fff"),
                ]);
            analysis.import_source = Some(ResultImportSource {
                source_name: "axis.raw".into(),
                format: ResultImportFormat::SpiceRaw,
                coordinate: Some(ResultImportCoordinate {
                    name: "bias".into(),
                    unit: unit.map(str::to_owned),
                }),
            });
            run.add_analysis(analysis);
            runs.push(run);
        }
        let overlay_dataset = runs[1].dataset_id;
        let simulation = SimulationState {
            retained: crate::state::RetainedSimulationState {
                runs: runs.into(),
                ..Default::default()
            },
            view: crate::state::SimulationViewState {
                active_run_idx: Some(0),
                overlay_dataset_ids: vec![overlay_dataset],
                ..Default::default()
            },
            ..Default::default()
        };
        let models = build_models(
            &simulation,
            &mut DerivedSeries::default(),
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );
        assert_eq!(
            models[0].traces.iter().any(|trace| trace.overlay),
            expected_overlay
        );
        assert_eq!(
            models[0].subtitle.contains("incompatible overlay"),
            !expected_overlay
        );
    }
}

#[test]
fn overlays_require_matching_source_signal_units_before_display_projection() {
    for analysis_type in [AnalysisType::Transient, AnalysisType::Ac] {
        for (active_unit, overlay_unit, expected_overlay) in [
            (Some("V"), Some("V"), true),
            (Some("mV"), Some("mV"), true),
            (Some("V"), Some("mV"), false),
            (Some("V"), Some("A"), false),
            (Some("V"), Some("1"), false),
            (Some("V"), None, false),
            (None, Some("V"), false),
            (None, None, true),
        ] {
            let mut runs = Vec::new();
            for (id, unit) in [(2, active_unit), (1, overlay_unit)] {
                let mut run = SimulationRun::new(id);
                let mut waveform =
                    WaveformData::new("V(out)", vec![1.0, 2.0], vec![1.0, 2.0], "#fff");
                waveform.unit = unit.map(str::to_owned);
                run.add_analysis(
                    AnalysisResult::new(1, analysis_type, "source").with_waveforms(vec![waveform]),
                );
                runs.push(run);
            }
            let overlay_dataset = runs[1].dataset_id;
            let simulation = SimulationState {
                retained: crate::state::RetainedSimulationState {
                    runs: runs.into(),
                    ..Default::default()
                },
                view: crate::state::SimulationViewState {
                    active_run_idx: Some(0),
                    overlay_dataset_ids: vec![overlay_dataset],
                    ..Default::default()
                },
                ..Default::default()
            };
            let models = build_models(
                &simulation,
                &mut DerivedSeries::default(),
                &Tokens::default(),
                false,
                ComplexNumberDisplay::MagnitudePhaseDegrees,
                None,
                &HashSet::new(),
            );
            assert_eq!(models.len(), 1);
            assert_eq!(
                models[0].traces.iter().any(|trace| trace.overlay),
                expected_overlay,
                "{analysis_type:?}: {active_unit:?} vs {overlay_unit:?}"
            );
            assert_eq!(
                models[0].subtitle.contains("incompatible overlay"),
                !expected_overlay
            );
        }
    }
}

/// The retained result keeps the sweep's values and not the source they came
/// from, so the abscissa called every DC sweep volts — a swept current source
/// read as a voltage, which is not a unit error but a quantity error. The deck
/// the run executed names the source, and it is that run's own evidence.
#[test]
fn a_dc_sweep_names_the_source_the_run_actually_swept() {
    let mut state = swept_current_source();
    assert_eq!(
        model_axis(&mut state),
        ("Ibias".to_owned(), "A".to_owned()),
        "the abscissa did not read the swept source out of the run's deck"
    );
}

#[test]
fn a_solved_current_source_sweep_keeps_amperes_without_an_executed_deck() {
    let result = crate::simulation::controller::test_execution::run_manual_deck(
        "Current sweep\nIBIAS 0 out 0\nR1 out 0 1k\n.dc IBIAS 0 1m 0.5m\n.end\n",
    );
    let analysis = crate::simulation::controller::dc_history_tests::retain(result);
    assert!(analysis.success, "{:?}", analysis.error_message);
    assert_eq!(analysis.waveforms[0].unit.as_deref(), Some("V"));
    let voltage = analysis.waveforms[0].y[2];
    assert!(
        (voltage - 1.0).abs() <= rspice_core::constants::RELTOL,
        "1 V within configured relative tolerance; got {voltage:.16e}"
    );
    let mut state = AppState::default();
    state.simulation = crate::simulation::controller::dc_history_tests::history(analysis);
    state.ui.results.session.viewer = super::super::super::ResultViewer::DcSweep;
    assert_eq!(model_axis(&mut state), ("IBIAS".to_owned(), "A".to_owned()));
    let stored = crate::io::capture_simulation_results(&state.simulation);
    crate::io::restore_simulation_results(stored, &mut state.simulation).unwrap();
    assert_eq!(model_axis(&mut state), ("IBIAS".to_owned(), "A".to_owned()));
}

/// A run whose decks were not retained has nothing to read, and inventing a
/// source would be the failure this replaces. The analysis default stands.
#[test]
fn a_dc_sweep_without_a_retained_deck_keeps_the_analysis_default() {
    let mut state = swept_current_source();
    assert_eq!(model_axis(&mut state), ("Ibias".to_owned(), "A".to_owned()));
    state.simulation.retained.executed_decks = Default::default();
    assert_eq!(model_axis(&mut state), ("x".to_owned(), "V".to_owned()));
}

/// The legend chip and the curve have to be the same colour, which means they
/// have to count palette slots the same way. The chip counted the active run's
/// traces and the canvas counted every trace it held, so a strip with an
/// overlay run drew an expression in one colour and named it in another.
#[test]
fn an_expression_chip_takes_the_palette_slot_its_curve_draws_in() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let earlier = state.simulation.retained.runs[0].dataset_id;
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
        ]),
    );
    state.simulation.view.overlay_dataset_ids.push(earlier);

    let tokens = Tokens::default();
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &tokens,
    );
    let model = &models[0];
    assert!(
        model.traces.len() > model.signal_trace_count,
        "the fixture needs an overlay run for the two counts to differ"
    );
    let chip = expr_color(&tokens, expr_palette_slot(model, 0));
    assert_ne!(
        chip,
        expr_color(&tokens, model.signal_trace_count),
        "the fixture no longer separates the legend's count from the canvas'"
    );
    let analysis = model.analysis_key;
    drop(models);

    state
        .ui
        .results
        .add_expression_trace(&state.simulation, analysis, "V(out)*2".to_owned())
        .expect("the strip accepts an expression");
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &tokens,
    );
    let model = Arc::clone(&models);
    drop(models);
    let resolved = resolve_strip_exprs(&mut state, &model[0], &tokens);
    assert_eq!(
        resolved.len(),
        1,
        "the expression resolved to one drawn curve"
    );
    assert_eq!(
        resolved[0].color, chip,
        "the chip and the curve took different palette slots"
    );
}

/// Two runs of the same signal arrive as two rows spelled identically. Which
/// is the overlay is the whole reason both are on the sheet.
#[test]
fn an_overlay_row_names_the_run_it_came_from() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let earlier = state.simulation.retained.runs[0].dataset_id;
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
        ]),
    );
    state.simulation.view.overlay_dataset_ids.push(earlier);
    state.ui.results.session.cursor_strip = Some(0);

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let rows = readout_rows(
        &models[0],
        CursorPair {
            a: Some(0.5),
            b: None,
        },
        presentation.readout(),
        quantity_policy,
    );

    assert_eq!(rows.len(), 2, "the active run and its overlay");
    assert_eq!(rows[0].name, "V(out)", "the active run owns the plain name");
    assert!(
        rows[1].name.contains("run "),
        "an overlay row does not say which run it is: {}",
        rows[1].name
    );
}

/// The cursor table is painted, not built from widgets, so nothing about it
/// reaches a screen reader on its own. The marker half of the same strip
/// already published its rows; this half stated the same numbers to a sighted
/// reader and nothing at all to anyone else.
#[test]
fn the_cursor_table_publishes_what_it_paints() {
    let mut state = super::branches::hysteresis_run();
    if !state.ui.results.session.cursor_tool.is_armed() {
        state.ui.results.session.toggle_cursor_tool();
    }
    state.ui.results.session.cursors.place(0.25);
    state.ui.results.session.cursors.place(0.75);
    state.ui.results.session.readout_collapsed = false;

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut labels: Vec<String> = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let height = readout_strip_height(&mut state);
                readout_strip(ui, &mut state, height);
            });
        });
        labels = output
            .platform_output
            .accesskit_update
            .expect("the workbench publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Table)
            .filter_map(|(_, node)| node.label())
            .map(str::to_owned)
            .collect();
    }

    let spoken = labels
        .iter()
        .find(|label| label.starts_with("Cursor readout"))
        .unwrap_or_else(|| panic!("the cursor table published no table node: {labels:?}"));
    assert!(
        spoken.contains("fwd") && spoken.contains("rev"),
        "the spoken table lost the branches the painted one shows: {spoken}"
    );
    assert!(
        spoken.contains("2 branches"),
        "the spoken table did not state the shape of the sweep: {spoken}"
    );
    assert!(
        spoken.contains("slope"),
        "the spoken table named no columns: {spoken}"
    );
}

#[test]
fn qpnoise_result_plot_preserves_physical_frequencies_and_separate_noise_units() {
    let result = crate::simulation::results::qpnoise_retained_test_fixture();
    assert!(result.success);
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(result);
    state.simulation.view.active_analysis_idx = Some(0);
    state.ui.results.session.viewer = super::super::super::ResultViewer::NoiseContrib;
    assert!(super::super::super::view_context::analysis_supports_viewer(
        state.ui.results.session.viewer,
        state.simulation.active_analysis().unwrap()
    ));
    assert!(super::super::super::viewer_is_available(
        &state,
        state.ui.results.session.viewer
    ));
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].x_scale, XScale::Linear);
    assert_eq!(models[0].x_label, "Output frequency");
    assert_eq!(models[0].x_unit, "Hz");
    let (min, max) = models[0].x_range.unwrap();
    assert!(min < 0.0 && max >= 700.0);
    let panes = models[0].unit_panes();
    for unit in ["V²/Hz", "A²/Hz", "V/√Hz", "A/√Hz", "dB", "V·A/Hz"] {
        assert!(panes.iter().any(|p| p.unit == unit), "missing {unit}");
    }
}
