//! Authored quasi-periodic request controls shared by plans, wire, and execution.

use rspice_core::analysis::quasi_periodic::{
    QuasiPeriodicAcConfig, QuasiPeriodicGridConfig, QuasiPeriodicLinearConfig,
    QuasiPeriodicSampling, QuasiPeriodicSolveConfig,
};
use rspice_core::engine::{
    QpnoiseFrequencyAxis, QpnoiseIntegrationMethod, QpnoiseLattices, QpnoiseNoiseFigure,
    QpnoiseOutput, QpnoiseSources, QpssConfig, QpssInitialState, QpssOscillator, QpssSourceTone,
    QpxfFrequencyAxis, QpxfInputLattices, QpxfSources,
};
use serde::{Deserialize, Serialize};

/// Harmonic balance tone request used by pipeline execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HbToneSpec {
    /// Tone frequency in Hz.
    pub frequency: f64,
    /// Number of harmonics requested for this tone.
    pub harmonics: usize,
    /// Optional independent source name this tone should drive.
    pub source: Option<String>,
    /// Optional label for display/debug.
    pub name: Option<String>,
}

impl HbToneSpec {
    pub fn new(frequency: f64, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpssControls {
    pub current_absolute_tolerance: f64,
    pub voltage_absolute_tolerance: f64,
    pub max_backtracks: usize,
    #[serde(default, skip_serializing_if = "QuasiPeriodicLinearConfig::is_default")]
    pub linear: QuasiPeriodicLinearConfig,
    pub max_mixing_order: Option<usize>,
    /// One count broadcasts to every tone; otherwise one count per tone.
    pub sampling: QuasiPeriodicSampling,
    pub initial_state: QpssInitialState,
    pub source_tones: Vec<QpssSourceTone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oscillator: Option<QpssOscillator>,
}

impl Default for QpssControls {
    fn default() -> Self {
        Self {
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-9,
            max_backtracks: 20,
            linear: QuasiPeriodicLinearConfig::default(),
            max_mixing_order: None,
            sampling: QuasiPeriodicSampling::Oversample(vec![2]),
            initial_state: QpssInitialState::Zero,
            source_tones: Vec::new(),
            oscillator: None,
        }
    }
}

impl QpssControls {
    pub fn to_core_config(
        &self,
        tones: &[HbToneSpec],
        max_iterations: usize,
        relative_tolerance: f64,
    ) -> Result<QpssConfig, String> {
        let (counts, exact) = match &self.sampling {
            QuasiPeriodicSampling::Oversample(counts) => (counts, false),
            QuasiPeriodicSampling::Exact(counts) => (counts, true),
        };
        let counts = if counts.len() == 1 {
            vec![counts[0]; tones.len()]
        } else {
            counts.clone()
        };
        let mut source_tones = self.source_tones.clone();
        for (tone, specification) in tones.iter().enumerate() {
            if let Some(source) = &specification.source {
                source_tones.push(QpssSourceTone {
                    source: source.clone(),
                    tone,
                });
            }
        }
        let config = QpssConfig {
            grid: QuasiPeriodicGridConfig {
                frequencies_hz: tones.iter().map(|tone| tone.frequency).collect(),
                harmonics: tones.iter().map(|tone| tone.harmonics).collect(),
                max_mixing_order: self.max_mixing_order,
                sampling: if exact {
                    QuasiPeriodicSampling::Exact(counts)
                } else {
                    QuasiPeriodicSampling::Oversample(counts)
                },
            },
            solver: QuasiPeriodicSolveConfig {
                relative_tolerance,
                current_absolute_tolerance: self.current_absolute_tolerance,
                voltage_absolute_tolerance: self.voltage_absolute_tolerance,
                max_iterations,
                max_backtracks: self.max_backtracks,
                linear: self.linear.clone(),
            },
            source_tones,
            initial_state: self.initial_state,
            oscillator: self.oscillator.clone(),
        };
        config
            .validate_configuration()
            .map_err(|error| error.to_string())?;
        Ok(config)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpacControls {
    pub magnitude: f64,
    pub phase_degrees: f64,
    pub solver: QuasiPeriodicAcConfig,
    /// Overrides generated-sweep settings; ordering is meaningful.
    pub explicit_offsets: Option<Vec<f64>>,
}

impl Default for QpacControls {
    fn default() -> Self {
        Self {
            magnitude: 1.0,
            phase_degrees: 0.0,
            solver: QuasiPeriodicAcConfig::default(),
            explicit_offsets: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpnoiseControls {
    pub frequency_axis: QpnoiseFrequencyAxis,
    pub explicit_frequencies: Option<Vec<f64>>,
    pub input_referral: bool,
    pub input_lattice: Vec<i32>,
    pub output_lattice: Vec<i32>,
    pub branch_current: Option<String>,
    pub additional_outputs: Vec<QpnoiseOutput>,
    /// None activates the legacy min/max bounds; Some selects any tone window.
    pub noise_lattices: Option<QpnoiseLattices>,
    pub sources: QpnoiseSources,
    pub integration_band: Option<[f64; 2]>,
    pub integration_method: QpnoiseIntegrationMethod,
    pub noise_figure: Option<QpnoiseNoiseFigure>,
    pub solver: QuasiPeriodicLinearConfig,
}
impl Default for QpnoiseControls {
    fn default() -> Self {
        Self {
            frequency_axis: QpnoiseFrequencyAxis::Output,
            explicit_frequencies: None,
            input_referral: true,
            input_lattice: vec![0, 0],
            output_lattice: vec![0, 0],
            branch_current: None,
            additional_outputs: Vec::new(),
            noise_lattices: None,
            sources: QpnoiseSources::All,
            integration_band: None,
            integration_method: QpnoiseIntegrationMethod::Linear,
            noise_figure: None,
            solver: Default::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpxfControls {
    pub frequency_axis: QpxfFrequencyAxis,
    pub explicit_frequencies: Option<Vec<f64>>,
    /// None keeps the original single-source field active.
    pub input_sources: Option<QpxfSources>,
    /// None keeps the original single-input-tuple field active.
    pub input_lattices: Option<QpxfInputLattices>,
    /// Some selects a retained MNA branch current instead of node voltage.
    pub branch_current: Option<String>,
    pub solver: QuasiPeriodicLinearConfig,
    pub group_delay_magnitude_floor: f64,
}
impl Default for QpxfControls {
    fn default() -> Self {
        Self {
            frequency_axis: QpxfFrequencyAxis::Output,
            explicit_frequencies: None,
            input_sources: None,
            input_lattices: None,
            branch_current: None,
            solver: Default::default(),
            group_delay_magnitude_floor: 0.0,
        }
    }
}
