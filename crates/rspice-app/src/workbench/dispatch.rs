//! The one dispatcher every command goes through, whoever raises it: a menu
//! row, a toolbar button, the palette, a shortcut or another command.
//!
//! A command runs behind one prologue, always in the same order: a project
//! operation in flight holds it back, a command that needs a project needs
//! one open, an active source editor takes the editing commands it handles,
//! a read-only schematic refuses an edit, a pending inspector edit commits
//! first, and a repeatable command is recorded for Repeat. Only then does the
//! command's owner carry it out, through the shell's legacy table
//! [`Command::run_legacy_arm`].

use crate::workbench::RSpiceApp;
#[cfg(test)]
use crate::workbench::commands::dispatch_probe;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::{active_schematic_editor, command_edits_schematic};

impl Command {
    /// Carry this command out behind the prologue every command shares.
    pub(super) fn dispatch(self, app: &mut RSpiceApp) {
        #[cfg(test)]
        if dispatch_probe::intercept(self) {
            return;
        }
        // Both guards are also `is_enabled`'s, so the command is drawn
        // disabled and a shortcut for it is silent, as in any desktop app.
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(
            &app.state.platform,
        ) && self.blocked_by_project_operation()
        {
            return;
        }
        if self.requires_open_project() && !app.state.platform.project_lifecycle.is_open() {
            return;
        }
        if let Some(request) = self.source_editor_request()
            && let Some(editor) = app.state.workbench.active_source_editor
        {
            if editor.editable || !request.mutates_source() {
                app.state.workbench.outbox.push(request);
            }
            return;
        }
        if active_schematic_editor(app)
            && command_edits_schematic(self)
            && app.state.schematic_edit_read_only()
        {
            app.state.deny_read_only_edit();
            return;
        }
        if self.commits_inspector_edit() && !app.state.commit_pending_inspector_edit() {
            return;
        }
        if self.repeatable() && self.is_enabled(app) {
            app.state.workbench.last_repeatable_command = Some(self.stable_id().to_owned());
        }
        self.run_legacy_arm(app);
    }
}
