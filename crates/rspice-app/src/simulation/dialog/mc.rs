//! App-facing aliases for the portable Monte Carlo draft.

pub(crate) mod checkpoint;
pub(crate) mod statistics;

pub use rspice_simulation_contract::mc_draft::{McDialogState, McDistribution, McVariationSource};

#[cfg(test)]
pub use rspice_simulation_contract::mc_draft::McConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monte_carlo_seed_migration_preserves_legacy_default_and_editor_buffers() {
        for (legacy, expected) in [
            ("0", ""),
            (" 000 ", ""),
            ("7", "7"),
            ("unfinished(", "unfinished("),
        ] {
            let json = serde_json::json!({ "seed": legacy });
            let ron = format!("(seed:{legacy:?})");
            for state in [
                serde_json::from_value::<McDialogState>(json).unwrap(),
                ron::from_str::<McDialogState>(&ron).unwrap(),
            ] {
                assert_eq!(state.seed, expected);
                let encoded = serde_json::to_value(&state).unwrap();
                assert!(encoded.get("seed").is_none());
                assert_eq!(encoded["explicit_seed"], expected);
            }
        }
    }

    #[test]
    fn monte_carlo_seed_round_trips_json_and_ron_through_draft_restore() {
        use crate::simulation::plan::AnalysisDraft;
        for seed in [None, Some(0), Some((1_u64 << 53) + 1), Some(u64::MAX)] {
            let config = McConfig {
                statistics: None,
                seed,
                ..Default::default()
            };
            let draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&config));
            let json = serde_json::to_string(&draft).unwrap();
            let ron = ron::to_string(&draft).unwrap();
            for mut restored in [
                serde_json::from_str::<AnalysisDraft>(&json).unwrap(),
                ron::from_str::<AnalysisDraft>(&ron).unwrap(),
            ] {
                restored.prepare_after_restore();
                let AnalysisDraft::MonteCarlo(mut state) = restored else {
                    panic!("wrong restored draft");
                };
                state.ensure_initialized();
                assert_eq!(state.to_config().unwrap().seed, seed);
            }
        }
    }

    #[test]
    fn monte_carlo_seed_corrupt_wire_values_are_rejected() {
        for source in [
            r#"{"explicit_seed":null}"#,
            r#"{"explicit_seed":0}"#,
            r#"{"seed":"7","explicit_seed":"0"}"#,
        ] {
            assert!(
                serde_json::from_str::<McDialogState>(source).is_err(),
                "{source}"
            );
        }
        for source in [
            "(explicit_seed:None)",
            "(explicit_seed:0)",
            "(seed:\"7\",explicit_seed:\"0\")",
        ] {
            assert!(ron::from_str::<McDialogState>(source).is_err(), "{source}");
        }
    }

    #[test]
    fn invalid_active_spread_and_distribution_are_refused() {
        let mut draft = McDialogState::from_config(&McConfig::default());
        draft.variation_pct = "NaN".to_owned();
        assert!(
            draft.to_config().is_err(),
            "NaN spread is not a runnable configuration"
        );
        draft.variation_pct = "5".to_owned();
        draft.distribution_idx = usize::MAX;
        assert!(
            draft.to_config().is_err(),
            "a missing distribution is not Worst Case"
        );
    }

    #[test]
    fn a_monte_carlo_subset_survives_a_reopen_and_keeps_the_cards_grammar() {
        use crate::simulation::plan::AnalysisDraft;

        let mut state = McDialogState::from_config(&McConfig::default());
        state.vary_only = "rload, cload  rseries".to_owned();
        assert_eq!(
            state.to_config().unwrap().params,
            ["rload", "cload", "rseries"],
            "commas and whitespace both separate names, as the card's own list does"
        );

        let draft = AnalysisDraft::MonteCarlo(state.clone());
        let json = serde_json::to_string(&draft).unwrap();
        let ron = ron::to_string(&draft).unwrap();
        for mut restored in [
            serde_json::from_str::<AnalysisDraft>(&json).unwrap(),
            ron::from_str::<AnalysisDraft>(&ron).unwrap(),
        ] {
            restored.prepare_after_restore();
            let AnalysisDraft::MonteCarlo(mut restored) = restored else {
                panic!("wrong restored draft");
            };
            restored.ensure_initialized();
            assert_eq!(restored.vary_only, state.vary_only);
            assert_eq!(
                restored.to_config().unwrap().params,
                ["rload", "cload", "rseries"]
            );
        }

        // A project saved before the field existed asks for the card's absent
        // list, which is every eligible parameter.
        let mut older: McDialogState = serde_json::from_str(r#"{"num_runs":"32"}"#).unwrap();
        older.ensure_initialized();
        assert!(older.to_config().unwrap().params.is_empty());
    }

    #[test]
    fn a_monte_carlo_subset_refuses_what_the_card_cannot_spell() {
        let mut state = McDialogState::from_config(&McConfig::default());
        for text in ["2rload", "r-load", "r.load", "V(out)", "r+load"] {
            state.vary_only = text.to_owned();
            assert!(
                state.to_config().is_err(),
                "{text:?} is not a parameter name the .MC PARAMS list accepts"
            );
        }
        // A space is a separator on the card, not a character in a name, so
        // "r load" is two names rather than one bad one.
        state.vary_only = "r load".to_owned();
        assert_eq!(state.to_config().unwrap().params, ["r", "load"]);
        // A repeat is what the card does with one: keep the first.
        state.vary_only = "rload, RLOAD".to_owned();
        assert_eq!(state.to_config().unwrap().params, ["rload"]);
        // Blank is the absent list, not an empty one: `.MC ... PARAMS` with
        // nothing after it is a parse error.
        state.vary_only = " , \t ".to_owned();
        assert!(state.to_config().unwrap().params.is_empty());
    }

    #[test]
    fn a_deck_stated_spread_contributes_no_subset_but_keeps_the_draft() {
        let mut state = McDialogState::from_config(&McConfig::default());
        state.vary_only = "rload".to_owned();
        state.variation_source_idx = McVariationSource::ALL
            .iter()
            .position(|source| *source == McVariationSource::DeckStatistics)
            .expect("deck statistics is a source");

        let config = state.to_config().expect("deck statistics runs");
        assert!(
            config.params.is_empty(),
            "the engine refuses a generic filter beside native statistics, so none is sent"
        );
        assert_eq!(state.vary_only, "rload", "the authored text is kept");
    }

    #[test]
    fn retired_controls_decode_without_reappearing_on_serialize() {
        let persisted = r#"{
            "num_runs": "250",
            "seed": "7",
            "distribution_idx": 1,
            "variation_pct": "2.5",
            "base_idx": 0,
            "process_variations": true,
            "mismatch_variations": false,
            "save_all_runs": true
        }"#;

        let state: McDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.num_runs, "250");
        assert_eq!(state.distribution_idx, 1);

        let encoded = serde_json::to_value(&state).expect("state encodes");
        for retired in [
            "base_idx",
            "process_variations",
            "mismatch_variations",
            "save_all_runs",
        ] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }
}

#[cfg(test)]
mod confidence_tests {
    use super::*;
    use crate::simulation::plan::AnalysisDraft;
    use crate::state::MonteCarloMeanMethod;

    #[test]
    fn monte_carlo_confidence_draft_round_trips_and_migrates() {
        let config = McConfig {
            statistics: None,
            confidence_pct: 90.12345678912345,
            confidence_method: MonteCarloMeanMethod::PercentileBootstrap {
                resamples: 257,
                seed: u64::MAX,
            },
            ..Default::default()
        };
        let draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&config));
        for mut decoded in [
            serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
            ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
        ] {
            decoded.prepare_after_restore();
            let AnalysisDraft::MonteCarlo(mut state) = decoded else {
                panic!("MC")
            };
            state.ensure_initialized();
            let restored = state.to_config().unwrap();
            assert_eq!(restored.confidence_pct, config.confidence_pct);
            assert_eq!(restored.confidence_method, config.confidence_method);
        }
        let mut old: serde_json::Value = serde_json::to_value(&draft).unwrap();
        let body = old["draft"].as_object_mut().unwrap();
        for key in [
            "confidence_pct",
            "confidence_method_idx",
            "bootstrap_resamples",
            "bootstrap_seed",
        ] {
            body.remove(key);
        }
        let mut decoded: AnalysisDraft = serde_json::from_value(old).unwrap();
        decoded.prepare_after_restore();
        let AnalysisDraft::MonteCarlo(state) = decoded else {
            panic!("MC")
        };
        assert_eq!(state.to_config().unwrap().confidence_pct, 95.0);
        assert_eq!(
            state.to_config().unwrap().confidence_method,
            MonteCarloMeanMethod::StudentT
        );
    }

    #[test]
    fn monte_carlo_confidence_validates_active_fields_and_preserves_inactive_drafts() {
        let mut state = McDialogState::from_config(&McConfig::default());
        for level in ["NaN", "inf", "0", "100", "-1"] {
            state.confidence_pct = level.into();
            assert!(state.to_config().is_err(), "{level}");
        }
        state.confidence_pct = "95".into();
        state.bootstrap_resamples = "unfinished(".into();
        state.bootstrap_seed = "unfinished(".into();
        assert!(state.to_config().is_ok());
        state.confidence_method_idx = 1;
        assert!(state.to_config().is_err());
        state.bootstrap_seed = u64::MAX.to_string();
        for count in ["0", "1", "2.5", "-1"] {
            state.bootstrap_resamples = count.into();
            assert!(state.to_config().is_err());
        }
        state.bootstrap_resamples = "2".into();
        assert!(state.to_config().is_ok());
        state.confidence_method_idx = 2;
        assert!(state.to_config().is_err());
    }
}
