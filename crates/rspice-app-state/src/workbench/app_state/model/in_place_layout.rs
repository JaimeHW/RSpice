//! What a saved layout holds while a symbol is edited in place.
//!
//! The edit in place puts the symbol's view in front and, unless the symbol
//! already had one, a tab of its own beside the sheet. Both are the edit's,
//! not the layout's: a project saved during the edit reopens on the sheet it
//! was edited on, without the tab, as the edit would have left it.

use crate::state::CellViewRef;

/// The symbol edited in place and the sheet it is edited on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InPlaceLayout {
    /// The symbol edited in place, whose view the edit put in front.
    pub symbol: CellViewRef,
    /// The sheet the symbol is edited on.
    pub parent: CellViewRef,
    /// Whether the symbol had a tab of its own before the edit opened one.
    pub symbol_was_open: bool,
}

impl InPlaceLayout {
    /// Make `workspace` the layout a save writes during the edit: the sheet
    /// in front and the edit's own tab gone. A workspace with another view
    /// in front is not the edit's, and is left as it is.
    pub fn apply(&self, workspace: &mut rspice_project::ProjectWorkspace) {
        if workspace.active_view != self.symbol {
            return;
        }
        workspace.active_view = self.parent.clone();
        if !self.symbol_was_open {
            workspace
                .open_views
                .retain(|open| open.reference != self.symbol);
        }
        workspace.project_active_occurrence();
    }
}
