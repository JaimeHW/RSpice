//! The shell's edit target: where each of the workbench's routed commands
//! goes, asked in the order it always has been.
//!
//! Undo and Redo step the netlist page's own history where that page is in
//! front with a step to take, and the design feature's everywhere else. The
//! edit and view commands go to the part in front that takes them: Results
//! first, then the design feature. Find opens the netlist page's find bar
//! there, the Navigator's search anywhere else, and in a Verilog-A cell view
//! the source find. Cancel lets go of what the design workspace holds first,
//! then of Results' cursors, and last steps the schematic's interaction
//! back.

use crate::workbench::RSpiceApp;
use rspice_app_code::workbench::code::CodeFeature;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_design::workbench::app::actions::workspace::HistoryDirection;
use rspice_app_design::workbench::design::DesignFeature;
use rspice_app_results::workbench::results::ResultsFeature;
use rspice_app_workbench::workbench::framework::command_registry::{
    CommandAvailability, WorkbenchCommand,
};
use rspice_app_workbench::workbench::framework::feature::held_back;
use rspice_app_workbench::workbench::framework::requests::SourceEditorRequest;
use rspice_app_workbench::workbench::state::workbench_session::WorkbenchSession;

use super::{
    active_schematic_editor, active_symbol_editor, history_command, netlist_history_steps,
};

/// The request the active source editor takes in `command`'s place, with
/// whether it accepts it: a read-only editor refuses a request that would
/// change its text. `None` when no source editor takes the command.
pub(in crate::workbench) fn source_editor_redirect(
    command: WorkbenchCommand,
    workbench: &WorkbenchSession,
) -> Option<(SourceEditorRequest, bool)> {
    let request = command.source_editor_request()?;
    let editor = workbench.active_source_editor?;
    Some((request, editor.editable || !request.mutates_source()))
}

impl RSpiceApp {
    /// Whether `command` would run if raised now: the dispatcher's own
    /// guards first, so what is drawn enabled is what runs, then the source
    /// editor that takes it in its place, then the part that takes it here.
    pub(in crate::workbench) fn workbench_command_enabled(
        &self,
        command: WorkbenchCommand,
    ) -> bool {
        let state = &self.state;
        if held_back(command.declared_spec().flags, &state.platform).is_some() {
            return false;
        }
        if let Some((_, accepted)) = source_editor_redirect(command, &state.workbench) {
            return accepted;
        }
        match command {
            // The netlist page's own history is the shell's to step; the
            // design feature's part answers for every other.
            WorkbenchCommand::Undo | WorkbenchCommand::Redo => {
                let direction = if command == WorkbenchCommand::Undo {
                    HistoryDirection::Undo
                } else {
                    HistoryDirection::Redo
                };
                (netlist_history_steps(state, direction)
                    && !state.model.safe_mode.project_read_only()
                    && !state.schematic_edit_in_progress())
                    || self.routed_command_key_enabled(command.key()) == Some(true)
            }
            // Whether the part that takes an edit or view command here can
            // carry it out: Results where it is in front, the design feature
            // in a schematic or symbol editor.
            WorkbenchCommand::Cut
            | WorkbenchCommand::Copy
            | WorkbenchCommand::Paste
            | WorkbenchCommand::Duplicate
            | WorkbenchCommand::Delete
            | WorkbenchCommand::SelectAll
            | WorkbenchCommand::DeselectAll
            | WorkbenchCommand::ZoomIn
            | WorkbenchCommand::ZoomOut
            | WorkbenchCommand::ZoomFit
            | WorkbenchCommand::ZoomOneToOne
            | WorkbenchCommand::ZoomToSelection
            | WorkbenchCommand::ZoomBox
            | WorkbenchCommand::ZoomPrevious => {
                self.routed_command_key_enabled(command.key()) == Some(true)
            }
            WorkbenchCommand::FindInDesign | WorkbenchCommand::Cancel => true,
        }
    }

    /// Whether `command` can run here, and why not.
    pub(in crate::workbench) fn workbench_command_availability(
        &self,
        command: WorkbenchCommand,
    ) -> CommandAvailability {
        if let Some(reason) = held_back(command.declared_spec().flags, &self.state.platform) {
            return CommandAvailability::Disabled(reason);
        }
        if self.workbench_command_enabled(command) {
            return CommandAvailability::Available;
        }
        CommandAvailability::Disabled(match command {
            // A step taken mid-drag would be folded into the drag when it
            // lands, so the history waits for the edit to finish or cancel.
            WorkbenchCommand::Undo | WorkbenchCommand::Redo
                if self.state.schematic_edit_in_progress() =>
            {
                "finish or cancel the edit in progress first"
            }
            WorkbenchCommand::Undo => "nothing to undo",
            WorkbenchCommand::Redo => "nothing to redo",
            WorkbenchCommand::Cut
            | WorkbenchCommand::Copy
            | WorkbenchCommand::Duplicate
            | WorkbenchCommand::Delete => "select an editable object",
            WorkbenchCommand::Paste => "clipboard has no compatible content",
            WorkbenchCommand::DeselectAll | WorkbenchCommand::ZoomToSelection => {
                "nothing is selected"
            }
            WorkbenchCommand::ZoomPrevious
                if !active_schematic_editor(self)
                    && !active_symbol_editor(self)
                    && self.routed_command_key_enabled(command.key()).is_none() =>
            {
                "open a schematic or symbol"
            }
            WorkbenchCommand::ZoomPrevious => "the view has not changed yet",
            _ => "command is unavailable in this context",
        })
    }

    /// Carry out `command` once the dispatcher's prologue has let it
    /// through, in the part that takes it where the reader is working.
    /// Anywhere no part takes an edit or view command there is nothing to
    /// edit or move, so nothing happens.
    pub(in crate::workbench) fn carry_out_workbench_command(&mut self, command: WorkbenchCommand) {
        let key = command.key();
        match command {
            WorkbenchCommand::Undo => history_command(self, HistoryDirection::Undo),
            WorkbenchCommand::Redo => history_command(self, HistoryDirection::Redo),
            WorkbenchCommand::Cut
            | WorkbenchCommand::Copy
            | WorkbenchCommand::Paste
            | WorkbenchCommand::Duplicate
            | WorkbenchCommand::Delete
            | WorkbenchCommand::SelectAll
            | WorkbenchCommand::DeselectAll
            | WorkbenchCommand::ZoomIn
            | WorkbenchCommand::ZoomOut
            | WorkbenchCommand::ZoomFit
            | WorkbenchCommand::ZoomOneToOne
            | WorkbenchCommand::ZoomToSelection
            | WorkbenchCommand::ZoomBox
            | WorkbenchCommand::ZoomPrevious => {
                self.route_command_key(key);
            }
            // A Verilog-A cell view has no instances or nets to find, so
            // Find searches its source bundle there.
            WorkbenchCommand::FindInDesign => {
                if !self.route_command_key_to::<CodeFeature>(key)
                    && !self.route_command_key_to::<DesignFeature>(key)
                {
                    self.execute_command_key(CodeCommand::FindCodeDocument.key());
                }
            }
            WorkbenchCommand::Cancel => {
                if !self.route_command_key_to::<DesignFeature>(key)
                    && !self.route_command_key_to::<ResultsFeature>(key)
                {
                    self.state.model.schematic.cancel_interaction_step();
                }
            }
        }
    }
}
