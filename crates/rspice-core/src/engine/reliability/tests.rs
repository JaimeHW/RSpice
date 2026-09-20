//! Real-circuit tests with deliberately synthetic, analytically soluble fits.

use super::*;
use crate::abort_signal::{ImmediateAbort, NoAbort};

const DECK: &str = "PMOS stress\n.param SUP=1\nVS source 0 {SUP}\nVG gate 0 0\nVD drain 0 0.2\nM1 drain gate source source PM W=10u L=1u DTEMP=10\n.model PM PMOS (LEVEL=1 VTO=-0.2 KP=100u)\n.end\n";

fn request() -> ReliabilityRunRequest {
    let study: ReliabilityStudy = serde_json::from_value(serde_json::json!({
        "model_pack": {
            "schema_version": 1, "id": "synthetic-contract", "process": "test-only",
            "qualification": "user_characterized", "source": "analytical fixture", "license": "test-only",
            "characterization": "synthetic normalization, not a process fit",
            "models": [{"id": "nbti", "mechanism": "nbti", "applicability": "synthetic PMOS",
                "validity": {"gate_source_v": {"min": -4.0, "max": 4.0}, "drain_source_v": {"min": -4.0, "max": 4.0},
                    "temperature_k": {"min": 200.0, "max": 500.0}, "current_density_a_per_m2": {"min": 0.0, "max": 1e16}, "max_equivalent_seconds": 1e10},
                "law": {"kind": "equivalent_time_power", "reference_time_s": 1.0, "reference_gate_magnitude_v": 1.0,
                    "reference_drain_magnitude_v": 1.0, "reference_temperature_k": 300.0, "gate_polarity": "negative",
                    "clock_gate_exponent": 2.0, "clock_drain_exponent": 0.0, "clock_activation_energy_ev": 0.0,
                    "time_exponent": 1.0, "parameters": [{"parameter": "VTO", "update": "additive", "scale_at_reference_time": -0.01}]}
            }]
        },
        "bindings": [{"device": "M1", "compact_model": "PM", "aging_models": ["nbti"], "conductor_area_m2": 1e-14}],
        "mission": [{"name": "low", "duration_s": 1.0, "temperature_c": 27.0, "parameters": {"SUP": 1.0}},
                    {"name": "high", "duration_s": 1.0, "temperature_c": 80.0, "parameters": {"SUP": 2.0}}],
        "repeat_mission": true, "transient_stress": null
    })).unwrap();
    ReliabilityRunRequest {
        study,
        target_years: vec![1.0 / SECONDS_PER_AGING_YEAR, 2.5 / SECONDS_PER_AGING_YEAR],
        enable_hci: false,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.0,
    }
}

fn close(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}

#[test]
fn reliability_dc_mission_uses_signed_stress_parameter_overrides_and_partial_cycles() {
    let netlist = Netlist::parse(DECK).unwrap();
    let engine = Engine::new(Default::default());
    let request = request();
    let result = engine
        .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    assert_eq!(result.request, request);
    let low = result.phases[0].devices[0].samples[0];
    let high = result.phases[1].devices[0].samples[0];
    close(low.gate_source_v, -1.0, 1e-10);
    close(high.gate_source_v, -2.0, 1e-10);
    close(low.drain_source_v, -0.8, 1e-10);
    close(high.drain_source_v, -1.8, 1e-10);
    close(low.temperature_k, 310.15, 1e-10);
    close(high.temperature_k, 363.15, 1e-10);
    assert!(low.current_density_a_per_m2 > 0.0);
    close(
        result.checkpoints[0].devices[0].contributions[0].parameters[0].shift,
        -0.01,
        1e-10,
    );
    close(
        result.checkpoints[1].devices[0].contributions[0].parameters[0].shift,
        -0.055,
        1e-10,
    );
    assert_eq!(
        netlist.params.get("SUP"),
        Some(1.0),
        "source request is immutable"
    );
}

#[test]
fn reliability_transient_window_keeps_warmup_bounds_currents_and_nonlinear_stress_rate() {
    let netlist =
        Netlist::parse(&DECK.replace("VG gate 0 0", "VG gate 0 PWL(0 0 1u -1 2u 0)")).unwrap();
    let mut request = request();
    request.study.mission.truncate(1);
    request.study.mission[0].duration_s = 1.5e-6;
    request.study.transient_stress = Some(ReliabilityTransientWindow {
        step_s: 1e-7,
        stop_s: 1.75e-6,
        start_s: 0.25e-6,
        max_step_s: Some(5e-8),
        use_initial_conditions: false,
    });
    request.target_years = vec![3e-6 / SECONDS_PER_AGING_YEAR];
    let result = Engine::new(Default::default())
        .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    let phase = &result.phases[0];
    assert_eq!(phase.time_s[0], 0.0);
    close(*phase.time_s.last().unwrap(), 1.5e-6, 1e-20);
    close(phase.devices[0].samples[0].gate_source_v, -1.25, 1e-9);
    close(
        phase.devices[0].samples.last().unwrap().gate_source_v,
        -1.25,
        1e-9,
    );
    assert!(
        phase.devices[0]
            .samples
            .iter()
            .all(|s| s.current_density_a_per_m2 > 0.0)
    );
    // Integral of (1+x)^2 over x=.25..1..25, repeated twice.
    let exact = 4e-6 * (8.0 - 1.25_f64.powi(3)) / 3.0;
    close(
        result.checkpoints[0].devices[0].contributions[0].equivalent_seconds,
        exact,
        4e-9,
    );
}

#[test]
fn reliability_stress_refuses_stale_bindings_missing_parameters_limits_and_abort() {
    let netlist = Netlist::parse(DECK).unwrap();
    let engine = Engine::new(Default::default());
    let mut request = request();
    assert!(matches!(
        engine.run_reliability_stress_with_abort(&netlist, &request, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
    request.study.bindings[0].compact_model = "WRONG".into();
    assert!(
        engine
            .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("uses compact model")
    );
    request.study.bindings[0].compact_model = "PM".into();
    request.study.mission[0]
        .parameters
        .insert("MISSING".into(), 1.0);
    assert!(
        engine
            .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("not defined")
    );
    let mut config = super::super::SimulationConfig::default();
    config.resource_limits.max_batch_runs = 1;
    assert!(matches!(
        Engine::new(config).run_reliability_stress_with_abort(&netlist, &request, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
}

#[test]
fn reliability_mission_threshold_preserves_observed_voltages_and_elapsed_time() {
    let netlist = Netlist::parse(DECK).unwrap();
    let mut request = request();
    request.min_stress_voltage = 1.5;
    let result = Engine::new(Default::default())
        .run_reliability_stress_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    close(
        result.phases[0].devices[0].samples[0].gate_source_v,
        -1.0,
        1e-10,
    );
    let contribution = &result.checkpoints[1].devices[0].contributions[0];
    close(contribution.equivalent_seconds, 4.0, 1e-9);
    close(contribution.elapsed_seconds, 2.5, 1e-12);
}

#[test]
fn reliability_aged_circuit_isolates_shared_models_and_restarts_from_fresh_values() {
    let deck = DECK.replace(".model PM PMOS (LEVEL=1 VTO=-0.2 KP=100u)",
        ".model PM PMOS (LEVEL=1 VTO={-0.2*SUP} KP=100u)\n.subckt cell d g s\nM1 d g s s PM W=10u L=1u DTEMP=10\n.ends\nX2 drain gate source cell");
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::new(Default::default());
    let result = engine
        .run_reliability_with_abort(&netlist, &request(), &NoAbort)
        .unwrap();
    assert_eq!(result.aged.len(), 4);
    for point in &result.aged {
        let fresh = result.stress.phases[point.phase_index]
            .fresh_operating_point
            .as_ref()
            .unwrap();
        let observable = |op: &ReliabilityOperatingPoint, name: &str| {
            *op.device_observables
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .unwrap()
                .1
        };
        close(
            observable(fresh, "X2.M1:ID"),
            observable(&point.operating_point, "X2.M1:ID"),
            1e-12,
        );
        assert!(
            observable(&point.operating_point, "M1:ID").abs() < observable(fresh, "M1:ID").abs()
        );
        let parameter = &point.parameters[0];
        close(
            parameter.fresh_value,
            if point.phase_index == 0 { -0.2 } else { -0.4 },
            1e-12,
        );
        close(
            parameter.aged_value,
            parameter.fresh_value + parameter.shift,
            1e-12,
        );
    }
    let encoded = serde_json::to_string(&result).unwrap();
    assert_eq!(
        serde_json::from_str::<ReliabilityRunResult>(&encoded).unwrap(),
        result
    );
    assert!(netlist.ast_overlay.instance_models.is_empty());
    // The aged override survives source replay, while the hierarchy remains intact.
    let checkpoint = &result.stress.checkpoints[0];
    let (aged, _) = engine
        .reliability_aged_netlist(&netlist, &request().study, checkpoint, &NoAbort)
        .unwrap();
    let (replayed, count) = Engine::create_perturbed_netlist_multi_with_limits_and_abort(
        &aged,
        &[("SUP".into(), 1.0)],
        engine.config.resource_limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(count, 1);
    assert_eq!(replayed.subcircuits.len(), 1);
    let actual = ReliabilityOperatingPoint::from_result(
        &engine.run_dc_op_with_abort(&replayed, &NoAbort).unwrap(),
    );
    assert_eq!(actual, result.aged[0].operating_point);
}

#[test]
fn reliability_aged_hierarchy_selects_bins_and_applies_relative_mobility_fit() {
    let deck = DECK.replace("M1 drain gate source source PM W=10u L=1u DTEMP=10", "X1 drain gate source cell")
        .replace(".model PM PMOS (LEVEL=1 VTO=-0.2 KP=100u)",
            ".model PM.1 PMOS (LEVEL=1 LMIN=0.5u LMAX=2u VTO=-0.2 KP=100u)\n.subckt cell d g s\nM1 d g s s PM W=10u L=1u DTEMP=10\n.ends");
    let netlist = Netlist::parse(&deck).unwrap();
    let mut request = request();
    request.study.bindings[0].device = "X1.M1".into();
    if let AgingLaw::EquivalentTimePower { parameters, .. } =
        &mut request.study.model_pack.models[0].law
    {
        parameters[0].parameter = "KP".into();
        parameters[0].update = AgingParameterUpdate::Relative;
        parameters[0].scale_at_reference_time = -0.1;
    }
    let result = Engine::new(Default::default())
        .run_reliability_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    let point = &result.aged[0];
    assert_eq!(point.parameters[0].compact_model, "PM.1");
    close(point.parameters[0].fresh_value, 100e-6, 1e-16);
    close(point.parameters[0].aged_value, 90e-6, 1e-16);
}

#[test]
fn reliability_aged_run_refuses_ignored_parameters_invalid_values_and_batch_overflow() {
    let netlist = Netlist::parse(DECK).unwrap();
    let mut request = request();
    if let AgingLaw::EquivalentTimePower { parameters, .. } =
        &mut request.study.model_pack.models[0].law
    {
        parameters[0].parameter = "IGNORED".into();
    }
    let engine = Engine::new(Default::default());
    assert!(
        engine
            .run_reliability_with_abort(&netlist, &request, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("no qualified parameter mapping")
    );
    if let AgingLaw::EquivalentTimePower { parameters, .. } =
        &mut request.study.model_pack.models[0].law
    {
        parameters[0].parameter = "KP".into();
    }
    assert!(
        engine
            .run_reliability_with_abort(&netlist, &request, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("outside its native domain")
    );
    let mut config = super::super::SimulationConfig::default();
    config.resource_limits.max_batch_runs = 5;
    assert!(matches!(
        Engine::new(config).run_reliability_with_abort(&netlist, &request, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        engine.run_reliability_with_abort(&netlist, &request, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn reliability_conductor_transient_retains_black_lifetime_without_inventing_resistance() {
    let netlist = Netlist::parse(
        "conductor\n.param SUP=1\nVS n 0 {SUP}\nR1 n 0 1k RM\n.model RM R (TC1=0)\n.end\n",
    )
    .unwrap();
    let mut request = request();
    request.enable_nbti = false;
    request.enable_em = true;
    request.study.bindings[0].device = "R1".into();
    request.study.bindings[0].compact_model = "RM".into();
    let model = &mut request.study.model_pack.models[0];
    model.mechanism = AgingMechanism::Electromigration;
    model.law = AgingLaw::BlackElectromigration {
        reference_lifetime_s: 10.0,
        reference_current_density_a_per_m2: 1e11,
        reference_temperature_k: 300.0,
        current_exponent: 2.0,
        activation_energy_ev: 0.0,
    };
    request.study.transient_stress = Some(ReliabilityTransientWindow {
        step_s: 1e-7,
        stop_s: 1e-6,
        start_s: 0.0,
        max_step_s: Some(2e-7),
        use_initial_conditions: false,
    });
    let result = Engine::new(Default::default())
        .run_reliability_with_abort(&netlist, &request, &NoAbort)
        .unwrap();
    let first = &result.stress.checkpoints[0].devices[0].contributions[0];
    close(first.electromigration_lifetime_fraction.unwrap(), 0.1, 1e-8);
    assert!(first.parameters.is_empty());
    for point in &result.aged {
        assert!(point.parameters.is_empty());
        assert_eq!(
            &point.operating_point,
            result.stress.phases[point.phase_index]
                .fresh_operating_point
                .as_ref()
                .unwrap()
        );
    }
}

#[test]
fn reliability_retained_results_and_worker_buffer_refuse_contradictions() {
    let result = Engine::new(Default::default())
        .run_reliability_with_abort(&Netlist::parse(DECK).unwrap(), &request(), &NoAbort)
        .unwrap();
    let limits = crate::ResourceLimits::default();
    result
        .validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    let identity = result
        .retained_identity_with_abort(&limits, &NoAbort)
        .unwrap();
    let (metadata, values) = result
        .clone()
        .into_transfer_parts_with_abort(&limits, &NoAbort)
        .unwrap();
    let metadata = serde_json::from_str(&serde_json::to_string(&metadata).unwrap()).unwrap();
    let restored =
        ReliabilityRunResult::from_transfer_parts_with_abort(metadata, values, &limits, &NoAbort)
            .unwrap();
    assert_eq!(restored, result);
    assert_eq!(
        identity,
        restored
            .retained_identity_with_abort(&limits, &NoAbort)
            .unwrap()
    );
    let mut altered = result.clone();
    altered.stress.checkpoints[0].devices[0].contributions[0].parameters[0].shift *= 2.0;
    assert!(
        altered
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .is_err()
    );
    altered = result.clone();
    altered.aged[0].parameters[0].aged_value -= 0.1;
    assert!(
        altered
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .is_err()
    );
    altered = result.clone();
    altered.stress.phases[0].time_s[1] = f64::NAN;
    assert!(
        altered
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .is_err()
    );
    let (metadata, mut values) = result
        .clone()
        .into_transfer_parts_with_abort(&limits, &NoAbort)
        .unwrap();
    assert!(
        metadata
            .validate_transfer_layout_with_abort(values.len() - 1, &limits, &NoAbort)
            .is_err()
    );
    values[0] = f64::INFINITY;
    assert!(
        ReliabilityRunResult::from_transfer_parts_with_abort(metadata, values, &limits, &NoAbort)
            .is_err()
    );
    assert!(matches!(
        result.validate_retained_payload_with_abort(&limits, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
    let mut small = limits;
    small.max_result_values = 1;
    assert!(matches!(
        result.validate_retained_payload_with_abort(&small, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
}
