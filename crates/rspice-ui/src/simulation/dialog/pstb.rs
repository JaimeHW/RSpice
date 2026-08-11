//! Periodic Stability (PSTB) Analysis Configuration
//!
//! Configuration for periodic stability analysis around a PSS operating point.
//! PSTB uses Floquet analysis to determine the stability of periodic circuits
//! such as oscillators and switched-capacitor filters.
//!
//! Every field here reaches [`crate::services::simulation_runner::PstbRunConfig`].
//! The margins themselves are always extracted — they are the result of the
//! analysis, not a choice about it — so what remains configurable is the
//! numerical contract the Floquet decomposition is judged under.
//!
//! # Example SPICE Output
//!
//! ```text
//! .pstb probe=LPROBE
//! + maxharm=10 nmults=10
//! ```

use super::options::parse_si_value;
use serde::{Deserialize, Deserializer};

// =============================================================================
// PSTB Configuration
// =============================================================================

/// Periodic stability (PSTB) analysis configuration
#[derive(Debug, Clone)]
pub struct PstbConfig {
    /// Probe instance name (loop break)
    pub probe: String,
    /// Maximum harmonics for analysis
    pub max_harmonics: u32,
    /// Number of Floquet multipliers to compute
    pub num_multipliers: u32,
    /// Magnitude above which a Floquet multiplier is called unstable. Slightly
    /// above unity so a marginally stable mode is not reported as a failure
    /// on rounding alone.
    pub stability_threshold: f64,
    /// Report period-doubling and other subharmonic modes.
    pub detect_subharmonics: bool,
    /// Convergence tolerance of the eigenvalue decomposition.
    pub eigenvalue_tolerance: f64,
}

impl Default for PstbConfig {
    fn default() -> Self {
        Self {
            probe: "LPROBE".to_string(),
            max_harmonics: 10,
            num_multipliers: 10,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }
    }
}

impl PstbConfig {
    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(".pstb probe={}", self.probe);

        cmd.push_str(&format!(" maxharm={}", self.max_harmonics));

        if self.num_multipliers != 10 {
            cmd.push_str(&format!(" nmults={}", self.num_multipliers));
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.probe.is_empty() {
            return Err("Probe instance must be specified".to_string());
        }
        if self.max_harmonics == 0 {
            return Err("Maximum harmonics must be at least 1".to_string());
        }
        if self.num_multipliers == 0 {
            return Err("Number of multipliers must be at least 1".to_string());
        }
        if !self.stability_threshold.is_finite() || self.stability_threshold <= 0.0 {
            return Err("Stability threshold must be a positive magnitude".to_string());
        }
        if !self.eigenvalue_tolerance.is_finite() || self.eigenvalue_tolerance <= 0.0 {
            return Err("Eigenvalue tolerance must be positive".to_string());
        }
        Ok(())
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PstbDialogState {
    pub probe: String,
    pub max_harmonics: String,
    pub num_multipliers: String,
    pub stability_threshold: String,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: String,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPstbDialogState {
    #[serde(default)]
    probe: String,
    #[serde(default)]
    max_harmonics: String,
    #[serde(default)]
    num_multipliers: String,
    #[serde(default)]
    stability_threshold: Option<String>,
    #[serde(default)]
    detect_subharmonics: Option<bool>,
    #[serde(default)]
    eigenvalue_tolerance: Option<String>,
    /// Retired. Floquet extraction always yields both margins and there is no
    /// schematic annotation channel for PSTB, so none of these three selected
    /// anything. Accepted so earlier projects still open; never written back.
    #[serde(default)]
    #[allow(dead_code)]
    annotate: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    phase_margin: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    gain_margin: serde::de::IgnoredAny,
}

impl<'de> Deserialize<'de> for PstbDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedPstbDialogState::deserialize(deserializer)?;
        let defaults = PstbConfig::default();
        Ok(Self {
            probe: persisted.probe,
            max_harmonics: persisted.max_harmonics,
            num_multipliers: persisted.num_multipliers,
            stability_threshold: persisted
                .stability_threshold
                .unwrap_or_else(|| format_tolerance(defaults.stability_threshold)),
            detect_subharmonics: persisted
                .detect_subharmonics
                .unwrap_or(defaults.detect_subharmonics),
            eigenvalue_tolerance: persisted
                .eigenvalue_tolerance
                .unwrap_or_else(|| format_tolerance(defaults.eigenvalue_tolerance)),
            initialized: false,
        })
    }
}

impl PstbDialogState {
    /// Initialize from config
    pub fn from_config(config: &PstbConfig) -> Self {
        Self {
            probe: config.probe.clone(),
            max_harmonics: config.max_harmonics.to_string(),
            num_multipliers: config.num_multipliers.to_string(),
            stability_threshold: format_tolerance(config.stability_threshold),
            detect_subharmonics: config.detect_subharmonics,
            eigenvalue_tolerance: format_tolerance(config.eigenvalue_tolerance),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PstbConfig, String> {
        let max_harm: u32 = self
            .max_harmonics
            .parse()
            .map_err(|_| "Invalid harmonics")?;
        let num_mult: u32 = self
            .num_multipliers
            .parse()
            .map_err(|_| "Invalid multipliers")?;
        let stability_threshold = parse_si_value(&self.stability_threshold)
            .map_err(|e| format!("Invalid stability threshold: {e}"))?;
        let eigenvalue_tolerance = parse_si_value(&self.eigenvalue_tolerance)
            .map_err(|e| format!("Invalid eigenvalue tolerance: {e}"))?;

        let config = PstbConfig {
            probe: self.probe.clone(),
            max_harmonics: max_harm,
            num_multipliers: num_mult,
            stability_threshold,
            detect_subharmonics: self.detect_subharmonics,
            eigenvalue_tolerance,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PstbConfig::default());
        }
    }
}

/// Round-trippable spelling for the two numeric contracts, which sit far
/// enough from unity that a plain `{}` would print a long decimal tail.
fn format_tolerance(value: f64) -> String {
    let rendered = format!("{value}");
    if rendered.len() > 12 {
        format!("{value:e}")
    } else {
        rendered
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_contracts_round_trip_through_the_editor() {
        let config = PstbConfig {
            stability_threshold: 1.05,
            eigenvalue_tolerance: 1e-12,
            detect_subharmonics: false,
            ..PstbConfig::default()
        };

        let restored = PstbDialogState::from_config(&config)
            .to_config()
            .expect("editor state converts back");

        assert!((restored.stability_threshold - 1.05).abs() < 1e-15);
        assert!((restored.eigenvalue_tolerance - 1e-12).abs() < 1e-24);
        assert!(!restored.detect_subharmonics);
    }

    #[test]
    fn retired_margin_flags_decode_to_the_default_contract() {
        let persisted = r#"{
            "probe": "LP",
            "max_harmonics": "8",
            "num_multipliers": "6",
            "annotate": true,
            "phase_margin": true,
            "gain_margin": false
        }"#;

        let state: PstbDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert!(state.detect_subharmonics);
        let config = {
            let mut state = state.clone();
            state.initialized = true;
            state.to_config().expect("legacy state converts")
        };
        assert!((config.stability_threshold - (1.0 + 1e-6)).abs() < 1e-15);

        let encoded = serde_json::to_value(&state).expect("state encodes");
        for retired in ["annotate", "phase_margin", "gain_margin"] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }
}
