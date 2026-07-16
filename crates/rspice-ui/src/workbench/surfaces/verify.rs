//! Verification evidence, specifications, checks, reliability, and history.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::SpecEntry;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot, workspace_title_row};
use super::super::state::VerificationPage;

const VERIFY_RESPONSIVE_BREAKPOINT: f32 = 820.0;
const VERIFY_KPI_BREAKPOINT: f32 = 1_260.0;
const VERIFY_PHONE_BREAKPOINT: f32 = 560.0;
const VERIFY_STACKED_CHART_HEIGHT: f32 = 250.0;
const VERIFY_PHONE_CHART_HEIGHT: f32 = 230.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx().clone();
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        let surface_width = ui.available_width();
        ScrollArea::vertical()
            .id_salt("workbench.verify.surface")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| {
                    verification_heading(ui, app);
                });
                match app.state.workbench.verification_page {
                    VerificationPage::Yield => cockpit(ui, app),
                    VerificationPage::Corners => corners(ui, app),
                    VerificationPage::Tuning => tuning_unavailable(ui),
                    VerificationPage::Optimization => optimization(ui, app),
                    VerificationPage::Reliability => reliability(ui, app),
                    VerificationPage::Regression => regression(ui, app),
                    VerificationPage::Drc => physical_drc(ui, app),
                }
            });
    });
    regression_baseline_picker(&ctx, app);
}

fn verification_heading(ui: &mut Ui, app: &mut RSpiceApp) {
    let run = verification_run(app);
    let (eyebrow, title, description) = match app.state.workbench.verification_page {
        VerificationPage::Yield => (
            run.map_or_else(
                || "PVT & MONTE CARLO · NO RETAINED DATASET".to_owned(),
                |run| {
                    format!(
                        "PVT & MONTE CARLO · RUN {} · DATASET {}",
                        run.id, run.dataset_id
                    )
                },
            ),
            "PVT & Monte Carlo verification",
            "Yield, retained-run specification margins, and traceable measurement evidence from the active project.",
        ),
        VerificationPage::Corners => (
            run.map_or_else(
                || "PROCESS CORNERS · NO RETAINED DATASET".to_owned(),
                |run| {
                    format!(
                        "PROCESS CORNERS · RUN {} · DATASET {}",
                        run.id, run.dataset_id
                    )
                },
            ),
            "Process-corner verification",
            "Retained corner sweep values and source-attributed execution state from the active immutable dataset.",
        ),
        VerificationPage::Tuning => (
            "PARAMETER TUNER · CAPABILITY UNAVAILABLE".to_owned(),
            "Parameter tuning unavailable",
            "Tuning remains inaccessible until variables are discovered from the active design and every change is transactionally netlisted, simulated, and retained.",
        ),
        VerificationPage::Optimization => (
            run.map_or_else(
                || "OPTIMIZATION · NO RETAINED DATASET".to_owned(),
                |run| format!("OPTIMIZATION · RUN {} · DATASET {}", run.id, run.dataset_id),
            ),
            "Optimization candidate",
            "Bounded design variables and convergence evidence produced by the production optimization engine.",
        ),
        VerificationPage::Reliability => (
            run.map_or_else(
                || "RELIABILITY · NO RETAINED DATASET".to_owned(),
                |run| format!("RELIABILITY · RUN {} · DATASET {}", run.id, run.dataset_id),
            ),
            "Reliability and safe-operating-area verification",
            "Electrical SOA and aging evidence from executed analyses; geometry remains owned by Physical DRC.",
        ),
        VerificationPage::Regression => (
            "REGRESSION · GOVERNED RETAINED BASELINE".to_owned(),
            "Golden regression comparison",
            "Candidate and immutable baseline measurements are aligned without inventing tolerance or pass criteria.",
        ),
        VerificationPage::Drc => (
            "PHYSICAL VERIFICATION · NO RETAINED LAYOUT EVIDENCE".to_owned(),
            "Design-rule checking",
            "Physical sign-off remains fail-closed until a layout source, rule deck, and immutable marker database are retained.",
        ),
    };
    let page = app.state.workbench.verification_page;
    let available = ui.available_width().max(1.0);
    let action_width = match page {
        VerificationPage::Corners | VerificationPage::Regression => 238.0,
        _ => 0.0,
    };
    let viewport_width = ui.ctx().content_rect().width();
    if action_width > 0.0 && viewport_width > VERIFY_PHONE_BREAKPOINT && available > 560.0 {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.allocate_ui_with_layout(
                egui::vec2((available - action_width - 8.0).max(1.0), 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| heading(ui, &eyebrow, title, description),
            );
            ui.allocate_ui_with_layout(
                egui::vec2(action_width, 0.0),
                egui::Layout::right_to_left(egui::Align::Min),
                |ui| verification_header_actions(ui, app, page),
            );
        });
    } else {
        heading(ui, &eyebrow, title, description);
        if action_width > 0.0 {
            ui.add_space(7.0);
            ui.horizontal_wrapped(|ui| verification_header_actions(ui, app, page));
        }
    }
}

fn verification_header_actions(ui: &mut Ui, app: &mut RSpiceApp, page: VerificationPage) {
    ui.spacing_mut().item_spacing.x = 6.0;
    match page {
        VerificationPage::Corners => {
            let has_corner_result = latest_analysis(app, crate::state::AnalysisType::Corner)
                .is_some_and(|result| !result.waveforms.is_empty());
            let export = Button::new("Export matrix").enabled(has_corner_result);
            let export = if has_corner_result {
                export.accent()
            } else {
                export
            };
            if export
                .show(ui)
                .on_disabled_hover_text(
                    "Run and retain a process-corner analysis before exporting.",
                )
                .clicked()
            {
                export_active_corner_matrix(app);
            }
            let nominal_available = corner_nominal_index(app).is_some();
            if Button::new("Compare nominal")
                .enabled(nominal_available)
                .show(ui)
                .on_disabled_hover_text(
                    "A revision-matched TT, nominal-voltage, room-temperature point is required.",
                )
                .clicked()
            {
                app.state.workbench.verification.corner_compare_nominal =
                    !app.state.workbench.verification.corner_compare_nominal;
                app.state.workbench.verification.action_receipt =
                    if app.state.workbench.verification.corner_compare_nominal {
                        "Corner matrix now reports exact deltas from the retained nominal point."
                            .to_owned()
                    } else {
                        "Corner matrix restored to absolute retained values.".to_owned()
                    };
            }
        }
        VerificationPage::Regression => {
            let can_run = regression_run_pair(app).is_some();
            let run = Button::new("Run regression").enabled(can_run);
            let run = if can_run { run.accent() } else { run };
            if run
                .show(ui)
                .on_disabled_hover_text(
                    "Retain two distinct immutable result datasets before comparing them.",
                )
                .clicked()
            {
                run_regression_comparison(app);
            }
            if Button::new("Select baseline").show(ui).clicked() {
                app.state
                    .workbench
                    .verification
                    .regression_baseline_picker_selection = active_regression_baseline(app);
                app.state
                    .workbench
                    .verification
                    .regression_baseline_picker_open = true;
            }
        }
        VerificationPage::Yield
        | VerificationPage::Tuning
        | VerificationPage::Optimization
        | VerificationPage::Reliability
        | VerificationPage::Drc => {}
    }
}

fn regression_baseline_picker(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .verification
        .regression_baseline_picker_open
    {
        return;
    }

    let active_run = app.state.simulation.active_run().map(|run| run.run_id);
    let mut candidates = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| Some(run.run_id) != active_run && run.analyses.iter().any(verified_analysis))
        .map(|run| {
            (
                run.run_id,
                run.id,
                run.label.clone(),
                run.dataset_id.to_string(),
                run.prepared_receipt().is_some(),
            )
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| right.1.cmp(&left.1));

    let mut selected = app
        .state
        .workbench
        .verification
        .regression_baseline_picker_selection;
    if selected.is_none_or(|selected| !candidates.iter().any(|row| row.0 == selected)) {
        selected = candidates.first().map(|row| row.0);
    }

    let mut open = true;
    let mut cancel = false;
    let mut commit = false;
    let viewport = ctx.content_rect().size();
    let width = (viewport.x - 16.0).clamp(1.0, 620.0);
    let candidate_height = (viewport.y - 210.0).clamp(90.0, 420.0);
    egui::Window::new("Select regression baseline")
        .id(egui::Id::new("workbench.verify.regression_baseline_picker"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(width)
        .min_width(width)
        .max_width(width)
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("GOVERNED RETAINED BASELINE")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.accent),
            );
            ui.label(
                egui::RichText::new(
                    "Choose the immutable dataset used for the next exact comparison.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
            );
            ui.add_space(6.0);

            if candidates.is_empty() {
                status_dot(
                    ui,
                    t.color.warn,
                    "No distinct source-attributed retained run is available",
                );
                ui.label("Retain another completed run before selecting a regression baseline.");
            } else {
                let list_width = ui.available_width();
                ScrollArea::vertical()
                    .id_salt("workbench.verify.regression_baseline_candidates")
                    .max_height(candidate_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .fill(t.color.bg_panel)
                            .stroke(egui::Stroke::new(1.0, t.color.border))
                            .show(ui, |ui| {
                                ui.set_width((list_width - 2.0).max(1.0));
                                for (run_id, _, label, dataset, prepared) in &candidates {
                                    let is_selected = selected == Some(*run_id);
                                    let response = ui.selectable_label(
                                        is_selected,
                                        egui::RichText::new(label)
                                            .font(theme::sans(tokens::FS_1, FontWeight::Medium)),
                                    );
                                    if response.clicked() {
                                        selected = Some(*run_id);
                                    }
                                    ui.horizontal_wrapped(|ui| {
                                        ui.add_space(10.0);
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "Dataset {dataset} · {}",
                                                if *prepared {
                                                    "prepared snapshot retained"
                                                } else {
                                                    "legacy retained provenance"
                                                }
                                            ))
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                            .color(t.color.text_dim),
                                        );
                                    });
                                }
                            });
                    });
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let can_commit = selected.is_some();
                let use_baseline = Button::new("Use baseline").enabled(can_commit);
                let use_baseline = if can_commit {
                    use_baseline.accent()
                } else {
                    use_baseline
                };
                if use_baseline.show(ui).clicked() {
                    commit = true;
                }
                if Button::new("Cancel").show(ui).clicked() {
                    cancel = true;
                }
            });
        });

    if commit {
        if let Some(run_id) = selected {
            if let Err(error) = commit_regression_baseline(app, run_id) {
                app.state.workbench.verification.action_receipt =
                    format!("Regression baseline was not changed: {error}");
                app.state
                    .workbench
                    .verification
                    .regression_baseline_picker_selection = selected;
                return;
            }
            let run_number = candidates
                .iter()
                .find(|row| row.0 == run_id)
                .map(|row| row.1)
                .unwrap_or_default();
            app.state.workbench.verification.action_receipt = format!(
                "Run {run_number} is the selected immutable regression baseline. Run regression to create a comparison receipt."
            );
        }
        open = false;
    }
    if cancel {
        open = false;
    }
    app.state
        .workbench
        .verification
        .regression_baseline_picker_selection = selected;
    app.state
        .workbench
        .verification
        .regression_baseline_picker_open = open;
}

fn cockpit(ui: &mut Ui, app: &mut RSpiceApp) {
    let run_index = verification_run_index(app);
    let evidence = specification_evidence(app, run_index);
    engineering_status_strip(ui, app, &evidence);

    let width = ui.available_width();
    let viewport_width = ui.ctx().content_rect().width();
    let t = Tokens::get(ui.ctx());
    let header_height = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 37.0 };
    let visible_remaining = (ui.clip_rect().bottom() - ui.cursor().top()).max(0.0);
    let layout = VerifyLayout::resolve(
        viewport_width,
        width,
        visible_remaining,
        evidence.len(),
        header_height,
        verification_table_row_height(viewport_width),
    );
    if layout.split {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui(
                egui::vec2(layout.left_width, layout.first_row_height),
                |ui| yield_chart(ui, app, layout.first_row_height),
            );
            ui.allocate_ui(
                egui::vec2(layout.right_width, layout.first_row_height),
                |ui| run_margin_matrix(ui, app, run_index, &evidence),
            );
        });
        let divider_x = ui.min_rect().left() + layout.left_width;
        let row_bottom = ui.cursor().top();
        ui.painter().vline(
            divider_x,
            egui::Rangef::new(row_bottom - layout.first_row_height, row_bottom),
            egui::Stroke::new(1.0, t.color.border),
        );
    } else {
        let chart_height = verification_stacked_chart_height(viewport_width);
        ui.allocate_ui(egui::vec2(width, chart_height), |ui| {
            yield_chart(ui, app, chart_height)
        });
        let divider_y = ui.cursor().top();
        ui.painter().hline(
            egui::Rangef::new(ui.min_rect().left(), ui.max_rect().right()),
            divider_y,
            egui::Stroke::new(1.0, t.color.border),
        );
        ui.add_space(1.0);
        run_margin_matrix(ui, app, run_index, &evidence);
    }
    let divider_y = ui.cursor().top();
    ui.painter().hline(
        egui::Rangef::new(ui.min_rect().left(), ui.max_rect().right()),
        divider_y,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    specification_matrix(ui, app, &evidence);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VerifyLayout {
    split: bool,
    left_width: f32,
    right_width: f32,
    first_row_height: f32,
}

impl VerifyLayout {
    fn resolve(
        viewport_width: f32,
        content_width: f32,
        visible_height: f32,
        specification_rows: usize,
        section_header_height: f32,
        row_height: f32,
    ) -> Self {
        let content_width = content_width.max(0.0);
        let specification_height =
            section_header_height + 27.0 + row_height * specification_rows.max(1) as f32;
        let first_row_height = (visible_height - specification_height - 1.0).max(210.0);
        let split = viewport_width > 1_020.0 && content_width >= 641.0;
        let left_width = if split {
            let usable = content_width - 1.0;
            (usable * 0.45).clamp(280.0, usable - 360.0)
        } else {
            content_width
        };
        let right_width = if split {
            content_width - left_width - 1.0
        } else {
            content_width
        };
        Self {
            split,
            left_width,
            right_width,
            first_row_height,
        }
    }
}

#[derive(Clone)]
struct SpecificationEvidence {
    spec: SpecEntry,
    values: Vec<(String, f64)>,
}

#[derive(Clone)]
struct TableCell {
    text: String,
    mono: bool,
    color: Option<egui::Color32>,
}

impl TableCell {
    fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: false,
            color: None,
        }
    }

    fn mono(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: None,
        }
    }

    fn tone(text: impl Into<String>, color: egui::Color32) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: Some(color),
        }
    }
}

fn verification_run_index(app: &RSpiceApp) -> Option<usize> {
    app.state
        .simulation
        .active_run_idx
        .filter(|index| *index < app.state.simulation.runs.len())
}

fn verification_run(app: &RSpiceApp) -> Option<&crate::state::SimulationRun> {
    verification_run_index(app).and_then(|index| app.state.simulation.runs.get(index))
}

fn verification_yield_results(app: &RSpiceApp) -> &[crate::services::yield_manager::YieldResult] {
    verification_run(app)
        .and_then(|run| {
            app.state
                .simulation
                .yield_results_for_dataset(run.dataset_id)
        })
        .unwrap_or(&[])
}

fn specification_evidence(app: &RSpiceApp, run_index: Option<usize>) -> Vec<SpecificationEvidence> {
    let run = run_index.and_then(|index| app.state.simulation.runs.get(index));
    app.state
        .workspace
        .specs
        .iter()
        .cloned()
        .map(|spec| {
            let values = run
                .and_then(|run| {
                    measurement_in_run(run, &spec.measurement)
                        .map(|value| (format!("Run {}", run.id), value))
                })
                .into_iter()
                .collect();
            SpecificationEvidence { spec, values }
        })
        .collect()
}

fn engineering_status_strip(ui: &mut Ui, app: &RSpiceApp, evidence: &[SpecificationEvidence]) {
    let t = Tokens::get(ui.ctx());
    let yield_results = verification_yield_results(app);
    let joint_yield = joint_sample_summary(yield_results);
    let covered = evidence.iter().filter(|row| !row.values.is_empty()).count();
    let worst = evidence
        .iter()
        .filter_map(|row| {
            row.values
                .iter()
                .filter_map(|(run, value)| {
                    normalized_margin(&row.spec, *value).map(|margin| (margin, row, run, *value))
                })
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let items = [
        (
            "Estimated yield".to_owned(),
            joint_yield.map_or_else(
                || "No evidence".to_owned(),
                |summary| format!("{:.2}%", summary.yield_percent()),
            ),
            joint_yield.map_or_else(
                || "No aligned all-spec sample trail".to_owned(),
                |summary| {
                    format!(
                        "Joint AND · {} specs · {} samples",
                        summary.specification_count, summary.total
                    )
                },
            ),
            joint_yield.map_or(t.color.warn, |summary| {
                if summary.passing == summary.total {
                    t.color.ok
                } else {
                    t.color.warn
                }
            }),
        ),
        (
            "Passing samples".to_owned(),
            joint_yield.map_or_else(
                || "0 / 0".to_owned(),
                |summary| format!("{} / {}", summary.passing, summary.total),
            ),
            joint_yield.map_or_else(
                || "No retained sample trail".to_owned(),
                |summary| {
                    format!(
                        "{} violate at least one executable specification",
                        summary.total - summary.passing
                    )
                },
            ),
            joint_yield.map_or(t.color.text, |summary| {
                if summary.passing == summary.total {
                    t.color.ok
                } else {
                    t.color.text
                }
            }),
        ),
        (
            "Worst margin".to_owned(),
            worst.map_or_else(
                || "No evidence".to_owned(),
                |(margin, _, _, _)| format!("{:+.2}%", margin * 100.0),
            ),
            worst.map_or_else(
                || "No bounded measurement available".to_owned(),
                |(_, row, run, _)| format!("{} · {run}", row.spec.measurement),
            ),
            worst.map_or(t.color.warn, |(margin, _, _, _)| {
                if margin < 0.0 {
                    t.color.err
                } else {
                    t.color.ok
                }
            }),
        ),
        (
            "Evidence coverage".to_owned(),
            format!("{} / {}", covered, evidence.len()),
            if evidence.is_empty() {
                "No project specifications".to_owned()
            } else {
                format!(
                    "{} specifications lack active-run evidence",
                    evidence.len() - covered
                )
            },
            if covered == evidence.len() && !evidence.is_empty() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    verification_kpi_strip(ui, &items);
}

fn verification_kpi_strip(ui: &mut Ui, items: &[(String, String, String, egui::Color32)]) {
    let t = Tokens::get(ui.ctx());
    let columns = verification_status_columns(ui.ctx().content_rect().width());
    let width = ui.available_width().max(1.0);
    let cell_width = width / columns as f32;
    let top = ui.cursor().top();
    ui.painter().hline(
        egui::Rangef::new(ui.min_rect().left(), ui.min_rect().left() + width),
        top,
        egui::Stroke::new(1.0, t.color.border),
    );
    for row in items.chunks(columns) {
        let row_height = row
            .iter()
            .map(|(_, _, detail, _)| {
                let galley = ui.painter().layout(
                    detail.clone(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                    (cell_width - 20.0).max(1.0),
                );
                (56.0 + galley.size().y).max(73.0)
            })
            .fold(73.0_f32, f32::max);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            for (index, (label, value, detail, tone)) in row.iter().enumerate() {
                let (rect, response) = ui
                    .allocate_exact_size(egui::vec2(cell_width, row_height), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        format!("{label}: {value}. {detail}"),
                    )
                });
                if ui.is_rect_visible(rect) {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
                    let content_rect = rect.shrink2(egui::vec2(10.0, 0.0));
                    let painter = ui.painter().with_clip_rect(content_rect);
                    painter.text(
                        rect.left_top() + egui::vec2(10.0, 10.0),
                        egui::Align2::LEFT_TOP,
                        label,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    );
                    painter.text(
                        rect.left_top() + egui::vec2(10.0, 29.0),
                        egui::Align2::LEFT_TOP,
                        value,
                        theme::mono(17.0, FontWeight::Medium),
                        *tone,
                    );
                    let detail_galley = painter.layout(
                        detail.clone(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_faint,
                        (cell_width - 20.0).max(1.0),
                    );
                    painter.galley(
                        rect.left_top() + egui::vec2(10.0, 52.0),
                        detail_galley,
                        t.color.text_faint,
                    );
                    if index + 1 < row.len() {
                        ui.painter().vline(
                            rect.right(),
                            rect.y_range(),
                            egui::Stroke::new(1.0, t.color.border),
                        );
                    }
                }
            }
        });
        ui.painter().hline(
            egui::Rangef::new(ui.min_rect().left(), ui.min_rect().left() + width),
            ui.cursor().top(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JointSampleSummary {
    passing: usize,
    total: usize,
    specification_count: usize,
}

impl JointSampleSummary {
    fn yield_percent(self) -> f64 {
        self.passing as f64 / self.total as f64 * 100.0
    }
}

fn joint_sample_summary(
    results: &[crate::services::yield_manager::YieldResult],
) -> Option<JointSampleSummary> {
    let first = results.first()?;
    let total = first.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    let passing = (0..total)
        .filter(|index| results.iter().all(|result| result.trail[*index]))
        .count();
    Some(JointSampleSummary {
        passing,
        total,
        specification_count: results.len(),
    })
}

fn worst_individual_yield_result(
    results: &[crate::services::yield_manager::YieldResult],
) -> Option<&crate::services::yield_manager::YieldResult> {
    results.iter().min_by(|a, b| {
        a.yield_percent
            .partial_cmp(&b.yield_percent)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn yield_chart(ui: &mut Ui, app: &RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, height.max(210.0)), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    let head = egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + 36.0));
    ui.painter().rect_filled(head, 0.0, t.color.bg_panel);
    ui.painter().hline(
        head.x_range(),
        head.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let Some(result) = worst_individual_yield_result(verification_yield_results(app)) else {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                "Monte Carlo distribution: no evidence for the active dataset",
            )
        });
        ui.painter().text(
            egui::pos2(rect.center().x, rect.center().y + 12.0),
            egui::Align2::CENTER_CENTER,
            "No Monte Carlo distribution evidence for the active dataset",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    };
    // A joint all-spec verdict is boolean and has no honest physical x-axis.
    // Keep the histogram on one real measurement distribution and label that
    // scope explicitly; the status strip above owns the joint AND yield.
    let title = format!(
        "Worst individual spec · {} distribution",
        result.spec.target
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title = elide_table_text(ui, &title, &title_font, (head.width() * 0.52).max(80.0));
    ui.painter().text(
        head.left_center() + egui::vec2(10.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );
    ui.painter().text(
        head.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        format!(
            "mean {} · σ {}",
            format_value(result.stats.mean, &result.spec.unit),
            format_value(result.stats.std_dev, &result.spec.unit)
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let samples: Vec<f64> = result
        .samples
        .iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .collect();
    if samples.is_empty() {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                format!(
                    "Worst individual specification {} distribution: summary retained, exact sample payload unavailable",
                    result.spec.target
                ),
            )
        });
        ui.painter().text(
            egui::pos2(rect.center().x, rect.center().y + 12.0),
            egui::Align2::CENTER_CENTER,
            "Summary retained; exact sample payload unavailable",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }
    let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bins = histogram_bins(&samples, 22, min, max);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Monte Carlo worst individual specification {} distribution. {} exact samples. Mean {}. Standard deviation {}. Range {} to {}. Limit {}. Histogram bin counts: {}",
                result.spec.target,
                samples.len(),
                format_value(result.stats.mean, &result.spec.unit),
                format_value(result.stats.std_dev, &result.spec.unit),
                format_value(min, &result.spec.unit),
                format_value(max, &result.spec.unit),
                yield_limit_text(&result.spec),
                bins.iter().map(usize::to_string).collect::<Vec<_>>().join(", ")
            ),
        )
    });
    let peak = bins.iter().copied().max().unwrap_or(1).max(1) as f32;
    let plot = egui::Rect::from_min_max(
        head.left_bottom() + egui::vec2(10.0, 12.0),
        rect.right_bottom() - egui::vec2(10.0, 24.0),
    );
    for step in 0..=4 {
        let y = egui::lerp(plot.bottom()..=plot.top(), step as f32 / 4.0);
        ui.painter().hline(
            plot.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid.gamma_multiply(0.55)),
        );
    }
    let bar_width = plot.width() / bins.len() as f32;
    let span = (max - min).max(f64::EPSILON);
    for (index, count) in bins.iter().copied().enumerate() {
        let height = plot.height() * count as f32 / peak;
        let center = min + span * (index as f64 + 0.5) / bins.len() as f64;
        let failing = !result.spec.evaluates(center);
        let bar = egui::Rect::from_min_max(
            egui::pos2(
                plot.left() + index as f32 * bar_width + 1.0,
                plot.bottom() - height,
            ),
            egui::pos2(
                plot.left() + (index + 1) as f32 * bar_width - 1.0,
                plot.bottom(),
            ),
        );
        ui.painter().rect_filled(
            bar,
            1.0,
            if failing {
                t.color.err.gamma_multiply(0.82)
            } else {
                t.color.accent.gamma_multiply(0.72)
            },
        );
    }
    for limit in [result.spec.min, result.spec.max].into_iter().flatten() {
        if limit >= min && limit <= max && max > min {
            let x = egui::remap(limit, min..=max, plot.left() as f64..=plot.right() as f64) as f32;
            ui.painter()
                .vline(x, plot.y_range(), egui::Stroke::new(1.0, t.color.err));
        }
    }
    for step in 0..=5 {
        let fraction = step as f64 / 5.0;
        let value = min + (max - min) * fraction;
        let x = egui::lerp(plot.left()..=plot.right(), fraction as f32);
        let align = if step == 0 {
            egui::Align2::LEFT_TOP
        } else if step == 5 {
            egui::Align2::RIGHT_TOP
        } else {
            egui::Align2::CENTER_TOP
        };
        ui.painter().text(
            egui::pos2(x, plot.bottom() + 4.0),
            align,
            format_scalar(value),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

fn yield_limit_text(spec: &crate::services::yield_manager::YieldSpec) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!(
            "{}…{} {}",
            format_scalar(min),
            format_scalar(max),
            spec.unit
        ),
        (Some(min), None) => format!("≥ {} {}", format_scalar(min), spec.unit),
        (None, Some(max)) => format!("≤ {} {}", format_scalar(max), spec.unit),
        (None, None) => "unbounded".to_owned(),
    }
}

fn histogram_bins(samples: &[f64], count: usize, min: f64, max: f64) -> Vec<usize> {
    let mut bins = vec![0; count.max(1)];
    let span = max - min;
    for sample in samples {
        let index = if span <= f64::EPSILON {
            bins.len() / 2
        } else {
            (((sample - min) / span) * bins.len() as f64)
                .floor()
                .clamp(0.0, (bins.len() - 1) as f64) as usize
        };
        bins[index] += 1;
    }
    bins
}

fn run_margin_matrix(
    ui: &mut Ui,
    app: &RSpiceApp,
    run_index: Option<usize>,
    evidence: &[SpecificationEvidence],
) {
    let run = run_index.and_then(|index| app.state.simulation.runs.get(index));
    table_section_header(
        ui,
        "Active dataset · specification margin",
        run.map(|run| format!("Run {}", run.id)).as_deref(),
        None,
    );
    let headers = vec![
        ("Specification".to_owned(), 0.55),
        ("Margin".to_owned(), 0.45),
    ];
    let rows = evidence
        .iter()
        .map(|row| {
            let margin = run
                .and_then(|run| measurement_in_run(run, &row.spec.measurement))
                .and_then(|value| normalized_margin(&row.spec, value));
            vec![
                TableCell::text(format!(
                    "{} · {}",
                    row.spec.measurement,
                    limit_text(&row.spec)
                )),
                margin.map_or_else(
                    || TableCell::tone("NO EVIDENCE", Tokens::get(ui.ctx()).color.warn),
                    |margin| {
                        TableCell::tone(
                            format!("{:+.2}%", margin * 100.0),
                            if margin < 0.0 {
                                Tokens::get(ui.ctx()).color.err
                            } else {
                                Tokens::get(ui.ctx()).color.ok
                            },
                        )
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    render_data_table(ui, "verification-run-margin", &headers, &rows, None);
}

fn specification_matrix(ui: &mut Ui, app: &mut RSpiceApp, evidence: &[SpecificationEvidence]) {
    let action = table_section_header(
        ui,
        "Measurement & specification matrix",
        Some(&format!(
            "{} coverage gaps",
            evidence.iter().filter(|row| row.values.is_empty()).count()
        )),
        Some("Edit specifications"),
    );
    if action {
        Command::EditSpecifications.execute(app);
    }
    let t = Tokens::get(ui.ctx());
    let headers = vec![
        ("Measurement".to_owned(), 0.25),
        ("Limit".to_owned(), 0.15),
        ("Active value".to_owned(), 0.16),
        ("Margin".to_owned(), 0.16),
        ("Run".to_owned(), 0.14),
        ("Status".to_owned(), 0.14),
    ];
    let rows = evidence
        .iter()
        .map(|row| {
            let active = row.values.first();
            let margin = active.and_then(|(_, value)| normalized_margin(&row.spec, *value));
            let failed = margin.is_some_and(|value| value < 0.0);
            vec![
                TableCell::text(&row.spec.measurement),
                TableCell::mono(limit_text(&row.spec)),
                TableCell::mono(active.map_or_else(
                    || "—".to_owned(),
                    |(_, value)| format_value(*value, &row.spec.unit),
                )),
                TableCell::mono(margin.map_or_else(
                    || "—".to_owned(),
                    |margin| format!("{:+.2}%", margin * 100.0),
                )),
                TableCell::text(active.map_or("—", |(run, _)| run)),
                TableCell::tone(
                    if row.values.is_empty() {
                        "NO EVIDENCE"
                    } else if failed {
                        "FAIL"
                    } else {
                        "PASS"
                    },
                    if row.values.is_empty() {
                        t.color.warn
                    } else if failed {
                        t.color.err
                    } else {
                        t.color.ok
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    render_data_table(
        ui,
        "verification-specification-matrix",
        &headers,
        &rows,
        None,
    );
}

fn table_section_header(
    ui: &mut Ui,
    title: &str,
    status: Option<&str>,
    action: Option<&str>,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let visible_width = ui
        .available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width();
    let stacked = visible_width < 760.0 && (status.is_some() || action.is_some());
    let height = if stacked {
        70.0
    } else if t.metrics.ctl_h >= 44.0 {
        44.0
    } else {
        37.0
    };
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let inner = rect
        .intersect(ui.clip_rect())
        .shrink2(egui::vec2(11.0, 0.0));
    let mut clicked = false;
    if stacked {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        child.spacing_mut().item_spacing.y = 4.0;
        child.add_space(6.0);
        child.label(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        child.horizontal(|ui| {
            ui.set_width(ui.available_width());
            if let Some(status) = status {
                ui.label(
                    egui::RichText::new(status)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(action) = action {
                    clicked = Button::new(action).ghost().show(ui).clicked();
                }
            });
        });
    } else {
        let mut title_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        title_ui.label(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        let mut meta_right = inner.right();
        if let Some(action) = action {
            let action_width = (ui
                .painter()
                .layout_no_wrap(
                    action.to_owned(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text,
                )
                .size()
                .x
                + 20.0)
                .max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 });
            let action_rect = egui::Rect::from_min_max(
                egui::pos2(meta_right - action_width, inner.top()),
                egui::pos2(meta_right, inner.bottom()),
            );
            let mut action_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(action_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            clicked = Button::new(action).ghost().show(&mut action_ui).clicked();
            meta_right = action_rect.left() - 8.0;
        }
        if let Some(status) = status {
            let mut status_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(egui::Rect::from_min_max(
                        inner.min,
                        egui::pos2(meta_right, inner.bottom()),
                    ))
                    .layout(egui::Layout::right_to_left(egui::Align::Center)),
            );
            status_ui.label(
                egui::RichText::new(status)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        }
    }
    clicked
}

fn render_data_table(
    ui: &mut Ui,
    id: &str,
    headers: &[(String, f32)],
    rows: &[Vec<TableCell>],
    desktop_min_width: Option<f32>,
) {
    let t = Tokens::get(ui.ctx());
    let row_height = verification_table_row_height(ui.ctx().content_rect().width());
    // Capture the viewport before entering the horizontal scroll area. Inside
    // it, `available_width()` is the unconstrained scroll content width and
    // previously stretched ordinary tables far past the visible surface.
    let viewport_width = ui.available_width().max(1.0);
    egui::ScrollArea::horizontal()
        .id_salt(id)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
        .show(ui, |ui| {
            let width = verification_table_width(
                ui.ctx().content_rect().width(),
                viewport_width,
                desktop_min_width,
            );
            ui.set_min_width(width);
            let (head, head_response) =
                ui.allocate_exact_size(egui::vec2(width, 27.0), egui::Sense::hover());
            head_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    ui.is_enabled(),
                    format!(
                        "Table columns: {}",
                        headers
                            .iter()
                            .map(|(label, _)| label.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                )
            });
            ui.painter().rect_filled(head, 0.0, t.color.bg_panel_2);
            paint_table_cells(ui, head, headers, true, |label| {
                TableCell::text(label.to_uppercase())
            });
            ui.painter().hline(
                head.x_range(),
                head.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            if rows.is_empty() {
                let empty_message = verification_table_empty_message(id);
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        empty_message,
                    )
                });
                ui.painter().text(
                    rect.left_center() + egui::vec2(8.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    empty_message,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                );
                return;
            }
            for cells in rows {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        headers
                            .iter()
                            .zip(cells)
                            .map(|((header, _), cell)| format!("{header}: {}", cell.text))
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                });
                if response.hovered() {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                }
                paint_table_cells(ui, rect, headers, false, |label| {
                    let index = headers
                        .iter()
                        .position(|(header, _)| header == label)
                        .unwrap_or(0);
                    cells
                        .get(index)
                        .cloned()
                        .unwrap_or_else(|| TableCell::text("—"))
                });
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom(),
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
        });
}

fn verification_table_width(
    viewport_width: f32,
    surface_width: f32,
    desktop_min_width: Option<f32>,
) -> f32 {
    let responsive_minimum = desktop_min_width.unwrap_or({
        if viewport_width <= VERIFY_RESPONSIVE_BREAKPOINT {
            540.0
        } else {
            0.0
        }
    });
    surface_width.max(responsive_minimum)
}

fn verification_status_columns(viewport_width: f32) -> usize {
    if viewport_width <= VERIFY_KPI_BREAKPOINT {
        2
    } else {
        4
    }
}

fn verification_table_empty_message(id: &str) -> &'static str {
    match id {
        "verification-run-margin" => "No retained run-margin evidence",
        "verification-specification-matrix" => "No specification evidence",
        "verify-corner-matrix" => "No retained corner-signal evidence",
        "verify-corner-worst-points" => "No point-attributed corner verdicts",
        "verify-optimization-results" => "No retained optimization traces",
        "verify-regression-checks" => "No aligned regression checks",
        _ => "No retained evidence",
    }
}

fn verification_stacked_chart_height(viewport_width: f32) -> f32 {
    if viewport_width <= VERIFY_PHONE_BREAKPOINT {
        VERIFY_PHONE_CHART_HEIGHT
    } else {
        VERIFY_STACKED_CHART_HEIGHT
    }
}

fn verification_table_row_height(viewport_width: f32) -> f32 {
    if viewport_width <= VERIFY_PHONE_BREAKPOINT {
        36.0
    } else if viewport_width <= VERIFY_RESPONSIVE_BREAKPOINT {
        31.0
    } else {
        28.0
    }
}

fn paint_table_cells(
    ui: &mut Ui,
    rect: egui::Rect,
    columns: &[(String, f32)],
    header: bool,
    mut cell: impl FnMut(&str) -> TableCell,
) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left();
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        let cell_rect = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        let value = cell(label);
        let font = if value.mono {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(
                tokens::FS_0,
                if header {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            )
        };
        let text = elide_table_text(ui, &value.text, &font, (cell_rect.width() - 16.0).max(0.0));
        let painter = ui
            .painter()
            .with_clip_rect(cell_rect.shrink2(egui::vec2(8.0, 0.0)));
        painter.text(
            cell_rect.left_center() + egui::vec2(8.0, 0.0),
            egui::Align2::LEFT_CENTER,
            text,
            font,
            value.color.unwrap_or(if header {
                t.color.text_faint
            } else {
                t.color.text_dim
            }),
        );
        x += width;
    }
}

fn elide_table_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let ellipsis = '…';
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once(ellipsis))
            .collect::<String>();
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once(ellipsis))
        .collect()
}

fn measurement_in_run(run: &crate::state::SimulationRun, name: &str) -> Option<f64> {
    run.analyses.iter().find_map(|analysis| {
        if !verified_analysis(analysis) {
            return None;
        }
        analysis
            .measurements
            .iter()
            .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .filter(|measurement| measurement.passed && measurement.error.is_none())
            .and_then(|measurement| measurement.value)
            .filter(|value| value.is_finite())
    })
}

#[cfg(test)]
fn signed_margin(spec: &SpecEntry, value: f64) -> f64 {
    if let Some(min) = spec.min
        && value < min
    {
        return value - min;
    }
    if let Some(max) = spec.max
        && value > max
    {
        return max - value;
    }
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => (value - min).min(max - value),
        (Some(min), None) => value - min,
        (None, Some(max)) => max - value,
        (None, None) => 0.0,
    }
}

/// Dimensionless specification margin used for cross-specification ranking.
/// One-sided limits are normalized by the limit magnitude; ranges are
/// normalized by their span. Unbounded tracked values have no comparable
/// margin and are deliberately excluded.
fn normalized_margin(spec: &SpecEntry, value: f64) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (spec.min, spec.max) {
        (Some(min), Some(max)) if max > min => {
            Some((value - min).min(max - value) / (max - min).max(SCALE_FLOOR))
        }
        (Some(min), None) => Some((value - min) / min.abs().max(SCALE_FLOOR)),
        (None, Some(max)) => Some((max - value) / max.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}

fn limit_text(spec: &SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!(
            "{}…{} {}",
            format_scalar(min),
            format_scalar(max),
            spec.unit
        ),
        (Some(min), None) => format!("≥ {} {}", format_scalar(min), spec.unit),
        (None, Some(max)) => format!("≤ {} {}", format_scalar(max), spec.unit),
        (None, None) => "tracked value".to_owned(),
    }
}

fn format_value(value: f64, unit: &str) -> String {
    if unit.is_empty() {
        format_scalar(value)
    } else {
        format!("{} {unit}", format_scalar(value))
    }
}

fn format_scalar(value: f64) -> String {
    let magnitude = value.abs();
    if magnitude != 0.0 && !(1.0e-3..1.0e6).contains(&magnitude) {
        format!("{value:.4e}")
    } else {
        format!("{value:.6}")
    }
}

fn latest_analysis(
    app: &RSpiceApp,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    app.state.simulation.active_run().and_then(|run| {
        run.analyses
            .iter()
            .rev()
            .find(|analysis| analysis.analysis_type == analysis_type && verified_analysis(analysis))
    })
}

fn verified_analysis(analysis: &crate::state::AnalysisResult) -> bool {
    analysis.success && analysis.provenance.is_some()
}

fn request_analysis_run(
    app: &mut RSpiceApp,
    kinds: &[crate::simulation::plan::AnalysisKind],
) -> Result<(), String> {
    let mut selected = None;
    {
        let plan = app.state.sim_setup.stable_analysis_plan_mut()?;
        for kind in kinds {
            let existing = plan
                .instances()
                .iter()
                .find(|instance| instance.kind() == *kind)
                .map(|instance| (instance.id(), instance.enabled()));
            let id = if let Some((id, enabled)) = existing {
                if !enabled {
                    plan.set_enabled(id, true)
                        .map_err(|error| error.to_string())?;
                }
                id
            } else {
                plan.insert(*kind).map_err(|error| error.to_string())?.0
            };
            selected = Some((*kind, id));
        }
    }
    app.state.sim_setup.refresh_legacy_analysis_projections();
    if let Some((kind, id)) = selected {
        app.state.workbench.active_analysis = kind.legacy_index();
        app.state.workbench.active_analysis_instance = Some(id);
    }
    if !app.state.can_run_simulation() {
        return Err(
            "Execution was not started because the project or analysis plan did not pass its run contract. Review preflight evidence before retrying."
                .to_owned(),
        );
    }
    app.state.request_run_set_simulation();
    Ok(())
}

fn open_analysis_configuration(
    app: &mut RSpiceApp,
    kind: crate::simulation::plan::AnalysisKind,
) -> Result<(), String> {
    let id = {
        let plan = app.state.sim_setup.stable_analysis_plan_mut()?;
        if let Some(instance) = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == kind)
        {
            instance.id()
        } else {
            plan.insert(kind).map_err(|error| error.to_string())?.0
        }
    };
    app.state.sim_setup.refresh_legacy_analysis_projections();
    app.state.workbench.active_analysis = kind.legacy_index();
    app.state.workbench.active_analysis_instance = Some(id);
    app.state
        .workbench
        .activate(super::super::state::Workspace::Simulate);
    Ok(())
}

fn record_verification_action(app: &mut RSpiceApp, result: Result<(), String>, success: &str) {
    app.state.workbench.verification.action_receipt = match result {
        Ok(()) => success.to_owned(),
        Err(error) => format!("Action blocked: {error}"),
    };
}

fn action_receipt(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    card(ui, "Verification action receipt", |ui| {
        ui.label(
            egui::RichText::new(&app.state.workbench.verification.action_receipt)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

fn corners(ui: &mut Ui, app: &mut RSpiceApp) {
    let result = latest_analysis(app, crate::state::AnalysisType::Corner);
    let point_count = result
        .and_then(|analysis| analysis.waveforms.first())
        .map_or(0, |waveform| waveform.x.len());
    let signal_count = result.map_or(0, |analysis| analysis.waveforms.len());
    // `AnalysisResult::measurements` has no corner-point identity. It must not
    // be presented as a pointwise specification matrix until that provenance
    // is retained by the result schema.
    let configured_specifications = app.state.workspace.specs.len();
    let (elapsed, run_detail) = verification_run(app).map_or_else(
        || {
            (
                "No dataset".to_owned(),
                "No active immutable run".to_owned(),
            )
        },
        |run| {
            (
                format!("{:.3} s", run.elapsed_time),
                format!("Run {} · dataset {}", run.id, run.dataset_id),
            )
        },
    );
    let t = Tokens::get(ui.ctx());
    let items = [
        (
            "PVT points retained".to_owned(),
            point_count.to_string(),
            if point_count == 0 {
                "No process-corner evidence".to_owned()
            } else {
                run_detail
            },
            if point_count == 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Signals retained".to_owned(),
            signal_count.to_string(),
            "source-attributed corner traces".to_owned(),
            if signal_count == 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Specifications configured".to_owned(),
            configured_specifications.to_string(),
            "No pointwise verdict payload retained".to_owned(),
            t.color.warn,
        ),
        (
            "Runtime".to_owned(),
            elapsed,
            "active immutable run".to_owned(),
            t.color.text,
        ),
    ];
    verification_kpi_strip(ui, &items);

    if let Some(result) = result {
        let axis = result
            .waveforms
            .first()
            .map(|waveform| waveform.x.as_slice())
            .unwrap_or(&[]);
        if axis.is_empty() {
            card(ui, "Corner result integrity", |ui| {
                ui.label("The retained corner result has no point axis. It is not presented as verification evidence.");
            });
        } else {
            let mut headers = vec![("Signal".to_owned(), 0.24)];
            let point_fraction = 0.64 / axis.len() as f32;
            let point_labels = corner_point_labels(app, result);
            headers.extend(axis.iter().enumerate().map(|(index, value)| {
                let label = point_labels
                    .as_ref()
                    .and_then(|labels| labels.get(index))
                    .cloned()
                    .unwrap_or_else(|| format!("P{:02} · {}", index + 1, format_scalar(*value)));
                (label, point_fraction)
            }));
            headers.push(("State".to_owned(), 0.12));
            let t = Tokens::get(ui.ctx());
            let nominal_index = app
                .state
                .workbench
                .verification
                .corner_compare_nominal
                .then(|| corner_nominal_index(app))
                .flatten();
            let rows = result
                .waveforms
                .iter()
                .map(|waveform| {
                    let aligned = waveform.x.as_slice() == axis && waveform.y.len() == axis.len();
                    let mut row = vec![TableCell::text(&waveform.name)];
                    if aligned {
                        let nominal = nominal_index
                            .and_then(|index| waveform.y.get(index))
                            .copied();
                        row.extend(waveform.y.iter().map(|value| {
                            TableCell::mono(nominal.map_or_else(
                                || format_scalar(*value),
                                |nominal| format!("{:+.6e}", *value - nominal),
                            ))
                        }));
                    } else {
                        row.extend((0..axis.len()).map(|_| TableCell::mono("—")));
                    }
                    row.push(TableCell::tone(
                        if aligned { "ALIGNED" } else { "INVALID" },
                        if aligned { t.color.ok } else { t.color.err },
                    ));
                    row
                })
                .collect::<Vec<_>>();
            table_section_header(
                ui,
                "Full PVT retained-value matrix",
                Some(if nominal_index.is_some() {
                    "exact delta from retained nominal point"
                } else {
                    "absolute source-attributed values"
                }),
                None,
            );
            render_data_table(
                ui,
                "verify-corner-matrix",
                &headers,
                &rows,
                Some((220.0 + axis.len() as f32 * 88.0).max(1_450.0)),
            );
            corner_evidence_details(ui, app, result, point_count);
        }
    } else {
        card(ui, "Corner evidence", |ui| {
            ui.label("No retained process-corner analysis is available for the active dataset. Run the configured corner plan to create evidence.");
        });
    }
}

fn export_active_corner_matrix(app: &mut RSpiceApp) {
    let result_id =
        latest_analysis(app, crate::state::AnalysisType::Corner).map(|result| result.id);
    let analysis_index = result_id.and_then(|result_id| {
        app.state.simulation.active_run().and_then(|run| {
            run.analyses.iter().position(|analysis| {
                analysis.id == result_id
                    && analysis.analysis_type == crate::state::AnalysisType::Corner
                    && verified_analysis(analysis)
            })
        })
    });
    if let Some(index) = analysis_index {
        app.state.simulation.select_analysis(index);
        Command::ExportWaveformsCsv.execute(app);
        app.state.workbench.verification.action_receipt =
            "The retained process-corner matrix was selected for CSV export.".to_owned();
    } else {
        app.state.workbench.verification.action_receipt =
            "Corner export blocked: no source-attributed successful corner result is active."
                .to_owned();
    }
}

fn corner_point_labels(
    app: &RSpiceApp,
    result: &crate::state::AnalysisResult,
) -> Option<Vec<String>> {
    use crate::simulation::plan::AnalysisDraft;

    let provenance = result.provenance.as_ref()?;
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    let instance = plan
        .instances()
        .iter()
        .find(|instance| instance.id() == provenance.source_instance_id())?;
    if instance.modified_revision() != provenance.source_revision() {
        return None;
    }
    let AnalysisDraft::Corner(state) = instance.draft() else {
        return None;
    };
    let labels = state.to_config().ok()?.corner_names();
    let points = result.waveforms.first()?.x.len();
    (labels.len() == points).then_some(labels)
}

fn corner_nominal_index(app: &RSpiceApp) -> Option<usize> {
    let result = latest_analysis(app, crate::state::AnalysisType::Corner)?;
    let labels = corner_point_labels(app, result)?;
    let exact_nominal = format!(
        "TT_1.00V_{:.0}C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    labels.iter().position(|label| label == &exact_nominal)
}

fn corner_evidence_details(
    ui: &mut Ui,
    app: &RSpiceApp,
    result: &crate::state::AnalysisResult,
    point_count: usize,
) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width();
    let split = ui.ctx().content_rect().width() > 1_020.0 && available >= 641.0;
    let render_worst = |ui: &mut Ui| {
        table_section_header(ui, "Worst PVT points", None, None);
        let headers = vec![
            ("Rank".to_owned(), 0.16),
            ("Point".to_owned(), 0.28),
            ("Limiting spec".to_owned(), 0.36),
            ("Margin".to_owned(), 0.20),
        ];
        let rows = vec![vec![
            TableCell::mono("—"),
            TableCell::text("No pointwise evidence"),
            TableCell::text("Point-attributed measurements not retained"),
            TableCell::tone("NO VERDICT", t.color.warn),
        ]];
        render_data_table(ui, "verify-corner-worst-points", &headers, &rows, None);
    };
    let render_reproducibility = |ui: &mut Ui| {
        table_section_header(ui, "Run reproducibility", None, None);
        let run = verification_run(app);
        property_row(
            ui,
            "Dataset",
            &run.map_or_else(
                || "unavailable".to_owned(),
                |run| run.dataset_id.to_string(),
            ),
        );
        property_row(
            ui,
            "Run provenance",
            if run.and_then(|run| run.prepared_receipt()).is_some() {
                "prepared snapshot retained"
            } else {
                "prepared snapshot unavailable"
            },
        );
        property_row(
            ui,
            "Analysis provenance",
            if result.provenance.is_some() {
                "source identity retained"
            } else {
                "legacy / unattributed"
            },
        );
        property_row(ui, "PVT points", &point_count.to_string());
    };
    ui.add_space(1.0);
    if split {
        let left = (available - 1.0) * 0.45;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                egui::vec2(left, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                render_worst,
            );
            ui.allocate_ui_with_layout(
                egui::vec2(available - left - 1.0, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                render_reproducibility,
            );
        });
    } else {
        render_worst(ui);
        ui.add_space(1.0);
        render_reproducibility(ui);
    }
}

fn tuning_unavailable(ui: &mut Ui) {
    card(ui, "Capability boundary", |ui| {
        status_dot(
            ui,
            Tokens::get(ui.ctx()).color.warn,
            "Parameter tuning is not available",
        );
        ui.label("RSpice does not expose a tuning surface until it can discover real design parameters, apply edits transactionally, regenerate the netlist, execute the configured analyses, and retain dataset-owned results for every candidate.");
    });
}

fn optimization(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Optimization execution", |ui| {
        ui.horizontal_wrapped(|ui| {
            if Button::new("Run optimization").accent().show(ui).clicked() {
                let result = request_analysis_run(
                    app,
                    &[crate::simulation::plan::AnalysisKind::Optimization],
                );
                record_verification_action(
                    app,
                    result,
                    "Optimization was dispatched through the active production analysis plan.",
                );
            }
            if Button::new("Configure optimization").show(ui).clicked() {
                let result = open_analysis_configuration(
                    app,
                    crate::simulation::plan::AnalysisKind::Optimization,
                );
                record_verification_action(
                    app,
                    result,
                    "The typed optimization configuration was opened.",
                );
            }
        });
        if let Some(result) = latest_analysis(app, crate::state::AnalysisType::Optimization) {
            let iterations = result
                .waveforms
                .first()
                .map_or(0, |waveform| waveform.x.len());
            property_row(ui, "Retained result", &format!("{iterations} iterations"));
            property_row(ui, "Dataset", "immutable active result");
        } else {
            ui.label("No optimization result is retained. Configure a bounded variable set and execute the production optimization analysis to create evidence.");
        }
    });
    if let Some(result) = latest_analysis(app, crate::state::AnalysisType::Optimization) {
        let t = Tokens::get(ui.ctx());
        let headers = vec![
            ("Trace".to_owned(), 0.38),
            ("Samples".to_owned(), 0.18),
            ("Final value".to_owned(), 0.26),
            ("State".to_owned(), 0.18),
        ];
        let rows = result
            .waveforms
            .iter()
            .map(|waveform| {
                vec![
                    TableCell::text(&waveform.name),
                    TableCell::mono(waveform.y.len().to_string()),
                    TableCell::mono(
                        waveform
                            .y
                            .last()
                            .map_or_else(|| "—".to_owned(), |value| format_scalar(*value)),
                    ),
                    TableCell::tone(
                        if result.success { "RETAINED" } else { "FAILED" },
                        if result.success {
                            t.color.ok
                        } else {
                            t.color.err
                        },
                    ),
                ]
            })
            .collect::<Vec<_>>();
        table_section_header(
            ui,
            "Candidate traces",
            Some("source-owned result evidence"),
            None,
        );
        render_data_table(ui, "verify-optimization-results", &headers, &rows, None);
    }
    action_receipt(ui, app);
}

fn reliability(ui: &mut Ui, app: &mut RSpiceApp) {
    let has_soa_evidence = latest_analysis(app, crate::state::AnalysisType::Soa).is_some();
    let has_reliability_evidence =
        latest_analysis(app, crate::state::AnalysisType::Reliability).is_some();
    card(ui, "Reliability execution", |ui| {
        ui.horizontal_wrapped(|ui| {
            if Button::new("Run preview plan").accent().show(ui).clicked() {
                let result = request_analysis_run(
                    app,
                    &[
                        crate::simulation::plan::AnalysisKind::Soa,
                        crate::simulation::plan::AnalysisKind::Reliability,
                    ],
                );
                record_verification_action(
                    app,
                    result,
                    "Electrical SOA and reliability preview analyses were dispatched through the active plan.",
                );
            }
            if Button::new("Edit mission profile").show(ui).clicked() {
                let result = open_analysis_configuration(
                    app,
                    crate::simulation::plan::AnalysisKind::Reliability,
                );
                record_verification_action(
                    app,
                    result,
                    "The typed reliability mission-profile configuration was opened.",
                );
            }
        });
        ui.label("Electrical reliability evaluates operating stress and mission aging. Physical geometry checks remain owned by the Physical DRC flow.");
    });
    card(ui, "Safe operating area", |ui| {
        status_dot(
            ui,
            Tokens::get(ui.ctx()).color.warn,
            "Dataset-owned SOA payload unavailable",
        );
        ui.label(if has_soa_evidence {
            "A successful source-attributed SOA execution receipt exists for the active dataset, but violation records are not yet stored inside that immutable dataset. No pass/fail verdict is issued."
        } else {
            "No successful source-attributed SOA execution receipt exists for the active dataset. No pass/fail verdict is issued."
        });
    });
    ui.add_space(1.0);
    card(ui, "Aging and reliability", |ui| {
        status_dot(
            ui,
            Tokens::get(ui.ctx()).color.warn,
            "Dataset-owned aging payload unavailable",
        );
        ui.label(if has_reliability_evidence {
            "A successful source-attributed reliability execution receipt exists for the active dataset, but device-aging records are not yet stored inside that immutable dataset. No reliability claim is issued."
        } else {
            "No successful source-attributed reliability execution receipt exists for the active dataset. No reliability claim is issued."
        });
        if Button::new("Configure reliability analysis…")
            .show(ui)
            .clicked()
        {
            app.state
                .workbench
                .activate(super::super::state::Workspace::Simulate);
            let kind = crate::simulation::plan::AnalysisKind::Reliability;
            let existing = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                        .map(|instance| instance.id())
                });
            let selected = if let Some(id) = existing {
                Some(id)
            } else {
                match app
                    .state
                    .sim_setup
                    .stable_analysis_plan_mut()
                    .and_then(|plan| plan.insert(kind).map_err(|error| error.to_string()))
                {
                    Ok((id, receipt)) => {
                        app.state.workbench.analysis_lifecycle_status = receipt.detail().to_owned();
                        app.state.sim_setup.refresh_legacy_analysis_projections();
                        Some(id)
                    }
                    Err(error) => {
                        app.state.workbench.analysis_lifecycle_status = error;
                        None
                    }
                }
            };
            app.state.workbench.active_analysis = kind.legacy_index();
            app.state.workbench.active_analysis_instance = selected;
        }
    });
    action_receipt(ui, app);
}

#[derive(Debug, Clone, PartialEq)]
struct RegressionCheck {
    name: String,
    source_identity: String,
    baseline: f64,
    current: f64,
}

impl RegressionCheck {
    fn delta(&self) -> f64 {
        self.current - self.baseline
    }

    fn changed(&self) -> bool {
        self.current.to_bits() != self.baseline.to_bits()
    }
}

fn derive_regression_checks(
    baseline: &crate::state::SimulationRun,
    current: &crate::state::SimulationRun,
) -> Vec<RegressionCheck> {
    let mut checks = Vec::new();
    for baseline_analysis in baseline
        .analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
    {
        let baseline_provenance = baseline_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        let Some(current_analysis) = current.analyses.iter().find(|analysis| {
            verified_analysis(analysis)
                && analysis.provenance.as_ref().is_some_and(|provenance| {
                    provenance.source_domain() == baseline_provenance.source_domain()
                        && provenance.source_instance_id()
                            == baseline_provenance.source_instance_id()
                })
        }) else {
            continue;
        };
        let current_provenance = current_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        for (index, baseline_measurement) in baseline_analysis.measurements.iter().enumerate() {
            let name = baseline_measurement.name.to_ascii_lowercase();
            let occurrence = baseline_analysis.measurements[..index]
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&name))
                .count();
            let Some(current_measurement) = current_analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&name))
                .nth(occurrence)
            else {
                continue;
            };
            let Some(baseline_value) = baseline_measurement.value.filter(|value| value.is_finite())
            else {
                continue;
            };
            let Some(current_value) = current_measurement.value.filter(|value| value.is_finite())
            else {
                continue;
            };
            checks.push(RegressionCheck {
                name,
                source_identity: format!(
                    "{} · {:?} → {:?}",
                    baseline_provenance.source_instance_id(),
                    baseline_provenance.source_revision(),
                    current_provenance.source_revision()
                ),
                baseline: baseline_value,
                current: current_value,
            });
        }
    }
    checks.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.source_identity.cmp(&right.source_identity))
    });
    checks
}

fn regression_run_pair(
    app: &RSpiceApp,
) -> Option<(&crate::state::SimulationRun, &crate::state::SimulationRun)> {
    let current = app.state.simulation.active_run()?;
    let selected = active_regression_baseline(app).and_then(|id| {
        app.state
            .simulation
            .runs
            .iter()
            .find(|run| run.run_id == id)
    });
    let baseline = selected?;
    (baseline.run_id != current.run_id).then_some((baseline, current))
}

fn active_regression_baseline(app: &RSpiceApp) -> Option<crate::product::RunId> {
    let plan_id = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)?;
    app.state
        .workspace
        .active_plan_data(plan_id)
        .and_then(|payload| payload.regression_baseline_run)
        .or(app.state.workbench.verification.regression_baseline_run)
}

fn commit_regression_baseline(
    app: &mut RSpiceApp,
    run_id: crate::product::RunId,
) -> Result<(), String> {
    let plan_id = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .ok_or_else(|| "the active simulation plan is unavailable".to_owned())?;
    if !app
        .state
        .simulation
        .runs
        .iter()
        .any(|run| run.run_id == run_id)
    {
        return Err(format!("retained run {run_id} no longer exists"));
    }

    let mut workspace = app.state.workspace.clone();
    let mut setup = app.state.sim_setup.clone();
    workspace
        .ensure_active_plan_data(plan_id)
        .regression_baseline_run = Some(run_id);
    setup
        .commit_active_plan_configuration_change(format!(
            "selected regression baseline run {run_id}"
        ))
        .map_err(|error| error.to_string())?;
    app.state.workspace = workspace;
    app.state.sim_setup = setup;
    app.state.workbench.verification.regression_baseline_run = Some(run_id);
    app.state.workbench.verification.regression_comparison = None;
    Ok(())
}

fn run_regression_comparison(app: &mut RSpiceApp) {
    let Some((baseline, current)) = regression_run_pair(app) else {
        app.state.workbench.verification.regression_comparison = None;
        app.state.workbench.verification.action_receipt =
            "Regression blocked: select a distinct retained baseline first.".to_owned();
        return;
    };
    let baseline_run = baseline.run_id;
    let candidate_run = current.run_id;
    let checks = derive_regression_checks(baseline, current);
    let aligned_waveforms = aligned_regression_waveforms(baseline, current).len();
    let changed_checks = checks.iter().filter(|check| check.changed()).count();
    app.state.workbench.verification.regression_comparison =
        Some(super::super::state::RegressionComparisonReceipt {
            baseline_run,
            candidate_run,
            aligned_checks: checks.len(),
            aligned_waveforms,
            changed_checks,
        });
    app.state.workbench.verification.action_receipt = format!(
        "Regression retained a comparison receipt for {} aligned measurements and {} aligned waveforms. No pass/fail verdict was issued because this project has no regression-tolerance contract.",
        checks.len(),
        aligned_waveforms
    );
}

fn aligned_regression_waveforms<'a>(
    baseline: &'a crate::state::SimulationRun,
    current: &'a crate::state::SimulationRun,
) -> Vec<(
    &'a crate::state::WaveformData,
    &'a crate::state::WaveformData,
)> {
    let mut aligned = Vec::new();
    for baseline_analysis in baseline
        .analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
    {
        let baseline_provenance = baseline_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        let Some(current_analysis) = current.analyses.iter().find(|analysis| {
            verified_analysis(analysis)
                && analysis.provenance.as_ref().is_some_and(|provenance| {
                    provenance.source_domain() == baseline_provenance.source_domain()
                        && provenance.source_instance_id()
                            == baseline_provenance.source_instance_id()
                })
        }) else {
            continue;
        };
        for (index, baseline_waveform) in baseline_analysis.waveforms.iter().enumerate() {
            let occurrence = baseline_analysis.waveforms[..index]
                .iter()
                .filter(|waveform| waveform.name == baseline_waveform.name)
                .count();
            if let Some(current_waveform) = current_analysis
                .waveforms
                .iter()
                .filter(|waveform| waveform.name == baseline_waveform.name)
                .nth(occurrence)
                .filter(|waveform| {
                    baseline_waveform.x.as_slice() == waveform.x.as_slice()
                        && baseline_waveform.y.len() == waveform.y.len()
                        && !baseline_waveform.y.is_empty()
                })
            {
                aligned.push((baseline_waveform, current_waveform));
            }
        }
    }
    aligned
}

fn regression(ui: &mut Ui, app: &mut RSpiceApp) {
    let pair = regression_run_pair(app);
    let checks = pair
        .map(|(baseline, current)| derive_regression_checks(baseline, current))
        .unwrap_or_default();
    let t = Tokens::get(ui.ctx());
    let aligned_waveforms = pair.map_or(0, |(baseline, current)| {
        aligned_regression_waveforms(baseline, current).len()
    });
    let changed = checks.iter().filter(|check| check.changed()).count();
    let worst_delta = checks
        .iter()
        .map(|check| check.delta().abs())
        .filter(|delta| delta.is_finite())
        .max_by(|left, right| left.total_cmp(right));
    let items = [
        (
            "Aligned checks".to_owned(),
            checks.len().to_string(),
            "finite measurements in both datasets".to_owned(),
            if checks.is_empty() {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Waveforms aligned".to_owned(),
            aligned_waveforms.to_string(),
            "exact axis and sample-count match".to_owned(),
            if aligned_waveforms == 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Largest exact delta".to_owned(),
            worst_delta.map_or_else(|| "No evidence".to_owned(), format_scalar),
            format!("{changed} changed values require review"),
            if changed == 0 && !checks.is_empty() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "Tolerance contract".to_owned(),
            "Not configured".to_owned(),
            "no pass/fail verdict issued".to_owned(),
            t.color.warn,
        ),
    ];
    verification_kpi_strip(ui, &items);

    let width = ui.available_width();
    let split = ui.ctx().content_rect().width() > 1_020.0 && width >= 641.0;
    if split {
        let left = ((width - 1.0) * 0.45).clamp(280.0, width - 361.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                egui::vec2(left, 250.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| regression_waveform_chart(ui, pair),
            );
            ui.allocate_ui_with_layout(
                egui::vec2(width - left - 1.0, 250.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| regression_baseline_contract(ui, app, pair),
            );
        });
    } else {
        ui.allocate_ui(egui::vec2(width, 230.0), |ui| {
            regression_waveform_chart(ui, pair)
        });
        ui.add_space(1.0);
        regression_baseline_contract(ui, app, pair);
    }
    ui.add_space(1.0);
    let headers = vec![
        ("Check".to_owned(), 0.17),
        ("Comparison".to_owned(), 0.14),
        ("Baseline".to_owned(), 0.14),
        ("Current".to_owned(), 0.14),
        ("Delta".to_owned(), 0.14),
        ("Tolerance".to_owned(), 0.13),
        ("Status".to_owned(), 0.14),
    ];
    let rows = checks
        .iter()
        .map(|check| {
            vec![
                TableCell::text(&check.name),
                TableCell::text(format!("exact · {}", check.source_identity)),
                TableCell::mono(format_scalar(check.baseline)),
                TableCell::mono(format_scalar(check.current)),
                TableCell::mono(format!("{:+.6e}", check.delta())),
                TableCell::mono("not configured"),
                TableCell::tone(
                    if check.changed() {
                        "CHANGED · REVIEW"
                    } else {
                        "IDENTICAL"
                    },
                    if check.changed() {
                        t.color.warn
                    } else {
                        t.color.ok
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    table_section_header(
        ui,
        "Regression checks",
        Some("exact values · verdict requires configured tolerance"),
        None,
    );
    render_data_table(ui, "verify-regression-checks", &headers, &rows, None);
}

fn regression_baseline_contract(
    ui: &mut Ui,
    app: &RSpiceApp,
    pair: Option<(&crate::state::SimulationRun, &crate::state::SimulationRun)>,
) {
    table_section_header(ui, "Baseline contract", Some("immutable reference"), None);
    if let Some((baseline, current)) = pair {
        property_row(ui, "Baseline run", &format!("Run {}", baseline.id));
        property_row(ui, "Candidate run", &format!("Run {}", current.id));
        property_row(ui, "Baseline dataset", &baseline.dataset_id.to_string());
        property_row(
            ui,
            "Baseline provenance",
            if baseline.prepared_receipt().is_some() {
                "prepared snapshot retained"
            } else {
                "legacy / unclassified"
            },
        );
        let receipt = app
            .state
            .workbench
            .verification
            .regression_comparison
            .as_ref()
            .filter(|receipt| {
                receipt.baseline_run == baseline.run_id && receipt.candidate_run == current.run_id
            });
        property_row(
            ui,
            "Comparison receipt",
            &receipt.map_or_else(
                || "not run / stale".to_owned(),
                |receipt| {
                    format!(
                        "{} checks · {} waveforms · {} changed",
                        receipt.aligned_checks, receipt.aligned_waveforms, receipt.changed_checks
                    )
                },
            ),
        );
        property_row(ui, "Replacement policy", "explicit selection required");
    } else {
        let t = Tokens::get(ui.ctx());
        egui::Frame::new()
            .fill(t.color.bg_panel)
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                status_dot(ui, t.color.warn, "No governed baseline selected");
                ui.label(
                    "Retain two immutable runs, then select the baseline used for exact comparison.",
                );
            });
    }
}

fn regression_waveform_chart(
    ui: &mut Ui,
    pair: Option<(&crate::state::SimulationRun, &crate::state::SimulationRun)>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let height = ui.available_height().max(210.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    let header = egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + 36.0));
    ui.painter().rect_filled(header, 0.0, t.color.bg_panel);
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let header_left = egui::Rect::from_min_max(
        header.min,
        egui::pos2(header.left() + header.width() * 0.45, header.bottom()),
    );
    let header_right =
        egui::Rect::from_min_max(egui::pos2(header_left.right(), header.top()), header.max);
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title = elide_table_text(
        ui,
        "Waveform comparison",
        &title_font,
        (header_left.width() - 20.0).max(0.0),
    );
    ui.painter().with_clip_rect(header_left).text(
        header_left.left_center() + egui::vec2(10.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );
    let aligned = pair
        .map(|(baseline, current)| aligned_regression_waveforms(baseline, current))
        .unwrap_or_default();
    let Some((baseline, current)) = aligned.first().copied() else {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                "Waveform comparison: no exactly aligned retained waveform pair",
            )
        });
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_CENTER,
            "No exactly aligned waveform pair",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    };
    let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let detail = elide_table_text(
        ui,
        &format!("{} · tolerance not configured", current.name),
        &detail_font,
        (header_right.width() - 20.0).max(0.0),
    );
    ui.painter().with_clip_rect(header_right).text(
        header_right.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        detail,
        detail_font,
        t.color.text_faint,
    );
    let finite_values = baseline
        .y
        .iter()
        .chain(current.y.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    let Some(y_min) = finite_values.iter().copied().min_by(f64::total_cmp) else {
        return;
    };
    let Some(y_max) = finite_values.iter().copied().max_by(f64::total_cmp) else {
        return;
    };
    let finite_x = current
        .x
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    let Some(x_min) = finite_x.iter().copied().min_by(f64::total_cmp) else {
        return;
    };
    let Some(x_max) = finite_x.iter().copied().max_by(f64::total_cmp) else {
        return;
    };
    let plot = egui::Rect::from_min_max(
        header.left_bottom() + egui::vec2(10.0, 10.0),
        rect.right_bottom() - egui::vec2(10.0, 18.0),
    );
    for step in 0..=4 {
        let y = egui::lerp(plot.bottom()..=plot.top(), step as f32 / 4.0);
        ui.painter().hline(
            plot.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid.gamma_multiply(0.55)),
        );
    }
    let points = |x_values: &[f64], y_values: &[f64]| {
        x_values
            .iter()
            .zip(y_values)
            .filter_map(|(x_value, y_value)| {
                if !x_value.is_finite() || !y_value.is_finite() {
                    return None;
                }
                let x_fraction = if (x_max - x_min).abs() <= f64::EPSILON {
                    0.5
                } else {
                    ((*x_value - x_min) / (x_max - x_min)) as f32
                };
                let y_fraction = if (y_max - y_min).abs() <= f64::EPSILON {
                    0.5
                } else {
                    ((*y_value - y_min) / (y_max - y_min)) as f32
                };
                Some(egui::pos2(
                    egui::lerp(plot.left()..=plot.right(), x_fraction),
                    egui::lerp(plot.bottom()..=plot.top(), y_fraction),
                ))
            })
            .collect::<Vec<_>>()
    };
    ui.painter().add(egui::Shape::line(
        points(baseline.x.as_slice(), baseline.y.as_slice()),
        egui::Stroke::new(1.5, t.color.text_faint),
    ));
    ui.painter().add(egui::Shape::line(
        points(current.x.as_slice(), current.y.as_slice()),
        egui::Stroke::new(1.8, t.color.accent),
    ));
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Aligned regression waveform {} with {} exact samples; baseline and current traces shown without a tolerance verdict",
                current.name,
                current.y.len()
            ),
        )
    });
}

fn physical_drc(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Physical evidence contract", |ui| {
        status_dot(
            ui,
            Tokens::get(ui.ctx()).color.err,
            "No retained physical DRC evidence",
        );
        property_row(ui, "Layout source", "not attached");
        property_row(ui, "Qualified rule deck", "not attached");
        property_row(ui, "Immutable marker database", "unavailable");
        property_row(ui, "Sign-off eligibility", "blocked");
        ui.label("Schematic connectivity checks are never reclassified as geometry verification. Physical DRC remains fail-closed until a real layout source and qualified rule-deck execution produce attributable marker evidence.");
    });
    card(
        ui,
        "Source electrical checks · separate evidence domain",
        |ui| {
            let current =
                app.state.dialogs.drc_checked_version == app.state.schematic.topology_version();
            property_row(
                ui,
                "Schematic topology",
                if current {
                    "current"
                } else {
                    "stale / not run"
                },
            );
            if let Some(result) = &app.state.dialogs.drc_results {
                let summary = result.summary();
                property_row(ui, "Critical", &summary.critical.to_string());
                property_row(ui, "Errors", &summary.errors.to_string());
                property_row(ui, "Advisories", &summary.warnings.to_string());
            }
            if Button::new("Run source electrical checks")
                .show(ui)
                .clicked()
            {
                Command::RunChecks.execute(app);
                app.state.workbench.verification.action_receipt =
                "Source electrical checks completed. They remain separate from physical DRC evidence."
                    .to_owned();
            }
        },
    );
    action_receipt(ui, app);
}

#[cfg(test)]
fn active_dataset_measurement(
    simulation: &crate::state::SimulationState,
    name: &str,
) -> Option<f64> {
    simulation
        .active_run()
        .and_then(|run| measurement_in_run(run, name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationState};

    fn attributed(analysis: AnalysisResult) -> AnalysisResult {
        attributed_with_id(analysis, AnalysisInstanceId::new())
    }

    fn attributed_with_id(
        analysis: AnalysisResult,
        source_instance_id: AnalysisInstanceId,
    ) -> AnalysisResult {
        analysis.with_provenance(
            crate::state::AnalysisResultProvenance::new(
                source_instance_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
                Vec::new(),
            )
            .expect("test provenance is internally valid"),
        )
    }

    #[test]
    fn verify_layout_matches_desktop_tablet_and_phone_compositions() {
        let desktop = VerifyLayout::resolve(1_440.0, 820.0, 620.0, 5, 37.0, 28.0);
        assert!(desktop.split);
        assert!((desktop.left_width + 1.0 + desktop.right_width - 820.0).abs() < f32::EPSILON);
        assert!(desktop.first_row_height > 400.0);

        let compact_desktop = VerifyLayout::resolve(1_024.0, 744.0, 600.0, 5, 37.0, 28.0);
        assert!(compact_desktop.split);

        let tablet = VerifyLayout::resolve(820.0, 768.0, 600.0, 5, 44.0, 31.0);
        assert!(!tablet.split);
        assert_eq!(tablet.left_width, 768.0);

        let phone = VerifyLayout::resolve(390.0, 390.0, 540.0, 5, 44.0, 36.0);
        assert!(!phone.split);
        assert_eq!(phone.right_width, 390.0);
    }

    #[test]
    fn responsive_verify_geometry_matches_mockup_breakpoints() {
        assert_eq!(verification_status_columns(1_261.0), 4);
        assert_eq!(verification_status_columns(1_260.0), 2);
        assert_eq!(verification_status_columns(821.0), 2);
        assert_eq!(verification_status_columns(820.0), 2);
        assert_eq!(verification_status_columns(390.0), 2);

        assert_eq!(verification_stacked_chart_height(561.0), 250.0);
        assert_eq!(verification_stacked_chart_height(560.0), 230.0);

        assert_eq!(verification_table_row_height(1_440.0), 28.0);
        assert_eq!(verification_table_row_height(820.0), 31.0);
        assert_eq!(verification_table_row_height(560.0), 36.0);

        assert_eq!(verification_table_width(1_280.0, 718.0, None), 718.0);
        assert_eq!(verification_table_width(560.0, 390.0, None), 540.0);
        assert_eq!(
            verification_table_width(1_280.0, 718.0, Some(1_450.0)),
            1_450.0
        );
    }

    #[test]
    fn histogram_bins_preserve_every_exact_sample() {
        let samples = [0.0, 0.2, 0.4, 0.8, 1.0];
        let bins = histogram_bins(&samples, 4, 0.0, 1.0);

        assert_eq!(bins.iter().sum::<usize>(), samples.len());
        assert_eq!(bins, vec![2, 1, 0, 2]);
    }

    #[test]
    fn signed_margin_is_positive_inside_and_negative_outside_bounds() {
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            min: Some(10.0),
            max: Some(20.0),
            unit: "dB".to_owned(),
        };

        assert_eq!(signed_margin(&spec, 15.0), 5.0);
        assert_eq!(signed_margin(&spec, 8.0), -2.0);
        assert_eq!(signed_margin(&spec, 23.0), -3.0);
        assert_eq!(normalized_margin(&spec, 15.0), Some(0.5));
        assert_eq!(normalized_margin(&spec, 8.0), Some(-0.2));
    }

    #[test]
    fn joint_sample_summary_requires_aligned_trails_and_ands_every_spec() {
        let make_result = |target: &str, trail: Vec<bool>| {
            let pass_count = trail.iter().filter(|passes| **passes).count();
            crate::services::yield_manager::YieldResult {
                spec: crate::services::YieldSpec::lower(target, 0.0, ""),
                total_runs: trail.len(),
                pass_count,
                fail_count: trail.len() - pass_count,
                yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
                stats: crate::services::DistributionStats::default(),
                samples: vec![1.0; trail.len()],
                trail,
            }
        };
        let aligned = [
            make_result("gain", vec![true, false, false]),
            make_result("bandwidth", vec![true, false, true]),
        ];
        let joint = joint_sample_summary(&aligned).expect("aligned joint trail");
        assert_eq!(joint.passing, 1);
        assert_eq!(joint.total, 3);
        assert_eq!(joint.specification_count, 2);
        assert!((joint.yield_percent() - 100.0 / 3.0).abs() < 1.0e-12);
        assert_eq!(
            worst_individual_yield_result(&aligned)
                .expect("worst individual detail")
                .spec
                .target,
            "gain"
        );

        let misaligned = [
            make_result("gain", vec![true, true]),
            make_result("bandwidth", vec![true]),
        ];
        assert_eq!(joint_sample_summary(&misaligned), None);
    }

    #[test]
    fn joint_headline_can_be_lower_than_every_individual_spec_yield() {
        let make_result = |target: &str, trail: Vec<bool>| {
            let pass_count = trail.iter().filter(|passes| **passes).count();
            crate::services::yield_manager::YieldResult {
                spec: crate::services::YieldSpec::lower(target, 0.0, ""),
                total_runs: trail.len(),
                pass_count,
                fail_count: trail.len() - pass_count,
                yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
                stats: crate::services::DistributionStats::default(),
                samples: vec![1.0; trail.len()],
                trail,
            }
        };
        let results = [
            make_result("gain", vec![true, true, false, true]),
            make_result("bandwidth", vec![true, false, true, true]),
        ];

        let joint = joint_sample_summary(&results).expect("aligned joint trail");
        assert_eq!((joint.passing, joint.total), (2, 4));
        assert_eq!(joint.yield_percent(), 50.0);
        assert!(
            results
                .iter()
                .all(|result| result.yield_percent > joint.yield_percent())
        );
    }

    #[test]
    fn specification_editor_never_falls_through_to_an_inactive_dataset() {
        let mut inactive = SimulationRun::new(1);
        inactive.add_analysis(attributed(
            AnalysisResult::new(1, AnalysisType::Ac, "inactive")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 99.0)]),
        ));
        let mut active = SimulationRun::new(2);
        active.add_analysis(attributed(
            AnalysisResult::new(1, AnalysisType::Ac, "active")
                .with_measurements(vec![rspice_core::MeasureResult::success("bandwidth", 42.0)]),
        ));
        let active_dataset = active.dataset_id;
        let mut simulation = SimulationState {
            runs: vec![inactive, active],
            active_run_idx: Some(1),
            ..SimulationState::default()
        };

        assert_eq!(
            simulation.active_run().map(|run| run.dataset_id),
            Some(active_dataset)
        );
        assert_eq!(
            active_dataset_measurement(&simulation, "bandwidth"),
            Some(42.0)
        );
        assert_eq!(active_dataset_measurement(&simulation, "gain"), None);

        simulation.active_run_idx = None;
        assert_eq!(active_dataset_measurement(&simulation, "bandwidth"), None);
        assert_eq!(active_dataset_measurement(&simulation, "gain"), None);
    }

    #[test]
    fn verification_requires_an_explicit_active_run_selection() {
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.runs.push(SimulationRun::new(1));
        app.state.simulation.active_run_idx = None;

        assert_eq!(verification_run_index(&app), None);
        assert!(verification_run(&app).is_none());
    }

    #[test]
    fn measurement_evidence_requires_success_finite_value_and_provenance() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "legacy")
                .with_measurements(vec![rspice_core::MeasureResult::success("legacy", 1.0)]),
        );
        run.add_analysis(attributed(
            AnalysisResult::failed(2, AnalysisType::Ac, "failed", "solver failed")
                .with_measurements(vec![rspice_core::MeasureResult::success("failed", 2.0)]),
        ));
        run.add_analysis(attributed(
            AnalysisResult::new(3, AnalysisType::Ac, "measure failed").with_measurements(vec![
                rspice_core::MeasureResult::failed("measure_failed", "crossing not found"),
            ]),
        ));
        run.add_analysis(attributed(
            AnalysisResult::new(4, AnalysisType::Ac, "valid")
                .with_measurements(vec![rspice_core::MeasureResult::success("valid", 4.0)]),
        ));

        assert_eq!(measurement_in_run(&run, "legacy"), None);
        assert_eq!(measurement_in_run(&run, "failed"), None);
        assert_eq!(measurement_in_run(&run, "measure_failed"), None);
        assert_eq!(measurement_in_run(&run, "valid"), Some(4.0));
    }

    #[test]
    fn regression_evidence_uses_only_aligned_finite_measurements() {
        let source_instance_id = AnalysisInstanceId::new();
        let mut baseline = SimulationRun::new(38);
        baseline.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Ac, "baseline").with_measurements(vec![
                rspice_core::MeasureResult::success("gain", 40.0),
                rspice_core::MeasureResult::success("bandwidth", 100.0),
                rspice_core::MeasureResult::success("baseline_only", 1.0),
            ]),
            source_instance_id,
        ));
        let mut current = SimulationRun::new(41);
        current.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Ac, "candidate").with_measurements(vec![
                rspice_core::MeasureResult::success("GAIN", 40.4),
                rspice_core::MeasureResult::success("bandwidth", 103.0),
                rspice_core::MeasureResult::success("candidate_only", 2.0),
            ]),
            source_instance_id,
        ));

        let checks = derive_regression_checks(&baseline, &current);
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].name, "bandwidth");
        assert!(checks[0].changed());
        assert_eq!(checks[1].name, "gain");
        assert!(checks[1].changed());
    }

    #[test]
    fn regression_evidence_retains_finite_goal_misses() {
        let source_instance_id = AnalysisInstanceId::new();
        let mut baseline_measurement = rspice_core::MeasureResult::success("gain", 40.0);
        baseline_measurement.expected = Some(41.0);
        baseline_measurement.tolerance = Some(0.1);
        baseline_measurement.passed = false;
        baseline_measurement.error = Some("value misses GOAL".to_owned());
        let mut current_measurement = rspice_core::MeasureResult::success("gain", 40.2);
        current_measurement.expected = Some(41.0);
        current_measurement.tolerance = Some(0.1);
        current_measurement.passed = false;
        current_measurement.error = Some("value misses GOAL".to_owned());

        let mut baseline = SimulationRun::new(38);
        baseline.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Ac, "baseline")
                .with_measurements(vec![baseline_measurement]),
            source_instance_id,
        ));
        let mut current = SimulationRun::new(41);
        current.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Ac, "candidate")
                .with_measurements(vec![current_measurement]),
            source_instance_id,
        ));

        let checks = derive_regression_checks(&baseline, &current);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].baseline, 40.0);
        assert_eq!(checks[0].current, 40.2);
    }

    #[test]
    fn regression_waveforms_require_exact_analysis_name_axis_and_length_alignment() {
        let source_instance_id = AnalysisInstanceId::new();
        let mut baseline = SimulationRun::new(38);
        baseline.add_analysis(attributed_with_id(
            AnalysisResult::new(9, AnalysisType::Transient, "unrelated transient").with_waveforms(
                vec![crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 2.0, 4.0],
                    vec![0.0, 2.0, 4.0],
                    "#808080",
                )],
            ),
            AnalysisInstanceId::new(),
        ));
        baseline.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Transient, "baseline").with_waveforms(vec![
                crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 2.0],
                    "#ffffff",
                ),
            ]),
            source_instance_id,
        ));
        let mut current = SimulationRun::new(41);
        current.add_analysis(attributed_with_id(
            AnalysisResult::new(1, AnalysisType::Transient, "candidate").with_waveforms(vec![
                crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.1, 2.1],
                    "#ffbf00",
                ),
                crate::state::WaveformData::new(
                    "V(other)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 0.5, 1.0],
                    "#00ff00",
                ),
            ]),
            source_instance_id,
        ));

        let aligned = aligned_regression_waveforms(&baseline, &current);
        assert_eq!(aligned.len(), 1);
        assert_eq!(aligned[0].0.name, "V(out)");

        current.analyses[0].waveforms[0].x = std::sync::Arc::new(vec![0.0, 1.0, 2.5]);
        assert!(aligned_regression_waveforms(&baseline, &current).is_empty());
    }
}
