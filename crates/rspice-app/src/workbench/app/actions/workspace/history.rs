//! Taking a project design history step, and committing an inspector draft,
//! from the workbench.
//!
//! The model decides whether a step can be taken and applies its record. The
//! workbench brings the document the step restores to the front first, and
//! does what applying the record asked of it before the model finishes the
//! step, so each happens where it did when the step was one operation.

use crate::state::CellViewRef;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_history::{HistoryEffects, StagedProjectStep};
use crate::workbench::state::InlineEditAuthority;

impl AppState {
    // Application half of a model operation; it becomes a request.
    pub(crate) fn undo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(staged) = self.model.stage_project_undo()? else {
            return Ok(None);
        };
        self.take_project_step(staged, "Undo").map(Some)
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn redo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(staged) = self.model.stage_project_redo()? else {
            return Ok(None);
        };
        self.take_project_step(staged, "Redo").map(Some)
    }

    fn take_project_step(
        &mut self,
        staged: StagedProjectStep,
        operation: &str,
    ) -> Result<String, String> {
        if let Some(destination) = staged.destination().cloned() {
            self.activate_history_document(&destination, operation);
        }
        let selected_model = self.models.selected_model.clone();
        let mut effects = HistoryEffects::default();
        let applied =
            self.model
                .apply_project_step(staged, selected_model.as_deref(), &mut effects);
        self.apply_history_effects(effects);
        let applied = applied?;
        self.model.finish_project_step(applied)
    }

    /// Bring the document a history step restores to the front: the editor
    /// takes up the drawing the model swapped in, and the console says so.
    pub(in crate::workbench) fn activate_history_document(
        &mut self,
        reference: &CellViewRef,
        operation: &str,
    ) {
        if self.model.activate_history_document(reference) {
            self.bump_active_schematic_epoch();
            self.prompt(format!(
                "{operation} switched to {} to restore it.",
                reference.display_path()
            ));
        }
    }

    /// Do what applying a history step's record asked of the workbench.
    fn apply_history_effects(&mut self, effects: HistoryEffects) {
        let HistoryEffects {
            clear_selected_model,
            void_veriloga_evidence,
            restore_active_schematic,
        } = effects;
        if clear_selected_model {
            self.models.selected_model = None;
        }
        if void_veriloga_evidence {
            self.code.code_workspace.veriloga.void_evidence();
        }
        if restore_active_schematic {
            self.restore_active_schematic_from_workspace();
        }
    }

    pub(crate) fn inline_edit_authority(&self) -> InlineEditAuthority {
        InlineEditAuthority {
            project: self.model.workspace.content.project.id(),
            document: self.model.workspace.content.active_schematic_reference(),
            occurrence: self.model.workspace.content.active_occurrence().cloned(),
            design_epoch: self.model.design_execution_epoch,
            document_epoch: self.model.active_schematic_epoch,
        }
    }

    /// Used by focus changes, document navigation, and commands before they
    /// read or replace the design. A failed commit retains the complete draft.
    pub(crate) fn commit_inline_component_edit(&mut self) -> Result<bool, String> {
        let Some(session) = self.workbench.inline_edit.session().cloned() else {
            return Ok(false);
        };
        if session.candidate.as_ref() == Some(&session.expected) {
            self.workbench.inline_edit.end();
            return Ok(false);
        }
        let result = if session.authority != self.inline_edit_authority() {
            Err("The document or hierarchy occurrence changed. Cancel this inspector draft and edit the current component.".to_owned())
        } else if let Some(candidate) = session.candidate {
            self.model.edit_component_transaction(
                &session.expected,
                candidate,
                &session.description,
            )
        } else {
            Err(session
                .error
                .unwrap_or_else(|| "Correct the inspector field before applying it.".to_owned()))
        };
        match &result {
            Ok(_) => self.workbench.inline_edit.end(),
            Err(error) => self.workbench.inline_edit.set_error(Some(error.clone())),
        }
        result
    }

    /// A command/navigation boundary reports a refusal once and leaves the
    /// inspector's draft available for correction or explicit cancellation.
    pub(crate) fn commit_pending_inspector_edit(&mut self) -> bool {
        match self.commit_inline_component_edit() {
            Ok(_) => true,
            Err(error) => {
                self.prompt(format!("Inspector edit was not applied: {error}"));
                false
            }
        }
    }
}
