//! Public-API qualification of the legacy GP forward excess-phase response.

use num_complex::Complex64;
use rspice_core::analysis::AcResult;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use std::f64::consts::PI;

// ngspice 46 const.h; the native GP model retains these thermal constants.
fn ngspice_thermal_voltage(temperature: f64) -> f64 {
    1.38064852e-23 * temperature / 1.6021766208e-19
}

#[test]
fn primitive_noise_absolute_densities_use_each_dialects_constants() {
    let temperature = 300.15;
    let source = Netlist::parse(
        "Absolute thermal and shot noise\nVR r 0 0\nR1 r 0 750\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 mm\n.model mm NPN IS=1e-16 BF=100 BR=1\n.options GMIN=0 RELTOL=1e-10 ABSTOL=1e-18 VNTOL=1e-12\n.end",
    ).unwrap();
    // Independent constants: SI, ngspice46 const.h, Xyce7.10 N_DEV_Const.h.
    for (dialect, k, q) in [
        (SpiceDialect::BestAvailable, 1.380649e-23, 1.602176634e-19),
        (SpiceDialect::Ngspice, 1.38064852e-23, 1.6021766208e-19),
        (SpiceDialect::Xyce, 1.3806226e-23, 1.6021918e-19),
    ] {
        let vt: f64 = k * temperature / q;
        let forward = 1e-16 * (0.7 / vt).exp_m1();
        let expected = [4.0 * k * temperature / 750.0, 2.0 * q * forward];
        let results = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
            .run_port_noise_correlation(
                &source,
                &["VR".into(), "VC".into()],
                &[1e3, 1e9],
                temperature,
            )
            .unwrap();
        for result in results {
            for (index, expected) in expected.into_iter().enumerate() {
                let actual = result.current_correlation[index][index];
                // Reverse leakage is <4e-16 A. This bound resolves even the
                // 8e-9 relative distinction between the two electron charges.
                assert!(
                    (actual.re / expected - 1.0).abs() < 1e-10,
                    "{dialect:?} source {index}: {actual} vs {expected:e}"
                );
                assert_eq!(actual.im, 0.0);
            }
            assert_eq!(result.current_correlation[0][1], Complex64::default());
        }
    }
}

#[test]
fn gp_bfs17_excess_phase_matches_independent_six_bias_complex_currents() {
    let original = include_str!("../../../tests/paranoia/control_structs/foreach_bjt_ft.sp");
    let mut in_control = false;
    let mut source = String::new();
    for line in original.lines() {
        if line.trim().eq_ignore_ascii_case(".control") {
            in_control = true;
        } else if line.trim().eq_ignore_ascii_case(".endc") {
            in_control = false;
        } else if !in_control {
            source.push_str(line);
            source.push('\n');
        }
    }
    assert_eq!(source.matches("ic 0 c 0.01").count(), 1);
    assert_eq!(source.matches("PTF=21.000").count(), 1);
    source = source.replace(
        "\n.end\n",
        "\n.options RELTOL=1e-10 ABSTOL=1e-18 VNTOL=1e-12 GMIN=0\n.end\n",
    );
    let data = include_str!("testdata/gp_bfs17_ptf_ngspice46.tsv");
    let mut failures = Vec::new();
    for current in ["0.5e-3", "1e-3", "5e-3", "10e-3", "50e-3", "100e-3"] {
        for phase in ["0", "21"] {
            let rows: Vec<_> = data
                .lines()
                .filter(|line| !line.starts_with('#'))
                .map(|line| line.split_whitespace().collect::<Vec<_>>())
                .filter(|row| row[0] == current && row[1] == phase)
                .collect();
            assert_eq!(rows.len(), 57);
            let frequencies: Vec<f64> = rows.iter().map(|row| row[2].parse().unwrap()).collect();
            let netlist = Netlist::parse(
                &source
                    .replace("ic 0 c 0.01", &format!("ic 0 c {current}"))
                    .replace("PTF=21.000", &format!("PTF={phase}")),
            )
            .unwrap();
            let results = engine(300.15).run_ac(&netlist, &frequencies).unwrap();
            let mut worst = (0.0, 0.0, Complex64::default(), Complex64::default());
            for (point, row) in results.iter().zip(rows) {
                let expected = Complex64::new(row[3].parse().unwrap(), row[4].parse().unwrap());
                let actual = branch(point, "vgain");
                let normalized_error =
                    (actual - expected).norm() / (2e-6 * expected.norm() + 1e-12);
                if normalized_error > worst.0 {
                    worst = (normalized_error, point.frequency, actual, expected);
                }
            }
            if worst.0 >= 1.0 {
                failures.push(format!("BFS17 Ic={current} PTF={phase}: {worst:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn gp_excess_phase_noise_matches_independent_complex_port_covariance() {
    let temperature = 300.15;
    let vt = ngspice_thermal_voltage(temperature);
    let current = 1e-16 * (0.7 / vt).exp_m1();
    let gm = (current + 1e-16) / vt;
    let rb = 100.0;
    let tf = 1e-9;
    let cbc = 2e-12;
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        for internal in [false, true] {
            for phase in [0.0_f64, 21.0, 90.0] {
                // The nominal GP charge factor is unity here. Bias the intrinsic
                // base at 0.7 V by including the independently known RB DC drop.
                let body = if internal {
                    "Q1 c b 0 mm\n"
                } else {
                    "RB b bi 100\nQ1 c bi 0 mm\n"
                };
                let source = Netlist::parse(&format!(
                "GP complex noise\nVB b 0 {}\nVC c 0 {}\n{body}.model mm {kind}(LEVEL=1 IS=1e-16 BF=100 BR=1 RB={} RBM=0 TF={tf} PTF={phase} CJC={cbc} MJC=0 XCJC=1)\n.options GMIN=0 RELTOL=1e-10 ABSTOL=1e-18 VNTOL=1e-12\n.end",
                polarity * (0.7 + rb * current / 100.0), polarity * 2.0,
                if internal { rb } else { 0.0 }
            )).unwrap();
                let noise = engine(temperature)
                    .run_port_noise_correlation(
                        &source,
                        &["VB".into(), "VC".into()],
                        &[1e4, 1e7, 1e8, 1e9],
                        temperature,
                    )
                    .unwrap();
                for point in noise {
                    let omega = 2.0 * PI * point.frequency;
                    let ymu = Complex64::new(0.0, omega * cbc);
                    let y = Complex64::new(1.0 / rb + gm / 100.0, omega * tf * gm) + ymu;
                    let forward =
                        gm * Complex64::from_polar(1.0, -omega * tf * phase.to_radians()) - ymu;
                    let rb_transfer = [1.0 / (rb * y) - 1.0, -forward / y];
                    let ib_transfer = [-1.0 / (rb * y), forward / y];
                    let rb_density = 4.0 * 1.38064852e-23 * temperature / rb;
                    let ib_density = 2.0 * 1.6021766208e-19 * current / 100.0;
                    let ic_density = 2.0 * 1.6021766208e-19 * current;
                    for row in 0..2 {
                        for col in 0..2 {
                            let expected = rb_density * rb_transfer[row] * rb_transfer[col].conj()
                                + ib_density * ib_transfer[row] * ib_transfer[col].conj()
                                + if row == 1 && col == 1 {
                                    ic_density
                                } else {
                                    0.0
                                };
                            let actual = point.current_correlation[row][col];
                            assert!(
                                (actual - expected).norm() < 2e-6 * expected.norm() + 1e-32,
                                "{kind} internal={internal} PTF={phase} f={} [{row},{col}]: {actual:?} != {expected:?}",
                                point.frequency
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn gp_zero_phase_or_transit_time_preserves_ac_exactly() {
    for private in ["", "RB=100 RBM=20 IRB=1e-5 RE=1 RC=2"] {
        for (base, disabled) in [("TF=1n", "TF=1n PTF=0"), ("TF=0", "TF=0 PTF=90")] {
            let make = |parameters: &str| {
                clamped(
                    "NPN",
                    1.0,
                    1.0,
                    1.0,
                    300.15,
                    &format!("{private} {parameters}"),
                )
            };
            let frequencies = [1e3, 1e8, 1e9];
            let engine = engine(300.15);
            let a = engine.run_ac(&make(base), &frequencies).unwrap();
            let b = engine.run_ac(&make(disabled), &frequencies).unwrap();
            for (a, b) in a.iter().zip(b) {
                assert_eq!(a.currents, b.currents);
                assert_eq!(a.voltages, b.voltages);
            }
        }
    }
}

#[test]
fn gp_nonfinite_excess_phase_is_diagnosed() {
    for private in ["", "RB=100 RBM=20 IRB=1e-5"] {
        for (phase, tf, frequency) in [
            (f64::NAN, 1e-9, 1e3),
            (f64::INFINITY, 1e-9, 1e3),
            (1e308, 1e308, 1e3),
            (1e308, 1e-9, 1e12),
        ] {
            let mut source = clamped("NPN", 1.0, 1.0, 1.0, 300.15, private);
            source.models[0]
                .params
                .extend([("PTF".into(), phase), ("TF".into(), tf)]);
            let error = engine(300.15)
                .run_ac(&source, &[frequency])
                .unwrap_err()
                .to_string();
            assert!(error.contains("PTF") && error.contains("Q1"), "{error}");
        }
    }
}

fn engine(temperature: f64) -> Engine {
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    config.temperature = temperature;
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    Engine::new(config)
}

fn branch(point: &AcResult, name: &str) -> Complex64 {
    let index = point
        .branch_names
        .iter()
        .position(|value| value.eq_ignore_ascii_case(name))
        .unwrap();
    point.currents[index]
}

fn clamped(
    kind: &str,
    polarity: f64,
    area: f64,
    multiplier: f64,
    tnom: f64,
    parameters: &str,
) -> Netlist {
    Netlist::parse(&format!(
        "GP phase response\nVC c 0 {}\nVB b 0 DC {} AC 1\nVE e 0 0\nQ1 c b e mm AREA={area} M={multiplier}\n.model mm {kind}(LEVEL=1 IS=1e-16 BF=100 BR=1 TNOM={} {parameters})\n.options GMIN=0\n.end\n",
        polarity * 2.0, polarity * 0.7, tnom - 273.15
    )).unwrap()
}

#[test]
fn gp_excess_phase_rotates_forward_transconductance_with_conservative_ports() {
    for (kind, polarity, temperature, area, multiplier) in [
        ("NPN", 1.0, 300.15, 1.0, 1.0),
        ("PNP", -1.0, 330.15, 0.2, 3.0),
    ] {
        let engine = engine(temperature);
        let vt = ngspice_thermal_voltage(temperature);
        let gm = area * multiplier * 1e-16 * (0.7 / vt).exp() / vt;
        for phase in [0.0_f64, 21.0, 90.0, -30.0] {
            let source = clamped(
                kind,
                polarity,
                area,
                multiplier,
                temperature,
                &format!("TF=1n PTF={phase}"),
            );
            for point in engine.run_ac(&source, &[1e3, 1e8, 2e9]).unwrap() {
                let delay = 1e-9 * phase.to_radians();
                let expected =
                    -gm * Complex64::from_polar(1.0, -2.0 * PI * point.frequency * delay);
                let actual = branch(&point, "VC");
                assert!(
                    (actual - expected).norm() <= gm * 2e-8,
                    "{kind} PTF={phase} T={temperature} f={}: {actual:?} != {expected:?}",
                    point.frequency
                );
                let kcl = actual + branch(&point, "VB") + branch(&point, "VE");
                assert!(kcl.norm() <= gm * 1e-10, "terminal KCL: {kcl:?}");
            }
        }
    }
}

#[test]
fn gp_excess_phase_uses_nominal_transit_time_after_temperature_scaling() {
    let temperature = 360.15;
    let engine = engine(temperature);
    let base = clamped("NPN", 1.0, 1.0, 1.0, 300.15, "TF=1n TTF1=.02 PTF=0");
    let delayed = clamped("NPN", 1.0, 1.0, 1.0, 300.15, "TF=1n TTF1=.02 PTF=45");
    let frequencies = [1e7, 1e8, 1e9];
    let zero = engine.run_ac(&base, &frequencies).unwrap();
    let phase = engine.run_ac(&delayed, &frequencies).unwrap();
    for (zero, phase) in zero.iter().zip(&phase) {
        let ratio = branch(phase, "VC") / branch(zero, "VC");
        let expected = Complex64::from_polar(1.0, -2.0 * PI * phase.frequency * 1e-9 * PI / 4.0);
        assert!(
            (ratio - expected).norm() < 1e-8,
            "nominal TF phase at {} Hz: {ratio:?} != {expected:?}",
            phase.frequency
        );
    }
}

#[test]
fn gp_tiny_excess_phase_and_frequency_order_preserve_the_response() {
    let temperature = 300.15;
    let vt = ngspice_thermal_voltage(temperature);
    let gm = 1e-16 * (0.7 / vt).exp() / vt;
    let phase = 1e-20_f64;
    let source = clamped(
        "NPN",
        1.0,
        1.0,
        1.0,
        temperature,
        &format!("TF=1n PTF={phase}"),
    );
    let engine = engine(temperature);
    let frequencies = [1e3, 1e9, 1e3, 1e8, 1e9];
    let points = engine.run_ac(&source, &frequencies).unwrap();
    for point in &points {
        let expected = gm * (2.0 * PI * point.frequency * 1e-9 * phase.to_radians()).sin();
        let actual = branch(point, "VC").im;
        assert!(
            (actual / expected - 1.0).abs() < 1e-8,
            "f={}: {actual:e} != {expected:e}",
            point.frequency
        );
    }
    assert_eq!(points[0].currents, points[2].currents);
    assert_eq!(points[1].currents, points[4].currents);
}

#[test]
fn gp_authored_phase_is_preserved_by_each_evaluator_preference() {
    // Xyce 7.10 drops PTF in AC (including NEWEXCESSPHASE=1). The evaluator
    // preference retains RSpice's authored physical delay rather than that
    // omission; this is an explicit compatibility difference.
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let zero = clamped("NPN", 1.0, 1.0, 1.0, 300.15, "TF=1n PTF=0");
        let delayed = clamped("NPN", 1.0, 1.0, 1.0, 300.15, "TF=1n PTF=90");
        let frequencies = [1e6, 1e8, 1e9];
        let zero = engine.run_ac(&zero, &frequencies).unwrap();
        let delayed = engine.run_ac(&delayed, &frequencies).unwrap();
        for (zero, delayed) in zero.iter().zip(delayed) {
            let expected =
                Complex64::from_polar(1.0, -2.0 * PI * delayed.frequency * 1e-9 * PI / 2.0);
            let actual = branch(&delayed, "VC") / branch(zero, "VC");
            assert!(
                (actual - expected).norm() < 1e-8,
                "{dialect:?}: {actual:?} != {expected:?}"
            );
        }
    }
}

#[test]
fn gp_pole_zero_requires_a_qualified_delay_descriptor() {
    for (tf, phase) in [(1e-9, 21.0), (1e-9, -21.0), (1e-9, 0.0), (0.0, 21.0)] {
        let source = Netlist::parse(&format!(
            "GP PZ descriptor\nVCC supply 0 3\nRL supply c 1k\nVB b 0 .6\nQ1 c b 0 mm\n.model mm NPN(LEVEL=1 IS=1e-16 BF=100 RB=100 RBM=20 TF={tf} PTF={phase} CJE=2p CJC=2p)\n.end"
        )).unwrap();
        let engine = engine(300.15);
        let collector = engine
            .build_circuit(&source)
            .unwrap()
            .get_node_by_name("c")
            .unwrap();
        let result = engine.run_pz(&source, collector, collector);
        if tf * phase == 0.0 {
            result.unwrap();
        } else {
            let error = result
                .expect_err("a pure PTF delay is not a finite G+sC descriptor")
                .to_string();
            assert!(error.contains("PTF") && error.contains("Q1"), "{error}");
        }
    }
}
