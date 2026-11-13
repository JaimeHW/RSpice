//! A transaction over the project's decks: the model, the netlist editor's
//! state and the include tabs the reader opened, changed together in copies
//! and installed only once the whole result validates.

use std::collections::HashSet;

use super::{Model, NetlistDocumentState};
use crate::workbench::state::WorkspaceDocumentId;

/// The deck parts a transaction builds its result in: copies of the live
/// model, netlist editor's state and include tabs, which the transaction
/// changes and validates, then commits, or drops to change nothing.
///
/// Committing only these three equals installing a whole candidate state
/// through `AppState::commit_candidate`, because a deck transaction writes no
/// other field of its candidate. Every other field of a whole candidate is
/// either a copy of the live field it was cloned from or a runtime session the
/// whole-state commit carries back from the live state, so both commits leave
/// those fields as they were. The model installs through
/// [`Model::install_candidate`], which carries across the notices raised and
/// not yet shown, the one part of the whole-state commit that reaches these
/// three.
pub(crate) struct DeckTransaction {
    pub(crate) model: Model,
    pub(crate) netlist: NetlistDocumentState,
    pub(crate) open_documents: HashSet<WorkspaceDocumentId>,
}

#[expect(
    dead_code,
    reason = "the netlist document's transactions adopt it in the next change"
)]
impl DeckTransaction {
    /// Begin a transaction over copies of the live deck parts.
    pub(crate) fn begin(
        model: &Model,
        netlist: &NetlistDocumentState,
        open_documents: &HashSet<WorkspaceDocumentId>,
    ) -> Self {
        Self {
            model: model.clone(),
            netlist: netlist.clone(),
            open_documents: open_documents.clone(),
        }
    }

    /// Install the validated result as the live deck parts.
    pub(crate) fn commit_deck(
        self,
        model: &mut Model,
        netlist: &mut NetlistDocumentState,
        open_documents: &mut HashSet<WorkspaceDocumentId>,
    ) {
        model.install_candidate(self.model);
        *netlist = self.netlist;
        *open_documents = self.open_documents;
    }
}
