//! Field-complete driven QPSS settings shared by authoring and execution.
use super::types::{AnalysisSpec, HbToneSpec};
use rspice_core::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicSampling, QuasiPeriodicSolveConfig,
};
use rspice_core::engine::{QpssConfig, QpssInitialState, QpssSourceTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpssControls {
    pub current_absolute_tolerance: f64,
    pub voltage_absolute_tolerance: f64,
    pub max_backtracks: usize,
    pub max_mixing_order: Option<usize>,
    /// One count broadcasts to every tone; otherwise one count per tone.
    pub sampling: QuasiPeriodicSampling,
    pub initial_state: QpssInitialState,
    pub source_tones: Vec<QpssSourceTone>,
}

impl Default for QpssControls {
    fn default() -> Self {
        Self {
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-9,
            max_backtracks: 20,
            max_mixing_order: None,
            sampling: QuasiPeriodicSampling::Oversample(vec![2]),
            initial_state: QpssInitialState::Zero,
            source_tones: Vec::new(),
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
            },
            source_tones,
            initial_state: self.initial_state,
        };
        config
            .validate_configuration()
            .map_err(|error| error.to_string())?;
        Ok(config)
    }
}

impl AnalysisSpec {
    pub(crate) fn from_driven_qpss_config(config: QpssConfig) -> Self {
        let tones = config
            .grid
            .frequencies_hz
            .iter()
            .zip(&config.grid.harmonics)
            .map(|(frequency, harmonics)| HbToneSpec::new(*frequency, *harmonics))
            .collect();
        Self::Qpss {
            tones,
            max_iterations: config.solver.max_iterations,
            relative_tolerance: config.solver.relative_tolerance,
            autonomous: false,
            oscillator_node: None,
            controls: QpssControls {
                current_absolute_tolerance: config.solver.current_absolute_tolerance,
                voltage_absolute_tolerance: config.solver.voltage_absolute_tolerance,
                max_backtracks: config.solver.max_backtracks,
                max_mixing_order: config.grid.max_mixing_order,
                sampling: config.grid.sampling,
                initial_state: config.initial_state,
                source_tones: config.source_tones,
            },
        }
    }

    pub fn driven_qpss_config(&self) -> Result<QpssConfig, String> {
        let Self::Qpss {
            tones,
            max_iterations,
            relative_tolerance,
            autonomous,
            controls,
            ..
        } = self
        else {
            return Err("expected a QPSS specification".into());
        };
        if *autonomous {
            return Err("autonomous QPSS requires an unknown-frequency and phase-condition solve, which is not connected yet".into());
        }
        controls.to_core_config(tones, *max_iterations, *relative_tolerance)
    }
}
