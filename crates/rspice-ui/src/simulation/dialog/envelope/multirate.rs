use super::*;
use crate::services::simulation_runner::EnvelopeMultirateConfig;
use rspice_core::analysis::quasi_periodic::{QuasiPeriodicLinearMethod, SpectralEnvelopeMethod};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeMultirateState {
    pub adaptive: bool,
    pub bdf2: bool,
    pub minimum_step: String,
    pub maximum_step: String,
    pub relative_tolerance: String,
    pub voltage_absolute_tolerance: String,
    pub current_absolute_tolerance: String,
    pub auxiliary_absolute_tolerance: String,
    pub maximum_rejections: String,
    pub maximum_steps: String,
    pub harmonics: String,
    pub oversample: String,
    pub collocation_points: String,
    pub max_mixing_order: String,
    pub source_tones: String,
    pub dc_initialization: bool,
    pub source_time_step: String,
    pub solver_relative: String,
    pub solver_current: String,
    pub solver_voltage: String,
    pub solver_iterations: String,
    pub solver_backtracks: String,
    pub linear_method: QuasiPeriodicLinearMethod,
    pub linear_restart: String,
    pub linear_cycles: String,
    pub linear_relative: String,
    pub event_charge: String,
    pub event_flux: String,
    pub event_current: String,
    pub event_voltage: String,
    pub event_current_rate: String,
    pub event_voltage_rate: String,
}

impl Default for EnvelopeMultirateState {
    fn default() -> Self {
        Self::from_config(&EnvelopeMultirateConfig::default())
    }
}

impl EnvelopeMultirateState {
    pub fn from_config(c: &EnvelopeMultirateConfig) -> Self {
        let counts = |v: &[usize]| {
            v.iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        };
        Self {
            adaptive: c.adaptive,
            bdf2: c.method == SpectralEnvelopeMethod::Bdf2,
            minimum_step: c.minimum_step.to_string(),
            maximum_step: c.maximum_step.map(|v| v.to_string()).unwrap_or_default(),
            relative_tolerance: c.relative_tolerance.to_string(),
            voltage_absolute_tolerance: c.voltage_absolute_tolerance.to_string(),
            current_absolute_tolerance: c.current_absolute_tolerance.to_string(),
            auxiliary_absolute_tolerance: c.auxiliary_absolute_tolerance.to_string(),
            maximum_rejections: c.maximum_rejections.to_string(),
            maximum_steps: c.maximum_steps.to_string(),
            harmonics: counts(&c.harmonics),
            oversample: counts(&c.oversample),
            collocation_points: counts(&c.collocation_points),
            max_mixing_order: c
                .max_mixing_order
                .map(|v| v.to_string())
                .unwrap_or_default(),
            source_tones: c
                .source_tones
                .iter()
                .map(|v| format!("{}={}", v.source, v.tone + 1))
                .collect::<Vec<_>>()
                .join(", "),
            dc_initialization: c.dc_initialization,
            source_time_step: c
                .source_time_step
                .map(|v| v.to_string())
                .unwrap_or_default(),
            solver_relative: c.solver.relative_tolerance.to_string(),
            solver_current: c.solver.current_absolute_tolerance.to_string(),
            solver_voltage: c.solver.voltage_absolute_tolerance.to_string(),
            solver_iterations: c.solver.max_iterations.to_string(),
            solver_backtracks: c.solver.max_backtracks.to_string(),
            linear_method: c.solver.linear.method,
            linear_restart: c.solver.linear.restart.to_string(),
            linear_cycles: c.solver.linear.max_cycles.to_string(),
            linear_relative: c.solver.linear.relative_tolerance.to_string(),
            event_charge: c.events.charge_coulombs.to_string(),
            event_flux: c.events.flux_webers.to_string(),
            event_current: c.events.current_amperes.to_string(),
            event_voltage: c.events.voltage_volts.to_string(),
            event_current_rate: c.events.current_rate_amperes_per_second.to_string(),
            event_voltage_rate: c.events.voltage_rate_volts_per_second.to_string(),
        }
    }

    pub fn to_config(&self, carrier_count: usize) -> Result<EnvelopeMultirateConfig, String> {
        let number = |text: &str, label: &str| {
            parse_si_value(text).map_err(|e| format!("Invalid Envelope {label}: {e}"))
        };
        let count = |text: &str, label: &str| {
            text.trim()
                .parse::<usize>()
                .map_err(|_| format!("Envelope {label} must be a nonnegative integer"))
        };
        let optional = |text: &str, label: &str| {
            if text.trim().is_empty() {
                Ok(None)
            } else {
                number(text, label).map(Some)
            }
        };
        let counts = |text: &str, label: &str| -> Result<Vec<usize>, String> {
            if text.trim().is_empty() {
                return Ok(Vec::new());
            }
            text.split(',').map(|v| count(v, label)).collect()
        };
        let mut c = EnvelopeMultirateConfig::default();
        c.adaptive = self.adaptive;
        c.method = if self.bdf2 {
            SpectralEnvelopeMethod::Bdf2
        } else {
            SpectralEnvelopeMethod::BackwardEuler
        };
        c.maximum_step = optional(&self.maximum_step, "maximum step")?;
        c.maximum_steps = count(&self.maximum_steps, "step limit")?;
        if self.adaptive {
            c.minimum_step = number(&self.minimum_step, "minimum step")?;
            c.relative_tolerance = number(&self.relative_tolerance, "relative error tolerance")?;
            c.voltage_absolute_tolerance =
                number(&self.voltage_absolute_tolerance, "voltage error tolerance")?;
            c.current_absolute_tolerance =
                number(&self.current_absolute_tolerance, "current error tolerance")?;
            c.auxiliary_absolute_tolerance = number(
                &self.auxiliary_absolute_tolerance,
                "auxiliary error tolerance",
            )?;
            c.maximum_rejections = count(&self.maximum_rejections, "rejection limit")?;
        }
        c.harmonics = counts(&self.harmonics, "harmonic orders")?;
        c.collocation_points = counts(&self.collocation_points, "collocation points")?;
        if c.collocation_points.is_empty() {
            c.oversample = counts(&self.oversample, "oversampling")?;
        }
        c.max_mixing_order = if carrier_count < 2 || self.max_mixing_order.trim().is_empty() {
            None
        } else {
            Some(count(&self.max_mixing_order, "mixing order")?)
        };
        for binding in self
            .source_tones
            .split([',', ';', '\n'])
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let (name, tone) = binding
                .rsplit_once('=')
                .ok_or("Envelope source assignments use source=tone, for example Vrf=1")?;
            c.source_tones.push(rspice_core::engine::QpssSourceTone {
                source: name.trim().into(),
                tone: count(tone, "source tone")?
                    .checked_sub(1)
                    .ok_or("Envelope source tone numbers start at 1")?,
            });
        }
        c.dc_initialization = self.dc_initialization;
        c.source_time_step = optional(&self.source_time_step, "waveform default step")?;
        c.solver.relative_tolerance = number(&self.solver_relative, "Newton relative tolerance")?;
        c.solver.current_absolute_tolerance =
            number(&self.solver_current, "Newton current tolerance")?;
        c.solver.voltage_absolute_tolerance =
            number(&self.solver_voltage, "Newton voltage tolerance")?;
        c.solver.max_iterations = count(&self.solver_iterations, "Newton iterations")?;
        c.solver.max_backtracks = count(&self.solver_backtracks, "Newton backtracks")?;
        c.solver.linear.method = self.linear_method;
        if self.linear_method != QuasiPeriodicLinearMethod::Direct {
            c.solver.linear.restart = count(&self.linear_restart, "Krylov restart")?;
            c.solver.linear.max_cycles = count(&self.linear_cycles, "Krylov cycles")?;
            c.solver.linear.relative_tolerance =
                number(&self.linear_relative, "linear relative tolerance")?;
        }
        c.events.charge_coulombs = number(&self.event_charge, "event charge tolerance")?;
        c.events.flux_webers = number(&self.event_flux, "event flux tolerance")?;
        c.events.current_amperes = number(&self.event_current, "event current tolerance")?;
        c.events.voltage_volts = number(&self.event_voltage, "event voltage tolerance")?;
        c.events.current_rate_amperes_per_second =
            number(&self.event_current_rate, "event current-rate tolerance")?;
        c.events.voltage_rate_volts_per_second =
            number(&self.event_voltage_rate, "event voltage-rate tolerance")?;
        Ok(c)
    }
}
