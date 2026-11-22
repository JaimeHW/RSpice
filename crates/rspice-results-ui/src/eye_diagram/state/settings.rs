//! What the reader set the eye sheet to, kept per result so a re-run of the
//! same plan card folds, measures and masks the eye as it was left.

use serde::{Deserialize, Serialize};

use super::{EyeFold, EyeMask};
use crate::eye_diagram::{DEFAULT_OPENING_BER, MAX_EYE_LEVELS, MIN_EYE_LEVELS};

/// The error rates the opening and total jitter are read at, as the bar
/// offers them.
pub const OPENING_RATES: [f64; 4] = [1e-9, 1e-12, 1e-15, 1e-18];

/// The eye sheet's settings for one result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EyeSettings {
    /// What folds the eye and which signal it folds.
    pub fold: EyeFold,
    /// Signal levels the eye is measured with, two to four.
    pub levels: u8,
    /// The error rate the opening and total jitter are read at.
    pub opening_ber: f64,
    /// The mask, and whether it is drawn and tested.
    pub mask: EyeMask,
    pub show_mask: bool,
}

impl Default for EyeSettings {
    fn default() -> Self {
        Self {
            fold: EyeFold::default(),
            levels: MIN_EYE_LEVELS,
            opening_ber: DEFAULT_OPENING_BER,
            mask: EyeMask::default(),
            show_mask: false,
        }
    }
}

impl EyeSettings {
    /// Whether the settings can be kept: levels in range, an error rate
    /// strictly between zero and one, and a drawable mask.
    pub fn validate(&self) -> Result<(), String> {
        if !(MIN_EYE_LEVELS..=MAX_EYE_LEVELS).contains(&self.levels) {
            return Err(format!(
                "An eye has {MIN_EYE_LEVELS} to {MAX_EYE_LEVELS} levels"
            ));
        }
        if !(self.opening_ber > 0.0 && self.opening_ber < 1.0) {
            return Err("The error rate must be between 0 and 1".into());
        }
        if let Some(offset) = self.fold.offset
            && !offset.is_finite()
        {
            return Err("The clock offset must be a time".into());
        }
        if let Some(clock) = &self.fold.clock
            && !clock.threshold.is_finite()
        {
            return Err("The clock threshold must be a voltage".into());
        }
        self.mask.validate()
    }
}
