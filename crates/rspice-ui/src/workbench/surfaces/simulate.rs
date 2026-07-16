//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod analysis_form;

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};

use crate::common::RSpiceApp;
use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDraft, AnalysisKind, AnalysisLifecycleCommand,
    AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::commands::Command;
use super::super::design_system::{
    WorkbenchIcon, heading, property_row, status_dot, workspace_title_row,
};

const PLAN_NAME: &str = "Lab characterization";
const SIMULATION_STACK_BREAKPOINT: f32 = 820.0;
const SIMULATION_SPLIT_MIN_WIDTH: f32 = 506.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const ANALYSIS_ROW_HEIGHT: f32 = 53.0;
const PREFLIGHT_CELL_HEIGHT: f32 = 42.0;
const STACKED_WORKSPACE_GAP: f32 = 9.0;
const ANALYSIS_CATALOG_MAX_WIDTH: f32 = 1_180.0;
const ANALYSIS_CATALOG_MAX_HEIGHT: f32 = 780.0;
const ANALYSIS_CATALOG_GROUP_HEIGHT: f32 = 29.0;
const ANALYSIS_CATALOG_ROW_HEIGHT: f32 = 57.0;
const ANALYSIS_CATALOG_PHONE_ROW_HEIGHT: f32 = 72.0;
const ANALYSIS_CATALOG_READINESS_WIDTH: f32 = 142.0;
const ANALYSIS_CATALOG_WINDOW_CHROME_X: f32 = 26.0;
const ANALYSIS_CATALOG_WINDOW_CHROME_Y: f32 = 51.0;
const ANALYSIS_CATALOG_COMPACT_WINDOW_CHROME_Y: f32 = 66.0;
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

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if let Err(error) = resolve_active_analysis_instance(app) {
        record_failure(app, "Analysis selection", &error);
    }

    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        let surface_width = ui.available_width();
        ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| plan_heading(ui, app, surface_width));
                analysis_workspace(ui, app, surface_width);
            });
    });
}

fn analysis_workspace(ui: &mut Ui, app: &mut RSpiceApp, surface_width: f32) {
    let viewport_width = ui.ctx().content_rect().width();
    if analysis_workspace_is_split(viewport_width, surface_width) {
        let divider = 1.0;
        let available = ui.available_width();
        let (left_width, right_width) = analysis_split_widths(available, viewport_width);
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
                |ui| analysis_editor(ui, app, viewport_width),
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
        analysis_editor(ui, app, viewport_width);
    }
}

fn analysis_workspace_is_split(viewport_width: f32, surface_width: f32) -> bool {
    viewport_width > SIMULATION_STACK_BREAKPOINT && surface_width >= SIMULATION_SPLIT_MIN_WIDTH
}

fn analysis_split_widths(available: f32, viewport_width: f32) -> (f32, f32) {
    let usable = (available - 1.0).max(1.0);
    let (left_fraction, left_min, right_min) = if viewport_width <= 1_020.0 {
        (0.29, 175.0, 330.0)
    } else {
        (0.34, 190.0, 360.0)
    };
    let left = (usable * left_fraction)
        .max(left_min)
        .min((usable - right_min).max(left_min));
    (left, (usable - left).max(1.0))
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
    let index_center = egui::pos2(rect.left() + 20.0, rect.top() + 20.0);
    painter.circle_filled(
        index_center,
        11.0,
        if selected {
            t.color.accent_dim
        } else {
            t.color.bg_inset
        },
    );
    painter.circle_stroke(
        index_center,
        11.0,
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

    let switch_hit_size = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 30.0 };
    let switch_hit = Rect::from_center_size(
        egui::pos2(rect.right() - 9.0 - switch_hit_size * 0.5, rect.center().y),
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
    paint_switch(ui, switch_hit.center(), row.enabled, toggle.hovered());

    let text_left = rect.left() + 41.0;
    let text_right = switch_hit.left() - 7.0;
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
    theme::paint_focus_ring_outset(ui, &response, rect);
    theme::paint_focus_ring_outset(ui, &toggle, switch_hit);

    if toggle.clicked() {
        Some(StackAction::SetEnabled(row.id, !row.enabled))
    } else if response.clicked() {
        Some(StackAction::Select(row.id))
    } else {
        None
    }
}

fn paint_switch(ui: &Ui, center: egui::Pos2, enabled: bool, hovered: bool) {
    let t = Tokens::get(ui.ctx());
    let rect = Rect::from_center_size(center, vec2(30.0, 17.0));
    let fill = if enabled {
        t.color.accent
    } else if hovered {
        t.color.bg_hover
    } else {
        t.color.bg_inset
    };
    ui.painter().rect(
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
    ui.painter().circle_filled(
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
    ui.painter().with_clip_rect(rect).text(
        rect.left_center(),
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

    let mut open = true;
    let mut query = app.state.sim_setup.palette_query.clone();
    let mut active = app.state.sim_setup.palette_active;
    let mut chosen = None;
    let mut request_close = false;
    let scroll_to_active = app.state.sim_setup.palette_scroll_to_active;
    let dialog_size = analysis_catalog_content_size(ctx.content_rect().size());
    let dialog_width = dialog_size.x;
    let dialog_height = dialog_size.y;
    let catalog_columns = analysis_catalog_column_count(ctx.content_rect().width());
    egui::Window::new("Add analysis")
        .id(egui::Id::new("workbench.simulate.analysis_catalog"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size(vec2(dialog_width, dialog_height))
        .min_width(dialog_width)
        .max_width(dialog_width)
        .min_height(dialog_height)
        .max_height(dialog_height)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new("ANALYSIS CATALOG")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(Tokens::get(ui.ctx()).color.accent),
            );
            ui.label(
                egui::RichText::new("Create an independent stable analysis instance.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
            let search = ui.add(
                egui::TextEdit::singleline(&mut query)
                    .hint_text("Search analyses")
                    .desired_width(f32::INFINITY),
            );
            if search.changed() {
                active = 0;
            }
            if app.state.sim_setup.palette_query.is_empty() {
                search.request_focus();
            }

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
                    chosen = filtered.get(active).copied();
                }
            }
            if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                request_close = true;
            }

            ui.add_space(3.0);
            ScrollArea::vertical()
                .id_salt("workbench.simulate.analysis_catalog.rows")
                .auto_shrink([false, false])
                .max_height((dialog_height - 94.0).max(160.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    if filtered.is_empty() {
                        ui.label(
                            egui::RichText::new("No analysis matches this search.")
                                .color(Tokens::get(ui.ctx()).color.text_dim),
                        );
                        return;
                    }
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
                        analysis_catalog_group_header(ui, group, members.len());
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
                    ui.add_space(8.0);
                });
        });

    if let Some(kind) = chosen {
        *action = Some(StackAction::Insert(kind));
        request_close = true;
    }
    if request_close {
        open = false;
    }
    app.state.sim_setup.palette_open = open;
    app.state.sim_setup.palette_query = query;
    app.state.sim_setup.palette_active = active;
    app.state.sim_setup.palette_scroll_to_active = false;
}

fn analysis_catalog_outer_size(viewport: Vec2) -> Vec2 {
    let inset = if viewport.x <= TITLE_ACTION_STACK_BREAKPOINT {
        8.0
    } else {
        24.0
    };
    vec2(
        (viewport.x - inset).clamp(1.0, ANALYSIS_CATALOG_MAX_WIDTH),
        (viewport.y - inset).clamp(1.0, ANALYSIS_CATALOG_MAX_HEIGHT),
    )
}

fn analysis_catalog_content_size(viewport: Vec2) -> Vec2 {
    let outer = analysis_catalog_outer_size(viewport);
    let chrome_y = if viewport.x <= SIMULATION_STACK_BREAKPOINT {
        ANALYSIS_CATALOG_COMPACT_WINDOW_CHROME_Y
    } else {
        ANALYSIS_CATALOG_WINDOW_CHROME_Y
    };
    vec2(
        (outer.x - ANALYSIS_CATALOG_WINDOW_CHROME_X).max(1.0),
        (outer.y - chrome_y).max(1.0),
    )
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
    for chunk in members.chunks(columns) {
        if columns == 1 {
            let (index, kind) = chunk[0];
            let disposition = analysis_catalog_disposition(rows, kind);
            if analysis_catalog_row(ui, kind, &disposition, index == active, scroll_to_active) {
                chosen = Some(kind);
            }
            continue;
        }

        let gap = 1.0;
        let column_width = ((ui.available_width() - gap) / 2.0).max(1.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for &(index, kind) in chunk {
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
                        ) {
                            chosen = Some(kind);
                        }
                    },
                );
            }
            if chunk.len() == 1 {
                ui.allocate_exact_size(
                    vec2(column_width, ANALYSIS_CATALOG_ROW_HEIGHT),
                    Sense::hover(),
                );
            }
        });
    }
    chosen
}

fn analysis_catalog_disposition(rows: &[AnalysisStackRow], kind: AnalysisKind) -> String {
    let configured = rows.iter().filter(|row| row.kind == kind).count();
    if configured == 0 {
        "Add instance".to_owned()
    } else {
        format!("Add another · {configured} in plan")
    }
}

fn analysis_catalog_group_header(ui: &mut Ui, group: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ANALYSIS_CATALOG_GROUP_HEIGHT),
        Sense::hover(),
    );
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
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

fn analysis_catalog_row(
    ui: &mut Ui,
    kind: AnalysisKind,
    disposition: &str,
    selected: bool,
    scroll_to_active: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let compact = ui.ctx().content_rect().width() <= TITLE_ACTION_STACK_BREAKPOINT;
    let height = if compact {
        ANALYSIS_CATALOG_PHONE_ROW_HEIGHT
    } else {
        ANALYSIS_CATALOG_ROW_HEIGHT
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
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
            ui.is_enabled(),
            selected,
            format!("Add {} analysis instance", kind.label()),
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
            (rect.right() - ANALYSIS_CATALOG_READINESS_WIDTH).max(copy_left + 96.0);
        (readiness_left - 12.0, readiness_left)
    };
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(copy_left, rect.top() + 7.0),
            egui::pos2(copy_right, rect.top() + 27.0),
        ),
        kind.label(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(copy_left, rect.top() + 28.0),
            egui::pos2(
                copy_right,
                if compact {
                    rect.top() + 46.0
                } else {
                    rect.bottom() - 6.0
                },
            ),
        ),
        kind.detail(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );

    if !compact {
        painter.vline(
            readiness_left,
            rect.y_range().shrink(7.0),
            Stroke::new(1.0, t.color.border),
        );
    }
    let preview = kind.availability() != crate::simulation::plan::AnalysisAvailability::Production;
    let readiness_top = if compact {
        rect.top() + 45.0
    } else {
        rect.top() + 8.0
    };
    painter.circle_filled(
        egui::pos2(readiness_left + 12.0, readiness_top + 5.0),
        2.5,
        if availability_label(kind) == "Production" {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    let readiness_text_width = (rect.right() - 10.0 - readiness_left - 21.0).max(1.0);
    if compact {
        paint_clipped_text(
            ui,
            Rect::from_min_max(
                egui::pos2(readiness_left + 21.0, readiness_top),
                egui::pos2(rect.right() - 10.0, readiness_top + 14.0),
            ),
            disposition,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        );
        if preview {
            paint_clipped_text(
                ui,
                Rect::from_min_max(
                    egui::pos2(readiness_left + 21.0, readiness_top + 13.0),
                    egui::pos2(rect.right() - 10.0, readiness_top + 26.0),
                ),
                analysis_catalog_readiness(kind).unwrap_or_default(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
        }
    } else {
        let action_galley = ui.painter().layout(
            disposition.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
            readiness_text_width,
        );
        let action_height = action_galley.size().y;
        painter.galley(
            egui::pos2(readiness_left + 21.0, readiness_top),
            action_galley,
            t.color.text_dim,
        );
        if preview {
            let detail_galley = ui.painter().layout(
                analysis_catalog_readiness(kind)
                    .unwrap_or_default()
                    .to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
                readiness_text_width,
            );
            painter.galley(
                egui::pos2(readiness_left + 21.0, readiness_top + action_height),
                detail_galley,
                t.color.text_faint,
            );
        }
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
        .on_hover_text(format!("Add {} analysis instance", kind.label()))
        .clicked()
}

const fn analysis_catalog_readiness(kind: AnalysisKind) -> Option<&'static str> {
    if kind.execution_blocker().is_some() {
        Some("Engine unavailable · blocked")
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
        // Configuration identities remain durable for forward compatibility,
        // but a catalog row is an action promise. Do not offer insertion until
        // the corresponding execution engine exists.
        .filter(|kind| kind.execution_blocker().is_none())
        .filter(|kind| {
            query.is_empty()
                || format!(
                    "{} {} {} {} {} {} {} {}",
                    kind.stable_id(),
                    kind.code(),
                    kind.glyph(),
                    kind.label(),
                    kind.detail(),
                    analysis_catalog_group(*kind),
                    kind.category().detail,
                    availability_label(*kind),
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .collect()
}

fn plan_heading(ui: &mut Ui, app: &mut RSpiceApp, surface_width: f32) {
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
                    "{enabled} enabled instances · {} active · {} executable types in catalog · {} PVT points · {prior}",
                    plan.instances().len(),
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

    let mut validate = false;
    if surface_width > TITLE_ACTION_STACK_BREAKPOINT {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let heading_width = (ui.available_width() - 108.0).max(220.0);
            ui.allocate_ui_with_layout(
                vec2(heading_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| heading(ui, &eyebrow, PLAN_NAME, &description),
            );
            validate = Button::new("Validate plan")
                .icon(Icon::Check)
                .accent()
                .enabled(plan_available)
                .show(ui)
                .clicked();
        });
    } else {
        heading(ui, &eyebrow, PLAN_NAME, &description);
        ui.add_space(6.0);
        validate = Button::new("Validate plan")
            .icon(Icon::Check)
            .accent()
            .enabled(plan_available)
            .show(ui)
            .clicked();
    }
    if validate {
        Command::PreflightChecks.execute(app);
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
            specification_table(ui, app);
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
            specification_table(ui, app);
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
    specification_table(ui, app);
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
                if Button::new("Clone").show(ui).clicked() {
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

fn specification_table(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let row_height = output_table_row_height(ui.ctx().content_rect().width());
    let dataset = selected_output_dataset(&app.state.simulation);
    let dataset_context = dataset.map_or_else(
        || "NO ACTIVE DATASET".to_owned(),
        |run| format!("RUN {} · DATASET {}", run.id, run.dataset_id),
    );
    let header = ui
        .allocate_exact_size(vec2(ui.available_width(), 37.0), Sense::hover())
        .0;
    ui.painter().hline(
        header.x_range(),
        header.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let title_rect = Rect::from_min_max(
        header.left_top() + vec2(11.0, 0.0),
        egui::pos2(header.left() + header.width() * 0.48, header.bottom()),
    );
    paint_clipped_text(
        ui,
        title_rect,
        "Outputs & specifications",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let context_rect = Rect::from_min_max(
        egui::pos2(header.left() + header.width() * 0.50, header.top()),
        header.right_bottom() - vec2(11.0, 0.0),
    );
    ui.painter().with_clip_rect(context_rect).text(
        context_rect.right_center(),
        Align2::RIGHT_CENTER,
        &dataset_context,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );

    specification_table_row(ui, "EXPRESSION", "SPEC", "STATUS", true, None, row_height);
    if app.state.workspace.specs.is_empty() {
        specification_table_row(
            ui,
            "No project specifications configured",
            "—",
            "optional",
            false,
            Some(t.color.text_faint),
            row_height,
        );
    } else {
        for spec in &app.state.workspace.specs {
            let limit = specification_limit(spec);
            let evidence =
                dataset.and_then(|run| measurement_in_output_dataset(run, &spec.measurement));
            let (status, color) = match evidence {
                Some(evidence) if !evidence.measurement_passed => {
                    ("measurement failed".to_owned(), t.color.err)
                }
                Some(evidence) if spec.passes(evidence.value) => ("pass".to_owned(), t.color.ok),
                Some(_) => ("fail".to_owned(), t.color.err),
                None => ("no evidence".to_owned(), t.color.warn),
            };
            specification_table_row(
                ui,
                &spec.measurement,
                &limit,
                &status,
                false,
                Some(color),
                row_height,
            );
        }
    }
}

fn output_table_row_height(viewport_width: f32) -> f32 {
    if viewport_width <= TITLE_ACTION_STACK_BREAKPOINT {
        36.0
    } else if viewport_width <= SIMULATION_STACK_BREAKPOINT {
        31.0
    } else {
        28.0
    }
}

fn specification_table_row(
    ui: &mut Ui,
    expression: &str,
    limit: &str,
    status: &str,
    header: bool,
    status_color: Option<Color32>,
    row_height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { row_height };
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let first = rect.left() + rect.width() * 0.56;
    let second = rect.left() + rect.width() * 0.80;
    for x in [first, second] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    for (cell, text, cell_color) in [
        (
            Rect::from_min_max(rect.min, egui::pos2(first, rect.bottom())),
            expression,
            color,
        ),
        (
            Rect::from_min_max(
                egui::pos2(first, rect.top()),
                egui::pos2(second, rect.bottom()),
            ),
            limit,
            color,
        ),
        (
            Rect::from_min_max(egui::pos2(second, rect.top()), rect.max),
            status,
            status_color.unwrap_or(color),
        ),
    ] {
        paint_clipped_text(
            ui,
            cell.shrink2(vec2(8.0, 0.0)),
            text,
            font.clone(),
            cell_color,
        );
    }
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
    }

    #[test]
    fn responsive_surface_geometry_matches_mockup_contract() {
        assert_eq!(ANALYSIS_ROW_HEIGHT, 53.0);
        assert_eq!(PREFLIGHT_CELL_HEIGHT, 42.0);
        assert_eq!(STACKED_WORKSPACE_GAP, 9.0);
        assert_eq!(output_table_row_height(1_440.0), 28.0);
        assert_eq!(output_table_row_height(820.0), 31.0);
        assert_eq!(output_table_row_height(560.0), 36.0);
        assert_eq!(analysis_column_min_height(720.0, 148.0), 572.0);
        assert_eq!(analysis_column_min_height(100.0, 120.0), 1.0);
        let row_rect = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(1_000.0, 572.0));
        let background = analysis_stack_background_rect(row_rect, 340.0);
        assert_eq!(background.min, row_rect.min);
        assert_eq!(background.width(), 340.0);
        assert_eq!(background.height(), row_rect.height());
    }

    #[test]
    fn analysis_catalog_uses_the_mockup_dialog_and_row_contracts() {
        assert_eq!(
            analysis_catalog_outer_size(vec2(1_440.0, 900.0)),
            vec2(1_180.0, 780.0)
        );
        assert_eq!(
            analysis_catalog_outer_size(vec2(390.0, 844.0)),
            vec2(382.0, 780.0)
        );
        assert_eq!(
            analysis_catalog_content_size(vec2(1_440.0, 900.0)),
            vec2(1_154.0, 729.0)
        );
        assert_eq!(
            analysis_catalog_content_size(vec2(390.0, 844.0)),
            vec2(356.0, 714.0)
        );
        assert_eq!(ANALYSIS_CATALOG_GROUP_HEIGHT, 29.0);
        assert_eq!(ANALYSIS_CATALOG_ROW_HEIGHT, 57.0);
        assert_eq!(ANALYSIS_CATALOG_PHONE_ROW_HEIGHT, 72.0);
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
            Some("Engine unavailable · blocked")
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
        assert_eq!(all.len(), AnalysisKind::ALL.len() - unavailable.len());
        assert!(unavailable.iter().all(|kind| !all.contains(kind)));
        assert_eq!(all.first(), Some(&AnalysisKind::OperatingPoint));
        assert_eq!(
            all,
            AnalysisKind::MANIFEST_ORDER
                .into_iter()
                .filter(|kind| !unavailable.contains(kind))
                .collect::<Vec<_>>()
        );
        assert!(
            all.iter()
                .position(|kind| *kind == AnalysisKind::MonteCarlo)
                < all
                    .iter()
                    .position(|kind| *kind == AnalysisKind::Reliability)
        );
        assert_eq!(
            filtered_catalog_kinds("periodic noise"),
            vec![AnalysisKind::Pnoise]
        );
        assert!(filtered_catalog_kinds("spectral lattice").is_empty());
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
