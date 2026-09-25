//! App-facing aliases for portable Monte Carlo statistical authoring.

pub use rspice_simulation_contract::mc_statistics::{McScope, McShape, McStatisticsConfig};

#[cfg(test)]
pub use rspice_simulation_contract::mc_statistics::{
    McCorrelationDraft, McParameterBounds, McParameterCorrelation, McParameterVariation,
    McVariationDraft,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn custom_statistics_preserve_editor_buffers_and_validate_correlations() {
        let mut draft = crate::simulation::dialog::McDialogState::from_config(
            &super::super::McConfig::default(),
        );
        draft.variation_source_idx = 2;
        draft.variation_pct = "unfinished(".into();
        draft.vary_only = "inactive unfinished".into();
        draft.variations = ["X", "Y", "Z"]
            .iter()
            .map(|name| McVariationDraft {
                parameter: (*name).into(),
                spread: "10m".into(),
                percent: false,
                ..Default::default()
            })
            .collect();
        draft.variations[0].lower = "900m".into();
        draft.variations[0].upper = "1.1".into();
        draft.variations[0].sigma_cutoff = "3".into();
        draft.variations[0].max_attempts = "12345".into();
        draft.correlations.push(McCorrelationDraft {
            parameters: "X, Y, Z".into(),
            coefficient: "0.5".into(),
            ..Default::default()
        });
        for restored in [
            serde_json::from_str::<crate::simulation::dialog::McDialogState>(
                &serde_json::to_string(&draft).unwrap(),
            )
            .unwrap(),
            ron::from_str::<crate::simulation::dialog::McDialogState>(
                &ron::to_string(&draft).unwrap(),
            )
            .unwrap(),
        ] {
            assert_eq!(
                restored.to_config().unwrap().statistics.unwrap().variations[0].spread,
                0.01
            );
            assert_eq!(restored.variation_pct, "unfinished(");
            let config = restored.to_config().unwrap().statistics.unwrap();
            assert_eq!(
                config.variations[0].bounds,
                Some(McParameterBounds {
                    lower: Some(0.9),
                    upper: Some(1.1),
                    sigma_cutoff: Some(3.0),
                    max_attempts: 12345
                })
            );
            assert_eq!(
                McVariationDraft::from_config(&config.variations[0])
                    .to_config()
                    .unwrap(),
                config.variations[0]
            );
        }
        let saved = draft.variations[0].clone();
        for (lower, upper, cutoff, attempts) in [
            ("2", "1", "3", "100"),
            ("NaN", "1", "3", "100"),
            ("0", "1", "0", "100"),
            ("0", "1", "3", "0"),
        ] {
            draft.variations[0].lower = lower.into();
            draft.variations[0].upper = upper.into();
            draft.variations[0].sigma_cutoff = cutoff.into();
            draft.variations[0].max_attempts = attempts.into();
            assert!(draft.to_config().is_err());
        }
        draft.variations[0] = saved;
        let legacy = serde_json::json!({ "parameter": "X", "scope": 0, "distribution": 0, "spread": "1", "percent": true });
        let legacy: McVariationDraft = serde_json::from_value(legacy).unwrap();
        assert!(legacy.to_config().unwrap().bounds.is_none());
        draft.correlations[0].coefficient = "-0.9".into();
        assert!(
            draft
                .to_config()
                .unwrap_err()
                .contains("positive semidefinite")
        );
        draft.correlations[0].parameters = "X, missing".into();
        assert!(draft.to_config().is_err());
        draft.variation_source_idx = 1;
        assert!(draft.to_config().unwrap().statistics.is_none());
    }
}
