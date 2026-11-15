//! App-owned form, source validation and arming for bus-tap placement.

use crate::state::Tool;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app_state::Model;
use egui::Context;
use rspice_schematic_editor::{
    bus_tap_placement::{self, DraftValidation},
    requests::EditorRequestSource,
    session::bus::PendingBusTapPlacement,
};

/// Retained, isolated draft for the bus-tap placement transaction. The parsed
/// electrical contract is not published to the schematic until the primary
/// action validates every field.
#[derive(Debug, Clone, Default)]
pub struct BusTapDialogState {
    pub open: bool,
    pub fields: rspice_schematic_editor::bus_tap_placement::BusTapFields,
    pub source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
}

impl BusTapDialogState {
    /// `bus` is the declaration the sheet leaves no doubt about, or empty.
    /// Nothing else is prefilled: a sample written into a field is a tap the
    /// reader never asked for.
    pub fn open(
        &mut self,
        source: rspice_schematic_editor::requests::EditorRequestSource,
        bus: String,
    ) {
        *self = Self {
            open: true,
            fields: rspice_schematic_editor::bus_tap_placement::BusTapFields {
                bus,
                ..Default::default()
            },
            source: Some(source),
        };
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Place bus tap";
const PRIMARY: &str = "Place";
const DESCRIPTION: &str = "Connect one bit or a range of a bus.";

/// Draw the bus-tap form over its draft and the model it arms.
pub(in crate::workbench) fn render_bus_tap_dialog(
    dialog: &mut BusTapDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }

    let validation = match placement_source(dialog, model) {
        Ok(_) => dialog.fields.validate(),
        Err(message) => DraftValidation::Invalid(message),
    };
    let fields = &mut dialog.fields;
    let choice = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        .primary_enabled(validation.can_commit())
        .status(validation.status())
        .show(ctx, |rows| {
            bus_tap_placement::show(rows, &validation, fields)
        });

    match choice {
        DialogChoice::Primary => {
            // The footer follows the body in the same immediate-mode
            // frame. Re-parse the post-edit draft so Enter can never
            // publish the prior frame's valid contract after a field was
            // changed to an invalid value.
            if let Ok(source) = placement_source(dialog, model)
                && let DraftValidation::Valid(pending) = dialog.fields.validate()
            {
                let placement = PendingBusTapPlacement::new(pending, source.clone());
                model.schematic.session.editor.pending_bus_tap = Some(placement);
                model.schematic.arm_tool(Tool::BusTap);
                dialog.close();
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.close();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

pub fn open_bus_tap(dialog: &mut BusTapDialogState, model: &Model) {
    let source = crate::workbench::app::schematic_editor_request_source(model);
    let bus = sole_declared_bus(model);
    dialog.open(source, bus);
}

/// The bus a tap starts from when the sheet leaves no doubt: the one
/// declaration it draws.
///
/// A sample value written into the field — this form opened holding
/// `DATA[15:0]` — is a tap on a bus the design may not have, placed by a
/// reader who pressed Enter. One declared bus is not a sample; it is the only
/// answer. Anything else starts empty, with the shape of a declaration as the
/// field's hint.
fn sole_declared_bus(model: &Model) -> String {
    let mut declarations = model
        .schematic
        .document()
        .buses
        .iter()
        .filter_map(|bus| bus.declaration.as_ref().map(ToString::to_string))
        .collect::<Vec<_>>();
    declarations.sort();
    declarations.dedup();
    match declarations.as_slice() {
        [only] => only.clone(),
        _ => String::new(),
    }
}

pub fn placement_source<'a>(
    dialog: &'a BusTapDialogState,
    model: &Model,
) -> Result<&'a EditorRequestSource, String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let source = dialog.source.as_ref().ok_or_else(|| {
        "The bus-tap placement context is unavailable. Close and reopen Place bus tap.".to_owned()
    })?;
    if source != &crate::workbench::app::schematic_editor_request_source(model) {
        return Err(
            "The schematic source or editing scope changed. Close and reopen Place bus tap."
                .to_owned(),
        );
    }
    Ok(source)
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;
    #[cfg(test)]
    use crate::state::BusTapOrientation;
    #[cfg(test)]
    use crate::workbench::design::test_host::DesignTestHost;

    /// Draw the form as its dialog step does.
    pub fn render(dialog: &mut BusTapDialogState, model: &mut Model, ctx: &Context) {
        render_bus_tap_dialog(dialog, model, ctx);
    }

    #[cfg(test)]
    fn test_source() -> EditorRequestSource {
        crate::workbench::app::schematic_editor_request_source(&DesignTestHost::default())
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

    #[test]
    fn opening_captures_the_source_and_closing_releases_it() {
        let mut draft = BusTapDialogState::default();
        let source = test_source();
        draft.open(source.clone(), String::new());

        assert!(draft.open);
        assert_eq!(draft.source.as_ref(), Some(&source));
        assert_eq!(draft.fields.orientation, BusTapOrientation::Automatic);
        draft.close();
        assert!(draft.source.is_none());
        let mut next = source;
        next.document_epoch += 1;
        draft.open(next.clone(), String::new());
        assert_eq!(draft.source.as_ref(), Some(&next));
    }
}
