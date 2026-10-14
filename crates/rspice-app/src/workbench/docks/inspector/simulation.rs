//! What the inspector says about the analysis a reader has selected: how its
//! last run went, how long it took, and the way to its results, plus anything
//! wrong with its configuration. The setup itself is the Analyses page's.

use egui::Ui;

use crate::state::{SimulationRun, SimulationRunLifecycle};
use crate::ui::plot::fmt_si_significant;
use crate::ui::widgets::problem_line;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;

use super::super::super::design_system::{
    property_row, property_row_problem, section_header, section_header_action,
};
use super::{Problem, muted_inspector_copy};

/// The selected analysis's last run, as the inspector states it.
struct LastRun {
    /// The run's display sequence, which the results link selects.
    sequence: u64,
    has_results: bool,
    /// Wall time, once the run has finished.
    runtime: Option<String>,
    status: &'static str,
    problem: Option<Problem>,
    /// The engine's message when the analysis failed.
    message: Option<String>,
}

/// The newest retained run that ran analysis `instance`: by the provenance of
/// a result it produced, or by the prepared task when it produced none.
fn last_run_of(
    app: &RSpiceApp,
    instance: crate::product::AnalysisInstanceId,
) -> Option<&SimulationRun> {
    app.state.simulation.retained.runs.iter().find(|run| {
        run.analyses.iter().any(|analysis| {
            analysis
                .provenance()
                .is_some_and(|provenance| provenance.source_instance_id() == instance)
        }) || run.prepared_receipt().is_some_and(|receipt| {
            receipt
                .tasks()
                .iter()
                .any(|task| task.instance_id() == instance)
        })
    })
}

/// How analysis `instance` came out of `run`: the status word, the problem it
/// carries, and the engine's own message when it failed.
fn run_outcome(
    run: &SimulationRun,
    instance: crate::product::AnalysisInstanceId,
) -> (&'static str, Option<Problem>, Option<&str>) {
    match run.lifecycle {
        SimulationRunLifecycle::Preparing => return ("Preparing", None, None),
        SimulationRunLifecycle::Running => return ("Running", None, None),
        SimulationRunLifecycle::Cancelling => return ("Stopping", None, None),
        SimulationRunLifecycle::Aborted => return ("Stopped", None, None),
        SimulationRunLifecycle::Interrupted => {
            return ("Interrupted", Some(Problem::Warning), None);
        }
        SimulationRunLifecycle::LegacyUnknown
        | SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed => {}
    }
    let analysis = run.analyses.iter().find(|analysis| {
        analysis
            .provenance()
            .is_some_and(|provenance| provenance.source_instance_id() == instance)
    });
    let succeeded = analysis.map_or(run.success, |analysis| analysis.success);
    if succeeded {
        ("Complete", None, None)
    } else {
        (
            "Failed",
            Some(Problem::Failure),
            analysis.and_then(|analysis| analysis.error_message.as_deref()),
        )
    }
}

pub(super) fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            let legacy_kind = crate::simulation::plan::AnalysisKind::from_legacy_index(
                app.state.workbench.active_analysis,
            );
            plan.instances()
                .iter()
                .find(|instance| {
                    Some(instance.id()) == app.state.workbench.active_analysis_instance
                })
                .or_else(|| {
                    legacy_kind.and_then(|kind| {
                        plan.instances()
                            .iter()
                            .find(|instance| instance.kind() == kind)
                    })
                })
                .or_else(|| plan.instances().first())
                .map(|instance| {
                    (
                        instance.id(),
                        instance.kind(),
                        instance.display_name().to_owned(),
                        instance.draft().clone(),
                    )
                })
        });
    let Some((id, kind, name, draft)) = selected else {
        muted_inspector_copy(ui, "Add an analysis to start.");
        return;
    };
    app.state.workbench.active_analysis_instance = Some(id);
    app.state.workbench.active_analysis = kind.legacy_index();

    let validation_error = app.state.sim_setup.analysis_draft_validation_error(&draft);
    let last_run = last_run_of(app, id).map(|run| {
        let (status, problem, message) = run_outcome(run, id);
        LastRun {
            sequence: run.id,
            has_results: !run.analyses.is_empty(),
            runtime: run
                .lifecycle
                .is_terminal()
                .then(|| fmt_si_significant(run.elapsed_time, "s", 3)),
            status,
            problem,
            message: message.map(str::to_owned),
        }
    });

    let Some(LastRun {
        sequence,
        has_results,
        runtime,
        status,
        problem,
        message,
    }) = last_run
    else {
        section_header(ui, &name, None);
        muted_inspector_copy(ui, "Not run yet.");
        if let Some(error) = validation_error {
            problem_line(ui, Problem::Failure, &error);
        }
        return;
    };
    if !has_results {
        section_header(ui, &name, None);
    } else if section_header_action(ui, &name, "Open results", true).clicked()
        && app.state.simulation.select_run_by_sequence(sequence)
    {
        Command::OpenWorkspace(Workspace::Results).execute(app);
    }
    property_row_problem(ui, "Status", status, problem);
    if let Some(message) = message {
        muted_inspector_copy(ui, &message);
    }
    if let Some(runtime) = runtime {
        property_row(ui, "Runtime", &runtime);
    }
    if let Some(error) = validation_error {
        problem_line(ui, Problem::Failure, &error);
    }
}

#[cfg(test)]
mod tests;
