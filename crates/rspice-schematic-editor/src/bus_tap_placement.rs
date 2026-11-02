//! Bus-tap fields and validation over the canonical bus contract.
//!
//! The form asks two questions — which bus, and which of its bits — and shows
//! the conductors the answer resolves to. It draws no preview: the canvas
//! already draws the real tap under the pointer once the tool is armed, from
//! the same contract this form publishes.

use crate::session::bus::PendingBusTap;
use crate::vector_preview::deck_bits;
use rspice_design::schematic::bus::{BusDeclaration, BusSlice, BusTapOrientation};
use rspice_ui_kit::widgets::FormRows;

#[cfg(test)]
const BUS_EXAMPLE: &str = "DATA[15:0]";
/// The fields' placeholders: the shape of an answer, never one. A sample
/// name in the field read as a bus already chosen.
const BUS_SHAPE: &str = "name[msb:lsb]";
const SLICE_SHAPE: &str = "name[bit] or name[msb:lsb]";
const ORIENTATION_LABELS: [&str; 5] = ["Automatic", "Left", "Right", "Up", "Down"];

/// Editable bus-tap form fields, separate from the host modal and document
/// context.
#[derive(Debug, Clone, Default)]
pub struct BusTapFields {
    pub bus: String,
    pub slice: String,
    pub orientation: BusTapOrientation,
}

impl BusTapFields {
    pub fn validate(&self) -> DraftValidation {
        validate_draft(self)
    }
}

pub fn bus_field_id() -> egui::Id {
    field_id("Bus")
}

#[derive(Debug)]
pub enum DraftValidation {
    /// Nothing typed yet, or only half of it. The message says what is
    /// missing; nothing has been refused.
    Incomplete(&'static str),
    Invalid(String),
    Valid(PendingBusTap),
}

impl DraftValidation {
    pub fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    /// What the form's footer says: a refusal behind its mark, or, while Place
    /// is waiting for something, what it is waiting for, as a plain fact. An
    /// untouched form has refused nothing, but it still says why Place is off.
    pub fn status(&self) -> Result<Option<&str>, &str> {
        match self {
            Self::Incomplete(message) => Ok(Some(message)),
            Self::Invalid(message) => Err(message.as_str()),
            Self::Valid(_) => Ok(None),
        }
    }

    /// The conductors the drafted tap resolves to, spelled as the deck will
    /// spell them.
    ///
    /// Only a complete tap resolves, so a draft in progress never shows a
    /// conductor list the next keystroke invalidates. A tapped bit is named by
    /// the declaration's base and the member's own index, which is why a tap
    /// reads the same under either index order: `DATA[7:0]` and `DATA[0:7]`
    /// both resolve member 3 to `DATA#3`. The names come from the same speller
    /// the projection uses rather than being assembled again here.
    pub fn resolved_bits(&self) -> Option<String> {
        let Self::Valid(pending) = self else {
            return None;
        };
        Some(deck_bits(
            &pending.bus_declaration.name,
            pending
                .slice
                .members()
                .into_iter()
                .map(|member| member.index),
        ))
    }
}

/// The rows of the bus-tap form. Returns the control that takes the caret.
pub fn show(
    rows: &mut FormRows<'_>,
    validation: &DraftValidation,
    draft: &mut BusTapFields,
) -> Option<egui::Id> {
    let bus = rows.text("Bus", bus_field_id(), &mut draft.bus, BUS_SHAPE);
    rows.text("Slice", field_id("Slice"), &mut draft.slice, SLICE_SHAPE);
    // The conductors wrap rather than eliding: a tap on sixteen bits whose
    // middle is an ellipsis has not said which bits it takes.
    if let Some(bits) = validation.resolved_bits() {
        rows.derived_mono(&bits);
    }
    let options = ORIENTATION_LABELS.map(str::to_owned);
    if let Some(index) = rows.select(
        "Orientation",
        "bus-tap-orientation",
        orientation_label(draft.orientation),
        &options,
    ) {
        draft.orientation = orientation_at(index);
    }
    Some(bus.id)
}

fn validate_draft(draft: &BusTapFields) -> DraftValidation {
    let bus = draft.bus.trim();
    let slice = draft.slice.trim();
    if bus.is_empty() && slice.is_empty() {
        return DraftValidation::Incomplete("Enter the bus and the slice to tap.");
    }
    if bus.is_empty() {
        return DraftValidation::Incomplete("Enter the declared bus, for example DATA[15:0].");
    }
    if slice.is_empty() {
        return DraftValidation::Incomplete("Enter the tap slice, for example DATA[7:0].");
    }

    let declaration = match BusDeclaration::parse(bus) {
        Ok(declaration) => declaration,
        Err(error) => return DraftValidation::Invalid(format!("Bus: {error}")),
    };
    let slice = match BusSlice::parse(slice) {
        Ok(slice) => slice,
        Err(error) => return DraftValidation::Invalid(format!("Slice: {error}")),
    };
    if let Err(error) = declaration.validate_slice(&slice) {
        return DraftValidation::Invalid(error.to_string());
    }
    match PendingBusTap::new(declaration, slice, draft.orientation) {
        Ok(pending) => DraftValidation::Valid(pending),
        Err(error) => DraftValidation::Invalid(error.to_string()),
    }
}

fn field_id(label: &str) -> egui::Id {
    egui::Id::new(("rspice.bus-tap", label))
}

const fn orientation_label(orientation: BusTapOrientation) -> &'static str {
    match orientation {
        BusTapOrientation::Automatic => "Automatic",
        BusTapOrientation::Left => "Left",
        BusTapOrientation::Right => "Right",
        BusTapOrientation::Up => "Up",
        BusTapOrientation::Down => "Down",
    }
}

const fn orientation_at(index: usize) -> BusTapOrientation {
    match index {
        1 => BusTapOrientation::Left,
        2 => BusTapOrientation::Right,
        3 => BusTapOrientation::Up,
        4 => BusTapOrientation::Down,
        _ => BusTapOrientation::Automatic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_drafts_never_publish_a_pending_configuration() {
        let mut draft = BusTapFields::default();
        assert!(!validate_draft(&draft).can_commit());
        draft.bus = BUS_EXAMPLE.to_owned();
        assert!(!validate_draft(&draft).can_commit());
    }

    /// A placeholder is the shape of an answer: one that parses is an answer,
    /// and reads as a bus already chosen.
    #[test]
    fn the_placeholders_are_shapes_not_answers() {
        assert!(BusDeclaration::parse(BUS_SHAPE).is_err());
        assert!(BusSlice::parse(SLICE_SHAPE).is_err());
    }

    /// An untouched form has not been refused, but it says why Place is off;
    /// a half draft says what is missing.
    #[test]
    fn an_empty_draft_states_what_place_waits_for_and_a_half_draft_what_is_missing() {
        let mut draft = BusTapFields::default();
        assert_eq!(
            validate_draft(&draft).status(),
            Ok(Some("Enter the bus and the slice to tap."))
        );
        draft.bus = BUS_EXAMPLE.to_owned();
        assert_eq!(
            validate_draft(&draft).status(),
            Ok(Some("Enter the tap slice, for example DATA[7:0]."))
        );
        draft.slice = "ADDR[7:0]".to_owned();
        assert!(validate_draft(&draft).status().is_err());
    }

    #[test]
    fn valid_scalar_and_slice_targets_are_accepted() {
        for slice in ["DATA[7]", "DATA[7:0]"] {
            let draft = BusTapFields {
                bus: BUS_EXAMPLE.to_owned(),
                slice: slice.to_owned(),
                orientation: BusTapOrientation::Automatic,
            };
            assert!(validate_draft(&draft).can_commit(), "{slice}");
        }
    }

    #[test]
    fn base_and_out_of_range_selectors_are_rejected_live() {
        for slice in ["ADDR[7:0]", "DATA[16]", "DATA[16:8]"] {
            let draft = BusTapFields {
                bus: BUS_EXAMPLE.to_owned(),
                slice: slice.to_owned(),
                orientation: BusTapOrientation::Automatic,
            };
            assert!(!validate_draft(&draft).can_commit(), "{slice}");
        }
    }

    #[test]
    fn every_orientation_round_trips_through_the_select_projection() {
        for (index, label) in ORIENTATION_LABELS.iter().enumerate() {
            assert_eq!(orientation_label(orientation_at(index)), *label);
        }
    }

    /// A tap names a conductor by the member's own index, so the two ways of
    /// writing the same bus agree bit for bit — which is the property the deck
    /// depends on, because both drawings must reach the engine as one node.
    ///
    /// The line states the conductors and nothing else: the bus and the slice
    /// are in the two fields directly above it.
    #[test]
    fn a_tap_resolves_to_the_same_deck_bit_under_either_index_order() {
        for bus in ["DATA[7:0]", "DATA[0:7]"] {
            let draft = BusTapFields {
                bus: bus.to_owned(),
                slice: "DATA[3]".to_owned(),
                orientation: BusTapOrientation::Automatic,
            };
            assert_eq!(
                validate_draft(&draft).resolved_bits().as_deref(),
                Some("DATA#3")
            );
        }

        // A slice resolves to every conductor it selects, in declaration order.
        let draft = BusTapFields {
            bus: BUS_EXAMPLE.to_owned(),
            slice: "DATA[3:0]".to_owned(),
            orientation: BusTapOrientation::Automatic,
        };
        assert_eq!(
            validate_draft(&draft).resolved_bits().as_deref(),
            Some("DATA#3 DATA#2 DATA#1 DATA#0")
        );

        // Nothing resolves until the whole tap does: a selector outside the
        // declared range names no conductor rather than a plausible one.
        for slice in ["DATA[16]", "ADDR[3]", ""] {
            let draft = BusTapFields {
                bus: BUS_EXAMPLE.to_owned(),
                slice: slice.to_owned(),
                orientation: BusTapOrientation::Automatic,
            };
            assert_eq!(validate_draft(&draft).resolved_bits(), None, "{slice}");
        }
    }
}
