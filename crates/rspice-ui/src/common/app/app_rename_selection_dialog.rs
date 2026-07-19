//! Mockup-owned stable-identity rename transaction.
//!
//! Edit ▸ Rename selected object captures one live schematic object by
//! durable ID, edits an isolated name draft, and publishes exactly one guarded
//! undo transaction. Components, net labels, and declared buses retain their
//! IDs; bus dependencies are rebased through the schematic's semantic bus
//! transaction rather than by rewriting display text.

use egui::{Context, Frame, Response, Stroke, TextEdit, Ui, Vec2};

use crate::state::{BusDeclaration, Component, NetLabel, SchematicState};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};

use super::{AppState, ConsoleMessage, RSpiceApp, RenameSelectionTarget};

const EYEBROW: &str = "EDIT \u{00b7} STABLE IDENTITY";
const TITLE: &str = "Rename selected object";
const PRIMARY: &str = "Rename object";
const BODY: &str = "Change a user-facing name while preserving stable identity and previewing netlist, requirement, and review references.";
const FIELD_ID: &str = "rename-selected-object-name";
const DIALOG_SIZE: DialogSize = DialogSize::Transaction;

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
    !state.schematic.read_only
        && !state.active_view_read_only()
        && selected_rename_target(state).is_some()
}

/// Capture the current selection into an isolated stable-identity draft.
pub(crate) fn open_selected_object_rename(state: &mut AppState) -> bool {
    if state.dialogs.rename_selection.open
        || state.dialogs.object_properties.open
        || state.tabbed_property_dialog.open
        || state.schematic.read_only
        || state.active_view_read_only()
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
        state.workspace.active_view.display_path(),
    );
    true
}

fn selected_rename_target(state: &AppState) -> Option<RenameSelectionTarget> {
    let schematic = &state.schematic;
    if let Some(id) = schematic.selection.single_component() {
        return schematic
            .components
            .iter()
            .find(|component| component.id == id && !component.kind.spice_prefix().is_empty())
            .cloned()
            .map(Box::new)
            .map(RenameSelectionTarget::Component);
    }
    if let Some(id) = schematic.selection.single_net_label() {
        return schematic
            .net_labels
            .iter()
            .find(|label| label.id == id)
            .cloned()
            .map(RenameSelectionTarget::NetLabel);
    }
    if let Some(id) = schematic.selection.single_bus() {
        return schematic
            .buses
            .iter()
            .find(|bus| bus.id == id && bus.declaration.is_some())
            .cloned()
            .map(RenameSelectionTarget::Bus);
    }
    None
}

impl RSpiceApp {
    pub(super) fn render_rename_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_selection.open {
            return;
        }
        let Some(target) = self.state.dialogs.rename_selection.target.as_ref() else {
            self.state.dialogs.rename_selection.close();
            self.state.push_user_message(ConsoleMessage::warning(
                "Rename closed because its stable target was unavailable.".to_owned(),
            ));
            return;
        };

        let authority_error = rename_session_error(&self.state);
        let validation = authority_error.map_or_else(
            || {
                validate_draft(
                    &self.state.schematic,
                    target,
                    &self.state.dialogs.rename_selection.draft,
                )
            },
            RenameValidation::Invalid,
        );
        let object = object_summary(target);
        let references = reference_summary(&self.state.schematic, target);
        let validation_message = self
            .state
            .dialogs
            .rename_selection
            .validation_error
            .as_deref()
            .or_else(|| validation.message())
            .map(str::to_owned);

        let mut edited = false;
        let choice = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(BODY)
            .size(DIALOG_SIZE)
            .ghost("Cancel")
            .primary_enabled(validation.can_commit())
            .initial_focus(DialogInitialFocus::BodyControl)
            .show_with_initial_body_focus(ctx, |ui| {
                let (focus, changed) = rename_body(
                    ui,
                    &object,
                    &references,
                    &mut self.state.dialogs.rename_selection.draft,
                    validation_message.as_deref(),
                );
                edited = changed;
                focus
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
                        )
                    },
                    RenameValidation::Invalid,
                );
                match validation {
                    RenameValidation::Valid(commit) => {
                        match apply_commit(&mut self.state.schematic, *commit) {
                            Ok(true) => {
                                let message = format!(
                                    "{} was renamed as one undoable stable-identity transaction.",
                                    object
                                );
                                self.state
                                    .push_user_message(ConsoleMessage::info(message.clone()));
                                self.state.ui.toasts.success(
                                    ctx,
                                    "Object renamed",
                                    "Stable identity and dependent schematic references were preserved.",
                                );
                                self.state.dialogs.rename_selection.close();
                            }
                            Ok(false) => {
                                self.state.dialogs.rename_selection.validation_error = Some(
                                    "The target no longer required a rename. Review its current name and try again."
                                        .to_owned(),
                                );
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

fn rename_body(
    ui: &mut Ui,
    object: &str,
    references: &str,
    draft: &mut String,
    validation_message: Option<&str>,
) -> (Option<egui::Id>, bool) {
    read_only_value(ui, "Object", object);
    ui.add_space(10.0);
    field_label(ui, "New name");
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        Vec2::new(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(draft)
            .id_source(FIELD_ID)
            .font(egui::TextStyle::Monospace)
            .hint_text("Enter a SPICE-safe name")
            .margin(egui::Margin::symmetric(8, 4)),
    );
    configure_name_accessibility(ui, &response, validation_message);
    let changed = response.changed();
    if changed {
        // A prior publication error belongs to an older candidate. The live
        // validation under this field now owns the current draft state.
        ui.ctx().request_repaint();
    }
    if let Some(message) = validation_message {
        ui.add_space(4.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.err),
            )
            .wrap(),
        );
    }
    ui.add_space(10.0);
    read_only_value(ui, "References", references);
    (Some(response.id), changed)
}

fn configure_name_accessibility(ui: &Ui, response: &Response, error: Option<&str>) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("New name");
        if let Some(error) = error {
            node.set_invalid(egui::accesskit::Invalid::True);
            node.set_description(format!("Required SPICE-safe name. {error}"));
        } else {
            node.clear_invalid();
            node.set_description("Required SPICE-safe name");
        }
    });
}

fn field_label(ui: &mut Ui, label: &str) {
    ui.label(
        egui::RichText::new(label.to_ascii_uppercase())
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(Tokens::get(ui.ctx()).color.text_faint),
    );
    ui.add_space(4.0);
}

fn read_only_value(ui: &mut Ui, label: &str, value: &str) {
    field_label(ui, label);
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_app)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                        .color(t.color.text),
                )
                .wrap(),
            );
        });
}

fn object_summary(target: &RenameSelectionTarget) -> String {
    match target {
        RenameSelectionTarget::Component(component) => format!(
            "{} \u{00b7} {} \u{00b7} stable ID COMPONENT-{:03}",
            component.name,
            component.kind.display_name(),
            component.id
        ),
        RenameSelectionTarget::NetLabel(label) => {
            format!("net {} \u{00b7} stable ID NET-{:03}", label.name, label.id)
        }
        RenameSelectionTarget::Bus(bus) => format!(
            "bus {} \u{00b7} stable ID BUS-{:03}",
            bus.declaration
                .as_ref()
                .map_or_else(|| "undeclared".to_owned(), ToString::to_string),
            bus.id
        ),
    }
}

fn reference_summary(schematic: &SchematicState, target: &RenameSelectionTarget) -> String {
    match target {
        RenameSelectionTarget::Component(component) => {
            let terminals = schematic
                .connections
                .iter()
                .filter(|connection| connection.component_id == component.id)
                .count();
            format!(
                "{terminals} terminal connection(s) \u{00b7} generated netlist designator updates on commit \u{00b7} 0 stable-ID requirement/review records"
            )
        }
        RenameSelectionTarget::NetLabel(label) => {
            let occurrences = schematic
                .net_labels
                .iter()
                .filter(|candidate| match schematic.document_policy.net_naming {
                    crate::state::NetNamingPolicy::StrictCaseSensitive => {
                        candidate.name == label.name
                    }
                    crate::state::NetNamingPolicy::SpiceCompatibleRelaxed => {
                        candidate.name.eq_ignore_ascii_case(&label.name)
                    }
                })
                .count();
            let comparison = match schematic.document_policy.net_naming {
                crate::state::NetNamingPolicy::StrictCaseSensitive => "exact-case",
                crate::state::NetNamingPolicy::SpiceCompatibleRelaxed => "case-insensitive",
            };
            format!(
                "{occurrences} {comparison} named-net occurrence(s) \u{00b7} stable label ID retained \u{00b7} 0 stable-ID requirement/review records"
            )
        }
        RenameSelectionTarget::Bus(bus) => {
            let taps = schematic
                .bus_taps
                .iter()
                .filter(|tap| tap.bus_id == bus.id)
                .count();
            format!(
                "{taps} dependent bus-tap selector(s) \u{00b7} selectors rebase atomically \u{00b7} 0 stable-ID requirement/review records"
            )
        }
    }
}

fn rename_session_error(state: &AppState) -> Option<String> {
    let dialog = &state.dialogs.rename_selection;
    if state.schematic.read_only || state.active_view_read_only() {
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
    (dialog.view_path != state.workspace.active_view.display_path()).then(|| {
        "The active cell/view changed while Rename was open. Cancel and reopen the current object."
            .to_owned()
    })
}

fn validate_draft(
    schematic: &SchematicState,
    target: &RenameSelectionTarget,
    draft: &str,
) -> RenameValidation {
    if schematic.read_only {
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
            let prefix = expected.kind.spice_prefix();
            if prefix.is_empty() {
                return RenameValidation::Invalid(
                    "This component type does not own a SPICE reference designator.".to_owned(),
                );
            }
            if !candidate
                .get(..prefix.len())
                .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
            {
                return RenameValidation::Invalid(format!(
                    "{} designators must begin with `{prefix}`.",
                    expected.kind.display_name()
                ));
            }
            let suffix = &candidate[prefix.len()..];
            if suffix.is_empty()
                || !suffix
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            {
                return RenameValidation::Invalid(format!(
                    "Enter `{prefix}` followed by one or more ASCII letters, digits, or underscores."
                ));
            }
            if schematic.components.iter().any(|component| {
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
                schematic.document_policy.net_naming,
            ) {
                return RenameValidation::Invalid(format!("Net name: {reason}."));
            }
            RenameValidation::Valid(Box::new(RenameCommit::NetLabel {
                expected: expected.clone(),
                name: candidate.to_owned(),
            }))
        }
        RenameSelectionTarget::Bus(expected) => {
            let Some(current) = schematic.buses.iter().find(|bus| bus.id == expected.id) else {
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
            match schematic.validate_bus_properties(expected, Some(&declaration)) {
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

fn apply_commit(schematic: &mut SchematicState, commit: RenameCommit) -> Result<bool, String> {
    match commit {
        RenameCommit::Component { expected, name } => {
            let Some(current) = schematic
                .components
                .iter()
                .find(|component| component.id == expected.id)
            else {
                return Err("The selected component no longer exists.".to_owned());
            };
            if current != expected.as_ref() {
                return Err("The selected component changed before commit.".to_owned());
            }
            Ok(schematic.with_undo("rename component", move |schematic| {
                if let Some(component) = schematic
                    .components
                    .iter_mut()
                    .find(|component| component.id == expected.id)
                {
                    component.name = name;
                    schematic.is_dirty = true;
                    schematic.bump_topology_version();
                }
            }))
        }
        RenameCommit::NetLabel { expected, name } => {
            let Some(current) = schematic
                .net_labels
                .iter()
                .find(|label| label.id == expected.id)
            else {
                return Err("The selected net label no longer exists.".to_owned());
            };
            if current != &expected {
                return Err("The selected net label changed before commit.".to_owned());
            }
            Ok(schematic.with_undo("rename net label", move |schematic| {
                if let Some(label) = schematic
                    .net_labels
                    .iter_mut()
                    .find(|label| label.id == expected.id)
                {
                    label.name = name;
                    schematic.is_dirty = true;
                    schematic.bump_topology_version();
                }
            }))
        }
        RenameCommit::Bus {
            expected,
            declaration,
        } => schematic
            .edit_bus_properties(&expected, Some(declaration))
            .map_err(|error| format!("The bus rename was rejected: {error}.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Bus, ComponentType, Point};

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
        app.state.schematic.selection.select_only_component(id);
        (app, id)
    }

    #[test]
    fn mockup_contract_is_exact() {
        assert_eq!(EYEBROW, "EDIT \u{00b7} STABLE IDENTITY");
        assert_eq!(TITLE, "Rename selected object");
        assert_eq!(PRIMARY, "Rename object");
        assert_eq!(
            BODY,
            "Change a user-facing name while preserving stable identity and previewing netlist, requirement, and review references."
        );
        assert_eq!(DIALOG_SIZE, DialogSize::Transaction);
    }

    #[test]
    fn availability_requires_one_live_renameable_object_and_write_access() {
        let (mut app, id) = component_app();
        assert!(rename_selection_available(&app.state));

        app.state.schematic.selection.select_component(id + 999);
        assert!(!rename_selection_available(&app.state));
        app.state.schematic.selection.select_only_component(id);
        app.state.schematic.read_only = true;
        assert!(!rename_selection_available(&app.state));
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
            .components
            .iter_mut()
            .find(|component| component.id == other)
            .unwrap()
            .name = "RLOAD".to_owned();
        let target = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .unwrap()
            .clone();

        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target.clone())),
                "C7"
            ),
            RenameValidation::Invalid(_)
        ));
        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target.clone())),
                "rload"
            ),
            RenameValidation::Invalid(_)
        ));
        assert!(matches!(
            validate_draft(
                &app.state.schematic,
                &RenameSelectionTarget::Component(Box::new(target)),
                "R_GAIN_2"
            ),
            RenameValidation::Valid(_)
        ));
    }

    #[test]
    fn successful_component_rename_retains_id_and_is_exactly_one_undo_step() {
        let (mut app, id) = component_app();
        let original_name = app.state.schematic.components[0].name.clone();
        assert!(open_selected_object_rename(&mut app.state));
        let target = app.state.dialogs.rename_selection.target.clone().unwrap();
        let commit = match validate_draft(&app.state.schematic, &target, "R_GAIN") {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid rename, got {other:?}"),
        };

        assert!(apply_commit(&mut app.state.schematic, *commit).unwrap());
        assert_eq!(app.state.schematic.components[0].id, id);
        assert_eq!(app.state.schematic.components[0].name, "R_GAIN");
        assert!(app.state.schematic.undo());
        assert_eq!(app.state.schematic.components[0].id, id);
        assert_eq!(app.state.schematic.components[0].name, original_name);
        assert!(
            !app.state.schematic.undo(),
            "rename created more than one undo step"
        );
        assert!(app.state.schematic.redo());
        assert_eq!(app.state.schematic.components[0].name, "R_GAIN");
    }

    #[test]
    fn stale_target_and_cancel_never_mutate() {
        let (mut app, id) = component_app();
        let original = app.state.schematic.components[0].clone();
        assert!(open_selected_object_rename(&mut app.state));
        app.state.dialogs.rename_selection.draft = "R_CANCELLED".to_owned();
        app.state.dialogs.rename_selection.close();
        assert_eq!(app.state.schematic.components[0], original);
        assert!(!app.state.schematic.can_undo());

        app.state.schematic.selection.select_only_component(id);
        assert!(open_selected_object_rename(&mut app.state));
        let target = app.state.dialogs.rename_selection.target.clone().unwrap();
        let pending_commit = match validate_draft(&app.state.schematic, &target, "R_STALE") {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected initially valid rename, got {other:?}"),
        };
        app.state.schematic.components[0].value = "2k".to_owned();
        let before = app.state.schematic.components[0].clone();
        assert!(matches!(
            validate_draft(&app.state.schematic, &target, "R_STALE"),
            RenameValidation::Invalid(_)
        ));
        assert!(apply_commit(&mut app.state.schematic, *pending_commit).is_err());
        assert_eq!(app.state.schematic.components[0], before);
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn net_labels_allow_case_insensitive_aliases_and_keep_stable_id() {
        let mut schematic = SchematicState::default();
        let first = schematic.add_net_label(Point::new(0, 0), "sense".to_owned());
        schematic.add_net_label(Point::new(10, 0), "SENSE".to_owned());
        let expected = schematic
            .net_labels
            .iter()
            .find(|label| label.id == first)
            .unwrap()
            .clone();
        let commit = match validate_draft(
            &schematic,
            &RenameSelectionTarget::NetLabel(expected),
            "gain_node",
        ) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid label rename, got {other:?}"),
        };

        assert!(apply_commit(&mut schematic, *commit).unwrap());
        let renamed = schematic
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
        schematic.buses.push(bus.clone());
        let commit = match validate_draft(&schematic, &RenameSelectionTarget::Bus(bus), "ADDR") {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected valid bus rename, got {other:?}"),
        };

        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(schematic.buses[0].id, 71);
        assert_eq!(
            schematic.buses[0].declaration.as_ref().unwrap().to_string(),
            "ADDR[7:0]"
        );
        assert!(schematic.undo());
        assert_eq!(
            schematic.buses[0].declaration.as_ref().unwrap().to_string(),
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
        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        assert!(!app.state.dialogs.rename_selection.open);
        assert_eq!(app.state.schematic.components[0].id, id);
        assert_eq!(app.state.schematic.components[0].name, "R_ENTER");

        app.state.schematic.selection.select_only_component(id);
        assert!(open_selected_object_rename(&mut app.state));
        app.state.dialogs.rename_selection.draft = "R_ESCAPE".to_owned();
        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_rename_selection_dialog(ctx)
        });
        assert!(!app.state.dialogs.rename_selection.open);
        assert_eq!(app.state.schematic.components[0].name, "R_ENTER");
    }
}
