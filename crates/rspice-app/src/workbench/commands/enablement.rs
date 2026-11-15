//! Whether a command can run right now: the one predicate every menu row,
//! toolbar button, palette entry and shortcut asks before it acts, and the
//! test [`ShellCommand::availability`] builds its reasons on.

use super::*;
use rspice_app_workbench::workbench::framework::feature::held_back;

impl ShellCommand {
    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        // The dispatcher's own guards, so what is drawn enabled is what runs.
        if held_back(self.declared_spec().flags, &state.platform).is_some() {
            return false;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.platform.project_lifecycle.is_open(), workspace)
            }
            Self::RevertActiveDocument => {
                state.platform.project_lifecycle.accepted().is_some()
                    && rspice_app_services::workbench::lifecycle::project_lifecycle::active_document_is_dirty(
                        state,
                    )
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::CloseActiveDocument => {
                rspice_app_services::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            Self::RepeatLastCommand => {
                active_schematic_editor(app)
                    && vocabulary::last_repeatable(
                        state.workbench.last_repeatable_command.as_deref(),
                    )
                    .is_some_and(|spec| app.command_key_enabled(spec.key))
            }
            Self::ConfigurationSets => state.platform.project_lifecycle.is_open(),
            Self::NewApplicationWindow
            | Self::DetachDocument
            | Self::MoveDocumentToWindow
            | Self::ReattachDocument
            | Self::ConsolidateWindows
            | Self::MonitorRecovery
                if state.application_modal_open() =>
            {
                false
            }
            Self::NewApplicationWindow => true,
            Self::DetachDocument => {
                rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::MoveDocumentToWindow => {
                state.workbench.window_session.windows().count() > 1
                    && rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(
                        state,
                    )
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::ReattachDocument => {
                !state.workbench.window_session.current().is_primary()
                    && rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(
                        state,
                    )
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::ConsolidateWindows | Self::MonitorRecovery => !state
                .workbench
                .window_session
                .secondary_window_ids()
                .is_empty(),
            Self::PreviousDocument | Self::NextDocument => {
                rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .filter(|document| document.open)
                    .count()
                    > 1
            }
            Self::CloseOtherDocuments => {
                rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable && !document.active)
            }
            Self::CloseAllDocuments => {
                rspice_app_workbench::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable)
            }
            // A run's log is a record; only the session's log is cleared.
            Self::ClearConsole => match state.workbench.console_page {
                ConsolePage::Console => {
                    state.workbench.console_view.channel == ConsoleChannel::Session
                        && !state.model.log_buffer.is_empty()
                }
                ConsolePage::Problems
                | ConsolePage::Measurements
                | ConsolePage::TaskLog
                | ConsolePage::References => false,
            },
            _ => true,
        }
    }

    /// Whether the state this command switches is on right now, for a
    /// command that switches one; `None` for a command that acts once.
    ///
    /// Each reading is the one the command flips, so choosing the row always
    /// turns its mark over. The console reads off in focus mode because
    /// focus mode hides it and the command leaves focus mode to show it.
    pub fn checked(self, app: &RSpiceApp) -> Option<bool> {
        let workbench = &app.state.workbench;
        match self {
            Self::ToggleNavigator => Some(workbench.navigator_visible),
            Self::ToggleInspector => Some(workbench.inspector_visible),
            Self::ToggleConsole => Some(workbench.console_visible && !workbench.focus_mode),
            Self::ToggleFocusMode => Some(workbench.focus_mode),
            Self::ToggleFullScreen => Some(workbench.full_screen_presentation),
            _ => None,
        }
    }
}
