//! Optimization convergence plots and exact retained candidate presentation.

use crate::presentation::{PlotView, stat_table, with_table_state};
use crate::strip::StripHeader;
use egui::{RichText, Ui};
use rspice_results::optimization::OptimizationObjectiveGoal;
use rspice_results::waveform::RetainedTrace;
use rspice_results::{analysis_result::AnalysisResult, optimization::history::OptimizationView};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_significant};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable, SortKey, Tone};
use std::collections::BTreeMap;

/// History and display inputs qualified for the host's active analysis.
pub struct OptimizationPlot<'a> {
    pub analysis_id: u64,
    pub label: &'a str,
    pub history: OptimizationView<'a>,
    pub selected: Option<usize>,
    pub plot_view: PlotView,
    pub cost_range: (f64, f64),
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
        cost_range: (cost_min, cost_max),
        significant_digits: digits,
    } = input;
    let mut requested = None;
    // The table and the convergence plot show every candidate at once: a stored history is
    // read whole off the frame, and is shown once it has arrived.
    let held = |values: &rspice_results::waveform::WaveformValues| {
        values.fetch_whole().ready().unwrap_or_default()
    };
    let cost = held(&view.cost.y);
    let cost_at = |index: usize| cost.get(index).copied().unwrap_or(f64::NAN);
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
    if !view.best_objectives.is_empty() {
        section_header(ui, "Objectives at best candidate", None);
        let values = view
            .best_objectives
            .iter()
            .enumerate()
            .map(|(index, observation)| {
                let term = &observation.objective;
                (
                    format!(
                        "{}. {} · {}",
                        index + 1,
                        term.measurement,
                        goal_word(term.goal)
                    ),
                    format!(
                        "value {}; target {}; scale {}; weight {}; cost {}",
                        value_text(observation.value, &term.unit, digits),
                        term.target
                            .map(|value| value_text(value, &term.unit, digits))
                            .unwrap_or_else(|| "—".into()),
                        value_text(term.scale, &term.unit, digits),
                        value_text(term.weight, "", digits),
                        value_text(observation.contribution, "", digits)
                    ),
                )
            })
            .collect::<Vec<_>>();
        let rows = values
            .iter()
            .map(|(name, value)| (name.as_str(), value.clone(), false))
            .collect::<Vec<_>>();
        stat_table(ui, &rows);
    }
    if !view.best_constraints.is_empty() {
        section_header(
            ui,
            if view.best_constraints.iter().all(|row| row.violation == 0.0) {
                "Best candidate satisfies all constraints"
            } else {
                "Best candidate is infeasible"
            },
            None,
        );
        let rows = view
            .best_constraints
            .iter()
            .enumerate()
            .map(|(index, observation)| {
                let term = &observation.constraint;
                let bound = |value: Option<f64>| {
                    value.map_or_else(
                        || "unbounded".to_owned(),
                        |value| value_text(value, &term.unit, digits),
                    )
                };
                (
                    format!("{}. {}", index + 1, term.measurement),
                    format!(
                        "value {}; limits {} to {}; tolerance {}; scale {}; violation {}",
                        value_text(observation.value, &term.unit, digits),
                        bound(term.lower),
                        bound(term.upper),
                        value_text(term.tolerance, &term.unit, digits),
                        value_text(term.scale, &term.unit, digits),
                        value_text(observation.violation, "", digits)
                    ),
                )
            })
            .collect::<Vec<_>>();
        stat_table(
            ui,
            &rows
                .iter()
                .map(|(name, value)| (name.as_str(), value.clone(), false))
                .collect::<Vec<_>>(),
        );
    }
    let auto_x0 = view.iterations[0];
    let auto_x1 = *view.iterations.last().unwrap_or(&auto_x0);
    let x_pad = if auto_x0 < auto_x1 {
        (auto_x1 - auto_x0) * 0.025
    } else {
        auto_x0.abs().mul_add(0.025, 1.0)
    };
    let y_pad = if cost_min < cost_max {
        (cost_max - cost_min) * 0.10
    } else {
        cost_min.abs().mul_add(0.10, 1.0)
    };
    let (x0, x1) = plot_view.x.unwrap_or((auto_x0 - x_pad, auto_x1 + x_pad));
    let (y0, y1) = plot_view.y.unwrap_or((cost_min - y_pad, cost_max + y_pad));
    let mut spec = PlotSpec::new(
        Axis::linear_with(x0, x1, "", 7).with_label("iteration"),
        XScale::Linear,
        Axis::linear_with(y0, y1, "", 7).with_label("cost"),
    )
    .accessible_name("Optimization convergence")
    .accessible_detail("Exact retained objective cost at each evaluated candidate.");
    if cost.len() == view.iterations.len() {
        spec.traces.push(
            Trace::new(
                view.iterations,
                &cost,
                Tokens::get(ui.ctx()).color.traces[0],
            )
            .marker_style(0)
            .cache_key(0x4F50_5400_u64 ^ analysis_id.rotate_left(17)),
        );
    }
    spec.markers.push(plot::Marker {
        x: view.iterations[best_index],
        y: view.best_cost,
        // The best candidate is a fact, not a success: primary ink.
        color: Tokens::get(ui.ctx()).color.text,
        label: format!("best {}", value_text(view.best_cost, "", digits)),
        drop_line: true,
        label_dy: 0.0,
        shape: plot::MarkerShape::Point,
    });
    if let Some(selection) = selected
        && let Some((&iteration, &cost)) = view.iterations.get(selection).zip(cost.get(selection))
        && selection != best_index
    {
        spec.markers.push(plot::Marker {
            x: iteration,
            y: cost,
            color: Tokens::get(ui.ctx()).color.accent,
            label: format!("selected {}", value_text(cost, "", digits)),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    let readout = |iteration: f64| {
        nearest_candidate_index(view.iterations, iteration).map_or_else(Vec::new, |index| {
            vec![
                ("candidate".to_owned(), index.to_string()),
                (
                    "iteration".to_owned(),
                    iteration_text(view.iterations[index], digits),
                ),
                ("cost".to_owned(), value_text(cost_at(index), "", digits)),
                (
                    "Δ best".to_owned(),
                    signed_readout(cost_at(index) - view.best_cost, digits),
                ),
            ]
        })
    };
    let available_height = ui.available_height();
    let plot_height = (available_height * 0.34)
        .clamp(140.0, 230.0)
        .min((available_height - 120.0).max(100.0));
    let response = ui
        .allocate_ui_with_layout(
            egui::vec2(ui.available_width(), plot_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_min_height(plot_height);
                plot::show(ui, &spec, cache, None, Some(&readout))
            },
        )
        .inner;
    if let Some(index) = response
        .clicked_x
        .and_then(|iteration| nearest_candidate_index(view.iterations, iteration))
    {
        requested = Some(index);
    }

    // Every retained design variable is a column: the table scrolls
    // sideways under the frozen iteration, so the structured alternative
    // agrees with the immutable candidate vector.
    let visible_variables = view
        .variables
        .iter()
        .map(|(name, waveform)| (*name, held(&waveform.y)))
        .collect::<Vec<_>>();
    let mut columns = vec![
        Column::number("Iteration", 105.0),
        Column::number("Cost", 160.0),
        Column::number("\u{0394} from best", 160.0),
    ];
    columns.extend(
        visible_variables
            .iter()
            .map(|(name, _)| Column::number(*name, 150.0)),
    );
    columns.push(Column::name("Disposition", 110.0));
    let candidates = view.iterations.len();
    let revision = egui::Id::new(("optimization", analysis_id, candidates, digits)).value();
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
            .max_height(ui.available_height())
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
                cells.extend(visible_variables.iter().map(|(_, values)| {
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
            requested = state.selected_row().or(requested);
        }
    });
    OptimizationResponse {
        fit: header.fit_clicked,
        view: response.view,
        selected: requested,
    }
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
