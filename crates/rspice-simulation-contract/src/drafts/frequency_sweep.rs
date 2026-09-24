//! Persisted frequency sweep fields shared by periodic analysis drafts.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencySweepDraft {
    pub start: String,
    pub stop: String,
    pub points: String,
    /// 0 = decade, 1 = octave, 2 = linear.
    pub sweep: usize,
}

impl Default for FrequencySweepDraft {
    fn default() -> Self {
        Self {
            start: "1k".to_owned(),
            stop: "10G".to_owned(),
            points: "101".to_owned(),
            sweep: 0,
        }
    }
}
