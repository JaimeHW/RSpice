//! Public BJT intrinsic API, operating-point accessors, and matrix stamping.

#![allow(clippy::needless_range_loop)]

use super::*;

impl Bjt {
    /// Calculate base-emitter junction capacitance
    /// Cbe = CJE / (1 - Vbe/VJE)^MJE + gm * TF
    pub fn cbe(&self, vbe: Value, gm: Value) -> Value {
        let p = self.polarity();
        let (_, factor) = self.vbic_depletion_charge_and_derivative(
            p * vbe,
            self.vje,
            self.mje,
            self.fc,
            self.aje,
        );
        let cj = self.cje * factor;
        let cd = gm * self.tf; // Diffusion capacitance
        cj + cd
    }

    /// Return cached collector, base, and emitter currents at the operating point.
    pub fn operating_point_currents(&self) -> (Value, Value, Value) {
        (self.ic, self.ib, self.ie)
    }

    /// Return the legacy Gummel-Poon collector and base shot-noise currents.
    ///
    /// Xyce's BJT noise model uses the solved total `iC` and `iB` operating-
    /// point currents, including the complete forward, reverse, and leakage
    /// current state. Reconstructing separate junction currents does not
    /// preserve that operating-point contract.
    /// The third entry is retained for the internal caller shape but is zero
    /// because the legacy model injects one total base-current source from B-E.
    pub fn noise_branch_currents(&self) -> (Value, Value, Value) {
        (self.ic.abs(), self.ib.abs(), 0.0)
    }

    /// Return flicker-noise coefficients, if enabled by the model card.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kf > 0.0 && self.kf.is_finite() {
            Some((self.kf, self.af.max(1e-12), self.ef.max(1e-12)))
        } else {
            None
        }
    }

    /// Return the VBIC base-emitter flicker-noise coefficients
    /// `(KFN, AFN, BFN)`, if enabled by the model card.
    ///
    /// vbicnoise.c rides this source on the intrinsic B-E junction current
    /// with the multiplicity folded as `m·KFN·|Ibe/m|^AFN / f^BFN`; the
    /// caller folds `m` (available as `self.m`) into the coefficient.
    pub fn vbic_flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kfn > 0.0 && self.kfn.is_finite() {
            Some((self.kfn, self.afn.max(1e-12), self.bfn.max(1e-12)))
        } else {
            None
        }
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;

        // Collector row
        if c > 0 {
            self.indices.cc = matrix.get_index(c - 1, c - 1);
        }
        if c > 0 && b > 0 {
            self.indices.cb = matrix.get_index(c - 1, b - 1);
        }
        if c > 0 && e > 0 {
            self.indices.ce = matrix.get_index(c - 1, e - 1);
        }
        // Base row
        if b > 0 && c > 0 {
            self.indices.bc = matrix.get_index(b - 1, c - 1);
        }
        if b > 0 {
            self.indices.bb = matrix.get_index(b - 1, b - 1);
        }
        if b > 0 && e > 0 {
            self.indices.be = matrix.get_index(b - 1, e - 1);
        }
        // Emitter row
        if e > 0 && c > 0 {
            self.indices.ec = matrix.get_index(e - 1, c - 1);
        }
        if e > 0 && b > 0 {
            self.indices.eb = matrix.get_index(e - 1, b - 1);
        }
        if e > 0 {
            self.indices.ee = matrix.get_index(e - 1, e - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        if self.vbic_mna_promoted() {
            // The promoted system spans the internal nodes, so it stamps
            // through the position-mapped path instead of the 3x3 linkage.
            struct PromotedStamper<'a> {
                matrix: &'a mut StaticMatrix,
                rhs: &'a mut [Value],
            }
            impl crate::device::MatrixStamper for PromotedStamper<'_> {
                #[inline]
                fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
                    if row > 0 && col > 0 {
                        self.matrix.add(row - 1, col - 1, value);
                    }
                }

                #[inline]
                fn stamp_rhs(&mut self, index: NodeId, value: Value) {
                    if index > 0 && index <= self.rhs.len() {
                        self.rhs[index - 1] += value;
                    }
                }
            }
            let mut stamper = PromotedStamper { matrix, rhs };
            self.stamp_vbic_mna(&mut stamper);
            return;
        }
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let nodes = self.external_terminal_nodes();
        // The current/Jacobian pair is evaluated at the limited junction
        // voltages.  Anchor its affine companion there as well so the direct
        // O(1) stamp includes the limiter tangent correction.
        let anchor = self.companion_anchor(vc, vb, ve, vs);
        let currents = [self.ic, self.ib, self.ie, self.isub];

        let stamp_entry =
            |matrix: &mut StaticMatrix, row_idx: usize, col_idx: usize, value: Value| {
                let row = nodes[row_idx];
                let col = nodes[col_idx];
                if row == 0 || col == 0 {
                    return;
                }

                match (row_idx, col_idx) {
                    (EXT_C, EXT_C) => {
                        if let Some(idx) = self.indices.cc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_B) => {
                        if let Some(idx) = self.indices.cb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_E) => {
                        if let Some(idx) = self.indices.ce {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_C) => {
                        if let Some(idx) = self.indices.bc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_B) => {
                        if let Some(idx) = self.indices.bb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_E) => {
                        if let Some(idx) = self.indices.be {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_C) => {
                        if let Some(idx) = self.indices.ec {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_B) => {
                        if let Some(idx) = self.indices.eb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_E) => {
                        if let Some(idx) = self.indices.ee {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    _ => matrix.add(row - 1, col - 1, value),
                }
            };

        for row_idx in 0..EXTERNAL_DIM {
            let ieq = currents[row_idx]
                - (0..EXTERNAL_DIM)
                    .map(|col_idx| rows[row_idx][col_idx] * anchor[col_idx])
                    .sum::<Value>();
            for col_idx in 0..EXTERNAL_DIM {
                stamp_entry(matrix, row_idx, col_idx, rows[row_idx][col_idx]);
            }
            if nodes[row_idx] > 0 {
                rhs[nodes[row_idx] - 1] -= ieq;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{MatrixStamper, NonlinearDevice};

    struct DenseStamper {
        matrix: Vec<Vec<Value>>,
        rhs: Vec<Value>,
    }

    impl DenseStamper {
        fn new(size: usize) -> Self {
            Self {
                matrix: vec![vec![0.0; size]; size],
                rhs: vec![0.0; size],
            }
        }

        fn residual(&self, solution: &[Value]) -> Vec<Value> {
            self.matrix
                .iter()
                .zip(&self.rhs)
                .map(|(row, rhs)| {
                    row.iter()
                        .zip(solution)
                        .map(|(coefficient, voltage)| coefficient * voltage)
                        .sum::<Value>()
                        - rhs
                })
                .collect()
        }
    }

    impl MatrixStamper for DenseStamper {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            if row > 0 && col > 0 {
                self.matrix[row - 1][col - 1] += value;
            }
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            if index > 0 {
                self.rhs[index - 1] += value;
            }
        }
    }

    fn full_matrix(size: usize) -> StaticMatrix {
        let triplets: Vec<_> = (0..size)
            .flat_map(|row| (0..size).map(move |col| (row, col, 0.0)))
            .collect();
        StaticMatrix::from_triplets(size, size, &triplets).expect("full test matrix")
    }

    fn assert_limited_direct_stamp_matches_generic(mut bjt: Bjt, candidate: [Value; 3]) {
        bjt.update(&[0.0; 3]);
        bjt.update(&candidate);
        assert!(
            bjt.legacy_junction_limited_for_trace(),
            "test bias must exercise legacy junction limiting"
        );

        let mut direct_matrix = full_matrix(3);
        bjt.link(&direct_matrix);
        let mut direct_rhs = vec![0.0; 3];
        bjt.stamp_direct(&mut direct_matrix, &mut direct_rhs, &candidate);

        let mut generic = DenseStamper::new(3);
        bjt.stamp_nonlinear(&candidate, &mut generic, &mut []);

        for probe in [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ] {
            let direct_residual = direct_matrix
                .residual_vector(&probe, &direct_rhs)
                .expect("direct residual");
            let generic_residual = generic.residual(&probe);
            for (direct, expected) in direct_residual.iter().zip(generic_residual) {
                let tolerance = 1e-13 * (1.0 + direct.abs().max(expected.abs()));
                assert!(
                    (direct - expected).abs() <= tolerance,
                    "direct residual {direct:.16e} differs from generic residual {expected:.16e}"
                );
            }
        }
    }

    #[test]
    fn limited_legacy_bjt_direct_stamp_matches_generic_companion_anchor() {
        assert_limited_direct_stamp_matches_generic(
            Bjt::new_npn("qn".to_string(), 1, 2, 3),
            [0.0, 5.0, 0.0],
        );
        assert_limited_direct_stamp_matches_generic(
            Bjt::new_pnp("qp".to_string(), 1, 2, 3),
            [0.0, -5.0, 0.0],
        );
    }

    /// ngspice bjtload.c:253-257 assigns `vbe = tVcrit` on MODEINITJCT for an
    /// instance the deck did not mark OFF, and Xyce's N_DEV_BJT.C reaches the
    /// same state. Neither arm is behind a compatibility mode, so the first
    /// load lands on tVcrit under every dialect. A raw bias below tVcrit is the
    /// discriminator: pnjlim against a tVcrit reference returns it unchanged,
    /// so only the explicit assignment moves the junction there.
    #[test]
    fn legacy_bjt_first_iteration_uses_explicit_vcrit_state_in_every_dialect() {
        for (mut bjt, xyce) in [
            (Bjt::new_npn("qn".to_string(), 1, 2, 3), false),
            (Bjt::new_pnp("qp".to_string(), 1, 2, 3), false),
            (Bjt::new_npn("qnx".to_string(), 1, 2, 3), true),
            (Bjt::new_pnp("qpx".to_string(), 1, 2, 3), true),
        ] {
            bjt.set_xyce_compatibility(xyce);
            let p = bjt.polarity();
            let raw = IntrinsicTerminalState {
                vcx: 3.0 * p,
                vci: 3.0 * p,
                vbx: 0.5 * p,
                vbi: 0.5 * p,
                vei: 0.2 * p,
                vbp: 3.0 * p,
                vsi: 0.0,
                vrth: 0.0,
            };
            let raw_branches = bjt.legacy_nonlinear_branch_voltages([
                raw.vcx, raw.vci, raw.vbx, raw.vbi, raw.vei, raw.vbp, raw.vsi, raw.vrth,
            ]);
            let initialized = bjt.limit_legacy_terminal_state_against_iterate(raw, false);
            let initialized_branches = bjt.legacy_nonlinear_branch_voltages([
                initialized.vcx,
                initialized.vci,
                initialized.vbx,
                initialized.vbi,
                initialized.vei,
                initialized.vbp,
                initialized.vsi,
                initialized.vrth,
            ]);
            let (vt, vcrit, _sub_vcrit) = bjt.legacy_limiting_parameters(0.0);

            assert!(
                raw_branches.vbe < vcrit,
                "the raw bias must sit below tVcrit for pnjlim to be a no-op on it \
                 (xyce={xyce}), raw vbe={:.17e} vcrit={vcrit:.17e}",
                raw_branches.vbe
            );
            assert!(
                (Bjt::limit_junction_voltage(raw_branches.vbe, vcrit, vt, vcrit)
                    - raw_branches.vbe)
                    .abs()
                    <= 1e-14,
                "pnjlim against the tVcrit reference must leave this bias alone (xyce={xyce})"
            );
            assert!((initialized_branches.vbe - vcrit).abs() <= 1e-14);
            assert!((initialized_branches.vbc - raw_branches.vbc).abs() <= 1e-14);

            bjt.initial_off = true;
            let initialized_off = bjt.limit_legacy_terminal_state_against_iterate(raw, false);
            let off_branches = bjt.legacy_nonlinear_branch_voltages([
                initialized_off.vcx,
                initialized_off.vci,
                initialized_off.vbx,
                initialized_off.vbi,
                initialized_off.vei,
                initialized_off.vbp,
                initialized_off.vsi,
                initialized_off.vrth,
            ]);
            assert!(off_branches.vbe.abs() <= 1e-14);
            assert!(off_branches.vbc.abs() <= 1e-14);
        }
    }

    #[test]
    fn repeated_legacy_bjt_update_is_idempotent_for_one_newton_candidate() {
        let mut bjt = Bjt::new_npn("q".to_string(), 1, 2, 3);
        bjt.set_xyce_compatibility(true);
        let candidate = [0.0, 0.0, 0.0];

        bjt.update(&candidate);
        let first_vbe = bjt.vbe;
        let first_vbc = bjt.vbc;
        let first_currents = [bjt.ic, bjt.ib, bjt.ie, bjt.isub];
        bjt.update(&candidate);

        assert_eq!(bjt.vbe.to_bits(), first_vbe.to_bits());
        assert_eq!(bjt.vbc.to_bits(), first_vbc.to_bits());
        assert_eq!(
            [bjt.ic, bjt.ib, bjt.ie, bjt.isub].map(Value::to_bits),
            first_currents.map(Value::to_bits)
        );
    }

    #[test]
    fn repeated_unlimited_legacy_bjt_candidate_advances_convergence_history() {
        // The first load is the MODEINITJCT device state, which replaces the
        // raw junctions and so owns its iterate. From the second on, a bias
        // below tVcrit leaves pnjlim a no-op, and a repeated candidate then has
        // to advance the convergence comparison or a legacy GP device would
        // stay non-converged for as long as the solver keeps handing it the
        // same bias.
        let mut bjt = Bjt::new_npn("q".to_string(), 1, 2, 3);

        bjt.update(&[0.0, 0.0, 0.0]);
        assert!(bjt.legacy_junction_limited_for_trace());

        let candidate = [0.0, 0.2, 0.0];
        bjt.update(&candidate);
        assert!(!bjt.legacy_junction_limited_for_trace());
        bjt.update(&candidate);

        assert!(
            bjt.is_converged(NonlinearConvergenceCriteria::default()),
            "an unchanged cached candidate must advance convergence history"
        );
    }

    #[test]
    fn xyce_voltlim_false_uses_raw_legacy_junction_state_and_preserves_off_seed() {
        let mut raw = Bjt::new_npn("qraw".to_string(), 1, 2, 3)
            .with_params(&std::collections::HashMap::new());
        raw.set_xyce_compatibility(true);
        raw.set_voltage_limiting_enabled(false);
        raw.update(&[0.0, 1.0, 0.0]);
        assert!((raw.vbe - 1.0).abs() <= 1e-14);
        assert!((raw.eval_anchor[EXT_B] - 1.0).abs() <= 1e-14);
        assert!(!raw.legacy_junction_limited_for_trace());

        raw.update(&[0.0, 2.0, 0.0]);
        assert!((raw.vbe - 2.0).abs() <= 1e-14);
        assert!(!raw.legacy_junction_limited_for_trace());

        let mut limited = Bjt::new_npn("qlimited".to_string(), 1, 2, 3)
            .with_params(&std::collections::HashMap::new());
        limited.set_xyce_compatibility(true);
        limited.update(&[0.0, 1.0, 0.0]);
        assert!(limited.vbe < 1.0);
        assert!(limited.legacy_junction_limited_for_trace());

        let mut off = Bjt::new_npn("qoff".to_string(), 1, 2, 3)
            .with_params(&std::collections::HashMap::new());
        off.set_xyce_compatibility(true);
        off.set_voltage_limiting_enabled(false);
        off.initial_off = true;
        off.update(&[0.0, 1.0, 0.0]);
        assert!(off.vbe.abs() <= 1e-14);
        assert!(
            (off.eval_anchor[EXT_B] - off.eval_anchor[EXT_E]).abs() <= 1e-14
                && (off.eval_anchor[EXT_B] - off.eval_anchor[EXT_C]).abs() <= 1e-14,
            "OFF anchor junctions must be zeroed, found {:?}",
            off.eval_anchor
        );
        assert!(!off.legacy_junction_limited_for_trace());
    }

    #[test]
    fn off_instance_starts_from_zero_junction_state_under_every_dialect() {
        // ngspice bjtload.c reaches its `vbe = vbc = 0` MODEINITJCT arm for an
        // OFF instance in every compatibility mode, so the zero-junction start
        // must not depend on the Xyce contract. A first evaluation deep in
        // forward bias is the discriminator: without the explicit branch state
        // pnjlim merely limits toward tVcrit and the instance still stamps as
        // conducting, which is what leaves an OFF keyword unable to pick the
        // branch a bistable operating point settles on.
        for xyce in [false, true] {
            let mut off = Bjt::new_npn("qoff".to_string(), 1, 2, 3)
                .with_params(&std::collections::HashMap::new());
            off.set_xyce_compatibility(xyce);
            off.initial_off = true;
            off.update(&[0.0, 1.0, 0.0]);
            assert!(
                off.vbe.abs() <= 1e-14 && off.vbc.abs() <= 1e-14,
                "OFF junctions must start at zero (xyce={xyce}), found vbe={:.17e} vbc={:.17e}",
                off.vbe,
                off.vbc
            );

            // OFF is a starting hint, not a constraint: once a previous
            // iterate exists the instance must track the bias like any other.
            // Each candidate has to differ, because repeated evaluation of one
            // Newton candidate is deliberately cached and idempotent.
            for bias in [0.9, 0.8, 0.7] {
                off.update(&[0.0, bias, 0.0]);
            }
            assert!(
                off.vbe > 0.0,
                "OFF must not pin the junction across later iterates (xyce={xyce}), found vbe={:.17e}",
                off.vbe
            );
        }

        // A normal instance keeps the tVcrit reference and is untouched by the
        // OFF arm, so only instances carrying the keyword change behaviour.
        let mut on =
            Bjt::new_npn("qon".to_string(), 1, 2, 3).with_params(&std::collections::HashMap::new());
        on.update(&[0.0, 1.0, 0.0]);
        assert!(
            on.vbe > 0.0,
            "a normal instance must stay forward biased, found vbe={:.17e}",
            on.vbe
        );
    }
}
