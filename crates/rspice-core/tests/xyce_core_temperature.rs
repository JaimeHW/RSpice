//! Temperature transport for authored Xyce CORE winding models.

use rspice_core::config::SpiceDialect;
use rspice_core::{Complex64, Engine, Netlist, SimulationConfig};

fn engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..Default::default()
    })
}

fn deck(
    turns_factor: f64,
    level: u8,
    shared: bool,
    dc: f64,
    thermal: &str,
    options: &str,
) -> Netlist {
    let secondary = if shared {
        format!(
            "L2 s 0 {}\nL3 t 0 {}\nR2 s 0 4\nR3 t 0 9\nI2 0 s DC .02\nK1 L1 L2 L3 1 core\n",
            20.0 * turns_factor,
            30.0 * turns_factor
        )
    } else {
        "K1 L1 1 core\n".to_owned()
    };
    Netlist::parse(&format!("Core temperature\nV1 in 0 DC {dc} AC 1\nR1 in p 1\nL1 p 0 {}\n{secondary}.model core CORE level={level} c=.001 {thermal}\n{options}\n.end\n", 10.0*turns_factor)).unwrap()
}

#[test]
fn core_temperature_scales_both_the_dc_field_and_the_complete_winding_matrix() {
    for (thermal, options, factor) in [
        ("tc1=.01", ".options device temp=77", 1.5),
        ("tc1=.01 tc2=.0001", ".options device temp=77", 1.75),
        ("tc1=.01 tc2=.0001", ".options device temp=-23", 0.75),
        ("tc1=-.01 tc2=.0001", ".options device temp=77", 0.75),
        ("tc1=.01", ".options device temp=77 tnom=40", 1.37),
        ("tc1=.01 tnom=27", ".options device temp=77 tnom=40", 1.5),
        (
            "tc1={slope} tc2={slope*slope}",
            ".param slope=.01\n.options device temp=77",
            1.75,
        ),
    ] {
        for shared in [false, true] {
            for dc in [-0.1, 0.0, 0.1] {
                let heated = deck(1.0, 2, shared, dc, thermal, options);
                let reference = deck(factor, 2, shared, dc, "", options);
                let actual = engine().run_ac(&heated, &[1.0, 1e5, 1e9]).unwrap();
                let expected = engine().run_ac(&reference, &[1.0, 1e5, 1e9]).unwrap();
                for (actual, expected) in actual.iter().zip(expected) {
                    assert_eq!(actual.node_names, expected.node_names);
                    assert_eq!(actual.branch_names, expected.branch_names);
                    for (actual, expected) in actual
                        .voltages
                        .iter()
                        .chain(&actual.currents)
                        .zip(expected.voltages.iter().chain(&expected.currents))
                    {
                        assert!(
                            (*actual - *expected).norm() < 1e-10,
                            "{thermal}; {options}; shared={shared}; bias={dc}; actual={actual}, expected={expected}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn temperature_scaled_core_matches_analytic_ac_and_noise_at_zero_bias() {
    let netlist = deck(
        1.0,
        2,
        false,
        0.0,
        "tc1=.01 tnom=27",
        ".options device temp=77",
    );
    // Actual Xyce 7.10 gives this 100 kHz output for TC1=.01, TNOM=27 C,
    // TEMP=77 C. Independently L(T)=2.552444016457999e-7 * (1+.01*50)^2.
    let inductance = 2.552_444_016_457_999e-7 * 2.25;
    let points = engine().run_ac(&netlist, &[1e5]).unwrap();
    let z = Complex64::new(0.0, std::f64::consts::TAU * 1e5 * inductance);
    let expected = z / (1.0 + z);
    let index = points[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("p"))
        .unwrap();
    assert!((points[0].voltages[index] - expected).norm() < 1e-12);
    assert!(
        (expected - Complex64::new(0.115_207_007_980_681_66, 0.319_271_598_005_242)).norm() < 1e-15
    );
    let noise = engine()
        .run_noise_named_with_input_source(&netlist, "p", None, "V1", &[1e5], 350.15)
        .unwrap();
    assert!((noise[0].input_gain_squared / expected.norm_sqr() - 1.0).abs() < 1e-10);
    let psd = 4.0 * 1.3806226e-23 * 350.15 * expected.norm_sqr();
    assert!((noise[0].output_noise_density / psd - 1.0).abs() < 1e-10);
}

#[test]
fn invalid_core_temperature_domains_are_named_errors() {
    for thermal in ["tc1=-.02", "tc1=-.03", "tc1=1e308", "tnom=-300"] {
        let netlist = deck(1.0, 2, false, 0.0, thermal, ".options device temp=77");
        let result = engine().run_ac(&netlist, &[1e3]);
        let error = result
            .expect_err("invalid thermal coefficients must not be ignored")
            .to_string();
        assert!(error.to_ascii_lowercase().contains("core"), "{error}");
        assert!(
            error.to_ascii_lowercase().contains("temperature") || error.contains("TNOM"),
            "{error}"
        );
    }
}

#[test]
fn core_temperature_preserves_the_scaled_transient_and_material_histories() {
    for level in [1, 2] {
        for shared in [false, true] {
            let source = |factor: f64, thermal: &str| {
                let secondary = if shared {
                    format!(
                        "L2 s 0 {}\nL3 t 0 {}\nR2 s 0 4\nR3 t 0 9\nK1 L1 L2 L3 1 core\n",
                        20.0 * factor,
                        30.0 * factor
                    )
                } else {
                    "K1 L1 1 core\n".to_owned()
                };
                Netlist::parse(&format!("Core thermal transient\nV1 in 0 SIN(0 .001 10k)\nR1 in p 1\nL1 p 0 {}\n{secondary}.model core CORE level={level} c=.2 {thermal}\n.options device temp=77\n.save all\n.end\n", 10.0*factor)).unwrap()
            };
            let heated = source(1.0, "tc1=.01");
            let reference = source(1.5, "");
            let actual = engine().run_tran(&heated, 1e-5, 1e-7).unwrap();
            let expected = engine().run_tran(&reference, 1e-5, 1e-7).unwrap();
            assert_eq!(actual.time.last().copied(), Some(1e-5));
            assert!(actual.time.len() > 50);
            assert_eq!(actual.time, expected.time, "level={level}, shared={shared}");
            assert_eq!(actual.branch_names, expected.branch_names);
            assert_eq!(actual.branch_currents, expected.branch_currents);
            for name in ["in", "p"] {
                assert_eq!(
                    actual.try_voltage_waveform_named(name).unwrap(),
                    expected.try_voltage_waveform_named(name).unwrap()
                );
            }
            for name in ["m", "h", "b"] {
                assert_eq!(
                    actual
                        .try_device_op_waveform_named("YMIN!K1", name)
                        .unwrap(),
                    expected
                        .try_device_op_waveform_named("YMIN!K1", name)
                        .unwrap()
                );
            }
        }
    }
}

#[test]
fn configured_temperature_is_reapplied_without_mutating_authored_turns() {
    let netlist = deck(1.0, 2, true, 0.1, "tc1=.01 tnom=27", "");
    let run = |temperature| {
        Engine::new(SimulationConfig {
            temperature,
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        })
        .run_ac(&netlist, &[1e5])
        .unwrap()
    };
    let nominal = run(300.15);
    let heated = run(350.15);
    let nominal_again = run(300.15);
    assert_eq!(nominal[0].voltages, nominal_again[0].voltages);
    assert_eq!(nominal[0].currents, nominal_again[0].currents);
    let expected = Engine::new(SimulationConfig {
        temperature: 350.15,
        spice_dialect: SpiceDialect::Xyce,
        ..Default::default()
    })
    .run_ac(&deck(1.5, 2, true, 0.1, "", ""), &[1e5])
    .unwrap();
    assert_eq!(heated[0].voltages, expected[0].voltages);
    assert_eq!(heated[0].currents, expected[0].currents);
    assert_ne!(heated[0].currents, nominal[0].currents);
}
