//! Two-state master equations and chronological affine history composition.
//!
//! dp/dt = capture*(1-p) - emission*p. Constant intervals are exact, and
//! repeated histories use the affine geometric series without iterating over
//! mission cycles. See Rzepa, *Efficient Physical Modeling of Bias Temperature
//! Instability*, 2018, section 4.5, equations 4.50-4.56:
//! https://www.iue.tuwien.ac.at/phd/rzepa/
//! This kinetic adapter does not supply a process calibration or electrostatics.

use super::*;
use crate::abort_signal::AbortSignal;
use std::collections::BTreeMap;

const MAX_TRAPS: usize = 1024;
const MAX_RATE_VALUES: usize = 131072;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgingRateInterpolation {
    /// Interpolate rates; permits exact zero capture or emission.
    Linear,
    /// Interpolate log rates; every table entry must be strictly positive.
    Logarithmic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingTrapParameter {
    pub parameter: String,
    pub update: AgingParameterUpdate,
    /// Shift for a unit change in this population's occupancy. Includes the
    /// population density/weight and coupling to the exact compact parameter.
    /// The actual shift is this value times (occupancy - initial_occupancy).
    pub shift_per_occupancy: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingTrap {
    pub id: String,
    pub initial_occupancy: f64,
    /// Per-second rates; Vgs is the fastest varying index, then Vds, then T.
    pub capture_rates_per_s: Vec<f64>,
    pub emission_rates_per_s: Vec<f64>,
    pub parameters: Vec<AgingTrapParameter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingTrapTable {
    /// Strictly increasing signed voltages; endpoints equal model validity.
    pub gate_source_v: Vec<f64>,
    pub drain_source_v: Vec<f64>,
    /// Strictly increasing Kelvin temperatures. Interpolation uses 1/T, so
    /// logarithmic rates reproduce Arrhenius behavior between temperature knots.
    pub temperature_k: Vec<f64>,
    pub interpolation: AgingRateInterpolation,
    pub traps: Vec<AgingTrap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingTrapOccupancy {
    pub trap_id: String,
    pub occupancy: f64,
}

impl AgingTrapTable {
    pub(super) fn validate(&self, validity: &AgingValidity) -> Result<(), AgingError> {
        for (axis, range, label) in [
            (&self.gate_source_v, validity.gate_source_v, "Vgs"),
            (&self.drain_source_v, validity.drain_source_v, "Vds"),
            (&self.temperature_k, validity.temperature_k, "temperature"),
        ] {
            if axis.is_empty()
                || axis.len() > MAX_RATE_VALUES
                || axis.iter().any(|v| !v.is_finite())
                || axis
                    .windows(2)
                    .any(|w| w[0] >= w[1] || !(w[1] - w[0]).is_finite())
                || axis.first() != Some(&range.min)
                || axis.last() != Some(&range.max)
            {
                return Err(AgingError::Invalid(format!(
                    "trap {label} knots must be increasing and cover exactly the calibration bounds"
                )));
            }
        }
        let count = self
            .gate_source_v
            .len()
            .saturating_mul(self.drain_source_v.len())
            .saturating_mul(self.temperature_k.len());
        if self.traps.is_empty()
            || self.traps.len() > MAX_TRAPS
            || count.saturating_mul(self.traps.len()).saturating_mul(2) > MAX_RATE_VALUES
        {
            return Err(AgingError::Invalid(
                "a trapping table needs 1..=1024 populations and at most 131072 rate values".into(),
            ));
        }
        let mut ids = std::collections::HashSet::new();
        let mut modes = BTreeMap::new();
        for trap in &self.traps {
            if trap.id.is_empty()
                || trap.id.len() > 128
                || trap.id.chars().any(|c| c.is_control())
                || !ids.insert(&trap.id)
                || !trap.initial_occupancy.is_finite()
                || !(0.0..=1.0).contains(&trap.initial_occupancy)
            {
                return Err(AgingError::Invalid(
                    "trap IDs must be unique and occupancies in [0,1]".into(),
                ));
            }
            for rates in [&trap.capture_rates_per_s, &trap.emission_rates_per_s] {
                if rates.len() != count
                    || rates.iter().any(|r| {
                        !r.is_finite()
                            || *r < 0.0
                            || (self.interpolation == AgingRateInterpolation::Logarithmic
                                && *r == 0.0)
                    })
                {
                    return Err(AgingError::Invalid("trap rates must match the grid and be finite, nonnegative (positive for logarithmic interpolation)".into()));
                }
            }
            if trap.parameters.is_empty() || trap.parameters.len() > 128 {
                return Err(AgingError::Invalid(
                    "a trap needs 1..=128 parameter couplings".into(),
                ));
            }
            let mut names = std::collections::HashSet::new();
            for p in &trap.parameters {
                super::validation::parameter(&p.parameter, p.shift_per_occupancy)?;
                let name = p.parameter.to_ascii_lowercase();
                if !names.insert(name.clone())
                    || modes.insert(name, p.update).is_some_and(|m| m != p.update)
                {
                    return Err(AgingError::Invalid("trap parameter couplings are duplicated or mix additive and relative updates".into()));
                }
            }
        }
        Ok(())
    }

    fn rates(&self, trap: &AgingTrap, stress: AgingStress) -> (f64, f64) {
        let axes = [
            bracket(&self.gate_source_v, stress.gate_source_v, false),
            bracket(&self.drain_source_v, stress.drain_source_v, false),
            bracket(&self.temperature_k, stress.temperature_k, true),
        ];
        let interpolate = |rates: &[f64]| {
            let mut sum = 0.0;
            for &(t, tw) in &axes[2] {
                for &(d, dw) in &axes[1] {
                    for &(g, gw) in &axes[0] {
                        let weight = tw * dw * gw;
                        if weight == 0.0 {
                            continue;
                        }
                        let value = rates
                            [(t * self.drain_source_v.len() + d) * self.gate_source_v.len() + g];
                        sum += weight
                            * match self.interpolation {
                                AgingRateInterpolation::Linear => value,
                                AgingRateInterpolation::Logarithmic => value.ln(),
                            };
                    }
                }
            }
            match self.interpolation {
                AgingRateInterpolation::Linear => sum,
                AgingRateInterpolation::Logarithmic => sum.exp(),
            }
        };
        (
            interpolate(&trap.capture_rates_per_s),
            interpolate(&trap.emission_rates_per_s),
        )
    }
}

fn bracket(axis: &[f64], value: f64, reciprocal: bool) -> [(usize, f64); 2] {
    match axis.binary_search_by(|v| v.partial_cmp(&value).expect("validated finite stress")) {
        Ok(i) => [(i, 1.0), (i, 0.0)],
        Err(i) => {
            // Calibration validation precedes every use, so no extrapolation.
            let a = axis[i - 1];
            let b = axis[i];
            let w = if reciprocal {
                ((value - a) / (b - a)) * (b / value)
            } else {
                (value - a) / (b - a)
            };
            [(i - 1, 1.0 - w), (i, w)]
        }
    }
}

/// An affine map p_out = exp(-decay)*p_in + capture. Keeping decay rather
/// than exp(-decay) preserves very slow traps even across trillions of cycles.
#[derive(Debug, Clone, Copy, Default)]
struct Transition {
    decay: f64,
    capture: f64,
}

impl Transition {
    fn interval(capture: f64, emission: f64, duration: f64) -> Result<Self, AgingError> {
        let scale = capture.max(emission);
        if scale == 0.0 || duration == 0.0 {
            return Ok(Self::default());
        }
        let total = capture / scale + emission / scale;
        let equilibrium = (capture / scale) / total;
        let decay = (scale * duration) * total;
        if decay == 0.0 {
            return Err(AgingError::Numeric(
                "trap transition exposure underflow".into(),
            ));
        }
        Ok(Self {
            decay,
            capture: equilibrium * -(-decay).exp_m1(),
        })
    }

    fn then(self, next: Self) -> Self {
        let decay = self.decay + next.decay;
        let capture = self.capture * (-next.decay).exp() + next.capture;
        Self {
            decay,
            capture: capture.min(-(-decay).exp_m1()),
        }
    }

    fn repeated(self, count: f64) -> Self {
        if count == 0.0 || self.decay == 0.0 {
            return Self::default();
        }
        let decay = self.decay * count;
        Self {
            decay,
            capture: (self.capture / -(-self.decay).exp_m1()) * -(-decay).exp_m1(),
        }
    }

    fn apply(self, initial: f64) -> f64 {
        (initial * (-self.decay).exp() + self.capture).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub(super) struct TrapHistory {
    transitions: Vec<Transition>,
}

impl TrapHistory {
    pub(super) fn new(table: &AgingTrapTable) -> Self {
        Self {
            transitions: vec![Transition::default(); table.traps.len()],
        }
    }

    pub(super) fn advance(
        &mut self,
        table: &AgingTrapTable,
        duration: f64,
        stress: AgingStress,
        active: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), AgingError> {
        for (transition, trap) in self.transitions.iter_mut().zip(&table.traps) {
            if abort.is_aborted() {
                return Err(AgingError::Aborted);
            }
            let (capture, emission) = table.rates(trap, stress);
            if !capture.is_finite() || !emission.is_finite() {
                return Err(AgingError::Numeric("interpolated trap rates".into()));
            }
            // The explicit minimum-stress threshold suppresses capture only;
            // emission/recovery proceeds at the observed bias and temperature.
            *transition = transition.then(Transition::interval(
                if active { capture } else { 0.0 },
                emission,
                duration,
            )?);
        }
        Ok(())
    }

    pub(super) fn append(
        &mut self,
        history: &Self,
        count: f64,
        abort: &dyn AbortSignal,
    ) -> Result<(), AgingError> {
        for (a, b) in self.transitions.iter_mut().zip(&history.transitions) {
            if abort.is_aborted() {
                return Err(AgingError::Aborted);
            }
            *a = a.then(b.repeated(count));
        }
        Ok(())
    }

    pub(super) fn evaluate(
        &self,
        table: &AgingTrapTable,
        result: &mut AgingEvaluation,
    ) -> Result<(), AgingError> {
        let mut shifts = BTreeMap::new();
        for (transition, trap) in self.transitions.iter().zip(&table.traps) {
            let occupancy = transition.apply(trap.initial_occupancy);
            result.trap_occupancies.push(AgingTrapOccupancy {
                trap_id: trap.id.clone(),
                occupancy,
            });
            for parameter in &trap.parameters {
                let entry = shifts
                    .entry(parameter.parameter.to_ascii_uppercase())
                    .or_insert((parameter.update, 0.0));
                entry.1 += (occupancy - trap.initial_occupancy) * parameter.shift_per_occupancy;
            }
        }
        for (parameter, (update, shift)) in shifts {
            if !shift.is_finite() || (update == AgingParameterUpdate::Relative && shift <= -1.0) {
                return Err(AgingError::Numeric(format!(
                    "trap-induced {parameter} shift"
                )));
            }
            result.parameters.push(AgingParameterChange {
                parameter,
                update,
                shift,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
