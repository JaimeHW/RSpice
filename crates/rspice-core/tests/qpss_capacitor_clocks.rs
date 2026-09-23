//! Independent clock forcing and retained response for time-varying capacitance.
use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicLinearMethod};
use rspice_core::config::{ExpressionDialect, SpiceDialect};
use rspice_core::engine::{QpacRequest, QpssConfig, SimulationConfig};
use rspice_core::netlist::NetlistParseOptions;
use rspice_core::{Complex64, Engine, Netlist, NoAbort};
use std::f64::consts::{SQRT_2, TAU};

fn engine() -> Engine {
    Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
}

fn deck(capacitance: &str, ic: &str) -> Netlist {
    Netlist::parse_with_options(
        &format!(
            "Clocked series RC\nVdrive drive 0 SIN(0 0.2 1k)\n\
             C1 drive out C={{{capacitance}}} {ic}\nR1 out 0 1k\n.end\n"
        ),
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap()
}

fn response(grid: &QuasiPeriodicGrid, offset: f64, drive: &[Complex64]) -> Vec<Complex64> {
    // Independent Fourier KCL: v/R + C(t)*(v'-vin')=0.
    // C(t)=102 nF - 2 nF cos(theta_2); differentiating C*v would
    // use the output frequency here and give a different mixing response.
    let n = grid.len();
    let mut matrix = vec![vec![Complex64::ZERO; n + 1]; n];
    for (row, k) in grid.indices().iter().enumerate() {
        for (col, m) in grid.indices().iter().enumerate() {
            let capacitance = if k[0] != m[0] {
                0.0
            } else {
                match k[1] - m[1] {
                    0 => 102e-9,
                    -1 | 1 => -1e-9,
                    _ => 0.0,
                }
            };
            let admittance = Complex64::new(
                0.0,
                TAU * (offset + grid.frequencies_hz()[col]) * capacitance,
            );
            matrix[row][col] = admittance;
            matrix[row][n] += admittance * drive[col];
        }
        matrix[row][row] += 1e-3;
    }
    for pivot in 0..n {
        let divisor = matrix[pivot][pivot];
        for value in matrix[pivot].iter_mut().skip(pivot) {
            *value /= divisor;
        }
        let pivot_row = matrix[pivot].clone();
        for (row, values) in matrix.iter_mut().enumerate().take(n) {
            if row == pivot {
                continue;
            }
            let factor = values[pivot];
            for (value, &entry) in values.iter_mut().zip(&pivot_row).skip(pivot) {
                *value -= factor * entry;
            }
        }
    }
    matrix.iter().map(|row| row[n]).collect()
}

#[test]
fn qpss_capacitor_clocks_and_nested_memory_match_carrier_and_qpac() {
    let second = 1e3 * SQRT_2;
    for depth in 0..=2 {
        let law = match depth {
            0 => format!("100n*(1.02-0.02*cos(2*pi*{second}*time)+v(0))"),
            1 => format!("100n*(1+0.1*(2*pi*{second})*SDT(0.2*sin(2*pi*{second}*time)))"),
            _ => format!("100n*(1+0.1*(2*pi*{second})^2*SDT(SDT(0.2*cos(2*pi*{second}*time))))"),
        };
        let netlist = deck(&law, if depth == 2 { "IC=.3" } else { "" });
        let mut config = QpssConfig::new(vec![1e3, second], vec![2, 2]);
        config.solver.relative_tolerance = 1e-10;
        config.solver.current_absolute_tolerance = 1e-14;
        if depth == 2 {
            config.solver.linear.method = QuasiPeriodicLinearMethod::Krylov;
        }
        let point = engine().run_qpss(&netlist, config).unwrap();
        let grid = engine()
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert_eq!(point.integral_names().len(), depth + 1);
        let out = point
            .node_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let source = point.node_names().len()
            + point
                .branch_names()
                .iter()
                .position(|name| name.eq_ignore_ascii_case("Vdrive"))
                .unwrap();
        let mut drive = vec![Complex64::ZERO; grid.len()];
        let cap_branch = point
            .branch_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("C1"));
        assert_eq!(cap_branch.is_some(), depth == 2);
        drive[grid.index_of(&[1, 0]).unwrap()] = Complex64::new(0.0, -0.1);
        drive[grid.index_of(&[-1, 0]).unwrap()] = Complex64::new(0.0, 0.1);
        for (index, expected) in response(&grid, 0.0, &drive).into_iter().enumerate() {
            assert!(
                (point.spectra()[out][index] - expected).norm() < 1e-8,
                "depth {depth}, carrier {index}"
            );
            assert!((point.spectra()[source][index] + expected / 1e3).norm() < 1e-11);
            if let Some(branch) = cap_branch {
                assert!(
                    (point.spectra()[point.node_names().len() + branch][index] - expected / 1e3)
                        .norm()
                        < 1e-11
                );
            }
        }
        for input in [[0, 0], [-1, 1]] {
            let request = QpacRequest {
                offsets_hz: vec![130.0],
                input_source: "Vdrive".into(),
                input_lattice: input.to_vec(),
                output_node: "out".into(),
                output_ref: "0".into(),
                output_lattice: vec![input[0], input[1] - 1],
                magnitude: 2.0,
                phase_degrees: 37.0,
                solver: Default::default(),
            };
            let output = grid.index_of(&request.output_lattice).unwrap();
            let result = engine()
                .run_qpac_from_qpss(&netlist, request, &point)
                .unwrap();
            drive.fill(Complex64::ZERO);
            drive[grid.index_of(&input).unwrap()] = Complex64::new(1.0, 0.0);
            let expected = response(&grid, 130.0, &drive);
            for (index, value) in expected.iter().enumerate() {
                assert!(
                    (result.unit_solutions[0].spectra[out][index] - value).norm() < 1e-8,
                    "depth {depth}, QPAC {input:?}/{index}"
                );
            }
            assert!(
                (result.output_response[0]
                    - expected[output] * Complex64::from_polar(2.0, 37_f64.to_radians()))
                .norm()
                    < 2e-8
            );
        }
    }
}

#[test]
fn qpss_capacitor_clocks_reject_nonstationary_and_unregistered_forcing() {
    for (law, reason) in [
        ("100n*(1+time)", "nonperiodic"),
        ("100n*(1+.1*sin(2*pi*1234*time))", "clock"),
        ("100n*(1+SDT(1e-12))", "nonzero"),
    ] {
        let error = engine()
            .run_qpss(
                &deck(law, ""),
                QpssConfig::new(vec![1e3, 1e3 * SQRT_2], vec![2, 2]),
            )
            .unwrap_err()
            .to_string();
        assert!(error.contains(reason), "{error}");
    }
}
