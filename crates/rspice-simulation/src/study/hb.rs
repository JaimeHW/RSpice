//! Frozen HB study configuration.
use super::*;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyHbConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyHbConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !matches!(self.request, AnalysisSpec::HarmonicBalance { .. }) {
            return Err("An HB study requires a harmonic-balance specification".into());
        }
        self.request.validate()?;
        self.operating_point.config.validate()
    }
}
