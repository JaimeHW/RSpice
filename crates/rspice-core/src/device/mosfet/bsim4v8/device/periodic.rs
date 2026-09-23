//! Physical native BSIM4 F/Q samples for periodic collocation.

use super::*;

struct Derivatives<'a, S> {
    matrix: &'a mut S,
    omit_row: NodeId,
}

impl<S: MatrixStamper> MatrixStamper for Derivatives<'_, S> {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row != self.omit_row {
            self.matrix.stamp(row, col, value);
        }
    }
    fn stamp_rhs(&mut self, _: usize, _: Value) {}
}

impl Bsim4v8Device {
    pub(crate) fn periodic_coupling_nodes(&self) -> [NodeId; 12] {
        [
            self.node_drain_external,
            self.node_drain,
            self.node_gate_external,
            self.node_gate_mid,
            self.node_gate,
            self.node_source_external,
            self.node_source,
            self.node_bulk_external,
            self.node_bulk,
            self.node_drain_body,
            self.node_source_body,
            self.node_charge_deficit,
        ]
    }

    fn periodic_op(&self, solution: &[Value]) -> Result<Bsim4v8Op, String> {
        let gate_mid_vgs = (self.core.model.rgate_mod == 3).then(|| {
            self.core.mtype
                * (Self::node_voltage(solution, self.node_gate_mid)
                    - Self::node_voltage(solution, self.node_source))
        });
        self.core.eval_with_junction_and_gate_mid_bias(
            self.raw_branch_voltages(solution),
            self.raw_junction_bias(solution),
            gate_mid_vgs,
            self.gmin,
            true,
        )
    }

    /// Noise laws use this raw orbit snapshot without accepting a transient
    /// step or updating the Newton limiter and integration histories.
    pub(crate) fn update_periodic_noise_probe(&mut self, solution: &[Value]) -> Result<(), String> {
        self.op = self.periodic_op(solution)?;
        self.bias = self.raw_branch_voltages(solution);
        Ok(())
    }

    fn periodic_nqs_rate(&self, charge: &Bsim4v8Charge) -> Result<Value, String> {
        if !self.uses_trnqs() {
            return Ok(0.0);
        }
        let rate = if charge.taunet > 0.0 && charge.taunet.is_finite() {
            TRNQS_SCALING / charge.taunet
        } else {
            charge.gcrg / charge.cox_wl * TRNQS_SCALING
        };
        if self.node_charge_deficit == 0
            || !(charge.cox_wl > 0.0 && charge.cox_wl.is_finite())
            || !(rate >= 0.0 && rate.is_finite())
        {
            return Err(format!("BSIM4 '{}': invalid periodic NQS state", self.name));
        }
        Ok(rate)
    }

    fn periodic_nqs_partition(&self, charge: &Bsim4v8Charge, mode: i32) -> Value {
        let forward = if charge.qchqs.abs() <= 1e-5 * charge.cox_wl {
            if self.core.model.xpart < 0.5 {
                0.4
            } else if self.core.model.xpart > 0.5 {
                0.0
            } else {
                0.5
            }
        } else {
            charge.qdrn / charge.qchqs
        };
        if mode > 0 { forward } else { 1.0 - forward }
    }

    /// Port indices, rather than node identity, preserve separate physical
    /// leads when external circuit terminals are tied. Contribution order is
    /// independent of VDS sign and instantaneous current values.
    fn periodic_current_terms(
        &self,
        op: &Bsim4v8Op,
        solution: &[Value],
        rate: Value,
    ) -> [(usize, usize, Value); 22] {
        let m = self.multiplier;
        let scale = self.core.mtype * m;
        let forward = op.mode > 0;
        let rbody = self.rbody_enabled();
        let db = if rbody { 9 } else { 8 };
        let sb = if rbody { 10 } else { 8 };
        let nodes = self.periodic_coupling_nodes();
        let resistor = |a: usize, b: usize, conductance: Value| {
            m * conductance
                * (Self::node_voltage(solution, nodes[a]) - Self::node_voltage(solution, nodes[b]))
        };
        let rds = self.rds_branch_at(solution, self.raw_branch_voltages(solution));
        let gate = if self.core.model.rgate_mod == 3 { 3 } else { 2 };
        let gate_conductance = if matches!(self.core.model.rgate_mod, 2 | 3) {
            op.gcrg
        } else {
            0.0
        };
        let body = &self.core.inst;
        let body_resistor = |a, b, g| if rbody { resistor(a, b, g) } else { 0.0 };
        let relaxation = scale * self.trnqs_qdef(solution) * rate;
        let partition = self.periodic_nqs_partition(op.charge.as_ref().unwrap(), op.mode);
        [
            (1, 6, scale * if forward { op.cd } else { -op.cd }),
            (sb, 6, scale * op.cbs),
            (db, 1, scale * op.cbd),
            (1, 8, if forward { scale * op.csub } else { 0.0 }),
            (6, 8, if forward { 0.0 } else { scale * op.csub }),
            (1, 8, scale * op.igidl),
            (6, 8, scale * op.igisl),
            (4, 6, scale * op.igs),
            (4, 1, scale * op.igd),
            (4, 6, scale * if forward { op.igcs } else { op.igcd }),
            (4, 1, scale * if forward { op.igcd } else { op.igcs }),
            (4, 8, scale * op.igb),
            (0, 1, resistor(0, 1, rds.gdtot)),
            (5, 6, resistor(5, 6, rds.gstot)),
            (gate, 4, resistor(gate, 4, gate_conductance)),
            (9, 8, body_resistor(9, 8, body.body_prime_drain_conductance)),
            (9, 7, body_resistor(9, 7, body.body_drain_bulk_conductance)),
            (
                10,
                8,
                body_resistor(10, 8, body.body_prime_source_conductance),
            ),
            (
                10,
                7,
                body_resistor(10, 7, body.body_source_bulk_conductance),
            ),
            (8, 7, body_resistor(8, 7, body.body_prime_bulk_conductance)),
            (1, 4, partition * relaxation),
            (6, 4, (1.0 - partition) * relaxation),
        ]
    }

    fn periodic_port_charges(&self, charge: &Bsim4v8Charge, solution: &[Value]) -> [Value; 11] {
        let (gate, middle, bulk, drain, source_body, drain_body) = if self.uses_trnqs() {
            self.trnqs_state_charges_with_probe(charge, solution, true)
        } else {
            (
                charge.qg_state(),
                charge.qgmid_state(),
                charge.qb_state_for_rbody(self.rbody_enabled()),
                charge.qd_state(),
                charge.qbs,
                charge.qbd,
            )
        };
        let (source_body, drain_body) = if self.rbody_enabled() {
            (source_body, drain_body)
        } else {
            (0.0, 0.0)
        };
        let source = -(gate + middle + bulk + drain + source_body + drain_body);
        [
            0.0,
            drain,
            0.0,
            middle,
            gate,
            0.0,
            source,
            0.0,
            bulk,
            drain_body,
            source_body,
        ]
        .map(|charge| self.core.mtype * self.multiplier * charge)
    }

    /// Sum each terminal's internal network. Internal resistor currents cancel
    /// within that terminal, while gate/body storage reaches its external lead.
    pub(crate) fn periodic_lead_fq(
        &self,
        solution: &[Value],
    ) -> Result<([Value; 4], [Value; 4]), String> {
        let op = self.periodic_op(solution)?;
        let charge = op.charge.as_ref().unwrap();
        let rate = self.periodic_nqs_rate(charge)?;
        let leads = [0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 3];
        let mut f = [0.0; 4];
        for (a, b, current) in self.periodic_current_terms(&op, solution, rate) {
            if leads[a] != leads[b] {
                f[leads[a]] += current;
                f[leads[b]] -= current;
            }
        }
        let mut q = [0.0; 4];
        for (lead, charge) in leads
            .into_iter()
            .zip(self.periodic_port_charges(charge, solution))
        {
            q[lead] += charge;
        }
        Ok((f, q))
    }

    /// Stamp -F and -Q directly from physical contributions, together with
    /// native analytic derivatives. Raw probes never advance Newton history.
    pub(crate) fn stamp_periodic_fq(
        &self,
        solution: &[Value],
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        let op = self.periodic_op(solution)?;
        let charge = op.charge.as_ref().unwrap();
        let rate = self.periodic_nqs_rate(charge)?;
        let nodes = self.periodic_coupling_nodes();
        self.stamp_op(
            &op,
            self.raw_branch_voltages(solution),
            self.raw_junction_bias(solution),
            solution,
            &mut Derivatives {
                matrix: f,
                omit_row: self.node_charge_deficit,
            },
        );
        for (a, b, current) in self.periodic_current_terms(&op, solution, rate) {
            f.stamp_rhs(nodes[a], -current);
            f.stamp_rhs(nodes[b], current);
        }
        for (node, charge) in nodes
            .into_iter()
            .zip(self.periodic_port_charges(charge, solution))
        {
            q.stamp_rhs(node, -charge);
        }
        if !self.uses_trnqs() {
            self.stamp_ac_charge_matrix(charge, op.mode, 1.0, q);
            return Ok(());
        }
        // The DC-only hidden-node gmin anchor is replaced by the actual
        // relaxation equation. ag0=0 extracts its static Jacobian.
        self.stamp_trnqs_charge_companion_with_probe(
            charge,
            op.mode,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            solution,
            &mut Derivatives {
                matrix: f,
                omit_row: 0,
            },
            true,
        );
        let scale = self.core.mtype * self.multiplier;
        let qdef = self.trnqs_qdef(solution);
        f.stamp_rhs(self.node_charge_deficit, -scale * qdef * rate);
        q.stamp_rhs(self.node_charge_deficit, -scale * TRNQS_SCALING * qdef);
        q.stamp_rhs(self.node_charge_deficit, scale * charge.qchqs);
        self.stamp_ac_charge_matrix(&Self::trnqs_overlap_charge(charge), op.mode, 1.0, q);
        let gate = -(charge.cggb + charge.cbgb);
        let drain = -(charge.cgdb + charge.cbdb);
        let source = -(charge.cgsb + charge.cbsb);
        let bulk = -(gate + drain + source);
        let (drain, source) = if op.mode > 0 {
            (drain, source)
        } else {
            (source, drain)
        };
        for (node, derivative) in [
            (self.node_drain, -drain),
            (self.node_gate, -gate),
            (self.node_source, -source),
            (self.node_bulk, -bulk),
            (self.node_charge_deficit, TRNQS_SCALING),
        ] {
            stamp(
                q,
                self.node_charge_deficit,
                node,
                self.multiplier * derivative,
            );
        }
        Ok(())
    }
}
