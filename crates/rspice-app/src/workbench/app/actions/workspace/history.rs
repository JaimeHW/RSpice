//! Taking a history step, and committing an inspector draft, from the
//! workbench.
//!
//! Undo and Redo take a step in the same five parts whoever takes it: the
//! design feature everywhere the shell lets the key through, and the shell
//! itself on the netlist page, whose own history it steps over the whole
//! application.
//!
//! The model decides whether a step can be taken and applies its record. The
//! workbench brings the document the step restores to the front first, and
//! takes up the schematic the record put back before the model finishes the
//! step, so each happens where it did when the step was one operation. What
//! the record asks of the models and code features, a selected model whose
//! library is gone and a Verilog-A compile of a source no longer there, it
//! raises as requests, answered as the command that took the step returns.

use crate::state::{CellViewRef, UndoSequence, ViewType};
use crate::workbench::app::actions::stimulus;
use crate::workbench::app_state::design_history::{HistoryEffects, StagedProjectStep};
use crate::workbench::app_state::{AppState, Model};
#[cfg(test)]
use crate::workbench::framework::feature::ModelsPort;
use crate::workbench::framework::requests::{ClearSelectedModel, VoidVerilogAEvidence};
use crate::workbench::state::{
    InlineEditAuthority, StimulusEditorState, WorkbenchSession, Workspace,
};
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;

use super::symbol_document::{
    SymbolUiState, redo_active_symbol_document, undo_active_symbol_document,
};
use super::waivers::restate_waived_findings;
use super::{
    SchematicRestoreParts, bump_active_schematic_epoch, restore_active_schematic_from_workspace,
    sync_active_schematic,
};

impl AppState {
    /// [`undo_project_design`] for the session.
    #[cfg(test)]
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
    #[cfg(test)]
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

/// Which way a history step goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HistoryDirection {
    Undo,
    Redo,
}

impl HistoryDirection {
    /// The command's name, as a refusal names the step it did not take.
    fn label(self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
        }
    }

    /// The step under way, as a refusal names it.
    fn gerund(self) -> &'static str {
        match self {
            Self::Undo => "undoing",
            Self::Redo => "redoing",
        }
    }
}

/// What a history step reads and writes of the design feature's state.
pub(crate) struct DesignHistoryParts<'a> {
    /// Whether a schematic edit is under way, which refuses the step.
    pub(crate) schematic_edit_in_progress: bool,
    /// The stimulus definition selected on the Stimulus workspace.
    pub(crate) selected_stimulus_definition: Option<&'a str>,
    /// The stimulus draft editor, whose own history that workspace steps
    /// first.
    pub(crate) stimulus_editor: &'a mut StimulusEditorState,
    /// What taking up a restored schematic reads and writes, the symbol
    /// editor's state among it.
    pub(crate) restore: SchematicRestoreParts<'a>,
}

/// What a history step reads and writes, lent afresh for each of its parts
/// so that the step only the shell takes, the netlist page's, can be handed
/// the whole application in between.
pub(crate) struct HistoryStepParts<'a> {
    pub(crate) design: DesignHistoryParts<'a>,
    pub(crate) model: &'a mut Model,
    pub(crate) wb: &'a mut WorkbenchSession,
    pub(crate) runs: &'a mut RunsState,
    pub(crate) run_controller: &'a mut SimulationController,
    /// The model the models feature has selected.
    pub(crate) selected_model: Option<&'a str>,
}

/// The netlist page's own history step `direction`, which the shell takes
/// over the whole application; `Ok(None)` when that history holds no step
/// that way.
pub(crate) type NetlistHistoryStep<H> =
    fn(&mut H, HistoryDirection) -> Result<Option<String>, String>;

/// Take a history step `direction`, in the same five parts whoever takes
/// it: refused while a schematic edit is under way, as a step taken then
/// would be folded into that edit's undo record when it lands; the stored
/// waivers read; the step taken; the published findings stated again where
/// it changed the waivers; and a run prepared or checked under the old
/// waivers dropped where a schematic's changed, as a waiver recorded from
/// the Problems page drops it.
pub(crate) fn take_history_step<H>(
    host: &mut H,
    lend: fn(&mut H) -> HistoryStepParts<'_>,
    netlist: Option<NetlistHistoryStep<H>>,
    direction: HistoryDirection,
) {
    {
        let parts = lend(host);
        if parts.design.schematic_edit_in_progress {
            parts.wb.prompt(format!(
                "Finish or cancel the edit in progress before {}.",
                direction.gerund()
            ));
            return;
        }
    }
    let waivers = lend(host).model.stored_waivers();
    take_step(host, lend, netlist, direction);
    let parts = lend(host);
    let facts = parts
        .runs
        .slot_facts(parts.design.restore.visibility.review_markers);
    if restate_waived_findings(parts.model, facts, &waivers) {
        rspice_app_runs::workbench::preflight::invalidate(
            &mut parts.runs.preflight,
            parts.run_controller,
        );
    }
}

/// The step itself. A stimulus draft is the workspace's own edit history,
/// like the symbol document's: it is stepped first while that workspace is
/// open, so Ctrl+Z means "take back what I just typed" rather than reaching
/// past the instrument into the design. Then the netlist page's, where the
/// shell hands that step; then the history holding the newer step. With
/// nothing to step the command is silent: the Edit menu already shows it
/// disabled.
fn take_step<H>(
    host: &mut H,
    lend: fn(&mut H) -> HistoryStepParts<'_>,
    netlist: Option<NetlistHistoryStep<H>>,
    direction: HistoryDirection,
) {
    {
        let parts = lend(host);
        if parts.wb.workspace == Workspace::Stimulus {
            let design = parts.design;
            let selection = design.selected_stimulus_definition;
            let stepped = match direction {
                HistoryDirection::Undo => {
                    stimulus::undo_draft(selection, design.stimulus_editor, parts.model)
                }
                HistoryDirection::Redo => {
                    stimulus::redo_draft(selection, design.stimulus_editor, parts.model)
                }
            };
            if stepped {
                return;
            }
        }
    }
    if let Some(netlist) = netlist {
        match netlist(host, direction) {
            Ok(Some(_)) => return,
            Err(error) => {
                lend(host).wb.prompt(error);
                return;
            }
            Ok(None) => {}
        }
    }
    let mut parts = lend(host);
    if direction == HistoryDirection::Redo {
        // A step committed in either history since an undo replaces what
        // the other can redo, so only the steps no newer edit replaced are
        // left to arbitrate.
        parts.model.retire_replaced_redo_steps();
    }
    let project_first = project_step_is_newer(
        project_sequence(parts.model, direction),
        document_sequence(parts.model, direction),
    );
    if project_first && take_project_history_step(&mut parts, direction) {
        return;
    }
    if !take_document_history_step(&mut parts, direction) && !project_first {
        take_project_history_step(&mut parts, direction);
    }
}

/// Where the project history's next step `direction` sits in the global
/// order.
fn project_sequence(model: &Model, direction: HistoryDirection) -> Option<UndoSequence> {
    match direction {
        HistoryDirection::Undo => model.project_undo_sequence(),
        HistoryDirection::Redo => model.project_redo_sequence(),
    }
}

/// Where the active document's next step `direction` sits in the global
/// order.
///
/// Symbol documents keep their history as plain snapshot stacks that no
/// commit boundary stamps, so they carry no position in that order and
/// answer `None` rather than a position that would sort as the oldest
/// record in the session.
fn document_sequence(model: &Model, direction: HistoryDirection) -> Option<UndoSequence> {
    if model.workspace.content.active_view_type() == ViewType::Symbol {
        return None;
    }
    match direction {
        HistoryDirection::Undo => model.schematic.history().undo_sequence(),
        HistoryDirection::Redo => model.schematic.history().redo_sequence(),
    }
}

/// Take the project history's step `direction`; false when it holds none
/// to take.
fn take_project_history_step(
    parts: &mut HistoryStepParts<'_>,
    direction: HistoryDirection,
) -> bool {
    let stepped = match direction {
        HistoryDirection::Undo => undo_project_design(
            parts.design.restore.reborrow(),
            parts.model,
            parts.wb,
            parts.runs,
            parts.selected_model,
        ),
        HistoryDirection::Redo => redo_project_design(
            parts.design.restore.reborrow(),
            parts.model,
            parts.wb,
            parts.runs,
            parts.selected_model,
        ),
    };
    match stepped {
        Ok(Some(_)) => rspice_app_runs::workbench::preflight::invalidate(
            &mut parts.runs.preflight,
            parts.run_controller,
        ),
        Ok(None) if project_sequence(parts.model, direction).is_some() => {
            parts.wb.prompt(format!(
                "{} is unavailable because the latest project edit's document or references changed.",
                direction.label()
            ));
        }
        Ok(None) => return false,
        Err(error) => parts.wb.prompt(error),
    }
    true
}

/// Take the active document's step `direction`, the symbol editor's own or
/// the schematic's; false when it holds none to take.
fn take_document_history_step(
    parts: &mut HistoryStepParts<'_>,
    direction: HistoryDirection,
) -> bool {
    if parts.model.workspace.content.active_view_type() == ViewType::Symbol {
        let stepped = match direction {
            HistoryDirection::Undo => {
                undo_active_symbol_document(parts.design.restore.symbol, parts.model)
            }
            HistoryDirection::Redo => {
                redo_active_symbol_document(parts.design.restore.symbol, parts.model)
            }
        };
        return match stepped {
            Ok(stepped) => stepped,
            Err(error) => {
                parts.wb.prompt(error);
                true
            }
        };
    }
    let can_step = match direction {
        HistoryDirection::Undo => parts.model.schematic.can_undo(),
        HistoryDirection::Redo => parts.model.can_redo_active_schematic(),
    };
    if !can_step {
        return false;
    }
    let stepped = match direction {
        HistoryDirection::Undo => parts.model.schematic.undo(),
        HistoryDirection::Redo => parts.model.schematic.redo(),
    };
    if stepped {
        *parts.design.restore.snap = parts.model.schematic.session.editor.snap_engine.clone();
        sync_active_schematic(parts.model, parts.wb);
    }
    stepped
}

/// Which stack Undo and Redo act on.
///
/// The project history and the active document's history are one timeline as
/// far as the operator is concerned, so the step that moved most recently
/// wins — that is what the global sequence records. A document history with
/// no position in that order cannot be compared, so a stamped project step
/// takes precedence over it and nothing takes precedence over a document
/// history when the project has no step at all.
fn project_step_is_newer(project: Option<UndoSequence>, document: Option<UndoSequence>) -> bool {
    match (project, document) {
        (Some(project), Some(document)) => project > document,
        (Some(_), None) => true,
        (None, _) => false,
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
