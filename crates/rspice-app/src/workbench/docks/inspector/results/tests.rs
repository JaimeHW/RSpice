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
