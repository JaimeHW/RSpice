//! Noise and phase-noise results keep their axis, pairing and carrier evidence
//! through conversion into a retained analysis result.

use super::*;

#[test]
fn descending_data_axis_is_retained_monotonically_with_every_series_paired() {
    let sim_result = crate::simulation::SimulationResult::Noise {
        frequencies: vec![10.0, 1.0, 10.0],
        output_noise: vec![100.0, 10.0, 101.0],
        input_noise: Some(vec![200.0, 20.0, 201.0]),
        contributors: HashMap::from([("R1:thermal".to_owned(), vec![300.0, 30.0, 301.0])]),
        summary: None,
        measurements: Vec::new(),
    };

    let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
        sim_result,
        AnalysisType::Noise,
        "NOISE DATA",
    );

    for waveform in &result.waveforms {
        assert_eq!(waveform.x.as_ref(), &[1.0, 10.0, 10.0]);
    }
    let waveform = |name: &str| {
        result
            .waveforms
            .iter()
            .find(|waveform| waveform.name == name)
            .unwrap_or_else(|| panic!("missing retained waveform {name}"))
    };
    assert_eq!(waveform("onoise").y.as_ref(), &[10.0, 100.0, 101.0]);
    assert_eq!(waveform("inoise").y.as_ref(), &[20.0, 200.0, 201.0]);
    assert_eq!(
        waveform("noise(R1:thermal)").y.as_ref(),
        &[30.0, 300.0, 301.0]
    );
}

#[test]
fn descending_axis_with_misaligned_worker_series_fails_closed_without_panicking() {
    let sim_result = crate::simulation::SimulationResult::Noise {
        frequencies: vec![10.0, 1.0],
        output_noise: vec![100.0],
        input_noise: Some(vec![200.0]),
        contributors: HashMap::from([("R1:thermal".to_owned(), vec![300.0])]),
        summary: None,
        measurements: Vec::new(),
    };

    let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
        sim_result,
        AnalysisType::Noise,
        "NOISE DATA",
    );

    assert!(result.waveforms.is_empty());
}

fn periodic_noise_result(
    reference: crate::services::simulation_runner::PnoiseReference,
) -> AnalysisResult {
    let mut controller = SimulationController::new();
    let mut config = crate::services::simulation_runner::PnoiseRunConfig::default();
    config.noise_ref = reference;
    config.pss_fundamental_freq = 2.4e9;
    controller.current_spec_options = Some(SpecExecutionOptions {
        pnoise: Some(config),
        ..SpecExecutionOptions::default()
    });
    // A driven carrier, where the drive sets the period and the authored
    // and converged fundamentals are the same bits. Dispatch is what sets
    // this in the product; these tests stand in for it.
    controller.current_periodic_carrier_hz = Some(2.4e9);
    let sim_result = crate::simulation::SimulationResult::Noise {
        frequencies: vec![1.0e3, 1.0e6],
        output_noise: vec![-90.0, -130.0],
        input_noise: None,
        contributors: HashMap::new(),
        summary: None,
        measurements: Vec::new(),
    };
    let mut result = controller.convert_to_analysis_result_with_metadata_owned(
        sim_result,
        AnalysisType::Pnoise,
        "PNOISE",
    );
    controller.retain_periodic_noise_result_metadata(&mut result);
    result
}

#[test]
fn phase_reference_retains_exact_dbc_per_hz_quantity_and_carrier() {
    let result = periodic_noise_result(crate::services::simulation_runner::PnoiseReference::Phase);
    assert!(result.validate_retained_evidence().is_ok());
    assert_eq!(result.waveforms[0].name, "phase_noise");
    assert_eq!(
        result.family_metadata,
        Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        })
    );
}

#[test]
fn output_reference_remains_psd_and_is_never_relabelled_as_phase_noise() {
    let result = periodic_noise_result(crate::services::simulation_runner::PnoiseReference::Output);
    assert!(result.validate_retained_evidence().is_ok());
    assert_eq!(result.waveforms[0].name, "onoise");
    assert_eq!(
        result.family_metadata,
        Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
            carrier_frequency_hz: Some(2.4e9),
        })
    );
}

#[test]
fn sp_noise_retains_temperature_complex_signs_and_extreme_finite_magnitudes() {
    let controller = SimulationController::new();
    let mut waveform = crate::simulation::WaveformData::new_complex(
        "CY(1,2)",
        vec![1e6, 2e6],
        vec![-1e-200, -1e200],
        vec![1e-200, 1e200],
    );
    waveform.y_unit = "A²/Hz".to_owned();
    let simulation = crate::simulation::SimulationResult::Ac {
        convergence: None,
        frequencies: vec![1e6, 2e6],
        waveforms: HashMap::from([("CY(1,2)".to_owned(), waveform)]),
        measurements: Vec::new(),
        reference_impedances_ohm: Some(vec![50.0, 50.0]),
        noise_reference_temperature_kelvin: Some(450.0),
    };
    let result = controller.convert_to_analysis_result_with_metadata_owned(
        simulation,
        AnalysisType::SParameter,
        "SP",
    );
    assert!(result.validate_retained_evidence().is_ok());
    let waveform = result
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "|CY(1,2)|")
        .unwrap();
    assert_eq!(waveform.unit.as_deref(), Some("A²/Hz"));
    assert!(
        waveform
            .y
            .iter()
            .all(|value| value.is_finite() && *value > 0.0)
    );
    let complex = waveform.complex.as_ref().unwrap();
    assert_eq!(complex.real.as_ref(), &[-1e-200, -1e200]);
    assert_eq!(complex.imag.as_ref(), &[1e-200, 1e200]);
    let mut restored = result.clone();
    restored.family_metadata =
        serde_json::from_str(&serde_json::to_string(&result.family_metadata).unwrap()).unwrap();
    assert_eq!(restored.result_data_digest(), result.result_data_digest());
    let mut changed = restored;
    let Some(AnalysisResultFamilyMetadata::SParameter {
        noise_reference_temperature_kelvin,
        ..
    }) = &mut changed.family_metadata
    else {
        panic!("missing noise authority")
    };
    *noise_reference_temperature_kelvin = Some(451.0);
    assert_ne!(changed.result_data_digest(), result.result_data_digest());
    *match &mut changed.family_metadata {
        Some(AnalysisResultFamilyMetadata::SParameter {
            noise_reference_temperature_kelvin,
            ..
        }) => noise_reference_temperature_kelvin,
        _ => unreachable!(),
    } = Some(f64::INFINITY);
    assert!(changed.validate_retained_evidence().is_err());
}

#[test]
fn sparameter_result_retains_solved_references_despite_a_stale_cached_deck() {
    let mut controller = SimulationController::new();
    controller.cached_netlist = Some(
        "* stale deck\nP1 IN 0 PORT=1 Z0=50 AC 1\nR1 IN OUT 50\nP2 OUT 0 PORT=2 Z0=50\n.end\n"
            .to_owned(),
    );
    controller.current_spec = Some(AnalysisSpec::SParameter {
        do_noise: false,
        start_freq: 1.0e6,
        stop_freq: 1.0e9,
        points_per_unit: 10,
        sweep: crate::simulation::multi_run::FrequencySweep::Decade,
        z0: 50.0,
        ports: Vec::new(),
    });
    for (analysis_type, references) in [
        (AnalysisType::SParameter, vec![75.0]),
        (AnalysisType::SParameter, vec![75.0, 100.0]),
        (AnalysisType::Psp, vec![75.0]),
        (AnalysisType::Hbsp, vec![75.0, 100.0]),
    ] {
        let simulation = crate::simulation::SimulationResult::Ac {
            convergence: None,
            noise_reference_temperature_kelvin: None,
            frequencies: vec![1e6],
            waveforms: HashMap::new(),
            measurements: Vec::new(),
            reference_impedances_ohm: Some(references.clone()),
        };
        let result = controller.convert_to_analysis_result_with_metadata_owned(
            simulation,
            analysis_type,
            "network",
        );
        assert_eq!(
            result.family_metadata,
            Some(AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: references,
            })
        );
        assert!(result.validate_retained_evidence().is_ok());
    }
}

/// The workspace's one self-starting oscillator, solved in autonomous
/// shooting mode. `services/simulation_runner/pss.rs` solves the same deck
/// and pins that the solver moves the period off the authored guess.
const NEGATIVE_RESISTANCE_OSCILLATOR: &str = "* negative-resistance lc oscillator\n\
         l1 osc 0 1u\n\
         c1 osc 0 1u\n\
         b1 osc 0 i=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)\n\
         i1 0 osc pulse(0 1 10u 10n 10n 1u 1)\n\
         .end\n";

/// An autonomous carrier's period is the shooting solver's unknown, so the
/// frequency a periodic-noise spectrum is stated against is a property of
/// the carrier and not of the card that asked for it. The Studio published
/// the authored guess: on this fixture it announced a 1.5873e5 Hz carrier
/// for a circuit that oscillated at 1.5912e5 Hz, 392 Hz away, and every
/// dBc/Hz level is stated relative to that number.
///
/// This is the same substitution `3d6cf193e` corrected on the refusal side
/// of `.PAC`, where comparing the result against the guess refused every
/// oscillator. Here it was not refused, only mislabelled.
#[test]
fn an_oscillator_publishes_the_carrier_it_converged_at_not_the_authored_guess() {
    use rspice_core::NoAbort;

    const PERIOD_GUESS: f64 = 6.3e-6;
    const TOLERANCE: f64 = 1.0e-6;
    let authored = 1.0 / PERIOD_GUESS;

    let netlist = rspice_core::Netlist::parse(NEGATIVE_RESISTANCE_OSCILLATOR)
        .expect("the oscillator deck parses");
    let mut engine_config = rspice_core::resolve_simulation_config(
        &rspice_core::engine::SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    );
    engine_config.tolerance = TOLERANCE;
    let carrier = rspice_core::engine::Engine::try_new_with_resolved_config(engine_config)
        .expect("the periodic engine resolves")
        .run_pss_operating_point_with_abort(
            &netlist,
            rspice_core::analysis::PssConfig::autonomous()
                .with_period_guess(PERIOD_GUESS)
                .with_harmonics(9)
                .with_tolerance(TOLERANCE)
                .with_max_iterations(100)
                .with_tstab_periods(30)
                .with_points_per_period(256)
                .with_oscillator_node("osc"),
            &NoAbort,
        )
        .expect("the autonomous carrier converges");
    let converged = carrier.analysis().result.frequency;
    assert_ne!(
        converged.to_bits(),
        authored.to_bits(),
        "the fixture only means anything while the solver moves the period"
    );

    let mut controller = SimulationController::new();
    let mut config = crate::services::simulation_runner::PnoiseRunConfig::default();
    config.noise_ref = crate::services::simulation_runner::PnoiseReference::Phase;
    // What the Studio authors for an autonomous producer, and what
    // `PeriodicStateArtifact::validate_consumer_basis` matches bit for bit:
    // the reciprocal of the period guess, never the converged frequency.
    config.pss_fundamental_freq = authored;
    controller.current_spec_options = Some(SpecExecutionOptions {
        pnoise: Some(config),
        ..SpecExecutionOptions::default()
    });
    // What dispatch captures from the resolved periodic dependency.
    controller.current_periodic_carrier_hz = Some(converged);

    let sim_result = crate::simulation::SimulationResult::Noise {
        frequencies: vec![1.0e3, 1.0e5],
        output_noise: vec![-90.0, -130.0],
        input_noise: None,
        contributors: HashMap::new(),
        summary: None,
        measurements: Vec::new(),
    };
    let mut result = controller.convert_to_analysis_result_with_metadata_owned(
        sim_result,
        AnalysisType::Pnoise,
        "PNOISE",
    );
    controller.retain_periodic_noise_result_metadata(&mut result);

    assert_eq!(
        result.family_metadata,
        Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(converged),
        }),
        "the published carrier is the one the circuit oscillated at"
    );
    assert!(result.validate_retained_evidence().is_ok());
}

/// With no carrier captured, a phase-noise result is refused, not guessed.
///
/// Every dBc/Hz level in the spectrum is stated relative to the carrier,
/// so a result that cannot say which carrier it was measured against is
/// not a weaker result -- it is an unreadable one. Publishing the authored
/// fundamental in its place is what made this look survivable: the field
/// was always populated, and always plausible, and for an oscillator
/// always wrong.
///
/// Dispatch cannot actually reach this state -- `validate_for_spec`
/// refuses a `.PNOISE` task that carries no periodic-state artifact before
/// the runner is handed anything -- and this pins what happens if that
/// ever stops being true.
#[test]
fn phase_noise_with_no_captured_carrier_is_refused_rather_than_labelled_with_a_guess() {
    let mut controller = SimulationController::new();
    let mut config = crate::services::simulation_runner::PnoiseRunConfig::default();
    config.noise_ref = crate::services::simulation_runner::PnoiseReference::Phase;
    config.pss_fundamental_freq = 2.4e9;
    controller.current_spec_options = Some(SpecExecutionOptions {
        pnoise: Some(config),
        ..SpecExecutionOptions::default()
    });
    // No dispatch has captured a carrier for this task.
    assert!(controller.current_periodic_carrier_hz.is_none());

    let sim_result = crate::simulation::SimulationResult::Noise {
        frequencies: vec![1.0e3, 1.0e6],
        output_noise: vec![-90.0, -130.0],
        input_noise: None,
        contributors: HashMap::new(),
        summary: None,
        measurements: Vec::new(),
    };
    let mut result = controller.convert_to_analysis_result_with_metadata_owned(
        sim_result,
        AnalysisType::Pnoise,
        "PNOISE",
    );
    controller.retain_periodic_noise_result_metadata(&mut result);

    assert_eq!(
        result.family_metadata,
        Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: None,
        }),
        "the authored fundamental must not stand in for a carrier nothing measured"
    );
    assert_eq!(
        result.validate_retained_evidence(),
        Err("phase-noise evidence is missing its retained carrier frequency".to_owned()),
    );
}
