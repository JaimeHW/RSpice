//! Physical closure for nonlinear shared-core transient simulation.

use rspice_core::engine::{
    ConvergenceConfig, Engine, SimulationConfig, SpiceDialect, TransientResult,
};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use rspice_core::numerics::integration::IntegrationMethod;

const AUTHORED_DECK: &str =
    include_str!("../../../tests/xyce/Netlists/LEAD_CURRENTS/lead_min2_trap.cir");

fn fixture() -> (Engine, Netlist) {
    fixture_from_source(AUTHORED_DECK)
}

fn fixture_from_source(source: &str) -> (Engine, Netlist) {
    // Retain the authored topology, model and solver options. SAVE ALL only
    // adds observation of the physical voltages absent from its alias oracle.
    let source = source.replace(".end", ".save all\n.end");
    let netlist = Netlist::parse_with_options(
        &source,
        NetlistParseOptions {
            expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1e-4;
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        convergence_config,
        integration_method: IntegrationMethod::Trapezoidal,
        transient_initial_timestep: Some(1e-10),
        temperature: 300.15,
        ..Default::default()
    });
    (engine, netlist)
}

fn assert_physical_closure(result: &TransientResult) {
    assert_eq!(*result.time.last().unwrap(), 1e-3);
    assert!(result.time.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(result.time.len() > 100);
    let waveform = |name| result.try_voltage_waveform_named(name).unwrap();
    let v1 = waveform("node1");
    let v2 = waveform("node2");
    let v3 = waveform("node3");
    let v4 = waveform("node4");
    let current = |name: &str| {
        let index = result
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .unwrap();
        &result.branch_currents[index]
    };
    let i2 = current("llead2");
    let i3 = current("llead3");
    for (index, &time) in result.time.iter().enumerate() {
        let input = 14.0 * (std::f64::consts::TAU * 120.0 * time).sin();
        // Perfect 10:100:100 coupling fixes both secondary voltages.
        // Their shared common mode follows from .01*V2 + .01*V3 + V4=0.
        let secondary = input * (10.0 / 102.0);
        for (actual, expected) in [
            (v1[index], input),
            (v2[index], secondary),
            (v3[index], secondary - 10.0 * input),
            (v4[index], secondary),
        ] {
            assert!(actual.is_finite());
            assert!(
                (actual - expected).abs() <= 1e-11 + 32.0 * f64::EPSILON * expected.abs(),
                "physical voltage at t={time:e}: {actual:e} versus {expected:e}"
            );
        }
        assert!((i2[index] + v2[index] / 100.0).abs() < 1e-12);
        assert!((i3[index] + v4[index]).abs() < 1e-12);
        assert!((v2[index] / 100.0 + v3[index] / 100.0 + v4[index]).abs() < 1e-12);
    }
}

#[test]
fn shared_level2_core_preserves_ideal_winding_voltages_and_load_currents() {
    let (engine, netlist) = fixture();
    let result = engine.run_tran(&netlist, 1e-3, 1e-4).unwrap();
    assert_physical_closure(&result);
}

#[test]
fn shared_level2_core_resolves_magnetization_current_through_the_knee() {
    let source = AUTHORED_DECK.replace("sin(0 14 120 0 0)", "sin(0 0.7 120 0 0)");
    let (engine, netlist) = fixture_from_source(&source);
    let result = engine.run_tran(&netlist, 1e-3, 1e-6).unwrap();
    assert_eq!(result.time.last().copied(), Some(1e-3));
    let primary = result.try_branch_current_waveform_named("VP1").unwrap();
    // Independent continuous reference: impose Faraday flux, H=B/mu0-M,
    // integrate dM/dt=P/(1+P)*V/(mu0*N*Area), then recover primary current
    // from ampere-turns and the resistor loads. RK4 steps of 100 ns and
    // 10 ns give 201.591557535 A and 201.591483636 A, respectively.
    // The legacy first-order M update at <=1 us must be within 1%; this
    // regression does not qualify its adaptive magnetic LTE contract.
    let expected_current = 201.591484;
    assert!(
        (primary.last().unwrap() - expected_current).abs() <= 0.01 * expected_current,
        "magnetization knee current: actual {:?}, reference {expected_current}",
        primary.last()
    );
    let magnetization = result.try_device_op_waveform_named("YMIN!K1", "m").unwrap();
    assert!(
        magnetization
            .iter()
            .all(|value| value.is_finite() && value.abs() < 2e6)
    );
    let secondary1 = result.try_branch_current_waveform_named("VP2").unwrap();
    let secondary2 = result.try_branch_current_waveform_named("VP4").unwrap();
    let omega = std::f64::consts::TAU * 120.0;
    let flux = |time: f64| 0.7 * 2.0 * (omega * time / 2.0).sin().powi(2) / (omega * 1e-5 * 10.0);
    let full_scale_flux = flux(1e-3);
    for (index, &time) in result.time.iter().enumerate() {
        let field =
            (10.0 * primary[index] + 100.0 * secondary1[index] + 100.0 * secondary2[index]) / 0.01;
        let actual_flux = 4e-7 * std::f64::consts::PI * (field + magnetization[index]);
        assert!((actual_flux - flux(time)).abs() <= 0.005 * full_scale_flux);
    }
}
