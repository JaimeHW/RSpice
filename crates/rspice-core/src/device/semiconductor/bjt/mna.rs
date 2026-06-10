//! MNA promotion of the VBIC internal, thermal, and excess-phase states.
//!
//! ngspice solves the VBIC network by making every internal state a matrix
//! unknown (vbicsetup.c); RSpice mirrors that topology here. The circuit
//! builder allocates the internal nodes through [`Bjt::assign_vbic_internal_nodes`],
//! aliasing collapsed states onto their parent nodes exactly like ngspice
//! collapses zero-resistance branches, and the device then participates in
//! the global Newton iteration through [`Bjt::update_vbic_mna`] and
//! [`Bjt::stamp_vbic_mna`] instead of nesting a private solver inside each
//! evaluation.
//!
//! Sign conventions: the intrinsic residual rows built by
//! `internal_kcl_linearization_from_eval` are oriented as "current in minus
//! current out" for the CX..SI rows and "sink minus power" for the thermal
//! row. MNA rows must accumulate currents *leaving* each node so that shared
//! rows (aliased nodes, engine gmin shunts) compose correctly, so the CX..SI
//! rows are negated when stamped while the thermal and excess-phase rows
//! already match the MNA orientation.

use super::*;

impl Bjt {
    /// True once the builder has promoted this instance's VBIC states to MNA
    /// unknowns.
    #[inline]
    pub(crate) fn vbic_mna_promoted(&self) -> bool {
        self.vbic_mna_promoted
    }

    /// True when the parasitic (bp) state carries its own KCL row; mirrors the
    /// `solve_vbp` condition of the intrinsic residual so the promoted
    /// topology and the reduced solve collapse the same states.
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_solves_vbp(&self) -> bool {
        Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0
    }

    /// Allocate the VBIC internal nodes per ngspice's collapse rules
    /// (vbicsetup.c:400-525). `alloc` receives a short state suffix and must
    /// return a fresh circuit node. Collapsed states alias their parent node
    /// so each retains exactly one matrix column; disabled states (thermal
    /// without self-heating, excess phase without TD) stay at ground and all
    /// of their stamps drop.
    pub fn assign_vbic_internal_nodes(&mut self, mut alloc: impl FnMut(&str) -> NodeId) {
        debug_assert!(self.uses_vbic_dynamic_charges());
        self.node_cx = if Self::series_active(self.rcx) {
            alloc("cx")
        } else {
            self.node_collector
        };
        self.node_ci = if Self::series_active(self.rci) {
            alloc("ci")
        } else {
            self.node_cx
        };
        self.node_bx = if Self::series_active(self.rbx) {
            alloc("bx")
        } else {
            self.node_base
        };
        self.node_bi = if Self::series_active(self.rbi) {
            alloc("bi")
        } else {
            self.node_bx
        };
        self.node_ei = if Self::series_active(self.re) {
            alloc("ei")
        } else {
            self.node_emitter
        };
        self.node_bp = if self.vbic_solves_vbp() {
            alloc("bp")
        } else {
            self.node_cx
        };
        self.node_si = if Self::series_active(self.rs) {
            alloc("si")
        } else {
            self.node_substrate
        };
        self.node_rth = if self.self_heating_enabled() {
            alloc("rth")
        } else {
            0
        };
        if self.td > 0.0 {
            self.node_xf1 = alloc("xf1");
            self.node_xf2 = alloc("xf2");
        } else {
            self.node_xf1 = 0;
            self.node_xf2 = 0;
        }
        self.vbic_mna_promoted = true;
    }

    /// Matrix node for a dynamic internal state index (collapsed states alias
    /// their parent node; disabled states map to ground).
    #[inline]
    pub(crate) fn vbic_internal_node(&self, idx: usize) -> NodeId {
        match idx {
            IDX_VCX => self.node_cx,
            IDX_VCI => self.node_ci,
            IDX_VBX => self.node_bx,
            IDX_VBI => self.node_bi,
            IDX_VEI => self.node_ei,
            IDX_VBP => self.node_bp,
            IDX_VSI => self.node_si,
            IDX_VRTH => self.node_rth,
            IDX_VXF1 => self.node_xf1,
            IDX_VXF2 => self.node_xf2,
            _ => 0,
        }
    }

    /// Whether internal state `idx` owns its own KCL row. Collapsed states are
    /// aliased onto a parent node whose row already carries their branch
    /// currents (via `external_terminal_branches` and the active rows), so
    /// they must not be stamped separately.
    #[inline]
    fn vbic_internal_row_active(&self, idx: usize) -> bool {
        match idx {
            IDX_VCX => Self::series_active(self.rcx),
            IDX_VCI => Self::series_active(self.rci),
            IDX_VBX => Self::series_active(self.rbx),
            IDX_VBI => Self::series_active(self.rbi),
            IDX_VEI => Self::series_active(self.re),
            IDX_VBP => self.vbic_solves_vbp(),
            IDX_VSI => Self::series_active(self.rs),
            IDX_VRTH => self.self_heating_enabled(),
            _ => false,
        }
    }

    /// Residual-row orientation relative to the MNA "currents leaving the
    /// node" convention: the CX..SI residual rows are current-in minus
    /// current-out (flip), while the thermal and excess-phase rows already
    /// accumulate leaving terms.
    #[inline]
    fn vbic_residual_row_sign(idx: usize) -> Value {
        if idx < IDX_VRTH { -1.0 } else { 1.0 }
    }

    /// All matrix nodes this device couples once promoted, for sparsity
    /// reservations. Zero entries are ground/disabled and must be skipped.
    pub(crate) fn vbic_mna_coupling_nodes(&self) -> [NodeId; EXTERNAL_DIM + DYNAMIC_INTERNAL_DIM] {
        [
            self.node_collector,
            self.node_base,
            self.node_emitter,
            self.node_substrate,
            self.node_cx,
            self.node_ci,
            self.node_bx,
            self.node_bi,
            self.node_ei,
            self.node_bp,
            self.node_si,
            self.node_rth,
            self.node_xf1,
            self.node_xf2,
        ]
    }

    /// Internal state (including excess phase) at the current linearization
    /// point, in dynamic-state index order.
    #[inline]
    pub(crate) fn vbic_mna_internal_state(&self) -> [Value; BJT_INTERNAL_STATE_DIM] {
        [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
            self.vxf1, self.vxf2,
        ]
    }

    /// External terminal voltages at the current linearization point.
    #[inline]
    pub(crate) fn vbic_mna_external_state(&self) -> [Value; EXTERNAL_DIM] {
        [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext]
    }

    /// Re-impose the collapse manifold after junction limiting: the
    /// least-squares projection may split aliased states, but aliased states
    /// share one matrix column, so the linearization point must keep them
    /// identical (mirroring the alias fixups of the reduced residual).
    fn impose_vbic_collapse_manifold(
        &self,
        state: &mut [Value; INTERNAL_DIM],
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) {
        if !Self::series_active(self.rcx) {
            state[IDX_VCX] = vc;
        }
        if !Self::series_active(self.rci) {
            state[IDX_VCI] = state[IDX_VCX];
        }
        if !Self::series_active(self.rbx) {
            state[IDX_VBX] = vb;
        }
        if !Self::series_active(self.rbi) {
            state[IDX_VBI] = state[IDX_VBX];
        }
        if !Self::series_active(self.re) {
            state[IDX_VEI] = ve;
        }
        if !self.vbic_solves_vbp() {
            state[IDX_VBP] = state[IDX_VCX];
        }
        if !Self::series_active(self.rs) {
            state[IDX_VSI] = vs;
        }
        if !self.self_heating_enabled() {
            state[IDX_VRTH] = 0.0;
        }
    }

    /// Per-iteration update for the promoted device: read the internal node
    /// voltages from the global solution, apply ngspice junction limiting
    /// against the previous iterate (vbicload.c:656-670), and evaluate the
    /// branch system once at the limited bias.
    pub(super) fn update_vbic_mna(&mut self, voltages: &[Value]) {
        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;
        self.vcx_prev = self.vcx;
        self.vbi_prev = self.vbi;
        self.vci_prev = self.vci;
        self.vbx_prev = self.vbx;
        self.vei_prev = self.vei;
        self.vbp_prev = self.vbp;
        self.vsi_prev = self.vsi;
        self.vrth_prev = self.vrth;
        self.ic_prev = self.ic;
        self.ib_prev = self.ib;
        self.ie_prev = self.ie;
        self.isub_prev = self.isub;
        self.intrinsic_linearization_prev = self.intrinsic_linearization;

        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let raw = [
            Self::node_voltage(voltages, self.node_cx),
            Self::node_voltage(voltages, self.node_ci),
            Self::node_voltage(voltages, self.node_bx),
            Self::node_voltage(voltages, self.node_bi),
            Self::node_voltage(voltages, self.node_ei),
            Self::node_voltage(voltages, self.node_bp),
            Self::node_voltage(voltages, self.node_si),
            Self::node_voltage(voltages, self.node_rth),
        ];
        let previous = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        let mut state = self.limit_vbic_internal_state_to_previous(raw, previous);
        self.impose_vbic_collapse_manifold(&mut state, vc, vb, ve, vs);

        let eval = self.evaluate_state(
            vc,
            vb,
            ve,
            vs,
            state[IDX_VCX],
            state[IDX_VCI],
            state[IDX_VBX],
            state[IDX_VBI],
            state[IDX_VEI],
            state[IDX_VBP],
            state[IDX_VSI],
            state[IDX_VRTH],
        );
        let terminal_currents = self.external_terminal_branches(eval);

        self.vc_ext = vc;
        self.vb_ext = vb;
        self.ve_ext = ve;
        self.vs_ext = vs;
        self.vcx = state[IDX_VCX];
        self.vci = state[IDX_VCI];
        self.vbx = state[IDX_VBX];
        self.vbi = state[IDX_VBI];
        self.vei = state[IDX_VEI];
        self.vbp = state[IDX_VBP];
        self.vsi = state[IDX_VSI];
        self.vrth = state[IDX_VRTH];
        self.vxf1 = Self::node_voltage(voltages, self.node_xf1);
        self.vxf2 = Self::node_voltage(voltages, self.node_xf2);
        self.vbe = self.vbi - self.vei;
        self.vbc = self.vbi - self.vci;
        self.ic = terminal_currents[EXT_C].current;
        self.ib = terminal_currents[EXT_B].current;
        self.ie = terminal_currents[EXT_E].current;
        self.isub = terminal_currents[EXT_S].current;
        self.intrinsic_linearization = eval.linearized;
        self.mna_eval = Some(eval);
        self.refresh_vbic_mna_dynamic_state();
        self.reduced_linearization_cache_valid.set(false);
        self.charge_snapshot_cache_valid.set(false);
    }

    /// Recompute the dynamic charge branches and excess-phase rows at the
    /// limited bias just written by `update_vbic_mna`.
    fn refresh_vbic_mna_dynamic_state(&mut self) {
        let internal = self.vbic_mna_internal_state();
        let external = self.vbic_mna_external_state();
        let (branches, inputs, d_itzf_d_vrth) =
            self.vbic_dynamic_charge_state_at_bias(external, internal, None);
        self.mna_charge_cache.set(branches);
        self.mna_charge_cache_valid.set(true);

        if self.td > 0.0 {
            let mut reduction = BjtDynamicReduction::default();
            reduction.internal_voltages = internal;
            reduction.external_voltages = external;
            reduction.vbic_transport = inputs.transport;
            reduction.vbic_d_itzf_d_vrth = d_itzf_d_vrth;
            self.mna_delay_branches = self.vbic_delay_static_branches(&reduction);
            self.mna_delay_thermal = self.vbic_delay_static_thermal_branch(&reduction);
        } else {
            self.mna_delay_branches = [BjtCurrentBranch::default(); 3];
            self.mna_delay_thermal = BjtCurrentBranch::default();
        }
    }

    /// Dynamic charge branches plus their linearization-point voltages, for
    /// the engine's transient companion and AC passes. Valid after `update`.
    pub(crate) fn vbic_mna_charge_state(
        &self,
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        [Value; BJT_INTERNAL_STATE_DIM],
        [Value; EXTERNAL_DIM],
    ) {
        let internal = self.vbic_mna_internal_state();
        let external = self.vbic_mna_external_state();
        if !self.mna_charge_cache_valid.get() {
            let (branches, _, _) = self.vbic_dynamic_charge_state_at_bias(external, internal, None);
            self.mna_charge_cache.set(branches);
            self.mna_charge_cache_valid.set(true);
        }
        (self.mna_charge_cache.get(), internal, external)
    }

    /// Charge branches evaluated directly at a solution vector (history
    /// initialization, accepted-step commit, and LTE candidate paths). The
    /// bias is read raw: aliased states share their parent solution entry,
    /// and accepted/candidate points are evaluated where they stand rather
    /// than at a limited iterate.
    pub(crate) fn vbic_mna_charge_state_at_solution(
        &self,
        voltages: &[Value],
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        [Value; BJT_INTERNAL_STATE_DIM],
        [Value; EXTERNAL_DIM],
    ) {
        let external = self.external_terminal_voltages(voltages);
        let internal = [
            Self::node_voltage(voltages, self.node_cx),
            Self::node_voltage(voltages, self.node_ci),
            Self::node_voltage(voltages, self.node_bx),
            Self::node_voltage(voltages, self.node_bi),
            Self::node_voltage(voltages, self.node_ei),
            Self::node_voltage(voltages, self.node_bp),
            Self::node_voltage(voltages, self.node_si),
            Self::node_voltage(voltages, self.node_rth),
            Self::node_voltage(voltages, self.node_xf1),
            Self::node_voltage(voltages, self.node_xf2),
        ];
        let (branches, _, _) = self.vbic_dynamic_charge_state_at_bias(external, internal, None);
        (branches, internal, external)
    }

    /// Stamp the full promoted static system: the four terminal KCL rows, the
    /// active internal KCL rows, and the excess-phase algebraic rows, all
    /// linearized at the limited bias from the last `update`.
    pub(in crate::device::semiconductor::bjt) fn stamp_vbic_mna(
        &self,
        stamper: &mut impl MatrixStamper,
    ) {
        let Some(eval) = self.mna_eval else {
            return;
        };
        let state = IntrinsicTerminalState {
            vcx: self.vcx,
            vci: self.vci,
            vbx: self.vbx,
            vbi: self.vbi,
            vei: self.vei,
            vbp: self.vbp,
            vsi: self.vsi,
            vrth: self.vrth,
        };
        let [vc, vb, ve, vs] = self.vbic_mna_external_state();
        let internal = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let internal_nodes: [NodeId; INTERNAL_DIM] = [
            self.node_cx,
            self.node_ci,
            self.node_bx,
            self.node_bi,
            self.node_ei,
            self.node_bp,
            self.node_si,
            self.node_rth,
        ];
        let external_nodes = self.external_terminal_nodes();

        // Active internal KCL rows (residual orientation, flipped onto the
        // MNA leaving-current convention).
        let (g_ii, g_ie, z_i) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        for row in 0..INTERNAL_DIM {
            if !self.vbic_internal_row_active(row) {
                continue;
            }
            let sign = Self::vbic_residual_row_sign(row);
            let row_node = internal_nodes[row];
            for col in 0..INTERNAL_DIM {
                if g_ii[row][col] != 0.0 {
                    stamper.stamp(row_node, internal_nodes[col], sign * g_ii[row][col]);
                }
            }
            for col in 0..EXTERNAL_DIM {
                if g_ie[row][col] != 0.0 {
                    stamper.stamp(row_node, external_nodes[col], sign * g_ie[row][col]);
                }
            }
            stamper.stamp_rhs(row_node, sign * z_i[row]);
        }

        // External terminal rows: current into the device from each terminal,
        // with collapse-aware branch selection.
        let terminal_currents = self.external_terminal_branches(eval);
        for row in 0..EXTERNAL_DIM {
            let row_node = external_nodes[row];
            let branch = terminal_currents[row];
            let mut source = -branch.current;
            for col in 0..INTERNAL_DIM {
                if branch.d_internal[col] != 0.0 {
                    stamper.stamp(row_node, internal_nodes[col], branch.d_internal[col]);
                    source += branch.d_internal[col] * internal[col];
                }
            }
            for col in 0..EXTERNAL_DIM {
                if branch.d_external[col] != 0.0 {
                    stamper.stamp(row_node, external_nodes[col], branch.d_external[col]);
                    source += branch.d_external[col] * external[col];
                }
            }
            stamper.stamp_rhs(row_node, source);
        }

        // Excess-phase network: algebraic xf rows plus the xf2-controlled
        // transport replacement (and its thermal power correction).
        if self.td > 0.0 {
            for branch in &self.mna_delay_branches {
                self.stamp_vbic_residual_branch(stamper, branch);
            }
            if self.self_heating_enabled() {
                self.stamp_vbic_residual_branch(stamper, &self.mna_delay_thermal);
            }
        }
    }

    /// Stamp one residual-convention current branch onto the promoted rows,
    /// applying the per-row orientation flip.
    fn stamp_vbic_residual_branch(
        &self,
        stamper: &mut impl MatrixStamper,
        branch: &BjtCurrentBranch,
    ) {
        if !branch.is_active() {
            return;
        }
        let internal = self.vbic_mna_internal_state();
        let external = self.vbic_mna_external_state();
        let external_nodes = self.external_terminal_nodes();
        let source = branch.linearization_dot(&internal, &external) - branch.current;

        let mut stamp_side = |row_node: NodeId, sign: Value| {
            if row_node == 0 {
                return;
            }
            for col in 0..BJT_INTERNAL_STATE_DIM {
                if branch.d_internal[col] != 0.0 {
                    stamper.stamp(
                        row_node,
                        self.vbic_internal_node(col),
                        sign * branch.d_internal[col],
                    );
                }
            }
            for col in 0..EXTERNAL_DIM {
                if branch.d_external[col] != 0.0 {
                    stamper.stamp(row_node, external_nodes[col], sign * branch.d_external[col]);
                }
            }
            stamper.stamp_rhs(row_node, sign * source);
        };

        if let Some(idx) = branch.pos_internal {
            stamp_side(
                self.vbic_internal_node(idx),
                Self::vbic_residual_row_sign(idx),
            );
        }
        if let Some(idx) = branch.neg_internal {
            stamp_side(
                self.vbic_internal_node(idx),
                -Self::vbic_residual_row_sign(idx),
            );
        }
        // External rows already use the MNA leaving-current orientation.
        // Delay branches carry no external incidence today; keep the mapping
        // complete so future residual branches stamp correctly.
        if let Some(idx) = branch.pos_external {
            stamp_side(external_nodes[idx], 1.0);
        }
        if let Some(idx) = branch.neg_external {
            stamp_side(external_nodes[idx], -1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::traits::NonlinearDevice;

    /// Dense stamper capturing the promoted system for finite-difference
    /// Jacobian validation.
    struct DenseStamper {
        n: usize,
        a: Vec<Vec<Value>>,
        b: Vec<Value>,
    }

    impl DenseStamper {
        fn new(n: usize) -> Self {
            Self {
                n,
                a: vec![vec![0.0; n]; n],
                b: vec![0.0; n],
            }
        }

        /// Linearized residual A*v - b == f(v) at the stamp's own bias.
        fn residual(&self, v: &[Value]) -> Vec<Value> {
            (0..self.n)
                .map(|row| {
                    (0..self.n)
                        .map(|col| self.a[row][col] * v[col])
                        .sum::<Value>()
                        - self.b[row]
                })
                .collect()
        }
    }

    impl MatrixStamper for DenseStamper {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            if row > 0 && col > 0 {
                self.a[row - 1][col - 1] += value;
            }
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            if index > 0 {
                self.b[index - 1] += value;
            }
        }
    }

    fn diffamp_pnp() -> Bjt {
        let mut params = std::collections::HashMap::new();
        for (key, value) in [
            ("LEVEL", 4.0),
            ("IS", 1e-16),
            ("IBEI", 1e-18),
            ("IBEN", 5e-15),
            ("IBCI", 2e-17),
            ("IBCN", 5e-15),
            ("ISP", 1e-15),
            ("RCX", 10.0),
            ("RCI", 60.0),
            ("RBX", 10.0),
            ("RBI", 40.0),
            ("RE", 2.0),
            ("RS", 20.0),
            ("RBP", 40.0),
            ("VEF", 10.0),
            ("VER", 4.0),
            ("IKF", 2e-3),
            ("ITF", 8e-2),
            ("XTF", 20.0),
            ("IKR", 2e-4),
            ("IKP", 2e-4),
            ("CJE", 1e-13),
            ("CJC", 2e-14),
            ("CJEP", 1e-13),
            ("CJCP", 4e-13),
            ("VO", 2.0),
            ("GAMM", 2e-11),
            ("HRCF", 2.0),
            ("QCO", 1e-12),
            ("AVC1", 2.0),
            ("AVC2", 15.0),
            ("TF", 10e-12),
            ("TR", 100e-12),
            ("TD", 2e-11),
            ("RTH", 300.0),
        ] {
            params.insert(key.to_string(), value);
        }
        // Diffamp cascode: collector node 1, base node 2, emitter node 3,
        // substrate tied to the collector node like the deck instances.
        let mut bjt = Bjt::new_pnp("QT".to_string(), 1, 2, 3);
        bjt = bjt.with_params(&params);
        bjt.set_substrate_node(1);
        let mut next = 4;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        bjt
    }

    /// The promoted stamp is a Newton linearization: for biases close enough
    /// that junction limiting stays inactive, f(v') - f(v) must match
    /// A(v) * (v' - v) to first order across every promoted column. A wrong or
    /// missing Jacobian entry shows up as a first-order mismatch.
    #[test]
    fn promoted_stamp_matches_finite_difference_jacobian() {
        let mut bjt = diffamp_pnp();
        let n = 13;

        // Bias near the diffamp PNP operating point with the b-c junction at
        // the saturation knife edge (vbci slightly forward, PNP polarity).
        let mut v = vec![0.0; n];
        let assign = |v: &mut Vec<Value>, node: NodeId, value: Value| {
            v[node - 1] = value;
        };
        assign(&mut v, bjt.node_collector, 2.6234);
        assign(&mut v, bjt.node_base, 2.6180);
        assign(&mut v, bjt.node_emitter, 3.3000);
        assign(&mut v, bjt.node_cx, 2.6238);
        assign(&mut v, bjt.node_ci, 2.6252);
        assign(&mut v, bjt.node_bx, 2.6181);
        assign(&mut v, bjt.node_bi, 2.6178);
        assign(&mut v, bjt.node_si, 2.6235);
        assign(&mut v, bjt.node_bp, 2.6239);
        assign(&mut v, bjt.node_ei, 3.2999);
        assign(&mut v, bjt.node_xf1, 2.05e-5);
        assign(&mut v, bjt.node_xf2, 2.05e-5);

        // Settle the limiter anchor at the bias so pnjlim stays inactive for
        // the FD probes.
        bjt.update(&v);
        bjt.update(&v);
        let mut base = DenseStamper::new(n);
        bjt.stamp_vbic_mna(&mut base);
        let f0 = base.residual(&v);

        let h = 1e-7;
        let mut worst: (Value, usize, usize) = (0.0, 0, 0);
        for col in 0..n {
            let mut vp = v.clone();
            vp[col] += h;
            bjt.update(&vp);
            let mut pert = DenseStamper::new(n);
            bjt.stamp_vbic_mna(&mut pert);
            let f1 = pert.residual(&vp);
            // Restore the limiter anchor for the next probe.
            bjt.update(&v);

            for row in 0..n {
                let fd = (f1[row] - f0[row]) / h;
                let analytic = base.a[row][col];
                let scale = analytic.abs().max(fd.abs()).max(1e-6);
                let err = (fd - analytic).abs() / scale;
                if err > worst.0 {
                    worst = (err, row, col);
                }
                assert!(
                    err < 5e-3,
                    "Jacobian mismatch at row {row} col {col}: fd={fd:.6e} analytic={analytic:.6e} rel_err={err:.3e}"
                );
            }
        }
        println!(
            "worst relative error {:.3e} at row {} col {}",
            worst.0, worst.1, worst.2
        );
    }

    /// The charge branches are the transient Jacobian: every dq/dv column
    /// must match a finite difference of the branch charge at the same bias,
    /// including at the saturation knife edge where the epi charge (qbcx)
    /// turns on exponentially.
    #[test]
    fn promoted_charge_branches_match_finite_difference() {
        let bjt = diffamp_pnp();
        let n = 13;

        let mut v = vec![0.0; n];
        let assign = |v: &mut Vec<Value>, node: NodeId, value: Value| {
            v[node - 1] = value;
        };
        assign(&mut v, bjt.node_collector, 2.6234);
        assign(&mut v, bjt.node_base, 2.6180);
        assign(&mut v, bjt.node_emitter, 3.3000);
        assign(&mut v, bjt.node_cx, 2.6238);
        assign(&mut v, bjt.node_ci, 2.6252);
        assign(&mut v, bjt.node_bx, 2.6181);
        assign(&mut v, bjt.node_bi, 2.6178);
        assign(&mut v, bjt.node_si, 2.6235);
        assign(&mut v, bjt.node_bp, 2.6239);
        assign(&mut v, bjt.node_ei, 3.2999);
        assign(&mut v, bjt.node_xf1, 2.05e-5);
        assign(&mut v, bjt.node_xf2, 2.05e-5);

        let (base_branches, base_internal, base_external) =
            bjt.vbic_mna_charge_state_at_solution(&v);

        let h = 1e-7;
        for col in 0..n {
            let mut vp = v.clone();
            vp[col] += h;
            let (pert_branches, pert_internal, pert_external) =
                bjt.vbic_mna_charge_state_at_solution(&vp);

            for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
                if !base_branches[branch_idx].is_active() {
                    continue;
                }
                let fd = (pert_branches[branch_idx].charge - base_branches[branch_idx].charge) / h;
                let mut analytic = 0.0;
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    let d_int = (pert_internal[idx] - base_internal[idx]) / h;
                    analytic += base_branches[branch_idx].d_internal[idx] * d_int;
                }
                for idx in 0..EXTERNAL_DIM {
                    let d_ext = (pert_external[idx] - base_external[idx]) / h;
                    analytic += base_branches[branch_idx].d_external[idx] * d_ext;
                }
                let scale = analytic.abs().max(fd.abs()).max(1e-16);
                let err = (fd - analytic).abs() / scale;
                assert!(
                    err < 5e-3,
                    "charge Jacobian mismatch branch {branch_idx} col {col}: fd={fd:.6e} analytic={analytic:.6e} rel_err={err:.3e}"
                );
            }
        }
    }
}
