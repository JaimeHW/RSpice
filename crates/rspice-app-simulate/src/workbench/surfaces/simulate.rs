//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod advanced_options;
mod analysis_editor;
mod analysis_form;
mod analysis_list;
pub mod catalog;
pub mod checkpoint_exchange;
mod contract_card;
pub mod lifecycle;
mod output_evidence;
mod page_connect_rules;
mod page_excitations;
pub mod page_kit;
pub mod page_models;
pub mod page_outputs;
pub mod page_runset;
pub mod page_save;
mod page_solver;
mod page_variables;
pub mod pages;
pub mod participation;
mod plan_configuration;
mod plan_issue_wording;
pub mod plan_manager;
mod title_row;
pub mod transient_checkpoint_exchange;
pub mod variable_import;
pub mod workflows;
pub mod workload;

use analysis_editor::*;
use analysis_list::*;
use catalog::*;
use contract_card::*;
use lifecycle::*;
use plan_issue_wording::*;
use title_row::*;
use workflows::*;

pub use contract_card::{engineering_statement, plan_statement_for};
pub use pages::reads_foreign_answers;

use std::collections::HashSet;

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, vec2};

use crate::product::{AnalysisInstanceId, ContentDigest, SimulationPlanId};
use crate::simulation::SimulationController;
use crate::simulation::dialog::{NoiseReferenceType, PssDialogState};
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDependencyRepairContext, AnalysisDraft, AnalysisKind,
    AnalysisLifecycleCommand, AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::state::SimulationPage;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, Problem, mono_input,
    paint_problem_mark, select,
};
use crate::workbench::app_state::{Model, SimSetupState};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{CommandSet, Cx};
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::slots::SimulateState;
use crate::workbench::state::{
    AnalysisRunPointsDraft, ClonePlanDraft, DesignVariableDraft, RenameAnalysisDraft,
    SavedOutputDraft, SimulationPlanManagerDraft, SimulationPlanManagerMode,
    SimulationWorkflowDialog, WorkbenchSession, Workspace,
};
use rspice_simulation::output_contract::SavedOutputSemanticStatus;
use rspice_simulation::output_contract::SavedOutputStorageEstimate;
use rspice_simulation::runner::{TfRunConfig, infer_tf_run_config};

use super::super::design_system::{
    StatusMark, WorkbenchIcon, elide_text, heading, property_row, property_row_control_columns,
    property_row_problem, property_row_wrapped, workspace_title_row,
};
use crate::workbench::simulate::commands::SimulateCommand;

/// The window width at and under which the Analyses route stacks its list
/// over the editor: a phone's. A desktop window keeps the list a column
/// beside the editor however much of the centre the docks take.
const SIMULATION_STACK_BREAKPOINT: f32 = 820.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
pub(super) use crate::ui::widgets::SWITCH_WIDTH as ANALYSIS_SWITCH_WIDTH;
pub(super) use crate::ui::widgets::paint_switch;
/// Side of a problem mark painted into a row's line, and the advance from its
/// left edge to the words after it: read from the kit's own row metrics.
use crate::ui::widgets::{
    PROBLEM_MARK_ADVANCE as ROW_MARK_ADVANCE, PROBLEM_MARK_SIDE as ROW_MARK_SIDE,
};
const ANALYSIS_STACK_TABLET_MIN_WIDTH: f32 = 175.0;
const ANALYSIS_STACK_DESKTOP_MIN_WIDTH: f32 = 190.0;
const ANALYSIS_EDITOR_TABLET_MIN_WIDTH: f32 = 330.0;
const ANALYSIS_EDITOR_DESKTOP_MIN_WIDTH: f32 = 360.0;
const STACKED_WORKSPACE_GAP: f32 = 9.0;
pub const ANALYSIS_CATEGORY_ORDER: [&str; 10] = [
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

#[derive(Debug, Clone, Copy)]
pub enum AnalysisAction {
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

/// Apply what the frame before left for the surface. A removal confirmed in
/// the destructive review is applied here, by the surface that owns the
/// plan, rather than by the modal that asked, and the hop a refused removal
/// offers is taken here for the same reason. Both land before the shell
/// takes the dispatcher's answers the surface reads, so those answers are
/// taken on the plan and the route it draws.
pub fn prepare(cx: &mut Cx<'_, SimulateFeature>) {
    apply_confirmed_analysis_removal(cx);
    lifecycle::apply_requested_blocker_reveal(cx.state);
    if let Err(error) = resolve_active_analysis_instance(cx.model, cx.state) {
        record_failure(cx, "Analysis selection", &error);
    }
}

/// Draw the surface [`prepare`] readied.
pub fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        // A run of the plan held for more memory than is free asks above
        // every page of it: Run anyway, or Stop, which Stop's command
        // carries out as it stops any run.
        memory_band(ui, cx);
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
        let page = cx.state.simulation_page;
        let window_width = ui.ctx().content_rect().width();
        let responsive_width = window_width.min(surface_width);
        if page == SimulationPage::Analyses && analysis_workspace_is_split(window_width) {
            split_analysis_route(ui, cx, surface_width, responsive_width);
            return;
        }
        let output = ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .vertical_scroll_offset(cx.state.simulation_surface_scroll_y)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let scroll_content_origin_y = ui.min_rect().top();
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                if page == SimulationPage::Analyses {
                    // A narrow surface: the stack above the editor, both in
                    // this one scroll, on one resolution of the plan's
                    // participation as on the split route.
                    workspace_title_row(ui, |ui| plan_heading(ui, cx, surface_width));
                    let participation = participation::PlanParticipation::resolve(cx.model);
                    ordered_instance_stack(ui, cx, &participation);
                    pages::dispatch_contract(ui, cx.model, cx.runs);
                    ui.add_space(STACKED_WORKSPACE_GAP);
                    analysis_editor(
                        ui,
                        cx,
                        responsive_width,
                        scroll_content_origin_y,
                        &participation,
                    );
                } else {
                    pages::show(ui, cx, page);
                }
            });
        settle_surface_scroll(ui, cx.state, output.state.offset.y);
    });
}

/// Keep the offset the surface's scroll ended on, moved by whatever the
/// editor's anchor asked for this frame.
fn settle_surface_scroll(ui: &Ui, state: &mut SimulateState, offset_y: f32) {
    let pending_delta = std::mem::take(&mut state.simulation_surface_pending_scroll_delta_y);
    state.simulation_surface_scroll_y =
        adjusted_scroll_for_stack_delta(offset_y, 0.0, pending_delta);
    if pending_delta != 0.0 {
        ui.ctx().request_repaint();
    }
}

impl SimulateFeature {
    /// Paint whichever plan-editing overlay is open, over whatever workspace the
    /// reader is on, and carry any refusal it raised to a toast.
    ///
    /// The frame calls this, not [`show`]. Only one of the nine overlays is
    /// genuinely chrome: the plan manager, which the toolbar's run configuration
    /// chip and `SimulateCommand::ManagePlans` open from the menu bar, the
    /// palette and the shortcut map, on any workspace. The other seven drafts are
    /// each opened by a control on one setup page — cloning the plan and renaming
    /// an analysis from the Analyses route, a design variable from Variables, a
    /// saved output from Outputs, and so on.
    ///
    /// The catalogue is the reason they are hosted together anyway. Its invoker is
    /// the navigator's creating action, which the panel draws on all nine setup
    /// routes, and `SimulateCommand::AddAnalysis`, which reaches it from the Simulate menu
    /// and the palette on any workspace — but the catalogue itself was drawn from
    /// the Analyses rail, so pressing that action anywhere else set
    /// `palette_open` with nothing to render it. That flag is one of the terms
    /// of [`crate::workbench::AppState::application_modal_open`], so the press
    /// took every keyboard shortcut in the application with it and left no
    /// painted modal to press Escape on. A host that runs once a frame wherever
    /// the reader is standing cannot arm an overlay that no pass draws.
    pub fn render_workflow_dialogs(cx: &mut Cx<'_, Self>, ctx: &egui::Context) {
        checkpoint_exchange::poll(ctx, cx);
        transient_checkpoint_exchange::poll(ctx, cx);
        simulation_workflow_dialog(ctx, cx);
        analysis_catalog(ctx, cx);
    }
}

/// The Analyses route with the stack and the editor side by side.
///
/// The title row stays put, and each column scrolls its own content, and
/// only when that content runs past the bottom of the window. The two used to
/// share the surface's one scroll: reading down a long form carried the stack
/// off the screen with it, and both columns were held to the window's height
/// under a title in that same scroll, so the route scrolled by the title's
/// height with nothing below to scroll to.
fn split_analysis_route(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    surface_width: f32,
    responsive_width: f32,
) {
    workspace_title_row(ui, |ui| plan_heading(ui, cx, surface_width));
    // Resolved once for the route and lent to both columns. Resolving it
    // expands the declared space, and the two columns each did it: the rail
    // through the workload it prices its rows from, the editor through the
    // participation its run-points control reads. One expansion answers both,
    // and it is the same one, so the two columns cannot report a different
    // space on the same frame.
    let participation = participation::PlanParticipation::resolve(cx.model);
    let body = ui.available_rect_before_wrap();
    let (stack_width, _) = analysis_split_widths(surface_width, responsive_width);
    let stack_rect = Rect::from_min_size(body.min, vec2(stack_width, body.height()));
    // One point of the page's own tone parts the stack's panel from the editor.
    let editor_rect =
        Rect::from_min_max(egui::pos2(stack_rect.right() + 1.0, body.top()), body.max);
    ui.painter()
        .rect_filled(stack_rect, 0.0, Tokens::get(ui.ctx()).color.bg_panel);

    let mut stack = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(stack_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    ScrollArea::vertical()
        .id_salt("workbench.simulate.analysis-stack")
        .auto_shrink([false, false])
        .show(&mut stack, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ordered_instance_stack(ui, cx, &participation);
            pages::dispatch_contract(ui, cx.model, cx.runs);
        });

    // The editor's height follows its width, so it withholds the scrollbar's
    // gutter whether or not a bar is up, for the reason `show` gives.
    let mut editor = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(editor_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    let editor_width = (editor_rect.width() - editor.spacing().scroll.allocated_width()).max(1.0);
    let output = ScrollArea::vertical()
        .id_salt("workbench.simulate.analysis-editor")
        .vertical_scroll_offset(cx.state.simulation_surface_scroll_y)
        .auto_shrink([false, false])
        .show(&mut editor, |ui| {
            let scroll_content_origin_y = ui.min_rect().top();
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_width(editor_width);
            analysis_editor(
                ui,
                cx,
                responsive_width,
                scroll_content_origin_y,
                &participation,
            );
        });
    settle_surface_scroll(ui, cx.state, output.state.offset.y);
    ui.advance_cursor_after_rect(body);
}

/// Whether the Analyses route puts its list beside the editor: on every
/// window wider than a phone's. The centre's own width does not decide it:
/// docks open at 1440 leave a centre a phone's width, and stacking the list
/// over the options there made one long page of what is two columns.
fn analysis_workspace_is_split(window_width: f32) -> bool {
    window_width > SIMULATION_STACK_BREAKPOINT
}

/// The list's and the editor's widths out of `available`. Each keeps its
/// least while the room allows; under both, the editor keeps its least and
/// the list narrows to what its rows' content needs, and the editor takes
/// the rest, its fields falling to one column where they no longer fit two.
pub fn analysis_split_widths(available: f32, viewport_width: f32) -> (f32, f32) {
    let usable = (available - 1.0).max(1.0);
    let (left_fraction, left_min, right_min) = analysis_column_constraints(viewport_width);
    let left = (usable * left_fraction)
        .max(left_min)
        .min((usable - right_min).max(analysis_row_content_min_width()))
        .min(usable);
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

/// Paint `problem`'s mark at `left`, centred on `center_y`; returns where the
/// words after it start.
fn paint_row_mark(ui: &Ui, left: f32, center_y: f32, problem: Problem) -> f32 {
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            egui::pos2(left + ROW_MARK_SIDE * 0.5, center_y),
            Vec2::splat(ROW_MARK_SIDE),
        ),
        problem,
    );
    left + ROW_MARK_ADVANCE
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

/// The newest run this plan produced that holds a dataset: where it sits in
/// history, which run it is, and whether an execution is still writing it.
#[derive(Debug, Clone, Copy)]
struct PriorPlanRun {
    index: usize,
    sequence: u64,
    executing: bool,
}

impl PriorPlanRun {
    /// How the analysis contract card names it, as a value on its own row.
    fn contract_summary(self) -> String {
        if self.executing {
            format!("Run {} in progress", self.sequence)
        } else {
            format!("Run {} finished", self.sequence)
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
fn prior_plan_run(model: &Model) -> Option<PriorPlanRun> {
    let plan_id = model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())?;
    let simulation = &model.simulation;
    let index = simulation.retained.runs.iter().position(|run| {
        !run.analyses.is_empty()
            && run
                .prepared_receipt()
                .and_then(crate::state::PreparedRunReceipt::simulation_plan_id)
                == Some(plan_id)
    })?;
    let run = &simulation.retained.runs[index];
    // Identity first, selection only as the fallback `has_active_execution`
    // itself falls back on: `is_running` without a sealed identity is the
    // runner's instantaneous activity, and the run being written is then the
    // one history has selected.
    let executing = simulation.execution.has_active_execution()
        && match simulation.execution.active_execution {
            Some(identity) => run.execution_identity() == Some(identity),
            None => simulation.view.active_run_idx == Some(index),
        };
    Some(PriorPlanRun {
        index,
        sequence: run.id,
        executing,
    })
}

fn adjusted_scroll_for_stack_delta(scroll_y: f32, before: f32, after: f32) -> f32 {
    (scroll_y + after - before).max(0.0)
}

/// Paint a control row's label into the property list's label column, and
/// leave the cursor at the value column.
///
/// `allocate_exact_size` rather than a laid-out label: a label allocation that
/// shrinks to its own text is what put these rows' controls in a column of
/// their own.
pub(super) fn paint_control_row_label(ui: &mut Ui, label: &str, row_width: f32) {
    let (label_left, value_left) = property_row_control_columns(ui, row_width);
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

#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
pub(crate) mod page_tests;
#[cfg(test)]
mod tests;
