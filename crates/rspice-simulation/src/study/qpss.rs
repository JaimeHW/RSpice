//! Frozen QPSS study configuration.
use super::*;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyQpssConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyQpssConfig {
    pub fn validate(&self) -> Result<(), String> {
        self.request.qpss_config()?;
        self.operating_point.config.validate()
    }
}
