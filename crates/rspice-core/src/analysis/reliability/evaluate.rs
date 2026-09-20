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
    pub equivalent_seconds: f64,
    pub parameters: Vec<AgingParameterChange>,
    /// Linear consumed lifetime, not a failure probability or resistance shift.
    pub electromigration_lifetime_fraction: Option<f64>,
}

/// A clock is bound to an immutable validated fit for its entire lifetime.
#[derive(Debug, Clone)]
pub struct AgingClock<'a> {
    model: &'a AgingModel,
    elapsed_seconds: f64,
    equivalent_seconds: f64,
    equivalent_compensation: f64,
}

impl<'a> AgingClock<'a> {
    pub fn new(model: &'a AgingModel) -> Result<Self, AgingError> {
        model.validate()?;
        Ok(Self {
            model,
            elapsed_seconds: 0.0,
            equivalent_seconds: 0.0,
            equivalent_compensation: 0.0,
        })
    }

    /// Commit one interval. Failed or cancelled advances leave the clock intact.
    /// Reversed/zero gate stress contributes no power-law aging and no recovery.
    /// All observations, including inactive intervals, must be within calibration.
    pub fn advance(
        &mut self,
        duration_s: f64,
        stress: AgingStress,
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
        let elapsed = self.elapsed_seconds + duration_s;
        if !elapsed.is_finite() || (duration_s > 0.0 && elapsed == self.elapsed_seconds) {
            return Err(AgingError::Numeric("elapsed time".into()));
        }
        let increment = if duration_s == 0.0 {
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
        self.elapsed_seconds = elapsed;
        Ok(())
    }

    pub fn evaluate(&self) -> Result<AgingEvaluation, AgingError> {
        self.evaluation_at(self.elapsed_seconds, self.equivalent_seconds)
    }

    fn log_acceleration(&self, stress: AgingStress) -> Option<f64> {
        match &self.model.law {
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
            electromigration_lifetime_fraction: None,
        };
        match &self.model.law {
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
