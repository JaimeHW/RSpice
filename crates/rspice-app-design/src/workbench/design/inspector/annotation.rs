//! The editable rows of a documentation shape and a note.
//!
//! A shape's stroke, dash and fill, and a note's size and alignment, take
//! the symbol editor's words, so what is drawn on a sheet and what is drawn
//! in a symbol are styled alike. Each change is one undo step.

use super::*;
use rspice_design::schematic::design_note::{
    DESIGN_NOTE_SIZE_CHOICES, DesignNote, SymbolTextAlign, design_note_size_label,
};
use rspice_design::schematic::documentation_shape::{
    DocumentationShape, SymbolDash, SymbolFill, SymbolStroke,
};

/// One choice row: `all` named by `name`, with `current` chosen. Returns the
/// choice picked this frame when it differs.
fn choice_row<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    id: (&str, u64),
    all: &[T],
    name: fn(T) -> &'static str,
    current: T,
    enabled: bool,
) -> Option<T> {
    let options = all
        .iter()
        .map(|item| (name(*item).to_owned(), name(*item).to_owned()))
        .collect::<Vec<_>>();
    let mut selected = name(current).to_owned();
    if !property_row_combo(ui, label, id, &mut selected, &options, enabled) {
        return None;
    }
    all.iter()
        .copied()
        .find(|item| name(*item) == selected)
        .filter(|item| *item != current)
}

/// Stroke, Dash and, on a closed shape, Fill.
pub(super) fn shape_style_rows(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    shape: &DocumentationShape,
) {
    let writable = !cx.model.schematic_edit_read_only();
    let mut style = shape.style;
    if let Some(stroke) = choice_row(
        ui,
        "Stroke",
        ("shape-stroke", shape.id),
        &SymbolStroke::ALL,
        SymbolStroke::label,
        style.stroke,
        writable,
    ) {
        style.stroke = stroke;
    }
    if let Some(dash) = choice_row(
        ui,
        "Dash",
        ("shape-dash", shape.id),
        &SymbolDash::ALL,
        SymbolDash::label,
        style.dash,
        writable,
    ) {
        style.dash = dash;
    }
    if shape.kind().is_closed()
        && let Some(fill) = choice_row(
            ui,
            "Fill",
            ("shape-fill", shape.id),
            &SymbolFill::ALL,
            SymbolFill::label,
            style.fill,
            writable,
        )
    {
        style.fill = fill;
    }
    if style == shape.style {
        return;
    }
    match cx.model.schematic.edit_documentation_shape_properties(
        shape.clone(),
        shape.geometry.clone(),
        style,
    ) {
        Ok(true) => sync_active_schematic(cx.model, cx.wb),
        Ok(false) => {}
        Err(error) => cx.wb.prompt(format!("Shape not changed: {error}")),
    }
}

/// Size and Align.
pub(super) fn note_lettering_rows(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, note: &DesignNote) {
    let writable = !cx.model.schematic_edit_read_only();
    let current = note.text_size_percent;
    let mut sizes = DESIGN_NOTE_SIZE_CHOICES
        .iter()
        .map(|percent| (percent.to_string(), design_note_size_label(*percent)))
        .collect::<Vec<_>>();
    if !DESIGN_NOTE_SIZE_CHOICES.contains(&current) {
        // A size a drawing brought in keeps its place in the list.
        sizes.push((current.to_string(), design_note_size_label(current)));
    }
    let mut selected = current.to_string();
    let mut size_percent = current;
    if property_row_combo(
        ui,
        "Size",
        ("note-size", note.id),
        &mut selected,
        &sizes,
        writable,
    ) && let Ok(picked) = selected.parse::<u16>()
    {
        size_percent = picked;
    }
    let align = choice_row(
        ui,
        "Align",
        ("note-align", note.id),
        &SymbolTextAlign::ALL,
        SymbolTextAlign::label,
        note.alignment(),
        writable,
    )
    .unwrap_or(note.alignment());
    if size_percent == current && align == note.alignment() {
        return;
    }
    match cx.model.schematic.edit_design_note_properties(
        note.clone(),
        note.kind,
        note.text.clone(),
        note.review.as_ref().map(|review| review.state),
        (size_percent, align),
    ) {
        Ok(true) => sync_active_schematic(cx.model, cx.wb),
        Ok(false) => {}
        Err(error) => cx.wb.prompt(format!("Note not changed: {error}")),
    }
}
