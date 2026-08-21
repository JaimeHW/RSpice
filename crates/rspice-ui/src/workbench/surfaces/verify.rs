//! Verification evidence, specifications, checks, reliability, and history.

mod tuning;

use egui::{ScrollArea, Ui};
use sha2::{Digest as _, Sha256};

use crate::state::SpecEntry;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{card, heading, property_row, status_dot, workspace_title_row};
use super::super::state::VerificationPage;
use tuning::{
    revert_tuning_session, sync_tuning_session, tuning, tuning_commit_block_reason,
    tuning_is_dirty, tuning_is_valid, tuning_review_dialog,
};

mod corners;
mod regression_contract;
mod regression_panel;

use corners::*;
use regression_contract::*;
use regression_panel::*;

const VERIFY_RESPONSIVE_BREAKPOINT: f32 = 820.0;
const VERIFY_KPI_BREAKPOINT: f32 = 1_260.0;
const VERIFY_PHONE_BREAKPOINT: f32 = 560.0;
const VERIFY_STACKED_CHART_HEIGHT: f32 = 250.0;
const VERIFY_PHONE_CHART_HEIGHT: f32 = 230.0;
const VERIFY_FIRST_ROW_MIN_HEIGHT: f32 = 210.0;
const VERIFY_FIRST_ROW_MAX_HEIGHT: f32 = 390.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    // Persisted legacy pages can outlive their retired prototype surfaces.
    // Fail closed before rendering so no unavailable workflow is exposed as
    // an interactive verification page.
    if !app.state.workbench.verification_page.is_operational() {
        app.state.workbench.verification_page = VerificationPage::Yield;
    }
    if app.state.workbench.verification_page == VerificationPage::Tuning {
        sync_tuning_session(app);
    }
    let ctx = ui.ctx().clone();
    let t = Tokens::get(ui.ctx());
    let viewport_height = ui.available_height();
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt((
                "workbench.verify.surface",
                app.state.workbench.verification_page.label(),
            ))
            .show(ui, |ui| {
                let surface_width = ui.available_width();
                let surface_top = ui.cursor().top();
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| {
                    verification_heading(ui, app);
                });
                let consumed_height = ui.cursor().top() - surface_top;
                let body_viewport_height =
                    remaining_viewport_height(viewport_height, consumed_height);
                match app.state.workbench.verification_page {
                    VerificationPage::Yield => cockpit(ui, app, body_viewport_height),
                    VerificationPage::Corners => corners(ui, app),
                    VerificationPage::Tuning => tuning(ui, app),
                    VerificationPage::Optimization => optimization(ui, app),
                    VerificationPage::Reliability => reliability(ui, app),
                    VerificationPage::Regression => regression(ui, app, body_viewport_height),
                    VerificationPage::Drc => physical_drc(ui, app),
                }
            });
    });
    regression_baseline_picker(&ctx, app);
    tuning_review_dialog(&ctx, app);
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
            "PARAMETER TUNER · NOMINAL SANDBOX · NON-DESTRUCTIVE".to_owned(),
            "Live design-space exploration",
            "Explore bounded active-plan variables without changing authoritative design data; review commits one plan revision and dispatches a retained production run.",
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
        VerificationPage::Tuning => 284.0,
        _ => 0.0,
    };
    if action_width > 0.0 && available > VERIFY_PHONE_BREAKPOINT {
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
        VerificationPage::Tuning => {
            let dirty = tuning_is_dirty(app);
            let valid = tuning_is_valid(app);
            let commit = Button::new("Review & commit changes").enabled(dirty && valid);
            let commit = if dirty && valid {
                commit.accent()
            } else {
                commit
            };
            if commit
                .show(ui)
                .on_disabled_hover_text(tuning_commit_block_reason(app))
                .clicked()
            {
                app.state.workbench.verification.tuning_review_open = true;
            }
            if Button::new("Revert to committed")
                .enabled(dirty)
                .show(ui)
                .clicked()
            {
                revert_tuning_session(app);
            }
        }
        VerificationPage::Yield
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

    let active_run = app.state.simulation.active_run();
    let active_run_id = active_run.map(|run| run.run_id);
    let mut candidates = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| {
            Some(run.run_id) != active_run_id
                && validate_regression_run(run).is_ok()
                && active_run.is_some_and(|candidate| {
                    validate_regression_run(candidate).is_ok()
                        && (!derive_regression_checks(run, candidate).is_empty()
                            || !regression_waveform_pairs(run, candidate).is_empty())
                })
        })
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

fn cockpit(ui: &mut Ui, app: &mut RSpiceApp, viewport_height: f32) {
    let surface_top = ui.cursor().top();
    let run_index = verification_run_index(app);
    let evidence = specification_evidence(app, run_index);
    engineering_status_strip(ui, app, &evidence);

    let width = ui.available_width();
    let viewport_width = ui.ctx().content_rect().width();
    let t = Tokens::get(ui.ctx());
    let header_height = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 37.0 };
    let visible_remaining =
        remaining_viewport_height(viewport_height, ui.cursor().top() - surface_top);
    let layout = VerifyLayout::resolve(
        viewport_width,
        width,
        visible_remaining,
        evidence.len(),
        header_height,
        verification_table_row_height(viewport_width),
    );
    if layout.split {
        // `Ui::horizontal` vertically centers children with different
        // intrinsic heights. That made the margin table drift toward the
        // middle of a tall, empty chart. Both panes own the same explicit
        // top-aligned row instead.
        let (row_rect, _) = ui.allocate_exact_size(
            egui::vec2(width, layout.first_row_height),
            egui::Sense::hover(),
        );
        let (left_rect, right_rect) = verification_split_rects(row_rect, layout.left_width);
        let mut left = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(left_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        left.set_clip_rect(left.clip_rect().intersect(left_rect));
        left.spacing_mut().item_spacing = egui::Vec2::ZERO;
        yield_chart(&mut left, app, layout.first_row_height);
        let mut right = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(right_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        right.set_clip_rect(right.clip_rect().intersect(right_rect));
        right.spacing_mut().item_spacing = egui::Vec2::ZERO;
        egui::ScrollArea::vertical()
            .id_salt("verify.cockpit.margin_matrix.vertical")
            .max_height(right_rect.height())
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(&mut right, |ui| {
                ui.set_min_width(right_rect.width());
                run_margin_matrix(ui, app, run_index, &evidence);
            });
        let divider_x = left_rect.right();
        ui.painter().vline(
            divider_x,
            row_rect.y_range(),
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

fn verification_split_rects(row_rect: egui::Rect, left_width: f32) -> (egui::Rect, egui::Rect) {
    let left_rect =
        egui::Rect::from_min_size(row_rect.min, egui::vec2(left_width, row_rect.height()));
    let right_rect = egui::Rect::from_min_max(
        egui::pos2(left_rect.right() + 1.0, row_rect.top()),
        row_rect.right_bottom(),
    );
    (left_rect, right_rect)
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
        let first_row_height =
            bounded_verification_first_row_height(visible_height, specification_height);
        let split = viewport_width > VERIFY_KPI_BREAKPOINT && content_width >= 641.0;
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

fn bounded_verification_first_row_height(visible_height: f32, specification_height: f32) -> f32 {
    let requested = visible_height - specification_height - 1.0;
    if requested.is_finite() {
        requested.clamp(VERIFY_FIRST_ROW_MIN_HEIGHT, VERIFY_FIRST_ROW_MAX_HEIGHT)
    } else {
        VERIFY_FIRST_ROW_MIN_HEIGHT
    }
}

fn remaining_viewport_height(viewport_height: f32, consumed_height: f32) -> f32 {
    (viewport_height - consumed_height.max(0.0)).max(0.0)
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

/// The plan whose limits this surface reads, when the plan is stable.
fn verification_plan_id(app: &RSpiceApp) -> Option<crate::product::SimulationPlanId> {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
}

/// How the selected dataset relates to the plan Verify is reading.
///
/// Classified through the one owner of that question, so this surface and the
/// studio's requirements page cannot reach different conclusions about the same
/// run.
fn verification_evidence_domain(app: &RSpiceApp) -> crate::state::EvidenceDomain {
    app.state
        .simulation
        .evidence_domain(verification_plan_id(app))
}

fn specification_evidence(app: &RSpiceApp, run_index: Option<usize>) -> Vec<SpecificationEvidence> {
    let plan_id = verification_plan_id(app);
    let domain = app.state.simulation.evidence_domain(plan_id);
    // A dataset another plan owns, or a manual deck no plan owns, is not this
    // plan's evidence. The studio's requirements page has always refused it;
    // Verify used to judge it anyway, so one surface reported a margin and the
    // other reported nothing for the same run and the same limit. The rows stay
    // — a limit with no evidence is exactly what has to be visible — but they
    // are answered by nothing, and the strip names the reason.
    let run = domain
        .answers_a_plan_limit()
        .then(|| run_index.and_then(|index| app.state.simulation.runs.get(index)))
        .flatten();
    let specs = plan_id.map_or_else(
        || app.state.workspace.specs.clone(),
        |plan_id| app.state.workspace.active_specs(plan_id).to_vec(),
    );
    specs
        .into_iter()
        .map(|spec| {
            let values = run
                .and_then(|run| {
                    worst_measurement_in_run(run, &spec)
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
            // When the selected dataset cannot answer this plan's limits, that
            // is the reason coverage is zero, and it is a different problem
            // from a plan whose analyses do not produce the measurements.
            // Saying which sends the engineer to the right place.
            verification_evidence_domain(app).refusal().map_or_else(
                || {
                    if evidence.is_empty() {
                        "No project specifications".to_owned()
                    } else {
                        format!(
                            "{} specifications lack active-run evidence",
                            evidence.len() - covered
                        )
                    }
                },
                str::to_owned,
            ),
            if covered == evidence.len() && !evidence.is_empty() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        sign_off_tile(app, &t),
    ];
    verification_kpi_strip(ui, &items);
}

/// Whether the evidence on this surface may be cited as sign-off.
///
/// A run that consumed a model which had not cleared its qualification gate
/// still produced real numbers, and this never hides or blocks them. It stamps
/// them, because the one thing that must not happen is an unqualified model's
/// output being carried into a sign-off package as though it were qualified.
fn sign_off_tile(app: &RSpiceApp, t: &Tokens) -> (String, String, String, egui::Color32) {
    let label = "Sign-off".to_owned();
    let Some(receipt) =
        verification_run(app).and_then(crate::state::SimulationRun::prepared_receipt)
    else {
        return (
            label,
            "Not assessed".to_owned(),
            "No prepared receipt for the active dataset".to_owned(),
            t.color.text_faint,
        );
    };
    let unqualified = receipt.unqualified_model_sources();
    if unqualified.is_empty() {
        return (
            label,
            "Eligible".to_owned(),
            "Every project model this run consumed was released".to_owned(),
            t.color.ok,
        );
    }
    let named = unqualified
        .iter()
        .take(3)
        .map(|identity| identity.model_name())
        .collect::<Vec<_>>()
        .join(", ");
    let detail = if unqualified.len() > 3 {
        format!("{named} and {} more are unqualified", unqualified.len() - 3)
    } else {
        format!("{named} unqualified at run time")
    };
    (label, "Not sign-off".to_owned(), detail, t.color.warn)
}

fn verification_kpi_strip(ui: &mut Ui, items: &[(String, String, String, egui::Color32)]) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let responsive_width = ui.ctx().content_rect().width();
    let columns = verification_status_columns(responsive_width);
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
    // Capture the viewport before entering the horizontal scroll area. Inside
    // it, `available_width()` is the unconstrained scroll content width and
    // previously stretched ordinary tables far past the visible surface.
    let viewport_width = ui.available_width().max(1.0);
    let responsive_width = ui.ctx().content_rect().width();
    let row_height = verification_table_row_height(responsive_width);
    egui::ScrollArea::horizontal()
        .id_salt(id)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
        .show(ui, |ui| {
            let width =
                verification_table_width(responsive_width, viewport_width, desktop_min_width);
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

fn render_virtual_data_table(
    ui: &mut Ui,
    id: &str,
    headers: &[(String, f32)],
    row_count: usize,
    rows_are_actionable: bool,
    mut row_at: impl FnMut(usize) -> Vec<TableCell>,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let mut clicked_row = None;
    let viewport_width = ui.available_width().max(1.0);
    let responsive_width = ui.ctx().content_rect().width();
    let row_height = verification_table_row_height(responsive_width);
    egui::ScrollArea::horizontal()
        .id_salt(id)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
        .show(ui, |ui| {
            let width = verification_table_width(responsive_width, viewport_width, None);
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

            if row_count == 0 {
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

            let visible_rows = row_count.min(12);
            egui::ScrollArea::vertical()
                .id_salt(format!("{id}.rows"))
                .max_height(row_height * visible_rows as f32)
                .auto_shrink([false, true])
                .show_rows(ui, row_height, row_count, |ui, range| {
                    for row_index in range {
                        let cells = row_at(row_index);
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(width, row_height),
                            if rows_are_actionable {
                                egui::Sense::click()
                            } else {
                                egui::Sense::hover()
                            },
                        );
                        response.widget_info(|| {
                            let widget_type = if rows_are_actionable {
                                egui::WidgetType::Button
                            } else {
                                egui::WidgetType::Label
                            };
                            egui::WidgetInfo::labeled(
                                widget_type,
                                ui.is_enabled(),
                                headers
                                    .iter()
                                    .zip(&cells)
                                    .map(|((header, _), cell)| format!("{header}: {}", cell.text))
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            )
                        });
                        if response.hovered() {
                            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                        }
                        if rows_are_actionable && response.clicked() {
                            clicked_row = Some(row_index);
                        }
                        paint_table_cells(ui, rect, headers, false, |label| {
                            let index = headers
                                .iter()
                                .position(|(header, _)| header == label)
                                .unwrap_or(0);
                            cells
                                .get(index)
                                .cloned()
                                .unwrap_or_else(|| TableCell::text("\u{2014}"))
                        });
                        ui.painter().hline(
                            rect.x_range(),
                            rect.bottom(),
                            egui::Stroke::new(1.0, t.color.border),
                        );
                        theme::paint_focus_ring(ui, &response, rect);
                    }
                });
        });
    clicked_row
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
        "verify-soa-rule-results" => "No evaluated SOA rules",
        "verify-reliability-device-results" => "No retained device-aging projections",
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

/// The worst measurement of a limit's name in a run, against that limit.
///
/// Verify used to take the first name-matching measurement it found in the
/// first attributed analysis. The studio's requirements page takes the worst of
/// every attributed measurement of that name, including the ones a result
/// family's own members retained, because a limit answered by one arbitrary
/// point of a sweep is not a verdict on the sweep. Two surfaces disagreeing
/// about which number answers one limit is not a display difference — one of
/// them is telling an engineer a specification passed while a retained point
/// failed it — so this reduces the same candidate set by the same rule.
fn worst_measurement_in_run(run: &crate::state::SimulationRun, spec: &SpecEntry) -> Option<f64> {
    run.analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
        .filter(|analysis| {
            analysis
                .provenance()
                .is_some_and(|provenance| spec.scope.admits(provenance.pvt_point()))
        })
        .flat_map(|analysis| {
            let analysis_level = analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&spec.measurement))
                .filter(|measurement| measurement.passed && measurement.error.is_none())
                .filter_map(|measurement| measurement.value);
            let member_level = analysis
                .family_metadata
                .iter()
                .flat_map(|metadata| metadata.member_measurements())
                .filter_map(|member| member.evidence_for(&spec.measurement))
                .filter(|evidence| evidence.is_measured())
                .filter_map(|evidence| evidence.value);
            analysis_level.chain(member_level)
        })
        .filter(|value| value.is_finite())
        // `total_cmp` on a set already filtered finite, so a tie cannot panic.
        .min_by(|left, right| {
            verification_margin(spec, *left).total_cmp(&verification_margin(spec, *right))
        })
}

/// How much room a value has before it breaks the limit. Negative once broken,
/// and more negative the worse the break, so the minimum is the worst point.
fn verification_margin(spec: &SpecEntry, value: f64) -> f64 {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => (value - minimum).min(maximum - value),
        (Some(minimum), None) => value - minimum,
        (None, Some(maximum)) => maximum - value,
        (None, None) => 0.0,
    }
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
    let mut plan_changed = false;
    let mut setup = app.state.sim_setup.clone();
    {
        let plan = setup.stable_analysis_plan_mut()?;
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
                    plan_changed = true;
                }
                id
            } else {
                plan_changed = true;
                plan.insert(*kind).map_err(|error| error.to_string())?.0
            };
            selected = Some((*kind, id));
        }
    }
    setup.refresh_legacy_analysis_projections();
    if plan_changed {
        app.state.sim_setup = setup;
        invalidate_plan_bound_preflight(app);
    }
    if let Some((kind, id)) = selected {
        app.state.workbench.active_analysis = kind.legacy_index();
        app.state.workbench.active_analysis_instance = Some(id);
    }
    crate::workbench::preflight::run_and_queue(app);
    if app.state.simulation.trigger_simulation {
        Ok(())
    } else {
        Err(
            "Execution was not started; review the Simulation preflight report and use its prerequisite repair action."
                .to_owned(),
        )
    }
}

fn open_analysis_configuration(
    app: &mut RSpiceApp,
    kind: crate::simulation::plan::AnalysisKind,
) -> Result<(), String> {
    let mut setup = app.state.sim_setup.clone();
    let (id, plan_changed) = {
        let plan = setup.stable_analysis_plan_mut()?;
        if let Some(instance) = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == kind)
        {
            (instance.id(), false)
        } else {
            (
                plan.insert(kind).map_err(|error| error.to_string())?.0,
                true,
            )
        }
    };
    setup.refresh_legacy_analysis_projections();
    if plan_changed {
        app.state.sim_setup = setup;
        invalidate_plan_bound_preflight(app);
    }
    app.state.workbench.active_analysis = kind.legacy_index();
    app.state.workbench.active_analysis_instance = Some(id);
    app.state
        .workbench
        .activate(super::super::state::Workspace::Simulate);
    Ok(())
}

/// Invalidate both visible preflight evidence and the controller's retained
/// one-use permit at the same boundary as an authoritative plan mutation.
/// Historical results remain immutable and are intentionally unaffected.
pub(super) fn invalidate_plan_bound_preflight(app: &mut RSpiceApp) {
    app.invalidate_simulation_preflight();
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

#[derive(Debug, Clone, PartialEq)]
struct RegressionCheck {
    target: crate::state::RegressionTargetSelector,
    name: String,
    source_identity: String,
    baseline: f64,
    current: f64,
}

#[derive(Debug, Clone)]
struct RegressionTargetObservation {
    target: crate::state::RegressionTargetSelector,
    validation_error: Option<String>,
}

#[derive(Debug, Clone)]
struct RegressionCoverageIssue {
    target: Option<crate::state::RegressionTargetSelector>,
    label: String,
    detail: String,
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
mod tests;
