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
    /// Each design variable's unit, by its name in the history: `F`, `Ω`,
    /// and none for a variable without one.
    pub variable_units: &'a BTreeMap<String, &'a str>,
    /// Each goal's least and greatest measured value, in the history's
    /// order.
    pub goal_ranges: &'a [(f64, f64)],
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
    /// The cost strip's axes as drawn, iteration then cost: the ranges the
    /// details panel states for the sheet.
    pub axes: Option<((f64, f64), (f64, f64))>,
}

/// Exact metadata and index resolved by the host's candidate-selection gate.
pub struct CandidateView<'a> {
    pub analysis: &'a AnalysisResult<RetainedTrace>,
    pub index: usize,
    pub iterations: &'a [f64],
    pub best_cost: &'a f64,
    pub best_variables: &'a BTreeMap<String, f64>,
    /// Each design variable's unit, by its name in the history.
    pub variable_units: &'a BTreeMap<String, &'a str>,
    /// The reader's displayed significant digits.
    pub significant_digits: usize,
}

/// A value at the reader's digits, in its own unit: an SI prefix where the
/// unit takes one, and none on a decibel or a percentage, which `500 mdB`
/// would misstate. Full precision belongs to copy and export.
fn value_text(value: f64, unit: &str, digits: usize) -> String {
    fmt_si_significant(value, unit, digits)
}

/// `name`'s unit, or none when the host knows of none.
fn unit_of<'u>(units: &BTreeMap<String, &'u str>, name: &str) -> &'u str {
    units.get(name).copied().unwrap_or("")
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
fn signed_readout(delta: f64, unit: &str, digits: usize) -> String {
    let sign = if delta < 0.0 { "−" } else { "+" };
    format!("{sign}{}", value_text(delta.abs(), unit, digits))
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
        variable_units,
        goal_ranges,
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

    // The cost, every design variable and every goal, one strip each over
    // the shared iteration axis, so a step in a variable reads against the
    // cost it bought and the measurement it moved.
    let variables = view
        .variables
        .iter()
        .map(|(name, waveform)| (*name, held(&waveform.y)))
        .collect::<Vec<_>>();
    let goals = view
        .goals
        .iter()
        .map(|(name, waveform)| {
            (
                *name,
                held(&waveform.y),
                waveform.unit.as_deref().unwrap_or(""),
            )
        })
        .collect::<Vec<_>>();
    let unfitted = || std::iter::repeat((0.0, 1.0));
    let histories = std::iter::once(("cost", &cost[..], cost_range, "", Role::Cost))
        .chain(
            variables
                .iter()
                .zip(variable_ranges.iter().copied().chain(unfitted()))
                .map(|((name, values), range)| {
                    (
                        *name,
                        &values[..],
                        range,
                        unit_of(variable_units, name),
                        Role::Variable,
                    )
                }),
        )
        .chain(
            goals
                .iter()
                .zip(goal_ranges.iter().copied().chain(unfitted()))
                .map(|((name, values, unit), range)| {
                    (*name, &values[..], range, *unit, Role::Goal)
                }),
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
    // The strips share one iteration axis, written once under the last of
    // them, as a Waves stack writes time once under its strips.
    let count = histories.len();
    let strip_height = (plot_area.height() - plot::X_AXIS_ROW_HEIGHT).max(0.0) / count as f32;
    let tokens = Tokens::get(ui.ctx());
    let strips = histories
        .iter()
        .enumerate()
        .map(|(index, &(name, values, range, unit, role))| HistoryStrip {
            name,
            unit,
            role,
            iterations: view.iterations,
            values,
            x: (x0, x1),
            y: if index == 0 {
                plot_view.y.unwrap_or_else(|| plot::padded_fit(range))
            } else {
                plot::padded_fit(range)
            },
            best_index,
            selected,
            color: tokens.color.traces[index % tokens.color.traces.len()],
            cache_key: 0x4F50_5400_u64 ^ analysis_id.rotate_left(17) ^ index as u64,
            digits,
        })
        .collect::<Vec<_>>();
    // One gutter for every strip, the widest any of them needs, so their
    // iteration grids stand one above another.
    let gutter = strips
        .iter()
        .map(|strip| strip.gutter(ui))
        .fold(plot::PLOT_GUTTER, f32::max);
    let mut view_change = plot::ViewChange::default();
    let mut axes = None;
    for (index, strip) in strips.iter().enumerate() {
        let last = index + 1 == count;
        let top = strip_height.mul_add(index as f32, plot_area.top());
        let rect = egui::Rect::from_min_max(
            egui::pos2(plot_area.left(), top),
            egui::pos2(
                plot_area.right(),
                if last {
                    plot_area.bottom()
                } else {
                    top + strip_height
                },
            ),
        );
        let mut strip_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(("rspice.results.optimization-history", index))
                .max_rect(rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        if let Some(role) = strip.role.words() {
            // The cost strip stands under the sheet's own header; each other
            // strip names its variable or goal and its value at the best
            // candidate.
            let best = strip.values.get(best_index).copied().unwrap_or(f64::NAN);
            StripHeader::new(
                strip.name,
                &format!("{role} · best {}", value_text(best, strip.unit, digits)),
                &[],
            )
            .maximizable(false)
            .show(&mut strip_ui);
        }
        let response = history_strip(&mut strip_ui, strip, gutter, last, cache);
        if index == 0 {
            axes = Some(response.axes);
        }
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
                y: if index == 0 { response.view.y } else { None },
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
                    units: variable_units,
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
        axes,
    }
}

/// What a history is a history of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Cost,
    Variable,
    Goal,
}

impl Role {
    /// What a strip's header calls it; the cost strip has the sheet's.
    const fn words(self) -> Option<&'static str> {
        match self {
            Self::Cost => None,
            Self::Variable => Some("design variable"),
            Self::Goal => Some("goal"),
        }
    }
}

/// One history's strip: its values at each iteration, the best candidate
/// marked, and the selected one when it is another.
struct HistoryStrip<'a> {
    name: &'a str,
    /// The history's unit, none for the cost or a bare variable.
    unit: &'a str,
    role: Role,
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

impl HistoryStrip<'_> {
    fn accessible_name(&self) -> String {
        format!("Optimization history of {}", self.name)
    }

    /// The left gutter this strip's value labels need.
    fn gutter(&self, ui: &Ui) -> f32 {
        let name = self.accessible_name();
        let spec = history_spec(ui, self, &name);
        plot::plot_rect(ui, &spec).left() - ui.available_rect_before_wrap().left()
    }

    fn value_at(&self, index: usize) -> f64 {
        self.values.get(index).copied().unwrap_or(f64::NAN)
    }
}

/// The strip's plot: its history as a line through a dot at each candidate,
/// the best candidate marked, and the selected one when it is another.
fn history_spec<'s>(ui: &Ui, strip: &HistoryStrip<'s>, accessible_name: &'s str) -> PlotSpec<'s> {
    let digits = strip.digits;
    let mut spec = PlotSpec::new(
        Axis::linear_with(strip.x.0, strip.x.1, "", 7).with_label("iteration"),
        XScale::Linear,
        Axis::linear_with(strip.y.0, strip.y.1, strip.unit, 5).with_label(strip.name),
    )
    .accessible_name(accessible_name)
    .accessible_detail("Exact value at each evaluated candidate.");
    if strip.values.len() == strip.iterations.len() {
        spec.traces.push(
            Trace::new(strip.iterations, strip.values, strip.color).cache_key(strip.cache_key),
        );
        // Each candidate is a dot where it was evaluated.
        spec.traces.push(
            Trace::new(strip.iterations, strip.values, strip.color)
                .draw(plot::DrawMode::Points)
                .width(4.0)
                .cache_key(strip.cache_key.rotate_left(1)),
        );
    }
    if let Some(&best) = strip.iterations.get(strip.best_index) {
        spec.markers.push(plot::Marker {
            x: best,
            y: strip.value_at(strip.best_index),
            // The best candidate is a fact, not a success: primary ink.
            color: Tokens::get(ui.ctx()).color.text,
            label: format!(
                "best {}",
                value_text(strip.value_at(strip.best_index), strip.unit, digits)
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
            y: strip.value_at(selection),
            color: Tokens::get(ui.ctx()).color.accent,
            label: format!(
                "selected {}",
                value_text(strip.value_at(selection), strip.unit, digits)
            ),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    spec
}

/// Draw `strip` on `gutter`, with the iteration row under it when `x_row`.
fn history_strip(
    ui: &mut Ui,
    strip: &HistoryStrip<'_>,
    gutter: f32,
    x_row: bool,
    cache: &mut plot::DecimationCache,
) -> plot::PlotResponse {
    let digits = strip.digits;
    let accessible_name = strip.accessible_name();
    let mut spec = history_spec(ui, strip, &accessible_name);
    spec.left_margin = gutter;
    if !x_row {
        spec = spec.without_x_axis_chrome();
    }
    let best_value = strip.value_at(strip.best_index);
    let readout = |iteration: f64| {
        nearest_candidate_index(strip.iterations, iteration).map_or_else(Vec::new, |index| {
            vec![
                (
                    "iteration".to_owned(),
                    iteration_text(strip.iterations[index], digits),
                ),
                (
                    strip.name.to_owned(),
                    value_text(strip.value_at(index), strip.unit, digits),
                ),
                (
                    "Δ best".to_owned(),
                    signed_readout(strip.value_at(index) - best_value, strip.unit, digits),
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
    /// Each variable's unit, by name.
    units: &'a BTreeMap<String, &'a str>,
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
        units,
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
                    Cell::number(signed_readout(cost - view.best_cost, "", digits))
                        .sort_by(SortKey::Number(cost - view.best_cost)),
                ];
                cells.extend(variables.iter().map(|(name, values)| {
                    let value = values.get(index).copied().unwrap_or(f64::NAN);
                    Cell::number(value_text(value, unit_of(units, name), digits))
                        .sort_by(SortKey::Number(value))
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
        variable_units,
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
                |value| signed_readout(value - *best_cost, "", digits),
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
        let unit = unit_of(variable_units, name);
        let value = value.map_or_else(
            || "unavailable".to_owned(),
            |value| {
                best_variables.get(name).map_or_else(
                    || value_text(value, unit, digits),
                    |best| {
                        format!(
                            "{} · Δ {}",
                            value_text(value, unit, digits),
                            signed_readout(value - *best, unit, digits)
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
