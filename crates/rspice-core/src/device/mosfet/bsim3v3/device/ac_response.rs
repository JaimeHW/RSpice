//! Finite-state realization of the native AC-only NQS small-signal law.
use super::*;

/// The AC equations are a linear response at the supplied carrier bias.
/// Equivalent Newton sources are not part of that response operator.
struct Derivatives<'a, S>(&'a mut S);

impl<S: MatrixStamper> MatrixStamper for Derivatives<'_, S> {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        self.0.stamp(row, col, value);
    }
    fn stamp_rhs(&mut self, _: usize, _: Value) {}
}

impl Bsim3v3Device {
    /// Stamp the complete ACNQSMOD=1 response as G+sC, using three auxiliary
    /// coordinates for filtered channel current and drain/source charge.
    /// The charge coordinates are scaled by 1e-9 C. All node IDs are one-based.
    ///
    /// At fixed bias, eliminating these coordinates gives the native b3acld.c
    /// factor 1/(1+s*taunet). At varying bias each filter obeys
    /// dy/dt = (input-y)/taunet(t), so relaxation is represented continuously
    /// without applying a frozen admittance independently to each sideband.
    /// This operator is for response linearization only; it does not change
    /// carrier equations or advance accepted device state. A caller must
    /// register distinct auxiliary coordinates outside the physical circuit.
    pub fn stamp_ac_nqs_response(
        &self,
        solution: &[Value],
        auxiliary: [NodeId; 3],
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        if !self.uses_ac_nqs() {
            return Err(format!(
                "BSIM3 '{}': AC NQS response requires ACNQSMOD=1",
                self.name
            ));
        }
        let physical = self.periodic_coupling_nodes();
        if auxiliary.iter().enumerate().any(|(index, node)| {
            *node == 0 || physical.contains(node) || auxiliary[..index].contains(node)
        }) {
            return Err(format!(
                "BSIM3 '{}': invalid AC NQS auxiliary coordinates",
                self.name
            ));
        }
        let bias = self.raw_branch_voltages(solution);
        let op = self.core.eval(bias, self.gmin, true)?;
        let charge = op.charge.as_ref().expect("charge-enabled evaluation");
        if !charge.taunet.is_finite() || charge.taunet < 0.0 {
            return Err(format!(
                "BSIM3 '{}': invalid AC NQS relaxation time",
                self.name
            ));
        }
        let rate = if charge.taunet == 0.0 {
            0.0
        } else {
            charge.taunet.recip()
        };
        if !rate.is_finite() {
            return Err(format!(
                "BSIM3 '{}': AC NQS relaxation rate is not representable",
                self.name
            ));
        }

        // ACNQSMOD overrides transient NQS in b3acld.c. Start from the full
        // quasi-static charge Jacobian even when the carrier uses NQSMOD=1.
        self.stamp_op(&op, bias, &mut Derivatives(f));
        self.stamp_charge_matrix(&Self::charge_matrix(charge, op.mode), 1.0, q);
        if self.node_charge_deficit != 0 {
            f.stamp(self.node_charge_deficit, self.node_charge_deficit, 1.0);
        }
        if rate == 0.0 {
            for node in auxiliary {
                f.stamp(node, node, 1.0);
            }
            return Ok(());
        }

        let (gm, gmb, forward, reverse) = if op.mode >= 0 {
            (op.gm, op.gmbs, op.gm + op.gmbs, 0.0)
        } else {
            (-op.gm, -op.gmbs, 0.0, op.gm + op.gmbs)
        };
        let channel = [op.gds + reverse, gm, -(op.gds + forward), gmb];
        let drain = [
            charge.cddb,
            charge.cdgb,
            charge.cdsb,
            -(charge.cddb + charge.cdgb + charge.cdsb),
        ];
        let source = [
            -(charge.cddb + charge.cgdb + charge.cbdb),
            -(charge.cdgb + charge.cggb + charge.cbgb),
            -(charge.cdsb + charge.cgsb + charge.cbsb),
            charge.cddb
                + charge.cgdb
                + charge.cbdb
                + charge.cdgb
                + charge.cggb
                + charge.cbgb
                + charge.cdsb
                + charge.cgsb
                + charge.cbsb,
        ];
        let (drain, source) = if op.mode >= 0 {
            (drain, source)
        } else {
            let swap = |row: [Value; 4]| [row[2], row[1], row[0], row[3]];
            (swap(source), swap(drain))
        };
        let nodes = [
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        ];
        let inputs = [channel, drain, source];
        let scales = [1.0, TRNQS_SCALING, TRNQS_SCALING];
        for (index, node) in auxiliary.into_iter().enumerate() {
            f.stamp(node, node, rate);
            q.stamp(node, node, 1.0);
            for (terminal, derivative) in nodes.into_iter().zip(inputs[index]) {
                let value = self.multiplier * derivative * rate / scales[index];
                if !value.is_finite() {
                    return Err(format!(
                        "BSIM3 '{}': AC NQS input derivative is not representable",
                        self.name
                    ));
                }
                f.stamp(node, terminal, -value);
            }
        }
        // Replace only channel current and intrinsic drain/source charge.
        // Bulk charge, junctions, overlaps and substrate currents stay native.
        for (node, sign) in [(self.node_drain, 1.0), (self.node_source, -1.0)] {
            f.stamp(node, auxiliary[0], sign);
            for (terminal, derivative) in nodes.into_iter().zip(channel) {
                f.stamp(node, terminal, -sign * self.multiplier * derivative);
            }
        }
        for (index, terminal, input) in [(1, self.node_drain, drain), (2, self.node_source, source)]
        {
            for (node, sign) in [(terminal, 1.0), (self.node_gate, -1.0)] {
                q.stamp(node, auxiliary[index], sign * TRNQS_SCALING);
                for (column, derivative) in nodes.into_iter().zip(input) {
                    q.stamp(node, column, -sign * self.multiplier * derivative);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mosfet::bsim3v3::{Bsim3v3Geometry, Bsim3v3Model};
    use std::{collections::HashMap, sync::Arc};

    #[derive(Default)]
    struct Matrix([[Value; 8]; 8]);
    impl MatrixStamper for Matrix {
        fn stamp(&mut self, row: usize, col: usize, value: Value) {
            if row != 0 && col != 0 {
                self.0[row - 1][col - 1] += value;
            }
        }
        fn stamp_rhs(&mut self, _: usize, _: Value) {}
    }

    #[test]
    fn ac_nqs_descriptor_reduces_to_native_frequency_operator() {
        for pmos in [false, true] {
            for capmod in 0..4 {
                for nqs in [false, true] {
                    let mt = if pmos { -1.0 } else { 1.0 };
                    let params: HashMap<_, _> = [
                        ("VTH0", mt * 0.37),
                        ("TOX", 4.1e-9),
                        ("U0", 270.0),
                        ("CAPMOD", capmod as Value),
                        ("ACNQSMOD", 1.0),
                        ("NQSMOD", usize::from(nqs) as Value),
                        ("CGDO", 7.9e-10),
                        ("CGSO", 6.3e-10),
                        ("CGBO", 1e-12),
                        ("CJ", 9.5e-4),
                        ("CJSW", 2.4e-10),
                        ("ALPHA0", 0.1),
                        ("BETA0", 10.0),
                    ]
                    .into_iter()
                    .map(|(key, value)| (key.to_owned(), value))
                    .collect();
                    let model = Arc::new(Bsim3v3Model::from_params(&params, pmos, 300.15));
                    let core = Bsim3v3::new(
                        "M1".into(),
                        model,
                        Bsim3v3Geometry {
                            l: 0.18e-6,
                            w: 10e-6,
                            drain_area: 4.2e-12,
                            source_area: 3.7e-12,
                            drain_perimeter: 20.84e-6,
                            source_perimeter: 18.2e-6,
                            off: true,
                            ..Default::default()
                        },
                        325.0,
                    )
                    .unwrap();
                    let device = Bsim3v3Device::new(
                        "M1".into(),
                        1,
                        2,
                        3,
                        4,
                        if nqs { 5 } else { 0 },
                        2.5,
                        core,
                    );
                    for drain in [0.8, -0.3] {
                        let solution = [drain, 1.1, 0.1, -0.5, 2e-6].map(|value| mt * value);
                        let before = device.accepted_nonlinear_checkpoint();
                        let (mut f, mut q) = (Matrix::default(), Matrix::default());
                        device
                            .stamp_ac_nqs_response(&solution, [6, 7, 8], &mut f, &mut q)
                            .unwrap();
                        assert_eq!(device.accepted_nonlinear_checkpoint(), before);
                        let mut reference = device.clone();
                        reference.seed_accepted_periodic_bias(&solution);
                        let (charge, mode) = reference.charge_at_with_probe(&solution, true);
                        let (mut g, mut c) = (Matrix::default(), Matrix::default());
                        reference.stamp_static_probe(&solution, &mut g);
                        reference.stamp_charge_matrix(
                            &Bsim3v3Device::charge_matrix(&charge, mode),
                            1.0,
                            &mut c,
                        );
                        for omega in [0.0, 1e7, 1e12, -1e12] {
                            let mut expected = std::array::from_fn::<_, 8, _>(|row| {
                                std::array::from_fn::<_, 8, _>(|col| {
                                    Complex64::new(g.0[row][col], omega * c.0[row][col])
                                })
                            });
                            reference.stamp_ac_nqs_correction(
                                &charge,
                                mode,
                                omega,
                                |row, col, value| {
                                    if row != 0 && col != 0 {
                                        expected[row - 1][col - 1] += value;
                                    }
                                },
                            );
                            let entry = |row: usize, col: usize| {
                                Complex64::new(f.0[row][col], omega * q.0[row][col])
                            };
                            for (row, expected_row) in expected.iter().enumerate().take(4) {
                                for (col, &target) in expected_row.iter().enumerate().take(4) {
                                    let mut actual = entry(row, col);
                                    for aux in 5..8 {
                                        actual -=
                                            entry(row, aux) * entry(aux, col) / entry(aux, aux);
                                    }
                                    assert!(
                                        (actual - target).norm() < 1e-12 + 1e-9 * target.norm(),
                                        "pmos={pmos} capmod={capmod} nqs={nqs} drain={drain} omega={omega} row={row} col={col}: {actual} vs {target}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
