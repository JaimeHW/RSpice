//! Dependent small-signal analyses must use the physical device-stamped operator.
//!
//! These public-API regressions deliberately use impedances above the former
//! blanket AC diagonal's equivalent resistance. A solver may diagnose a truly
//! singular operator, but it must not make these nonsingular circuits easier by
//! changing their transfer, noise, sensitivity, or pole locations.

use rspice_core::constants::K_BOLTZMANN;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const TEMPERATURE_K: f64 = 300.15;
const HIGH_RESISTANCE_OHM: f64 = 1.0e18;
// Primitive resistor-noise collection currently models finite resistors below
// 1e12 ohm. This value remains high enough that a 1e-15 S blanket shunt would
// introduce a readily detectable ~2e-4 relative error in the output PSD.
const HIGH_NOISE_RESISTANCE_OHM: f64 = 1.0e11;

fn physical_engine() -> Engine {
    let mut config = SimulationConfig::default();
    // These tests qualify the authored circuit, not an explicitly requested
    // simulator shunt. Junction gmin is irrelevant to the linear fixtures but
    // is also cleared so the test contract is unambiguous.
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    Engine::try_new(config).expect("physical test configuration is valid")
}

fn node_id(engine: &Engine, netlist: &Netlist, name: &str) -> usize {
    engine
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

fn assert_relative(actual: f64, expected: f64, relative_tolerance: f64, quantity: &str) {
    assert!(actual.is_finite(), "{quantity} is non-finite: {actual}");
    assert_ne!(expected, 0.0, "relative oracle must be nonzero");
    let relative_error = ((actual - expected) / expected).abs();
    assert!(
        relative_error <= relative_tolerance,
        "{quantity}: actual={actual:.17e}, expected={expected:.17e}, relative error={relative_error:.3e}"
    );
}

#[test]
fn parameter_ac_magnitude_sensitivity_reports_the_null_cusp() {
    let netlist =
        Netlist::parse("AC output null\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain}\n.end\n")
            .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    // The API's nominal override is zero even though the authored value is one.
    // Vout=gain, so |Vout| has opposing one-sided derivatives at this point.
    let error = engine
        .run_sensitivity_ac(&netlist, output, "gain", 0.0, &[1.0], None)
        .expect_err("a cusp cannot be reported as zero sensitivity")
        .to_string();
    assert!(error.contains("gain"), "{error}");
    assert!(error.contains("nondifferentiable-magnitude"), "{error}");
}

#[test]
fn parameter_ac_magnitude_sensitivity_respects_the_run_budget() {
    let netlist = Netlist::parse(
        "AC study budget\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain}\n.end\n",
    )
    .unwrap();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_batch_runs = 2;
    let engine = Engine::try_new(config).unwrap();
    let output = node_id(&engine, &netlist, "out");
    assert!(matches!(
        engine.run_sensitivity_ac(&netlist, output, "gain", 1.0, &[1.0], None),
        Err(rspice_core::SimulationError::ResourceLimit(_))
    ));
}

#[test]
fn parameter_ac_magnitude_sensitivity_projects_at_the_nominal_point() {
    let netlist = Netlist::parse(
        "AC near an output null\n.param gain=1\nV1 in 0 AC 1 60\nE1 out 0 in 0 {gain}\n.end\n",
    )
    .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    for nominal in [-1e-4_f64, 1e-4] {
        // The complex transfer is exactly linear even when the stencil crosses
        // its null. Differencing magnitudes gives +/-0.1 instead of +/-1.
        let derivative = engine
            .run_sensitivity_ac(&netlist, output, "gain", nominal, &[1.0], Some(1e-3))
            .unwrap()[0];
        assert_relative(derivative, nominal.signum(), 2e-12, "magnitude derivative");
    }

    let stationary = Netlist::parse(
        "Smooth zero magnitude\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain*gain}\n.end\n",
    )
    .unwrap();
    let derivative = engine
        .run_sensitivity_ac(&stationary, output, "gain", 0.0, &[1.0], None)
        .unwrap();
    assert_eq!(derivative, [0.0], "|gain squared| has a zero derivative");
}

#[test]
fn ideal_voltage_short_has_exactly_zero_output_noise() {
    let netlist = Netlist::parse(
        "* an ideal voltage source shorts every parallel noise source\n\
         VCLAMP out 0 DC 0\n\
         RNOISE out 0 1k\n\
         .end\n",
    )
    .expect("noise deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_noise_ports(&netlist, output, None, &[1.0e3], TEMPERATURE_K)
        .expect("noise analysis solves");

    assert_eq!(result.len(), 1);
    // V(out)=0 is an exact ideal-source branch equation. Therefore every
    // parallel current-noise transfer is exactly zero and S_v,out = 0 V²/Hz.
    assert_eq!(result[0].output_noise_density, 0.0);
}

#[test]
fn high_impedance_resistor_noise_is_four_k_t_r() {
    let netlist = Netlist::parse(
        "* open-circuit resistor thermal-noise voltage\n\
         RNOISE out 0 1e11\n\
         .end\n",
    )
    .expect("noise deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_noise_ports(&netlist, output, None, &[1.0e3], TEMPERATURE_K)
        .expect("noise analysis solves");
    let expected_psd = 4.0 * K_BOLTZMANN * TEMPERATURE_K * HIGH_NOISE_RESISTANCE_OHM;

    assert_eq!(result.len(), 1);
    // A resistor has Norton PSD 4kT/R A²/Hz and transfer R V/A, hence
    // S_v,out = (4kT/R) R² = 4kTR V²/Hz.
    assert_relative(
        result[0].output_noise_density,
        expected_psd,
        1.0e-12,
        "high-Z resistor output-noise PSD",
    );
}

#[test]
fn adjoint_sensitivity_preserves_extreme_resistors_and_small_outputs() {
    let engine = physical_engine();
    for (resistance, current) in [
        (1e-200, 1e200),
        (1e-20, 1e20),
        (1.0, 1e-200),
        (1e200, 1e-200),
    ] {
        let netlist = Netlist::parse(&format!(
            "Scaled resistor sensitivity\nI1 0 out {current:e}\nR1 out 0 {resistance:e}\n.end\n"
        ))
        .unwrap();
        let result = engine
            .run_sensitivity_linearized(&netlist, 1, None)
            .unwrap();
        assert_relative(result.output_value, current * resistance, 2e-12, "V=IR");
        let resistor = result
            .get("R1")
            .expect("finite nonzero conductance must remain eligible");
        assert_relative(resistor.absolute, current, 2e-12, "dV/dR=I");
        assert_relative(
            resistor.normalized.value().unwrap(),
            1.0,
            2e-12,
            "normalized resistance derivative",
        );
        let source = result.get("I1").unwrap();
        assert_relative(source.absolute, resistance, 2e-12, "dV/dI=R");
        assert_relative(
            source.normalized.value().unwrap(),
            1.0,
            2e-12,
            "normalized source derivative",
        );
    }
}

#[test]
fn high_impedance_adjoint_sensitivity_matches_closed_form() {
    let netlist = Netlist::parse(
        "* one-node high-Z adjoint oracle\n\
         IBIAS 0 out DC 1e-18\n\
         RLOAD out 0 1e18\n\
         .end\n",
    )
    .expect("sensitivity deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_sensitivity_linearized(&netlist, output, None)
        .expect("linearized adjoint sensitivity solves");

    // V = I R = 1 V, so dV/dR = I and dV/dI = R. The corresponding
    // dimensionless normalized sensitivities are both exactly one.
    assert_relative(result.output_value, 1.0, 1.0e-12, "DC output voltage");
    let resistor = result.get("RLOAD").expect("resistor sensitivity");
    assert_relative(resistor.absolute, 1.0e-18, 1.0e-10, "dV(out)/dRLOAD");
    assert_relative(
        resistor.normalized.value().unwrap(),
        1.0,
        1.0e-10,
        "normalized RLOAD sensitivity",
    );
    let source = result.get("IBIAS").expect("source sensitivity");
    assert_relative(
        source.absolute,
        HIGH_RESISTANCE_OHM,
        1.0e-10,
        "dV(out)/dIBIAS",
    );
    assert_relative(
        source.normalized.value().unwrap(),
        1.0,
        1.0e-10,
        "normalized IBIAS sensitivity",
    );
}

#[test]
fn high_impedance_transfer_function_is_unperturbed() {
    let netlist = Netlist::parse(
        "* one-node high-Z transimpedance\n\
         IIN 0 out DC 0\n\
         RLOAD out 0 1e18\n\
         .end\n",
    )
    .expect("transfer-function deck parses");
    let engine = physical_engine();

    let result = engine
        .run_transfer_function(&netlist, "out", None, false, "iin")
        .expect("transfer-function analysis solves");

    // A unit input current develops Vout = Iin R. The same lone resistor is
    // both the input impedance seen by IIN and the output impedance seen by
    // the independent output test current.
    assert_relative(
        result.gain,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "transimpedance gain",
    );
    assert_relative(
        result.input_impedance,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "input impedance",
    );
    assert_relative(
        result.output_impedance,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "output impedance",
    );
}

#[test]
fn high_impedance_rc_pole_remains_at_minus_one_over_rc() {
    let netlist = Netlist::parse(
        "* one-state high-Z RC pole\n\
         IIN 0 out DC 0 AC 1\n\
         RLOAD out 0 1e18\n\
         CLOAD out 0 1\n\
         .end\n",
    )
    .expect("pole-zero deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_pz(&netlist, output, output)
        .expect("pole-zero analysis solves");

    assert_eq!(result.poles.len(), 1, "poles={:?}", result.poles);
    let pole = result.poles[0];
    // For Y(s)=1/R+sC, the natural root is s=-1/(RC)=-1e-18 rad/s.
    assert_relative(pole.re, -1.0e-18, 1.0e-10, "RC pole real part");
    assert_eq!(pole.im, 0.0, "the first-order passive pole must be real");
}
