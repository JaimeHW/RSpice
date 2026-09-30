//! Select application context for the runtime's analysis directive writer.

use super::*;

impl SimulationController {
    /// The engine directive one frozen draft emits. `state` supplies live
    /// circuit context and the projected prerequisite closure where needed.
    pub(crate) fn analysis_draft_directive(
        &self,
        state: &AppState,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<String, String> {
        let state = &analysis_inputs(state);
        let spec = rspice_simulation::analysis_preparation::analysis_draft_spec(state, draft)?;
        rspice_simulation::analysis_preparation::analysis_spec_to_spice_line(state, draft, &spec)
    }
}
