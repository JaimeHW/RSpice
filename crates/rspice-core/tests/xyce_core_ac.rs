//! Independent AC and noise oracles for Xyce LEVEL=2 magnetic cores.
use rspice_core::config::SpiceDialect;
use rspice_core::{Complex64, Engine, Netlist, SimulationConfig};

fn engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..Default::default()
    })
}

fn biased_inductance(happ: f64, gap_ratio: f64) -> f64 {
    // Independently converge the authored provisional DC update M=P*Happ.
    // Fixed-point iteration is contractive for these selected biases; the
    // production implementation instead uses a bracketed residual solve.
    let susceptibility = |m: f64| {
        let he = happ + (5e-5 - gap_ratio) * m;
        let root = he.hypot(0.1);
        let man = 1e6 * he / (1000.0 + root);
        let man_prime = 1e6 * (1000.0 + 0.01 / root) / (1000.0 + root).powi(2);
        let departure = (man - m).hypot(31.25);
        let irreversible = departure / (2.0 * (500.0 - 5e-5 * departure));
        (0.001 * man_prime + 0.999 * irreversible)
            / (1.0 + (gap_ratio - 5e-5) * 0.001 * man_prime + gap_ratio * 0.999 * irreversible)
    };
    let mut m = 0.0;
    for _ in 0..128 {
        m = susceptibility(m) * happ;
    }
    let p = susceptibility(m);
    assert!((m - p * happ).abs() <= 8.0 * f64::EPSILON * m.abs().max(1.0));
    4e-7 * std::f64::consts::PI * 1e-5 / 0.01 * 100.0 * (1.0 + (1.0 - gap_ratio) * p)
}

#[test]
fn biased_core_tangent_is_independent_of_unrelated_dc_iterations_and_frequency_order() {
    for extra in [
        "",
        "VX x 0 1\nRX x y 1k\nDX y 0 dxmodel\n.model dxmodel D is=1e-14\n",
    ] {
        let netlist = Netlist::parse(&format!("Converged DC core\nV1 in 0 DC .1 AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 c=.001\n{extra}.end\n")).unwrap();
        let points = engine().run_ac(&netlist, &[1e5, 1.0, 1e9, 1e5]).unwrap();
        for point in &points {
            let z = Complex64::new(
                0.0,
                std::f64::consts::TAU * point.frequency * biased_inductance(100.0, 0.0),
            );
            let p = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("p"))
                .unwrap();
            assert!((point.voltages[p] - z / (1.0 + z)).norm() < 1e-10);
        }
        assert_eq!(points[0].voltages, points[3].voltages);
        assert_eq!(points[0].currents, points[3].currents);
    }
}

#[test]
fn level2_core_ac_resolves_the_material_tangent_at_dc_bias() {
    for dc in [0.0, 0.1, -0.1] {
        for gap in [0.0, 0.01] {
            for scaling in [
                "",
                "factorms=1 mvarscaling=.02 meqnscaling=3 rvarscaling=2 reqnscaling=.5",
            ] {
                let netlist = Netlist::parse(&format!("Core AC tangent\nV1 in 0 DC {dc} AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 c=.001 gap={gap} {scaling}\n.end\n")).unwrap();
                let points = engine().run_ac(&netlist, &[0.0, 1.0, 1e5, 1e9]).unwrap();
                let l = biased_inductance(1000.0 * dc, gap);
                for point in points {
                    let z = Complex64::new(0.0, std::f64::consts::TAU * point.frequency * l);
                    let expected = z / (1.0 + z);
                    let p = point
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case("p"))
                        .unwrap();
                    assert!(
                        (point.voltages[p] - expected).norm() < 1e-10,
                        "bias={dc}, gap={gap}, scaling={scaling}, frequency={}: actual={}, expected={expected}",
                        point.frequency,
                        point.voltages[p]
                    );
                    assert_eq!(point.branch_names, ["V1", "L1"]);
                    assert_eq!(point.currents.len(), 2);
                    assert!((point.currents[0] + point.currents[1]).norm() < 1e-12);
                    assert!((point.currents[1] - (1.0 - expected)).norm() < 1e-10);
                }
            }
        }
    }
}

#[test]
fn shared_level2_ac_uses_all_ampere_turns_and_the_authored_core_coupling() {
    for (dc, secondary_dc) in [
        (0.0, 0.0),
        (0.1, 0.0),
        (-0.1, 0.0),
        (0.1, -0.05),
        (0.1, 0.02),
    ] {
        for coupling in [0.0, 0.25, 1.0] {
            let netlist = Netlist::parse(&format!("Loaded core AC\nV1 in 0 DC {dc} AC 1\nR1 in p 1\nL1 p 0 10\nL2 s 0 20\nR2 s 0 4\nI2 0 s DC {secondary_dc}\nK1 L1 L2 {coupling} core\n.model core CORE level=2 c=.001\n.end\n")).unwrap();
            for point in engine().run_ac(&netlist, &[0.0, 1.0, 1e5, 1e9]).unwrap() {
                let z = Complex64::new(
                    0.0,
                    std::f64::consts::TAU
                        * point.frequency
                        * biased_inductance(1000.0 * dc + 2000.0 * secondary_dc, 0.0),
                );
                // Actual Xyce nonlinear CORE ignores the K-card scalar.
                // R2 reflected through its 2:1 turns ratio is one ohm.
                let expected = z / (1.0 + 2.0 * z);
                let voltage = |name: &str| {
                    point.voltages[point
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(name))
                        .unwrap()]
                };
                assert!(
                    (voltage("p") - expected).norm() < 1e-10,
                    "bias={dc}, coupling={coupling}, frequency={}, actual={}, expected={expected}",
                    point.frequency,
                    voltage("p")
                );
                assert!((voltage("s") - 2.0 * expected).norm() < 1e-10);
                assert_eq!(point.branch_names, ["V1", "L1", "L2"]);
                assert_eq!(point.currents.len(), 3);
                assert!((point.currents[0] + point.currents[1]).norm() < 1e-12);
                assert!((point.currents[2] + expected / 2.0).norm() < 1e-10);
            }
        }
    }
}

#[test]
fn level2_core_noise_uses_the_material_transfer_function() {
    let netlist = Netlist::parse("Core resistor noise\nV1 in 0 DC 0 AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 c=.001\n.end\n").unwrap();
    let points = engine()
        .run_noise_named_with_input_source(&netlist, "p", None, "V1", &[1.0, 1e5, 1e9], 300.0)
        .unwrap();
    for point in points {
        let z = Complex64::new(
            0.0,
            std::f64::consts::TAU * point.frequency * biased_inductance(0.0, 0.0),
        );
        let gain = (z / (1.0 + z)).norm_sqr();
        // Xyce 7.10's thermal-noise convention uses its historical k_B.
        let expected = 4.0 * 1.3806226e-23 * 300.0 * gain;
        assert!(
            (point.output_noise_density / expected - 1.0).abs() < 1e-10,
            "frequency={}, actual={}, expected={}, gain={}, expected_gain={gain}",
            point.frequency,
            point.output_noise_density,
            expected,
            point.input_gain_squared
        );
        assert!((point.input_gain_squared / gain - 1.0).abs() < 1e-10);
    }
}

#[test]
fn level2_core_pole_zero_uses_the_material_inductance() {
    let netlist = Netlist::parse("Core RL pole\nV1 in 0 DC 0 AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 c=.001\n.end\n").unwrap();
    let result = engine()
        .run_pz_ports(&netlist, 1, None, 2, None, false, true, true)
        .unwrap();
    let expected = -1.0 / biased_inductance(0.0, 0.0);
    assert_eq!(result.poles.len(), 1, "{result:?}");
    assert!((result.poles[0].re / expected - 1.0).abs() < 1e-10);
    assert!(result.poles[0].im.abs() < 1e-10 * expected.abs());
    assert_eq!(result.zeros.len(), 1, "{result:?}");
    assert!(result.zeros[0].norm() < 1e-10);
}

#[test]
fn undefined_level2_material_tangent_is_reported_by_ac_and_noise() {
    // At M=H=0, alpha*sqrt((Man-M)^2+(beta_m*Ms)^2) == K.
    // The irreversible susceptibility has a pole. Do not replace it with zero.
    let netlist = Netlist::parse("Singular core\nV1 in 0 DC 0 AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 ms=1 a=1 k=1 alpha=1 betam=1 c=.001\n.end\n").unwrap();
    let ac = engine().run_ac(&netlist, &[1e3]).unwrap_err();
    let noise = engine()
        .run_noise_named_with_input_source(&netlist, "p", None, "V1", &[1e3], 300.0)
        .unwrap_err();
    for error in [ac, noise] {
        let message = error.to_string();
        assert!(
            message.contains("L1")
                && message.contains("undefined or unconverged DC material tangent"),
            "{message}"
        );
    }
}

#[test]
fn noncontractive_dc_initialization_matches_high_precision_material_root() {
    // 60-digit decimal solution of M=Happ*P(Happ,M), at Happ=+/-1000 A/m,
    // default geometry and c=.001: M=+/-254743.931239469431584742774744835...
    // Ordinary fixed-point iteration oscillates here; a residual solve must
    // converge or report failure, rather than publishing the last iteration.
    for dc in [-1.0, 1.0] {
        let netlist = Netlist::parse(&format!("Noncontractive core initialization\nV1 in 0 DC {dc} AC 1\nR1 in p 1\nL1 p 0 10\nK1 L1 1 core\n.model core CORE level=2 c=.001\n.end\n")).unwrap();
        let inductance =
            4e-7 * std::f64::consts::PI * 1e-5 / 0.01 * 100.0 * (1.0 + 254.743_931_239_469_44);
        for point in engine().run_ac(&netlist, &[1.0, 1e5, 1e9]).unwrap() {
            let z = Complex64::new(0.0, std::f64::consts::TAU * point.frequency * inductance);
            let index = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("p"))
                .unwrap();
            assert!((point.voltages[index] - z / (1.0 + z)).norm() < 1e-10);
        }
    }
}

#[test]
fn core_stamps_preserve_independent_linear_transformer_coupling() {
    for coupling in [0.0, 0.25, 0.8] {
        let netlist = Netlist::parse(&format!("Independent transformer and core\nV1 in 0 AC 1\nR1 in p 1\nL1 p 0 1m\nL2 s 0 4m\nR2 s 0 4\nK1 L1 L2 {coupling}\nV3 cin 0 AC 1\nR3 cin cp 1\nL3 cp 0 10\nK3 L3 1 core\n.model core CORE level=2 c=.001\n.end\n")).unwrap();
        for point in engine().run_ac(&netlist, &[0.0, 1.0, 1e3, 1e6]).unwrap() {
            let omega = std::f64::consts::TAU * point.frequency;
            let jw = Complex64::new(0.0, omega);
            let mutual = coupling * 0.002;
            let impedance = jw * 0.001 + (omega * mutual).powi(2) / (4.0 + jw * 0.004);
            let current = 1.0 / (1.0 + impedance);
            let voltage = |name: &str| {
                point.voltages[point
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap()]
            };
            assert!((voltage("p") - impedance * current).norm() < 1e-10);
            assert!(
                (voltage("s") - jw * mutual * current * 4.0 / (4.0 + jw * 0.004)).norm() < 1e-10
            );
            let z = jw * biased_inductance(0.0, 0.0);
            assert!((voltage("cp") - z / (1.0 + z)).norm() < 1e-10);
        }
    }
}
