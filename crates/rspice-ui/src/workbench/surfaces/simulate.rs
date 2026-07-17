//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod analysis_form;

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};

use crate::common::RSpiceApp;
use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDraft, AnalysisKind, AnalysisLifecycleCommand,
    AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::simulation::{SavedOutputSemanticStatus, SavedOutputStorageEstimate};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, IconButton, mono_input, select,
};
use crate::workbench::state::{
    ClonePlanDraft, DesignVariableDraft, SavedOutputDraft, SimulationWorkflowDialog,
};

use super::super::commands::Command;
use super::super::design_system::{
    WorkbenchIcon, heading, property_row, status_dot, workspace_title_row,
};

const SIMULATION_STACK_BREAKPOINT: f32 = 820.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const ANALYSIS_ROW_HEIGHT: f32 = 53.0;
const ANALYSIS_INDEX_DIAMETER: f32 = 22.0;
const ANALYSIS_ROW_LEFT_PADDING: f32 = 9.0;
const ANALYSIS_INDEX_LABEL_GAP: f32 = 8.0;
const ANALYSIS_ROW_TRAILING_PADDING: f32 = 9.0;
const ANALYSIS_LABEL_MIN_WIDTH: f32 = 64.0;
const ANALYSIS_SWITCH_WIDTH: f32 = 30.0;
const ANALYSIS_SWITCH_MAX_HIT_WIDTH: f32 = 44.0;
const ANALYSIS_SWITCH_LABEL_GAP: f32 = 7.0;
const ANALYSIS_STACK_TABLET_MIN_WIDTH: f32 = 175.0;
const ANALYSIS_STACK_DESKTOP_MIN_WIDTH: f32 = 190.0;
const ANALYSIS_EDITOR_TABLET_MIN_WIDTH: f32 = 330.0;
const ANALYSIS_EDITOR_DESKTOP_MIN_WIDTH: f32 = 360.0;
const PREFLIGHT_CELL_HEIGHT: f32 = 42.0;
const STACKED_WORKSPACE_GAP: f32 = 9.0;
const SETUP_CARD_HEADER_HEIGHT: f32 = 37.0;
const SETUP_TABLE_HEADER_HEIGHT: f32 = 27.0;
const ANALYSIS_CATALOG_GROUP_HEIGHT: f32 = 29.0;
const ANALYSIS_CATALOG_ROW_HEIGHT: f32 = 57.0;
const ANALYSIS_CATALOG_READINESS_WIDTH: f32 = 142.0;
const ANALYSIS_CATEGORY_ORDER: [&str; 10] = [
    "Core analyses",
    "Transfer & linearization",
    "RF & periodic solvers",
    "Periodic small-signal",
    "Quasi-periodic small-signal",
    "Time-domain stochastic",
    "Sweeps & variation",
    "Measurements",
    "Verification checks",
    "Optimization workflow",
];

#[derive(Clone)]
struct SelectedAnalysis {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    draft: AnalysisDraft,
    dependencies: Vec<AnalysisDependency>,
    lifecycle: AnalysisLifecycleState,
    position: usize,
    plan_length: usize,
    issues: Vec<AnalysisPlanIssue>,
}

#[derive(Clone)]
struct AnalysisStackRow {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    enabled: bool,
    lifecycle: AnalysisLifecycleState,
    summary: String,
    issue_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct AnalysisCatalogRowLayout {
    height: f32,
    draw_bottom_border: bool,
    draw_right_border: bool,
}

#[derive(Debug, Clone, Copy)]
enum AnalysisAction {
    Clone,
    Earlier(usize),
    Later(usize),
    BindDependencies,
    Validate,
    Remove,
    SetEnabled(bool),
}

#[derive(Debug, Clone, Copy)]
enum StackAction {
    Select(AnalysisInstanceId),
    SetEnabled(AnalysisInstanceId, bool),
    Insert(AnalysisKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetupCardBorder {
    All,
    SplitLeft,
    SplitRight,
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if let Err(error) = resolve_active_analysis_instance(app) {
        record_failure(app, "Analysis selection", &error);
    }

    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Resolve the content width after the scroll area has reserved
                // its solid scrollbar track. Reusing the outer width clips the
                // right edge beneath that track.
                let surface_width = ui.available_width();
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| plan_heading(ui, app, surface_width));
                analysis_workspace(ui, app, surface_width);
            });
    });
    simulation_workflow_dialog(ui.ctx(), app);
}

fn analysis_workspace(ui: &mut Ui, app: &mut RSpiceApp, surface_width: f32) {
    let viewport_width = ui.ctx().content_rect().width();
    let responsive_width = viewport_width.min(surface_width);
    if analysis_workspace_is_split(responsive_width, surface_width) {
        let divider = 1.0;
        let available = ui.available_width();
        let (left_width, right_width) = analysis_split_widths(available, responsive_width);
        let column_min_height =
            analysis_column_min_height(ui.clip_rect().bottom(), ui.cursor().top());
        let stack_background = ui.painter().add(egui::Shape::Noop);
        let row = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(vec2(left_width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_min_height(column_min_height);
                ordered_instance_stack(ui, app)
            });
            ui.allocate_ui_with_layout(
                vec2(right_width, column_min_height),
                Layout::top_down(Align::Min),
                |ui| analysis_editor(ui, app, responsive_width),
            );
        });
        ui.painter().set(
            stack_background,
            egui::Shape::rect_filled(
                analysis_stack_background_rect(row.response.rect, left_width),
                0.0,
                Tokens::get(ui.ctx()).color.bg_panel,
            ),
        );
    } else {
        ordered_instance_stack(ui, app);
        ui.add_space(STACKED_WORKSPACE_GAP);
        analysis_editor(ui, app, responsive_width);
    }
}

fn analysis_workspace_is_split(viewport_width: f32, surface_width: f32) -> bool {
    viewport_width > SIMULATION_STACK_BREAKPOINT
        && surface_width >= analysis_split_min_width(viewport_width)
}

fn analysis_split_widths(available: f32, viewport_width: f32) -> (f32, f32) {
    let usable = (available - 1.0).max(1.0);
    let (left_fraction, left_min, right_min) = analysis_column_constraints(viewport_width);
    let left = (usable * left_fraction)
        .max(left_min)
        .min((usable - right_min).max(left_min));
    (left, (usable - left).max(1.0))
}

fn analysis_column_constraints(viewport_width: f32) -> (f32, f32, f32) {
    if viewport_width <= 1_020.0 {
        (
            0.29,
            ANALYSIS_STACK_TABLET_MIN_WIDTH.max(analysis_row_content_min_width()),
            ANALYSIS_EDITOR_TABLET_MIN_WIDTH,
        )
    } else {
        (
            0.34,
            ANALYSIS_STACK_DESKTOP_MIN_WIDTH.max(analysis_row_content_min_width()),
            ANALYSIS_EDITOR_DESKTOP_MIN_WIDTH,
        )
    }
}

fn analysis_split_min_width(viewport_width: f32) -> f32 {
    let (_, rail_min, editor_min) = analysis_column_constraints(viewport_width);
    rail_min + 1.0 + editor_min
}

const fn analysis_row_content_min_width() -> f32 {
    ANALYSIS_ROW_LEFT_PADDING
        + ANALYSIS_INDEX_DIAMETER
        + ANALYSIS_INDEX_LABEL_GAP
        + ANALYSIS_LABEL_MIN_WIDTH
        + ANALYSIS_SWITCH_LABEL_GAP
        + ANALYSIS_SWITCH_MAX_HIT_WIDTH
        + ANALYSIS_ROW_TRAILING_PADDING
}

fn analysis_column_min_height(clip_bottom: f32, content_top: f32) -> f32 {
    (clip_bottom - content_top).max(1.0)
}

fn analysis_stack_background_rect(row_rect: Rect, left_width: f32) -> Rect {
    Rect::from_min_max(
        row_rect.min,
        egui::pos2(
            (row_rect.left() + left_width).min(row_rect.right()),
            row_rect.bottom(),
        ),
    )
}

fn ordered_instance_stack(ui: &mut Ui, app: &mut RSpiceApp) {
    let rows = match analysis_stack_rows(app) {
        Ok(rows) => rows,
        Err(error) => {
            flat_notice(ui, |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            });
            return;
        }
    };
    let active = app.state.workbench.active_analysis_instance;
    let mut action = None;
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.set_width(ui.available_width());
        if rows.is_empty() {
            flat_notice(ui, |ui| {
                status_dot(ui, t.color.warn, "No analysis instances");
                ui.label("Add an analysis from the Simulation Studio navigator.");
            });
        } else {
            let mut displayed_position = 0usize;
            for group in ANALYSIS_CATEGORY_ORDER {
                let members = rows
                    .iter()
                    .filter(|row| analysis_catalog_group(row.kind) == group)
                    .collect::<Vec<_>>();
                if members.is_empty() {
                    continue;
                }
                analysis_group_header(ui, group, members.len());
                for row in members {
                    displayed_position += 1;
                    if let Some(row_action) =
                        analysis_stack_row(ui, row, displayed_position, active == Some(row.id))
                    {
                        action = Some(row_action);
                    }
                }
            }
        }
    });

    analysis_catalog_window(ui.ctx(), app, &rows, &mut action);

    match action {
        Some(StackAction::Select(id)) => {
            app.state.workbench.active_analysis_instance = Some(id);
        }
        Some(StackAction::SetEnabled(id, enabled)) => {
            apply_analysis_action(app, id, AnalysisAction::SetEnabled(enabled));
        }
        Some(StackAction::Insert(kind)) => insert_analysis_instance(app, kind),
        None => {}
    }
}

fn flat_notice(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let response = egui::Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            // `Frame` otherwise shrinks to the labels inside it. The mockup's
            // empty-state rails are full-column surfaces, so preserve the
            // containing column width before laying out wrapped copy.
            ui.set_width((width - 20.0).max(1.0));
            ui.spacing_mut().item_spacing.y = 3.0;
            add_contents(ui);
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn analysis_group_header(ui: &mut Ui, label: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
    ui.painter().text(
        rect.right_center() - vec2(10.0, 0.0),
        Align2::RIGHT_CENTER,
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn analysis_stack_row(
    ui: &mut Ui,
    row: &AnalysisStackRow,
    position: usize,
    selected: bool,
) -> Option<StackAction> {
    let t = Tokens::get(ui.ctx());
    let row_height = ANALYSIS_ROW_HEIGHT;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("Select {} analysis instance {}", row.kind.label(), row.id),
        )
    });
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let fill = if selected {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    painter.rect_filled(rect, 0.0, fill);
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    if selected {
        painter.vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }

    let opacity = if row.enabled { 1.0 } else { 0.72 };
    let index_center = egui::pos2(
        rect.left() + ANALYSIS_ROW_LEFT_PADDING + ANALYSIS_INDEX_DIAMETER * 0.5,
        rect.top() + 20.0,
    );
    painter.circle_filled(
        index_center,
        ANALYSIS_INDEX_DIAMETER * 0.5,
        if selected {
            t.color.accent_dim
        } else {
            t.color.bg_inset
        },
    );
    painter.circle_stroke(
        index_center,
        ANALYSIS_INDEX_DIAMETER * 0.5,
        Stroke::new(
            1.0,
            if selected {
                t.color.accent
            } else {
                t.color.border
            },
        ),
    );
    painter.text(
        index_center,
        Align2::CENTER_CENTER,
        format!("{position:02}"),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_faint.gamma_multiply(opacity)
        },
    );

    let switch_hit_size = if t.metrics.ctl_h >= ANALYSIS_SWITCH_MAX_HIT_WIDTH {
        ANALYSIS_SWITCH_MAX_HIT_WIDTH
    } else {
        ANALYSIS_SWITCH_WIDTH
    };
    let switch_hit = Rect::from_center_size(
        egui::pos2(
            rect.right() - ANALYSIS_ROW_TRAILING_PADDING - switch_hit_size * 0.5,
            rect.center().y,
        ),
        Vec2::splat(switch_hit_size),
    );
    let toggle = ui.interact(switch_hit, response.id.with("enabled"), Sense::click());
    toggle.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            row.enabled,
            format!("Enable {} instance {}", row.kind.label(), row.id),
        )
    });
    paint_switch(ui, switch_hit.center(), row.enabled, toggle.hovered(), rect);

    let text_left = rect.left()
        + ANALYSIS_ROW_LEFT_PADDING
        + ANALYSIS_INDEX_DIAMETER
        + ANALYSIS_INDEX_LABEL_GAP;
    let text_right = switch_hit.left() - ANALYSIS_SWITCH_LABEL_GAP;
    let first_line = format!(
        "{} · {}",
        row.kind.stable_id().to_uppercase(),
        row.kind.label()
    );
    let second_line = format!("{} · {}", row.id, row.summary);
    let (status, status_color) = if row.issue_count > 0 {
        ("dependency blocked", t.color.err)
    } else {
        let color = if availability_label(row.kind) == "Production" {
            t.color.ok
        } else {
            t.color.warn
        };
        (availability_label(row.kind), color)
    };
    let line_top = rect.top() + 7.0;
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top),
            egui::pos2(text_right, line_top + 13.0),
        ),
        &first_line,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text.gamma_multiply(opacity),
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top + 14.0),
            egui::pos2(text_right, line_top + 27.0),
        ),
        &second_line,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint.gamma_multiply(opacity),
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top + 28.0),
            egui::pos2(text_right, rect.bottom() - 3.0),
        ),
        &format!("{status} · {}", row.lifecycle),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        status_color.gamma_multiply(opacity),
    );
    // The canonical rail hides horizontal overflow. Keep keyboard focus
    // indicators inside the row as well, so they cannot bleed through the
    // one-point pane divider into the analysis editor.
    theme::paint_focus_ring(ui, &response, rect);
    theme::paint_focus_ring(ui, &toggle, switch_hit.intersect(rect));

    if toggle.clicked() {
        Some(StackAction::SetEnabled(row.id, !row.enabled))
    } else if response.clicked() {
        Some(StackAction::Select(row.id))
    } else {
        None
    }
}

fn paint_switch(ui: &Ui, center: egui::Pos2, enabled: bool, hovered: bool, row_rect: Rect) {
    let t = Tokens::get(ui.ctx());
    let rect = Rect::from_center_size(center, vec2(ANALYSIS_SWITCH_WIDTH, 17.0));
    let fill = if enabled {
        t.color.accent
    } else if hovered {
        t.color.bg_hover
    } else {
        t.color.bg_inset
    };
    let painter = ui
        .painter()
        .with_clip_rect(row_rect.intersect(ui.clip_rect()));
    painter.rect(
        rect,
        8.5,
        fill,
        Stroke::new(
            1.0,
            if enabled {
                t.color.accent
            } else {
                t.color.border_strong
            },
        ),
        egui::StrokeKind::Inside,
    );
    let knob_x = if enabled {
        rect.right() - 7.5
    } else {
        rect.left() + 7.5
    };
    painter.circle_filled(
        egui::pos2(knob_x, rect.center().y),
        5.5,
        if enabled {
            t.color.accent_ink
        } else {
            t.color.text_dim
        },
    );
}

fn paint_clipped_text(ui: &Ui, rect: Rect, text: &str, font: egui::FontId, color: Color32) {
    let clipped = rect.intersect(ui.clip_rect());
    if !clipped.is_positive() {
        return;
    }
    ui.painter().with_clip_rect(clipped).text(
        clipped.left_center(),
        Align2::LEFT_CENTER,
        text,
        font,
        color,
    );
}

fn analysis_stack_rows(app: &RSpiceApp) -> Result<Vec<AnalysisStackRow>, String> {
    let setup = &app.state.sim_setup;
    let plan = setup.stable_analysis_plan()?;
    let issues = plan.validation_issues();
    Ok(plan
        .instances()
        .iter()
        .map(|instance| AnalysisStackRow {
            id: instance.id(),
            kind: instance.kind(),
            enabled: instance.enabled(),
            lifecycle: instance.lifecycle(),
            summary: setup.analysis_draft_summary(instance.draft()),
            issue_count: issues
                .iter()
                .filter(|issue| issue_applies_to(issue, instance.id()))
                .count(),
        })
        .collect())
}

fn analysis_catalog_window(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    rows: &[AnalysisStackRow],
    action: &mut Option<StackAction>,
) {
    if !app.state.sim_setup.palette_open {
        return;
    }

    let mut query = app.state.sim_setup.palette_query.clone();
    let mut active = app.state.sim_setup.palette_active;
    let mut chosen = None;
    let mut request_close = false;
    let scroll_to_active = app.state.sim_setup.palette_scroll_to_active;
    let catalog_columns = analysis_catalog_column_count(ctx.content_rect().width());
    let choice = Dialog::new("Simulation Studio", "Add analysis or workflow", "Close")
        .description(
            "Search and add an explicitly classified solver, run-set controller, measurement, check, or optimization workflow.",
        )
        .size(DialogSize::AnalysisCatalog)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_on_enter(false)
        .flush_body()
        .manual_body_scroll()
        .hint(
            "Solvers, run-set controllers, measurements, checks, and optimization workflows are classified explicitly.",
        )
        .note_only_footer()
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            let mut search_id = None;
            egui::Frame::NONE
                .fill(t.color.bg_inset)
                .show(ui, |ui| {
                    let width = ui.available_width();
                    ui.allocate_ui_with_layout(
                        vec2(width, 48.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = 9.0;
                            ui.add_space(12.0);
                            let (icon_rect, _) =
                                ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                            WorkbenchIcon::Search.paint(
                                ui.painter(),
                                icon_rect,
                                t.color.text_dim,
                            );
                            let keycap_font = theme::mono(tokens::FS_0, FontWeight::Regular);
                            let keycap_width = (ui
                                .painter()
                                .layout_no_wrap(
                                    "Esc".to_owned(),
                                    keycap_font.clone(),
                                    t.color.text_dim,
                                )
                                .size()
                                .x
                                + 10.0)
                                .max(19.0);
                            let input_width =
                                (ui.available_width() - keycap_width - 21.0).max(1.0);
                            let search = ui.add_sized(
                                vec2(input_width, 48.0),
                                analysis_catalog_search_field(&mut query),
                            );
                            search_id = Some(search.id);
                            if search.changed() {
                                active = 0;
                            }
                            let (keycap_rect, _) = ui.allocate_exact_size(
                                vec2(keycap_width, 18.0),
                                Sense::hover(),
                            );
                            ui.painter().rect_filled(
                                keycap_rect,
                                3.0,
                                t.color.bg_panel_2,
                            );
                            ui.painter().rect_stroke(
                                keycap_rect,
                                3.0,
                                Stroke::new(1.0, t.color.border_strong),
                                egui::StrokeKind::Inside,
                            );
                            ui.painter().text(
                                keycap_rect.center(),
                                Align2::CENTER_CENTER,
                                "Esc",
                                keycap_font,
                                t.color.text_dim,
                            );
                            ui.add_space(12.0);
                        },
                    );
                });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );

            let filtered = filtered_catalog_kinds(&query);
            if filtered.is_empty() {
                active = 0;
            } else {
                active = active.min(filtered.len() - 1);
                if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
                    active = (active + 1).min(filtered.len() - 1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
                    active = active.saturating_sub(1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                    chosen = filtered
                        .get(active)
                        .copied()
                        .filter(|kind| kind.execution_blocker().is_none());
                }
            }

            let results_height = ui.available_height().max(1.0);
            egui::Frame::NONE.fill(t.color.bg_app).show(ui, |ui| {
                ui.set_min_height(results_height);
                ScrollArea::vertical()
                    .id_salt("workbench.simulate.analysis_catalog.rows")
                    .auto_shrink([false, false])
                    .max_height(results_height)
                    .min_scrolled_height(results_height)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        if filtered.is_empty() {
                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new("No analysis matches this search.")
                                    .color(Tokens::get(ui.ctx()).color.text_dim),
                            );
                            return;
                        }
                        let mut rendered_groups = 0;
                        for group in ANALYSIS_CATEGORY_ORDER {
                            let members = filtered
                                .iter()
                                .copied()
                                .enumerate()
                                .filter(|(_, kind)| analysis_catalog_group(*kind) == group)
                                .collect::<Vec<_>>();
                            if members.is_empty() {
                                continue;
                            }
                            analysis_catalog_group_header(
                                ui,
                                group,
                                members.len(),
                                rendered_groups > 0,
                            );
                            rendered_groups += 1;
                            if let Some(kind) = analysis_catalog_group_rows(
                                ui,
                                &members,
                                rows,
                                active,
                                scroll_to_active,
                                catalog_columns,
                            ) {
                                chosen = Some(kind);
                            }
                        }
                    });
            });
            search_id
        });

    if choice == DialogChoice::Cancelled {
        request_close = true;
    }

    if let Some(kind) = chosen {
        *action = Some(StackAction::Insert(kind));
        request_close = true;
    }
    if request_close {
        app.state.sim_setup.palette_open = false;
    }
    app.state.sim_setup.palette_query = query;
    app.state.sim_setup.palette_active = active;
    app.state.sim_setup.palette_scroll_to_active = false;
}

fn analysis_catalog_search_field(query: &mut String) -> egui::TextEdit<'_> {
    egui::TextEdit::singleline(query)
        .id_source("workbench.simulate.analysis_catalog.search")
        .font(theme::sans(tokens::FS_2, FontWeight::Regular))
        .hint_text("Search solvers, sweeps, measurements, checks…")
        .vertical_align(Align::Center)
        .frame(egui::Frame::NONE)
}

const fn analysis_catalog_column_count(viewport_width: f32) -> usize {
    if viewport_width >= 1_200.0 { 2 } else { 1 }
}

fn analysis_catalog_group_rows(
    ui: &mut Ui,
    members: &[(usize, AnalysisKind)],
    rows: &[AnalysisStackRow],
    active: usize,
    scroll_to_active: bool,
    columns: usize,
) -> Option<AnalysisKind> {
    let mut chosen = None;
    let chunk_count = members.len().div_ceil(columns);
    for (chunk_index, chunk) in members.chunks(columns).enumerate() {
        let draw_bottom_border = chunk_index + 1 < chunk_count;
        if columns == 1 {
            let (index, kind) = chunk[0];
            let disposition = analysis_catalog_disposition(rows, kind);
            let row_height =
                analysis_catalog_row_height(ui, kind, &disposition, ui.available_width());
            if analysis_catalog_row(
                ui,
                kind,
                &disposition,
                index == active,
                scroll_to_active,
                AnalysisCatalogRowLayout {
                    height: row_height,
                    draw_bottom_border,
                    draw_right_border: false,
                },
            ) {
                chosen = Some(kind);
            }
            continue;
        }

        let gap = 0.0;
        let column_width = ((ui.available_width() - gap) / 2.0).max(1.0);
        let row_height = chunk
            .iter()
            .map(|&(_, kind)| {
                let disposition = analysis_catalog_disposition(rows, kind);
                analysis_catalog_row_height(ui, kind, &disposition, column_width)
            })
            .fold(ANALYSIS_CATALOG_ROW_HEIGHT, f32::max);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (column, &(index, kind)) in chunk.iter().enumerate() {
                let disposition = analysis_catalog_disposition(rows, kind);
                ui.allocate_ui_with_layout(
                    vec2(column_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(column_width);
                        if analysis_catalog_row(
                            ui,
                            kind,
                            &disposition,
                            index == active,
                            scroll_to_active,
                            AnalysisCatalogRowLayout {
                                height: row_height,
                                draw_bottom_border,
                                draw_right_border: column == 0,
                            },
                        ) {
                            chosen = Some(kind);
                        }
                    },
                );
            }
            if chunk.len() == 1 {
                ui.allocate_exact_size(vec2(column_width, row_height), Sense::hover());
            }
        });
    }
    chosen
}

fn analysis_catalog_disposition(rows: &[AnalysisStackRow], kind: AnalysisKind) -> String {
    if kind.execution_blocker().is_some() {
        return "Unavailable".to_owned();
    }
    let configured = rows.iter().filter(|row| row.kind == kind).count();
    if configured == 0 {
        "Add instance".to_owned()
    } else {
        format!("Add another · {configured} in plan")
    }
}

fn analysis_catalog_group_header(ui: &mut Ui, group: &str, count: usize, has_predecessor: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ANALYSIS_CATALOG_GROUP_HEIGHT),
        Sense::hover(),
    );
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    if has_predecessor {
        painter.hline(
            rect.x_range(),
            rect.top(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    painter.text(
        rect.left_center() + vec2(12.0, 0.0),
        Align2::LEFT_CENTER,
        group.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    painter.text(
        rect.right_center() - vec2(12.0, 0.0),
        Align2::RIGHT_CENTER,
        count,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

const fn analysis_catalog_kind_label(kind: AnalysisKind) -> &'static str {
    match kind {
        AnalysisKind::MonteCarlo
        | AnalysisKind::Temperature
        | AnalysisKind::Corner
        | AnalysisKind::DcMismatch => "Run-set controller",
        AnalysisKind::Fourier | AnalysisKind::Disto => "Derived measurement",
        AnalysisKind::Reliability | AnalysisKind::Soa => "Verification workspace",
        AnalysisKind::Optimization => "Optimization workspace",
        _ => "Numerical solver",
    }
}

fn analysis_catalog_detail_galley(
    ui: &Ui,
    kind: AnalysisKind,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = width.max(1.0);
    job.wrap.max_rows = 2;
    job.append(
        &analysis_catalog_kind_label(kind).to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.04 * tokens::FS_0,
            ..Default::default()
        },
    );
    job.append(
        "  ·  ",
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            ..Default::default()
        },
    );
    job.append(
        kind.detail(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_dim,
            ..Default::default()
        },
    );
    ui.painter().layout_job(job)
}

fn analysis_catalog_row_height(
    ui: &Ui,
    kind: AnalysisKind,
    disposition: &str,
    row_width: f32,
) -> f32 {
    let t = Tokens::get(ui.ctx());
    let compact = row_width <= TITLE_ACTION_STACK_BREAKPOINT;
    let code_right = if compact { 54.0 } else { 70.0 };
    let copy_left = code_right + if compact { 9.0 } else { 12.0 };
    let (copy_width, readiness_width) = if compact {
        (
            (row_width - 10.0 - copy_left).max(1.0),
            (row_width - 10.0 - copy_left).max(1.0),
        )
    } else {
        let readiness_left =
            (row_width - 12.0 - ANALYSIS_CATALOG_READINESS_WIDTH).max(copy_left + 96.0);
        (
            (readiness_left - 12.0 - copy_left).max(1.0),
            (row_width - 12.0 - readiness_left - 24.0).max(1.0),
        )
    };
    let title = ui.painter().layout(
        kind.label().to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        copy_width,
    );
    let detail = analysis_catalog_detail_galley(ui, kind, copy_width);
    let copy_height = title.size().y + 3.0 + detail.size().y;
    let action = ui.painter().layout(
        disposition.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        readiness_width,
    );
    let readiness_detail = analysis_catalog_readiness(kind).map(|detail| {
        ui.painter().layout(
            detail.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
            readiness_width,
        )
    });
    let readiness_height = action.size().y
        + readiness_detail
            .as_ref()
            .map_or(0.0, |detail| 2.0 + detail.size().y);
    if compact {
        (16.0 + copy_height + 5.0 + readiness_height)
            .max(ANALYSIS_CATALOG_ROW_HEIGHT)
            .ceil()
    } else {
        (14.0 + copy_height.max(readiness_height))
            .max(ANALYSIS_CATALOG_ROW_HEIGHT)
            .ceil()
    }
}

fn analysis_catalog_row(
    ui: &mut Ui,
    kind: AnalysisKind,
    disposition: &str,
    selected: bool,
    scroll_to_active: bool,
    layout: AnalysisCatalogRowLayout,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= TITLE_ACTION_STACK_BREAKPOINT;
    let blocker = kind.execution_blocker();
    let enabled = ui.is_enabled() && blocker.is_none();
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), layout.height), sense);
    if selected && scroll_to_active {
        let reveal = Rect::from_min_max(
            egui::pos2(rect.left(), rect.top() - ANALYSIS_CATALOG_GROUP_HEIGHT),
            rect.max,
        );
        ui.scroll_to_rect(reveal, Some(Align::Min));
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            if let Some(reason) = blocker {
                format!("{} unavailable: {reason}", kind.label())
            } else {
                format!("Add {} analysis instance", kind.label())
            },
        )
    });
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(
        rect,
        0.0,
        if selected {
            t.color.bg_panel
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_app
        },
    );
    if layout.draw_bottom_border {
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
    }
    if layout.draw_right_border {
        painter.vline(
            rect.right() - 0.5,
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    if selected {
        painter.vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }

    let code_right = rect.left() + if compact { 54.0 } else { 70.0 };
    painter.vline(
        code_right,
        rect.y_range().shrink(7.0),
        Stroke::new(1.0, t.color.border),
    );
    painter.text(
        rect.left_center() + vec2(12.0, 0.0),
        Align2::LEFT_CENTER,
        kind.code(),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.accent,
    );

    let copy_left = code_right + if compact { 9.0 } else { 12.0 };
    let (copy_right, readiness_left) = if compact {
        (rect.right() - 10.0, copy_left)
    } else {
        let readiness_left =
            (rect.right() - 12.0 - ANALYSIS_CATALOG_READINESS_WIDTH).max(copy_left + 96.0);
        (readiness_left - 12.0, readiness_left)
    };
    let copy_width = (copy_right - copy_left).max(1.0);
    let title_galley = ui.painter().layout(
        kind.label().to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        copy_width,
    );
    let detail_galley = analysis_catalog_detail_galley(ui, kind, copy_width);
    let copy_top = rect.top() + if compact { 8.0 } else { 7.0 };
    let copy_clip = Rect::from_min_max(
        egui::pos2(copy_left, copy_top),
        egui::pos2(copy_right, rect.bottom() - if compact { 8.0 } else { 7.0 }),
    );
    let copy_painter = painter.with_clip_rect(copy_clip);
    copy_painter.galley(
        egui::pos2(copy_left, copy_top),
        title_galley.clone(),
        t.color.text,
    );
    copy_painter.galley(
        egui::pos2(copy_left, copy_top + title_galley.size().y + 3.0),
        detail_galley.clone(),
        t.color.text_dim,
    );

    if !compact {
        painter.vline(
            readiness_left,
            rect.y_range().shrink(7.0),
            Stroke::new(1.0, t.color.border),
        );
    }
    let readiness = analysis_catalog_readiness(kind);
    let copy_height = title_galley.size().y + 3.0 + detail_galley.size().y;
    let readiness_top = if compact {
        copy_top + copy_height + 5.0
    } else {
        rect.top() + 8.0
    };
    let readiness_content_left = if compact {
        readiness_left
    } else {
        readiness_left + 12.0
    };
    painter.circle_filled(
        egui::pos2(readiness_content_left + 2.5, readiness_top + 5.0),
        2.5,
        if blocker.is_some() {
            t.color.err
        } else if availability_label(kind) == "Production" {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    let readiness_text_left = readiness_content_left + 12.0;
    let readiness_text_width = (rect.right() - 12.0 - readiness_text_left).max(1.0);
    let action_galley = ui.painter().layout(
        disposition.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if enabled {
            t.color.text
        } else {
            t.color.text_dim
        },
        readiness_text_width,
    );
    let action_height = action_galley.size().y;
    painter.galley(
        egui::pos2(readiness_text_left, readiness_top),
        action_galley,
        if enabled {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(readiness) = readiness {
        let detail_galley = ui.painter().layout(
            readiness.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if blocker.is_some() {
                t.color.err
            } else {
                t.color.text_faint
            },
            readiness_text_width,
        );
        painter.galley(
            egui::pos2(readiness_text_left, readiness_top + action_height + 2.0),
            detail_galley,
            if blocker.is_some() {
                t.color.err
            } else {
                t.color.text_faint
            },
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    if let Some(reason) = blocker {
        response.on_hover_text(reason).clicked() && enabled
    } else {
        response
            .on_hover_text(format!("Add {} analysis instance", kind.label()))
            .clicked()
            && enabled
    }
}

const fn analysis_catalog_readiness(kind: AnalysisKind) -> Option<&'static str> {
    if let Some(reason) = kind.execution_blocker() {
        Some(reason)
    } else {
        match kind.availability() {
            crate::simulation::plan::AnalysisAvailability::Production => None,
            crate::simulation::plan::AnalysisAvailability::Preview => {
                Some("Preview engine · non-sign-off")
            }
            crate::simulation::plan::AnalysisAvailability::Compatibility => {
                Some("Compatibility path · non-sign-off")
            }
        }
    }
}

fn filtered_catalog_kinds(query: &str) -> Vec<AnalysisKind> {
    let query = query.trim().to_ascii_lowercase();
    AnalysisKind::MANIFEST_ORDER
        .into_iter()
        .filter(|kind| {
            query.is_empty()
                || format!(
                    "{} {} {} {} {} {} {} {} {}",
                    kind.stable_id(),
                    kind.code(),
                    kind.glyph(),
                    kind.label(),
                    kind.detail(),
                    analysis_catalog_group(*kind),
                    kind.category().detail,
                    availability_label(*kind),
                    kind.execution_blocker().unwrap_or_default(),
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .collect()
}

fn plan_heading(ui: &mut Ui, app: &mut RSpiceApp, surface_width: f32) {
    let plan_name = app.state.sim_setup.active_plan_name().as_str().to_owned();
    let (eyebrow, description, plan_available) = match app.state.sim_setup.stable_analysis_plan() {
        Ok(plan) => {
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .count();
            let prior = app.state.simulation.runs.first().map_or_else(
                || "no prior dataset".to_owned(),
                |run| format!("prior Run {} immutable", run.id),
            );
            (
                format!("Simulation plan · revision {}", plan.revision().get()),
                format!(
                    "{enabled} enabled instances · {} active · {} catalog types · {} executable · {} PVT points · {prior}",
                    plan.instances().len(),
                    AnalysisKind::MANIFEST_ORDER.len(),
                    AnalysisKind::MANIFEST_ORDER
                        .into_iter()
                        .filter(|kind| kind.execution_blocker().is_none())
                        .count(),
                    configured_pvt_count(plan.instances()),
                ),
                true,
            )
        }
        Err(error) => (
            "Simulation plan · unavailable".to_owned(),
            format!("Stable plan unavailable · {error}"),
            false,
        ),
    };

    let mut clone_plan = false;
    let mut validate = false;
    if surface_width > TITLE_ACTION_STACK_BREAKPOINT {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let heading_width = (ui.available_width() - 222.0).max(220.0);
            ui.allocate_ui_with_layout(
                vec2(heading_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| heading(ui, &eyebrow, &plan_name, &description),
            );
            clone_plan = Button::new("Clone plan")
                .icon(Icon::Copy)
                .enabled(plan_available)
                .show(ui)
                .clicked();
            validate = Button::new("Validate plan")
                .icon(Icon::Check)
                .accent()
                .enabled(plan_available)
                .show(ui)
                .clicked();
        });
    } else {
        heading(ui, &eyebrow, &plan_name, &description);
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let action_width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
            clone_plan = Button::new("Clone plan")
                .icon(Icon::Copy)
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
            validate = Button::new("Validate plan")
                .icon(Icon::Check)
                .accent()
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
        });
    }
    if clone_plan {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::ClonePlan(
            ClonePlanDraft::for_source(&plan_name),
        ));
    }
    if validate {
        Command::PreflightChecks.execute(app);
    }
}

fn simulation_workflow_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(workflow) = app.state.workbench.simulation_workflow.clone() else {
        return;
    };
    match workflow {
        SimulationWorkflowDialog::ClonePlan(draft) => clone_plan_dialog(ctx, app, draft),
        SimulationWorkflowDialog::DesignVariable(draft) => design_variable_dialog(ctx, app, draft),
        SimulationWorkflowDialog::SavedOutput(draft) => saved_output_dialog(ctx, app, draft),
    }
}

fn clone_plan_dialog(ctx: &egui::Context, app: &mut RSpiceApp, mut draft: ClonePlanDraft) {
    draft.validation_error = validate_clone_plan_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let source = format!(
        "{} · revision {}",
        app.state.sim_setup.active_plan_name(),
        app.state
            .sim_setup
            .stable_analysis_plan()
            .map_or(0, |plan| plan.revision().get())
    );
    let choice = Dialog::new(
        "SIMULATION · EXPLICIT PLAN LINEAGE",
        "Clone simulation plan",
        "Create cloned plan",
    )
    .description(
        "Create a separately owned simulation plan while preserving source lineage and immutable result manifests.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_setting_row(ui, "New plan name", "Unique within this project.", |ui| {
            mono_input(ui, &mut draft.name, ui.available_width().min(330.0));
        });
        workflow_setting_row(
            ui,
            "Source",
            "Frozen source plan and working revision.",
            |ui| {
                ui.label(
                    egui::RichText::new(&source)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
            },
        );
        workflow_setting_row(
            ui,
            "Copy contents",
            "Dependencies remain linked by stable identifiers.",
            |ui| {
                ui.vertical(|ui| {
                    workflow_checkbox(
                        ui,
                        "Analyses and advanced options",
                        &mut draft.copy_analyses_options,
                    );
                    workflow_checkbox(
                        ui,
                        "Variables, outputs and specifications",
                        &mut draft.copy_variables_outputs_specs,
                    );
                    workflow_checkbox(
                        ui,
                        "PVT and model bindings",
                        &mut draft.copy_pvt_model_bindings,
                    );
                    workflow_checkbox(
                        ui,
                        "Regression baseline ownership",
                        &mut draft.copy_regression_baseline,
                    );
                });
            },
        );
        workflow_setting_row(
            ui,
            "Results",
            "Result datasets are never duplicated with a plan.",
            |ui| {
                ui.label("none · manifests remain linked");
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_clone_plan);
}

fn design_variable_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: DesignVariableDraft,
) {
    draft.validation_error = validate_design_variable_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let consumer_summary = design_variable_consumers(app, &draft);
    let resolved_value = design_variable_from_draft(app, &draft)
        .and_then(|variable| variable.resolved_value_si())
        .map_or_else(
            |_| "unresolved".to_owned(),
            |value| format!("{value:.8e} SI"),
        );
    let name_conflicts = if draft
        .validation_error
        .as_deref()
        .is_some_and(|error| error.to_ascii_lowercase().contains("name"))
    {
        "resolve validation error"
    } else {
        "none"
    };
    let choice = Dialog::new(
        "SIMULATION PLAN · TYPED PARAMETER · DEPENDENCY PREVIEW",
        "Create design variable",
        "Create variable",
    )
    .description(
        "Create a typed, scoped simulation parameter with explicit constraints and deterministic netlist ownership.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_split(ui, |ui| {
            workflow_text_field(ui, "Name", &mut draft.name, true);
            workflow_text_field(ui, "Expression", &mut draft.expression, true);
            workflow_select_field(
                ui,
                "design-variable-quantity",
                "Quantity",
                &mut draft.quantity,
                &[
                    "Resistance",
                    "Capacitance",
                    "Voltage",
                    "Current",
                    "Temperature",
                    "Dimensionless",
                ],
            );
            workflow_select_field(
                ui,
                "design-variable-scope",
                "Ownership scope",
                &mut draft.scope,
                &[
                    "Lab characterization · testbench",
                    "Project",
                    "Selected cell",
                    "Selected analysis only",
                ],
            );
            workflow_text_field(ui, "Description", &mut draft.description, false);
        }, |ui| {
            workflow_section_heading(ui, "Constraints and consumers");
            workflow_text_field(ui, "Allowed range", &mut draft.allowed_range, true);
            workflow_select_field(
                ui,
                "design-variable-sweep",
                "Sweep eligibility",
                &mut draft.sweep_eligibility,
                &["Nested sweep + optimization", "Optimization only", "Fixed parameter"],
            );
            workflow_select_field(
                ui,
                "design-variable-override",
                "Override policy",
                &mut draft.override_policy,
                &["Explicit test-local override", "Inherit owner only"],
            );
            ui.add_space(8.0);
            property_row(ui, "Resolved value", &resolved_value);
            property_row(ui, "Name conflicts", name_conflicts);
            property_row(ui, "Prospective consumers", &consumer_summary);
            property_row(ui, "Result effect", "dependent future runs only");
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_design_variable);
}

fn saved_output_dialog(ctx: &egui::Context, app: &mut RSpiceApp, mut draft: SavedOutputDraft) {
    draft.validation_error = validate_saved_output_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let candidate = saved_output_from_draft(app, &draft).ok();
    let preflight = candidate.as_ref().map(|output| {
        app.simulation_controller
            .saved_output_preflight(&app.state, output)
    });
    let inferred_unit = candidate.as_ref().map_or_else(
        || inferred_output_unit(&draft.expression),
        |output| output.inferred_unit(),
    );
    let consumers = saved_output_consumers(app, &draft);
    let expression_validity = preflight.as_ref().map_or_else(
        || {
            draft
                .validation_error
                .clone()
                .unwrap_or_else(|| "candidate output is incomplete".to_owned())
        },
        |report| saved_output_semantic_summary(report.semantic_status()),
    );
    let storage_increment = preflight.as_ref().map_or_else(
        || "indeterminate until the output contract is valid".to_owned(),
        |report| {
            saved_output_storage_summary(
                report.storage_estimate(),
                report.compatible_analysis_count(),
            )
        },
    );
    let choice = Dialog::new(
        "SIMULATION PLAN · DATA CONTRACT · STORAGE IMPACT",
        "Add saved output or derived expression",
        "Add output",
    )
    .description(
        "Add a validated output contract with explicit analysis compatibility, retention, precision, and streaming behavior.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        workflow_split(ui, |ui| {
            workflow_select_field(
                ui,
                "saved-output-kind",
                "Output kind",
                &mut draft.kind,
                &[
                    "Raw voltage / current",
                    "Derived expression",
                    "Device operating-point quantity",
                    "Noise contributor",
                    "RF port quantity",
                ],
            );
            workflow_text_field(ui, "Name", &mut draft.name, true);
            workflow_multiline_field(ui, "Source or expression", &mut draft.expression);
            workflow_select_field(
                ui,
                "saved-output-analyses",
                "Compatible analyses",
                &mut draft.compatible_analyses,
                &["OP + TRAN + AC", "All compatible analyses", "Selected analysis only"],
            );
        }, |ui| {
            workflow_section_heading(ui, "Save, precision, and provenance");
            workflow_select_field(
                ui,
                "saved-output-save-policy",
                "Save policy",
                &mut draft.save_policy,
                &[
                    "Every accepted point",
                    "Selected + final points",
                    "On demand from retained state",
                    "Failure diagnostics only",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-precision",
                "Stored precision",
                &mut draft.precision,
                &[
                    "f64 / complex128",
                    "f32 display cache + full source precision",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-streaming",
                "Streaming",
                &mut draft.streaming,
                &["Live plot · adaptive display decimation", "Store only"],
            );
            ui.add_space(8.0);
            property_row(ui, "Inferred unit", inferred_unit);
            property_row(ui, "Estimated increment", &storage_increment);
            property_row(ui, "Expression validity", &expression_validity);
            property_row(ui, "Consumers", &consumers);
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_saved_output);
}

fn validate_clone_plan_draft(app: &RSpiceApp, draft: &ClonePlanDraft) -> Result<(), String> {
    let name = crate::common::app::SimulationPlanName::new(draft.name.clone())
        .map_err(|error| error.to_string())?;
    let key = name.as_str().to_lowercase();
    if app
        .state
        .sim_setup
        .active_plan_name()
        .as_str()
        .to_lowercase()
        == key
        || app
            .state
            .sim_setup
            .inactive_plans()
            .iter()
            .any(|plan| plan.name().as_str().to_lowercase() == key)
    {
        return Err(format!(
            "A simulation plan named '{}' already exists.",
            name.as_str()
        ));
    }
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    if plan.has_executing_instances() {
        return Err(
            "The active plan owns queued or executing work and cannot be cloned.".to_owned(),
        );
    }
    Ok(())
}

fn commit_clone_plan(app: &mut RSpiceApp, draft: &ClonePlanDraft) -> Result<String, String> {
    validate_clone_plan_draft(app, draft)?;
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let source_plan_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(source_plan_id);
    let options = crate::common::app::SimulationPlanCloneOptions {
        copy_analyses: draft.copy_analyses_options,
        copy_advanced_options: draft.copy_analyses_options,
        copy_variables_outputs_and_specifications: draft.copy_variables_outputs_specs,
        copy_pvt_and_model_bindings: draft.copy_pvt_model_bindings,
        copy_regression_baseline_ownership: draft.copy_regression_baseline,
    };
    let outcome = setup
        .clone_active_plan(draft.name.clone(), options)
        .map_err(|error| error.to_string())?;
    workspace
        .clone_plan_data(
            outcome.source_plan_id,
            outcome.cloned_plan_id,
            outcome.contents.copy_variables_outputs_and_specifications,
            outcome.contents.copy_regression_baseline_ownership,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.state.workbench.preflight = Default::default();
    app.state.workbench.analysis_lifecycle_status = format!(
        "Cloned plan {} from {} at revision {}; result manifests were not duplicated.",
        outcome.cloned_plan_id,
        outcome.source_plan_id,
        outcome.source_revision.get()
    );
    Ok(format!(
        "Created and activated '{}' with independent plan identity {}.",
        app.state.sim_setup.active_plan_name(),
        outcome.cloned_plan_id
    ))
}

fn design_variable_from_draft(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, String> {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility,
    };

    let quantity = DesignVariableQuantity::ALL
        .get(draft.quantity)
        .copied()
        .ok_or_else(|| "Quantity selection is invalid.".to_owned())?;
    let scope = match draft.scope {
        0 => DesignVariableScope::Testbench,
        1 => DesignVariableScope::Project,
        2 => DesignVariableScope::SelectedCell {
            cell: app.state.workspace.active_view.clone(),
        },
        3 => DesignVariableScope::SelectedAnalysis {
            analysis_id: app
                .state
                .workbench
                .active_analysis_instance
                .ok_or_else(|| "Select an analysis before using analysis-only scope.".to_owned())?,
        },
        _ => return Err("Ownership scope selection is invalid.".to_owned()),
    };
    let allowed_range = parse_design_variable_range(&draft.allowed_range)?;
    let sweep_eligibility = DesignVariableSweepEligibility::ALL
        .get(draft.sweep_eligibility)
        .copied()
        .ok_or_else(|| "Sweep eligibility selection is invalid.".to_owned())?;
    let override_policy = DesignVariableOverridePolicy::ALL
        .get(draft.override_policy)
        .copied()
        .ok_or_else(|| "Override policy selection is invalid.".to_owned())?;
    DesignVariable::new(
        draft.name.clone(),
        draft.expression.clone(),
        quantity,
        scope,
        draft.description.clone(),
        allowed_range,
        sweep_eligibility,
        override_policy,
    )
}

fn parse_design_variable_range(
    value: &str,
) -> Result<Option<crate::state::DesignVariableRange>, String> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    let split = value
        .split_once('…')
        .or_else(|| value.split_once("..="))
        .or_else(|| value.split_once(".."));
    let Some((minimum, maximum)) = split else {
        return Err("Allowed range must be written as minimum … maximum.".to_owned());
    };
    let minimum = minimum.trim();
    let maximum = maximum.trim();
    if minimum.is_empty() || maximum.is_empty() {
        return Err("Allowed range requires both a minimum and maximum.".to_owned());
    }
    Ok(Some(crate::state::DesignVariableRange {
        minimum: minimum.to_owned(),
        maximum: maximum.to_owned(),
    }))
}

fn validate_design_variable_draft(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<(), String> {
    let variable = design_variable_from_draft(app, draft)?;
    let plan_id = app.state.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = app.state.workspace.clone();
    workspace
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())
}

fn commit_design_variable(
    app: &mut RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<String, String> {
    let variable = design_variable_from_draft(app, draft)?;
    let variable_name = variable.name.clone();
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Created design variable {variable_name}."
        ))
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.preflight = Default::default();
    app.state.workbench.analysis_lifecycle_status = format!(
        "Configuration receipt #{} · revision {} to {} · {}",
        receipt.sequence(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.detail()
    );
    Ok(format!(
        "Created design variable {variable_name} in plan {}.",
        app.state.sim_setup.active_plan_name()
    ))
}

fn saved_output_from_draft(
    app: &RSpiceApp,
    draft: &SavedOutputDraft,
) -> Result<crate::state::SavedOutput, String> {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };
    let kind = SavedOutputKind::ALL
        .get(draft.kind)
        .copied()
        .ok_or_else(|| "Output kind selection is invalid.".to_owned())?;
    let compatible_analyses = match draft.compatible_analyses {
        0 => SavedOutputCompatibility::OpTranAc,
        1 => SavedOutputCompatibility::AllCompatibleAnalyses,
        2 => SavedOutputCompatibility::SelectedAnalysis {
            analysis_id: app
                .state
                .workbench
                .active_analysis_instance
                .ok_or_else(|| "Select an analysis before using analysis-only scope.".to_owned())?,
        },
        _ => return Err("Compatible analyses selection is invalid.".to_owned()),
    };
    let save_policy = SavedOutputPolicy::ALL
        .get(draft.save_policy)
        .copied()
        .ok_or_else(|| "Save policy selection is invalid.".to_owned())?;
    let stored_precision = SavedOutputPrecision::ALL
        .get(draft.precision)
        .copied()
        .ok_or_else(|| "Stored precision selection is invalid.".to_owned())?;
    let streaming = SavedOutputStreaming::ALL
        .get(draft.streaming)
        .copied()
        .ok_or_else(|| "Streaming selection is invalid.".to_owned())?;
    SavedOutput::new(
        kind,
        draft.name.clone(),
        draft.expression.clone(),
        compatible_analyses,
        save_policy,
        stored_precision,
        streaming,
    )
}

fn validate_saved_output_draft(app: &RSpiceApp, draft: &SavedOutputDraft) -> Result<(), String> {
    let output = saved_output_from_draft(app, draft)?;
    let report = app
        .simulation_controller
        .saved_output_preflight(&app.state, &output);
    if let SavedOutputSemanticStatus::Invalid { reason } = report.semantic_status() {
        return Err(reason.clone());
    }
    let plan_id = app.state.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = app.state.workspace.clone();
    workspace
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn commit_saved_output(app: &mut RSpiceApp, draft: &SavedOutputDraft) -> Result<String, String> {
    validate_saved_output_draft(app, draft)?;
    let output = saved_output_from_draft(app, draft)?;
    let output_name = output.name.clone();
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!("Added saved output {output_name}."))
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.preflight = Default::default();
    app.state.workbench.analysis_lifecycle_status = format!(
        "Configuration receipt #{} · revision {} to {} · {}",
        receipt.sequence(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.detail()
    );
    Ok(format!(
        "Added saved output {output_name} to plan {}.",
        app.state.sim_setup.active_plan_name()
    ))
}

fn design_variable_consumers(app: &RSpiceApp, draft: &DesignVariableDraft) -> String {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return "plan unavailable".to_owned();
    };
    let selected_only = (draft.scope == 3).then_some(app.state.workbench.active_analysis_instance);
    let labels = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .filter(|instance| selected_only.is_none_or(|selected| selected == Some(instance.id())))
        .map(|instance| instance.kind().code())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        "no enabled analysis consumers".to_owned()
    } else {
        labels.join(" · ")
    }
}

fn saved_output_consumers(app: &RSpiceApp, draft: &SavedOutputDraft) -> String {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return "plan unavailable".to_owned();
    };
    let selected = app.state.workbench.active_analysis_instance;
    let labels = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .filter(|instance| match draft.compatible_analyses {
            0 => matches!(
                instance.kind(),
                AnalysisKind::OperatingPoint | AnalysisKind::Transient | AnalysisKind::Ac
            ),
            1 => true,
            2 => selected == Some(instance.id()),
            _ => false,
        })
        .map(|instance| instance.kind().code())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        "no compatible enabled analyses".to_owned()
    } else {
        labels.join(" · ")
    }
}

fn inferred_output_unit(expression: &str) -> &'static str {
    let expression = expression.trim();
    if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
    {
        "volts"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
    {
        "amperes"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
    {
        "dimensionless"
    } else {
        "resolved from output schema"
    }
}

fn saved_output_semantic_summary(status: &SavedOutputSemanticStatus) -> String {
    match status {
        SavedOutputSemanticStatus::Valid { detail } => detail.clone(),
        SavedOutputSemanticStatus::RuntimeBound { reason } => reason.clone(),
        SavedOutputSemanticStatus::Invalid { reason } => reason.clone(),
    }
}

fn saved_output_storage_summary(
    estimate: &SavedOutputStorageEstimate,
    compatible_analyses: usize,
) -> String {
    match estimate {
        SavedOutputStorageEstimate::ExactBytes(bytes) => format!(
            "{} · {compatible_analyses} compatible {}",
            format_storage_bytes(*bytes),
            if compatible_analyses == 1 {
                "analysis"
            } else {
                "analyses"
            }
        ),
        SavedOutputStorageEstimate::Indeterminate { reason } => {
            format!("indeterminate · {reason}")
        }
    }
}

fn format_storage_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    let bytes_f64 = bytes as f64;
    if bytes_f64 >= GIB {
        format!("{:.2} GiB", bytes_f64 / GIB)
    } else if bytes_f64 >= MIB {
        format!("{:.2} MiB", bytes_f64 / MIB)
    } else if bytes_f64 >= KIB {
        format!("{:.2} KiB", bytes_f64 / KIB)
    } else {
        format!("{bytes} B")
    }
}

fn finish_workflow_choice<D>(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    choice: DialogChoice,
    mut draft: D,
    commit: fn(&mut RSpiceApp, &D) -> Result<String, String>,
) where
    D: Clone + Into<SimulationWorkflowDialog> + WorkflowDraft,
{
    match choice {
        DialogChoice::Primary => match commit(app, &draft) {
            Ok(message) => {
                app.state.workbench.simulation_workflow = None;
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Simulation plan updated", message);
            }
            Err(error) => {
                set_workflow_error(&mut draft, error);
                app.state.workbench.simulation_workflow = Some(draft.into());
            }
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.simulation_workflow = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {
            app.state.workbench.simulation_workflow = Some(draft.into());
        }
    }
}

trait WorkflowDraft {
    fn set_error(&mut self, error: String);
}

impl WorkflowDraft for ClonePlanDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for DesignVariableDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for SavedOutputDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

fn set_workflow_error(draft: &mut impl WorkflowDraft, error: String) {
    draft.set_error(error);
}

impl From<ClonePlanDraft> for SimulationWorkflowDialog {
    fn from(draft: ClonePlanDraft) -> Self {
        Self::ClonePlan(draft)
    }
}

impl From<DesignVariableDraft> for SimulationWorkflowDialog {
    fn from(draft: DesignVariableDraft) -> Self {
        Self::DesignVariable(draft)
    }
}

impl From<SavedOutputDraft> for SimulationWorkflowDialog {
    fn from(draft: SavedOutputDraft) -> Self {
        Self::SavedOutput(draft)
    }
}

fn workflow_split(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    if ui.available_width() >= 620.0 {
        let width = ui.available_width();
        let divider = 1.0;
        let pane_width = ((width - divider) * 0.5).max(1.0);
        let response = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, left)
            });
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, right)
            });
        });
        let divider_x = response.response.rect.left() + pane_width;
        ui.painter().vline(
            divider_x,
            response.response.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    } else {
        let left_response = workflow_split_pane(ui, left);
        ui.painter().hline(
            left_response.rect.x_range(),
            left_response.rect.bottom(),
            Stroke::new(1.0, t.color.border_strong),
        );
        workflow_split_pane(ui, right);
    }
}

fn workflow_split_pane(ui: &mut Ui, body: impl FnOnce(&mut Ui)) -> egui::Response {
    let outer_width = ui.available_width();
    egui::Frame::new()
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width((outer_width - 20.0).max(1.0));
            ui.spacing_mut().item_spacing.y = 6.0;
            body(ui);
        })
        .response
}

fn workflow_setting_row(ui: &mut Ui, title: &str, detail: &str, value: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let response = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width((width - 24.0).max(1.0));
            if width >= 560.0 {
                ui.columns(2, |columns| {
                    columns[0].label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
                    );
                    columns[0].label(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    value(&mut columns[1]);
                });
            } else {
                ui.label(
                    egui::RichText::new(title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
                );
                ui.label(
                    egui::RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(5.0);
                value(ui);
            }
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn workflow_text_field(ui: &mut Ui, label: &str, value: &mut String, monospace: bool) {
    workflow_field_label(ui, label);
    if monospace {
        mono_input(ui, value, ui.available_width());
    } else {
        let height = Tokens::get(ui.ctx()).metrics.ctl_h;
        ui.add_sized(
            vec2(ui.available_width(), height),
            egui::TextEdit::singleline(value).margin(egui::Margin::symmetric(8, 4)),
        );
    }
}

fn workflow_multiline_field(ui: &mut Ui, label: &str, value: &mut String) {
    workflow_field_label(ui, label);
    ui.add_sized(
        vec2(ui.available_width(), 74.0),
        egui::TextEdit::multiline(value)
            .font(egui::TextStyle::Monospace)
            .margin(egui::Margin::symmetric(8, 6)),
    );
}

fn workflow_select_field(
    ui: &mut Ui,
    salt: &str,
    label: &str,
    value: &mut usize,
    options: &[&str],
) {
    workflow_field_label(ui, label);
    let choices = options
        .iter()
        .map(|option| (*option).to_owned())
        .collect::<Vec<_>>();
    *value = (*value).min(choices.len().saturating_sub(1));
    let current = choices.get(*value).map_or("", String::as_str);
    if let Some(selected) = select(ui, salt, label, current, &choices, ui.available_width()) {
        *value = selected;
    }
}

fn workflow_field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn workflow_section_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 25.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
}

fn workflow_checkbox(ui: &mut Ui, label: &str, value: &mut bool) {
    ui.add_sized(
        vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.row_h),
        egui::Checkbox::new(value, label),
    );
}

fn workflow_validation_message(ui: &mut Ui, error: Option<&str>) {
    if let Some(error) = error {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(error)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.err),
        );
    }
}

fn analysis_editor(ui: &mut Ui, app: &mut RSpiceApp, viewport_width: f32) {
    let selected = match selected_analysis(app) {
        Ok(Some(selected)) => selected,
        Ok(None) => {
            flat_notice(ui, |ui| {
                status_dot(
                    ui,
                    Tokens::get(ui.ctx()).color.warn,
                    "No active analysis instance",
                );
                ui.label("The stable plan is empty. Add an analysis from Simulation Studio.");
            });
            lifecycle_receipt_strip(ui, app);
            preflight_strip(ui, app);
            setup_tables(ui, app);
            return;
        }
        Err(error) => {
            record_failure(app, "Analysis editor", &error);
            flat_notice(ui, |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            });
            lifecycle_receipt_strip(ui, app);
            preflight_strip(ui, app);
            setup_tables(ui, app);
            return;
        }
    };

    let prior_datasets = app.state.simulation.runs.first().map_or_else(
        || "No prior datasets".to_owned(),
        |run| format!("Run {} retained · immutable", run.id),
    );
    let mut draft = selected.draft.clone();
    let serialized_before = serde_json::to_vec(&draft);
    let mut action = None;

    let t = Tokens::get(ui.ctx());
    let editor_response = egui::Frame::new()
        .fill(t.color.bg_app)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            analysis_form_header(ui, &selected);
            if availability_label(selected.kind) != "Production" {
                capability_banner(ui, selected.kind);
            }
            lifecycle_toolbar(ui, &selected, &mut action);
            analysis_contract(ui, &selected, &prior_datasets, viewport_width <= 760.0);
            let note = analysis_form_body(ui, app, &mut draft);
            form_status(ui, app, &draft, note);
        })
        .response;
    ui.painter().hline(
        editor_response.rect.x_range(),
        editor_response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );

    let draft_changed = match (serialized_before, serde_json::to_vec(&draft)) {
        (Ok(before), Ok(after)) => before != after,
        (Err(error), _) | (_, Err(error)) => {
            record_failure(
                app,
                "Analysis edit",
                &format!("the draft could not be serialized exactly: {error}"),
            );
            lifecycle_receipt_strip(ui, app);
            return;
        }
    };

    if draft_changed {
        commit_draft(app, selected.id, draft);
    }
    if let Some(action) = action {
        apply_analysis_action(app, selected.id, action);
    }
    lifecycle_receipt_strip(ui, app);
    preflight_strip(ui, app);
    setup_tables(ui, app);
}

fn analysis_form_header(ui: &mut Ui, selected: &SelectedAnalysis) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 42.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let icon_rect = Rect::from_min_size(
        egui::pos2(rect.left() + 11.0, rect.center().y - 12.0),
        Vec2::splat(24.0),
    );
    ui.painter().rect_filled(icon_rect, 3.0, t.color.accent_dim);
    analysis_icon(selected.kind).paint(ui.painter(), icon_rect.shrink(4.0), t.color.accent);
    let text_left = icon_rect.right() + 9.0;
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 5.0),
            egui::pos2(rect.right() - 120.0, rect.top() + 22.0),
        ),
        selected.kind.label(),
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 21.0),
            egui::pos2(rect.right() - 120.0, rect.bottom() - 3.0),
        ),
        &format!("{} · lifecycle {}", selected.id, selected.lifecycle),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let (status, color) = if selected.issues.is_empty() {
        (
            availability_label(selected.kind),
            if availability_label(selected.kind) == "Production" {
                t.color.ok
            } else {
                t.color.warn
            },
        )
    } else {
        ("preflight blocked", t.color.err)
    };
    ui.painter().text(
        rect.right_center() - vec2(11.0, 0.0),
        Align2::RIGHT_CENTER,
        status,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        color,
    );
}

fn analysis_icon(kind: AnalysisKind) -> WorkbenchIcon {
    match kind {
        AnalysisKind::OperatingPoint
        | AnalysisKind::PoleZero
        | AnalysisKind::Stb
        | AnalysisKind::SParameter
        | AnalysisKind::Hbsp
        | AnalysisKind::Psp
        | AnalysisKind::Optimization
        | AnalysisKind::Soa
        | AnalysisKind::DcMismatch => WorkbenchIcon::Target,
        AnalysisKind::MonteCarlo | AnalysisKind::Temperature | AnalysisKind::Corner => {
            WorkbenchIcon::Grid
        }
        AnalysisKind::Reliability => WorkbenchIcon::Verify,
        AnalysisKind::Ac
        | AnalysisKind::DcSweep
        | AnalysisKind::Fourier
        | AnalysisKind::TransferFunction
        | AnalysisKind::Disto => WorkbenchIcon::Results,
        AnalysisKind::Transient
        | AnalysisKind::Noise
        | AnalysisKind::Sensitivity
        | AnalysisKind::Pss
        | AnalysisKind::Qpss
        | AnalysisKind::HarmonicBalance
        | AnalysisKind::Hbnoise
        | AnalysisKind::Pac
        | AnalysisKind::Pnoise
        | AnalysisKind::Pxf
        | AnalysisKind::Pstb
        | AnalysisKind::Qpac
        | AnalysisKind::Qpnoise
        | AnalysisKind::Qpxf
        | AnalysisKind::TransientNoise
        | AnalysisKind::Envelope => WorkbenchIcon::Simulate,
    }
}

fn capability_banner(ui: &mut Ui, kind: AnalysisKind) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    let response = egui::Frame::new()
        .fill(t.color.accent_dim)
        .inner_margin(egui::Margin::symmetric(8, 7))
        .show(ui, |ui| {
            ui.set_width(content_width);
            let copy = kind.execution_blocker().map_or_else(
                || {
                    format!(
                        "{} is retained for design review and produces non-sign-off data.",
                        availability_label(kind)
                    )
                },
                |blocker| {
                    format!(
                        "Execution is blocked: {blocker}. The configuration remains editable and persistent."
                    )
                },
            );
            ui.label(
                egui::RichText::new(copy)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn lifecycle_toolbar(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    action: &mut Option<AnalysisAction>,
) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    let response = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.horizontal_wrapped(|ui| {
                if Button::new("Clone").icon(Icon::Copy).show(ui).clicked() {
                    *action = Some(AnalysisAction::Clone);
                }
                if Button::new("Earlier")
                    .enabled(selected.position > 0)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AnalysisAction::Earlier(selected.position - 1));
                }
                if Button::new("Later")
                    .enabled(selected.position + 1 < selected.plan_length)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AnalysisAction::Later(selected.position + 1));
                }
                if Button::new("Bind dependencies").show(ui).clicked() {
                    *action = Some(AnalysisAction::BindDependencies);
                }
                if Button::new("Validate").icon(Icon::Check).show(ui).clicked() {
                    *action = Some(AnalysisAction::Validate);
                }
                if Button::new("Remove").show(ui).clicked() {
                    *action = Some(AnalysisAction::Remove);
                }
            });
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn analysis_contract(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    prior_datasets: &str,
    stacked: bool,
) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.x = 10.0;
            ui.spacing_mut().item_spacing.y = 0.0;
            let properties = |ui: &mut Ui| {
                property_row(ui, "Stable instance identity", &selected.id.to_string());
                property_row(
                    ui,
                    "Ordered position",
                    &format!("{} / {}", selected.position + 1, selected.plan_length),
                );
                prerequisite_rows(ui, selected);
                property_row(ui, "Availability", availability_label(selected.kind));
                property_row(ui, "Prior datasets", prior_datasets);
            };
            let evidence = |ui: &mut Ui| {
                let (color, title, detail) = if let Some(issue) = selected.issues.first() {
                    (
                        t.color.err,
                        "Dependency graph blocked",
                        format_plan_issue(issue),
                    )
                } else {
                    (
                        t.color.ok,
                        "Dependency graph valid",
                        "Dependency identity, order, and enabled state are valid for this instance."
                            .to_owned(),
                    )
                };
                status_dot(ui, color, title);
                ui.label(
                    egui::RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            };
            if stacked {
                properties(ui);
                ui.add_space(8.0);
                evidence(ui);
            } else {
                ui.columns(2, |columns| {
                    properties(&mut columns[0]);
                    evidence(&mut columns[1]);
                });
            }
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn analysis_form_body(ui: &mut Ui, app: &RSpiceApp, draft: &mut AnalysisDraft) -> &'static str {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    egui::Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(egui::Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.y = 0.0;
            let note = analysis_form::form(
                ui,
                draft,
                app.state.ui.preferences.quantity_presentation_policy(),
                app.state.ui.number_locale,
            );
            if let Some(error) = app.state.sim_setup.analysis_draft_validation_error(draft) {
                ui.add_space(6.0);
                ui.label(egui::RichText::new(error).color(t.color.err));
            }
            note
        })
        .inner
}

fn form_status(ui: &mut Ui, app: &RSpiceApp, draft: &AnalysisDraft, note: &str) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    let valid = app
        .state
        .sim_setup
        .analysis_draft_validation_error(draft)
        .is_none();
    let response = egui::Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.horizontal_wrapped(|ui| {
                status_dot(
                    ui,
                    if valid { t.color.ok } else { t.color.err },
                    if valid {
                        "Analysis configuration valid"
                    } else {
                        "Analysis configuration blocked"
                    },
                );
                ui.label(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn prerequisite_rows(ui: &mut Ui, selected: &SelectedAnalysis) {
    if selected.kind.prerequisites().is_empty() {
        property_row(ui, "Prerequisites", "none declared");
        return;
    }
    for prerequisite in selected.kind.prerequisites() {
        let target = selected
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite() == *prerequisite)
            .map_or_else(
                || "unbound · preflight blocked".to_owned(),
                |dependency| dependency.target().to_string(),
            );
        property_row(
            ui,
            &format!("{} prerequisite", prerequisite.stable_id().to_uppercase()),
            &target,
        );
    }
}

fn lifecycle_receipt_strip(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let tombstones = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.tombstones().len());
    let width = ui.available_width().max(1.0);
    let compact = width <= 760.0;
    let detail = &app.state.workbench.analysis_lifecycle_status;
    let title_width = 94.0;
    let tombstone_width = 104.0;
    let detail_width = if compact {
        (width - 18.0).max(1.0)
    } else {
        (width - 18.0 - title_width - tombstone_width - 16.0).max(1.0)
    };
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        detail_width,
    );
    let height = if compact {
        (detail_galley.size().y + 34.0).max(52.0)
    } else {
        (detail_galley.size().y + 12.0).max(36.0)
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let top_center = if compact {
        rect.top() + 14.0
    } else {
        rect.center().y
    };
    painter.text(
        egui::pos2(rect.left() + 9.0, top_center),
        Align2::LEFT_CENTER,
        "Lifecycle receipt",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let tombstone_text = format!(
        "{tombstones} tombstone{}",
        if tombstones == 1 { "" } else { "s" }
    );
    painter.text(
        egui::pos2(rect.right() - 9.0, top_center),
        Align2::RIGHT_CENTER,
        tombstone_text,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let detail_position = if compact {
        egui::pos2(rect.left() + 9.0, rect.top() + 27.0)
    } else {
        egui::pos2(rect.left() + 9.0 + title_width + 8.0, rect.top() + 6.0)
    };
    painter.galley(detail_position, detail_galley, t.color.text_dim);
    response.on_hover_text(detail);
}

fn preflight_strip(ui: &mut Ui, app: &RSpiceApp) {
    let topology_ok = !app.state.schematic.components.is_empty();
    let retained_report = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .filter(|report| {
            report.project_revision == app.state.workspace.project.revision().get()
                && report.topology_revision == app.state.schematic.topology_version()
        });
    let netlist_state = retained_report.map(|report| {
        topology_ok
            && !report.blockers.iter().any(|issue| {
                issue.remediation == super::super::state::PreflightRemediation::DesignChecks
            })
    });
    let (enabled_count, configurations_ok, graph_ok) =
        match app.state.sim_setup.stable_analysis_plan() {
            Ok(plan) => {
                let enabled = plan
                    .instances()
                    .iter()
                    .filter(|instance| instance.enabled())
                    .collect::<Vec<_>>();
                (
                    enabled.len(),
                    enabled.iter().all(|instance| {
                        app.state
                            .sim_setup
                            .analysis_draft_validation_error(instance.draft())
                            .is_none()
                    }),
                    plan.validation_issues().is_empty(),
                )
            }
            Err(_) => (0, false, false),
        };
    let specifications_configured = !app.state.workspace.specs.is_empty();
    let specifications_ok = specifications_configured
        && app.state.workspace.specs.iter().all(|spec| {
            !spec.measurement.trim().is_empty()
                && spec
                    .min
                    .zip(spec.max)
                    .is_none_or(|(minimum, maximum)| minimum <= maximum)
        });
    let items = [
        (
            netlist_state,
            "Netlist",
            if !topology_ok {
                "design is empty".to_owned()
            } else if retained_report.is_none() {
                if app.state.workbench.preflight.report.is_some() {
                    "preflight receipt expired".to_owned()
                } else {
                    "preflight not run".to_owned()
                }
            } else if netlist_state == Some(false) {
                "preflight has design blockers".to_owned()
            } else {
                format!(
                    "revision {} current",
                    app.state.schematic.topology_version()
                )
            },
        ),
        (
            Some(graph_ok),
            "Instance graph",
            if graph_ok {
                format!("dependency ordered · {enabled_count} enabled")
            } else {
                "resolve lifecycle diagnostics".to_owned()
            },
        ),
        (
            specifications_configured.then_some(specifications_ok),
            "Outputs",
            if !specifications_configured {
                "not configured · optional".to_owned()
            } else if specifications_ok {
                format!("{} specifications valid", app.state.workspace.specs.len())
            } else {
                "invalid specification bounds".to_owned()
            },
        ),
        (
            retained_report.map(super::super::state::PreflightReport::is_runnable),
            "Execution graph",
            if enabled_count == 0 {
                "enable an analysis instance".to_owned()
            } else if !configurations_ok {
                "correct invalid fields".to_owned()
            } else if !graph_ok {
                "dependency graph blocked".to_owned()
            } else if retained_report.is_none() {
                if app.state.workbench.preflight.report.is_some() {
                    "rerun expired preflight".to_owned()
                } else {
                    "run preflight to authorize dispatch".to_owned()
                }
            } else if retained_report.is_some_and(|report| !report.is_runnable()) {
                "resolve retained preflight blockers".to_owned()
            } else {
                let task_count = retained_report
                    .and_then(|report| report.prepared.as_ref())
                    .map_or(enabled_count, |prepared| prepared.task_count);
                format!("{task_count} analysis tasks ready")
            },
        ),
    ];

    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= 760.0;
    let columns = if compact { 2 } else { 4 };
    let rows = items.len().div_ceil(columns);
    let cell_width = ui.available_width() / columns as f32;
    let text_width = (cell_width - 36.0).max(1.0);
    let row_height = items
        .iter()
        .map(|item| {
            let detail = ui.painter().layout(
                item.2.clone(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
                text_width,
            );
            (26.0 + detail.size().y).max(PREFLIGHT_CELL_HEIGHT)
        })
        .fold(PREFLIGHT_CELL_HEIGHT, f32::max);
    let size = vec2(ui.available_width(), row_height * rows as f32);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let cell_width = rect.width() / columns as f32;
    for (index, item) in items.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let cell = Rect::from_min_size(
            rect.min + vec2(cell_width * column as f32, row_height * row as f32),
            vec2(cell_width, row_height),
        );
        preflight_cell(
            ui,
            cell,
            item.0,
            item.1,
            &item.2,
            column + 1 < columns,
            row + 1 < rows,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn preflight_cell(
    ui: &mut Ui,
    rect: Rect,
    pass: Option<bool>,
    name: &str,
    detail: &str,
    draw_right_border: bool,
    draw_bottom_border: bool,
) {
    let t = Tokens::get(ui.ctx());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    if draw_right_border {
        painter.vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    if draw_bottom_border {
        painter.hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
    }
    let (mark, mark_color) = match pass {
        Some(true) => ("✓", t.color.ok),
        Some(false) => ("△", t.color.err),
        None => ("·", t.color.text_faint),
    };
    painter.text(
        rect.left_center() + vec2(9.0, 0.0),
        Align2::LEFT_CENTER,
        mark,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        mark_color,
    );
    let text_left = rect.left() + 28.0;
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 5.0),
            egui::pos2(rect.right() - 8.0, rect.center().y + 1.0),
        ),
        name,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (rect.right() - 8.0 - text_left).max(1.0),
    );
    painter.galley(
        egui::pos2(text_left, rect.top() + 21.0),
        detail_galley,
        t.color.text_dim,
    );
}

fn setup_tables(ui: &mut Ui, app: &mut RSpiceApp) {
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
        .ok();
    let payload = plan_id
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id).cloned())
        .unwrap_or_default();
    let design_variables = payload.design_variables;
    let saved_outputs = payload.saved_outputs;
    let saved_output_reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, &saved_outputs);
    let specifications = if plan_id.is_some() {
        payload.specs
    } else {
        app.state.workspace.specs.clone()
    };
    let dataset = selected_output_dataset(&app.state.simulation).cloned();
    let row_height = Tokens::get(ui.ctx()).metrics.row_h;
    let variable_rows = design_variables.len().max(1);
    let output_rows = (saved_outputs.len() + specifications.len()).max(1);
    let variable_card_height = setup_card_height(row_height, variable_rows);
    let output_card_height = setup_card_height(row_height, output_rows);
    let split = ui.available_width() > SIMULATION_STACK_BREAKPOINT;
    let mut add_variable = false;
    let mut add_output = false;

    if split {
        let card_height = variable_card_height.max(output_card_height);
        let divider = 1.0;
        let usable = (ui.available_width() - divider).max(1.0);
        let left_width = usable * 0.46;
        let right_width = usable - left_width;
        let row = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(
                vec2(left_width, card_height),
                Layout::top_down(Align::Min),
                |ui| {
                    add_variable = design_variables_card(
                        ui,
                        &design_variables,
                        card_height,
                        SetupCardBorder::SplitLeft,
                    );
                },
            );
            ui.allocate_ui_with_layout(
                vec2(right_width, card_height),
                Layout::top_down(Align::Min),
                |ui| {
                    add_output = outputs_specifications_card(
                        ui,
                        &saved_outputs,
                        &saved_output_reports,
                        &specifications,
                        dataset.as_ref(),
                        card_height,
                        SetupCardBorder::SplitRight,
                    );
                },
            );
        });
        ui.painter().vline(
            row.response.rect.left() + left_width + divider * 0.5,
            row.response.rect.y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
    } else {
        add_variable = design_variables_card(
            ui,
            &design_variables,
            variable_card_height,
            SetupCardBorder::All,
        );
        add_output = outputs_specifications_card(
            ui,
            &saved_outputs,
            &saved_output_reports,
            &specifications,
            dataset.as_ref(),
            output_card_height,
            SetupCardBorder::All,
        );
    }

    if add_variable {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::DesignVariable(
            DesignVariableDraft::default(),
        ));
    } else if add_output {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::SavedOutput(
            SavedOutputDraft::default(),
        ));
    }
}

fn setup_card_height(row_height: f32, rows: usize) -> f32 {
    SETUP_CARD_HEADER_HEIGHT + SETUP_TABLE_HEADER_HEIGHT + row_height * rows.max(1) as f32
}

fn design_variables_card(
    ui: &mut Ui,
    variables: &[crate::state::DesignVariable],
    height: f32,
    border: SetupCardBorder,
) -> bool {
    setup_table_card(ui, "design-variables", height, border, |ui| {
        let add = setup_card_header(ui, "Design variables", "Add design variable");
        setup_table_row(
            ui,
            [0.36, 0.32, 0.32],
            ["NAME", "VALUE", "SCOPE"],
            true,
            [None, None, None],
        );
        if variables.is_empty() {
            setup_table_row(
                ui,
                [0.36, 0.32, 0.32],
                ["No design variables configured", "—", "optional"],
                false,
                [None, None, None],
            );
        } else {
            for variable in variables {
                setup_table_row(
                    ui,
                    [0.36, 0.32, 0.32],
                    [
                        variable.name.as_str(),
                        variable.expression.as_str(),
                        variable.scope.label(),
                    ],
                    false,
                    [None, None, None],
                );
            }
        }
        add
    })
}

fn outputs_specifications_card(
    ui: &mut Ui,
    outputs: &[crate::state::SavedOutput],
    output_reports: &[crate::simulation::SavedOutputPreflightReport],
    specifications: &[crate::state::SpecEntry],
    dataset: Option<&crate::state::SimulationRun>,
    height: f32,
    border: SetupCardBorder,
) -> bool {
    setup_table_card(ui, "outputs-specifications", height, border, |ui| {
        let add = setup_card_header(ui, "Outputs & specifications", "Add saved output");
        setup_table_row(
            ui,
            [0.42, 0.29, 0.29],
            ["EXPRESSION", "SPEC", "STATUS"],
            true,
            [None, None, None],
        );
        if outputs.is_empty() && specifications.is_empty() {
            setup_table_row(
                ui,
                [0.42, 0.29, 0.29],
                [
                    "No saved outputs or specifications configured",
                    "—",
                    "optional",
                ],
                false,
                [None, None, None],
            );
        } else {
            let t = Tokens::get(ui.ctx());
            for (index, output) in outputs.iter().enumerate() {
                let report = output_reports.get(index);
                let (status, color) = match report.map(|report| report.semantic_status()) {
                    Some(SavedOutputSemanticStatus::Valid { .. }) => ("valid", t.color.ok),
                    Some(SavedOutputSemanticStatus::RuntimeBound { .. }) => {
                        ("runtime-bound", t.color.warn)
                    }
                    Some(SavedOutputSemanticStatus::Invalid { reason }) => {
                        (reason.as_str(), t.color.err)
                    }
                    None => ("preflight report unavailable", t.color.err),
                };
                setup_table_row(
                    ui,
                    [0.42, 0.29, 0.29],
                    [
                        output.source_expression.as_str(),
                        output.status_label(),
                        status,
                    ],
                    false,
                    [None, None, Some(color)],
                );
            }
            for spec in specifications {
                let limit = specification_limit(spec);
                let evidence =
                    dataset.and_then(|run| measurement_in_output_dataset(run, &spec.measurement));
                let (status, color) = match evidence {
                    Some(evidence) if !evidence.measurement_passed => {
                        ("measurement failed", t.color.err)
                    }
                    Some(evidence) if spec.passes(evidence.value) => ("pass", t.color.ok),
                    Some(_) => ("fail", t.color.err),
                    None => ("no evidence", t.color.warn),
                };
                setup_table_row(
                    ui,
                    [0.42, 0.29, 0.29],
                    [spec.measurement.as_str(), limit.as_str(), status],
                    false,
                    [None, None, Some(color)],
                );
            }
        }
        add
    })
}

fn setup_table_card(
    ui: &mut Ui,
    card_id: &'static str,
    height: f32,
    border: SetupCardBorder,
    body: impl FnOnce(&mut Ui) -> bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let response = egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_width(width);
        ui.set_min_height(height);
        ui.spacing_mut().item_spacing.y = 0.0;
        // The mockup keeps all three columns in the card at narrow widths.
        // A fixed-width nested ScrollArea both hid the trailing column and
        // allowed sibling cards to alias egui scroll state; proportional rows
        // already provide the intended responsive behavior without state.
        // Keep every stateful descendant in a stable card-local namespace as
        // well. This prevents sibling setup cards from ever sharing widget or
        // scroll state if either card gains another interactive child later.
        ui.push_id(("simulation-setup-card", card_id), body).inner
    });
    let rect = response.response.rect;
    match border {
        SetupCardBorder::All => {
            ui.painter().rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
        }
        SetupCardBorder::SplitLeft | SetupCardBorder::SplitRight => {
            ui.painter().hline(
                rect.x_range(),
                rect.top() + 0.5,
                Stroke::new(1.0, t.color.border),
            );
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
            let x = if border == SetupCardBorder::SplitLeft {
                rect.left() + 0.5
            } else {
                rect.right() - 0.5
            };
            ui.painter()
                .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border));
        }
    };
    response.inner
}

fn setup_card_header(ui: &mut Ui, title: &str, add_tooltip: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (header, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), SETUP_CARD_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            header.left_top() + vec2(11.0, 0.0),
            egui::pos2(header.right() - 39.0, header.bottom()),
        ),
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let add_rect = Rect::from_center_size(
        egui::pos2(header.right() - 17.5, header.center().y),
        vec2(28.0, 27.0),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(add_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    IconButton::new(Icon::Add)
        .size(28.0, 27.0)
        .tooltip(add_tooltip)
        .show(&mut child)
        .clicked()
}

fn setup_table_row(
    ui: &mut Ui,
    fractions: [f32; 3],
    cells: [&str; 3],
    header: bool,
    cell_colors: [Option<Color32>; 3],
) {
    let t = Tokens::get(ui.ctx());
    let height = if header {
        SETUP_TABLE_HEADER_HEIGHT
    } else {
        t.metrics.row_h
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let rects = setup_table_column_rects(rect, fractions);
    let first = rects[0].right();
    let second = rects[1].right();
    for x in [first, second] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let default_color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    for index in 0..3 {
        paint_clipped_text(
            ui,
            rects[index].shrink2(vec2(8.0, 0.0)),
            cells[index],
            font.clone(),
            cell_colors[index].unwrap_or(default_color),
        );
    }
}

fn setup_table_column_rects(rect: Rect, fractions: [f32; 3]) -> [Rect; 3] {
    let first = rect.left() + rect.width() * fractions[0];
    let second = first + rect.width() * fractions[1];
    [
        Rect::from_min_max(rect.min, egui::pos2(first, rect.bottom())),
        Rect::from_min_max(
            egui::pos2(first, rect.top()),
            egui::pos2(second, rect.bottom()),
        ),
        Rect::from_min_max(egui::pos2(second, rect.top()), rect.max),
    ]
}

fn specification_limit(spec: &crate::state::SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => {
            format!("{minimum:.6}…{maximum:.6} {}", spec.unit)
        }
        (Some(minimum), None) => format!("≥ {minimum:.6} {}", spec.unit),
        (None, Some(maximum)) => format!("≤ {maximum:.6} {}", spec.unit),
        (None, None) => "waveform".to_owned(),
    }
}

fn selected_output_dataset(
    simulation: &crate::state::SimulationState,
) -> Option<&crate::state::SimulationRun> {
    simulation.active_run()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct OutputMeasurementEvidence {
    value: f64,
    measurement_passed: bool,
}

fn measurement_in_output_dataset(
    run: &crate::state::SimulationRun,
    name: &str,
) -> Option<OutputMeasurementEvidence> {
    run.analyses
        .iter()
        .rev()
        .filter(|analysis| analysis.success && analysis.provenance.is_some())
        .find_map(|analysis| {
            analysis
                .measurements
                .iter()
                .rev()
                .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
                .and_then(|measurement| {
                    measurement
                        .value
                        .filter(|value| value.is_finite())
                        .map(|value| OutputMeasurementEvidence {
                            value,
                            measurement_passed: measurement.passed && measurement.error.is_none(),
                        })
                })
        })
}

fn resolve_active_analysis_instance(app: &mut RSpiceApp) -> Result<(), String> {
    let current = app.state.workbench.active_analysis_instance;
    let legacy_index = app.state.workbench.active_analysis;
    let resolved = {
        let plan = app.state.sim_setup.stable_analysis_plan()?;
        match current {
            Some(id) if plan.instance(id).is_some() => Some(id),
            Some(_) => plan.instances().first().map(|instance| instance.id()),
            None => AnalysisKind::from_legacy_index(legacy_index)
                .and_then(|kind| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                })
                .or_else(|| plan.instances().first())
                .map(|instance| instance.id()),
        }
    };
    app.state.workbench.active_analysis_instance = resolved;
    Ok(())
}

fn selected_analysis(app: &RSpiceApp) -> Result<Option<SelectedAnalysis>, String> {
    let Some(id) = app.state.workbench.active_analysis_instance else {
        return Ok(None);
    };
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let Some((position, instance)) = plan
        .instances()
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.id() == id)
    else {
        return Err(format!(
            "selected analysis instance {id} is not present in the stable plan"
        ));
    };
    let issues = plan
        .validation_issues()
        .into_iter()
        .filter(|issue| issue_applies_to(issue, id))
        .collect();
    Ok(Some(SelectedAnalysis {
        id,
        kind: instance.kind(),
        draft: instance.draft().clone(),
        dependencies: instance.dependencies().to_vec(),
        lifecycle: instance.lifecycle(),
        position,
        plan_length: plan.instances().len(),
        issues,
    }))
}

fn commit_draft(app: &mut RSpiceApp, id: AnalysisInstanceId, draft: AnalysisDraft) {
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .edit(id, |target| target.clone_from(&draft))
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(((), receipt)) => {
            record_receipt(app, &receipt);
        }
        Err(error) => record_failure(app, "Edit", &error),
    }
}

type InsertAnalysisResult = Result<
    (
        AnalysisInstanceId,
        AnalysisLifecycleReceipt,
        Option<Result<AnalysisLifecycleReceipt, String>>,
    ),
    String,
>;

fn insert_analysis_instance(app: &mut RSpiceApp, kind: AnalysisKind) {
    app.state.sim_setup.palette_open = false;
    app.state.sim_setup.palette_query.clear();
    app.state.sim_setup.palette_active = 0;
    app.state.sim_setup.palette_scroll_to_active = false;
    if let Some(reason) = kind.execution_blocker() {
        record_failure(app, "Insert", reason);
        return;
    }
    let result: InsertAnalysisResult = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => match plan.insert(kind) {
            Ok((id, insert_receipt)) => {
                let bind_receipt = (!kind.prerequisites().is_empty()).then(|| {
                    plan.auto_bind_dependencies(id)
                        .map_err(|error| error.to_string())
                });
                Ok((id, insert_receipt, bind_receipt))
            }
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error),
    };
    match result {
        Ok((id, insert_receipt, bind_receipt)) => {
            app.state.workbench.active_analysis_instance = Some(id);
            match bind_receipt {
                None => record_receipt(app, &insert_receipt),
                Some(Ok(bind_receipt)) => {
                    let issue = app
                        .state
                        .sim_setup
                        .stable_analysis_plan()
                        .ok()
                        .and_then(|plan| {
                            plan.validation_issues()
                                .iter()
                                .find(|issue| issue_applies_to(issue, id))
                                .map(format_plan_issue)
                        });
                    let readiness = issue.map_or_else(
                        || "Dependencies are explicitly bound.".to_owned(),
                        |issue| format!("Preflight remains blocked: {issue}"),
                    );
                    app.state.workbench.analysis_lifecycle_status = format!(
                        "Receipts #{} and #{} committed for instance {id}. {} {} {readiness} Prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        bind_receipt.sequence(),
                        insert_receipt.detail(),
                        bind_receipt.detail(),
                    );
                }
                Some(Err(error)) => {
                    app.state.workbench.analysis_lifecycle_status = format!(
                        "Receipt #{} committed for instance {id}. {} Automatic dependency binding was rejected fail-closed: {error}. The inserted instance remains selected and preflight blocked; prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        insert_receipt.detail(),
                    );
                }
            }
        }
        Err(error) => record_failure(app, "Add analysis", &error),
    }
}

fn apply_analysis_action(app: &mut RSpiceApp, id: AnalysisInstanceId, action: AnalysisAction) {
    match action {
        AnalysisAction::Clone => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan.clone_instance(id).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok((clone_id, receipt)) => {
                    app.state.workbench.active_analysis_instance = Some(clone_id);
                    record_receipt(app, &receipt);
                }
                Err(error) => record_failure(app, "Clone", &error),
            }
        }
        AnalysisAction::Earlier(position) | AnalysisAction::Later(position) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .reorder(id, position)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Reorder", &error),
            }
        }
        AnalysisAction::BindDependencies => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .auto_bind_dependencies(id)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Bind dependencies", &error),
            }
        }
        AnalysisAction::Validate => validate_analysis_instance(app, id),
        AnalysisAction::Remove => remove_analysis_instance(app, id),
        AnalysisAction::SetEnabled(enabled) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .set_enabled(id, enabled)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Enable state", &error),
            }
        }
    }
}

fn validate_analysis_instance(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let result = (|| {
        let plan = app.state.sim_setup.stable_analysis_plan()?;
        let instance = plan
            .instance(id)
            .ok_or_else(|| format!("analysis instance {id} no longer exists"))?;
        if let Some(issue) = plan
            .validation_issues()
            .iter()
            .find(|issue| issue_applies_to(issue, id))
        {
            return Err(format_plan_issue(issue));
        }
        if let Some(error) = app
            .state
            .sim_setup
            .analysis_draft_validation_error(instance.draft())
        {
            return Err(error);
        }
        Ok(instance.kind())
    })();

    match result {
        Ok(kind) => {
            app.state.workbench.analysis_lifecycle_status = format!(
                "Validation passed for {} instance {id}. Dependency identity, order, and enabled state are valid for this instance.",
                kind.label()
            );
        }
        Err(error) => record_failure(app, "Validate", &error),
    }
}

fn remove_analysis_instance(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let prior_run_ids = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .map(|run| run.run_id)
        .collect();
    let position = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .position(|instance| instance.id() == id)
        })
        .unwrap_or(0);
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .remove(id, prior_run_ids)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(receipt) => {
            let next = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    let next_position = position.min(plan.instances().len().saturating_sub(1));
                    plan.instances()
                        .get(next_position)
                        .map(|instance| instance.id())
                });
            app.state.workbench.active_analysis_instance = next;
            record_receipt(app, &receipt);
        }
        Err(error) => record_failure(app, "Remove", &error),
    }
}

fn record_receipt(app: &mut RSpiceApp, receipt: &AnalysisLifecycleReceipt) {
    let related = receipt
        .related_instance_id()
        .map_or_else(String::new, |id| format!(" · related instance {id}"));
    app.state.workbench.analysis_lifecycle_status = format!(
        "Receipt #{} · {} committed for instance {}{related} · revision {} to {} · outcome {}. {} Prior datasets remain immutable.",
        receipt.sequence(),
        lifecycle_command_label(receipt.command()),
        receipt.instance_id(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.outcome(),
        receipt.detail(),
    );
}

fn record_failure(app: &mut RSpiceApp, action: &str, error: &str) {
    app.state.workbench.analysis_lifecycle_status = format!(
        "{action} rejected fail-closed: {error}. The stable plan is unchanged and prior datasets remain immutable."
    );
}

const fn lifecycle_command_label(command: AnalysisLifecycleCommand) -> &'static str {
    match command {
        AnalysisLifecycleCommand::Insert => "Insert",
        AnalysisLifecycleCommand::Edit => "Edit",
        AnalysisLifecycleCommand::Clone => "Clone",
        AnalysisLifecycleCommand::Disable => "Enable or disable",
        AnalysisLifecycleCommand::Reorder => "Reorder",
        AnalysisLifecycleCommand::Dependency => "Bind dependencies",
        AnalysisLifecycleCommand::Validate => "Validate",
        AnalysisLifecycleCommand::Preflight => "Preflight",
        AnalysisLifecycleCommand::Execute => "Execute",
        AnalysisLifecycleCommand::Remove => "Remove",
    }
}

const fn availability_label(kind: AnalysisKind) -> &'static str {
    kind.availability().label()
}

const fn analysis_catalog_group(kind: AnalysisKind) -> &'static str {
    kind.category().label
}

fn configured_pvt_count(instances: &[crate::simulation::plan::AnalysisInstance]) -> usize {
    let mut count = 0usize;
    let mut has_nominal_analysis = false;
    for instance in instances.iter().filter(|instance| instance.enabled()) {
        match instance.draft() {
            AnalysisDraft::Corner(draft) => {
                count = count
                    .saturating_add(draft.to_config().map_or(0, |config| config.num_corners()));
            }
            AnalysisDraft::Temperature(draft) => {
                count =
                    count.saturating_add(draft.to_config().map_or(0, |config| config.num_temps()));
            }
            AnalysisDraft::OperatingPoint(_)
            | AnalysisDraft::Transient(_)
            | AnalysisDraft::Ac(_)
            | AnalysisDraft::DcSweep(_)
            | AnalysisDraft::Noise(_)
            | AnalysisDraft::PoleZero(_)
            | AnalysisDraft::Sensitivity(_)
            | AnalysisDraft::MonteCarlo(_)
            | AnalysisDraft::Pss(_)
            | AnalysisDraft::Stb(_)
            | AnalysisDraft::HarmonicBalance(_)
            | AnalysisDraft::SParameter(_)
            | AnalysisDraft::Pac(_)
            | AnalysisDraft::Pnoise(_)
            | AnalysisDraft::Pxf(_)
            | AnalysisDraft::Pstb(_)
            | AnalysisDraft::TransferFunction(_)
            | AnalysisDraft::Envelope(_)
            | AnalysisDraft::Fourier(_)
            | AnalysisDraft::Reliability(_)
            | AnalysisDraft::Optimization(_)
            | AnalysisDraft::Soa(_)
            | AnalysisDraft::Disto(_)
            | AnalysisDraft::Qpss(_)
            | AnalysisDraft::Hbsp(_)
            | AnalysisDraft::Hbnoise(_)
            | AnalysisDraft::Psp(_)
            | AnalysisDraft::Qpac(_)
            | AnalysisDraft::Qpnoise(_)
            | AnalysisDraft::Qpxf(_)
            | AnalysisDraft::TransientNoise(_)
            | AnalysisDraft::DcMismatch(_) => has_nominal_analysis = true,
        }
    }
    count
        .saturating_add(usize::from(has_nominal_analysis))
        .max(1)
}

fn issue_applies_to(issue: &AnalysisPlanIssue, id: AnalysisInstanceId) -> bool {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => true,
        AnalysisPlanIssue::DuplicateInstanceId { id: issue_id }
        | AnalysisPlanIssue::ReusedTombstonedId { id: issue_id }
        | AnalysisPlanIssue::KindDraftMismatch { id: issue_id, .. }
        | AnalysisPlanIssue::InvalidInstanceRevision { id: issue_id }
        | AnalysisPlanIssue::InvalidLifecycle { id: issue_id, .. } => *issue_id == id,
        AnalysisPlanIssue::MissingPrerequisite { dependent, .. }
        | AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. }
        | AnalysisPlanIssue::DuplicateDependencyRole { dependent, .. }
        | AnalysisPlanIssue::SelfDependency { dependent }
        | AnalysisPlanIssue::DanglingDependency { dependent, .. }
        | AnalysisPlanIssue::WrongDependencyKind { dependent, .. }
        | AnalysisPlanIssue::DisabledDependency { dependent, .. }
        | AnalysisPlanIssue::DependencyNotEarlier { dependent, .. } => *dependent == id,
        AnalysisPlanIssue::DependencyCycle { members } => members.contains(&id),
        AnalysisPlanIssue::DuplicateTombstoneId { .. }
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. }
        | AnalysisPlanIssue::InvalidReceiptSequence { .. }
        | AnalysisPlanIssue::InvalidReceiptRevision { .. }
        | AnalysisPlanIssue::DanglingReceiptInstance { .. }
        | AnalysisPlanIssue::ReceiptKindMismatch { .. }
        | AnalysisPlanIssue::EmptyReceiptDetail { .. }
        | AnalysisPlanIssue::InvalidNextReceiptSequence { .. } => false,
    }
}

fn format_plan_issue(issue: &AnalysisPlanIssue) -> String {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => {
            "The simulation plan has no enabled analysis instances.".to_owned()
        }
        AnalysisPlanIssue::DuplicateInstanceId { id } => {
            format!("Stable analysis identity {id} is duplicated.")
        }
        AnalysisPlanIssue::DuplicateTombstoneId { id } => {
            format!("Retired analysis identity {id} has duplicate tombstones.")
        }
        AnalysisPlanIssue::ReusedTombstonedId { id } => {
            format!("Retired analysis identity {id} was reused by an active instance.")
        }
        AnalysisPlanIssue::KindDraftMismatch {
            id,
            expected,
            actual,
        } => format!(
            "Analysis {id} requires a {} draft, but contains {}.",
            expected.label(),
            actual.label()
        ),
        AnalysisPlanIssue::InvalidInstanceRevision { id } => {
            format!("Analysis {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidLifecycle { id, state, enabled } => {
            format!("Analysis {id} lifecycle {state} conflicts with enabled state {enabled}.")
        }
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} requires an earlier enabled {} instance.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::UnexpectedDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} does not accept {} as a prerequisite.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DuplicateDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} binds {} more than once.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::SelfDependency { dependent } => {
            format!("Analysis {dependent} cannot depend on itself.")
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target } => {
            format!("Analysis {dependent} references missing prerequisite instance {target}.")
        }
        AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            target,
            actual,
        } => format!(
            "Analysis {dependent} requires {} at {target}, but that instance is {}.",
            prerequisite.label(),
            actual.label()
        ),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} is disabled.")
        }
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} must appear earlier.")
        }
        AnalysisPlanIssue::DependencyCycle { members } => format!(
            "Analysis dependency cycle contains {} instance{}.",
            members.len(),
            if members.len() == 1 { "" } else { "s" }
        ),
        AnalysisPlanIssue::InvalidTombstoneRevision { id } => {
            format!("Retired analysis identity {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidReceiptSequence { sequence } => {
            format!("Lifecycle receipt sequence {sequence} is not contiguous.")
        }
        AnalysisPlanIssue::InvalidReceiptRevision { sequence } => {
            format!("Lifecycle receipt {sequence} has an invalid revision transition.")
        }
        AnalysisPlanIssue::DanglingReceiptInstance { sequence, id } => {
            format!("Lifecycle receipt {sequence} references unknown analysis instance {id}.")
        }
        AnalysisPlanIssue::ReceiptKindMismatch {
            sequence,
            expected,
            actual,
        } => format!(
            "Lifecycle receipt {sequence} identifies {}, but its retained analysis is {}.",
            actual.label(),
            expected.label()
        ),
        AnalysisPlanIssue::EmptyReceiptDetail { sequence } => {
            format!("Lifecycle receipt {sequence} has no status detail.")
        }
        AnalysisPlanIssue::InvalidNextReceiptSequence { expected, actual } => {
            format!("Next lifecycle receipt sequence is {actual}; expected {expected}.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationState};

    fn attributed(analysis: AnalysisResult) -> AnalysisResult {
        analysis.with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
                Vec::new(),
            )
            .expect("valid test provenance"),
        )
    }

    #[test]
    fn desktop_analysis_split_matches_mockup_ratio_and_minimums() {
        let (left, right) = analysis_split_widths(1001.0, 1_440.0);
        assert!((left - 340.0).abs() < f32::EPSILON);
        assert!((right - 660.0).abs() < f32::EPSILON);

        let (left, right) = analysis_split_widths(551.0, 1_440.0);
        assert!((left - 190.0).abs() < f32::EPSILON);
        assert!((right - 360.0).abs() < f32::EPSILON);

        let (left, right) = analysis_split_widths(751.0, 1_020.0);
        assert!((left - 217.5).abs() < f32::EPSILON);
        assert!((right - 532.5).abs() < f32::EPSILON);
    }

    #[test]
    fn responsive_breakpoints_match_mockup_contract() {
        assert_eq!(SIMULATION_STACK_BREAKPOINT, 820.0);
        assert_eq!(TITLE_ACTION_STACK_BREAKPOINT, 560.0);
        assert!(!analysis_workspace_is_split(820.0, 700.0));
        assert!(analysis_workspace_is_split(821.0, 506.0));
        assert!(!analysis_workspace_is_split(1_440.0, 505.0));
        assert!(!analysis_workspace_is_split(1_440.0, 550.0));
        assert!(analysis_workspace_is_split(1_440.0, 551.0));
        assert_eq!(analysis_split_min_width(1_020.0), 506.0);
        assert_eq!(analysis_split_min_width(1_021.0), 551.0);
    }

    #[test]
    fn responsive_surface_geometry_matches_mockup_contract() {
        assert_eq!(ANALYSIS_ROW_HEIGHT, 53.0);
        assert_eq!(ANALYSIS_INDEX_DIAMETER, 22.0);
        assert_eq!(ANALYSIS_ROW_LEFT_PADDING, 9.0);
        assert_eq!(ANALYSIS_STACK_TABLET_MIN_WIDTH, 175.0);
        assert_eq!(ANALYSIS_STACK_DESKTOP_MIN_WIDTH, 190.0);
        assert!(ANALYSIS_STACK_TABLET_MIN_WIDTH >= analysis_row_content_min_width());
        assert_eq!(PREFLIGHT_CELL_HEIGHT, 42.0);
        assert_eq!(STACKED_WORKSPACE_GAP, 9.0);
        assert_eq!(SETUP_CARD_HEADER_HEIGHT, 37.0);
        assert_eq!(SETUP_TABLE_HEADER_HEIGHT, 27.0);
        assert_eq!(setup_card_height(28.0, 0), 92.0);
        assert_eq!(setup_card_height(31.0, 3), 157.0);
        assert_eq!(analysis_column_min_height(720.0, 148.0), 572.0);
        assert_eq!(analysis_column_min_height(100.0, 120.0), 1.0);
        let row_rect = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(1_000.0, 572.0));
        let background = analysis_stack_background_rect(row_rect, 340.0);
        assert_eq!(background.min, row_rect.min);
        assert_eq!(background.width(), 340.0);
        assert_eq!(background.height(), row_rect.height());
    }

    #[test]
    fn narrow_setup_tables_keep_every_column_inside_the_card() {
        let card = Rect::from_min_size(egui::Pos2::ZERO, vec2(320.0, 28.0));
        for fractions in [[0.36, 0.32, 0.32], [0.42, 0.29, 0.29]] {
            let columns = setup_table_column_rects(card, fractions);
            assert_eq!(columns[0].left(), card.left());
            assert_eq!(columns[2].right(), card.right());
            assert!(columns.iter().all(|column| column.is_positive()));
            assert!(
                columns
                    .windows(2)
                    .all(|pair| pair[0].right() == pair[1].left())
            );
        }
    }

    #[test]
    fn sibling_setup_cards_render_without_egui_id_clashes() {
        fn collect_text(shape: &egui::epaint::Shape, rendered: &mut String) {
            match shape {
                egui::epaint::Shape::Text(text) => {
                    rendered.push_str(&text.galley.job.text);
                    rendered.push('\n');
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect_text(shape, rendered);
                    }
                }
                _ => {}
            }
        }

        for (width, design_border, outputs_border) in [
            (420.0, SetupCardBorder::All, SetupCardBorder::All),
            (
                960.0,
                SetupCardBorder::SplitLeft,
                SetupCardBorder::SplitRight,
            ),
        ] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 320.0))),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        design_variables_card(ui, &[], 92.0, design_border);
                        outputs_specifications_card(ui, &[], &[], &[], None, 92.0, outputs_border);
                    });
                },
            );
            let mut rendered = String::new();
            for clipped in &output.shapes {
                collect_text(&clipped.shape, &mut rendered);
            }

            assert!(rendered.contains("Design variables"));
            assert!(rendered.contains("Outputs & specifications"));
            assert!(
                !rendered.contains("First use of")
                    && !rendered.contains("Second use of")
                    && !rendered.contains("Double use of"),
                "egui rendered an ID-clash diagnostic at {width}px:\n{rendered}"
            );
        }
    }

    #[test]
    fn analysis_catalog_search_text_is_centered_in_its_48_point_row() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut query = "noise".to_owned();
        let mut geometry = None;
        let _output = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 720.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let output = ui
                        .allocate_ui_with_layout(
                            vec2(480.0, 48.0),
                            Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| analysis_catalog_search_field(&mut query).show(ui),
                        )
                        .inner;
                    geometry = Some((
                        output.response.rect,
                        output.galley_pos,
                        output.galley.size(),
                    ));
                });
            },
        );
        let (response, galley_pos, galley_size) = geometry.expect("search field rendered");

        assert!((response.height() - 48.0).abs() <= 0.5);
        assert!((galley_pos.y + galley_size.y * 0.5 - response.center().y).abs() <= 0.5);
    }

    #[test]
    fn saved_output_storage_preview_uses_exact_binary_units_and_analysis_count() {
        assert_eq!(format_storage_bytes(960), "960 B");
        assert_eq!(format_storage_bytes(1_536), "1.50 KiB");
        assert_eq!(format_storage_bytes(2 * 1024 * 1024), "2.00 MiB");
        assert_eq!(
            saved_output_storage_summary(&SavedOutputStorageEstimate::ExactBytes(960), 1),
            "960 B · 1 compatible analysis"
        );
        assert_eq!(
            saved_output_storage_summary(&SavedOutputStorageEstimate::ExactBytes(1_536), 2),
            "1.50 KiB · 2 compatible analyses"
        );
        assert_eq!(
            saved_output_storage_summary(
                &SavedOutputStorageEstimate::Indeterminate {
                    reason: "adaptive transient grid".to_owned(),
                },
                1,
            ),
            "indeterminate · adaptive transient grid"
        );
    }

    #[test]
    fn analysis_catalog_uses_the_mockup_dialog_and_row_contracts() {
        assert_eq!(ANALYSIS_CATALOG_GROUP_HEIGHT, 29.0);
        assert_eq!(ANALYSIS_CATALOG_ROW_HEIGHT, 57.0);
        assert_eq!(ANALYSIS_CATALOG_READINESS_WIDTH, 142.0);
        assert_eq!(analysis_catalog_column_count(1_199.99), 1);
        assert_eq!(analysis_catalog_column_count(1_200.0), 2);
        assert_eq!(
            analysis_catalog_readiness(AnalysisKind::Pac),
            Some("Preview engine · non-sign-off")
        );
        assert_eq!(analysis_catalog_readiness(AnalysisKind::Transient), None);
        assert_eq!(
            analysis_catalog_readiness(AnalysisKind::Qpss),
            Some("the QPSS spectral-lattice solver is not available in this engine build")
        );
        assert_eq!(
            analysis_catalog_disposition(&[], AnalysisKind::Qpss),
            "Unavailable"
        );
    }

    #[test]
    fn analysis_catalog_search_preserves_canonical_group_order() {
        let all = filtered_catalog_kinds("");
        let unavailable = [
            AnalysisKind::Qpss,
            AnalysisKind::Hbsp,
            AnalysisKind::Hbnoise,
            AnalysisKind::Psp,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
            AnalysisKind::TransientNoise,
            AnalysisKind::DcMismatch,
        ];
        assert_eq!(all.len(), AnalysisKind::ALL.len());
        assert!(unavailable.iter().all(|kind| all.contains(kind)));
        assert_eq!(all.first(), Some(&AnalysisKind::OperatingPoint));
        assert_eq!(all, AnalysisKind::MANIFEST_ORDER.to_vec());
        assert!(
            all.iter()
                .position(|kind| *kind == AnalysisKind::MonteCarlo)
                < all
                    .iter()
                    .position(|kind| *kind == AnalysisKind::Reliability)
        );
        assert_eq!(
            filtered_catalog_kinds("periodic noise"),
            vec![AnalysisKind::Pnoise, AnalysisKind::Qpnoise]
        );
        assert_eq!(
            filtered_catalog_kinds("spectral lattice"),
            vec![AnalysisKind::Qpss, AnalysisKind::Qpnoise]
        );
    }

    #[test]
    fn unavailable_analysis_cannot_be_inserted_through_the_surface_action() {
        let mut app = RSpiceApp::test_instance();
        let before = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .instances()
            .len();

        insert_analysis_instance(&mut app, AnalysisKind::Qpss);

        let after = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .instances()
            .len();
        assert_eq!(after, before);
        assert!(
            app.state
                .workbench
                .analysis_lifecycle_status
                .contains("not available")
        );
    }

    #[test]
    fn design_variable_workflow_commits_to_the_active_plan_atomically() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let revision_before = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision();
        let count_before = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .map_or(0, |payload| payload.design_variables.len());

        commit_design_variable(&mut app, &DesignVariableDraft::default())
            .expect("valid variable commits");

        let payload = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .expect("plan payload");
        assert_eq!(payload.design_variables.len(), count_before + 1);
        assert_eq!(
            payload
                .design_variables
                .last()
                .map(|variable| variable.name.as_str()),
            Some("RLOAD_TEST")
        );
        assert!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("stable plan")
                .revision()
                .get()
                > revision_before.get()
        );
    }

    #[test]
    fn invalid_design_variable_workflow_leaves_authoritative_state_unchanged() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let revision_before = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision();
        let count_before = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .map_or(0, |payload| payload.design_variables.len());
        let mut draft = DesignVariableDraft::default();
        draft.allowed_range = "20 kohm … 30 kohm".to_owned();

        assert!(commit_design_variable(&mut app, &draft).is_err());
        assert_eq!(
            app.state
                .workspace
                .active_plan_data(plan_id)
                .map_or(0, |payload| payload.design_variables.len()),
            count_before
        );
        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("stable plan")
                .revision(),
            revision_before
        );
    }

    #[test]
    fn saved_output_workflow_commits_a_typed_plan_contract() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let count_before = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .map_or(0, |payload| payload.saved_outputs.len());

        commit_saved_output(&mut app, &SavedOutputDraft::default()).expect("valid output commits");

        let output = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .expect("plan payload")
            .saved_outputs
            .last()
            .expect("saved output");
        assert_eq!(
            count_before + 1,
            app.state
                .workspace
                .active_plan_data(plan_id)
                .unwrap()
                .saved_outputs
                .len()
        );
        assert_eq!(output.name, "V(afe_out)");
        assert_eq!(output.source_expression, "V(afe_out)");
    }

    #[test]
    fn clone_workflow_creates_fresh_plan_and_payload_identities_without_results() {
        let mut app = RSpiceApp::test_instance();
        commit_design_variable(&mut app, &DesignVariableDraft::default()).expect("source variable");
        commit_saved_output(&mut app, &SavedOutputDraft::default()).expect("source output");
        let source_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable source plan")
            .id();
        let source_payload = app
            .state
            .workspace
            .active_plan_data(source_id)
            .expect("source payload")
            .clone();
        let retained_runs_before = app
            .state
            .simulation
            .runs
            .iter()
            .map(|run| (run.run_id, run.dataset_id, run.id))
            .collect::<Vec<_>>();
        let mut draft = ClonePlanDraft::for_source(app.state.sim_setup.active_plan_name().as_str());
        draft.name = "Independent characterization".to_owned();

        commit_clone_plan(&mut app, &draft).expect("valid clone");

        let clone_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("active clone")
            .id();
        assert_ne!(clone_id, source_id);
        assert_eq!(app.state.sim_setup.active_plan_name().as_str(), draft.name);
        let clone_payload = app
            .state
            .workspace
            .active_plan_data(clone_id)
            .expect("cloned payload");
        assert_eq!(clone_payload.design_variables.len(), 1);
        assert_eq!(clone_payload.saved_outputs.len(), 1);
        assert_ne!(
            clone_payload.design_variables[0].id,
            source_payload.design_variables[0].id
        );
        assert_ne!(
            clone_payload.saved_outputs[0].id,
            source_payload.saved_outputs[0].id
        );
        assert_eq!(
            app.state
                .simulation
                .runs
                .iter()
                .map(|run| (run.run_id, run.dataset_id, run.id))
                .collect::<Vec<_>>(),
            retained_runs_before
        );
    }

    #[test]
    fn cancelling_a_simulation_workflow_never_invokes_its_commit() {
        let mut app = RSpiceApp::test_instance();
        let source_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable source plan")
            .id();
        let inactive_before = app.state.sim_setup.inactive_plans().len();
        let mut draft = ClonePlanDraft::for_source(app.state.sim_setup.active_plan_name().as_str());
        draft.name = "Cancelled clone".to_owned();
        app.state.workbench.simulation_workflow =
            Some(SimulationWorkflowDialog::ClonePlan(draft.clone()));

        finish_workflow_choice(
            &egui::Context::default(),
            &mut app,
            DialogChoice::Cancelled,
            draft,
            commit_clone_plan,
        );

        assert!(app.state.workbench.simulation_workflow.is_none());
        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("stable source plan")
                .id(),
            source_id
        );
        assert_eq!(app.state.sim_setup.inactive_plans().len(), inactive_before);
    }

    #[test]
    fn output_specifications_never_mix_measurements_across_retained_datasets() {
        let mut older = SimulationRun::new(1);
        older.add_analysis(attributed(
            AnalysisResult::new(1, AnalysisType::Ac, "older")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
        ));
        let mut selected = SimulationRun::new(2);
        selected.add_analysis(attributed(
            AnalysisResult::new(1, AnalysisType::Ac, "selected")
                .with_measurements(vec![rspice_core::MeasureResult::success("bandwidth", 42.0)]),
        ));

        let selected_dataset = selected.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![older, selected];
        simulation.active_run_idx = Some(1);

        let run = selected_output_dataset(&simulation).expect("selected dataset");
        assert_eq!(run.dataset_id, selected_dataset);
        assert_eq!(
            measurement_in_output_dataset(run, "bandwidth"),
            Some(OutputMeasurementEvidence {
                value: 42.0,
                measurement_passed: true,
            })
        );
        assert_eq!(measurement_in_output_dataset(run, "gain"), None);
    }

    #[test]
    fn output_specifications_reject_unattributed_failed_and_non_finite_measurements() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "legacy")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
        );
        run.add_analysis(attributed(
            AnalysisResult::failed(2, AnalysisType::Ac, "failed", "solver failed")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 2.0)]),
        ));
        run.add_analysis(attributed(
            AnalysisResult::new(3, AnalysisType::Ac, "non-finite").with_measurements(vec![
                rspice_core::MeasureResult {
                    name: "gain".to_owned(),
                    value: Some(f64::NAN),
                    error: Some("non-finite".to_owned()),
                    passed: false,
                    expected: None,
                    tolerance: None,
                },
            ]),
        ));

        assert_eq!(measurement_in_output_dataset(&run, "gain"), None);
    }

    #[test]
    fn output_specifications_retain_finite_measurement_contract_failures() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(attributed(
            AnalysisResult::new(1, AnalysisType::Ac, "goal miss").with_measurements(vec![
                rspice_core::MeasureResult {
                    name: "gain".to_owned(),
                    value: Some(9.0),
                    error: Some("goal miss".to_owned()),
                    passed: false,
                    expected: Some(10.0),
                    tolerance: Some(0.1),
                },
            ]),
        ));

        assert_eq!(
            measurement_in_output_dataset(&run, "gain"),
            Some(OutputMeasurementEvidence {
                value: 9.0,
                measurement_passed: false,
            })
        );
    }

    #[test]
    fn output_specifications_require_an_explicit_active_dataset() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "retained")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
        );
        let mut simulation = SimulationState::default();
        simulation.runs.push(run);

        assert!(selected_output_dataset(&simulation).is_none());
    }
}
