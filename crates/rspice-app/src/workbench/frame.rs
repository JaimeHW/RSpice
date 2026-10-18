//! One workbench frame: chrome, docks, and the central surface.
//!
//! This is the shell's rendering entry point. It lived in `workbench.rs`
//! itself, which meant the module root declared 58 submodules and then also
//! painted the application — so the file that should answer "what is the
//! shell made of" was 470 lines of layout code with the answer buried at the
//! top.

use std::time::Duration;

use egui::{CentralPanel, Context, Frame};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::tokens::Tokens;

// The shell's submodules and re-exports, as this code saw them when it lived
// in `workbench.rs` itself.
use super::layout::LayoutSpec;
use super::state::Workspace;
use super::*;

/// Render one complete workbench frame.
///
/// `root` is the frame `Ui` eframe hands to [`eframe::App::ui`]. Panels are
/// allocated out of it in order: egui subtracts each panel's edge from the
/// remaining rect, so call order is the layout.
pub fn show(root: &mut egui::Ui, app: &mut RSpiceApp) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    app.state.reconcile_schematic_drag(ctx, false);
    app.state.apply_schematic_editing_preferences();
    reconcile_platform_full_screen(app);
    // Preflight's engine pass builds the run's decks off the UI thread, and
    // is finished, or stopped, on whichever frame finds it so, whatever that
    // frame shows.
    preflight::poll(&mut app.state, &mut app.simulation_controller);
    // The announcement goes first so that it is the record that survives.
    // Both of these report a finished run, and the mirror drops a line whose
    // destination is already announced — so whichever runs first is the one
    // the reader keeps. The announcement is the better record: it is titled
    // with the run and says what the run retained, where the mirrored console
    // line is the batch summary filed under its severity.
    announce_run_completion(ctx, app);
    synchronize_activity_stream(ctx, app);
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let context_docks_enabled = app.state.workbench.current_route().surface_id().archetype()
        != SurfaceArchetype::SpecialistWorkspace;
    let layout = LayoutSpec::resolve_for_shell(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        chrome::document_bar::is_visible(app),
        chrome::sheet_strip::is_visible(&app.state),
        app.state.project_lifecycle.is_open(),
        context_docks_enabled,
        &app.state.workbench,
    );
    // egui persists panel rectangles independently of application state.
    // Reconcile that cache before any command or panel is rendered so
    // responsive defaults and Reset Layout remain authoritative.
    docks::synchronize_panel_memory(ctx, app, layout);
    // Touch target inflation is capability-driven. A fine-pointer browser
    // resized through tablet/phone widths retains the compact mockup metrics.
    let large_targets = app.state.workbench.coarse_pointer;
    let requested_target = if app
        .state
        .ui
        .preferences
        .choice(ChoicePreference::MinimumTouchTarget)
        == 1
    {
        48.0
    } else {
        44.0
    };
    app.state
        .ui
        .theme
        .apply_responsive_metrics_with_target(ctx, large_targets.then_some(requested_target));
    let reduced_motion = app
        .state
        .ui
        .preferences
        .toggle(TogglePreference::ReducedMotion);
    // Custom widgets read this same policy instead of hard-coding their own
    // durations, so reduced motion is an application-wide contract rather
    // than an egui-default-only hint.
    ctx.global_style_mut(|style| style.animation_time = if reduced_motion { 0.0 } else { 0.16 });
    app.state.workbench.reconcile_drawer_mode(
        layout.navigator_uses_drawer,
        layout.inspector_uses_drawer,
        layout.workspaces_uses_drawer,
    );
    if app.state.workbench.full_screen_presentation
        && !app.state.application_modal_open()
        && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        app.state.exit_full_screen_presentation();
    }
    if app.state.workbench.full_screen_presentation {
        show_full_screen_presentation(root, app, layout);
        return;
    }

    // Global chrome is allocated first. Workbench columns are then allocated
    // before the document strip and console so both rows remain confined to
    // the center stack exactly as in the mockup grid.
    if layout.show_status_bar {
        chrome::status_bar::show(root, app, layout);
    }
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(root, app, layout);
    }
    chrome::title_bar::show(root, app, layout);
    chrome::toolbar::show(root, app, layout);
    // egui allocates same-edge side panels from the outside inward in call
    // order. Reserve the canonical 51 px activity rail first, then place the
    // navigator between that rail and the active document surface.
    if layout.show_activity_rail {
        chrome::activity_rail::show(root, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    chrome::document_bar::show(root, app, layout);
    // The sheet strip hangs from the document row: allocated after it, so the
    // sheets of a drawing read beneath the document that owns them.
    chrome::sheet_strip::show(root, app, layout);
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }

    // The launcher is a canonical route even though its mockup composition is
    // an application-modal manager. Browser navigation and restored history
    // therefore own its lifetime; the transient flag only retains modal focus
    // state for the current frame.
    let launcher_route_active =
        app.state.workbench.current_route().surface_id() == SurfaceId::ProjectLauncher;
    if launcher_route_active && !app.state.workbench.project_launcher_open {
        app.state.workbench.open_project_launcher();
    } else if !launcher_route_active && app.state.workbench.project_launcher_open {
        app.state.workbench.project_launcher_open = false;
    }
    tools::project_launcher::show(ctx, app);
    // Shortcuts and dialogs record the intent to run; the pass itself belongs
    // to the frame loop, so a caller never has to name the workflow to ask
    // for it. Consumed before the dialog so a blocked report opens this frame.
    if app.state.workbench.preflight.take_run_and_queue_request() {
        preflight::run_and_queue(app);
    }
    // A finding the reader chose to follow. The page that offered it holds
    // the session, not the application root, so the jump happens here.
    crate::workbench::diagnostic_routes::apply_pending(ctx, app);
    docks::show_run_details(ctx, app);
    tools::notification_center::show(ctx, app, layout.title_bar_height);
    // Export requests originate in retained result-document engines but IO is
    // owned by the app boundary.
    if let Some(keys) = app.state.ui.export_result_quantities_requested.take() {
        crate::workbench::menu_bar::action_export_result_selection_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
            &keys,
        );
    }
    if std::mem::take(&mut app.state.ui.export_csv_requested) {
        crate::workbench::menu_bar::action_export_csv_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
        );
    }
    if std::mem::take(&mut app.state.ui.open_producing_plan_requested) {
        commands::vocabulary::Command::OpenProducingPlan.execute(app);
    }

    if app.state.workbench.workspace != Workspace::Design {
        app.state.ui.canvas_hover = None;
        app.state.ui.canvas_view_center = None;
    }
    if let Some(action) = show_toasts(ctx, &mut app.state, layout) {
        commands::result_navigation::perform_notification_action(app, action);
    }
    app.state.reconcile_schematic_drag(ctx, true);
    apply_platform_full_screen_request(ctx, app);
}

/// Draw the toasts if this is the window they belong in: the one with the
/// keyboard, or the main window when none has it, so they are never drawn in
/// two windows at once. Returns the offer the reader took.
///
/// An open panel lists a notice the moment it is raised, in the corner the
/// toast would land on. Drawing both would stack one record on itself; the
/// toast keeps its clock and shows for whatever is left of it afterwards.
fn show_toasts(
    ctx: &Context,
    state: &mut AppState,
    layout: LayoutSpec,
) -> Option<crate::ui::widgets::NotificationAction> {
    if tools::notification_center::focused_window(ctx) != ctx.viewport_id()
        || tools::notification_center::showing_here(ctx, state)
    {
        return None;
    }
    let large_targets = tools::notification_center::large_targets(ctx, state);
    state.ui.toasts.show(
        ctx,
        layout.title_bar_height,
        layout.toolbar_height,
        large_targets,
    )
}

/// Announce each run the moment it stops running, and each campaign when it
/// ends, and offer what they left.
///
/// The console already records the batch outcome, but a console line is not a
/// notice and carries nowhere to go. This watches every run's lifecycle
/// rather than reading the log, so the offer is bound to a run identity
/// instead of to the wording of a message. A campaign's end is not a run, so
/// the controller reports it.
fn announce_run_completion(ctx: &Context, app: &mut RSpiceApp) {
    let runs = app
        .state
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| (run.run_id, run.lifecycle));
    let ended = app.state.ui.run_watch.observe(runs);
    let campaigns = app.state.ui.run_watch.take_campaign_outcomes();
    if ended.is_empty() && campaigns.is_empty() {
        return;
    }
    let notices = ended
        .into_iter()
        .filter_map(|run_id| run_notice(&app.state, run_id))
        .chain(campaigns.into_iter().map(campaign_notice))
        .collect::<Vec<_>>();
    // The reader's Task attention setting says which outcomes may interrupt
    // them. Every outcome is kept in the bell whatever it says.
    let attention = app
        .state
        .ui
        .preferences
        .workspace()
        .map(WorkspacePreferences::background_task_attention)
        .unwrap_or_default();
    let focused = ctx.input(|input| input.viewport().focused);
    let mut request = None;
    for notice in notices {
        let wanted = attention_request(focused, notice.elapsed_seconds, notice.kind, attention);
        if wanted.is_some() {
            // A tab has no taskbar to mark. Its own notification says it,
            // where the reader turned those on.
            crate::workbench::browser::background_alerts::alert(&notice.title, &notice.detail);
        }
        request = match (request, wanted) {
            (Some(egui::UserAttentionType::Critical), _)
            | (_, Some(egui::UserAttentionType::Critical)) => {
                Some(egui::UserAttentionType::Critical)
            }
            (request, wanted) => request.or(wanted),
        };
        let category = crate::ui::widgets::NotificationCategory::Job;
        let toasts = &mut app.state.ui.toasts;
        if !attention.retains(notice.is_failure()) {
            toasts.record_quietly(
                ctx,
                category,
                notice.kind,
                notice.title,
                notice.detail,
                notice.action,
            );
            continue;
        }
        match notice.action {
            Some(action) => toasts.notify_with_action(
                ctx,
                category,
                notice.kind,
                notice.title,
                notice.detail,
                action,
            ),
            None => {
                toasts.notify_with_title(ctx, category, notice.kind, notice.title, notice.detail)
            }
        }
    }
    // Once a frame, as urgently as the most serious outcome asks. The window
    // is marked in the taskbar or dock; it never takes focus. A browser has
    // neither, and ignores the request.
    if let Some(urgency) = request {
        ctx.send_viewport_cmd_to(
            egui::ViewportId::ROOT,
            egui::ViewportCommand::RequestUserAttention(urgency),
        );
    }
}

/// Whether an outcome just announced asks for the reader's attention from
/// outside the window, and how urgently.
///
/// Only an outcome that raised a toast, only while the main window is not
/// the one in front, and only for work long enough that the reader may have
/// turned to something else. A browser tab out of sight is a window not in
/// front; it raises the browser's notification instead of a taskbar mark.
fn attention_request(
    window_focused: Option<bool>,
    elapsed_seconds: f64,
    kind: crate::ui::widgets::ToastKind,
    attention: BackgroundTaskAttention,
) -> Option<egui::UserAttentionType> {
    use crate::ui::widgets::ToastKind;
    use crate::workbench::browser::background_alerts::AWAY_AFTER_SECONDS;

    if window_focused != Some(false)
        || elapsed_seconds.is_nan()
        || elapsed_seconds < AWAY_AFTER_SECONDS
        || !attention.retains(kind != ToastKind::Success)
    {
        return None;
    }
    Some(if kind == ToastKind::Error {
        egui::UserAttentionType::Critical
    } else {
        egui::UserAttentionType::Informational
    })
}

/// One run or campaign outcome, in the words the shell announces it with.
struct RunNotice {
    kind: crate::ui::widgets::ToastKind,
    title: String,
    detail: String,
    action: Option<crate::ui::widgets::NotificationAction>,
    /// How long the work took, in seconds.
    elapsed_seconds: f64,
}

impl RunNotice {
    /// Anything short of a clean finish, for the Task attention setting
    /// that notifies on failure only: a warning is something to look at.
    fn is_failure(&self) -> bool {
        self.kind != crate::ui::widgets::ToastKind::Success
    }
}

/// What a run that kept no analysis has to say for itself.
const NOTHING_RETAINED: &str =
    "No analysis result was retained, so there is nothing to open in Results.";

/// What one ended run announces, or `None` when its end is not news.
///
/// The words agree with the run's end line in the Console: a run that
/// finished some analyses and failed others "completed with errors", and the
/// warning count is the run log's own.
fn run_notice(state: &AppState, run_id: crate::product::RunId) -> Option<RunNotice> {
    use crate::state::SimulationRunLifecycle as Lifecycle;
    use crate::ui::widgets::{NotificationAction, ToastKind};

    let run = state.simulation.retained.run_by_stable_id(run_id)?;
    let sequence = run.id;
    let retained = run.analyses.len();
    let membership = run.campaign_membership();
    let plan = run
        .prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .and_then(|plan_id| plan_name(state, plan_id));
    // A campaign member's plan is in its title instead.
    let saved = saved_detail(
        plan.as_deref().filter(|_| membership.is_none()),
        retained,
        run.elapsed_time,
    );
    let (kind, title, detail) = match run.lifecycle {
        // The reader's own Stop. The Console and the status bar already say
        // it, and a warning for doing what they asked would be noise.
        Lifecycle::Aborted => return None,
        // A member that finished is counted in its campaign's summary; a
        // notice per member would bury the one that failed.
        Lifecycle::Completed if membership.is_some() => return None,
        Lifecycle::Completed => match run.log.count(LogSeverity::Warning) {
            0 => (
                ToastKind::Success,
                format!("Run {sequence} complete"),
                saved,
            ),
            1 => (
                ToastKind::Warn,
                format!("Run {sequence} complete with 1 warning"),
                saved,
            ),
            warnings => (
                ToastKind::Warn,
                format!("Run {sequence} complete with {warnings} warnings"),
                saved,
            ),
        },
        Lifecycle::Failed if run.analyses.iter().any(|analysis| analysis.success) => (
            ToastKind::Error,
            format!("Run {sequence} completed with errors"),
            saved,
        ),
        Lifecycle::Failed => (
            ToastKind::Error,
            format!("Run {sequence} failed"),
            crate::simulation::SimulationController::run_failure(sequence, &run.log).map_or_else(
                || {
                    if retained == 0 {
                        NOTHING_RETAINED.to_owned()
                    } else {
                        saved
                    }
                },
                str::to_owned,
            ),
        ),
        Lifecycle::Interrupted => (
            ToastKind::Warn,
            format!("Run {sequence} interrupted"),
            match retained {
                0 => NOTHING_RETAINED.to_owned(),
                1 => "1 analysis saved with this run.".to_owned(),
                _ => format!("{retained} analyses saved with this run."),
            },
        ),
        _ => return None,
    };
    // A member that failed is announced as its campaign's, under the plan
    // the reader queued; what the run itself would have said follows.
    let (title, detail) = match membership {
        Some(member) if kind == ToastKind::Error => (
            match &plan {
                Some(plan) => format!("Campaign '{}': '{plan}' failed", member.name()),
                None => format!(
                    "Campaign '{}': member {} of {} failed",
                    member.name(),
                    member.member_index(),
                    member.member_count()
                ),
            },
            format!("{title} \u{b7} {detail}"),
        ),
        _ => (title, detail),
    };
    let action = if retained > 0 {
        Some(NotificationAction::OpenRunInResults {
            run_sequence: sequence,
        })
    } else if run.lifecycle == Lifecycle::Failed {
        // Nothing to open in Results, so Results is not offered: a control
        // that arrives at an empty dataset would be worse than none. A
        // failure still has an account of itself, and it starts at the
        // run's line in the session's log, which the run wrote as it ended.
        state
            .log_buffer
            .entries()
            .filter(|entry| {
                entry.anchor
                    == Some(crate::diagnostics::LogAnchor::ResultRun {
                        run_sequence: sequence,
                    })
            })
            .last()
            .map(|entry| NotificationAction::ShowInConsole { log_id: entry.id })
    } else {
        None
    };
    Some(RunNotice {
        kind,
        title,
        detail,
        action,
        elapsed_seconds: run.elapsed_time,
    })
}

/// What a campaign outcome the controller reported announces. Both offer the
/// Console line that says what happened.
fn campaign_notice(
    outcome: crate::simulation::controller::prepared_run::CampaignOutcome,
) -> RunNotice {
    use crate::simulation::controller::prepared_run::CampaignOutcome;
    use crate::ui::widgets::{NotificationAction, ToastKind};

    match outcome {
        CampaignOutcome::MemberRefused {
            campaign,
            plan,
            reason,
            log_id,
        } => RunNotice {
            kind: ToastKind::Error,
            title: format!("Campaign '{campaign}': '{plan}' failed"),
            detail: reason,
            action: Some(NotificationAction::ShowInConsole { log_id }),
            // Refused before it ran; the campaign's end speaks for the wait.
            elapsed_seconds: 0.0,
        },
        CampaignOutcome::Finished {
            name,
            completed,
            failed,
            elapsed_seconds,
            log_id,
        } => RunNotice {
            kind: if failed == 0 {
                ToastKind::Success
            } else {
                ToastKind::Error
            },
            title: format!("Campaign '{name}' finished"),
            detail: format!(
                "{completed} completed, {failed} failed \u{b7} {}",
                crate::quantity::format_elapsed(elapsed_seconds)
            ),
            action: Some(NotificationAction::ShowInConsole { log_id }),
            elapsed_seconds,
        },
    }
}

/// `Nightly AC · 3 analyses saved · 12.40 s`. The plan is left out for a run
/// that had none, such as a typed deck. The time is written the way the
/// run's end line in the Console writes it.
fn saved_detail(plan: Option<&str>, retained: usize, elapsed_seconds: f64) -> String {
    let saved = match retained {
        0 => "no analyses saved".to_owned(),
        1 => "1 analysis saved".to_owned(),
        _ => format!("{retained} analyses saved"),
    };
    plan.map(str::to_owned)
        .into_iter()
        .chain([saved, crate::quantity::format_elapsed(elapsed_seconds)])
        .collect::<Vec<_>>()
        .join(" \u{b7} ")
}

/// The name a plan goes by in the plan manager, active or not. `None` once
/// the plan is gone.
fn plan_name(state: &AppState, plan_id: crate::product::SimulationPlanId) -> Option<String> {
    if state
        .sim_setup
        .analysis_plan
        .as_ref()
        .is_some_and(|plan| plan.id() == plan_id)
    {
        return Some(state.sim_setup.active_plan_name().as_str().to_owned());
    }
    state
        .sim_setup
        .inactive_plans()
        .iter()
        .find(|plan| plan.id() == plan_id)
        .map(|plan| plan.name().as_str().to_owned())
}

fn show_full_screen_presentation(root: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    if layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }

    let mut exit_requested = false;
    egui::Area::new(egui::Id::new("rspice-full-screen-exit"))
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 12.0))
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(8, 6))
                .show(ui, |ui| {
                    exit_requested = ui.button("Exit full screen  \u{00b7}  F11 / Esc").clicked();
                });
        });
    if exit_requested {
        app.state.exit_full_screen_presentation();
    }

    if let Some(keys) = app.state.ui.export_result_quantities_requested.take() {
        crate::workbench::menu_bar::action_export_result_selection_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
            &keys,
        );
    }
    if std::mem::take(&mut app.state.ui.export_csv_requested) {
        crate::workbench::menu_bar::action_export_csv_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
        );
    }
    if std::mem::take(&mut app.state.ui.open_producing_plan_requested) {
        commands::vocabulary::Command::OpenProducingPlan.execute(app);
    }
    if app.state.workbench.workspace != Workspace::Design {
        app.state.ui.canvas_hover = None;
        app.state.ui.canvas_view_center = None;
    }
    // A detached window with the keyboard draws the toasts instead.
    if tools::notification_center::focused_window(ctx) == ctx.viewport_id() {
        let large_targets = tools::notification_center::large_targets(ctx, &app.state);
        if let Some(action) = app.state.ui.toasts.show(ctx, 0.0, 0.0, large_targets) {
            commands::result_navigation::perform_notification_action(app, action);
        }
    }
    apply_platform_full_screen_request(ctx, app);
}

/// Render one secondary native application viewport.
///
/// Application-global modal owners and exports remain in the primary
/// viewport. The window still receives the complete workbench chrome, dock
/// composition, active document, and workspace surface; the notification
/// panel when its own bell opened it; and the toasts while it has the
/// keyboard.
pub(crate) fn show_secondary(root: &mut egui::Ui, app: &mut RSpiceApp) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    app.state.reconcile_schematic_drag(ctx, false);
    app.state.apply_schematic_editing_preferences();
    reconcile_platform_full_screen(app);
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let context_docks_enabled = app.state.workbench.current_route().surface_id().archetype()
        != SurfaceArchetype::SpecialistWorkspace;
    let layout = LayoutSpec::resolve_for_shell(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        chrome::document_bar::is_visible(app),
        chrome::sheet_strip::is_visible(&app.state),
        app.state.project_lifecycle.is_open(),
        context_docks_enabled,
        &app.state.workbench,
    );
    docks::synchronize_panel_memory(ctx, app, layout);
    app.state.workbench.reconcile_drawer_mode(
        layout.navigator_uses_drawer,
        layout.inspector_uses_drawer,
        layout.workspaces_uses_drawer,
    );

    if layout.show_status_bar {
        chrome::status_bar::show(root, app, layout);
    }
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(root, app, layout);
    }
    chrome::title_bar::show(root, app, layout);
    chrome::toolbar::show(root, app, layout);
    if layout.show_activity_rail {
        chrome::activity_rail::show(root, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    chrome::document_bar::show(root, app, layout);
    chrome::sheet_strip::show(root, app, layout);
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });
    if layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }
    tools::notification_center::show(ctx, app, layout.title_bar_height);
    if let Some(action) = show_toasts(ctx, &mut app.state, layout) {
        commands::result_navigation::perform_notification_action(app, action);
    }
    app.state.reconcile_schematic_drag(ctx, true);
    apply_platform_full_screen_request(ctx, app);
}

/// Truthful browser/backend fallback for a secondary logical window. The host
/// embeds it in a movable egui window, so nested top-level panels would escape
/// that frame; render the owned document surface directly instead.
pub(crate) fn show_embedded_secondary(ui: &mut egui::Ui, app: &mut RSpiceApp, title: &str) -> bool {
    app.state.reconcile_schematic_drag(ui.ctx(), false);
    let mut reattach = false;
    ui.horizontal(|ui| {
        ui.strong(title);
        ui.label(format!(
            "{} workspace \u{00b7} embedded single-host fallback",
            app.state.workbench.workspace.label()
        ));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            reattach = ui.button("Reattach to main window").clicked();
        });
    });
    ui.separator();
    surfaces::show(ui, app);
    app.state.reconcile_schematic_drag(ui.ctx(), true);
    reattach
}

/// Render route-owned overlays after application dialogs so a workflow opened
/// from Preferences is the top modal while the originating dialog remains
/// retained underneath and can be restored on Close.
///
/// The simulation plan workflows are here for the same reason the account and
/// tool managers are: every one of them is opened by a command the toolbar,
/// the menus and the palette can reach from any workspace, so a host that only
/// runs on one surface would make the control dead everywhere else.
pub(crate) fn show_route_overlays(ctx: &Context, app: &mut RSpiceApp) {
    account_organization::show(ctx, app);
    tools::jobs_manager::show(ctx, app);
    tools::specialist_tool_browser::show(ctx, app);
    surfaces::show_simulation_workflow_dialogs(ctx, app);
}

fn synchronize_activity_stream(ctx: &Context, app: &mut RSpiceApp) {
    use crate::ui::widgets::{MirroredEntry, NotificationCategory, ToastKind};

    let revision = app.state.log_buffer.revision();
    let observed = app.state.ui.toasts.observed_log_revision();
    if revision == observed {
        return;
    }
    // Not subject to the Task attention setting: a studio or netlist failure
    // is not a background run, and is always kept in the bell.
    let now = ctx.input(|input| input.time);
    let wall_now = crate::time_compat::unix_epoch();
    let entries = app
        .state
        .log_buffer
        .entries()
        .filter(|entry| entry.id >= observed && entry.severity == LogSeverity::Error)
        .filter_map(|entry| {
            // Only a studio or netlist failure is lifted. A run's failures
            // are announced by the shell's run announcement and listed in
            // Problems, and the engine's lines are that run's own account,
            // so the run side is never lifted. A plan entry is a record of
            // what was committed or refused. An Info or Warning line is a
            // record too: the Console is where it is read.
            let category = match entry.source {
                LogSource::Studio | LogSource::Netlist => NotificationCategory::System,
                LogSource::Plan
                | LogSource::Run
                | LogSource::Engine
                | LogSource::Hdl
                | LogSource::VerilogA => return None,
            };
            Some(MirroredEntry {
                log_id: entry.id,
                category,
                kind: ToastKind::Error,
                message: entry.message.clone(),
                action: None,
                created: project_log_timestamp(now, wall_now, entry.timestamp),
            })
        })
        .collect::<Vec<_>>();
    app.state.ui.toasts.synchronize_activity(revision, entries);
}

/// Project a log entry's wall-clock timestamp onto egui's monotonic clock
/// while preserving the event's original age.
fn project_log_timestamp(now: f64, wall_now: Duration, timestamp: Duration) -> f64 {
    now - wall_now.saturating_sub(timestamp).as_secs_f64()
}

fn pointer_is_coarse(ctx: &Context, retained_touch_capability: bool) -> bool {
    let touch_event = ctx.input(|input| {
        input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Touch { .. }))
    });
    retained_touch_capability || touch_event || platform_pointer_is_coarse()
}

#[cfg(not(target_arch = "wasm32"))]
const fn platform_pointer_is_coarse() -> bool {
    cfg!(any(target_os = "android", target_os = "ios"))
}

#[cfg(target_arch = "wasm32")]
fn platform_pointer_is_coarse() -> bool {
    use wasm_bindgen::{JsCast as _, JsValue};

    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(callable) = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("matchMedia"))
    else {
        return false;
    };
    let Ok(match_media) = callable.dyn_into::<js_sys::Function>() else {
        return false;
    };
    let Ok(result) = match_media.call1(window.as_ref(), &JsValue::from_str("(pointer: coarse)"))
    else {
        return false;
    };
    js_sys::Reflect::get(&result, &JsValue::from_str("matches"))
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn reconcile_platform_full_screen(_app: &mut RSpiceApp) {}

#[cfg(target_arch = "wasm32")]
fn reconcile_platform_full_screen(app: &mut RSpiceApp) {
    // Browser chrome (notably Escape) may leave fullscreen without routing a
    // command through egui. Reflect the platform truth back into retained UI
    // state so the menu checkmark and next toggle remain accurate.
    if app.state.ui.full_screen_request.is_none()
        && let Some(document) = web_sys::window().and_then(|window| window.document())
    {
        let platform_full_screen = document.fullscreen_element().is_some();
        let platform_exit_ended_presentation =
            app.state.workbench.full_screen && !platform_full_screen;
        app.state.workbench.full_screen = platform_full_screen;
        if platform_exit_ended_presentation {
            app.state.clear_full_screen_presentation(false);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_platform_full_screen_request(ctx: &Context, app: &mut RSpiceApp) {
    if let Some(enabled) = app.state.ui.take_full_screen_request() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(enabled));
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_platform_full_screen_request(_ctx: &Context, app: &mut RSpiceApp) {
    let Some(enabled) = app.state.ui.take_full_screen_request() else {
        return;
    };
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        app.state.clear_full_screen_presentation(false);
        log::error!("Fullscreen request failed: browser document is unavailable");
        return;
    };

    if enabled {
        let Some(root) = document.document_element() else {
            app.state.clear_full_screen_presentation(false);
            log::error!("Fullscreen request failed: document root is unavailable");
            return;
        };
        if let Err(error) = root.request_fullscreen() {
            app.state.clear_full_screen_presentation(false);
            log::warn!("Browser rejected fullscreen request: {error:?}");
        }
    } else {
        document.exit_fullscreen();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::Drawer;

    #[test]
    fn every_canonical_workspace_has_a_surface_owner() {
        assert_eq!(Workspace::ALL.len(), 8);
        assert_eq!(Workspace::PHONE_PRIMARY.len(), 4);
        assert!(Workspace::ALL.contains(&Workspace::Netlist));
        assert!(Workspace::ALL.contains(&Workspace::Stimulus));
    }

    /// A reported failure is toasted with an offer to show its Console line,
    /// and the mirror then meets that same line: one failure, one record.
    #[test]
    fn a_reported_failure_leaves_one_activity_record() {
        use crate::ui::widgets::{NotificationAction, ToastKind};

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state
            .report_failure("Project save failed", "The disk is full.");
        synchronize_activity_stream(&ctx, &mut app);
        synchronize_activity_stream(&ctx, &mut app);

        let line = app
            .state
            .log_buffer
            .entries()
            .last()
            .expect("the failure is in the Console");
        assert_eq!(line.severity, LogSeverity::Error);
        assert_eq!(line.message, "Project save failed: The disk is full.");
        let line = line.id;
        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity.len(), 1, "the mirrored line is the same notice");
        assert_eq!(activity[0].title(), "Project save failed");
        assert_eq!(activity[0].kind(), ToastKind::Error);
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::ShowInConsole { log_id: line })
        );
    }

    /// A completion is announced on the transition into a terminal lifecycle,
    /// not on the terminal state itself. Opening a project whose history
    /// already holds a finished run must not claim one just completed — and
    /// the notice that is raised has to carry the dataset it announces.
    ///
    /// A failure that finished an analysis first "completed with errors", as
    /// the run's end line in the Console says. The reader's own Stop is not
    /// announced at all: the Console and the status bar already say it.
    #[test]
    fn run_outcome_notices_distinguish_failure_and_interruption_and_leave_a_stop_unannounced() {
        use crate::diagnostics::LogAnchor;
        use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle as Lifecycle};
        use crate::ui::widgets::{NotificationAction, ToastKind};

        for retained in [false, true] {
            let ctx = Context::default();
            let mut app = RSpiceApp::test_instance();
            let run = app.state.simulation.start_run();
            if retained {
                run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "partial"));
            }
            run.lifecycle = Lifecycle::Running;
            announce_run_completion(&ctx, &mut app);
            app.state.simulation.retained.runs[0].success = false;
            app.state.simulation.retained.runs[0]
                .finish_lifecycle(Lifecycle::Aborted)
                .unwrap();
            announce_run_completion(&ctx, &mut app);
            assert!(
                app.state.ui.toasts.activity().is_empty(),
                "a run the reader stopped is not news to them"
            );
        }

        let mut observed = Vec::new();
        let mut expected = Vec::new();
        for retained in [false, true] {
            for (lifecycle, title, kind) in [
                (
                    Lifecycle::Failed,
                    if retained {
                        "Run 1 completed with errors"
                    } else {
                        "Run 1 failed"
                    },
                    ToastKind::Error,
                ),
                (Lifecycle::Interrupted, "Run 1 interrupted", ToastKind::Warn),
            ] {
                let ctx = Context::default();
                let mut app = RSpiceApp::test_instance();
                let run = app.state.simulation.start_run();
                if retained {
                    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "partial"));
                }
                run.lifecycle = Lifecycle::Running;
                announce_run_completion(&ctx, &mut app);
                app.state.simulation.retained.runs[0].success = false;
                app.state.simulation.retained.runs[0]
                    .finish_lifecycle(lifecycle)
                    .unwrap();
                // The line the run writes in the session's log as it ends.
                let end_line = app.state.log_buffer.log_anchored(
                    LogSeverity::Error,
                    LogSource::Run,
                    title,
                    Some(LogAnchor::ResultRun { run_sequence: 1 }),
                );
                announce_run_completion(&ctx, &mut app);
                announce_run_completion(&ctx, &mut app);
                let activity = app.state.ui.toasts.activity();
                assert_eq!(activity.len(), 1, "one terminal transition, one notice");
                observed.push((activity[0].title().to_owned(), activity[0].kind()));
                expected.push((title.to_owned(), kind));
                // One offer per notice. A retained dataset is the better
                // destination; without one, only a failure has an account
                // worth reading, and it starts at the run's line in the
                // session's log.
                let offer = if retained {
                    Some(NotificationAction::OpenRunInResults { run_sequence: 1 })
                } else {
                    matches!(lifecycle, Lifecycle::Failed)
                        .then_some(NotificationAction::ShowInConsole { log_id: end_line })
                };
                assert_eq!(activity[0].action(), offer);
            }
        }
        assert_eq!(observed, expected);
    }

    #[test]
    fn a_run_is_announced_when_it_finishes_and_offers_the_dataset_it_made() {
        use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle};
        use crate::ui::widgets::{NotificationAction, ToastKind};

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state
            .simulation
            .start_run()
            .add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "tran"));
        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Completed;
        app.state.simulation.retained.runs[0].success = true;

        announce_run_completion(&ctx, &mut app);
        assert!(
            app.state.ui.toasts.activity().is_empty(),
            "a run that was already finished when first seen did not just complete"
        );

        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        assert!(app.state.ui.toasts.activity().is_empty());

        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Completed;
        announce_run_completion(&ctx, &mut app);
        let activity = app.state.ui.toasts.activity();
        assert_eq!(
            activity.len(),
            1,
            "the transition raises exactly one notice"
        );
        assert_eq!(activity[0].title(), "Run 1 complete");
        assert_eq!(
            activity[0].message(),
            "1 analysis saved \u{b7} 0.000 s",
            "one analysis is an analysis, not an \"analys\"; a run with no plan names none"
        );
        assert_eq!(activity[0].kind(), ToastKind::Success);
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::OpenRunInResults { run_sequence: 1 }),
            "and it offers the dataset the run produced"
        );

        announce_run_completion(&ctx, &mut app);
        assert_eq!(
            app.state.ui.toasts.activity().len(),
            1,
            "a terminal run is not announced again on every later frame"
        );
    }

    /// One finished run leaves one record.
    ///
    /// Two reporters describe every completion — the shell announces the run,
    /// and the run writes its console line. Only the announcement reaches the
    /// notice centre; the line stays in the Console. This runs both in the
    /// order the frame runs them.
    #[test]
    fn one_finished_run_leaves_one_activity_record() {
        use crate::diagnostics::{LogAnchor, LogSeverity};
        use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle};
        use crate::ui::widgets::NotificationAction;

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state
            .simulation
            .start_run()
            .add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "tran"));
        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Running;
        announce_run_completion(&ctx, &mut app);

        // What `finish_simulation_batch` leaves behind: a sealed lifecycle
        // and an anchored console line naming the run it sealed.
        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Completed;
        app.state.simulation.retained.runs[0].success = true;
        app.state.log_buffer.log_anchored(
            LogSeverity::Info,
            LogSource::Run,
            "Simulation completed successfully",
            Some(LogAnchor::ResultRun { run_sequence: 1 }),
        );

        announce_run_completion(&ctx, &mut app);
        synchronize_activity_stream(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(
            activity.len(),
            1,
            "one completion is one event, however many reporters saw it: {:?}",
            activity
                .iter()
                .map(|record| record.title())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            activity[0].title(),
            "Run 1 complete",
            "and the record kept is the one that names the run and what it retained"
        );
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::OpenRunInResults { run_sequence: 1 }),
            "still offering the dataset the completion notice offers today"
        );
        assert_eq!(app.state.ui.toasts.unread_count(), 1);
    }

    /// A run that retained nothing is still reported, but never with a way
    /// into Results: an offer that arrived at an empty dataset would be worse
    /// than none. A failure's account of itself is in the Console, so that is
    /// the one place it offers to go.
    ///
    /// And it is reported *once*: the run's own failure line stays in the
    /// Console, because a run's failures are the shell's announcement and a
    /// Problems row.
    #[test]
    fn a_run_that_retained_nothing_is_announced_once_and_never_offers_its_dataset() {
        use crate::diagnostics::{LogAnchor, LogSeverity};
        use crate::state::SimulationRunLifecycle;

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run();
        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        app.state.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Failed;
        app.state.simulation.retained.runs[0].success = false;
        // Exactly what `finish_simulation_batch` leaves behind for a run that
        // sealed with nothing retained.
        let end_line = app.state.log_buffer.log_anchored(
            LogSeverity::Error,
            LogSource::Run,
            "Simulation completed with errors",
            Some(LogAnchor::ResultRun { run_sequence: 1 }),
        );

        announce_run_completion(&ctx, &mut app);
        synchronize_activity_stream(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(
            activity.len(),
            1,
            "one completion is one event, however many reporters saw it: {:?}",
            activity
                .iter()
                .map(|record| record.title())
                .collect::<Vec<_>>()
        );
        assert_eq!(activity[0].title(), "Run 1 failed");
        assert_eq!(
            activity[0].action(),
            Some(crate::ui::widgets::NotificationAction::ShowInConsole { log_id: end_line }),
            "nothing was retained, so Results is not offered; the failure's account starts at \
             the run's line in the Console, so that line is"
        );
        assert_eq!(app.state.ui.toasts.unread_count(), 1);
    }

    /// A receipt naming `plan_id`, as a run prepared from that plan carries.
    fn plan_receipt(plan_id: crate::product::SimulationPlanId) -> crate::state::PreparedRunReceipt {
        use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
        use crate::state::{PreparedRunTaskReceipt, PreparedSourceCheckReceipt};

        crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(plan_id),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: ContentDigest::from_bytes([1; 32]),
            source_content_digest: ContentDigest::from_bytes([2; 32]),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(
                ContentDigest::from_bytes([3; 32]),
            ),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy: crate::state::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    ContentDigest::from_bytes([4; 32]),
                )
                .expect("task receipt"),
            ],
        })
        .expect("plan receipt")
    }

    /// One line in a run's own log.
    fn run_log_line(severity: LogSeverity, message: &str) -> rspice_app_types::run_log::RunLogLine {
        rspice_app_types::run_log::RunLogLine {
            unix_ms: 0,
            severity,
            source: LogSource::Run,
            message: message.to_owned(),
        }
    }

    /// Every run seen running is announced when it ends, not just the
    /// newest. A campaign starts its next run in the update that finishes
    /// the last, so the newest run had always moved on by the time a frame
    /// looked, and only the final run of a sequence was ever announced.
    #[test]
    fn every_run_seen_running_is_announced_not_only_the_newest() {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);

        // One update: the first run fails and the second starts.
        let first = app.state.simulation.retained.runs[0].id;
        app.state.simulation.retained.runs[0].lifecycle = Lifecycle::Failed;
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        let titles = |app: &RSpiceApp| {
            app.state
                .ui
                .toasts
                .activity()
                .iter()
                .map(|record| record.title().to_owned())
                .collect::<Vec<_>>()
        };
        assert_eq!(titles(&app), [format!("Run {first} failed")]);

        let second = app.state.simulation.retained.runs[0].id;
        app.state.simulation.retained.runs[0].lifecycle = Lifecycle::Completed;
        announce_run_completion(&ctx, &mut app);
        announce_run_completion(&ctx, &mut app);
        assert_eq!(
            titles(&app),
            [
                format!("Run {second} complete"),
                format!("Run {first} failed")
            ],
            "each run once, in the order they ended"
        );
    }

    /// A run that opens and ends between two frames was never seen running
    /// by the frame, but it was seen by the controller that opened it.
    #[test]
    fn a_run_that_ends_before_any_frame_sees_it_is_still_announced() {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        let run = app.state.simulation.start_run();
        run.lifecycle = Lifecycle::Completed;
        let run_id = run.run_id;
        app.state.ui.run_watch.run_started(run_id);

        announce_run_completion(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].title(), "Run 1 complete");
    }

    /// A completion with warnings is not a clean one. Its count is the run
    /// log's, so the notice and the run's end line in the Console agree.
    #[test]
    fn a_run_that_completed_with_warnings_says_so() {
        use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle as Lifecycle};
        use crate::ui::widgets::ToastKind;

        for (warnings, title) in [
            (1, "Run 1 complete with 1 warning"),
            (3, "Run 1 complete with 3 warnings"),
        ] {
            let ctx = Context::default();
            let mut app = RSpiceApp::test_instance();
            let run = app.state.simulation.start_run();
            run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "tran"));
            run.lifecycle = Lifecycle::Running;
            announce_run_completion(&ctx, &mut app);
            let run = &mut app.state.simulation.retained.runs[0];
            for _ in 0..warnings {
                run.log
                    .push(run_log_line(LogSeverity::Warning, "gmin stepping was used"));
            }
            run.log
                .push(run_log_line(LogSeverity::Info, "Run 1 completed"));
            run.lifecycle = Lifecycle::Completed;

            announce_run_completion(&ctx, &mut app);

            let activity = app.state.ui.toasts.activity();
            assert_eq!(activity.len(), 1);
            assert_eq!(activity[0].title(), title);
            assert_eq!(activity[0].kind(), ToastKind::Warn);
        }
    }

    /// The detail names the plan the run was prepared from, what it saved,
    /// and how long it took, in the run log's own format. A run that finished
    /// some analyses and failed others completed with errors, as its end
    /// line in the Console says, and still offers what it saved.
    #[test]
    fn a_partly_failed_run_names_its_plan_and_what_it_saved() {
        use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle as Lifecycle};
        use crate::ui::widgets::{NotificationAction, ToastKind};

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .create_plan("Nightly AC")
            .expect("a plan to run");
        let run = app
            .state
            .simulation
            .start_prepared_run(plan_receipt(plan_id))
            .expect("a prepared run");
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "ac"));
        run.add_analysis(AnalysisResult::failed(
            2,
            AnalysisType::Transient,
            "tran",
            "timestep too small",
        ));
        run.lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        let run = &mut app.state.simulation.retained.runs[0];
        run.lifecycle = Lifecycle::Failed;
        run.elapsed_time = 12.4;

        announce_run_completion(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].title(), "Run 1 completed with errors");
        assert_eq!(activity[0].kind(), ToastKind::Error);
        assert_eq!(
            activity[0].message(),
            "Nightly AC \u{b7} 2 analyses saved \u{b7} 12.40 s"
        );
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::OpenRunInResults { run_sequence: 1 })
        );
    }

    /// A run that failed outright says why in the words its end line quotes:
    /// the first error it wrote about itself, not the end line.
    #[test]
    fn a_failed_run_quotes_its_first_failure() {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        let run = &mut app.state.simulation.retained.runs[0];
        run.log.push(run_log_line(
            LogSeverity::Error,
            "Transient: timestep too small at 1.2 us",
        ));
        run.log
            .push(run_log_line(LogSeverity::Error, "A later error"));
        run.log.push(run_log_line(
            LogSeverity::Error,
            "Run 1 failed after 0.31 s: Transient: timestep too small at 1.2 us",
        ));
        run.lifecycle = Lifecycle::Failed;

        announce_run_completion(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity[0].title(), "Run 1 failed");
        assert_eq!(
            activity[0].message(),
            "Transient: timestep too small at 1.2 us"
        );

        // With nothing but its end line, a run's failure has no account of
        // its own, and the detail says what it left instead.
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        let run = &mut app.state.simulation.retained.runs[0];
        run.log.push(run_log_line(
            LogSeverity::Error,
            "Run 1 failed after 0.31 s",
        ));
        run.lifecycle = Lifecycle::Failed;
        announce_run_completion(&ctx, &mut app);
        assert_eq!(
            app.state.ui.toasts.activity()[0].message(),
            NOTHING_RETAINED
        );
    }

    /// A campaign is announced by what the reader has to act on: a member
    /// that failed, as it fails, and the campaign's end. A member that
    /// finished cleanly is counted in the summary and raises nothing of its
    /// own, so the failure is not buried among a column of successes.
    #[test]
    fn a_two_member_campaign_announces_its_failed_member_and_its_end() {
        use crate::diagnostics::LogAnchor;
        use crate::simulation::controller::prepared_run::CampaignOutcome;
        use crate::state::{SimulationCampaignMembership, SimulationRunLifecycle as Lifecycle};
        use crate::ui::widgets::{NotificationAction, ToastKind};

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        let campaign = crate::product::SimulationCampaignId::new();
        let bias = app.state.sim_setup.create_plan("Bias").expect("a plan");
        let sweep = app.state.sim_setup.create_plan("Sweep").expect("a plan");
        let start_member = |simulation: &mut crate::state::SimulationState,
                            plan: crate::product::SimulationPlanId,
                            index: u32|
         -> u64 {
            let run = simulation
                .start_prepared_run(plan_receipt(plan))
                .expect("a member run");
            run.set_campaign_membership(
                SimulationCampaignMembership::new(campaign, "Nightly", index, 2)
                    .expect("a membership"),
            )
            .expect("the run joins the campaign");
            run.lifecycle = Lifecycle::Running;
            run.id
        };
        let first = start_member(&mut app.state.simulation, bias, 1);
        announce_run_completion(&ctx, &mut app);

        // One update: the first member fails and the second starts.
        let run = app
            .state
            .simulation
            .retained
            .run_by_sequence_mut(first)
            .expect("the first member");
        run.log.push(run_log_line(
            LogSeverity::Error,
            "Operating point: no convergence",
        ));
        run.lifecycle = Lifecycle::Failed;
        let end_line = app.state.log_buffer.log_anchored(
            LogSeverity::Error,
            LogSource::Run,
            "Run 1 failed after 0.20 s: Operating point: no convergence",
            Some(LogAnchor::ResultRun {
                run_sequence: first,
            }),
        );
        let second = start_member(&mut app.state.simulation, sweep, 2);
        announce_run_completion(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity.len(), 1, "the failed member, as it fails");
        assert_eq!(activity[0].title(), "Campaign 'Nightly': 'Bias' failed");
        assert_eq!(activity[0].kind(), ToastKind::Error);
        assert_eq!(
            activity[0].message(),
            format!("Run {first} failed \u{b7} Operating point: no convergence")
        );
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::ShowInConsole { log_id: end_line }),
            "the run's own offer"
        );

        // The second member finishes cleanly, which ends the campaign.
        app.state
            .simulation
            .retained
            .run_by_sequence_mut(second)
            .expect("the second member")
            .lifecycle = Lifecycle::Completed;
        let summary = app.state.log_buffer.log(
            LogSeverity::Error,
            LogSource::Run,
            "Campaign 'Nightly' completed with errors: 1 completed, 1 failed",
        );
        app.state
            .ui
            .run_watch
            .campaign_outcome(CampaignOutcome::Finished {
                name: "Nightly".to_owned(),
                completed: 1,
                failed: 1,
                elapsed_seconds: 42.5,
                log_id: summary,
            });
        announce_run_completion(&ctx, &mut app);
        announce_run_completion(&ctx, &mut app);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(
            activity.len(),
            2,
            "the clean member raises nothing of its own: {:?}",
            activity
                .iter()
                .map(|record| record.title())
                .collect::<Vec<_>>()
        );
        assert_eq!(activity[0].title(), "Campaign 'Nightly' finished");
        assert_eq!(activity[0].kind(), ToastKind::Error, "a member failed");
        assert_eq!(
            activity[0].message(),
            "1 completed, 1 failed \u{b7} 42.50 s"
        );
        assert_eq!(
            activity[0].action(),
            Some(NotificationAction::ShowInConsole { log_id: summary })
        );

        // A campaign nothing failed in finished as a success.
        app.state
            .ui
            .run_watch
            .campaign_outcome(CampaignOutcome::Finished {
                name: "Nightly".to_owned(),
                completed: 2,
                failed: 0,
                elapsed_seconds: 40.0,
                log_id: summary,
            });
        announce_run_completion(&ctx, &mut app);
        assert_eq!(app.state.ui.toasts.activity()[0].kind(), ToastKind::Success);
    }

    #[test]
    fn drawer_state_is_transient_navigation_not_a_workspace() {
        let mut state = WorkbenchState::default();
        state.toggle_drawer(Drawer::Navigator);
        assert_eq!(state.workspace, Workspace::Design);
        assert_eq!(state.drawer, Some(Drawer::Navigator));
    }

    #[test]
    fn application_full_screen_has_exactly_one_enter_and_exit_request() {
        let mut app = RSpiceApp::test_instance();
        app.state.enter_full_screen_presentation();

        assert!(app.state.workbench.full_screen_presentation);
        assert!(app.state.workbench.full_screen);
        assert_eq!(app.state.ui.take_full_screen_request(), Some(true));
        assert_eq!(app.state.ui.take_full_screen_request(), None);

        app.state.exit_full_screen_presentation();
        assert!(!app.state.workbench.full_screen_presentation);
        assert!(!app.state.workbench.full_screen);
        assert_eq!(app.state.ui.take_full_screen_request(), Some(false));
        assert_eq!(app.state.ui.take_full_screen_request(), None);
    }

    #[test]
    fn log_timestamp_projection_preserves_original_event_age() {
        assert_eq!(
            project_log_timestamp(120.0, Duration::from_secs(45), Duration::from_secs(15)),
            90.0
        );
    }

    #[test]
    fn log_timestamp_projection_bounds_future_source_timestamps_to_now() {
        assert_eq!(
            project_log_timestamp(120.0, Duration::from_secs(10), Duration::from_secs(15)),
            120.0
        );
    }

    /// The plan manager is opened by a global command — the run configuration
    /// chip, the Simulate menu and the palette all reach it from any
    /// workspace — but the only host that painted it ran inside the Simulate
    /// surface. Off that surface the command set its draft and nothing
    /// appeared, so every one of those controls was dead.
    ///
    /// The assertion is the rendered tree rather than the draft: the draft was
    /// already correct when the defect shipped.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_plan_manager_paints_from_a_workspace_that_is_not_simulate() {
        use crate::workbench::commands::vocabulary::Command;

        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.workspace = Workspace::Design;

        Command::ManageSimulationPlans.execute(&mut app);
        assert_ne!(app.state.workbench.workspace, Workspace::Simulate);

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| show_route_overlays(ctx, &mut app),
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("route overlay accessibility tree")
            .nodes;
        let hosts = nodes
            .iter()
            .filter(|(_, node)| node.label() == Some("Simulation plans"))
            .count();
        assert_eq!(
            hosts,
            1,
            "the plan manager has {hosts} hosts off the Simulate surface; \
             labels: {:?}",
            nodes
                .iter()
                .filter_map(|(_, node)| node.label())
                .collect::<Vec<_>>()
        );
    }

    /// Every accessibility label the toast stack draws this frame.
    #[cfg(not(target_arch = "wasm32"))]
    fn toasts_on_screen(ctx: &Context, toasts: &mut crate::ui::widgets::Toasts) -> Vec<String> {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                toasts.show(ctx, 32.0, 40.0, false);
            },
        );
        output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| node.label().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The Task attention setting governs run and campaign notices, and
    /// only them. Every outcome is kept in the bell however it is set; the
    /// setting decides which also raise a toast. A studio or netlist failure
    /// is not a background run, and reaches the bell however it is set.
    ///
    /// This used to be the other way round: the setting was consulted only
    /// by the console mirror, where "Silent" dropped studio failures, while
    /// every run outcome toasted whatever it said.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn background_attention_governs_run_toasts_and_never_the_studio_stream() {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        for (attention, clean_toasts, failure_toasts) in [
            (BackgroundTaskAttention::BadgeAndNotify, true, true),
            (BackgroundTaskAttention::NotifyOnFailureOnly, false, true),
            (BackgroundTaskAttention::Silent, false, false),
        ] {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut app = RSpiceApp::test_instance();
            app.state
                .ui
                .preferences
                .workspace_mut()
                .unwrap()
                .set_background_task_attention(attention);
            app.state.simulation.start_run().lifecycle = Lifecycle::Running;
            app.state.simulation.start_run().lifecycle = Lifecycle::Running;
            announce_run_completion(&ctx, &mut app);
            app.state.simulation.retained.runs[1].lifecycle = Lifecycle::Completed;
            app.state.simulation.retained.runs[0].lifecycle = Lifecycle::Failed;
            app.state.log_buffer.log(
                LogSeverity::Error,
                LogSource::Studio,
                "The project could not be saved",
            );

            announce_run_completion(&ctx, &mut app);
            synchronize_activity_stream(&ctx, &mut app);

            let mut kept = app
                .state
                .ui
                .toasts
                .activity()
                .iter()
                .map(|record| record.message().to_owned())
                .collect::<Vec<_>>();
            kept.sort();
            assert_eq!(
                kept,
                [
                    NOTHING_RETAINED.to_owned(),
                    "The project could not be saved".to_owned(),
                    "no analyses saved \u{b7} 0.000 s".to_owned(),
                ],
                "{attention:?}: every outcome is kept in the bell"
            );
            assert_eq!(app.state.ui.toasts.unread_count(), 3, "{attention:?}");
            let on_screen = toasts_on_screen(&ctx, &mut app.state.ui.toasts);
            let shows = |title: &str| on_screen.iter().any(|label| label.contains(title));
            assert_eq!(shows("Run 1 complete"), clean_toasts, "{attention:?}");
            assert_eq!(shows("Run 2 failed"), failure_toasts, "{attention:?}");
        }
    }

    /// An outcome asks for attention from outside the window only when it
    /// raised a toast, the main window is not the one in front, and the work
    /// took long enough that the reader may have turned away. A failure asks
    /// urgently; anything else asks plainly. The browser build never asks.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn only_long_work_that_ends_behind_the_readers_back_asks_for_attention() {
        use crate::ui::widgets::ToastKind::{Error, Success, Warn};
        use BackgroundTaskAttention::{BadgeAndNotify, NotifyOnFailureOnly, Silent};
        use egui::UserAttentionType::{Critical, Informational};

        let cases = [
            (
                (Some(false), 30.0, Success, BadgeAndNotify),
                Some(Informational),
            ),
            (
                (Some(false), 30.0, Warn, BadgeAndNotify),
                Some(Informational),
            ),
            ((Some(false), 30.0, Error, BadgeAndNotify), Some(Critical)),
            ((Some(false), 5.0, Error, BadgeAndNotify), Some(Critical)),
            ((Some(false), 4.9, Error, BadgeAndNotify), None),
            ((Some(false), f64::NAN, Error, BadgeAndNotify), None),
            ((Some(true), 30.0, Error, BadgeAndNotify), None),
            // A host that cannot say is taken to have the reader's eye.
            ((None, 30.0, Error, BadgeAndNotify), None),
            ((Some(false), 30.0, Success, NotifyOnFailureOnly), None),
            (
                (Some(false), 30.0, Warn, NotifyOnFailureOnly),
                Some(Informational),
            ),
            (
                (Some(false), 30.0, Error, NotifyOnFailureOnly),
                Some(Critical),
            ),
            ((Some(false), 30.0, Success, Silent), None),
            ((Some(false), 30.0, Error, Silent), None),
        ];
        for ((focused, elapsed, kind, attention), expected) in cases {
            assert_eq!(
                attention_request(focused, elapsed, kind, attention),
                expected,
                "{focused:?} {elapsed} {kind:?} {attention:?}"
            );
        }
    }

    /// A long run that fails while the reader is in another window marks
    /// RSpice's main window for them, once, and never brings it forward.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_long_failure_behind_the_readers_back_marks_the_main_window() {
        use crate::state::SimulationRunLifecycle as Lifecycle;
        use egui::{UserAttentionType, ViewportCommand, ViewportId, ViewportInfo};

        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        app.state.simulation.start_run().lifecycle = Lifecycle::Running;
        announce_run_completion(&ctx, &mut app);
        for run in &mut app.state.simulation.retained.runs {
            run.lifecycle = Lifecycle::Failed;
            run.elapsed_time = 90.0;
        }
        let mut input = egui::RawInput::default();
        input.viewports.insert(
            ViewportId::ROOT,
            ViewportInfo {
                focused: Some(false),
                ..Default::default()
            },
        );

        let output = ctx.run_ui(input, |ui| announce_run_completion(ui.ctx(), &mut app));

        let commands = output
            .viewport_output
            .get(&ViewportId::ROOT)
            .map(|viewport| viewport.commands.clone())
            .unwrap_or_default();
        let requests = commands
            .iter()
            .filter(|command| matches!(command, ViewportCommand::RequestUserAttention(_)))
            .collect::<Vec<_>>();
        assert_eq!(
            requests,
            [&ViewportCommand::RequestUserAttention(
                UserAttentionType::Critical
            )],
            "two failures, one request"
        );
        assert!(
            !commands
                .iter()
                .any(|command| matches!(command, ViewportCommand::Focus)),
            "the window never takes focus: {commands:?}"
        );
    }

    /// The notice centre is for failures the reader has to be told about, so
    /// it lifts an Error from the studio or a netlist import and nothing else.
    /// A run's failure is the shell's run announcement and a Problems row; the
    /// engine's lines are that run's own account; a plan entry is a record of
    /// what was committed; Info and Warning lines are records the Console
    /// holds.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn only_studio_and_netlist_failures_reach_the_notice_centre() {
        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        let sources = [
            LogSource::Studio,
            LogSource::Plan,
            LogSource::Run,
            LogSource::Engine,
            LogSource::Netlist,
            LogSource::Hdl,
            LogSource::VerilogA,
        ];
        for source in sources {
            for severity in [
                LogSeverity::Error,
                LogSeverity::Warning,
                LogSeverity::Info,
                LogSeverity::Debug,
            ] {
                app.state.log_buffer.log(
                    severity,
                    source,
                    format!("{} {}", source.name(), severity.name()),
                );
            }
        }

        synchronize_activity_stream(&ctx, &mut app);
        let mut lifted: Vec<String> = app
            .state
            .ui
            .toasts
            .activity()
            .iter()
            .map(|record| record.message().to_owned())
            .collect();
        lifted.sort();
        assert_eq!(
            lifted,
            vec![
                format!("APP {}", LogSeverity::Error.name()),
                format!("NET {}", LogSeverity::Error.name()),
            ]
        );
    }

    /// A prompt is feedback on a gesture, not an event: it reaches neither the
    /// Console nor the notice centre.
    #[test]
    fn a_prompt_is_neither_logged_nor_notified() {
        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        let revision = app.state.log_buffer.revision();

        app.state.prompt("Nothing was deleted.");
        synchronize_activity_stream(&ctx, &mut app);

        assert_eq!(app.state.log_buffer.revision(), revision);
        assert!(
            app.state
                .log_buffer
                .entries()
                .all(|entry| entry.message != "Nothing was deleted.")
        );
        assert!(app.state.ui.toasts.activity().is_empty());
        assert_eq!(
            app.state
                .workbench
                .prompt
                .as_ref()
                .map(|prompt| prompt.text.as_str()),
            Some("Nothing was deleted.")
        );
    }
}
