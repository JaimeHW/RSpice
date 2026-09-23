//! Conservative carrier-to-transient history for native BSIM3.
use super::*;

impl Bsim3v3Device {
    /// The NQS storage coordinate is Q/(M*1e-9), not the raw deficit unknown.
    /// Holding it fixed still permits algebraic terminal voltages to change.
    /// Derivatives follow periodic_coupling_nodes, including the hidden node.
    pub(crate) fn shooting_nqs_state(&self, solution: &[Value]) -> (Value, [Value; 5]) {
        let op = self
            .core
            .eval(self.raw_branch_voltages(solution), self.gmin, true)
            .expect("validated BSIM3 charge model");
        let charge = op.charge.as_ref().unwrap();
        let (drain, source) = if op.mode > 0 {
            (charge.cqdb, charge.cqsb)
        } else {
            (charge.cqsb, charge.cqdb)
        };
        (
            Self::node_voltage(solution, self.node_charge_deficit)
                - self.core.mtype * charge.qcheq / TRNQS_SCALING,
            [
                -drain / TRNQS_SCALING,
                -charge.cqgb / TRNQS_SCALING,
                -source / TRNQS_SCALING,
                -charge.cqbb / TRNQS_SCALING,
                1.0,
            ],
        )
    }

    pub(crate) fn shooting_nqs_seed(&self, state: Value, solution: &[Value]) -> Value {
        let (current, _) = self.shooting_nqs_state(solution);
        Self::node_voltage(solution, self.node_charge_deficit) + state - current
    }

    /// Initial consistency uses dQ/dt in shooting coordinates. For NQS only
    /// terminal overlap/junction charge remains, and the hidden row is exactly
    /// M*1e-9*w'. Its equilibrium-charge coupling belongs to the nonlinear
    /// storage constraint, not a second terminal charge state.
    pub(crate) fn stamp_shooting_initial_charge(
        &self,
        solution: &[Value],
        matrix: &mut impl MatrixStamper,
        physical_probe: bool,
    ) {
        let (charge, mode) = self.charge_at_with_probe(solution, physical_probe);
        if !self.uses_trnqs() {
            self.stamp_charge_matrix(&Self::charge_matrix(&charge, mode), 1.0, matrix);
            return;
        }
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
            ..Default::default()
        };
        self.stamp_charge_matrix(&overlap, 1.0, matrix);
        matrix.stamp(
            self.node_charge_deficit,
            self.node_charge_deficit,
            self.multiplier * TRNQS_SCALING,
        );
    }

    pub(crate) fn stamp_shooting_initial_relaxation(
        &self,
        solution: &[Value],
        matrix: &mut impl MatrixStamper,
        physical_probe: bool,
    ) {
        if self.uses_trnqs() {
            let (charge, mode) = self.charge_at_with_probe(solution, physical_probe);
            self.stamp_trnqs_charge_companion_with_probe(
                &charge,
                mode,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                solution,
                matrix,
                physical_probe,
            );
        }
    }

    /// Voltage dependencies of the terminal storage law. These are coordinate
    /// edges, not a decomposition into reciprocal two-terminal capacitors.
    /// Suppressed intrinsic charge and zero overlap/junction parameters must
    /// not create artificial shooting states.
    pub(crate) fn shooting_terminal_storage_nodes(&self) -> [Option<(NodeId, NodeId)>; 5] {
        let p = &self.core.size;
        let intrinsic = self.core.model.xpart >= 0.0 && !self.uses_trnqs();
        let overlap_extension = self.core.model.cap_mod != 0;
        let (bd, bdsw, bdswg, bs, bssw, bsswg) = eval::junction_zero_bias_caps_acm0_or_1(
            &self.core.model,
            &self.core.model_temp,
            p,
            &self.core.inst,
        );
        let active = [
            intrinsic || p.cgdo != 0.0 || (overlap_extension && p.cgdl != 0.0),
            intrinsic || p.cgso != 0.0 || (overlap_extension && p.cgsl != 0.0),
            intrinsic || p.cgbo != 0.0,
            bd != 0.0 || bdsw != 0.0 || bdswg != 0.0,
            bs != 0.0 || bssw != 0.0 || bsswg != 0.0,
        ];
        let nodes = [
            (self.node_gate, self.node_drain),
            (self.node_gate, self.node_source),
            (self.node_gate, self.node_bulk),
            (self.node_drain, self.node_bulk),
            (self.node_source, self.node_bulk),
        ];
        std::array::from_fn(|index| {
            (active[index] && nodes[index].0 != nodes[index].1).then_some(nodes[index])
        })
    }

    /// Per-instance, device-polarity CKTstate charges and their physical time
    /// derivatives. NQS keeps equilibrium channel charge and stored deficit
    /// separate, just as the native transient integration does.
    pub(crate) fn periodic_history_sample(
        &self,
        solution: &[Value],
        node_rates: &[Value],
    ) -> Result<([Value; 5], [Value; 5]), String> {
        let op = self
            .core
            .eval(self.raw_branch_voltages(solution), self.gmin, true)?;
        let charge = op.charge.as_ref().expect("charge-enabled BSIM3 evaluation");
        let rate = |node| Self::node_voltage(node_rates, node);
        let (dg, dd, ds, db) = (
            rate(self.node_gate),
            rate(self.node_drain),
            rate(self.node_source),
            rate(self.node_bulk),
        );
        let mt = self.core.mtype;
        let (qg, qb, qd) = self.trnqs_state_charges(charge);
        let (cg, cb, cd, qcheq, qcdump, cqcheq, cqcdump) = if self.uses_trnqs() {
            let (cqdb, cqsb) = if op.mode > 0 {
                (charge.cqdb, charge.cqsb)
            } else {
                (charge.cqsb, charge.cqdb)
            };
            (
                mt * (charge.cgdo * (dg - dd) + charge.cgso * (dg - ds) + charge.cgbo * (dg - db)),
                mt * (-charge.cgbo * (dg - db)
                    - charge.capbd * (dd - db)
                    - charge.capbs * (ds - db)),
                mt * (-charge.cgdo * (dg - dd) + charge.capbd * (dd - db)),
                charge.qcheq,
                self.trnqs_qcdump_state(solution),
                mt * (charge.cqgb * dg + cqdb * dd + cqsb * ds + charge.cqbb * db),
                self.trnqs_qcdump_state(node_rates),
            )
        } else {
            let c = Self::charge_matrix(charge, op.mode);
            let flow = |g, d, s| mt * (g * (dg - db) + d * (dd - db) + s * (ds - db));
            (
                flow(c.gcggb, c.gcgdb, c.gcgsb),
                flow(c.gcbgb, c.gcbdb, c.gcbsb),
                flow(c.gcdgb, c.gcddb, c.gcdsb),
                0.0,
                0.0,
                0.0,
                0.0,
            )
        };
        let charges = [qg, qb, qd, qcheq, qcdump];
        let currents = [cg, cb, cd, cqcheq, cqcdump];
        if charges
            .iter()
            .chain(&currents)
            .any(|value| !value.is_finite())
        {
            return Err(format!(
                "BSIM3 '{}' has non-finite periodic charge history",
                self.name
            ));
        }
        Ok((charges, currents))
    }

    /// The solved periodic origin is already an accepted operating point:
    /// retire OFF startup and seed the native limiter without clipping it.
    pub(crate) fn seed_accepted_periodic_bias(&mut self, solution: &[Value]) {
        let bias = self.raw_branch_voltages(solution);
        self.op = eval::eval_dc(
            &self.core.model,
            &self.core.model_temp,
            &self.core.size,
            &self.core.inst,
            bias,
            self.gmin,
        );
        self.bias = bias;
        self.converged_ref = bias;
        self.von_prev = self.op.von;
        self.has_history = true;
        self.limit_anchor_valid.set(true);
        self.last_limited.set(false);
        self.initial_off_seed_pending = false;
        self.initial_off_seed_evaluations = 0;
        self.initial_off_seed_raw = None;
    }
}
