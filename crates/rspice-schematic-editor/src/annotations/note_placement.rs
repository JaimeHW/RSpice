//! Design-note placement fields.
//!
//! Which kind of note, what it says, and how it is lettered. The canvas draws
//! the note itself once the tool is armed, so the form draws no second copy
//! of it.

use rspice_design::schematic::design_note::{
    DesignNoteField, DesignNoteKind, SymbolTextAlign, name_property_field, named_property_field,
};
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
    size_percent: &mut u16,
    align: &mut SymbolTextAlign,
) -> Option<egui::Id> {
    let labels = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
    if let Some(index) = rows.select("Type", "design-note-kind", kind.label(), &labels) {
        *kind = DesignNoteKind::ALL[index];
    }
    if *kind == DesignNoteKind::PropertyDisplay {
        field_row(rows, text);
    }
    let response = rows.text_multiline("Text", text_id(), text, sample(*kind), TEXT_LINES);
    crate::object_properties::note_lettering_rows(rows, size_percent, align);
    Some(response.id)
}

/// A Property display note's Field row: the field its text names, picked
/// from the list of every field it can show. Picking one writes it into the
/// text, where the words around it can be typed. Returns `true` when the
/// text changed.
pub fn field_row(rows: &mut FormRows<'_>, text: &mut String) -> bool {
    let fields = DesignNoteField::all().collect::<Vec<_>>();
    let labels = fields
        .iter()
        .map(|field| field.label().to_owned())
        .collect::<Vec<_>>();
    let shown = named_property_field(text).map_or("Choose a field", DesignNoteField::label);
    let Some(index) = rows.select("Field", "design-note-field", shown, &labels) else {
        return false;
    };
    let named = name_property_field(text, fields[index]);
    let changed = named != *text;
    *text = named;
    changed
}

/// The shape of a note of this kind, as the field's hint.
///
/// It is a hint and never a value: the form used to open with `Bias network`
/// written into the field, so an untouched form placed a note saying that.
/// Nor is it a sample note, which reads as text someone wrote: prose takes no
/// hint, and only the kinds with a syntax of their own show it.
const fn sample(kind: DesignNoteKind) -> &'static str {
    match kind {
        // A Property display note's field is picked in the row above, and
        // the text takes any words around it.
        DesignNoteKind::PlainText
        | DesignNoteKind::ReviewNote
        | DesignNoteKind::PropertyDisplay => "",
        DesignNoteKind::RequirementLink => "Requirement ID",
        DesignNoteKind::SpiceDirective => ".param, .model, .ic, .include ...",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No hint is a sample note: prose takes none, and a kind with a syntax
    /// of its own shows the syntax, not an instance of it someone could have
    /// written.
    #[test]
    fn no_hint_is_a_sample_note() {
        assert_eq!(sample(DesignNoteKind::PlainText), "");
        assert_eq!(sample(DesignNoteKind::ReviewNote), "");
        for kind in DesignNoteKind::ALL {
            let hint = sample(kind);
            assert!(!hint.contains("bias network"), "{kind:?}: {hint}");
            assert!(!hint.contains("Bias network"), "{kind:?}: {hint}");
            assert!(
                !hint.chars().any(|c| c.is_ascii_digit()),
                "{kind:?}: {hint}"
            );
        }
    }
}
