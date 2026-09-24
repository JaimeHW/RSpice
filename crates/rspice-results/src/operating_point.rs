//! Identity-bound MNA state retained from an accepted operating-point result.
//!
//! The solver may seed a later run from this record only after validating its
//! source and result digests and its complete, finite MNA ordering.

use serde::{Deserialize, Serialize};

use rspice_app_types::product::ContentDigest;

/// Complete, identity-bound MNA state retained by an earlier accepted OP.
/// Node order excludes ground and is followed by branch order in `solution`,
/// exactly matching the core engine's MNA vector contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpPreviousState {
    pub source_content_digest: ContentDigest,
    pub producer_snapshot_digest: ContentDigest,
    pub producer_result_digest: ContentDigest,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub solution: Vec<f64>,
}

impl OpPreviousState {
    pub fn validate(&self) -> Result<(), String> {
        let valid_identity = |name: &str| {
            !name.is_empty() && name.trim() == name && !name.chars().any(char::is_whitespace)
        };
        if self.node_names.iter().any(|name| !valid_identity(name))
            || self.branch_names.iter().any(|name| !valid_identity(name))
            || self
                .node_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("0"))
        {
            return Err(
                "Previous operating-point MNA ordering contains an invalid identity".into(),
            );
        }
        let unique = |names: &[String]| {
            let mut identities = std::collections::HashSet::with_capacity(names.len());
            names
                .iter()
                .all(|name| identities.insert(name.to_ascii_lowercase()))
        };
        if !unique(&self.node_names) || !unique(&self.branch_names) {
            return Err(
                "Previous operating-point MNA ordering contains duplicate identities".into(),
            );
        }
        if self
            .node_names
            .len()
            .saturating_add(self.branch_names.len())
            != self.solution.len()
            || self.solution.is_empty()
            || self.solution.iter().any(|value| !value.is_finite())
        {
            return Err("Previous operating-point MNA state is incomplete or non-finite".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_state_rejects_case_insensitive_duplicate_mna_identities() {
        let previous = OpPreviousState {
            source_content_digest: ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned(), "OUT".to_owned()],
            branch_names: vec!["V1".to_owned()],
            solution: vec![0.5, 0.5, -0.5e-3],
        };
        assert!(previous.validate().unwrap_err().contains("duplicate"));
    }
}
