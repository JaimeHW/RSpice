//! Integral-defined capacitance: independent carrier and response equations.
use rspice_core::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{Engine, QpssConfig, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use std::f64::consts::TAU;

fn parse(deck: &str) -> Netlist {
    Netlist::parse_with_options(
        deck,
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap()
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
}

fn deck(depth: usize, startup: &str) -> Netlist {
    let (memory, phase) = if depth == 1 {
        ("(2*pi*1k)*SDT(V(ctrl))", 0)
    } else {
        ("(2*pi*1k)^2*SDT(SDT(V(ctrl)))", 90)
    };
    parse(&format!(
        "Capacitor memory\nVctrl ctrl 0 SIN(0 0.2 1k 0 0 {phase})\n\
        Bdrive 0 out I={{0.2*sin(2*pi*1k*time)/1k+100n*(1.02-0.02*cos(2*pi*1k*time))*2*pi*1k*0.2*cos(2*pi*1k*time)}}\n\
        R1 out 0 1k\nC1 out 0 C={{100n*(1+0.1*{memory})}}\n{startup}\n.end\n"
    ))
}

fn pac() -> PacConfig {
    PacConfig::new()
        .with_fundamental(1e3)
        .with_sweep(130., 130., 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("Vctrl")
        .with_output_node("out")
}

fn response(input: i32, depth: usize) -> Vec<Complex64> {
    // δi=C(t)*δv' + 0.1*C0*w0^depth*v'(t)*δctrl/(j*w_input)^depth.
    let memory_transfer = (Complex64::new(TAU * 1e3, 0.)
        / Complex64::new(0., TAU * (130. + input as f64 * 1e3)))
    .powu(depth as u32);
    let mut a = vec![vec![Complex64::ZERO; 6]; 5];
    for k in -2_i32..=2 {
        for m in -2_i32..=2 {
            let c = match k - m {
                0 => 1.02e-7,
                -1 | 1 => -1e-9,
                _ => 0.,
            };
            a[(k + 2) as usize][(m + 2) as usize] = Complex64::new(
                if k == m { 1e-3 } else { 0. },
                TAU * (130. + m as f64 * 1e3) * c,
            );
        }
        if (k - input).abs() == 1 {
            a[(k + 2) as usize][5] = -1e-8 * 0.1 * TAU * 1e3 * memory_transfer;
        }
    }
    for pivot in 0..5 {
        let divisor = a[pivot][pivot];
        for value in a[pivot].iter_mut().skip(pivot) {
            *value /= divisor;
        }
        let pivot_row = a[pivot].clone();
        for (row, values) in a.iter_mut().enumerate().take(5) {
            if row == pivot {
                continue;
            }
            let factor = values[pivot];
            for (value, &entry) in values.iter_mut().zip(&pivot_row).skip(pivot) {
                *value -= factor * entry;
            }
        }
    }
    a.iter().map(|row| row[5]).collect()
}

#[test]
fn capacitor_integral_hb_retains_nested_memory_and_pac_derivatives() {
    for (depth, startup, krylov) in [(1, "", false), (2, ".options hbint tahb=1", true)] {
        let netlist = deck(depth, startup);
        let mut config = HbConfig::new(1e3)
            .with_harmonics(4)
            .with_collocation_points(33);
        config.tolerance = 1e-9;
        config.abstol = 1e-13;
        config.use_krylov = krylov;
        let hb = engine().run_hb(&netlist, config).unwrap();
        let voltage = &hb
            .result
            .spectral_voltages
            .iter()
            .find(|row| row.node_name.eq_ignore_ascii_case("out"))
            .unwrap()
            .coefficients;
        assert!(voltage[0].norm() < 1e-8);
        assert!((voltage[1] - Complex64::new(0., -0.2)).norm() < 1e-8);
        assert!(voltage[2..].iter().all(|value| value.norm() < 1e-8));
        let name = format!("C:C1:sdt:{}", depth - 1);
        let integral = hb
            .operating_point
            .integral_spectra()
            .iter()
            .find(|row| row.name == name)
            .unwrap();
        assert!((integral.coefficients[0].re * (TAU * 1e3).powi(depth as i32) - 0.2).abs() < 1e-8);
        let capacitor = &hb.result.reactive_spectra[0];
        assert!((capacitor.current_coefficients[0].re + 1e-9 * 0.2 * TAU * 1e3).abs() < 1e-12);
        let result = engine()
            .run_pac_from_hb_with_abort(&netlist, pac(), &hb.operating_point, &NoAbort)
            .unwrap();
        for input in -2..=2 {
            for (row, expected) in response(input, depth).into_iter().enumerate() {
                let actual = result
                    .result
                    .conversion_matrix
                    .get(0, row as i32 - 2, input)
                    .unwrap();
                assert!(
                    (actual - expected).norm() < 1e-8,
                    "depth {depth}, {row}/{input}: {actual} vs {expected}"
                );
            }
        }
    }
    for rate in ["1e-12", "1e-12+sin(2*pi*1k*time)"] {
        let invalid = parse(&format!(
            "Drifting memory\nV1 out 0 SIN(0 1 1k)\nC1 out 0 C={{100n*(1+SDT({rate}))}}\n.end\n"
        ));
        let error = engine()
            .run_hb(&invalid, HbConfig::new(1e3).with_harmonics(3))
            .unwrap_err()
            .to_string();
        assert!(error.contains("nonzero"), "{error}");
    }
}

#[test]
fn capacitor_integral_pss_response_preserves_memory_constants() {
    let netlist = deck(2, "");
    let mut config = PssConfig::new(1e3)
        .with_points_per_period(256)
        .with_tstab_periods(0)
        .with_tolerance(1e-8);
    config.abstol = 1e-12;
    let point = engine()
        .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
        .unwrap();
    assert_eq!(point.shooting_state().last().copied(), Some(0.0));
    let monodromy = &point.analysis().monodromy;
    assert_eq!(monodromy[1], [0.0, 1.0, 0.0]);
    assert_eq!(monodromy[2][0], 0.0);
    assert_eq!(monodromy[2][2], 1.0);
    assert!((monodromy[2][1] - 1e-3).abs() < 1e-9);
    let result = engine()
        .run_pac_from_pss_with_abort(&netlist, pac(), &point, &NoAbort)
        .unwrap();
    for input in -2..=2 {
        for (row, expected) in response(input, 2).into_iter().enumerate() {
            let actual = result
                .result
                .conversion_matrix
                .get(0, row as i32 - 2, input)
                .unwrap();
            assert!(
                (actual - expected).norm() < 1e-4,
                "{row}/{input}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn capacitor_integral_qpss_coexists_with_behavioral_memory() {
    let f2 = 1e3 * std::f64::consts::SQRT_2;
    let netlist = parse(&format!(
        "Two tone memory\nV1 out 0 SIN(0 0.2 1k)\nV2 ctrl 0 SIN(0 0.2 {f2})\n\
        C1 out 0 C={{100n*(1+0.1*(2*pi*{f2})*SDT(V(ctrl)))}}\n\
        Bobserve observed 0 V={{2*pi*{f2}*SDT(V(ctrl))}}\nR2 observed 0 1k\n.end\n"
    ));
    let config = QpssConfig::new(vec![1e3, f2], vec![2, 2]);
    let point = engine().run_qpss(&netlist, config).unwrap();
    engine()
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    assert_eq!(
        point.integral_names(),
        ["B:BOBSERVE:sdt:0", "C:C1:sdt:0", "C:C1:voltage_rate"]
    );
    let grid = rspice_core::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
        point.config().grid.clone(),
        &Default::default(),
        &NoAbort,
    )
    .unwrap();
    let row = point.node_names().len()
        + point
            .branch_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("V1"))
            .unwrap();
    for (tuple, expected) in [
        ([1, 0], -1.02e-7 * 0.1 * TAU * 1e3),
        ([1, 1], 1e-9 * 0.1 * TAU * 1e3),
        ([1, -1], 1e-9 * 0.1 * TAU * 1e3),
    ] {
        let actual = point.spectra()[row][grid.index_of(&tuple).unwrap()];
        assert!(
            (actual - Complex64::new(expected, 0.)).norm() < 1e-10,
            "{tuple:?}: {actual} vs {expected}"
        );
    }
}
