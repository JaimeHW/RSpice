//! Retained result measurements use the calculator's interval mathematics and
//! report sweep branches and missing coverage in the visible readout.

use super::*;

/// A trace that repeats has its period and frequency read with its other
/// readings, as `period()` and `freq()` read them, and each reading of a
/// trace the calculator names keeps as that function of it.
#[test]
fn a_repeating_trace_reads_its_period_and_frequency() {
    let x: Vec<f64> = (0..=2000).map(|i| f64::from(i) * 5e-6).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|t| (std::f64::consts::TAU * 1e3 * t).sin())
        .collect();
    let mut state = transient(x, y);
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let model = &models[0];
    let rows = readout::measurement_values(
        &mut state.results.results.session.derived,
        model,
        Some((2.5e-3, 7.5e-3)),
        4,
        state.model.preferences.quantity_presentation_policy(),
        &rspice_results_ui::waves::as_returned,
    );
    let labels: Vec<&str> = rows.iter().map(|row| row.label.as_str()).collect();
    assert_eq!(
        labels,
        [
            "V(out) min",
            "V(out) max",
            "V(out) p-p",
            "V(out) avg",
            "V(out) rms",
            "V(out) period",
            "V(out) freq",
        ]
    );
    // Five whole cycles average to nothing the trace resolves.
    assert_eq!(rows[3].value, "0 V", "{rows:?}");
    // One millisecond, read to four digits whether the crossings land on
    // it exactly or a rounding away.
    assert!(
        matches!(rows[5].value.as_str(), "1 ms" | "1.000 ms"),
        "{rows:?}"
    );
    assert!(
        matches!(rows[6].value.as_str(), "1 kHz" | "1.000 kHz"),
        "{rows:?}"
    );
    assert_eq!(rows[5].keep.as_deref(), Some("period(V(out))"));
    assert_eq!(rows[6].keep.as_deref(), Some("freq(V(out))"));
}

fn transient(x: Vec<f64>, y: Vec<f64>) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#fff")]),
    );
    state.model.simulation.complete_run();
    state
}

#[test]
fn measurement_readout_uses_interpolated_endpoints_and_matches_calculator() {
    let mut state = transient(vec![0.0, 1.0], vec![0.0, 1.0]);
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let model = &models[0];
    let trace = &model.traces[0];
    let stats = trace_interval_statistics(
        &mut state.results.results.session.derived,
        model,
        trace,
        Some((0.25, 0.75)),
        None,
    )
    .unwrap();
    assert_eq!((stats.min, stats.max, stats.mean), (0.25, 0.75, 0.5));
    assert!((stats.rms - (13.0_f64 / 48.0).sqrt()).abs() < 1e-14);
    let rows = readout::measurement_values(
        &mut state.results.results.session.derived,
        model,
        Some((0.25, 0.75)),
        8,
        state.model.preferences.quantity_presentation_policy(),
        &rspice_results_ui::waves::as_returned,
    );
    let readings = [
        ("min", stats.min),
        ("max", stats.max),
        ("p-p", stats.max - stats.min),
        ("avg", stats.mean),
        ("rms", stats.rms),
    ];
    assert_eq!(
        rows.len(),
        readings.len(),
        "a ramp does not repeat: {rows:?}"
    );
    for (row, (suffix, expected)) in rows.iter().zip(readings) {
        assert_eq!(row.label, format!("V(out) {suffix}"));
        assert_eq!(
            row.keep,
            Some(format!(
                "{}(V(out))",
                if suffix == "p-p" { "pp" } else { suffix }
            ))
        );
        assert_eq!(
            &row.value,
            &model.format_trace_value(
                trace,
                expected,
                8,
                state.model.preferences.quantity_presentation_policy()
            )
        );
    }
    let full = trace_interval_statistics(
        &mut state.results.results.session.derived,
        model,
        trace,
        None,
        None,
    )
    .unwrap();
    let expression = calculator::parser::try_parse("rms(V(out))").unwrap();
    let calculator = calculator::evaluator::evaluate(
        &expression,
        &calculator::SimulationContext::new(&state.model.simulation),
    )
    .unwrap();
    assert!(
        matches!(calculator, calculator::CalcValue::Real(calculator::RealValue::Scalar(value)) if (value - full.rms).abs() < 1e-14)
    );
    let reversed = trace_interval_statistics(
        &mut state.results.results.session.derived,
        model,
        trace,
        Some((0.75, 0.25)),
        None,
    )
    .unwrap();
    assert_eq!(stats, reversed);
    assert!(
        trace_interval_statistics(
            &mut state.results.results.session.derived,
            model,
            trace,
            Some((f64::NAN, 0.5)),
            None,
        )
        .is_err()
    );
    assert_eq!(
        trace_interval_statistics(
            &mut state.results.results.session.derived,
            model,
            trace,
            Some((0.5, 0.5)),
            None,
        )
        .unwrap()
        .mean,
        0.5
    );
}

#[test]
fn measurement_readout_identifies_each_sweep_branch() {
    let mut state = branches::hysteresis_run();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let rows = readout::measurement_values(
        &mut state.results.results.session.derived,
        &models[0],
        Some((0.25, 0.75)),
        8,
        state.model.preferences.quantity_presentation_policy(),
        &rspice_results_ui::waves::as_returned,
    );
    assert_eq!(rows.len(), 10);
    assert!(rows[0].label.ends_with("fwd min"));
    assert!(rows[5].label.ends_with("rev min"));
    assert!(
        rows.iter().all(|row| row.keep.is_none()),
        "a branch has no name the calculator reads"
    );
    let model = &models[0];
    for (branch, expected) in [(0, (0.5, 1.5, 1.0)), (1, (2.5, 3.5, 3.0))] {
        let stats = trace_interval_statistics(
            &mut state.results.results.session.derived,
            model,
            &model.traces[0],
            Some((0.25, 0.75)),
            Some(branch),
        )
        .unwrap();
        assert_eq!((stats.min, stats.max, stats.mean), expected);
    }
}

#[test]
fn measurement_readout_explains_missing_coverage_instead_of_hiding_it() {
    let mut state = transient(
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![1.0, 1.0, f64::NAN, 1.0, 1.0],
    );
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let rows = readout::measurement_values(
        &mut state.results.results.session.derived,
        &models[0],
        None,
        8,
        state.model.preferences.quantity_presentation_policy(),
        &rspice_results_ui::waves::as_returned,
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].label, "V(out)");
    assert!(rows[0].value.contains("Unavailable"));
    assert!(rows[0].value.contains("50.0% interval coverage"));
}

#[test]
fn measurement_cursor_slope_accepts_resolvable_subpicosecond_intervals() {
    let mut state = transient(vec![0.0, 0.5e-12], vec![0.0, 0.1]);
    let presentation = state.model.preferences.result_presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    for (a, b) in [(0.0, 0.5e-12), (0.5e-12, 0.0)] {
        let rows = readout_rows(
            &models[0],
            CursorPair {
                a: Some(a),
                b: Some(b),
            },
            presentation.readout(),
            state.model.preferences.quantity_presentation_policy(),
            &rspice_results_ui::waves::as_returned,
        );
        // 0.1 V over 0.5 ps, with its SI prefix and no padding zeros.
        assert_eq!(rows[0].slope, "200 GV/s");
    }
    let coincident = readout_rows(
        &models[0],
        CursorPair {
            a: Some(0.0),
            b: Some(0.0),
        },
        presentation.readout(),
        state.model.preferences.quantity_presentation_policy(),
        &rspice_results_ui::waves::as_returned,
    );
    assert_eq!(coincident[0].slope, READOUT_ABSENT);
}

#[test]
fn interval_measurements_and_copied_values_cover_every_visible_trace() {
    // The readout used to measure the first four visible traces and copy the
    // first six, with nothing saying the rest were left out.
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    let traces = (1..=8)
        .map(|n| {
            RetainedTrace::new(
                format!("V(n{n})"),
                vec![0.0, 1.0],
                vec![0.0, f64::from(n)],
                "#fff",
            )
        })
        .collect();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(traces),
    );
    state.model.simulation.complete_run();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let model = &models[0];
    assert_eq!(model.traces.iter().filter(|trace| trace.visible).count(), 8);
    let policy = state.model.preferences.quantity_presentation_policy();
    let rows = readout::measurement_values(
        &mut state.results.results.session.derived,
        model,
        Some((0.25, 0.75)),
        8,
        policy,
        &rspice_results_ui::waves::as_returned,
    );
    for n in 1..=8 {
        for stat in ["min", "max", "p-p", "avg", "rms"] {
            let label = format!("V(n{n}) {stat}");
            assert!(rows.iter().any(|row| row.label == label), "no {label} row");
        }
    }
    let mut copied = String::new();
    append_copied_cursor(
        &mut copied,
        "A",
        0.5,
        model,
        SampleInterpolation::Linear,
        policy,
    );
    for n in 1..=8 {
        assert!(
            copied.contains(&format!("V(n{n}) = ")),
            "V(n{n}) not copied: {copied}"
        );
    }
}

/// The Measurements rows name each trace as the drawing spells its net, as
/// the legend does: the engine's `V(OUT)` for a net drawn `Out` reads
/// `V(Out) min`.
#[test]
fn a_measurement_row_names_its_trace_as_drawn() {
    let mut state = ResultsTestHost::default();
    state
        .model
        .schematic
        .add_net_label(crate::state::Point::new(0, 0), "Out".to_owned());
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(OUT)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    state.model.simulation.complete_run();
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    // The Measurements plan is made off the frame: its rows show on a frame
    // after it arrives.
    let mut painted = Vec::new();
    let shown = (0..3_000).any(|_| {
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                readout::right_panel(ui, &mut state.results_parts());
            });
        });
        painted = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let shown = painted.iter().any(|text| text == "V(Out) min");
        if !shown {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        shown
    });
    assert!(shown, "{painted:?}");
    assert!(
        !painted.iter().any(|text| text.contains("OUT")),
        "{painted:?}"
    );
}
