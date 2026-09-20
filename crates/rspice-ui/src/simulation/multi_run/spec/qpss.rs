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
        if tones.len() < 2
            || tones.iter().any(|tone| {
                !tone.frequency.is_finite()
                    || tone.frequency <= 0.0
                    || tone.harmonics == 0
                    || tone.harmonics > i32::MAX as usize
            })
        {
            return Err(
                "QPSS requires at least two finite positive clocks with positive harmonic orders"
                    .into(),
            );
        }
        if tones.iter().enumerate().any(|(index, tone)| {
            tones[..index]
                .iter()
                .any(|other| other.frequency == tone.frequency)
        }) {
            return Err("QPSS tone frequencies must be distinct".into());
        }
        if !relative_tolerance.is_finite()
            || relative_tolerance <= 0.0
            || relative_tolerance >= 1.0
            || !self.current_absolute_tolerance.is_finite()
            || self.current_absolute_tolerance <= 0.0
            || !self.voltage_absolute_tolerance.is_finite()
            || self.voltage_absolute_tolerance <= 0.0
            || max_iterations == 0
            || self.max_backtracks > 60
            || self.max_mixing_order == Some(0)
        {
            return Err("QPSS requires positive finite tolerances, relative tolerance below one, a positive iteration budget, at most 60 backtracks, and a positive optional mixing order".into());
        }
        let (counts, exact) = match &self.sampling {
            QuasiPeriodicSampling::Oversample(counts) => (counts, false),
            QuasiPeriodicSampling::Exact(counts) => (counts, true),
        };
        let counts = if counts.len() == 1 {
            vec![counts[0]; tones.len()]
        } else {
            counts.clone()
        };
        if counts.len() != tones.len() || counts.contains(&0) {
            return Err("QPSS sampling needs one positive count or one count per tone".into());
        }
        if exact
            && counts
                .iter()
                .zip(tones)
                .any(|(count, tone)| *count < tone.harmonics.saturating_mul(2).saturating_add(1))
        {
            return Err("each QPSS phase grid needs at least 2H+1 points".into());
        }
        let mut source_tones = self.source_tones.clone();
        for (tone, specification) in tones.iter().enumerate() {
            if let Some(source) = &specification.source {
                source_tones.push(QpssSourceTone {
                    source: source.clone(),
                    tone,
                });
            }
        }
        let mut seen = std::collections::HashSet::new();
        for binding in &source_tones {
            if binding.source.is_empty()
                || binding.source.trim() != binding.source
                || binding.tone >= tones.len()
                || !seen.insert((binding.source.to_ascii_lowercase(), binding.tone))
            {
                return Err("QPSS source assignments need exact source names, valid tone numbers and no duplicate source/tone pairs".into());
            }
        }
        Ok(QpssConfig {
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
        })
    }
}

impl AnalysisSpec {
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
