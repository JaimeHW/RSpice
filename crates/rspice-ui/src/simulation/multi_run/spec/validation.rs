//! Specification validation.
//!
//! Rejects a specification the engine could not execute — a missing
//! dependency, an empty sweep, an output that names nothing — before a run
//! starts rather than partway through.

use super::{AnalysisSpec, OptimizationGoal};
use crate::simulation::config::{AcSweepType, NoiseAnalysisConfig, NoiseSweepType};
use crate::simulation::dialog::OpConfig;

impl AnalysisSpec {
    /// Validate analysis parameters.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AnalysisSpec::LegacyDcOp => Ok(()),
            AnalysisSpec::DcOp {
                temperature_mode,
                temperature_celsius,
                initial_guess,
                node_initialization,
                homotopy,
                annotation,
                device_detail,
                save_device_op,
                accuracy,
                selected_devices,
                previous_state,
                violation_devices,
                violation_source_content_digest,
                run_point,
            } => OpConfig {
                temperature_mode: *temperature_mode,
                temperature_celsius: *temperature_celsius,
                initial_guess: *initial_guess,
                node_initialization: *node_initialization,
                homotopy: *homotopy,
                annotation: *annotation,
                device_detail: *device_detail,
                save_device_op: *save_device_op,
                accuracy: *accuracy,
                selected_devices: selected_devices.clone(),
                previous_state: previous_state.clone(),
                violation_devices: violation_devices.clone(),
                violation_source_content_digest: *violation_source_content_digest,
                run_point: *run_point,
            }
            .validate(),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
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
                            return Err("DC sweep secondary source2 must differ from source_name"
                                .to_string());
                        }
                        if *step2 == 0.0 {
                            return Err("DC sweep secondary step2 cannot be zero".to_string());
                        }
                        if (stop2 - start2).signum() != step2.signum() {
                            return Err(
                                "DC sweep secondary step direction must match start2/stop2"
                                    .to_string(),
                            );
                        }
                    }
                    _ => {
                        return Err(
                            "DC sweep secondary sweep requires source2/start2/stop2/step2"
                                .to_string(),
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
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                f2_over_f1,
                ..
            } => {
                if *start_freq <= 0.0 {
                    return Err("DISTO start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("DISTO stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("DISTO stop_freq must be > start_freq".to_string());
                }
                if *points_per_unit == 0 {
                    return Err("DISTO points_per_unit must be > 0".to_string());
                }
                if let Some(ratio) = f2_over_f1
                    && (!ratio.is_finite() || *ratio <= 1.0)
                {
                    return Err("DISTO f2_over_f1 must be finite and > 1".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                ..
            } => {
                if *stop_time <= 0.0 {
                    return Err("Transient stop_time must be > 0".to_string());
                }
                if *step_time <= 0.0 {
                    return Err("Transient step_time must be > 0".to_string());
                }
                if *step_time > *stop_time {
                    return Err("Transient step_time must be <= stop_time".to_string());
                }
                if !start_time.is_finite() || *start_time < 0.0 {
                    return Err("Transient start_time must be finite and >= 0".to_string());
                }
                if *start_time >= *stop_time {
                    return Err("Transient start_time must be < stop_time".to_string());
                }
                if let Some(max_step) = max_timestep
                    && (!max_step.is_finite() || *max_step <= 0.0)
                {
                    return Err(
                        "Transient max_timestep must be finite and > 0 when set".to_string()
                    );
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
            AnalysisSpec::Pss {
                method,
                fundamental_freq,
                tone_sources,
                tstab_periods: _,
                points_per_period,
                tolerance,
                oscillator_mode,
                oscillator_node,
                num_harmonics,
            } => {
                if *method != super::PssMethod::Shooting {
                    return Err(
                        "Legacy HB-PSS mode is not executable; use a Harmonic Balance analysis"
                            .to_owned(),
                    );
                }
                if !fundamental_freq.is_finite() || *fundamental_freq <= 0.0 {
                    return Err("PSS fundamental_freq must be finite and > 0".to_string());
                }
                if !*oscillator_mode && tone_sources.is_empty() {
                    return Err("PSS must bind at least one periodic tone source".to_owned());
                }
                for (index, source) in tone_sources.iter().enumerate() {
                    if source.trim().is_empty() || source.chars().any(char::is_control) {
                        return Err(format!("PSS tone source {} is invalid", index + 1));
                    }
                    if tone_sources[..index]
                        .iter()
                        .any(|prior| prior.eq_ignore_ascii_case(source))
                    {
                        return Err(format!("PSS tone source '{source}' is duplicated"));
                    }
                }
                if *points_per_period < 16 {
                    return Err("PSS points_per_period must be at least 16".to_owned());
                }
                if num_harmonics
                    .max(&1)
                    .checked_mul(2)
                    .is_none_or(|minimum| *points_per_period < minimum)
                {
                    return Err(
                        "PSS points_per_period must be at least twice num_harmonics".to_owned()
                    );
                }
                if !tolerance.is_finite() || *tolerance <= 0.0 {
                    return Err("PSS tolerance must be finite and > 0".to_string());
                }
                if *oscillator_mode
                    && oscillator_node
                        .as_deref()
                        .is_none_or(|node| node.trim().is_empty())
                {
                    return Err("PSS oscillator_node must be set in oscillator mode".to_string());
                }
                Ok(())
            }
            AnalysisSpec::PssSpectrum { num_harmonics } => {
                // A spectrum of nothing is not a result. The PSS request that
                // seeds this one already refuses a count its sample rate
                // cannot resolve, so this only has to reject the empty ask.
                if *num_harmonics == 0 {
                    return Err("PSS spectrum must retain at least one harmonic".to_owned());
                }
                Ok(())
            }
            AnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                gmres_restart,
                ..
            } => {
                if tones.is_empty() {
                    return Err("HB must define at least one tone".to_string());
                }
                for (idx, tone) in tones.iter().enumerate() {
                    if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                        return Err(format!("HB tone {} frequency must be > 0", idx + 1));
                    }
                    if tone.harmonics == 0 {
                        return Err(format!("HB tone {} harmonics must be > 0", idx + 1));
                    }
                }
                if !reltol.is_finite() || *reltol <= 0.0 {
                    return Err("HB reltol must be > 0".to_string());
                }
                if !abstol.is_finite() || *abstol <= 0.0 {
                    return Err("HB abstol must be > 0".to_string());
                }
                if *max_iterations == 0 {
                    return Err("HB max_iterations must be > 0".to_string());
                }
                if !damping.is_finite() || *damping <= 0.0 || *damping > 1.0 {
                    return Err("HB damping must be in (0, 1]".to_string());
                }
                if *oversample == 0 {
                    return Err("HB oversample must be > 0".to_string());
                }
                if let Some(points) = collocation_points {
                    if *points == 0 || *points % 2 == 0 {
                        return Err(
                            "HB collocation_points must be a positive odd integer".to_string()
                        );
                    }
                    let core_tones = tones
                        .iter()
                        .map(|tone| {
                            rspice_core::analysis::HbTone::new(tone.frequency, tone.harmonics)
                        })
                        .collect();
                    let core_config = rspice_core::analysis::HbConfig::multi_tone(core_tones);
                    let minimum = core_config.minimum_collocation_points().ok_or_else(|| {
                        "HB harmonic count exceeds the addressable collocation grid".to_string()
                    })?;
                    if *points < minimum {
                        return Err(format!(
                            "HB collocation_points must contain at least {minimum} points for the configured tones"
                        ));
                    }
                }
                if *max_mixing_order == 0 {
                    return Err("HB max_mixing_order must be > 0".to_string());
                }
                if *gmres_restart == 0 {
                    return Err("HB gmres_restart must be > 0".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
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
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
                ..
            } => {
                if probe_node.trim().is_empty() {
                    return Err("STB probe_node is required".to_string());
                }
                if *start_freq <= 0.0 {
                    return Err("STB start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("STB stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("STB stop_freq must be > start_freq".to_string());
                }
                if *points_per_decade == 0 {
                    return Err("STB points_per_decade must be > 0".to_string());
                }
                Ok(())
            }
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                z0,
                ports,
                ..
            } => {
                if *start_freq <= 0.0 {
                    return Err("S-parameter start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("S-parameter stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("S-parameter stop_freq must be > start_freq".to_string());
                }
                if *points_per_unit == 0 {
                    return Err("S-parameter points_per_unit must be > 0".to_string());
                }
                if *z0 <= 0.0 {
                    return Err("S-parameter z0 must be > 0".to_string());
                }
                if ports.len() < 2 {
                    return Err("S-parameter requires at least two ports".to_string());
                }
                for (idx, port) in ports.iter().enumerate() {
                    if port.node_pos.trim().is_empty() {
                        return Err(format!(
                            "S-parameter port{} positive node is required",
                            idx + 1
                        ));
                    }
                    if port.node_neg.trim().is_empty() {
                        return Err(format!(
                            "S-parameter port{} negative node is required",
                            idx + 1
                        ));
                    }
                    if let Some(port_z0) = port.z0
                        && (!port_z0.is_finite() || port_z0 <= 0.0)
                    {
                        return Err(format!("S-parameter port{} z0 must be > 0", idx + 1));
                    }
                }
                Ok(())
            }
            AnalysisSpec::Envelope {
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path: _,
            } => {
                let carrier_tones =
                    std::iter::once(fundamental_freq).chain(additional_carrier_tones.iter());
                let mut seen_tones = std::collections::HashSet::new();
                for tone in carrier_tones {
                    if !tone.is_finite() || *tone <= 0.0 {
                        return Err("Envelope carrier tones must be finite and > 0".to_string());
                    }
                    if !seen_tones.insert(tone.to_bits()) {
                        return Err("Envelope carrier tones must be unique".to_string());
                    }
                }
                if !stop_time.is_finite() || *stop_time <= 0.0 {
                    return Err("Envelope stop_time must be finite and > 0".to_string());
                }
                if *num_harmonics == 0 {
                    return Err("Envelope num_harmonics must be > 0".to_string());
                }
                if let Some(step) = envelope_step
                    && (!step.is_finite() || *step <= 0.0)
                {
                    return Err(
                        "Envelope envelope_step must be finite and > 0 when set".to_string()
                    );
                }
                if envelope_step.is_some_and(|step| step > *stop_time) {
                    return Err("Envelope envelope_step cannot exceed stop_time".to_string());
                }
                let legacy_source_inference = *initial_periodic_solve
                    == super::EnvelopeInitialPeriodicSolve::TransientSpectralEstimate
                    && *adaptive_mode == super::EnvelopeAdaptiveMode::FixedEnvelopeStep;
                if modulation_sources.is_empty() && !legacy_source_inference {
                    return Err(
                        "Envelope modulation_sources are required for periodic or adaptive execution"
                            .to_string(),
                    );
                }
                // Every source present in either a legacy or current request
                // has a stable, canonical identity.
                let mut seen_sources = std::collections::HashSet::new();
                for source in modulation_sources {
                    let trimmed = source.trim();
                    if trimmed.is_empty() || trimmed != source {
                        return Err(
                            "Envelope modulation source names must be nonempty and trimmed"
                                .to_string(),
                        );
                    }
                    if !seen_sources.insert(trimmed.to_ascii_lowercase()) {
                        return Err("Envelope modulation source names must be unique".to_string());
                    }
                }
                Ok(())
            }
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
                ..
            } => {
                if !fundamental_freq.is_finite() || *fundamental_freq <= 0.0 {
                    return Err("Fourier fundamental_freq must be finite and > 0".to_string());
                }
                if *num_harmonics == 0 {
                    return Err("Fourier num_harmonics must be > 0".to_string());
                }
                if output_node.trim().is_empty() {
                    return Err("Fourier output_node is required".to_string());
                }
                crate::services::simulation_runner::validate_fourier_output_accessor(
                    output_node,
                    Some(output_ref),
                )?;
                if !start_time.is_finite() || *start_time < 0.0 {
                    return Err("Fourier start_time must be finite and >= 0".to_string());
                }
                if !stop_time.is_finite() || *stop_time <= *start_time {
                    return Err(
                        "Fourier stop_time must be finite and greater than start_time".to_string(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => {
                if target_years.is_empty() {
                    return Err("Reliability target_years must not be empty".to_string());
                }
                if target_years
                    .iter()
                    .any(|years| !years.is_finite() || *years <= 0.0)
                {
                    return Err("Reliability target_years must be finite and > 0".to_string());
                }
                if !enable_hci && !enable_nbti && !enable_em {
                    return Err("Reliability requires at least one enabled mechanism".to_string());
                }
                if !min_stress_voltage.is_finite() || *min_stress_voltage < 0.0 {
                    return Err(
                        "Reliability min_stress_voltage must be finite and >= 0".to_string()
                    );
                }
                Ok(())
            }
            AnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
                ..
            } => {
                if variables.is_empty() {
                    return Err("Optimization variables must not be empty".to_string());
                }
                if objective_node.trim().is_empty() {
                    return Err("Optimization objective_node is required".to_string());
                }
                if objective_ref.trim().is_empty() {
                    return Err("Optimization objective_ref is required".to_string());
                }
                if objective_node.eq_ignore_ascii_case(objective_ref) {
                    return Err(
                        "Optimization objective_node and objective_ref must differ".to_string()
                    );
                }
                if *max_iterations == 0 {
                    return Err("Optimization max_iterations must be > 0".to_string());
                }
                if !cost_tolerance.is_finite() || *cost_tolerance <= 0.0 {
                    return Err("Optimization cost_tolerance must be finite and > 0".to_string());
                }
                if !fd_step.is_finite() || *fd_step <= 0.0 {
                    return Err("Optimization fd_step must be finite and > 0".to_string());
                }
                if !initial_step.is_finite() || *initial_step <= 0.0 {
                    return Err("Optimization initial_step must be finite and > 0".to_string());
                }
                if !min_step.is_finite() || *min_step <= 0.0 {
                    return Err("Optimization min_step must be finite and > 0".to_string());
                }
                if min_step > initial_step {
                    return Err("Optimization min_step must be <= initial_step".to_string());
                }
                if *goal == OptimizationGoal::Target {
                    if target.is_none() || target.is_some_and(|v| !v.is_finite()) {
                        return Err(
                            "Optimization target goal requires a finite target value".to_string()
                        );
                    }
                } else if target.is_some_and(|v| !v.is_finite()) {
                    return Err("Optimization target must be finite when provided".to_string());
                }

                let mut seen = std::collections::HashSet::new();
                for var in variables {
                    if var.name.trim().is_empty() {
                        return Err("Optimization variable name must not be empty".to_string());
                    }
                    if !var.min.is_finite() || !var.max.is_finite() || !var.initial.is_finite() {
                        return Err(format!(
                            "Optimization variable '{}' bounds/initial must be finite",
                            var.name
                        ));
                    }
                    if var.max <= var.min {
                        return Err(format!(
                            "Optimization variable '{}' requires max > min",
                            var.name
                        ));
                    }
                    if var.initial < var.min || var.initial > var.max {
                        return Err(format!(
                            "Optimization variable '{}' initial must be within [{}, {}]",
                            var.name, var.min, var.max
                        ));
                    }
                    if !seen.insert(var.name.to_ascii_uppercase()) {
                        return Err(format!(
                            "Optimization variable '{}' is defined more than once",
                            var.name
                        ));
                    }
                }
                Ok(())
            }
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => {
                if !stop_time.is_finite() || *stop_time <= 0.0 {
                    return Err("SOA stop_time must be finite and > 0".to_string());
                }
                if !step_time.is_finite() || *step_time <= 0.0 {
                    return Err("SOA step_time must be finite and > 0".to_string());
                }
                if step_time > stop_time {
                    return Err("SOA step_time must be <= stop_time".to_string());
                }
                if !check_vgs_max && !check_vds_max && !check_vbe_max && !check_vce_max {
                    return Err("SOA requires at least one enabled check".to_string());
                }
                if *check_vgs_max && (!max_vgs.is_finite() || *max_vgs <= 0.0) {
                    return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
                }
                if *check_vds_max && (!max_vds.is_finite() || *max_vds <= 0.0) {
                    return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
                }
                if *check_vbe_max && (!max_vbe.is_finite() || *max_vbe <= 0.0) {
                    return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
                }
                if *check_vce_max && (!max_vce.is_finite() || *max_vce <= 0.0) {
                    return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Qpss {
                tones,
                max_iterations,
                relative_tolerance,
                autonomous,
                oscillator_node,
            } => validate_qpss(
                tones,
                *max_iterations,
                *relative_tolerance,
                *autonomous,
                oscillator_node.as_deref(),
            ),
            AnalysisSpec::Hbsp {
                start_freq,
                stop_freq,
                points_per_unit,
                ports,
                max_sideband,
                ..
            } => validate_periodic_network(
                *start_freq,
                *stop_freq,
                *points_per_unit,
                ports,
                *max_sideband,
            ),
            AnalysisSpec::Psp {
                start_freq,
                stop_freq,
                points_per_unit,
                ports,
                max_sideband,
                mixed_mode,
                noise_parameters,
                ..
            } => {
                validate_periodic_network(
                    *start_freq,
                    *stop_freq,
                    *points_per_unit,
                    ports,
                    *max_sideband,
                )?;
                if *mixed_mode {
                    return Err(
                        "PSP mixed-mode conversion is not implemented; disable mixed mode"
                            .to_owned(),
                    );
                }
                if *noise_parameters {
                    return Err(
                        "PSP noise parameters require a correlated periodic-noise solve and are not implemented"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::Hbnoise {
                start_freq,
                stop_freq,
                points_per_unit,
                output_node,
                input_source,
                max_sideband,
                noise_figure,
                ..
            } => {
                validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
                if output_node.trim().is_empty() || input_source.trim().is_empty() {
                    return Err("HBNOISE requires an output node and input source".to_owned());
                }
                if *max_sideband == 0 {
                    return Err("HBNOISE max_sideband must be > 0".to_owned());
                }
                if *noise_figure {
                    return Err(
                        "HBNOISE noise figure requires explicit source impedance and available-noise temperature references"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::Qpac {
                start_freq,
                stop_freq,
                points_per_unit,
                input_source,
                output_node,
                ..
            }
            | AnalysisSpec::Qpxf {
                start_freq,
                stop_freq,
                points_per_unit,
                input_source,
                output_node,
                ..
            } => {
                validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
                if input_source.trim().is_empty() || output_node.trim().is_empty() {
                    return Err(
                        "quasi-periodic transfer requires an input source and output node"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::Qpnoise {
                start_freq,
                stop_freq,
                points_per_unit,
                output_node,
                input_source,
                lattice_min,
                lattice_max,
                ..
            } => {
                validate_frequency_sweep(*start_freq, *stop_freq, *points_per_unit)?;
                if output_node.trim().is_empty() || input_source.trim().is_empty() {
                    return Err("QPNOISE requires an output node and input source".to_owned());
                }
                if lattice_min
                    .iter()
                    .zip(lattice_max)
                    .any(|(min, max)| min > max)
                {
                    return Err("QPNOISE lattice minima must not exceed maxima".to_owned());
                }
                Ok(())
            }
            AnalysisSpec::TransientNoise {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                seed,
                noise_fmax,
                scale,
                ..
            } => {
                if !stop_time.is_finite() || *stop_time <= 0.0 {
                    return Err("TNOISE stop_time must be finite and > 0".to_owned());
                }
                if !step_time.is_finite() || *step_time <= 0.0 || step_time > stop_time {
                    return Err("TNOISE step_time must be finite, > 0, and <= stop_time".to_owned());
                }
                if !start_time.is_finite() || *start_time < 0.0 || start_time >= stop_time {
                    return Err(
                        "TNOISE start_time must be finite, >= 0, and < stop_time".to_owned()
                    );
                }
                if !max_timestep.is_finite() || *max_timestep <= 0.0 || max_timestep > stop_time {
                    return Err(
                        "TNOISE max_timestep must be finite, > 0, and <= stop_time".to_owned()
                    );
                }
                if *seed == 0
                    || !noise_fmax.is_finite()
                    || *noise_fmax <= 0.0
                    || !scale.is_finite()
                    || *scale <= 0.0
                {
                    return Err(
                        "TNOISE requires a nonzero seed, positive fmax, and positive scale"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::DcMismatch {
                output_expression,
                sigma_multiplier,
                contributor_limit,
                include_process,
                include_mismatch,
                ..
            } => {
                if output_expression.trim().is_empty() {
                    return Err("DCMATCH output_expression is required".to_owned());
                }
                if !sigma_multiplier.is_finite() || *sigma_multiplier <= 0.0 {
                    return Err("DCMATCH sigma_multiplier must be finite and > 0".to_owned());
                }
                if *contributor_limit == 0 {
                    return Err("DCMATCH contributor_limit must be > 0".to_owned());
                }
                if !include_process && !include_mismatch {
                    return Err("DCMATCH requires process or mismatch contributions".to_owned());
                }
                Ok(())
            }
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                ..
            } => {
                if input_source.trim().is_empty() {
                    return Err("TF input_source is required".to_owned());
                }
                if input_source != input_source.trim()
                    || input_source.trim().chars().any(char::is_whitespace)
                {
                    return Err("TF input_source must be one independent-source name".to_owned());
                }
                validate_tf_output_expression(output_expression)?;
                if !transfer_gain && !input_resistance && !output_resistance {
                    return Err(
                        "TF requires transfer gain, input resistance, or output resistance"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            AnalysisSpec::Pac
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pstb
            | AnalysisSpec::MonteCarlo { .. }
            | AnalysisSpec::Parametric
            | AnalysisSpec::Corner => Ok(()),
        }
    }
}

fn validate_tf_output_expression(expression: &str) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("TF output_expression must not contain surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err("TF output_expression must use V(node), V(node,ref), or I(element)".to_owned());
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("TF output_expression must contain one balanced probe call".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    let valid = if function.eq_ignore_ascii_case("V") {
        matches!(arguments.as_slice(), [node] if !node.is_empty())
            || matches!(arguments.as_slice(), [node, reference] if !node.is_empty() && !reference.is_empty())
    } else if function.eq_ignore_ascii_case("I") {
        matches!(arguments.as_slice(), [element] if !element.is_empty())
    } else {
        false
    };
    if !valid
        || arguments.iter().any(|argument| {
            *argument != argument.trim() || argument.chars().any(char::is_whitespace)
        })
    {
        return Err("TF output_expression must use V(node), V(node,ref), or I(element)".to_owned());
    }
    Ok(())
}

fn validate_frequency_sweep(start: f64, stop: f64, points: usize) -> Result<(), String> {
    if !start.is_finite() || start <= 0.0 || !stop.is_finite() || stop <= start {
        return Err("frequency sweep requires finite 0 < start < stop".to_owned());
    }
    if points == 0 {
        return Err("frequency sweep point count must be > 0".to_owned());
    }
    Ok(())
}

fn validate_qpss(
    tones: &[super::HbToneSpec],
    max_iterations: usize,
    relative_tolerance: f64,
    autonomous: bool,
    oscillator_node: Option<&str>,
) -> Result<(), String> {
    if tones.len() < 2 {
        return Err("QPSS requires at least two tones".to_owned());
    }
    if tones
        .iter()
        .any(|tone| !tone.frequency.is_finite() || tone.frequency <= 0.0 || tone.harmonics == 0)
    {
        return Err(
            "QPSS tones require positive finite frequencies and harmonic orders".to_owned(),
        );
    }
    if max_iterations == 0 || !relative_tolerance.is_finite() || relative_tolerance <= 0.0 {
        return Err("QPSS requires positive iterations and relative tolerance".to_owned());
    }
    if autonomous && oscillator_node.is_none_or(|node| node.trim().is_empty()) {
        return Err("autonomous QPSS requires an oscillator node".to_owned());
    }
    Ok(())
}

fn validate_periodic_network(
    start: f64,
    stop: f64,
    points: usize,
    ports: &[super::SpPort],
    max_sideband: usize,
) -> Result<(), String> {
    validate_frequency_sweep(start, stop, points)?;
    if ports.len() < 2 || max_sideband == 0 {
        return Err(
            "periodic network analysis requires at least two ports and positive max_sideband"
                .to_owned(),
        );
    }
    for (index, port) in ports.iter().enumerate() {
        if port.node_pos.trim().is_empty() || port.node_neg.trim().is_empty() {
            return Err(format!(
                "periodic network port {} requires both nodes",
                index + 1
            ));
        }
        if port.z0.is_some_and(|z0| !z0.is_finite() || z0 <= 0.0) {
            return Err(format!(
                "periodic network port {} z0 must be > 0",
                index + 1
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::multi_run::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve, HbToneSpec,
        PssMethod, TfAccuracy, TfNormalization,
    };

    fn tf_spec(output_expression: &str) -> AnalysisSpec {
        AnalysisSpec::Tf {
            input_source: "VIN_DIFF".to_owned(),
            output_expression: output_expression.to_owned(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
        }
    }

    #[test]
    fn tf_validation_accepts_only_exact_probe_grammar_and_one_source_token() {
        for valid in ["V(out)", "v(out,ref)", "I(Vsense)"] {
            assert!(tf_spec(valid).validate().is_ok(), "{valid}");
        }
        for invalid in [
            "",
            "out",
            "V()",
            "V(out,)",
            "V(a,b,c)",
            "I(V1,V2)",
            "P(R1)",
            " V(out)",
            "V(out) ",
            "V (out)",
            "V( out)",
            "V(out, ref)",
            "V((out))",
            "V(out) extra",
        ] {
            assert!(tf_spec(invalid).validate().is_err(), "{invalid}");
        }

        let mut invalid_source = tf_spec("V(out)");
        let AnalysisSpec::Tf { input_source, .. } = &mut invalid_source else {
            unreachable!()
        };
        *input_source = " VIN_DIFF".to_owned();
        assert!(invalid_source.validate().is_err());
        let AnalysisSpec::Tf { input_source, .. } = &mut invalid_source else {
            unreachable!()
        };
        *input_source = "VIN DIFF".to_owned();
        assert!(invalid_source.validate().is_err());
    }

    #[test]
    fn tf_validation_rejects_an_all_disabled_result_contract() {
        let mut spec = tf_spec("V(out)");
        let AnalysisSpec::Tf {
            transfer_gain,
            input_resistance,
            output_resistance,
            ..
        } = &mut spec
        else {
            unreachable!()
        };
        *transfer_gain = false;
        *input_resistance = false;
        *output_resistance = false;

        assert!(
            spec.validate()
                .expect_err("TF must retain at least one scalar")
                .contains("requires transfer gain")
        );
    }

    #[test]
    fn a_pss_spectrum_must_retain_at_least_one_harmonic() {
        assert!(
            AnalysisSpec::PssSpectrum { num_harmonics: 0 }
                .validate()
                .is_err()
        );
        assert!(
            AnalysisSpec::PssSpectrum { num_harmonics: 20 }
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn a_pss_spectrum_is_its_own_run_type_and_reads_as_a_coefficient_spectrum() {
        use crate::simulation::multi_run::AnalysisRunType;

        let spectrum = AnalysisSpec::PssSpectrum { num_harmonics: 20 };
        // Its own run type, not the PSS one: the two are separate retained
        // analyses and a shared type would collapse them in every label and
        // availability check that keys off it.
        assert_eq!(spectrum.run_type(), AnalysisRunType::PssSpectrum);
        assert_eq!(spectrum.run_type().display_name(), "PSS Spectrum");
    }

    #[test]
    fn legacy_pss_specs_receive_compatible_execution_defaults() {
        let spec: AnalysisSpec = serde_json::from_str(
            r#"{"Pss":{"fundamental_freq":1000000.0,"num_harmonics":9,"tolerance":0.000001}}"#,
        )
        .expect("legacy PSS spec deserializes");

        assert_eq!(
            spec,
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 1.0e6,
                tone_sources: vec!["VIN_DIFF".to_owned()],
                tstab_periods: 20,
                points_per_period: 512,
                tolerance: 1.0e-6,
                oscillator_mode: false,
                oscillator_node: None,
                num_harmonics: 9,
            }
        );
    }

    #[test]
    fn pss_validation_requires_an_explicit_autonomous_probe() {
        let spec = AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-6,
            oscillator_mode: true,
            oscillator_node: None,
            num_harmonics: 9,
        };

        assert!(spec.validate().is_err());
    }

    #[test]
    fn pss_validation_rejects_autonomous_harmonic_balance_before_dispatch() {
        let spec = AnalysisSpec::Pss {
            method: PssMethod::HarmonicBalance,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-6,
            oscillator_mode: true,
            oscillator_node: Some("out".to_owned()),
            num_harmonics: 9,
        };

        let error = spec
            .validate()
            .expect_err("autonomous harmonic balance must fail validation");
        assert!(error.contains("HB-PSS"));
    }

    #[test]
    fn legacy_envelope_spec_migrates_without_inventing_a_source_binding() {
        let spec: AnalysisSpec = serde_json::from_str(
            r#"{"Envelope":{"fundamental_freq":1000000.0,"stop_time":0.01,"num_harmonics":9,"max_step":0.000001}}"#,
        )
        .expect("legacy Envelope spec deserializes");

        assert_eq!(
            spec,
            AnalysisSpec::Envelope {
                fundamental_freq: 1.0e6,
                additional_carrier_tones: Vec::new(),
                stop_time: 0.01,
                num_harmonics: 9,
                envelope_step: Some(1.0e-6),
                modulation_sources: Vec::new(),
                initial_periodic_solve: EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
                adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
                extraction_path: EnvelopeExtractionPath::Preview,
            }
        );
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn envelope_validation_rejects_duplicate_tones_and_invalid_source_names() {
        let spec = |additional_carrier_tones, modulation_sources| AnalysisSpec::Envelope {
            fundamental_freq: 1.0e6,
            additional_carrier_tones,
            stop_time: 0.01,
            num_harmonics: 9,
            envelope_step: Some(1.0e-6),
            modulation_sources,
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
            adaptive_mode: EnvelopeAdaptiveMode::Enabled,
            extraction_path: EnvelopeExtractionPath::Preview,
        };

        assert!(
            spec(vec![1.0e6], vec!["VIN_AM".to_owned()])
                .validate()
                .unwrap_err()
                .contains("unique")
        );
        assert!(
            spec(Vec::new(), vec![" VIN_AM".to_owned()])
                .validate()
                .unwrap_err()
                .contains("trimmed")
        );
        assert!(
            spec(vec![2.0e6], vec!["VIN_AM".to_owned(), "vin_am".to_owned()])
                .validate()
                .unwrap_err()
                .contains("unique")
        );
        assert!(
            spec(Vec::new(), Vec::new())
                .validate()
                .unwrap_err()
                .contains("required")
        );
    }

    #[test]
    fn fourier_validation_rejects_non_finite_or_negative_windows() {
        let spec = |fundamental_freq, start_time, stop_time| AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics: 9,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            start_time,
            stop_time,
            compute_thd: true,
            normalize: false,
        };

        assert!(spec(f64::NAN, 0.0, 1.0).validate().is_err());
        assert!(spec(1.0, f64::NAN, 1.0).validate().is_err());
        assert!(spec(1.0, -1.0, 1.0).validate().is_err());
        assert!(spec(1.0, 0.0, f64::INFINITY).validate().is_err());
        assert!(spec(1.0, 0.0, 1.0).validate().is_ok());
    }

    #[test]
    fn fourier_validation_rejects_a_reference_on_a_current_accessor() {
        let spec = AnalysisSpec::Fourier {
            fundamental_freq: 1.0,
            num_harmonics: 9,
            output_node: "I(V1)".to_owned(),
            output_ref: "0".to_owned(),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        };

        assert_eq!(
            spec.validate().expect_err("current references are invalid"),
            "Fourier current output must not specify a voltage reference"
        );
    }

    fn hb_spec(collocation_points: Option<usize>) -> AnalysisSpec {
        AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1.0e6, 3)],
            reltol: 1.0e-6,
            abstol: 1.0e-12,
            max_iterations: 40,
            damping: 1.0,
            oversample: 2,
            collocation_points,
            max_mixing_order: 3,
            use_krylov: false,
            gmres_restart: 20,
            source_stepping: false,
            verbose: false,
        }
    }

    #[test]
    fn hb_validation_rejects_undersized_exact_grid() {
        let err = hb_spec(Some(5)).validate().expect_err("grid is undersized");
        assert!(err.contains("at least 7 points"));
    }

    #[test]
    fn hb_validation_rejects_non_finite_tolerances() {
        assert!(hb_spec_with_reltol(f64::NAN).validate().is_err());
    }

    fn hb_spec_with_reltol(reltol: f64) -> AnalysisSpec {
        let mut spec = hb_spec(None);
        let AnalysisSpec::HarmonicBalance {
            reltol: configured, ..
        } = &mut spec
        else {
            unreachable!();
        };
        *configured = reltol;
        spec
    }
}
