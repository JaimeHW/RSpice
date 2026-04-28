//! Harmonic Balance Tone Setup
//!
//! Multi-tone RF configuration for harmonic balance analysis.
//! Matches Cadence SpectreRF's tone configuration interface.
//!
//! # Features
//!
//! - Define fundamental tones (LO, RF, IF)
//! - Configure harmonic orders
//! - Mixing products specification
//! - Intermodulation setup
//! - Two-tone and multi-tone test configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Tone Type
// =============================================================================

/// Type of RF tone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ToneType {
    /// Local oscillator tone
    Lo,
    /// RF input tone
    Rf,
    /// IF (intermediate frequency) tone
    If,
    /// Clock tone
    Clk,
    /// Data rate tone
    Data,
    /// Custom/general tone
    #[default]
    Custom,
}

impl ToneType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ToneType::Lo => "LO",
            ToneType::Rf => "RF",
            ToneType::If => "IF",
            ToneType::Clk => "CLK",
            ToneType::Data => "DATA",
            ToneType::Custom => "Custom",
        }
    }
}

// =============================================================================
// Tone Definition
// =============================================================================

/// A single RF tone definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tone {
    /// Unique tone ID
    pub id: u32,
    /// Tone name
    pub name: String,
    /// Tone type
    pub tone_type: ToneType,
    /// Frequency in Hz
    pub frequency: f64,
    /// Amplitude (peak, in V or dBm depending on source)
    pub amplitude: f64,
    /// Whether amplitude is in dBm
    pub amplitude_in_dbm: bool,
    /// Phase in degrees
    pub phase: f64,
    /// Number of harmonics to analyze
    pub harmonics: usize,
    /// Whether this tone is enabled
    pub enabled: bool,
    /// Source component name
    pub source: Option<String>,
}

impl Default for Tone {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            tone_type: ToneType::Custom,
            frequency: 1e9,
            amplitude: 0.0,
            amplitude_in_dbm: true,
            phase: 0.0,
            harmonics: 5,
            enabled: true,
            source: None,
        }
    }
}

impl Tone {
    /// Create a new tone
    pub fn new(id: u32, name: impl Into<String>, frequency: f64) -> Self {
        Self {
            id,
            name: name.into(),
            frequency,
            ..Default::default()
        }
    }

    /// Create an LO tone
    pub fn lo(id: u32, frequency: f64, power_dbm: f64) -> Self {
        Self {
            id,
            name: "LO".to_string(),
            tone_type: ToneType::Lo,
            frequency,
            amplitude: power_dbm,
            amplitude_in_dbm: true,
            harmonics: 10,
            ..Default::default()
        }
    }

    /// Create an RF tone
    pub fn rf(id: u32, frequency: f64, power_dbm: f64) -> Self {
        Self {
            id,
            name: "RF".to_string(),
            tone_type: ToneType::Rf,
            frequency,
            amplitude: power_dbm,
            amplitude_in_dbm: true,
            harmonics: 3,
            ..Default::default()
        }
    }

    /// Set harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.harmonics = n;
        self
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set phase
    pub fn with_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        self
    }

    /// Convert amplitude to Vpeak (assuming 50 ohm)
    pub fn amplitude_vpeak(&self) -> f64 {
        if self.amplitude_in_dbm {
            dbm_to_vpeak(self.amplitude, 50.0)
        } else {
            self.amplitude
        }
    }
}

/// Convert dBm to Vpeak for given impedance
fn dbm_to_vpeak(dbm: f64, impedance: f64) -> f64 {
    let p_watts = 10.0f64.powf((dbm - 30.0) / 10.0);
    (2.0 * p_watts * impedance).sqrt()
}

// =============================================================================
// Intermodulation Products
// =============================================================================

/// Intermodulation product specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermodProduct {
    /// Tone 1 harmonic index
    pub h1: i32,
    /// Tone 2 harmonic index
    pub h2: i32,
    /// Product name (e.g., "IM3", "IM5")
    pub name: String,
    /// Whether to compute this product
    pub enabled: bool,
}

impl IntermodProduct {
    /// Create a new IM product
    pub fn new(h1: i32, h2: i32) -> Self {
        let order = h1.abs() + h2.abs();
        Self {
            h1,
            h2,
            name: format!("IM{}", order),
            enabled: true,
        }
    }

    /// IM3 upper product (2*f1 - f2)
    pub fn im3_upper() -> Self {
        IntermodProduct::new(2, -1)
    }

    /// IM3 lower product (2*f2 - f1)
    pub fn im3_lower() -> Self {
        IntermodProduct::new(-1, 2)
    }

    /// IM5 products
    pub fn im5_products() -> Vec<Self> {
        vec![IntermodProduct::new(3, -2), IntermodProduct::new(-2, 3)]
    }

    /// Order of the intermodulation product
    pub fn order(&self) -> i32 {
        self.h1.abs() + self.h2.abs()
    }

    /// Calculate frequency given two tone frequencies
    pub fn frequency(&self, f1: f64, f2: f64) -> f64 {
        (self.h1 as f64 * f1 + self.h2 as f64 * f2).abs()
    }
}

// =============================================================================
// Mixing Specification
// =============================================================================

/// Mixing/conversion specification for mixer analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MixingSpec {
    /// Input frequency mix coefficients (map from tone ID to harmonic)
    pub input_mix: HashMap<u32, i32>,
    /// Output frequency mix coefficients
    pub output_mix: HashMap<u32, i32>,
    /// Sideband selection (upper/lower)
    pub sideband: Sideband,
}

/// Sideband selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Sideband {
    #[default]
    Upper,
    Lower,
    Both,
}

impl MixingSpec {
    /// Create a downconversion specification
    /// RF - LO -> IF
    pub fn downconvert(rf_id: u32, lo_id: u32) -> Self {
        let mut spec = Self::default();
        spec.input_mix.insert(rf_id, 1);
        spec.output_mix.insert(rf_id, 1);
        spec.output_mix.insert(lo_id, -1);
        spec
    }

    /// Create an upconversion specification
    /// IF + LO -> RF
    pub fn upconvert(if_id: u32, lo_id: u32) -> Self {
        let mut spec = Self::default();
        spec.input_mix.insert(if_id, 1);
        spec.output_mix.insert(if_id, 1);
        spec.output_mix.insert(lo_id, 1);
        spec
    }
}

// =============================================================================
// Harmonic Balance Configuration
// =============================================================================

/// Complete HB tone setup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HbToneSetup {
    /// All defined tones
    pub tones: Vec<Tone>,
    /// Next tone ID
    next_id: u32,
    /// Maximum total harmonics (limits analysis size)
    pub max_harmonics: usize,
    /// Intermodulation products to compute
    pub intermod_products: Vec<IntermodProduct>,
    /// Mixing specification (for conversion gain, etc.)
    pub mixing: Option<MixingSpec>,
    /// Reference impedance (ohms)
    pub reference_impedance: f64,
    /// Whether to oversample for accuracy
    pub oversample: bool,
    /// Oversample ratio
    pub oversample_ratio: usize,
}

impl HbToneSetup {
    /// Create a new setup
    pub fn new() -> Self {
        Self {
            max_harmonics: 100,
            reference_impedance: 50.0,
            oversample: true,
            oversample_ratio: 4,
            ..Default::default()
        }
    }

    /// Add a tone
    pub fn add_tone(&mut self, mut tone: Tone) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        tone.id = id;
        self.tones.push(tone);
        id
    }

    /// Get a tone by ID
    pub fn get_tone(&self, id: u32) -> Option<&Tone> {
        self.tones.iter().find(|t| t.id == id)
    }

    /// Get mutable tone by ID
    pub fn get_tone_mut(&mut self, id: u32) -> Option<&mut Tone> {
        self.tones.iter_mut().find(|t| t.id == id)
    }

    /// Remove a tone
    pub fn remove_tone(&mut self, id: u32) {
        self.tones.retain(|t| t.id != id);
    }

    /// Get enabled tones
    pub fn enabled_tones(&self) -> Vec<&Tone> {
        self.tones.iter().filter(|t| t.enabled).collect()
    }

    /// Calculate total harmonic count for HB
    pub fn total_harmonics(&self) -> usize {
        let enabled = self.enabled_tones();
        if enabled.is_empty() {
            return 0;
        }

        // Product of (2*n + 1) for each tone
        enabled
            .iter()
            .map(|t| 2 * t.harmonics + 1)
            .product::<usize>()
            .min(self.max_harmonics)
    }

    /// Calculate frequencies for a specific harmonic combination
    pub fn mix_frequency(&self, indices: &[i32]) -> f64 {
        self.enabled_tones()
            .iter()
            .zip(indices.iter())
            .map(|(t, &h)| t.frequency * h as f64)
            .sum::<f64>()
            .abs()
    }

    /// Create a standard two-tone test setup
    pub fn two_tone_test(f1: f64, f2: f64, power_dbm: f64) -> Self {
        let mut setup = Self::new();
        setup.add_tone(Tone::new(0, "Tone1", f1).with_harmonics(7));
        setup.add_tone(Tone::new(1, "Tone2", f2).with_harmonics(7));

        // Update amplitudes
        for tone in &mut setup.tones {
            tone.amplitude = power_dbm;
            tone.amplitude_in_dbm = true;
        }

        // Add IM products
        setup.intermod_products.push(IntermodProduct::im3_upper());
        setup.intermod_products.push(IntermodProduct::im3_lower());
        setup
            .intermod_products
            .extend(IntermodProduct::im5_products());

        setup
    }

    /// Create a mixer analysis setup
    pub fn mixer_analysis(lo_freq: f64, rf_freq: f64, lo_power: f64, rf_power: f64) -> Self {
        let mut setup = Self::new();
        let lo_id = setup.add_tone(Tone::lo(0, lo_freq, lo_power));
        let rf_id = setup.add_tone(Tone::rf(1, rf_freq, rf_power));

        setup.mixing = Some(MixingSpec::downconvert(rf_id, lo_id));
        setup
    }

    /// Validate the setup
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled_tones().is_empty() {
            return Err("At least one tone must be enabled".to_string());
        }

        for tone in self.enabled_tones() {
            if tone.frequency <= 0.0 {
                return Err(format!("Tone '{}' has invalid frequency", tone.name));
            }
            if tone.harmonics == 0 {
                return Err(format!(
                    "Tone '{}' must have at least 1 harmonic",
                    tone.name
                ));
            }
        }

        if self.total_harmonics() > self.max_harmonics {
            return Err(format!(
                "Total harmonics {} exceeds maximum {}",
                self.total_harmonics(),
                self.max_harmonics
            ));
        }

        Ok(())
    }

    /// Generate HB analysis command
    pub fn to_hb_command(&self) -> String {
        let mut cmd = ".HB".to_string();

        for tone in self.enabled_tones() {
            cmd.push_str(&format!(
                " TONE={:.6e} harmonics={}",
                tone.frequency, tone.harmonics
            ));
        }

        cmd
    }
}

// =============================================================================
// Tests
// =============================================================================

