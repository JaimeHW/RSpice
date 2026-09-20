//! Validation shared by pack loading and every numerical entry point.

use super::*;
use std::collections::HashSet;

pub(super) fn positive(value: f64, label: &str) -> Result<(), AgingError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(AgingError::Invalid(format!(
            "{label} must be finite and positive"
        )))
    }
}

fn nonnegative(value: f64, label: &str) -> Result<(), AgingError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(AgingError::Invalid(format!(
            "{label} must be finite and nonnegative"
        )))
    }
}

fn text(value: &str, label: &str) -> Result<(), AgingError> {
    if value.trim().is_empty() || value.len() > 16384 || value.contains('\0') {
        return Err(AgingError::Invalid(format!(
            "{label} must be nonempty text of at most 16384 bytes"
        )));
    }
    Ok(())
}

impl AgingRange {
    fn validate(&self, label: &str) -> Result<(), AgingError> {
        if !self.min.is_finite() || !self.max.is_finite() || self.min > self.max {
            return Err(AgingError::Invalid(format!(
                "{label} must have finite ordered bounds"
            )));
        }
        Ok(())
    }

    pub(super) fn require(&self, value: f64, label: &str) -> Result<(), AgingError> {
        if !value.is_finite() || value < self.min || value > self.max {
            return Err(AgingError::OutsideCalibration(format!(
                "{label} = {value}, expected {}..={}",
                self.min, self.max
            )));
        }
        Ok(())
    }
}

impl AgingModelPack {
    pub fn validate(&self) -> Result<(), AgingError> {
        if self.schema_version != 1 {
            return Err(AgingError::Invalid(format!(
                "unsupported schema version {}",
                self.schema_version
            )));
        }
        for (value, label) in [
            (&self.id, "pack identity"),
            (&self.process, "process"),
            (&self.source, "source"),
            (&self.license, "license"),
            (&self.characterization, "characterization"),
        ] {
            text(value, label)?;
        }
        if self.models.is_empty() || self.models.len() > MAX_AGING_MODELS {
            return Err(AgingError::Invalid("a pack needs 1..=1024 models".into()));
        }
        let mut ids = HashSet::new();
        for model in &self.models {
            model.validate()?;
            if !ids.insert(&model.id) {
                return Err(AgingError::Invalid(format!(
                    "duplicate model identity {}",
                    model.id
                )));
            }
        }
        Ok(())
    }
}

impl AgingModel {
    pub fn validate(&self) -> Result<(), AgingError> {
        text(&self.id, "model identity")?;
        text(&self.applicability, "model applicability")?;
        let v = &self.validity;
        v.gate_source_v.validate("Vgs validity")?;
        v.drain_source_v.validate("Vds validity")?;
        v.temperature_k.validate("temperature validity")?;
        positive(v.temperature_k.min, "minimum temperature")?;
        v.current_density_a_per_m2
            .validate("current-density validity")?;
        nonnegative(v.current_density_a_per_m2.min, "minimum current density")?;
        positive(v.max_equivalent_seconds, "maximum equivalent exposure")?;
        match &self.law {
            AgingLaw::EquivalentTimePower {
                reference_time_s,
                reference_gate_magnitude_v,
                reference_drain_magnitude_v,
                reference_temperature_k,
                gate_polarity,
                clock_gate_exponent,
                clock_drain_exponent,
                clock_activation_energy_ev,
                time_exponent,
                parameters,
            } => {
                if self.mechanism == AgingMechanism::Electromigration {
                    return Err(AgingError::Invalid(
                        "electromigration requires a lifetime law".into(),
                    ));
                }
                if self.mechanism == AgingMechanism::Nbti
                    && *gate_polarity != AgingGatePolarity::Negative
                {
                    return Err(AgingError::Invalid(
                        "NBTI requires negative gate stress".into(),
                    ));
                }
                positive(*reference_time_s, "reference time")?;
                positive(*reference_gate_magnitude_v, "reference gate magnitude")?;
                positive(*reference_drain_magnitude_v, "reference drain magnitude")?;
                positive(*reference_temperature_k, "reference temperature")?;
                v.temperature_k
                    .require(*reference_temperature_k, "reference temperature")?;
                nonnegative(*clock_gate_exponent, "clock gate exponent")?;
                nonnegative(*clock_drain_exponent, "clock drain exponent")?;
                // HCI temperature fits need not have a positive activation energy.
                if !clock_activation_energy_ev.is_finite() {
                    return Err(AgingError::Invalid(
                        "clock activation energy must be finite".into(),
                    ));
                }
                positive(*time_exponent, "time exponent")?;
                if parameters.is_empty() || parameters.len() > 128 {
                    return Err(AgingError::Invalid(
                        "a power law needs 1..=128 parameter shifts".into(),
                    ));
                }
                let mut names = HashSet::new();
                for parameter in parameters {
                    let name = &parameter.parameter;
                    if name.is_empty()
                        || name.len() > 128
                        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    {
                        return Err(AgingError::Invalid(
                            "parameter names must be alphanumeric identifiers".into(),
                        ));
                    }
                    if !names.insert(name.to_ascii_lowercase()) {
                        return Err(AgingError::Invalid(format!("duplicate parameter {name}")));
                    }
                    if !parameter.scale_at_reference_time.is_finite()
                        || parameter.scale_at_reference_time == 0.0
                    {
                        return Err(AgingError::Invalid(format!(
                            "{name} needs a finite nonzero shift scale"
                        )));
                    }
                }
            }
            AgingLaw::BlackElectromigration {
                reference_lifetime_s,
                reference_current_density_a_per_m2,
                reference_temperature_k,
                current_exponent,
                activation_energy_ev,
            } => {
                if self.mechanism != AgingMechanism::Electromigration {
                    return Err(AgingError::Invalid(
                        "Black's law is an electromigration model".into(),
                    ));
                }
                positive(*reference_lifetime_s, "reference lifetime")?;
                positive(
                    *reference_current_density_a_per_m2,
                    "reference current density",
                )?;
                v.current_density_a_per_m2.require(
                    *reference_current_density_a_per_m2,
                    "reference current density",
                )?;
                positive(*reference_temperature_k, "reference temperature")?;
                v.temperature_k
                    .require(*reference_temperature_k, "reference temperature")?;
                positive(*current_exponent, "current exponent")?;
                nonnegative(*activation_energy_ev, "activation energy")?;
            }
        }
        Ok(())
    }
}
