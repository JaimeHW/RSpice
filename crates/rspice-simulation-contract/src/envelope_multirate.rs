//! Slow-time Fourier execution and physical envelope result projection.
use rspice_core::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicSampling, QuasiPeriodicSolveConfig,
    SpectralEnvelopeMethod,
};
use rspice_core::engine::{
    EnvelopeCarrierBasis, EnvelopeEventTolerances, QpssSourceTone, SpectralEnvelopeConfig,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeMultirateConfig {
    pub method: SpectralEnvelopeMethod,
    pub adaptive: bool,
    pub minimum_step: f64,
    pub maximum_step: Option<f64>,
    pub relative_tolerance: f64,
    pub voltage_absolute_tolerance: f64,
    pub current_absolute_tolerance: f64,
    pub auxiliary_absolute_tolerance: f64,
    pub maximum_rejections: usize,
    pub maximum_steps: usize,
    /// Empty uses the main harmonic order for each carrier.
    pub harmonics: Vec<usize>,
    /// A single count broadcasts to each carrier. Exact counts override it.
    pub oversample: Vec<usize>,
    pub collocation_points: Vec<usize>,
    pub max_mixing_order: Option<usize>,
    pub source_tones: Vec<QpssSourceTone>,
    pub solver: QuasiPeriodicSolveConfig,
    pub dc_initialization: bool,
    pub source_time_step: Option<f64>,
    pub events: EnvelopeEventTolerances,
}

impl Default for EnvelopeMultirateConfig {
    fn default() -> Self {
        Self {
            method: SpectralEnvelopeMethod::Bdf2,
            adaptive: true,
            minimum_step: 1e-12,
            maximum_step: None,
            relative_tolerance: 1e-3,
            voltage_absolute_tolerance: 1e-6,
            current_absolute_tolerance: 1e-9,
            auxiliary_absolute_tolerance: 1e-9,
            maximum_rejections: 24,
            maximum_steps: 100_000,
            harmonics: Vec::new(),
            oversample: vec![2],
            collocation_points: Vec::new(),
            max_mixing_order: None,
            source_tones: Vec::new(),
            solver: Default::default(),
            dc_initialization: false,
            source_time_step: None,
            events: Default::default(),
        }
    }
}

impl EnvelopeMultirateConfig {
    pub fn core_config(
        &self,
        carriers: &[f64],
        order: usize,
        stop: f64,
        step: f64,
        sources: &[String],
    ) -> Result<SpectralEnvelopeConfig, String> {
        let positive = |v: f64| v.is_finite() && v > 0.0;
        if !positive(step)
            || !positive(self.maximum_step.unwrap_or(step))
            || (self.adaptive
                && (!positive(self.minimum_step)
                    || self.maximum_step.unwrap_or(step) < self.minimum_step
                    || !self.relative_tolerance.is_finite()
                    || !(0.0..1.0).contains(&self.relative_tolerance)
                    || ![
                        self.voltage_absolute_tolerance,
                        self.current_absolute_tolerance,
                        self.auxiliary_absolute_tolerance,
                    ]
                    .into_iter()
                    .all(positive)))
            || self.maximum_steps == 0
        {
            return Err("Multirate Envelope needs positive step bounds and absolute tolerances, relative tolerance in [0,1), and a positive step limit".into());
        }
        self.events.validate().map_err(|e| e.to_string())?;
        if carriers.is_empty() {
            return Err("Envelope requires a carrier frequency".into());
        }
        if carriers.len() == 1 && self.max_mixing_order.is_some() {
            return Err("Envelope mixing-order limits require multiple carriers".into());
        }
        let expand = |values: &[usize], label: &str| -> Result<Vec<usize>, String> {
            let values = if values.len() == 1 {
                vec![values[0]; carriers.len()]
            } else {
                values.to_vec()
            };
            if values.len() != carriers.len() || values.contains(&0) {
                return Err(format!(
                    "Envelope {label} needs one positive count, or a count for each carrier"
                ));
            }
            Ok(values)
        };
        let harmonics = if self.harmonics.is_empty() {
            vec![order; carriers.len()]
        } else {
            expand(&self.harmonics, "harmonic orders")?
        };
        let sampling = if self.collocation_points.is_empty() {
            QuasiPeriodicSampling::Oversample(expand(&self.oversample, "oversampling")?)
        } else {
            QuasiPeriodicSampling::Exact(expand(&self.collocation_points, "collocation points")?)
        };
        let carrier = if carriers.len() == 1 {
            let samples = match &sampling {
                QuasiPeriodicSampling::Exact(points) => points[0],
                QuasiPeriodicSampling::Oversample(factors) => harmonics[0]
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(1))
                    .and_then(|v| v.checked_mul(factors[0]))
                    .and_then(usize::checked_next_power_of_two)
                    .ok_or("Envelope carrier grid overflowed")?,
            };
            EnvelopeCarrierBasis::Periodic {
                frequency_hz: carriers[0],
                harmonics: harmonics[0],
                samples,
            }
        } else {
            EnvelopeCarrierBasis::QuasiPeriodic {
                grid: QuasiPeriodicGridConfig {
                    frequencies_hz: carriers.to_vec(),
                    harmonics,
                    sampling,
                    max_mixing_order: self.max_mixing_order,
                },
            }
        };
        let mut names = std::collections::HashSet::new();
        for source in sources {
            if source.trim().is_empty()
                || source.trim() != source
                || !names.insert(source.to_ascii_lowercase())
            {
                return Err("Envelope modulation sources must be exact and unique".into());
            }
        }
        let mut bindings = std::collections::HashSet::new();
        for binding in &self.source_tones {
            if binding.source.trim().is_empty()
                || binding.source.trim() != binding.source
                || binding.tone >= carriers.len()
                || names.contains(&binding.source.to_ascii_lowercase())
                || !bindings.insert((binding.source.to_ascii_lowercase(), binding.tone))
            {
                return Err("Envelope carrier bindings must name unique source/tone pairs, within the tone list, separate from modulation sources".into());
            }
        }
        let config = SpectralEnvelopeConfig {
            carrier,
            solver: self.solver.clone(),
            source_tones: self.source_tones.clone(),
            modulation_sources: sources.to_vec(),
            stop_time: stop,
            source_time_step: self.source_time_step.unwrap_or(step),
        };
        config.validate().map_err(|e| e.to_string())?;
        Ok(config)
    }
}
