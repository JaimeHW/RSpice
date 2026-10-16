//! App-owned document authority and modal lifetime for shape placement.

use egui::Context;
use rspice_schematic_editor::annotations::shape_placement;

use crate::state::{PendingDocumentationShapePlacement, Tool};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::{RSpiceApp, schematic_editor_request_source};
use crate::workbench::app_state::AppState;

const TITLE: &str = "Draw documentation shape";
const PRIMARY: &str = "Draw";
const DESCRIPTION: &str = "Pick the shape, then drag it on the sheet.";

impl RSpiceApp {
    pub(in crate::workbench) fn render_documentation_shape_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.documentation_shape.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let refusal = validation.as_ref().err().cloned();

        let draft = &mut self.state.dialogs.documentation_shape;
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
                if let Ok(pending) = validate_draft(&self.state) {
                    self.state
                        .schematic
                        .session
                        .editor
                        .pending_documentation_shape = Some(pending);
                    self.state
                        .schematic
                        .session
                        .editor
                        .documentation_shape_drawing
                        .clear();
                    self.state.schematic.arm_tool(Tool::DocumentationShape);
                    self.state.dialogs.documentation_shape.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.documentation_shape.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> Result<PendingDocumentationShapePlacement, String> {
    let draft = &state.dialogs.documentation_shape;
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(source) = draft.source.as_ref() else {
        return Err("Reopen Draw documentation shape to capture the active schematic.".to_owned());
    };
    if source.design_epoch != state.design_execution_epoch {
        return Err(
            "The design document changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document_epoch != state.active_schematic_epoch {
        return Err(
            "The active schematic buffer changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    if source.topology_version != state.schematic.topology_version() {
        return Err(
            "The schematic topology changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document.display_path() != state.workspace.content.active_view.display_path() {
        return Err(
            "The active cell/view changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.expected_shapes != state.schematic.document().documentation_shapes {
        return Err(
            "The schematic graphics changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if *source != schematic_editor_request_source(state) {
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

    #[test]
    fn arming_preserves_the_dialog_source_and_rejects_a_different_occurrence() {
        let mut state = AppState::default();
        assert!(validate_draft(&state).is_err());
        let master = crate::state::CellViewRef::new("work", "shape_child", "schematic");
        state.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            crate::state::ViewType::Schematic,
        );
        let source = schematic_editor_request_source(&state);
        state
            .dialogs
            .documentation_shape
            .open(source.clone(), Vec::new());
        let Ok(pending) = validate_draft(&state) else {
            panic!("current dialog must arm the tool");
        };
        assert_eq!(pending.source.as_ref(), Some(&source));
        state.workspace.ascend_one().unwrap();
        state
            .workspace
            .descend_into("X2".to_owned(), master, crate::state::ViewType::Schematic);
        assert!(validate_draft(&state).is_err());
        assert_eq!(state.dialogs.documentation_shape.source, Some(source));
        state.dialogs.documentation_shape.close();
        assert!(state.dialogs.documentation_shape.source.is_none());
        assert!(state.schematic.document().documentation_shapes.is_empty());
        assert!(!state.schematic.can_undo());
    }
}
