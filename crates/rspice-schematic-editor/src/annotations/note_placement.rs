//! Design-note placement fields.
//!
//! Two questions: which kind of note, and what it says. The canvas draws the
//! note itself once the tool is armed, so the form draws no second copy of it.

use rspice_design::schematic::design_note::DesignNoteKind;
use rspice_ui_kit::widgets::FormRows;

/// Three lines: enough for a note worth placing, short enough that the form
/// stays a card.
const TEXT_LINES: usize = 3;

pub fn text_id() -> egui::Id {
    egui::Id::new(("rspice.design-note", "text"))
}

/// The rows of the design-note form. Returns the control that takes the caret.
pub fn show(
    rows: &mut FormRows<'_>,
    kind: &mut DesignNoteKind,
    text: &mut String,
) -> Option<egui::Id> {
    let labels = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
    if let Some(index) = rows.select("Type", "design-note-kind", kind.label(), &labels) {
        *kind = DesignNoteKind::ALL[index];
    }
    let response = rows.text_multiline("Text", text_id(), text, sample(*kind), TEXT_LINES);
    Some(response.id)
}

/// The shape of a note of this kind, as the field's hint.
///
/// It is a hint and never a value: the form used to open with `Bias network`
/// written into the field, so an untouched form placed a note saying that.
const fn sample(kind: DesignNoteKind) -> &'static str {
    match kind {
        DesignNoteKind::PlainText => "Bias network",
        DesignNoteKind::PropertyDisplay => "${component_count} components",
        DesignNoteKind::RequirementLink => "REQ-19",
        DesignNoteKind::ReviewNote => "Check the bias network",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every kind offers a shape to type over, and none of them is a value.
    #[test]
    fn every_kind_has_a_hint_and_none_of_them_is_a_default_value() {
        for kind in DesignNoteKind::ALL {
            assert!(!sample(kind).is_empty(), "{kind:?}");
        }
    }
}
