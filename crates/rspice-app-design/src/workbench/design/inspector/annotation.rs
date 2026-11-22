//! The editable rows of a documentation shape and a note.
//!
//! A shape's stroke, dash and fill take the symbol editor's words, so a box
//! drawn on a sheet and a box drawn in a symbol are styled alike. Each change
//! is one undo step.

use super::*;
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
