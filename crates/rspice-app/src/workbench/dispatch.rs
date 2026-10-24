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
//!
//! The dispatcher is handed a command value or a key. A key names a command
//! only when the registry lists it, so a dispatch by key reaches exactly the
//! listed commands; the few the registry leaves out are raised only by a
//! route that holds the value. Placing a kind the registry does not list
//! answers the shared id `place-component`, which names no command, so Repeat
//! after it does nothing. Placing a port answers the pin command's id and
//! runs that command.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::PlatformState;
#[cfg(test)]
use crate::workbench::commands::dispatch_probe;
use crate::workbench::commands::vocabulary::{Command, legacy_command};
use crate::workbench::commands::{active_schematic_editor, command_edits_schematic};
use crate::workbench::documents::text_editor_commands::SourceEditorRequest;
use crate::workbench::{RSpiceApp, WorkbenchSession};

/// The dispatcher by key: where a key becomes the command it names.
impl RSpiceApp {
    /// Carry out the command `key` names behind the prologue every command
    /// shares. A key no command answers to does nothing.
    pub(crate) fn execute_command_key(&mut self, key: CommandKey) {
        if let Some(command) = legacy_command(key) {
            command.dispatch(self);
        }
    }

    /// Whether the command `key` names is available here, as the shortcut
    /// resolver asks it before a chord may fire.
    pub(crate) fn command_key_available(&self, key: CommandKey) -> bool {
        legacy_command(key).is_some_and(|command| command.availability(self).is_available())
    }

    /// Whether the command `key` names would run if raised now.
    pub(crate) fn command_key_enabled(&self, key: CommandKey) -> bool {
        legacy_command(key).is_some_and(|command| command.is_enabled(self))
    }
}

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::state::ComponentType;
    use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, Command};

    /// The commands the registry leaves out. No key names one, so a menu, a
    /// shortcut profile or Repeat never reaches it.
    const UNREGISTERED: &[Command] = &[
        Command::SaveReportDocument,
        Command::AddReportPage,
        Command::ReportPageProperties,
    ];

    /// The kinds whose placement the registry lists, each under its own id.
    const REGISTERED_PLACEMENTS: &[ComponentType] = &[
        ComponentType::Resistor,
        ComponentType::Capacitor,
        ComponentType::Inductor,
        ComponentType::Diode,
        ComponentType::Ground,
        ComponentType::VoltageSource,
        ComponentType::CurrentSource,
    ];

    fn variant(command: Command) -> String {
        let name = format!("{command:?}");
        name.split('(').next().unwrap_or_default().to_owned()
    }

    /// Every variant `enum Command` declares, read from its source, so a
    /// variant added there is seen here without being listed anywhere.
    fn declared_variants() -> BTreeSet<String> {
        const VOCABULARY: &str = include_str!("commands/vocabulary.rs");
        let body = VOCABULARY
            .split_once("pub enum Command {")
            .and_then(|(_, rest)| rest.split_once("\n}"))
            .map(|(body, _)| body)
            .expect("vocabulary.rs declares `pub enum Command`");
        body.lines()
            .filter_map(|line| line.strip_prefix("    "))
            .filter(|line| line.starts_with(|c: char| c.is_ascii_uppercase()))
            .filter_map(|line| line.split(|c: char| !c.is_ascii_alphanumeric()).next())
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn the_registry_lists_every_command_but_the_unregistered_ones() {
        let declared = declared_variants();
        assert!(
            declared.len() > 200,
            "`enum Command` parsed as only {} variants: the parse broke, not the enum",
            declared.len()
        );
        let registered: BTreeSet<String> = COMMAND_REGISTRY
            .iter()
            .map(|command| variant(*command))
            .collect();
        assert!(registered.is_subset(&declared), "{registered:?}");
        let unlisted: BTreeSet<String> = declared.difference(&registered).cloned().collect();
        let expected: BTreeSet<String> = UNREGISTERED
            .iter()
            .map(|command| variant(*command))
            .collect();
        assert_eq!(
            unlisted, expected,
            "a new command goes in COMMAND_REGISTRY; one only a value may name goes in UNREGISTERED"
        );
        for command in UNREGISTERED {
            assert_eq!(
                Command::from_stable_id(command.stable_id()),
                None,
                "{command:?}"
            );
        }
    }

    #[test]
    fn a_placement_the_registry_leaves_out_names_no_command_or_the_pin_command() {
        let registered: Vec<ComponentType> = COMMAND_REGISTRY
            .iter()
            .filter_map(|command| match command {
                Command::Place(kind) => Some(*kind),
                _ => None,
            })
            .collect();
        assert_eq!(registered, REGISTERED_PLACEMENTS);

        for kind in ComponentType::ALL {
            let place = Command::Place(kind);
            if REGISTERED_PLACEMENTS.contains(&kind) {
                assert_eq!(Command::from_stable_id(place.stable_id()), Some(place));
            } else if kind == ComponentType::Port {
                assert_eq!(place.stable_id(), Command::PlacePin.stable_id());
                assert_eq!(
                    Command::from_stable_id(place.stable_id()),
                    Some(Command::PlacePin)
                );
            } else {
                assert_eq!(place.stable_id(), "place-component", "{kind:?}");
            }
        }
        assert_eq!(Command::from_stable_id("place-component"), None);
    }
}
