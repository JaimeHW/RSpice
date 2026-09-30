//! Compatibility boundary between the retired singleton draft layout and the
//! stable per-instance simulation-plan domain.
//!
//! Project migration reads legacy fields through this adapter. New execution
//! and presentation code must operate on `AnalysisDraft` values and must not
//! treat these singleton fields as plan identity.

use rspice_simulation_contract::analysis_draft::AnalysisDraft;

use crate::workbench::app_state::SimSetupState;

impl SimSetupState {
    /// Validate one raw instance draft with the same parser used by the
    /// existing controller, without mutating the authoritative plan.
    pub(crate) fn analysis_draft_validation_error(&self, draft: &AnalysisDraft) -> Option<String> {
        if let Some(error) = draft.manifest_configuration_error() {
            return Some(error);
        }
        if let Some(blocker) = draft.kind().execution_blocker() {
            return Some(format!("Execution unavailable: {blocker}"));
        }
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(state) if matches!(state.temperature_mode_idx, 0 | 3))
        {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.validation_error(draft.legacy_index())
    }

    /// Render the existing concise summary from one exact instance draft.
    pub(crate) fn analysis_draft_summary(&self, draft: &AnalysisDraft) -> String {
        if let Some(summary) = draft.manifest_summary() {
            return summary;
        }
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(state) if matches!(state.temperature_mode_idx, 0 | 3))
        {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.summary(draft.legacy_index())
    }
}
