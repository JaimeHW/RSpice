//! Rename one selected schematic object.
//!
//! Edit ▸ Rename captures one live object by durable ID, edits an isolated
//! name draft, and publishes exactly one guarded undo transaction. Components,
//! net labels, logical named nets and declared buses keep their IDs; bus
//! dependencies are rebased through the schematic's bus transaction rather
//! than by rewriting display text.
//!
//! The form asks for the new name and says how far it reaches when that is
//! more than the object itself. It does not state the internal ID of what it
//! is renaming, and it does not report what it preserved.

use egui::{Context, Id};

use crate::schematic::view::sheet_visibility::object_is_on_active_sheet;
use crate::state::{BusDeclaration, Component, NetLabel, SchematicState};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};

use crate::workbench::app::{RSpiceApp, RenameSelectionTarget};
use crate::workbench::app_state::AppState;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod transaction_tests;

const TITLE: &str = "Rename";
const PRIMARY: &str = "Rename";
const FIELD_ID: &str = "rename-selected-object-name";

#[derive(Debug, Clone)]
enum RenameCommit {
    Component {
        expected: Box<Component>,
        name: String,
    },
    NetLabel {
        expected: NetLabel,
        name: String,
    },
    NamedNet {
        expected: crate::workbench::app::NamedNetTarget,
        name: String,
    },
    Bus {
        expected: crate::state::Bus,
        declaration: BusDeclaration,
    },
}

#[derive(Debug, Clone)]
enum RenameValidation {
    Valid(Box<RenameCommit>),
    Unchanged,
    Invalid(String),
}

impl RenameValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) | Self::Unchanged => None,
        }
    }
}

/// Whether the active schematic currently owns exactly one live, renameable
/// object. This is the single availability predicate used by menus, the
/// command palette, and keyboard dispatch.
pub(crate) fn rename_selection_available(state: &AppState) -> bool {
    !state.schematic_edit_read_only() && selected_rename_target(state).is_some()
}

/// Capture the current selection into an isolated stable-identity draft.
pub(crate) fn open_selected_object_rename(state: &mut AppState) -> bool {
    if state.dialogs.rename_selection.open
        || state.dialogs.object_properties.open
        || state.tabbed_property_dialog.open
        || state.schematic_edit_read_only()
    {
        return false;
    }
    let Some(target) = selected_rename_target(state) else {
        return false;
    };
    state.dialogs.rename_selection.open(
        target,
        state.design_execution_epoch,
        state.active_schematic_epoch,
        state.schematic.topology_version(),
        state.workspace.content.active_view.display_path(),
    );
    true
}

fn selected_rename_target(state: &AppState) -> Option<RenameSelectionTarget> {
    let schematic = &state.schematic;
    if let Some(target) = crate::workbench::app::selected_named_net_target(state) {
        return Some(RenameSelectionTarget::NamedNet(target));
    }
    if let Some(id) = schematic.session.editor.selection.single_component() {
        return schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id && !component.kind.spice_prefix().is_empty())
            .cloned()
            .map(Box::new)
            .map(RenameSelectionTarget::Component);
    }
    if let Some(id) = schematic.session.editor.selection.single_net_label() {
        return schematic
            .document()
            .net_labels
            .iter()
            .find(|label| label.id == id)
            .cloned()
            .map(RenameSelectionTarget::NetLabel);
    }
    if let Some(id) = schematic.session.editor.selection.single_bus() {
        return schematic
            .document()
            .buses
            .iter()
            .find(|bus| bus.id == id && bus.declaration.is_some())
            .cloned()
            .map(RenameSelectionTarget::Bus);
    }
    None
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_rename_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_selection.open {
            return;
        }
        let Some(target) = self.state.dialogs.rename_selection.target.as_ref() else {
            self.state.dialogs.rename_selection.close();
            self.state
                .prompt("Rename closed: the object it was renaming is gone.");
            return;
        };

        let authority_error = rename_session_error(&self.state);
        let validation = authority_error.map_or_else(
            || {
                validate_draft(
                    &self.state.schematic,
                    target,
                    &self.state.dialogs.rename_selection.draft,
                    &|id| object_is_on_active_sheet(&self.state, id),
                )
            },
            RenameValidation::Invalid,
        );
        let object = object_context(target);
        let previous = target.current_name().to_owned();
        let scope = rename_scope(&self.state.schematic, target);
        let validation_message = self
            .state
            .dialogs
            .rename_selection
            .validation_error
            .as_deref()
            .or_else(|| validation.message())
            .map(str::to_owned);

        let mut edited = false;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(object.as_str()))
            .primary_enabled(validation.can_commit())
            .show(ctx, |rows| {
                let field = Id::new(FIELD_ID);
                let response = rows.text(
                    "New name",
                    field,
                    &mut self.state.dialogs.rename_selection.draft,
                    "A SPICE-safe name",
                );
                // A prior publication error belongs to an older candidate;
                // the live validation under this field now owns what is
                // typed.
                edited = response.changed();
                rows.field_line(
                    Some(field),
                    match (validation_message.as_deref(), scope.as_deref()) {
                        (Some(message), _) => FieldLine::Error(message),
                        (None, Some(scope)) => FieldLine::Derived(scope),
                        (None, None) => FieldLine::Blank,
                    },
                );
                Some(field)
            });
        if edited {
            self.state.dialogs.rename_selection.validation_error = None;
        }

        match choice {
            DialogChoice::Primary => {
                let Some(target) = self.state.dialogs.rename_selection.target.clone() else {
                    return;
                };
                let validation = rename_session_error(&self.state).map_or_else(
                    || {
                        validate_draft(
                            &self.state.schematic,
                            &target,
                            &self.state.dialogs.rename_selection.draft,
                            &|id| object_is_on_active_sheet(&self.state, id),
                        )
                    },
                    RenameValidation::Invalid,
                );
                match validation {
                    RenameValidation::Valid(commit) => {
                        match apply_commit(&mut self.state, *commit) {
                            // One line in the log, because a rename reaches
                            // saved outputs and configuration paths the
                            // canvas does not show.
                            Ok(true) => {
                                let renamed =
                                    self.state.dialogs.rename_selection.draft.trim().to_owned();
                                self.state
                                    .prompt(format!("Renamed {previous} to {renamed}."));
                                self.state.dialogs.rename_selection.close();
                            }
                            Ok(false) => {
                                self.state.dialogs.rename_selection.validation_error =
                                    Some("That is already its name.".to_owned());
                            }
                            Err(error) => {
                                self.state.dialogs.rename_selection.validation_error = Some(error);
                            }
                        }
                    }
                    RenameValidation::Invalid(message) => {
                        self.state.dialogs.rename_selection.validation_error = Some(message);
                    }
                    RenameValidation::Unchanged => {}
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.rename_selection.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

/// What the form is renaming, as its one context line: the name it has now
/// and what kind of thing it is. No internal ID — the reader cannot act on
/// one, and the object is the same object either way.
fn object_context(target: &RenameSelectionTarget) -> String {
    match target {
        RenameSelectionTarget::Component(component) => format!(
            "{} \u{00b7} {}",
            component.name,
            component.kind.display_name()
        ),
        RenameSelectionTarget::NetLabel(label) => format!("{} \u{00b7} net label", label.name),
        RenameSelectionTarget::NamedNet(net) => format!("{} \u{00b7} net", net.name),
        RenameSelectionTarget::Bus(bus) => format!(
            "{} \u{00b7} bus",
            bus.declaration
                .as_ref()
                .map_or_else(|| "undeclared".to_owned(), ToString::to_string)
        ),
    }
}

/// How far the rename reaches, for the two targets that carry other names
/// with them. A component and a single label rename only themselves, and a
/// form does not report that it is ordinary.
fn rename_scope(schematic: &SchematicState, target: &RenameSelectionTarget) -> Option<String> {
    match target {
        RenameSelectionTarget::Component(_) | RenameSelectionTarget::NetLabel(_) => None,
        RenameSelectionTarget::NamedNet(net) => {
            let labels = count(net.labels.len(), "label", "labels");
            let ports = count(net.ports.len(), "port", "ports");
            Some(format!("Renames {}.", clause(labels, ports)?))
        }
        RenameSelectionTarget::Bus(bus) => {
            let taps = schematic
                .document()
                .bus_taps
                .iter()
                .filter(|tap| tap.bus_id == bus.id)
                .count();
            Some(format!("Renames {} with it.", count(taps, "tap", "taps")?))
        }
    }
}

fn count(value: usize, one: &str, many: &str) -> Option<String> {
    match value {
        0 => None,
        1 => Some(format!("1 {one}")),
        value => Some(format!("{value} {many}")),
    }
}

fn clause(left: Option<String>, right: Option<String>) -> Option<String> {
    match (left, right) {
        (Some(left), Some(right)) => Some(format!("{left} and {right}")),
        (Some(only), None) | (None, Some(only)) => Some(only),
        (None, None) => None,
    }
}

fn rename_session_error(state: &AppState) -> Option<String> {
    let dialog = &state.dialogs.rename_selection;
    if state.schematic_edit_read_only() {
        return Some("The active schematic is read-only; no name can be changed.".to_owned());
    }
    if dialog.design_execution_epoch != state.design_execution_epoch {
        return Some(
            "The design document changed while Rename was open. Cancel and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.active_schematic_epoch != state.active_schematic_epoch {
        return Some(
            "The active schematic buffer changed while Rename was open. Cancel and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.topology_version != state.schematic.topology_version() {
        return Some(
            "Schematic connectivity changed while Rename was open. Cancel and reopen the current object."
                .to_owned(),
        );
    }
    (dialog.view_path != state.workspace.content.active_view.display_path()).then(|| {
        "The active cell/view changed while Rename was open. Cancel and reopen the current object."
            .to_owned()
    })
}

/// `on_sheet` admits the active sheet's objects, the only ones a bus rename
/// there can reach.
fn validate_draft(
    schematic: &SchematicState,
    target: &RenameSelectionTarget,
    draft: &str,
    on_sheet: &dyn Fn(u64) -> bool,
) -> RenameValidation {
    if schematic.session.read_only {
        return RenameValidation::Invalid(
            "The active schematic is read-only; no name can be changed.".to_owned(),
        );
    }
    let candidate = draft.trim();
    if candidate.is_empty() {
        return RenameValidation::Invalid("Enter a non-empty name.".to_owned());
    }
    match target {
        RenameSelectionTarget::Component(expected) => {
            let Some(current) = schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == expected.id)
            else {
                return stale("The selected component no longer exists.");
            };
            if current != expected.as_ref() {
                return stale("The selected component changed while Rename was open.");
            }
            if candidate == expected.name {
                return RenameValidation::Unchanged;
            }
            if let Err(error) = expected.validate_reference_designator(candidate) {
                return RenameValidation::Invalid(error);
            }
            if schematic.document().components.iter().any(|component| {
                component.id != expected.id && component.name.eq_ignore_ascii_case(candidate)
            }) {
                return RenameValidation::Invalid(format!(
                    "A component named `{candidate}` already exists; SPICE designators are case-insensitively unique."
                ));
            }
            RenameValidation::Valid(Box::new(RenameCommit::Component {
                expected: expected.clone(),
                name: candidate.to_owned(),
            }))
        }
        RenameSelectionTarget::NetLabel(expected) => {
            let Some(current) = schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.id == expected.id)
            else {
                return stale("The selected net label no longer exists.");
            };
            if current != expected {
                return stale("The selected net label changed while Rename was open.");
            }
            if candidate == expected.name {
                return RenameValidation::Unchanged;
            }
            if let Err(reason) = crate::state::NetLabel::validate_name(
                candidate,
                schematic.document().document_policy.net_naming,
            ) {
                return RenameValidation::Invalid(format!("Net name: {reason}."));
            }
            RenameValidation::Valid(Box::new(RenameCommit::NetLabel {
                expected: expected.clone(),
                name: candidate.to_owned(),
            }))
        }
        RenameSelectionTarget::NamedNet(expected) => {
            let candidate = match crate::workbench::app::validate_named_net_rename(
                schematic, expected, candidate,
            ) {
                Ok(candidate) => candidate,
                Err(message) => return RenameValidation::Invalid(message),
            };
            if candidate == expected.name {
                return RenameValidation::Unchanged;
            }
            RenameValidation::Valid(Box::new(RenameCommit::NamedNet {
                expected: expected.clone(),
                name: candidate,
            }))
        }
        RenameSelectionTarget::Bus(expected) => {
            let Some(current) = schematic
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == expected.id)
            else {
                return stale("The selected bus no longer exists.");
            };
            if current != expected {
                return stale("The selected bus changed while Rename was open.");
            }
            let Some(original) = expected.declaration.as_ref() else {
                return RenameValidation::Invalid(
                    "Only a declared bus owns a stable user-facing bus name.".to_owned(),
                );
            };
            if candidate == original.name {
                return RenameValidation::Unchanged;
            }
            let declaration =
                match BusDeclaration::new(candidate, original.msb, original.lsb, original.notation)
                {
                    Ok(declaration) => declaration,
                    Err(error) => {
                        return RenameValidation::Invalid(format!("Bus name: {error}."));
                    }
                };
            match schematic.validate_bus_properties(expected, Some(&declaration), on_sheet) {
                Ok(_) => RenameValidation::Valid(Box::new(RenameCommit::Bus {
                    expected: expected.clone(),
                    declaration,
                })),
                Err(error) => RenameValidation::Invalid(format!(
                    "The connected bus network cannot be renamed: {error}."
                )),
            }
        }
    }
}

fn stale(message: &str) -> RenameValidation {
    RenameValidation::Invalid(format!(
        "{message} Cancel and reopen Rename from the current object."
    ))
}

fn apply_commit(state: &mut AppState, commit: RenameCommit) -> Result<bool, String> {
    let content = &state.workspace.content;
    let catalog = content
        .design_management
        .sheet_catalog(&content.active_schematic_reference().key());
    let on_sheet =
        |id| rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id);
    let schematic = &mut state.schematic;
    match commit {
        RenameCommit::Component { expected, name } => {
            state.rename_component_transaction(&expected, name)
        }
        RenameCommit::NetLabel { expected, name } => schematic.rename_net_label(expected, name),
        RenameCommit::NamedNet { expected, name } => {
            crate::workbench::app::apply_named_net_rename(schematic, expected, name)
        }
        RenameCommit::Bus {
            expected,
            declaration,
        } => schematic
            .edit_bus_properties(&expected, Some(declaration), &on_sheet)
            .map_err(|error| format!("The bus rename was rejected: {error}.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Bus, ComponentType, NetLabel, Point, Wire};

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn component_app() -> (RSpiceApp, u64) {
        let mut app = RSpiceApp::test_instance();
        let id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        (app, id)
    }

    /// The form names what it is renaming and how far the rename reaches,
    /// and states no internal ID for either.
    #[test]
    fn the_context_line_names_the_object_without_its_internal_id() {
        let (app, _) = component_app();
        let target = selected_rename_target(&app.state).expect("one renameable object");
        let context = object_context(&target);
        assert_eq!(context, "R1 \u{00b7} Resistor");
        assert!(!context.contains("COMPONENT-"));
        assert_eq!(rename_scope(&app.state.schematic, &target), None);
    }

    #[test]
    fn availability_requires_one_live_renameable_object_and_write_access() {
        let (mut app, id) = component_app();
        assert!(rename_selection_available(&app.state));

        app.state
            .schematic
            .session
            .editor
            .selection
            .select_component(id + 999);
        assert!(!rename_selection_available(&app.state));
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        app.state.schematic.session.read_only = true;
        assert!(!rename_selection_available(&app.state));
    }

    #[test]
    fn named_wire_and_interface_port_open_the_net_scoped_rename_target() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(81, vec![Point::new(0, 0), Point::new(40, 0)]));
        app.state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(82, Point::new(20, 0), "sense"));
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_wire(81);
        assert!(open_selected_object_rename(&mut app.state));
        assert!(matches!(
            app.state.dialogs.rename_selection.target,
            Some(RenameSelectionTarget::NamedNet(ref target))
                if target.name == "sense" && target.labels[0].id == 82
        ));
        app.state.dialogs.rename_selection.close();

        let port = Component::new(83, ComponentType::Port, Point::new(0, 0))
            .with_name_value("P83", "sense");
        app.state
            .schematic
            .document_mut_for_test()
            .components
            .push(port);
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(83);
        assert!(open_selected_object_rename(&mut app.state));
        assert!(matches!(
            app.state.dialogs.rename_selection.target,
            Some(RenameSelectionTarget::NamedNet(ref target))
                if target.name == "sense"
                    && target.ports.iter().any(|port| port.id == 83)
        ));
    }

    #[test]
    fn component_validation_enforces_prefix_and_case_insensitive_uniqueness() {
        let (mut app, id) = component_app();
        let other = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 0));
        app.state
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == other)
            .unwrap()
            .name = "RLOAD".to_owned();
        let target = app
            .state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .unwrap()
            .clone();

        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target.clone())),
                "C7",
                &|_| true,
            ),
            RenameValidation::Invalid(_)
        ));
        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target.clone())),
                "rload",
                &|_| true,
            ),
            RenameValidation::Invalid(_)
        ));
        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target)),
                "R_GAIN_2",
                &|_| true,
            ),
            RenameValidation::Valid(_)
        ));
    }

    #[test]
    fn successful_component_rename_retains_id_and_is_exactly_one_undo_step() {
        let (mut app, id) = component_app();
        let original_name = app.state.schematic.document().components[0].name.clone();
        assert!(open_selected_object_rename(&mut app.state));
        let target = app.state.dialogs.rename_selection.target.clone().unwrap();
        let commit = match validate_draft(&app.state.schematic, &target, "R_GAIN", &|_| true) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid rename, got {other:?}"),
        };

        assert!(apply_commit(&mut app.state, *commit).unwrap());
        assert_eq!(app.state.schematic.document().components[0].id, id);
        assert_eq!(app.state.schematic.document().components[0].name, "R_GAIN");
        app.action_edit_undo();
        assert_eq!(app.state.schematic.document().components[0].id, id);
        assert_eq!(
            app.state.schematic.document().components[0].name,
            original_name
        );
        assert!(
            app.state.project_undo_sequence().is_none() && !app.state.schematic.can_undo(),
            "rename created more than one undo step"
        );
        app.action_edit_redo();
        assert_eq!(app.state.schematic.document().components[0].name, "R_GAIN");
    }

    #[test]
    fn stale_target_and_cancel_never_mutate() {
        let (mut app, id) = component_app();
        let original = app.state.schematic.document().components[0].clone();
        assert!(open_selected_object_rename(&mut app.state));
        app.state.dialogs.rename_selection.draft = "R_CANCELLED".to_owned();
        app.state.dialogs.rename_selection.close();
        assert_eq!(app.state.schematic.document().components[0], original);
        assert!(!app.state.schematic.can_undo());

        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        assert!(open_selected_object_rename(&mut app.state));
        let target = app.state.dialogs.rename_selection.target.clone().unwrap();
        let pending_commit =
            match validate_draft(&app.state.schematic, &target, "R_STALE", &|_| true) {
                RenameValidation::Valid(commit) => commit,
                other => panic!("expected initially valid rename, got {other:?}"),
            };
        app.state.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
        let before = app.state.schematic.document().components[0].clone();
        assert!(matches!(
            validate_draft(&app.state.schematic, &target, "R_STALE", &|_| true),
            RenameValidation::Invalid(_)
        ));
        assert!(apply_commit(&mut app.state, *pending_commit).is_err());
        assert_eq!(app.state.schematic.document().components[0], before);
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn net_labels_allow_case_insensitive_aliases_and_keep_stable_id() {
        let mut schematic = SchematicState::default();
        let first = schematic.add_net_label(Point::new(0, 0), "sense".to_owned());
        schematic.add_net_label(Point::new(10, 0), "SENSE".to_owned());
        let expected = schematic
            .document()
            .net_labels
            .iter()
            .find(|label| label.id == first)
            .unwrap()
            .clone();
        let commit = match validate_draft(
            &schematic,
            &RenameSelectionTarget::NetLabel(expected),
            "gain_node",
            &|_| true,
        ) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid label rename, got {other:?}"),
        };

        let mut state = AppState::default();
        state.schematic = schematic;
        assert!(apply_commit(&mut state, *commit).unwrap());
        let renamed = state
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|label| label.id == first)
            .unwrap();
        assert_eq!(renamed.id, first);
        assert_eq!(renamed.name, "gain_node");
    }

    #[test]
    fn declared_bus_rename_preserves_range_and_uses_semantic_undo() {
        let mut schematic = SchematicState::default();
        let bus = Bus::segment(
            71,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        schematic.document_mut_for_test().buses.push(bus.clone());
        let commit = match validate_draft(
            &schematic,
            &RenameSelectionTarget::Bus(bus),
            "ADDR",
            &|_| true,
        ) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid bus rename, got {other:?}"),
        };

        let mut state = AppState::default();
        state.schematic = schematic;
        assert!(apply_commit(&mut state, *commit).unwrap());
        let schematic = &mut state.schematic;
        assert_eq!(schematic.document().buses[0].id, 71);
        assert_eq!(
            schematic.document().buses[0]
                .declaration
                .as_ref()
                .unwrap()
                .to_string(),
            "ADDR[7:0]"
        );
        assert!(schematic.undo());
        assert_eq!(
            schematic.document().buses[0]
                .declaration
                .as_ref()
                .unwrap()
                .to_string(),
            "DATA[7:0]"
        );
        assert!(!schematic.undo());
    }

    #[test]
    fn rendered_enter_commits_and_escape_cancels() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let (mut app, id) = component_app();
        assert!(open_selected_object_rename(&mut app.state));
        app.state.dialogs.rename_selection.draft = "R_ENTER".to_owned();
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        assert!(!app.state.dialogs.rename_selection.open);
        assert_eq!(app.state.schematic.document().components[0].id, id);
        assert_eq!(app.state.schematic.document().components[0].name, "R_ENTER");

        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        assert!(open_selected_object_rename(&mut app.state));
        app.state.dialogs.rename_selection.draft = "R_ESCAPE".to_owned();
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        assert!(!app.state.dialogs.rename_selection.open);
        assert_eq!(app.state.schematic.document().components[0].name, "R_ENTER");
    }

    fn saved_output(name: &str, expression: &str) -> crate::state::SavedOutput {
        crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            crate::state::SavedOutputCompatibility::OpTranAc,
            crate::state::SavedOutputPolicy::EveryAcceptedPoint,
            crate::state::SavedOutputPrecision::FullSourcePrecision,
            crate::state::SavedOutputStreaming::StoreOnly,
        )
        .expect("a raw probe is a valid saved output")
    }

    #[test]
    fn instance_rename_remaps_configuration_paths_and_saved_outputs() {
        let (mut app, id) = component_app();
        let configuration = app
            .state
            .workspace
            .content
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Release".to_owned(),
                root: app.state.workspace.content.active_view.clone(),
                dut_path: "/R1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Rename test".to_owned(),
            })
            .expect("valid configuration root");
        let plan_id = crate::product::SimulationPlanId::new();
        app.state.workspace.content.simulation_plan_payloads.push(
            crate::state::SimulationPlanPayloadRecord {
                plan_id,
                payload: crate::state::SimulationPlanPayload {
                    saved_outputs: vec![
                        saved_output("device current", "I(R1)"),
                        saved_output("internal node", "V(/R1/mid)"),
                        saved_output("unrelated node", "V(out)"),
                    ],
                    ..Default::default()
                },
            },
        );
        app.state.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        assert!(open_selected_object_rename(&mut app.state));
        let target = app.state.dialogs.rename_selection.target.clone().unwrap();
        let commit = match validate_draft(&app.state.schematic, &target, "RLOAD", &|_| true) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid rename, got {other:?}"),
        };

        assert!(apply_commit(&mut app.state, *commit).unwrap());

        let expressions = app
            .state
            .workspace
            .content
            .simulation_plan_payloads
            .iter()
            .find(|record| record.plan_id == plan_id)
            .expect("the payload this test owns")
            .payload
            .saved_outputs
            .iter()
            .map(|output| output.source_expression.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            expressions,
            vec!["I(RLOAD)", "V(/RLOAD/mid)", "V(out)"],
            "only the references that name the renamed instance move"
        );
        assert_eq!(
            app.state
                .workspace
                .content
                .configuration_sets
                .find(configuration)
                .expect("the configuration this test owns")
                .dut_path(),
            "/RLOAD",
            "the configuration DUT path follows the renamed instance"
        );
        assert!(app.state.workspace.content.project_metadata_dirty);
        app.action_edit_undo();
        assert_eq!(app.state.schematic.document().components[0].name, "R1");
        assert_eq!(
            app.state
                .workspace
                .content
                .configuration_sets
                .find(configuration)
                .unwrap()
                .dut_path(),
            "/R1",
            "Undo must restore the configuration together with the schematic"
        );
        assert_eq!(
            app.state
                .workspace
                .content
                .plan_data(plan_id)
                .unwrap()
                .saved_outputs[0]
                .source_expression,
            "I(R1)",
            "Undo must restore the executable saved output"
        );
        app.action_edit_redo();
        assert_eq!(app.state.schematic.document().components[0].name, "RLOAD");
        assert_eq!(
            app.state
                .workspace
                .content
                .configuration_sets
                .find(configuration)
                .unwrap()
                .dut_path(),
            "/RLOAD"
        );
        assert_eq!(
            app.state
                .workspace
                .content
                .plan_data(plan_id)
                .unwrap()
                .saved_outputs[0]
                .source_expression,
            "I(RLOAD)"
        );
    }

    fn declaration(text: &str) -> crate::state::BusDeclaration {
        crate::state::BusDeclaration::parse(text).unwrap()
    }

    fn bus_by_id(state: &AppState, id: u64) -> Bus {
        state
            .schematic
            .document()
            .buses
            .iter()
            .find(|bus| bus.id == id)
            .unwrap()
            .clone()
    }

    /// `DATA[7:0]` with a multi-bit `DATA[3:0]` tap ending at (50, 20) on
    /// the active sheet, optionally on a `DATA[3:0]` bus there, and on a
    /// second sheet a `DATA[3:0]` bus passing through that same point.
    /// Returns the tapped bus, the active-sheet destination, and the bus on
    /// the other sheet.
    fn tap_over_another_sheets_bus(state: &mut AppState, destination: bool) -> (u64, u64, u64) {
        use crate::state::{
            BusSlice, BusTapOrientation, SheetDefinition, SheetPortPolicy, SheetTemplate,
        };
        let schematic = &mut state.schematic;
        let source = schematic
            .add_bus(
                vec![Point::new(0, 0), Point::new(100, 0)],
                Some(declaration("DATA[7:0]")),
            )
            .unwrap();
        let target = if destination {
            schematic
                .add_bus(
                    vec![Point::new(0, 20), Point::new(100, 20)],
                    Some(declaration("DATA[3:0]")),
                )
                .unwrap()
        } else {
            0
        };
        schematic
            .place_bus_tap(
                source,
                Point::new(50, 0),
                Point::new(50, 20),
                BusSlice::parse("DATA[3:0]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        let hidden = schematic
            .add_bus(
                vec![Point::new(50, 10), Point::new(50, 40)],
                Some(declaration("DATA[3:0]")),
            )
            .unwrap();
        let document = state.schematic.document();
        let ids: Vec<u64> = document
            .buses
            .iter()
            .map(|bus| bus.id)
            .chain(document.bus_taps.iter().map(|tap| tap.id))
            .collect();
        let key = state.workspace.content.active_schematic_reference().key();
        let management = &mut state.workspace.content.design_management;
        let first = management
            .bootstrap_for_cell_view(&key, "Sheet 1", ids)
            .unwrap();
        let catalog = management.sheet_catalog_mut(&key).unwrap();
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), second, [hidden])
            .unwrap();
        catalog.set_active(first).unwrap();
        (source, target, hidden)
    }

    #[test]
    fn a_bus_rename_is_not_refused_over_another_sheets_bus() {
        let mut state = AppState::default();
        let (source, target, hidden) = tap_over_another_sheets_bus(&mut state, true);

        let commit = RenameCommit::Bus {
            expected: bus_by_id(&state, source),
            declaration: declaration("ADDR[7:0]"),
        };
        apply_commit(&mut state, commit)
            .expect("the tap has one destination on this sheet, which is renamed with it");

        assert_eq!(
            bus_by_id(&state, target).declaration,
            Some(declaration("ADDR[3:0]"))
        );
        assert_eq!(
            bus_by_id(&state, hidden).declaration,
            Some(declaration("DATA[3:0]"))
        );
    }

    #[test]
    fn a_bus_rename_never_renames_another_sheets_bus() {
        let mut state = AppState::default();
        let (source, _, hidden) = tap_over_another_sheets_bus(&mut state, false);

        let commit = RenameCommit::Bus {
            expected: bus_by_id(&state, source),
            declaration: declaration("ADDR[7:0]"),
        };
        apply_commit(&mut state, commit).unwrap();

        assert_eq!(
            bus_by_id(&state, source).declaration,
            Some(declaration("ADDR[7:0]"))
        );
        assert_eq!(
            bus_by_id(&state, hidden).declaration,
            Some(declaration("DATA[3:0]")),
            "the other sheet's bus kept its name"
        );
    }
}
