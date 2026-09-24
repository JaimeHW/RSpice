//! Shared sweep vocabulary for retained analysis specifications.

use serde::{Deserialize, Serialize};

/// Frequency sweep mode used by AC/noise analyses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum FrequencySweep {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl FrequencySweep {
    /// Keyword expected by the simulation runner.
    pub fn runner_keyword(self) -> &'static str {
        match self {
            FrequencySweep::Decade => "dec",
            FrequencySweep::Octave => "oct",
            FrequencySweep::Linear => "lin",
        }
    }
}

/// The rest of an AC sensitivity band, beyond the start frequency.
///
/// Held beside `frequency` rather than replacing it, because one frequency
/// and a sweep starting there are the same card with a different count, and
/// nothing should have two spellings for the band's lower edge.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensitivitySweepSpec {
    pub stop_frequency: f64,
    pub points: u32,
    pub variation: FrequencySweep,
}
