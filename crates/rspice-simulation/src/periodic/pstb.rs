use rspice_core::Value;

/// Explicit configuration for PSTB execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PstbRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: Value,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: Value,
}

impl Default for PstbRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 10,
            num_multipliers: 10,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }
    }
}

impl PstbRunConfig {
    /// Check what the card cannot carry.
    ///
    /// The probe name, the harmonic and multiplier counts, the stability
    /// boundary and the eigen-tolerance are all fields of
    /// [`PstbCard`](rspice_core::netlist::PstbCard), and the engine refuses each by name
    /// in `run_pstb_card_from_pss_with_abort`. Restating them here is how the
    /// Studio came to hold a threshold rule its own engine did not: only the
    /// prerequisite-PSS fields, which no card has a home for, are checked.
    pub fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PSTB requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PSTB requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PSTB requires a positive PSS tolerance".to_string());
        }
        Ok(())
    }

    /// The authored `.PSTB` card this configuration states.
    pub fn to_card(&self) -> rspice_core::netlist::PstbCard {
        rspice_core::netlist::PstbCard {
            probe_instance: self.probe_instance.trim().to_owned(),
            max_harmonics: self.max_harmonics,
            num_multipliers: self.num_multipliers,
            stability_threshold: self.stability_threshold,
            detect_subharmonics: self.detect_subharmonics,
            eigenvalue_tolerance: self.eigenvalue_tolerance,
        }
    }
}
