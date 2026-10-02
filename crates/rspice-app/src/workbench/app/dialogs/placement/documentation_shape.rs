//! App-owned document authority and modal lifetime for shape placement.

use egui::Context;
use rspice_schematic_editor::annotations::shape_placement;

use crate::state::{PendingDocumentationShapePlacement, Tool};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use crate::workbench::app::{RSpiceApp, schematic_editor_request_source};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} GRAPHICS";
const TITLE: &str = "Draw documentation shape";
const PRIMARY: &str = "Arm shape tool";
const DESCRIPTION: &str =
    "Draw lines, rectangles, polygons, arcs, or callouts on non-electrical documentation layers.";
const DIALOG_SIZE: DialogSize = DialogSize::Transaction;
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No schematic graphics have been changed.";

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid(PendingDocumentationShapePlacement),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) => None,
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_documentation_shape_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.documentation_shape.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let discard_confirm = self.state.dialogs.documentation_shape.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DIALOG_SIZE)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let mut response = dialog.show_transaction(ctx, |ui| {
            let draft = &mut self.state.dialogs.documentation_shape;
            let response =
                shape_placement::show(ui, validation_message.as_deref(), &mut draft.kind);
            if response.edited {
                draft.mark_edited();
            }
            response.focus
        });
        match response.choice {
            DialogChoice::Primary => {
                if let DraftValidation::Valid(pending) = validate_draft(&self.state) {
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
                self.state.dialogs.documentation_shape.attempt_close();
                if self.state.dialogs.documentation_shape.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.documentation_shape;
    if state.schematic_edit_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    let Some(source) = draft.source.as_ref() else {
        return DraftValidation::Invalid(
            "Reopen Draw documentation shape to capture the active schematic.".to_owned(),
        );
    };
    if source.design_epoch != state.design_execution_epoch {
        return DraftValidation::Invalid(
            "The design document changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document_epoch != state.active_schematic_epoch {
        return DraftValidation::Invalid(
            "The active schematic buffer changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    if source.topology_version != state.schematic.topology_version() {
        return DraftValidation::Invalid(
            "The schematic topology changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if source.document.display_path() != state.workspace.content.active_view.display_path() {
        return DraftValidation::Invalid(
            "The active cell/view changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.expected_shapes != state.schematic.document().documentation_shapes {
        return DraftValidation::Invalid(
            "The schematic graphics changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if *source != schematic_editor_request_source(state) {
        return DraftValidation::Invalid(
            "The active schematic context changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    DraftValidation::Valid(
        PendingDocumentationShapePlacement::new(
            draft.kind,
            source.topology_version,
            &draft.expected_shapes,
        )
        .with_source(source.clone()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arming_preserves_the_dialog_source_and_rejects_a_different_occurrence() {
        let mut state = AppState::default();
        assert!(!validate_draft(&state).can_commit());
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
        let DraftValidation::Valid(pending) = validate_draft(&state) else {
            panic!("current dialog must arm the tool");
        };
        assert_eq!(pending.source.as_ref(), Some(&source));
        state.workspace.ascend_one().unwrap();
        state
            .workspace
            .descend_into("X2".to_owned(), master, crate::state::ViewType::Schematic);
        assert!(!validate_draft(&state).can_commit());
        assert_eq!(state.dialogs.documentation_shape.source, Some(source));
        state.dialogs.documentation_shape.close();
        assert!(state.dialogs.documentation_shape.source.is_none());
        assert!(state.schematic.document().documentation_shapes.is_empty());
        assert!(!state.schematic.can_undo());
    }
}
