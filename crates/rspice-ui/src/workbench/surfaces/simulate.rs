//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod advanced_options;
mod analysis_form;
mod catalog;
mod lifecycle;
mod output_evidence;
mod page_excitations;
mod page_kit;
mod page_models;
mod page_outputs;
mod page_runset;
mod page_save;
mod page_solver;
mod page_specs;
mod page_variables;
mod pages;
mod participation;
mod plan_manager;
mod readiness;
mod variable_import;
mod workflows;
mod workload;

use catalog::*;
use lifecycle::*;
use readiness::*;
use workflows::*;

use std::collections::HashSet;

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};

use crate::product::{AnalysisInstanceId, ContentDigest, SimulationPlanId};
use crate::simulation::dialog::{NoiseReferenceType, PssDialogState};
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDependencyRepairContext, AnalysisDraft, AnalysisKind,
    AnalysisLifecycleCommand, AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::simulation::{
    SavedOutputSemanticStatus, SavedOutputStorageEstimate, SimulationController,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, mono_input, select,
};
use crate::workbench::app_state::SimSetupState;
use crate::workbench::state::{
    AnalysisRunPointsDraft, ClonePlanDraft, DesignVariableDraft, RenameAnalysisDraft,
    SavedOutputDraft, SimulationPage, SimulationPlanManagerDraft, SimulationPlanManagerMode,
    SimulationWorkflowDialog, Workspace,
};
use crate::workbench::{AppState, RSpiceApp};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    PROPERTY_ROW_TRAILING_PAD, StatusMark, WorkbenchIcon, elide_text, heading, paint_status_mark,
    property_row, property_row_control_columns, property_row_toned, property_row_wrapped,
    status_dot, workspace_title_row,
};

const SIMULATION_STACK_BREAKPOINT: f32 = 820.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const TITLE_ACTION_SPACING: f32 = 6.0;
/// The narrowest the plan heading is allowed to become before the title row
/// gives up on holding the actions beside it.
const TITLE_HEADING_MIN_WIDTH: f32 = 220.0;
const ANALYSIS_ROW_HEIGHT: f32 = 53.0;
const ANALYSIS_GROUP_HEADER_HEIGHT: f32 = 26.0;
const ANALYSIS_INDEX_DIAMETER: f32 = 22.0;
const ANALYSIS_ROW_LEFT_PADDING: f32 = 9.0;
const ANALYSIS_INDEX_LABEL_GAP: f32 = 8.0;
const ANALYSIS_ROW_TRAILING_PADDING: f32 = 9.0;
const ANALYSIS_LABEL_MIN_WIDTH: f32 = 64.0;
pub(super) const ANALYSIS_SWITCH_WIDTH: f32 = 30.0;
const ANALYSIS_SWITCH_MAX_HIT_WIDTH: f32 = 44.0;
const ANALYSIS_SWITCH_LABEL_GAP: f32 = 7.0;
const ANALYSIS_STACK_TABLET_MIN_WIDTH: f32 = 175.0;
const ANALYSIS_STACK_DESKTOP_MIN_WIDTH: f32 = 190.0;
const ANALYSIS_EDITOR_TABLET_MIN_WIDTH: f32 = 330.0;
const ANALYSIS_EDITOR_DESKTOP_MIN_WIDTH: f32 = 360.0;
const PREFLIGHT_CELL_HEIGHT: f32 = 42.0;
/// Gap between the instance contract's two columns.
const CONTRACT_COLUMN_GAP: f32 = 10.0;
/// The narrowest instance contract that can carry its two columns side by side.
///
/// The mockup collapses this block, the analysis form's grid and the preflight
/// strip together at `@container (width <= 560px)`
/// (`styles/30-simulation/090-simulation-cockpit.css`). A CSS grid child
/// shrinks to whatever its track gives it; an egui control row does not — the
/// participation row is a select, a point picker and a stat laid out beside a
/// label column, and it has a measured floor. So the breakpoint here is that
/// floor doubled rather than the mockup's number, which is the same rule
/// resolved against the widget set that actually draws it.
const CONTRACT_SPLIT_MIN_WIDTH: f32 =
    2.0 * participation::CONTROL_ROW_MIN_WIDTH + CONTRACT_COLUMN_GAP;
const STACKED_WORKSPACE_GAP: f32 = 9.0;
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
    /// What this instance is called, resolved through the plan's fallback.
    name: String,
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
    /// What this instance is called. Resolved once, when the row is built, so
    /// the title, the accessibility name, and the enable switch cannot drift
    /// apart within a frame.
    name: String,
    enabled: bool,
    lifecycle: AnalysisLifecycleState,
    /// Where this instance stands in the plan's newest run, when that run
    /// authorized a task for it. `None` when the plan has never run, or when
    /// this instance was added since.
    run_state: Option<InstanceRunState>,
    /// The identity's leading characters, which is as much of it as a 53-point
    /// row can carry beside the summary. The full identity is a property row in
    /// the contract block, where it can be read and copied whole; leading the
    /// meta line with all thirty-six characters spent the row on a string
    /// nobody reads across and clipped the summary that says what will run.
    short_id: String,
    summary: String,
    /// What this instance costs the queue: points, tasks, modelled duration.
    /// `None` for a disabled instance, which contributes nothing and is priced
    /// nowhere. Projected from [`workload::PlanWorkload`], the one owner of
    /// that arithmetic, so a row and the Run Set page's rate table cannot
    /// disagree about the same instance.
    workload: Option<String>,
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
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    // A removal confirmed in the destructive review is applied here, by the
    // surface that owns the plan, rather than by the modal that asked.
    apply_confirmed_analysis_removal(app);
    if let Err(error) = resolve_active_analysis_instance(&mut app.state) {
        record_failure(&mut app.state, "Analysis selection", &error);
    }

    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        // The track this surface is laid out in, with the scrollbar's gutter
        // withheld from it whether or not a bar is showing.
        //
        // Reading `available_width()` inside the scroll area answers to whether
        // a bar is up *this pass*, and that is not a fixed question: a route's
        // height is a function of its width, a bar appears when the route is
        // taller than the viewport, and the bar's reveal is animated over some
        // ten passes. So a route that grew just past the viewport — opening the
        // advanced-options panel on Solver does exactly that — narrowed a
        // fraction of a point per pass for a third of a second, and every
        // control on the page walked left with it while the reader reached for
        // one. Spending the gutter unconditionally makes the width a function
        // of the frame's width alone, and the loop cannot form.
        let track = ui.available_width();
        let gutter = ui.spacing().scroll.allocated_width();
        let surface_width = (track - gutter).max(1.0);
        let output = ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .vertical_scroll_offset(app.state.workbench.simulation_surface_scroll_y)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let scroll_content_origin_y = ui.min_rect().top();
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                let page = app.state.workbench.simulation_page;
                if page == SimulationPage::Analyses {
                    workspace_title_row(ui, |ui| plan_heading(ui, app, surface_width));
                    analysis_workspace(ui, app, surface_width, scroll_content_origin_y);
                } else {
                    pages::show(ui, app, page);
                }
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
}

/// Paint whichever plan-editing overlay is open, over whatever workspace the
/// reader is on, and carry any refusal it raised to a toast.
///
/// The frame calls this, not [`show`]. Only one of the nine overlays is
/// genuinely chrome: the plan manager, which the toolbar's run configuration
/// chip and `Command::ManageSimulationPlans` open from the menu bar, the
/// palette and the shortcut map, on any workspace. The other seven drafts are
/// each opened by a control on one setup page — cloning the plan and renaming
/// an analysis from the Analyses route, a design variable from Variables, a
/// saved output from Outputs, and so on.
///
/// The catalogue is the reason they are hosted together anyway. Its invoker is
/// the navigator's creating action, which the panel draws on all nine setup
/// routes, and `Command::AddAnalysis`, which reaches it from the Simulate menu
/// and the palette on any workspace — but the catalogue itself was drawn from
/// the Analyses rail, so pressing that action anywhere else set
/// `palette_open` with nothing to render it. That flag is one of the terms of
/// [`AppState::application_modal_open`], so the press took every keyboard
/// shortcut in the application with it and left no painted modal to press
/// Escape on. A host that runs once a frame wherever the reader is standing
/// cannot arm an overlay that no pass draws.
///
/// The refusal drain is here for the same reason and not a weaker one: the
/// plan manager commits from any workspace, and the strip that would have
/// stated its refusal is drawn on one route.
pub(in crate::workbench) fn show_workflow_dialogs(ctx: &egui::Context, app: &mut RSpiceApp) {
    simulation_workflow_dialog(ctx, app);
    analysis_catalog(ctx, app);
    drain_lifecycle_refusal(ctx, &mut app.state);
}

/// Host the analysis catalogue, and act on the kind it chose.
///
/// The rows are resolved here rather than lent from the rail that used to draw
/// it. On eight of the nine setup routes, and on every other workspace,
/// nothing has resolved the plan by the time this runs, and a plan that does
/// not resolve at all yields none — so the window opens on an empty slice and
/// every row reads "Add instance", which is the honest disposition when
/// nothing can be said about what is already configured. Refusing to open
/// would be the worse answer: the reader pressed a control, and an insert
/// against a broken plan refuses in its own words rather than in silence.
fn analysis_catalog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.sim_setup.palette_open {
        return;
    }
    let participation = participation::PlanParticipation::resolve(&app.state);
    let rows = analysis_stack_rows(app, &participation).unwrap_or_default();
    let Some(kind) = analysis_catalog_window(ctx, &mut app.state.sim_setup, &rows) else {
        return;
    };
    let Some(id) = insert_analysis_instance(app, kind) else {
        return;
    };
    // Whoever adds an analysis means to configure it, and the catalogue can be
    // opened from anywhere. Carrying the reader to the one route that edits the
    // new instance is the same move `page_excitations::reveal` makes towards
    // Design: the insert alone would leave a receipt on a page nobody is on.
    app.state.workbench.active_analysis_instance = Some(id);
    app.state.workbench.simulation_page = SimulationPage::Analyses;
    app.state.workbench.activate(Workspace::Simulate);
}

/// Carry a refused plan command to a reader who is not on the Analyses page.
///
/// Only that one route draws the lifecycle strip, so this is the whole of the
/// error surface for the other seven. It sits here because this is the single
/// entry point every route passes through, which is what keeps the drain out of
/// the twenty-odd places that announce an outcome.
///
/// The guard is the outcome's sequence, not its text: three of the announcing
/// sites are on the render path rather than in a click handler, and a text
/// comparison would let a restated refusal toast again the moment an unrelated
/// receipt passed through. A receipt advances the guard without toasting, so a
/// registry edit never becomes noise.
fn drain_lifecycle_refusal(ctx: &egui::Context, state: &mut AppState) {
    let outcome = &state.workbench.analysis_lifecycle_status;
    if outcome.sequence() <= state.workbench.analysis_lifecycle_toasted_sequence {
        return;
    }
    let refusal = outcome.is_refusal().then(|| outcome.message().to_owned());
    state.workbench.analysis_lifecycle_toasted_sequence =
        state.workbench.analysis_lifecycle_status.sequence();
    if let Some(message) = refusal {
        state
            .ui
            .toasts
            .error_with_title(ctx, "Plan change refused", message);
    }
}

fn analysis_workspace(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    surface_width: f32,
    scroll_content_origin_y: f32,
) {
    let viewport_width = ui.ctx().content_rect().width();
    let responsive_width = viewport_width.min(surface_width);
    // Resolved once for the route and lent to both columns. Resolving it
    // expands the declared space, and the two columns each did it: the rail
    // through the workload it prices its rows from, the editor through the
    // participation its run-points control reads. One expansion answers both,
    // and it is the same one, so the two columns cannot report a different
    // space on the same frame.
    let participation = participation::PlanParticipation::resolve(&app.state);
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
                ordered_instance_stack(ui, app, &participation)
            });
            ui.allocate_ui_with_layout(
                vec2(right_width, column_min_height),
                Layout::top_down(Align::Min),
                |ui| {
                    analysis_editor(
                        ui,
                        app,
                        responsive_width,
                        scroll_content_origin_y,
                        &participation,
                    );
                },
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
        ordered_instance_stack(ui, app, &participation);
        ui.add_space(STACKED_WORKSPACE_GAP);
        analysis_editor(
            ui,
            app,
            responsive_width,
            scroll_content_origin_y,
            &participation,
        );
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

fn ordered_instance_stack(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    participation: &participation::PlanParticipation,
) {
    let rows = match analysis_stack_rows(app, participation) {
        Ok(rows) => rows,
        Err(error) => {
            flat_notice(ui, |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                page_kit::note_line(ui, &error, page_kit::Tone::Error);
            });
            return;
        }
    };
    let active = app.state.workbench.active_analysis_instance;
    let mut action = None;
    let mut add_analysis = false;
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.set_width(ui.available_width());
        if rows.is_empty() {
            // The notice used to name the navigator and stop there, which made
            // it a dead end on the two states where the navigator is not on
            // screen — the panel collapsed, and focus mode. The empty rail is
            // where a plan is started, so it carries the action itself.
            flat_notice(ui, |ui| {
                status_dot(ui, t.color.warn, "No analysis instances");
                page_kit::note_line(
                    ui,
                    "This plan runs nothing until it holds one.",
                    page_kit::Tone::Dim,
                );
                ui.add_space(4.0);
                add_analysis = Button::new(Command::AddAnalysis.spec().label)
                    .accent()
                    .min_width(ui.available_width())
                    .show(ui)
                    .clicked();
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

    if add_analysis {
        Command::AddAnalysis.execute(app);
    }

    match action {
        Some(StackAction::Select(id)) => {
            app.state.workbench.active_analysis_instance = Some(id);
        }
        Some(StackAction::SetEnabled(id, enabled)) => {
            apply_analysis_action(app, id, AnalysisAction::SetEnabled(enabled));
        }
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
            // The run chip is painted, so it reaches a screen reader only if
            // the row's own name carries it. A status a sighted reader can see
            // and an assistive one cannot is not a status the row has.
            match row.run_state {
                Some(state) => format!(
                    "Select {} analysis instance {}, {} in the last run",
                    row.name,
                    row.id,
                    state.label()
                ),
                None => format!("Select {} analysis instance {}", row.name, row.id),
            },
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
            format!("Enable {} instance {}", row.name, row.id),
        )
    });
    paint_switch(ui, switch_hit.center(), row.enabled, toggle.hovered(), rect);

    let text_left = rect.left()
        + ANALYSIS_ROW_LEFT_PADDING
        + ANALYSIS_INDEX_DIAMETER
        + ANALYSIS_INDEX_LABEL_GAP;
    let text_right = switch_hit.left() - ANALYSIS_SWITCH_LABEL_GAP;
    // The code says which kind, the name says which instance. An unnamed
    // instance resolves its name to the kind label, so this row reads exactly
    // as it did before anyone named anything.
    let first_line = format!("{} · {}", row.kind.stable_id().to_uppercase(), row.name);
    let second_line = format!("{} · {}", row.short_id, row.summary);
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
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top),
            egui::pos2(text_right, line_top + 13.0),
        ),
        &first_line,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text.gamma_multiply(opacity),
    );
    // What this instance costs the queue, at the end of the meta line. The rail
    // is where an operator decides which instances to leave enabled, and the
    // price of that decision was only readable one page away.
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cost_width = row.workload.as_ref().map_or(0.0, |cost| {
        ui.painter()
            .layout_no_wrap(cost.clone(), meta_font.clone(), t.color.text)
            .size()
            .x
            + ANALYSIS_SWITCH_LABEL_GAP
    });
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top + 14.0),
            egui::pos2((text_right - cost_width).max(text_left), line_top + 27.0),
        ),
        &second_line,
        meta_font.clone(),
        t.color.text_faint.gamma_multiply(opacity),
    );
    if let Some(cost) = &row.workload {
        paint_text_elided_to_fit(
            ui,
            Rect::from_min_max(
                egui::pos2(
                    (text_right - cost_width + ANALYSIS_SWITCH_LABEL_GAP).max(text_left),
                    line_top + 14.0,
                ),
                egui::pos2(text_right, line_top + 27.0),
            ),
            cost,
            meta_font,
            t.color.text_dim.gamma_multiply(opacity),
        );
    }
    // The run chip sits at the end of the status line rather than in the line,
    // because it is the one fact here that changes without the plan changing.
    // Its own colour, too: "failed" and "not production" are different kinds of
    // bad news and a single tone for the line would merge them.
    let run_chip = row.run_state.map(|state| {
        (
            state.label(),
            match state {
                InstanceRunState::Running => t.color.accent,
                InstanceRunState::Complete => t.color.ok,
                InstanceRunState::Failed => t.color.err,
                InstanceRunState::Queued => t.color.text_faint,
            },
        )
    });
    let chip_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let chip_width = run_chip.map_or(0.0, |(label, _)| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), chip_font.clone(), t.color.text)
            .size()
            .x
            + ANALYSIS_SWITCH_LABEL_GAP
    });
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top + 28.0),
            egui::pos2(
                (text_right - chip_width).max(text_left),
                rect.bottom() - 3.0,
            ),
        ),
        &format!("{status} · {}", row.lifecycle),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        status_color.gamma_multiply(opacity),
    );
    if let Some((label, color)) = run_chip {
        paint_text_elided_to_fit(
            ui,
            Rect::from_min_max(
                egui::pos2(
                    (text_right - chip_width + ANALYSIS_SWITCH_LABEL_GAP).max(text_left),
                    line_top + 28.0,
                ),
                egui::pos2(text_right, rect.bottom() - 3.0),
            ),
            label,
            chip_font,
            color.gamma_multiply(opacity),
        );
    }
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

pub(super) fn paint_switch(
    ui: &Ui,
    center: egui::Pos2,
    enabled: bool,
    hovered: bool,
    row_rect: Rect,
) {
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

/// Paint one line into `rect`, elided rather than cut.
///
/// Every row on this surface puts a name beside a status that is measured
/// first, so the space the name gets is whatever the status leaves. This used
/// to hard-clip at that boundary, which cuts a glyph in half and says nothing
/// about what was removed: the analysis header's identity line read
/// `transient · 4f2a1b3c-9d` at 1000 points and gave the reader no way to know
/// the identity continued — or that the lifecycle at the end of it was gone.
///
/// The mockup's rule for that row is `overflow:hidden; text-overflow:ellipsis`
/// and [`elide_text`] is this application's spelling of it. The clip rect stays
/// as the guarantee: the elision makes the line fit, and the clip is what makes
/// "fits" true rather than approximately true.
fn paint_text_elided_to_fit(ui: &Ui, rect: Rect, text: &str, font: egui::FontId, color: Color32) {
    let clipped = rect.intersect(ui.clip_rect());
    if !clipped.is_positive() {
        return;
    }
    let text = elide_text(ui, text, &font, clipped.width());
    if text.is_empty() {
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

/// What the plan's newest run says about one instance.
///
/// Deliberately no `Running` for an analysis that has produced nothing yet.
/// The only identity-backed evidence that a particular instance is executing is
/// the provisional result a live transient writes; everything else the session
/// knows — `is_running`, `progress`, the status line — is about the *run*, and
/// spreading it across every card would tell the reader that thirty analyses
/// are executing at once. A card that says "queued" while its analysis is in
/// fact running understates; a card that says "running" when it is not is a
/// lie, and the chip is worth having only if it never tells one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InstanceRunState {
    /// A live partial result exists for this instance: it is executing now.
    Running,
    Complete,
    Failed,
    /// The run's authorized queue holds a task for this instance that has not
    /// produced a result.
    Queued,
}

impl InstanceRunState {
    const fn label(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::Queued => "queued",
        }
    }
}

/// Resolve every instance's standing in the plan's newest run, once per frame.
///
/// Keyed by the provenance identity the result carries rather than by the
/// task's position in the receipt. Positional matching desynchronizes the
/// moment a task is skipped for a failed prerequisite, and a chip that shifts
/// one row down is worse than no chip: it attributes a failure to an analysis
/// that never ran.
fn instance_run_states(app: &RSpiceApp) -> Vec<(AnalysisInstanceId, InstanceRunState)> {
    let Some(PriorPlanRun { index, .. }) = prior_plan_run(app) else {
        return Vec::new();
    };
    let Some(run) = app.state.simulation.runs.get(index) else {
        return Vec::new();
    };
    let Some(receipt) = run.prepared_receipt() else {
        return Vec::new();
    };
    receipt
        .tasks()
        .iter()
        .map(|task| {
            let id = task.instance_id();
            let state = match run.find_analysis_by_source_instance(id) {
                // A live partial is written `success == false` with a reserved
                // message, so it has to be told from a real failure before the
                // success bit is read at all.
                Some(result) if result.is_live_partial() => InstanceRunState::Running,
                Some(result) if result.success => InstanceRunState::Complete,
                Some(_) => InstanceRunState::Failed,
                None => InstanceRunState::Queued,
            };
            (id, state)
        })
        .collect()
}

/// The leading characters of an instance identity, for the rail's meta line.
///
/// Elided by [`crate::product::short_identity`], which is where the width every
/// short identity in this workbench is abbreviated to is decided. Not a second
/// identity: nothing resolves an instance by this string, and the row's
/// accessibility name still carries the whole one.
fn short_instance_id(id: AnalysisInstanceId) -> String {
    crate::product::short_identity(id)
}

fn analysis_stack_rows(
    app: &RSpiceApp,
    participation: &participation::PlanParticipation,
) -> Result<Vec<AnalysisStackRow>, String> {
    let setup = &app.state.sim_setup;
    let plan = setup.stable_analysis_plan()?;
    let issues = plan.validation_issues();
    let run_states = instance_run_states(app);
    // Resolved once for the whole rail rather than once per row: the projection
    // prices every enabled instance in a single pass, and asking it per row
    // would expand the declared space as many times as there are analyses.
    // Priced against the participation the route already resolved, so the rail
    // costs no expansion of its own.
    let workload = workload::PlanWorkload::resolve_with(app, participation).ok();
    Ok(plan
        .instances()
        .iter()
        .map(|instance| {
            let closure_ids = dependency_closure_ids(plan, instance.id());
            AnalysisStackRow {
                id: instance.id(),
                kind: instance.kind(),
                name: instance.display_name().to_owned(),
                enabled: instance.enabled(),
                lifecycle: instance.lifecycle(),
                run_state: run_states
                    .iter()
                    .find(|(id, _)| *id == instance.id())
                    .map(|(_, state)| *state),
                short_id: short_instance_id(instance.id()),
                summary: setup.analysis_draft_summary(instance.draft()),
                workload: workload
                    .as_ref()
                    .and_then(|plan| plan.row_cost_for(instance.id())),
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

/// The newest run this plan produced that holds a dataset: where it sits in
/// history, which run it is, and whether an execution is still writing it.
#[derive(Debug, Clone, Copy)]
struct PriorPlanRun {
    index: usize,
    sequence: u64,
    executing: bool,
}

impl PriorPlanRun {
    /// How the plan heading names it, mid-sentence among the plan's standing
    /// facts.
    fn heading_claim(self) -> String {
        if self.executing {
            format!("Run {} in progress", self.sequence)
        } else {
            format!("prior Run {} immutable", self.sequence)
        }
    }

    /// How the analysis contract card names it, as a value on its own row.
    fn contract_summary(self) -> String {
        if self.executing {
            format!("Run {} in progress", self.sequence)
        } else {
            format!("Run {} retained · immutable", self.sequence)
        }
    }
}

/// The newest retained run this plan produced.
///
/// One derivation for the heading's claim about a prior dataset, the control
/// that opens it, and the contract card that summarizes it, so the three
/// cannot name different runs. Two of them used to re-derive it inline and
/// without the `analyses` guard, which is a different question — a run exists
/// from the moment it is started, and `start_run_with_receipt` puts it at the
/// front of history with its receipt attached and nothing in it. The heading
/// therefore announced "prior Run 7 immutable" for the run that was at that
/// moment executing, while the hop beside it stayed disabled saying no dataset
/// existed. Both sentences were about the same row.
///
/// `executing` is what keeps the surviving derivation honest once results do
/// land in that run: "immutable" is a claim about a sealed dataset, and the
/// run an execution still owns is not one. The hop stays live — Results shows
/// the partials as they arrive — and only the wording changes.
fn prior_plan_run(app: &RSpiceApp) -> Option<PriorPlanRun> {
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())?;
    let simulation = &app.state.simulation;
    let index = simulation.runs.iter().position(|run| {
        !run.analyses.is_empty()
            && run
                .prepared_receipt()
                .and_then(crate::state::PreparedRunReceipt::simulation_plan_id)
                == Some(plan_id)
    })?;
    let run = &simulation.runs[index];
    // Identity first, selection only as the fallback `has_active_execution`
    // itself falls back on: `is_running` without a sealed identity is the
    // runner's instantaneous activity, and the run being written is then the
    // one history has selected.
    let executing = simulation.has_active_execution()
        && match simulation.active_execution {
            Some(identity) => run.execution_identity() == Some(identity),
            None => simulation.active_run_idx == Some(index),
        };
    Some(PriorPlanRun {
        index,
        sequence: run.id,
        executing,
    })
}

fn plan_heading(ui: &mut Ui, app: &mut RSpiceApp, surface_width: f32) {
    let plan_name = app.state.sim_setup.active_plan_name().as_str().to_owned();
    // The heading states whether a prior dataset exists; the two controls
    // below make that statement reachable. All three read the one derivation,
    // so the sentence, the hop's label and the hop's own availability are
    // three views of a single answer rather than three answers.
    let prior = prior_plan_run(app);
    let (eyebrow, description, plan_available) = match app.state.sim_setup.stable_analysis_plan() {
        Ok(plan) => {
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .count();
            let prior = prior.map_or_else(
                || "no prior dataset".to_owned(),
                PriorPlanRun::heading_claim,
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
                    app.state.sim_setup.run_set.point_count().max(1),
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

    // The split toggle sits beside the hop because both answer "and now show
    // me what the last run produced" — one by leaving this page, one by
    // keeping it.
    let open_prior_label = prior.map_or_else(
        || "Open prior run".to_owned(),
        |prior| format!("Open Run {}", prior.sequence),
    );
    let split_availability = Command::ToggleResultsSplit.availability(app);
    let split_enabled = split_availability.is_available();
    let split_reason = match split_availability {
        crate::workbench::commands::CommandAvailability::Disabled(reason) => Some(reason),
        crate::workbench::commands::CommandAvailability::Available
        | crate::workbench::commands::CommandAvailability::Hidden => None,
    };
    let split_label = if app.state.workbench.split_with_results {
        "Unsplit results"
    } else {
        "Split with results"
    };
    let mut manage_plans = false;
    let mut clone_plan = false;
    let mut validate = false;
    let mut open_prior = false;
    let mut toggle_split = false;
    // The standing every other setup route states, from the same owner. This
    // route already offers the *action* — "Validate plan" is `PreflightChecks`
    // — and stated none of the standing, so the one page that lists what a run
    // will dispatch was the one page that never said whether the report
    // authorizing it still held.
    let mut rerun_preflight = false;
    let chip = pages::PreflightChip::resolve(app);
    let chip_reserve = chip.as_ref().map_or(0.0, |chip| chip.reserve(ui));
    let actions_width =
        title_action_group_width(ui, &open_prior_label, split_label, TITLE_ACTION_SPACING);
    let heading_width = ui.available_width() - actions_width - chip_reserve - TITLE_ACTION_SPACING;
    if surface_width > TITLE_ACTION_STACK_BREAKPOINT && heading_width >= TITLE_HEADING_MIN_WIDTH {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = TITLE_ACTION_SPACING;
            ui.allocate_ui_with_layout(
                vec2(heading_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| heading(ui, &eyebrow, &plan_name, &description),
            );
            if let Some(chip) = &chip {
                rerun_preflight = chip.show(ui);
            }
            open_prior = prior_run_button(ui, &open_prior_label, prior.is_some());
            toggle_split = split_button(ui, split_label, split_enabled, split_reason);
            manage_plans = Button::new("Plans")
                .enabled(plan_available)
                .show(ui)
                .clicked();
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
            ui.spacing_mut().item_spacing.x = TITLE_ACTION_SPACING;
            let action_width = ((ui.available_width() - 12.0) / 3.0).max(1.0);
            if let Some(chip) = &chip {
                rerun_preflight = chip.show(ui);
            }
            open_prior = prior_run_button(ui, &open_prior_label, prior.is_some());
            toggle_split = split_button(ui, split_label, split_enabled, split_reason);
            manage_plans = Button::new("Plans")
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
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
    if manage_plans {
        Command::ManageSimulationPlans.execute(app);
    }
    if clone_plan {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::ClonePlan(
            ClonePlanDraft::for_source(
                app.state.sim_setup.stable_analysis_plan().unwrap().id(),
                &plan_name,
            ),
        ));
    }
    // One dispatch for two controls, because they are one request: the chip is
    // the standing and the offer to refresh it, and the accent action is the
    // same refresh named as a step.
    if validate || rerun_preflight {
        Command::PreflightChecks.execute(app);
    }
    if open_prior && let Some(PriorPlanRun { index, .. }) = prior {
        // Selecting first is what carries the object: the command owns the
        // workspace half of the destination and nothing else.
        if app.state.simulation.select_run(index) {
            Command::OpenRunInResults.execute(app);
        }
    }
    if toggle_split {
        Command::ToggleResultsSplit.execute(app);
    }
}

/// What the five title-row actions will take, asked before the heading beside
/// them is laid out.
///
/// The heading grows into whatever the actions leave, so the row cannot be laid
/// out until their real extent is known. The width this replaces was authored
/// beside the row — a second account of the same arithmetic, and one that had
/// drifted: it reserved 570 points for a group that measures more, so at the
/// 1000-point gate the accent action was clipped by the surface edge.
fn title_action_group_width(
    ui: &mut Ui,
    open_prior_label: &str,
    split_label: &str,
    spacing: f32,
) -> f32 {
    let widths = [
        Button::new(open_prior_label).measured_width(ui),
        Button::new(split_label).measured_width(ui),
        Button::new("Plans").measured_width(ui),
        Button::new("Clone plan")
            .icon(Icon::Copy)
            .measured_width(ui),
        Button::new("Validate plan")
            .icon(Icon::Check)
            .accent()
            .measured_width(ui),
    ];
    widths.iter().sum::<f32>() + spacing * (widths.len() - 1) as f32
}

/// The hop to the dataset this plan last produced.
///
/// Disabled rather than hidden when there is none: "no prior dataset" is the
/// heading's own claim, and a control that vanishes with it would leave the
/// reader wondering whether one ever existed.
fn prior_run_button(ui: &mut Ui, label: &str, available: bool) -> bool {
    let response = Button::new(label).enabled(available).show(ui);
    if !available {
        response.on_hover_text(
            "This plan has produced no retained dataset yet, so there is nothing to open.",
        );
        return false;
    }
    response
        .on_hover_text("Activate this plan's newest retained dataset in Results")
        .clicked()
}

/// Show the retained dataset beside this page instead of instead of it.
fn split_button(ui: &mut Ui, label: &str, enabled: bool, reason: Option<&str>) -> bool {
    let response = Button::new(label).enabled(enabled).show(ui);
    if !enabled {
        response.on_hover_text(reason.unwrap_or("the split stage is unavailable for this route"));
        return false;
    }
    response
        .on_hover_text("Show the newest retained result document beside this plan")
        .clicked()
}

fn envelope_source_catalog(ui: &Ui, app: &RSpiceApp) -> EnvelopeSourceCatalog {
    let source_digest = envelope_source_catalog_input_digest(&app.state);
    let cache_id = egui::Id::new("simulation-envelope-source-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<EnvelopeSourceCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let catalog = build_envelope_source_catalog_with_digest(&app.state, source_digest);
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

fn build_envelope_source_catalog(app: &AppState) -> EnvelopeSourceCatalog {
    build_envelope_source_catalog_with_digest(app, envelope_source_catalog_input_digest(app))
}

fn envelope_source_catalog_input_digest(app: &AppState) -> ContentDigest {
    crate::simulation::controller::prepared_run::design_inspection_input_digest(app)
}

fn build_envelope_source_catalog_with_digest(
    app: &AppState,
    source_digest: ContentDigest,
) -> EnvelopeSourceCatalog {
    let catalog = (|| -> Result<(Vec<String>, String), String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                app,
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

/// The elaborated node and independent-source vocabulary the noise form
/// offers, measured from the same design deck the run would execute.
#[derive(Clone)]
struct NoiseDomainCatalog {
    source_digest: ContentDigest,
    nodes: Vec<String>,
    sources: Vec<String>,
    /// Why the design could not be elaborated, when it could not be.
    diagnostic: Option<String>,
}

impl NoiseDomainCatalog {
    fn domain(&self) -> analysis_form::NoiseDomain<'_> {
        analysis_form::NoiseDomain {
            nodes: &self.nodes,
            sources: &self.sources,
            unavailable: self.diagnostic.as_deref(),
        }
    }
}

/// Elaborating a design costs a full circuit build, so the catalog is keyed by
/// the same cheap digest the periodic-source catalog uses and rebuilt only
/// when the design behind it changes.
fn noise_domain_catalog(ui: &Ui, app: &RSpiceApp) -> NoiseDomainCatalog {
    let source_digest = envelope_source_catalog_input_digest(&app.state);
    let cache_id = egui::Id::new("simulation-noise-domain-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<NoiseDomainCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let catalog = build_noise_domain_catalog_with_digest(app, source_digest);
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

fn build_noise_domain_catalog_with_digest(
    app: &RSpiceApp,
    source_digest: ContentDigest,
) -> NoiseDomainCatalog {
    let catalog = (|| -> Result<(Vec<String>, Vec<String>), String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                &app.state,
            )
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        let circuit = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .build_circuit(&netlist)
            .map_err(|error| error.to_string())?;
        // `node_names_sorted` orders by MNA index, which is deck order. A
        // picker is read by eye, so the offered prefix is alphabetical -- the
        // same case-insensitive order the elaborated source list already
        // arrives in.
        let mut nodes = circuit.node_names_sorted();
        nodes.sort_by(|left, right| {
            left.to_ascii_lowercase()
                .cmp(&right.to_ascii_lowercase())
                .then_with(|| left.cmp(right))
        });
        Ok((nodes, circuit.independent_source_names()))
    })();
    let (nodes, sources, diagnostic) = match catalog {
        Ok((nodes, sources)) => (nodes, sources, None),
        Err(error) => (Vec::new(), Vec::new(), Some(error)),
    };
    NoiseDomainCatalog {
        source_digest,
        nodes,
        sources,
        diagnostic,
    }
}

fn analysis_editor(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    viewport_width: f32,
    scroll_content_origin_y: f32,
    resolved_participation: &participation::PlanParticipation,
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
                page_kit::note_line(
                    ui,
                    "The stable plan is empty. Add an analysis from Simulation Studio.",
                    page_kit::Tone::Dim,
                );
            });
            lifecycle_receipt_strip(ui, app);
            preflight_strip(ui, app);
            return;
        }
        Err(error) => {
            app.state.workbench.simulation_surface_editor_anchor_y = None;
            record_failure(&mut app.state, "Analysis editor", &error);
            flat_notice(ui, |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                page_kit::note_line(ui, &error, page_kit::Tone::Error);
            });
            lifecycle_receipt_strip(ui, app);
            preflight_strip(ui, app);
            return;
        }
    };

    // The card's summary and the hop under it are the same run, read once.
    // The summary used to find its own, over a predicate that accepted a run
    // holding nothing — so a started run made the card claim a retained
    // immutable dataset while the hop beside it stayed disabled.
    let prior_run = prior_plan_run(app);
    let prior_datasets = prior_run.map_or_else(
        || "No prior datasets".to_owned(),
        PriorPlanRun::contract_summary,
    );
    let mut draft = selected.draft.clone();
    let serialized_before = serde_json::to_vec(&draft);
    let mut action = None;
    let mut open_prior_dataset = false;
    let envelope_sources = matches!(draft, AnalysisDraft::Envelope(_) | AnalysisDraft::Pss(_))
        .then_some(&dependency_sources);
    let validation_error = analysis_validation_error(&app.state, &draft, envelope_sources);
    // Distinct fields, borrowed separately, so the projection can be written
    // into the state while the controller that reads it stays shared.
    let plan_statement = plan_statement_for(&mut app.state, &app.simulation_controller, &draft);
    let mut participation_action = None;
    // The run-space forms route to the page that owns the declaration rather
    // than editing it in place. Collected here and applied below, for the same
    // reason the participation action is: the frame is already borrowing `app`.
    let mut run_space_route = None;

    let t = Tokens::get(ui.ctx());
    let editor_response = egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let header_command = analysis_form_header(ui, &selected, validation_error.as_deref());
        if availability_label(selected.kind) != "Production" {
            capability_banner(ui, selected.kind);
        }
        lifecycle_toolbar(ui, &selected, &mut action);
        analysis_contract(
            ui,
            &selected,
            &plan_statement,
            ContractDatasets {
                summary: &prior_datasets,
                sequence: prior_run.map(|prior| prior.sequence),
                open: &mut open_prior_dataset,
            },
            resolved_participation,
            &mut participation_action,
            viewport_width <= 760.0,
            &mut action,
        );
        (
            analysis_form_body(ui, app, &mut draft, envelope_sources, &mut run_space_route),
            header_command,
        )
    });
    if let Some(page) = run_space_route {
        app.state.workbench.simulation_page = page;
    }
    let (form_anchor_y, header_command) = editor_response.inner;
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
                &mut app.state,
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
    // Dispatched before the draft comparison so that clicking the hop, which
    // takes focus off whatever field was being typed into, does not also
    // commit an unintended edit on the way out of the page.
    if open_prior_dataset
        && let Some(PriorPlanRun { index, .. }) = prior_run
        && app.state.simulation.select_run(index)
    {
        Command::OpenRunInResults.execute(app);
        return;
    }
    match header_command {
        Some(HeaderCommand::OpenOptions) => {
            if let Err(error) = page_solver::open_for_analysis(app, selected.id) {
                record_failure(&mut app.state, "Analysis options", &error);
            }
            ui.ctx().request_repaint();
            return;
        }
        Some(HeaderCommand::Rename) => {
            // Opened on what the header showed, and told which analysis it is
            // by the two facts the header's own second line states.
            app.state.workbench.simulation_workflow = Some(
                SimulationWorkflowDialog::RenameAnalysis(RenameAnalysisDraft::for_instance(
                    selected.id,
                    format!("{} · {}", selected.kind.label(), selected.id),
                    &selected.name,
                )),
            );
            ui.ctx().request_repaint();
            return;
        }
        None => {}
    }
    // Ahead of the draft comparison for the same reason the hop is: changing
    // participation moves focus off whatever field was being typed into, and
    // that must not also commit an edit to the analysis's own values.
    if let Some(asked) = participation_action {
        match asked {
            participation::ParticipationAction::Set(run_at) => {
                let _ = participation::commit_run_at(app, selected.id, run_at);
            }
            participation::ParticipationAction::ChoosePoints => {
                participation::open_point_picker(&mut app.state, selected.id);
            }
        }
        ui.ctx().request_repaint();
        return;
    }
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

/// What the analysis editor's header was asked to do this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderCommand {
    OpenOptions,
    Rename,
}

fn analysis_form_header(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    validation_error: Option<&str>,
) -> Option<HeaderCommand> {
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
    // The workbench's own buttons rather than egui's, laid out right-to-left
    // from the header's trailing inset. The rects this replaces were authored
    // beside the row at 26 points, but the egui buttons put into them drew at
    // their own 30, so the header's two controls stood taller than the
    // lifecycle row directly beneath them.
    let mut trailing = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_max(
                egui::pos2(text_left, rect.top()),
                egui::pos2(rect.right() - 11.0, rect.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    trailing.spacing_mut().item_spacing.x = 6.0;
    let options = Button::new("Options\u{2026}")
        .show(&mut trailing)
        .on_hover_text("Open typed numerical options for this exact analysis instance")
        .clicked();
    let rename = Button::new("Name\u{2026}")
        .show(&mut trailing)
        .on_hover_text("Name this analysis instance so every surface reports it by that name")
        .clicked();
    let actions_left = trailing.min_rect().left();
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
    let status_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let status_right = actions_left - 8.0;
    let status_width = ui
        .painter()
        .layout_no_wrap(status.to_owned(), status_font.clone(), color)
        .size()
        .x;
    ui.painter().text(
        egui::pos2(status_right, rect.center().y),
        Align2::RIGHT_CENTER,
        status,
        status_font,
        color,
    );
    let text_right = (status_right - status_width - 10.0).max(text_left);
    // The title is the instance, the line under it is the kind. When nothing
    // has been named the two agree, which is the state every plan starts in.
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 5.0),
            egui::pos2(text_right, rect.top() + 22.0),
        ),
        &selected.name,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 21.0),
            egui::pos2(text_right, rect.bottom() - 3.0),
        ),
        &format!(
            "{} · {} · lifecycle {}",
            selected.kind.label(),
            selected.id,
            selected.lifecycle
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    if let Some(error) = validation_error {
        response.on_hover_text(error);
    }
    match (rename, options) {
        (true, _) => Some(HeaderCommand::Rename),
        (false, true) => Some(HeaderCommand::OpenOptions),
        (false, false) => None,
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

/// What the contract card says about prior datasets, and the way out to one.
///
/// The summary sentence and the hop are passed together because they are one
/// claim: a card that names Run 7 and a control that opens something else
/// would be two answers to the same question.
struct ContractDatasets<'a> {
    summary: &'a str,
    /// The display sequence of the run the summary names, when it names one
    /// that retained a dataset.
    sequence: Option<u64>,
    open: &'a mut bool,
}

/// The engine directive this draft contributes to the deck.
///
/// Read through [`crate::simulation::SimulationController::analysis_draft_directive`],
/// which is the same path the directive-parse ratchet proves parses. What the
/// operator reads here is therefore the statement the engine is actually
/// offered, not a re-spelling of it that happens to look like one.
///
/// The builders read the legacy singleton slots rather than the draft, so the
/// draft has to be projected into the state first. Projected in place and
/// restored exactly, rather than onto a cloned application: a whole `AppState`
/// carries the design, the datasets and the model libraries, and cloning that
/// once a frame to read one line of text would be the most expensive thing on
/// this route.
///
/// The setup view is cloned for the restore, minus its plan catalog. Every
/// inactive plan is held there in full — drafts, receipts, tombstones — and the
/// clone carried all of them on every frame the Analyses page drew, to read one
/// directive line. The catalog is moved aside for the call instead and moved
/// back before this returns; nothing on the directive route reads it
/// (`analysis_draft_directive` builds its spec from the legacy analysis slots
/// and the options block, and the only reader of `inactive_plans` in
/// `crate::simulation` is the campaign activator).
///
/// Takes the two things it needs rather than the whole application: it has no
/// business reaching the schematic, the workspace or the documents, and saying
/// so in the signature is what keeps that true.
fn plan_statement_for(
    state: &mut AppState,
    controller: &SimulationController,
    draft: &AnalysisDraft,
) -> Result<String, String> {
    let stored_plans = std::mem::take(&mut state.sim_setup.inactive_plans);
    let restore = state.sim_setup.clone();
    state.sim_setup.apply_analysis_draft_projection(draft);
    let statement = controller.analysis_draft_directive(state, draft);
    state.sim_setup = restore;
    state.sim_setup.inactive_plans = stored_plans;
    statement
}

fn analysis_contract(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    plan_statement: &Result<String, String>,
    datasets: ContractDatasets<'_>,
    resolved: &participation::PlanParticipation,
    participation_action: &mut Option<participation::ParticipationAction>,
    stacked: bool,
    action: &mut Option<AnalysisAction>,
) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    // Its own width decides this, not only the viewport's. The block's left
    // column carries control rows with measured floors, and half of a 690-point
    // editor is not enough for the participation row's — which is how the
    // Analyses route came to paint the STB and Noise forms' right-hand column
    // outside the pane at the 1000-point gate.
    let stacked = stacked || content_width < CONTRACT_SPLIT_MIN_WIDTH;
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.x = CONTRACT_COLUMN_GAP;
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut property_action = None;
            let mut open_dataset = false;
            let mut properties = |ui: &mut Ui| {
                property_row(ui, "Stable instance identity", &selected.id.to_string());
                property_row(
                    ui,
                    "Ordered position",
                    &format!("{} / {}", selected.position + 1, selected.plan_length),
                );
                property_action = prerequisite_rows(ui, selected);
                if let Some(asked) = participation::participation_row(ui, resolved, selected.id) {
                    *participation_action = Some(asked);
                }
                property_row(ui, "Availability", availability_label(selected.kind));
                // What this instance actually contributes to the deck. Every
                // other row here describes the analysis; this one is the
                // analysis, in the dialect the engine reads.
                match plan_statement {
                    Ok(statement) => property_row(ui, "Plan statement", statement).on_hover_text(
                        "The exact directive this instance contributes to the executable deck.",
                    ),
                    Err(error) => property_row_toned(ui, "Plan statement", error, t.color.warn)
                        .on_hover_text(
                            "This analysis emits no directive until its configuration resolves, \
                             so it would contribute nothing to the deck.",
                        ),
                };
                property_row(ui, "Prior datasets", datasets.summary);
                // The row above names the run; this opens it. Kept as a
                // separate control rather than making the row clickable,
                // because every other property row here is read-only and one
                // secretly-live row is worse than an explicit button.
                if let Some(sequence) = datasets.sequence {
                    ui.add_space(4.0);
                    let label = format!("Open Run {sequence} in Results");
                    open_dataset |= Button::new(&label)
                        .show(ui)
                        .on_hover_text(
                            "Activate this immutable dataset in Results without changing the plan",
                        )
                        .clicked();
                }
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
            *datasets.open = open_dataset;
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
    route: &mut Option<crate::workbench::state::SimulationPage>,
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
    // Resolved from the plan, once, and handed to the form to read. The Corner
    // and Temperature forms state the declared run space; they must state the
    // plan's, and the only way to guarantee that is to give them no copy of
    // their own to state instead.
    let run_space = analysis_form::RunSpaceContext {
        run_set: &app.state.sim_setup.run_set,
        reference: app.state.sim_setup.reference_pvt,
        // Borrowed, not `plan_payload`: that clones the whole payload, and
        // this runs once per analysis form per frame for one Copy enum.
        nominal_failure: app
            .state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .map(|plan| plan.id())
            .and_then(|plan_id| app.state.workspace.plan_data(plan_id))
            .map(|payload| payload.specification_policy.nominal_failure)
            .unwrap_or_default(),
        model_binding_count: app.state.sim_setup.model_bindings.len(),
        parallelism: crate::simulation::execution::execution_target_parallelism(),
    };
    // Only the noise form reads the elaborated vocabulary, and measuring it
    // costs a circuit build. Every other analysis leaves it unmeasured.
    let noise_domain =
        matches!(draft, AnalysisDraft::Noise(_)).then(|| noise_domain_catalog(ui, app));
    // Only the stability form offers the drawing's loop probes, and the scan
    // allocates. Every other analysis leaves it unmeasured.
    let placed_loop_probes = matches!(draft, AnalysisDraft::Stb(_))
        .then(|| app.state.schematic.placed_loop_probe_names())
        .unwrap_or_default();
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
                envelope_sources.map_or(&[], |catalog| catalog.names.as_slice()),
                &placed_loop_probes,
                noise_domain
                    .as_ref()
                    .map(NoiseDomainCatalog::domain)
                    .unwrap_or_default(),
                op_context,
                &run_space,
                route,
            );
            // The form's own account of what this configuration will do. Some
            // of these sentences are the only place a setting's consequence is
            // stated — the DC retrace note names the two traces the run will
            // report — so discarding it left the field grid speaking for
            // itself and the reader to guess.
            page_kit::card_note(ui, note);
        })
        .response
        .rect
        .top()
}

fn analysis_validation_error(
    app: &AppState,
    draft: &AnalysisDraft,
    envelope_sources: Option<&EnvelopeSourceCatalog>,
) -> Option<String> {
    app.sim_setup
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

/// Paint a control row's label into the property list's label column, and
/// leave the cursor at the value column.
///
/// `allocate_exact_size` rather than a laid-out label: a label allocation that
/// shrinks to its own text is what put these rows' controls in a column of
/// their own.
pub(super) fn paint_control_row_label(ui: &mut Ui, label: &str, row_width: f32) {
    let (label_left, value_left) = property_row_control_columns(row_width);
    let (rect, _) = ui.allocate_exact_size(
        vec2(value_left, ui.spacing().interact_size.y),
        Sense::hover(),
    );
    ui.painter()
        .with_clip_rect(rect.intersect(ui.clip_rect()))
        .text(
            egui::pos2(rect.left() + label_left, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            Tokens::get(ui.ctx()).color.text_dim,
        );
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
            ui.spacing_mut().item_spacing.x = 0.0;
            let width = ui.available_width().max(1.0);
            // The property rows above and below this one are the same block,
            // so the control starts where their values do. The share this
            // replaces was a second account of the same column that collapsed
            // to the label's own text width, opening a second label column a
            // hundred points left of the first.
            paint_control_row_label(ui, &label, width);
            let select_width = (ui.available_width() - PROPERTY_ROW_TRAILING_PAD).max(1.0);
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

/// The studio's own specification join, reachable from the sibling surface that
/// reads the same limits.
///
/// Exposed for one reason: Verify answers the same limits against the same
/// dataset, and the two agreeing has to be a test rather than a convention. A
/// difference here is not cosmetic — it is one surface reporting a
/// specification passing while the other reports the retained point that broke
/// it.
#[cfg(test)]
pub(crate) fn measurement_in_output_dataset_for_test(
    run: &crate::state::SimulationRun,
    spec: &crate::state::SpecEntry,
) -> Option<f64> {
    output_evidence::measurement_in_output_dataset(run, spec).map(|evidence| evidence.value)
}

#[cfg(test)]
mod click_tests;
#[cfg(test)]
mod frame_cost_tests;
#[cfg(test)]
mod geometry_tests;
#[cfg(test)]
mod page_navigation_tests;
#[cfg(test)]
mod page_raster;
#[cfg(test)]
mod page_runset_parity_tests;
#[cfg(test)]
mod page_tests;
#[cfg(test)]
mod statement_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod workflow_dialog_tests;
#[cfg(test)]
mod workload_coherence_tests;
