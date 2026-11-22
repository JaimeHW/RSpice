//! App-owned document authority for shape placement: each Place, Shape
//! command arms its shape at once, and the shape is drawn by a drag.

use crate::state::{DocumentationShapeKind, PendingDocumentationShapePlacement, Tool};

use crate::workbench::app::schematic_editor_request_source;
use crate::workbench::app_state::Model;

/// Arm the documentation-shape tool for `kind` on the active schematic.
///
/// Each shape is its own command (Place ▸ Shape ▸ Rectangle, Line, Polygon,
/// Arc, Callout), so nothing is asked first: the document authority is
/// captured here, and the tool is armed at once. A read-only schematic arms
/// nothing; the refusal is the caller's to state.
pub(crate) fn arm_documentation_shape(
    model: &mut Model,
    kind: DocumentationShapeKind,
) -> Result<(), &'static str> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.");
    }
    let source = schematic_editor_request_source(model);
    let expected = model.schematic.document().documentation_shapes.clone();
    let pending = PendingDocumentationShapePlacement::new(kind, source.topology_version, &expected)
        .with_source(source);
    model.schematic.session.editor.pending_documentation_shape = Some(pending);
    model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .clear();
    model.schematic.arm_tool(Tool::DocumentationShape);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::design::test_host::DesignTestHost;

    /// Each shape arms its tool at once, and nothing is drawn until the
    /// reader drags on the sheet.
    #[test]
    fn each_shape_arms_its_tool_without_a_dialog() {
        for kind in DocumentationShapeKind::ALL {
            let mut state = DesignTestHost::default();
            assert_eq!(arm_documentation_shape(&mut state.model, kind), Ok(()));
            assert_eq!(
                state.model.schematic.session.editor.tool,
                Tool::DocumentationShape
            );
            let pending = state
                .model
                .schematic
                .session
                .editor
                .pending_documentation_shape
                .as_ref()
                .expect("the shape is pending");
            assert_eq!(pending.kind, kind);
            assert_eq!(
                pending.source.as_ref(),
                Some(&schematic_editor_request_source(&state))
            );
            assert!(
                state
                    .model
                    .schematic
                    .document()
                    .documentation_shapes
                    .is_empty()
            );
        }
    }

    /// A read-only schematic arms nothing and says why.
    #[test]
    fn a_read_only_schematic_arms_no_shape() {
        let mut state = DesignTestHost::default();
        state.model.schematic.session.read_only = true;
        assert_eq!(
            arm_documentation_shape(&mut state.model, DocumentationShapeKind::Rectangle),
            Err("The active schematic is read-only.")
        );
        assert_ne!(
            state.model.schematic.session.editor.tool,
            Tool::DocumentationShape
        );
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .pending_documentation_shape
                .is_none()
        );
    }
}
