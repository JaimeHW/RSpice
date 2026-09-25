//! Authored optimization search controls and validated variable domains.

use std::collections::HashSet;

use rspice_core::Value;

mod domain;
pub use domain::{OptimizationCoordinates, OptimizationVariableDomain};
mod types;
pub use types::{OptimizerAlgo, OptimizerConfig};

/// Stopping and stochastic-search controls shared by drafts, workers and runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OptimizationSearchControls {
    pub var_tolerance: Value,
    pub sa_initial_temp: Value,
    pub sa_cooling_rate: Value,
    pub random_seed: u64,
    pub variable_domains: std::collections::BTreeMap<String, OptimizationVariableDomain>,
}

impl Default for OptimizationSearchControls {
    fn default() -> Self {
        Self {
            var_tolerance: 1e-6,
            sa_initial_temp: 100.0,
            sa_cooling_rate: 0.95,
            random_seed: 0xDEAD_BEEF_CAFE_BABE,
            variable_domains: Default::default(),
        }
    }
}

impl OptimizationSearchControls {
    pub fn validate_domains<'a>(
        &self,
        variables: impl Iterator<Item = (&'a str, f64, f64, f64)>,
        gradient: bool,
    ) -> Result<(), String> {
        let variables = variables.collect::<Vec<_>>();
        let mut seen = HashSet::new();
        for (name, domain) in &self.variable_domains {
            if !seen.insert(name.to_ascii_lowercase()) {
                return Err(format!("Repeated domain for variable {name:?}"));
            }
            let (_, min, max, initial) = variables
                .iter()
                .find(|(candidate, _, _, _)| candidate.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("Variable domain {name:?} has no configured variable"))?;
            domain
                .coordinates(*min, *max, *initial)
                .map_err(|error| format!("Variable {name:?}: {error}"))?;
            if gradient && domain.is_discrete() {
                return Err("Discrete variables require pattern search or annealing".into());
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.var_tolerance.is_finite() || self.var_tolerance <= 0.0 {
            return Err("Optimization gradient tolerance must be finite and positive".into());
        }
        if !self.sa_initial_temp.is_finite() || self.sa_initial_temp <= 0.0 {
            return Err("Annealing temperature must be finite and positive".into());
        }
        if !self.sa_cooling_rate.is_finite()
            || !(0.0..1.0).contains(&self.sa_cooling_rate)
            || self.sa_cooling_rate == 0.0
        {
            return Err("Annealing cooling rate must be between zero and one".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_search_controls_preserve_the_authored_wire_shape() {
        let defaults = OptimizationSearchControls::default();
        let restored: OptimizationSearchControls = serde_json::from_str("{}").unwrap();
        assert_eq!(restored, defaults);
        assert_eq!(defaults.random_seed, 0xDEAD_BEEF_CAFE_BABE);
        assert_eq!(defaults.var_tolerance, 1e-6);
        assert_eq!(defaults.sa_initial_temp, 100.0);
        assert_eq!(defaults.sa_cooling_rate, 0.95);
        assert!(defaults.variable_domains.is_empty());
    }

    #[test]
    fn domain_coordinates_refuse_invalid_grids_and_preserve_physical_endpoints() {
        use OptimizationVariableDomain as Domain;

        assert!(Domain::Logarithmic.coordinates(0.0, 1.0, 0.5).is_err());
        assert!(
            Domain::Quantized { step: 2.0 }
                .coordinates(0.0, 10.0, 3.0)
                .is_err()
        );
        assert!(
            Domain::Discrete {
                values: vec![2.0, 1.0]
            }
            .coordinates(0.0, 3.0, 1.0)
            .is_err()
        );
        assert!(
            Domain::Quantized { step: 4.0 }
                .coordinates(1e16, 1e16 + 16.0, 1e16 + 2.0)
                .is_err()
        );
        let decimal = Domain::Quantized { step: 0.1 }
            .coordinates(0.0, 0.3, 0.3)
            .unwrap();
        assert_eq!(decimal.physical(decimal.max), 0.3);
    }
}
