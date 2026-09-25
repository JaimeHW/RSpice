//! App-facing aliases for the portable optimization draft.

pub use rspice_simulation_contract::optimization_draft::{
    OptimizationAlgorithmMode, OptimizationDialogState, OptimizationGoalMode,
    OptimizationVariableConfig,
};

#[cfg(test)]
pub use rspice_simulation_contract::optimization_draft::OptimizationConfig;
#[cfg(test)]
use rspice_simulation_contract::optimization_search::OptimizationSearchControls;

#[cfg(test)]
mod search_tests {
    use super::*;
    #[test]
    fn optimization_units_and_expression_survive_draft_restoration_and_legacy_omission() {
        let config = OptimizationConfig {
            objective_unit: "mW".into(),
            objective_expression: Some("-V(in)*I(V1)".into()),
            ..Default::default()
        };
        let draft = OptimizationDialogState::from_config(&config);
        let json: OptimizationDialogState =
            serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
        let ron: OptimizationDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        assert_eq!(json.to_config().unwrap(), config);
        assert_eq!(ron.to_config().unwrap(), config);
        assert!(config.to_spice().contains("expr={-V(in)*I(V1)}"));
        assert!(config.to_spice().contains("unit=\"mW\""));
        let mut legacy = serde_json::to_value(&draft).unwrap();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("objective_expression");
        let legacy: OptimizationDialogState = serde_json::from_value(legacy).unwrap();
        assert!(legacy.to_config().unwrap().objective_expression.is_none());
        let mut old = serde_json::to_value(&draft).unwrap();
        old.as_object_mut().unwrap().remove("objective_unit");
        let legacy: OptimizationDialogState = serde_json::from_value(old).unwrap();
        assert!(legacy.to_config().unwrap().objective_unit.is_empty());
        let mut invalid = draft.clone();
        invalid.objective_unit = "not-a-unit".into();
        assert!(invalid.to_config().is_err());
    }

    #[test]
    fn optimization_search_controls_survive_drafts_and_legacy_loading() {
        let mut config = OptimizationConfig::default();
        config.search = OptimizationSearchControls {
            var_tolerance: 7.123456789e-8,
            sa_initial_temp: 12.25,
            sa_cooling_rate: 0.875,
            random_seed: u64::MAX,
            variable_domains: std::collections::BTreeMap::from([
                (
                    "RLOAD".into(),
                    crate::simulation::optimizer::OptimizationVariableDomain::Logarithmic,
                ),
                (
                    "VDD".into(),
                    crate::simulation::optimizer::OptimizationVariableDomain::Quantized {
                        step: 0.1,
                    },
                ),
            ]),
        };
        let draft = OptimizationDialogState::from_config(&config);
        let json = serde_json::to_string(&draft).unwrap();
        let restored: OptimizationDialogState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.to_config().unwrap(), config);
        let restored: OptimizationDialogState =
            ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        assert_eq!(restored.to_config().unwrap(), config);
        let mut old = serde_json::to_value(&draft).unwrap();
        for name in [
            "var_tolerance",
            "sa_initial_temp",
            "sa_cooling_rate",
            "random_seed",
            "variable_domains",
        ] {
            old.as_object_mut().unwrap().remove(name);
        }
        let legacy: OptimizationDialogState = serde_json::from_value(old).unwrap();
        assert_eq!(
            legacy.to_config().unwrap().search,
            OptimizationSearchControls::default()
        );
    }
}
