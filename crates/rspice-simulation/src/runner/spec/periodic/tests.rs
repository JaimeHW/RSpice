//! Spectral study and periodic configuration tests.

use super::*;

fn envelope_current_config(
    method: rspice_simulation_contract::analysis_spec::EnvelopeInitialPeriodicSolve,
) -> svc_runner::EnvelopeRunConfig {
    svc_runner::EnvelopeRunConfig {
        multirate: None,
        initialization: svc_runner::EnvelopeInitializationConfig {
            pss_stabilization_periods: 0,
            pss_points_per_period: Some(64),
            ..Default::default()
        },
        fundamental_freq: 1e6,
        additional_carrier_tones: Vec::new(),
        stop_time: 4e-6,
        num_harmonics: 1,
        envelope_step: Some(0.5e-6),
        modulation_sources: vec!["Vmod".into()],
        initial_periodic_solve: method,
        adaptive_mode:
            rspice_simulation_contract::analysis_spec::EnvelopeAdaptiveMode::FixedEnvelopeStep,
        extraction_path:
            rspice_simulation_contract::analysis_spec::EnvelopeExtractionPath::Projection,
    }
}

#[test]
fn envelope_current_branches_survive_all_initializers_and_worker_transport() {
    use rspice_simulation_contract::analysis_spec::EnvelopeInitialPeriodicSolve as Init;
    let deck = "Current envelopes\nV1 in mod SIN(0 1 1Meg)\nVmod mod 0 PWL(0 0 4u 0)\nR1 in out 1k\nR2 out 0 1k\n.save I(V1) I(R1)\n.end\n";
    for method in [
        Init::TransientSpectralEstimate,
        Init::HarmonicBalance,
        Init::PeriodicSteadyState,
    ] {
        let result = run_envelope(
            deck,
            envelope_current_config(method),
            None,
            &rspice_core::abort_signal::NoAbort,
        )
        .unwrap();
        let result = crate::runner::worker_contract::round_trip_response_for_test(result);
        let SimulationResult::Transient {
            time, waveforms, ..
        } = result
        else {
            panic!("envelope result")
        };
        assert_eq!(waveforms.len(), 2);
        assert!(!time.is_empty());
        for (name, sign) in [("ENV(I(V1))", 1.0), ("ENV(I(R1))", -1.0)] {
            let wave = waveforms
                .values()
                .find(|wave| wave.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert_eq!(wave.y_unit, "A");
            assert!(wave.is_complex);
            assert_eq!(wave.x_values, time);
            for (real, imag) in wave.y_values.iter().zip(wave.y_imag.as_ref().unwrap()) {
                assert!(
                    real.abs() < 1e-7 && (imag - sign * 0.5e-3).abs() < 1e-7,
                    "{method:?} {name}: {real} + j{imag}"
                );
            }
        }
    }
}

#[test]
fn envelope_current_preserves_native_device_lead_units_and_values() {
    let deck = "MOS current envelope\nVd d 0 5\nVg g 0 SIN(2 0.01 1Meg)\nVmod mod 0 PWL(0 0 4u 0)\nM1 d g 0 0 N W=1u L=1u\n.model N NMOS LEVEL=1 VTO=1 KP=1m\n.save @M1[id]\n.end\n";
    let result = run_envelope(
        deck,
        envelope_current_config(
            rspice_simulation_contract::analysis_spec::EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
        ),
        None,
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let SimulationResult::Transient { waveforms, .. } = result else {
        panic!("envelope result")
    };
    assert_eq!(waveforms.len(), 1);
    let wave = waveforms.values().next().unwrap();
    assert!(wave.name.eq_ignore_ascii_case("ENV(@M1[id])"));
    assert_eq!(wave.y_unit, "A");
    for (real, imag) in wave.y_values.iter().zip(wave.y_imag.as_ref().unwrap()) {
        assert!(
            real.abs() < 2e-8 && (imag + 1e-5).abs() < 2e-8,
            "{real} + j{imag}"
        );
    }
}

#[test]
fn envelope_current_includes_ideal_capacitor_charge_in_source_current() {
    // The matched, unexcited line selects the core physical-event path,
    // which records exact charge impulses separately from finite samples.
    let deck = "Impulse current envelope\nV1 in 0 PWL(0 1 .5u 1 .5u 0 1u 0 1u 1 1.5u 1 1.5u 0 2u 0 2u 1 2.5u 1 2.5u 0 3u 0 3u 1 3.5u 1 3.5u 0 4u 0 4u 1)\nVmod mod 0 PWL(0 0 4u 0)\nC1 in 0 1n\nRnear near 0 50\nT1 near 0 far 0 Z0=50 TD=8u\nRfar far 0 50\n.save I(V1) I(C1)\n.end\n";
    let result = run_envelope(
        deck,
        envelope_current_config(
            rspice_simulation_contract::analysis_spec::EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
        ),
        None,
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let SimulationResult::Transient { waveforms, .. } = result else {
        panic!("envelope result")
    };
    for (name, expected) in [("ENV(I(V1))", -4e-3), ("ENV(I(C1))", 4e-3)] {
        let wave = waveforms
            .values()
            .find(|wave| wave.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert_eq!(wave.y_unit, "A");
        for (real, imag) in wave.y_values.iter().zip(wave.y_imag.as_ref().unwrap()) {
            assert!(
                (real - expected).abs() < 1e-7 && imag.abs() < 1e-7,
                "{name}: {real} + j{imag}"
            );
        }
    }
}

#[test]
fn fourier_results_preserve_voltage_and_current_dimensions() {
    assert_eq!(fourier_output_unit("V(out)"), "V");
    assert_eq!(fourier_output_unit("  i(Rload)"), "A");
}

/// Two outputs on one card are two projections of one transient, so the
/// run reports two spectra with their own derived quantities beside them,
/// from a single solve.
#[test]
fn an_authored_fourier_run_reports_every_output() {
    use crate::execution_artifact::ExecutionArtifactEnvelope;
    use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    let time = (0..=64)
        .map(|index| f64::from(index) / 64.0)
        .collect::<Vec<_>>();
    let fundamental = time
        .iter()
        .map(|time| (std::f64::consts::TAU * time).sin())
        .collect::<Vec<_>>();
    let halved = fundamental.iter().map(|value| value / 2.0).collect();
    let transient = SimulationResult::Transient {
        spectra: Vec::new(),
        time: time.clone(),
        waveforms: HashMap::from([
            (
                "out".to_owned(),
                WaveformData::new_time_domain("out", time.clone(), fundamental),
            ),
            (
                "mid".to_owned(),
                WaveformData::new_time_domain("mid", time, halved),
            ),
        ]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: None,
        events: Default::default(),
    };
    let artifact = ExecutionArtifactEnvelope::from_transient_result(
        ContentDigest::from_bytes([3; 32]),
        AnalysisInstanceId::new(),
        ObjectRevision::new(1).unwrap(),
        ContentDigest::from_bytes([4; 32]),
        &transient,
        &["out".to_owned(), "mid".to_owned()],
        false,
    )
    .unwrap()
    .unwrap();
    let request = |additional_outputs: Vec<String>| FourierRunRequest {
        num_harmonics: 3,

        num_periods: 1,
        output_node: "out".to_owned(),
        output_ref: "0".to_owned(),
        additional_outputs,
        start_time: 0.0,
        stop_time: 1.0,
        compute_thd: true,
        normalize: false,
    };
    let SimulationResult::Ac {
        frequencies,
        waveforms,
        ..
    } = run_fourier(
        1.0,
        request(vec!["V(mid)".to_owned()]),
        artifact.trajectory().unwrap(),
        svc_runner::ServiceContext::with_defaults(None, &rspice_core::abort_signal::NoAbort),
    )
    .unwrap()
    else {
        panic!("a Fourier run reports a spectrum");
    };

    let first = &waveforms["V(out) Spectrum"];
    let second = &waveforms["V(mid) Spectrum"];
    assert_eq!(first.x_values, frequencies);
    assert_eq!(second.x_values, frequencies);
    // The same solve, projected twice: the halved output's fundamental is
    // half of the first's.
    let magnitude = |waveform: &WaveformData| {
        let real = waveform.y_values[1];
        let imaginary = waveform.y_imag.as_ref().unwrap()[1];
        real.hypot(imaginary)
    };
    assert!(
        (magnitude(second) / magnitude(first) - 0.5).abs() < 1.0e-9,
        "{:?} vs {:?}",
        magnitude(second),
        magnitude(first)
    );
    for name in ["V(out) THD(%)", "V(mid) THD(%)", "V(out) DC", "V(mid) DC"] {
        assert!(waveforms.contains_key(name), "{:?}", waveforms.keys());
    }
    // A list qualifies its derived quantities by output, so the unqualified
    // names belong to a one-output run alone.
    assert!(!waveforms.contains_key("THD(%)"));
    assert!(!waveforms.contains_key("DC"));

    let SimulationResult::Ac { waveforms, .. } = run_fourier(
        1.0,
        request(Vec::new()),
        artifact.trajectory().unwrap(),
        svc_runner::ServiceContext::with_defaults(None, &rspice_core::abort_signal::NoAbort),
    )
    .unwrap() else {
        panic!("a Fourier run reports a spectrum");
    };
    assert!(waveforms.contains_key("V(out) Spectrum"));
    assert!(waveforms.contains_key("THD(%)"));
    assert!(waveforms.contains_key("DC"));

    // One output twice is a name collision, refused rather than silently
    // half-reported.
    assert!(matches!(
        run_fourier(
            1.0,
            request(vec!["V(out)".to_owned()]),
            artifact.trajectory().unwrap(),
            svc_runner::ServiceContext::with_defaults(None, &rspice_core::abort_signal::NoAbort),
        ),
        Err(SimulationError::InvalidConfig(_))
    ));
}

#[test]
fn convergence_fourier_retains_its_source_quality_through_native_conversion() {
    use crate::execution_artifact::ExecutionArtifactEnvelope;
    use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use rspice_results::analysis_type::AnalysisType;
    use rspice_results::convergence_quality::TransientConvergenceEvidence;
    let time = (0..=64)
        .map(|index| f64::from(index) / 64.0)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|time| (std::f64::consts::TAU * time).sin())
        .collect();
    let mut metrics = rspice_core::diagnostics::ConvergenceQuality::default();
    metrics.record_force_accept(10);
    let quality = std::sync::Arc::new(
        TransientConvergenceEvidence::capture(metrics, &time, &rspice_core::abort_signal::NoAbort)
            .unwrap(),
    );
    let transient = SimulationResult::Transient {
        spectra: Vec::new(),
        time: time.clone(),
        waveforms: HashMap::from([(
            "out".to_owned(),
            WaveformData::new_time_domain("out", time, values),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Some(quality.clone()),
        events: Default::default(),
    };
    let artifact = ExecutionArtifactEnvelope::from_transient_result(
        ContentDigest::from_bytes([1; 32]),
        AnalysisInstanceId::new(),
        ObjectRevision::new(1).unwrap(),
        ContentDigest::from_bytes([2; 32]),
        &transient,
        &["out".to_owned()],
        false,
    )
    .unwrap()
    .unwrap();
    let spectrum = run_fourier(
        1.0,
        FourierRunRequest {
            num_harmonics: 3,

            num_periods: 1,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            additional_outputs: Vec::new(),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        },
        artifact.trajectory().unwrap(),
        svc_runner::ServiceContext::with_defaults(None, &rspice_core::abort_signal::NoAbort),
    )
    .unwrap();
    assert_eq!(spectrum.transient_convergence(), Some(&quality));
    assert!(
        std::sync::Arc::ptr_eq(spectrum.transient_convergence().unwrap(), &quality),
        "derived spectra must share immutable quality buffers"
    );
    let retained =
        crate::result_conversion::convert(spectrum, AnalysisType::Fourier, "Spectrum", || 0.0);
    assert!(retained.success);
    assert_eq!(retained.convergence.as_ref(), Some(&quality));
    assert_eq!(
        quality
            .transient
            .time_basis
            .as_ref()
            .unwrap()
            .force_accepted_times_s,
        [10.0 / 64.0]
    );
}

#[test]
fn fourier_limits_cover_source_copies_and_every_selected_output() {
    use rspice_core::{NoAbort, ResourceLimits};
    let time: Vec<_> = (0..=64).map(|index| f64::from(index) / 64.0).collect();
    let names: Vec<_> = (0..14).map(|index| format!("n{index}")).collect();
    let waveforms = names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let values = time
                .iter()
                .map(|t| (index + 1) as f64 * (std::f64::consts::TAU * t).sin())
                .collect();
            (
                name.clone(),
                WaveformData::new_time_domain(name, time.clone(), values),
            )
        })
        .collect();
    let transient = SimulationResult::Transient {
        time,
        waveforms,
        spectra: Vec::new(),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: None,
        events: Default::default(),
    };
    let trajectory = TransientTrajectoryArtifact::from_result(&transient, &names, false)
        .unwrap()
        .unwrap();
    let spec = AnalysisSpec::Fourier {
        fundamental_freq: 1.0,
        num_harmonics: 3,
        num_periods: 1,
        output_node: "n0".into(),
        output_ref: "0".into(),
        additional_outputs: names
            .iter()
            .skip(1)
            .map(|name| format!("V({name})"))
            .collect(),
        start_time: 0.0,
        stop_time: 1.0,
        compute_thd: true,
        normalize: false,
    };
    let run = |spec, limits| {
        run_spectral_from_trajectory_with_context(
            spec,
            &trajectory,
            svc_runner::ServiceContext {
                source_path: None,
                limits,
                abort: &NoAbort,
            },
        )
    };
    let mut limits = ResourceLimits::default();
    limits.max_matrix_unknowns = 1;
    limits.max_analysis_points = 65;
    limits.max_result_values = 228;
    let SimulationResult::Ac {
        waveforms,
        frequencies,
        ..
    } = run(spec.clone(), limits).unwrap()
    else {
        panic!("Fourier result");
    };
    assert_eq!(frequencies, [0.0, 1.0, 2.0, 3.0]);
    assert_eq!(waveforms.len(), 42);
    let fundamental = &waveforms["V(n0) Spectrum"];
    assert!(
        (fundamental.y_values[1].hypot(fundamental.y_imag.as_ref().unwrap()[1]) - 1.0).abs()
            < 0.002
    );
    let mut fewer_points = limits;
    fewer_points.max_analysis_points = 64;
    assert!(
        matches!(run(spec.clone(), fewer_points), Err(SimulationError::ResourceLimit {
        resource, requested: 65, limit: 64,
    }) if resource == "analysis_points")
    );
    let mut fewer_values = limits;
    fewer_values.max_result_values = 227;
    assert!(
        matches!(run(spec.clone(), fewer_values), Err(SimulationError::ResourceLimit {
        resource, requested: 228, limit: 227,
    }) if resource == "result_values")
    );
    let mut one_output = spec;
    if let AnalysisSpec::Fourier {
        additional_outputs, ..
    } = &mut one_output
    {
        additional_outputs.clear();
    }
    fewer_values.max_result_values = 198;
    assert!(
        matches!(run(one_output, fewer_values), Err(SimulationError::ResourceLimit {
        resource, requested: 199, limit: 198,
    }) if resource == "result_values")
    );
}
