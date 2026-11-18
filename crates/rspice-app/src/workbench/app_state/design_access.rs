//! The design feature's operations, for a caller holding the whole
//! application state.
//!
//! The design feature takes the slots it reads and never names the
//! application state. What the shell, the services and the run tier still
//! ask of it over the whole state is supplied here, reading and writing the
//! same slots in the same order as before.

use super::AppState;
use rspice_app_design::workbench::app::actions::property_edit::open_property_editor_in;
use rspice_app_design::workbench::app::actions::workspace::symbol_document::{
    record_symbol_edit, run_active_symbol_pin_checks, store_active_symbol_document,
};
use rspice_app_design::workbench::app::actions::workspace::{
    NavigationParts, bump_active_schematic_epoch, commit_inline_component_edit,
    commit_pending_inspector_edit, new_schematic_document, restore_active_schematic_from_workspace,
    sync_active_schematic,
};
use rspice_app_state::diagnostics::LogAnchor;
use rspice_app_state::state::{CellViewRef, SchematicState, SymbolDocument};
use rspice_app_state::workbench::app_state::waivers::{StoredWaiverEdit, WaiverTarget};

impl AppState {
    /// [`new_schematic_document`] for the session.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        new_schematic_document(
            &self.model,
            &self.design.schematic_snap,
            self.design.schematic_routing_mode,
        )
    }

    /// [`open_property_editor_in`] for the session.
    pub(crate) fn open_property_editor(&mut self, component_id: u64) {
        let mut parts = self.design.property_form_parts();
        open_property_editor_in(
            &mut parts,
            &mut self.model,
            &mut self.workbench,
            component_id,
        );
    }

    /// [`NavigationParts::navigate_to_log_anchor`] for the session.
    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigation().navigate_to_log_anchor(anchor);
    }

    /// [`Model::sync_active_schematic_to_workspace`], stating what it has to
    /// say on the status bar.
    ///
    /// [`Model::sync_active_schematic_to_workspace`]: rspice_app_state::workbench::app_state::model::Model::sync_active_schematic_to_workspace
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
    /// schematic master; or, with a net selected, at the first card that
    /// connects it, with every place the deck writes it marked.
    ///
    /// The Netlist workspace, the generated primary document and the caret
    /// move as one transaction, and the status bar states where the instance
    /// was found, or why it cannot be shown, in which case nothing moves. The
    /// schematic selection is deliberately left alone: the netlist
    /// navigator's reveal reads it to bring the round trip back.
    pub(crate) fn show_selected_instance_in_netlist(&mut self) {
        if let Some(net) = self.model.selected_design_net() {
            self.show_net_in_netlist(&net);
            return;
        }
        let location = match self.model.selected_instance_generated_line() {
            Ok(location) => location,
            Err(reason) => {
                let detail = self.generation_failure_detail();
                self.prompt(format!(
                    "Cannot show the instance in the netlist: {reason}.{detail}"
                ));
                return;
            }
        };
        rspice_app_code::workbench::documents::netlist_document::show_generated_deck(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.workbench,
        );
        rspice_app_code::workbench::documents::netlist_document::cross_probe::clear_net_marks(
            &mut self.code.netlist,
        );
        self.code.netlist.cursor_line = location.line.saturating_sub(1);
        self.code.netlist.requested_line = Some(location.line);
        let announced = self.workbench.messages().format(
            if location.stale {
                rspice_app_workbench::workbench::localization::MessageId::NetlistShowInstanceStale
            } else {
                rspice_app_workbench::workbench::localization::MessageId::NetlistShowInstanceLocated
            },
            &[
                ("instance", location.instance.as_str()),
                ("line", &location.line.to_string()),
            ],
        );
        self.prompt(announced);
    }

    /// Open the generated deck at the first card of the open cell view that
    /// connects `net`, and mark every place its cards write it.
    fn show_net_in_netlist(&mut self, net: &str) {
        use rspice_app_code::workbench::documents::netlist_document;
        use rspice_app_workbench::workbench::localization::MessageId;

        let location = match self.model.net_generated_lines(net) {
            Ok(location) => location,
            Err(reason) => {
                let detail = self.generation_failure_detail();
                self.prompt(format!(
                    "Cannot show net {net} in the netlist: {reason}.{detail}"
                ));
                return;
            }
        };
        netlist_document::show_generated_deck(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.workbench,
        );
        let first = location.lines[0];
        self.code.netlist.cursor_line = first.saturating_sub(1);
        self.code.netlist.requested_line = Some(first);
        netlist_document::cross_probe::mark_net_in_generated_netlist(
            &self.model,
            &mut self.code.netlist,
            net,
            &location.lines,
        );
        let announced = self.workbench.messages().format(
            if location.stale {
                MessageId::NetlistShowNetStale
            } else {
                MessageId::NetlistShowNetLocated
            },
            &[("net", net), ("line", &first.to_string())],
        );
        self.prompt(announced);
    }

    /// What the last generation said when it failed, to follow a refusal.
    fn generation_failure_detail(&self) -> String {
        self.model
            .netlist_artifacts
            .generation_error
            .as_deref()
            .map(|error| format!(" {error}"))
            .unwrap_or_default()
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

    /// Waive the finding at `target` for `reason`, as an edit of the document
    /// that stores it, and state every published finding again.
    // Application half of a model operation; it becomes a request.
    pub(crate) fn waive_finding(
        &mut self,
        target: &WaiverTarget,
        reason: &str,
    ) -> Result<(), String> {
        let edit = StoredWaiverEdit::waive(target, reason)?;
        self.edit_waivers(&target.cell_view, edit)
    }

    /// Remove the waiver stored at `target`, as an edit of the document that
    /// stores it, and state every published finding again.
    // Application half of a model operation; it becomes a request.
    pub(crate) fn remove_finding_waiver(&mut self, target: &WaiverTarget) -> Result<(), String> {
        self.edit_waivers(&target.cell_view, StoredWaiverEdit::remove(target))
    }

    fn edit_waivers(
        &mut self,
        cell_view: &rspice_app_state::state::CellViewRef,
        edit: StoredWaiverEdit,
    ) -> Result<(), String> {
        if let Some(edit) = self.model.edit_held_waivers(cell_view, edit)? {
            self.edit_active_symbol_waivers(edit)?;
            // The pin checks read the symbol document, waivers and all,
            // so they are asked again rather than left stale.
            let facts = self.slot_facts();
            run_active_symbol_pin_checks(&mut self.model, facts);
        }
        self.runs.preflight.invalidate();
        self.restate_design_checks();
        Ok(())
    }

    fn edit_active_symbol_waivers(&mut self, edit: StoredWaiverEdit) -> Result<(), String> {
        let before = self.model.load_active_symbol_document()?;
        let mut document = before.clone();
        edit.apply(&mut document.waivers)?;
        if document == before {
            return Ok(());
        }
        self.store_active_symbol_document(&document)?;
        record_symbol_edit(
            &mut self.design.symbol,
            &self.model,
            &before,
            "Edit waivers",
        );
        Ok(())
    }
}
