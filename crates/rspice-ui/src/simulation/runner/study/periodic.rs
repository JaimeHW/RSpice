//! Complete periodic consumer controls and execution on a freshly solved trial.
use super::super::SpecExecutionOptions;
use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "analysis", content = "config", deny_unknown_fields)]
pub enum StudyPeriodicOptions {
    Pac(services::PacRunConfig),
    Pxf(services::PxfRunConfig),
    Pnoise(services::PnoiseRunConfig),
    Pstb(services::PstbRunConfig),
}

impl StudyPeriodicOptions {
    pub(crate) fn execution_options(&self) -> SpecExecutionOptions {
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
    pub(super) fn periodic_execution_options(
        &self,
    ) -> Result<SpecExecutionOptions, SimulationError> {
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
                        | AnalysisSpec::Hbnoise { .. },
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

    pub(super) fn is_periodic(&self) -> bool {
        matches!(
            self.request,
            AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. }
        )
    }

    pub(super) fn run_periodic(
        &self,
        engine: &rspice_core::Engine,
        analysis: &StudyAnalysis,
        circuit: &rspice_core::Netlist,
        numeric_options: &str,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        self.periodic_execution_options()?;
        let (physical, result) = match analysis {
            StudyAnalysis::Pss(pss) => {
                pss.run_with_circuit(engine, circuit, &self.producer_numeric_options, abort)?
            }
            StudyAnalysis::Native(producer @ AnalysisSpec::HarmonicBalance { .. }) => {
                let physical = super::pss::circuit_with_options(
                    circuit,
                    &self.producer_numeric_options,
                    abort,
                )?;
                let result = super::super::spec::run_native_study_on_materialized(
                    producer.clone(),
                    &physical,
                    abort,
                )?;
                (physical, result)
            }
            _ => {
                return Err(SimulationError::InvalidConfig(
                    "Periodic study requires its configured PSS or HB producer".into(),
                ));
            }
        };
        let consumer = super::pss::circuit_with_options(&physical, numeric_options, abort)?;
        let carrier = match &result {
            SimulationResult::Transient {
                periodic_state: Some(point),
                ..
            } => services::PeriodicCarrierState::Shooting(point),
            SimulationResult::HarmonicBalance {
                operating_point: point,
                ..
            } => services::PeriodicCarrierState::HarmonicBalance(point),
            _ => {
                return Err(SimulationError::SolverError(
                    "Study producer returned no retained periodic state".into(),
                ));
            }
        };
        super::super::spec::run_periodic_study_consumer(
            self.request.clone(),
            self.periodic_options.as_ref(),
            &consumer,
            carrier,
            abort,
        )
    }
}

#[cfg(test)]
mod tests;
