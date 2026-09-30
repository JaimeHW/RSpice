use super::normalize_pac_node_name;
use rspice_core::Value;
use rspice_design::schematic::ground_names::is_ground_reference as is_ground_like;
use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;

/// Frequency sweep type for periodic transfer-function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PxfFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PxfFrequencySweep {
    fn to_variation(self) -> rspice_core::netlist::FreqVariation {
        match self {
            Self::Decade => rspice_core::netlist::FreqVariation::Dec,
            Self::Octave => rspice_core::netlist::FreqVariation::Oct,
            Self::Linear => rspice_core::netlist::FreqVariation::Lin,
        }
    }
}

/// Explicit configuration for PXF execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PxfRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PxfFrequencySweep,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: Value,
    pub abstol: Value,
    /// Which periodic solve this run linearizes around; the card's `FROM=`.
    pub carrier: PeriodicCarrier,
}

impl Default for PxfRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: PxfFrequencySweep::Decade,
            input_source: "VIN".to_string(),
            input_sideband: 1,
            output_node: "VOUT".to_string(),
            output_ref: None,
            output_sideband: 1,
            max_sideband: 5,
            reltol: 1e-3,
            abstol: 1e-12,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PxfRunConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PXF requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PXF requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PXF requires a positive PSS tolerance".to_string());
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("PXF start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("PXF stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("PXF points per unit must be greater than zero".to_string());
        }
        if self.max_sideband < 0 {
            return Err("PXF max sideband must be non-negative".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("PXF input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("PXF output node must be specified".to_string());
        }
        if self.input_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF input sideband {} exceeds configured max sideband {}",
                self.input_sideband, self.max_sideband
            ));
        }
        if self.output_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF output sideband {} exceeds configured max sideband {}",
                self.output_sideband, self.max_sideband
            ));
        }
        if let Some(reference) = self
            .output_ref
            .as_deref()
            .map(str::trim)
            .filter(|node| !node.is_empty() && !is_ground_like(node))
            && reference.eq_ignore_ascii_case(self.output_node.trim())
        {
            return Err("PXF output node and output reference cannot be the same node".to_string());
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("PXF relative tolerance must be positive".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("PXF absolute tolerance must be positive".to_string());
        }
        // The carrier itself is not a range check: it names a family, and
        // whether the state this run was handed belongs to that family is
        // `PeriodicCarrierState::accepted_by`, asked where both are in hand.
        Ok(())
    }

    /// The authored `.PXF` card this configuration states.
    ///
    /// The names are canonicalized here rather than in the card: a Studio form
    /// may hold `V(out)` where a deck line holds `out`, and
    /// [`normalize_pac_node_name`] is what `.PAC` already uses to make the two
    /// the same node. Everything past this point is the engine's reading of a
    /// card, identical to the one the CLI and the wasm surface run.
    pub fn to_card(&self) -> rspice_core::netlist::PxfCard {
        rspice_core::netlist::PxfCard {
            sweep: rspice_core::netlist::PeriodicSweep {
                variation: self.sweep.to_variation(),
                points: self.points_per_unit,
                start_freq: self.start_freq,
                stop_freq: self.stop_freq,
            },
            input_source: self.input_source.trim().to_owned(),
            input_sideband: self.input_sideband,
            output_node: normalize_pac_node_name(&self.output_node),
            output_ref: self
                .output_ref
                .as_deref()
                .map(str::trim)
                .filter(|node| !node.is_empty())
                .map(str::to_owned),
            output_sideband: self.output_sideband,
            max_sideband: self.max_sideband,
            reltol: self.reltol,
            abstol: self.abstol,
            // The selector the request states, in the engine's own vocabulary.
            // It records which family this run linearizes around; the entry
            // that runs below is chosen from the state actually handed over,
            // and the two are checked against each other once.
            source: match self.carrier {
                PeriodicCarrier::Preceding => {
                    rspice_core::netlist::PeriodicSourceSelector::Preceding
                }
                PeriodicCarrier::Pss => rspice_core::netlist::PeriodicSourceSelector::Pss,
                PeriodicCarrier::Hb => rspice_core::netlist::PeriodicSourceSelector::Hb,
            },
        }
    }
}
