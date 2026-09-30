//! Frozen PSS study configuration.
use super::*;
use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
use rspice_simulation_contract::config::OpConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyOperatingPoint {
    pub instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub config: OpConfig,
    pub numeric_options: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyPssConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyPssConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        if !matches!(
            self.request,
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                ..
            }
        ) {
            return Err("A PSS study requires a shooting PSS specification".into());
        }
        self.request.validate()?;
        self.operating_point.config.validate()
    }

    pub(super) fn validate_measurements(&self, measurements: &[String]) -> Result<(), String> {
        self.validate()?;
        let AnalysisSpec::Pss { num_harmonics, .. } = self.request else {
            unreachable!()
        };
        for request in measurements {
            let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
            if mode.eq_ignore_ascii_case("last") {
                continue;
            }
            if mode.eq_ignore_ascii_case("scalar")
                && ["pss.frequency", "pss.period", "pss.iterations"]
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(key))
            {
                continue;
            }
            if mode.eq_ignore_ascii_case("bin") {
                let (index, _, signal) =
                    rspice_simulation_contract::study_measurement::parse_study_bin(key)?;
                if index <= num_harmonics && signal.is_some() {
                    continue;
                }
                return Err(
                    "PSS bin observations need an explicit signal and a retained harmonic index"
                        .into(),
                );
            }
            return Err("PSS studies require last:signal, bin:index:quantity:signal or scalar:pss.frequency/period/iterations".into());
        }
        Ok(())
    }
}
