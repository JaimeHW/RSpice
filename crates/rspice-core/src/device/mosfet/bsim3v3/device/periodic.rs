//! Unlimited native BSIM3 F/Q samples, including transient NQS storage.

use super::*;

/// Reuse the native analytic Jacobian without its Newton companion sources.
struct JacobianOnly<'a, S>(&'a mut S);

impl<S: MatrixStamper> MatrixStamper for JacobianOnly<'_, S> {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        self.0.stamp(row, col, value);
    }
    fn stamp_rhs(&mut self, _row: usize, _value: Value) {}
}

impl Bsim3v3Device {
    pub(crate) fn periodic_coupling_nodes(&self) -> [NodeId; 5] {
        [
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
            self.node_charge_deficit,
        ]
    }

    fn periodic_op(&self, solution: &[Value]) -> Result<Bsim3v3Op, String> {
        self.core
            .eval(self.raw_branch_voltages(solution), self.gmin, true)
    }

    /// Refresh the native noise-law snapshot at an unlimited orbit sample.
    /// This is not an accepted transient step and does not advance its limiter.
    pub(crate) fn update_periodic_noise_probe(&mut self, solution: &[Value]) -> Result<(), String> {
        self.op = self.periodic_op(solution)?;
        self.bias = self.raw_branch_voltages(solution);
        Ok(())
    }

    /// Drain partition of the relaxing channel current (b3ld.c). At vanishing
    /// channel charge the model's XPART limit replaces the singular quotient.
    fn periodic_nqs_drain_partition(&self, charge: &Bsim3v3Charge, mode: i32) -> Value {
        let forward = if charge.qcheq.abs() <= 1.0e-5 * charge.cox_wl {
            if self.core.model.xpart < 0.5 {
                0.4
            } else if self.core.model.xpart > 0.5 {
                0.0
            } else {
                0.5
            }
        } else {
            charge.qdrn_channel / charge.qcheq
        };
        if mode > 0 { forward } else { 1.0 - forward }
    }

    /// Separate physical current contributions, in fixed terminal order even
    /// when VDS changes sign. This preserves HB's per-contributor tolerances.
    /// Terminal indices are drain, gate, source, bulk.
    fn periodic_current_terms(&self, op: &Bsim3v3Op, solution: &[Value]) -> [(usize, Value); 12] {
        let scale = self.core.mtype * self.multiplier;
        let channel = scale * if op.mode > 0 { op.cd } else { -op.cd };
        let source_junction = scale * op.cbs;
        let drain_junction = scale * op.cbd;
        let substrate = scale * op.csub;
        let (drain_substrate, source_substrate) = if op.mode > 0 {
            (substrate, 0.0)
        } else {
            (0.0, substrate)
        };
        let (relaxation, partition) = if self.uses_trnqs() {
            let charge = op.charge.as_ref().expect("charge-enabled evaluation");
            (
                scale * self.trnqs_qdef(solution) * charge.gtau,
                self.periodic_nqs_drain_partition(charge, op.mode),
            )
        } else {
            (0.0, 0.0)
        };
        [
            (0, channel),
            (2, -channel),
            (3, source_junction),
            (2, -source_junction),
            (3, drain_junction),
            (0, -drain_junction),
            (0, drain_substrate),
            (2, source_substrate),
            (3, -substrate),
            (1, -relaxation),
            (0, partition * relaxation),
            (2, (1.0 - partition) * relaxation),
        ]
    }

    fn periodic_terminal_charges(&self, charge: &Bsim3v3Charge) -> [Value; 4] {
        let scale = self.core.mtype * self.multiplier;
        let (gate, bulk, drain) = self.trnqs_state_charges(charge);
        [drain, gate, -(drain + gate + bulk), bulk].map(|q| scale * q)
    }

    /// Physical terminal currents/charges, keeping distinct leads even when
    /// their circuit nodes coincide. Series resistors remain external owners.
    pub(crate) fn periodic_lead_fq(
        &self,
        solution: &[Value],
    ) -> Result<([Value; 4], [Value; 4]), String> {
        let op = self.periodic_op(solution)?;
        let mut currents = [0.0; 4];
        for (terminal, current) in self.periodic_current_terms(&op, solution) {
            currents[terminal] += current;
        }
        let charges = self.periodic_terminal_charges(op.charge.as_ref().unwrap());
        Ok((currents, charges))
    }

    /// Stamp -F, -Q and positive analytic derivatives at the supplied state.
    /// No junction limiting, OFF startup, or previous-sample history enters a
    /// collocation equation. NQS is the same explicit state used by transient.
    pub(crate) fn stamp_periodic_fq(
        &self,
        solution: &[Value],
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        let op = self.periodic_op(solution)?;
        let charge = op.charge.as_ref().unwrap();
        let nodes = self.periodic_coupling_nodes();
        self.stamp_op(
            &op,
            self.raw_branch_voltages(solution),
            &mut JacobianOnly(f),
        );
        for (terminal, current) in self.periodic_current_terms(&op, solution) {
            f.stamp_rhs(nodes[terminal], -current);
        }
        for (node, charge) in nodes
            .into_iter()
            .zip(self.periodic_terminal_charges(charge))
        {
            q.stamp_rhs(node, -charge);
        }
        if !self.uses_trnqs() {
            self.stamp_charge_matrix(&Self::charge_matrix(charge, op.mode), 1.0, q);
            return Ok(());
        }
        if self.node_charge_deficit == 0
            || !(charge.cox_wl > 0.0 && charge.cox_wl.is_finite())
            || !(charge.gtau > 0.0 && charge.gtau.is_finite())
        {
            return Err(format!("BSIM3 '{}': invalid periodic NQS state", self.name));
        }
        // ag0=0 extracts only the native NQS static Jacobian. Its companion
        // sources are discarded; physical terms above never use J*x-RHS.
        self.stamp_trnqs_charge_companion(
            charge,
            op.mode,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            solution,
            &mut JacobianOnly(f),
        );
        let scale = self.core.mtype * self.multiplier;
        let qdef = self.trnqs_qdef(solution);
        f.stamp_rhs(self.node_charge_deficit, -scale * qdef * charge.gtau);
        // Keep the stored deficit and equilibrium channel charge separate.
        q.stamp_rhs(self.node_charge_deficit, -scale * TRNQS_SCALING * qdef);
        q.stamp_rhs(self.node_charge_deficit, scale * charge.qcheq);

        // NQS terminal Q contains only overlap and junction depletion. The
        // channel charge lives in the explicit deficit equation instead.
        let overlap = Bsim3v3ChargeMatrix {
            gcggb: charge.cgdo + charge.cgso + charge.cgbo,
            gcgdb: -charge.cgdo,
            gcgsb: -charge.cgso,
            gcdgb: -charge.cgdo,
            gcddb: charge.capbd + charge.cgdo,
            gcsgb: -charge.cgso,
            gcssb: charge.capbs + charge.cgso,
            gcbgb: -charge.cgbo,
            gcbdb: -charge.capbd,
            gcbsb: -charge.capbs,
            ..Bsim3v3ChargeMatrix::default()
        };
        self.stamp_charge_matrix(&overlap, 1.0, q);
        let (drain, source) = if op.mode > 0 {
            (charge.cqdb, charge.cqsb)
        } else {
            (charge.cqsb, charge.cqdb)
        };
        for (node, derivative) in
            nodes
                .into_iter()
                .zip([-drain, -charge.cqgb, -source, -charge.cqbb, TRNQS_SCALING])
        {
            q.stamp(self.node_charge_deficit, node, self.multiplier * derivative);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mosfet::bsim3v3::{Bsim3v3Geometry, Bsim3v3Model};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Default)]
    struct Sample {
        rhs: [Value; 5],
        jac: [[Value; 5]; 5],
        terms: Vec<usize>,
    }
    impl MatrixStamper for Sample {
        fn stamp(&mut self, row: usize, col: usize, value: Value) {
            if row > 0 && col > 0 {
                self.jac[row - 1][col - 1] += value;
            }
        }
        fn stamp_rhs(&mut self, row: usize, value: Value) {
            if row > 0 {
                self.rhs[row - 1] += value;
                self.terms.push(row);
            }
        }
    }

    fn device(pmos: bool, capmod: usize, nqs: bool, xpart: Value) -> Bsim3v3Device {
        let params: HashMap<_, _> = [
            ("TOX", 4.1e-9),
            ("VTH0", if pmos { -0.37 } else { 0.37 }),
            ("U0", 270.0),
            ("K1", 0.59),
            ("K2", 0.0026),
            ("CAPMOD", capmod as Value),
            ("NQSMOD", usize::from(nqs) as Value),
            ("XPART", xpart),
            ("CGDO", 7.9e-10),
            ("CGSO", 6.3e-10),
            ("CGBO", 1e-12),
            ("CJ", 9.5e-4),
            ("CJSW", 2.4e-10),
            ("ALPHA0", 0.1),
            ("BETA0", 10.0),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
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
                ..Bsim3v3Geometry::default()
            },
            300.15,
        )
        .unwrap();
        Bsim3v3Device::new("M1".into(), 1, 2, 3, 4, if nqs { 5 } else { 0 }, 2.5, core)
    }

    fn sample(device: &Bsim3v3Device, state: &[Value; 5]) -> (Sample, Sample) {
        let (mut f, mut q) = (Sample::default(), Sample::default());
        device.stamp_periodic_fq(state, &mut f, &mut q).unwrap();
        (f, q)
    }

    #[test]
    fn bsim3_continuation_history_rates_and_limiter_origin_match_native_charge() {
        for pmos in [false, true] {
            for nqs in [false, true] {
                for drain in [0.8, -0.3] {
                    let mut device = device(pmos, 2, nqs, 0.4);
                    let state =
                        [drain, 1.1, 0.1, -0.5, 2e-6].map(|value| value * device.core.mtype);
                    let rates = [0.03, -0.05, 0.07, -0.11, 0.13];
                    let h = 1e-6;
                    let plus = std::array::from_fn::<_, 5, _>(|i| state[i] + h * rates[i]);
                    let minus = std::array::from_fn::<_, 5, _>(|i| state[i] - h * rates[i]);
                    let (charges, current) =
                        device.periodic_history_sample(&state, &rates).unwrap();
                    let (qp, _) = device.periodic_history_sample(&plus, &rates).unwrap();
                    let (qm, _) = device.periodic_history_sample(&minus, &rates).unwrap();
                    for i in 0..5 {
                        let finite_difference = (qp[i] - qm[i]) / (2.0 * h);
                        assert!(
                            (finite_difference - current[i]).abs()
                                < 1e-22 + 3e-6 * current[i].abs(),
                            "pmos={pmos} nqs={nqs} drain={drain} state={i}: {} vs {finite_difference}",
                            current[i]
                        );
                    }
                    device.seed_accepted_periodic_bias(&state);
                    let (native, _) = device.charge_at(&state);
                    // Native inverse-mode limiting reconstructs branch
                    // differences, allowing ordinary subtraction roundoff.
                    for (actual, expected) in
                        [native.qg_state(), native.qb_state(), native.qd_state()]
                            .into_iter()
                            .zip(&charges[..3])
                    {
                        assert!(
                            (actual - expected).abs()
                                <= 32.0 * Value::EPSILON * actual.abs().max(expected.abs())
                        );
                    }
                    let saved = device.accepted_nonlinear_checkpoint().unwrap();
                    assert!(saved.flags[0] && saved.flags[1]);
                    assert!(!saved.flags[2] && !saved.flags[4]);
                    assert_eq!(device.bias, device.converged_ref);
                }
            }
        }
    }

    #[test]
    fn bsim3_periodic_fq_derivatives_conserve_charge_in_both_modes_and_polarities() {
        for pmos in [false, true] {
            for nqs in [false, true] {
                for capmod in 0..=3 {
                    for xpart in [0.4, 0.5, 1.0] {
                        let device = device(pmos, capmod, nqs, xpart);
                        let polarity = device.core.mtype;
                        for drain in [0.8, -0.3] {
                            let state = [drain, 1.1, 0.1, -0.5, 2e-6].map(|v| polarity * v);
                            let (f, q) = sample(&device, &state);
                            for column in 0..5 {
                                let h = if column == 4 { 1e-8 } else { 1e-6 };
                                let mut plus = state;
                                let mut minus = state;
                                plus[column] += h;
                                minus[column] -= h;
                                let (fp, qp) = sample(&device, &plus);
                                let (fm, qm) = sample(&device, &minus);
                                for (label, actual, plus, minus, floor) in
                                    [("F", &f, &fp, &fm, 1e-10), ("Q", &q, &qp, &qm, 2e-19)]
                                {
                                    assert_eq!(actual.terms, plus.terms);
                                    assert_eq!(actual.terms, minus.terms);
                                    for row in 0..5 {
                                        let fd = -(plus.rhs[row] - minus.rhs[row]) / (2.0 * h);
                                        let analytic = actual.jac[row][column];
                                        // ngspice b3ld.c's CAPMOD=3 smoothed
                                        // VdseffCV body derivative is an
                                        // approximation (the T0<0 branch uses
                                        // 1-T5, not T4-T5). Preserve that native
                                        // Jacobian and compare these columns
                                        // relative to their terminal row scale.
                                        let native_ctm =
                                            capmod == 3 && (column == 2 || column == 3);
                                        let row_scale = actual.jac[row][..4]
                                            .iter()
                                            .map(|v| v.abs())
                                            .fold(0.0, Value::max);
                                        let allowance =
                                            if native_ctm { 1e-3 * row_scale } else { 0.0 };
                                        assert!(
                                            (fd - analytic).abs()
                                                <= floor
                                                    + allowance
                                                    + 5e-4 * fd.abs().max(analytic.abs()),
                                            "{label} pmos={pmos} nqs={nqs} capmod={capmod} xpart={xpart} drain={drain} [{row},{column}] fd={fd:e} analytic={analytic:e}"
                                        );
                                    }
                                }
                            }
                            let (currents, charges) = device.periodic_lead_fq(&state).unwrap();
                            for row in 0..4 {
                                assert!((f.rhs[row] + currents[row]).abs() < 1e-14);
                                assert!((q.rhs[row] + charges[row]).abs() < 1e-25);
                            }
                            assert!(currents.iter().sum::<Value>().abs() < 1e-14);
                            assert!(charges.iter().sum::<Value>().abs() < 1e-25);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bsim3_periodic_samples_ignore_newton_limiting_and_off_startup() {
        let mut device = device(false, 3, true, 0.4);
        let state = [0.8, 1.1, 0.1, -0.5, 2e-6];
        let original = sample(&device, &state);
        device.update(&[0.0; 5]);
        device.update(&[20.0, 8.0, 0.0, -2.0, 0.0]);
        let after = sample(&device, &state);
        assert_eq!(original.0.rhs, after.0.rhs);
        assert_eq!(original.1.rhs, after.1.rhs);
        assert_eq!(original.0.jac, after.0.jac);
        assert_eq!(original.1.jac, after.1.jac);
        let reversed = sample(&device, &[-0.3, 1.1, 0.1, -0.5, 2e-6]);
        assert_eq!(original.0.terms, reversed.0.terms);
        assert_eq!(original.1.terms, reversed.1.terms);
    }
}
