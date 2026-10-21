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

use super::result_navigation::{RunOrigin, app_with_prepared_run};
use super::*;
use crate::workbench::app_state::session::shortcuts::{
    ShortcutEnvironment, ShortcutInputSnapshot, ShortcutResolverState,
};
use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, CommandPlatform};
use crate::workbench::{
    ShortcutBindingSlot, ShortcutPreferences, ShortcutSequence, ShortcutStroke,
    shortcut_context_precedence_rank,
};

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

/// One row per [`COMMAND_REGISTRY`] entry, in registry order.
pub(crate) fn legacy_command_rows() -> Vec<CommandRow> {
    let no_project = no_project_app();
    let project_open = project_open_app();
    let results_retained = results_retained_app();
    COMMAND_REGISTRY
        .iter()
        .copied()
        .enumerate()
        .map(|(index, command)| {
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

fn state_row(command: Command, app: &RSpiceApp) -> StateRow {
    StateRow {
        availability: command.availability(app).into(),
        enabled: command.is_enabled(app),
        checked: command.checked(app),
        context_live: command.shortcut_context().matches(app),
    }
}

/// `primary O+primary+shift "Ctrl+Shift+O" on Desktop,Browser,Tablet,Phone`.
fn binding_row(binding: &crate::workbench::commands::registry::ShortcutBinding) -> String {
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
        crate::workbench::workflows::project_workflow::close_project_discard(&mut app.state),
        "an idle test instance closes its project"
    );
    assert!(!app.state.project_lifecycle.is_open());
    app
}

/// A project open in the Design workspace on its bootstrapped schematic.
fn project_open_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app
}

/// A project with one retained run of the active plan, selected and shown in
/// the Results workspace.
fn results_retained_app() -> RSpiceApp {
    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    app.state.workbench.activate(Workspace::Results);
    assert!(app.state.simulation.retained.has_results());
    app
}

/// The alias literals `from_stable_id` tests before its registry lookup.
fn aliases_answered_by_from_stable_id() -> BTreeSet<&'static str> {
    const VOCABULARY: &str = include_str!("../vocabulary.rs");
    let body = VOCABULARY
        .split_once("pub fn from_stable_id(")
        .and_then(|(_, rest)| rest.split_once("COMMAND_REGISTRY"))
        .map(|(body, _)| body)
        .expect("from_stable_id checks its aliases before it searches the registry");
    body.split("id == \"")
        .skip(1)
        .map(|rest| {
            rest.split_once('"')
                .expect("an alias is a closed string literal")
                .0
        })
        .collect()
}

#[test]
fn legacy_rows_cover_the_registry_in_order() {
    let rows = legacy_command_rows();
    assert_eq!(rows.len(), COMMAND_REGISTRY.len());
    for (index, (row, command)) in rows.iter().zip(COMMAND_REGISTRY).enumerate() {
        assert_eq!(row.index, index);
        assert_eq!(row.id, command.stable_id(), "row {index} is {command:?}");
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
    let live: HashSet<&str> = COMMAND_REGISTRY
        .iter()
        .map(|command| command.stable_id())
        .collect();
    for &(alias, target) in LEGACY_ALIASES {
        assert!(!live.contains(alias), "alias {alias} is also a live id");
        let resolved = Command::from_stable_id(alias)
            .unwrap_or_else(|| panic!("alias {alias} resolves to nothing"));
        assert!(
            COMMAND_REGISTRY.contains(&resolved),
            "alias {alias} resolves to an unregistered {resolved:?}"
        );
        assert_eq!(resolved.stable_id(), target, "alias {alias}");
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
                        command,
                        ShortcutBindingSlot::Primary,
                        vec![CommandPlatform::Desktop],
                        Some(ShortcutSequence::single(chord)),
                    )
                    .expect("binding accepted");
            }
            assert!(profile.is_effective_profile_valid(), "{bound:?}");
            let precedence = profile.policies().context_precedence();
            assert_eq!(
                shortcut_context_precedence_rank(earlier.shortcut_context(), precedence),
                shortcut_context_precedence_rank(later.shortcut_context(), precedence),
            );

            let resolve = |available: &dyn Fn(Command) -> bool| {
                ShortcutResolverState::default()
                    .resolve(
                        &ShortcutInputSnapshot::from_events_for_test(
                            std::slice::from_ref(&press),
                            false,
                        ),
                        &profile,
                        CommandPlatform::Desktop,
                        environment,
                        Duration::ZERO,
                        available,
                    )
                    .command
            };
            assert_eq!(resolve(&|_| true), Some(earlier), "{bound:?}");
            // The later command was a live candidate the tie-break passed over.
            assert_eq!(
                resolve(&|command| command != earlier),
                Some(later),
                "{bound:?}"
            );
        }
    }
}
