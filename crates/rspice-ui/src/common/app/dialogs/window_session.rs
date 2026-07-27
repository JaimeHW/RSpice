//! Device-local application-window, document, and restore-session manager.
//!
//! All mutations in this module change presentation ownership only. Source
//! documents, immutable results, jobs, undo history, and audit identities
//! remain in their existing project-owned stores.

use egui::{ComboBox, Context, Grid, RichText};

use crate::common::app::{
    NewWindowInitialContent, RSpiceApp, WindowLayoutChoice, WindowSessionPage, WindowWorkflow,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::commands::Command;
use crate::workbench::state::{ProjectPage, Workspace, WorkspaceDocumentId, WorkspaceLayoutState};
use crate::workbench::{ApplicationWindowId, WorkspacePreset};

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

pub(crate) fn open_window_workflow(app: &mut RSpiceApp, workflow: WindowWorkflow) {
    let source_window = app.state.workbench.window_session.current();
    let active = crate::workbench::chrome::document_bar::document_descriptors(&app.state)
        .into_iter()
        .find(|document| document.active && document.open);
    app.state.dialogs.window_session.open_workflow(workflow);
    let dialog = &mut app.state.dialogs.window_session;
    dialog.source_window = source_window;
    if let Some(active) = active {
        dialog.source_document = Some(active.id);
        dialog.source_label = active.label;
    }
    dialog.destination = app
        .state
        .workbench
        .window_session
        .windows()
        .map(|(id, _)| id)
        .find(|id| *id != source_window);
    if workflow == WindowWorkflow::MonitorRecovery {
        dialog.recover_all_future_windows =
            app.state.workbench.window_session.clamp_restored_windows();
        dialog.recovery_selected = app
            .state
            .workbench
            .window_session
            .windows()
            .filter(|(id, state)| {
                !id.is_primary() && (state.bounds.is_off_screen() || state.bounds.recovery_pending)
            })
            .map(|(id, _)| id)
            .collect();
        if dialog.recovery_selected.is_empty() {
            dialog.recovery_selected = app
                .state
                .workbench
                .window_session
                .secondary_window_ids()
                .into_iter()
                .collect();
        }
    }
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_window_session_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.window_session.open {
            return;
        }
        if let Some(workflow) = self.state.dialogs.window_session.workflow {
            self.render_window_workflow(ctx, workflow);
        } else {
            self.render_window_manager(ctx);
        }
    }

    fn render_window_manager(&mut self, ctx: &Context) {
        let documents =
            crate::workbench::chrome::document_bar::all_document_descriptors(&self.state);
        let open_count = documents.iter().filter(|document| document.open).count();
        let dirty_count = documents.iter().filter(|document| document.dirty).count();
        let mut page = self.state.dialogs.window_session.page;
        let mut action = None;

        let choice = Dialog::new(
            "WINDOW \u{00b7} SINGLE SESSION OWNER",
            "Windows, documents and session",
            "Close",
        )
        .description(
            "Manage application windows, exclusive document presentation, and exact session restoration.",
        )
        .size(DialogSize::Manager)
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
                WindowSessionPage::Windows => {
                    render_windows_page(ui, self, &documents, &mut action)
                }
                WindowSessionPage::Documents => {
                    render_documents_page(ui, &documents, &mut action)
                }
                WindowSessionPage::SessionRestore => {
                    render_session_page(ui, self, open_count, dirty_count, &mut action)
                }
            }
        });

        self.state.dialogs.window_session.page = page;
        if choice != DialogChoice::None {
            self.state.dialogs.window_session.close();
            return;
        }
        if let Some(action) = action {
            self.commit_window_manager_action(ctx, action);
        }
    }

    fn commit_window_manager_action(&mut self, ctx: &Context, action: WindowSessionAction) {
        match action {
            WindowSessionAction::Activate(document) => {
                let owner = self.state.workbench.window_session.owner(&document);
                if owner.is_primary() {
                    self.state.workbench.activate(document.workspace());
                    let _ = crate::workbench::chrome::document_bar::activate_document_by_id(
                        &mut self.state,
                        &document,
                    );
                } else {
                    ctx.send_viewport_cmd_to(owner.viewport_id(), egui::ViewportCommand::Focus);
                }
            }
            WindowSessionAction::Previous => {
                crate::workbench::chrome::document_bar::cycle_document(&mut self.state, true);
            }
            WindowSessionAction::Next => {
                crate::workbench::chrome::document_bar::cycle_document(&mut self.state, false);
            }
            WindowSessionAction::CloseOthers => {
                crate::workbench::chrome::document_bar::close_other_documents(&mut self.state);
            }
            WindowSessionAction::CloseAll => {
                crate::workbench::chrome::document_bar::close_all_documents(&mut self.state);
            }
            WindowSessionAction::WorkspaceLayouts => {
                self.state.dialogs.window_session.close();
                Command::WorkspaceLayouts.execute(self);
            }
            WindowSessionAction::ResetLayout => Command::ResetLayout.execute(self),
            WindowSessionAction::OpenRecovery => {
                self.state.dialogs.window_session.close();
                Command::ProjectPage(ProjectPage::Recovery).execute(self);
            }
            WindowSessionAction::OpenWorkflow(workflow) => open_window_workflow(self, workflow),
            WindowSessionAction::Focus(window) => {
                if window.is_primary() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                } else {
                    ctx.send_viewport_cmd_to(window.viewport_id(), egui::ViewportCommand::Focus);
                }
            }
            WindowSessionAction::CloseWindow(window) => {
                if let Ok(documents) = self.state.workbench.window_session.close_window(window) {
                    self.state.push_user_message(
                        crate::common::app::ConsoleMessage::info(format!(
                            "Closed application window {}; {} document presentation(s) returned to the main window",
                            window.value(),
                            documents.len()
                        )),
                    );
                }
            }
        }
    }

    fn render_window_workflow(&mut self, ctx: &Context, workflow: WindowWorkflow) {
        let (eyebrow, title, primary) = workflow_copy(workflow);
        let source_window = self.state.dialogs.window_session.source_window;
        let source_document = self.state.dialogs.window_session.source_document.clone();
        let source_label = self.state.dialogs.window_session.source_label.clone();
        let mut initial_content = self.state.dialogs.window_session.initial_content;
        let mut destination = self.state.dialogs.window_session.destination;
        let mut layout_choice = self.state.dialogs.window_session.layout_choice;
        let mut restore_on_launch = self.state.dialogs.window_session.restore_on_launch;
        let mut synchronize_chrome = self.state.dialogs.window_session.synchronize_chrome;
        let mut reattach_at_end = self.state.dialogs.window_session.reattach_at_end;
        let mut recovery_selected = self.state.dialogs.window_session.recovery_selected.clone();
        let mut recover_future = self.state.dialogs.window_session.recover_all_future_windows;
        let error = self.state.dialogs.window_session.error.clone();

        let choice = Dialog::new(eyebrow, title, primary)
            .description(workflow_description(workflow))
            .secondary("Cancel")
            .size(DialogSize::Manager)
            .show(ctx, |ui| {
                match workflow {
                    WindowWorkflow::NewApplicationWindow => render_new_window_workflow(
                        ui,
                        &source_label,
                        &mut initial_content,
                        &mut layout_choice,
                        &mut restore_on_launch,
                    ),
                    WindowWorkflow::DetachDocument => render_detach_workflow(
                        ui,
                        self,
                        source_window,
                        &source_label,
                        &mut destination,
                        &mut synchronize_chrome,
                        &mut restore_on_launch,
                    ),
                    WindowWorkflow::MoveDocument => render_move_workflow(
                        ui,
                        self,
                        source_window,
                        &source_label,
                        &mut destination,
                    ),
                    WindowWorkflow::ReattachDocument => {
                        render_reattach_workflow(ui, &source_label, &mut reattach_at_end)
                    }
                    WindowWorkflow::ConsolidateWindows => render_consolidate_workflow(ui, self),
                    WindowWorkflow::MonitorRecovery => render_monitor_recovery_workflow(
                        ui,
                        self,
                        &mut recovery_selected,
                        &mut recover_future,
                    ),
                }
                if let Some(error) = &error {
                    ui.add_space(8.0);
                    ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
                }
            });

        {
            let dialog = &mut self.state.dialogs.window_session;
            dialog.initial_content = initial_content;
            dialog.destination = destination;
            dialog.layout_choice = layout_choice;
            dialog.restore_on_launch = restore_on_launch;
            dialog.synchronize_chrome = synchronize_chrome;
            dialog.reattach_at_end = reattach_at_end;
            dialog.recovery_selected = recovery_selected;
            dialog.recover_all_future_windows = recover_future;
        }
        match choice {
            DialogChoice::Primary => {
                if let Err(error) =
                    self.commit_window_workflow(workflow, source_window, source_document)
                {
                    self.state.dialogs.window_session.error = Some(error);
                } else {
                    self.state.dialogs.window_session.close();
                }
            }
            DialogChoice::Secondary | DialogChoice::Cancelled => {
                self.state.dialogs.window_session.close();
            }
            DialogChoice::None | DialogChoice::Ghost => {}
        }
    }

    fn commit_window_workflow(
        &mut self,
        workflow: WindowWorkflow,
        source_window: ApplicationWindowId,
        source_document: Option<WorkspaceDocumentId>,
    ) -> Result<(), String> {
        let dialog = self.state.dialogs.window_session.clone();
        if self
            .state
            .workbench
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
            && self.state.workbench.window_session.owner(document) != source_window
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
                let current_workspace = self
                    .state
                    .workbench
                    .window_session
                    .state(source_window)
                    .map(|window| window.workspace)
                    .unwrap_or(self.state.workbench.workspace);
                let (workspace, layout) = match dialog.initial_content {
                    NewWindowInitialContent::EmptyProjectWorkspace => {
                        (Workspace::Project, layout_for_choice(dialog.layout_choice))
                    }
                    NewWindowInitialContent::CloneCurrentWorkspaceLayout => (
                        current_workspace,
                        self.state.workbench.current_workspace_layout(),
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
                    self.state.workbench.window_session.windows().count() + 1
                );
                let destination = self.state.workbench.window_session.create_window(
                    title,
                    workspace,
                    layout,
                    dialog.restore_on_launch,
                );
                if dialog.initial_content == NewWindowInitialContent::MoveActiveDocument {
                    let document =
                        source_document.expect("move-active validation ran before window creation");
                    if let Err(error) = self.state.workbench.window_session.move_document(
                        document,
                        source_window,
                        destination,
                    ) {
                        let _ = self
                            .state
                            .workbench
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
                    self.state
                        .workbench
                        .window_session
                        .move_document(document, source_window, destination)
                        .map_err(|error| error.to_string())?;
                } else {
                    let title = if dialog.source_label.is_empty() {
                        "Detached document".to_owned()
                    } else {
                        dialog.source_label.clone()
                    };
                    let destination = self
                        .state
                        .workbench
                        .window_session
                        .detach_document(
                            document,
                            title,
                            self.state.workbench.current_workspace_layout(),
                            dialog.restore_on_launch,
                        )
                        .map_err(|error| error.to_string())?;
                    if let Some(window) = self.state.workbench.window_session.state_mut(destination)
                    {
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
                self.state
                    .workbench
                    .window_session
                    .move_document(document, source_window, destination)
                    .map_err(|error| error.to_string())?;
                let _ = self
                    .state
                    .workbench
                    .window_session
                    .remove_empty_window(source_window);
            }
            WindowWorkflow::ReattachDocument => {
                let document = source_document
                    .ok_or_else(|| "No active detached document is available.".to_owned())?;
                if source_window.is_primary() {
                    return Err("The active document is already in the main window.".to_owned());
                }
                self.state
                    .workbench
                    .window_session
                    .reattach_document(document, source_window, dialog.reattach_at_end)
                    .map_err(|error| error.to_string())?;
                let _ = self
                    .state
                    .workbench
                    .window_session
                    .remove_empty_window(source_window);
            }
            WindowWorkflow::ConsolidateWindows => {
                let count = self.state.workbench.window_session.consolidate();
                if count == 0 {
                    return Err(
                        "There are no detached application windows to consolidate.".to_owned()
                    );
                }
            }
            WindowWorkflow::MonitorRecovery => {
                if dialog.recovery_selected.is_empty() {
                    return Err("Select at least one application window to recover.".to_owned());
                }
                self.state
                    .workbench
                    .window_session
                    .set_clamp_restored_windows(dialog.recover_all_future_windows);
                let count = self
                    .state
                    .workbench
                    .window_session
                    .recover_windows(dialog.recovery_selected);
                if count == 0 {
                    return Err("The selected application windows are no longer open.".to_owned());
                }
            }
        }
        Ok(())
    }
}

fn render_windows_page(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
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
    Grid::new("window-session.windows")
        .num_columns(5)
        .striped(true)
        .spacing([18.0, 7.0])
        .show(ui, |ui| {
            ui.strong("Application window");
            ui.strong("Monitor");
            ui.strong("Documents");
            ui.strong("State");
            ui.strong("Action");
            ui.end_row();
            for (id, window) in app.state.workbench.window_session.windows() {
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
                } else if id == app.state.workbench.window_session.current() {
                    "active"
                } else {
                    "open"
                });
                ui.horizontal(|ui| {
                    if ui.button("Focus").clicked() {
                        *action = Some(WindowSessionAction::Focus(id));
                    }
                    if !id.is_primary() && ui.button("Close").clicked() {
                        *action = Some(WindowSessionAction::CloseWindow(id));
                    }
                });
                ui.end_row();
            }
        });
    ui.add_space(10.0);
    ui.columns(2, |columns| {
        columns[0].strong("Document ownership");
        columns[0].label(
            "Detaching changes presentation only. Working documents, results, jobs, undo history, and audit identity remain project-owned.",
        );
        columns[1].strong("Display topology");
        columns[1].label(
            "Restored geometry is clamped to available monitors and can be recovered at any time.",
        );
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
    app: &RSpiceApp,
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
            ui.label(
                app.state
                    .workbench
                    .window_session
                    .windows()
                    .count()
                    .to_string(),
            );
            ui.end_row();
            ui.label("Open documents");
            ui.label(open_count.to_string());
            ui.end_row();
            ui.label("Modified documents");
            ui.label(dirty_count.to_string());
            ui.end_row();
            ui.label("Autosave checkpoint");
            ui.label(match app.state.ui.autosave_minutes {
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

fn render_new_window_workflow(
    ui: &mut egui::Ui,
    active_name: &str,
    initial: &mut NewWindowInitialContent,
    layout: &mut WindowLayoutChoice,
    restore: &mut bool,
) {
    ComboBox::from_label("Initial content")
        .selected_text(match initial {
            NewWindowInitialContent::EmptyProjectWorkspace => "Empty project workspace",
            NewWindowInitialContent::CloneCurrentWorkspaceLayout => {
                "Clone current workspace layout"
            }
            NewWindowInitialContent::MoveActiveDocument => "Move active document",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                initial,
                NewWindowInitialContent::EmptyProjectWorkspace,
                "Empty project workspace",
            );
            ui.selectable_value(
                initial,
                NewWindowInitialContent::CloneCurrentWorkspaceLayout,
                "Clone current workspace layout",
            );
            ui.add_enabled_ui(!active_name.is_empty(), |ui| {
                ui.selectable_value(
                    initial,
                    NewWindowInitialContent::MoveActiveDocument,
                    format!("Move active document \u{00b7} {active_name}"),
                );
            });
        });
    monitor_combo(ui);
    layout_choice(ui, layout);
    ui.checkbox(restore, "Restore this window on next launch");
}

fn render_detach_workflow(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    source: ApplicationWindowId,
    active_name: &str,
    destination: &mut Option<ApplicationWindowId>,
    synchronize: &mut bool,
    restore: &mut bool,
) {
    document_summary(
        ui,
        active_name,
        "Working state remains owned by this project.",
    );
    destination_combo(ui, app, source, destination, true);
    monitor_combo(ui);
    ui.checkbox(
        synchronize,
        "Keep source navigator and inspector synchronized",
    );
    ui.checkbox(restore, "Restore detached state on next launch");
}

fn render_move_workflow(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    source: ApplicationWindowId,
    active_name: &str,
    destination: &mut Option<ApplicationWindowId>,
) {
    document_summary(
        ui,
        active_name,
        "Unsaved edits, view state, selection, and source-return history move with the document.",
    );
    destination_combo(ui, app, source, destination, false);
}

fn render_reattach_workflow(ui: &mut egui::Ui, active_name: &str, at_end: &mut bool) {
    document_summary(
        ui,
        active_name,
        "Return this document to the main application window without changing project or result data.",
    );
    ComboBox::from_label("Insertion")
        .selected_text(if *at_end {
            "End of owning workspace"
        } else {
            "After active document in owning workspace"
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(at_end, false, "After active document in owning workspace");
            ui.selectable_value(at_end, true, "End of owning workspace");
        });
}

fn render_consolidate_workflow(ui: &mut egui::Ui, app: &RSpiceApp) {
    ui.label(
        "All detached documents return to their owning workspaces in the main window. Active jobs, selections, and immutable results are unchanged.",
    );
    Grid::new("window-session.consolidate")
        .num_columns(4)
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Window");
            ui.strong("Documents");
            ui.strong("Destination");
            ui.strong("Restore state");
            ui.end_row();
            for (id, window) in app.state.workbench.window_session.windows() {
                if id.is_primary() {
                    continue;
                }
                ui.label(&window.title);
                ui.label(window.documents.len().to_string());
                ui.label(format!("Main \u{00b7} {}", window.workspace.label()));
                ui.label("remove detached window");
                ui.end_row();
            }
        });
}

fn render_monitor_recovery_workflow(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    selected: &mut std::collections::BTreeSet<ApplicationWindowId>,
    recover_future: &mut bool,
) {
    ui.label(
        "Recovery changes window geometry only. Engineering documents and application state are unchanged.",
    );
    Grid::new("window-session.monitor-recovery")
        .num_columns(4)
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Recover");
            ui.strong("Window");
            ui.strong("Saved monitor");
            ui.strong("Current visibility");
            ui.end_row();
            for (id, window) in app.state.workbench.window_session.windows() {
                if id.is_primary() {
                    continue;
                }
                let mut checked = selected.contains(&id);
                if ui.checkbox(&mut checked, "").changed() {
                    if checked {
                        selected.insert(id);
                    } else {
                        selected.remove(&id);
                    }
                }
                ui.label(&window.title);
                ui.label(
                    window
                        .bounds
                        .monitor_size
                        .map(|size| format!("{:.0} \u{00d7} {:.0}", size[0], size[1]))
                        .unwrap_or_else(|| "Unavailable".to_owned()),
                );
                ui.label(
                    if window.bounds.is_off_screen() || window.bounds.recovery_pending {
                        "off-screen or topology changed"
                    } else {
                        "visible"
                    },
                );
                ui.end_row();
            }
        });
    ui.checkbox(
        recover_future,
        "Clamp all future restored windows to available work areas",
    );
}

fn destination_combo(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    source: ApplicationWindowId,
    destination: &mut Option<ApplicationWindowId>,
    allow_new: bool,
) {
    let selected = destination
        .and_then(|id| {
            app.state
                .workbench
                .window_session
                .state(id)
                .map(|window| window.title.clone())
        })
        .unwrap_or_else(|| "New application window".to_owned());
    ComboBox::from_label("Destination")
        .selected_text(selected)
        .show_ui(ui, |ui| {
            if allow_new {
                ui.selectable_value(destination, None, "New application window");
            }
            for (id, window) in app.state.workbench.window_session.windows() {
                if id != source {
                    ui.selectable_value(destination, Some(id), &window.title);
                }
            }
        });
}

fn layout_choice(ui: &mut egui::Ui, layout: &mut WindowLayoutChoice) {
    ComboBox::from_label("Workspace layout")
        .selected_text(match layout {
            WindowLayoutChoice::Engineering => "Engineering",
            WindowLayoutChoice::Review => "Review",
            WindowLayoutChoice::PlotPresentation => "Plot presentation",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(layout, WindowLayoutChoice::Engineering, "Engineering");
            ui.selectable_value(layout, WindowLayoutChoice::Review, "Review");
            ui.selectable_value(
                layout,
                WindowLayoutChoice::PlotPresentation,
                "Plot presentation",
            );
        });
}

fn monitor_combo(ui: &mut egui::Ui) {
    ComboBox::from_label("Monitor")
        .selected_text("Use operating-system default")
        .show_ui(ui, |ui| {
            let _selected = ui.selectable_label(true, "Use operating-system default");
        });
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

fn document_summary(ui: &mut egui::Ui, name: &str, detail: &str) {
    ui.group(|ui| {
        ui.strong(if name.is_empty() {
            "No active document"
        } else {
            name
        });
        ui.label(detail);
    });
}

const fn workflow_copy(workflow: WindowWorkflow) -> (&'static str, &'static str, &'static str) {
    match workflow {
        WindowWorkflow::NewApplicationWindow => (
            "WINDOW \u{00b7} INDEPENDENT WORKSPACE",
            "New application window",
            "Create window",
        ),
        WindowWorkflow::DetachDocument => (
            "WINDOW \u{00b7} DOCUMENT OWNERSHIP",
            "Detach active document",
            "Detach document",
        ),
        WindowWorkflow::MoveDocument => (
            "WINDOW \u{00b7} DOCUMENT TRANSFER",
            "Move document to another window",
            "Move document",
        ),
        WindowWorkflow::ReattachDocument => (
            "WINDOW \u{00b7} CONSOLIDATE",
            "Reattach active document",
            "Reattach document",
        ),
        WindowWorkflow::ConsolidateWindows => (
            "WINDOW \u{00b7} SESSION LAYOUT",
            "Consolidate application windows",
            "Consolidate windows",
        ),
        WindowWorkflow::MonitorRecovery => (
            "WINDOW \u{00b7} DISPLAY TOPOLOGY RECOVERY",
            "Recover off-screen windows",
            "Recover selected windows",
        ),
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
