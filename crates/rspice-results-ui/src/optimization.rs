//! Optimization convergence plots and exact retained candidate presentation.

use crate::presentation::{PlotView, panel_note, stat_table};
use crate::strip::StripHeader;
use crate::waveform::WaveformData;
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};
use rspice_results::{analysis_result::AnalysisResult, optimization::history::OptimizationView};
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::section_header;
use std::collections::BTreeMap;

/// History and display inputs qualified for the host's active analysis.
pub struct OptimizationPlot<'a> {
    pub analysis_id: u64,
    pub label: &'a str,
    pub history: OptimizationView<'a>,
    pub selected: Option<usize>,
    pub plot_view: PlotView,
    pub cost_range: (f64, f64),
}

/// Navigation and candidate requests to apply to the same bound source.
pub struct OptimizationResponse {
    pub fit: bool,
    pub view: plot::ViewChange,
    pub selected: Option<usize>,
}

/// Exact metadata and index resolved by the host's candidate-selection gate.
pub struct CandidateView<'a> {
    pub analysis: &'a AnalysisResult<WaveformData>,
    pub index: usize,
    pub iterations: &'a [f64],
    pub best_cost: &'a f64,
    pub best_variables: &'a BTreeMap<String, f64>,
    pub converged: &'a bool,
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
    } = input;
    let mut requested = None;
    let outcome = if view.converged {
        "converged"
    } else {
        "stopped"
    };
    let best_index = view.best_index;

    let header = StripHeader::new(
        "OPTIMIZATION",
        &format!(
            "{} · {} iterations · {} · best cost {:.9e}",
            label,
            view.iterations.len(),
            outcome,
            view.best_cost
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
                    format!("{}. {} ({:?})", index + 1, term.measurement, term.goal),
                    format!(
                        "unit {}; value {:.9e}; target {}; scale {:.6e}; weight {:.6e}; cost {:.9e}",
                        if term.unit.is_empty() { "native" } else { &term.unit },
                        observation.value,
                        term.target
                            .map(|value| format!("{value:.9e}"))
                            .unwrap_or_else(|| "—".into()),
                        term.scale,
                        term.weight,
                        observation.contribution
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
        let rows = view.best_constraints.iter().enumerate().map(|(index, observation)| {
            let term = &observation.constraint;
            (format!("{}. {}", index + 1, term.measurement), format!("unit {}; value {:.9e}; limits {} to {}; tolerance {:.6e}; scale {:.6e}; violation {:.9e}", if term.unit.is_empty() { "native" } else { &term.unit }, observation.value,
                term.lower.map(|v| format!("{v:.9e}")).unwrap_or_else(|| "unbounded".into()),
                term.upper.map(|v| format!("{v:.9e}")).unwrap_or_else(|| "unbounded".into()), term.tolerance, term.scale, observation.violation))
        }).collect::<Vec<_>>();
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
    spec.traces.push(
        Trace::new(
            view.iterations,
            &view.cost.y,
            Tokens::get(ui.ctx()).color.traces[0],
        )
        .marker_style(0)
        .cache_key(0x4F50_5400_u64 ^ analysis_id.rotate_left(17)),
    );
    spec.markers.push(plot::Marker {
        x: view.iterations[best_index],
        y: view.best_cost,
        color: Tokens::get(ui.ctx()).color.ok,
        label: format!("best {:.6e}", view.best_cost),
        drop_line: true,
        label_dy: 0.0,
        shape: plot::MarkerShape::Point,
    });
    if let Some(selection) = selected
        && let Some((&iteration, &cost)) = view
            .iterations
            .get(selection)
            .zip(view.cost.y.get(selection))
        && selection != best_index
    {
        spec.markers.push(plot::Marker {
            x: iteration,
            y: cost,
            color: Tokens::get(ui.ctx()).color.accent,
            label: format!("selected {:.6e}", cost),
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
                    format!("{:.17e}", view.iterations[index]),
                ),
                ("cost".to_owned(), format!("{:.17e}", view.cost.y[index])),
                (
                    "Δ best".to_owned(),
                    format!("{:+.17e}", view.cost.y[index] - view.best_cost),
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

    // The table owns horizontal scrolling, so every retained design variable
    // remains addressable. Truncating the list here made the exact structured
    // alternative disagree with the immutable candidate vector.
    let visible_variables = view.variables;
    let width = ui
        .available_width()
        .max(530.0 + visible_variables.len() as f32 * 150.0);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.optimization-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(width);
            let mut table = TableBuilder::new(ui)
                .id_salt("rspice.results.optimization")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(105.0))
                .column(Column::initial(160.0))
                .column(Column::initial(160.0));
            for _ in &visible_variables {
                table = table.column(Column::initial(150.0));
            }
            table
                .column(Column::remainder().at_least(110.0))
                .header(31.0, |mut header| {
                    header.col(|ui| table_header(ui, "ITERATION"));
                    header.col(|ui| table_header(ui, "COST"));
                    header.col(|ui| table_header(ui, "Δ FROM BEST"));
                    for (name, _) in &visible_variables {
                        header.col(|ui| table_header(ui, name));
                    }
                    header.col(|ui| table_header(ui, "DISPOSITION"));
                })
                // Only the candidates the viewport can show. An optimizer run
                // is thousands of iterations long, and every row here formats
                // three quantities plus one per design variable.
                .body(|body| {
                    body.rows(29.0, view.iterations.len(), |mut row| {
                        let index = row.index();
                        let iteration = view.iterations[index];
                        let is_selected = selected == Some(index);
                        row.set_selected(is_selected);
                        row.col(|ui| {
                            if ui
                                .selectable_label(
                                    is_selected,
                                    RichText::new(format!("{iteration:.9e}")).monospace(),
                                )
                                .clicked()
                            {
                                requested = Some(index);
                            }
                        });
                        row.col(|ui| mono(ui, &format!("{:.17e}", view.cost.y[index])));
                        row.col(|ui| {
                            mono(
                                ui,
                                &format!("{:+.17e}", view.cost.y[index] - view.best_cost),
                            )
                        });
                        for (_, waveform) in &visible_variables {
                            row.col(|ui| mono(ui, &format!("{:.17e}", waveform.y[index])));
                        }
                        row.col(|ui| {
                            if best_index == index {
                                outcome_badge(
                                    ui,
                                    if index + 1 == view.iterations.len() && view.converged {
                                        "best · converged"
                                    } else {
                                        "best"
                                    },
                                    true,
                                );
                            } else if index + 1 == view.iterations.len() {
                                outcome_badge(ui, outcome, view.converged);
                            } else {
                                ui.label("evaluated");
                            }
                        });
                    });
                });
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
        converged,
    } = input;
    let cost = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "OPT_COST")
        .and_then(|waveform| waveform.y.get(index))
        .copied();

    section_header(ui, "Selected optimization candidate", Some("RETAINED"));
    let mut rows = vec![
        ("Iteration", format!("{:.17e}", iterations[index]), true),
        (
            "Cost",
            cost.map_or_else(|| "unavailable".to_owned(), |value| format!("{value:.17e}")),
            true,
        ),
        (
            "Cost from best",
            cost.map_or_else(
                || "unavailable".to_owned(),
                |value| format!("{:+.17e}", value - *best_cost),
            ),
            true,
        ),
    ];
    let mut variables: Vec<_> = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let name = waveform.name.strip_prefix("OPT_")?;
            (name != "COST").then(|| (name, waveform.y.get(index).copied()))
        })
        .collect();
    variables.sort_by(|left, right| left.0.cmp(right.0));
    rows.extend(variables.into_iter().map(|(name, value)| {
        let value = value.map_or_else(
            || "unavailable".to_owned(),
            |value| {
                best_variables.get(name).map_or_else(
                    || format!("{value:.17e}"),
                    |best| format!("{value:.17e} · Δ {:+.17e}", value - *best),
                )
            },
        );
        (name, value, false)
    }));
    rows.push((
        "Run outcome",
        if *converged { "converged" } else { "stopped" }.to_owned(),
        false,
    ));
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
    panel_note(
        ui,
        "Variable deltas compare this candidate with the exact retained optimum. Feasibility and specification pass/fail are not inferred unless the result explicitly retains those contracts.",
    );
}

fn candidate_matches_best(
    analysis: &AnalysisResult<WaveformData>,
    index: usize,
    best_cost: f64,
    best_variables: &BTreeMap<String, f64>,
) -> bool {
    let cost_matches = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "OPT_COST")
        .and_then(|waveform| waveform.y.get(index))
        .is_some_and(|cost| cost.to_bits() == best_cost.to_bits());
    cost_matches
        && best_variables.iter().all(|(name, best)| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name.strip_prefix("OPT_") == Some(name.as_str()))
                .and_then(|waveform| waveform.y.get(index))
                .is_some_and(|value| value.to_bits() == best.to_bits())
        })
}

fn table_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
}

fn mono(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).font(theme::mono(tokens::FS_0, FontWeight::Regular)));
}

fn outcome_badge(ui: &mut Ui, label: &str, converged: bool) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_ascii_uppercase())
            .strong()
            .color(if converged { t.color.ok } else { t.color.warn }),
    );
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
