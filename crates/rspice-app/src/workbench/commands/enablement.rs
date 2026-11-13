//! Whether a command can run right now: the one predicate every menu row,
//! toolbar button, palette entry and shortcut asks before it acts, and the
//! test [`Command::availability`] builds its reasons on.

use super::*;

impl Command {
    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        // The dispatcher's own guards, so what is drawn enabled is what runs.
        if self.held_by_project_operation(&state.platform) {
            return false;
        }
        if self.lacks_open_project(&state.platform) {
            return false;
        }
        if let Some((_, accepted)) = self.source_editor_redirect(&state.workbench) {
            return accepted;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.platform.project_lifecycle.is_open(), workspace)
            }
            Self::RevertActiveDocument => {
                state.platform.project_lifecycle.accepted().is_some()
                    && crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(
                        state,
                    )
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::CloseActiveDocument => {
                crate::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            // The netlist page's own history is the shell's to step; the
            // design feature's part answers for every other.
            Self::Undo | Self::Redo => {
                let direction = if self == Self::Undo {
                    HistoryDirection::Undo
                } else {
                    HistoryDirection::Redo
                };
                (netlist_history_steps(state, direction)
                    && !state.model.safe_mode.project_read_only()
                    && !state.schematic_edit_in_progress())
                    || app.routed_command_key_enabled(self.key()) == Some(true)
            }
            // Whether the part that takes an edit command here can carry it out.
            Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Duplicate
            | Self::Delete
            | Self::SelectAll
            | Self::DeselectAll => app.routed_command_key_enabled(self.key()) == Some(true),
            Self::RepeatLastCommand => {
                active_schematic_editor(app)
                    && vocabulary::last_repeatable(
                        state.workbench.last_repeatable_command.as_deref(),
                    )
                    .is_some_and(|spec| app.command_key_enabled(spec.key))
            }
            Self::ConfigurationSets => state.platform.project_lifecycle.is_open(),
            // A view command is the answer of the part that takes it where the
            // reader is working: Results where it is in front, the design
            // feature in a schematic or symbol editor.
            Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::ZoomToSelection
            | Self::ZoomBox
            | Self::ZoomPrevious => app.routed_command_key_enabled(self.key()) == Some(true),
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
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::MoveDocumentToWindow => {
                state.workbench.window_session.windows().count() > 1
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ReattachDocument => {
                !state.workbench.window_session.current().is_primary()
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ConsolidateWindows | Self::MonitorRecovery => !state
                .workbench
                .window_session
                .secondary_window_ids()
                .is_empty(),
            Self::PreviousDocument | Self::NextDocument => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .filter(|document| document.open)
                    .count()
                    > 1
            }
            Self::CloseOtherDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable && !document.active)
            }
            Self::CloseAllDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
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
