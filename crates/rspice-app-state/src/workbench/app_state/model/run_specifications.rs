//! The requirements the active retained run was judged against, read from the
//! model alone, so the Results workspace and its printed capture agree.

use rspice_results::specification::report::resolved_specifications;

use super::Model;
use crate::state::SpecEntry;

impl Model {
    /// The requirements the workspace's active retained dataset was judged
    /// against, owned.
    ///
    /// The model's spelling of [`resolved_specifications`], for the hardcopy
    /// capture, which takes a presentation of the Results workspace rather
    /// than drawing one. It captured `workspace.specs` directly, and both the
    /// printed table and the printed document's identity derive from what it
    /// captured — so a limit edited after a completed run restated itself on
    /// the page beside the verdict the run had actually earned, and moved the
    /// identity of an immutable result.
    pub fn active_run_specifications(&self) -> Vec<SpecEntry> {
        self.simulation.active_run().map_or_else(
            || self.workspace.content.specs.clone(),
            |run| resolved_specifications(run, &self.workspace.content.specs).into_owned(),
        )
    }
}
