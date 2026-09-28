//! Checked label repairs on a disposable connectivity candidate.

use crate::state::SchematicState;

impl SchematicState {
    pub(crate) fn repair_candidate_net_labels(
        &mut self,
        canonical: &str,
        labels: &[(u64, String)],
    ) -> Result<(), String> {
        crate::state::NetLabel::validate_name(canonical, self.document.document_policy.net_naming)
            .map_err(|error| format!("Canonical global name is invalid: {error}."))?;
        for (id, expected) in labels {
            let label = self
                .document
                .net_labels
                .iter_mut()
                .find(|label| label.id == *id)
                .ok_or_else(|| format!("Net label #{id} no longer exists."))?;
            if label.name != *expected {
                return Err(format!(
                    "Net label #{id} changed from '{expected}' to '{}'. Refresh the report.",
                    label.name
                ));
            }
            canonical.clone_into(&mut label.name);
        }
        Ok(())
    }
}
