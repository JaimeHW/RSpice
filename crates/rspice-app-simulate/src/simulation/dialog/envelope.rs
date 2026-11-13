//! App-facing aliases and integration tests for portable Envelope authoring.

#[cfg(test)]
use rspice_simulation_contract::analysis_spec::{
    EnvelopeAdaptiveMode, EnvelopeInitialPeriodicSolve,
};
#[cfg(test)]
pub use rspice_simulation_contract::envelope_draft::EnvelopeConfig;
pub use rspice_simulation_contract::envelope_draft::EnvelopeDialogState;
#[cfg(test)]
mod initialization;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_requires_a_circuit_source_and_serializes_every_control() {
        let mut config = EnvelopeConfig::default();
        assert_eq!(config.carrier_tones, vec![1.0e6]);
        assert!(config.modulation_sources.is_empty());
        assert!(config.validate().unwrap_err().contains("required"));
        config.modulation_sources = vec!["Vmod".to_owned()];
        config
            .validate()
            .expect("an explicitly selected modulation source is valid");
        assert_eq!(
            config.initial_periodic_solve,
            EnvelopeInitialPeriodicSolve::HarmonicBalance
        );
        assert_eq!(config.adaptive_mode, EnvelopeAdaptiveMode::Enabled);

        let command = config.to_spice();
        for owned_keyword in [
            "carriers=[1Meg]",
            "stop=10m",
            "step=1u",
            "harmonic_order=9",
            "modulation_sources=[Vmod]",
            "initial_periodic_solve=hb",
            "adaptive=enabled",
            "extraction=projection",
        ] {
            assert!(
                command.contains(owned_keyword),
                "missing {owned_keyword}: {command}"
            );
        }
    }

    #[test]
    fn rejects_duplicate_carriers_and_noncanonical_modulation_sources() {
        let mut config = EnvelopeConfig::default();
        config.carrier_tones.push(1.0e6);
        assert!(config.validate().unwrap_err().contains("unique"));

        config.carrier_tones.pop();
        config.modulation_sources = vec![" VIN_AM".to_owned()];
        assert!(config.validate().unwrap_err().contains("whitespace"));
    }

    #[test]
    fn harmonic_order_validation_matches_the_positive_integer_field_contract() {
        let expected = "Harmonic order must be a positive integer (minimum 1)";
        let mut config = EnvelopeConfig::default();
        config.harmonic_order = 0;
        assert_eq!(config.validate().unwrap_err(), expected);

        let mut state = EnvelopeDialogState::from_config(&EnvelopeConfig::default());
        state.harmonic_order = "1.5".to_owned();
        assert_eq!(state.to_config().unwrap_err(), expected);
    }
}
