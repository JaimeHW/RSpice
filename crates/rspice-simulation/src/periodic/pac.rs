use super::normalize_pac_node_name;
use crate::error::{ServiceRunError, ServiceRunResult};
use rspice_core::Value;
use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;

/// Frequency sweep type for PAC analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PacFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PacFrequencySweep {
    fn to_core(self) -> rspice_core::analysis::pac::PacSweepType {
        match self {
            Self::Decade => rspice_core::analysis::pac::PacSweepType::Decade,
            Self::Octave => rspice_core::analysis::pac::PacSweepType::Octave,
            Self::Linear => rspice_core::analysis::pac::PacSweepType::Linear,
        }
    }
}

/// Explicit configuration for PAC execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PacRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PacFrequencySweep,
    /// Lowest output sideband index the lifted solve spans.
    ///
    /// The engine's `.PAC` card states the range in either of two ways and
    /// refuses both at once: `MAXSIDEBAND=n` is the symmetric `-n..=n`, and
    /// `SIDEBANDMIN=`/`SIDEBANDMAX=` state the two ends independently. Core's
    /// own `PacConfig` has carried the pair since it existed
    /// (`rspice-core/src/analysis/pac/config.rs`); this request collapsed it to
    /// one symmetric number, so an asymmetric range the engine solves was not
    /// expressible from the Studio at all.
    pub sideband_min: i32,
    /// Highest output sideband index the lifted solve spans.
    pub sideband_max: i32,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: Value,
    pub include_dc: bool,
    pub reltol: Value,
    pub abstol: Value,
    /// Which periodic solve this run linearizes around.
    ///
    /// The card's `FROM=` keyword, in the engine's own vocabulary. It is part
    /// of the request rather than of the dispatch because the three positions
    /// bind different producers in the plan, and because the state the run is
    /// handed has to be checked against the family the request named.
    pub carrier: PeriodicCarrier,
}

impl Default for PacRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: PacFrequencySweep::Decade,
            sideband_min: -5,
            sideband_max: 5,
            input_source: "VRF".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: None,
            pac_magnitude: 1.0,
            include_dc: true,
            reltol: 1e-3,
            abstol: 1e-12,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PacRunConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PAC requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PAC requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PAC requires a positive PSS tolerance".to_string());
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("PAC start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("PAC stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("PAC points per unit must be greater than zero".to_string());
        }
        // The engine's own two refusals on this range, in this order: the ends
        // must not cross, and a card that withholds sideband zero while
        // analysing no other sideband asks for a run with nothing to publish.
        if self.sideband_min > self.sideband_max {
            return Err(format!(
                "PAC sideband range {}..={} is empty",
                self.sideband_min, self.sideband_max
            ));
        }
        if self.sideband_min == 0 && self.sideband_max == 0 && !self.include_dc {
            return Err("PAC configuration must include at least one sideband".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("PAC input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("PAC output node must be specified".to_string());
        }
        if !self.pac_magnitude.is_finite() || self.pac_magnitude <= 0.0 {
            return Err("PAC magnitude must be positive".to_string());
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("PAC relative tolerance must be positive".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("PAC absolute tolerance must be positive".to_string());
        }
        // The carrier itself is not a range check: it names a family, and
        // whether the state this run was handed belongs to that family is
        // `PeriodicCarrierState::accepted_by`, asked where both are in hand.
        Ok(())
    }

    pub fn to_core(&self) -> ServiceRunResult<rspice_core::analysis::pac::PacConfig> {
        use rspice_core::analysis::pac::PacConfig;

        let mut pac_config = PacConfig::new()
            .with_sweep(self.start_freq, self.stop_freq, self.points_per_unit)
            .with_sweep_type(self.sweep.to_core())
            .with_sidebands(self.sideband_min, self.sideband_max)
            .with_input_source(self.input_source.trim())
            .with_output_node(&normalize_pac_node_name(&self.output_node))
            .with_tolerances(self.reltol, self.abstol)
            .with_dc(self.include_dc)
            .with_fundamental(self.pss_fundamental_freq);

        if let Some(output_ref) = &self.output_ref {
            let trimmed = output_ref.trim();
            if !trimmed.is_empty() {
                pac_config = pac_config.with_output_ref(trimmed);
            }
        }

        pac_config.validate().map_err(|error| {
            ServiceRunError::Failure(format!("PAC configuration error: {error}"))
        })?;
        Ok(pac_config)
    }
}
