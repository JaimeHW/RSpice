//! What the Results inspector says about the run behind the active result:
//! which run it is and how long it took, the outputs the analysis saved, the
//! analysis's own sweep, and the solver's forced acceptances when there were
//! any. Everything that only proves where the run came from is in Run
//! details….

use egui::{Pos2, Rect, Ui};

use super::super::super::design_system::{
    panel_inset, property_row, property_row_problem, section_header,
};
use crate::ui::widgets::Button;
use crate::workbench::documents::result_document::operational_state::RunMark;
use rspice_app_workbench::workbench::docks::navigator::grouped_count;
use rspice_results::saved_output::{SavedOutputMaterializationStatus, SavedOutputReceipt};
use rspice_results_ui::operational::{self, OperationalResponse};
use rspice_ui_kit::panels::Problem;
use rspice_ui_kit::panels::section_block_gap;

/// How long the run took; while it runs, how long it has run so far, counted on as the frame is
/// drawn again.
pub(super) fn run_duration(
    ui: &Ui,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
) -> String {
    let duration = duration_words(run, manifest);
    if run_is_going(run) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(100));
    }
    duration
}

fn run_is_going(run: &crate::state::SimulationRun) -> bool {
    use crate::state::SimulationRunLifecycle;
    matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    )
}

fn duration_words(
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
) -> String {
    if run_is_going(run)
        && let Ok(now) = rspice_app_state::time_compat::checked_unix_epoch()
    {
        let elapsed = (now.as_secs_f64() - run.timestamp).max(0.0);
        return format!("{elapsed:.1} s so far");
    }
    manifest.elapsed_time.clone()
}

/// The Run section: the run's name and duration, its state when it did not
/// complete, and the route to its details. Returns the run whose details
/// the reader asked for.
pub(super) fn result_run(
    ui: &mut Ui,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
    configurations: &rspice_design::configuration_set::ConfigurationSetCatalog,
    mark: Option<&RunMark>,
) -> (Option<u64>, OperationalResponse) {
    use crate::state::SimulationRunLifecycle;

    section_header(ui, "Run", None);
    property_row(ui, "Dataset", &manifest.run_label);
    // The run's state, while it is not complete: the mark its row in the
    // data browser carries, else the words of a run that ended short.
    let mut asked = OperationalResponse::default();
    let problem = match run.lifecycle {
        SimulationRunLifecycle::Completed => None,
        SimulationRunLifecycle::Failed => Some(Some(Problem::Failure)),
        SimulationRunLifecycle::Aborted | SimulationRunLifecycle::Interrupted => {
            Some(Some(Problem::Warning))
        }
        _ => Some(None),
    };
    if let Some(mark) = mark {
        let row = property_row(ui, "State", "");
        let (_, value_left) =
            rspice_ui_kit::panels::property_row_control_columns(ui, row.rect.width());
        // The mark's glyph stands where a value's first letter does.
        let left = row.rect.left() + value_left - operational::OPERATIONAL_MARK_PADDING_X;
        let slot = Rect::from_min_max(
            Pos2::new(left, row.rect.top()),
            Pos2::new(left + mark.width(ui), row.rect.bottom()),
        );
        asked = mark.show_in(ui, slot, run.dataset_id);
    } else if let Some(problem) = problem {
        property_row_problem(ui, "State", &manifest.lifecycle, problem);
    }
    if let Some(recorded) = run
        .prepared_receipt()
        .and_then(|receipt| receipt.configuration())
    {
        // A result from a configuration that moved since is one to look at
        // before relying on: the mark says so, the words say how.
        use crate::state::workspace::RecordedConfigurationStanding;
        let standing =
            crate::state::workspace::recorded_configuration_standing(configurations, recorded);
        property_row_problem(
            ui,
            "Configuration",
            &crate::state::workspace::recorded_configuration_words(configurations, recorded),
            (standing != RecordedConfigurationStanding::AsRun).then_some(Problem::Warning),
        );
    }
    property_row(ui, "Duration", &run_duration(ui, run, manifest));
    section_block_gap(ui);
    let details = Button::new("Run details\u{2026}")
        .show(ui)
        .clicked()
        .then_some(run.id);
    (details, asked)
}

/// The analysis's saved outputs, one row each: the output's name, then the
/// points it holds, or why it holds none. A deferred output keeps its recipe
/// rather than its samples, and its row carries the action that evaluates it.
/// Returns the receipt the reader asked to materialize.
pub(super) fn saved_outputs(ui: &mut Ui, receipts: &[SavedOutputReceipt]) -> Option<usize> {
    if receipts.is_empty() {
        return None;
    }
    section_header(ui, "Saved outputs", None);
    let mut requested = None;
    for (index, receipt) in receipts.iter().enumerate() {
        let (status, detail) = saved_output_status(&receipt.status);
        let row = property_row(ui, &receipt.name, &status);
        let row = match detail {
            Some(detail) => row.on_hover_text(detail),
            None => row,
        };
        if receipt.status == SavedOutputMaterializationStatus::Deferred
            && materialize_action(ui, row.rect, &receipt.name)
        {
            requested = Some(index);
        }
    }
    requested
}

/// What a saved output holds, and the reason behind a status that holds no
/// points.
fn saved_output_status(status: &SavedOutputMaterializationStatus) -> (String, Option<String>) {
    let points = |count: u64| {
        format!(
            "{} pts",
            grouped_count(usize::try_from(count).unwrap_or(usize::MAX))
        )
    };
    match status {
        SavedOutputMaterializationStatus::Materialized { sample_count, .. } => {
            (points(*sample_count), None)
        }
        SavedOutputMaterializationStatus::MaterializedDcFamily { members } => (
            points(members.iter().fold(0_u64, |total, member| {
                total.saturating_add(member.sample_count)
            })),
            Some(format!(
                "Evaluated for each of the sweep's {} DC members.",
                members.len()
            )),
        ),
        SavedOutputMaterializationStatus::Deferred => (
            "Deferred".to_owned(),
            Some(
                "The recipe is retained rather than the samples. Materialize evaluates it \
                 against this run's retained data."
                    .to_owned(),
            ),
        ),
        SavedOutputMaterializationStatus::SuppressedOnSuccess => (
            "Suppressed".to_owned(),
            Some("A failure-only output, inactive because the analysis succeeded.".to_owned()),
        ),
        SavedOutputMaterializationStatus::Unavailable { reason } => {
            ("Unavailable".to_owned(), Some(reason.clone()))
        }
    }
}

/// A deferred output's Materialize, right-aligned in the value column of the
/// row it acts on. Returns whether it was pressed.
fn materialize_action(ui: &mut Ui, row: Rect, name: &str) -> bool {
    let accessible = format!("Materialize {name}");
    let button = Button::new("Materialize")
        .compact()
        .ghost()
        .accessible_label(&accessible);
    let width = button.measured_width(ui);
    let right = row.right() - panel_inset(ui);
    let slot = Rect::from_min_max(
        Pos2::new(right - width, row.top()),
        Pos2::new(right, row.bottom()),
    );
    let clicked = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(slot)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                button
                    .show(ui)
                    .on_hover_text("Evaluate this output from the run's retained data.")
                    .clicked()
            },
        )
        .inner;
    // Placing the action left the cursor under it; the row ends at its own
    // bottom.
    let _ = ui.advance_cursor_after_rect(row);
    clicked
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

#[cfg(test)]
mod duration_tests {
    use super::*;
    use crate::state::{SimulationRun, SimulationRunLifecycle};
    use rspice_app_state::time_compat::with_unix_epoch;
    use rspice_results::manifest::ManifestViewModel;
    use std::time::Duration;

    #[test]
    fn a_running_runs_duration_counts_on_until_it_ends() {
        let mut run = SimulationRun::new(1);
        run.timestamp = 1_000.0;
        run.lifecycle = SimulationRunLifecycle::Running;
        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());
        let at = |seconds: f64| {
            with_unix_epoch(Ok(Duration::from_secs_f64(seconds)), || {
                duration_words(&run, &manifest)
            })
        };
        assert_eq!(at(1_002.5), "2.5 s so far");
        assert_eq!(at(1_007.0), "7.0 s so far");

        run.lifecycle = SimulationRunLifecycle::Completed;
        run.elapsed_time = 7.25;
        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());
        assert_eq!(
            with_unix_epoch(Ok(Duration::from_secs(2_000)), || duration_words(
                &run, &manifest
            )),
            "7.250 s",
            "a finished run states the time it took"
        );
    }
}
