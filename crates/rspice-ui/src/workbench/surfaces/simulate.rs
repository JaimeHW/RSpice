//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod analysis_form;
mod catalog;
mod lifecycle;
mod setup_tables;
mod workflows;

use catalog::*;
use lifecycle::*;
use setup_tables::*;
use workflows::*;

use std::collections::HashSet;

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};

use crate::product::{AnalysisInstanceId, ContentDigest};
use crate::simulation::dialog::{NoiseReferenceType, PssDialogState};
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDependencyRepairContext, AnalysisDraft, AnalysisKind,
    AnalysisLifecycleCommand, AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::simulation::{SavedOutputSemanticStatus, SavedOutputStorageEstimate};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, IconButton, mono_input, select,
};
use crate::workbench::RSpiceApp;
use crate::workbench::state::{
    ClonePlanDraft, DesignVariableDraft, SavedOutputDraft, SimulationWorkflowDialog,
};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    StatusMark, WorkbenchIcon, heading, paint_status_mark, property_row, status_dot,
    workspace_title_row,
};

const SIMULATION_STACK_BREAKPOINT: f32 = 820.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const ANALYSIS_ROW_HEIGHT: f32 = 53.0;
const ANALYSIS_GROUP_HEADER_HEIGHT: f32 = 26.0;
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
    prerequisite_roles: Vec<AnalysisKind>,
    prerequisite_candidates: Vec<(AnalysisKind, Vec<DependencyCandidate>)>,
    lifecycle: AnalysisLifecycleState,
    position: usize,
    plan_length: usize,
    issues: Vec<AnalysisPlanIssue>,
    contextual_dependency_error: Option<String>,
    repair_label: Option<String>,
    configure_pss_label: Option<String>,
    enabled: bool,
}

#[derive(Clone)]
struct DependencyCandidate {
    id: AnalysisInstanceId,
    label: String,
}

#[derive(Clone)]
struct EnvelopeSourceCatalog {
    source_digest: ContentDigest,
    names: Vec<String>,
    netlist_source: Option<String>,
    diagnostic: Option<String>,
}

impl EnvelopeSourceCatalog {
    fn dependency_repair_context(&self) -> AnalysisDependencyRepairContext {
        if let Some(diagnostic) = &self.diagnostic {
            return AnalysisDependencyRepairContext::periodic_sources_unavailable(format!(
                "the exact elaborated periodic-source catalog is unavailable: {diagnostic}"
            ));
        }
        let Some(source) = &self.netlist_source else {
            return AnalysisDependencyRepairContext::periodic_sources_unavailable(
                "the exact elaborated periodic-source source is unavailable",
            );
        };
        AnalysisDependencyRepairContext::exact_periodic_sources(source.clone())
            .unwrap_or_else(AnalysisDependencyRepairContext::periodic_sources_unavailable)
    }

    fn selection_error(&self, requested: &[String]) -> Option<String> {
        if requested.is_empty() {
            return None;
        }
        if let Some(diagnostic) = &self.diagnostic {
            return Some(format!(
                "The circuit modulation-source catalog is unavailable: {diagnostic}"
            ));
        }
        let missing = requested
            .iter()
            .filter(|requested| {
                !self
                    .names
                    .iter()
                    .any(|available| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        (!missing.is_empty()).then(|| {
            format!(
                "Unknown or DC-only circuit modulation source{}: {}",
                if missing.len() == 1 { "" } else { "s" },
                missing.join(", ")
            )
        })
    }

    #[cfg(test)]
    fn exact_periodic_selection_error(&self, requested: &[String]) -> Option<String> {
        if let Some(error) = self.selection_error(requested) {
            return Some(error.replace("modulation source", "periodic tone source"));
        }
        if let Some(diagnostic) = &self.diagnostic {
            return Some(format!(
                "The circuit periodic-source catalog is unavailable: {diagnostic}"
            ));
        }
        let omitted = self
            .names
            .iter()
            .filter(|available| {
                !requested
                    .iter()
                    .any(|requested| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        (!omitted.is_empty()).then(|| {
            format!(
                "PSS Tones must include every elaborated periodic source; omitted: {}",
                omitted.join(", ")
            )
        })
    }
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
    BindDependency {
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
    },
    RepairDependencies,
    PrepareAutonomousPss,
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
        let output = ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .vertical_scroll_offset(app.state.workbench.simulation_surface_scroll_y)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let scroll_content_origin_y = ui.min_rect().top();
                // Resolve the content width after the scroll area has reserved
                // its solid scrollbar track. Reusing the outer width clips the
                // right edge beneath that track.
                let surface_width = ui.available_width();
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| plan_heading(ui, app, surface_width));
                analysis_workspace(ui, app, surface_width, scroll_content_origin_y);
            });
        let pending_delta = std::mem::take(
            &mut app
                .state
                .workbench
                .simulation_surface_pending_scroll_delta_y,
        );
        app.state.workbench.simulation_surface_scroll_y =
            adjusted_scroll_for_stack_delta(output.state.offset.y, 0.0, pending_delta);
        if pending_delta != 0.0 {
            ui.ctx().request_repaint();
        }
    });
    simulation_workflow_dialog(ui.ctx(), app);
}

fn analysis_workspace(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    surface_width: f32,
    scroll_content_origin_y: f32,
) {
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
                |ui| analysis_editor(ui, app, responsive_width, scroll_content_origin_y),
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
        analysis_editor(ui, app, responsive_width, scroll_content_origin_y);
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
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ANALYSIS_GROUP_HEADER_HEIGHT),
        Sense::hover(),
    );
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
        .map(|instance| {
            let closure_ids = dependency_closure_ids(plan, instance.id());
            AnalysisStackRow {
                id: instance.id(),
                kind: instance.kind(),
                enabled: instance.enabled(),
                lifecycle: instance.lifecycle(),
                summary: setup.analysis_draft_summary(instance.draft()),
                issue_count: issues
                    .iter()
                    .filter(|issue| {
                        closure_ids
                            .iter()
                            .any(|closure_id| issue_applies_to(issue, *closure_id))
                    })
                    .count(),
            }
        })
        .collect())
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

fn envelope_source_catalog(ui: &Ui, app: &RSpiceApp) -> EnvelopeSourceCatalog {
    let source_digest = envelope_source_catalog_input_digest(app);
    let cache_id = egui::Id::new("simulation-envelope-source-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<EnvelopeSourceCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let catalog = build_envelope_source_catalog_with_digest(app, source_digest);
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

fn build_envelope_source_catalog(app: &RSpiceApp) -> EnvelopeSourceCatalog {
    build_envelope_source_catalog_with_digest(app, envelope_source_catalog_input_digest(app))
}

fn envelope_source_catalog_input_digest(app: &RSpiceApp) -> ContentDigest {
    crate::simulation::controller::prepared_run::design_inspection_input_digest(&app.state)
}

fn build_envelope_source_catalog_with_digest(
    app: &RSpiceApp,
    source_digest: ContentDigest,
) -> EnvelopeSourceCatalog {
    let catalog = (|| -> Result<(Vec<String>, String), String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                &app.state,
            )
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        let names = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .transient_source_names(&netlist)
            .map_err(|error| error.to_string())?;
        Ok((names, source))
    })();
    let (names, netlist_source, diagnostic) = match catalog {
        Ok((names, source)) => (names, Some(source), None),
        Err(error) => (Vec::new(), None, Some(error)),
    };
    EnvelopeSourceCatalog {
        source_digest,
        names,
        netlist_source,
        diagnostic,
    }
}

fn analysis_editor(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    viewport_width: f32,
    scroll_content_origin_y: f32,
) {
    let dependency_sources = envelope_source_catalog(ui, app);
    let selected = match selected_analysis(app, &dependency_sources) {
        Ok(Some(selected)) => selected,
        Ok(None) => {
            app.state.workbench.simulation_surface_editor_anchor_y = None;
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
            app.state.workbench.simulation_surface_editor_anchor_y = None;
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
    let envelope_sources = matches!(draft, AnalysisDraft::Envelope(_) | AnalysisDraft::Pss(_))
        .then_some(&dependency_sources);
    let validation_error = analysis_validation_error(app, &draft, envelope_sources);

    let t = Tokens::get(ui.ctx());
    let editor_response = egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        analysis_form_header(ui, &selected, validation_error.as_deref());
        if availability_label(selected.kind) != "Production" {
            capability_banner(ui, selected.kind);
        }
        lifecycle_toolbar(ui, &selected, &mut action);
        analysis_contract(
            ui,
            &selected,
            &prior_datasets,
            viewport_width <= 760.0,
            &mut action,
        );
        analysis_form_body(ui, app, &mut draft, envelope_sources)
    });
    let form_anchor_y = editor_response.inner;
    let form_anchor_content_y = content_space_anchor(form_anchor_y, scroll_content_origin_y);
    let editor_response = editor_response.response;
    ui.painter().hline(
        editor_response.rect.x_range(),
        editor_response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );

    if let Some(anchor_y) = app
        .state
        .workbench
        .simulation_surface_editor_anchor_y
        .take()
    {
        app.state
            .workbench
            .simulation_surface_pending_scroll_delta_y +=
            editor_anchor_scroll_delta(anchor_y, form_anchor_content_y);
    }

    let draft_changed = match (serialized_before, serde_json::to_vec(&draft)) {
        (Ok(before), Ok(after)) => before != after,
        (Err(error), _) | (_, Err(error)) => {
            record_failure(
                app,
                "Analysis edit",
                &format!("the draft could not be serialized exactly: {error}"),
            );
            app.state.workbench.simulation_surface_editor_anchor_y = Some(form_anchor_content_y);
            lifecycle_receipt_strip(ui, app);
            return;
        }
    };

    // Retain the selected form's content-space anchor every frame. Any edit,
    // dependency bind, enable toggle, insertion, removal, or repair that changes
    // content above it is then compensated on the next frame without maintaining
    // a fragile action whitelist.
    app.state.workbench.simulation_surface_editor_anchor_y = Some(form_anchor_content_y);
    if draft_changed {
        commit_draft(app, selected.id, draft);
        ui.ctx().request_repaint();
    }
    if let Some(action) = action {
        apply_analysis_action(app, selected.id, action);
        ui.ctx().request_repaint();
    }
    lifecycle_receipt_strip(ui, app);
    preflight_strip(ui, app);
    setup_tables(ui, app);
}

fn editor_anchor_scroll_delta(anchor_y: f32, current_y: f32) -> f32 {
    current_y - anchor_y
}

fn content_space_anchor(screen_y: f32, scroll_content_origin_y: f32) -> f32 {
    screen_y - scroll_content_origin_y
}

fn adjusted_scroll_for_stack_delta(scroll_y: f32, before: f32, after: f32) -> f32 {
    (scroll_y + after - before).max(0.0)
}

fn analysis_form_header(ui: &mut Ui, selected: &SelectedAnalysis, validation_error: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 42.0), Sense::hover());
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
    let (status, color) = if selected.issues.is_empty()
        && selected.contextual_dependency_error.is_none()
        && validation_error.is_none()
    {
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
    if let Some(error) = validation_error {
        response.on_hover_text(error);
    }
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
                let can_bind_existing = selected
                    .prerequisite_candidates
                    .iter()
                    .any(|(_, candidates)| !candidates.is_empty());
                if Button::new("Bind existing")
                    .enabled(can_bind_existing)
                    .show(ui)
                    .on_disabled_hover_text(
                        "No compatible enabled prerequisite is already positioned earlier in the plan. Use the guided prerequisite action below to create or repair the required chain.",
                    )
                    .clicked()
                {
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
    action: &mut Option<AnalysisAction>,
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
            let mut property_action = None;
            let mut properties = |ui: &mut Ui| {
                property_row(ui, "Stable instance identity", &selected.id.to_string());
                property_row(
                    ui,
                    "Ordered position",
                    &format!("{} / {}", selected.position + 1, selected.plan_length),
                );
                property_action = prerequisite_rows(ui, selected);
                property_row(ui, "Availability", availability_label(selected.kind));
                property_row(ui, "Prior datasets", prior_datasets);
            };
            let mut evidence = |ui: &mut Ui| {
                let (color, title, detail) = if let Some(issue) = selected.issues.first() {
                    (
                        t.color.err,
                        "Dependency graph blocked",
                        format_plan_issue(issue),
                    )
                } else if let Some(error) = &selected.contextual_dependency_error {
                    (t.color.err, "Dependency graph blocked", error.clone())
                } else if selected.repair_label.is_some() || selected.configure_pss_label.is_some() {
                    (
                        t.color.warn,
                        "Prerequisites not prepared",
                        "This disabled analysis does not block preflight, but its required prerequisite chain must be prepared before it can be enabled."
                            .to_owned(),
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
                if let Some(repair_label) = selected.repair_label.as_deref() {
                    ui.add_space(7.0);
                    if Button::new(repair_label)
                        .icon(Icon::Add)
                        .show(ui)
                        .on_hover_text(
                            "Atomically reuse, enable, move, or add the complete prerequisite chain before this analysis.",
                        )
                        .clicked()
                    {
                        *action = Some(AnalysisAction::RepairDependencies);
                    }
                }
                if let Some(configure_label) = selected.configure_pss_label.as_deref() {
                    ui.add_space(7.0);
                    if Button::new(configure_label)
                        .icon(Icon::Add)
                        .show(ui)
                        .on_hover_text(
                            "Select an existing prepared autonomous PSS when one is available; otherwise insert one disabled prerequisite with its inferable dependency chain, then open its complete configuration form.",
                        )
                        .clicked()
                    {
                        *action = Some(AnalysisAction::PrepareAutonomousPss);
                    }
                }
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
            if property_action.is_some() {
                *action = property_action;
            }
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn analysis_form_body(
    ui: &mut Ui,
    app: &RSpiceApp,
    draft: &mut AnalysisDraft,
    envelope_sources: Option<&EnvelopeSourceCatalog>,
) -> f32 {
    let project_revision = app.state.workspace.project.revision();
    let previous_state = app
        .state
        .simulation
        .newest_retained_op_state(project_revision)
        .is_some();
    let soa_violations = app
        .state
        .simulation
        .active_soa_violation_context(project_revision)
        .is_some();
    let op_context = analysis_form::OpContextAvailability {
        previous_state,
        soa_violations,
    };
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
            let _note = analysis_form::form(
                ui,
                draft,
                app.state.ui.preferences.quantity_presentation_policy(),
                app.state.ui.number_locale,
                envelope_sources.map_or(&[], |catalog| catalog.names.as_slice()),
                op_context,
            );
        })
        .response
        .rect
        .top()
}

fn analysis_validation_error(
    app: &RSpiceApp,
    draft: &AnalysisDraft,
    envelope_sources: Option<&EnvelopeSourceCatalog>,
) -> Option<String> {
    app.state
        .sim_setup
        .analysis_draft_validation_error(draft)
        .or_else(|| match (draft, envelope_sources) {
            (AnalysisDraft::Envelope(setup), Some(catalog)) => setup
                .to_config()
                .ok()
                .and_then(|config| catalog.selection_error(&config.modulation_sources)),
            (AnalysisDraft::Pss(setup), Some(catalog)) => catalog
                .dependency_repair_context()
                .validate_pss_sources(setup)
                .err(),
            _ => None,
        })
}

fn prerequisite_rows(ui: &mut Ui, selected: &SelectedAnalysis) -> Option<AnalysisAction> {
    if selected.prerequisite_roles.is_empty() {
        property_row(ui, "Prerequisites", "none declared");
        return None;
    }
    let mut requested = None;
    for prerequisite in &selected.prerequisite_roles {
        let bound = selected
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite() == *prerequisite);
        let target = bound.map_or_else(
            || {
                if selected.enabled {
                    "unbound · preflight blocked".to_owned()
                } else {
                    "unbound · required when enabled".to_owned()
                }
            },
            |dependency| dependency.target().to_string(),
        );
        let label = format!("{} prerequisite", prerequisite.stable_id().to_uppercase());
        let candidates = selected
            .prerequisite_candidates
            .iter()
            .find_map(|(kind, candidates)| (*kind == *prerequisite).then_some(candidates));
        let Some(candidates) = candidates.filter(|candidates| !candidates.is_empty()) else {
            property_row(ui, &label, &target);
            continue;
        };
        let options = candidates
            .iter()
            .map(|candidate| candidate.label.clone())
            .collect::<Vec<_>>();
        let current = bound
            .and_then(|dependency| {
                candidates
                    .iter()
                    .find(|candidate| candidate.id == dependency.target())
            })
            .map_or("Select compatible instance", |candidate| {
                candidate.label.as_str()
            });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let width = ui.available_width().max(1.0);
            let label_width = ((width - 8.0) * 0.4).max(1.0);
            ui.allocate_ui_with_layout(
                vec2(label_width, ui.spacing().interact_size.y),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.label(
                        egui::RichText::new(&label)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                    );
                },
            );
            let select_width = ui.available_width().max(1.0);
            let salt = format!(
                "analysis.{}.prerequisite.{}",
                selected.id,
                prerequisite.stable_id()
            );
            if let Some(index) = select(
                ui,
                &salt,
                &format!("Select {label}"),
                current,
                &options,
                select_width,
            ) && let Some(candidate) = candidates.get(index)
            {
                requested = Some(AnalysisAction::BindDependency {
                    prerequisite: *prerequisite,
                    target: candidate.id,
                });
            }
        });
    }
    requested
}

fn lifecycle_receipt_strip(ui: &mut Ui, app: &RSpiceApp) -> egui::Response {
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
    // The receipt is a status band, not an expanding log viewer. Its exact
    // height is stable across short edit receipts and verbose insertion or
    // dependency receipts so a form edit cannot change the scroll extent and
    // move every field in a near-bottom viewport. The complete immutable
    // receipt remains available through the hover text below.
    let height = lifecycle_receipt_height(compact);
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
    response.on_hover_text(detail)
}

const fn lifecycle_receipt_height(compact: bool) -> f32 {
    if compact { 64.0 } else { 40.0 }
}

fn preflight_strip(ui: &mut Ui, app: &RSpiceApp) {
    let configured_root = app.state.workspace.simulation_root_reference();
    let configured_schematic = app
        .state
        .workspace
        .simulation_root_schematic(&app.state.workspace.active_view, &app.state.schematic);
    let topology_ok =
        configured_schematic.is_some_and(|schematic| !schematic.components.is_empty());
    let project_revision = app.state.workspace.project.revision().get();
    let (topology_root, topology_revision, topology_closure) =
        super::super::preflight::configured_topology_revision(&app.state);
    let current_plan = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    let retained_report = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .filter(|report| {
            report.is_current_for(
                project_revision,
                &topology_root,
                topology_revision,
                &topology_closure,
                current_plan,
            )
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
            if configured_schematic.is_none() {
                format!(
                    "configured root {} is unresolved",
                    configured_root.display_path()
                )
            } else if !topology_ok {
                "configured design is empty".to_owned()
            } else if retained_report.is_none() {
                if app.state.workbench.preflight.report.is_some() {
                    "preflight receipt expired".to_owned()
                } else {
                    "preflight not run".to_owned()
                }
            } else if netlist_state == Some(false) {
                "preflight has design blockers".to_owned()
            } else {
                format!("revision {topology_revision} current")
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
            retained_report.map(|report| {
                report.is_runnable_for(
                    project_revision,
                    &topology_root,
                    topology_revision,
                    &topology_closure,
                    current_plan,
                )
            }),
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
            } else if retained_report.is_some_and(|report| {
                !report.is_runnable_for(
                    project_revision,
                    &topology_root,
                    topology_revision,
                    &topology_closure,
                    current_plan,
                )
            }) {
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
    let status = match pass {
        Some(true) => "passed",
        Some(false) => "blocked",
        None => "not available",
    };
    let response = ui.interact(rect, ui.id().with(("preflight-cell", name)), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{name}: {status}. {detail}"),
        )
    });
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
        Some(true) => (StatusMark::Success, t.color.ok),
        Some(false) => (StatusMark::Warning, t.color.err),
        None => (StatusMark::Neutral, t.color.text_faint),
    };
    paint_status_mark(
        &painter,
        Rect::from_center_size(rect.left_center() + vec2(14.0, 0.0), Vec2::splat(11.0)),
        mark,
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

#[cfg(test)]
fn dependency_repair_cta(
    plan: &crate::simulation::plan::SimulationPlan,
    issues: &[AnalysisPlanIssue],
    dependencies: &[AnalysisDependency],
) -> Option<String> {
    let context = AnalysisDependencyRepairContext::exact_periodic_sources(
        "periodic fixture\nVIN_DIFF in 0 SIN(0 1 1k)\nR1 in 0 1k\n.end\n",
    )
    .expect("test periodic-source fixture is exact");
    dependency_repair_cta_with_context(plan, issues, dependencies, &context)
}

fn compatible_dependency_repair_label(
    plan: &crate::simulation::plan::SimulationPlan,
    dependent: AnalysisInstanceId,
    prerequisite: AnalysisKind,
    repair_context: &AnalysisDependencyRepairContext,
) -> String {
    let qualifier = if prerequisite == AnalysisKind::Transient
        && plan
            .instance(dependent)
            .is_some_and(|instance| instance.kind() == AnalysisKind::Fourier)
    {
        "compatible "
    } else {
        ""
    };
    let Some(position) = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == dependent)
    else {
        return format!("Repair {} prerequisite", prerequisite.label());
    };
    let before = &plan.instances()[..position];
    let after = &plan.instances()[position + 1..];
    if before.iter().rev().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Bind {qualifier}{}", prerequisite.label())
    } else if before.iter().rev().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!("Enable {qualifier}{}", prerequisite.label())
    } else if after.iter().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Move {qualifier}{} earlier", prerequisite.label())
    } else if after.iter().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!(
            "Enable and move {qualifier}{} earlier",
            prerequisite.label()
        )
    } else {
        format!("Add {qualifier}{}", prerequisite.label())
    }
}

fn dependency_closure_ids(
    plan: &crate::simulation::plan::SimulationPlan,
    root: AnalysisInstanceId,
) -> HashSet<AnalysisInstanceId> {
    let mut closure = HashSet::new();
    if plan
        .instance(root)
        .is_some_and(|instance| !instance.enabled())
    {
        closure.insert(root);
        return closure;
    }
    let mut pending = vec![root];
    while let Some(id) = pending.pop() {
        if !closure.insert(id) {
            continue;
        }
        if let Some(instance) = plan.instance(id) {
            pending.extend(instance.dependencies().iter().filter_map(|dependency| {
                let target = dependency.target();
                let role_is_unique = instance
                    .dependencies()
                    .iter()
                    .filter(|candidate| candidate.prerequisite() == dependency.prerequisite())
                    .count()
                    == 1;
                if target == id
                    || !role_is_unique
                    || !instance
                        .prerequisite_roles()
                        .contains(&dependency.prerequisite())
                {
                    return None;
                }
                plan.instance(target)
                    .filter(|candidate| candidate.kind() == dependency.prerequisite())
                    .map(|candidate| candidate.id())
            }));
        }
    }
    closure
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
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            prerequisite,
            target,
            detail,
        } => format!(
            "Analysis {dependent} cannot use {} instance {target}: {detail}.",
            prerequisite.label()
        ),
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
mod tests;
