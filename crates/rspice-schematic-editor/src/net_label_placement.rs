//! Net-label and off-sheet connector fields, entered at the click.
//!
//! The click already said where, on which sheet, and — through the armed tool
//! — which of the two objects is being placed. What is left is the name, and
//! for a connector the direction it carries across sheets.

use rspice_design::schematic::net_label::NetLabelKind;
use rspice_ui_kit::widgets::CanvasEntryRows;

const FIELD_ID: &str = "place-net-label-name";
const DIRECTION_FIELD_ID: &str = "place-net-label-direction";
const DIRECTION_LABEL: &str = "Direction";
const NAME_HINT: &str = "vout or DATA[7]";
/// The one fact the document's naming policy adds to what the reader is
/// typing. The rest of the policy speaks through the refusal it causes.
const NAME_TOOLTIP: &str = "Names are case-sensitive.";

pub fn name_id() -> egui::Id {
    egui::Id::new(FIELD_ID)
}

/// The rows of the entry. Returns the control that takes the caret.
pub fn show(
    rows: &mut CanvasEntryRows<'_>,
    name: &mut String,
    kind: &mut NetLabelKind,
) -> Option<egui::Id> {
    let response = rows
        .text("Name", name_id(), name, NAME_HINT)
        .on_hover_text(NAME_TOOLTIP);
    if let Some(selected) = kind.off_sheet_direction() {
        let options = NetLabelKind::DIRECTIONS
            .map(|direction| NetLabelKind::direction_label(direction).to_owned());
        if let Some(index) = rows.select(
            DIRECTION_LABEL,
            DIRECTION_FIELD_ID,
            NetLabelKind::direction_label(selected),
            &options,
        ) {
            *kind = NetLabelKind::OffSheet {
                direction: NetLabelKind::DIRECTIONS[index],
            };
        }
    }
    Some(response.id)
}
