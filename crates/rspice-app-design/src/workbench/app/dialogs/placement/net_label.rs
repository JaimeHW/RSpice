//! Typed net-label placement, entered at the click that anchored it.
//!
//! The Label tool captures one snapped canvas anchor and raises a small entry
//! beside it. The entry owns the isolated name draft and publishes a durable
//! [`NetLabel`] only after the complete document authority and naming policy
//! have been revalidated.
//!
//! The off-sheet connector tool reaches the same transaction. Its extra
//! declaration is a direction, so the entry offers exactly one more control
//! and publishes name, anchor and kind together — a connector never exists as
//! a plain label in any recorded state.

use egui::Context;
use rspice_schematic_editor::net_label_placement;
use rspice_schematic_editor::session::net_label_sequence::{NetLabelSequence, entry_names};

use crate::state::{CrossSheetPortDirection, NetLabel, NetLabelKind, Point, Tool};
use crate::ui::widgets::{CanvasEntry, CanvasEntryChoice};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::{deny_read_only_edit, sync_active_schematic};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

/// Isolated draft for one snapped, typed net-label placement.
///
/// The canvas click supplies only the anchor and immutable document
/// authority. No durable schematic object exists until the dialog's primary
/// action validates and publishes the complete label.
#[derive(Debug, Clone, Default)]
pub struct NetLabelPlacementDialogState {
    pub open: bool,
    pub name: String,
    pub(crate) anchor: Option<crate::state::Point>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    /// Which label the armed tool is placing. The direction of an off-sheet
    /// connector is part of the draft, so it publishes with the name in one
    /// transaction rather than needing a second edit.
    pub kind: crate::state::NetLabelKind,
    /// Where on the screen the entry sits, taken from the click that opened
    /// it. Kept, so the card does not follow the pointer once it is up.
    pub screen_anchor: Option<egui::Pos2>,
    pub(crate) validation_error: Option<String>,
}

impl NetLabelPlacementDialogState {
    pub(crate) fn open(
        &mut self,
        anchor: crate::state::Point,
        authority: SchematicEditAuthority,
        kind: crate::state::NetLabelKind,
    ) {
        *self = Self {
            open: true,
            name: String::new(),
            anchor: Some(anchor),
            authority: Some(authority),
            kind,
            screen_anchor: None,
            validation_error: None,
        };
    }

    /// The entry's place on the screen, fixed the first frame it is drawn.
    pub(crate) fn screen_anchor_or(&mut self, pointer: egui::Pos2) -> egui::Pos2 {
        *self.screen_anchor.get_or_insert(pointer)
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Net label";
const CONNECTOR_TITLE: &str = "Off-sheet connector";

/// The most names one entry may ask for, so a mistyped range cannot arm a
/// sequence no one will click through.
const MAX_NAMES: usize = 1024;

#[derive(Debug, Clone)]
pub struct NetLabelPlacementCommit {
    authority: SchematicEditAuthority,
    anchor: Point,
    name: String,
    kind: NetLabelKind,
    /// What the tool places on the next clicks.
    following: Option<NetLabelSequence>,
}

#[derive(Debug, Clone)]
pub enum DraftValidation {
    Incomplete(&'static str),
    Invalid(String),
    /// Boxed: the commit dwarfs the two message-only variants.
    Valid(Box<NetLabelPlacementCommit>),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    /// Why the name is refused, if it is.
    ///
    /// An entry nothing has been typed into has refused nothing: it opens
    /// under the pointer and says so by being there.
    fn refusal(&self) -> Option<&str> {
        match self {
            Self::Incomplete(_) | Self::Valid(_) => None,
            Self::Invalid(message) => Some(message.as_str()),
        }
    }
}

/// Capture a snapped label anchor in the entry's draft without changing the
/// schematic.
///
/// The armed tool decides which label is being placed, so the canvas never has
/// to carry a second entry point for the connector.
///
/// Returning `false` means the active document does not currently own edit
/// authority or another application modal already owns the interaction
/// (`dialog_modal_open`, whether the dialog gate was held).
pub fn open_net_label_placement_in(
    dialog: &mut NetLabelPlacementDialogState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    dialog_modal_open: bool,
    anchor: Point,
) -> bool {
    if dialog_modal_open {
        return false;
    }
    if model.schematic_edit_read_only() {
        deny_read_only_edit(model, wb);
        return false;
    }

    let kind = if model.schematic.session.editor.tool == Tool::OffSheetConnector {
        NetLabelKind::OffSheet {
            direction: CrossSheetPortDirection::default(),
        }
    } else {
        NetLabelKind::Local
    };
    let authority = SchematicEditAuthority::capture(&*model);
    dialog.open(anchor, authority, kind);
    true
}

/// Draw the net-label entry over its draft, publishing to the model.
pub(in crate::workbench) fn render_net_label_dialog(
    dialog: &mut NetLabelPlacementDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }

    let validation = validate_draft(dialog, model);
    let refusal = dialog
        .validation_error
        .clone()
        .or_else(|| validation.refusal().map(str::to_owned));
    let off_sheet = dialog.kind.off_sheet_direction().is_some();
    // The pointer is still where the click that opened this was, so the
    // entry can take its place from it without the canvas having to hand
    // one over.
    let anchor = dialog.screen_anchor_or(
        ctx.pointer_latest_pos()
            .unwrap_or_else(|| ctx.viewport_rect().center()),
    );

    let typed_before = dialog.name.clone();
    let draft = &mut *dialog;
    let choice = CanvasEntry::new(
        "place-net-label",
        if off_sheet { CONNECTOR_TITLE } else { TITLE },
        anchor,
    )
    .commit_enabled(validation.can_commit())
    .status(match refusal.as_deref() {
        Some(message) => Err(message),
        None => Ok(None),
    })
    .show(ctx, |rows| {
        net_label_placement::show(rows, &mut draft.name, &mut draft.kind)
    });
    // Typing answers the refusal the last attempt recorded.
    if dialog.name != typed_before {
        dialog.validation_error = None;
    }

    match choice {
        CanvasEntryChoice::Commit => {
            // Revalidate after this frame's text edit so Enter can never
            // publish a candidate that became invalid in the same pass.
            match validate_draft(dialog, model) {
                DraftValidation::Valid(commit) => {
                    match apply_commit(model, workbench, *commit) {
                        // The label is on the sheet: the drawing is the
                        // report, and a second one would only cover it.
                        Ok(_) => dialog.close(),
                        Err(error) => {
                            dialog.validation_error = Some(error);
                        }
                    }
                }
                DraftValidation::Incomplete(message) => {
                    dialog.validation_error = Some(message.to_owned());
                }
                DraftValidation::Invalid(message) => {
                    dialog.validation_error = Some(message);
                }
            }
        }
        CanvasEntryChoice::Cancelled => dialog.close(),
        CanvasEntryChoice::None => {}
    }
}

fn validate_draft(draft: &NetLabelPlacementDialogState, model: &Model) -> DraftValidation {
    let Some(authority) = draft.authority.as_ref() else {
        return DraftValidation::Invalid(
            "The placement authority is unavailable. Cancel and select the anchor again."
                .to_owned(),
        );
    };
    if let Err(error) = authority.validate(model, "Place net label") {
        return DraftValidation::Invalid(error);
    }
    let Some(anchor) = draft.anchor else {
        return DraftValidation::Invalid(
            "The snapped placement anchor is unavailable. Cancel and select it again.".to_owned(),
        );
    };
    // A list or a bus range names one label per click; on a bus, a range
    // names the bus whole.
    let names = entry_names(&draft.name, anchor_on_bus(model, anchor));
    let Some(first) = names.first() else {
        return DraftValidation::Incomplete("Enter the electrical net name.");
    };
    if names.len() > MAX_NAMES {
        return DraftValidation::Invalid(format!("Name at most {MAX_NAMES} nets at once."));
    }
    let policy = model.schematic.document().document_policy.net_naming;
    for name in &names {
        if let Err(reason) = NetLabel::validate_name(name, policy) {
            return DraftValidation::Invalid(if names.len() > 1 {
                format!("{name}: net name: {reason}.")
            } else {
                format!("Net name: {reason}.")
            });
        }
    }
    DraftValidation::Valid(Box::new(NetLabelPlacementCommit {
        authority: authority.clone(),
        anchor,
        name: first.clone(),
        kind: draft.kind,
        following: NetLabelSequence::following(&names, draft.kind),
    }))
}

/// Whether a bus on the active sheet runs through `anchor`.
fn anchor_on_bus(model: &Model, anchor: Point) -> bool {
    model.schematic.document().buses.iter().any(|bus| {
        bus.contains_point(anchor)
            && crate::schematic::view::sheet_visibility::object_is_on_active_sheet(model, bus.id)
    })
}

/// Place the name an armed sequence holds next at `anchor`, as one undo
/// step, and arm the one after it. Returns whether a sequence answered the
/// click; with none armed, the click opens the entry. `dialog_modal_open` is
/// whether the dialog gate was held.
pub fn place_sequenced_net_label_in(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    dialog_modal_open: bool,
    anchor: Point,
) -> bool {
    let Some(mut sequence) = model
        .schematic
        .session
        .editor
        .pending_net_label_sequence
        .clone()
    else {
        return false;
    };
    let Some(name) = sequence.next_name() else {
        model.schematic.session.editor.pending_net_label_sequence = None;
        return false;
    };
    if dialog_modal_open {
        return true;
    }
    if model.schematic_edit_read_only() {
        deny_read_only_edit(model, wb);
        return true;
    }
    let policy = model.schematic.document().document_policy.net_naming;
    if let Err(reason) = NetLabel::validate_name(&name, policy) {
        model.schematic.session.editor.pending_net_label_sequence = None;
        wb.prompt(format!("{name}: net name: {reason}."));
        return true;
    }
    let kind = sequence.kind;
    let undo_label = if kind.off_sheet_direction().is_some() {
        "place off-sheet connector"
    } else {
        "place net label"
    };
    let placed = model.schematic.with_undo(undo_label, |schematic| {
        schematic.add_net_label_with_kind(anchor, name.clone(), kind);
    });
    if !placed {
        wb.prompt(format!("{name} was not placed."));
        return true;
    }
    model.schematic.session.editor.pending_net_label_sequence =
        sequence.advance().then_some(sequence);
    sync_active_schematic(model, wb);
    true
}

fn apply_commit(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    commit: NetLabelPlacementCommit,
) -> Result<u64, String> {
    let kind = commit.kind;
    let off_sheet = kind.off_sheet_direction().is_some();
    // History and authority errors name what was actually placed, so a
    // connector never appears in either as an ordinary label.
    commit
        .authority
        .validate(&*model, if off_sheet { CONNECTOR_TITLE } else { TITLE })?;
    NetLabel::validate_name(
        &commit.name,
        model.schematic.document().document_policy.net_naming,
    )
    .map_err(|reason| format!("Net name: {reason}."))?;

    let mut placed_id = None;
    let undo_label = if off_sheet {
        "place off-sheet connector"
    } else {
        "place net label"
    };
    let changed = model.schematic.with_undo(undo_label, |schematic| {
        let id = schematic.add_net_label_with_kind(commit.anchor, commit.name, kind);
        placed_id = Some(id);
    });
    match (changed, placed_id) {
        (true, Some(id)) => {
            // The tool stays armed with the next name, which the ghost shows.
            model.schematic.session.editor.pending_net_label_sequence = commit.following;
            sync_active_schematic(model, workbench);
            Ok(id)
        }
        _ => Err(
            "The active schematic rejected the placement; no label or undo record was created."
                .to_owned(),
        ),
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;

    /// Draw the entry as its dialog step does.
    pub fn render(
        dialog: &mut NetLabelPlacementDialogState,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        ctx: &Context,
    ) {
        render_net_label_dialog(dialog, model, workbench, ctx);
    }

    /// The draft's validation, over what the entry reads.
    pub fn validate_draft(dialog: &NetLabelPlacementDialogState, model: &Model) -> DraftValidation {
        super::validate_draft(dialog, model)
    }

    /// The entry's publication, over what it writes.
    pub fn apply_commit(
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        commit: NetLabelPlacementCommit,
    ) -> Result<u64, String> {
        super::apply_commit(model, workbench, commit)
    }

    pub fn key_event(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// The kind is a drawing and review declaration, never an electrical one.
    /// Sheets are projected into separate coordinate namespaces, so two
    /// connectors that share a name have no geometry in common — the name is
    /// the only thing that can join them, exactly as for a plain label.
    #[test]
    fn connectors_sharing_a_name_join_into_one_node_across_sheet_namespaces() {
        let mut schematic = crate::state::SchematicState::default();
        let near = schematic
            .add_wire(vec![Point::origin(), Point::new(40, 0)])
            .expect("first sheet conductor");
        let far = schematic
            .add_wire(vec![Point::new(1_000_000, 0), Point::new(1_000_040, 0)])
            .expect("second sheet conductor");
        assert_ne!(near, far);
        let near_label = schematic.next_id();
        schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::off_sheet(
                near_label,
                Point::origin(),
                "BIAS",
                CrossSheetPortDirection::Output,
            ));
        let far_label = schematic.next_id();
        schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::off_sheet(
                far_label,
                Point::new(1_000_000, 0),
                "BIAS",
                CrossSheetPortDirection::Input,
            ));

        let joined: Vec<_> = rspice_design::connectivity::summary::design_nets(&schematic)
            .into_iter()
            .filter(|net| net.name == "BIAS")
            .collect();
        assert_eq!(joined.len(), 1, "the shared name is one node");
        assert!(joined[0].authored_name);
        assert_eq!(joined[0].wire_ids.len(), 2, "both conductors joined");
    }
}
