use rspice_core::Value;

/// Explicit configuration for PSTB execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(from = "PersistedPstbRunConfig")]
pub struct PstbRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: Value,
    pub detect_subharmonics: bool,
}

/// The settings a study file states. Retired keys decode so older study files
/// still read; they are never written back.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPstbRunConfig {
    pss_fundamental_freq: Value,
    pss_num_harmonics: usize,
    pss_tolerance: Value,
    probe_instance: String,
    max_harmonics: usize,
    num_multipliers: usize,
    stability_threshold: Value,
    detect_subharmonics: bool,
    /// Retired. The Floquet solve is a direct Schur decomposition whose
    /// residual bound is derived from the matrix order, so there is no
    /// tolerance for this to set.
    #[serde(default, rename = "eigenvalue_tolerance")]
    _eigenvalue_tolerance: serde::de::IgnoredAny,
}

impl From<PersistedPstbRunConfig> for PstbRunConfig {
    fn from(persisted: PersistedPstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: persisted.pss_fundamental_freq,
            pss_num_harmonics: persisted.pss_num_harmonics,
            pss_tolerance: persisted.pss_tolerance,
            probe_instance: persisted.probe_instance,
            max_harmonics: persisted.max_harmonics,
            num_multipliers: persisted.num_multipliers,
            stability_threshold: persisted.stability_threshold,
            detect_subharmonics: persisted.detect_subharmonics,
        }
    }
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
        }
    }
}

impl PstbRunConfig {
    /// Check what the card cannot carry.
    ///
    /// The probe name, the harmonic and multiplier counts and the stability
    /// boundary are all fields of
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A study file's settings as a build that still carried `key` wrote
    /// them, and as one that never had it.
    fn stated_and_absent(key: &str, value: serde_json::Value) -> [PstbRunConfig; 2] {
        let mut absent = serde_json::to_value(PstbRunConfig::default()).unwrap();
        absent.as_object_mut().unwrap().remove(key);
        let mut stated = absent.clone();
        stated[key] = value;
        [stated, absent].map(|settings| {
            serde_json::from_value(settings).expect("study settings with or without the key read")
        })
    }

    #[test]
    fn retired_pstb_eigentol_in_a_study_file_reads_as_absent() {
        let [stated, absent] = stated_and_absent("eigenvalue_tolerance", serde_json::json!(1e-9));
        assert_eq!(stated, absent);
        let written = serde_json::to_value(&stated).unwrap();
        assert!(written.get("eigenvalue_tolerance").is_none(), "{written}");
    }
}
