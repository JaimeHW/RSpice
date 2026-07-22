//! Numerical capability contracts between dependent analyses.
//!
//! The plan editor and immutable execution boundary use the same predicates so
//! a dependency accepted by quick repair cannot later fail for a different
//! interpretation of its sampling or time-window requirements.

/// Execution-relevant capability published by a PSS prerequisite.
///
/// Keep this deliberately independent of both dialog state and prepared
/// `AnalysisSpec` so the editable plan and immutable execution boundary use
/// the same compatibility predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::simulation) struct PeriodicStateCapability {
    pub shooting: bool,
    pub autonomous: bool,
}

/// Validate the execution contract shared by PAC, PXF, PNOISE, and PSTB.
pub(in crate::simulation) fn validate_periodic_state_contract(
    consumer: &str,
    capability: PeriodicStateCapability,
    require_autonomous: bool,
) -> Result<(), String> {
    if !capability.shooting {
        return Err(format!(
            "{consumer} requires a shooting-PSS periodic-state artifact; harmonic-balance PSS does not retain the shooting state and monodromy contract"
        ));
    }
    if require_autonomous && !capability.autonomous {
        return Err(format!(
            "{consumer} phase-noise analysis requires an autonomous producer PSS state"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::simulation) struct FourierTransientRequirement {
    pub start_time: f64,
    pub stop_time: f64,
    pub fundamental_freq: f64,
    pub num_harmonics: u32,
}

impl FourierTransientRequirement {
    pub fn required_sample_interval(self) -> Result<f64, String> {
        self.validate()?;
        let maximum_frequency = self.fundamental_freq * (f64::from(self.num_harmonics) + 1.0);
        Ok(1.0 / (maximum_frequency * 8.0))
    }

    fn validate(self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err(
                "Fourier fundamental frequency must be finite and greater than zero".to_owned(),
            );
        }
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            return Err("Fourier start time must be finite and non-negative".to_owned());
        }
        if !self.stop_time.is_finite() || self.stop_time <= self.start_time {
            return Err("Fourier stop time must be finite and after its start time".to_owned());
        }
        let period = 1.0 / self.fundamental_freq;
        if self.stop_time - self.start_time < period {
            return Err("Fourier window must contain at least one fundamental period".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::simulation) struct TransientCapability {
    pub start_time: f64,
    pub stop_time: f64,
    pub step_time: f64,
    pub max_timestep: Option<f64>,
}

impl TransientCapability {
    fn validate(self) -> Result<(), String> {
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            return Err("Transient start time must be finite and non-negative".to_owned());
        }
        if !self.stop_time.is_finite() || self.stop_time <= self.start_time {
            return Err("Transient stop time must be finite and after its start time".to_owned());
        }
        if !self.step_time.is_finite() || self.step_time <= 0.0 {
            return Err(
                "Transient sample interval must be finite and greater than zero".to_owned(),
            );
        }
        if self
            .max_timestep
            .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            return Err(
                "Transient maximum timestep must be finite and greater than zero when set"
                    .to_owned(),
            );
        }
        Ok(())
    }
}

pub(in crate::simulation) fn validate_fourier_transient_contract(
    requirement: FourierTransientRequirement,
    capability: TransientCapability,
) -> Result<(), String> {
    requirement.validate()?;
    capability.validate()?;

    let scale = requirement
        .stop_time
        .abs()
        .max(capability.stop_time.abs())
        .max(1.0);
    let tolerance = 16.0 * f64::EPSILON * scale;
    if requirement.start_time + tolerance < capability.start_time
        || requirement.stop_time > capability.stop_time + tolerance
    {
        return Err(format!(
            "Fourier window [{:.12e}, {:.12e}] is outside the transient window [{:.12e}, {:.12e}]",
            requirement.start_time,
            requirement.stop_time,
            capability.start_time,
            capability.stop_time
        ));
    }

    let available_interval = capability
        .max_timestep
        .map_or(capability.step_time, |limit| {
            limit.min(capability.step_time)
        });
    let required_interval = requirement.required_sample_interval()?;
    if available_interval > required_interval * (1.0 + 16.0 * f64::EPSILON) {
        return Err(format!(
            "Transient sample interval {available_interval:.12e}s is too coarse for the requested Fourier basis; use {required_interval:.12e}s or less"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_rejects_a_coarse_or_short_transient() {
        let requirement = FourierTransientRequirement {
            start_time: 1.0e-6,
            stop_time: 11.0e-6,
            fundamental_freq: 1.0e6,
            num_harmonics: 10,
        };
        let compatible = TransientCapability {
            start_time: 0.0,
            stop_time: 20.0e-6,
            step_time: 10.0e-9,
            max_timestep: None,
        };
        validate_fourier_transient_contract(requirement, compatible)
            .expect("adequate time coverage and sampling are accepted");

        let coarse = TransientCapability {
            step_time: 20.0e-9,
            ..compatible
        };
        assert!(validate_fourier_transient_contract(requirement, coarse).is_err());

        let short = TransientCapability {
            stop_time: 10.0e-6,
            ..compatible
        };
        assert!(validate_fourier_transient_contract(requirement, short).is_err());
    }

    #[test]
    fn periodic_contract_rejects_hb_and_driven_phase_noise_producers() {
        let driven_shooting = PeriodicStateCapability {
            shooting: true,
            autonomous: false,
        };
        validate_periodic_state_contract("PAC", driven_shooting, false)
            .expect("PAC accepts a driven shooting state");
        assert!(
            validate_periodic_state_contract(
                "PAC",
                PeriodicStateCapability {
                    shooting: false,
                    autonomous: false,
                },
                false,
            )
            .unwrap_err()
            .contains("shooting-PSS")
        );
        assert!(
            validate_periodic_state_contract("PNOISE", driven_shooting, true)
                .unwrap_err()
                .contains("autonomous")
        );
    }
}
