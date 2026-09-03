use super::*;

/// One classic-MOSFET bulk diode as the stamper sees it: which two nodes it
/// spans and the linearized conductance and equivalent current it contributes.
#[derive(Clone, Copy)]
struct ClassicDiodeBranch {
    anode: NodeId,
    cathode: NodeId,
    conductance: Value,
    equivalent_current: Value,
}

/// The linearized channel the direct operating-point stamp writes: the branch
/// voltages it was evaluated at, the three transconductances and the
/// equivalent current.
#[derive(Clone, Copy)]
struct ClassicMosDirectOperatingPoint {
    eval_vds: Value,
    eval_vbs: Value,
    gm: Value,
    gds: Value,
    gmb: Value,
    id_eq: Value,
}

impl Mosfet {
    #[inline]
    fn classic_diode_stamp_plan(
        matrix: &StaticMatrix,
        anode: NodeId,
        cathode: NodeId,
    ) -> Option<ClassicMosDiodeStampPlan> {
        let index = |row: NodeId, column: NodeId| {
            (row > 0 && column > 0)
                .then(|| matrix.get_index(row - 1, column - 1))
                .flatten()
        };
        let plan = ClassicMosDiodeStampPlan {
            aa: ClassicMosValueSlot::from_index(index(anode, anode)),
            ac: ClassicMosValueSlot::from_index(index(anode, cathode)),
            ca: ClassicMosValueSlot::from_index(index(cathode, anode)),
            cc: ClassicMosValueSlot::from_index(index(cathode, cathode)),
        };
        let complete = (anode == 0 || plan.aa.offset().is_some())
            && (anode == 0 || cathode == 0 || plan.ac.offset().is_some())
            && (cathode == 0 || anode == 0 || plan.ca.offset().is_some())
            && (cathode == 0 || plan.cc.offset().is_some());
        complete.then_some(plan)
    }

    /// Capture the immutable sparse locations used by the compact transient
    /// linearization stream. This is performed once after the matrix pattern
    /// is linked, never in a Newton iteration.
    pub(crate) fn classic_static_stamp_plan(
        &self,
        matrix: &StaticMatrix,
    ) -> Option<ClassicMosStaticStampPlan> {
        let channel_complete = [
            (self.node_drain, self.node_drain, self.indices.dd),
            (self.node_drain, self.node_gate, self.indices.dg),
            (self.node_drain, self.node_source, self.indices.ds),
            (self.node_drain, self.node_bulk, self.indices.db),
            (self.node_source, self.node_drain, self.indices.sd),
            (self.node_source, self.node_gate, self.indices.sg),
            (self.node_source, self.node_source, self.indices.ss),
            (self.node_source, self.node_bulk, self.indices.sb),
        ]
        .into_iter()
        .all(|(row, column, index)| row == 0 || column == 0 || index.is_some());
        if !channel_complete {
            return None;
        }
        let (bs_anode, bs_cathode) = self.body_source_diode_nodes();
        let (bd_anode, bd_cathode) = self.body_drain_diode_nodes();
        Some(ClassicMosStaticStampPlan {
            pattern: matrix.pattern_token(),
            indices: ClassicMosValueIndices::from(&self.indices),
            node_drain: self.node_drain,
            node_gate: self.node_gate,
            node_source: self.node_source,
            node_bulk: self.node_bulk,
            body_anode_is_bulk: matches!(self.mos_type, MosType::Nmos),
            body_source: Self::classic_diode_stamp_plan(matrix, bs_anode, bs_cathode)?,
            body_drain: Self::classic_diode_stamp_plan(matrix, bd_anode, bd_cathode)?,
        })
    }

    /// Export the already-evaluated static terms without repeating any device
    /// law. The arithmetic order matches `stamp_cached_direct` exactly.
    #[inline]
    pub(crate) fn classic_cached_static_terms(&self) -> ClassicMosCachedStaticTerms {
        debug_assert!(self.has_branch_history);
        let (_, _, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(self.eval_vbs, true);
        let (_, _, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(self.eval_vds, self.eval_vbs, true);
        ClassicMosCachedStaticTerms {
            gm: self.gm,
            gds: self.gds,
            gmb: self.gmb,
            id_eq: self.id_eq,
            gbs,
            ieq_bs,
            gbd,
            ieq_bd,
        }
    }

    #[cfg(feature = "parallel")]
    #[inline]
    pub(crate) fn classic_residual_row_plan(&self) -> ClassicMosResidualRowPlan {
        let (body_source_anode, body_source_cathode) = self.body_source_diode_nodes();
        let (body_drain_anode, body_drain_cathode) = self.body_drain_diode_nodes();
        ClassicMosResidualRowPlan {
            node_drain: self.node_drain,
            node_gate: self.node_gate,
            node_source: self.node_source,
            node_bulk: self.node_bulk,
            body_source_anode,
            body_source_cathode,
            body_drain_anode,
            body_drain_cathode,
        }
    }

    #[inline]
    fn stamp_classic_diode_plan(
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        pattern: CscPatternToken,
        plan: &ClassicMosDiodeStampPlan,
        branch: ClassicDiodeBranch,
    ) {
        let ClassicDiodeBranch {
            anode,
            cathode,
            conductance,
            equivalent_current,
        } = branch;
        if conductance == 0.0 && equivalent_current == 0.0 {
            return;
        }
        if let Some(index) = plan.aa.checked_index(pattern) {
            matrix.stamp_direct(index, conductance);
        }
        if let Some(index) = plan.ac.checked_index(pattern) {
            matrix.stamp_direct(index, -conductance);
        }
        if anode > 0 {
            rhs[anode - 1] -= equivalent_current;
        }
        if let Some(index) = plan.ca.checked_index(pattern) {
            matrix.stamp_direct(index, -conductance);
        }
        if let Some(index) = plan.cc.checked_index(pattern) {
            matrix.stamp_direct(index, conductance);
        }
        if cathode > 0 {
            rhs[cathode - 1] += equivalent_current;
        }
    }

    /// Stamp a compact cached linearization through its prelinked plan.
    /// Contributions retain the canonical per-device ordering exactly.
    #[inline]
    pub(crate) fn stamp_classic_cached_static_terms(
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        plan: &ClassicMosStaticStampPlan,
        terms: &ClassicMosCachedStaticTerms,
    ) {
        let source_diagonal = terms.gm + terms.gds + terms.gmb;
        if let Some(index) = plan.indices.dd.checked_index(plan.pattern) {
            matrix.stamp_direct(index, terms.gds);
        }
        if let Some(index) = plan.indices.dg.checked_index(plan.pattern) {
            matrix.stamp_direct(index, terms.gm);
        }
        if let Some(index) = plan.indices.ds.checked_index(plan.pattern) {
            matrix.stamp_direct(index, -terms.gm - terms.gds - terms.gmb);
        }
        if let Some(index) = plan.indices.db.checked_index(plan.pattern) {
            matrix.stamp_direct(index, terms.gmb);
        }
        if let Some(index) = plan.indices.sd.checked_index(plan.pattern) {
            matrix.stamp_direct(index, -terms.gds);
        }
        if let Some(index) = plan.indices.sg.checked_index(plan.pattern) {
            matrix.stamp_direct(index, -terms.gm);
        }
        if let Some(index) = plan.indices.ss.checked_index(plan.pattern) {
            matrix.stamp_direct(index, source_diagonal);
        }
        if let Some(index) = plan.indices.sb.checked_index(plan.pattern) {
            matrix.stamp_direct(index, -terms.gmb);
        }
        if plan.node_drain > 0 {
            rhs[plan.node_drain - 1] -= terms.id_eq;
        }
        if plan.node_source > 0 {
            rhs[plan.node_source - 1] += terms.id_eq;
        }
        let ((bs_anode, bs_cathode), (bd_anode, bd_cathode)) = plan.body_diode_nodes();
        Self::stamp_classic_diode_plan(
            matrix,
            rhs,
            plan.pattern,
            &plan.body_source,
            ClassicDiodeBranch {
                anode: bs_anode,
                cathode: bs_cathode,
                conductance: terms.gbs,
                equivalent_current: terms.ieq_bs,
            },
        );
        Self::stamp_classic_diode_plan(
            matrix,
            rhs,
            plan.pattern,
            &plan.body_drain,
            ClassicDiodeBranch {
                anode: bd_anode,
                cathode: bd_cathode,
                conductance: terms.gbd,
                equivalent_current: terms.ieq_bd,
            },
        );
    }

    #[inline]
    fn stamp_classic_diode_values(
        values: &mut [Value],
        rhs: &mut [Value],
        plan: &ClassicMosDiodeStampPlan,
        branch: ClassicDiodeBranch,
    ) {
        let ClassicDiodeBranch {
            anode,
            cathode,
            conductance,
            equivalent_current,
        } = branch;
        if conductance == 0.0 && equivalent_current == 0.0 {
            return;
        }
        if let Some(offset) = plan.aa.offset() {
            values[offset] += conductance;
        }
        if let Some(offset) = plan.ac.offset() {
            values[offset] += -conductance;
        }
        if anode > 0 {
            rhs[anode - 1] -= equivalent_current;
        }
        if let Some(offset) = plan.ca.offset() {
            values[offset] += -conductance;
        }
        if let Some(offset) = plan.cc.offset() {
            values[offset] += conductance;
        }
        if cathode > 0 {
            rhs[cathode - 1] += equivalent_current;
        }
    }

    /// Stamp a compact cached linearization into a once-validated CSC value
    /// slice. Arithmetic and per-device contribution order match
    /// [`Self::stamp_classic_cached_static_terms`] exactly.
    #[inline]
    pub(crate) fn stamp_classic_cached_static_values(
        values: &mut [Value],
        rhs: &mut [Value],
        plan: &ClassicMosStaticStampPlan,
        terms: &ClassicMosCachedStaticTerms,
    ) {
        let source_diagonal = terms.gm + terms.gds + terms.gmb;
        if let Some(offset) = plan.indices.dd.offset() {
            values[offset] += terms.gds;
        }
        if let Some(offset) = plan.indices.dg.offset() {
            values[offset] += terms.gm;
        }
        if let Some(offset) = plan.indices.ds.offset() {
            values[offset] += -terms.gm - terms.gds - terms.gmb;
        }
        if let Some(offset) = plan.indices.db.offset() {
            values[offset] += terms.gmb;
        }
        if let Some(offset) = plan.indices.sd.offset() {
            values[offset] += -terms.gds;
        }
        if let Some(offset) = plan.indices.sg.offset() {
            values[offset] += -terms.gm;
        }
        if let Some(offset) = plan.indices.ss.offset() {
            values[offset] += source_diagonal;
        }
        if let Some(offset) = plan.indices.sb.offset() {
            values[offset] += -terms.gmb;
        }
        if plan.node_drain > 0 {
            rhs[plan.node_drain - 1] -= terms.id_eq;
        }
        if plan.node_source > 0 {
            rhs[plan.node_source - 1] += terms.id_eq;
        }
        let ((bs_anode, bs_cathode), (bd_anode, bd_cathode)) = plan.body_diode_nodes();
        Self::stamp_classic_diode_values(
            values,
            rhs,
            &plan.body_source,
            ClassicDiodeBranch {
                anode: bs_anode,
                cathode: bs_cathode,
                conductance: terms.gbs,
                equivalent_current: terms.ieq_bs,
            },
        );
        Self::stamp_classic_diode_values(
            values,
            rhs,
            &plan.body_drain,
            ClassicDiodeBranch {
                anode: bd_anode,
                cathode: bd_cathode,
                conductance: terms.gbd,
                equivalent_current: terms.ieq_bd,
            },
        );
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;

        // Drain row (4 columns)
        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }
        if d > 0 && b > 0 {
            self.indices.db = matrix.get_index(d - 1, b - 1);
        }
        // Source row (4 columns)
        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
        if s > 0 && b > 0 {
            self.indices.sb = matrix.get_index(s - 1, b - 1);
        }
    }

    fn stamp_direct_operating_point(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        operating_point: ClassicMosDirectOperatingPoint,
        cache_matches: bool,
    ) {
        let ClassicMosDirectOperatingPoint {
            eval_vds,
            eval_vbs,
            gm,
            gds,
            gmb,
            id_eq,
        } = operating_point;
        // Stamp matrix using direct indexing
        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds - gmb);
        }
        if let Some(idx) = self.indices.db {
            matrix.stamp_direct(idx, gmb);
        }
        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + gmb);
        }
        if let Some(idx) = self.indices.sb {
            matrix.stamp_direct(idx, -gmb);
        }

        // Stamp RHS
        if self.node_drain > 0 {
            rhs[self.node_drain - 1] -= id_eq;
        }
        if self.node_source > 0 {
            rhs[self.node_source - 1] += id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(eval_vbs, cache_matches);
        self.stamp_body_source_linearization_direct(matrix, rhs, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(eval_vds, eval_vbs, cache_matches);
        self.stamp_body_drain_linearization_direct(matrix, rhs, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    /// Stamp using O(1) direct indexing (call after link).
    pub(crate) fn stamp_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let cache_matches = self.cached_linearization_matches_eval(eval_vgs, eval_vds, eval_vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
            (gm, gds, gmb, id_eq)
        };

        self.stamp_direct_operating_point(
            matrix,
            rhs,
            ClassicMosDirectOperatingPoint {
                eval_vds,
                eval_vbs,
                gm,
                gds,
                gmb,
                id_eq,
            },
            cache_matches,
        );
    }

    /// Stamp the Newton linearization already cached by [`NonlinearDevice::update`].
    ///
    /// The transient classic-MOS fast path proves that device state matches
    /// its current iterate before calling this, so reloading terminal voltages,
    /// rerunning limiting, and comparing the cached bias is redundant.
    pub(crate) fn stamp_cached_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        debug_assert!(self.has_branch_history);
        self.stamp_direct_operating_point(
            matrix,
            rhs,
            ClassicMosDirectOperatingPoint {
                eval_vds: self.eval_vds,
                eval_vbs: self.eval_vbs,
                gm: self.gm,
                gds: self.gds,
                gmb: self.gmb,
                id_eq: self.id_eq,
            },
            true,
        );
    }

    /// Stamp a cached linearization into a once-validated CSC value slice
    /// when the body terminal is ground. This mirrors
    /// [`Self::stamp_cached_direct`] without repeating the matrix-pattern
    /// check for every scalar contribution.
    #[inline]
    pub(crate) fn stamp_grounded_bulk_cached_values(
        &self,
        values: &mut [Value],
        rhs: &mut [Value],
    ) {
        debug_assert!(self.has_branch_history);
        debug_assert_eq!(self.node_bulk, 0);

        if let Some(index) = self.indices.dd {
            values[index.offset()] += self.gds;
        }
        if let Some(index) = self.indices.dg {
            values[index.offset()] += self.gm;
        }
        if let Some(index) = self.indices.ds {
            values[index.offset()] += -self.gm - self.gds - self.gmb;
        }
        if let Some(index) = self.indices.db {
            values[index.offset()] += self.gmb;
        }
        if let Some(index) = self.indices.sd {
            values[index.offset()] += -self.gds;
        }
        if let Some(index) = self.indices.sg {
            values[index.offset()] += -self.gm;
        }
        if let Some(index) = self.indices.ss {
            values[index.offset()] += self.gm + self.gds + self.gmb;
        }
        if let Some(index) = self.indices.sb {
            values[index.offset()] += -self.gmb;
        }
        if self.node_drain > 0 {
            rhs[self.node_drain - 1] -= self.id_eq;
        }
        if self.node_source > 0 {
            rhs[self.node_source - 1] += self.id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(self.eval_vbs, true);
        if gbs != 0.0 || ieq_bs != 0.0 {
            if let Some(index) = self.indices.ss {
                values[index.offset()] += gbs;
            }
            if bs_anode > 0 {
                rhs[bs_anode - 1] -= ieq_bs;
            }
            if bs_cathode > 0 {
                rhs[bs_cathode - 1] += ieq_bs;
            }
        }

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(self.eval_vds, self.eval_vbs, true);
        if gbd != 0.0 || ieq_bd != 0.0 {
            if let Some(index) = self.indices.dd {
                values[index.offset()] += gbd;
            }
            if bd_anode > 0 {
                rhs[bd_anode - 1] -= ieq_bd;
            }
            if bd_cathode > 0 {
                rhs[bd_cathode - 1] += ieq_bd;
            }
        }
    }

    /// Evaluate the true, unlimited static device at `solution` and add its
    /// linearized `A*x` and RHS contributions directly to row accumulators.
    ///
    /// Residual acceptance needs physical currents, not the Newton Jacobian.
    /// Keeping this pure avoids mutating limiter history while also avoiding
    /// sparse writes that would only be multiplied back by the same candidate.
    pub(crate) fn add_physical_static_residual_row_terms_at(
        &self,
        solution: &[Value],
        constants: &ClassicMosTransientConstants,
        row_ax: &mut [Value],
        row_rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(solution);
        let cache_matches = self.cached_linearization_matches_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_transient_operating_point(vgs, vds, vbs, constants);
            (gm, gds, gmb, id_eq)
        };
        let vd = Self::terminal_voltage(solution, self.node_drain);
        let vg = Self::terminal_voltage(solution, self.node_gate);
        let vs = Self::terminal_voltage(solution, self.node_source);
        let vb = Self::terminal_voltage(solution, self.node_bulk);
        let source_diagonal = gm + gds + gmb;

        if self.node_drain > 0 {
            let row = self.node_drain - 1;
            row_ax[row] += gds * vd;
            if self.node_gate > 0 {
                row_ax[row] += gm * vg;
            }
            if self.node_source > 0 {
                row_ax[row] -= source_diagonal * vs;
            }
            if self.node_bulk > 0 {
                row_ax[row] += gmb * vb;
            }
            row_rhs[row] -= id_eq;
        }
        if self.node_source > 0 {
            let row = self.node_source - 1;
            if self.node_drain > 0 {
                row_ax[row] -= gds * vd;
            }
            if self.node_gate > 0 {
                row_ax[row] -= gm * vg;
            }
            row_ax[row] += source_diagonal * vs;
            if self.node_bulk > 0 {
                row_ax[row] -= gmb * vb;
            }
            row_rhs[row] += id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(vbs, cache_matches);
        Self::add_diode_residual_row_terms(
            solution,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: bs_anode,
                cathode: bs_cathode,
                conductance: gbs,
                equivalent_current: ieq_bs,
            },
        );
        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(vds, vbs, cache_matches);
        Self::add_diode_residual_row_terms(
            solution,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: bd_anode,
                cathode: bd_cathode,
                conductance: gbd,
                equivalent_current: ieq_bd,
            },
        );
    }

    /// Add an already-evaluated physical static contribution through the
    /// compact topology plan used by the serial transient assembler.
    ///
    /// This preserves the full-device residual arithmetic order while keeping
    /// the proof walk on two tightly packed arrays instead of revisiting model
    /// cards and mutable limiter history.
    pub(crate) fn add_cached_physical_static_residual_terms(
        solution: &[Value],
        plan: &ClassicMosStaticStampPlan,
        terms: &ClassicMosCachedStaticTerms,
        row_ax: &mut [Value],
        row_rhs: &mut [Value],
    ) {
        let vd = Self::terminal_voltage(solution, plan.node_drain);
        let vg = Self::terminal_voltage(solution, plan.node_gate);
        let vs = Self::terminal_voltage(solution, plan.node_source);
        let vb = Self::terminal_voltage(solution, plan.node_bulk);
        let source_diagonal = terms.gm + terms.gds + terms.gmb;

        if plan.node_drain > 0 {
            let row = plan.node_drain - 1;
            row_ax[row] += terms.gds * vd;
            if plan.node_gate > 0 {
                row_ax[row] += terms.gm * vg;
            }
            if plan.node_source > 0 {
                row_ax[row] -= source_diagonal * vs;
            }
            if plan.node_bulk > 0 {
                row_ax[row] += terms.gmb * vb;
            }
            row_rhs[row] -= terms.id_eq;
        }
        if plan.node_source > 0 {
            let row = plan.node_source - 1;
            if plan.node_drain > 0 {
                row_ax[row] -= terms.gds * vd;
            }
            if plan.node_gate > 0 {
                row_ax[row] -= terms.gm * vg;
            }
            row_ax[row] += source_diagonal * vs;
            if plan.node_bulk > 0 {
                row_ax[row] -= terms.gmb * vb;
            }
            row_rhs[row] += terms.id_eq;
        }

        let ((bs_anode, bs_cathode), (bd_anode, bd_cathode)) = plan.body_diode_nodes();
        Self::add_diode_residual_row_terms(
            solution,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: bs_anode,
                cathode: bs_cathode,
                conductance: terms.gbs,
                equivalent_current: terms.ieq_bs,
            },
        );
        Self::add_diode_residual_row_terms(
            solution,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: bd_anode,
                cathode: bd_cathode,
                conductance: terms.gbd,
                equivalent_current: terms.ieq_bd,
            },
        );
    }

    /// Add the already-evaluated physical static contribution for one MNA row.
    ///
    /// A row-parallel residual proof calls this for devices incident on each
    /// row in original instance order. The arithmetic within a row is exactly
    /// the same as [`Self::add_physical_static_residual_row_terms_at`], while
    /// disjoint row outputs allow deterministic parallel execution.
    #[cfg(feature = "parallel")]
    pub(crate) fn add_cached_physical_static_residual_row_terms(
        solution: &[Value],
        plan: &ClassicMosResidualRowPlan,
        terms: &ClassicMosCachedStaticTerms,
        row: usize,
        row_ax: &mut Value,
        row_rhs: &mut Value,
    ) {
        let vd = Self::terminal_voltage(solution, plan.node_drain);
        let vg = Self::terminal_voltage(solution, plan.node_gate);
        let vs = Self::terminal_voltage(solution, plan.node_source);
        let vb = Self::terminal_voltage(solution, plan.node_bulk);
        let source_diagonal = terms.gm + terms.gds + terms.gmb;

        if plan.node_drain > 0 && row == plan.node_drain - 1 {
            *row_ax += terms.gds * vd;
            if plan.node_gate > 0 {
                *row_ax += terms.gm * vg;
            }
            if plan.node_source > 0 {
                *row_ax -= source_diagonal * vs;
            }
            if plan.node_bulk > 0 {
                *row_ax += terms.gmb * vb;
            }
            *row_rhs -= terms.id_eq;
        }
        if plan.node_source > 0 && row == plan.node_source - 1 {
            if plan.node_drain > 0 {
                *row_ax -= terms.gds * vd;
            }
            if plan.node_gate > 0 {
                *row_ax -= terms.gm * vg;
            }
            *row_ax += source_diagonal * vs;
            if plan.node_bulk > 0 {
                *row_ax -= terms.gmb * vb;
            }
            *row_rhs += terms.id_eq;
        }

        Self::add_cached_diode_residual_row_terms(
            solution,
            row,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: plan.body_source_anode,
                cathode: plan.body_source_cathode,
                conductance: terms.gbs,
                equivalent_current: terms.ieq_bs,
            },
        );
        Self::add_cached_diode_residual_row_terms(
            solution,
            row,
            row_ax,
            row_rhs,
            ClassicDiodeBranch {
                anode: plan.body_drain_anode,
                cathode: plan.body_drain_cathode,
                conductance: terms.gbd,
                equivalent_current: terms.ieq_bd,
            },
        );
    }

    #[cfg(feature = "parallel")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn add_cached_diode_residual_row_terms(
        solution: &[Value],
        row: usize,
        row_ax: &mut Value,
        row_rhs: &mut Value,
        branch: ClassicDiodeBranch,
    ) {
        let ClassicDiodeBranch {
            anode,
            cathode,
            conductance,
            equivalent_current,
        } = branch;
        if conductance == 0.0 && equivalent_current == 0.0 {
            return;
        }
        let va = Self::terminal_voltage(solution, anode);
        let vc = Self::terminal_voltage(solution, cathode);
        if anode > 0 && row == anode - 1 {
            *row_ax += conductance * va;
            if cathode > 0 {
                *row_ax -= conductance * vc;
            }
            *row_rhs -= equivalent_current;
        }
        if cathode > 0 && row == cathode - 1 {
            if anode > 0 {
                *row_ax -= conductance * va;
            }
            *row_ax += conductance * vc;
            *row_rhs += equivalent_current;
        }
    }

    #[inline]
    fn add_diode_residual_row_terms(
        solution: &[Value],
        row_ax: &mut [Value],
        row_rhs: &mut [Value],
        branch: ClassicDiodeBranch,
    ) {
        let ClassicDiodeBranch {
            anode,
            cathode,
            conductance,
            equivalent_current,
        } = branch;
        if conductance == 0.0 && equivalent_current == 0.0 {
            return;
        }
        let va = Self::terminal_voltage(solution, anode);
        let vc = Self::terminal_voltage(solution, cathode);
        if anode > 0 {
            let row = anode - 1;
            row_ax[row] += conductance * va;
            if cathode > 0 {
                row_ax[row] -= conductance * vc;
            }
            row_rhs[row] -= equivalent_current;
        }
        if cathode > 0 {
            let row = cathode - 1;
            if anode > 0 {
                row_ax[row] -= conductance * va;
            }
            row_ax[row] += conductance * vc;
            row_rhs[row] += equivalent_current;
        }
    }

    /// Stamp the physical equations directly at a static candidate.
    ///
    /// Newton voltage limiting is iteration history, not part of the device
    /// equations. Residual and line-search probes must therefore bypass it or
    /// an otherwise valid operating point can be rejected based on the path
    /// taken to reach it.
    pub(crate) fn stamp_static_probe_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let cache_matches = self.cached_linearization_matches_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) = self.linearized_operating_point(vgs, vds, vbs);
            (gm, gds, gmb, id_eq)
        };
        self.stamp_direct_operating_point(
            matrix,
            rhs,
            ClassicMosDirectOperatingPoint {
                eval_vds: vds,
                eval_vbs: vbs,
                gm,
                gds,
                gmb,
                id_eq,
            },
            cache_matches,
        );
    }

    /// Get polarity multiplier (+1 for NMOS, -1 for PMOS)
    pub(crate) fn polarity(&self) -> Value {
        match self.mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_matrix(size: usize) -> StaticMatrix {
        let triplets: Vec<_> = (0..size)
            .flat_map(|row| (0..size).map(move |col| (row, col, 0.0)))
            .collect();
        StaticMatrix::from_triplets(size, size, &triplets).expect("full test matrix")
    }

    #[test]
    fn static_probe_bypasses_classic_mos_limiter_history() {
        let initial = [0.0, 0.0, 0.0, 0.0];
        let candidate = [0.0, 0.0, 5.0, 5.0];
        let mut history_mos = Mosfet::new_pmos("mp".to_string(), 1, 2, 3, 4);
        history_mos.update(&initial);
        history_mos.update(&candidate);

        let mut limited_matrix = full_matrix(4);
        history_mos.link(&limited_matrix);
        let mut limited_rhs = vec![0.0; 4];
        history_mos.stamp_direct(&mut limited_matrix, &mut limited_rhs, &candidate);

        let mut static_matrix = full_matrix(4);
        history_mos.link(&static_matrix);
        let mut static_rhs = vec![0.0; 4];
        history_mos.stamp_static_probe_direct(&mut static_matrix, &mut static_rhs, &candidate);

        let mut fresh_mos = Mosfet::new_pmos("mp".to_string(), 1, 2, 3, 4);
        let mut expected_matrix = full_matrix(4);
        fresh_mos.link(&expected_matrix);
        let mut expected_rhs = vec![0.0; 4];
        fresh_mos.stamp_direct(&mut expected_matrix, &mut expected_rhs, &candidate);

        assert_ne!(limited_rhs, static_rhs, "test bias must exercise limiting");
        assert_eq!(static_rhs, expected_rhs);
        assert_eq!(static_matrix.values_mut(), expected_matrix.values_mut());
    }

    #[test]
    fn cached_direct_stamp_matches_verified_cached_bias_exactly() {
        let candidate = [1.2, 1.8, 0.2, -0.1];
        let mut mos = Mosfet::new_nmos("mn".to_string(), 1, 2, 3, 4);
        mos.update(&candidate);

        let mut verified_matrix = full_matrix(4);
        mos.link(&verified_matrix);
        let mut verified_rhs = vec![0.0; 4];
        mos.stamp_direct(&mut verified_matrix, &mut verified_rhs, &candidate);

        let mut cached_matrix = full_matrix(4);
        mos.link(&cached_matrix);
        let mut cached_rhs = vec![0.0; 4];
        mos.stamp_cached_direct(&mut cached_matrix, &mut cached_rhs);

        assert_eq!(cached_rhs, verified_rhs);
        assert_eq!(cached_matrix.values_mut(), verified_matrix.values_mut());
    }

    #[test]
    fn batched_cached_static_stamp_matches_validated_stamp_exactly() {
        let candidate = [1.2, 1.8, 0.2, -0.1];
        let mut mos = Mosfet::new_nmos("mn".to_string(), 1, 2, 3, 4);
        let linked_matrix = full_matrix(4);
        mos.link(&linked_matrix);
        let constants = mos.classic_transient_constants();
        mos.update_with_classic_transient_constants(&candidate, &constants);
        let plan = mos
            .classic_static_stamp_plan(&linked_matrix)
            .expect("full matrix supports compact MOS stamp");
        let terms = mos.classic_cached_static_terms();

        let mut validated_matrix = linked_matrix.clone_structure();
        let mut validated_rhs = vec![0.0; 4];
        Mosfet::stamp_classic_cached_static_terms(
            &mut validated_matrix,
            &mut validated_rhs,
            &plan,
            &terms,
        );

        let mut batched_matrix = linked_matrix.clone_structure();
        let mut batched_rhs = vec![0.0; 4];
        let values = batched_matrix
            .values_mut_for_pattern(linked_matrix.pattern_token())
            .expect("clone retains the linked pattern");
        Mosfet::stamp_classic_cached_static_values(values, &mut batched_rhs, &plan, &terms);

        assert_eq!(batched_rhs, validated_rhs);
        assert_eq!(batched_matrix.values_mut(), validated_matrix.values_mut());
    }

    #[test]
    fn grounded_bulk_batched_cached_stamp_matches_validated_stamp_exactly() {
        let candidate = [1.2, 1.8, 0.2];
        let mut mos = Mosfet::new_nmos("mn".to_string(), 1, 2, 3, 0);
        let linked_matrix = full_matrix(3);
        mos.link(&linked_matrix);
        let constants = mos.classic_transient_constants();
        mos.update_with_classic_transient_constants(&candidate, &constants);

        let mut validated_matrix = linked_matrix.clone_structure();
        let mut validated_rhs = vec![0.0; 3];
        mos.stamp_cached_direct(&mut validated_matrix, &mut validated_rhs);

        let mut batched_matrix = linked_matrix.clone_structure();
        let mut batched_rhs = vec![0.0; 3];
        let values = batched_matrix
            .values_mut_for_pattern(linked_matrix.pattern_token())
            .expect("clone retains the linked pattern");
        mos.stamp_grounded_bulk_cached_values(values, &mut batched_rhs);

        assert_eq!(batched_rhs, validated_rhs);
        assert_eq!(batched_matrix.values_mut(), validated_matrix.values_mut());
    }

    #[test]
    fn physical_residual_rows_reuse_matching_device_cache_exactly() {
        let cases = [
            (
                "nmos",
                Mosfet::new_nmos("cached_n".to_string(), 1, 2, 3, 4),
                vec![1.2, 1.8, 0.2, -0.1],
            ),
            (
                "pmos",
                Mosfet::new_pmos("cached_p".to_string(), 1, 2, 3, 4),
                vec![1.2, 0.1, 1.8, 2.0],
            ),
            (
                "source-body tied nmos",
                Mosfet::new_nmos("cached_tied".to_string(), 1, 2, 3, 3),
                vec![1.2, 1.8, 0.2],
            ),
        ];

        for (case, mut cached_mos, candidate) in cases {
            let fresh_mos = cached_mos.clone();
            let linked_matrix = full_matrix(candidate.len());
            cached_mos.link(&linked_matrix);
            let constants = cached_mos.classic_transient_constants();
            cached_mos.update_with_classic_transient_constants(&candidate, &constants);
            assert!(cached_mos.cached_linearization_is_physical(), "{case}");

            let mut cached_ax = vec![0.0; candidate.len()];
            let mut cached_rhs = vec![0.0; candidate.len()];
            cached_mos.add_physical_static_residual_row_terms_at(
                &candidate,
                &constants,
                &mut cached_ax,
                &mut cached_rhs,
            );
            let compact = {
                let cached_terms = cached_mos.classic_cached_static_terms();
                let static_plan = cached_mos
                    .classic_static_stamp_plan(&linked_matrix)
                    .expect("full matrix supports compact MOS stamp");
                let mut compact_ax = vec![0.0; candidate.len()];
                let mut compact_rhs = vec![0.0; candidate.len()];
                Mosfet::add_cached_physical_static_residual_terms(
                    &candidate,
                    &static_plan,
                    &cached_terms,
                    &mut compact_ax,
                    &mut compact_rhs,
                );
                (compact_ax, compact_rhs)
            };
            #[cfg(feature = "parallel")]
            let rowwise = {
                let cached_terms = cached_mos.classic_cached_static_terms();
                let residual_plan = cached_mos.classic_residual_row_plan();
                let mut rowwise_ax = vec![0.0; candidate.len()];
                let mut rowwise_rhs = vec![0.0; candidate.len()];
                for row in 0..candidate.len() {
                    Mosfet::add_cached_physical_static_residual_row_terms(
                        &candidate,
                        &residual_plan,
                        &cached_terms,
                        row,
                        &mut rowwise_ax[row],
                        &mut rowwise_rhs[row],
                    );
                }
                (rowwise_ax, rowwise_rhs)
            };

            let mut evaluated_ax = vec![0.0; candidate.len()];
            let mut evaluated_rhs = vec![0.0; candidate.len()];
            fresh_mos.add_physical_static_residual_row_terms_at(
                &candidate,
                &constants,
                &mut evaluated_ax,
                &mut evaluated_rhs,
            );

            assert_eq!(cached_ax, evaluated_ax, "{case} A*x");
            assert_eq!(cached_rhs, evaluated_rhs, "{case} RHS");
            assert_eq!(compact.0, cached_ax, "{case} compact A*x");
            assert_eq!(compact.1, cached_rhs, "{case} compact RHS");
            #[cfg(feature = "parallel")]
            {
                assert_eq!(rowwise.0, cached_ax, "{case} rowwise A*x");
                assert_eq!(rowwise.1, cached_rhs, "{case} rowwise RHS");
            }
        }
    }
}
