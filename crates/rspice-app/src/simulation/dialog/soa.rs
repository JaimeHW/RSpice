//! App-facing aliases for portable SOA analysis authoring.

#[cfg(test)]
use crate::services::simulation_runner::{SoaObservationConfig, SoaRuleConfig};
#[cfg(test)]
pub use rspice_simulation_contract::soa_draft::SoaConfig;
pub use rspice_simulation_contract::soa_draft::{SoaDialogState, SoaEnvelopeDraft, SoaRuleDraft};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_disabled_limits_do_not_block_another_enabled_rule() {
        let mut draft = SoaDialogState::from_config(&SoaConfig::default());
        draft.check_vgs_max = false;
        draft.max_vgs = "unfinished edit".into();
        assert!(draft.to_config().is_ok());
        draft.check_vgs_max = true;
        assert!(draft.to_config().is_err());
    }

    #[test]
    fn soa_authored_observation_settings_survive_restore_and_full_precision_cards() {
        let config = SoaConfig {
            rules: vec![SoaRuleConfig {
                duration_mode: Default::default(),
                minimum_duration_s: None,
                current_envelope: None,
                power_derating: None,
                voltage_basis: Default::default(),
                parameter: crate::results::safety::SoAParameter::Id,
                max_value: 0.0123456789012345,
                devices: vec!["X1:M1".into()],
                models: vec!["NM".into()],
            }],
            stop_time: 1.23456789123e-6,
            observation: SoaObservationConfig {
                thresholds: Default::default(),
                start_time: 1.23456789123e-7,
                max_step: Some(1.23456789123e-10),
                use_initial_conditions: true,
                devices: vec!["X1:M1".into()],
                models: vec!["NM".into()],
            },
            ..Default::default()
        };
        let draft = SoaDialogState::from_config(&config);
        let mut json: SoaDialogState =
            serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
        let mut ron: SoaDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        for restored in [&mut json, &mut ron] {
            restored.ensure_initialized();
            assert_eq!(restored.to_config().unwrap(), config);
        }
        let card = config.to_spice();
        for value in [
            config.stop_time,
            config.rules[0].max_value,
            config.observation.start_time,
            config.observation.max_step.unwrap(),
        ] {
            assert!(card.contains(&value.to_string()));
        }
        assert!(card.contains("devices=(X1:M1) models=(NM)"));
        let mut legacy = serde_json::to_value(&draft).unwrap();
        for key in [
            "rules",
            "start_time",
            "max_step",
            "use_initial_conditions",
            "devices",
            "models",
        ] {
            legacy.as_object_mut().unwrap().remove(key);
        }
        let mut restored: SoaDialogState = serde_json::from_value(legacy).unwrap();
        restored.ensure_initialized();
        assert_eq!(
            restored.to_config().unwrap().observation,
            SoaObservationConfig::default()
        );
        assert!(restored.to_config().unwrap().rules.is_empty());
        assert_eq!(restored.to_config().unwrap().stop_time, config.stop_time);
    }
}
