//! App-level persistence checks for portable periodic-network drafts.

pub use rspice_simulation_contract::periodic_network_draft::PeriodicNetworkDraft;
#[cfg(test)]
use rspice_simulation_contract::periodic_network_draft::validate_periodic_network;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn periodic_port_noise_draft_retains_references_and_accepts_spot_frequencies() {
        let mut draft = PeriodicNetworkDraft::default();
        draft.sweep.stop = draft.sweep.start.clone();
        draft.noise_parameters = true;
        draft.noise.report_parameters = true;
        draft.noise.input_sideband = "-1".into();
        draft.noise.output_sideband = "1".into();
        draft.noise.reference_temperature = "315".into();
        draft.noise.termination_temperature = "0".into();
        draft.noise.image_sideband = "2".into();
        let expected = draft.noise_reference().unwrap();
        assert!(validate_periodic_network(&draft).is_none());
        let json: PeriodicNetworkDraft =
            serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
        let ron: PeriodicNetworkDraft = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        assert_eq!(json.noise_reference().unwrap(), expected);
        assert_eq!(ron.noise_reference().unwrap(), expected);
        let mut legacy = serde_json::to_value(&draft).unwrap();
        legacy.as_object_mut().unwrap().remove("noise");
        let legacy: PeriodicNetworkDraft = serde_json::from_value(legacy).unwrap();
        assert!(legacy.noise_parameters);
        assert_eq!(legacy.noise_reference().unwrap(), None);
        for invalid in ["-1", "-2147483648", "2147483648"] {
            draft.max_sideband = invalid.into();
            assert!(draft.noise_reference().is_err());
        }
        draft.max_sideband = "4".into();
        draft.noise.image_sideband = draft.noise.input_sideband.clone();
        assert!(validate_periodic_network(&draft).is_some());
        draft.noise.report_parameters = false;
        assert!(validate_periodic_network(&draft).is_none());
    }
}
