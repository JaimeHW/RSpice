//! Configurations that execute directly on a study's materialized circuit.

use super::*;
use crate::simulation::multi_run::AnalysisSpec;

#[derive(Debug, Clone)]
pub enum StudyAnalysis {
    Basic(AnalysisConfig),
    Native(AnalysisSpec),
    Pss(Box<super::StudyPssConfig>),
    Qpss(Box<super::StudyQpssConfig>),
    Hb(Box<super::StudyHbConfig>),
}

#[cfg(test)]
mod tests;

impl From<AnalysisConfig> for StudyAnalysis {
    fn from(value: AnalysisConfig) -> Self {
        Self::Basic(value)
    }
}

impl StudyAnalysis {
    pub(crate) fn validate(&self) -> Result<(), Vec<String>> {
        match self {
            Self::Basic(config) => config.validate(),
            Self::Native(spec @ AnalysisSpec::HarmonicBalance { .. }) => {
                spec.validate().map_err(|error| vec![error])
            }
            Self::Native(spec @ AnalysisSpec::Qpss { .. }) => spec
                .driven_qpss_config()
                .map(|_| ())
                .map_err(|error| vec![error]),
            Self::Pss(config) => config.validate().map_err(|error| vec![error]),
            Self::Qpss(config) => config.validate().map_err(|error| vec![error]),
            Self::Hb(config) => config.validate().map_err(|error| vec![error]),
            Self::Native(_) => Err(vec!["Unsupported native study analysis".into()]),
        }
    }

    pub(crate) fn as_basic(&self) -> Option<&AnalysisConfig> {
        match self {
            Self::Basic(config) => Some(config),
            Self::Native(_) | Self::Pss(_) | Self::Qpss(_) | Self::Hb(_) => None,
        }
    }

    pub(crate) fn as_basic_mut(&mut self) -> Option<&mut AnalysisConfig> {
        match self {
            Self::Basic(config) => Some(config),
            Self::Native(_) | Self::Pss(_) | Self::Qpss(_) | Self::Hb(_) => None,
        }
    }
}
