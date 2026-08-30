//! Round-trip and tamper cover for the frozen execution artifacts.
//!
//! Every seed and state buffer a worker returns is re-read here against the
//! digest that authorized it. A foreign or edited buffer is the dangerous
//! case: it still converges, so nothing downstream notices that the answer
//! belongs to a different frozen configuration.

use super::*;
use crate::simulation::results::WaveformData;

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn transient() -> SimulationResult {
    let time = vec![0.0, 0.5, 1.0];
    SimulationResult::Transient {
        time: time.clone(),
        waveforms: HashMap::from([
            (
                "V(out)".to_owned(),
                WaveformData::new_time_domain("V(out)", time.clone(), vec![0.0, 1.0, 0.0]),
            ),
            (
                "V(unused)".to_owned(),
                WaveformData::new_time_domain("V(unused)", time, vec![4.0, 5.0, 6.0]),
            ),
        ]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
    }
}

fn pss_spec(method: PssMethod) -> AnalysisSpec {
    AnalysisSpec::Pss {
        method,
        fundamental_freq: 1.0,
        tone_sources: vec!["V1".to_owned()],
        tstab_periods: 10,
        points_per_period: 16,
        tolerance: 1.0e-6,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 8,
    }
}

fn periodic_result() -> SimulationResult {
    let config = rspice_core::analysis::PssConfig::new(1.0)
        .with_harmonics(8)
        .with_tolerance(1.0e-6)
        .with_max_iterations(100)
        .with_tstab_periods(10)
        .with_points_per_period(16);
    let time = (0..=config.points_per_period)
        .map(|index| index as f64 / config.points_per_period as f64)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|time| (2.0 * std::f64::consts::PI * time).sin())
        .collect::<Vec<_>>();
    let certificate = rspice_core::analysis::FloquetSpectrumCertificate::new(
        1,
        0.0,
        rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(1),
    )
    .unwrap();
    let result = rspice_core::analysis::pss::PssResult {
        period: 1.0,
        frequency: 1.0,
        iterations: 3,
        residual_norm: 1.0e-10,
        time: time.clone(),
        waveforms: vec![rspice_core::analysis::pss::PeriodicWaveform::from_values(
            values.clone(),
        )],
        node_names: vec!["out".to_owned()],
        period_detected: false,
        floquet_multipliers: vec![num_complex::Complex64::new(0.9, 0.0)],
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate },
        floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind::Driven,
        trivial_floquet_multiplier_index: None,
    };
    let analysis = rspice_core::engine::PssAnalysisResult {
        result,
        iterations: 3,
        final_residual: 1.0e-10,
        period: 1.0,
        monodromy: vec![vec![0.9]],
        floquet_multipliers: vec![num_complex::Complex64::new(0.9, 0.0)],
        is_stable: true,
    };
    let operating_point =
        rspice_core::engine::PssOperatingPoint::try_from_parts(config, analysis, vec![0.25])
            .unwrap();
    SimulationResult::Transient {
        time: time.clone(),
        waveforms: HashMap::from([(
            "V(out)".to_owned(),
            WaveformData::new_time_domain("V(out)", time, values),
        )]),
        measurements: Vec::new(),
        periodic_state: Some(Arc::new(operating_point)),
        convergence: Default::default(),
        events: Default::default(),
    }
}

fn authenticated_periodic_result() -> SimulationResult {
    let netlist = rspice_core::netlist::Netlist::parse(
        "* authenticated artifact PSS fixture\n\
         V1 in 0 DC 1\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         C1 out 0 1p\n\
         .end\n",
    )
    .unwrap();
    let config = rspice_core::analysis::PssConfig::new(1.0)
        .with_harmonics(8)
        .with_tolerance(1.0e-6)
        .with_max_iterations(100)
        .with_tstab_periods(10)
        .with_points_per_period(16);
    let operating_point = rspice_core::engine::Engine::default()
        .run_pss_operating_point_with_abort(&netlist, config, &rspice_core::abort_signal::NoAbort)
        .unwrap();
    assert!(operating_point.producer_identity().is_some());
    let result = &operating_point.analysis().result;
    let waveforms = result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .map(|(node_name, waveform)| {
            (
                format!("V({node_name})"),
                WaveformData::new_time_domain(
                    format!("V({node_name})"),
                    result.time.clone(),
                    waveform.values.clone(),
                ),
            )
        })
        .collect();
    SimulationResult::Transient {
        time: result.time.clone(),
        waveforms,
        measurements: Vec::new(),
        periodic_state: Some(Arc::new(operating_point)),
        convergence: Default::default(),
        events: Default::default(),
    }
}

fn hb_spec() -> AnalysisSpec {
    AnalysisSpec::HarmonicBalance {
        tones: vec![crate::simulation::multi_run::HbToneSpec {
            frequency: 1.0,
            harmonics: 8,
            source: Some("V1".to_owned()),
            name: Some("fundamental".to_owned()),
        }],
        reltol: 1.0e-6,
        abstol: 1.0e-12,
        max_iterations: 100,
        damping: 1.0,
        oversample: 2,
        collocation_points: None,
        max_mixing_order: 5,
        use_krylov: false,
        gmres_restart: 30,
        source_stepping: false,
        verbose: false,
    }
}

fn hb_result() -> SimulationResult {
    let producer = hb_spec();
    let AnalysisSpec::HarmonicBalance {
        tones,
        reltol,
        abstol,
        max_iterations,
        damping,
        oversample,
        collocation_points,
        max_mixing_order,
        use_krylov,
        gmres_restart,
        source_stepping,
        verbose,
    } = producer
    else {
        unreachable!()
    };
    let config = crate::services::simulation_runner::build_core_hb_config(
        &crate::services::simulation_runner::HbRunConfig {
            tones: tones
                .into_iter()
                .map(|tone| crate::services::simulation_runner::HbToneRunConfig {
                    frequency: tone.frequency,
                    harmonics: tone.harmonics,
                    source: tone.source,
                    name: tone.name,
                })
                .collect(),
            reltol,
            abstol,
            max_iterations,
            damping,
            oversample,
            collocation_points,
            max_mixing_order,
            use_krylov,
            gmres_restart,
            source_stepping,
            verbose,
        },
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let coefficients = (0..=config.num_harmonics)
        .map(|harmonic| {
            num_complex::Complex64::new(
                harmonic as f64 * 0.1,
                if harmonic == 0 { 0.0 } else { -0.25 },
            )
        })
        .collect::<Vec<_>>();
    let branch_coefficients = (0..=config.num_harmonics)
        .map(|harmonic| {
            num_complex::Complex64::new(
                -1.0e-3 / (harmonic + 1) as f64,
                if harmonic == 0 {
                    0.0
                } else {
                    harmonic as f64 * 1.0e-5
                },
            )
        })
        .collect::<Vec<_>>();
    let operating_point = rspice_core::engine::HbOperatingPoint::try_from_parts_with_mna_branches(
        config,
        vec!["out".to_owned()],
        vec![coefficients.clone()],
        vec!["V1".to_owned()],
        vec![branch_coefficients],
        4,
        1.0e-10,
    )
    .unwrap();
    SimulationResult::HarmonicBalance {
        frequencies: (0..=8).map(|harmonic| harmonic as f64).collect(),
        waveforms: HashMap::from([(
            "V(out)".to_owned(),
            WaveformData::new_complex(
                "V(out)",
                (0..=8).map(|harmonic| harmonic as f64).collect(),
                coefficients.iter().map(|value| value.re * 2.0).collect(),
                coefficients.iter().map(|value| value.im * 2.0).collect(),
            ),
        )]),
        measurements: Vec::new(),
        operating_point: Arc::new(operating_point),
    }
}

#[test]
fn hb_state_transfer_round_trips_and_rejects_tamper() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(18).unwrap();
    let snapshot = digest(41);
    let config_digest = digest(42);
    let binding = PreparedDependencyBinding::hb_state(producer, revision, config_digest);
    let artifact = ExecutionArtifactEnvelope::from_hb_result(
        snapshot,
        producer,
        revision,
        config_digest,
        &hb_spec(),
        &hb_result(),
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();
    let hbsp = AnalysisSpec::Hbsp {
        start_freq: 1.0e3,
        stop_freq: 1.0e6,
        points_per_unit: 3,
        sweep: crate::simulation::multi_run::FrequencySweep::Decade,
        ports: vec![
            crate::simulation::multi_run::SpPort {
                node_pos: "p1".to_owned(),
                node_neg: "0".to_owned(),
                z0: Some(50.0),
            },
            crate::simulation::multi_run::SpPort {
                node_pos: "p2".to_owned(),
                node_neg: "0".to_owned(),
                z0: Some(50.0),
            },
        ],
        max_sideband: 1,
        mixed_mode: false,
        noise_parameters: false,
    };
    validate_prepared_dependency_contract(&hbsp, &hb_spec()).unwrap();
    resolved.validate_for_spec(&hbsp).unwrap();
    let hbnoise = AnalysisSpec::Hbnoise {
        start_freq: 1.0e3,
        stop_freq: 1.0e6,
        points_per_unit: 10,
        sweep: crate::simulation::multi_run::FrequencySweep::Decade,
        output_node: "out".to_owned(),
        output_ref: "0".to_owned(),
        input_source: "vin".to_owned(),
        max_sideband: 4,
        integrated_noise: true,
        noise_figure: false,
        contributor_ranking: true,
    };
    validate_prepared_dependency_contract(&hbnoise, &hb_spec()).unwrap();
    resolved.validate_for_spec(&hbnoise).unwrap();
    assert_eq!(
        resolved.hb_state().unwrap().operating_point().iterations(),
        4
    );
    assert_eq!(
        resolved
            .hb_state()
            .unwrap()
            .operating_point()
            .mna_branch_names(),
        &["V1"]
    );

    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    assert_eq!(
        buffers.len(),
        4,
        "node and MNA branch rows each carry real and imaginary buffers"
    );
    assert_eq!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone()).unwrap(),
        resolved
    );
    let mut tampered = buffers.clone();
    tampered[2][3] += 1.0;
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, tampered),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));

    let mut identity: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    let branch_name =
        identity["artifacts"][0]["payload"]["HbState"]["mna_branch_spectra"][0]["branch_name"]
            .as_str()
            .unwrap();
    assert_eq!(branch_name, "V1");
    identity["artifacts"][0]["payload"]["HbState"]["mna_branch_spectra"][0]["branch_name"] =
        serde_json::Value::String("VDRIFT".to_owned());
    let identity = serde_json::to_string(&identity).unwrap();
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&identity, buffers),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));

    let mut config: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    config["artifacts"][0]["payload"]["HbState"]["config"]["tolerance"] = serde_json::json!(1.0e-4);
    let config = serde_json::to_string(&config).unwrap();
    let (_, buffers) = resolved.encode_transfer().unwrap();
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&config, buffers),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));
}

#[test]
fn hb_artifact_rejects_returned_state_from_another_frozen_config() {
    let mut producer = hb_spec();
    let AnalysisSpec::HarmonicBalance { reltol, .. } = &mut producer else {
        unreachable!()
    };
    *reltol = 1.0e-5;
    let error = ExecutionArtifactEnvelope::from_hb_result(
        digest(51),
        AnalysisInstanceId::new(),
        ObjectRevision::new(19).unwrap(),
        digest(52),
        &producer,
        &hb_result(),
    )
    .expect_err("HB state from a different frozen basis must fail closed");
    assert!(matches!(error, ExecutionArtifactError::ContractMismatch(_)));
    assert!(error.to_string().contains("frozen producer specification"));
}

fn dc_operating_point_result() -> SimulationResult {
    let mut configuration = crate::simulation::dialog::OpConfig::default();
    configuration.temperature_celsius = 125.0;
    configuration.run_point.supply_voltage = Some(1.2);
    configuration.run_point.nominal_supply_voltage = Some(1.0);
    configuration.run_point.supply_source_names = vec!["V1".to_owned()];
    SimulationResult::DcOp(Box::new(crate::simulation::results::DcOpResult {
        configuration,
        mna_node_names: vec!["in".to_owned(), "out".to_owned()],
        mna_branch_names: vec!["V1".to_owned()],
        mna_solution: vec![1.2, 0.8, -0.001],
        ..Default::default()
    }))
}

#[test]
fn shooting_pss_seed_round_trips_in_one_exact_buffer_and_rejects_tamper() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(17).unwrap();
    let snapshot = digest(21);
    let config = digest(22);
    let source = digest(23);
    let binding = PreparedDependencyBinding::dc_operating_point_seed(producer, revision, config);
    let result = dc_operating_point_result();
    let SimulationResult::DcOp(op_result) = &result else {
        unreachable!()
    };
    let artifact = ExecutionArtifactEnvelope::from_dc_operating_point_result(
        snapshot,
        producer,
        revision,
        config,
        source,
        &op_result.configuration,
        &result,
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();
    resolved
        .validate_for_spec(&pss_spec(PssMethod::Shooting))
        .unwrap();
    let seed = resolved.dc_operating_point_seed().unwrap();
    assert_eq!(seed.effective_source_content_digest(), source);
    assert_eq!(seed.temperature_celsius().to_bits(), 125.0_f64.to_bits());
    assert_eq!(seed.supply_voltage(), Some(1.2));

    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    assert_eq!(
        buffers.len(),
        1,
        "OP transfer owns one Float64 solution buffer"
    );
    assert_eq!(buffers[0], vec![1.2, 0.8, -0.001]);
    assert_eq!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone()).unwrap(),
        resolved
    );

    let mut nonfinite = buffers.clone();
    nonfinite[0][1] = f64::NAN;
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, nonfinite),
        Err(ExecutionArtifactError::InvalidPayload(_))
    ));

    let mut decoded: ResolvedExecutionDependenciesTransferMetadata =
        serde_json::from_str(&metadata).unwrap();
    decoded.artifacts.push(decoded.artifacts[0].clone());
    decoded.bindings.push(decoded.bindings[0].clone());
    let duplicate_metadata = serde_json::to_string(&decoded).unwrap();
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&duplicate_metadata, buffers),
        Err(ExecutionArtifactError::Transport(message)) if message.contains("referenced more than once")
    ));
}

#[test]
fn shooting_pss_requires_op_and_legacy_hb_pss_fails_closed() {
    let op = AnalysisSpec::LegacyDcOp;
    validate_prepared_dependency_contract(&pss_spec(PssMethod::Shooting), &op)
        .expect("shooting PSS consumes an earlier OP seed");
    let error = validate_prepared_dependency_contract(&pss_spec(PssMethod::HarmonicBalance), &op)
        .unwrap_err();
    assert!(error.to_string().contains("not executable"));
}

#[test]
fn op_seed_rejects_worker_returned_environment_tamper() {
    let result = dc_operating_point_result();
    let SimulationResult::DcOp(op_result) = &result else {
        unreachable!()
    };
    let prepared_config = op_result.configuration.clone();
    for tamper in [0_u8, 1, 2] {
        let mut returned = result.clone();
        let SimulationResult::DcOp(returned_op) = &mut returned else {
            unreachable!()
        };
        match tamper {
            0 => returned_op.configuration.temperature_celsius = 25.0,
            1 => returned_op.configuration.run_point.supply_voltage = Some(1.3),
            _ => returned_op.configuration.accuracy = crate::simulation::dialog::OpAccuracy::Robust,
        }
        let error = ExecutionArtifactEnvelope::from_dc_operating_point_result(
            digest(1),
            AnalysisInstanceId::new(),
            ObjectRevision::new(1).unwrap(),
            digest(2),
            digest(3),
            &prepared_config,
            &returned,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("prepared producer configuration")
        );
    }
}

#[test]
fn exact_binding_resolves_and_tampered_payload_fails_closed() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(3).unwrap();
    let binding = PreparedDependencyBinding::transient_trajectory(producer, revision, digest(2));
    let mut artifact = ExecutionArtifactEnvelope::from_transient_result(
        digest(1),
        producer,
        revision,
        digest(2),
        &transient(),
        &["out".to_owned()],
    )
    .unwrap()
    .unwrap();
    assert_eq!(artifact.trajectory().unwrap().waveforms.len(), 1);
    assert!(artifact.trajectory().unwrap().waveform("unused").is_none());
    let encoded = serde_json::to_string(&artifact).expect("artifact serializes");
    assert!(
        encoded.contains("\"3ff0000000000000\""),
        "exact floating-point bits must use a JavaScript-safe string encoding"
    );
    let decoded: ExecutionArtifactEnvelope =
        serde_json::from_str(&encoded).expect("artifact deserializes exactly");
    assert_eq!(decoded, artifact);
    let artifacts = HashMap::from([(producer, artifact.clone())]);
    ResolvedExecutionDependencies::resolve(digest(1), vec![binding.clone()], &artifacts)
        .expect("exact artifact resolves");

    let ExecutionArtifactPayload::TransientTrajectory(trajectory) = &mut artifact.payload else {
        panic!("expected transient payload")
    };
    Arc::make_mut(trajectory)
        .waveforms
        .get_mut("V(out)")
        .unwrap()[1] = 2.0;
    let artifacts = HashMap::from([(producer, artifact)]);
    assert!(matches!(
        ResolvedExecutionDependencies::resolve(digest(1), vec![binding], &artifacts),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));
}

#[test]
fn wrong_or_stale_producer_artifacts_are_rejected() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(5).unwrap();
    let binding = PreparedDependencyBinding::transient_trajectory(producer, revision, digest(3));
    let stale = ExecutionArtifactEnvelope::from_transient_result(
        digest(9),
        producer,
        revision,
        digest(3),
        &transient(),
        &["out".to_owned()],
    )
    .unwrap()
    .unwrap();
    let artifacts = HashMap::from([(producer, stale)]);
    assert!(matches!(
        ResolvedExecutionDependencies::resolve(digest(1), vec![binding], &artifacts),
        Err(ExecutionArtifactError::StaleSnapshot { .. })
    ));
}

#[test]
fn fourier_contract_rejects_out_of_window_or_undersampled_transients() {
    let fourier = AnalysisSpec::Fourier {
        fundamental_freq: 2.0,
        num_harmonics: 4,
        output_node: "out".to_owned(),
        output_ref: "0".to_owned(),
        start_time: 0.0,
        stop_time: 1.0,
        compute_thd: true,
        normalize: false,
    };
    let transient = |stop_time, step_time| AnalysisSpec::Transient {
        stop_time,
        step_time,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    };

    validate_prepared_dependency_contract(&fourier, &transient(1.0, 0.005))
        .expect("compatible transient contract");
    let outside = validate_prepared_dependency_contract(&fourier, &transient(0.75, 0.005))
        .expect_err("producer must cover the complete Fourier window");
    assert!(outside.to_string().contains("outside"));
    let undersampled = validate_prepared_dependency_contract(&fourier, &transient(1.0, 0.05))
        .expect_err("producer sampling must cover the harmonic basis");
    assert!(undersampled.to_string().contains("too coarse"));
}

#[test]
fn large_artifact_transfer_uses_constant_size_metadata_and_exact_buffers() {
    const SAMPLE_COUNT: usize = 65_536;

    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(8).unwrap();
    let snapshot = digest(11);
    let config = digest(12);
    let binding = PreparedDependencyBinding::transient_trajectory(producer, revision, config);
    let time = (0..SAMPLE_COUNT)
        .map(|index| index as f64 * 1.0e-9)
        .collect::<Vec<_>>();
    let expected_time_sample = time[12_345].to_bits();
    let values = time
        .iter()
        .map(|time| (2.0 * std::f64::consts::PI * 1.0e6 * time).sin())
        .collect::<Vec<_>>();
    let result = SimulationResult::Transient {
        time: time.clone(),
        waveforms: HashMap::from([(
            "V(out)".to_owned(),
            WaveformData::new_time_domain("V(out)", time, values),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
    };
    let artifact = ExecutionArtifactEnvelope::from_transient_result(
        snapshot,
        producer,
        revision,
        config,
        &result,
        &["out".to_owned()],
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();

    let (borrowed_metadata, borrowed_buffers) = resolved.encode_transfer_borrowed().unwrap();
    let trajectory = resolved.transient_trajectory().unwrap();
    assert_eq!(borrowed_buffers.len(), 2);
    assert_eq!(borrowed_buffers[0].as_ptr(), trajectory.time().as_ptr());
    assert_eq!(
        borrowed_buffers[1].as_ptr(),
        trajectory.waveform("out").unwrap().as_ptr()
    );

    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    assert_eq!(metadata, borrowed_metadata);
    assert!(
        metadata.len() < 4_096,
        "sample-independent transfer metadata unexpectedly grew to {} bytes",
        metadata.len()
    );
    assert_eq!(buffers.len(), 2);
    assert_eq!(buffers[0].len(), SAMPLE_COUNT);
    assert_eq!(buffers[1].len(), SAMPLE_COUNT);
    assert_eq!(buffers[0][12_345].to_bits(), expected_time_sample);

    let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone())
        .expect("transfer buffers reconstruct exactly");
    assert_eq!(restored, resolved);

    let mut tampered = buffers;
    tampered[1][32_768] = -tampered[1][32_768];
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, tampered),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));
}

#[test]
fn pss_consumers_require_a_shooting_periodic_state_contract() {
    for consumer in [
        AnalysisSpec::Pac,
        AnalysisSpec::Pxf,
        AnalysisSpec::Pnoise,
        AnalysisSpec::Pstb,
    ] {
        validate_prepared_dependency_contract(&consumer, &pss_spec(PssMethod::Shooting))
            .expect("shooting PSS satisfies the periodic-state contract");
        let error =
            validate_prepared_dependency_contract(&consumer, &pss_spec(PssMethod::HarmonicBalance))
                .expect_err("HB PSS must not fabricate a shooting-state artifact");
        assert!(error.to_string().contains("shooting-PSS"));
    }
}

#[test]
fn prepared_phase_pnoise_requires_an_autonomous_pss_artifact() {
    let mut pnoise = crate::services::simulation_runner::PnoiseRunConfig::default();
    pnoise.noise_ref = crate::services::simulation_runner::PnoiseReference::Phase;
    let options = SpecExecutionOptions {
        pnoise: Some(pnoise),
        ..SpecExecutionOptions::default()
    };

    let driven = pss_spec(PssMethod::Shooting);
    let error = validate_prepared_dependency_contract_with_options(
        &AnalysisSpec::Pnoise,
        &options,
        &driven,
    )
    .expect_err("phase PNOISE must reject a driven periodic-state artifact");
    assert!(error.to_string().contains("autonomous"));

    let mut autonomous = pss_spec(PssMethod::Shooting);
    let AnalysisSpec::Pss {
        oscillator_mode, ..
    } = &mut autonomous
    else {
        unreachable!()
    };
    *oscillator_mode = true;
    validate_prepared_dependency_contract_with_options(
        &AnalysisSpec::Pnoise,
        &options,
        &autonomous,
    )
    .expect("phase PNOISE accepts an autonomous shooting-PSS artifact");
}

#[test]
fn periodic_state_transfer_round_trips_and_rejects_tamper_or_config_drift() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(9).unwrap();
    let snapshot = digest(21);
    let config_digest = digest(22);
    let binding = PreparedDependencyBinding::periodic_state(producer, revision, config_digest);
    let artifact = ExecutionArtifactEnvelope::from_periodic_result(
        snapshot,
        producer,
        revision,
        config_digest,
        &pss_spec(PssMethod::Shooting),
        &periodic_result(),
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();

    let stale_binding =
        PreparedDependencyBinding::periodic_state(producer, revision, config_digest);
    let stale_artifact = ExecutionArtifactEnvelope::from_periodic_result(
        digest(20),
        producer,
        revision,
        config_digest,
        &pss_spec(PssMethod::Shooting),
        &periodic_result(),
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        ResolvedExecutionDependencies::resolve(
            snapshot,
            vec![stale_binding],
            &HashMap::from([(producer, stale_artifact)]),
        ),
        Err(ExecutionArtifactError::StaleSnapshot { .. })
    ));
    resolved
        .validate_for_spec(&AnalysisSpec::Pac)
        .expect("PAC accepts the exact periodic-state binding");
    resolved
        .periodic_state()
        .unwrap()
        .validate_consumer_basis("PAC", 1.0, 8, 1.0e-6, false)
        .expect("exact producer basis matches");
    assert!(
        resolved
            .periodic_state()
            .unwrap()
            .validate_consumer_basis("PAC", 1.0, 9, 1.0e-6, false)
            .is_err()
    );

    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    assert!(metadata.len() < 16_384);
    let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone())
        .expect("periodic state reconstructs from worker transfer buffers");
    assert_eq!(restored, resolved);

    let mut tampered = buffers;
    tampered.last_mut().unwrap()[0] += 1.0;
    assert!(matches!(
        ResolvedExecutionDependencies::decode_transfer(&metadata, tampered),
        Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
    ));
}

#[test]
fn authenticated_periodic_state_transfer_preserves_identity_and_rejects_tamper() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(10).unwrap();
    let snapshot = digest(24);
    let config_digest = digest(25);
    let binding = PreparedDependencyBinding::periodic_state(producer, revision, config_digest);
    let artifact = ExecutionArtifactEnvelope::from_periodic_result(
        snapshot,
        producer,
        revision,
        config_digest,
        &pss_spec(PssMethod::Shooting),
        &authenticated_periodic_result(),
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();
    let original_identity = resolved
        .periodic_state()
        .unwrap()
        .operating_point()
        .producer_identity()
        .cloned()
        .unwrap();

    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone())
        .expect("authenticated PSS state reconstructs through artifact transport");
    assert_eq!(
        restored
            .periodic_state()
            .unwrap()
            .operating_point()
            .producer_identity(),
        Some(&original_identity)
    );
    assert_eq!(
        restored
            .periodic_state()
            .unwrap()
            .operating_point()
            .shooting_state_basis(),
        ["C:C1"]
    );

    let mut state_tamper = buffers.clone();
    state_tamper.last_mut().unwrap()[0] += 0.5;
    let error = ResolvedExecutionDependencies::decode_transfer(&metadata, state_tamper)
        .expect_err("transported shooting-state tamper must fail core authentication");
    assert!(
        matches!(error, ExecutionArtifactError::InvalidPayload(_))
            && error
                .to_string()
                .contains("numerical payload does not match"),
        "{error}"
    );

    fn tamper_retained_identity(value: &mut serde_json::Value) -> bool {
        match value {
            serde_json::Value::Object(object) => {
                if let Some(serde_json::Value::Object(identity)) =
                    object.get_mut("producer_identity")
                {
                    identity.insert(
                        "retained_state_identity".to_owned(),
                        serde_json::Value::String("0".repeat(64)),
                    );
                    return true;
                }
                object.values_mut().any(tamper_retained_identity)
            }
            serde_json::Value::Array(values) => values.iter_mut().any(tamper_retained_identity),
            _ => false,
        }
    }

    let mut identity_tamper: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    assert!(tamper_retained_identity(&mut identity_tamper));
    let identity_tamper = serde_json::to_string(&identity_tamper).unwrap();
    let error = ResolvedExecutionDependencies::decode_transfer(&identity_tamper, buffers)
        .expect_err("transported producer identity tamper must fail closed");
    assert!(
        matches!(error, ExecutionArtifactError::InvalidPayload(_))
            || matches!(error, ExecutionArtifactError::Transport(_)),
        "{error}"
    );
}

#[test]
fn periodic_state_artifact_rejects_floquet_contract_and_compatibility_tamper() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(12).unwrap();
    let envelope = ExecutionArtifactEnvelope::from_periodic_result(
        digest(51),
        producer,
        revision,
        digest(52),
        &pss_spec(PssMethod::Shooting),
        &periodic_result(),
    )
    .unwrap()
    .unwrap();
    let ExecutionArtifactPayload::PeriodicState(periodic) = envelope.payload else {
        panic!("fixture must publish a periodic-state artifact")
    };
    let periodic = Arc::unwrap_or_clone(periodic);
    periodic.validate().unwrap();

    let mut legacy = periodic.clone();
    legacy.floquet_evidence = rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown;
    legacy.floquet_verdict = rspice_core::analysis::FloquetStabilityVerdict::Indeterminate;
    legacy.floquet_authenticated = false;
    assert!(legacy.validate().is_err());

    let mut inflated = periodic.clone();
    let rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } =
        &mut inflated.floquet_evidence
    else {
        panic!("fixture must carry qualified evidence")
    };
    certificate.qualification_tolerance = 1.0;
    assert!(inflated.validate().is_err());

    let mut roots = periodic.clone();
    roots.analysis_floquet_real[0] = 0.8;
    assert!(roots.validate().is_err());

    let mut stable = periodic.clone();
    stable.analysis_is_stable = false;
    assert!(stable.validate().is_err());

    let mut orbit = periodic.clone();
    orbit.floquet_orbit_kind = rspice_core::analysis::FloquetOrbitKind::Autonomous;
    assert!(orbit.validate().is_err());

    let mut trivial = periodic;
    trivial.trivial_floquet_multiplier_index = Some(usize::MAX);
    assert!(trivial.validate().is_err());
}

#[test]
fn dependency_transfer_missing_floquet_metadata_never_becomes_qualified() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(13).unwrap();
    let snapshot = digest(61);
    let config_digest = digest(62);
    let binding = PreparedDependencyBinding::periodic_state(producer, revision, config_digest);
    let artifact = ExecutionArtifactEnvelope::from_periodic_result(
        snapshot,
        producer,
        revision,
        config_digest,
        &pss_spec(PssMethod::Shooting),
        &periodic_result(),
    )
    .unwrap()
    .unwrap();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();
    let (metadata, buffers) = resolved.encode_transfer().unwrap();
    let mut metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();

    fn strip_contract(value: &mut serde_json::Value) -> bool {
        match value {
            serde_json::Value::Object(object) => {
                if object.contains_key("floquet_evidence") {
                    object.remove("floquet_evidence");
                    object.remove("floquet_orbit_kind");
                    object.remove("trivial_floquet_multiplier_index");
                    object.remove("floquet_verdict");
                    object.remove("floquet_authenticated");
                    return true;
                }
                object.values_mut().any(strip_contract)
            }
            serde_json::Value::Array(values) => values.iter_mut().any(strip_contract),
            _ => false,
        }
    }

    assert!(strip_contract(&mut metadata));
    let metadata = serde_json::to_string(&metadata).unwrap();
    let error = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers)
        .expect_err("missing Floquet evidence must fail closed");
    assert!(
        error.to_string().contains("authenticated Floquet")
            || error.to_string().contains("complete Floquet evidence"),
        "{error}"
    );
}

#[test]
fn required_periodic_artifact_rejects_a_pss_result_without_retained_state() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(10).unwrap();
    let error = ExecutionArtifactEnvelope::from_periodic_result(
        digest(31),
        producer,
        revision,
        digest(32),
        &pss_spec(PssMethod::Shooting),
        &transient(),
    )
    .expect_err("a PSS prerequisite cannot publish without numerical state");
    assert!(error.to_string().contains("did not retain"));
}

#[test]
fn periodic_artifact_rejects_a_returned_state_from_the_wrong_frozen_config() {
    let producer = AnalysisInstanceId::new();
    let revision = ObjectRevision::new(11).unwrap();
    let mut frozen_spec = pss_spec(PssMethod::Shooting);
    let AnalysisSpec::Pss {
        points_per_period, ..
    } = &mut frozen_spec
    else {
        unreachable!("test helper always returns PSS")
    };
    *points_per_period += 1;

    let error = ExecutionArtifactEnvelope::from_periodic_result(
        digest(41),
        producer,
        revision,
        digest(42),
        &frozen_spec,
        &periodic_result(),
    )
    .expect_err("a worker result from another PSS configuration must fail closed");

    assert!(matches!(error, ExecutionArtifactError::ContractMismatch(_)));
    assert!(error.to_string().contains("frozen producer specification"));
}
