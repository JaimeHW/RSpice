//! Optimization convergence plots and exact retained candidate presentation.

use crate::presentation::{PlotView, stat_table, with_table_state};
use crate::strip::StripHeader;
use egui::{RichText, Ui};
use rspice_results::optimization::OptimizationObjectiveGoal;
use rspice_results::waveform::{HeldColumn, RetainedTrace};
use rspice_results::{analysis_result::AnalysisResult, optimization::history::OptimizationView};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_significant};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable, LineHeights, SortKey, Tone};
use rspice_ui_kit::widgets::{Button, SegmentedWidth, segmented};
use std::collections::BTreeMap;

/// History and display inputs qualified for the host's active analysis.
pub struct OptimizationPlot<'a> {
    pub analysis_id: u64,
    pub label: &'a str,
    pub history: OptimizationView<'a>,
    pub selected: Option<usize>,
    pub plot_view: PlotView,
    pub cost_range: (f64, f64),
    /// Each design variable's least and greatest value, in the history's
    /// order.
    pub variable_ranges: &'a [(f64, f64)],
    /// The table at the sheet's foot.
    pub table: OptimizationTable,
    /// The reader's displayed significant digits.
    pub significant_digits: usize,
}

/// Navigation and candidate requests to apply to the same bound source.
pub struct OptimizationResponse {
    pub fit: bool,
    pub view: plot::ViewChange,
    pub selected: Option<usize>,
}

/// Exact metadata and index resolved by the host's candidate-selection gate.
pub struct CandidateView<'a> {
    pub analysis: &'a AnalysisResult<RetainedTrace>,
    pub index: usize,
    pub iterations: &'a [f64],
    pub best_cost: &'a f64,
    pub best_variables: &'a BTreeMap<String, f64>,
    /// The reader's displayed significant digits.
    pub significant_digits: usize,
}

/// A value at the reader's digits, in its own unit: an SI prefix where the
/// unit takes one, and none on a decibel or a percentage, which `500 mdB`
/// would misstate. Full precision belongs to copy and export.
fn value_text(value: f64, unit: &str, digits: usize) -> String {
    fmt_si_significant(value, unit, digits)
}

/// What an objective asks of its measurement, in the reader's words.
fn goal_word(goal: OptimizationObjectiveGoal) -> &'static str {
    match goal {
        OptimizationObjectiveGoal::Minimize => "minimize",
        OptimizationObjectiveGoal::Maximize => "maximize",
        OptimizationObjectiveGoal::Target => "reach the target",
    }
}

/// A difference, signed so the direction reads at a glance, in the notation
/// every other value on the sheet uses.
fn signed_readout(delta: f64, digits: usize) -> String {
    let sign = if delta < 0.0 { "−" } else { "+" };
    format!("{sign}{}", value_text(delta.abs(), "", digits))
}

/// An optimizer iteration: a count, never a measurement, so it prints whole
/// rather than as `3.000000000e0`.
fn iteration_text(iteration: f64, digits: usize) -> String {
    if iteration.fract() == 0.0 && iteration.abs() < 1e15 {
        format!("{}", iteration as i64)
    } else {
        value_text(iteration, "", digits)
    }
}

fn nearest_candidate_index(iterations: &[f64], requested: f64) -> Option<usize> {
    iterations
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .min_by(|(_, left), (_, right)| {
            (**left - requested)
                .abs()
                .total_cmp(&(**right - requested).abs())
        })
        .map(|(index, _)| index)
}

/// Which of the run's tables stands at the sheet's foot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizationTable {
    /// Every evaluated candidate: its cost and its variables.
    #[default]
    Candidates,
    /// Each goal as the best candidate met it.
    Goals,
    /// Each constraint as the best candidate held it.
    Constraints,
}

impl OptimizationTable {
    const ALL: [Self; 3] = [Self::Candidates, Self::Goals, Self::Constraints];

    const fn label(self) -> &'static str {
        match self {
            Self::Candidates => "Candidates",
            Self::Goals => "Goals",
            Self::Constraints => "Constraints",
        }
    }

    /// The tables the run holds rows for: goals and constraints only when
    /// the run states some.
    fn offered(view: &OptimizationView<'_>) -> Vec<Self> {
        Self::ALL
            .into_iter()
            .filter(|table| match table {
                Self::Candidates => true,
                Self::Goals => !view.best_objectives.is_empty(),
                Self::Constraints => !view.best_constraints.is_empty(),
            })
            .collect()
    }

    /// This table if the run holds it, else the candidates.
    fn held(self, view: &OptimizationView<'_>) -> Self {
        if Self::offered(view).contains(&self) {
            self
        } else {
            Self::Candidates
        }
    }
}

/// What the sheet's bar asked for.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationBarResponse {
    pub table: Option<OptimizationTable>,
    pub apply_best: bool,
}

/// The sheet's bar: which table stands at the foot, and "Apply best to
/// design", which writes the best candidate into the plan's design
/// variables. `apply` is what the button will write, or why it cannot.
pub fn domain_bar(
    ui: &mut Ui,
    view: &OptimizationView<'_>,
    table: OptimizationTable,
    apply: Result<&str, &str>,
) -> OptimizationBarResponse {
    let mut out = OptimizationBarResponse::default();
    let offered = OptimizationTable::offered(view);
    if offered.len() > 1 {
        crate::chrome::bars::bar_label(ui, "Table");
        let labels: Vec<&str> = offered.iter().map(|table| table.label()).collect();
        let mut index = offered
            .iter()
            .position(|candidate| *candidate == table.held(view))
            .unwrap_or_default();
        if segmented(
            ui,
            "rspice.results.optimization.table",
            &labels,
            &mut index,
            SegmentedWidth::Natural,
        ) {
            out.table = offered.get(index).copied();
        }
        ui.add_space(8.0);
    }
    let (enabled, hover) = match apply {
        Ok(writes) => (true, writes),
        Err(reason) => (false, reason),
    };
    let response = Button::new("Apply best to design")
        .enabled(enabled)
        .show(ui);
    let response = if enabled {
        response.on_hover_text(hover)
    } else {
        response.on_disabled_hover_text(hover)
    };
    out.apply_best = enabled && response.clicked();
    out
}

/// Share of the sheet the table at the foot may take before it scrolls.
const TABLE_SHARE: f32 = 0.4;
/// Rows the table keeps however short the sheet.
const TABLE_LEAST_ROWS: f32 = 3.0;

pub fn show(
    ui: &mut Ui,
    input: OptimizationPlot<'_>,
    cache: &mut plot::DecimationCache,
) -> OptimizationResponse {
    let OptimizationPlot {
        analysis_id,
        label,
        history: view,
        selected,
        plot_view,
        cost_range,
        variable_ranges,
        table,
        significant_digits: digits,
    } = input;
    let mut requested = None;
    // The table and the histories show every candidate at once: a stored
    // history is read whole off the frame, and is shown once it has arrived.
    let held = |values: &rspice_results::waveform::WaveformValues| {
        values.fetch_whole().ready().unwrap_or_default()
    };
    let cost = held(&view.cost.y);
    let outcome = if view.converged {
        "converged"
    } else {
        "stopped"
    };
    let best_index = view.best_index;

    let header = StripHeader::new(
        "OPTIMIZATION",
        &format!(
            "{} · {} iterations · {} · best cost {}",
            label,
            view.iterations.len(),
            outcome,
            value_text(view.best_cost, "", digits)
        ),
        &[],
    )
    .zoomed(plot_view.is_zoomed())
    .show(ui);

    let table = table.held(&view);
    let area = ui.available_rect_before_wrap();
    let lines = LineHeights::grid(&Tokens::get(ui.ctx()));
    let rows = match table {
        OptimizationTable::Candidates => view.iterations.len(),
        OptimizationTable::Goals => view.best_objectives.len(),
        OptimizationTable::Constraints => view.best_constraints.len(),
    }
    .max(1) as f32;
    let cap = ((area.height() * TABLE_SHARE - lines.head) / lines.row)
        .floor()
        .max(TABLE_LEAST_ROWS);
    let table_height = (lines.head + rows.min(cap) * lines.row).min(area.height());
    let table_area = egui::Rect::from_min_max(
        egui::pos2(area.left(), (area.bottom() - table_height).max(area.top())),
        area.max,
    );
    let plot_area = egui::Rect::from_min_max(area.min, egui::pos2(area.right(), table_area.top()));

    // The cost and every design variable, one strip each over the shared
    // iteration axis, so a step in a variable reads against the cost it
    // bought.
    let variables = view
        .variables
        .iter()
        .map(|(name, waveform)| (*name, held(&waveform.y)))
        .collect::<Vec<_>>();
    let histories = std::iter::once(("cost", &cost[..], cost_range))
        .chain(
            variables
                .iter()
                .zip(
                    variable_ranges
                        .iter()
                        .copied()
                        .chain(std::iter::repeat((0.0, 1.0))),
                )
                .map(|((name, values), range)| (*name, &values[..], range)),
        )
        .collect::<Vec<_>>();
    let auto_x0 = view.iterations[0];
    let auto_x1 = *view.iterations.last().unwrap_or(&auto_x0);
    let x_pad = if auto_x0 < auto_x1 {
        (auto_x1 - auto_x0) * 0.025
    } else {
        auto_x0.abs().mul_add(0.025, 1.0)
    };
    let (x0, x1) = plot_view.x.unwrap_or((auto_x0 - x_pad, auto_x1 + x_pad));
    let strip_height = plot_area.height() / histories.len() as f32;
    let mut view_change = plot::ViewChange::default();
    for (strip, &(name, values, range)) in histories.iter().enumerate() {
        let rect = egui::Rect::from_min_size(
            egui::pos2(
                plot_area.left(),
                strip_height.mul_add(strip as f32, plot_area.top()),
            ),
            egui::vec2(plot_area.width(), strip_height),
        );
        let mut strip_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(("rspice.results.optimization-history", strip))
                .max_rect(rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let response = history_strip(
            &mut strip_ui,
            &HistoryStrip {
                name,
                iterations: view.iterations,
                values,
                x: (x0, x1),
                y: if strip == 0 {
                    plot_view.y.unwrap_or_else(|| plot::padded_fit(range))
                } else {
                    plot::padded_fit(range)
                },
                best_index,
                selected,
                color: Tokens::get(ui.ctx()).color.traces
                    [strip % Tokens::get(ui.ctx()).color.traces.len()],
                cache_key: 0x4F50_5400_u64 ^ analysis_id.rotate_left(17) ^ strip as u64,
                digits,
            },
            cache,
        );
        if let Some(index) = response
            .clicked_x
            .and_then(|iteration| nearest_candidate_index(view.iterations, iteration))
        {
            requested = Some(index);
        }
        if !view_change.any() && response.view.any() {
            // Only the cost strip's value axis is the reader's to set; the
            // variables' strips each fit their own.
            view_change = plot::ViewChange {
                y: if strip == 0 { response.view.y } else { None },
                ..response.view
            };
        }
    }

    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("rspice.results.optimization-table")
            .max_rect(table_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    match table {
        OptimizationTable::Candidates => {
            if let Some(index) = candidates_table(
                &mut table_ui,
                &CandidateRows {
                    view: &view,
                    cost: &cost,
                    variables: &variables,
                    selected,
                    analysis_id,
                    digits,
                },
                table_area.height(),
            ) {
                requested = Some(index);
            }
        }
        OptimizationTable::Goals => goals_table(&mut table_ui, &view, table_area.height(), digits),
        OptimizationTable::Constraints => {
            constraints_table(&mut table_ui, &view, table_area.height(), digits);
        }
    }
    ui.advance_cursor_after_rect(area);
    OptimizationResponse {
        fit: header.fit_clicked,
        view: view_change,
        selected: requested,
    }
}

/// One history's strip: its values at each iteration, the best candidate
/// marked, and the selected one when it is another.
struct HistoryStrip<'a> {
    name: &'a str,
    iterations: &'a [f64],
    values: &'a [f64],
    x: (f64, f64),
    y: (f64, f64),
    best_index: usize,
    selected: Option<usize>,
    color: egui::Color32,
    cache_key: u64,
    digits: usize,
}

fn history_strip(
    ui: &mut Ui,
    strip: &HistoryStrip<'_>,
    cache: &mut plot::DecimationCache,
) -> plot::PlotResponse {
    let digits = strip.digits;
    let accessible_name = format!("Optimization history of {}", strip.name);
    let mut spec = PlotSpec::new(
        Axis::linear_with(strip.x.0, strip.x.1, "", 7).with_label("iteration"),
        XScale::Linear,
        Axis::linear_with(strip.y.0, strip.y.1, "", 5).with_label(strip.name),
    )
    .accessible_name(&accessible_name)
    .accessible_detail("Exact retained value at each evaluated candidate.");
    if strip.values.len() == strip.iterations.len() {
        spec.traces.push(
            Trace::new(strip.iterations, strip.values, strip.color)
                .marker_style(0)
                .cache_key(strip.cache_key),
        );
    }
    let value_at = |index: usize| strip.values.get(index).copied().unwrap_or(f64::NAN);
    if let Some(&best) = strip.iterations.get(strip.best_index) {
        spec.markers.push(plot::Marker {
            x: best,
            y: value_at(strip.best_index),
            // The best candidate is a fact, not a success: primary ink.
            color: Tokens::get(ui.ctx()).color.text,
            label: format!(
                "best {}",
                value_text(value_at(strip.best_index), "", digits)
            ),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    if let Some(selection) = strip.selected
        && selection != strip.best_index
        && let Some(&iteration) = strip.iterations.get(selection)
    {
        spec.markers.push(plot::Marker {
            x: iteration,
            y: value_at(selection),
            color: Tokens::get(ui.ctx()).color.accent,
            label: format!("selected {}", value_text(value_at(selection), "", digits)),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    let best_value = value_at(strip.best_index);
    let readout = |iteration: f64| {
        nearest_candidate_index(strip.iterations, iteration).map_or_else(Vec::new, |index| {
            vec![
                (
                    "iteration".to_owned(),
                    iteration_text(strip.iterations[index], digits),
                ),
                (
                    strip.name.to_owned(),
                    value_text(value_at(index), "", digits),
                ),
                (
                    "Δ best".to_owned(),
                    signed_readout(value_at(index) - best_value, digits),
                ),
            ]
        })
    };
    plot::show(ui, &spec, cache, None, Some(&readout))
}

/// What the candidates' table reads.
struct CandidateRows<'a, 'v> {
    view: &'a OptimizationView<'v>,
    cost: &'a [f64],
    variables: &'a [(&'v str, HeldColumn)],
    selected: Option<usize>,
    analysis_id: u64,
    digits: usize,
}

/// Every evaluated candidate: a column for the iteration, the cost and its
/// distance from the best, then one per design variable. Returns the row
/// the reader picked.
fn candidates_table(ui: &mut Ui, rows: &CandidateRows<'_, '_>, height: f32) -> Option<usize> {
    let CandidateRows {
        view,
        cost,
        variables,
        selected,
        analysis_id,
        digits,
    } = *rows;
    let cost_at = |index: usize| cost.get(index).copied().unwrap_or(f64::NAN);
    let outcome = if view.converged {
        "converged"
    } else {
        "stopped"
    };
    let mut columns = vec![
        Column::name("Iteration", 64.0).align(Align::Right),
        Column::name("Cost", 72.0).align(Align::Right),
        Column::name("\u{0394} from best", 72.0).align(Align::Right),
    ];
    columns.extend(
        variables
            .iter()
            .map(|(name, _)| Column::name(*name, 72.0).align(Align::Right)),
    );
    columns.push(Column::name("Disposition", 96.0));
    let candidates = view.iterations.len();
    let best_index = view.best_index;
    let revision = egui::Id::new(("optimization", analysis_id, candidates, digits)).value();
    let mut requested = None;
    with_table_state(ui, "optimization", |ui, state| {
        if state.selected_row() != selected {
            match selected {
                Some(index) => state.select(index, None),
                None => state.selection = None,
            }
        }
        let response = DataTable::new("rspice.results.optimization", &columns, candidates)
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(height)
            .show(ui, |index| {
                let iteration = view.iterations[index];
                let cost = cost_at(index);
                let mut cells = vec![
                    Cell::number(iteration_text(iteration, digits))
                        .sort_by(SortKey::Number(iteration)),
                    Cell::number(value_text(cost, "", digits)).sort_by(SortKey::Number(cost)),
                    Cell::number(signed_readout(cost - view.best_cost, digits))
                        .sort_by(SortKey::Number(cost - view.best_cost)),
                ];
                cells.extend(variables.iter().map(|(_, values)| {
                    let value = values.get(index).copied().unwrap_or(f64::NAN);
                    Cell::number(value_text(value, "", digits)).sort_by(SortKey::Number(value))
                }));
                let last = index + 1 == candidates;
                cells.push(if best_index == index {
                    disposition(
                        if last && view.converged {
                            "best · converged"
                        } else {
                            "best"
                        },
                        true,
                    )
                } else if last {
                    disposition(outcome, view.converged)
                } else {
                    Cell::text("evaluated")
                });
                cells
            });
        if response.clicked.is_some() || response.selection_changed {
            requested = state.selected_row();
        }
    });
    requested
}

/// Each goal as the best candidate met it: what it measures, what it asks,
/// the value reached and what that cost.
fn goals_table(ui: &mut Ui, view: &OptimizationView<'_>, height: f32, digits: usize) {
    let columns = [
        Column::name("Measurement", 96.0),
        Column::name("Goal", 80.0),
        Column::name("Value", 72.0).align(Align::Right),
        Column::name("Target", 72.0).align(Align::Right),
        Column::name("Scale", 64.0).align(Align::Right),
        Column::name("Weight", 56.0).align(Align::Right),
        Column::name("Cost", 64.0).align(Align::Right),
    ];
    let goals = view.best_objectives;
    let revision = egui::Id::new(("optimization-goals", goals.len(), digits)).value();
    with_table_state(ui, "optimization-goals", |ui, state| {
        DataTable::new("rspice.results.optimization-goals", &columns, goals.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(height)
            .show(ui, |index| {
                let observation = &goals[index];
                let term = &observation.objective;
                vec![
                    Cell::text(term.measurement.clone()),
                    Cell::text(goal_word(term.goal)),
                    Cell::number(value_text(observation.value, &term.unit, digits))
                        .sort_by(SortKey::Number(observation.value)),
                    term.target.map_or_else(Cell::absent, |target| {
                        Cell::number(value_text(target, &term.unit, digits))
                            .sort_by(SortKey::Number(target))
                    }),
                    Cell::number(value_text(term.scale, &term.unit, digits))
                        .sort_by(SortKey::Number(term.scale)),
                    Cell::number(value_text(term.weight, "", digits))
                        .sort_by(SortKey::Number(term.weight)),
                    Cell::number(value_text(observation.contribution, "", digits))
                        .sort_by(SortKey::Number(observation.contribution)),
                ]
            });
    });
}

/// Each constraint as the best candidate held it: its limits, the value
/// reached and by how much it broke them.
fn constraints_table(ui: &mut Ui, view: &OptimizationView<'_>, height: f32, digits: usize) {
    let columns = [
        Column::name("Measurement", 96.0),
        Column::name("Value", 72.0).align(Align::Right),
        Column::name("Lower", 72.0).align(Align::Right),
        Column::name("Upper", 72.0).align(Align::Right),
        Column::name("Tolerance", 64.0).align(Align::Right),
        Column::name("Scale", 64.0).align(Align::Right),
        Column::name("Violation", 64.0).align(Align::Right),
    ];
    let constraints = view.best_constraints;
    let revision = egui::Id::new(("optimization-constraints", constraints.len(), digits)).value();
    with_table_state(ui, "optimization-constraints", |ui, state| {
        DataTable::new(
            "rspice.results.optimization-constraints",
            &columns,
            constraints.len(),
        )
        .state(state)
        .freeze(1)
        .revision(revision)
        .max_height(height)
        .show(ui, |index| {
            let observation = &constraints[index];
            let term = &observation.constraint;
            let bound = |value: Option<f64>| {
                value.map_or_else(Cell::absent, |value| {
                    Cell::number(value_text(value, &term.unit, digits))
                        .sort_by(SortKey::Number(value))
                })
            };
            let violation = Cell::number(value_text(observation.violation, "", digits))
                .sort_by(SortKey::Number(observation.violation));
            vec![
                Cell::text(term.measurement.clone()),
                Cell::number(value_text(observation.value, &term.unit, digits))
                    .sort_by(SortKey::Number(observation.value)),
                bound(term.lower),
                bound(term.upper),
                Cell::number(value_text(term.tolerance, &term.unit, digits))
                    .sort_by(SortKey::Number(term.tolerance)),
                Cell::number(value_text(term.scale, &term.unit, digits))
                    .sort_by(SortKey::Number(term.scale)),
                if observation.violation > 0.0 {
                    violation.toned(Tone::Caution)
                } else {
                    violation
                },
            ]
        });
    });
}

pub fn right_panel(ui: &mut Ui, input: CandidateView<'_>) {
    let CandidateView {
        analysis,
        index,
        iterations,
        best_cost,
        best_variables,
        significant_digits: digits,
    } = input;
    let cost = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "OPT_COST")
        .and_then(|waveform| waveform.y.sample(index));

    section_header(ui, "Selected optimization candidate", None);
    // The iteration is an index, not a measurement: it printed as
    // "3.00000000000000000e0".
    let mut rows = vec![
        ("Iteration", iteration_text(iterations[index], digits), true),
        (
            "Cost",
            cost.map_or_else(
                || "unavailable".to_owned(),
                |value| value_text(value, "", digits),
            ),
            true,
        ),
        (
            "Cost from best",
            cost.map_or_else(
                || "unavailable".to_owned(),
                |value| signed_readout(value - *best_cost, digits),
            ),
            true,
        ),
    ];
    let mut variables: Vec<_> = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let name = waveform.name.strip_prefix("OPT_")?;
            (name != "COST").then(|| (name, waveform.y.sample(index)))
        })
        .collect();
    variables.sort_by(|left, right| left.0.cmp(right.0));
    rows.extend(variables.into_iter().map(|(name, value)| {
        let value = value.map_or_else(
            || "unavailable".to_owned(),
            |value| {
                best_variables.get(name).map_or_else(
                    || value_text(value, "", digits),
                    |best| {
                        format!(
                            "{} · Δ {}",
                            value_text(value, "", digits),
                            signed_readout(value - *best, digits)
                        )
                    },
                )
            },
        );
        (name, value, false)
    }));
    // The run converged or stopped once, not once per candidate; the sheet's
    // own header states it.
    rows.push((
        "Best candidate",
        if candidate_matches_best(analysis, index, *best_cost, best_variables) {
            "yes"
        } else {
            "no"
        }
        .to_owned(),
        false,
    ));
    stat_table(ui, &rows);
}

fn candidate_matches_best(
    analysis: &AnalysisResult<RetainedTrace>,
    index: usize,
    best_cost: f64,
    best_variables: &BTreeMap<String, f64>,
) -> bool {
    let cost_matches = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "OPT_COST")
        .and_then(|waveform| waveform.y.sample(index))
        .is_some_and(|cost| cost.to_bits() == best_cost.to_bits());
    cost_matches
        && best_variables.iter().all(|(name, best)| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name.strip_prefix("OPT_") == Some(name.as_str()))
                .and_then(|waveform| waveform.y.sample(index))
                .is_some_and(|value| value.to_bits() == best.to_bits())
        })
}

pub(crate) fn mono(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).font(theme::mono(tokens::FS_0, FontWeight::Regular)));
}

/// The outcome cell, in sentence case. A run that stopped short of
/// convergence is the one verdict here and keeps the warning tone; the best
/// candidate and a converged run are facts and read as plain text.
fn disposition(label: &str, converged: bool) -> Cell {
    let mut chars = label.chars();
    let text = chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect::<String>()
    });
    Cell::text(text).toned(if converged {
        Tone::Neutral
    } else {
        Tone::Caution
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_selection_snaps_to_exact_candidate() {
        let iterations = [0.0, 1.0, 3.0, 8.0];
        assert_eq!(nearest_candidate_index(&iterations, 2.4), Some(2));
        assert_eq!(nearest_candidate_index(&iterations, 7.9), Some(3));
    }

    #[test]
    fn convergence_selection_rejects_non_finite_history() {
        assert_eq!(nearest_candidate_index(&[f64::NAN], 1.0), None);
    }
}
