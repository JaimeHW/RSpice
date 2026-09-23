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

#[cfg(test)]
#[test]
fn hbnoise_reference_changes_identity_and_absence_preserves_legacy_bytes() {
    let mut spec = AnalysisSpec::Hbnoise {
        input_sideband: 0,
        output_sideband: 0,
        noise_reference: None,
        start_freq: 1e3,
        stop_freq: 1e4,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
        output_node: "out".into(),
        output_ref: "0".into(),
        input_source: "V1".into(),
        max_sideband: 1,
        integrated_noise: false,
        noise_figure: false,
        contributor_ranking: false,
    };
    let digest = |spec: &AnalysisSpec| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, spec);
        writer.finish()
    };
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_kind_tag(&spec));
    legacy.f64(1e3);
    legacy.f64(1e4);
    legacy.usize(3);
    encode_frequency_sweep(&mut legacy, FrequencySweep::Linear);
    legacy.string("out");
    legacy.string("0");
    legacy.string("V1");
    legacy.usize(1);
    legacy.bool(false);
    legacy.bool(false);
    legacy.bool(false);
    assert_eq!(digest(&spec), legacy.finish());
    let baseline = digest(&spec);
    for (input, output) in [(1, 0), (0, -1), (1, -1)] {
        let mut shifted = spec.clone();
        if let AnalysisSpec::Hbnoise {
            input_sideband,
            output_sideband,
            ..
        } = &mut shifted
        {
            *input_sideband = input;
            *output_sideband = output;
        }
        shifted.validate().unwrap();
        assert_ne!(digest(&shifted), baseline);
        let json = serde_json::to_string(&shifted).unwrap();
        let restored: AnalysisSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(digest(&shifted), digest(&restored));
    }
    if let AnalysisSpec::Hbnoise {
        noise_reference,
        noise_figure,
        ..
    } = &mut spec
    {
        *noise_figure = true;
        *noise_reference = Some(crate::services::simulation_runner::HbNoiseReference {
            source_resistor: "Rs".into(),
            temperature_kelvin: 290.0,
        });
    }
    let configured = digest(&spec);
    assert_ne!(configured, baseline);
    for (resistor, temperature) in [("Rs2", 290.0), ("Rs", 300.0)] {
        let mut changed = spec.clone();
        if let AnalysisSpec::Hbnoise {
            input_sideband: 0,
            output_sideband: 0,
            noise_reference: Some(reference),
            ..
        } = &mut changed
        {
            reference.source_resistor = resistor.into();
            reference.temperature_kelvin = temperature;
        }
        assert_ne!(digest(&changed), configured);
    }
}

#[cfg(test)]
#[test]
fn envelope_initializer_authenticates_every_control_and_preserves_legacy_identity() {
    use crate::services::simulation_runner::EnvelopeInitializationConfig;
    let spec = AnalysisSpec::Envelope {
        initialization: Default::default(),
        fundamental_freq: 1e6,
        additional_carrier_tones: vec![],
        stop_time: 4e-6,
        num_harmonics: 3,
        envelope_step: Some(0.5e-6),
        modulation_sources: vec!["VMOD".into()],
        initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
        adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
        extraction_path: EnvelopeExtractionPath::Projection,
    };
    let digest = |spec: &AnalysisSpec| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, spec);
        writer.finish()
    };
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_kind_tag(&spec));
    legacy.f64(1e6);
    legacy.sequence(0);
    legacy.f64(4e-6);
    legacy.usize(3);
    legacy.option(Some(&0.5e-6), |w, value| w.f64(*value));
    legacy.sequence(1);
    legacy.string("VMOD");
    legacy.u8(0);
    legacy.u8(1);
    legacy.u8(0);
    assert_eq!(digest(&spec), legacy.finish());
    let changes = serde_json::json!({
        "max_iterations": 73, "reltol": 2.5e-7, "abstol": 3e-13,
        "damping": 0.8, "verbose": true, "pss_stabilization_periods": 7,
        "pss_stabilization_time": 12.5e-6,
        "pss_points_per_period": 512, "pss_integration": "Gear2",
        "hb_min_damping": 0.025, "hb_oversample": 4, "hb_collocation_points": 33,
        "hb_use_krylov": true, "hb_gmres_restart": 19, "hb_source_stepping": true,
        "hb_exact_jacobian": false
    });
    let mut default = serde_json::to_value(EnvelopeInitializationConfig::default()).unwrap();
    // The absent zero time keeps existing serialized initializer statements.
    assert!(default.get("pss_stabilization_time").is_none());
    default["pss_stabilization_time"] = serde_json::json!(0.0);
    assert_eq!(
        changes.as_object().unwrap().len(),
        default.as_object().unwrap().len()
    );
    for (field, value) in changes.as_object().unwrap() {
        let mut config = default.clone();
        config[field] = value.clone();
        let mut changed = spec.clone();
        if let AnalysisSpec::Envelope { initialization, .. } = &mut changed {
            *initialization = serde_json::from_value(config).unwrap();
        }
        assert_ne!(digest(&spec), digest(&changed), "{field}");
    }
}

#[cfg(test)]
#[test]
fn optimization_units_and_expression_are_authenticated_without_changing_legacy_identity() {
    let spec = AnalysisSpec::Optimization {
        search: Default::default(),
        variables: vec![],
        objective_unit: String::new(),
        objective_expression: None,
        objective_node: "out".into(),
        objective_ref: "0".into(),
        goal: OptimizationGoal::Target,
        target: Some(1.2),
        algorithm: OptimizationAlgorithm::PatternSearch,
        max_iterations: 80,
        cost_tolerance: 1e-8,
        fd_step: 1e-4,
        initial_step: 0.2,
        min_step: 1e-7,
    };
    let digest = |spec: &AnalysisSpec| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, spec);
        writer.finish()
    };
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_kind_tag(&spec));
    legacy.sequence(0);
    legacy.string("out");
    legacy.string("0");
    legacy.u8(2);
    legacy.option(Some(&1.2), |w, value| w.f64(*value));
    legacy.u8(1);
    legacy.usize(80);
    for value in [1e-8, 1e-4, 0.2, 1e-7] {
        legacy.f64(value);
    }
    assert_eq!(digest(&spec), legacy.finish());
    let mut configured = spec.clone();
    if let AnalysisSpec::Optimization {
        objective_expression,
        ..
    } = &mut configured
    {
        *objective_expression = Some("I(V1)".into());
    }
    assert_ne!(digest(&spec), digest(&configured));
    let before = digest(&configured);
    if let AnalysisSpec::Optimization {
        objective_expression,
        ..
    } = &mut configured
    {
        *objective_expression = Some("-V(out)*I(V1)".into());
    }
    assert_ne!(before, digest(&configured));
    let before = digest(&configured);
    if let AnalysisSpec::Optimization { objective_unit, .. } = &mut configured {
        *objective_unit = "mW".into();
    }
    assert_ne!(before, digest(&configured));
}

#[cfg(test)]
#[test]
fn soa_legacy_identity_is_preserved_and_every_scoped_rule_field_is_authenticated() {
    use crate::services::{safety::SoAParameter, simulation_runner::SoaRuleConfig};
    let spec = AnalysisSpec::Soa {
        import_model_voltage_ratings: false,
        observation: Default::default(),
        rules: vec![],
        stop_time: 1e-6,
        step_time: 1e-9,
        check_vgs_max: true,
        max_vgs: 1.8,
        check_vds_max: true,
        max_vds: 3.3,
        check_vbe_max: true,
        max_vbe: 0.9,
        check_vce_max: true,
        max_vce: 5.0,
    };
    let digest = |spec: &AnalysisSpec| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, spec);
        writer.finish()
    };
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_kind_tag(&spec));
    legacy.f64(1e-6);
    legacy.f64(1e-9);
    for value in [1.8, 3.3, 0.9, 5.0] {
        legacy.bool(true);
        legacy.f64(value);
    }
    assert_eq!(digest(&spec), legacy.finish());
    for (warning_fraction, critical_fraction) in [
        (Some(0.8), Some(1.2)),
        (None, Some(1.2)),
        (Some(0.9), Some(1.5)),
        (Some(0.9), None),
    ] {
        let mut changed = spec.clone();
        let AnalysisSpec::Soa { observation, .. } = &mut changed else {
            unreachable!()
        };
        observation.thresholds = crate::services::safety::SoaThresholds {
            warning_fraction,
            critical_fraction,
        };
        assert_ne!(digest(&spec), digest(&changed));
    }
    let mut imported = spec.clone();
    let AnalysisSpec::Soa {
        import_model_voltage_ratings,
        ..
    } = &mut imported
    else {
        unreachable!()
    };
    *import_model_voltage_ratings = true;
    assert_ne!(digest(&spec), digest(&imported));
    let mut configured = spec.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut configured else {
        unreachable!()
    };
    rules.push(SoaRuleConfig {
        duration_mode: Default::default(),
        minimum_duration_s: None,
        current_envelope: None,
        power_derating: None,
        voltage_basis: Default::default(),
        parameter: SoAParameter::Vgs,
        max_value: 1e-3,
        devices: vec!["M1".into()],
        models: vec!["NM".into()],
    });
    assert_ne!(digest(&spec), digest(&configured));
    for field in 0..5 {
        let mut changed = configured.clone();
        let AnalysisSpec::Soa { rules, .. } = &mut changed else {
            unreachable!()
        };
        match field {
            0 => rules[0].parameter = SoAParameter::Vgd,
            1 => rules[0].max_value = 2e-3,
            2 => rules[0].devices = vec!["M2".into()],
            3 => rules[0].models = vec!["PM".into()],
            _ => rules[0].voltage_basis = crate::services::safety::SoaVoltageBasis::IntrinsicNodes,
        }
        assert_ne!(digest(&changed), digest(&configured), "rule field {field}");
    }

    let mut timed = configured.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut timed else {
        unreachable!()
    };
    rules[0].minimum_duration_s = Some(1e-9);
    assert_ne!(digest(&timed), digest(&configured));
    let mut longer = timed.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut longer else {
        unreachable!()
    };
    rules[0].minimum_duration_s = Some(2e-9);
    assert_ne!(digest(&timed), digest(&longer));
    let mut cumulative = timed.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut cumulative else {
        unreachable!()
    };
    rules[0].duration_mode = crate::services::safety::SoaDurationMode::Cumulative {
        recovery_time_s: None,
    };
    assert_ne!(digest(&timed), digest(&cumulative));
    let mut recovering = cumulative.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut recovering else {
        unreachable!()
    };
    rules[0].duration_mode = crate::services::safety::SoaDurationMode::Cumulative {
        recovery_time_s: Some(1e-9),
    };
    assert_ne!(digest(&cumulative), digest(&recovering));
    let mut faster = recovering.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut faster else {
        unreachable!()
    };
    rules[0].duration_mode = crate::services::safety::SoaDurationMode::Cumulative {
        recovery_time_s: Some(0.5e-9),
    };
    assert_ne!(digest(&recovering), digest(&faster));
    let mut derated = configured.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut derated else {
        unreachable!()
    };
    rules[0].parameter = SoAParameter::Pdiss;
    rules[0].power_derating = Some(crate::services::safety::SoaPowerDerating {
        reference_temperature_kelvin: 300.0,
        watts_per_kelvin: 0.001,
    });
    for reference in [true, false] {
        let mut changed = derated.clone();
        let AnalysisSpec::Soa { rules, .. } = &mut changed else {
            unreachable!()
        };
        let curve = rules[0].power_derating.as_mut().unwrap();
        if reference {
            curve.reference_temperature_kelvin = 320.0;
        } else {
            curve.watts_per_kelvin = 0.002;
        }
        assert_ne!(digest(&changed), digest(&derated));
    }
    let mut curves = configured.clone();
    let AnalysisSpec::Soa { rules, .. } = &mut curves else {
        unreachable!()
    };
    rules[0].parameter = SoAParameter::Id;
    rules[0].current_envelope = Some(crate::services::safety::SoaCurrentEnvelope::test_fixture());
    assert_ne!(digest(&curves), digest(&configured));
    for field in 0..10 {
        let mut changed = curves.clone();
        let AnalysisSpec::Soa { rules, .. } = &mut changed else {
            unreachable!()
        };
        let c = rules[0].current_envelope.as_mut().unwrap();
        match field {
            0 => c.source.push('x'),
            1 => c.conditions.push('x'),
            2 => c.voltages_v[1] *= 1.1,
            3 => c.dc_currents_a.as_mut().unwrap()[1] *= 0.9,
            4 => c.pulse_width_s = None,
            5 => c.voltage_interpolation = crate::services::safety::SoaVoltageInterpolation::Linear,
            6 => {
                c.pulse_interpolation = crate::services::safety::SoaPulseInterpolation::LongerPulse
            }
            7 => c.pulses[0].duration_s *= 0.9,
            8 => c.pulses[0].currents_a[1] *= 1.1,
            _ => c.pulses.pop().map(|_| ()).unwrap(),
        }
        assert_ne!(digest(&changed), digest(&curves), "curve field {field}");
    }
}

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
            table_options,
        } => {
            writer.string(table_name);
            encode_f64_slice(writer, frequencies);
            if *table_options != crate::simulation::config::AcDataTableOptions::default() {
                writer.domain("ac-data-table-options");
                writer.bool(table_options.from_netlist);
                writer.usize(table_options.parameter_columns.len());
                for column in &table_options.parameter_columns {
                    writer.string(&column.name);
                    encode_f64_slice(writer, &column.values);
                }
            }
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
            study,
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
            if let Some(study) = study {
                super::reliability::encode(writer, study);
            }
        }
        AnalysisSpec::Optimization {
            search,
            variables,
            objective_unit,
            objective_expression,
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
            if search != &crate::services::simulation_runner::OptimizationSearchControls::default()
            {
                writer.string("optimization-search-v1");
                writer.f64(search.var_tolerance);
                writer.f64(search.sa_initial_temp);
                writer.f64(search.sa_cooling_rate);
                writer.u64(search.random_seed);
            }
            if !search.variable_domains.is_empty() {
                writer.string("optimization-variable-domains-v1");
                writer.sequence(search.variable_domains.len());
                for (name, domain) in &search.variable_domains {
                    use crate::simulation::optimizer::OptimizationVariableDomain as Domain;
                    writer.string(name);
                    match domain {
                        Domain::Linear => writer.u8(0),
                        Domain::Logarithmic => writer.u8(1),
                        Domain::Quantized { step } => {
                            writer.u8(2);
                            writer.f64(*step);
                        }
                        Domain::Discrete { values } => {
                            writer.u8(3);
                            writer.sequence(values.len());
                            for value in values {
                                writer.f64(*value);
                            }
                        }
                    }
                }
            }
            if !objective_unit.is_empty() {
                writer.string("optimization-objective-unit/v1");
                writer.string(objective_unit);
            }
            if let Some(expression) = objective_expression {
                writer.string("optimization-expression-v1");
                writer.string(expression);
            }
        }
        AnalysisSpec::Soa {
            import_model_voltage_ratings,
            observation,
            rules,
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
            if *import_model_voltage_ratings {
                writer.string("soa-model-voltage-ratings-v1");
            }
            if *observation != crate::services::simulation_runner::SoaObservationConfig::default() {
                writer.string("soa-observation-v1");
                writer.f64(observation.start_time);
                writer.option(observation.max_step.as_ref(), |writer, value| {
                    writer.f64(*value)
                });
                writer.bool(observation.use_initial_conditions);
                writer.sequence(observation.devices.len());
                for name in &observation.devices {
                    writer.string(name);
                }
                writer.sequence(observation.models.len());
                for name in &observation.models {
                    writer.string(name);
                }
            }
            if !rules.is_empty() {
                writer.string("soa-scoped-rules-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    writer.string(rule.parameter.stress_code());
                    writer.f64(rule.max_value);
                    writer.sequence(rule.devices.len());
                    for name in &rule.devices {
                        writer.string(name);
                    }
                    writer.sequence(rule.models.len());
                    for name in &rule.models {
                        writer.string(name);
                    }
                }
            }
            if rules
                .iter()
                .any(|rule| rule.voltage_basis != Default::default())
            {
                writer.string("soa-voltage-bases-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    writer.bool(
                        rule.voltage_basis
                            == crate::services::safety::SoaVoltageBasis::IntrinsicNodes,
                    );
                }
            }
            if !observation.thresholds.is_default() {
                writer.string("soa-severity-thresholds-v1");
                writer.option(
                    observation.thresholds.warning_fraction.as_ref(),
                    |writer, value| writer.f64(*value),
                );
                writer.option(
                    observation.thresholds.critical_fraction.as_ref(),
                    |writer, value| writer.f64(*value),
                );
            }
            if rules.iter().any(|rule| rule.minimum_duration_s.is_some()) {
                writer.string("soa-minimum-excursion-duration-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    writer.option(rule.minimum_duration_s.as_ref(), |writer, value| {
                        writer.f64(*value)
                    });
                }
            }
            if rules.iter().any(|rule| !rule.duration_mode.is_default()) {
                writer.string("soa-cumulative-duration-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    match rule.duration_mode {
                        crate::services::safety::SoaDurationMode::PerExcursion => {
                            writer.bool(false)
                        }
                        crate::services::safety::SoaDurationMode::Cumulative {
                            recovery_time_s,
                        } => {
                            writer.bool(true);
                            writer.option(recovery_time_s.as_ref(), |writer, value| {
                                writer.f64(*value)
                            });
                        }
                    }
                }
            }
            if rules.iter().any(|rule| rule.power_derating.is_some()) {
                writer.string("soa-power-derating-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    writer.option(rule.power_derating.as_ref(), |writer, curve| {
                        writer.f64(curve.reference_temperature_kelvin);
                        writer.f64(curve.watts_per_kelvin);
                    });
                }
            }
            if rules.iter().any(|rule| rule.current_envelope.is_some()) {
                writer.string("soa-current-voltage-curves-v1");
                writer.sequence(rules.len());
                for rule in rules {
                    writer.option(rule.current_envelope.as_ref(), |writer, curve| {
                        writer.string(&curve.source);
                        writer.string(&curve.conditions);
                        encode_f64_slice(writer, &curve.voltages_v);
                        writer.option(curve.dc_currents_a.as_ref(), |writer, row| {
                            encode_f64_slice(writer, row)
                        });
                        writer.option(curve.pulse_width_s.as_ref(), |writer, width| {
                            writer.f64(*width)
                        });
                        writer.bool(
                            curve.voltage_interpolation
                                == crate::services::safety::SoaVoltageInterpolation::Logarithmic,
                        );
                        writer.bool(
                            curve.pulse_interpolation
                                == crate::services::safety::SoaPulseInterpolation::Logarithmic,
                        );
                        writer.sequence(curve.pulses.len());
                        for pulse in &curve.pulses {
                            writer.f64(pulse.duration_s);
                            encode_f64_slice(writer, &pulse.currents_a);
                        }
                    });
                }
            }
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
            initialization,
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
            if initialization
                != &crate::services::simulation_runner::EnvelopeInitializationConfig::default()
            {
                writer.string("envelope-initialization-v1");
                writer.usize(initialization.max_iterations);
                writer.f64(initialization.reltol);
                writer.f64(initialization.abstol);
                writer.f64(initialization.damping);
                writer.bool(initialization.verbose);
                writer.usize(initialization.pss_stabilization_periods);
                writer.option(
                    initialization.pss_points_per_period.as_ref(),
                    |writer, value| writer.usize(*value),
                );
                writer.f64(initialization.hb_min_damping);
                writer.usize(initialization.hb_oversample);
                writer.option(
                    initialization.hb_collocation_points.as_ref(),
                    |writer, value| writer.usize(*value),
                );
                writer.bool(initialization.hb_use_krylov);
                writer.usize(initialization.hb_gmres_restart);
                writer.bool(initialization.hb_source_stepping);
                writer.bool(initialization.hb_exact_jacobian);
                writer.u8(initialization.pss_integration as u8);
                if initialization.pss_stabilization_time != 0.0 {
                    writer.string("envelope-stabilization-time-v1");
                    writer.f64(initialization.pss_stabilization_time);
                }
            }

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
            controls,
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
            // Preserve identities of old default requests; authored controls
            // append a versioned tail rather than changing their prefix.
            if controls != &crate::simulation::multi_run::QpssControls::default() {
                writer.string("qpss-controls-v1");
                writer.f64(controls.current_absolute_tolerance);
                writer.f64(controls.voltage_absolute_tolerance);
                writer.usize(controls.max_backtracks);
                writer.option(controls.max_mixing_order.as_ref(), |w, value| {
                    w.usize(*value)
                });
                let (mode, counts) = match &controls.sampling {
                    rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling::Oversample(
                        counts,
                    ) => (0, counts),
                    rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling::Exact(counts) => {
                        (1, counts)
                    }
                };
                writer.usize(mode);
                writer.sequence(counts.len());
                for count in counts {
                    writer.usize(*count);
                }
                writer.bool(
                    controls.initial_state
                        == rspice_core::engine::QpssInitialState::DcOperatingPoint,
                );
                writer.sequence(controls.source_tones.len());
                for binding in &controls.source_tones {
                    writer.string(&binding.source);
                    writer.usize(binding.tone);
                }
                if !controls.linear.is_default() {
                    writer.string("qpss-linear-controls-v1");
                    writer.usize(match controls.linear.method {
                        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Auto=>0,
                        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Direct=>1,
                        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov=>2,
                    });
                    writer.usize(controls.linear.restart);
                    writer.usize(controls.linear.max_cycles);
                    writer.f64(controls.linear.relative_tolerance);
                }
            }
        }
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Psp { .. } => {
            encode_manifest_network(writer, spec);
        }
        AnalysisSpec::Hbnoise {
            input_sideband,
            output_sideband,
            noise_reference,
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
            if let Some(reference) = noise_reference {
                writer.string("hbnoise-source-reference-v1");
                writer.string(&reference.source_resistor);
                writer.f64(reference.temperature_kelvin);
            }
            if *input_sideband != 0 || *output_sideband != 0 {
                writer.string("hbnoise-conversion-sidebands-v1");
                writer.i32(*input_sideband);
                writer.i32(*output_sideband);
            }
        }
        AnalysisSpec::Qpac { controls, .. } => {
            encode_quasi_periodic_transfer(writer, spec);
            if controls != &crate::simulation::multi_run::QpacControls::default() {
                writer.string("qpac-controls-v1");
                writer.f64(controls.magnitude);
                writer.f64(controls.phase_degrees);
                writer.option(controls.explicit_offsets.as_ref(), |w, values| {
                    encode_f64_slice(w, values)
                });
                writer.usize(match controls.solver.linear.method {
                    rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Auto => 0,
                    rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Direct => 1,
                    rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov => 2,
                });
                writer.usize(controls.solver.linear.restart);
                writer.usize(controls.solver.linear.max_cycles);
                writer.f64(controls.solver.linear.relative_tolerance);
                writer.f64(controls.solver.current_absolute_tolerance);
                writer.f64(controls.solver.voltage_absolute_tolerance);
            }
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
            controls,
        } => {
            let generated = controls.explicit_frequencies.is_none();
            writer.f64(if generated { *start_freq } else { 0.0 });
            writer.f64(if generated { *stop_freq } else { 0.0 });
            writer.usize(if generated { *points_per_unit } else { 1 });
            encode_frequency_sweep(
                writer,
                if generated {
                    *sweep
                } else {
                    FrequencySweep::Linear
                },
            );
            writer.string(if controls.branch_current.is_none() {
                output_node
            } else {
                ""
            });
            writer.string(if controls.branch_current.is_none() {
                output_ref
            } else {
                ""
            });
            writer.string(if controls.input_referral {
                input_source
            } else {
                ""
            });
            let legacy_range = controls.noise_lattices.is_none();
            // Preserve the original two-tone layout; wider tuples get a versioned extension.
            for values in [lattice_min, lattice_max] {
                if legacy_range && lattice_min.len() == 2 && lattice_max.len() == 2 {
                    for value in values {
                        writer.i32(*value);
                    }
                } else {
                    writer.i32(0);
                    writer.i32(0);
                }
            }
            writer.bool(*integrated_noise);
            writer.bool(*contributor_ranking);
            if legacy_range && (lattice_min.len() != 2 || lattice_max.len() != 2) {
                writer.string("qpnoise-range-dimensions-v1");
                for values in [lattice_min, lattice_max] {
                    writer.sequence(values.len());
                    for value in values {
                        writer.i32(*value);
                    }
                }
            }
            encode_qpnoise_controls(writer, controls, *integrated_noise);
        }
        AnalysisSpec::Qpxf {
            group_delay,
            controls,
            ..
        } => {
            encode_quasi_periodic_transfer(writer, spec);
            writer.bool(*group_delay);
            encode_qpxf_controls(writer, controls);
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
            writer.u64(seed.unwrap_or(0));
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
            // Keep existing explicit-seed identities unchanged while separating
            // inherited seeds from an explicit zero, with or without a floor.
            if seed.is_none() {
                writer.string("inherited-transient-noise-seed");
            }
        }
        AnalysisSpec::DcMismatch {
            moment_options,
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
            if *moment_options != rspice_core::netlist::StatisticalMomentOptions::default() {
                writer.domain("rspice.dcmatch-moment-controls/v1");
                writer.f64(moment_options.relative_tolerance);
                writer.usize(moment_options.max_points);
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
        reltol,
        abstol,
        mixed_mode,
        noise_parameters,
        noise_reference,
    ) = match spec {
        AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ports,
            max_sideband,
            reltol,
            abstol,
            mixed_mode,
            noise_parameters,
            noise_reference,
        }
        | AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ports,
            max_sideband,
            reltol,
            abstol,
            mixed_mode,
            noise_parameters,
            noise_reference,
        } => (
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
            ports,
            *max_sideband,
            *reltol,
            *abstol,
            *mixed_mode,
            *noise_parameters,
            noise_reference.as_ref(),
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
    writer.f64(reltol);
    writer.f64(abstol);
    writer.bool(mixed_mode);
    writer.bool(noise_parameters);
    if let Some(reference) = noise_reference {
        writer.string("periodic-port-noise-reference-v1");
        writer.usize(reference.input_port);
        writer.usize(reference.output_port);
        writer.i32(reference.input_sideband);
        writer.i32(reference.output_sideband);
        writer.f64(reference.reference_temperature_kelvin);
        writer.f64(reference.termination_temperature_kelvin);
        writer.option(reference.image_sideband.as_ref(), |w, sideband| {
            w.i32(*sideband)
        });
    }
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
            ..
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
            input_lattice.as_slice(),
            output_lattice.as_slice(),
        ),
        _ => unreachable!("quasi-periodic transfer encoder accepts only QPAC or QPXF"),
    };
    // An explicit list makes generated-sweep editor buffers inactive.
    let (start_freq, stop_freq, points_per_unit, sweep) = if matches!(spec, AnalysisSpec::Qpac { controls, .. } if controls.explicit_offsets.is_some())
        || matches!(spec, AnalysisSpec::Qpxf { controls, .. } if controls.explicit_frequencies.is_some())
    {
        (0.0, 0.0, 1, FrequencySweep::Linear)
    } else {
        (start_freq, stop_freq, points_per_unit, sweep)
    };
    let (input_source, output_node, output_ref, input_lattice) =
        if let AnalysisSpec::Qpxf { controls, .. } = spec {
            (
                if controls.input_sources.is_some() {
                    ""
                } else {
                    input_source
                },
                if controls.branch_current.is_some() {
                    ""
                } else {
                    output_node
                },
                if controls.branch_current.is_some() {
                    ""
                } else {
                    output_ref
                },
                if controls.input_lattices.is_some() {
                    &[][..]
                } else {
                    input_lattice
                },
            )
        } else {
            (input_source, output_node, output_ref, input_lattice)
        };
    writer.f64(start_freq);
    writer.f64(stop_freq);
    writer.usize(points_per_unit);
    encode_frequency_sweep(writer, sweep);
    writer.string(input_source);
    writer.string(output_node);
    writer.string(output_ref);
    if input_lattice.len() != 2 || output_lattice.len() != 2 {
        writer.string("quasi-periodic-tuple-dimensions-v1");
        writer.usize(input_lattice.len());
        writer.usize(output_lattice.len());
    }
    for value in input_lattice {
        writer.i32(*value);
    }
    for value in output_lattice {
        writer.i32(*value);
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

/// Conditional extension preserves the bytes of the original single-source,
/// single-sideband voltage request while authenticating every new option.
fn encode_qpxf_controls(
    writer: &mut CanonicalWriter,
    controls: &crate::simulation::multi_run::QpxfControls,
) {
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
    use rspice_core::engine::{QpxfFrequencyAxis, QpxfInputLattices, QpxfSources};
    if controls == &crate::simulation::multi_run::QpxfControls::default() {
        return;
    }
    writer.string("qpxf-controls-v1");
    writer.u8(match controls.frequency_axis {
        QpxfFrequencyAxis::Output => 0,
        QpxfFrequencyAxis::Offset => 1,
    });
    writer.option(controls.explicit_frequencies.as_ref(), |w, v| {
        encode_f64_slice(w, v)
    });
    writer.option(controls.input_sources.as_ref(), |w, s| match s {
        QpxfSources::AllIndependent => w.u8(0),
        QpxfSources::Named(names) => {
            w.u8(1);
            w.sequence(names.len());
            for name in names {
                w.string(name);
            }
        }
    });
    writer.option(controls.input_lattices.as_ref(), |w, s| match s {
        QpxfInputLattices::AllRetained => w.u8(0),
        QpxfInputLattices::Explicit(tuples) => {
            w.u8(1);
            w.sequence(tuples.len());
            for tuple in tuples {
                w.sequence(tuple.len());
                for k in tuple {
                    w.i32(*k);
                }
            }
        }
        QpxfInputLattices::MaxOrders(orders) => {
            w.u8(2);
            w.sequence(orders.len());
            for n in orders {
                w.usize(*n);
            }
        }
    });
    writer.option(controls.branch_current.as_ref(), |w, v| w.string(v));
    writer.u8(match controls.solver.method {
        QuasiPeriodicLinearMethod::Auto => 0,
        QuasiPeriodicLinearMethod::Direct => 1,
        QuasiPeriodicLinearMethod::Krylov => 2,
    });
    writer.usize(controls.solver.restart);
    writer.usize(controls.solver.max_cycles);
    writer.f64(controls.solver.relative_tolerance);
    writer.f64(controls.group_delay_magnitude_floor);
}

/// Authenticate every selected measurement, source window and numerical setting.
fn encode_qpnoise_controls(
    w: &mut CanonicalWriter,
    c: &crate::simulation::multi_run::QpnoiseControls,
    integrated_noise: bool,
) {
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
    use rspice_core::engine::{
        QpnoiseFrequencyAxis, QpnoiseIntegrationMethod, QpnoiseLattices, QpnoiseObservation,
        QpnoiseSources,
    };
    fn tuple(w: &mut CanonicalWriter, t: &[i32]) {
        w.sequence(t.len());
        for k in t {
            w.i32(*k);
        }
    }
    let mut normalized = c.clone();
    if !integrated_noise {
        normalized.integration_band = None;
        normalized.integration_method = QpnoiseIntegrationMethod::Linear;
    }
    if !normalized.input_referral {
        normalized.input_lattice.clear();
    }
    let c = &normalized;
    if c == &crate::simulation::multi_run::QpnoiseControls::default() {
        return;
    }
    w.string("qpnoise-controls-v1");
    w.u8(match c.frequency_axis {
        QpnoiseFrequencyAxis::Output => 0,
        QpnoiseFrequencyAxis::Offset => 1,
    });
    w.option(c.explicit_frequencies.as_ref(), |w, v| {
        encode_f64_slice(w, v)
    });
    w.bool(c.input_referral);
    tuple(w, &c.input_lattice);
    tuple(w, &c.output_lattice);
    w.option(c.branch_current.as_ref(), |w, b| w.string(b));
    w.sequence(c.additional_outputs.len());
    for output in &c.additional_outputs {
        match &output.observation {
            QpnoiseObservation::Voltage { positive, negative } => {
                w.u8(0);
                w.string(positive);
                w.string(negative);
            }
            QpnoiseObservation::BranchCurrent { branch } => {
                w.u8(1);
                w.string(branch);
            }
        }
        tuple(w, &output.lattice);
    }
    w.option(c.noise_lattices.as_ref(), |w, l| match l {
        QpnoiseLattices::AllRetained => w.u8(0),
        QpnoiseLattices::Explicit { tuples } => {
            w.u8(1);
            w.sequence(tuples.len());
            for t in tuples {
                tuple(w, t);
            }
        }
        QpnoiseLattices::MaxOrders { orders } => {
            w.u8(2);
            w.sequence(orders.len());
            for n in orders {
                w.usize(*n);
            }
        }
        QpnoiseLattices::Range { minimum, maximum } => {
            w.u8(3);
            tuple(w, minimum);
            tuple(w, maximum);
        }
    });
    match &c.sources {
        QpnoiseSources::All => w.u8(0),
        QpnoiseSources::Only(names) | QpnoiseSources::Except(names) => {
            w.u8(if matches!(c.sources, QpnoiseSources::Only(_)) {
                1
            } else {
                2
            });
            w.sequence(names.len());
            for name in names {
                w.string(name);
            }
        }
    }
    w.option(c.integration_band.as_ref(), |w, b| encode_f64_slice(w, b));
    w.u8(match c.integration_method {
        QpnoiseIntegrationMethod::Linear => 0,
        QpnoiseIntegrationMethod::LogLog => 1,
    });
    w.option(c.noise_figure.as_ref(), |w, f| {
        w.string(&f.source_resistor);
        w.f64(f.reference_temperature);
        w.option(f.reference_lattices.as_ref(), |w, ts| {
            w.sequence(ts.len());
            for t in ts {
                tuple(w, t);
            }
        });
    });
    w.u8(match c.solver.method {
        QuasiPeriodicLinearMethod::Auto => 0,
        QuasiPeriodicLinearMethod::Direct => 1,
        QuasiPeriodicLinearMethod::Krylov => 2,
    });
    w.usize(c.solver.restart);
    w.usize(c.solver.max_cycles);
    w.f64(c.solver.relative_tolerance);
}
