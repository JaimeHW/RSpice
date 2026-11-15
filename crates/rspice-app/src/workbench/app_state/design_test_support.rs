//! The design feature's operations the tests take over the whole
//! application state.
//!
//! Each is the session form a test drives the design feature through, kept
//! with the state it reads rather than in the feature's own code.

use rspice_design::drc::DrcResult;

use super::AppState;
use super::waivers::StoredWaivers;
use crate::workbench::state::InlineEditAuthority;
use rspice_app_design::workbench::app::actions::hierarchy_net_highlight::light_net_named;
use rspice_app_design::workbench::app::actions::workspace::design_checks::check_active_cellview;
use rspice_app_design::workbench::app::actions::workspace::history::{
    activate_history_document, redo_project_design, undo_project_design,
};
use rspice_app_design::workbench::app::actions::workspace::inline_edit_authority;
use rspice_app_design::workbench::app::actions::workspace::symbol_document::{
    active_symbol_pin_finding_count, can_undo_active_symbol_document, redo_active_symbol_document,
    run_active_symbol_pin_checks, store_active_symbol_editor_bundle, undo_active_symbol_document,
};
use rspice_app_design::workbench::app::actions::workspace::waivers::restate_waived_findings;
use rspice_app_state::state::{CellViewRef, SymbolDocument, SymbolEditorMetadata};

impl AppState {
    /// [`check_active_cellview`] for the session.
    pub(crate) fn check_active_cellview(&mut self) -> Result<DrcResult, String> {
        let facts = self.slot_facts();
        check_active_cellview(&mut self.model, &mut self.workbench, facts)
    }

    /// Check Design: the configured design, walked from the simulation root,
    /// with every schematic of the project's own libraries it never reaches.
    pub(crate) fn check_design(&mut self) -> Result<DrcResult, String> {
        rspice_app_runs::workbench::preflight::check_design(self)
    }

    /// [`undo_project_design`] for the session.
    pub(crate) fn undo_project_design(&mut self) -> Result<Option<String>, String> {
        undo_project_design(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &mut self.workbench,
            &self.runs,
            self.models.selected_model.as_deref(),
        )
    }

    /// [`redo_project_design`] for the session.
    pub(crate) fn redo_project_design(&mut self) -> Result<Option<String>, String> {
        redo_project_design(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &mut self.workbench,
            &self.runs,
            self.models.selected_model.as_deref(),
        )
    }

    /// [`activate_history_document`] for the session.
    pub(crate) fn activate_history_document(&mut self, reference: &CellViewRef, operation: &str) {
        activate_history_document(
            &mut self.design.symbol,
            &mut self.model,
            &mut self.workbench,
            reference,
            operation,
        );
    }

    /// [`inline_edit_authority`] for the session's model.
    pub(crate) fn inline_edit_authority(&self) -> InlineEditAuthority {
        inline_edit_authority(&self.model)
    }

    /// [`store_active_symbol_editor_bundle`] for the session.
    pub(crate) fn store_active_symbol_editor_bundle(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<(), String> {
        store_active_symbol_editor_bundle(
            &mut self.design.symbol,
            &mut self.model,
            &mut self.workbench,
            document,
            metadata,
        )
    }

    /// [`run_active_symbol_pin_checks`] for the session.
    pub(crate) fn run_active_symbol_pin_checks(&mut self) {
        let facts = self.slot_facts();
        run_active_symbol_pin_checks(&mut self.model, facts);
    }

    /// [`active_symbol_pin_finding_count`] for the session's model.
    pub(crate) fn active_symbol_pin_finding_count(&self) -> Option<usize> {
        active_symbol_pin_finding_count(&self.model)
    }

    /// The levels of the open symbol's published pin findings, once its
    /// checks have run.
    pub(crate) fn active_symbol_pin_finding_levels(
        &self,
    ) -> Option<Vec<super::diagnostic_store::DiagnosticLevel>> {
        let subject = self.model.workspace.content.active_view.clone();
        self.model
            .diagnostics
            .batch(&super::diagnostic_store::DiagnosticInputs::SymbolPins { subject })
            .map(|batch| batch.records.iter().map(|record| record.level).collect())
    }

    /// [`can_undo_active_symbol_document`] for the session.
    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        can_undo_active_symbol_document(&self.design.symbol, &self.model)
    }

    /// [`undo_active_symbol_document`] for the session.
    pub(crate) fn undo_active_symbol_document(&mut self) -> Result<bool, String> {
        undo_active_symbol_document(&mut self.design.symbol, &mut self.model)
    }

    /// [`redo_active_symbol_document`] for the session.
    pub(crate) fn redo_active_symbol_document(&mut self) -> Result<bool, String> {
        redo_active_symbol_document(&mut self.design.symbol, &mut self.model)
    }

    /// [`restate_waived_findings`] for the session.
    pub(crate) fn restate_waived_findings(&mut self, before: &StoredWaivers) -> bool {
        let facts = self.slot_facts();
        restate_waived_findings(&mut self.model, facts, before)
    }

    /// [`light_net_named`] for the session's model.
    pub(crate) fn light_net_named(&mut self, name: &str) -> bool {
        light_net_named(&mut self.model, name)
    }
}
