//! The command catalogue as rows: everything a registered command declares
//! about itself, and what it answers in three canonical application states.
//!
//! A replacement for the closed `Command` enum must reproduce these rows
//! exactly — identity, label, group, every flag, shortcut context, default
//! bindings, registry order, availability and check state. The comparison is
//! a live equality between two builders, never a recorded file, so the rows
//! stay true to whatever the catalogue is today.

use std::collections::{BTreeSet, HashSet};
use std::time::Duration;

use super::*;
use crate::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutBinding, ShortcutKind,
};
use crate::state::shortcuts::{
    ShortcutBindingSlot, ShortcutPreferences, ShortcutSequence, ShortcutStroke,
    shortcut_context_precedence_rank,
};
use crate::workbench::app_state::session::shortcuts::ShortcutContextMatch;
use crate::workbench::app_state::session::shortcuts::{
    ShortcutEnvironment, ShortcutInputSnapshot, ShortcutResolverState,
};
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::commands::vocabulary::{
    COMMAND_REGISTRY, CommandRegistry, command_registry, legacy_command,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandSpec, ids};
use crate::workbench::framework::host::Host;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;

/// The retired ids `from_stable_id` still answers, each with the live id it
/// resolves to. Shortcut profiles written by older builds name them.
pub(crate) const LEGACY_ALIASES: &[(&str, &str)] = &[
    ("model-metadata-audit", "model-qualification"),
    ("fit-canvas", "canvas-fit-sheet"),
    ("export-waveforms", "export-results"),
    ("cursor-manager", "toggle-linked-cursors"),
    ("visualization-cursor-manager", "toggle-linked-cursors"),
    ("visualization-document-properties", "plot-properties"),
    ("run-checks", "check-cellview"),
];

/// A command's answer to [`Command::availability`], in a form that does not
/// name the type that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AvailabilityRow {
    Available,
    Disabled(&'static str),
    Hidden,
}

impl From<CommandAvailability> for AvailabilityRow {
    fn from(availability: CommandAvailability) -> Self {
        match availability {
            CommandAvailability::Available => Self::Available,
            CommandAvailability::Disabled(reason) => Self::Disabled(reason),
            CommandAvailability::Hidden => Self::Hidden,
        }
    }
}

/// What one command answers in one application state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StateRow {
    pub(crate) availability: AvailabilityRow,
    pub(crate) enabled: bool,
    pub(crate) checked: Option<bool>,
    /// Whether the command's shortcut context is live, as menus and the
    /// palette ask it.
    pub(crate) context_live: bool,
}

/// Everything one registered command declares, and what it answers in the
/// three canonical states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandRow {
    /// Position in the registry. The shortcut resolver breaks ties on it.
    pub(crate) index: usize,
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) group: &'static str,
    pub(crate) requires_open_project: bool,
    pub(crate) blocked_by_project_operation: bool,
    pub(crate) mutates_window_session: bool,
    pub(crate) palette_visible: bool,
    pub(crate) source_editor_only: bool,
    pub(crate) navigates_source_language: bool,
    pub(crate) repeatable: bool,
    pub(crate) commits_inspector_edit: bool,
    /// Refused while the active schematic is read-only.
    pub(crate) edits_active_document: bool,
    /// What the command asks of a source editor on screen instead of running.
    pub(crate) source_editor_request: Option<String>,
    pub(crate) context: &'static str,
    pub(crate) context_suppressed_by_text_focus: bool,
    /// Factory bindings in declaration order.
    pub(crate) bindings: Vec<String>,
    /// Platforms whose host reserves the primary chord.
    pub(crate) primary_reserved_on: Vec<&'static str>,
    pub(crate) no_project: StateRow,
    pub(crate) project_open: StateRow,
    pub(crate) results_retained: StateRow,
}

/// One row per command the registry orders, in order: a shell command read
/// from its [`Command`] value, a feature's from what its command set
/// declares.
pub(crate) fn legacy_command_rows() -> Vec<CommandRow> {
    let no_project = no_project_app();
    let project_open = project_open_app();
    let results_retained = results_retained_app();
    let apps = [&no_project, &project_open, &results_retained];
    command_registry()
        .order()
        .iter()
        .copied()
        .enumerate()
        .map(|(index, key)| {
            let Some(command) = legacy_command(key) else {
                let spec = SheetsCommand::ALL
                    .into_iter()
                    .find(|command| command.key() == key)
                    .map(SheetsCommand::declared_spec)
                    .or_else(|| {
                        SettingsCommand::ALL
                            .into_iter()
                            .find(|command| command.key() == key)
                            .map(SettingsCommand::declared_spec)
                    })
                    .or_else(|| HardcopyCommand::from_key(key).map(HardcopyCommand::declared_spec))
                    .or_else(|| ResultsCommand::from_key(key).map(ResultsCommand::declared_spec))
                    .or_else(|| {
                        CloudCommand::ALL
                            .into_iter()
                            .find(|command| command.key() == key)
                            .map(CloudCommand::declared_spec)
                    })
                    .or_else(|| ModelsCommand::from_key(key).map(ModelsCommand::declared_spec))
                    .unwrap_or_else(|| panic!("{key:?} is ordered but nothing declares it"));
                return feature_row(index, &spec, apps);
            };
            let spec = command.spec();
            let context = command.shortcut_context();
            CommandRow {
                index,
                id: command.stable_id(),
                label: spec.label,
                group: spec.group,
                requires_open_project: command.requires_open_project(),
                blocked_by_project_operation: command.blocked_by_project_operation(),
                mutates_window_session: command.mutates_window_session(),
                palette_visible: command.palette_visible(),
                source_editor_only: command.source_editor_only(),
                navigates_source_language: command.navigates_source_language(),
                repeatable: command.repeatable(),
                commits_inspector_edit: command.commits_inspector_edit(),
                edits_active_document: command_edits_schematic(command),
                source_editor_request: command
                    .source_editor_request()
                    .map(|request| format!("{request:?}")),
                context: context.label(),
                context_suppressed_by_text_focus: context.suppressed_by_text_focus(),
                bindings: command
                    .shortcut_bindings()
                    .iter()
                    .map(binding_row)
                    .collect(),
                primary_reserved_on: CommandPlatform::ALL
                    .into_iter()
                    .filter(|platform| command.primary_is_reserved_on(*platform))
                    .map(CommandPlatform::label)
                    .collect(),
                no_project: state_row(command, &no_project),
                project_open: state_row(command, &project_open),
                results_retained: state_row(command, &results_retained),
            }
        })
        .collect()
}

/// One row per command the registry orders, read through the registry: the
/// order, the id, the label, the group, the palette listing, the shortcut
/// context, the bindings and the reserved chords come from its specs and its
/// catalog, and the rest from the command its key resolves to.
// Refactor scaffolding: deleted when the split completes.
fn registry_command_rows() -> Vec<CommandRow> {
    let no_project = no_project_app();
    let project_open = project_open_app();
    let results_retained = results_retained_app();
    let apps = [&no_project, &project_open, &results_retained];
    let registry = command_registry();
    let catalog = CommandRegistry;
    catalog
        .commands()
        .iter()
        .copied()
        .enumerate()
        .map(|(index, key)| {
            let id = key.as_str();
            let spec = registry
                .spec(key)
                .unwrap_or_else(|| panic!("{id} is ordered but has no spec"));
            let Some(command) = Command::from_stable_id(id) else {
                return feature_row(index, spec, apps);
            };
            let context = catalog
                .context(key)
                .unwrap_or_else(|| panic!("{id} has no shortcut context"));
            let state = |app: &RSpiceApp| StateRow {
                context_live: context.matches(&app.state.workbench, &app.state.model),
                ..state_row(command, app)
            };
            CommandRow {
                index,
                id: spec.key.as_str(),
                label: spec.label,
                group: spec.group,
                requires_open_project: command.requires_open_project(),
                blocked_by_project_operation: command.blocked_by_project_operation(),
                mutates_window_session: command.mutates_window_session(),
                palette_visible: spec.flags.contains(CommandFlags::PALETTE_VISIBLE),
                source_editor_only: command.source_editor_only(),
                navigates_source_language: command.navigates_source_language(),
                repeatable: command.repeatable(),
                commits_inspector_edit: command.commits_inspector_edit(),
                edits_active_document: command_edits_schematic(command),
                source_editor_request: command
                    .source_editor_request()
                    .map(|request| format!("{request:?}")),
                context: context.label(),
                context_suppressed_by_text_focus: context.suppressed_by_text_focus(),
                bindings: catalog
                    .default_bindings(key)
                    .iter()
                    .map(binding_row)
                    .collect(),
                primary_reserved_on: CommandPlatform::ALL
                    .into_iter()
                    .filter(|platform| catalog.primary_is_reserved_on(key, *platform))
                    .map(CommandPlatform::label)
                    .collect(),
                no_project: state(&no_project),
                project_open: state(&project_open),
                results_retained: state(&results_retained),
            }
        })
        .collect()
}

/// The row of a command a feature declares as `spec`, its answers read from
/// the dispatcher by key in each of `apps`. A feature's command declares no
/// window, source-editor or repeat facts.
fn feature_row(index: usize, spec: &CommandSpec, apps: [&RSpiceApp; 3]) -> CommandRow {
    let state = |app: &RSpiceApp| StateRow {
        availability: app.command_availability(spec.key).into(),
        enabled: app.command_key_enabled(spec.key),
        checked: Host::checked(app, spec.key),
        context_live: spec.context.matches(&app.state.workbench, &app.state.model),
    };
    CommandRow {
        index,
        id: spec.key.as_str(),
        label: spec.label,
        group: spec.group,
        requires_open_project: spec.flags.contains(CommandFlags::REQUIRES_OPEN_PROJECT),
        blocked_by_project_operation: spec
            .flags
            .contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION),
        mutates_window_session: false,
        palette_visible: spec.flags.contains(CommandFlags::PALETTE_VISIBLE),
        source_editor_only: false,
        navigates_source_language: false,
        repeatable: false,
        commits_inspector_edit: spec.flags.contains(CommandFlags::COMMITS_INSPECTOR_EDIT),
        edits_active_document: spec.flags.contains(CommandFlags::EDITS_ACTIVE_DOCUMENT),
        source_editor_request: None,
        context: spec.context.label(),
        context_suppressed_by_text_focus: spec.context.suppressed_by_text_focus(),
        bindings: spec.bindings.iter().map(binding_row).collect(),
        primary_reserved_on: CommandPlatform::ALL
            .into_iter()
            .filter(|platform| {
                platform.has_browser_reserved_primary()
                    && spec
                        .flags
                        .contains(CommandFlags::PRIMARY_RESERVED_BY_BROWSER)
            })
            .map(CommandPlatform::label)
            .collect(),
        no_project: state(apps[0]),
        project_open: state(apps[1]),
        results_retained: state(apps[2]),
    }
}

fn state_row(command: Command, app: &RSpiceApp) -> StateRow {
    StateRow {
        availability: command.availability(app).into(),
        enabled: command.is_enabled(app),
        checked: command.checked(app),
        context_live: command
            .shortcut_context()
            .matches(&app.state.workbench, &app.state.model),
    }
}

/// `primary O+primary+shift "Ctrl+Shift+O" on Desktop,Browser,Tablet,Phone`.
fn binding_row(binding: &ShortcutBinding) -> String {
    let kind = match binding.kind {
        ShortcutKind::Primary => "primary",
        ShortcutKind::Alternate => "alternate",
    };
    let chord = binding.chord;
    let mut keys = format!("{:?}", chord.key);
    for (held, modifier) in [
        (chord.primary, "primary"),
        (chord.alt, "alt"),
        (chord.shift, "shift"),
    ] {
        if held {
            keys.push('+');
            keys.push_str(modifier);
        }
    }
    let platforms = binding
        .platforms
        .iter()
        .map(|platform| platform.label())
        .collect::<Vec<_>>()
        .join(",");
    format!("{kind} {keys} {:?} on {platforms}", chord.label)
}

/// No project open: the landing the product's own Close Project leaves.
fn no_project_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(
        crate::workbench::workflows::project_workflow::close_project_discard(
            &mut app.state,
            crate::workbench::lifecycle::project_lifecycle::ProjectCloseDestination::EmptyWorkbench,
        ),
        "an idle test instance closes its project"
    );
    assert!(!app.state.platform.project_lifecycle.is_open());
    app
}

/// A project open in the Design workspace on its bootstrapped schematic.
fn project_open_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app
}

/// A project with one retained run of the active plan, selected and shown in
/// the Results workspace.
fn results_retained_app() -> RSpiceApp {
    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    app.state.workbench.activate(Workspace::Results);
    assert!(app.state.model.simulation.retained.has_results());
    app
}

/// The alias literals the command registry tests before its key lookup:
/// every string literal in the alias table, whose targets are commands.
fn aliases_answered_by_from_stable_id() -> BTreeSet<&'static str> {
    const VOCABULARY: &str = include_str!("../vocabulary.rs");
    let body = VOCABULARY
        .split_once("const LEGACY_ALIASES: &[(&str, CommandKey)] = &[")
        .and_then(|(_, rest)| rest.split_once("\n];"))
        .map(|(body, _)| body)
        .expect("the registry's aliases are one table in vocabulary.rs");
    body.split('"').skip(1).step_by(2).collect()
}

#[test]
fn legacy_rows_cover_the_registry_in_order() {
    let rows = legacy_command_rows();
    let order = command_registry().order();
    assert_eq!(rows.len(), order.len());
    for (index, (row, key)) in rows.iter().zip(order).enumerate() {
        assert_eq!(row.index, index);
        assert_eq!(row.id, key.as_str(), "row {index} is {key:?}");
    }

    // The three states are three different answers, not one state three times.
    let differing = |state: fn(&CommandRow) -> &StateRow, other: fn(&CommandRow) -> &StateRow| {
        rows.iter().filter(|row| state(row) != other(row)).count()
    };
    assert!(differing(|row| &row.no_project, |row| &row.project_open) > 0);
    assert!(differing(|row| &row.project_open, |row| &row.results_retained) > 0);
}

#[test]
fn legacy_row_ids_are_unique_and_resolve_to_their_command() {
    let mut ids = HashSet::new();
    for command in COMMAND_REGISTRY.iter().copied() {
        let id = command.stable_id();
        assert!(ids.insert(id), "duplicate command id {id}");
        assert_eq!(Command::from_stable_id(id), Some(command), "{id}");
    }
}

#[test]
fn every_legacy_alias_resolves_to_a_registered_command_and_shadows_no_live_id() {
    let registry = command_registry();
    let live: HashSet<&str> = registry.order().iter().map(|key| key.as_str()).collect();
    for &(alias, target) in LEGACY_ALIASES {
        assert!(!live.contains(alias), "alias {alias} is also a live id");
        let resolved = registry
            .resolve(alias)
            .unwrap_or_else(|| panic!("alias {alias} resolves to nothing"));
        assert!(
            registry.order().contains(&resolved),
            "alias {alias} resolves to an unregistered {resolved:?}"
        );
        assert_eq!(resolved.as_str(), target, "alias {alias}");
    }

    let recorded: BTreeSet<&str> = LEGACY_ALIASES.iter().map(|(alias, _)| *alias).collect();
    assert_eq!(
        recorded.len(),
        LEGACY_ALIASES.len(),
        "an alias is listed twice"
    );
    assert_eq!(
        aliases_answered_by_from_stable_id(),
        recorded,
        "LEGACY_ALIASES must list every alias from_stable_id answers"
    );
}

#[test]
fn legacy_rows_are_deterministic() {
    assert_eq!(legacy_command_rows(), legacy_command_rows());
}

// Refactor scaffolding: deleted when the split completes.
#[test]
fn registry_rows_equal_legacy_rows() {
    let registry = registry_command_rows();
    let legacy = legacy_command_rows();
    assert_eq!(registry.len(), legacy.len());
    for (registry, legacy) in registry.iter().zip(&legacy) {
        assert_eq!(registry, legacy, "row {} ({})", legacy.index, legacy.id);
    }
}

#[test]
fn the_command_registry_validates() {
    assert_eq!(command_registry().validate(ids::named()), Ok(()));
}

/// A retired id is answered with the key of the command it names now, and
/// is never itself a key the registry declares.
#[test]
fn the_registry_answers_every_legacy_alias_with_a_live_key() {
    let registry = command_registry();
    for &(alias, target) in LEGACY_ALIASES {
        assert!(
            registry.spec(CommandKey::new(alias)).is_none(),
            "alias {alias} is also a key"
        );
        let resolved = registry
            .resolve(alias)
            .unwrap_or_else(|| panic!("alias {alias} resolves to nothing"));
        assert_eq!(resolved.as_str(), target, "alias {alias}");
        assert!(registry.spec(resolved).is_some(), "alias {alias}");
    }
}

/// The shortcut resolver breaks equal-precedence ties by position in the
/// registry's order, which lists the shell's own commands in
/// [`COMMAND_REGISTRY`]'s order, so ties among them break as they always did.
// Refactor scaffolding: deleted when the split completes.
#[test]
fn the_registry_order_breaks_ties_as_the_command_registry_does() {
    let order = command_registry().order();
    let shell_order = order
        .iter()
        .copied()
        .filter(|key| legacy_command(*key).is_some())
        .collect::<Vec<_>>();
    assert_eq!(shell_order.len(), COMMAND_REGISTRY.len());
    for (index, (key, command)) in shell_order.iter().zip(COMMAND_REGISTRY).enumerate() {
        assert_eq!(*key, command.key(), "position {index} is {command:?}");
    }
    let position = |command: Command| {
        order
            .iter()
            .position(|key| *key == command.key())
            .unwrap_or_else(|| panic!("{command:?} is ordered"))
    };
    for (earlier, later) in [
        (Command::Duplicate, Command::DuplicateLine),
        (Command::PlaceWire, Command::ToggleLineComment),
    ] {
        assert!(position(earlier) < position(later), "{earlier:?}");
    }
}

/// Two candidates of equal context precedence answering one chord are
/// ordered by their position in [`COMMAND_REGISTRY`], so a replacement
/// registry has to keep that order. The environment makes an editor-tier
/// canvas context and the source editor live at once; the profile audit sees
/// their view families as disjoint and accepts the shared chord.
#[test]
fn the_shortcut_resolver_breaks_equal_precedence_ties_by_registry_index() {
    let environment = ShortcutEnvironment {
        workspace: Workspace::Design,
        active_view: crate::state::ViewType::Schematic,
        canvas_focus: false,
        source_editor_focus: true,
    };
    let chord = ShortcutStroke::new(egui::Key::F12, true, true, true);
    let press = egui::Event::Key {
        key: egui::Key::F12,
        physical_key: Some(egui::Key::F12),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            command: true,
            ctrl: true,
            alt: true,
            shift: true,
            ..egui::Modifiers::NONE
        },
    };
    let registry_index = |command: Command| {
        COMMAND_REGISTRY
            .iter()
            .position(|registered| *registered == command)
            .unwrap_or_else(|| panic!("{command:?} is registered"))
    };

    for (earlier, later) in [
        (Command::Duplicate, Command::DuplicateLine),
        (Command::PlaceWire, Command::ToggleLineComment),
    ] {
        assert!(registry_index(earlier) < registry_index(later));
        for bound in [[earlier, later], [later, earlier]] {
            let mut profile = ShortcutPreferences::default();
            for command in bound {
                profile
                    .set_binding(
                        &vocabulary::CommandRegistry,
                        command.key(),
                        ShortcutBindingSlot::Primary,
                        vec![CommandPlatform::Desktop],
                        Some(ShortcutSequence::single(chord)),
                    )
                    .expect("binding accepted");
            }
            assert!(
                profile.is_effective_profile_valid(&vocabulary::CommandRegistry),
                "{bound:?}"
            );
            let precedence = profile.policies().context_precedence();
            assert_eq!(
                shortcut_context_precedence_rank(earlier.shortcut_context(), precedence),
                shortcut_context_precedence_rank(later.shortcut_context(), precedence),
            );

            let resolve = |available: &dyn Fn(CommandKey) -> bool| {
                ShortcutResolverState::default()
                    .resolve(
                        &ShortcutInputSnapshot::from_events_for_test(
                            std::slice::from_ref(&press),
                            false,
                        ),
                        &profile,
                        &CommandRegistry,
                        CommandPlatform::Desktop,
                        environment,
                        Duration::ZERO,
                        available,
                    )
                    .command
            };
            assert_eq!(resolve(&|_| true), Some(earlier.key()), "{bound:?}");
            // The later command was a live candidate the tie-break passed over.
            assert_eq!(
                resolve(&|command| command != earlier.key()),
                Some(later.key()),
                "{bound:?}"
            );
        }
    }
}
