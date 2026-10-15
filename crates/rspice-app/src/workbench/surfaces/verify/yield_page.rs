//! Monte Carlo yield: the ADE yield view of one run's Monte Carlo trials.
//!
//! One row per specification the trials were judged against. Its statistics
//! — mean, standard deviation, extremes, capability — are computed from the
//! exact samples the run retained, never from a summary that may disagree with
//! them; a specification whose samples were not kept shows a dash rather than a
//! number nobody can check. Cpk is the one-sided index against a one-sided
//! limit, as a capability study of a single limit publishes it.
//!
//! The toolbar's yield is the joint one: a trial passes only when it meets
//! every specification, so it can be lower than every row's own yield. The
//! selected row's distribution is drawn under the table; until the reader picks
//! a row, the row with the lowest yield is the one shown.

use rspice_results::yield_analysis::YieldResult;

use super::histogram::{HistogramModel, histogram_model, paint_histogram};
use super::table::{Cell, Column, DataTable, TABLE_HEAD_H, TABLE_ROW_H, Verdict};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// Least and greatest height of the distribution pane under the table.
const DETAIL_MIN_H: f32 = 220.0;
const DETAIL_MAX_H: f32 = 340.0;

/// One distribution's statistics, computed from its samples.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SampleStats {
    pub(super) mean: f64,
    pub(super) std_dev: f64,
    pub(super) min: f64,
    pub(super) max: f64,
    pub(super) cpk: Option<f64>,
}

/// One specification's row.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct YieldRow {
    pub(super) output: String,
    /// The limit, as a specification, so it is spelled the way every other
    /// surface spells a limit.
    pub(super) limit: SpecEntry,
    pub(super) stats: Option<SampleStats>,
    pub(super) passing: usize,
    pub(super) total: usize,
}

impl YieldRow {
    pub(super) fn from_result(result: &YieldResult) -> Self {
        let samples = result
            .samples
            .iter()
            .copied()
            .filter(|sample| sample.is_finite())
            .collect::<Vec<_>>();
        let stats = (!samples.is_empty()).then(|| {
            let computed = rspice_results::yield_analysis::calculate_distribution_stats(
                &samples,
                &result.spec,
            );
            SampleStats {
                mean: computed.mean,
                std_dev: computed.std_dev,
                min: computed.min,
                max: computed.max,
                cpk: computed.cpk.filter(|cpk| cpk.is_finite()),
            }
        });
        Self {
            output: result.spec.target.clone(),
            limit: SpecEntry {
                measurement: result.spec.target.clone(),
                expression: String::new(),
                min: result.spec.min,
                max: result.spec.max,
                unit: result.spec.unit.clone(),
                scope: crate::state::SpecPointScope::AllPoints,
            },
            stats,
            passing: result.pass_count,
            total: result.total_runs,
        }
    }

    pub(super) fn yield_percent(&self) -> Option<f64> {
        (self.total > 0).then(|| self.passing as f64 / self.total as f64 * 100.0)
    }

    /// A specification fails when any trial broke it.
    pub(super) fn verdict(&self) -> Verdict {
        if self.passing < self.total {
            Verdict::Fail
        } else {
            Verdict::Pass
        }
    }
}

/// The rows of `results`, computed once per retained generation of them.
///
/// The statistics sort every sample of every specification; asked for on every
/// frame, that is the most expensive thing on the page for a value that cannot
/// change until the run's evidence does.
fn yield_rows(ctx: &egui::Context, app: &RSpiceApp, results: &[YieldResult]) -> Vec<YieldRow> {
    let key = egui::Id::new((
        "workbench.verify.yield.rows",
        verification_run(app).map(|run| run.dataset_id.to_string()),
        app.state.simulation.view.data_version,
        results.len(),
        results
            .iter()
            .map(|result| (result.total_runs, result.pass_count, result.samples.len()))
            .collect::<Vec<_>>(),
    ));
    if let Some(rows) = ctx.data(|data| data.get_temp::<Vec<YieldRow>>(key)) {
        return rows;
    }
    let rows = results
        .iter()
        .map(YieldRow::from_result)
        .collect::<Vec<_>>();
    ctx.data_mut(|data| data.insert_temp(key, rows.clone()));
    rows
}

/// The row whose histogram is shown: the reader's pick while it names a row,
/// otherwise the row with the lowest yield.
pub(super) fn shown_yield_row(rows: &[YieldRow], picked: Option<&str>) -> Option<usize> {
    picked
        .and_then(|picked| rows.iter().position(|row| row.output == picked))
        .or_else(|| {
            rows.iter()
                .enumerate()
                .min_by(|left, right| {
                    let yield_of = |row: &YieldRow| row.yield_percent().unwrap_or(f64::INFINITY);
                    yield_of(left.1).total_cmp(&yield_of(right.1))
                })
                .map(|(index, _)| index)
        })
}

/// The toolbar's line: the joint yield over every specification.
pub(super) fn yield_summary(results: &[YieldResult]) -> Option<Summary> {
    if results.is_empty() {
        return None;
    }
    Some(match joint_sample_summary(results) {
        Some(joint) => {
            let text = format!(
                "yield {:.1} % ({} / {})",
                joint.yield_percent(),
                joint.passing,
                joint.total
            );
            if joint.passing < joint.total {
                Summary::failing(text)
            } else {
                Summary::plain(text)
            }
        }
        None => Summary::warning(
            "Joint yield not computed: the specifications were judged over different trials",
        ),
    })
}

pub(super) fn yield_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let runs = run_choice(app, |run| {
        app.state
            .simulation
            .retained
            .yield_results_for_dataset(run.dataset_id)
            .is_some_and(|results| !results.is_empty())
    });
    let run_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let results = verification_yield_results(app);
    let rows = yield_rows(ui.ctx(), app, results);
    let shown = shown_yield_row(
        &rows,
        app.state
            .workbench
            .verification
            .yield_selected_spec
            .as_deref(),
    );
    let histogram: Option<HistogramModel> = shown
        .and_then(|index| results.get(index))
        .and_then(|result| histogram_model(&result.samples, result.spec.min, result.spec.max));
    let summary = yield_summary(results).map(|summary| {
        if runs.offers_choice() {
            summary
        } else {
            append_run(summary, run_name.as_deref())
        }
    });
    let actions = [ToolbarAction::new("Edit specifications")];
    let output = PageToolbar {
        title: "Monte Carlo yield",
        runs: &runs,
        summary,
        actions: &actions,
    }
    .show(ui);
    if output.action == Some(0) {
        Command::EditSpecifications.execute(app);
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.simulation, index);
    }
    action_problem_line(ui, app);

    if rows.is_empty() {
        let text = run_name.map_or_else(
            || "No Monte Carlo results. Add a Monte Carlo analysis to the plan, then run it.".to_owned(),
            |run| {
                format!(
                    "No Monte Carlo results in {run}. Add a Monte Carlo analysis to the plan, then run it."
                )
            },
        );
        if empty_line(ui, &text, Some("Configure Monte Carlo")) {
            let result =
                open_analysis_configuration(app, crate::simulation::plan::AnalysisKind::MonteCarlo);
            record_action(&mut app.state.workbench.verification, result);
        }
        return;
    }

    let body = ui.available_height();
    let detail = (body * 0.45).clamp(DETAIL_MIN_H, DETAIL_MAX_H);
    let table_height = (body - detail - 1.0).max(TABLE_HEAD_H + TABLE_ROW_H);
    let columns = [
        Column::name("Output", 120.0),
        Column::words("Spec", 104.0),
        Column::number("Mean", 96.0),
        Column::number("Std dev", 96.0),
        Column::number("Min", 96.0),
        Column::number("Max", 96.0),
        Column::number("Cpk", 64.0),
        Column::number("Yield", 76.0),
        Column::words("Pass/fail", 70.0),
    ];
    let clicked = DataTable::new("verify.yield.specs", &columns, rows.len())
        .clickable()
        .selected(shown)
        .max_height(table_height)
        .show(ui, |index| {
            let row = &rows[index];
            let unit = row.limit.unit.as_str();
            let stat = |value: Option<f64>| {
                value.map_or_else(Cell::absent, |value| {
                    Cell::number(format_value(value, unit))
                })
            };
            vec![
                Cell::text(&row.output),
                limit_cell(&row.limit),
                stat(row.stats.map(|stats| stats.mean)),
                stat(row.stats.map(|stats| stats.std_dev)),
                stat(row.stats.map(|stats| stats.min)),
                stat(row.stats.map(|stats| stats.max)),
                row.stats
                    .and_then(|stats| stats.cpk)
                    .map_or_else(Cell::absent, |cpk| Cell::number(format!("{cpk:.2}"))),
                row.yield_percent().map_or_else(Cell::absent, |percent| {
                    Cell::number(format!("{percent:.1} %"))
                }),
                Cell::verdict(row.verdict()),
            ]
        })
        .clicked;
    if let Some(index) = clicked {
        app.state.workbench.verification.yield_selected_spec = Some(rows[index].output.clone());
    }

    let Some(index) = shown else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    let width = ui.available_width();
    ui.painter().hline(
        ui.max_rect().left()..=ui.max_rect().left() + width,
        top + 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    #[cfg(test)]
    super::raster::probe(
        ui,
        "split",
        egui::Rect::from_min_size(
            egui::pos2(ui.max_rect().left(), top),
            egui::vec2(width, 1.0),
        ),
    );
    let row = &rows[index];
    let height = ui.available_height().clamp(DETAIL_MIN_H, DETAIL_MAX_H);
    match &histogram {
        Some(model) => paint_histogram(ui, &row.output, &row.limit.unit, model, height),
        None => {
            let _ = empty_line(
                ui,
                &format!("This run kept no samples of {}.", row.output),
                None,
            );
        }
    }
}
