//! Editable periodic-initializer settings retained independently in each envelope draft.

use crate::envelope_initialization::{EnvelopeInitializationConfig, EnvelopeShootingIntegration};
use crate::options::parse_si_value;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeInitializationState {
    pub max_iterations: String,
    pub reltol: String,
    pub abstol: String,
    pub damping: String,
    pub verbose: bool,
    pub pss_stabilization_periods: String,
    pub pss_stabilization_time: String,
    pub pss_points_per_period: String,
    pub hb_min_damping: String,
    pub hb_oversample: String,
    pub hb_collocation_points: String,
    pub hb_use_krylov: bool,
    pub hb_gmres_restart: String,
    pub hb_source_stepping: bool,
    pub hb_exact_jacobian: bool,
    pub pss_integration_idx: usize,
}

impl Default for EnvelopeInitializationState {
    fn default() -> Self {
        Self::from_config(&EnvelopeInitializationConfig::default())
    }
}

impl EnvelopeInitializationState {
    pub fn from_config(config: &EnvelopeInitializationConfig) -> Self {
        Self {
            max_iterations: config.max_iterations.to_string(),
            reltol: config.reltol.to_string(),
            abstol: config.abstol.to_string(),
            damping: config.damping.to_string(),
            verbose: config.verbose,
            pss_stabilization_periods: config.pss_stabilization_periods.to_string(),
            pss_stabilization_time: if config.pss_stabilization_time == 0.0 {
                String::new()
            } else {
                config.pss_stabilization_time.to_string()
            },
            pss_points_per_period: config
                .pss_points_per_period
                .map(|value| value.to_string())
                .unwrap_or_default(),
            hb_min_damping: config.hb_min_damping.to_string(),
            hb_oversample: config.hb_oversample.to_string(),
            hb_collocation_points: config
                .hb_collocation_points
                .map(|value| value.to_string())
                .unwrap_or_default(),
            hb_use_krylov: config.hb_use_krylov,
            hb_gmres_restart: config.hb_gmres_restart.to_string(),
            hb_source_stepping: config.hb_source_stepping,
            hb_exact_jacobian: config.hb_exact_jacobian,
            pss_integration_idx: config.pss_integration as usize,
        }
    }

    pub fn to_config(&self, method: usize) -> Result<EnvelopeInitializationConfig, String> {
        let mut config = EnvelopeInitializationConfig::default();
        if method == 2 {
            return Ok(config);
        }
        if method < 2 {
            config.max_iterations = parse_integer(&self.max_iterations, "max_iterations")?;
        }
        if method < 2 {
            config.reltol = parse_si_value(&self.reltol)
                .map_err(|error| format!("Initializer reltol: {error}"))?;
        }
        if method < 2 {
            config.abstol = parse_si_value(&self.abstol)
                .map_err(|error| format!("Initializer abstol: {error}"))?;
        }
        if method < 2 {
            config.damping = parse_si_value(&self.damping)
                .map_err(|error| format!("Initializer damping: {error}"))?;
        }
        if method < 2 {
            config.verbose = self.verbose;
        }
        if method == 1 {
            config.pss_stabilization_periods =
                parse_integer(&self.pss_stabilization_periods, "pss_stabilization_periods")?;
            config.pss_stabilization_time = if self.pss_stabilization_time.trim().is_empty() {
                0.0
            } else {
                parse_si_value(&self.pss_stabilization_time)
                    .map_err(|error| format!("Initializer stabilization time: {error}"))?
            };
        }
        if method == 1 {
            config.pss_points_per_period = if self.pss_points_per_period.trim().is_empty() {
                None
            } else {
                Some(parse_integer(
                    &self.pss_points_per_period,
                    "pss_points_per_period",
                )?)
            };
        }
        if method == 0 {
            config.hb_min_damping = parse_si_value(&self.hb_min_damping)
                .map_err(|error| format!("Initializer hb_min_damping: {error}"))?;
        }
        if method == 0 {
            config.hb_oversample = parse_integer(&self.hb_oversample, "hb_oversample")?;
        }
        if method == 0 {
            config.hb_collocation_points = if self.hb_collocation_points.trim().is_empty() {
                None
            } else {
                Some(parse_integer(
                    &self.hb_collocation_points,
                    "hb_collocation_points",
                )?)
            };
        }
        if method == 0 {
            config.hb_use_krylov = self.hb_use_krylov;
        }
        if method == 0 {
            config.hb_gmres_restart = parse_integer(&self.hb_gmres_restart, "hb_gmres_restart")?;
        }
        if method == 0 {
            config.hb_source_stepping = self.hb_source_stepping;
        }
        if method == 0 {
            config.hb_exact_jacobian = self.hb_exact_jacobian;
        }
        if method == 1 {
            config.pss_integration = match self.pss_integration_idx {
                0 => EnvelopeShootingIntegration::Automatic,
                1 => EnvelopeShootingIntegration::BackwardEuler,
                2 => EnvelopeShootingIntegration::Trapezoidal,
                3 => EnvelopeShootingIntegration::Gear2,
                4 => EnvelopeShootingIntegration::TrapGear,
                _ => return Err("Invalid envelope shooting integration method".into()),
            };
        }
        Ok(config)
    }
}

fn parse_integer(value: &str, label: &str) -> Result<usize, String> {
    value
        .trim()
        .parse()
        .map_err(|_| format!("Initializer {label} must be a nonnegative integer"))
}
