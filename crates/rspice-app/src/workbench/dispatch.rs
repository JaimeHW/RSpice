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

use crate::workbench::app_state::PlatformState;
#[cfg(test)]
use crate::workbench::commands::dispatch_probe;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::{active_schematic_editor, command_edits_schematic};
use crate::workbench::documents::text_editor_commands::SourceEditorRequest;
use crate::workbench::{RSpiceApp, WorkbenchSession};

impl Command {
    /// Carry this command out behind the prologue every command shares.
    pub(super) fn dispatch(self, app: &mut RSpiceApp) {
        #[cfg(test)]
        if dispatch_probe::intercept(self) {
            return;
        }
        // Both guards are also `is_enabled`'s, so the command is drawn
        // disabled and a shortcut for it is silent, as in any desktop app.
        if self.held_by_project_operation(&app.state.platform) {
            return;
        }
        if self.lacks_open_project(&app.state.platform) {
            return;
        }
        if let Some((request, accepted)) = self.source_editor_redirect(&app.state.workbench) {
            if accepted {
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

    /// Whether a project operation still in progress holds this command
    /// back. Availability asks the same question, so a command the
    /// dispatcher would refuse is drawn disabled.
    pub(crate) fn held_by_project_operation(self, platform: &PlatformState) -> bool {
        crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform)
            && self.blocked_by_project_operation()
    }

    /// Whether this command needs an open project and none is open.
    pub(crate) fn lacks_open_project(self, platform: &PlatformState) -> bool {
        self.requires_open_project() && !platform.project_lifecycle.is_open()
    }

    /// The request the active source editor takes in this command's place,
    /// with whether it accepts it: a read-only editor refuses a request that
    /// would change its text. `None` when no source editor takes the command.
    pub(crate) fn source_editor_redirect(
        self,
        workbench: &WorkbenchSession,
    ) -> Option<(SourceEditorRequest, bool)> {
        let request = self.source_editor_request()?;
        let editor = workbench.active_source_editor?;
        Some((request, editor.editable || !request.mutates_source()))
    }
}
