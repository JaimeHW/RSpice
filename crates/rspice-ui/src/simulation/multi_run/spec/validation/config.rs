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
            // Whether the sweep retraces adds no constraint of its own: the
            // range and step are what have to be valid, and they are
            // checked below for either direction of travel.
            hysteresis: _,
        } => {
            if source_name.trim().is_empty() {
                return Err("DC sweep source_name is required".to_string());
            }
            if *step == 0.0 {
                return Err("DC sweep step cannot be zero".to_string());
            }
            if (stop - start).signum() != step.signum() {
                return Err("DC sweep step direction must match start/stop".to_string());
            }

            match (source2, start2, stop2, step2) {
                (None, None, None, None) => {}
                (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
                    if source2.trim().is_empty() {
                        return Err("DC sweep secondary source2 is required".to_string());
                    }
                    if source2.eq_ignore_ascii_case(source_name) {
                        return Err(
                            "DC sweep secondary source2 must differ from source_name".to_string()
                        );
                    }
                    if *step2 == 0.0 {
                        return Err("DC sweep secondary step2 cannot be zero".to_string());
                    }
                    if (stop2 - start2).signum() != step2.signum() {
                        return Err(
                            "DC sweep secondary step direction must match start2/stop2".to_string()
                        );
                    }
                }
                _ => {
                    return Err(
                        "DC sweep secondary sweep requires source2/start2/stop2/step2".to_string(),
                    );
                }
            }
            Ok(())
        }
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            ..
        } => {
            if *start_freq <= 0.0 {
                return Err("AC start_freq must be > 0".to_string());
            }
            if *stop_freq <= 0.0 {
                return Err("AC stop_freq must be > 0".to_string());
            }
            if *stop_freq <= *start_freq {
                return Err("AC stop_freq must be > start_freq".to_string());
            }
            if *points_per_unit == 0 {
                return Err("AC points_per_unit must be > 0".to_string());
            }
            Ok(())
        }
        AnalysisSpec::AcData {
            table_name,
            frequencies,
        } => {
            if table_name.trim().is_empty() {
                return Err("AC DATA table_name is required".to_string());
            }
            if frequencies.is_empty() {
                return Err("AC DATA frequencies must not be empty".to_string());
            }
            for (idx, frequency) in frequencies.iter().enumerate() {
                if !frequency.is_finite() || *frequency < 0.0 {
                    return Err(format!(
                        "AC DATA frequency {} must be finite and >= 0",
                        idx + 1
                    ));
                }
            }
            Ok(())
        }
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
            if output_var.trim().is_empty() {
                return Err("Sensitivity output_var is required".to_string());
            }
            if *ac_mode {
                if let Some(freq) = frequency
                    && *freq <= 0.0
                {
                    return Err("Sensitivity frequency must be > 0 for AC mode".to_string());
                }
            } else if frequency.is_some() {
                return Err("Sensitivity frequency is only valid in AC mode".to_string());
            } else if sweep.is_some() {
                return Err("Sensitivity sweep is only valid in AC mode".to_string());
            }
            // The band is checked by building it with the engine's own grid
            // function, so a plan the Studio accepts is one the engine will
            // solve — and a plan it refuses is refused in the engine's words.
            if let (Some(start), Some(sweep)) = (frequency, sweep) {
                rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
                    match sweep.variation {
                        crate::simulation::multi_run::FrequencySweep::Decade => {
                            rspice_core::netlist::FreqVariation::Dec
                        }
                        crate::simulation::multi_run::FrequencySweep::Octave => {
                            rspice_core::netlist::FreqVariation::Oct
                        }
                        crate::simulation::multi_run::FrequencySweep::Linear => {
                            rspice_core::netlist::FreqVariation::Lin
                        }
                    },
                    sweep.points as usize,
                    *start,
                    sweep.stop_frequency,
                    &rspice_core::abort_signal::NoAbort,
                )
                .map_err(|error| error.to_string())?;
            }
            // The filter reaches the `.SENS` card as written, so a token the
            // card would read as its own keyword, or a character its grammar
            // has no place for, is refused here rather than silently changing
            // the analysis the engine runs.
            crate::simulation::config::validate_sensitivity_filter(filter)
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
