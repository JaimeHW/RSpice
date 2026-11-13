//! The design feature's operations the tests take over the whole
//! application state.
//!
//! Each is the session form a test drives the design feature through, kept
//! with the state it reads rather than in the feature's own code.

use rspice_design::drc::DrcResult;

use super::AppState;
use super::waivers::StoredWaivers;
use crate::state::{CellViewRef, SymbolDocument, SymbolEditorMetadata};
use crate::workbench::app::actions::hierarchy_net_highlight::light_net_named;
use crate::workbench::app::actions::workspace::design_checks::check_active_cellview;
use crate::workbench::app::actions::workspace::history::{
    activate_history_document, redo_project_design, undo_project_design,
};
use crate::workbench::app::actions::workspace::inline_edit_authority;
use crate::workbench::app::actions::workspace::symbol_document::{
    redo_active_symbol_document, refuse_unsavable_active_symbol, store_active_symbol_editor_bundle,
    undo_active_symbol_document,
};
use crate::workbench::app::actions::workspace::waivers::restate_waived_findings;
use crate::workbench::state::InlineEditAuthority;

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

    /// [`refuse_unsavable_active_symbol`] for the session's model.
    pub(crate) fn refuse_unsavable_active_symbol(&self) -> Option<String> {
        refuse_unsavable_active_symbol(&self.model)
    }

    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        self.design
            .symbol
            .history
            .can_undo(&self.model.workspace.content.active_key())
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
