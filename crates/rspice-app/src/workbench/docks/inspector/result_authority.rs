//! What the Results inspector says about the run behind the active result:
//! which run it is and how long it took, the analysis's own sweep, and the
//! solver's forced acceptances when there were any. Everything that only
//! proves where the run came from is in Run details….

use egui::Ui;

use super::super::super::design_system::{property_row, property_row_problem, section_header};
use super::section_block_gap;
use crate::ui::widgets::Button;
use rspice_ui_kit::panels::Problem;

/// The Run section: the run's name and duration, its state when it did not
/// complete, and the route to its details. Returns the run whose details
/// the reader asked for.
pub(super) fn result_run(
    ui: &mut Ui,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
) -> Option<u64> {
    use crate::state::SimulationRunLifecycle;

    section_header(ui, "Run", None);
    property_row(ui, "Dataset", &manifest.run_label);
    property_row(ui, "Duration", &manifest.elapsed_time);
    let problem = match run.lifecycle {
        SimulationRunLifecycle::Completed => None,
        SimulationRunLifecycle::Failed => Some(Some(Problem::Failure)),
        SimulationRunLifecycle::Aborted | SimulationRunLifecycle::Interrupted => {
            Some(Some(Problem::Warning))
        }
        _ => Some(None),
    };
    if let Some(problem) = problem {
        property_row_problem(ui, "State", &manifest.lifecycle, problem);
    }
    section_block_gap(ui);
    Button::new("Run details\u{2026}")
        .show(ui)
        .clicked()
        .then_some(run.id)
}

/// A DC sweep's primary and secondary sources and the order it solved them
/// in. A single sweep is the normal case and says so by naming nothing more.
pub(super) fn result_dc_sweep(ui: &mut Ui, analysis: &crate::state::AnalysisResult) {
    use crate::state::{AnalysisResultPayload, AnalysisType, DcSweepFamily};
    if analysis.analysis_type != AnalysisType::DcSweep || !analysis.success {
        return;
    }
    let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
        return;
    };
    section_header(ui, "DC sweep", None);
    property_row(ui, "Primary source", &evidence.source);
    property_row(ui, "Primary traversal", evidence.direction.label());
    match &evidence.family {
        DcSweepFamily::Single => {}
        DcSweepFamily::Retraced => {
            property_row(ui, "Sweep family", "Forward and reverse continuation");
        }
        DcSweepFamily::Nested { source, values } => {
            property_row(ui, "Secondary source", source);
            property_row(ui, "Secondary points", &values.len().to_string());
        }
    }
}

/// The points the solver accepted past its truncation-error criterion, when
/// there were any. A run that forced none has nothing to say here; the full
/// statistics are in Run details….
pub(super) fn result_convergence(ui: &mut Ui, analysis: &crate::state::AnalysisResult) {
    let Some(quality) = analysis.convergence.as_deref() else {
        return;
    };
    let forced = forced_lte_acceptances(quality);
    if forced == 0 {
        return;
    }
    section_header(ui, "Transient convergence", None);
    property_row(ui, "LTE acceptances", &format!("{forced} forced"))
        .on_hover_text("LTE is local truncation error.");
}

/// Every point any phase of the run accepted past its LTE criterion: the
/// source transient's and, for a periodic run, its initialization's.
pub(super) fn forced_lte_acceptances(
    quality: &rspice_results::convergence_quality::TransientConvergenceEvidence,
) -> u64 {
    quality.transient.force_accepted_points
        + quality.initialization.as_ref().map_or(0, |initialization| {
            initialization.report.force_accepted_points
        })
}
