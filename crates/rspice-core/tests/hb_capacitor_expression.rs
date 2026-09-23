//! Independent carrier and conversion-matrix oracles for variable capacitance.
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{Engine, QpssConfig, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use rspice_core::{
    Complex64,
    analysis::{HbConfig, PssConfig},
};
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

fn deck(options: &str, ic: &str) -> Netlist {
    parse(&format!(
        "Variable capacitor\nVctrl ctrl 0 SIN(0.2 0.1 1k 0 0 90)\n\
        Bdrive 0 out I={{(0.5+0.2*sin(2*pi*1k*time))/1k+100n*(1+0.1*(0.5+0.2*sin(2*pi*1k*time))+0.2*(0.2+0.1*cos(2*pi*1k*time)))*2*pi*1k*0.2*cos(2*pi*1k*time)}}\n\
        R1 out 0 1k\nC1 out 0 C={{100n*(1+0.1*V(out)+0.2*V(ctrl))}} {ic}\n{options}\n.end\n"
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

// Direct Fourier equations for δi = C(t)*δv' + C_v*v'(t)*δv
// + C_ctrl*v'(t)*δctrl. The derivative frequency belongs to the input column.
fn response(input: i32) -> Vec<Complex64> {
    let mut a = vec![vec![Complex64::ZERO; 6]; 5];
    for k in -2_i32..=2 {
        for m in -2..=2 {
            let c = match k - m {
                0 => Complex64::new(1.09e-7, 0.),
                1 => Complex64::new(1e-9, -1e-9),
                -1 => Complex64::new(1e-9, 1e-9),
                _ => Complex64::ZERO,
            };
            a[(k + 2) as usize][(m + 2) as usize] =
                Complex64::new(if k == m { 1e-3 } else { 0. }, 0.)
                    + Complex64::new(0., TAU * (130. + m as f64 * 1e3)) * c
                    + if (k - m).abs() == 1 {
                        Complex64::new(1e-8 * 0.1 * TAU * 1e3, 0.)
                    } else {
                        Complex64::ZERO
                    };
        }
        if (k - input).abs() == 1 {
            a[(k + 2) as usize][5].re = -2e-8 * 0.1 * TAU * 1e3;
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
fn expression_capacitor_hb_and_retained_pac_match_dynamic_law() {
    for (options, krylov, ic) in [
        ("", false, ""),
        (".options hbint tahb=1", true, ""),
        (".options hbint tahb=1", true, "IC=.5"),
    ] {
        let netlist = deck(options, ic);
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
            .find(|v| v.node_name.eq_ignore_ascii_case("out"))
            .unwrap()
            .coefficients;
        assert!((voltage[0].re - 0.5).abs() < 1e-8);
        assert!((voltage[1] - Complex64::new(0., -0.2)).norm() < 1e-8);
        assert!(voltage[2..].iter().all(|value| value.norm() < 1e-8));
        assert!(
            hb.result
                .mna_branch_currents
                .iter()
                .all(|branch| !branch.device_name.contains("voltage_rate"))
        );
        let capacitor = &hb.result.reactive_spectra[0];
        if !ic.is_empty() {
            let branch = hb
                .result
                .mna_branch_currents
                .iter()
                .find(|row| row.device_name.eq_ignore_ascii_case("C1"))
                .unwrap();
            for (actual, expected) in branch
                .coefficients
                .iter()
                .zip(&capacitor.current_coefficients)
            {
                assert!((actual - expected).norm() < 1e-11);
            }
        }
        assert!((capacitor.current_coefficients[0].re - 2e-9 * 0.1 * TAU * 1e3).abs() < 1e-12);
        assert!((capacitor.current_coefficients[1].re - 1.09e-7 * 0.2 * TAU * 1e3).abs() < 1e-12);
        let result = engine()
            .run_pac_from_hb_with_abort(&netlist, pac(), &hb.operating_point, &NoAbort)
            .unwrap();
        for input in -2..=2 {
            for (row, expected) in response(input).into_iter().enumerate() {
                let actual = result
                    .result
                    .conversion_matrix
                    .get(0, row as i32 - 2, input)
                    .unwrap();
                assert!(
                    (actual - expected).norm() < 1e-8,
                    "{row} {input}: {actual} vs {expected}"
                );
            }
        }
    }
}

#[test]
fn expression_capacitor_retained_shooting_response_matches_dynamic_law() {
    let netlist = deck("", "IC=.5");
    let mut config = PssConfig::new(1e3)
        .with_points_per_period(256)
        .with_tstab_periods(0)
        .with_tolerance(1e-8);
    config.abstol = 1e-12;
    let point = engine()
        .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
        .unwrap();
    let result = engine()
        .run_pac_from_pss_with_abort(&netlist, pac(), &point, &NoAbort)
        .unwrap();
    for input in -2..=2 {
        for (row, expected) in response(input).into_iter().enumerate() {
            let actual = result
                .result
                .conversion_matrix
                .get(0, row as i32 - 2, input)
                .unwrap();
            assert!(
                (actual - expected).norm() < 3e-5,
                "{row} {input}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn expression_capacitor_qpss_preserves_multitone_current() {
    let netlist = parse(
        "Two tone variable capacitor\nV1 out 0 SIN(0 0.2 1k)\nV2 ctrl 0 SIN(0 0.1 1414.213562373095 0 0 90)\nC1 out 0 C={100n*(1+0.3*V(ctrl))}\n.end\n",
    );
    let config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![2, 2]);
    let point = engine().run_qpss(&netlist, config).unwrap();
    let grid = rspice_core::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
        point.config().grid.clone(),
        &Default::default(),
        &NoAbort,
    )
    .unwrap();
    let row = point
        .branch_names()
        .iter()
        .position(|name| name.eq_ignore_ascii_case("V1"))
        .unwrap();
    for (tuple, expected) in [
        ([1, 0], -1e-7 * 0.1 * TAU * 1e3),
        ([1, 1], -1e-7 * 0.015 * 0.1 * TAU * 1e3),
        ([1, -1], -1e-7 * 0.015 * 0.1 * TAU * 1e3),
    ] {
        let actual =
            point.spectra()[point.node_names().len() + row][grid.index_of(&tuple).unwrap()];
        assert!(
            (actual - Complex64::new(expected, 0.)).norm() < 1e-10,
            "{tuple:?}: {actual} vs {expected}"
        );
    }
}

#[test]
fn expression_capacitor_time_law_drives_downstream_integral() {
    let netlist = parse(
        "Timed capacitor with integral observer\n\
        Bdrive 0 input I={sin(2*pi*1k*time)/1k+100n*(1+0.2*cos(2*pi*1k*time))*2*pi*1k*cos(2*pi*1k*time)}\n\
        R1 input 0 1k\nC1 input 0 C={100n*(1+0.2*cos(2*pi*1k*time))}\n\
        Bobserve observed 0 V={2*pi*1k*SDT(V(input))}\nR2 observed 0 1k\n.options hbint tahb=0\n.end\n",
    );
    let hb = engine()
        .run_hb(
            &netlist,
            HbConfig::new(1e3)
                .with_harmonics(3)
                .with_collocation_points(33),
        )
        .unwrap();
    let observed = &hb
        .result
        .spectral_voltages
        .iter()
        .find(|row| row.node_name.eq_ignore_ascii_case("observed"))
        .unwrap()
        .coefficients;
    assert!((observed[0].re - 1.).abs() < 1e-6, "{:?}", observed);
    assert!((observed[1] + 1.).norm() < 1e-6, "{:?}", observed);
}
