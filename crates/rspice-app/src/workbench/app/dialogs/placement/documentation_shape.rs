//! App-owned document authority and modal lifetime for shape placement.

use egui::Context;
use rspice_schematic_editor::annotations::shape_placement;

use crate::state::{PendingDocumentationShapePlacement, Tool};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::schematic_editor_request_source;
use crate::workbench::app_state::Model;

/// Isolated draft for the Draw documentation shape transaction.
#[derive(Debug, Clone, Default)]
pub(crate) struct DocumentationShapeDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DocumentationShapeKind,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
    pub(crate) expected_shapes: Vec<crate::state::DocumentationShape>,
}

impl DocumentationShapeDialogState {
    pub(crate) fn open(
        &mut self,
        source: rspice_schematic_editor::requests::EditorRequestSource,
        expected_shapes: Vec<crate::state::DocumentationShape>,
    ) {
        *self = Self {
            open: true,
            kind: crate::state::DocumentationShapeKind::Rectangle,
            source: Some(source),
            expected_shapes,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Draw documentation shape";
const PRIMARY: &str = "Draw";
const DESCRIPTION: &str = "Pick the shape, then drag it on the sheet.";

pub(in crate::workbench) fn render_documentation_shape_dialog(
    dialog: &mut DocumentationShapeDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }
    let validation = validate_draft(dialog, model);
    let refusal = validation.as_ref().err().cloned();

    let draft = &mut *dialog;
    let choice = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        .primary_enabled(validation.is_ok())
        .status(match refusal.as_deref() {
            Some(message) => Err(message),
            None => Ok(None),
        })
        .status_fixed_at_open()
        .status_lines(2)
        .show(ctx, |rows| shape_placement::show(rows, &mut draft.kind));
    match choice {
        DialogChoice::Primary => {
            if let Ok(pending) = validate_draft(dialog, model) {
                model.schematic.session.editor.pending_documentation_shape = Some(pending);
                model
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing
                    .clear();
                model.schematic.arm_tool(Tool::DocumentationShape);
                dialog.close();
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.close();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn validate_draft(
    draft: &DocumentationShapeDialogState,
    model: &Model,
) -> Result<PendingDocumentationShapePlacement, String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(source) = draft.source.as_ref() else {
        return Err("Reopen Draw documentation shape to capture the active schematic.".to_owned());
    };
    if source.design_epoch != model.design_execution_epoch {
        return Err(
            "The design document changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document_epoch != model.active_schematic_epoch {
        return Err(
            "The active schematic buffer changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    if source.topology_version != model.schematic.topology_version() {
        return Err(
            "The schematic topology changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document.display_path() != model.workspace.content.active_view.display_path() {
        return Err(
            "The active cell/view changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.expected_shapes != model.schematic.document().documentation_shapes {
        return Err(
            "The schematic graphics changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if *source != schematic_editor_request_source(model) {
        return Err(
            "The active schematic context changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    Ok(PendingDocumentationShapePlacement::new(
        draft.kind,
        source.topology_version,
        &draft.expected_shapes,
    )
    .with_source(source.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::AppState;

    /// The draft's validation, over what the form reads.
    fn validate_draft(state: &AppState) -> Result<PendingDocumentationShapePlacement, String> {
        super::validate_draft(&state.design.documentation_shape, &state.model)
    }

    #[test]
    fn arming_preserves_the_dialog_source_and_rejects_a_different_occurrence() {
        let mut state = AppState::default();
        assert!(validate_draft(&state).is_err());
        let master = crate::state::CellViewRef::new("work", "shape_child", "schematic");
        state.model.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            crate::state::ViewType::Schematic,
        );
        let source = schematic_editor_request_source(&state);
        state
            .design
            .documentation_shape
            .open(source.clone(), Vec::new());
        let Ok(pending) = validate_draft(&state) else {
            panic!("current dialog must arm the tool");
        };
        assert_eq!(pending.source.as_ref(), Some(&source));
        state.model.workspace.ascend_one().unwrap();
        state.model.workspace.descend_into(
            "X2".to_owned(),
            master,
            crate::state::ViewType::Schematic,
        );
        assert!(validate_draft(&state).is_err());
        assert_eq!(state.design.documentation_shape.source, Some(source));
        state.design.documentation_shape.close();
        assert!(state.design.documentation_shape.source.is_none());
        assert!(
            state
                .model
                .schematic
                .document()
                .documentation_shapes
                .is_empty()
        );
        assert!(!state.model.schematic.can_undo());
    }
}
