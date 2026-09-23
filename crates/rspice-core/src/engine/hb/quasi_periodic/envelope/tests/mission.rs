use super::*;

fn prepared() -> PreparedSpectralEnvelope {
    let netlist = Netlist::parse("Envelope mission\nVmod input 0 PWL(0 0 .73m 0 .73m 1 2m 1 2m 2)\nVrf rf 0 SIN(0 .4 1Meg 0 0 90)\nRin input out 1k\nRrf rf out 1k\nCout out 0 1u\nCshunt input 0 2u\n.end\n").unwrap();
    Engine::new(SimulationConfig::default())
        .prepare_spectral_envelope_with_abort(&netlist, config(&["Vmod"]), &NoAbort)
        .unwrap()
}

fn request(prepared: &PreparedSpectralEnvelope) -> SpectralEnvelopeMissionConfig {
    let n = prepared.carrier_sources.len();
    SpectralEnvelopeMissionConfig {
        stepping: SpectralEnvelopeStepping::Fixed {
            method: SpectralEnvelopeMethod::BackwardEuler,
            step: 0.0002,
        },
        reporting_times: vec![0.0, 0.0005, 0.001, 0.002],
        retain_accepted_steps: true,
        maximum_steps: 1000,
        events: SpectralEnvelopeEventConfig {
            solver: prepared.config.solver.clone(),
            charge_tolerances: vec![1e-13; n],
            rate_tolerances: vec![1e-10; n],
        },
    }
}

fn check_events(result: &NetlistEnvelopeMission) {
    let state = &result.final_state;
    let source = state.node_names().len()
        + state
            .branch_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case("Vmod"))
            .unwrap();
    assert_eq!(
        result
            .transitions
            .iter()
            .map(|e| e.time)
            .collect::<Vec<_>>(),
        vec![0.00073, 0.002]
    );
    for event in &result.transitions {
        close(
            event.current_impulses[source][state.grid().dc_index()],
            Complex64::new(-2e-6, 0.0),
            1e-12,
        );
        let samples = result
            .samples
            .iter()
            .filter(|s| s.time == event.time)
            .collect::<Vec<_>>();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].source_side, EnvelopeSourceSide::LeftLimit);
        assert_eq!(samples[1].source_side, EnvelopeSourceSide::RightLimit);
        assert_eq!(samples[1].order, 0);
    }
    assert_eq!(state.time(), 0.002);
    assert_eq!(state.source_side(), EnvelopeSourceSide::RightLimit);
}

#[test]
fn netlist_envelope_mission_fixed_preserves_rc_history_events_and_reports() {
    let mut prepared = prepared();
    let request = request(&prepared);
    let limits = prepared.limits;
    let result = prepared
        .run_mission_with_abort(&request, None, &NoAbort)
        .unwrap();
    assert_eq!(prepared.limits.max_result_values, limits.max_result_values);
    check_events(&result);
    assert_eq!(result.rejected_steps, 0);
    assert_eq!(result.accepted_steps, result.spectral_solves);
    assert!(
        result.accepted_steps < 20,
        "must skip thousands of RF cycles"
    );
    for report in &request.reporting_times {
        assert!(result.samples.iter().any(|sample| sample.time == *report));
    }
    let out = result
        .final_state
        .node_names()
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .unwrap();
    let dc = result.final_state.grid().dc_index();
    let rf = result.final_state.grid().index_of(&[1]).unwrap();
    let expected_carrier =
        Complex64::new(0.0002, 0.0) / Complex64::new(0.002, std::f64::consts::TAU);
    let mut expected = 0.0;
    let mut previous_time = 0.0;
    for sample in &result.samples {
        let step = sample.time - previous_time;
        if step > 0.0 {
            let source = if previous_time >= 0.00073 { 0.001 } else { 0.0 };
            expected = (source + 1e-6 / step * expected) / (0.002 + 1e-6 / step);
        }
        close(
            sample.spectra[out][dc],
            Complex64::new(expected, 0.0),
            1e-10,
        );
        close(sample.spectra[out][rf], expected_carrier, 1e-10);
        previous_time = sample.time;
    }
    let outgoing = result
        .samples
        .iter()
        .position(|s| s.time == 0.00073 && s.source_side == EnvelopeSourceSide::RightLimit)
        .unwrap();
    assert_eq!(result.samples[outgoing + 1].order, 1);
}

#[test]
fn netlist_envelope_mission_adaptive_retains_only_reports_and_recovers_after_failure() {
    let mut prepared = prepared();
    let mut request = request(&prepared);
    request.retain_accepted_steps = false;
    request.stepping = SpectralEnvelopeStepping::Adaptive {
        initial_step: 0.0005,
        control: SpectralEnvelopeControl {
            method: SpectralEnvelopeMethod::Bdf2,
            minimum_step: 1e-9,
            maximum_step: 0.001,
            relative_tolerance: 1e-4,
            absolute_tolerances: vec![1e-9; prepared.carrier_sources.len()],
            max_rejections: 24,
        },
    };
    let original_limits = prepared.limits;
    request.maximum_steps = 1;
    assert!(matches!(
        prepared.run_mission_with_abort(&request, None, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert_eq!(
        prepared.limits.max_result_values,
        original_limits.max_result_values
    );
    request.maximum_steps = 1000;
    assert!(matches!(
        prepared.run_mission_with_abort(&request, None, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
    let result = prepared
        .run_mission_with_abort(&request, None, &NoAbort)
        .unwrap();
    assert_eq!(
        prepared.limits.max_result_values,
        original_limits.max_result_values
    );
    check_events(&result);
    assert_eq!(
        result.samples.iter().map(|s| s.time).collect::<Vec<_>>(),
        vec![0.0, 0.0005, 0.00073, 0.00073, 0.001, 0.002, 0.002]
    );
    assert!(result.rejected_steps > 0);
    assert!(
        result.accepted_steps < 2000,
        "slow steps must be fewer than RF periods: {}",
        result.accepted_steps
    );
    assert!(result.maximum_error_ratio <= 1.0);
    let expected = 0.5 * (1.0 - (-0.00127_f64 / 0.0005).exp());
    close(
        coefficient(&result.final_state, "out", &[0]),
        Complex64::new(expected, 0.0),
        5e-4,
    );
    assert_eq!(result.final_state.time(), 0.002);
    request.reporting_times = vec![0.001, 0.0005];
    assert!(
        prepared
            .run_mission_with_abort(&request, None, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("increase strictly")
    );
    request.reporting_times = vec![0.002];
    prepared.limits.max_result_values = 1;
    assert!(matches!(
        prepared.run_mission_with_abort(&request, None, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert_eq!(prepared.limits.max_result_values, 1);
}
