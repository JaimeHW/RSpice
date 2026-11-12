//! Taking a project design history step, and committing an inspector draft,
//! from the workbench.
//!
//! The model decides whether a step can be taken and applies its record. The
//! workbench brings the document the step restores to the front first, and
//! takes up the schematic the record put back before the model finishes the
//! step, so each happens where it did when the step was one operation. What
//! the record asks of the models and code features, a selected model whose
//! library is gone and a Verilog-A compile of a source no longer there, it
//! raises as requests, answered as the command that took the step returns.

use crate::state::CellViewRef;
use crate::workbench::app_state::design_history::{HistoryEffects, StagedProjectStep};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::framework::feature::ModelsPort;
use crate::workbench::framework::requests::{ClearSelectedModel, VoidVerilogAEvidence};
use crate::workbench::state::{InlineEditAuthority, WorkbenchSession};
use rspice_app_runs::workbench::slots::runs::RunsState;

use super::symbol_document::SymbolUiState;
use super::{
    SchematicRestoreParts, bump_active_schematic_epoch, restore_active_schematic_from_workspace,
};

impl AppState {
    /// [`undo_project_design`] for the session.
    pub(crate) fn undo_project_design(&mut self) -> Result<Option<String>, String> {
        undo_project_design(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &mut self.workbench,
            &self.runs,
            ModelsPort::selected_model(&*self.models),
        )
    }

    /// [`redo_project_design`] for the session.
    pub(crate) fn redo_project_design(&mut self) -> Result<Option<String>, String> {
        redo_project_design(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &mut self.workbench,
            &self.runs,
            ModelsPort::selected_model(&*self.models),
        )
    }

    /// [`activate_history_document`] for the session.
    #[cfg(test)]
    pub(crate) fn activate_history_document(&mut self, reference: &CellViewRef, operation: &str) {
        activate_history_document(
            &mut self.design.symbol,
            &mut self.model,
            &mut self.workbench,
            reference,
            operation,
        );
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

/// Undo the newest project design step, if one can be taken; `None` when
/// there is none to take.
// Application half of a model operation; it becomes a request.
pub(crate) fn undo_project_design(
    design: SchematicRestoreParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    runs: &RunsState,
    selected_model: Option<&str>,
) -> Result<Option<String>, String> {
    let Some(staged) = model.stage_project_undo()? else {
        return Ok(None);
    };
    take_project_step(design, model, wb, runs, selected_model, staged, "Undo").map(Some)
}

/// Redo the newest project design step undone, if one can be taken; `None`
/// when there is none to take.
// Application half of a model operation; it becomes a request.
pub(crate) fn redo_project_design(
    design: SchematicRestoreParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    runs: &RunsState,
    selected_model: Option<&str>,
) -> Result<Option<String>, String> {
    let Some(staged) = model.stage_project_redo()? else {
        return Ok(None);
    };
    take_project_step(design, model, wb, runs, selected_model, staged, "Redo").map(Some)
}

/// Take the staged step: its document to the front, its record applied, the
/// schematic it put back taken up and the requests it raises raised, even
/// when applying it failed part way, and then the model finishes it.
fn take_project_step(
    design: SchematicRestoreParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    runs: &RunsState,
    selected_model: Option<&str>,
    staged: StagedProjectStep,
    operation: &str,
) -> Result<String, String> {
    if let Some(destination) = staged.destination().cloned() {
        activate_history_document(&mut *design.symbol, model, wb, &destination, operation);
    }
    let mut effects = HistoryEffects::default();
    let applied = model.apply_project_step(staged, selected_model, &mut effects);
    let HistoryEffects {
        clear_selected_model,
        void_veriloga_evidence,
        restore_active_schematic,
    } = effects;
    if clear_selected_model {
        wb.outbox.push(ClearSelectedModel);
    }
    if void_veriloga_evidence {
        wb.outbox.push(VoidVerilogAEvidence);
    }
    if restore_active_schematic {
        restore_active_schematic_from_workspace(design, model, wb, runs);
    }
    let applied = applied?;
    model.finish_project_step(applied)
}

/// Bring the document a history step restores to the front: the editor
/// takes up the drawing the model swapped in, and the console says so.
pub(crate) fn activate_history_document(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    reference: &CellViewRef,
    operation: &str,
) {
    if model.activate_history_document(reference) {
        bump_active_schematic_epoch(symbol, model, wb);
        wb.prompt(format!(
            "{operation} switched to {} to restore it.",
            reference.display_path()
        ));
    }
}
