//! End-to-end policy for floating components at the shared DC-bias boundary.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

fn voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

#[test]
fn internal_current_source_preserves_physical_differential_solution() {
    let netlist = parse(
        "internal source on floating resistor island\n\
         vref ref 0 0\n\
         i1 a b dc 1m\n\
         r1 a b 1k\n\
         c1 a 0 1u\n\
         c2 b 0 1u\n\
         .op\n\
         .end\n",
    );

    let result = engine()
        .run_dc_op(&netlist)
        .expect("an internal current source does not drive component common mode");
    let differential = voltage(&result, "a") - voltage(&result, "b");
    assert!(
        (differential.abs() - 1.0).abs() <= 1.0e-9,
        "1 mA through 1 kOhm must establish a 1 V differential, got {differential:.17e}"
    );
}

#[test]
fn zero_dc_and_ac_only_current_sources_preserve_warning_only_behavior() {
    for source in ["i1 0 out dc 0", "i1 0 out ac 1"] {
        let netlist = parse(&format!(
            "inactive DC current source\n\
             vref ref 0 0\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let result = engine()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{source} must not be a fatal DC drive: {error}"));
        assert_eq!(voltage(&result, "out"), 0.0);
    }
}

#[test]
fn installed_zero_waveforms_and_zero_gain_sources_are_warning_only() {
    for source in [
        "i1 0 out pat(1 0 0 1n 1n 1n b0)",
        "g1 0 out ref 0 0",
        "f1 0 out vref 0",
        "b1 0 out i=0",
    ] {
        let netlist = parse(&format!(
            "zero installed current source\n\
             vref ref 0 0\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let result = engine()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{source} must have zero net DC drive: {error}"));
        assert_eq!(voltage(&result, "out"), 0.0);
    }
}

#[test]
fn loaded_zero_pwl_current_uses_the_installed_snapshot() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-zero-current-{}-{unique}.csv",
        std::process::id()
    ));
    std::fs::write(&path, "0,0\n1,1\n").expect("write PWL fixture");
    let path_text = path.to_string_lossy().replace('\\', "/");
    let netlist = parse(&format!(
        "zero loaded PWL current\n\
         vref ref 0 0\n\
         i1 0 out pwl file=\"{path_text}\"\n\
         c1 out 0 1u\n\
         .op\n\
         .end\n"
    ));
    let result = engine().run_dc_op(&netlist);
    std::fs::remove_file(&path).expect("remove PWL fixture");
    let result = result.expect("the loaded t=0 sample is the authoritative DC current");
    assert_eq!(voltage(&result, "out"), 0.0);
}

#[test]
fn nonzero_controlled_current_is_rejected_from_its_accepted_bias() {
    for source in ["g1 0 out ref 0 1m", "b1 0 out i=1m"] {
        let netlist = parse(&format!(
            "controlled current drives a floating component\n\
             vref ref 0 1\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let error = engine()
            .run_dc_op(&netlist)
            .expect_err("a nonzero controlled source must not publish a shunt-dependent bias");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "unexpected {source} error: {error}"
        );
    }
}

#[test]
fn dc_sweep_reclassifies_the_installed_current_at_every_point() {
    let netlist = parse(
        "swept source becomes a floating DC drive\n\
         vref ref 0 0\n\
         i1 0 out dc 0\n\
         c1 out 0 1u\n\
         .dc i1 0 1m 1m\n\
         .end\n",
    );
    let error = engine()
        .run_dc_sweep(&netlist, "i1", 0.0, 1.0e-3, 1.0e-3)
        .expect_err("the nonzero sweep point must not reuse zero-source classification");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected sweep error: {error}"
    );
}

#[test]
fn every_ac_bias_uses_the_shared_floating_component_policy() {
    let netlist = parse(
        "AC analysis still requires a physical DC bias\n\
         i1 0 out dc 1m ac 1\n\
         c1 out 0 1u\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let error = engine()
        .run_ac(&netlist, &[1.0e3])
        .expect_err("AC must refuse the same current-driven floating DC bias as .OP");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected AC startup error: {error}"
    );
}

#[test]
fn rshunt_supplies_the_shared_ac_bias_path() {
    let netlist = parse(
        "RSHUNT physically defines the AC operating point\n\
         i1 0 out dc 1m ac 1\n\
         c1 out 0 1u\n\
         .options rshunt=1g\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let results = engine()
        .run_ac(&netlist, &[1.0e3])
        .expect("RSHUNT must permit every analysis that consumes the DC bias");
    let point = &results[0];
    let out = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("OUT node is present");
    // At 1 kHz the capacitor is in parallel, so this deck primarily checks
    // shared bias admission. A separate pure-RSHUNT point below pins the AC
    // transimpedance exactly.
    assert!(point.voltages[out].norm().is_finite());
}

#[test]
fn rshunt_is_a_physical_ac_transimpedance_without_a_synthetic_device() {
    let netlist = parse(
        "RSHUNT-only AC transimpedance\n\
         i1 0 out dc 0 ac 1\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let engine = Engine::new(SimulationConfig {
        rshunt: Some(1.0e3),
        ..Default::default()
    });
    let circuit = engine.build_circuit(&netlist).expect("circuit builds");
    assert_eq!(
        circuit.global_shunt_conductance().to_bits(),
        1.0e-3_f64.to_bits()
    );
    assert!(circuit.has_global_shunt());
    assert!(circuit.no_dc_path_nodes().is_empty());
    assert!(circuit.fatal_no_dc_path_nodes().is_empty());
    assert!(
        circuit
            .branch_names_sorted()
            .iter()
            .all(|name| !name.to_ascii_uppercase().contains("RSHUNT")),
        "RSHUNT must not create a synthetic output-visible resistor"
    );

    let results = engine.run_ac(&netlist, &[1.0e3]).expect("AC solves");
    let point = &results[0];
    let out = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("OUT node is present");
    let transimpedance = point.voltages[out].norm();
    assert!(
        (transimpedance - 1.0e3).abs() <= 2.0e-9,
        "expected 1 kOhm AC transimpedance, got {transimpedance:.17e}"
    );
}

#[test]
fn rshunt_is_stamped_during_uic_transient_without_an_operating_point() {
    let netlist = parse(
        "RSHUNT in UIC transient\n\
         i1 0 out 1m\n\
         c1 out 0 1u\n\
         .options rshunt=1k\n\
         .tran 10u 100u uic\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&netlist, 100.0e-6, 10.0e-6)
        .expect("UIC transient with RSHUNT solves");
    let out = result
        .try_voltage_waveform_named("out")
        .expect("OUT waveform is present");
    assert_eq!(out[0], 0.0);
    assert!(out.iter().all(|value| value.is_finite()));
    assert!(out.last().is_some_and(|value| *value > 0.0 && *value < 1.0));
}

#[test]
fn invalid_programmatic_rshunt_values_fail_configuration_validation() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::from_bits(1)] {
        let config = SimulationConfig {
            rshunt: Some(invalid),
            ..Default::default()
        };
        let error = match Engine::try_new(config) {
            Ok(_) => panic!("invalid RSHUNT must fail closed"),
            Err(error) => error,
        };
        assert!(
            error.to_string().contains("rshunt"),
            "unexpected error for {invalid:?}: {error}"
        );
    }
}

#[test]
fn non_uic_transient_uses_dc_policy_while_uic_deliberately_bypasses_it() {
    let ordinary = parse(
        "ordinary transient asks for an operating point\n\
         i1 0 out dc 1m\n\
         c1 out 0 1u\n\
         .tran 1u 2u\n\
         .end\n",
    );
    let error = engine()
        .run_tran(&ordinary, 2.0e-6, 1.0e-6)
        .expect_err("non-UIC transient must enforce the shared DC policy");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected transient startup error: {error}"
    );

    let uic = parse(
        "UIC transient intentionally skips the operating point\n\
         i1 0 out dc 1m\n\
         c1 out 0 1u\n\
         .tran 1u 2u uic\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&uic, 2.0e-6, 1.0e-6)
        .expect("UIC must remain exempt because it requests no operating point");
    assert!(
        result
            .try_voltage_waveform_named("out")
            .is_some_and(|waveform| waveform.iter().all(|value| value.is_finite()))
    );

    let inactive_at_t0 = parse(
        "DC source value must not override the t0 waveform contract\n\
         i1 0 out dc 1m pulse(0 0 0 1n 1n 1u 2u)\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&inactive_at_t0, 2.0e-9, 1.0e-9)
        .expect("a zero t=0 waveform must not be rejected from its separate DC value");
    assert_eq!(
        result
            .try_voltage_waveform_named("out")
            .expect("OUT waveform exists")[0],
        0.0
    );
}

#[test]
fn transient_bias_audit_uses_the_t0_waveform_instead_of_the_dc_source_value() {
    let floating = parse(
        "t0 waveform drives a floating transient operating point\n\
         i1 0 out dc 0 pulse(1m 1m 0 1n 1n 1u 2u)\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let error = engine()
        .run_tran(&floating, 2.0e-9, 1.0e-9)
        .expect_err("the nonzero t=0 waveform must not publish a GMIN-defined bias");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected transient startup error: {error}"
    );

    let grounded = parse(
        "t0 waveform has a physical transient bias path\n\
         i1 0 out dc 0 pulse(1m 1m 0 1n 1n 1u 2u)\n\
         r1 out 0 1k\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&grounded, 2.0e-9, 1.0e-9)
        .expect("a physical resistor must admit the same t=0 waveform");
    assert!(
        result
            .try_voltage_waveform_named("out")
            .is_some_and(|waveform| waveform.iter().all(|value| value.is_finite()))
    );
}

#[test]
fn capacitor_initial_condition_dc_constraint_is_dialect_exact() {
    let netlist = parse(
        "capacitor IC operating-point dialect contract\n\
         i1 0 out 1m\n\
         c1 out 0 1u ic=0\n\
         .op\n\
         .end\n",
    );

    let xyce = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let xyce_circuit = xyce.build_circuit(&netlist).expect("Xyce circuit builds");
    assert!(xyce_circuit.no_dc_path_nodes().is_empty());
    assert!(xyce_circuit.fatal_no_dc_path_nodes().is_empty());
    let xyce_result = xyce
        .run_dc_op(&netlist)
        .expect("Xyce IC branch supplies the exact DC voltage constraint");
    assert_eq!(voltage(&xyce_result, "out"), 0.0);

    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        assert_eq!(circuit.no_dc_path_nodes(), ["OUT"]);
        assert_eq!(circuit.fatal_no_dc_path_nodes(), ["OUT"]);
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("transient-only capacitor IC must not invent a DC constraint");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "unexpected {dialect:?} error: {error}"
        );
    }
}
