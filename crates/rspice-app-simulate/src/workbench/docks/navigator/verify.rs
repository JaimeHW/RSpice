//! The Verify workspace's navigator: one row per view, each with the one line
//! its model has to say, and the plan's runs under them.
//!
//! Every second line is counted from what its view draws — the specification
//! sheet, its points and trials, the run's checks, the regression receipt —
//! so the navigator cannot state a count its view does not. A view with
//! nothing to say has no second line, and one that fails leads with the
//! problem's mark.

use egui::Ui;

use crate::product::RunId;
use rspice_results::verification::{CellStatus, CheckStatus, JointYield, RowStatus};

use crate::state::SimulationRunLifecycle;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::app_state::Model;
use crate::workbench::slots::SimulateState;
use crate::workbench::surfaces::verify::specifications::{counts_line, counts_problem};
use crate::workbench::surfaces::verify::{reference_eligibility, reference_run};

use super::super::super::design_system::section_header;
use super::flow_row::flow_row;
use crate::state::VerificationPage;

/// Room the Runs section needs under the views before the navigator scrolls:
/// its header and two runs.
const RUNS_HEIGHT: f32 = 126.0;

/// Whether a navigator `available_height` tall keeps its scroll bar showing:
/// the views and the runs under them need more.
pub fn verification_navigator_requires_scroll(available_height: f32) -> bool {
    let flow_height = VerificationPage::NAVIGATION.len() as f32 * super::FLOW_ROW_HEIGHT;
    available_height < flow_height + RUNS_HEIGHT
}

/// What the Verify workspace's navigator reads: the model, and the view and
/// regression receipt the session holds.
pub(in crate::workbench) struct VerifyNavigator<'a> {
    pub(in crate::workbench) model: &'a Model,
    pub(in crate::workbench) simulate: &'a SimulateState,
}

/// What the reader asked of the navigator, carried out once it has drawn.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) struct VerifyNavigatorAsk {
    /// The view to open.
    pub(in crate::workbench) open: Option<VerificationPage>,
    /// The retained run, by index, to make the one Verify judges.
    pub(in crate::workbench) run: Option<usize>,
    /// The run to make the plan's reference.
    pub(in crate::workbench) reference: Option<RunId>,
    /// The retained run, by index, to open in Results.
    pub(in crate::workbench) results: Option<usize>,
}

/// Draw the views and the runs, and say what the reader asked of them.
pub(in crate::workbench) fn verify(
    ui: &mut Ui,
    navigator: &VerifyNavigator<'_>,
) -> VerifyNavigatorAsk {
    let mut ask = VerifyNavigatorAsk::default();
    let mut active = navigator.simulate.verification_page;
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    for page in VerificationPage::NAVIGATION {
        let flow = verification_flow_presentation(navigator.model, navigator.simulate, page);
        if flow_row(ui, &flow, active == page).clicked() {
            active = page;
            ask.open = Some(page);
        }
    }
    runs(ui, navigator, &mut ask);
    ask
}

/// The plan's retained runs, newest first: the one Verify judges with its
/// roll-up, every other with the word for how it ended, and the reference
/// marked. A run's menu makes it the reference or opens it in Results.
fn runs(ui: &mut Ui, navigator: &VerifyNavigator<'_>, ask: &mut VerifyNavigatorAsk) {
    let model = navigator.model;
    let plan_id = model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id());
    let mut runs = model
        .simulation
        .retained
        .runs
        .iter()
        .enumerate()
        .filter(|(_, run)| run.evidence_domain(plan_id).answers_a_plan_limit())
        .collect::<Vec<_>>();
    if runs.is_empty() {
        return;
    }
    runs.sort_by(|(_, left), (_, right)| right.id.cmp(&left.id));
    section_header(ui, "Runs", None);
    let active = model.simulation.view.active_run_idx;
    let reference = reference_run(model, navigator.simulate);
    for (index, run) in runs {
        let selected = active == Some(index);
        let (detail, problem) = match model.simulation.verification_sheet() {
            Some(sheet) if selected => (counts_line(sheet), counts_problem(sheet)),
            _ => run_outcome(run),
        };
        let flow = VerificationFlowPresentation {
            label: run.label.clone(),
            detail,
            status: if reference == Some(run.run_id) {
                "reference".to_owned()
            } else {
                String::new()
            },
            problem,
        };
        let response = ui
            .push_id(("verify.runs", run.run_id), |ui| {
                flow_row(ui, &flow, selected)
            })
            .inner;
        if response.clicked() && !selected {
            ask.run = Some(index);
        }
        let eligible = reference_eligibility(model, run.run_id);
        egui::Popup::context_menu(&response).show(|ui| {
            ui.set_min_width(180.0);
            let set = ui.add_enabled(
                eligible.is_ok() && reference != Some(run.run_id),
                egui::Button::new("Set as reference"),
            );
            let set = match &eligible {
                Err(reason) => set.on_disabled_hover_text(reason),
                Ok(()) => set,
            };
            if set.clicked() {
                ask.reference = Some(run.run_id);
                ui.close();
            }
            if ui.button("Open in Results").clicked() {
                ask.results = Some(index);
                ui.close();
            }
        });
    }
}

/// How a run that Verify is not judging ended, in a word, and the problem it
/// leads with when it failed or its acceptance is blocked.
fn run_outcome(run: &crate::state::SimulationRun) -> (String, Option<Problem>) {
    let (word, problem) = match run.lifecycle {
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => {
            match run.specification_verdicts() {
                None => ("completed", None),
                Some(_) if run.specification_acceptance_is_blocked() => {
                    ("blocked", Some(Problem::Failure))
                }
                Some(_) => ("accepted", None),
            }
        }
        SimulationRunLifecycle::Failed => ("failed", Some(Problem::Failure)),
        SimulationRunLifecycle::Aborted => ("cancelled", None),
        SimulationRunLifecycle::Interrupted => ("interrupted", Some(Problem::Warning)),
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => ("running", None),
    };
    (word.to_owned(), problem)
}

/// One view's row: its name, the one line its model has to say, and the
/// problem it leads with.
#[derive(Clone)]
pub(in crate::workbench) struct VerificationFlowPresentation {
    pub(in crate::workbench) label: String,
    pub(in crate::workbench) detail: String,
    pub(in crate::workbench) status: String,
    pub(in crate::workbench) problem: Option<Problem>,
}

pub(in crate::workbench) fn verification_flow_presentation(
    model: &Model,
    simulate: &SimulateState,
    page: VerificationPage,
) -> VerificationFlowPresentation {
    let (detail, problem) = match page {
        VerificationPage::Specifications => model
            .simulation
            .verification_sheet()
            .map_or((String::new(), None), |sheet| {
                (counts_line(sheet), counts_problem(sheet))
            }),
        VerificationPage::Corners => corners_line(model),
        VerificationPage::MonteCarlo => monte_carlo_line(model),
        VerificationPage::Checks => checks_line(model),
        VerificationPage::Reference => reference_line(model, simulate),
    };
    VerificationFlowPresentation {
        label: page.label().to_owned(),
        detail,
        status: String::new(),
        problem,
    }
}

/// "7 points · 1 fails": the points the run ran at, and how many of them some
/// specification fails or errs at. A run of one point has none to compare.
fn corners_line(model: &Model) -> (String, Option<Problem>) {
    let Some(sheet) = model.simulation.verification_sheet() else {
        return (String::new(), None);
    };
    let points = sheet.points.len();
    if points < 2 {
        return (String::new(), None);
    }
    let failing = (0..points)
        .filter(|point| {
            sheet.rows.iter().any(|row| {
                row.cells
                    .get(*point)
                    .is_some_and(|cell| matches!(cell.status, CellStatus::Fail | CellStatus::Error))
            })
        })
        .count();
    if failing == 0 {
        (format!("{points} points"), None)
    } else {
        (
            format!(
                "{points} points \u{b7} {failing} {}",
                if failing == 1 { "fails" } else { "fail" }
            ),
            Some(Problem::Failure),
        )
    }
}

/// "92.5 % joint yield · 40 trials": the share of trials that pass every
/// specification with a limit, or how many trials ran when no one share can
/// be stated.
fn monte_carlo_line(model: &Model) -> (String, Option<Problem>) {
    let Some(sheet) = model.simulation.verification_sheet() else {
        return (String::new(), None);
    };
    let failing = sheet.rows.iter().any(|row| {
        row.statistics.is_some() && matches!(row.status, Some(RowStatus::Fail | RowStatus::Error))
    });
    let problem = failing.then_some(Problem::Failure);
    match sheet.joint_yield {
        Some(JointYield::Counted { passing, total, .. }) if total > 0 => (
            format!(
                "{:.1} % joint yield \u{b7} {total} trials",
                passing as f64 / total as f64 * 100.0
            ),
            problem,
        ),
        _ => sheet
            .rows
            .iter()
            .filter_map(|row| row.statistics.as_ref())
            .map(|statistics| statistics.trials.len())
            .max()
            .filter(|trials| *trials > 0)
            .map_or((String::new(), None), |trials| {
                (format!("{trials} trials"), problem)
            }),
    }
}

/// "2 violations · 14 checks": how many of the run's checks fail, of how many.
fn checks_line(model: &Model) -> (String, Option<Problem>) {
    let checks = model.simulation.verification_checks();
    let total = checks.rows.len();
    if total == 0 {
        return (String::new(), None);
    }
    let violations = checks
        .rows
        .iter()
        .filter(|row| row.status == CheckStatus::Fail)
        .count();
    let line = match violations {
        0 => format!("{total} check{}", if total == 1 { "" } else { "s" }),
        _ => format!(
            "{violations} violation{} \u{b7} {total} checks",
            if violations == 1 { "" } else { "s" }
        ),
    };
    let problem = match checks.worst() {
        CheckStatus::Fail => Some(Problem::Failure),
        CheckStatus::Warning | CheckStatus::Error => Some(Problem::Warning),
        CheckStatus::Pass | CheckStatus::Info => None,
    };
    (line, problem)
}

/// "1 changed · Run 9": how many checks moved against the reference run in
/// the comparison of the run Verify judges, or the reference run alone when
/// that comparison has not been made.
fn reference_line(model: &Model, simulate: &SimulateState) -> (String, Option<Problem>) {
    let Some(reference) = reference_run(model, simulate) else {
        return (String::new(), None);
    };
    let Some(run) = model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.run_id == reference)
    else {
        return (String::new(), None);
    };
    let compared = simulate
        .verification
        .regression_comparison
        .as_ref()
        .filter(|receipt| {
            receipt.baseline_run == reference
                && model
                    .simulation
                    .active_run()
                    .is_some_and(|active| active.run_id == receipt.candidate_run)
        });
    match compared {
        Some(receipt) => (
            format!("{} changed \u{b7} Run {}", receipt.changed_checks, run.id),
            (receipt.failed_checks + receipt.failed_waveforms > 0).then_some(Problem::Failure),
        ),
        None => (format!("Run {}", run.id), None),
    }
}
