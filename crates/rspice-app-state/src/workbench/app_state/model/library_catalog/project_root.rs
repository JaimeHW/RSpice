//! The project's top cell: the schematic runs start from when no
//! configuration is used for runs, and which cells can be it.
//!
//! The top cell is chosen, never guessed. Outside a project's creation and a
//! rename that follows the cell, it changes only here, as one step of Undo.

use super::Model;
use crate::state::workspace::DEFAULT_SCHEMATIC_VIEW;
use crate::state::{CellViewRef, ViewType};

impl Model {
    /// Make `library`/`cell` the project's top cell, as one step of Undo.
    pub fn set_project_top(&mut self, library: &str, cell: &str) -> Result<(), String> {
        let mut candidate = self.workspace.content.project.properties();
        if (candidate.root_library.as_str(), candidate.top_cell.as_str()) == (library, cell) {
            return Err(format!("{library}/{cell} is already the project top."));
        }
        candidate.root_library = library.to_owned();
        candidate.top_cell = cell.to_owned();
        self.publish_project_properties(candidate, format!("set project top to {library}/{cell}"))
    }

    /// Why the project cannot take `library`/`cell` as its top cell, or
    /// `None` when it can: the top cell is the schematic view runs start
    /// from, so it has to be a cell of the catalog with a schematic view the
    /// project holds a drawing for.
    pub fn project_top_refusal(&self, library: &str, cell: &str) -> Option<String> {
        let Some(found) = self
            .library_manager
            .get_library(library)
            .and_then(|found| found.get_cell(cell))
        else {
            return Some(format!("{library}/{cell} is not a cell of this project."));
        };
        if !found
            .get_view(DEFAULT_SCHEMATIC_VIEW)
            .is_some_and(|view| runs_from(view.view_type))
        {
            return Some(format!(
                "{library}/{cell} has no {DEFAULT_SCHEMATIC_VIEW} view to run from."
            ));
        }
        let reference = CellViewRef::new(library, cell, DEFAULT_SCHEMATIC_VIEW);
        (!self.holds_drawing(&reference))
            .then(|| format!("{library}/{cell} has no drawing in this project to run from."))
    }

    /// Every cell the project can take as its top cell, by library and then
    /// cell, ignoring case: what a picker of the top cell offers.
    pub fn project_top_candidates(&self) -> Vec<(String, String)> {
        let mut candidates = Vec::new();
        for (_, library) in self.library_manager.libraries_by_key() {
            for found in library.cells.values() {
                let runnable = found
                    .get_view(DEFAULT_SCHEMATIC_VIEW)
                    .is_some_and(|view| runs_from(view.view_type));
                if runnable
                    && self.holds_drawing(&CellViewRef::new(
                        &library.name,
                        &found.name,
                        DEFAULT_SCHEMATIC_VIEW,
                    ))
                {
                    candidates.push((library.name.clone(), found.name.clone()));
                }
            }
        }
        candidates
            .sort_by_cached_key(|(library, cell)| (library.to_lowercase(), cell.to_lowercase()));
        candidates
    }

    /// Whether the project holds a drawing for `reference`: the one in front
    /// or one of its schematic buffers.
    fn holds_drawing(&self, reference: &CellViewRef) -> bool {
        let key = reference.key();
        self.workspace
            .content
            .active_view
            .key()
            .eq_ignore_ascii_case(&key)
            || self.workspace.content.schematic_buffers.contains_key(&key)
    }
}

/// Whether a view of `view_type` is one a run can start from.
const fn runs_from(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}
