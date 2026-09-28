//! Checked label repairs on a disposable connectivity candidate.

use crate::state::SchematicState;

impl SchematicState {
    pub(crate) fn repair_candidate_net_labels(
        &mut self,
        canonical: &str,
        labels: &[(u64, String)],
    ) -> Result<(), String> {
        self.design.repair_candidate_net_labels(canonical, labels)
    }
}
