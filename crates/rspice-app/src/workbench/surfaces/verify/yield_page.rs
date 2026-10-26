//! Monte Carlo yield: the ADE yield view of one run's Monte Carlo trials.
//!
//! One row per output the trials measured, with the plan's limits on it beside
//! its statistics, the way ADE and ADS put the spec beside each output. A
//! limit is entered in its cell. It is the plan's requirement, the one every
//! other page judges against, and the retained trials are judged against it
//! again the frame it changes, with no run.
//!
//! A row's yield is the population the sign-off divides: a trial that did not
//! converge is a failing trial, and a guard band narrows what passes. Its
//! verdict is the plan's Monte Carlo gate when one is set, otherwise every
//! trial. The statistics are computed from the exact samples the run kept; an
//! output whose samples were not kept shows a dash rather than a number nobody
//! can check. Cpk is the one-sided index against a one-sided limit, as a
//! capability study of a single limit publishes it.
//!
//! The toolbar's yield is the joint one: a trial passes only when it meets
//! every limit, so it can be lower than every row's own yield. The selected
//! row's distribution is drawn under the table; until the reader picks a row,
//! the row with the lowest yield is the one shown.

use crate::workbench::framework::command_registry::ids;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::number_format::fmt_si_bound;
use rspice_results::specification::{
    MonteCarloSpecificationGate, SpecPointScope, SpecificationDefinition,
};
use rspice_results::yield_analysis::{SpecLimitType, YieldResult, YieldSpec};
use rspice_vocabulary::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};

use super::super::super::state::{YieldLimitBound, YieldLimitDraft};
use super::histogram::{HistogramModel, histogram_model, paint_histogram};
use super::table::{
    Cell, CellEditor, Column, DataTable, EditOutcome, TABLE_HEAD_H, TABLE_ROW_H, Verdict,
};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// Least and greatest height of the distribution pane under the table.
const DETAIL_MIN_H: f32 = 220.0;
const DETAIL_MAX_H: f32 = 340.0;

/// The columns a limit is entered in.
const LOWER_COLUMN: usize = 1;
const UPPER_COLUMN: usize = 2;

/// One distribution's statistics, computed from its samples.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SampleStats {
    pub(super) mean: f64,
    pub(super) std_dev: f64,
    pub(super) min: f64,
    pub(super) max: f64,
    pub(super) cpk: Option<f64>,
}

/// One output's row.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct YieldRow {
    /// The output as the trials name it.
    pub(super) output: String,
    /// The unit the row's statistics are stated in.
    pub(super) unit: String,
    /// The plan's limit on this output, when it names one.
    pub(super) limit: Option<SpecEntry>,
    pub(super) stats: Option<SampleStats>,
    /// Trials that held, of trials judged, when a bounded limit judged them.
    pub(super) judged: Option<(usize, usize)>,
    /// Whether the output's name can be the measurement a limit bounds.
    pub(super) nameable: bool,
}

impl YieldRow {
    pub(super) fn yield_percent(&self) -> Option<f64> {
        self.judged
            .filter(|(_, total)| *total > 0)
            .map(|(passing, total)| passing as f64 / total as f64 * 100.0)
    }

    /// The row's verdict under the plan's Monte Carlo gate.
    ///
    /// A gated plan asks the gate's own predicate, as the sign-off does, so a
    /// population the sign-off accepts is never a row that reads `Fail`. An
    /// ungated plan has no yield to clear: every trial must hold. A row no
    /// limit judged has no verdict.
    pub(super) fn verdict(&self, gate: &MonteCarloSpecificationGate) -> Option<Verdict> {
        let (passing, total) = self.judged?;
        let holds = match gate {
            MonteCarloSpecificationGate::NotGated => passing == total,
            MonteCarloSpecificationGate::YieldAtLeast { .. } => gate.clears(
                u64::try_from(passing).unwrap_or(u64::MAX),
                u64::try_from(total).unwrap_or(u64::MAX),
            ),
        };
        Some(if holds { Verdict::Pass } else { Verdict::Fail })
    }

    /// The unit a limit on this output is stated in.
    pub(super) fn limit_unit(&self) -> &str {
        self.limit
            .as_ref()
            .map_or(self.unit.as_str(), |limit| limit.unit.as_str())
    }

    pub(super) fn bound(&self, bound: YieldLimitBound) -> Option<f64> {
        let limit = self.limit.as_ref()?;
        match bound {
            YieldLimitBound::Lower => limit.min,
            YieldLimitBound::Upper => limit.max,
        }
    }

    fn has_bound(&self) -> bool {
        self.limit
            .as_ref()
            .is_some_and(|limit| limit.min.is_some() || limit.max.is_some())
    }
}

/// The statistics of the finite `samples`, against the limits of `spec`.
fn sample_stats(samples: &[f64], spec: &YieldSpec) -> Option<SampleStats> {
    let finite = samples
        .iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .collect::<Vec<_>>();
    (!finite.is_empty()).then(|| {
        let computed = rspice_results::yield_analysis::calculate_distribution_stats(&finite, spec);
        SampleStats {
            mean: computed.mean,
            std_dev: computed.std_dev,
            min: computed.min,
            max: computed.max,
            cpk: computed.cpk.filter(|cpk| cpk.is_finite()),
        }
    })
}

/// One row per output a Monte Carlo analysis measured: the plan's limit on it
/// from `specs`, and the population that limit judged from `results`.
pub(super) fn yield_rows_of<W>(
    analysis: &rspice_results::analysis_result::AnalysisResult<W>,
    results: &[YieldResult],
    specs: &[SpecEntry],
) -> Vec<YieldRow> {
    let Some(AnalysisResultFamilyMetadata::MonteCarlo {
        variables,
        member_measurements,
        ..
    }) = &analysis.family_metadata
    else {
        return Vec::new();
    };
    variables
        .iter()
        .map(|variable| {
            let judged = results
                .iter()
                .find(|result| result.spec.target == variable.name);
            let limit = specs
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(&variable.name))
                .cloned();
            let (unit, stats) = match judged {
                Some(result) => (
                    result.spec.unit.clone(),
                    sample_stats(&result.samples, &result.spec),
                ),
                None => {
                    let unit = member_measurements
                        .iter()
                        .find_map(|member| {
                            member
                                .evidence_for(&variable.name)?
                                .unit
                                .as_ref()?
                                .symbol()
                                .map(str::to_owned)
                        })
                        .or_else(|| limit.as_ref().map(|limit| limit.unit.clone()))
                        .unwrap_or_default();
                    let unbounded = YieldSpec {
                        target: variable.name.clone(),
                        limit_type: SpecLimitType::Range,
                        min: None,
                        max: None,
                        target_val: None,
                        unit: unit.clone(),
                        weight: 1.0,
                    };
                    let stats = sample_stats(&variable.samples, &unbounded);
                    (unit, stats)
                }
            };
            YieldRow {
                output: variable.name.clone(),
                unit,
                limit,
                stats,
                judged: judged.map(|result| (result.pass_count, result.total_runs)),
                nameable: rspice_vocabulary::text_validation::validate_parameter_name(
                    &variable.name,
                )
                .is_ok(),
            }
        })
        .collect()
}

/// The rows of the run Verify reads, computed once per retained generation.
///
/// The statistics sort every sample of every output; asked for on every
/// frame, that is the most expensive thing on the page for values that cannot
/// change until the run's evidence does. The frame judges the trials again
/// whenever a limit changes, and that is a new data version.
fn yield_rows(ctx: &egui::Context, app: &RSpiceApp) -> Vec<YieldRow> {
    let simulation = &app.state.model.simulation;
    let Some(analysis) = simulation.yield_analysis() else {
        return Vec::new();
    };
    let key = egui::Id::new((
        "workbench.verify.yield.rows",
        verification_run(app).map(|run| run.dataset_id.to_string()),
        analysis.id,
        simulation.view.data_version,
    ));
    if let Some(rows) = ctx.data(|data| data.get_temp::<Vec<YieldRow>>(key)) {
        return rows;
    }
    let rows = yield_rows_of(
        &analysis.data,
        verification_yield_results(app),
        &verification_specs(app),
    );
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

/// The toolbar's line: the joint yield over every judged limit, in the
/// failing tone when a row fails.
pub(super) fn yield_summary(results: &[YieldResult], failing: bool) -> Option<Summary> {
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
            if failing {
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

/// What a typed limit says in `unit`: `None` for an empty cell, which
/// removes the bound.
///
/// Through the interactive quantity parser under the reader's own policy, so
/// a cell reads `10 MHz`, `900m` and `40 dB` the way every other field does,
/// and refuses a spelling the engine would read as another value rather than
/// quietly taking `10M` for ten milli.
pub(super) fn parse_limit(
    text: &str,
    unit: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<Option<f64>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    let kind = match unit {
        "s" => QuantityInputKind::Time,
        "Hz" => QuantityInputKind::Frequency,
        _ => QuantityInputKind::EngineeringScalar,
    };
    let source = if kind == QuantityInputKind::EngineeringScalar && !unit.is_empty() {
        text.strip_suffix(unit).unwrap_or(text).trim_end()
    } else {
        text
    };
    parse_ui_quantity(source, kind, policy, locale)
        .map(Some)
        .map_err(|error| error.to_string())
}

/// The words a limit cell opens with: what it shows, when typing that back
/// gives the same number, otherwise the whole value, which always does.
fn limit_spelling(
    value: f64,
    unit: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> String {
    let shown = fmt_si_bound(value, unit);
    if parse_limit(&shown, unit, policy, locale) == Ok(Some(value)) {
        shown
    } else if unit.is_empty() {
        format!("{value}")
    } else {
        format!("{value} {unit}")
    }
}

const fn bound_word(bound: YieldLimitBound) -> &'static str {
    match bound {
        YieldLimitBound::Lower => "lower",
        YieldLimitBound::Upper => "upper",
    }
}

/// Set one bound of `output`'s limit in the plan's governed requirements,
/// or remove it with `None`, and return the line the plan's receipt states.
///
/// The requirement keeps everything this cell does not state — its scope,
/// role, guard band, producing analysis and expression — and an authored
/// `EqualWithin` keeps its shape until one of its bounds is edited. A limit
/// left with no bound stays a tracked requirement: removing a requirement is
/// the specification editor's to do, not a side effect of clearing a cell.
pub(super) fn set_limit(
    workspace: &mut rspice_project::ProjectWorkspace,
    plan_id: rspice_vocabulary::product::SimulationPlanId,
    output: &str,
    bound: YieldLimitBound,
    value: Option<f64>,
    unit: &str,
) -> Result<String, String> {
    let mut definitions = workspace
        .ensure_active_plan_data(plan_id)
        .specification_definitions
        .clone();
    let existing = definitions
        .iter()
        .position(|definition| definition.measurement.eq_ignore_ascii_case(output));
    let mut entry = existing.map_or_else(
        || SpecEntry {
            measurement: output.to_owned(),
            expression: String::new(),
            min: None,
            max: None,
            unit: unit.to_owned(),
            scope: SpecPointScope::AllPoints,
        },
        |index| definitions[index].projected_entry(),
    );
    match bound {
        YieldLimitBound::Lower => entry.min = value,
        YieldLimitBound::Upper => entry.max = value,
    }
    entry.validate()?;
    match existing {
        Some(index) => definitions[index].apply_legacy_projection(&entry),
        None if value.is_some() => {
            definitions.push(SpecificationDefinition::new_from_projection(&entry));
        }
        None => return Err(format!("{output} has no {} limit", bound_word(bound))),
    }
    workspace.replace_active_specification_definitions(plan_id, definitions);
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    Ok(match value {
        Some(value) => format!(
            "Set the {} limit of {output} to {}.",
            bound_word(bound),
            fmt_si_bound(value, &entry.unit)
        ),
        None => format!("Removed the {} limit of {output}.", bound_word(bound)),
    })
}

pub(super) fn yield_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let runs = run_choice(app, |run| {
        run.analyses.iter().any(|analysis| {
            analysis.success
                && matches!(
                    analysis.family_metadata,
                    Some(AnalysisResultFamilyMetadata::MonteCarlo { .. })
                )
        })
    });
    let run_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let results = verification_yield_results(app);
    let rows = yield_rows(ui.ctx(), app);
    let plan_id = verification_plan_id(app);
    let entry_allowed =
        plan_id.is_some() && verification_evidence_domain(app).answers_a_plan_limit();
    let gate = plan_id
        .and_then(|plan_id| app.state.model.workspace.content.plan_data(plan_id))
        .map(|payload| payload.specification_policy.monte_carlo.clone())
        .unwrap_or_default();
    let policy = app.state.model.preferences.quantity_presentation_policy();
    let locale = app.state.workbench.number_locale;
    let shown = shown_yield_row(
        &rows,
        app.state
            .simulate
            .verification
            .yield_selected_spec
            .as_deref(),
    );
    let analysis = app.state.model.simulation.yield_analysis();
    let trials = analysis.and_then(|analysis| match &analysis.family_metadata {
        Some(AnalysisResultFamilyMetadata::MonteCarlo {
            runs_requested,
            member_measurements,
            variables,
            ..
        }) => Some((*runs_requested, member_measurements.is_empty(), variables)),
        _ => None,
    });
    let histogram: Option<HistogramModel> = shown.and_then(|index| {
        let row = &rows[index];
        match results
            .iter()
            .find(|result| result.spec.target == row.output)
        {
            Some(result) => histogram_model(&result.samples, result.spec.min, result.spec.max),
            None => trials
                .and_then(|(_, _, variables)| {
                    variables
                        .iter()
                        .find(|variable| variable.name == row.output)
                })
                .and_then(|variable| histogram_model(&variable.samples, None, None)),
        }
    });
    let failing = rows
        .iter()
        .any(|row| row.verdict(&gate) == Some(Verdict::Fail));
    let summary = yield_summary(results, failing)
        .or_else(|| {
            trials.map(|(count, _, _)| {
                if rows.iter().any(YieldRow::has_bound) {
                    Summary::plain(format!("{count} trials \u{b7} not judged"))
                } else {
                    Summary::plain(format!("{count} trials \u{b7} no limits"))
                }
            })
        })
        .map(|summary| {
            if runs.offers_choice() {
                summary
            } else {
                append_run(summary, run_name.as_deref())
            }
        });
    // A run from before trials kept their own records can show its outputs,
    // but nothing can judge them.
    let unrecorded =
        trials.is_some_and(|(_, unrecorded, _)| unrecorded) && rows.iter().any(YieldRow::has_bound);
    let actions = [ToolbarAction::new("Edit specifications")];
    let output = PageToolbar {
        title: "Monte Carlo yield",
        runs: &runs,
        summary,
        actions: &actions,
    }
    .show(ui);
    if output.action == Some(0) {
        app.execute_command_key(ids::EDIT_SPECIFICATIONS);
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.model.simulation, index);
    }
    action_problem_line(ui, app);
    if unrecorded {
        let _ = rspice_ui_kit::widgets::problem_line(
            ui,
            Problem::Warning,
            "This run kept no per-trial records; run it again to judge its yield.",
        );
    }

    if rows.is_empty() {
        app.state.simulate.verification.yield_limit_draft = None;
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
            record_action(&mut app.state.simulate.verification, result);
        }
        return;
    }

    let body = ui.available_height();
    let detail = (body * 0.45).clamp(DETAIL_MIN_H, DETAIL_MAX_H);
    let table_height = (body - detail - 1.0).max(TABLE_HEAD_H + TABLE_ROW_H);
    let columns = [
        Column::name("Output", 120.0),
        Column::number("Lower", 96.0)
            .tooltip("Lower limit: a trial passes at or above it. Click to enter one."),
        Column::number("Upper", 96.0)
            .tooltip("Upper limit: a trial passes at or below it. Click to enter one."),
        Column::number("Mean", 96.0),
        Column::number("Std dev", 96.0),
        Column::number("Min", 96.0),
        Column::number("Max", 96.0),
        Column::number("Cpk", 64.0),
        Column::number("Yield", 76.0),
        Column::words("Pass/fail", 70.0),
    ];
    let mut draft = app
        .state
        .simulate
        .verification
        .yield_limit_draft
        .take()
        .filter(|draft| entry_allowed && rows.iter().any(|row| row.output == draft.output));
    let editor = draft.as_mut().and_then(|draft| {
        Some(CellEditor {
            row: rows.iter().position(|row| row.output == draft.output)?,
            column: match draft.bound {
                YieldLimitBound::Lower => LOWER_COLUMN,
                YieldLimitBound::Upper => UPPER_COLUMN,
            },
            text: &mut draft.text,
            focus: std::mem::take(&mut draft.focus),
        })
    });
    let response = DataTable::new("verify.yield.specs", &columns, rows.len())
        .clickable()
        .selected(shown)
        .max_height(table_height)
        .editing(editor)
        .show(ui, |index| {
            let row = &rows[index];
            let unit = row.unit.as_str();
            let stat = |value: Option<f64>| {
                value.map_or_else(Cell::absent, |value| {
                    Cell::number(format_value(value, unit))
                })
            };
            let limit = |bound: YieldLimitBound| {
                let cell = row.bound(bound).map_or_else(Cell::absent, |value| {
                    Cell::number(fmt_si_bound(value, row.limit_unit()))
                });
                if !entry_allowed {
                    cell
                } else if row.nameable {
                    cell.editable()
                } else {
                    cell.hint("A limit bounds a measurement, and this output's name is not one.")
                }
            };
            vec![
                Cell::text(&row.output),
                limit(YieldLimitBound::Lower),
                limit(YieldLimitBound::Upper),
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
                row.verdict(&gate).map_or_else(Cell::empty, Cell::verdict),
            ]
        });
    if let Some(index) = response.clicked {
        app.state.simulate.verification.yield_selected_spec = Some(rows[index].output.clone());
    }
    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && open.text.trim() != open.seed.trim()
            {
                let unit = rows
                    .iter()
                    .find(|row| row.output == open.output)
                    .map_or_else(String::new, |row| row.limit_unit().to_owned());
                let applied = parse_limit(&open.text, &unit, policy, locale).and_then(|value| {
                    let plan_id = plan_id.ok_or_else(|| "the plan is not ready".to_owned())?;
                    let mut workspace = app.state.model.workspace.clone();
                    let mut setup = app.state.model.sim_setup.clone();
                    let detail = set_limit(
                        &mut workspace.content,
                        plan_id,
                        &open.output,
                        open.bound,
                        value,
                        &unit,
                    )?;
                    let receipt = setup
                        .commit_active_plan_configuration_change(detail)
                        .map_err(|error| error.to_string())?;
                    app.state.model.workspace = workspace;
                    app.state.model.sim_setup = setup;
                    Ok(receipt.status_line())
                });
                match applied {
                    Ok(receipt) => {
                        invalidate_plan_bound_preflight(app);
                        app.state.record_plan_receipt(receipt);
                        record_action(&mut app.state.simulate.verification, Ok(()));
                        ui.ctx().request_repaint();
                    }
                    Err(problem) => {
                        record_action(
                            &mut app.state.simulate.verification,
                            Err(format!(
                                "The {} limit of {} was not set: {problem}.",
                                bound_word(open.bound),
                                open.output
                            )),
                        );
                        draft = Some(YieldLimitDraft {
                            focus: true,
                            ..open
                        });
                    }
                }
            }
        }
        None => {}
    }
    if let Some((index, column)) = response.edit {
        let row = &rows[index];
        let bound = if column == LOWER_COLUMN {
            YieldLimitBound::Lower
        } else {
            YieldLimitBound::Upper
        };
        let seed = row
            .bound(bound)
            .map(|value| limit_spelling(value, row.limit_unit(), policy, locale))
            .unwrap_or_default();
        draft = Some(YieldLimitDraft {
            output: row.output.clone(),
            bound,
            text: seed.clone(),
            seed,
            focus: true,
        });
    }
    app.state.simulate.verification.yield_limit_draft = draft;

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
        Some(model) => paint_histogram(ui, &row.output, &row.unit, model, height),
        None => {
            let _ = empty_line(
                ui,
                &format!("This run kept no samples of {}.", row.output),
                None,
            );
        }
    }
}
