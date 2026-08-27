//! Optimization convergence and candidate history viewer.

use std::collections::BTreeMap;

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, SimulationState, WaveformData,
};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::frame_work::{self, DatasetWalk};
use super::strip::StripHeader;
use super::{AnalysisPresentationKey, OptimizationSelection, panel_note, stat_table, well_hint};

struct OptimizationView<'a> {
    analysis: &'a AnalysisResult,
    iterations: &'a [f64],
    cost: &'a WaveformData,
    variables: Vec<(&'a str, &'a WaveformData)>,
    best_cost: f64,
    best_index: usize,
    converged: bool,
}

fn active_optimization(simulation: &SimulationState) -> Option<OptimizationView<'_>> {
    optimization_for_analysis(simulation.active_analysis()?)
}

fn optimization_for_analysis(analysis: &AnalysisResult) -> Option<OptimizationView<'_>> {
    frame_work::note(DatasetWalk::OptimizationView);
    let Some(AnalysisResultFamilyMetadata::Optimization {
        iterations,
        best_cost,
        best_variables,
        converged,
    }) = analysis.family_metadata.as_ref()
    else {
        return None;
    };
    if !analysis.success
        || analysis.analysis_type != AnalysisType::Optimization
        || iterations.is_empty()
        || {
            frame_work::note(DatasetWalk::EvidenceValidation);
            analysis.validate_retained_evidence().is_err()
        }
    {
        return None;
    }
    let cost = analysis.waveforms.iter().find(|waveform| {
        waveform.name == "OPT_COST"
            && waveform.x.as_slice() == iterations
            && waveform.y.len() == iterations.len()
            && waveform.y.iter().all(|value| value.is_finite())
    })?;
    let mut variables: Vec<_> = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let name = waveform.name.strip_prefix("OPT_")?;
            (name != "COST"
                && waveform.x.as_slice() == iterations
                && waveform.y.len() == iterations.len()
                && waveform.y.iter().all(|value| value.is_finite()))
            .then_some((name, waveform))
        })
        .collect();
    variables.sort_by(|left, right| left.0.cmp(right.0));
    if variables.is_empty()
        || variables.len() != best_variables.len()
        || variables.windows(2).any(|pair| pair[0].0 == pair[1].0)
        || best_variables.keys().any(|name| {
            !variables
                .iter()
                .any(|(candidate, _)| *candidate == name.as_str())
        })
    {
        return None;
    }
    let best_index = (0..iterations.len()).find(|&index| {
        cost.y[index].to_bits() == best_cost.to_bits()
            && variables.iter().all(|(name, waveform)| {
                best_variables
                    .get(*name)
                    .is_some_and(|best| waveform.y[index].to_bits() == best.to_bits())
            })
    })?;
    Some(OptimizationView {
        analysis,
        iterations,
        cost,
        variables,
        best_cost: *best_cost,
        best_index,
        converged: *converged,
    })
}

/// Serialize the validated candidate history that the optimization sheet
/// draws, including the terminal optimum metadata.
pub(crate) fn export_csv(analysis: &AnalysisResult) -> Option<super::ResultSheetCsv> {
    let view = optimization_for_analysis(analysis)?;
    let mut contents = String::from("field,value\n");
    contents.push_str(&format!("converged,{}\n", view.converged));
    contents.push_str(&format!("best_cost,{:.17e}\n", view.best_cost));
    contents.push_str(&format!("best_index,{}\n", view.best_index));
    contents.push_str(&format!(
        "best_iteration,{:.17e}\n\niteration,cost",
        view.iterations[view.best_index]
    ));
    for (name, _) in &view.variables {
        contents.push(',');
        contents.push_str(&super::csv_field(name));
    }
    contents.push('\n');
    for index in 0..view.iterations.len() {
        contents.push_str(&format!(
            "{:.17e},{:.17e}",
            view.iterations[index], view.cost.y[index]
        ));
        for (_, waveform) in &view.variables {
            contents.push_str(&format!(",{:.17e}", waveform.y[index]));
        }
        contents.push('\n');
    }
    Some(super::ResultSheetCsv {
        default_name: "rspice-optimization.csv",
        detail: format!("{} optimization iterations", view.iterations.len()),
        contents,
    })
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

pub(super) fn active_metadata_is_valid(state: &AppState) -> bool {
    active_optimization(&state.simulation).is_some()
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(run) = state.simulation.active_run() else {
        well_hint(ui, "Select a dataset with retained optimization evidence");
        return;
    };
    let Some(view) = active_optimization(&state.simulation) else {
        well_hint(
            ui,
            "Select a validated optimization analysis with a retained cost history",
        );
        return;
    };
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, view.analysis);
    let selected = state.ui.results.selected_optimization;
    let mut requested = None;
    let outcome = if view.converged {
        "converged"
    } else {
        "stopped"
    };
    let best_index = view.best_index;

    let plot_view = state
        .ui
        .results
        .plot_view(super::ResultViewer::Optimization, 0);
    let header = StripHeader::new(
        "OPTIMIZATION",
        &format!(
            "{} · {} iterations · {} · best cost {:.9e}",
            view.analysis.label,
            view.iterations.len(),
            outcome,
            view.best_cost
        ),
        &[],
    )
    .zoomed(plot_view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Optimization, 0);
    }

    let auto_x0 = view.iterations[0];
    let auto_x1 = *view.iterations.last().unwrap_or(&auto_x0);
    let x_pad = if auto_x0 < auto_x1 {
        (auto_x1 - auto_x0) * 0.025
    } else {
        auto_x0.abs().mul_add(0.025, 1.0)
    };
    let (cost_min, cost_max) = super::finite_extremes(&view.cost.y).unwrap_or((0.0, 1.0));
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
        .cache_key(0x4F50_5400_u64 ^ view.analysis.id.rotate_left(17)),
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
    if let Some(selection) = selected.filter(|selection| selection.analysis == analysis_key)
        && let Some((&iteration, &cost)) = view
            .iterations
            .get(selection.iteration_index)
            .zip(view.cost.y.get(selection.iteration_index))
        && selection.iteration_index != best_index
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
                plot::show(ui, &spec, &mut state.ui.results.cache, None, Some(&readout))
            },
        )
        .inner;
    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Optimization, 0)
            .apply(&response.view);
    }
    if let Some(index) = response
        .clicked_x
        .and_then(|iteration| nearest_candidate_index(view.iterations, iteration))
    {
        requested = Some(OptimizationSelection {
            analysis: analysis_key,
            iteration_index: index,
        });
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
                .body(|mut body| {
                    for (index, iteration) in view.iterations.iter().copied().enumerate() {
                        let is_selected = selected.is_some_and(|selection| {
                            selection.analysis == analysis_key && selection.iteration_index == index
                        });
                        body.row(29.0, |mut row| {
                            row.set_selected(is_selected);
                            row.col(|ui| {
                                if ui
                                    .selectable_label(
                                        is_selected,
                                        RichText::new(format!("{iteration:.9e}")).monospace(),
                                    )
                                    .clicked()
                                {
                                    requested = Some(OptimizationSelection {
                                        analysis: analysis_key,
                                        iteration_index: index,
                                    });
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
                    }
                });
        });
    if let Some(selection) = requested {
        state.ui.results.selected_optimization = Some(selection);
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_optimization else {
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "Select an iteration row to inspect its exact retained cost and variables.",
        );
        return;
    };
    let Some(run) = state.simulation.active_run() else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(ui, "Select a retained optimization analysis and candidate.");
        return;
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The selected candidate no longer belongs to the active retained dataset.",
        );
        return;
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "Select an optimization candidate in the active analysis.",
        );
        return;
    }
    let Some(AnalysisResultFamilyMetadata::Optimization {
        iterations,
        best_cost,
        best_variables,
        converged,
    }) = analysis.family_metadata.as_ref()
    else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The active analysis no longer contains retained optimization history.",
        );
        return;
    };
    let index = selection.iteration_index;
    if index >= iterations.len() {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The selected candidate no longer exists in the retained optimization history.",
        );
        return;
    }
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
    analysis: &AnalysisResult,
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
