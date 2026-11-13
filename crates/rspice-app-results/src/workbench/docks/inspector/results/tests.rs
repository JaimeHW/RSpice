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
    let statistics = finite_trace_statistics(&[f64::NAN, -2.0, 1.0, f64::INFINITY, 4.0])
        .expect("three finite samples");

    assert_eq!(statistics.minimum, -2.0);
    assert_eq!(statistics.maximum, 4.0);
    assert_eq!(statistics.mean, 1.0);
    assert!(finite_trace_statistics(&[f64::NAN, f64::INFINITY]).is_none());
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
            .find(|(_, node)| node.label() == Some("Colour"))
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
