use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, PoleZeroConfig, PzAnalysisType,
    SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep};

pub(super) fn analysis_config_from_spec(spec: &AnalysisSpec) -> Option<AnalysisConfig> {
    match spec {
        AnalysisSpec::DcOp => Some(AnalysisConfig::DcOp),
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => Some(AnalysisConfig::DcSweep(DcSweepConfig {
            source: source_name.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            source2: source2.clone(),
            start2: *start2,
            stop2: *stop2,
            step2: *step2,
        })),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => Some(AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: *start_time,
            max_timestep: *max_timestep,
            uic: *uic,
        })),
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => Some(AnalysisConfig::Ac(AcAnalysisConfig {
            sweep_type: ac_sweep_type_from_spec(*sweep),
            num_points: *points_per_unit,
            start_freq: *start_freq,
            stop_freq: *stop_freq,
        })),
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => Some(AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: input_node.clone(),
            input_ref: input_ref.clone(),
            output_node: output_node.clone(),
            output_ref: output_ref.clone(),
            transfer_type: transfer_type.clone(),
            analysis_type: pz_analysis_type_from_spec(analysis_type),
        })),
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
        } => Some(AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: output_var.clone(),
            ac_mode: *ac_mode,
            frequency: *frequency,
        })),
        _ => None,
    }
}

#[inline]
fn ac_sweep_type_from_spec(sweep: FrequencySweep) -> AcSweepType {
    match sweep {
        FrequencySweep::Decade => AcSweepType::Decade,
        FrequencySweep::Octave => AcSweepType::Octave,
        FrequencySweep::Linear => AcSweepType::Linear,
    }
}

#[inline]
fn pz_analysis_type_from_spec(mode: &str) -> PzAnalysisType {
    if mode.eq_ignore_ascii_case("POL") {
        PzAnalysisType::PolesOnly
    } else if mode.eq_ignore_ascii_case("ZER") {
        PzAnalysisType::ZerosOnly
    } else {
        PzAnalysisType::PoleZero
    }
}
