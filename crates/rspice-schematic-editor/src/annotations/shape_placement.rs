//! Documentation-shape placement: which shape the draw tool draws.

use rspice_design::schematic::documentation_shape::DocumentationShapeKind;
use rspice_ui_kit::widgets::FormRows;

/// The one row of the shape form. Returns the control that takes the caret.
pub fn show(rows: &mut FormRows<'_>, kind: &mut DocumentationShapeKind) -> Option<egui::Id> {
    let labels = DocumentationShapeKind::ALL.map(|kind| kind.label().to_owned());
    let output =
        rows.select_with_response("Shape", "documentation-shape-kind", kind.label(), &labels);
    if let Some(index) = output.picked {
        *kind = DocumentationShapeKind::ALL[index];
    }
    Some(output.response.id)
}
