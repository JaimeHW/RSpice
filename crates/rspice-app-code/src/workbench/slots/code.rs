//! The code feature's dialogs, creating a Verilog-A device and editing the
//! open Verilog-A view's build settings, and its netlist and Verilog-A
//! editors' session, with the netlist documents opened and the outline groups
//! collapsed; and what code shows other features.

use crate::workbench::app::dialogs::library::new_veriloga_device::new_veriloga_device_available_in;
use crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState;
use crate::workbench::documents::netlist_document::{NetlistDocumentState, NetlistRoot};
use crate::workbench::framework::feature::{CodePort, NetlistDocumentKey};
use rspice_app_runs::simulation::{DeckOrigin, ManualDeck};
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

/// What the code feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub struct CodeState {
    /// New Verilog-A device: a cell whose module the reader writes next.
    pub new_veriloga_device:
        crate::workbench::app::dialogs::library::new_veriloga_device::NewVerilogADeviceFormState,
    /// Build settings: the open Verilog-A view's build profile, as a form.
    pub veriloga_build_settings: Option<crate::workbench::app::VerilogABuildSettingsDraft>,
    /// Netlist editor state (diagnostics, diff pips).
    pub netlist: crate::workbench::documents::netlist_document::NetlistDocumentState,
    /// The Verilog-A editor's revision-bound operation receipts.
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
    /// Rename symbol over the deck: the name, the new name and what renaming
    /// to it does.
    pub(crate) deck_rename:
        Option<crate::workbench::documents::netlist_document::references::DeckRename>,
}

impl CodeState {
    /// The netlist and code editors' findings, as the diagnostics store
    /// reads them where the editors keep them.
    pub fn diagnostic_sources(
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
        }
    }

    /// What the document bar's Netlist tabs read of the feature, beside the
    /// model and the message catalogue.
    pub fn netlist_tabs<'a>(
        &'a self,
        model: &'a rspice_app_state::workbench::app_state::model::Model,
        messages: crate::workbench::MessageCatalog,
    ) -> crate::workbench::chrome::document_bar::code::NetlistTabsReads<'a> {
        crate::workbench::chrome::document_bar::code::NetlistTabsReads {
            model,
            netlist: &self.netlist,
            open_documents: &self.netlist_open_documents,
            messages,
        }
    }

    /// The project's decks went back to their accepted list: the netlist
    /// editor's session, its open deck and the tabs it opened belong to the
    /// list that was thrown away, and the project's first deck opens again.
    pub fn on_project_changed(&mut self) {
        self.netlist = Default::default();
        self.deck_rename = None;
        self.netlist_open_documents.clear();
    }

    /// The deck in front, which a manual run reads.
    pub fn manual_deck(&self) -> Option<ManualDeck<'_>> {
        manual_deck_in(&self.netlist)
    }

    /// The receipt of the project's current Verilog-A compilation.
    pub fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        veriloga_compilation_in(&self.code_workspace)
    }
}

/// The deck Run runs on the Netlist workspace, which a manual run reads:
/// the deck last brought to the front, while it is open and the plan has
/// not been brought to the front since.
pub fn manual_deck_in(netlist: &NetlistDocumentState) -> Option<ManualDeck<'_>> {
    let deck = netlist.open_deck.as_ref()?;
    (netlist.run_target() == crate::workbench::documents::netlist_tabs::RunTarget::Deck(deck.id))
        .then(|| ManualDeck {
            name: deck
                .root
                .path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("deck"),
            text: &deck.root.text,
            origin: match &deck.stored_files {
                Some(files) => DeckOrigin::Stored { files },
                None => DeckOrigin::File(&deck.root.path),
            },
        })
}

/// The receipt of the project's current Verilog-A compilation, as the
/// Verilog-A editor's state holds it.
pub fn veriloga_compilation_in(
    code_workspace: &CodeWorkspaceRuntimeState,
) -> Option<&ProjectCompileReceipt> {
    code_workspace
        .veriloga
        .receipt
        .as_ref()
        .map(|receipt| &receipt.compilation)
}

impl CodePort for CodeState {
    fn manual_deck(&self) -> Option<ManualDeck<'_>> {
        CodeState::manual_deck(self)
    }

    fn shown_netlist_document_key(&self) -> NetlistDocumentKey {
        netlist_document_key(self.netlist.front.root_kind())
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

    fn new_veriloga_device_available(&self, model: &crate::workbench::app_state::Model) -> bool {
        new_veriloga_device_available_in(model)
    }

    fn read_only_veriloga_well(
        &self,
        ui: &mut egui::Ui,
        id: (&'static str, u64),
        source: &mut String,
        revision: u64,
        messages: crate::workbench::MessageCatalog,
    ) {
        use crate::workbench::documents::code_workspace::{
            CodeEditorLanguage, EditorFindings, release_closed_code_editors,
            show_veriloga_document_versioned,
        };

        // One source is read here at a time: the editor of the one read
        // before is let go.
        release_closed_code_editors(ui.ctx(), "read-only-veriloga-well", &[egui::Id::new(id)]);
        show_veriloga_document_versioned(
            ui,
            id,
            source,
            revision,
            CodeEditorLanguage::VerilogA,
            EditorFindings::Outdated,
            None,
            false,
            messages,
            &[],
        );
    }
}

/// The key a change check compares for `document`: each document its own,
/// so a switch between any two of them is a change.
const fn netlist_document_key(document: NetlistRoot) -> NetlistDocumentKey {
    NetlistDocumentKey::new(match document {
        NetlistRoot::Generated => 0,
        NetlistRoot::Deck => 1,
        NetlistRoot::Comparison => 2,
        NetlistRoot::RunInput => 3,
        NetlistRoot::AddedLines => 4,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::documents::netlist_document::NetlistTab;

    /// A run's sealed deck and the generated netlist are different
    /// documents, so the switch between them is a change a reader re-projects
    /// on, as is every other switch.
    #[test]
    fn every_shown_netlist_document_has_its_own_key() {
        let deck = crate::state::DeckId::new();
        let documents = [
            NetlistTab::Generated,
            NetlistTab::Deck(deck),
            NetlistTab::Compare {
                left: crate::workbench::documents::netlist_document::CompareSide::PreviousGenerated(
                    0,
                ),
                right: crate::workbench::documents::netlist_document::CompareSide::Generated,
            },
            NetlistTab::RunInput {
                run: 1,
                point: None,
            },
            NetlistTab::AddedLines,
        ];
        let mut code = CodeState::default();
        let keys = documents.map(|document| {
            code.netlist.front = document;
            code.shown_netlist_document_key()
        });
        assert_ne!(keys[0], keys[3], "the generated netlist and a run's deck");
        for (index, key) in keys.iter().enumerate() {
            for other in &keys[index + 1..] {
                assert_ne!(key, other);
            }
        }
    }
}
