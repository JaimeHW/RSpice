//! Optimization: the optimizer's iterations, one row each.
//!
//! The columns are the traces the run retained against the iteration axis: the
//! cost first, then each design variable the optimizer moved. A trace that does
//! not share the iteration axis is not a column, because its samples would sit
//! beside iterations they were not taken at. The toolbar names the best
//! iteration and whether the optimizer converged.

use super::table::{Cell, Column, DataTable};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// An iteration number as a count, or in engineering notation when the
/// optimizer stepped by fractions.
fn iteration_text(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1.0e15 {
        format!("{}", value as i64)
    } else {
        format_scalar(value)
    }
}

/// The prefix the optimizer gives every trace it retains.
const TRACE_PREFIX: &str = "OPT_";
/// The trace that carries the cost.
const COST_TRACE: &str = "OPT_COST";

/// The optimizer's iterations as the table states them.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct IterationTable {
    pub(super) iterations: Vec<f64>,
    /// Each column's head and its value at every iteration; the cost first.
    pub(super) traces: Vec<(String, Vec<f64>)>,
    /// The row whose cost is lowest.
    pub(super) best: Option<usize>,
    pub(super) converged: Option<bool>,
}

impl IterationTable {
    pub(super) fn from_result(result: &crate::state::AnalysisResult) -> Option<Self> {
        let iterations = result.waveforms.first()?.x.as_ref().clone();
        if iterations.is_empty() {
            return None;
        }
        let mut traces = result
            .waveforms
            .iter()
            .filter(|waveform| {
                waveform.x.len() == iterations.len() && waveform.y.len() == iterations.len()
            })
            .map(|waveform| {
                let head = if waveform.name.eq_ignore_ascii_case(COST_TRACE) {
                    "Cost".to_owned()
                } else {
                    waveform
                        .name
                        .strip_prefix(TRACE_PREFIX)
                        .unwrap_or(&waveform.name)
                        .to_owned()
                };
                (head, waveform.y.as_ref().clone())
            })
            .collect::<Vec<_>>();
        traces.sort_by(|left, right| {
            (left.0 != "Cost")
                .cmp(&(right.0 != "Cost"))
                .then_with(|| left.0.cmp(&right.0))
        });
        let best = traces
            .iter()
            .find(|(head, _)| head == "Cost")
            .and_then(|(_, costs)| {
                costs
                    .iter()
                    .enumerate()
                    .filter(|(_, cost)| cost.is_finite())
                    .min_by(|left, right| left.1.total_cmp(right.1))
                    .map(|(index, _)| index)
            });
        let converged = match &result.family_metadata {
            Some(crate::state::AnalysisResultFamilyMetadata::Optimization {
                converged, ..
            }) => Some(*converged),
            _ => None,
        };
        Some(Self {
            iterations,
            traces,
            best,
            converged,
        })
    }

    /// The toolbar's line: convergence, and the best iteration's cost.
    pub(super) fn summary(&self) -> Summary {
        let count = self.iterations.len();
        let best = self.best.and_then(|row| {
            let cost = self
                .traces
                .iter()
                .find(|(head, _)| head == "Cost")
                .and_then(|(_, costs)| costs.get(row))?;
            Some(format!(
                "best cost {} at iteration {}",
                format_scalar(*cost),
                iteration_text(self.iterations[row])
            ))
        });
        let tail = format!("{count} iteration{}", if count == 1 { "" } else { "s" });
        let text = [
            self.converged.map(|converged| {
                if converged {
                    "converged"
                } else {
                    "not converged"
                }
                .to_owned()
            }),
            best,
            Some(tail),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" \u{00b7} ");
        if self.converged == Some(false) {
            Summary::warning(text)
        } else {
            Summary::plain(text)
        }
    }
}

/// What the optimization toolbar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptimizationAction {
    Configure,
    Run,
}

pub(super) fn optimization(ui: &mut Ui, app: &mut RSpiceApp) {
    let runs = run_choice(app, |run| {
        run.analyses.iter().any(|analysis| {
            analysis.analysis_type == crate::state::AnalysisType::Optimization
                && verified_analysis(analysis)
        })
    });
    let run_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let table = latest_analysis(app, crate::state::AnalysisType::Optimization)
        .and_then(IterationTable::from_result);
    let actions = [OptimizationAction::Configure, OptimizationAction::Run];
    let toolbar_actions = [
        ToolbarAction::new("Configure optimization"),
        ToolbarAction::new("Run optimization"),
    ];
    let summary = table.as_ref().map(|table| {
        let summary = table.summary();
        if runs.offers_choice() {
            summary
        } else {
            append_run(summary, run_name.as_deref())
        }
    });
    let output = PageToolbar {
        title: "Optimization",
        runs: &runs,
        summary,
        actions: &toolbar_actions,
    }
    .show(ui);
    let kind = crate::simulation::plan::AnalysisKind::Optimization;
    match output.action.and_then(|index| actions.get(index)) {
        Some(OptimizationAction::Configure) => {
            let result = open_analysis_configuration(app, kind);
            record_action(&mut app.state.workbench.verification, result);
        }
        Some(OptimizationAction::Run) => {
            let result = request_analysis_run(app, &[kind]);
            record_action(&mut app.state.workbench.verification, result);
        }
        None => {}
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.simulation, index);
    }
    action_problem_line(ui, app);

    let Some(table) = table else {
        let text = run_name.map_or_else(
            || "No optimization result. Configure the optimizer, then run it.".to_owned(),
            |run| format!("No optimization result in {run}. Configure the optimizer, then run it."),
        );
        let _ = empty_line(ui, &text, None);
        return;
    };
    let mut columns = vec![Column::number("Iteration", 80.0)];
    columns.extend(
        table
            .traces
            .iter()
            .map(|(head, _)| Column::number(head, 110.0)),
    );
    // The spare width goes to a trailing column of nothing rather than
    // stretching numbers away from their heads.
    columns.push(Column::name("", 0.0));
    let height = ui.available_height();
    let _ = DataTable::new(
        "verify.optimization.iterations",
        &columns,
        table.iterations.len(),
    )
    .max_height(height)
    .show(ui, |row| {
        let mut cells = vec![Cell::number(iteration_text(table.iterations[row]))];
        cells.extend(table.traces.iter().map(|(_, values)| {
            values
                .get(row)
                .filter(|value| value.is_finite())
                .map_or_else(Cell::absent, |value| Cell::number(format_scalar(*value)))
        }));
        cells.push(Cell::empty());
        cells
    });
}
