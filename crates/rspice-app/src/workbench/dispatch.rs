//! The one dispatcher every command goes through, whoever raises it: a menu
//! row, a toolbar button, the palette, a shortcut or another command.
//!
//! A command runs behind one prologue, always in the same order: a project
//! operation in flight holds it back, a command that needs a project needs
//! one open, an active source editor takes the editing commands it handles,
//! a read-only schematic refuses an edit, a pending inspector edit commits
//! first, and a repeatable command is recorded for Repeat. Only then does the
//! command's owner carry it out: the feature that declares it, in its
//! context, or for a key no feature declares the shell's legacy table
//! [`Command::run_legacy_arm`]. A command a feature's command raised runs as
//! soon as that one returns, behind the same prologue.
//!
//! The dispatcher is handed a command value or a key. A key names a command
//! when the registry lists it, or when a feature declares it and leaves it
//! out of the registry for the one container that holds its key; no menu,
//! shortcut profile or Repeat reaches one of those. Placing a kind the
//! registry does not list answers the shared id `place-component`, which
//! names no command, so Repeat after it does nothing. Placing a port answers
//! the pin command's id and runs that command.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app::HardcopyWorkflow;
use crate::workbench::app_state::PlatformState;
#[cfg(test)]
use crate::workbench::commands::dispatch_probe;
use crate::workbench::commands::vocabulary::{Command, legacy_command};
use crate::workbench::commands::{active_schematic_editor, command_edits_schematic, file_action};
use crate::workbench::documents::text_editor_commands::SourceEditorRequest;
use crate::workbench::features;
use crate::workbench::framework::command_registry::{
    CommandAvailability, CommandFlags, CommandSpec,
};
use crate::workbench::framework::feature::Execute;
use crate::workbench::framework::requests::{
    self, ApplyWaveformDefaults, ExportDataset, ExportPublicationSnapshot, ExportSchematicSvg,
    FollowSchematicGrid, PrintActiveView, RevealExecutedDeck, SelectAnalysisInstance,
    SetFamilyFilter, ShowWaveforms,
};
use crate::workbench::hardcopy;
use crate::workbench::menu_bar::FileMenuAction;
use crate::workbench::{RSpiceApp, WorkbenchSession};

/// The dispatcher by key: where a key becomes the command it names, a
/// feature's first and the shell's legacy table's otherwise.
impl RSpiceApp {
    /// Carry out the command `key` names behind the prologue every command
    /// shares. A key no command answers to does nothing.
    pub(crate) fn execute_command_key(&mut self, key: CommandKey) {
        if let Some(spec) = features::spec(key) {
            self.dispatch_feature_command(spec);
        } else if let Some(command) = legacy_command(key) {
            command.dispatch(self);
        }
    }

    /// Whether the command `key` names can run here, and why not: the
    /// shell's own guards first, then the command's owner. `Hidden` for a key
    /// no command answers to.
    pub(crate) fn command_availability(&self, key: CommandKey) -> CommandAvailability {
        let Some(spec) = features::spec(key) else {
            return legacy_command(key).map_or(CommandAvailability::Hidden, |command| {
                command.availability(self)
            });
        };
        if held_by_project_operation(spec.flags, &self.state.platform) {
            return CommandAvailability::Disabled("project operation is still in progress");
        }
        if lacks_open_project(spec.flags, &self.state.platform) {
            return CommandAvailability::Disabled("no project is open");
        }
        features::availability(&self.state, key).unwrap_or(CommandAvailability::Hidden)
    }

    /// Whether the command `key` names is available here, as the shortcut
    /// resolver asks it before a chord may fire.
    pub(crate) fn command_key_available(&self, key: CommandKey) -> bool {
        self.command_availability(key).is_available()
    }

    /// Whether the command `key` names would run if raised now. A feature's
    /// command runs exactly when it is available: no source editor takes
    /// one in its place.
    pub(crate) fn command_key_enabled(&self, key: CommandKey) -> bool {
        if features::spec(key).is_some() {
            return self.command_availability(key).is_available();
        }
        legacy_command(key).is_some_and(|command| command.is_enabled(self))
    }

    /// Carry out the routed command `key` in the feature that takes it where
    /// the reader is working, and say whether one did. The shell's own arm
    /// for the command asks it where that feature's turn comes.
    pub(crate) fn route_command_key(&mut self, key: CommandKey) -> bool {
        features::route(self, key)
    }

    /// Whether the routed command `key` can run in the feature that takes it
    /// where the reader is working; `None` when no feature takes it there.
    pub(crate) fn routed_command_key_enabled(&self, key: CommandKey) -> Option<bool> {
        features::routed_enabled(&self.state, key)
    }

    /// Carry out the feature command `spec` declares behind the same
    /// prologue as [`Command::dispatch`], then every command it raised.
    fn dispatch_feature_command(&mut self, spec: &CommandSpec) {
        let flags = spec.flags;
        if held_by_project_operation(flags, &self.state.platform)
            || lacks_open_project(flags, &self.state.platform)
        {
            return;
        }
        if flags.contains(CommandFlags::EDITS_ACTIVE_DOCUMENT)
            && active_schematic_editor(self)
            && self.state.schematic_edit_read_only()
        {
            self.state.deny_read_only_edit();
            return;
        }
        if flags.contains(CommandFlags::COMMITS_INSPECTOR_EDIT)
            && !self.state.commit_pending_inspector_edit()
        {
            return;
        }
        features::execute(self, spec.key);
        self.after_contribution();
    }

    /// Open the hardcopy feature's `workflow` on the active view, in its
    /// context, for a request another part raised: a result document's
    /// figure export, or the print another feature offers as its dialog's
    /// next step.
    pub(in crate::workbench) fn open_requested_hardcopy_workflow(
        &mut self,
        workflow: HardcopyWorkflow,
    ) {
        hardcopy::open_workflow(&mut features::cx(self), workflow);
    }

    /// Answer what a feature's call raised for straight after it: the
    /// commands it ran next, the print workflow it asked for, the schematic
    /// or publication snapshot it exported, the snap that follows a new
    /// schematic grid, the shortcut resolver's reset when a shortcut library
    /// was put in use, the open waveform view starting again from changed
    /// defaults, the dataset export, the analysis instance and the executed
    /// deck a results command asked for, and the waveforms and the family
    /// member a hand-off into Results opens its viewer or form on.
    pub(crate) fn after_contribution(&mut self) {
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, Execute(key)| app.execute_command_key(key),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, PrintActiveView| {
                app.open_requested_hardcopy_workflow(HardcopyWorkflow::Print);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ExportSchematicSvg| file_action(app, FileMenuAction::ExportSvg),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ExportPublicationSnapshot| {
                file_action(app, FileMenuAction::ExportPublicationSnapshot);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, FollowSchematicGrid| {
                let state = &mut app.state;
                state.design.follow_schematic_grid(&state.model);
            },
        );
        self.state.take_shortcut_resolver_reset();
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ApplyWaveformDefaults| {
                let state = &mut app.state;
                state
                    .results
                    .start_waveform_viewer_session(&state.model.preferences);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ExportDataset| file_action(app, FileMenuAction::ExportCsvWaveforms),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, SelectAnalysisInstance(instance)| {
                app.state.simulate.active_analysis_instance = instance;
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, RevealExecutedDeck(sequence)| {
                crate::workbench::commands::result_navigation::open_task_deck(
                    &mut app.state,
                    sequence,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             ShowWaveforms {
                 visibility,
                 selected,
             }| {
                let results = &mut app.state.results.results;
                results.reveal_waveforms(visibility);
                results.session.selected_trace = selected;
                results.session.clear_cursors();
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, SetFamilyFilter(filter)| {
                crate::workbench::documents::result_document::set_family_filter(
                    &mut app.state,
                    filter,
                );
            },
        );
    }
}

/// Whether a project operation still in progress holds back a command that
/// declares `flags`.
fn held_by_project_operation(flags: CommandFlags, platform: &PlatformState) -> bool {
    flags.contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION)
        && crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform)
}

/// Whether a command that declares `flags` needs an open project and none is
/// open.
fn lacks_open_project(flags: CommandFlags, platform: &PlatformState) -> bool {
    flags.contains(CommandFlags::REQUIRES_OPEN_PROJECT) && !platform.project_lifecycle.is_open()
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

    /// A command the registry leaves out is a feature's, declared in its
    /// command set; the legacy enum lists every command it declares.
    #[test]
    fn the_registry_lists_every_command_the_enum_declares() {
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
        assert_eq!(
            registered, declared,
            "a new command goes in COMMAND_REGISTRY"
        );
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
