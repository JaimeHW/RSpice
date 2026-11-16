//! Typed workbench commands and their single dispatch boundary.
//!
//! The workbench never paints a visible action without routing it here.  A
//! command is omitted from a menu when its behavior is not implemented; the
//! UI does not advertise speculative or placeholder capability.

use crate::workbench::commands::vocabulary::ShellCommand;
use crate::workbench::menu_bar::{FileMenuAction, dispatch_file_menu_action};
use crate::workbench::{AppState, RSpiceApp};
use rspice_app_design::workbench::app::actions::workspace::HistoryDirection;
use rspice_app_workbench::workbench::app::dialogs::window_session::{
    WindowSessionPage, WindowWorkflow, open_window_workflow,
};
use rspice_app_workbench::workbench::framework::command_registry::{CommandOwner, ids};
use rspice_app_workbench::workbench::state::pane_focus::PaneStep;

use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::{
    console_views::ConsoleChannel, workbench_session::ConsolePage,
};

mod edit_target;
mod enablement;
mod registry;
mod veriloga_commits;
pub(crate) mod vocabulary;

impl ShellCommand {
    /// Shared by menu/palette dispatch and the keyboard dispatcher.
    pub(crate) const fn commits_inspector_edit(self) -> bool {
        matches!(
            self,
            Self::OpenWorkspace(_)
                | Self::PreviousDocument
                | Self::NextDocument
                | Self::CloseOtherDocuments
                | Self::CloseAllDocuments
                | Self::ToggleInspector
                | Self::ToggleFocusMode
                | Self::ResetLayout
        )
    }

    /// The part of the product that carries the command out: the slice whose
    /// code its dispatch reaches.
    const fn owner(self) -> CommandOwner {
        match self {
            Self::OpenWorkspace(_)
            | Self::Exit
            | Self::RepeatLastCommand
            | Self::KeyboardShortcuts
            | Self::HelpCenter
            | Self::Examples
            | Self::ReleaseNotes
            | Self::SystemDiagnostics
            | Self::SupportBundle
            | Self::LegalPrivacy
            | Self::About => CommandOwner::Shell,
            Self::ToggleFullScreen
            | Self::RevertActiveDocument
            | Self::CloseActiveDocument
            | Self::ToggleNavigator
            | Self::ToggleInspector
            | Self::ToggleConsole
            | Self::OpenConsole
            | Self::OpenProblems
            | Self::OpenNotifications
            | Self::ToggleConsoleMaximized
            | Self::ClearConsole
            | Self::ToggleFocusMode
            | Self::ResetLayout
            | Self::NewApplicationWindow
            | Self::DetachDocument
            | Self::MoveDocumentToWindow
            | Self::ReattachDocument
            | Self::ConsolidateWindows
            | Self::MonitorRecovery
            | Self::PreviousDocument
            | Self::NextDocument
            | Self::NextPane
            | Self::PreviousPane
            | Self::CloseOtherDocuments
            | Self::CloseAllDocuments
            | Self::WindowManager
            | Self::MinimizeWindow
            | Self::ZoomWindow
            | Self::CommandPalette => CommandOwner::Framework,
            Self::ConfigurationSets => CommandOwner::Project,
        }
    }

    /// What the command does once the dispatcher's prologue has let it
    /// through. The dispatcher is its one caller, so no command skips the
    /// prologue.
    pub(in crate::workbench) fn carry_out(self, app: &mut RSpiceApp) {
        match self {
            Self::OpenWorkspace(workspace) => {
                if workspace_available(app.state.platform.project_lifecycle.is_open(), workspace) {
                    activate_workspace(app, workspace);
                }
            }
            Self::RevertActiveDocument => file_action(app, FileMenuAction::RevertActiveDocument),
            Self::CloseActiveDocument => file_action(app, FileMenuAction::CloseActiveDocument),
            Self::Exit => file_action(app, FileMenuAction::Exit),
            Self::ToggleFullScreen => {
                if app.state.workbench.full_screen_presentation {
                    app.state.exit_full_screen_presentation();
                } else {
                    app.state.enter_full_screen_presentation();
                }
            }
            Self::ToggleNavigator => {
                if app.state.workbench.navigator_visible {
                    app.state.workbench.dismiss_navigator();
                } else {
                    app.state.workbench.navigator_visible = true;
                }
            }
            Self::ToggleInspector => {
                if app.state.workbench.inspector_visible {
                    app.state.workbench.dismiss_inspector();
                } else {
                    app.state.workbench.inspector_visible = true;
                }
            }
            Self::ToggleConsole => {
                if app.state.workbench.focus_mode {
                    app.state.workbench.focus_mode = false;
                    app.state.workbench.console_visible = true;
                } else {
                    app.state.workbench.console_visible = !app.state.workbench.console_visible;
                }
                if !app.state.workbench.console_visible {
                    app.state.workbench.console_maximized = false;
                }
            }
            Self::OpenProblems => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
                app.state.workbench.console_page = rspice_app_workbench::workbench::state::workbench_session::ConsolePage::Problems;
            }
            Self::OpenConsole => app.state.workbench.open_console(),
            Self::OpenNotifications => app.state.workbench.notification_center_open = true,
            Self::ToggleConsoleMaximized => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
                app.state.workbench.console_visible = true;
            }
            // The console's clear control is this command's only route, and it
            // sits on one page at a time. Clearing every page from a control
            // labelled for the visible one would discard work the user cannot
            // see, so the command owns the same page scope the control shows.
            Self::ClearConsole => match app.state.workbench.console_page {
                ConsolePage::Console
                    if app.state.workbench.console_view.channel == ConsoleChannel::Session =>
                {
                    app.state.clear_primary_log()
                }
                ConsolePage::Console => {}
                ConsolePage::Problems
                | ConsolePage::Measurements
                | ConsolePage::TaskLog
                | ConsolePage::References => {}
            },
            Self::ToggleFocusMode => {
                app.state.workbench.focus_mode = !app.state.workbench.focus_mode;
                if app.state.workbench.focus_mode {
                    app.state.workbench.close_drawer();
                }
            }
            Self::ResetLayout => app.state.workbench.reset_layout(),
            Self::NewApplicationWindow => {
                open_window_workflow(app, WindowWorkflow::NewApplicationWindow)
            }
            Self::DetachDocument => open_window_workflow(app, WindowWorkflow::DetachDocument),
            Self::MoveDocumentToWindow => open_window_workflow(app, WindowWorkflow::MoveDocument),
            Self::ReattachDocument => open_window_workflow(app, WindowWorkflow::ReattachDocument),
            Self::ConsolidateWindows => {
                open_window_workflow(app, WindowWorkflow::ConsolidateWindows)
            }
            Self::MonitorRecovery => open_window_workflow(app, WindowWorkflow::MonitorRecovery),
            Self::PreviousDocument => {
                rspice_app_workbench::workbench::chrome::document_bar::cycle_document(
                    &mut app.state,
                    true,
                );
            }
            Self::NextDocument => {
                rspice_app_workbench::workbench::chrome::document_bar::cycle_document(
                    &mut app.state,
                    false,
                );
            }
            Self::NextPane => app.state.workbench.pane_focus.request(PaneStep::Next),
            Self::PreviousPane => app.state.workbench.pane_focus.request(PaneStep::Previous),
            Self::CloseOtherDocuments => {
                let count =
                    rspice_app_workbench::workbench::chrome::document_bar::close_other_documents(
                        &mut app.state,
                    );
                app.state.prompt(format!(
                    "Closed {count} other document tab(s); the project data is kept."
                ));
            }
            Self::CloseAllDocuments => {
                let count =
                    rspice_app_workbench::workbench::chrome::document_bar::close_all_documents(
                        &mut app.state,
                    );
                app.state.prompt(format!(
                    "Closed {count} document tab(s); the project data is kept."
                ));
            }
            Self::WindowManager => app
                .state
                .workbench
                .window_session_dialog
                .open(WindowSessionPage::Windows),
            Self::MinimizeWindow => {
                app.state.workbench.outbox.push(
                    rspice_app_workbench::workbench::framework::requests::WindowFrame::Minimize,
                )
            }
            Self::ZoomWindow => app
                .state
                .workbench
                .outbox
                .push(rspice_app_workbench::workbench::framework::requests::WindowFrame::Zoom),
            Self::RepeatLastCommand => match vocabulary::last_repeatable(
                app.state.workbench.last_repeatable_command.as_deref(),
            ) {
                Some(spec) if app.command_key_enabled(spec.key) => {
                    app.execute_command_key(spec.key);
                }
                Some(spec) => app.state.prompt(format!(
                    "{} cannot run here, so it was not repeated.",
                    spec.label
                )),
                None => app.state.prompt("No placement or edit has run yet."),
            },
            Self::ConfigurationSets => {
                crate::workbench::session::project_transactions::show_configuration_page(
                    &mut app.state,
                );
            }
            Self::CommandPalette => open_command_palette(&mut app.state),
            Self::KeyboardShortcuts => app.state.shell.shortcuts_help = true,
            Self::HelpCenter => {
                crate::workbench::app::dialogs::help::site::open_docs(&mut app.state)
            }
            Self::Examples => app.state.project.open_examples(),
            Self::ReleaseNotes => crate::workbench::app::dialogs::help::site::open_release_notes(
                &mut app.state.workbench.outbox,
            ),
            Self::SystemDiagnostics => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::Diagnostics,
            ),
            Self::SupportBundle => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::SupportBundle,
            ),
            Self::LegalPrivacy => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::LegalPrivacy,
            ),
            Self::About => app.state.shell.about = true,
        }
    }
}

mod command_helpers;

use command_helpers::*;
pub(in crate::workbench) use command_helpers::{
    active_schematic_editor, answer_project_request, carry_out_code_hand_off, file_action,
    run_project_file_action,
};
pub(crate) use command_helpers::{repeat_menu_label, workbench_menu_label};
pub(in crate::workbench) use edit_target::source_editor_redirect;
mod design_requests;
#[cfg(test)]
pub(crate) use design_requests::answer_surface_requests;
pub(in crate::workbench) use design_requests::{
    answer_surface_request, open_connectivity_source_map, select_probed_trace,
};

#[cfg(test)]
pub(crate) mod tests;
