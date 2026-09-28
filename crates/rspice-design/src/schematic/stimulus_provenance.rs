//! Durable provenance of a stimulus copied onto a component.

use super::component::Component;
use crate::parameters::normalize_params;
use serde::{Deserialize, Serialize};

/// The copy a placed source took when it adopted a definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusProvenance {
    /// The definition's name, as the library spells it.
    pub definition: String,
    /// The revision that was copied.
    pub revision: u32,
    /// `Component::value` as it was copied.
    pub value: String,
    /// `Component::params` as it was copied.
    pub params: String,
}

impl StimulusProvenance {
    /// Stamp the exact adopted card and its provenance onto a placed instance.
    pub fn stamp_onto(&self, component: &mut Component) {
        component.value = self.value.clone();
        component.params = self.params.clone();
        component.stimulus_provenance = Some(self.clone());
    }

    /// Whether the component still carries exactly the card it copied.
    ///
    /// Both sides are normalized, so a parameter string someone re-ordered by
    /// editing an unrelated field does not read as an edit to the waveform.
    #[must_use]
    pub fn matches_card(&self, component: &Component) -> bool {
        component.value.trim() == self.value.trim()
            && normalize_params(&component.params) == normalize_params(&self.params)
    }
}
