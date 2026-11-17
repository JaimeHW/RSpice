//! Reference: the run Verify reads, against a reference run of the same plan.
//!
//! One table of every check the two runs share — each measurement and, when
//! the plan compares them, each waveform — stating the reference value, the
//! current one, the change, the tolerance and whether the change is inside
//! it. A tolerance is edited in its cell and is the plan's as soon as it is
//! entered; a rule whose target neither run holds is a row with a problem
//! mark, deleted from its menu or with Delete. A selected waveform row opens
//! its overlay under the table.
//!
//! The comparison is built again only when the run read, the reference or the
//! plan's tolerances change, never on a frame that changed none of them. It
//! renders what [`super::regression_contract`] decides and never re-derives a
//! verdict.

use crate::product::{DatasetId, ObjectRevision, RunId, SimulationPlanId};
use crate::state::{
    RegressionSpecificationPolicy, RegressionTargetKind, RegressionTargetSelector,
    RegressionToleranceRule,
};
use crate::workbench::state::{ReferenceToleranceDraft, RegressionComparisonReceipt};

use super::checks::row_menu;
use super::table::{
    Cell, CellEditor, Column, DataTable, EditOutcome, RowKey, SortKey, TABLE_EDGE_INSET,
    TABLE_HEAD_H, TABLE_ROW_H, Verdict,
};
use super::toolbar::{PageToolbar, RunChoice, RunRole, Summary, ToolbarAction, empty_line};
use super::*;

/// Least and greatest height of the overlay under the table.
const DETAIL_MIN_H: f32 = 220.0;
const DETAIL_MAX_H: f32 = 320.0;
/// Height of the overlay's header row.
const OVERLAY_HEADER_H: f32 = 24.0;
/// The column a reader edits.
const TOLERANCE: usize = 5;
/// The toolbar's one action.
const EXPORT: &str = "Export JUnit / TAP";

/// The Reference view's comparison, kept between frames and built again only
/// when what it compares changes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReferenceComparison {
    /// The runs the reference may be, and what they were found for.
    references: Option<References>,
    /// What the comparison was built for.
    basis: Option<Basis>,
    /// The comparison, when the run read and its reference make a pair.
    compared: Option<Compared>,
    rows: Vec<ReferenceRow>,
    keys: Vec<RowKey>,
    /// How many times the comparison has been built.
    builds: u64,
}

/// The runs a reference commit accepts against the run read now.
#[derive(Debug, Clone, PartialEq)]
struct References {
    plan: Option<SimulationPlanId>,
    current: Option<(RunId, DatasetId)>,
    retained: Vec<RunId>,
    /// Each run by its index in the retained runs, and its label; newest
    /// first.
    runs: Vec<(usize, String)>,
}

/// What a comparison compares: when none of it changed, neither did the
/// comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Pairing {
    plan: Option<(SimulationPlanId, ObjectRevision)>,
    policy: RegressionSpecificationPolicy,
    reference: Option<(RunId, Option<DatasetId>)>,
    current: Option<(RunId, DatasetId)>,
}

#[derive(Debug, Clone, PartialEq)]
struct Basis {
    pairing: Pairing,
    rules: Vec<RegressionToleranceRule>,
}

/// The comparison of a pair, as the toolbar and the export state it.
#[derive(Debug, Clone, PartialEq)]
struct Compared {
    /// The reference's and the current run's numbers.
    reference_run: u64,
    current_run: u64,
    changed: usize,
    outside: usize,
    receipt: RegressionComparisonReceipt,
    cases: Vec<RegressionExportCase>,
}

/// What a row compares.
#[derive(Debug, Clone, PartialEq)]
enum Values {
    Measurement {
        reference: f64,
        current: f64,
    },
    /// A waveform, by the number of points in each run.
    Waveform {
        reference: usize,
        current: usize,
    },
    /// A target the two runs could not be compared on, or a rule whose target
    /// neither run holds.
    Gap {
        detail: String,
        orphan: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct ReferenceRow {
    target: Option<RegressionTargetSelector>,
    name: String,
    values: Values,
    /// The unit the compared values are in, as their producer states it;
    /// empty when it states none.
    unit: String,
    rule: Option<RegressionToleranceRule>,
    verdict: Option<RegressionVerdict>,
}

impl ReferenceRow {
    fn orphan(&self) -> bool {
        matches!(self.values, Values::Gap { orphan: true, .. })
    }

    /// Whether the row's tolerance can be entered: it compares a target.
    fn editable(&self) -> bool {
        self.target.is_some() && !matches!(self.values, Values::Gap { .. })
    }
}

/// The tolerance rules the active plan holds.
fn plan_rules(model: &Model) -> &[RegressionToleranceRule] {
    model
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| model.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.regression_tolerances.as_slice())
        .unwrap_or_default()
}

/// The runs a reference commit accepts against the run read now, by their
/// index in the retained runs, newest first.
pub(super) fn reference_runs(model: &Model) -> Vec<usize> {
    let runs = &model.simulation.retained.runs;
    let mut eligible = (0..runs.len())
        .filter(|index| regression_baseline_eligibility(model, runs[*index].run_id).is_ok())
        .collect::<Vec<_>>();
    eligible.sort_by(|left, right| runs[*right].id.cmp(&runs[*left].id));
    eligible
}

/// Build the comparison again if what it compares changed since it was built.
pub(crate) fn refresh(cx: &mut Cx<'_, SimulateFeature>) {
    let mut comparison = std::mem::take(&mut cx.state.verification.reference);
    comparison.refresh(cx.model, cx.state);
    cx.state.verification.reference = comparison;
}

impl ReferenceComparison {
    /// How many checks changed against the reference run, how many of them
    /// broke their tolerance, and the reference run's number; `None` while
    /// the run read and its reference make no pair.
    pub(crate) fn tally(&self) -> Option<(usize, usize, u64)> {
        self.compared
            .as_ref()
            .map(|compared| (compared.changed, compared.outside, compared.reference_run))
    }

    fn refresh(&mut self, model: &Model, simulate: &SimulateState) {
        self.find_references(model);
        let current = model
            .simulation
            .active_run()
            .map(|run| (run.run_id, run.dataset_id));
        let reference = active_regression_baseline(model, simulate).map(|id| {
            let run = model
                .simulation
                .retained
                .runs
                .iter()
                .find(|run| run.run_id == id);
            (id, run.map(|run| run.dataset_id))
        });
        let pairing = Pairing {
            plan: model
                .sim_setup
                .analysis_plan
                .as_ref()
                .map(|plan| (plan.id(), plan.revision())),
            policy: active_regression_specification_policy(model),
            reference,
            current,
        };
        let rules = plan_rules(model);
        if self
            .basis
            .as_ref()
            .is_some_and(|basis| basis.pairing == pairing && basis.rules == rules)
        {
            return;
        }
        self.basis = Some(Basis {
            pairing,
            rules: rules.to_vec(),
        });
        self.builds += 1;
        self.build(model, simulate, rules, pairing.policy);
    }

    fn find_references(&mut self, model: &Model) {
        let plan = model
            .sim_setup
            .analysis_plan
            .as_ref()
            .map(crate::simulation::plan::SimulationPlan::id);
        let current = model
            .simulation
            .active_run()
            .map(|run| (run.run_id, run.dataset_id));
        let runs = &model.simulation.retained.runs;
        if self.references.as_ref().is_some_and(|found| {
            found.plan == plan
                && found.current == current
                && found.retained.iter().eq(runs.iter().map(|run| &run.run_id))
        }) {
            return;
        }
        self.references = Some(References {
            plan,
            current,
            retained: runs.iter().map(|run| run.run_id).collect(),
            runs: reference_runs(model)
                .into_iter()
                .map(|index| (index, runs[index].label.clone()))
                .collect(),
        });
    }

    fn build(
        &mut self,
        model: &Model,
        simulate: &SimulateState,
        rules: &[RegressionToleranceRule],
        policy: RegressionSpecificationPolicy,
    ) {
        self.rows.clear();
        self.keys.clear();
        self.compared = None;
        let (Some((reference, current)), Some(plan)) = (
            regression_run_pair(model, simulate),
            model.sim_setup.analysis_plan.as_ref(),
        ) else {
            return;
        };
        let comparison = compare_regression_runs(reference, current, rules, policy);
        let Ok(receipt) = comparison.receipt(plan.id(), plan.revision(), rules, reference, current)
        else {
            return;
        };
        let cases = comparison.export_cases(rules);
        let orphans = orphaned_regression_targets(&comparison.coverage_issues);
        let rule_of = |target: &RegressionTargetSelector| regression_rule(rules, target).cloned();
        let mut changed = receipt.changed_checks;
        for (check, verdict) in comparison.checks.iter().zip(&comparison.check_verdicts) {
            self.rows.push(ReferenceRow {
                target: Some(check.target.clone()),
                name: check.name.clone(),
                values: Values::Measurement {
                    reference: check.baseline,
                    current: check.current,
                },
                unit: check.unit.clone(),
                rule: rule_of(&check.target),
                verdict: Some(verdict.clone()),
            });
        }
        for (pair, verdict) in comparison
            .waveforms
            .iter()
            .zip(&comparison.waveform_verdicts)
        {
            if worst_delta(verdict).is_some_and(|delta| delta > 0.0) {
                changed += 1;
            }
            self.rows.push(ReferenceRow {
                target: Some(pair.target.clone()),
                name: pair.current.name.clone(),
                values: Values::Waveform {
                    reference: pair.baseline.y.len(),
                    current: pair.current.y.len(),
                },
                unit: pair
                    .current
                    .unit
                    .clone()
                    .or_else(|| pair.baseline.unit.clone())
                    .unwrap_or_default(),
                rule: rule_of(&pair.target),
                verdict: Some(verdict.clone()),
            });
        }
        for gap in &comparison.coverage_issues {
            self.rows.push(ReferenceRow {
                target: gap.target.clone(),
                name: gap.label.clone(),
                values: Values::Gap {
                    detail: gap.detail.clone(),
                    orphan: gap
                        .target
                        .as_ref()
                        .is_some_and(|target| orphans.contains(target)),
                },
                unit: String::new(),
                rule: gap.target.as_ref().and_then(rule_of),
                verdict: None,
            });
        }
        let mut seen = std::collections::BTreeMap::<String, usize>::new();
        self.keys = self
            .rows
            .iter()
            .map(|row| {
                let kind = match row.values {
                    Values::Measurement { .. } => "measurement",
                    Values::Waveform { .. } => "waveform",
                    Values::Gap { .. } => "gap",
                };
                let identity = row.target.as_ref().map_or_else(
                    || format!("{kind} {}", row.name),
                    |target| {
                        format!(
                            "{kind} {:?} {} {} {}",
                            target.source_domain,
                            target.source_instance_id,
                            target.name,
                            target.occurrence
                        )
                    },
                );
                let count = seen.entry(identity.clone()).or_default();
                *count += 1;
                RowKey::Text(if *count == 1 {
                    identity
                } else {
                    format!("{identity} #{count}")
                })
            })
            .collect();
        self.compared = Some(Compared {
            reference_run: reference.id,
            current_run: current.id,
            changed,
            outside: receipt.failed_checks + receipt.failed_waveforms,
            receipt,
            cases,
        });
    }
}

/// The largest change a verdict found, when it was judged.
fn worst_delta(verdict: &RegressionVerdict) -> Option<f64> {
    match verdict {
        RegressionVerdict::Pass { worst_delta, .. }
        | RegressionVerdict::Fail { worst_delta, .. } => Some(*worst_delta),
        RegressionVerdict::NotConfigured | RegressionVerdict::NotEvaluated(_) => None,
    }
}

pub(super) fn reference_view(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    refresh(cx);
    let comparison = &cx.state.verification.reference;
    let reference = active_regression_baseline(cx.model, cx.state);
    let candidates = comparison
        .references
        .as_ref()
        .map(|found| found.runs.clone())
        .unwrap_or_default();
    let runs = RunChoice {
        selected: reference
            .and_then(|id| {
                cx.model
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .position(|run| run.run_id == id)
            })
            .filter(|index| candidates.iter().any(|(candidate, _)| candidate == index)),
        candidates,
        role: RunRole::Reference,
    };
    let summary = comparison.compared.as_ref().map(|compared| {
        let text = format!(
            "{} changed \u{00b7} against Run {}",
            compared.changed, compared.reference_run
        );
        if compared.outside > 0 {
            Summary::failing(format!(
                "{} outside tolerance \u{00b7} {text}",
                compared.outside
            ))
        } else {
            Summary::plain(text)
        }
    });
    let actions = [ToolbarAction::new(EXPORT)
        .enabled(comparison.compared.is_some())
        .disabled_reason("Choose a reference run first.")];
    let output = PageToolbar {
        title: VerificationPage::Reference.label(),
        runs: &runs,
        summary,
        actions: &actions,
    }
    .show(ui);
    if let Some(index) = output.run
        && Some(index) != runs.selected
    {
        let result = choose_reference(cx, index);
        record_action(&mut cx.state.verification, result);
    }
    if output.action == Some(0)
        && let Err(problem) = export(cx)
    {
        cx.state.verification.action_problem = Some(problem);
    }
    action_problem_line(ui, cx.model, cx.state);

    let session = &mut cx.state.verification;
    let comparison = &session.reference;
    if comparison.compared.is_none() {
        let _ = empty_line(ui, empty_text(cx.model, comparison), None);
        return;
    }
    let rows = &comparison.rows;
    if rows.is_empty() {
        let _ = empty_line(
            ui,
            "The two runs share no measurement or waveform to compare.",
            None,
        );
        return;
    }

    let mut draft = session.reference_edit.take().filter(|draft| {
        rows.iter()
            .any(|row| row.editable() && row.target.as_ref() == Some(&draft.target))
    });
    let editor = draft.as_mut().and_then(|draft| {
        Some(CellEditor {
            row: rows
                .iter()
                .position(|row| row.target.as_ref() == Some(&draft.target))?,
            column: TOLERANCE,
            text: &mut draft.text,
            focus: std::mem::take(&mut draft.focus),
        })
    });
    let overlay = session
        .reference_table
        .selected_row()
        .and_then(|row| rows.get(row))
        .is_some_and(|row| matches!(row.values, Values::Waveform { .. }));
    let body = ui.available_height();
    let detail = if overlay {
        (body * 0.45).clamp(DETAIL_MIN_H, DETAIL_MAX_H)
    } else {
        0.0
    };
    let columns = reference_columns();
    let response = DataTable::new("verify.reference", &columns, rows.len())
        .state(&mut session.reference_table)
        .keys(&comparison.keys)
        .freeze(1)
        .verdict(RESULT_COLUMN)
        .editing(editor)
        .max_height((body - detail - 1.0).max(TABLE_HEAD_H + TABLE_ROW_H))
        .show(ui, |index| row_cells(&rows[index]));
    if response.selection_changed {
        ui.ctx().request_repaint();
    }

    let mut entered = None;
    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && open.text.trim() != open.seed.trim()
            {
                entered = Some(open);
            }
        }
        None => {}
    }
    if let Some((row, column)) = response.edit
        && column == TOLERANCE
        && let Some(row) = rows.get(row).filter(|row| row.editable())
        && let Some(target) = row.target.clone()
    {
        let seed = row
            .rule
            .as_ref()
            .map(format_tolerance_rule)
            .unwrap_or_default();
        draft = Some(ReferenceToleranceDraft {
            target,
            text: seed.clone(),
            seed,
            focus: true,
        });
    }

    let mut deleted = None;
    let request = response
        .context_menu
        .filter(|request| rows.get(request.cell.row).is_some_and(ReferenceRow::orphan));
    if let Some((row, _)) = row_menu(ui, "verify.reference", request, |_| vec![("Delete", ())]) {
        deleted = Some(row);
    }
    let selected = session.reference_table.selected_row();
    if response.focused
        && let Some(row) = selected.filter(|row| rows.get(*row).is_some_and(ReferenceRow::orphan))
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
    {
        deleted = Some(row);
    }
    let deleted = deleted.and_then(|row| rows.get(row)?.target.clone());
    let named = |target: &RegressionTargetSelector| {
        rows.iter()
            .find(|row| row.target.as_ref() == Some(target))
            .map_or_else(|| target.name.clone(), |row| row.name.clone())
    };
    let entered = entered.map(|open| {
        let name = named(&open.target);
        (open, name)
    });

    if let Some(row) = selected.and_then(|row| rows.get(row))
        && matches!(row.values, Values::Waveform { .. })
        && let Some(pair) = row
            .target
            .as_ref()
            .and_then(|target| waveform_pair(cx.model, comparison, target))
    {
        waveform_detail(ui, &pair, row.rule.as_ref());
    }
    session.reference_edit = draft;

    if let Some((open, name)) = entered {
        match set_tolerance(cx, &open.target, &name, &open.text) {
            Ok(()) => {
                record_action(&mut cx.state.verification, Ok(()));
                ui.ctx().request_repaint();
            }
            Err(problem) => {
                record_action(
                    &mut cx.state.verification,
                    Err(format!("The tolerance of {name} was not set: {problem}.")),
                );
                cx.state.verification.reference_edit = Some(ReferenceToleranceDraft {
                    focus: true,
                    ..open
                });
            }
        }
    }
    if let Some(target) = deleted {
        let result = delete_rule(cx, &target);
        record_action(&mut cx.state.verification, result);
        ui.ctx().request_repaint();
    }
}

/// What the page says when there is no pair to compare.
fn empty_text(model: &Model, comparison: &ReferenceComparison) -> &'static str {
    if model.simulation.active_run().is_none() {
        "No run yet. Run the plan twice to compare one run against the other."
    } else if comparison
        .references
        .as_ref()
        .is_none_or(|found| found.runs.is_empty())
    {
        "Run the plan again to compare against a reference."
    } else {
        "Choose a reference run to compare this run against."
    }
}

/// Make the retained run at `index` the reference, as the plan's.
fn choose_reference(cx: &mut Cx<'_, SimulateFeature>, index: usize) -> Result<(), String> {
    let run_id = cx
        .model
        .simulation
        .retained
        .runs
        .get(index)
        .map(|run| run.run_id)
        .ok_or("That run is no longer retained.")?;
    commit_regression_baseline(cx, run_id)
        .map_err(|error| format!("The reference was not changed: {error}."))?;
    refresh(cx);
    Ok(())
}

/// Enter `text` as the tolerance of the row compared on `target`, called
/// `name`: the plan's at once, in one plan commit, and the comparison built
/// again on it.
fn set_tolerance(
    cx: &mut Cx<'_, SimulateFeature>,
    target: &RegressionTargetSelector,
    name: &str,
    text: &str,
) -> Result<(), String> {
    let rule = parse_tolerance_rule(text, target)?;
    if let Some(receipt) = commit_regression_tolerance(cx, target, rule, name)? {
        cx.model.record_plan_receipt(receipt);
    }
    refresh(cx);
    Ok(())
}

/// Delete the rule on `target`, whose target neither run holds.
fn delete_rule(
    cx: &mut Cx<'_, SimulateFeature>,
    target: &RegressionTargetSelector,
) -> Result<(), String> {
    remove_orphaned_regression_rules(cx, std::slice::from_ref(target))
        .map_err(|error| format!("The rule was not deleted: {error}."))?;
    refresh(cx);
    Ok(())
}

/// Write the comparison's JUnit and TAP documents, once its receipt is found
/// to still describe the runs and the tolerances in force.
fn export(cx: &mut Cx<'_, SimulateFeature>) -> Result<(), String> {
    let compared = cx
        .state
        .verification
        .reference
        .compared
        .as_ref()
        .ok_or("There is no comparison to export.")?;
    let current = regression_run_pair(cx.model, cx.state).and_then(|(reference, current)| {
        let plan = cx.model.sim_setup.analysis_plan.as_ref()?;
        regression_receipt_matches_contract(
            &compared.receipt,
            plan.id(),
            plan.revision(),
            regression_tolerance_digest(plan_rules(cx.model)),
            reference,
            current,
        )
        .then_some(())
    });
    if current.is_none() {
        return Err(
            "JUnit / TAP export failed: the runs or the tolerances changed since the comparison."
                .to_owned(),
        );
    }
    let (junit, tap) = regression_ci_documents(&compared.receipt, &compared.cases)
        .map_err(|error| format!("JUnit / TAP export failed: {error}."))?;
    export_regression_ci(cx, &junit, &tap);
    Ok(())
}

/// The Result column, the verdict of each row.
const RESULT_COLUMN: usize = 6;

fn reference_columns() -> [Column; 7] {
    [
        Column::name("Check", 132.0),
        Column::number("Reference", 88.0),
        Column::number("Current", 88.0),
        Column::number("\u{0394}", 96.0).tooltip("Current minus reference"),
        Column::number("\u{0394} %", 64.0).tooltip("The change as a share of the reference"),
        // A waveform's rule is a whole phrase, "abs 20m + rel 2 % · skew 20n
        // · 0 … 3u", and a column that elides it states nothing.
        Column::words("Tolerance", 240.0),
        Column::words("Result", 112.0),
    ]
}

/// The change from `reference` to `current`, no finer than the coarser of
/// the two as printed.
fn change_text(reference: f64, current: f64, unit: &str) -> String {
    difference_text(
        current - reference,
        reference.abs().max(current.abs()),
        unit,
    )
}

fn row_cells(row: &ReferenceRow) -> Vec<Cell> {
    let unit = row.unit.as_str();
    let number =
        |value: f64| Cell::number(format_value(value, unit)).sort_by(SortKey::Number(value));
    let tolerance = row
        .rule
        .as_ref()
        .map_or_else(Cell::empty, |rule| Cell::text(format_tolerance_rule(rule)));
    let points =
        |count: usize| Cell::number(format!("{count} pts")).sort_by(SortKey::Number(count as f64));
    let (reference, current, delta, percent, tolerance) = match &row.values {
        Values::Measurement { reference, current } => {
            let delta = current - reference;
            let percent = (*reference != 0.0).then(|| delta / reference.abs() * 100.0);
            (
                number(*reference),
                number(*current),
                Cell::number(change_text(*reference, *current, unit))
                    .sort_by(SortKey::Number(delta)),
                percent.map_or_else(Cell::absent, |percent| {
                    Cell::number(format!("{percent:+.2} %")).sort_by(SortKey::Number(percent))
                }),
                tolerance.editable(),
            )
        }
        Values::Waveform { reference, current } => (
            points(*reference),
            points(*current),
            row.verdict
                .as_ref()
                .and_then(worst_delta)
                .map_or_else(Cell::absent, |delta| {
                    Cell::number(format!("max {}", format_value(delta, unit)))
                        .sort_by(SortKey::Number(delta))
                }),
            Cell::absent(),
            tolerance.editable(),
        ),
        Values::Gap { .. } => (
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            tolerance,
        ),
    };
    vec![
        Cell::text(&row.name),
        reference,
        current,
        delta,
        percent,
        tolerance,
        result_cell(row),
    ]
}

/// A row's result: a change outside its tolerance in the error colour, one
/// inside neutral, a target with no tolerance plain, and a target that could
/// not be compared, or a rule with no target, behind a warning mark.
fn result_cell(row: &ReferenceRow) -> Cell {
    match (&row.values, &row.verdict) {
        (Values::Gap { orphan: true, .. }, _) => Cell::problem("No target", Problem::Warning)
            .hint("Neither run holds this rule's target."),
        (Values::Gap { detail, .. }, _) | (_, Some(RegressionVerdict::NotEvaluated(detail))) => {
            Cell::problem(format!("Blocked \u{00b7} {detail}"), Problem::Warning)
        }
        (_, Some(RegressionVerdict::Pass { .. })) => Cell::verdict(Verdict::Pass),
        (_, Some(RegressionVerdict::Fail { .. })) => Cell::verdict(Verdict::Fail),
        (_, Some(RegressionVerdict::NotConfigured)) => Cell::text("No tolerance"),
        (_, None) => Cell::empty(),
    }
}

/// The one word a row's result states.
fn result_word(row: &ReferenceRow) -> &'static str {
    match (&row.values, &row.verdict) {
        (Values::Gap { orphan: true, .. }, _) => "No target",
        (Values::Gap { .. }, _) | (_, Some(RegressionVerdict::NotEvaluated(_))) => "Blocked",
        (_, Some(RegressionVerdict::Pass { .. })) => "Pass",
        (_, Some(RegressionVerdict::Fail { .. })) => "Fail",
        (_, Some(RegressionVerdict::NotConfigured)) => "No tolerance",
        (_, None) => "",
    }
}

/// What the inspector states of the row selected in the Reference view.
pub(crate) struct ReferenceFacts {
    pub(crate) title: String,
    pub(crate) result: &'static str,
    pub(crate) rows: Vec<(&'static str, String)>,
}

/// The facts of the row selected in the Reference view that its cells do not
/// state in full.
pub(crate) fn selected_reference(session: &VerificationSessionState) -> Option<ReferenceFacts> {
    let comparison = &session.reference;
    let compared = comparison.compared.as_ref()?;
    let row = comparison
        .rows
        .get(session.reference_table.selected_row()?)?;
    let mut rows = Vec::new();
    let unit = row.unit.as_str();
    let of_run = |value: String, run: u64| format!("{value} \u{00b7} Run {run}");
    match &row.values {
        Values::Measurement { reference, current } => {
            rows.push((
                "Reference",
                of_run(format_value(*reference, unit), compared.reference_run),
            ));
            rows.push((
                "Current",
                of_run(format_value(*current, unit), compared.current_run),
            ));
            let delta = current - reference;
            let change = if *reference == 0.0 {
                change_text(*reference, *current, unit)
            } else {
                format!(
                    "{} ({:+.2} %)",
                    change_text(*reference, *current, unit),
                    delta / reference.abs() * 100.0
                )
            };
            rows.push(("Change", change));
        }
        Values::Waveform { reference, current } => {
            rows.push((
                "Reference",
                of_run(format!("{reference} points"), compared.reference_run),
            ));
            rows.push((
                "Current",
                of_run(format!("{current} points"), compared.current_run),
            ));
            if let Some(delta) = row.verdict.as_ref().and_then(worst_delta) {
                rows.push(("Largest change", format_value(delta, unit)));
            }
        }
        Values::Gap { detail, orphan } => {
            rows.push((
                "Problem",
                if *orphan {
                    "Neither run holds this rule's target.".to_owned()
                } else {
                    detail.clone()
                },
            ));
        }
    }
    match &row.rule {
        Some(rule) => {
            // A waveform's skew and window have rows of their own, so the
            // tolerance states only the allowance.
            let text = format_tolerance_rule(rule);
            let allowance = text.split(" \u{00b7} ").next().unwrap_or_default();
            rows.push(("Tolerance", allowance.to_owned()));
            if rule.target.kind == RegressionTargetKind::Waveform {
                rows.push((
                    "Time skew",
                    crate::simulation::dialog::options::format_si_value(rule.time_skew_allowance),
                ));
                rows.push(("Window", format_regression_window(rule.comparison_window)));
            }
        }
        None => rows.push(("Tolerance", "None".to_owned())),
    }
    match &row.verdict {
        Some(
            RegressionVerdict::Pass { allowed_delta, .. }
            | RegressionVerdict::Fail { allowed_delta, .. },
        ) => {
            rows.push(("Allowed change", format_value(*allowed_delta, unit)));
        }
        Some(RegressionVerdict::NotEvaluated(detail)) => {
            rows.push(("Not evaluated", detail.clone()));
        }
        Some(RegressionVerdict::NotConfigured) | None => {}
    }
    Some(ReferenceFacts {
        title: row.name.clone(),
        result: result_word(row),
        rows,
    })
}

/// The waveform the row compared on `target` names in each run of the pair
/// the comparison was built for.
fn waveform_pair<'m>(
    model: &'m Model,
    comparison: &ReferenceComparison,
    target: &RegressionTargetSelector,
) -> Option<RegressionWaveformPair<'m>> {
    let (reference_id, _) = comparison.basis.as_ref()?.pairing.reference?;
    let reference = model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.run_id == reference_id)?;
    let current = model.simulation.active_run()?;
    let locate = |run: &'m crate::state::SimulationRun| {
        let analysis = run.analyses.iter().find(|analysis| {
            verified_analysis(analysis)
                && analysis.provenance.as_ref().is_some_and(|provenance| {
                    provenance.source_domain() == target.source_domain
                        && provenance.source_instance_id() == target.source_instance_id
                })
        })?;
        let trace = analysis
            .waveforms
            .iter()
            .filter(|waveform| waveform.name.eq_ignore_ascii_case(&target.name))
            .nth(usize::try_from(target.occurrence).ok()?)?;
        Some((trace, analysis.analysis_type))
    };
    let (baseline, analysis_type) = locate(reference)?;
    let (current, _) = locate(current)?;
    Some(RegressionWaveformPair {
        target: target.clone(),
        baseline,
        current,
        analysis_type,
    })
}

/// The selected waveform's overlay, in a pane directly under the table.
fn waveform_detail(
    ui: &mut Ui,
    pair: &RegressionWaveformPair<'_>,
    rule: Option<&RegressionToleranceRule>,
) {
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    let left = ui.max_rect().left();
    let width = ui.available_width();
    ui.painter().hline(
        left..=left + width,
        top + 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    let height = ui.available_height().clamp(DETAIL_MIN_H, DETAIL_MAX_H);
    let pane = egui::Rect::from_min_size(egui::pos2(left, top + 1.0), egui::vec2(width, height));
    let _ = ui.allocate_exact_size(pane.size(), egui::Sense::hover());
    #[cfg(any(test, feature = "test-support"))]
    super::raster::probe(ui, "split", pane);
    ui.painter().rect_filled(pane, 0.0, t.color.bg_panel);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(pane)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    plot_ui.set_clip_rect(pane.intersect(ui.clip_rect()));
    waveform_overlay(&mut plot_ui, pair, rule);
}

/// The reference, the current waveform and the tolerance envelope around the
/// reference.
fn waveform_overlay(
    ui: &mut Ui,
    pair: &RegressionWaveformPair<'_>,
    rule: Option<&RegressionToleranceRule>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let height = ui.available_height().max(1.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let reference = pair.baseline;
    let current = pair.current;
    let sweep_unit = axes::sweep_unit(pair.analysis_type);
    let current_color = t.color.traces[0];
    let header = egui::Rect::from_min_size(rect.min, egui::vec2(width, OVERLAY_HEADER_H));
    ui.painter().text(
        egui::pos2(header.left() + TABLE_EDGE_INSET, header.center().y),
        egui::Align2::LEFT_CENTER,
        &current.name,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let mut legend = vec![
        (
            "tolerance",
            egui::Stroke::new(1.0, t.color.text_faint),
            true,
        ),
        ("current", egui::Stroke::new(1.5, current_color), false),
        ("reference", egui::Stroke::new(1.5, t.color.text_dim), false),
    ];
    if rule.is_none() {
        legend.remove(0);
    }
    axes::legend(
        ui,
        header.shrink2(egui::vec2(TABLE_EDGE_INSET, 0.0)),
        &legend,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Waveform {}: reference {} samples, current {} samples",
                current.name,
                reference.y.len(),
                current.y.len()
            ),
        )
    });

    let mut finite_values = reference
        .y
        .iter()
        .chain(current.y.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    if let Some(rule) = rule {
        finite_values.extend(
            reference
                .y
                .iter()
                .copied()
                .filter(|value| value.is_finite())
                .flat_map(|value| {
                    let allowed = permitted_delta(value, rule);
                    [value - allowed, value + allowed]
                }),
        );
    }
    let finite_x = reference
        .x
        .iter()
        .chain(current.x.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    let (Some(y_min), Some(y_max), Some(x_min), Some(x_max)) = (
        finite_values.iter().copied().min_by(f64::total_cmp),
        finite_values.iter().copied().max_by(f64::total_cmp),
        finite_x.iter().copied().min_by(f64::total_cmp),
        finite_x.iter().copied().max_by(f64::total_cmp),
    ) else {
        return;
    };
    let area = egui::Rect::from_min_max(
        header.left_bottom() + egui::vec2(TABLE_EDGE_INSET, 2.0),
        rect.right_bottom() - egui::vec2(TABLE_EDGE_INSET, 4.0),
    );
    if area.width() <= 4.0 || area.height() <= 4.0 {
        return;
    }
    let axes = axes::Axes {
        x: (x_min, x_max),
        y: (y_min, y_max),
        x_unit: sweep_unit,
        y_unit: current.unit.as_deref().unwrap_or(""),
    };
    let plot = axes::paint(ui, area, &axes);
    let points = |x_values: &[f64], y_values: &[f64]| axes::points(plot, &axes, x_values, y_values);
    ui.painter().add(egui::Shape::line(
        points(reference.x.as_slice(), reference.y.as_slice()),
        egui::Stroke::new(1.5, t.color.text_dim),
    ));
    if let Some(rule) = rule {
        for side in [-1.0, 1.0] {
            let envelope = reference
                .y
                .iter()
                .map(|value| *value + side * permitted_delta(*value, rule))
                .collect::<Vec<_>>();
            ui.painter().extend(egui::Shape::dashed_line(
                &points(reference.x.as_slice(), &envelope),
                egui::Stroke::new(1.0, t.color.text_faint),
                4.0,
                3.0,
            ));
        }
    }
    ui.painter().add(egui::Shape::line(
        points(current.x.as_slice(), current.y.as_slice()),
        egui::Stroke::new(1.5, current_color),
    ));
}

#[cfg(test)]
mod tests;
