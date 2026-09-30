//! Complete periodic consumer controls and execution on a freshly solved trial.
use super::*;
use crate::execution_options::SpecExecutionOptions;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "analysis", content = "config", deny_unknown_fields)]
pub enum StudyPeriodicOptions {
    Pac(crate::periodic::PacRunConfig),
    Pxf(crate::periodic::PxfRunConfig),
    Pnoise(crate::periodic::PnoiseRunConfig),
    Pstb(crate::periodic::PstbRunConfig),
}

impl StudyPeriodicOptions {
    pub fn execution_options(&self) -> SpecExecutionOptions {
        let mut options = SpecExecutionOptions::default();
        match self {
            Self::Pac(config) => options.pac = Some(config.clone()),
            Self::Pxf(config) => options.pxf = Some(config.clone()),
            Self::Pnoise(config) => options.pnoise = Some(config.clone()),
            Self::Pstb(config) => options.pstb = Some(config.clone()),
        }
        options
    }
}

impl StudyPostprocess {
    pub fn periodic_execution_options(&self) -> Result<SpecExecutionOptions, SimulationError> {
        let matches = matches!(
            (&self.request, &self.periodic_options),
            (AnalysisSpec::Pac, Some(StudyPeriodicOptions::Pac(_)))
                | (AnalysisSpec::Pxf, Some(StudyPeriodicOptions::Pxf(_)))
                | (AnalysisSpec::Pnoise, Some(StudyPeriodicOptions::Pnoise(_)))
                | (AnalysisSpec::Pstb, Some(StudyPeriodicOptions::Pstb(_)))
                | (
                    AnalysisSpec::Psp { .. }
                        | AnalysisSpec::Fourier { .. }
                        | AnalysisSpec::Fft { .. }
                        | AnalysisSpec::Hbsp { .. }
                        | AnalysisSpec::Hbnoise { .. }
                        | AnalysisSpec::Qpac { .. }
                        | AnalysisSpec::Qpxf { .. }
                        | AnalysisSpec::Qpnoise { .. },
                    None
                )
        );
        if !matches {
            return Err(SimulationError::InvalidConfig(
                "Study consumer is missing its matching complete execution configuration".into(),
            ));
        }
        Ok(self
            .periodic_options
            .as_ref()
            .map(StudyPeriodicOptions::execution_options)
            .unwrap_or_default())
    }

    pub fn is_periodic(&self) -> bool {
        matches!(
            self.request,
            AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. }
                | AnalysisSpec::Qpac { .. }
                | AnalysisSpec::Qpxf { .. }
                | AnalysisSpec::Qpnoise { .. }
        )
    }
}
