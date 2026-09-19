//! Canonical encoding for one analysis specification.
//!
//! Split from the encoder it belongs to because this is the part that grows:
//! every analysis kind contributes an arm, and a kind that learns a parameter
//! learns a field in the digest. The bytes are the protocol, so nothing here
//! may be reordered, widened or re-tagged without a version bump — see
//! `CANONICAL_VERSION` in the parent.

use super::encode_dc_modes;

use crate::services::simulation_runner::{
    CornerProcess, PacFrequencySweep, PeriodicCarrier, PnoiseFrequencySweep, PxfFrequencySweep,
};
use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::dialog::{IntegrationMethod, OpConfig};
use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    FrequencySweep, OptimizationAlgorithm, OptimizationGoal,
};

use super::{CanonicalWriter, canonical_analysis_kind, encode_op_config, encode_op_fields};

pub(super) fn encode_analysis_spec(writer: &mut CanonicalWriter, spec: &AnalysisSpec) {
    writer.domain("analysis-spec");
    writer.u8(analysis_kind_tag(spec));
    match spec {
        AnalysisSpec::LegacyDcOp => encode_op_config(writer, &OpConfig::default()),
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
        } => encode_op_fields(
            writer,
            *temperature_mode,
            *temperature_celsius,
            *initial_guess,
            *node_initialization,
            *homotopy,
            *annotation,
            *device_detail,
            *save_device_op,
            *accuracy,
            selected_devices,
            previous_state.as_ref(),
            violation_devices,
            violation_source_content_digest.as_ref(),
            run_point.clone(),
        ),
        AnalysisSpec::Pac
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pstb
        | AnalysisSpec::Parametric
        | AnalysisSpec::Corner => {}
        // Monte Carlo digested nothing, because the trial count and the spread
        // travel on the card rather than in the specification. The varied
        // subset does decide which run this is, so it is appended as a
        // conditional tail: an unnamed subset writes nothing and digests
        // exactly as it did before the field existed.
        AnalysisSpec::MonteCarlo { params, .. } => {
            if !params.is_empty() {
                writer.sequence(params.len());
                for name in params {
                    writer.string(name);
                }
            }
        }
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            transfer_gain,
            input_resistance,
            output_resistance,
            normalization,
            accuracy,
        } => {
            writer.string(input_source);
            writer.string(output_expression);
            writer.bool(*transfer_gain);
            writer.bool(*input_resistance);
            writer.bool(*output_resistance);
            writer.u8(match normalization {
                crate::simulation::multi_run::TfNormalization::None => 0,
                crate::simulation::multi_run::TfNormalization::RelativeToNominal => 1,
                crate::simulation::multi_run::TfNormalization::PerSourceUnit => 2,
            });
            writer.u8(match accuracy {
                crate::simulation::multi_run::TfAccuracy::Fast => 0,
                crate::simulation::multi_run::TfAccuracy::Balanced => 1,
                crate::simulation::multi_run::TfAccuracy::Accurate => 2,
                crate::simulation::multi_run::TfAccuracy::Robust => 3,
            });
        }
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
        } => {
            writer.string(source_name);
            writer.f64(*start);
            writer.f64(*stop);
            writer.f64(*step);
            writer.option(source2.as_ref(), |w, v| w.string(v));
            writer.option(start2.as_ref(), |w, v| w.f64(*v));
            writer.option(stop2.as_ref(), |w, v| w.f64(*v));
            writer.option(step2.as_ref(), |w, v| w.f64(*v));
            // A retracing sweep visits twice the points and reports two
            // branches, so it is a different analysis from the one-way sweep
            // over the same range. Leaving it out of the identity would let
            // the two share a cache entry.
            writer.bool(*hysteresis);
            encode_dc_modes(writer, modes);
        }
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
        }
        AnalysisSpec::AcData {
            table_name,
            frequencies,
        } => {
            writer.string(table_name);
            encode_f64_slice(writer, frequencies);
        }
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.option(f2_over_f1.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => {
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.f64(*start_time);
            writer.option(max_timestep.as_ref(), |w, v| w.f64(*v));
            writer.bool(*uic);
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
            writer.string(output_node);
            writer.string(reference_node);
            writer.string(input_source);
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_decade);
            writer.u64(match sweep {
                NoiseSweepType::Decade => 0,
                NoiseSweepType::Octave => 1,
                NoiseSweepType::Linear => 2,
                NoiseSweepType::ExplicitFrequencyList => 3,
                NoiseSweepType::Unsupported(value) => value.saturating_add(4),
            });
            writer.option(explicit_frequencies.as_ref(), |w, frequencies| {
                encode_f64_slice(w, frequencies);
            });
            writer.option(data_table_name.as_ref(), |w, name| w.string(name));
            encode_noise_contribution_detail(writer, *contribution_detail);
            encode_noise_integration_mode(writer, *integration_mode);
            writer.f64(*temperature);
        }
        AnalysisSpec::Pss {
            method,
            fundamental_freq,
            tone_sources,
            tstab_periods,
            points_per_period,
            tolerance,
            oscillator_mode,
            oscillator_node,
            num_harmonics,
            integration_method,
            tstab,
            max_iterations,
            abstol,
            damping,
            max_period_change,
            verbose,
        } => {
            writer.u8(match method {
                crate::simulation::multi_run::PssMethod::Shooting => 0,
                crate::simulation::multi_run::PssMethod::HarmonicBalance => 1,
            });
            writer.f64(*fundamental_freq);
            writer.sequence(tone_sources.len());
            for source in tone_sources {
                writer.string(source);
            }
            writer.usize(*tstab_periods);
            writer.usize(*points_per_period);
            writer.f64(*tolerance);
            writer.bool(*oscillator_mode);
            writer.option(oscillator_node.as_ref(), |w, value| w.string(value));
            writer.usize(*num_harmonics);
            // Appended, never interleaved: the bytes are the protocol, so a
            // field added to an arm goes on its end. A request that took the
            // engine's default integration method therefore writes one more
            // byte than it used to and earns a new digest — which is correct,
            // because a digest identifies the encoding as well as the request,
            // and the same reasoning carried the DC sweep's retrace flag.
            writer.option(integration_method.as_ref(), |w, method| {
                w.u8(encode_integration_method(*method));
            });
            writer.f64(*tstab);
            writer.usize(*max_iterations);
            writer.f64(*abstol);
            writer.f64(*damping);
            writer.f64(*max_period_change);
            writer.bool(*verbose);
        }
        AnalysisSpec::PssSpectrum { num_harmonics } => {
            writer.usize(*num_harmonics);
        }
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            min_damping,
            oversample,
            collocation_points,
            max_mixing_order,
            use_krylov,
            gmres_restart,
            source_stepping,
            use_exact_jacobian,
            verbose,
        } => {
            writer.sequence(tones.len());
            for tone in tones {
                writer.f64(tone.frequency);
                writer.usize(tone.harmonics);
                writer.option(tone.source.as_ref(), |w, v| w.string(v));
                writer.option(tone.name.as_ref(), |w, v| w.string(v));
            }
            writer.f64(*reltol);
            writer.f64(*abstol);
            writer.usize(*max_iterations);
            writer.f64(*damping);
            writer.f64(*min_damping);
            writer.usize(*oversample);
            writer.option(collocation_points.as_ref(), |w, v| w.usize(*v));
            writer.usize(*max_mixing_order);
            writer.bool(*use_krylov);
            writer.usize(*gmres_restart);
            writer.bool(*source_stepping);
            writer.bool(*use_exact_jacobian);
            writer.bool(*verbose);
        }
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
            filter,
            sweep,
        } => {
            writer.string(output_var);
            writer.bool(*ac_mode);
            writer.option(frequency.as_ref(), |w, v| w.f64(*v));
            // The filter is appended only when it differs from what a plan
            // saved before filters existed computed. `PARAM:*` is that value,
            // not the empty string: a plan restored from an older project
            // keeps its identity because it keeps its computation, and an
            // emptied filter is a different run that must digest differently.
            // Never `writer.option`: an absent tail is the old encoding.
            if filter != crate::simulation::config::DESIGN_PARAMETERS_FILTER || sweep.is_some() {
                writer.string(filter);
                writer.option(sweep.as_ref(), |writer, sweep| {
                    writer.f64(sweep.stop_frequency);
                    writer.u64(u64::from(sweep.points));
                    encode_frequency_sweep(writer, sweep.variation);
                });
            }
        }
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => {
            writer.string(input_node);
            writer.string(input_ref);
            writer.string(output_node);
            writer.string(output_ref);
            writer.string(transfer_type);
            writer.string(analysis_type);
        }
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
            compute_nyquist,
        } => {
            writer.string(probe_node);
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            encode_frequency_sweep(writer, *sweep);
            writer.usize(*points_per_decade);
            writer.bool(*compute_nyquist);
        }
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            encode_f64_slice(writer, target_years);
            writer.bool(*enable_hci);
            writer.bool(*enable_nbti);
            writer.bool(*enable_em);
            writer.f64(*min_stress_voltage);
        }
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
        } => {
            writer.sequence(variables.len());
            for variable in variables {
                writer.string(&variable.name);
                writer.f64(variable.min);
                writer.f64(variable.max);
                writer.f64(variable.initial);
            }
            writer.string(objective_node);
            writer.string(objective_ref);
            writer.u8(match goal {
                OptimizationGoal::Minimize => 0,
                OptimizationGoal::Maximize => 1,
                OptimizationGoal::Target => 2,
            });
            writer.option(target.as_ref(), |w, v| w.f64(*v));
            writer.u8(match algorithm {
                OptimizationAlgorithm::GradientDescent => 0,
                OptimizationAlgorithm::PatternSearch => 1,
                OptimizationAlgorithm::SimulatedAnnealing => 2,
            });
            writer.usize(*max_iterations);
            writer.f64(*cost_tolerance);
            writer.f64(*fd_step);
            writer.f64(*initial_step);
            writer.f64(*min_step);
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
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.bool(*check_vgs_max);
            writer.f64(*max_vgs);
            writer.bool(*check_vds_max);
            writer.f64(*max_vds);
            writer.bool(*check_vbe_max);
            writer.f64(*max_vbe);
            writer.bool(*check_vce_max);
            writer.f64(*max_vce);
        }
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
            do_noise,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.f64(*z0);
            writer.sequence(ports.len());
            for port in ports {
                writer.string(&port.node_pos);
                writer.string(&port.node_neg);
                writer.option(port.z0.as_ref(), |w, v| w.f64(*v));
            }
            writer.bool(*do_noise);
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
            extraction_path,
        } => {
            writer.f64(*fundamental_freq);
            encode_f64_slice(writer, additional_carrier_tones);
            writer.f64(*stop_time);
            writer.usize(*num_harmonics);
            writer.option(envelope_step.as_ref(), |w, v| w.f64(*v));
            writer.sequence(modulation_sources.len());
            for source in modulation_sources {
                writer.string(source);
            }
            encode_envelope_initial_periodic_solve(writer, *initial_periodic_solve);
            encode_envelope_adaptive_mode(writer, *adaptive_mode);
            encode_envelope_extraction_path(writer, *extraction_path);
        }
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            num_periods,
            output_node,
            output_ref,
            additional_outputs,
            start_time,
            stop_time,
            compute_thd,
            normalize,
        } => {
            writer.f64(*fundamental_freq);
            writer.usize(*num_harmonics);
            writer.string(output_node);
            writer.string(output_ref);
            writer.f64(*start_time);
            writer.f64(*stop_time);
            writer.bool(*compute_thd);
            writer.bool(*normalize);
            // Which outputs a run decomposes decides which run it is, so the
            // list past the first is appended as a conditional tail: a
            // one-output specification writes nothing here and digests to
            // exactly the bytes it did before the list existed.
            if !additional_outputs.is_empty() {
                writer.sequence(additional_outputs.len());
                for output in additional_outputs {
                    writer.string(output);
                }
            }
            if *num_periods != 1 {
                writer.string("fourier-periods-v1");
                writer.usize(*num_periods);
            }
        }
        AnalysisSpec::Qpss {
            tones,
            max_iterations,
            relative_tolerance,
            autonomous,
            oscillator_node,
        } => {
            writer.sequence(tones.len());
            for tone in tones {
                writer.f64(tone.frequency);
                writer.usize(tone.harmonics);
                writer.option(tone.source.as_ref(), |w, value| w.string(value));
                writer.option(tone.name.as_ref(), |w, value| w.string(value));
            }
            writer.usize(*max_iterations);
            writer.f64(*relative_tolerance);
            writer.bool(*autonomous);
            writer.option(oscillator_node.as_ref(), |w, value| w.string(value));
        }
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Psp { .. } => {
            encode_manifest_network(writer, spec);
        }
        AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            integrated_noise,
            noise_figure,
            contributor_ranking,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.string(output_node);
            writer.string(output_ref);
            writer.string(input_source);
            writer.usize(*max_sideband);
            writer.bool(*integrated_noise);
            writer.bool(*noise_figure);
            writer.bool(*contributor_ranking);
        }
        AnalysisSpec::Qpac { .. } => {
            encode_quasi_periodic_transfer(writer, spec);
        }
        AnalysisSpec::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            output_node,
            output_ref,
            input_source,
            lattice_min,
            lattice_max,
            integrated_noise,
            contributor_ranking,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.string(output_node);
            writer.string(output_ref);
            writer.string(input_source);
            for value in lattice_min {
                writer.i32(*value);
            }
            for value in lattice_max {
                writer.i32(*value);
            }
            writer.bool(*integrated_noise);
            writer.bool(*contributor_ranking);
        }
        AnalysisSpec::Qpxf { group_delay, .. } => {
            encode_quasi_periodic_transfer(writer, spec);
            writer.bool(*group_delay);
        }
        AnalysisSpec::TransientNoise {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            seed,
            noise_fmax,
            noise_fmin,
            scale,
            uic,
        } => {
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.f64(*start_time);
            writer.f64(*max_timestep);
            writer.u64(*seed);
            writer.f64(*noise_fmax);
            writer.f64(*scale);
            writer.bool(*uic);
            // Appended, and written only when authored, so an absent floor
            // digests to exactly the bytes this arm produced before the
            // control existed. `writer.option` would tag the `None` with two
            // bytes and change the identity of every plan already saved —
            // the same reason `hb_operating_point_digest` appends its MNA
            // branches conditionally instead of wrapping them in an option.
            if let Some(noise_fmin) = noise_fmin {
                writer.f64(*noise_fmin);
            }
        }
        AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            contributor_limit,
            include_process,
            include_mismatch,
            normalized_contributions,
            contribution_threshold,
        } => {
            writer.string(output_expression);
            writer.f64(*sigma_multiplier);
            writer.usize(*contributor_limit);
            writer.bool(*include_process);
            writer.bool(*include_mismatch);
            writer.bool(*normalized_contributions);
            // Appended, and written only when authored, so an unauthored
            // threshold digests to exactly the bytes this arm produced before
            // the control existed. `writer.option` would tag the `None` and
            // change the identity of every plan already saved — the same
            // reason the transient-noise floor above appends conditionally.
            if let Some(threshold) = contribution_threshold {
                writer.f64(*threshold);
            }
        }
        // A brand-new arm, so `writer.option` is free here: there is no
        // previously sealed encoding of this shape for an absent field to
        // move. The card's own qualifiers are optional in the engine too.
        AnalysisSpec::Fft { request } => {
            writer.string(&request.output);
            writer.usize(request.points);
            writer.string(&request.window);
            writer.option(request.start.as_ref(), |w, value| w.f64(*value));
            writer.option(request.stop.as_ref(), |w, value| w.f64(*value));
            writer.option(request.format.as_ref(), |w, value| {
                w.string(value.keyword())
            });
            writer.option(request.alfa.as_ref(), |w, value| w.f64(*value));
            writer.option(request.fundamental.as_ref(), |w, value| w.f64(*value));
            writer.option(request.fmin.as_ref(), |w, value| w.f64(*value));
            writer.option(request.fmax.as_ref(), |w, value| w.f64(*value));
        }
    }
}

fn encode_manifest_network(writer: &mut CanonicalWriter, spec: &AnalysisSpec) {
    let (
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        ports,
        max_sideband,
        mixed_mode,
        noise_parameters,
    ) = match spec {
        AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ports,
            max_sideband,
            mixed_mode,
            noise_parameters,
        }
        | AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ports,
            max_sideband,
            mixed_mode,
            noise_parameters,
        } => (
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
            ports,
            *max_sideband,
            *mixed_mode,
            *noise_parameters,
        ),
        _ => unreachable!("network encoder accepts only HBSP or PSP"),
    };
    writer.f64(start_freq);
    writer.f64(stop_freq);
    writer.usize(points_per_unit);
    encode_frequency_sweep(writer, sweep);
    writer.sequence(ports.len());
    for port in ports {
        writer.string(&port.node_pos);
        writer.string(&port.node_neg);
        writer.option(port.z0.as_ref(), |w, value| w.f64(*value));
    }
    writer.usize(max_sideband);
    writer.bool(mixed_mode);
    writer.bool(noise_parameters);
}

fn encode_quasi_periodic_transfer(writer: &mut CanonicalWriter, spec: &AnalysisSpec) {
    let (
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        input_source,
        output_node,
        output_ref,
        input_lattice,
        output_lattice,
    ) = match spec {
        AnalysisSpec::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node,
            output_ref,
            input_lattice,
            output_lattice,
        }
        | AnalysisSpec::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node,
            output_ref,
            input_lattice,
            output_lattice,
            ..
        } => (
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
            input_source.as_str(),
            output_node.as_str(),
            output_ref.as_str(),
            *input_lattice,
            *output_lattice,
        ),
        _ => unreachable!("quasi-periodic transfer encoder accepts only QPAC or QPXF"),
    };
    writer.f64(start_freq);
    writer.f64(stop_freq);
    writer.usize(points_per_unit);
    encode_frequency_sweep(writer, sweep);
    writer.string(input_source);
    writer.string(output_node);
    writer.string(output_ref);
    for value in input_lattice {
        writer.i32(value);
    }
    for value in output_lattice {
        writer.i32(value);
    }
}

pub(super) fn encode_f64_slice(writer: &mut CanonicalWriter, values: &[f64]) {
    writer.sequence(values.len());
    for value in values {
        writer.f64(*value);
    }
}

/// One integration method's tag, appended in the enum's own order.
///
/// A separate function rather than an inline `match` because the same tag set
/// will be read by every analysis that learns to name a method; a second copy
/// is how two arms end up disagreeing about which byte means Gear.
fn encode_integration_method(method: IntegrationMethod) -> u8 {
    match method {
        IntegrationMethod::Trap => 0,
        IntegrationMethod::Euler => 1,
        IntegrationMethod::Gear2 => 2,
        IntegrationMethod::TrapGear => 3,
    }
}

fn encode_frequency_sweep(writer: &mut CanonicalWriter, sweep: FrequencySweep) {
    writer.u8(match sweep {
        FrequencySweep::Decade => 0,
        FrequencySweep::Octave => 1,
        FrequencySweep::Linear => 2,
    });
}

pub(super) fn encode_noise_contribution_detail(
    writer: &mut CanonicalWriter,
    detail: NoiseContributionDetail,
) {
    writer.u8(match detail {
        NoiseContributionDetail::Top50 => 0,
        NoiseContributionDetail::AllContributors => 1,
        NoiseContributionDetail::Top20 => 2,
        NoiseContributionDetail::SummaryOnly => 3,
    });
}

pub(super) fn encode_noise_integration_mode(
    writer: &mut CanonicalWriter,
    mode: NoiseIntegrationMode,
) {
    writer.u8(match mode {
        NoiseIntegrationMode::Enabled => 0,
        NoiseIntegrationMode::OutputNoiseOnly => 1,
        NoiseIntegrationMode::Disabled => 2,
    });
}

fn encode_envelope_initial_periodic_solve(
    writer: &mut CanonicalWriter,
    value: EnvelopeInitialPeriodicSolve,
) {
    writer.u8(match value {
        EnvelopeInitialPeriodicSolve::HarmonicBalance => 0,
        EnvelopeInitialPeriodicSolve::PeriodicSteadyState => 1,
        EnvelopeInitialPeriodicSolve::TransientSpectralEstimate => 2,
    });
}

fn encode_envelope_adaptive_mode(writer: &mut CanonicalWriter, value: EnvelopeAdaptiveMode) {
    writer.u8(match value {
        EnvelopeAdaptiveMode::Enabled => 0,
        EnvelopeAdaptiveMode::FixedEnvelopeStep => 1,
        EnvelopeAdaptiveMode::EventAlignedOnly => 2,
    });
}

fn encode_envelope_extraction_path(writer: &mut CanonicalWriter, value: EnvelopeExtractionPath) {
    writer.u8(match value {
        EnvelopeExtractionPath::Projection => 0,
    });
}

pub(in crate::simulation) const fn analysis_kind_tag(spec: &AnalysisSpec) -> u8 {
    canonical_analysis_kind(spec).tag()
}

pub(super) fn corner_process_tag(process: CornerProcess) -> u8 {
    match process {
        CornerProcess::TT => 0,
        CornerProcess::SS => 1,
        CornerProcess::FF => 2,
        CornerProcess::SF => 3,
        CornerProcess::FS => 4,
    }
}

pub(super) fn pac_sweep_tag(sweep: PacFrequencySweep) -> u8 {
    match sweep {
        PacFrequencySweep::Decade => 0,
        PacFrequencySweep::Octave => 1,
        PacFrequencySweep::Linear => 2,
    }
}

pub(super) fn pxf_sweep_tag(sweep: PxfFrequencySweep) -> u8 {
    match sweep {
        PxfFrequencySweep::Decade => 0,
        PxfFrequencySweep::Octave => 1,
        PxfFrequencySweep::Linear => 2,
    }
}

pub(super) fn pnoise_sweep_tag(sweep: PnoiseFrequencySweep) -> u8 {
    match sweep {
        PnoiseFrequencySweep::Decade => 0,
        PnoiseFrequencySweep::Octave => 1,
        PnoiseFrequencySweep::Linear => 2,
    }
}

/// The named carrier a periodic small-signal request states, or nothing.
///
/// Written as a *conditional tail* on each of the three execution-option arms
/// rather than as a field in the middle of them: every request recorded before
/// the carrier was authorable took the card's absent `FROM=`, which is
/// `Preceding`, so that position must add no bytes at all. A named carrier is
/// a different run — it binds a different producer — so it earns a tag, and
/// the tag goes last where it cannot move anything that already exists.
///
/// This is the shape `hb_operating_point_digest` uses for its own tail: bytes
/// appear only for the state that has them. `writer.option` is deliberately
/// not used, because its `None` tag would be a byte on every saved plan.
pub(super) fn encode_periodic_carrier_tail(writer: &mut CanonicalWriter, carrier: PeriodicCarrier) {
    match carrier {
        PeriodicCarrier::Preceding => {}
        PeriodicCarrier::Pss => writer.u8(0),
        PeriodicCarrier::Hb => writer.u8(1),
    }
}
