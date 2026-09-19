//! Observation window, startup and concrete device/model selection for SOA.

use rspice_core::netlist::{Element, ElementKind};

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaObservationConfig {
    /// Only samples at or after this time contribute to the checker.
    #[serde(default)]
    pub start_time: f64,
    /// Independent upper bound on integration steps.
    #[serde(default)]
    pub max_step: Option<f64>,
    /// Start from authored ICs without solving an initial operating point.
    #[serde(default)]
    pub use_initial_conditions: bool,
    /// Exact, case-insensitive flattened instance names; empty includes all.
    #[serde(default)]
    pub devices: Vec<String>,
    /// Exact model names; empty includes all. Both filters must match when set.
    #[serde(default)]
    pub models: Vec<String>,
}

impl SoaObservationConfig {
    pub fn validate(&self, stop_time: f64) -> Result<(), String> {
        if !self.start_time.is_finite() || self.start_time < 0.0 || self.start_time >= stop_time {
            return Err(
                "SOA start time must be finite, nonnegative and before the stop time".into(),
            );
        }
        if self
            .max_step
            .is_some_and(|step| !step.is_finite() || step <= 0.0)
        {
            return Err("SOA maximum step must be finite and positive".into());
        }
        for (kind, names) in [("device", &self.devices), ("model", &self.models)] {
            let mut seen = std::collections::HashSet::new();
            for name in names {
                if name.trim().is_empty()
                    || name
                        .chars()
                        .any(|ch| ch.is_whitespace() || matches!(ch, ',' | ';' | '=' | '*' | '?'))
                {
                    return Err(format!(
                        "SOA {kind} filters require exact names separated by spaces"
                    ));
                }
                let key = if kind == "device" {
                    name.replace(':', ".")
                } else {
                    name.clone()
                };
                if !seen.insert(key.to_ascii_uppercase()) {
                    return Err(format!("SOA {kind} filter repeats '{name}'"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_selection(&self, elements: &[Element]) -> Result<(), String> {
        for name in &self.devices {
            if !elements
                .iter()
                .any(|element| same_name(&element.name, name) && Self::model(element).is_some())
            {
                return Err(format!(
                    "SOA device filter '{name}' does not name an eligible concrete device"
                ));
            }
        }
        for name in &self.models {
            if !elements.iter().any(|element| {
                Self::model(element).is_some_and(|model| model.eq_ignore_ascii_case(name))
            }) {
                return Err(format!("SOA model filter '{name}' has no eligible devices"));
            }
        }
        Ok(())
    }

    pub(super) fn model(element: &Element) -> Option<&str> {
        match &element.kind {
            ElementKind::Mosfet { model, .. }
            | ElementKind::Bjt { model, .. }
            | ElementKind::Jfet { model, .. }
            | ElementKind::Mesfet { model, .. } => Some(model),
            _ => None,
        }
    }

    pub(super) fn includes(&self, element: &Element) -> bool {
        if !self.devices.is_empty()
            && !self
                .devices
                .iter()
                .any(|name| same_name(name, &element.name))
        {
            return false;
        }
        let Some(model) = Self::model(element) else {
            return false;
        };
        self.models.is_empty()
            || self
                .models
                .iter()
                .any(|name| name.eq_ignore_ascii_case(model))
    }
}

/// The flattener uses dots; authored SPICE paths also accept colons.
pub(super) fn same_name(left: &str, right: &str) -> bool {
    let canonical = |byte: u8| {
        if byte == b':' {
            b'.'
        } else {
            byte.to_ascii_uppercase()
        }
    };
    left.bytes().map(canonical).eq(right.bytes().map(canonical))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_observation_refuses_invalid_windows_steps_and_ambiguous_filters() {
        let mut config = SoaObservationConfig::default();
        config.validate(1.0).unwrap();
        for start in [-1.0, 1.0, f64::NAN] {
            config.start_time = start;
            assert!(config.validate(1.0).is_err());
        }
        config.start_time = 0.0;
        config.max_step = Some(0.0);
        assert!(config.validate(1.0).is_err());
        config.max_step = Some(0.01);
        config.devices = vec!["M1".into(), "m1".into()];
        assert!(config.validate(1.0).is_err());
        config.devices = vec!["X1:M1".into(), "x1.m1".into()];
        assert!(config.validate(1.0).is_err());
        config.devices = vec!["X1:M1".into()];
        config.models = vec!["NM".into()];
        config.validate(1.0).unwrap();
    }
}
