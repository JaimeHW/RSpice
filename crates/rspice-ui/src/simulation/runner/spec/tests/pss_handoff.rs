//! Bound OP environment must survive shooting and every PSS consumer.
use super::*;
use crate::simulation::dialog::{OpConfig, OpTemperatureMode};
use crate::simulation::multi_run::{FrequencySweep, PssMethod};
use svc_runner::*;

#[test]
fn pss_op_handoff_preserves_environment_for_all_consumers() {
    let basis = "PSS OP handoff\nVIN drive 0 SIN(0 .1 1k) AC 1\nVDD in drive DC .2\nRS in out {1000+10*(TEMP-37)} TC1=.01\nRL out 0 1k\nC1 out 0 100n\nLPROBE lp 0 .01\nRPROBE lp 0 1\nP1 p1 0 PORT=1 Z0=50\nP2 p2 0 PORT=2 Z0=50\nRSP p1 p2 {1000+10*(TEMP-37)}\n.options TEMP=12 TNOM=27 GMIN=1e-10\n.end\n";
    let op_deck = splice_before_terminal_end_card(basis, ".options GMIN=1e-7");
    let pss_deck = splice_before_terminal_end_card(basis, ".options GMIN=0 TEMP=12");
    let consumer_deck = splice_before_terminal_end_card(basis, ".options GMIN=0 TEMP=77");
    let mut op = OpConfig {
        temperature_celsius: 37.0,
        temperature_mode: OpTemperatureMode::Explicit,
        ..Default::default()
    };
    op.run_point.supply_voltage = Some(2.0);
    op.run_point.nominal_supply_voltage = Some(1.0);
    op.run_point.supply_source_names = vec!["VDD".into()];
    let dependencies = super::qpss::op_dependencies(basis, &op_deck, &pss_deck, op);
    let environment = dependencies
        .dc_operating_point_seed()
        .unwrap()
        .environment();
    let producer = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq: 1000.0,
        tone_sources: ["VIN", "P1", "P2"].into_iter().map(str::to_owned).collect(),
        tstab_periods: 0,
        points_per_period: 128,
        tolerance: 1e-6,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 3,
        integration_method: None,
        tstab: 0.0,
        max_iterations: 30,
        abstol: 1e-10,
        damping: 1.0,
        max_period_change: 0.05,
        verbose: false,
    };
    let run = |spec, options, deck: &str, deps: &ResolvedExecutionDependencies| {
        run_spec_request(
            &EngineBridge::new(),
            spec,
            options,
            deck,
            None,
            deps,
            &rspice_core::NoAbort,
        )
    };
    let result = run(
        producer.clone(),
        Default::default(),
        &pss_deck,
        &dependencies,
    )
    .unwrap();
    let last = |name| result.study_measurement(name).unwrap().value.unwrap();
    assert!((last("last:V(in)") - last("last:V(drive)") - 0.4).abs() < 1e-10);
    let binding = PreparedDependencyBinding::periodic_state(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        digest(73),
    );
    let artifact = ExecutionArtifactEnvelope::from_periodic_result_with_environment(
        digest(74),
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer,
        &result,
        Some(environment),
    )
    .unwrap()
    .unwrap();
    let mut dependencies = ResolvedExecutionDependencies::resolve(
        digest(74),
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    dependencies.bind_source(&consumer_deck, crate::state::content_digest(basis));
    let (metadata, buffers) = dependencies.encode_transfer().unwrap();
    let changed = metadata.replace(
        "\"temperature_celsius\":37.0",
        "\"temperature_celsius\":47.0",
    );
    assert_ne!(changed, metadata);
    assert!(ResolvedExecutionDependencies::decode_transfer(&changed, buffers.clone()).is_err());
    let dependencies = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
    let carrier = PeriodicCarrier::Pss;
    let pac = SpecExecutionOptions {
        pac: Some(PacRunConfig {
            pss_fundamental_freq: 1000.0,
            pss_num_harmonics: 3,
            pss_tolerance: 1e-6,
            start_freq: 100.0,
            stop_freq: 200.0,
            points_per_unit: 3,
            sweep: PacFrequencySweep::Linear,
            sideband_min: -1,
            sideband_max: 0,
            input_source: "VIN".into(),
            output_node: "out".into(),
            pac_magnitude: 2.5,
            carrier,
            ..Default::default()
        }),
        ..Default::default()
    };
    let pxf = SpecExecutionOptions {
        pxf: Some(PxfRunConfig {
            pss_fundamental_freq: 1000.0,
            pss_num_harmonics: 3,
            pss_tolerance: 1e-6,
            start_freq: 100.0,
            stop_freq: 200.0,
            points_per_unit: 3,
            sweep: PxfFrequencySweep::Linear,
            input_source: "VIN".into(),
            output_node: "out".into(),
            max_sideband: 1,
            input_sideband: 0,
            output_sideband: 0,
            carrier,
            ..Default::default()
        }),
        ..Default::default()
    };
    let noise = SpecExecutionOptions {
        pnoise: Some(PnoiseRunConfig {
            pss_fundamental_freq: 1000.0,
            pss_num_harmonics: 3,
            pss_tolerance: 1e-6,
            start_freq: 100.0,
            stop_freq: 200.0,
            points_per_unit: 3,
            sweep: PnoiseFrequencySweep::Linear,
            input_source: "VIN".into(),
            output_node: "out".into(),
            max_sideband: 1,
            noise_ref: PnoiseReference::Input,
            integrated_noise: true,
            carrier,
            ..Default::default()
        }),
        ..Default::default()
    };
    let stability = SpecExecutionOptions {
        pstb: Some(PstbRunConfig {
            pss_fundamental_freq: 1000.0,
            pss_num_harmonics: 3,
            pss_tolerance: 1e-6,
            probe_instance: "LPROBE".into(),
            max_harmonics: 2,
            num_multipliers: 2,
            stability_threshold: 1.01,
            detect_subharmonics: false,
            ..Default::default()
        }),
        ..Default::default()
    };
    let network = AnalysisSpec::Psp {
        start_freq: 100.0,
        stop_freq: 200.0,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
        ports: ["p1", "p2"]
            .into_iter()
            .map(|node| SpPort {
                node_pos: node.into(),
                node_neg: "0".into(),
                z0: Some(50.0),
            })
            .collect(),
        max_sideband: 1,
        reltol: 1.0e-3,
        abstol: 1.0e-12,
        mixed_mode: false,
        noise_parameters: false,
        noise_reference: None,
    };
    let h = rspice_core::Complex64::new(1.0, 0.0)
        / rspice_core::Complex64::new(2.1, std::f64::consts::TAU * 100.0 * 1100.0 * 1e-7);
    let parallel = 1100.0 / 2.1;
    let psd = 4.0 * 1.380649e-23 * 310.15 * parallel
        / (1.0 + (std::f64::consts::TAU * 100.0 * parallel * 1e-7).powi(2));
    assert!(
        run(
            AnalysisSpec::Pac,
            pac.clone(),
            &consumer_deck.replace("RL out 0 1k", "RL out 0 2k"),
            &dependencies
        )
        .is_err()
    );
    for (spec, options, observation, expected) in [
        (
            AnalysisSpec::Pac,
            pac,
            "bin:0:real:V(out)[sb=+0]",
            2.5 * h.re,
        ),
        (
            AnalysisSpec::Pxf,
            pxf,
            "bin:0:real:H(sb0->sb0, V(out))",
            h.re,
        ),
        (AnalysisSpec::Pnoise, noise, "bin:0:real:output_noise", psd),
        (
            AnalysisSpec::Pstb,
            stability,
            "scalar:pstb.max_multiplier_magnitude",
            (-0.1_f64).exp(),
        ),
        (
            network,
            Default::default(),
            "bin:0:real:S11",
            1000.0 / 1100.0,
        ),
    ] {
        let result = run(spec, options, &consumer_deck, &dependencies).unwrap();
        let actual = result
            .study_measurement(observation)
            .unwrap()
            .value
            .unwrap();
        assert!(
            (actual - expected).abs() < expected.abs() * 2e-3,
            "{observation}: {actual} != {expected}"
        );
    }
}
