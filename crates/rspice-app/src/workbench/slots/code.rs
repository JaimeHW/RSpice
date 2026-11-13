//! The code feature's dialogs, creating a Verilog-A device and editing the
//! open Verilog-A view's build settings, and its netlist and Code & Automation
//! editors' session, with the netlist documents opened and the outline groups
//! collapsed; and what code shows other features.

use crate::simulation::NetlistShown;
use crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState;
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, NetlistDocumentState};
use crate::workbench::framework::feature::CodePort;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

/// What the code feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct CodeState {
    /// New Verilog-A device: a cell whose module the reader writes next.
    pub(crate) new_veriloga_device:
        crate::workbench::app::dialogs::library::new_veriloga_device::NewVerilogADeviceFormState,
    /// Build settings: the open Verilog-A view's build profile, as a form.
    pub(crate) veriloga_build_settings: Option<crate::workbench::app::VerilogABuildSettingsDraft>,
    /// Netlist editor state (diagnostics, diff pips, tuner mode).
    pub netlist: crate::workbench::documents::netlist_document::NetlistDocumentState,
    /// Visible Code & Automation page and revision-bound operation receipts.
    pub code_workspace: crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState,
    /// Netlist secondary documents explicitly opened by the user. Resolved
    /// include closures can contain thousands of files, so availability alone
    /// must never materialize every include as a tab.
    pub netlist_open_documents:
        std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    /// Netlist outline groups the engineer has collapsed. Collapsed rather
    /// than expanded, so a deck that gains a category discloses it instead of
    /// hiding it behind a preference nobody set.
    pub netlist_outline_collapsed: std::collections::BTreeSet<crate::state::OutlineSectionKind>,
}

impl CodeState {
    /// The netlist and code editors' findings, as the diagnostics store
    /// reads them where the editors keep them.
    pub(in crate::workbench) fn diagnostic_sources(
        &self,
    ) -> crate::workbench::app_state::diagnostic_store::SourceCollections<'_> {
        let code = &self.code_workspace;
        let veriloga = code.veriloga.receipt.as_ref();
        crate::workbench::app_state::diagnostic_store::SourceCollections {
            netlist: &self.netlist.diagnostics,
            netlist_revision: self.netlist.revision,
            netlist_validated: self.netlist.validated_revision(),
            veriloga: veriloga.map(|receipt| &receipt.diagnostics),
            veriloga_token: veriloga.map(|receipt| receipt.compilation.token()),
            veriloga_failure: &code.veriloga.last_failure,
            veriloga_failure_token: code.veriloga.last_failure_token.as_ref(),
            automation: &code.automation.diagnostics,
            automation_validated: code.automation.receipt.is_some(),
            automation_token: code
                .automation
                .receipt
                .as_ref()
                .map(|receipt| &receipt.token)
                .or(code.automation.diagnostic_token.as_ref())
                .map(|token| (token.project_id, token.revision)),
        }
    }

    /// What the document bar's Netlist tabs read of the feature, beside the
    /// model, the runs and the message catalogue.
    pub(crate) fn netlist_tabs<'a>(
        &'a self,
        model: &'a rspice_app_state::workbench::app_state::model::Model,
        runs: &'a rspice_app_runs::workbench::slots::runs::RunsState,
        messages: crate::workbench::MessageCatalog,
    ) -> crate::workbench::chrome::document_bar::code::NetlistTabsReads<'a> {
        crate::workbench::chrome::document_bar::code::NetlistTabsReads {
            model,
            netlist: &self.netlist,
            code_workspace: &self.code_workspace,
            open_documents: &self.netlist_open_documents,
            runs,
            messages,
        }
    }

    /// The project's netlist source went back to its accepted revision: the
    /// netlist editor's session and the secondary documents it opened belong
    /// to the source that was thrown away.
    pub(crate) fn on_project_changed(&mut self) {
        self.netlist = Default::default();
        self.netlist_open_documents.clear();
    }

    /// Which netlist document the editor shows, as a manual-deck run reads it.
    pub(crate) fn netlist_shown(&self) -> NetlistShown {
        netlist_shown_in(&self.netlist)
    }

    /// The receipt of the project's current Verilog-A compilation.
    pub(crate) fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        veriloga_compilation_in(&self.code_workspace)
    }
}

/// Which netlist document the netlist editor's session shows, as a
/// manual-deck run reads it.
pub(crate) fn netlist_shown_in(netlist: &NetlistDocumentState) -> NetlistShown {
    NetlistShown {
        settled: netlist.active_document_initialized,
        comparison: netlist.active_document == ActiveNetlistDocument::GeneratedDiff,
        owned_source: netlist.active_document == ActiveNetlistDocument::OwnedSource,
    }
}

/// The receipt of the project's current Verilog-A compilation, as the Code &
/// Automation page's state holds it.
pub(crate) fn veriloga_compilation_in(
    code_workspace: &CodeWorkspaceRuntimeState,
) -> Option<&ProjectCompileReceipt> {
    code_workspace
        .veriloga
        .receipt
        .as_ref()
        .map(|receipt| &receipt.compilation)
}

impl CodePort for CodeState {
    fn owned_source_active(&self) -> bool {
        self.netlist.active_document == ActiveNetlistDocument::OwnedSource
    }

    fn shows_owned_root(&self) -> bool {
        self.netlist.active_document == ActiveNetlistDocument::OwnedSource
            && self.netlist.active_dependency_identity.is_none()
    }

    fn netlist_cursor_line(&self) -> usize {
        self.netlist.cursor_line
    }

    fn netlist_shown(&self) -> NetlistShown {
        CodeState::netlist_shown(self)
    }

    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        CodeState::veriloga_compilation(self)
    }

    fn interface_check_finding(
        &self,
        sources: &crate::state::ProjectSourceRegistry,
        libraries: &crate::state::LibraryManager,
    ) -> Option<(crate::state::CellViewRef, String)> {
        crate::workbench::documents::code_workspace::interface_check_finding(
            &self.code_workspace.veriloga,
            sources,
            libraries,
        )
    }

    fn manual_deck_run_block_reason(
        &self,
        model: &crate::workbench::app_state::Model,
        messages: crate::workbench::MessageCatalog,
    ) -> Option<String> {
        crate::workbench::documents::netlist_document::manual_deck_run_block_reason_in(
            model,
            &self.netlist,
            messages,
        )
    }
}
