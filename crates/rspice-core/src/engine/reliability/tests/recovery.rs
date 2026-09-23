use super::*;

#[test]
fn reliability_recovery_keeps_mission_order_aged_models_and_authenticated_transport() {
    let netlist = Netlist::parse(
        &DECK
            .replace("VG gate 0 0", "VG gate 0 {GATE}")
            .replace(".param SUP=1", ".param SUP=1 GATE=0"),
    )
    .unwrap();
    let mut request = request();
    request.study.model_pack.models[0].law = serde_json::from_value(serde_json::json!({
        "kind":"tabulated_two_state", "table":{
            "gate_source_v":[-4.0,-1.0,0.0,4.0], "drain_source_v":[-4.0,4.0],
            "temperature_k":[200.0,500.0], "interpolation":"linear",
            "traps":[{"id":"interface", "initial_occupancy":0.0,
                "capture_rates_per_s":[2.0,2.0,0.0,0.0, 2.0,2.0,0.0,0.0, 2.0,2.0,0.0,0.0, 2.0,2.0,0.0,0.0],
                "emission_rates_per_s":[0.0,0.0,3.0,3.0, 0.0,0.0,3.0,3.0, 0.0,0.0,3.0,3.0, 0.0,0.0,3.0,3.0],
                "parameters":[{"parameter":"VTO", "update":"additive", "shift_per_occupancy":-0.1}]}]
        }
    })).unwrap();
    for phase in &mut request.study.mission {
        phase.parameters.insert("SUP".into(), 1.0);
        phase.parameters.insert("GATE".into(), 0.0);
        phase.duration_s = 1.0;
    }
    request.study.mission[1]
        .parameters
        .insert("GATE".into(), 1.0);
    request.target_years = vec![1.0, 2.0, 4.5, 2e9]
        .into_iter()
        .map(|s| s / SECONDS_PER_AGING_YEAR)
        .collect();
    request.min_stress_voltage = 0.5;
    let engine = Engine::new(Default::default());
    let result = engine
        .run_reliability_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    let b = (1.0 - (-2.0f64).exp()) * (-3.0f64).exp();
    let expected = [
        1.0 - (-2.0f64).exp(),
        b,
        1.0 - (1.0 - b * (1.0 + (-5.0f64).exp())) * (-1.0f64).exp(),
        b / (1.0 - (-5.0f64).exp()),
    ];
    for (checkpoint, p) in result.stress.checkpoints.iter().zip(expected) {
        let c = &checkpoint.devices[0].contributions[0];
        close(c.trap_occupancies[0].occupancy, p, 2e-12);
        close(c.parameters[0].shift, -0.1 * p, 2e-13);
    }
    let current = |point: &ReliabilityAgedPoint| {
        *point
            .operating_point
            .device_observables
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("M1:ID"))
            .unwrap()
            .1
    };
    assert!(current(&result.aged[1]).abs() > current(&result.aged[0]).abs());
    let limits = crate::resource::ResourceLimits::default();
    result
        .validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    let (meta, buffer) = result
        .clone()
        .into_transfer_parts_with_abort(&limits, &NoAbort)
        .unwrap();
    let unpacked =
        ReliabilityRunResult::from_transfer_parts_with_abort(meta, buffer, &limits, &NoAbort)
            .unwrap();
    assert_eq!(unpacked, result);
    let mut corrupt = result.clone();
    corrupt.stress.checkpoints[0].devices[0].contributions[0].trap_occupancies[0].occupancy = 0.0;
    assert!(
        corrupt
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .is_err()
    );
    // Same total exposures, opposite order: a physically different final state.
    request.study.mission.reverse();
    let reversed = engine
        .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    close(
        reversed.checkpoints[1].devices[0].contributions[0].trap_occupancies[0].occupancy,
        1.0 - (-2.0f64).exp(),
        1e-12,
    );
}

#[test]
fn reliability_recovery_preserves_nested_waveform_repetition_and_partial_windows() {
    let mut request = request();
    request.study.mission.truncate(1);
    request.study.mission[0].duration_s = 2.5;
    request.study.model_pack.models[0].law = serde_json::from_value(serde_json::json!({
        "kind":"tabulated_two_state", "table":{
            "gate_source_v":[-4.0,0.0,4.0], "drain_source_v":[-4.0,4.0],
            "temperature_k":[200.0,500.0], "interpolation":"linear",
            "traps":[{"id":"oxide", "initial_occupancy":0.0,
                "capture_rates_per_s":[4.0,0.0,0.0,4.0,0.0,0.0,4.0,0.0,0.0,4.0,0.0,0.0],
                "emission_rates_per_s":[0.0,4.0,4.0,0.0,4.0,4.0,0.0,4.0,4.0,0.0,4.0,4.0],
                "parameters":[{"parameter":"VTO", "update":"additive", "shift_per_occupancy":-0.1}]}]
        }
    })).unwrap();
    request.target_years = vec![5.75 / SECONDS_PER_AGING_YEAR];
    let a = AgingStress {
        gate_source_v: -4.0,
        drain_source_v: 0.0,
        temperature_k: 300.0,
        current_density_a_per_m2: 0.0,
    };
    let b = AgingStress {
        gate_source_v: 0.0,
        ..a
    };
    let data = vec![ReliabilityPhaseStress {
        phase_index: 0,
        fresh_operating_point: None,
        time_s: vec![0.0, 0.5, 1.0],
        devices: vec![ReliabilityDeviceStress {
            device: "M1".into(),
            compact_model: "PM".into(),
            samples: vec![a, b, a],
        }],
    }];
    let checkpoints = mission::integrate(&request, &data, Default::default(), 0, &NoAbort).unwrap();
    // Each waveform ramp uses endpoint half steps; each phase restarts the
    // waveform after 2.5 s. Explicit independent two-state recurrence oracle.
    let mut p = 0.0;
    let apply = |p: &mut f64, c: f64, e: f64, dt: f64| {
        let equilibrium = c / (c + e);
        *p = equilibrium + (*p - equilibrium) * (-(c + e) * dt).exp();
    };
    for duration in [2.5f64, 2.5, 0.75] {
        let mut left = duration;
        while left > 0.0 {
            for (c0, c1) in [(4.0, 0.0), (0.0, 4.0)] {
                if left == 0.0 {
                    break;
                }
                let dt = left.min(0.5);
                let end = c0 + (c1 - c0) * (dt / 0.5);
                apply(&mut p, c0, 4.0 - c0, dt * 0.5);
                apply(&mut p, end, 4.0 - end, dt * 0.5);
                left -= dt;
            }
        }
    }
    close(
        checkpoints[0].devices[0].contributions[0].trap_occupancies[0].occupancy,
        p,
        2e-13,
    );
}
