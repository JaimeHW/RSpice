//! Contract tests for consumers of an already-authenticated shooting-PSS
//! operating point. These deliberately exercise a sideband span larger than
//! the producer's optional saved harmonic count; the retained time orbit's
//! Nyquist capacity is the governing numerical limit.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::pss::PssConfig;
use rspice_core::engine::{Engine, PssOperatingPoint, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

#[test]
fn nonlinear_descriptor_orbits_preserve_implicit_voltages_and_physical_currents() {
    for dialect in [
        rspice_core::config::SpiceDialect::Ngspice,
        rspice_core::config::SpiceDialect::Xyce,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        for divider in [false, true] {
            for source in ["V1 src 0 SIN(0 1 1)", "B1 src 0 V=sin(2*pi*time)"] {
                let control = if source.starts_with('V') { "V1" } else { "B1" };
                let devices = if divider {
                    "R1 src in 1\nR2 in 0 1\nD1 in 0 DM\nE1 out 0 in 0 2".to_owned()
                } else {
                    format!("R1 src 0 1k\nH1 out 0 {control} 2\nD1 out 0 DM")
                };
                let deck = Netlist::parse(&format!("Implicit nonlinear orbit\n{source}\n{devices}\nCout out 0 1u\n.model DM D(IS=1e-12)\n.end\n")).unwrap();
                // Authored N=1, IS=1e-12 at nominal 27C, with each dialect's
                // published SPICE physical constants.
                let nvt = 300.15
                    * if dialect == rspice_core::config::SpiceDialect::Xyce {
                        1.3806226e-23 / 1.6021918e-19
                    } else {
                        1.38064852e-23 / 1.6021766208e-19
                    };
                let isat = 1e-12;
                // Independent closed-form junction law over this orbit's
                // forward and reverse-leakage branches (below exp limiting).
                let law = |v: f64| {
                    if v >= -3.0 * nvt {
                        let e = (v / nvt).exp();
                        (isat * (e - 1.0), isat * e / nvt)
                    } else {
                        let a = (3.0 * nvt / (v * std::f64::consts::E)).powi(3);
                        (-isat * (1.0 + a), 3.0 * isat * a / v)
                    }
                };
                let point = engine
                    .run_pss_operating_point_with_abort(
                        &deck,
                        PssConfig::new(1.0)
                            .with_points_per_period(128)
                            .with_tstab_periods(0),
                        &NoAbort,
                    )
                    .unwrap();
                assert!(point.shooting_state_basis().is_empty());
                assert!(point.analysis().floquet_multipliers.is_empty());
                let result = &point.analysis().result;
                let output = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                let branch = result
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(if divider { "E1" } else { "H1" }))
                    .unwrap();
                for (index, &time) in result.time.iter().enumerate() {
                    let phase = std::f64::consts::TAU * time;
                    let source = phase.sin();
                    let source_rate = std::f64::consts::TAU * phase.cos();
                    let (voltage, current) = if divider {
                        let (mut lo, mut hi) = (-1.0, 1.0);
                        for _ in 0..100 {
                            let v = 0.5 * (lo + hi);
                            if 2.0 * v + law(v).0 > source {
                                hi = v;
                            } else {
                                lo = v;
                            }
                        }
                        let v = 0.5 * (lo + hi);
                        (2.0 * v, -2e-6 * source_rate / (2.0 + law(v).1))
                    } else {
                        let v = -0.002 * source;
                        (v, 2e-9 * source_rate - law(v).0)
                    };
                    let actual = result.waveforms[output].values[index];
                    assert!(
                        (actual - voltage).abs() < 2e-11,
                        "{dialect:?}, divider={divider}, t={time}: V {actual} vs {voltage}"
                    );
                    let actual = result.branch_waveforms[branch].values[index];
                    assert!(
                        (actual - current).abs() < 2e-12,
                        "{dialect:?}, divider={divider}, t={time}: I {actual} vs {current}"
                    );
                }
            }
        }
    }
}

#[test]
fn nonlinear_descriptor_coupled_diode_island_matches_authored_orbit() {
    for dialect in [
        rspice_core::config::SpiceDialect::Ngspice,
        rspice_core::config::SpiceDialect::Xyce,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let model = "D1 a 0 DM\nD2 b 0 DM\n.model DM D(IS=1e-12)";
        let nvt = 300.15
            * if dialect == rspice_core::config::SpiceDialect::Xyce {
                1.3806226e-23 / 1.6021918e-19
            } else {
                1.38064852e-23 / 1.6021766208e-19
            };
        let isat = 1e-12;
        let a = "(0.4+0.05*sin(2*pi*time))";
        let b = "(0.3+0.03*cos(2*pi*time))";
        let deck=Netlist::parse(&format!("Coupled nonlinear forcing\n{model}\nR1 a 0 100\nR2 b 0 200\nR3 a b 150\nB1 0 a I={a}/100+({a}-{b})/150+{isat:.17e}*(exp({a}/{nvt:.17e})-1)\nB2 0 b I={b}/200+({b}-{a})/150+{isat:.17e}*(exp({b}/{nvt:.17e})-1)\nE1 out 0 a b 2\nCout out 0 1u\n.end\n")).unwrap();
        let point = engine
            .run_pss_operating_point_with_abort(
                &deck,
                PssConfig::new(1.0)
                    .with_points_per_period(64)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .unwrap();
        assert!(point.shooting_state_basis().is_empty());
        let result = &point.analysis().result;
        let node = |name: &str| {
            result
                .node_names
                .iter()
                .position(|entry| entry.eq_ignore_ascii_case(name))
                .unwrap()
        };
        let branch = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("E1"))
            .unwrap();
        for (index, &time) in result.time.iter().enumerate() {
            let omega = std::f64::consts::TAU;
            let a = 0.4 + 0.05 * (omega * time).sin();
            let b = 0.3 + 0.03 * (omega * time).cos();
            for (name, expected) in [("a", a), ("b", b), ("out", 2.0 * (a - b))] {
                assert!(
                    (result.waveforms[node(name)].values[index] - expected).abs() < 2e-11,
                    "{dialect:?}, {name}, t={time}"
                );
            }
            let expected =
                -2e-6 * omega * (0.05 * (omega * time).cos() + 0.03 * (omega * time).sin());
            assert!((result.branch_waveforms[branch].values[index] - expected).abs() < 2e-12);
        }
    }
}

#[test]
fn behavioral_descriptor_orbits_preserve_physical_forcing_and_currents() {
    for dialect in [
        rspice_core::config::SpiceDialect::Ngspice,
        rspice_core::config::SpiceDialect::Xyce,
    ] {
        for current_source in [false, true] {
            let devices = if current_source {
                "B1 in 0 I=exp(0.2*sin(2*pi*time))\nL1 in 0 0.1\nH1 out 0 L1 2\nCout out 0 0.2"
            } else {
                "B1 in 0 V=exp(0.2*sin(2*pi*time))\nR1 in 0 4\nCin in 0 0.3 IC=0\nH1 out 0 B1 2\nCout out 0 0.2 IC=0"
            };
            let deck = Netlist::parse(&format!("Analytic B forcing\n{devices}\n.end\n")).unwrap();
            let point = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
                .run_pss_operating_point_with_abort(
                    &deck,
                    PssConfig::new(1.0)
                        .with_points_per_period(128)
                        .with_tstab_periods(0),
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("{dialect:?}, current={current_source}: {error}"));
            assert!(point.shooting_state_basis().is_empty());
            let result = &point.analysis().result;
            let node = |name: &str| {
                result
                    .node_names
                    .iter()
                    .position(|entry| entry.eq_ignore_ascii_case(name))
                    .unwrap()
            };
            let branch = |name: &str| {
                result
                    .branch_names
                    .iter()
                    .position(|entry| entry.eq_ignore_ascii_case(name))
                    .unwrap()
            };
            for (index, &time) in result.time.iter().enumerate() {
                let omega = std::f64::consts::TAU;
                let phase = omega * time;
                let value = (0.2 * phase.sin()).exp();
                let rate = 0.2 * omega * phase.cos() * value;
                let acceleration = ((0.2 * omega * phase.cos()).powi(2)
                    - 0.2 * omega * omega * phase.sin())
                    * value;
                let (input_voltage, input_current, output_voltage, output_current) =
                    if current_source {
                        (-0.1 * rate, -value, -2.0 * value, 0.4 * rate)
                    } else {
                        (
                            value,
                            -value / 4.0 - 0.3 * rate,
                            -value / 2.0 - 0.6 * rate,
                            0.4 * (rate / 4.0 + 0.3 * acceleration),
                        )
                    };
                for (name, expected) in [("in", input_voltage), ("out", output_voltage)] {
                    let actual = result.waveforms[node(name)].values[index];
                    assert!(
                        (actual - expected).abs() < 2e-9,
                        "{name}, t={time}: {actual} vs {expected}"
                    );
                }
                for (name, expected) in [
                    (if current_source { "L1" } else { "B1" }, input_current),
                    ("H1", output_current),
                ] {
                    let actual = result.branch_waveforms[branch(name)].values[index];
                    assert!(
                        (actual - expected).abs() < 2e-8,
                        "{name}, t={time}: {actual} vs {expected}"
                    );
                }
            }
        }
    }
}

#[test]
fn coupled_descriptor_controlled_cutsets_preserve_mutual_flux_and_forcing() {
    let deck = Netlist::parse("Controlled current cutsets\nV1 in 0 SIN(0.7 1 1 0 0 37)\nRin in 0 4\nG1 0 a in 0 2\nL1 a b 0.2\nR1 b 0 3\nF1 0 c V1 3\nL2 c d 0.8\nR2 d 0 5\nK1 L1 L2 0.25\nH1 out 0 L1 2\nCout out 0 0.2\n.end\n").unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(
            &deck,
            PssConfig::new(1.0)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    assert!(point.shooting_state_basis().is_empty());
    let result = &point.analysis().result;
    let node = |name: &str| {
        result
            .node_names
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let branch = |name: &str| {
        result
            .branch_names
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(name))
            .unwrap()
    };
    for (index, &time) in result.time.iter().enumerate() {
        let omega = std::f64::consts::TAU;
        let phase = omega * time + 37_f64.to_radians();
        let v = 0.7 + phase.sin();
        let slope = omega * phase.cos();
        for (name, expected) in [
            ("a", 6.0 * v + 0.325 * slope),
            ("c", -3.75 * v - 0.4 * slope),
            ("out", 4.0 * v),
        ] {
            let actual = result.waveforms[node(name)].values[index];
            assert!(
                (actual - expected).abs() < 2e-10,
                "{name}, t={time}: {actual} != {expected}"
            );
        }
        for (name, expected) in [("L1", 2.0 * v), ("L2", -0.75 * v), ("H1", -0.8 * slope)] {
            let actual = result.branch_waveforms[branch(name)].values[index];
            assert!(
                (actual - expected).abs() < 2e-10,
                "{name}, t={time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn coupled_descriptor_orbit_preserves_physical_ic_branch_currents() {
    let deck = Netlist::parse("Coupled IC orbit\nV1 in 0 SIN(0 1 1)\nR1 in 0 4\nCin in 0 0.3 IC=0\nH1 out 0 V1 2\nCout out 0 0.2 IC=0\n.end\n").unwrap();
    let engine = Engine::new(
        SimulationConfig::default().with_spice_dialect(rspice_core::config::SpiceDialect::Xyce),
    );
    let point = engine
        .run_pss_operating_point_with_abort(
            &deck,
            PssConfig::new(1.0)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    assert!(point.shooting_state_basis().is_empty());
    let result = &point.analysis().result;
    let branch = |name: &str| {
        result
            .branch_names
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(name))
            .unwrap()
    };
    for (index, &time) in result.time.iter().enumerate() {
        let omega = std::f64::consts::TAU;
        let phase = omega * time;
        let source_current = -phase.sin() / 4.0 - 0.3 * omega * phase.cos();
        let output_current = 0.2 * (omega * phase.cos() / 2.0 - 0.6 * omega.powi(2) * phase.sin());
        assert!(
            (result.branch_waveforms[branch("V1")].values[index] - source_current).abs() < 2e-10
        );
        assert!(
            (result.branch_waveforms[branch("H1")].values[index] - output_current).abs() < 2e-9
        );
        assert!(
            (result.branch_waveforms[branch("H1")].values[index]
                + result.branch_waveforms[branch("Cout")].values[index])
                .abs()
                < 2e-12
        );
    }
}

#[test]
fn coupled_descriptor_orbits_preserve_physical_modes_and_currents() {
    let omega = std::f64::consts::TAU;
    for case in 0..5 {
        let devices = match case {
            0 => "R1 in 0 4\nH1 out 0 V1 0",
            1 => "R1 in 0 4\nH1 out 0 V1 2",
            2 => "R1 in 0 4\nCin in 0 0.3\nH1 out 0 V1 2",
            3 => "L1 in mid 0.1\nR1 mid 0 1\nH1 out 0 L1 2",
            _ => "R1 in mid 1\nR2 mid 0 1\nE1 out 0 mid 0 2",
        };
        let deck = Netlist::parse(&format!(
            "Coupled descriptor orbit\nV1 in 0 SIN(0.7 1 1 0 0 37)\n{devices}\nCout out 0 0.2\n.end\n"
        )).unwrap();
        let engine = Engine::default();
        let point = engine
            .run_pss_operating_point_with_abort(
                &deck,
                PssConfig::new(1.0)
                    .with_points_per_period(1024)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("case {case}: {error}"));
        assert_eq!(
            point.shooting_state_basis(),
            if case == 3 { &["L:L1"][..] } else { &[][..] }
        );
        assert_eq!(
            point.analysis().floquet_multipliers.len(),
            usize::from(case == 3)
        );
        if case == 3 {
            assert!((point.analysis().floquet_multipliers[0].re - (-10.0_f64).exp()).abs() < 2e-7);
        }
        let result = &point.analysis().result;
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let branch = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(if case == 4 { "E1" } else { "H1" }))
            .unwrap();
        let mut voltage_error = 0.0_f64;
        let mut current_error = 0.0_f64;
        for (index, &time) in result.time.iter().enumerate() {
            let phase = omega * time + 37_f64.to_radians();
            let v = 0.7 + phase.sin();
            let slope = omega * phase.cos();
            let acceleration = -omega * omega * phase.sin();
            let (expected, rate) = match case {
                0 => (0.0, 0.0),
                1 => (-v / 2.0, -slope / 2.0),
                2 => (-v / 2.0 - 0.6 * slope, -slope / 2.0 - 0.6 * acceleration),
                3 => {
                    let lag = omega * 0.1;
                    let current = 0.7 + (phase.sin() - lag * phase.cos()) / (1.0 + lag * lag);
                    (2.0 * current, 2.0 * (v - current) / 0.1)
                }
                _ => (v, slope),
            };
            voltage_error =
                voltage_error.max((result.waveforms[output].values[index] - expected).abs());
            current_error = current_error
                .max((result.branch_waveforms[branch].values[index] + 0.2 * rate).abs());
        }
        assert!(
            voltage_error < 3e-5,
            "case {case}, voltage error {voltage_error:e}"
        );
        assert!(
            current_error < 2e-4,
            "case {case}, current error {current_error:e}"
        );
        if case == 3 {
            let pac = engine
                .run_pac_from_pss_with_abort(
                    &deck,
                    PacConfig::new()
                        .with_fundamental(1.0)
                        .with_sweep(0.25, 0.25, 1)
                        .with_sweep_type(PacSweepType::Linear)
                        .with_sidebands(0, 0)
                        .with_input_source("V1")
                        .with_output_node("out"),
                    &point,
                    &NoAbort,
                )
                .unwrap();
            let expected = num_complex::Complex64::new(2.0, 0.0)
                / num_complex::Complex64::new(1.0, omega * 0.025);
            assert!((pac.result.conversion_matrix.get(0, 0, 0).unwrap() - expected).norm() < 2e-12);
        }
    }
}

#[test]
fn vcvs_charge_constraints_preserve_the_free_rc_mode() {
    for gain in [0.0, 2.0, -0.25] {
        for prescribed in [false, true] {
            let input = if prescribed {
                "V1 in 0 SIN(0 1 1)\nR1 in 0 1\n"
            } else {
                "I1 0 in SIN(0 1 1)\nR1 in 0 1\nC1 in 0 0.1\n"
            };
            let deck = Netlist::parse(&format!(
                "Controlled charge orbit\n{input}E1 out 0 in 0 {gain}\nC2 out 0 0.2\n.end\n"
            ))
            .unwrap();
            let point = Engine::default()
                .run_pss_operating_point_with_abort(
                    &deck,
                    PssConfig::new(1.0)
                        .with_points_per_period(1024)
                        .with_tstab_periods(0),
                    &NoAbort,
                )
                .unwrap();
            let expected_states: &[&str] = if prescribed { &[] } else { &["C:C1"] };
            assert_eq!(point.shooting_state_basis(), expected_states);
            let analysis = point.analysis();
            assert_eq!(analysis.floquet_multipliers.len(), usize::from(!prescribed));
            if !prescribed {
                let multiplier = analysis.floquet_multipliers[0];
                assert!((multiplier.re - (-10.0_f64).exp()).abs() < 2e-7);
                assert_eq!(multiplier.im, 0.0);
            }
            let result = &analysis.result;
            let input = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("in"))
                .unwrap();
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let omega = std::f64::consts::TAU;
            for ((&time, &vin), &vout) in result
                .time
                .iter()
                .zip(&result.waveforms[input].values)
                .zip(&result.waveforms[output].values)
            {
                let phase = omega * time;
                let expected = if prescribed {
                    phase.sin()
                } else {
                    (phase.sin() - 0.1 * omega * phase.cos()) / (1.0 + (0.1 * omega).powi(2))
                };
                assert!(
                    (vin - expected).abs() < 2e-5,
                    "gain {gain}, t={time}, {vin} vs {expected}"
                );
                assert!((vout - gain * vin).abs() < 2e-12);
            }
            let branch = result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("E1"))
                .unwrap();
            let derivative = if prescribed {
                omega
            } else {
                -result.waveforms[input].values[0] / 0.1
            };
            assert!(
                (result.branch_waveforms[branch].values[0] + 0.2 * gain * derivative).abs() < 2e-12,
                "the initial source current must include dependent-capacitor displacement current"
            );
            if gain == 2.0 && !prescribed {
                let pac = Engine::default()
                    .run_pac_from_pss_with_abort(
                        &deck,
                        PacConfig::new()
                            .with_fundamental(1.0)
                            .with_sweep(0.25, 0.25, 1)
                            .with_sweep_type(PacSweepType::Linear)
                            .with_sidebands(0, 0)
                            .with_input_source("I1")
                            .with_output_node("out"),
                        &point,
                        &NoAbort,
                    )
                    .unwrap();
                let actual = pac.result.conversion_matrix.get(0, 0, 0).unwrap();
                let expected = num_complex::Complex64::new(gain, 0.0)
                    / num_complex::Complex64::new(1.0, omega * 0.25 * 0.1);
                assert!((actual - expected).norm() < 2e-12);
            }
        }
    }
}

fn retained_linear_operating_point(engine: &Engine, netlist: &Netlist) -> PssOperatingPoint {
    let config = PssConfig::new(F0)
        .with_harmonics(20)
        .with_points_per_period(256)
        .with_tstab_periods(0);
    engine
        .run_pss_operating_point_with_abort(netlist, config, &NoAbort)
        .expect("linear producer yields an authenticated retained PSS state")
}

fn linear_deck() -> Netlist {
    Netlist::parse(
        "* retained PSS consumer contract\n\
         vin in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         r2 out 0 1k\n\
         c1 out 0 1p\n\
         .end\n",
    )
    .expect("deck parses")
}

#[test]
fn mockup_sideband_span_uses_orbit_nyquist_capacity_not_saved_harmonic_count() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = linear_deck();
    let operating_point = retained_linear_operating_point(&engine, &netlist);
    assert_eq!(operating_point.config().num_harmonics, 20);
    assert_eq!(operating_point.spectral_harmonic_capacity(), 128);
    let pac = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e3, 1.0e3, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-20, 20)
        .with_input_source("vin")
        .with_output_node("out");
    engine
        .run_pac_from_pss_with_abort(&netlist, pac, &operating_point, &NoAbort)
        .expect("PAC +/-20 consumes the 256-point retained orbit");

    let pnoise = engine
        .run_pnoise_from_pss_with_abort(
            &netlist,
            &[1.0e3],
            "out",
            None,
            Some("vin"),
            20,
            &operating_point,
            &NoAbort,
        )
        .expect("PNOISE +/-20 consumes the 256-point retained orbit");
    assert_eq!(pnoise.output_noise.len(), 1);
}

#[test]
fn dependent_basis_includes_the_carrier_outside_the_conversion_window() {
    let deck = Netlist::parse(
        "Carrier bandwidth\nI1 0 out SIN(0 1 10)\nR1 out 0 RM 1\nC1 out 0 1u\n.model RM R(KF=1 AF=2 EF=1)\n.end\n",
    ).unwrap();
    let engine = Engine::default();
    let point = engine
        .run_pss_operating_point_with_abort(
            &deck,
            PssConfig::new(1.0)
                .with_harmonics(2)
                .with_points_per_period(1024)
                .with_tstab(0.0),
            &NoAbort,
        )
        .unwrap();
    let omega_c = std::f64::consts::TAU * 1e-6;
    let expected_noise = 0.25 * (1.0 / 9.75 + 1.0 / 10.25)
        / (1.0 + (10.0 * omega_c).powi(2))
        / (1.0 + (0.25 * omega_c).powi(2));
    for retained in [false, true] {
        let noise = if retained {
            engine.run_pnoise_from_pss_with_abort(
                &deck,
                &[0.25],
                "out",
                None,
                None,
                0,
                &point,
                &NoAbort,
            )
        } else {
            engine.run_pnoise(&deck, 1.0, &[0.25], "out", None, None, 0)
        }
        .unwrap();
        let flicker = noise
            .contributors
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
            .unwrap()
            .1[0];
        assert!((flicker / expected_noise - 1.0).abs() < 2e-9);

        let config = PacConfig::new()
            .with_fundamental(1.0)
            .with_sweep(0.25, 0.25, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(0, 0)
            .with_input_source("I1")
            .with_output_node("out");
        let pac = if retained {
            engine.run_pac_from_pss_with_abort(&deck, config, &point, &NoAbort)
        } else {
            engine.run_pac(&deck, config)
        }
        .unwrap();
        let transfer = pac.result.conversion_matrix.get(0, 0, 0).unwrap();
        let expected = num_complex::Complex64::new(1.0, 0.0)
            / num_complex::Complex64::new(1.0, 0.25 * omega_c);
        assert!((transfer - expected).norm() < 2e-12);
    }
}

#[test]
fn retained_pss_branch_currents_drive_flicker_including_zero_dc_resistance() {
    for resistance in [1.0_f64, 0.0] {
        let deck = Netlist::parse(&format!(
            "Retained branch noise\nI1 0 input SIN(0 1 10)\nR2 input out 10\nR1 out 0 RM {resistance} AC=2\nC1 input 0 1u\n.model RM R(KF=1 AF=2 EF=1)\n.options device zeroresistancetol=2\n.end\n"
        )).unwrap();
        let engine = Engine::default();
        let point = engine
            .run_pss_operating_point_with_abort(
                &deck,
                PssConfig::new(1.0)
                    .with_harmonics(2)
                    .with_points_per_period(2048)
                    .with_tstab(0.0),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("R={resistance}: {error}"));
        let result = &point.analysis().result;
        assert_eq!(result.branch_names, ["R1"]);
        assert_eq!(result.branch_waveforms[0].values.len(), result.time.len());
        let omega_c = std::f64::consts::TAU * 1e-6;
        for (&time, &current) in result.time.iter().zip(&result.branch_waveforms[0].values) {
            let angle = std::f64::consts::TAU * 10.0 * time;
            let lag = 10.0 * omega_c * (10.0 + resistance);
            let expected = (angle.sin() - lag * angle.cos()) / (1.0 + lag * lag);
            assert!(
                (current - expected).abs() < 1e-6,
                "R={resistance}, t={time}: {current} vs {expected}"
            );
        }
        let expected = (1.0 / 9.75 + 1.0 / 10.25) * (1.0 + (0.25 * omega_c * 10.0).powi(2))
            / (1.0 + (10.0 * omega_c * (10.0 + resistance)).powi(2))
            / (1.0 + (0.25 * omega_c * 12.0).powi(2));
        for sidebands in [0, 8] {
            let noise = engine
                .run_pnoise_from_pss_with_abort(
                    &deck,
                    &[0.25],
                    "out",
                    None,
                    None,
                    sidebands,
                    &point,
                    &NoAbort,
                )
                .unwrap();
            let actual = noise
                .contributors
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
                .unwrap()
                .1[0];
            assert!(
                (actual / expected - 1.0).abs() < 2e-9,
                "R={resistance}, K={sidebands}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn zero_ohm_constraints_preserve_currents_without_spurious_charge_modes() {
    for (terminals, polarity) in [("out 0", 1.0), ("0 out", -1.0)] {
        let deck = Netlist::parse(&format!(
            "Shorted charge\nI1 0 out SIN(0.5 1 10 0 0 37)\nR1 {terminals} RM 0 AC=2\nC1 out 0 1u\nD1 out 0 DM\n.model RM R(KF=1 AF=2 EF=1)\n.model DM D(IS=0 CJO=1n M=0)\n.end\n"
        )).unwrap();
        let engine = Engine::default();
        let point = engine
            .run_pss_operating_point_with_abort(
                &deck,
                PssConfig::new(1.0)
                    .with_harmonics(2)
                    .with_points_per_period(1024)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .expect("an ideal short prescribes zero voltage across both charge branches");
        assert!(point.shooting_state_basis().is_empty());
        assert!(point.shooting_state().is_empty());
        assert!(point.analysis().monodromy.is_empty());
        assert_eq!(
            point.analysis().result.floquet_evidence,
            rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes
        );
        let result = &point.analysis().result;
        assert_eq!(result.branch_names, ["R1"]);
        for (&time, &current) in result.time.iter().zip(&result.branch_waveforms[0].values) {
            let expected = polarity
                * (0.5 + (std::f64::consts::TAU * 10.0 * time + 37.0_f64.to_radians()).sin());
            assert!(
                (current - expected).abs() < 2e-12,
                "{terminals}, t={time}: {current} vs {expected}"
            );
        }
        assert!(
            result.waveforms[0]
                .values
                .iter()
                .all(|voltage| *voltage == 0.0)
        );
        let noise = engine
            .run_pnoise_from_pss_with_abort(&deck, &[0.25], "out", None, None, 0, &point, &NoAbort)
            .unwrap();
        let flicker = noise
            .contributors
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
            .unwrap()
            .1[0];
        let expected = (4.0 + 1.0 / 9.75 + 1.0 / 10.25)
            / (1.0 + (std::f64::consts::TAU * 0.25 * 2.0 * 1.001e-6).powi(2));
        assert!(
            (flicker / expected - 1.0).abs() < 2e-12,
            "{flicker} vs {expected}"
        );
    }
}

#[test]
fn zero_ohm_reduction_does_not_accept_nonunique_or_contradictory_mna_constraints() {
    for constraints in [
        "R1 out 0 0\nR2 out 0 0",
        "V1 out 0 0\nR1 out 0 0",
        "V1 out 0 1\nR1 out 0 0",
        "V1 a 0 1\nR1 a out 0\nR2 out 0 0",
    ] {
        let deck = Netlist::parse(&format!(
            "Ideal constraint failure\nI1 0 out SIN(0 1 1)\nC1 out 0 1u\n{constraints}\n.end\n"
        ))
        .unwrap();
        assert!(
            Engine::default()
                .run_pss_operating_point_with_abort(
                    &deck,
                    PssConfig::new(1.0)
                        .with_points_per_period(16)
                        .with_tstab_periods(0),
                    &NoAbort
                )
                .is_err(),
            "invalid ideal constraints must not publish an operating point: {constraints}"
        );
    }
}
