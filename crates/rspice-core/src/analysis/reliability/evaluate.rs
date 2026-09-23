//! Stateful effective-age integration with checked engineering coordinates.

use super::*;
use crate::abort_signal::AbortSignal;

/// Boltzmann constant in eV/K from the exact SI constants.
const BOLTZMANN_EV_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

/// One constant-stress interval, supplied by a circuit or characterized trace.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingStress {
    pub gate_source_v: f64,
    pub drain_source_v: f64,
    pub temperature_k: f64,
    /// Magnitude; callers must provide a physical conductor cross section.
    pub current_density_a_per_m2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingParameterChange {
    pub parameter: String,
    pub update: AgingParameterUpdate,
    pub shift: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingEvaluation {
    pub model_id: String,
    pub mechanism: AgingMechanism,
    pub elapsed_seconds: f64,
    /// Effective reference exposure for irreversible laws; elapsed history for
    /// trapping tables, which have no single equivalent-age coordinate.
    pub equivalent_seconds: f64,
    pub parameters: Vec<AgingParameterChange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trap_occupancies: Vec<AgingTrapOccupancy>,
    /// Linear consumed lifetime, not a failure probability or resistance shift.
    pub electromigration_lifetime_fraction: Option<f64>,
}

/// A clock is bound to an immutable validated fit for its entire lifetime.
#[derive(Debug, Clone)]
pub struct AgingClock<'a> {
    model: &'a AgingModel,
    elapsed_seconds: f64,
    elapsed_compensation: f64,
    equivalent_seconds: f64,
    equivalent_compensation: f64,
    traps: Option<super::traps::TrapHistory>,
}

impl<'a> AgingClock<'a> {
    pub fn new(model: &'a AgingModel) -> Result<Self, AgingError> {
        model.validate()?;
        Ok(Self {
            model,
            elapsed_seconds: 0.0,
            elapsed_compensation: 0.0,
            equivalent_seconds: 0.0,
            equivalent_compensation: 0.0,
            traps: match &model.law {
                AgingLaw::TabulatedTwoState { table } => {
                    Some(super::traps::TrapHistory::new(table))
                }
                _ => None,
            },
        })
    }

    /// Commit one interval. Failed or cancelled advances leave the clock intact.
    /// Reversed/zero gate stress contributes no power-law aging. Trapping tables
    /// use their characterized rates at every bias, including recovery bias.
    /// All observations, including inactive intervals, must be within calibration.
    pub fn advance(
        &mut self,
        duration_s: f64,
        stress: AgingStress,
        abort: &dyn AbortSignal,
    ) -> Result<(), AgingError> {
        self.advance_with_activity(duration_s, stress, true, abort)
    }

    /// An explicit circuit-analysis stress threshold can suspend the clock
    /// without fabricating zero terminal voltages or removing elapsed time.
    /// Inactive stress still undergoes the full calibration-domain checks.
    pub fn advance_with_activity(
        &mut self,
        duration_s: f64,
        stress: AgingStress,
        active: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), AgingError> {
        if abort.is_aborted() {
            return Err(AgingError::Aborted);
        }
        if !duration_s.is_finite() || duration_s < 0.0 {
            return Err(AgingError::Invalid(
                "stress duration must be finite and nonnegative".into(),
            ));
        }
        let valid = &self.model.validity;
        valid
            .gate_source_v
            .require(stress.gate_source_v, "Vgs (V)")?;
        valid
            .drain_source_v
            .require(stress.drain_source_v, "Vds (V)")?;
        valid
            .temperature_k
            .require(stress.temperature_k, "temperature (K)")?;
        valid
            .current_density_a_per_m2
            .require(stress.current_density_a_per_m2, "current density (A/m²)")?;
        let elapsed_increment = duration_s - self.elapsed_compensation;
        let elapsed = self.elapsed_seconds + elapsed_increment;
        if !elapsed.is_finite() {
            return Err(AgingError::Numeric("elapsed time".into()));
        }
        if let AgingLaw::TabulatedTwoState { table } = &self.model.law {
            let mut candidate = self.clone();
            candidate
                .traps
                .as_mut()
                .expect("trapping clock")
                .advance(table, duration_s, stress, active, abort)?;
            candidate.set_trapping_elapsed(elapsed, elapsed_increment)?;
            if abort.is_aborted() {
                return Err(AgingError::Aborted);
            }
            *self = candidate;
            return Ok(());
        }
        let increment = if duration_s == 0.0 || !active {
            0.0
        } else if let Some(log_acceleration) = self.log_acceleration(stress) {
            if log_acceleration == 0.0 {
                // Preserve exact reference intervals, particularly the declared
                // last calibrated second, without a round trip through exp/ln.
                duration_s
            } else {
                checked_exp(
                    duration_s.ln() + log_acceleration,
                    "equivalent exposure increment",
                )?
            }
        } else {
            0.0
        };
        let corrected = increment - self.equivalent_compensation;
        let equivalent = self.equivalent_seconds + corrected;
        if !equivalent.is_finite() || equivalent < 0.0 {
            return Err(AgingError::Numeric(
                "accumulated equivalent exposure".into(),
            ));
        }
        if equivalent > valid.max_equivalent_seconds {
            return Err(AgingError::OutsideCalibration(format!(
                "equivalent exposure {equivalent} s exceeds {} s",
                valid.max_equivalent_seconds
            )));
        }
        // Reject an unrepresentable/invalid parameter update before changing state.
        self.evaluation_at(elapsed, equivalent)?;
        if abort.is_aborted() {
            return Err(AgingError::Aborted);
        }
        self.equivalent_compensation = (equivalent - self.equivalent_seconds) - corrected;
        self.equivalent_seconds = equivalent;
        self.elapsed_compensation = (elapsed - self.elapsed_seconds) - elapsed_increment;
        self.elapsed_seconds = elapsed;
        Ok(())
    }

    pub fn evaluate(&self) -> Result<AgingEvaluation, AgingError> {
        self.evaluation_at(self.elapsed_seconds, self.equivalent_seconds)
    }

    /// Append a complete chronological trapping history an integral number of
    /// times without enumerating cycles. Both histories must use this exact
    /// immutable model. The appended history starts from the current occupancy,
    /// not from its own initial occupancy. Failed/cancelled appends are atomic.
    /// f64 represents large integral repetition counts without a u64 limit.
    pub fn append_repeated_history(
        &mut self,
        history: &Self,
        repetitions: f64,
        abort: &dyn AbortSignal,
    ) -> Result<(), AgingError> {
        if abort.is_aborted() {
            return Err(AgingError::Aborted);
        }
        if !std::ptr::eq(self.model, history.model)
            || self.traps.is_none()
            || !repetitions.is_finite()
            || repetitions < 0.0
            || repetitions.fract() != 0.0
        {
            return Err(AgingError::Invalid("repeated history requires the same trapping model and a nonnegative integral repetition count".into()));
        }
        let increment = history.elapsed_seconds * repetitions - self.elapsed_compensation;
        let elapsed = self.elapsed_seconds + increment;
        if !elapsed.is_finite() {
            return Err(AgingError::Numeric("repeated trapping duration".into()));
        }
        let mut candidate = self.clone();
        candidate.traps.as_mut().unwrap().append(
            history.traps.as_ref().unwrap(),
            repetitions,
            abort,
        )?;
        candidate.set_trapping_elapsed(elapsed, increment)?;
        if abort.is_aborted() {
            return Err(AgingError::Aborted);
        }
        *self = candidate;
        Ok(())
    }

    fn set_trapping_elapsed(&mut self, elapsed: f64, increment: f64) -> Result<(), AgingError> {
        if elapsed > self.model.validity.max_equivalent_seconds {
            return Err(AgingError::OutsideCalibration(format!(
                "trapping history {elapsed} s exceeds {} s",
                self.model.validity.max_equivalent_seconds
            )));
        }
        self.elapsed_compensation = (elapsed - self.elapsed_seconds) - increment;
        self.elapsed_seconds = elapsed;
        self.equivalent_seconds = elapsed;
        self.evaluate()?;
        Ok(())
    }

    fn log_acceleration(&self, stress: AgingStress) -> Option<f64> {
        match &self.model.law {
            AgingLaw::TabulatedTwoState { .. } => None,
            AgingLaw::EquivalentTimePower {
                reference_gate_magnitude_v,
                reference_drain_magnitude_v,
                reference_temperature_k,
                gate_polarity,
                clock_gate_exponent,
                clock_drain_exponent,
                clock_activation_energy_ev,
                ..
            } => {
                let active = match gate_polarity {
                    AgingGatePolarity::Positive => stress.gate_source_v > 0.0,
                    AgingGatePolarity::Negative => stress.gate_source_v < 0.0,
                    AgingGatePolarity::Either => stress.gate_source_v != 0.0,
                };
                if !active || (*clock_drain_exponent > 0.0 && stress.drain_source_v == 0.0) {
                    return None;
                }
                Some(
                    log_power(
                        stress.gate_source_v.abs(),
                        *reference_gate_magnitude_v,
                        *clock_gate_exponent,
                    ) + log_power(
                        stress.drain_source_v.abs(),
                        *reference_drain_magnitude_v,
                        *clock_drain_exponent,
                    ) + thermal_clock(
                        *clock_activation_energy_ev,
                        *reference_temperature_k,
                        stress.temperature_k,
                    ),
                )
            }
            AgingLaw::BlackElectromigration {
                reference_current_density_a_per_m2,
                reference_temperature_k,
                current_exponent,
                activation_energy_ev,
                ..
            } => (stress.current_density_a_per_m2 > 0.0).then(|| {
                log_power(
                    stress.current_density_a_per_m2,
                    *reference_current_density_a_per_m2,
                    *current_exponent,
                ) + thermal_clock(
                    *activation_energy_ev,
                    *reference_temperature_k,
                    stress.temperature_k,
                )
            }),
        }
    }

    fn evaluation_at(
        &self,
        elapsed_seconds: f64,
        equivalent_seconds: f64,
    ) -> Result<AgingEvaluation, AgingError> {
        let mut result = AgingEvaluation {
            model_id: self.model.id.clone(),
            mechanism: self.model.mechanism,
            elapsed_seconds,
            equivalent_seconds,
            parameters: Vec::new(),
            trap_occupancies: Vec::new(),
            electromigration_lifetime_fraction: None,
        };
        match &self.model.law {
            AgingLaw::TabulatedTwoState { table } => {
                self.traps
                    .as_ref()
                    .expect("trapping clock")
                    .evaluate(table, &mut result)?;
            }
            AgingLaw::EquivalentTimePower {
                reference_time_s,
                time_exponent,
                parameters,
                ..
            } => {
                for parameter in parameters {
                    let scale = parameter.scale_at_reference_time;
                    let shift = if equivalent_seconds == 0.0 {
                        0.0
                    } else {
                        checked_exp(
                            scale.abs().ln()
                                + time_exponent * (equivalent_seconds.ln() - reference_time_s.ln()),
                            "parameter shift",
                        )?
                        .copysign(scale)
                    };
                    if parameter.update == AgingParameterUpdate::Relative && shift <= -1.0 {
                        return Err(AgingError::Numeric(format!(
                            "relative {} shift would remove or reverse its fresh value",
                            parameter.parameter
                        )));
                    }
                    result.parameters.push(AgingParameterChange {
                        parameter: parameter.parameter.clone(),
                        update: parameter.update,
                        shift,
                    });
                }
            }
            AgingLaw::BlackElectromigration {
                reference_lifetime_s,
                ..
            } => {
                result.electromigration_lifetime_fraction = Some(if equivalent_seconds == 0.0 {
                    0.0
                } else {
                    checked_exp(
                        equivalent_seconds.ln() - reference_lifetime_s.ln(),
                        "consumed EM lifetime",
                    )?
                });
            }
        }
        Ok(result)
    }
}

fn thermal_clock(energy_ev: f64, reference_k: f64, temperature_k: f64) -> f64 {
    energy_ev * (1.0 / reference_k - 1.0 / temperature_k) / BOLTZMANN_EV_PER_K
}

fn log_power(value: f64, reference: f64, exponent: f64) -> f64 {
    if exponent == 0.0 {
        0.0
    } else {
        exponent * (value.ln() - reference.ln())
    }
}

fn checked_exp(log_value: f64, label: &str) -> Result<f64, AgingError> {
    let value = log_value.exp();
    if !log_value.is_finite() || !value.is_finite() || value == 0.0 {
        Err(AgingError::Numeric(label.into()))
    } else {
        Ok(value)
    }
}
