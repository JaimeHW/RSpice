//! Device-local application-window, document, and restore-session manager.
//!
//! All mutations in this module change presentation ownership only. Source
//! documents, immutable results, jobs, undo history, and audit identities
//! remain in their existing project-owned stores.

use egui::{Context, Grid, RichText};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{CommandForm, Dialog, DialogChoice, DialogSize};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::lifecycle::window_session::ApplicationWindowId;
use crate::workbench::state::{Workspace, WorkspaceDocumentId, WorkspaceLayoutState};
use rspice_app_state::workbench::preferences::WorkspacePreset;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WindowSessionPage {
    #[default]
    Windows,
    Documents,
    SessionRestore,
}

impl WindowSessionPage {
    pub const ALL: [Self; 3] = [Self::Windows, Self::Documents, Self::SessionRestore];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::Documents => "Documents",
            Self::SessionRestore => "Session restore",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowWorkflow {
    NewApplicationWindow,
    DetachDocument,
    MoveDocument,
    ReattachDocument,
    ConsolidateWindows,
    MonitorRecovery,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NewWindowInitialContent {
    #[default]
    EmptyProjectWorkspace,
    CloneCurrentWorkspaceLayout,
    MoveActiveDocument,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WindowLayoutChoice {
    #[default]
    Engineering,
    Review,
    PlotPresentation,
}

#[derive(Debug, Clone, Default)]
pub struct WindowSessionDialogState {
    pub open: bool,
    pub page: WindowSessionPage,
    pub workflow: Option<WindowWorkflow>,
    pub source_window: ApplicationWindowId,
    pub source_document: Option<crate::workbench::state::WorkspaceDocumentId>,
    pub source_label: String,
    pub initial_content: NewWindowInitialContent,
    pub destination: Option<ApplicationWindowId>,
    pub layout_choice: WindowLayoutChoice,
    pub restore_on_launch: bool,
    pub synchronize_chrome: bool,
    pub recover_all_future_windows: bool,
    pub reattach_at_end: bool,
    pub recovery_selected: std::collections::BTreeSet<ApplicationWindowId>,
    pub error: Option<String>,
}

impl WindowSessionDialogState {
    pub fn open(&mut self, page: WindowSessionPage) {
        *self = Self::default();
        self.open = true;
        self.page = page;
    }

    pub fn open_workflow(&mut self, workflow: WindowWorkflow) {
        *self = Self {
            open: true,
            workflow: Some(workflow),
            restore_on_launch: true,
            synchronize_chrome: true,
            recover_all_future_windows: true,
            ..Self::default()
        };
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
enum WindowSessionAction {
    Activate(WorkspaceDocumentId),
    Previous,
    Next,
    CloseOthers,
    CloseAll,
    WorkspaceLayouts,
    ResetLayout,
    OpenRecovery,
    OpenWorkflow(WindowWorkflow),
    Focus(ApplicationWindowId),
    CloseWindow(ApplicationWindowId),
}

pub fn open_window_workflow(app: &mut impl WorkbenchHost, workflow: WindowWorkflow) {
    let source_window = app.workbench().window_session.current();
    let active = crate::workbench::chrome::document_bar::document_descriptors(app.session())
        .into_iter()
        .find(|document| document.active && document.open);
    let workbench = app.workbench_mut();
    workbench.window_session_dialog.open_workflow(workflow);
    let dialog = &mut workbench.window_session_dialog;
    dialog.source_window = source_window;
    if let Some(active) = active {
        dialog.source_document = Some(active.id);
        dialog.source_label = active.label;
    }
    dialog.destination = workbench
        .window_session
        .windows()
        .map(|(id, _)| id)
        .find(|id| *id != source_window);
    if workflow == WindowWorkflow::MonitorRecovery {
        dialog.recover_all_future_windows = workbench.window_session.clamp_restored_windows();
        dialog.recovery_selected = workbench
            .window_session
            .windows()
            .filter(|(id, state)| {
                !id.is_primary() && (state.bounds.is_off_screen() || state.bounds.recovery_pending)
            })
            .map(|(id, _)| id)
            .collect();
        if dialog.recovery_selected.is_empty() {
            dialog.recovery_selected = workbench
                .window_session
                .secondary_window_ids()
                .into_iter()
                .collect();
        }
    }
}

pub fn render_window_session_dialog(app: &mut impl WorkbenchHost, ctx: &Context) {
    if !app.workbench().window_session_dialog.open {
        return;
    }
    if let Some(workflow) = app.workbench().window_session_dialog.workflow {
        render_window_workflow(app, ctx, workflow);
    } else {
        render_window_manager(app, ctx);
    }
}

fn render_window_manager(app: &mut impl WorkbenchHost, ctx: &Context) {
    let documents = crate::workbench::chrome::document_bar::all_document_descriptors(app.session());
    let open_count = documents.iter().filter(|document| document.open).count();
    let dirty_count = documents.iter().filter(|document| document.dirty).count();
    let mut page = app.workbench().window_session_dialog.page;
    let mut action = None;

    let choice = Dialog::new("Windows, documents and session", "Close")
        .description("Manage application windows, exclusive document views, and session restore.")
        .size(DialogSize::Manager)
        // A bar would only ever show while the surface settles; the
        // gutter it withheld was bare surface down the right edge.
        .overlay_scrollbar()
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for candidate in WindowSessionPage::ALL {
                    if ui
                        .selectable_label(page == candidate, candidate.label())
                        .clicked()
                    {
                        page = candidate;
                    }
                }
            });
            ui.separator();
            match page {
                WindowSessionPage::Windows => render_windows_page(ui, app, &documents, &mut action),
                WindowSessionPage::Documents => render_documents_page(ui, &documents, &mut action),
                WindowSessionPage::SessionRestore => {
                    render_session_page(ui, app, open_count, dirty_count, &mut action)
                }
            }
        });

    app.workbench_mut().window_session_dialog.page = page;
    if choice != DialogChoice::None {
        app.workbench_mut().window_session_dialog.close();
        return;
    }
    if let Some(action) = action {
        commit_window_manager_action(app, ctx, action);
    }
}

fn commit_window_manager_action(
    app: &mut impl WorkbenchHost,
    ctx: &Context,
    action: WindowSessionAction,
) {
    match action {
        WindowSessionAction::Activate(document) => {
            let owner = app.workbench().window_session.owner(&document);
            if owner.is_primary() {
                app.workbench_mut().activate(document.workspace());
                let _ = crate::workbench::chrome::document_bar::activate_document_by_id(
                    app.session_mut(),
                    &document,
                );
            } else {
                ctx.send_viewport_cmd_to(owner.viewport_id(), egui::ViewportCommand::Focus);
            }
        }
        WindowSessionAction::Previous => {
            crate::workbench::chrome::document_bar::cycle_document(app.session_mut(), true);
        }
        WindowSessionAction::Next => {
            crate::workbench::chrome::document_bar::cycle_document(app.session_mut(), false);
        }
        WindowSessionAction::CloseOthers => {
            crate::workbench::chrome::document_bar::close_other_documents(app.session_mut());
        }
        WindowSessionAction::CloseAll => {
            crate::workbench::chrome::document_bar::close_all_documents(app.session_mut());
        }
        WindowSessionAction::WorkspaceLayouts => {
            app.workbench_mut().window_session_dialog.close();
            app.execute(ids::WORKSPACE_LAYOUTS);
        }
        WindowSessionAction::ResetLayout => app.execute(ids::RESET_LAYOUT),
        WindowSessionAction::OpenRecovery => {
            app.workbench_mut().window_session_dialog.close();
            app.execute(ids::PROJECT_RECOVERY);
        }
        WindowSessionAction::OpenWorkflow(workflow) => open_window_workflow(app, workflow),
        WindowSessionAction::Focus(window) => {
            if window.is_primary() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            } else {
                ctx.send_viewport_cmd_to(window.viewport_id(), egui::ViewportCommand::Focus);
            }
        }
        WindowSessionAction::CloseWindow(window) => {
            if let Ok(documents) = app.workbench_mut().window_session.close_window(window) {
                app.workbench_mut().prompt(match documents.len() {
                    0 => format!("Closed window {}.", window.value()),
                    1 => format!(
                        "Closed window {}; its document returned to the main window.",
                        window.value()
                    ),
                    count => format!(
                        "Closed window {}; its {count} documents returned to the main window.",
                        window.value()
                    ),
                });
            }
        }
    }
}

fn render_window_workflow(app: &mut impl WorkbenchHost, ctx: &Context, workflow: WindowWorkflow) {
    let source_window = app.workbench().window_session_dialog.source_window;
    let source_document = app
        .workbench()
        .window_session_dialog
        .source_document
        .clone();
    let choice = match workflow {
        WindowWorkflow::ConsolidateWindows | WindowWorkflow::MonitorRecovery => {
            let dialog = &app.workbench().window_session_dialog;
            let mut selected = dialog.recovery_selected.clone();
            let mut recover_future = dialog.recover_all_future_windows;
            let error = dialog.error.clone();
            let choice = show_window_list_workflow(
                ctx,
                workflow,
                app,
                &mut selected,
                &mut recover_future,
                error.as_deref(),
            );
            let dialog = &mut app.workbench_mut().window_session_dialog;
            if selected != dialog.recovery_selected
                || recover_future != dialog.recover_all_future_windows
            {
                // A refusal from the last attempt is about the draft it was made on.
                dialog.error = None;
            }
            dialog.recovery_selected = selected;
            dialog.recover_all_future_windows = recover_future;
            choice
        }
        WindowWorkflow::NewApplicationWindow
        | WindowWorkflow::DetachDocument
        | WindowWorkflow::MoveDocument
        | WindowWorkflow::ReattachDocument => {
            let others = app
                .workbench()
                .window_session
                .windows()
                .filter(|(id, _)| *id != source_window)
                .map(|(id, window)| (id, window.title.clone()))
                .collect::<Vec<_>>();
            show_window_form(
                ctx,
                workflow,
                &others,
                &mut app.workbench_mut().window_session_dialog,
            )
        }
    };
    match choice {
        DialogChoice::Primary => {
            if let Err(error) =
                commit_window_workflow(app, workflow, source_window, source_document)
            {
                app.workbench_mut().window_session_dialog.error = Some(error);
            } else {
                app.workbench_mut().window_session_dialog.close();
            }
        }
        DialogChoice::Secondary | DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.workbench_mut().window_session_dialog.close();
        }
        DialogChoice::None => {}
    }
}

fn commit_window_workflow(
    app: &mut impl WorkbenchHost,
    workflow: WindowWorkflow,
    source_window: ApplicationWindowId,
    source_document: Option<WorkspaceDocumentId>,
) -> Result<(), String> {
    let dialog = app.workbench().window_session_dialog.clone();
    if app
        .workbench()
        .window_session
        .state(source_window)
        .is_none()
    {
        return Err(
            "The source application window was closed before this operation could commit."
                .to_owned(),
        );
    }
    if let Some(document) = source_document.as_ref()
        && app.workbench().window_session.owner(document) != source_window
    {
        return Err(
            "The active document moved to another window; review the current ownership and try again."
                .to_owned(),
        );
    }

    match workflow {
        WindowWorkflow::NewApplicationWindow => {
            if dialog.initial_content == NewWindowInitialContent::MoveActiveDocument
                && source_document.is_none()
            {
                return Err("No active document is available to move.".to_owned());
            }
            let current_workspace = app
                .workbench()
                .window_session
                .state(source_window)
                .map(|window| window.workspace)
                .unwrap_or(app.workbench().workspace);
            let (workspace, layout) = match dialog.initial_content {
                NewWindowInitialContent::EmptyProjectWorkspace => {
                    (Workspace::Project, layout_for_choice(dialog.layout_choice))
                }
                NewWindowInitialContent::CloneCurrentWorkspaceLayout => (
                    current_workspace,
                    app.workbench().current_workspace_layout(),
                ),
                NewWindowInitialContent::MoveActiveDocument => (
                    source_document
                        .as_ref()
                        .map_or(current_workspace, WorkspaceDocumentId::workspace),
                    layout_for_choice(dialog.layout_choice),
                ),
            };
            let title = format!(
                "RSpice window {}",
                app.workbench().window_session.windows().count() + 1
            );
            let destination = app.workbench_mut().window_session.create_window(
                title,
                workspace,
                layout,
                dialog.restore_on_launch,
            );
            if dialog.initial_content == NewWindowInitialContent::MoveActiveDocument {
                let document =
                    source_document.expect("move-active validation ran before window creation");
                if let Err(error) = app.workbench_mut().window_session.move_document(
                    document,
                    source_window,
                    destination,
                ) {
                    let _ = app
                        .workbench_mut()
                        .window_session
                        .remove_empty_window(destination);
                    return Err(error.to_string());
                }
            }
        }
        WindowWorkflow::DetachDocument => {
            let document = source_document
                .ok_or_else(|| "No active document is available to detach.".to_owned())?;
            if let Some(destination) = dialog.destination {
                app.workbench_mut()
                    .window_session
                    .move_document(document, source_window, destination)
                    .map_err(|error| error.to_string())?;
            } else {
                let title = if dialog.source_label.is_empty() {
                    "Detached document".to_owned()
                } else {
                    dialog.source_label.clone()
                };
                let workbench = app.workbench_mut();
                let destination = workbench
                    .window_session
                    .detach_document(
                        document,
                        title,
                        workbench.current_workspace_layout(),
                        dialog.restore_on_launch,
                    )
                    .map_err(|error| error.to_string())?;
                if let Some(window) = workbench.window_session.state_mut(destination) {
                    window.synchronize_chrome_with_primary = dialog.synchronize_chrome;
                }
            }
        }
        WindowWorkflow::MoveDocument => {
            let document = source_document
                .ok_or_else(|| "No active document is available to move.".to_owned())?;
            let destination = dialog
                .destination
                .ok_or_else(|| "Select a destination application window.".to_owned())?;
            app.workbench_mut()
                .window_session
                .move_document(document, source_window, destination)
                .map_err(|error| error.to_string())?;
            let _ = app
                .workbench_mut()
                .window_session
                .remove_empty_window(source_window);
        }
        WindowWorkflow::ReattachDocument => {
            let document = source_document
                .ok_or_else(|| "No active detached document is available.".to_owned())?;
            if source_window.is_primary() {
                return Err("The active document is already in the main window.".to_owned());
            }
            app.workbench_mut()
                .window_session
                .reattach_document(document, source_window, dialog.reattach_at_end)
                .map_err(|error| error.to_string())?;
            let _ = app
                .workbench_mut()
                .window_session
                .remove_empty_window(source_window);
        }
        WindowWorkflow::ConsolidateWindows => {
            let count = app.workbench_mut().window_session.consolidate();
            if count == 0 {
                return Err("There are no detached application windows to consolidate.".to_owned());
            }
        }
        WindowWorkflow::MonitorRecovery => {
            if dialog.recovery_selected.is_empty() {
                return Err("Select at least one application window to recover.".to_owned());
            }
            app.workbench_mut()
                .window_session
                .set_clamp_restored_windows(dialog.recover_all_future_windows);
            let count = app
                .workbench_mut()
                .window_session
                .recover_windows(dialog.recovery_selected);
            if count == 0 {
                return Err("The selected application windows are no longer open.".to_owned());
            }
        }
    }
    Ok(())
}

fn render_windows_page(
    ui: &mut egui::Ui,
    app: &impl WorkbenchHost,
    documents: &[crate::workbench::chrome::document_bar::WorkspaceDocumentDescriptor],
    action: &mut Option<WindowSessionAction>,
) {
    ui.horizontal(|ui| {
        if ui.button("New window\u{2026}").clicked() {
            *action = Some(WindowSessionAction::OpenWorkflow(
                WindowWorkflow::NewApplicationWindow,
            ));
        }
        if ui.button("Detach active\u{2026}").clicked() {
            *action = Some(WindowSessionAction::OpenWorkflow(
                WindowWorkflow::DetachDocument,
            ));
        }
        if ui.button("Consolidate\u{2026}").clicked() {
            *action = Some(WindowSessionAction::OpenWorkflow(
                WindowWorkflow::ConsolidateWindows,
            ));
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Monitor recovery\u{2026}").clicked() {
                *action = Some(WindowSessionAction::OpenWorkflow(
                    WindowWorkflow::MonitorRecovery,
                ));
            }
        });
    });
    ui.add_space(8.0);
    let current = app.workbench().window_session.current();
    // Focus and Close act on a window other than this one; with no other
    // window open, no row has an action and the column goes.
    let actions = app.workbench().window_session.windows().count() > 1;
    Grid::new("window-session.windows")
        .num_columns(if actions { 5 } else { 4 })
        .striped(true)
        .spacing([18.0, 7.0])
        .show(ui, |ui| {
            ui.strong("Application window");
            ui.strong("Monitor");
            ui.strong("Documents");
            ui.strong("State");
            if actions {
                ui.strong("Action");
            }
            ui.end_row();
            for (id, window) in app.workbench().window_session.windows() {
                ui.label(&window.title);
                ui.label(
                    window
                        .bounds
                        .monitor_size
                        .map(|size| format!("{:.0} \u{00d7} {:.0}", size[0], size[1]))
                        .unwrap_or_else(|| "Operating-system default".to_owned()),
                );
                ui.label(
                    documents
                        .iter()
                        .filter(|document| document.open && document.owner == id)
                        .count()
                        .to_string(),
                );
                ui.label(if window.bounds.recovery_pending {
                    "recovery pending"
                } else if id == current {
                    "active"
                } else {
                    "open"
                });
                if actions {
                    ui.horizontal(|ui| {
                        // The window this dialog is in already has the focus.
                        if id != current && ui.button("Focus").clicked() {
                            *action = Some(WindowSessionAction::Focus(id));
                        }
                        if !id.is_primary() && ui.button("Close").clicked() {
                            *action = Some(WindowSessionAction::CloseWindow(id));
                        }
                    });
                }
                ui.end_row();
            }
        });
}

fn render_documents_page(
    ui: &mut egui::Ui,
    documents: &[crate::workbench::chrome::document_bar::WorkspaceDocumentDescriptor],
    action: &mut Option<WindowSessionAction>,
) {
    ui.horizontal(|ui| {
        let multiple = documents.iter().filter(|document| document.open).count() > 1;
        if ui
            .add_enabled(multiple, egui::Button::new("Previous"))
            .clicked()
        {
            *action = Some(WindowSessionAction::Previous);
        }
        if ui
            .add_enabled(multiple, egui::Button::new("Next"))
            .clicked()
        {
            *action = Some(WindowSessionAction::Next);
        }
        if ui
            .add_enabled(multiple, egui::Button::new("Close other documents"))
            .clicked()
        {
            *action = Some(WindowSessionAction::CloseOthers);
        }
        if ui
            .add_enabled(multiple, egui::Button::new("Close all documents"))
            .clicked()
        {
            *action = Some(WindowSessionAction::CloseAll);
        }
    });
    ui.add_space(8.0);
    section_heading(ui, "DOCUMENT OWNERSHIP");
    Grid::new("window-session.documents")
        .num_columns(6)
        .striped(true)
        .spacing([18.0, 7.0])
        .show(ui, |ui| {
            ui.strong("Document");
            ui.strong("Workspace");
            ui.strong("Window");
            ui.strong("State");
            ui.strong("Changes");
            ui.strong("Action");
            ui.end_row();
            for document in documents {
                ui.label(&document.label);
                ui.label(document.id.workspace().label());
                ui.label(if document.owner.is_primary() {
                    "Main".to_owned()
                } else {
                    format!("Window {}", document.owner.value())
                });
                ui.label(if document.active {
                    "Active"
                } else if document.open {
                    "Open"
                } else {
                    "Closed"
                });
                ui.label(if document.dirty { "Modified" } else { "Saved" });
                if ui
                    .add_enabled(document.open, egui::Button::new("Show"))
                    .clicked()
                {
                    *action = Some(WindowSessionAction::Activate(document.id.clone()));
                }
                ui.end_row();
            }
        });
}

fn render_session_page(
    ui: &mut egui::Ui,
    app: &impl WorkbenchHost,
    open_count: usize,
    dirty_count: usize,
    action: &mut Option<WindowSessionAction>,
) {
    section_heading(ui, "SESSION RESTORE");
    Grid::new("window-session.restore")
        .num_columns(2)
        .spacing([24.0, 8.0])
        .show(ui, |ui| {
            ui.label("Application session");
            ui.label("Restore windows and documents after a clean exit");
            ui.end_row();
            ui.label("Windows retained");
            ui.label(app.workbench().window_session.windows().count().to_string());
            ui.end_row();
            ui.label("Open documents");
            ui.label(open_count.to_string());
            ui.end_row();
            ui.label("Modified documents");
            ui.label(dirty_count.to_string());
            ui.end_row();
            ui.label("Autosave checkpoint");
            ui.label(match app.platform().autosave_minutes {
                0 => "Disabled".to_owned(),
                minutes => format!("Every {minutes} minutes while modified"),
            });
            ui.end_row();
            ui.label("Recovery authority");
            ui.label("Project-bound checkpoints and accepted saved baselines");
            ui.end_row();
        });
    ui.add_space(12.0);
    ui.horizontal(|ui| {
        if ui.button("Workspace layouts\u{2026}").clicked() {
            *action = Some(WindowSessionAction::WorkspaceLayouts);
        }
        if ui.button("Reset workspace layout\u{2026}").clicked() {
            *action = Some(WindowSessionAction::ResetLayout);
        }
        if ui.button("Open project recovery\u{2026}").clicked() {
            *action = Some(WindowSessionAction::OpenRecovery);
        }
    });
}

/// What a window form refuses before anything is chosen, in the words the
/// footer states it in. `None` when its primary can run.
fn window_form_refusal(
    workflow: WindowWorkflow,
    document: bool,
    source: ApplicationWindowId,
    other_windows: bool,
) -> Option<&'static str> {
    match workflow {
        WindowWorkflow::DetachDocument if !document => Some("No document is open to detach."),
        WindowWorkflow::MoveDocument if !document => Some("No document is open to move."),
        WindowWorkflow::MoveDocument if !other_windows => {
            Some("No other window is open to move it to.")
        }
        WindowWorkflow::ReattachDocument if !document => Some("No detached document is active."),
        WindowWorkflow::ReattachDocument if source.is_primary() => {
            Some("The active document is already in the main window.")
        }
        _ => None,
    }
}

const NEW_WINDOW_CONTENT: [&str; 3] = [
    "Empty project workspace",
    "Current workspace layout",
    "Active document",
];
const NEW_WINDOW_LAYOUTS: [&str; 3] = ["Engineering", "Review", "Plot presentation"];
const REATTACH_PLACES: [&str; 2] = ["After the active document", "At the end"];
const NEW_WINDOW: &str = "New window";

/// New window, detach, move and reattach: a command form over the active
/// document, where it goes and how the window comes back.
fn show_window_form(
    ctx: &Context,
    workflow: WindowWorkflow,
    others: &[(ApplicationWindowId, String)],
    draft: &mut WindowSessionDialogState,
) -> DialogChoice {
    let (title, primary) = workflow_copy(workflow);
    let refusal = window_form_refusal(
        workflow,
        draft.source_document.is_some(),
        draft.source_window,
        !others.is_empty(),
    );
    let error = draft.error.clone();
    let status = match (error.as_deref(), refusal) {
        (Some(error), _) => Err(error),
        (None, Some(refusal)) => Err(refusal),
        (None, None) => Ok(None),
    };
    let document = draft.source_label.clone();
    let before = (
        draft.initial_content,
        draft.destination,
        draft.layout_choice,
        draft.restore_on_launch,
        draft.synchronize_chrome,
        draft.reattach_at_end,
    );
    let choice = CommandForm::new(title, primary)
        .describe(workflow_description(workflow))
        .primary_enabled(refusal.is_none())
        .status(status)
        .show(ctx, |rows| {
            if workflow != WindowWorkflow::NewApplicationWindow && !document.is_empty() {
                rows.value("Document", &document);
            }
            match workflow {
                WindowWorkflow::NewApplicationWindow => {
                    // Only a document that is open can move into the window.
                    let offered = if draft.source_document.is_some() {
                        3
                    } else {
                        2
                    };
                    let contents = NEW_WINDOW_CONTENT[..offered]
                        .iter()
                        .map(|content| (*content).to_owned())
                        .collect::<Vec<_>>();
                    let current = match draft.initial_content {
                        NewWindowInitialContent::EmptyProjectWorkspace => 0,
                        NewWindowInitialContent::CloneCurrentWorkspaceLayout => 1,
                        NewWindowInitialContent::MoveActiveDocument => 2,
                    };
                    if let Some(picked) = rows.select(
                        "Content",
                        "window-session.new-window.content",
                        NEW_WINDOW_CONTENT[current],
                        &contents,
                    ) {
                        draft.initial_content = match picked {
                            1 => NewWindowInitialContent::CloneCurrentWorkspaceLayout,
                            2 => NewWindowInitialContent::MoveActiveDocument,
                            _ => NewWindowInitialContent::EmptyProjectWorkspace,
                        };
                    }
                    if draft.initial_content == NewWindowInitialContent::MoveActiveDocument {
                        rows.derived(&document);
                    }
                    // A copy of this workspace brings its own docks.
                    if draft.initial_content != NewWindowInitialContent::CloneCurrentWorkspaceLayout
                    {
                        let layouts = NEW_WINDOW_LAYOUTS.map(str::to_owned);
                        let current = match draft.layout_choice {
                            WindowLayoutChoice::Engineering => 0,
                            WindowLayoutChoice::Review => 1,
                            WindowLayoutChoice::PlotPresentation => 2,
                        };
                        if let Some(picked) = rows.select(
                            "Layout",
                            "window-session.new-window.layout",
                            NEW_WINDOW_LAYOUTS[current],
                            &layouts,
                        ) {
                            draft.layout_choice = match picked {
                                1 => WindowLayoutChoice::Review,
                                2 => WindowLayoutChoice::PlotPresentation,
                                _ => WindowLayoutChoice::Engineering,
                            };
                        }
                    }
                    rows.checks(
                        "Options",
                        &mut [("Restore on next launch", &mut draft.restore_on_launch)],
                    );
                }
                WindowWorkflow::DetachDocument | WindowWorkflow::MoveDocument => {
                    let detach = workflow == WindowWorkflow::DetachDocument;
                    // Detaching may open a window of its own; moving goes to
                    // one already open.
                    let destinations = detach
                        .then(|| (None, NEW_WINDOW.to_owned()))
                        .into_iter()
                        .chain(others.iter().map(|(id, title)| (Some(*id), title.clone())))
                        .collect::<Vec<_>>();
                    let selected = destinations
                        .iter()
                        .find(|(id, _)| *id == draft.destination)
                        .or_else(|| destinations.first())
                        .map_or_else(String::new, |(_, title)| title.clone());
                    let labels = destinations
                        .iter()
                        .map(|(_, title)| title.clone())
                        .collect::<Vec<_>>();
                    if let Some(picked) = rows.select(
                        "Destination",
                        "window-session.destination",
                        &selected,
                        &labels,
                    ) && let Some((id, _)) = destinations.get(picked)
                    {
                        draft.destination = *id;
                    }
                    if detach && draft.destination.is_none() {
                        rows.checks(
                            "Options",
                            &mut [
                                (
                                    "Synchronize navigator and inspector",
                                    &mut draft.synchronize_chrome,
                                ),
                                ("Restore on next launch", &mut draft.restore_on_launch),
                            ],
                        );
                    }
                }
                WindowWorkflow::ReattachDocument => {
                    let places = REATTACH_PLACES.map(str::to_owned);
                    if let Some(picked) = rows.select(
                        "Insert",
                        "window-session.reattach.place",
                        REATTACH_PLACES[usize::from(draft.reattach_at_end)],
                        &places,
                    ) {
                        draft.reattach_at_end = picked == 1;
                    }
                }
                WindowWorkflow::ConsolidateWindows | WindowWorkflow::MonitorRecovery => {}
            }
            None
        });
    let after = (
        draft.initial_content,
        draft.destination,
        draft.layout_choice,
        draft.restore_on_launch,
        draft.synchronize_chrome,
        draft.reattach_at_end,
    );
    if after != before {
        // A refusal from the last attempt is about the draft it was made on.
        draft.error = None;
    }
    choice
}

/// What consolidate or recovery refuses, in the words the footer states it
/// in. `None` when its primary can run.
fn window_list_refusal(
    workflow: WindowWorkflow,
    detached: bool,
    selected: bool,
) -> Option<&'static str> {
    match workflow {
        WindowWorkflow::ConsolidateWindows if !detached => Some("No window is detached."),
        WindowWorkflow::MonitorRecovery if !detached => Some("No other window is open."),
        WindowWorkflow::MonitorRecovery if !selected => Some("Select a window to recover."),
        _ => None,
    }
}

/// Consolidate and monitor recovery: a command form over the windows other
/// than the main one.
fn show_window_list_workflow(
    ctx: &Context,
    workflow: WindowWorkflow,
    app: &impl WorkbenchHost,
    selected: &mut std::collections::BTreeSet<ApplicationWindowId>,
    recover_future: &mut bool,
    error: Option<&str>,
) -> DialogChoice {
    let (title, primary) = workflow_copy(workflow);
    let detached = app
        .workbench()
        .window_session
        .windows()
        .filter(|(id, _)| !id.is_primary())
        .map(|(id, window)| {
            let lost = window.bounds.is_off_screen() || window.bounds.recovery_pending;
            let label = if lost && workflow == WindowWorkflow::MonitorRecovery {
                format!("{} \u{00b7} off screen", window.title)
            } else {
                window.title.clone()
            };
            (id, label, window.documents.len())
        })
        .collect::<Vec<_>>();
    let refusal = window_list_refusal(workflow, !detached.is_empty(), !selected.is_empty());
    let status = match (error, refusal) {
        (Some(error), _) => Err(error),
        (None, Some(refusal)) => Err(refusal),
        (None, None) => Ok(None),
    };
    CommandForm::new(title, primary)
        .describe(workflow_description(workflow))
        .primary_enabled(refusal.is_none())
        .status(status)
        .show(ctx, |rows| {
            match workflow {
                WindowWorkflow::MonitorRecovery if !detached.is_empty() => {
                    let mut checked = detached
                        .iter()
                        .map(|(id, _, _)| selected.contains(id))
                        .collect::<Vec<_>>();
                    let mut items = detached
                        .iter()
                        .zip(checked.iter_mut())
                        .map(|((_, label, _), checked)| (label.as_str(), checked))
                        .collect::<Vec<_>>();
                    rows.checks("Windows", &mut items);
                    for ((id, _, _), checked) in detached.iter().zip(&checked) {
                        if *checked {
                            selected.insert(*id);
                        } else {
                            selected.remove(id);
                        }
                    }
                    rows.checks(
                        "Options",
                        &mut [("Keep restored windows on screen", recover_future)],
                    );
                }
                WindowWorkflow::ConsolidateWindows if !detached.is_empty() => {
                    let titles = detached
                        .iter()
                        .map(|(_, label, _)| label.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let documents = detached.iter().map(|(_, _, count)| count).sum::<usize>();
                    rows.value("Windows", &titles);
                    rows.value("Documents", &documents.to_string());
                }
                // Nothing to act on: the form still states it, and the footer
                // says why the primary is off.
                _ => rows.value("Windows", "None"),
            }
            None
        })
}

fn layout_for_choice(choice: WindowLayoutChoice) -> WorkspaceLayoutState {
    match choice {
        WindowLayoutChoice::Engineering => {
            WorkspaceLayoutState::for_preset(WorkspacePreset::Engineering)
        }
        WindowLayoutChoice::Review => WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: true,
            console_visible: false,
            inspector_width: 332.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        },
        WindowLayoutChoice::PlotPresentation => WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: false,
            console_visible: false,
            focus_mode: true,
            ..WorkspaceLayoutState::default()
        },
    }
}

const fn workflow_copy(workflow: WindowWorkflow) -> (&'static str, &'static str) {
    match workflow {
        WindowWorkflow::NewApplicationWindow => ("New application window", "Create window"),
        WindowWorkflow::DetachDocument => ("Detach active document", "Detach document"),
        WindowWorkflow::MoveDocument => ("Move document to another window", "Move document"),
        WindowWorkflow::ReattachDocument => ("Reattach active document", "Reattach document"),
        WindowWorkflow::ConsolidateWindows => {
            ("Consolidate application windows", "Consolidate windows")
        }
        WindowWorkflow::MonitorRecovery => {
            ("Recover off-screen windows", "Recover selected windows")
        }
    }
}

const fn workflow_description(workflow: WindowWorkflow) -> &'static str {
    match workflow {
        WindowWorkflow::NewApplicationWindow => {
            "Create an independent native workspace without duplicating engineering source state."
        }
        WindowWorkflow::DetachDocument => {
            "Move the active document presentation into another application window."
        }
        WindowWorkflow::MoveDocument => {
            "Transfer exclusive presentation ownership to an existing application window."
        }
        WindowWorkflow::ReattachDocument => {
            "Return the active detached document to the main application window."
        }
        WindowWorkflow::ConsolidateWindows => {
            "Return every detached document to the main window and close secondary windows."
        }
        WindowWorkflow::MonitorRecovery => {
            "Move selected application windows back onto an available display work area."
        }
    }
}

fn section_heading(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.add_space(4.0);
}
