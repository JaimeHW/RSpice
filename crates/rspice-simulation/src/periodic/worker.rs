use super::*;
use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;
use rspice_simulation_contract::worker_run_config::{
    WorkerPacRunConfig, WorkerPeriodicCarrier, WorkerPnoiseReference, WorkerPnoiseRunConfig,
    WorkerPstbRunConfig, WorkerPxfRunConfig,
};
use rspice_simulation_contract::worker_spec::WorkerSweepType;

impl From<&PacRunConfig> for WorkerPacRunConfig {
    fn from(value: &PacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            sideband_max: value.sideband_max,
            // Written only where the range is not symmetric, so a symmetric
            // request is the same document it has always been.
            sideband_min: (value.sideband_min != -value.sideband_max).then_some(value.sideband_min),
            input_source: value.input_source.clone(),
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: WorkerPeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<WorkerPacRunConfig> for PacRunConfig {
    fn from(value: WorkerPacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: PacFrequencySweep::from(value.sweep),
            sideband_max: value.sideband_max,
            sideband_min: value.sideband_min.unwrap_or(-value.sideband_max),
            input_source: value.input_source,
            output_node: value.output_node,
            output_ref: value.output_ref,
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: PeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<&PxfRunConfig> for WorkerPxfRunConfig {
    fn from(value: &PxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            input_source: value.input_source.clone(),
            input_sideband: value.input_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: WorkerPeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<WorkerPxfRunConfig> for PxfRunConfig {
    fn from(value: WorkerPxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: PxfFrequencySweep::from(value.sweep),
            input_source: value.input_source,
            input_sideband: value.input_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: PeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<&PnoiseRunConfig> for WorkerPnoiseRunConfig {
    fn from(value: &PnoiseRunConfig) -> Self {
        Self {
            sampling: value.sampling.clone(),
            input_sideband: value.input_sideband,
            output_sideband: value.output_sideband,
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            input_source: value.input_source.clone(),
            noise_ref: WorkerPnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: WorkerPeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<WorkerPnoiseRunConfig> for PnoiseRunConfig {
    fn from(value: WorkerPnoiseRunConfig) -> Self {
        Self {
            sampling: value.sampling.clone(),
            input_sideband: value.input_sideband,
            output_sideband: value.output_sideband,
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: PnoiseFrequencySweep::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            input_source: value.input_source,
            noise_ref: PnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: PeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<&PstbRunConfig> for WorkerPstbRunConfig {
    fn from(value: &PstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance.clone(),
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

impl From<WorkerPstbRunConfig> for PstbRunConfig {
    fn from(value: WorkerPstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance,
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

impl From<PacFrequencySweep> for WorkerSweepType {
    fn from(value: PacFrequencySweep) -> Self {
        match value {
            PacFrequencySweep::Decade => Self::Decade,
            PacFrequencySweep::Octave => Self::Octave,
            PacFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<PxfFrequencySweep> for WorkerSweepType {
    fn from(value: PxfFrequencySweep) -> Self {
        match value {
            PxfFrequencySweep::Decade => Self::Decade,
            PxfFrequencySweep::Octave => Self::Octave,
            PxfFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<PnoiseFrequencySweep> for WorkerSweepType {
    fn from(value: PnoiseFrequencySweep) -> Self {
        match value {
            PnoiseFrequencySweep::Decade => Self::Decade,
            PnoiseFrequencySweep::Octave => Self::Octave,
            PnoiseFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for PacFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for PxfFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for PnoiseFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<PnoiseReference> for WorkerPnoiseReference {
    fn from(value: PnoiseReference) -> Self {
        match value {
            PnoiseReference::Output => Self::Output,
            PnoiseReference::Input => Self::Input,
            PnoiseReference::Phase => Self::Phase,
        }
    }
}

impl From<WorkerPnoiseReference> for PnoiseReference {
    fn from(value: WorkerPnoiseReference) -> Self {
        match value {
            WorkerPnoiseReference::Output => Self::Output,
            WorkerPnoiseReference::Input => Self::Input,
            WorkerPnoiseReference::Phase => Self::Phase,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampled_pnoise_studio_worker_preserves_sampling_controls() {
        use rspice_core::analysis::pnoise::{PeriodicNoiseEdge, PeriodicNoiseSampling};
        let config = PnoiseRunConfig {
            sampling: Some(PeriodicNoiseSampling::Delay {
                edge: PeriodicNoiseEdge::default(),
                reference_node: "clk".into(),
                reference_ref: Some("vss".into()),
                reference_edge: PeriodicNoiseEdge {
                    threshold_volts: 0.7,
                    ..Default::default()
                },
                periods: 3,
            }),
            ..Default::default()
        };
        let worker = WorkerPnoiseRunConfig::from(&config);
        let decoded: WorkerPnoiseRunConfig =
            serde_json::from_str(&serde_json::to_string(&worker).unwrap()).unwrap();
        let restored = PnoiseRunConfig::from(decoded);
        assert_eq!(restored, config);
    }
}
