//! Validation for the analyses that convert straight into a runner
//! configuration.
//!
//! The same family `runner/spec/config.rs` dispatches: a kind whose whole
//! request is an `AnalysisConfig`, so what there is to refuse is the sweep
//! axis and the probe names, and nothing about a solve that precedes it.

use crate::simulation::config::{AcSweepType, NoiseAnalysisConfig, NoiseSweepType};
use crate::simulation::multi_run::AnalysisSpec;

/// Validate one configured specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
            hysteresis,
            modes,
        } => crate::simulation::config::DcSweepConfig {
            source: source_name.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            source2: source2.clone(),
            start2: *start2,
            stop2: *stop2,
            step2: *step2,
            hysteresis: *hysteresis,
            modes: modes.clone(),
        }
        .validate()
        .map_err(|errors| errors.join("; ")),
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            use crate::simulation::multi_run::FrequencySweep;
            crate::simulation::config::AcAnalysisConfig {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                num_points: *points_per_unit,
                sweep_type: match sweep {
                    FrequencySweep::Decade => AcSweepType::Decade,
                    FrequencySweep::Octave => AcSweepType::Octave,
                    FrequencySweep::Linear => AcSweepType::Linear,
                },
            }
            .validate()
            .map_err(|errors| errors.join("; "))
        }
        AnalysisSpec::AcData {
            table_name,
            frequencies,
            table_options,
        } => table_options
            .config(table_name, frequencies.clone())
            .validate()
            .map_err(|errors| errors.join("; ")),
        AnalysisSpec::Noise {
            output_node,
            reference_node,
            input_source,
            start_freq,
            stop_freq,
            points_per_decade,
            sweep,
            explicit_frequencies,
            data_table_name,
            contribution_detail,
            integration_mode,
            temperature,
        } => {
            let sweep_type = match sweep {
                NoiseSweepType::Decade | NoiseSweepType::ExplicitFrequencyList => {
                    AcSweepType::Decade
                }
                NoiseSweepType::Octave => AcSweepType::Octave,
                NoiseSweepType::Linear => AcSweepType::Linear,
                NoiseSweepType::Unsupported(index) => {
                    return Err(format!(
                        "Noise sweep mode {index} is outside the supported schema"
                    ));
                }
            };
            NoiseAnalysisConfig {
                output_node: output_node.clone(),
                reference_node: reference_node.clone(),
                input_source: input_source.clone(),
                sweep_type,
                num_points: *points_per_decade,
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                explicit_frequencies: explicit_frequencies.clone(),
                data_table_name: data_table_name.clone(),
                contribution_detail: *contribution_detail,
                integration_mode: *integration_mode,
                temperature_kelvin: *temperature,
            }
            .validate()
            .map_err(|errors| format!("Noise configuration is invalid: {}", errors.join("; ")))
        }
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
            filter,
            sweep,
        } => {
            let config = crate::simulation::config::SensitivityConfig {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
                filter: filter.clone(),
                sweep: sweep.map(crate::simulation::config::SensitivitySweep::from_spec),
            };
            config.validate().map_err(|errors| errors.join("; "))?;
            // The band is checked by building it with the engine's own grid
            // function, so a plan the Studio accepts is one the engine will
            // solve — and a plan it refuses is refused in the engine's words.
            if let Some(sweep) = config.ac_sweep_config() {
                rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
                    sweep.sweep_type.freq_variation(),
                    sweep.num_points,
                    sweep.start_freq,
                    sweep.stop_freq,
                    &rspice_core::abort_signal::NoAbort,
                )
                .map_err(|error| error.to_string())?;
            }
            Ok(())
        }
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => {
            if input_node.trim().is_empty() {
                return Err("Pole-zero input_node is required".to_string());
            }
            if input_ref.trim().is_empty() {
                return Err("Pole-zero input_ref is required".to_string());
            }
            if output_node.trim().is_empty() {
                return Err("Pole-zero output_node is required".to_string());
            }
            if output_ref.trim().is_empty() {
                return Err("Pole-zero output_ref is required".to_string());
            }
            let transfer = transfer_type.trim().to_ascii_uppercase();
            if transfer != "VOL" && transfer != "CUR" {
                return Err("Pole-zero transfer_type must be VOL or CUR".to_string());
            }
            let analysis = analysis_type.trim().to_ascii_uppercase();
            if analysis != "PZ" && analysis != "POL" && analysis != "ZER" {
                return Err("Pole-zero analysis_type must be PZ, POL, or ZER".to_string());
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("configured", other)),
    }
}
