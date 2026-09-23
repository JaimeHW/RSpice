use super::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
struct Sample {
    rhs: [Value; 7],
    jac: [[Value; 7]; 7],
    rows: Vec<usize>,
}
impl MatrixStamper for Sample {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row != 0 && col != 0 {
            self.jac[row - 1][col - 1] += value;
        }
    }
    fn stamp_rhs(&mut self, row: usize, value: Value) {
        if row != 0 {
            self.rhs[row - 1] += value;
            self.rows.push(row);
        }
    }
}

#[test]
fn native_classic_mos_periodic_equations_preserve_current_storage_and_rate_derivatives() {
    for level in [1, 2, 3, 4, 5, 6, 9] {
        for polarity in [1.0, -1.0] {
            for reverse in [false, true] {
                let legacy = matches!(level, 4 | 5);
                let params: HashMap<_, _> = [
                    ("LEVEL", level as Value),
                    ("VTO", polarity * 0.6),
                    ("VFB", -0.8),
                    ("K1", 0.5),
                    ("KP", 50e-6),
                    ("GAMMA", 0.5),
                    ("PHI", 0.7),
                    ("TOX", if legacy { 0.02 } else { 20e-9 }),
                    ("CGSO", 1e-9),
                    ("CGDO", 2e-9),
                    ("CGBO", 0.5e-9),
                    ("CJ", 0.001),
                    ("CJSW", 1e-10),
                    ("MJ", 0.4),
                    ("MJSW", 0.23),
                ]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v))
                .collect();
                let mut device = if polarity > 0.0 {
                    Mosfet::new_nmos("m".into(), 1, 2, 3, 4)
                } else {
                    Mosfet::new_pmos("m".into(), 1, 2, 3, 4)
                }
                .with_params(&params)
                .with_geometry(10e-6, 1e-6);
                device.source_area = 5e-12;
                device.drain_area = 4e-12;
                device.source_perimeter = 22e-6;
                device.drain_perimeter = 20e-6;
                if reverse {
                    device.body_junction_model = MosBodyJunctionModel::XyceClassicLinearizedReverse;
                }
                let rates = (!legacy).then_some([5, 6, 7]);
                let solution = [
                    polarity * if reverse { -0.2 } else { 0.2 },
                    polarity * 1.3,
                    0.0,
                    -polarity * 0.35,
                    0.03,
                    -0.02,
                    0.01,
                ];
                let mut stationary = solution;
                stationary[4..].fill(0.0);
                let sample = |v: &[Value]| {
                    let (mut f, mut q) = (Sample::default(), Sample::default());
                    device
                        .stamp_periodic_physical_fq(v, rates, &mut f, &mut q)
                        .unwrap();
                    (f, q)
                };
                let (f, q) = sample(&solution);
                let (static_f, _) = sample(&stationary);
                // Native static stamp has Norton RHS; periodic F is the full
                // physical current. They must produce the same KCL residual.
                let mut accepted = device.clone();
                accepted.seed_accepted_periodic_bias(&stationary);
                let mut native = Sample::default();
                let mut rhs = [0.0; 7];
                accepted.stamp_nonlinear(&stationary, &mut native, &mut rhs);
                let mut native_c = Sample::default();
                device.stamp_shooting_initial_charge(&solution, &mut native_c, true);
                let (lead_f, lead_q) = device.periodic_terminal_fq(&solution, rates).unwrap();
                for row in 0..4 {
                    let residual: Value = native.jac[row]
                        .iter()
                        .zip(stationary)
                        .map(|(g, v)| g * v)
                        .sum::<Value>()
                        - rhs[row]
                        - native.rhs[row];
                    assert!(
                        (residual + static_f.rhs[row]).abs() < 1e-12,
                        "level={level} p={polarity} reverse={reverse} row={row}: {residual} vs {}",
                        -static_f.rhs[row]
                    );
                    assert!((lead_f[row] + f.rhs[row]).abs() < 1e-16);
                    assert!((lead_q[row] + q.rhs[row]).abs() < 1e-25);
                    for col in 0..4 {
                        let rate_correction = if legacy {
                            0.0
                        } else {
                            [3, 1, 4]
                                .into_iter()
                                .enumerate()
                                .map(|(i, negative)| {
                                    let incidence = Value::from(col + 1 == 2)
                                        - Value::from(col + 1 == negative);
                                    f.jac[row][4 + i] * RATE_TIME_SCALE * incidence
                                })
                                .sum()
                        };
                        assert!(
                            (q.jac[row][col] + rate_correction - native_c.jac[row][col]).abs()
                                < 1e-24,
                            "native C mismatch level={level} row={row} col={col}"
                        );
                    }
                }
                // Differentiate the dynamic F increment separately: legacy
                // channel Jacobians intentionally follow reference conventions.
                for col in 0..7 {
                    let h = 1e-6;
                    let mut plus = solution;
                    plus[col] += h;
                    let mut minus = solution;
                    minus[col] -= h;
                    let (fp, qp) = sample(&plus);
                    let (fm, qm) = sample(&minus);
                    plus[4..].fill(0.0);
                    minus[4..].fill(0.0);
                    let (sp, _) = sample(&plus);
                    let (sm, _) = sample(&minus);
                    assert_eq!(f.rows, fp.rows);
                    assert_eq!(q.rows, qp.rows);
                    for row in 0..7 {
                        let numerical_q = -(qp.rhs[row] - qm.rhs[row]) / (2.0 * h);
                        assert!(
                            (q.jac[row][col] - numerical_q).abs()
                                < (if row < 4 { 1e-23 } else { 1e-18 }) + numerical_q.abs() * 2e-6,
                            "Q derivative level={level} row={row} col={col}"
                        );
                        let numerical_f = -((fp.rhs[row] - sp.rhs[row])
                            - (fm.rhs[row] - sm.rhs[row]))
                            / (2.0 * h);
                        let derivative =
                            f.jac[row][col] - if col < 4 { static_f.jac[row][col] } else { 0.0 };
                        assert!(
                            (derivative - numerical_f).abs() < 1e-12 + numerical_f.abs() * 2e-5,
                            "rate derivative level={level} row={row} col={col}: {derivative} vs {numerical_f}"
                        );
                    }
                }
                let checkpoint = device.nonlinear_state_snapshot();
                let _ = sample(&[0.0; 7]);
                assert_eq!(checkpoint, device.nonlinear_state_snapshot());
            }
        }
    }
}
