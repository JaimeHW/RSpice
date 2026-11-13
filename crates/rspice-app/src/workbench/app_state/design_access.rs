//! The design feature's operations, for a caller holding the whole
//! application state.
//!
//! The design feature takes the slots it reads and never names the
//! application state. What the shell, the services and the run tier still
//! ask of it over the whole state is supplied here, reading and writing the
//! same slots in the same order as before.

use super::AppState;
use crate::diagnostics::LogAnchor;
use crate::state::{CellViewRef, SchematicState, SymbolDocument};
use crate::workbench::app::actions::workspace::symbol_document::store_active_symbol_document;
use crate::workbench::app::actions::workspace::{
    NavigationParts, bump_active_schematic_epoch, commit_inline_component_edit,
    commit_pending_inspector_edit, new_schematic_document, restore_active_schematic_from_workspace,
    sync_active_schematic,
};

impl AppState {
    /// [`new_schematic_document`] for the session.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        new_schematic_document(
            &self.model,
            &self.design.schematic_snap,
            self.design.schematic_routing_mode,
        )
    }

    /// [`NavigationParts::navigate_to_log_anchor`] for the session.
    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigation().navigate_to_log_anchor(anchor);
    }

    /// [`Model::sync_active_schematic_to_workspace`], stating what it has to
    /// say on the status bar.
    ///
    /// [`Model::sync_active_schematic_to_workspace`]: crate::workbench::app_state::Model::sync_active_schematic_to_workspace
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        sync_active_schematic(&mut self.model, &mut self.workbench);
    }

    /// [`restore_active_schematic_from_workspace`] for the session.
    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        restore_active_schematic_from_workspace(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &self.workbench,
            &self.runs,
        );
    }

    /// [`NavigationParts::open_workspace_view`] for the session.
    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.navigation().open_workspace_view(reference);
    }

    /// What moving the canvas to another cell view or occurrence reads and
    /// writes of the session.
    pub(crate) fn navigation(&mut self) -> NavigationParts<'_> {
        self.design
            .navigation_parts(&mut self.model, &mut self.workbench, &self.runs)
    }

    /// [`bump_active_schematic_epoch`] for the session.
    pub(crate) fn bump_active_schematic_epoch(&mut self) {
        bump_active_schematic_epoch(&mut self.design.symbol, &mut self.model, &self.workbench);
    }

    /// Open the generated deck at the selected instance's card, the way
    /// [`NavigationParts::open_selected_instance_master`] opens its
    /// schematic master.
    ///
    /// The Netlist workspace, the generated primary document and the caret
    /// move as one transaction, and the status bar states where the instance
    /// was found. The schematic selection is deliberately left alone: the netlist
    /// navigator's reveal reads it to bring the round trip back.
    pub(crate) fn show_selected_instance_in_netlist(&mut self) {
        let Ok(location) = self.model.selected_instance_generated_line() else {
            return;
        };
        crate::workbench::documents::netlist_document::open_generated_primary_in(
            &mut self.model,
            &mut self.code.netlist,
        );
        self.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        self.workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        self.code.netlist.cursor_line = location.line.saturating_sub(1);
        self.code.netlist.requested_line = Some(location.line);
        let announced = self.workbench.messages().format(
            if location.stale {
                crate::workbench::MessageId::NetlistShowInstanceStale
            } else {
                crate::workbench::MessageId::NetlistShowInstanceLocated
            },
            &[
                ("instance", location.instance.as_str()),
                ("line", &location.line.to_string()),
            ],
        );
        self.prompt(announced);
    }

    /// [`commit_inline_component_edit`] for the session.
    pub(crate) fn commit_inline_component_edit(&mut self) -> Result<bool, String> {
        commit_inline_component_edit(&mut self.model, &mut self.workbench)
    }

    /// [`commit_pending_inspector_edit`] for the session.
    pub(crate) fn commit_pending_inspector_edit(&mut self) -> bool {
        commit_pending_inspector_edit(&mut self.model, &mut self.workbench)
    }

    /// [`store_active_symbol_document`] for the session.
    pub(crate) fn store_active_symbol_document(
        &mut self,
        document: &SymbolDocument,
    ) -> Result<(), String> {
        store_active_symbol_document(&mut self.design.symbol, &mut self.model, document)
    }
}
