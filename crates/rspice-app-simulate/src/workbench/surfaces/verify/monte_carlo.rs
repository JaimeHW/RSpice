//! Monte Carlo: the yield of every specification over the run's trials.
//!
//! One row per specification the trials answered, its limit beside its
//! statistics, as the ADE Assembler yield view and the ADS yield summary put
//! them: the yield with its confidence interval, the distribution's mean,
//! sigma and extremes, its capability, and how many trials failed. Under the
//! table, the selected row's histogram beside the trials that failed it (over
//! them on a phone), one frame filling the page to the dock; a trial opens in
//! Results as the one member of its family. Nothing selected, the view
//! selects the first row whose yield does not hold, else the first.
//!
//! Every number is the verification sheet's. A trial passes or fails by the
//! sheet's word, a yield is the acceptance gate's own count, and a row's
//! yield is judged by the plan's Monte Carlo gate the way the sign-off
//! judges it: an ungated plan holds only when every trial does.
//!
//! The limit is entered in its cell. It is the plan's requirement, the one
//! every view judges against, and the run is judged against it again the
//! frame it changes, with no run.

use rspice_results::specification::{MonteCarloSpecificationGate, SpecificationComparison};
use rspice_results::verification::{JointYield, Row, Sheet, Statistics, Trial, YieldCount};
use rspice_results::yield_analysis::{DistributionStats, monte_carlo_provenance};

use super::super::super::state::YieldLimitDraft;
use super::corners::{Evidence, FactSection, limit_text, open_in_results, point_name};
use super::histogram::{Bound, Side, histogram_model, paint_histogram};
use super::table::{
    Cell, CellEditor, Column, DataTable, EditOutcome, RowKey, SortKey, TABLE_EDGE_INSET,
    TABLE_HEAD_H, TABLE_ROW_H, TableState, Verdict,
};
use super::toolbar::{PageToolbar, RunChoice, Summary, empty_line};
use super::*;
use crate::workbench::simulate::commands::SimulateCommand;

/// The share of the body the table may take before its rows scroll; the
/// detail fills everything under it.
const TABLE_SHARE: f32 = 0.4;
/// Width of the failing-trial list beside the histogram.
const TRIALS_W: f32 = 300.0;

/// The column a limit is entered in.
pub(super) const LIMIT_COLUMN: usize = 1;

/// The trials one row states: every one, or those at the point the reader
/// picked.
#[derive(Debug, Clone)]
pub(super) struct Population<'a> {
    pub(super) statistics: &'a Statistics,
    pub(super) yield_count: Option<YieldCount>,
    pub(super) distribution: &'a DistributionStats,
    /// The trials, by their place in the row's statistics.
    pub(super) trials: Vec<usize>,
}

impl Population<'_> {
    fn trial(&self, position: usize) -> &Trial {
        &self.statistics.trials[position]
    }

    /// The failing trials, by their place in the row's statistics.
    pub(super) fn failing(&self) -> Vec<usize> {
        self.trials
            .iter()
            .copied()
            .filter(|position| !self.trial(*position).passing)
            .collect()
    }

    /// Every trial's finite value and whether it passed, for the histogram.
    fn samples(&self) -> Vec<(f64, bool)> {
        self.trials
            .iter()
            .map(|position| self.trial(*position))
            .filter_map(|trial| Some((trial.value?, trial.passing)))
            .collect()
    }
}

/// The trials of `statistics` at `point`, or every trial for `None`; `None`
/// when no trial of the row ran there.
pub(super) fn population(statistics: &Statistics, point: Option<usize>) -> Option<Population<'_>> {
    let Some(point) = point else {
        return Some(Population {
            statistics,
            yield_count: statistics.yield_count,
            distribution: &statistics.distribution,
            trials: (0..statistics.trials.len()).collect(),
        });
    };
    let trials = (0..statistics.trials.len())
        .filter(|position| statistics.trials[*position].point == Some(point))
        .collect::<Vec<_>>();
    if let Some(here) = statistics
        .per_point
        .iter()
        .find(|here| here.point == Some(point))
    {
        return Some(Population {
            statistics,
            yield_count: here.yield_count,
            distribution: &here.distribution,
            trials,
        });
    }
    (statistics.per_point.is_empty()
        && trials.len() == statistics.trials.len()
        && !trials.is_empty())
    .then_some(Population {
        statistics,
        yield_count: statistics.yield_count,
        distribution: &statistics.distribution,
        trials,
    })
}

/// The points the trials ran at, when they ran at more than one.
pub(super) fn trial_points(sheet: &Sheet) -> Vec<usize> {
    let mut points = sheet
        .rows
        .iter()
        .filter_map(|row| row.statistics.as_ref())
        .flat_map(|statistics| statistics.per_point.iter().filter_map(|here| here.point))
        .collect::<Vec<_>>();
    points.sort_unstable();
    points.dedup();
    points
}

/// The point the view states, once the reader's pick is checked against the
/// run: a point the trials no longer ran at states every point.
pub(super) fn shown_point(sheet: &Sheet, session: &VerificationSessionState) -> Option<usize> {
    session
        .monte_carlo_point
        .filter(|point| trial_points(sheet).contains(point))
}

/// The specifications the table lists, by their place in the sheet.
pub(super) fn monte_carlo_rows(sheet: &Sheet, point: Option<usize>) -> Vec<usize> {
    (0..sheet.rows.len())
        .filter(|row| {
            sheet.rows[*row]
                .statistics
                .as_ref()
                .is_some_and(|statistics| population(statistics, point).is_some())
        })
        .collect()
}

/// Whether a yield holds under the plan's Monte Carlo gate: the gate's own
/// predicate when one is set, and every trial when none is.
pub(super) fn yield_holds(count: &YieldCount, gate: &MonteCarloSpecificationGate) -> bool {
    let (passing, total) = (
        u64::try_from(count.passing).unwrap_or(u64::MAX),
        u64::try_from(count.total).unwrap_or(u64::MAX),
    );
    match gate {
        MonteCarloSpecificationGate::NotGated => passing == total,
        MonteCarloSpecificationGate::YieldAtLeast { .. } => gate.clears(passing, total),
    }
}

/// The plan's Monte Carlo gate; ungated when Verify reads no plan.
pub(crate) fn monte_carlo_gate(model: &Model) -> MonteCarloSpecificationGate {
    verification_plan_id(model)
        .and_then(|plan_id| model.workspace.content.plan_data(plan_id))
        .map(|payload| payload.specification_policy.monte_carlo.clone())
        .unwrap_or_default()
}

/// The specifications whose yield does not hold under `gate`, in sheet order.
/// It is the one word on whether Monte Carlo fails: the toolbar names them,
/// and the navigator's line takes its tone from them.
pub(crate) fn yields_below_gate<'s>(
    sheet: &'s Sheet,
    gate: &MonteCarloSpecificationGate,
) -> Vec<&'s Row> {
    sheet
        .rows
        .iter()
        .filter(|row| {
            row.statistics
                .as_ref()
                .and_then(|statistics| statistics.yield_count)
                .is_some_and(|count| !yield_holds(&count, gate))
        })
        .collect()
}

/// A percentage to the places a reader needs: tenths, or whole percent
/// across a wide interval.
fn percent(value: f64, whole: bool) -> String {
    if whole {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

/// A Wilson interval as the summary and a hint state it.
pub(super) fn interval_text((low, high): (f64, f64)) -> String {
    let whole = high - low >= 5.0;
    format!(
        "95 % CI {}\u{2013}{} %",
        percent(low, whole),
        percent(high, whole)
    )
}

/// The gate as the summary names it: `yield ≥ 95 %`.
fn gate_text(gate: &MonteCarloSpecificationGate) -> Option<String> {
    match gate {
        MonteCarloSpecificationGate::NotGated => None,
        MonteCarloSpecificationGate::YieldAtLeast { percent } => {
            Some(format!("yield \u{2265} {percent} %"))
        }
    }
}

/// The toolbar's line: the joint yield over every judged specification, and
/// whether the plan's gate clears; in the failing tone when a specification's
/// yield does not hold.
///
/// The joint yield is the run's, over every trial at every point: the gate
/// signs off the whole population, whichever point the table states.
pub(super) fn monte_carlo_summary(sheet: &Sheet, gate: &MonteCarloSpecificationGate) -> Summary {
    let judged = sheet.rows.iter().any(|row| {
        row.statistics
            .as_ref()
            .is_some_and(|statistics| statistics.yield_count.is_some())
    });
    let failing = yields_below_gate(sheet, gate)
        .into_iter()
        .map(|row| row.name.as_str())
        .collect::<Vec<_>>();
    let mut text = match sheet.joint_yield {
        Some(JointYield::Counted { passing, total, .. }) if total > 0 => {
            let count = YieldCount {
                passing,
                total,
                interval: rspice_results::population::wilson_interval(passing, total),
            };
            let mut text = format!("Joint yield {:.1} % ({passing} / {total})", count.percent());
            if let Some(interval) = count.interval {
                text.push_str(&format!(" \u{00b7} {}", interval_text(interval)));
            }
            text
        }
        Some(JointYield::DifferentTrials) => {
            "Joint yield not computed: judged over different trials".to_owned()
        }
        Some(JointYield::Counted { .. }) | None => {
            let trials = sheet
                .rows
                .iter()
                .filter_map(|row| row.statistics.as_ref())
                .map(|statistics| statistics.trials.len())
                .max()
                .unwrap_or(0);
            format!("{trials} trials \u{00b7} no limits")
        }
    };
    if let Some(gate_words) = gate_text(gate).filter(|_| judged) {
        match failing.as_slice() {
            [] => text.push_str(&format!(" \u{00b7} clears {gate_words}")),
            [name] => text.push_str(&format!(" \u{00b7} {name} below {gate_words}")),
            several => text.push_str(&format!(" \u{00b7} {} below {gate_words}", several.len())),
        }
    }
    if failing.is_empty() {
        Summary::plain(text)
    } else {
        Summary::failing(text)
    }
}

/// A row's yield cell: the percentage, its interval on hover, coloured only
/// when it does not hold.
fn yield_cell(count: Option<YieldCount>, gate: &MonteCarloSpecificationGate) -> Cell {
    let Some(count) = count.filter(|count| count.total > 0) else {
        return Cell::absent();
    };
    let shown = count.percent();
    let cell = Cell::number(format!("{shown:.1} %")).sort_by(SortKey::Number(shown));
    let cell = match count.interval {
        Some(interval) => cell.hint(format!(
            "{} of {} trials \u{00b7} {} (Wilson)",
            count.passing,
            count.total,
            interval_text(interval)
        )),
        None => cell,
    };
    if yield_holds(&count, gate) {
        cell
    } else {
        cell.judged(Verdict::Fail)
    }
}

/// Apply what the reader typed into a limit cell, or say why it was not.
///
/// The text reads as every limit cell reads it, and the change is the plan's
/// through the one edit every Verify view makes: one plan change with its
/// receipt and its step of the setup's history, the run judged again at once.
fn commit_limit(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &YieldLimitDraft,
    comparison: &SpecificationComparison,
    unit: &str,
) -> Result<(), String> {
    let policy = cx.model.preferences.quantity_presentation_policy();
    let locale = cx.wb.number_locale;
    let limit = specification_edit::parse_limit(&draft.text, unit, comparison, policy, locale)?;
    let output = draft.output.clone();
    specification_edit::edit_specifications(cx, |definitions| {
        let index = specification_edit::definition_index(definitions, None, &output)
            .ok_or_else(|| format!("{output} has no specification"))?;
        Ok(specification_edit::set_limit(
            &mut definitions[index],
            limit,
        ))
    })
    .map(|_| ())
}

/// The trial the reader selected in the failing-trial list: its
/// specification's place in the sheet and its own in that row's statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedTrial {
    pub(crate) row: usize,
    pub(crate) trial: usize,
}

/// The specification row selected in the Monte Carlo table, while the table
/// lists it.
pub(super) fn selected_row(sheet: &Sheet, session: &VerificationSessionState) -> Option<usize> {
    let Some(RowKey::Number(key)) = session.monte_carlo_table.selected_key() else {
        return None;
    };
    let row = usize::try_from(*key).ok()?;
    monte_carlo_rows(sheet, shown_point(sheet, session))
        .contains(&row)
        .then_some(row)
}

/// The failing trial selected under the selected row, while the list holds
/// it.
pub(crate) fn selected_trial(
    sheet: &Sheet,
    session: &VerificationSessionState,
) -> Option<SelectedTrial> {
    let row = selected_row(sheet, session)?;
    let Some(RowKey::Number(key)) = session.monte_carlo_trials_table.selected_key() else {
        return None;
    };
    let trial = usize::try_from(*key).ok()?;
    let statistics = sheet.rows[row].statistics.as_ref()?;
    population(statistics, shown_point(sheet, session))?
        .failing()
        .contains(&trial)
        .then_some(SelectedTrial { row, trial })
}

/// Where a trial sits in its analysis's family, for the family view's filter.
fn trial_evidence(run: &crate::state::SimulationRun, trial: &Trial) -> Option<Evidence> {
    let analysis = run.analyses.get(trial.analysis)?;
    let sample = match &trial.member {
        Some(member) => Some(
            analysis
                .family_metadata
                .as_ref()?
                .member_measurements()
                .iter()
                .position(|measured| measured.member == *member)?,
        ),
        None => None,
    };
    Some(Evidence {
        analysis: trial.analysis,
        sample,
    })
}

/// A trial's number as the analysis requested it.
fn trial_number(trial: &Trial) -> String {
    trial
        .index()
        .map_or_else(|| "\u{2014}".to_owned(), |index| format!("{}", index + 1))
}

/// A seed as the run records it.
fn seed_text(seed: u64) -> String {
    format!("0x{seed:X}")
}

/// What the inspector states of the selected failing trial.
pub(crate) fn trial_facts(model: &Model, session: &VerificationSessionState) -> Vec<FactSection> {
    let Some(sheet) = model.simulation.verification_sheet() else {
        return Vec::new();
    };
    let Some(selected) = selected_trial(sheet, session) else {
        return Vec::new();
    };
    let row = &sheet.rows[selected.row];
    let Some(trial) = row
        .statistics
        .as_ref()
        .and_then(|statistics| statistics.trials.get(selected.trial))
    else {
        return Vec::new();
    };
    let mut rows = vec![(
        "Value".to_owned(),
        trial.value.map_or_else(
            || "No value".to_owned(),
            |value| format_value(value, &row.unit),
        ),
    )];
    let limit = limit_text(row);
    if !limit.is_empty() {
        rows.push(("Limit".to_owned(), limit));
    }
    if let Some(seed) = trial.seed() {
        rows.push(("Seed".to_owned(), seed_text(seed)));
    }
    if let Some(point) = trial.point.and_then(|point| sheet.points.get(point))
        && sheet.points.len() > 1
    {
        rows.push(("Point".to_owned(), point_name(point)));
    }
    if let Some(analysis) = model
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(trial.analysis))
    {
        rows.push(("Analysis".to_owned(), analysis.label.clone()));
    }
    vec![FactSection {
        title: format!("Trial {} \u{00b7} {}", trial_number(trial), row.name),
        rows,
    }]
}

/// The Monte Carlo campaigns the view's statistics came from, as the
/// inspector states them when no trial is selected: samples completed of
/// those requested, seed, sampling and `duration`. One section per analysis
/// whose trials a specification counts, in the order they first appear, or
/// every Monte Carlo analysis of the run while none is judged. Several are
/// told apart by label, and by the point each ran at where labels repeat.
pub(crate) fn campaign_facts(model: &Model, duration: Option<&str>) -> Vec<FactSection> {
    let Some(run) = model.simulation.active_run() else {
        return Vec::new();
    };
    let sheet = model.simulation.verification_sheet();
    // Each judged analysis with the sheet point its trials ran at.
    let mut judged: Vec<(usize, Option<usize>)> = Vec::new();
    let trials = sheet
        .into_iter()
        .flat_map(|sheet| &sheet.rows)
        .filter_map(|row| row.statistics.as_ref())
        .flat_map(|statistics| &statistics.trials);
    for trial in trials {
        if !judged
            .iter()
            .any(|(analysis, _)| *analysis == trial.analysis)
        {
            judged.push((trial.analysis, trial.point));
        }
    }
    let analyses = if judged.is_empty() {
        (0..run.analyses.len()).map(|index| (index, None)).collect()
    } else {
        judged
    };
    let campaigns = analyses
        .into_iter()
        .filter_map(|(index, point)| {
            let analysis = run.analyses.get(index)?;
            let provenance = monte_carlo_provenance(run.run_id, run.dataset_id, analysis)?;
            Some((analysis.label.as_str(), point, provenance))
        })
        .collect::<Vec<_>>();
    let several = campaigns.len() > 1;
    campaigns
        .iter()
        .map(|(label, point, provenance)| {
            let repeated = campaigns
                .iter()
                .filter(|(other, _, _)| other == label)
                .count()
                > 1;
            let at = point
                .and_then(|point| sheet?.points.get(point))
                .map(point_name)
                .filter(|_| repeated);
            let title = match (several, at) {
                (false, _) => "Monte Carlo".to_owned(),
                (true, Some(at)) => format!("{label} \u{00b7} {at}"),
                (true, None) => (*label).to_owned(),
            };
            let mut rows = vec![
                (
                    "Samples".to_owned(),
                    format!(
                        "{} / {}",
                        provenance.runs_completed, provenance.runs_requested
                    ),
                ),
                ("Seed".to_owned(), format!("0x{:X}", provenance.seed)),
                (
                    "Sampling".to_owned(),
                    provenance.sampling_mode.display_name().to_owned(),
                ),
            ];
            if let Some(duration) = duration {
                rows.push(("Duration".to_owned(), duration.to_owned()));
            }
            FactSection { title, rows }
        })
        .collect()
}

/// Whether the run Verify reads holds a Monte Carlo analysis, and whether its
/// trials kept their own records.
fn monte_carlo_in_run(model: &Model) -> Option<bool> {
    model.simulation.active_run().and_then(|run| {
        run.analyses
            .iter()
            .find_map(|analysis| match &analysis.family_metadata {
                Some(
                    rspice_results::family_metadata::AnalysisResultFamilyMetadata::MonteCarlo {
                        member_measurements,
                        ..
                    },
                ) => Some(!member_measurements.is_empty()),
                _ => None,
            })
    })
}

pub(super) fn monte_carlo(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let recorded = monte_carlo_in_run(cx.model);
    let gate = monte_carlo_gate(cx.model);
    let sheet = cx.model.simulation.verification_sheet();
    let point = sheet.and_then(|sheet| shown_point(sheet, &cx.state.verification));
    let rows = sheet.map_or_else(Vec::new, |sheet| monte_carlo_rows(sheet, point));
    let points = sheet.map_or_else(Vec::new, trial_points);
    let choice = RunChoice {
        role: super::toolbar::RunRole::Point,
        candidates: if points.is_empty() {
            Vec::new()
        } else {
            std::iter::once((0, "All points".to_owned()))
                .chain(sheet.into_iter().flat_map(|sheet| {
                    points
                        .iter()
                        .map(|point| (point + 1, point_name(&sheet.points[*point])))
                }))
                .collect()
        },
        selected: Some(point.map_or(0, |point| point + 1)),
    };
    let summary = sheet
        .filter(|_| !rows.is_empty())
        .map(|sheet| monte_carlo_summary(sheet, &gate));
    let output = PageToolbar {
        title: VerificationPage::MonteCarlo.label(),
        runs: &choice,
        summary,
        actions: &[],
    }
    .show(ui);
    if let Some(picked) = output.run {
        let session = &mut cx.state.verification;
        session.monte_carlo_point = picked.checked_sub(1);
        session.monte_carlo_trials_table.selection = None;
    }
    action_problem_line(ui, cx.model, cx.state);

    let Some(recorded) = recorded else {
        cx.state.verification.yield_limit_draft = None;
        if empty_line(
            ui,
            "This run has no Monte Carlo analysis.",
            Some("Add a Monte Carlo analysis"),
        ) {
            let result =
                open_analysis_configuration(cx, crate::simulation::plan::AnalysisKind::MonteCarlo);
            record_action(&mut cx.state.verification, result);
        }
        return;
    };
    if !recorded {
        let _ = crate::ui::widgets::problem_line(
            ui,
            Problem::Warning,
            "This run kept no per-trial records; run it again to judge its yield.",
        );
    }
    let Some(sheet) = cx
        .model
        .simulation
        .verification_sheet()
        .filter(|_| !rows.is_empty())
    else {
        cx.state.verification.yield_limit_draft = None;
        if recorded
            && empty_line(
                ui,
                "No specification measures this run's Monte Carlo trials.",
                Some("Open specifications"),
            )
        {
            cx.execute(SimulateCommand::Verify(VerificationPage::Specifications).key());
        }
        return;
    };

    let entry_allowed = verification_plan_id(cx.model).is_some()
        && verification_evidence_domain(cx.model).answers_a_plan_limit();
    let session = &mut cx.state.verification;
    if selected_row(sheet, session).is_none() {
        let first = first_shown(sheet, &rows, point, &gate);
        session
            .monte_carlo_table
            .select(first, Some(RowKey::Number(rows[first] as u64)));
    }
    // The table takes the height of its rows, scrolling past its share of
    // the body; the detail fills everything under it.
    let table_h = (ui.available_height() * TABLE_SHARE).max(TABLE_HEAD_H + TABLE_ROW_H);
    // At their least the columns fit the page beside both docks at 1440
    // points; the name column takes whatever is left.
    let columns = [
        Column::name("Specification", 120.0),
        Column::number("Limit", 112.0)
            .tooltip("The plan's limit on this measurement. Click to enter one."),
        Column::number("Yield", 76.0),
        Column::number("Mean", 88.0),
        Column::number("Std dev", 88.0),
        Column::number("Min", 88.0),
        Column::number("Max", 88.0),
        Column::number("Cpk", 56.0),
        Column::number("Fails", 56.0),
    ];
    let keys = rows
        .iter()
        .map(|row| RowKey::Number(*row as u64))
        .collect::<Vec<_>>();
    let nameable = |row: &Row| {
        rspice_vocabulary::text_validation::validate_parameter_name(&row.measurement).is_ok()
    };
    let mut draft = session.yield_limit_draft.take().filter(|draft| {
        entry_allowed
            && rows
                .iter()
                .any(|row| sheet.rows[*row].measurement == draft.output)
    });
    let editor = draft.as_mut().and_then(|draft| {
        Some(CellEditor {
            row: rows
                .iter()
                .position(|row| sheet.rows[*row].measurement == draft.output)?,
            column: LIMIT_COLUMN,
            text: &mut draft.text,
            focus: std::mem::take(&mut draft.focus),
        })
    });
    let response = DataTable::new("verify.monte_carlo", &columns, rows.len())
        .state(&mut session.monte_carlo_table)
        .keys(&keys)
        .freeze(1)
        .max_height(table_h)
        .editing(editor)
        .show(ui, |index| {
            let row = &sheet.rows[rows[index]];
            let Some(shown) = row
                .statistics
                .as_ref()
                .and_then(|statistics| population(statistics, point))
            else {
                return Vec::new();
            };
            let unit = row.unit.as_str();
            let measured = shown.distribution.count > 0;
            let stat = |value: f64| {
                if measured && value.is_finite() {
                    Cell::number(format_value(value, unit)).sort_by(SortKey::Number(value))
                } else {
                    Cell::absent()
                }
            };
            let limit = {
                let text = limit_text(row);
                let cell = if text.is_empty() {
                    Cell::absent()
                } else {
                    Cell::number(text)
                };
                if !entry_allowed {
                    cell
                } else if nameable(row) {
                    cell.editable()
                } else {
                    cell.hint("A limit bounds a measurement, and this one's name is not one.")
                }
            };
            let fails = shown.failing().len();
            vec![
                Cell::text(&row.name),
                limit,
                yield_cell(shown.yield_count, &gate),
                stat(shown.distribution.mean),
                stat(shown.distribution.std_dev),
                stat(shown.distribution.min),
                stat(shown.distribution.max),
                shown
                    .distribution
                    .cpk
                    .filter(|cpk| measured && cpk.is_finite())
                    .map_or_else(Cell::absent, |cpk| {
                        Cell::number(format!("{cpk:.2}")).sort_by(SortKey::Number(cpk))
                    }),
                Cell::number(fails.to_string()).sort_by(SortKey::Number(fails as f64)),
            ]
        });
    if response.selection_changed {
        session.monte_carlo_trials_table.selection = None;
    }
    if let Some((index, _)) = response.edit {
        let seed = limit_text(&sheet.rows[rows[index]]);
        draft = Some(YieldLimitDraft {
            output: sheet.rows[rows[index]].measurement.clone(),
            text: seed.clone(),
            seed,
            focus: true,
        });
    }
    let committed = match response.edited {
        Some(EditOutcome::Cancel) => {
            draft = None;
            None
        }
        Some(EditOutcome::Commit) => draft
            .take()
            .filter(|open| open.text.trim() != open.seed.trim())
            .and_then(|open| {
                let row = sheet
                    .rows
                    .iter()
                    .find(|row| row.measurement == open.output)?;
                Some((open, row.comparison.clone(), row.unit.clone()))
            }),
        None => None,
    };
    let selected = selected_row(sheet, &cx.state.verification);
    let detail = selected.and_then(|row| {
        let statistics = sheet.rows[row].statistics.as_ref()?;
        let shown = population(statistics, point)?;
        let limits = bounds_of(&sheet.rows[row].comparison);
        Some((
            row,
            histogram_model(&shown.samples(), &limits),
            shown.failing(),
            shown.trials.len(),
        ))
    });
    cx.state.verification.yield_limit_draft = draft;
    let opened = detail.and_then(|(row, histogram, failing, trials)| {
        let sheet = cx.model.simulation.verification_sheet()?;
        let row = &sheet.rows[row];
        let statistics = row.statistics.as_ref()?;
        let open = detail_pane(
            ui,
            Detail {
                row,
                statistics,
                histogram: histogram.as_ref(),
                failing: &failing,
                trials,
            },
            &mut cx.state.verification.monte_carlo_trials_table,
        )?;
        let run = cx.model.simulation.active_run()?;
        trial_evidence(run, &statistics.trials[open])
    });
    if let Some(evidence) = opened {
        open_in_results(cx, evidence);
    }
    if let Some((open, comparison, unit)) = committed {
        match commit_limit(cx, &open, &comparison, &unit) {
            Ok(()) => {
                record_action(&mut cx.state.verification, Ok(()));
                ui.ctx().request_repaint();
            }
            Err(problem) => {
                record_action(
                    &mut cx.state.verification,
                    Err(format!(
                        "The limit of {} was not set: {problem}.",
                        open.output
                    )),
                );
                cx.state.verification.yield_limit_draft = Some(YieldLimitDraft {
                    focus: true,
                    ..open
                });
            }
        }
    }
}

/// The row the view shows when the reader has selected none: the first whose
/// yield at the shown point does not hold, else the first. By its place in
/// `rows`, which is never empty here.
fn first_shown(
    sheet: &Sheet,
    rows: &[usize],
    point: Option<usize>,
    gate: &MonteCarloSpecificationGate,
) -> usize {
    rows.iter()
        .position(|row| {
            sheet.rows[*row]
                .statistics
                .as_ref()
                .and_then(|statistics| population(statistics, point)?.yield_count)
                .is_some_and(|count| !yield_holds(&count, gate))
        })
        .unwrap_or(0)
}

/// The bounds a limit draws on the histogram, each with the side that fails.
fn bounds_of(comparison: &SpecificationComparison) -> Vec<Bound> {
    let floor = |value| Bound {
        value,
        side: Side::Floor,
    };
    let ceiling = |value| Bound {
        value,
        side: Side::Ceiling,
    };
    match *comparison {
        SpecificationComparison::Tracked => Vec::new(),
        SpecificationComparison::Minimum { limit } => vec![floor(limit)],
        SpecificationComparison::Maximum { limit } => vec![ceiling(limit)],
        SpecificationComparison::Range { minimum, maximum } => {
            vec![floor(minimum), ceiling(maximum)]
        }
        SpecificationComparison::EqualWithin { target, tolerance } => {
            vec![floor(target - tolerance), ceiling(target + tolerance)]
        }
    }
}

/// What the detail frame states of the selected row.
struct Detail<'a> {
    row: &'a Row,
    statistics: &'a Statistics,
    histogram: Option<&'a super::histogram::HistogramModel>,
    /// The failing trials, by their place in the row's statistics.
    failing: &'a [usize],
    /// How many trials the row states at the shown point.
    trials: usize,
}

/// Where the detail's parts sit in the frame under the table.
struct DetailLayout {
    /// The plot's head band, its title in it.
    band: egui::Rect,
    plot: egui::Rect,
    /// The failing-trial list, its column heads at its top.
    list: egui::Rect,
}

impl DetailLayout {
    /// Two halves side by side: one head band across both, the plot's title
    /// on the left and the list's column heads on the right, both halves
    /// running to the frame's foot.
    fn beside(frame: egui::Rect) -> Self {
        let split = frame.right() - TRIALS_W.min(frame.width() * 0.5);
        let band = egui::Rect::from_min_size(frame.min, egui::vec2(frame.width(), TABLE_HEAD_H));
        Self {
            band,
            plot: egui::Rect::from_min_max(
                egui::pos2(frame.left(), band.bottom()),
                egui::pos2(split - 1.0, frame.bottom()),
            ),
            list: egui::Rect::from_min_max(egui::pos2(split, frame.top()), frame.max),
        }
    }

    /// The histogram over the list, each the frame's full width under its own
    /// head band: a phone has no width for two halves. The list takes the
    /// height of its rows up to half the frame; the plot takes the rest.
    fn stacked(frame: egui::Rect, failing: usize) -> Self {
        let rows = TABLE_HEAD_H + TABLE_ROW_H * failing.max(1) as f32;
        let list_h = rows.min((frame.height() * 0.5).floor());
        let band = egui::Rect::from_min_size(frame.min, egui::vec2(frame.width(), TABLE_HEAD_H));
        let list =
            egui::Rect::from_min_max(egui::pos2(frame.left(), frame.bottom() - list_h), frame.max);
        Self {
            band,
            plot: egui::Rect::from_min_max(
                egui::pos2(frame.left(), band.bottom()),
                egui::pos2(frame.right(), list.top() - 1.0),
            ),
            list,
        }
    }
}

/// The selected row's detail, filling the page under the table: the
/// histogram and the trials that failed it, beside each other or, on a
/// phone, one over the other. Each states its head in a band at the table
/// head's height, fill and type, and the detail runs to the frame's foot on
/// the page's ground. Returns the failing trial the reader double-clicked,
/// by its place in the row's statistics.
fn detail_pane(ui: &mut Ui, detail: Detail<'_>, state: &mut TableState) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let Detail {
        row,
        statistics,
        histogram,
        failing,
        trials,
    } = detail;
    let stacked = phone_width(ui.ctx());
    let width = ui.available_width();
    let least = if stacked {
        2.0 * (TABLE_HEAD_H + TABLE_ROW_H) + 2.0
    } else {
        TABLE_HEAD_H + TABLE_ROW_H + 1.0
    };
    let height = ui.available_height().max(least);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let frame = egui::Rect::from_min_max(egui::pos2(rect.left(), rect.top() + 1.0), rect.max);
    let DetailLayout { band, plot, list } = if stacked {
        DetailLayout::stacked(frame, failing.len())
    } else {
        DetailLayout::beside(frame)
    };
    #[cfg(any(test, feature = "test-support"))]
    {
        super::raster::probe(
            ui,
            "split",
            egui::Rect::from_min_size(rect.min, egui::vec2(width, 1.0)),
        );
        super::raster::probe(ui, "detail.band", band);
        super::raster::probe(ui, "detail.plot", plot);
        super::raster::probe(ui, "detail.list", list);
    }

    let painter = ui.painter();
    let rule = egui::Stroke::new(1.0, t.color.border);
    painter.hline(rect.x_range(), rect.top() + 0.5, rule);
    painter.rect_filled(band, 0.0, t.color.bg_panel_2);
    painter.hline(band.x_range(), band.bottom() - 0.5, rule);
    if stacked {
        painter.hline(list.x_range(), list.top() - 0.5, rule);
    } else {
        painter.vline(list.left() - 0.5, frame.y_range(), rule);
    }
    let title = plot_title(ui, row, trials, failing.len());
    ui.painter()
        .with_clip_rect(band.with_max_x(plot.right()))
        .galley(
            egui::pos2(
                band.left() + TABLE_EDGE_INSET,
                band.center().y - title.size().y * 0.5,
            ),
            title,
            t.color.text_dim,
        );

    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(plot)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    plot_ui.set_clip_rect(plot);
    match histogram {
        Some(model) => paint_histogram(&mut plot_ui, &row.name, &row.unit, model),
        None => quiet_line(
            &mut plot_ui,
            &format!("This run kept no values of {}.", row.name),
        ),
    }

    let mut list_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(list)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    list_ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
    // The value the trial failed with stands beside its number, so a narrow
    // page that scrolls the list still shows it; the seed, kept to repeat
    // the trial, comes last.
    let columns = [
        Column::number("Trial", 72.0),
        Column::number("Value", 96.0),
        Column::number("Seed", 112.0),
    ];
    let keys = failing
        .iter()
        .map(|position| RowKey::Number(*position as u64))
        .collect::<Vec<_>>();
    let response = DataTable::new("verify.monte_carlo.trials", &columns, failing.len())
        .state(state)
        .keys(&keys)
        .max_height(list.height())
        .show(&mut list_ui, |index| {
            let trial = &statistics.trials[failing[index]];
            vec![
                Cell::number(trial_number(trial)).sort_by(
                    trial
                        .index()
                        .map_or(SortKey::Missing, |index| SortKey::Number(index as f64)),
                ),
                trial.value.map_or_else(Cell::absent, |value| {
                    Cell::number(format_value(value, &row.unit))
                        .judged(Verdict::Fail)
                        .sort_by(SortKey::Number(value))
                }),
                trial
                    .seed()
                    .map_or_else(Cell::absent, |seed| Cell::number(seed_text(seed))),
            ]
        });
    if failing.is_empty() {
        quiet_line(&mut list_ui, &format!("No trial fails {}.", row.name));
    }
    response
        .double_clicked
        .and_then(|cell| failing.get(cell.row).copied())
}

/// The plot's title in the head band: the output, how many trials it states
/// and how many of them fail, that count in the failing ink when any do.
fn plot_title(ui: &Ui, row: &Row, trials: usize, failing: usize) -> std::sync::Arc<egui::Galley> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let mut job = egui::text::LayoutJob::default();
    let mut add = |text: &str, color: egui::Color32| {
        job.append(text, 0.0, egui::TextFormat::simple(font.clone(), color));
    };
    add(&row.name, t.color.text);
    add(
        &format!(
            " \u{00b7} {trials} {} \u{00b7} ",
            if trials == 1 { "trial" } else { "trials" }
        ),
        t.color.text_dim,
    );
    add(
        &format!("{failing} fail"),
        if failing > 0 {
            t.color.err
        } else {
            t.color.text_dim
        },
    );
    ui.painter().layout_job(job)
}

/// One line of faint words at the top of a detail region, read out as a
/// label.
fn quiet_line(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, line) = ui.allocate_exact_size(egui::vec2(width, TABLE_ROW_H), egui::Sense::hover());
    line.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().text(
        egui::pos2(rect.left() + TABLE_EDGE_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

#[cfg(test)]
mod tests;
