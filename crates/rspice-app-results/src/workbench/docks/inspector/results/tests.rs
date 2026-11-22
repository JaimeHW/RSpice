//! The result inspector reads the workspace's answers: a trace's statistics,
//! its cross-probe into Design, and the run details beside them.

use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn convergence_inspector_speaks_only_of_forced_acceptances() {
    use crate::state::{AnalysisResult, AnalysisType, TransientConvergenceEvidence};
    // A forced acceptance is a row on the section's label/value grid; no
    // evidence, or evidence that forced nothing, has no section at all.
    for (evidence, forced, expected) in [
        (false, false, None),
        (true, false, None),
        (true, true, Some("1 forced")),
    ] {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient");
        if evidence {
            let mut quality = rspice_core::diagnostics::ConvergenceQuality::default();
            if forced {
                quality.record_force_accept(1);
            }
            analysis.convergence = Some(std::sync::Arc::new(
                TransientConvergenceEvidence::capture(
                    quality,
                    &[0.0, 1.0],
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap(),
            ));
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                result_authority::result_convergence(ui, &analysis);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        let row = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("LTE acceptances"))
            .and_then(|(_, node)| node.value());
        assert_eq!(row, expected, "evidence {evidence}, forced {forced}");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dc_inspector_names_its_sources_and_primary_traversal() {
    use crate::simulation::controller::test_execution::dc_history::{nested_config, retain, solve};
    let mut config = nested_config();
    config.start = 1.0;
    config.stop = 0.0;
    config.step = -0.5;
    let analysis = retain(solve(config));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(312.0);
            result_authority::result_dc_sweep(ui, &analysis);
        });
    });
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    for expected in ["V1", "V2", "Descending", "3"] {
        assert!(
            nodes.iter().any(|(_, n)| n.value() == Some(expected)),
            "Missing DC evidence: {expected}"
        );
    }
}

#[test]
fn result_trace_statistics_ignore_non_finite_samples() {
    use crate::workbench::documents::result_document::trace_statistics::measure;

    let statistics = measure(
        &vec![0.0, 1.0, 2.0, 3.0, 4.0].into(),
        &vec![f64::NAN, -2.0, 1.0, f64::INFINITY, 4.0].into(),
    );
    assert_eq!(statistics.extremes, Some((-2.0, 4.0)));
    assert_eq!((statistics.finite, statistics.samples), (3, 5));
    assert!(
        statistics
            .average
            .as_ref()
            .is_err_and(|reason| reason.contains("missing or undefined samples")),
        "an average over holes is refused, as avg() refuses it: {:?}",
        statistics.average
    );
    assert_eq!(
        measure(
            &vec![0.0, 1.0].into(),
            &vec![f64::NAN, f64::INFINITY].into()
        )
        .extremes,
        None
    );
}

/// One frame of the inspector over `host` at 480 x 900, and the value of each
/// property row it drew, by label.
fn inspector_rows(
    ctx: &egui::Context,
    host: &mut crate::workbench::documents::result_document::ResultsTestHost,
) -> std::collections::HashMap<String, String> {
    let mut run_details = None;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(480.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                results(ui, &mut host.results_parts(), &mut run_details);
            });
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
        .iter()
        .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
        .collect()
}

/// A selected trace's Average is its average over time, as the calculator's
/// `avg()` takes it. A solver steps finely at an edge, so the mean of the
/// samples leans toward the edge: on an RC step sampled densely after the
/// step it read 0.78 V where the time average is 0.8013 V.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_selected_trace_states_its_average_over_time() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsTestHost};
    use rspice_app_state::workbench::preferences::ScalarPreference;
    use rspice_results_ui::selection::SelectedResultTrace;

    // V(out) = 1 - exp(-t / RC) over 5 ms, RC = 1 ms, the samples crowding
    // after the step as an adaptive step does. Over [0, T] its average is
    // 1 - (RC / T)(1 - exp(-T / RC)) = 0.8013476 V.
    let (rc, stop) = (1.0e-3, 5.0e-3);
    let time: Vec<f64> = (0..=4_000)
        .map(|step| stop * (f64::from(step) / 4_000.0).powi(3))
        .collect();
    let out: Vec<f64> = time.iter().map(|t| 1.0 - (-t / rc).exp()).collect();
    let mean_of_samples = out.iter().sum::<f64>() / out.len() as f64;
    assert!(
        mean_of_samples < 0.7,
        "the fixture leans: {mean_of_samples}"
    );
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![RetainedTrace::new("V(out)", time, out, "")]),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    host.results.results.session.selected_trace =
        Some(SelectedResultTrace::from_identity(analysis, "V(out)"));
    host.model
        .preferences
        .set_scalar(ScalarPreference::DisplayedSignificantDigits, 5)
        .expect("five digits");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let rows = inspector_rows(&ctx, &mut host);
    assert_eq!(rows.get("Average").map(String::as_str), Some("801.35 mV"));
    assert!(!rows.contains_key("Mean"), "{rows:?}");
}

/// A selected trace whose samples a results store keeps is measured off the
/// frame, once, streaming its columns: no frame reads them, its rows say they
/// are computing until the measurement is in, and then they state it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_stored_selected_trace_is_measured_without_a_frame_reading_its_columns() {
    use crate::workbench::documents::result_document::test_fixtures::stored_transient;
    use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsTestHost};
    use rspice_results::waveform::column_work::ColumnWork;
    use rspice_results_ui::selection::SelectedResultTrace;

    let time: Vec<f64> = (0..=10_000).map(|step| f64::from(step) * 1.0e-6).collect();
    let ramp: Vec<f64> = time.iter().map(|t| t * 1.0e3).collect();
    let (analysis, columns) =
        stored_transient(time, vec![("V(out)".to_owned(), ramp)], Some([7; 32]));
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(analysis);
    let run = host.model.simulation.active_run().expect("a run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    host.results.results.session.selected_trace =
        Some(SelectedResultTrace::from_identity(key, "V(out)"));
    // A sheet whose own readout measures nothing, so the frame's reads are
    // the inspector's.
    host.results.results.session.viewer =
        crate::workbench::documents::result_document::ResultViewer::Table;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let before = ColumnWork::read();
    let mut rows = inspector_rows(&ctx, &mut host);
    for _ in 0..500 {
        if rows
            .get("Average")
            .is_some_and(|value| !value.starts_with("Computing"))
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        rows = inspector_rows(&ctx, &mut host);
    }
    let work = before.since();
    assert_eq!(
        (work.stored_views, work.stored_reads),
        (0, 0),
        "a frame read a stored column whole"
    );
    // The ramp runs from 0 to 10 V over 10 ms: its average is 5 V.
    assert_eq!(rows.get("Average").map(String::as_str), Some("5 V"));
    assert_eq!(rows.get("Maximum").map(String::as_str), Some("10 V"));
    // Each pass reads the column a block at a time; this column is one block long.
    let passes = || {
        columns
            .iter()
            .map(|column| column.whole_reads())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        passes(),
        [2, 3],
        "the background measurement passed over the axis twice and the trace three times"
    );
    for _ in 0..10 {
        let _ = inspector_rows(&ctx, &mut host);
    }
    assert_eq!(passes(), [2, 3], "later frames measure nothing again");
}

/// Run details opens at once on a large run: the dataset digest, which reads
/// every sample the run retained, is computed off the frame and fills its row
/// when it is known.
#[test]
fn run_details_opens_without_reading_the_run_on_the_frame() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::ResultsTestHost;
    use rspice_results::waveform::column_work::ColumnWork;

    let time: Vec<f64> = (0..200_000).map(|step| f64::from(step) * 1.0e-9).collect();
    let wave: Vec<f64> = time.iter().map(|t| (t * 1.0e7).sin()).collect();
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![RetainedTrace::new("V(out)", time, wave, "#fff")]),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let (run_id, expected) = (run.id, run.dataset_content_digest().to_string());

    let before = ColumnWork::read();
    let mut run_details = None;
    super::super::run_details::open(&mut run_details, &host.model.simulation, run_id);
    assert_eq!(
        before.since().memory_views,
        0,
        "opening Run details read the run's samples on the frame"
    );
    let details = run_details.expect("Run details opens on a retained run");
    assert_eq!(details.run_id, run_id);
    for _ in 0..500 {
        if details.dataset_digest().is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert_eq!(details.dataset_digest(), Some(expected.as_str()));
}

/// The Inspector's colour swatch for a selected trace is the colour the strip
/// drew its curve in. The strip gives curves their palette places in the
/// order they were plotted; the swatch took its column's place, so with the
/// first column off the plot it showed V(c) in the third hue over a curve
/// drawn in the second.
#[test]
fn the_colour_swatch_is_the_colour_the_strip_drew_the_curve_in() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::browser_plot::last_strip_models;
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, ResultsTestHost, waves,
    };
    use rspice_results_ui::selection::SelectedResultTrace;

    let trace = |name: &str, visible: bool| {
        let mut trace = RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "");
        trace.visible = visible;
        trace
    };
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            trace("V(a)", false),
            trace("V(b)", true),
            trace("V(c)", true),
        ]),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    host.results.results.session.selected_trace =
        Some(SelectedResultTrace::from_identity(analysis, "V(c)"));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut swatch = None;
    for _ in 0..2 {
        let mut run_details = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::Panel::right("inspector")
                    .exact_size(320.0)
                    .show(ctx, |ui| {
                        results(ui, &mut host.results_parts(), &mut run_details);
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    waves::show(ui, &mut host.results_parts());
                });
            },
        );
        swatch = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .find(|(_, node)| node.label() == Some("Color"))
            .and_then(|(_, node)| node.value().map(str::to_owned));
    }
    let parts = host.results_parts();
    let drawn = last_strip_models(&parts)
        .and_then(|models| models.iter().find(|model| model.analysis_key == analysis))
        .and_then(|model| model.traces.iter().find(|trace| trace.waveform_index == 2))
        .expect("the strip drew V(c)")
        .color;
    assert_eq!(
        swatch.as_deref(),
        Some(format!("#{:02X}{:02X}{:02X}", drawn.r(), drawn.g(), drawn.b()).as_str()),
        "the swatch is not the curve's colour"
    );
    let column = crate::ui::tokens::Tokens::get(&ctx).color.traces[2];
    assert_ne!(column, drawn, "the fixture tells the two apart");
}

/// A selected trace's statistics read to the reader's significant digits,
/// as the measurement rows beside them do, and follow the reader's choice.
/// They were held at four ("490.2 mV") beside measurements at seven
/// ("490.2291 mV").
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn trace_statistics_read_to_the_readers_significant_digits() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsTestHost};
    use rspice_app_state::workbench::preferences::ScalarPreference;
    use rspice_results_ui::selection::SelectedResultTrace;

    let samples = vec![0.490_229_1, 0.512_345_6, 0.734_567_8];
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], samples, ""),
        ]),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    host.results.results.session.selected_trace =
        Some(SelectedResultTrace::from_identity(analysis, "V(out)"));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let minimum = |host: &mut ResultsTestHost| {
        let mut value = None;
        for _ in 0..2 {
            let mut run_details = None;
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(480.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        results(ui, &mut host.results_parts(), &mut run_details);
                    });
                },
            );
            value = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                .find(|(_, node)| node.label() == Some("Minimum"))
                .and_then(|(_, node)| node.value().map(str::to_owned));
        }
        value.expect("the Minimum row")
    };
    let digits = host
        .model
        .preferences
        .scalar(ScalarPreference::DisplayedSignificantDigits);
    assert_eq!(digits, 4, "the reader's default");
    assert_eq!(minimum(&mut host), "490.2 mV");
    host.model
        .preferences
        .set_scalar(ScalarPreference::DisplayedSignificantDigits, 5)
        .expect("five digits");
    assert_eq!(minimum(&mut host), "490.23 mV");
}
