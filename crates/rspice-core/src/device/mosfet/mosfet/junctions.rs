use super::*;

impl Mosfet {
    /// Evaluate the bulk-junction current and its small-signal conductance
    /// together.  Both quantities use the same normalized voltage and
    /// exponential, so computing them in separate helpers needlessly doubled
    /// the transcendental work in every classic-MOS Newton update.
    #[inline]
    fn junction_diode_current_and_conductance(
        &self,
        isat: Value,
        v: Value,
        gmin: Value,
    ) -> (Value, Value) {
        let isat = if isat.is_finite() && isat > 0.0 {
            isat
        } else {
            0.0
        };
        let gmin = gmin.max(0.0);
        let nvt = self.body_junction_thermal_voltage();
        if self.uses_xyce_classic_reverse_body_junction() && v <= 0.0 {
            let conductance = isat / nvt + gmin;
            return (conductance * v, conductance);
        }
        if v == 0.0 {
            return (0.0, isat / nvt + gmin);
        }
        if v <= -3.0 * nvt {
            return (gmin * v - isat, gmin);
        }

        let expv = (v / nvt).clamp(-80.0, 80.0).exp();
        (isat * (expv - 1.0) + gmin * v, (isat / nvt) * expv + gmin)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_junction_thermal_voltage(&self) -> Value {
        self.vt.max(1e-12)
    }

    #[inline]
    fn uses_xyce_classic_reverse_body_junction(&self) -> bool {
        self.body_junction_model == MosBodyJunctionModel::XyceClassicLinearizedReverse
            && matches!(self.level, 1 | 2 | 3 | 6)
    }

    #[inline]
    #[cfg(test)]
    pub(in crate::device::mosfet::mosfet) fn junction_diode_current(
        &self,
        isat: Value,
        v: Value,
        gmin: Value,
    ) -> Value {
        let isat = if isat.is_finite() && isat > 0.0 {
            isat
        } else {
            0.0
        };
        let gmin = gmin.max(0.0);
        let nvt = self.body_junction_thermal_voltage();
        if self.uses_xyce_classic_reverse_body_junction() && v <= 0.0 {
            return (isat / nvt + gmin) * v;
        }
        if v <= -3.0 * nvt {
            gmin * v - isat
        } else {
            let expv = (v / nvt).clamp(-80.0, 80.0).exp();
            isat * (expv - 1.0) + gmin * v
        }
    }

    #[inline]
    #[cfg(test)]
    pub(in crate::device::mosfet::mosfet) fn junction_diode_conductance(
        &self,
        isat: Value,
        v: Value,
        gmin: Value,
    ) -> Value {
        let isat = if isat.is_finite() && isat > 0.0 {
            isat
        } else {
            0.0
        };
        let gmin = gmin.max(0.0);
        let nvt = self.body_junction_thermal_voltage();
        if self.uses_xyce_classic_reverse_body_junction() && v <= 0.0 {
            return isat / nvt + gmin;
        }
        if v <= -3.0 * nvt {
            gmin
        } else {
            let expv = (v / nvt).clamp(-80.0, 80.0).exp();
            (isat / nvt) * expv + gmin
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn effective_body_junction_saturation_current(
        &self,
        area: Value,
    ) -> Value {
        let area_scaled = if self.js_bulk > 0.0 && area > 0.0 {
            self.js_bulk * area
        } else {
            self.is_bulk
        };
        if area_scaled.is_finite() && area_scaled > 0.0 {
            area_scaled
        } else {
            0.0
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn source_zero_bias_bottom_junction_capacitance(
        &self,
    ) -> Value {
        self.source_bulk_cap_zero_bias
            .unwrap_or(self.cj * self.source_area)
            .max(0.0)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn drain_zero_bias_bottom_junction_capacitance(
        &self,
    ) -> Value {
        self.drain_bulk_cap_zero_bias
            .unwrap_or(self.cj * self.drain_area)
            .max(0.0)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn source_zero_bias_sidewall_junction_capacitance(
        &self,
    ) -> Value {
        (self.cjsw * self.source_perimeter).max(0.0)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn drain_zero_bias_sidewall_junction_capacitance(
        &self,
    ) -> Value {
        (self.cjsw * self.drain_perimeter).max(0.0)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn junction_depletion_scaling(
        arg: Value,
        grading: Value,
    ) -> Value {
        if !arg.is_finite() || arg <= 0.0 {
            return 0.0;
        }
        if (grading - 0.5).abs() < 1e-15 {
            1.0 / arg.sqrt()
        } else {
            (-grading * arg.ln()).exp()
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn junction_depletion_charge_term(
        c0: Value,
        bulk_potential: Value,
        grading: Value,
        arg: Value,
        scaling: Value,
    ) -> Value {
        if c0 <= 0.0 {
            return 0.0;
        }
        if (1.0 - grading).abs() < 1e-12 {
            -c0 * bulk_potential * arg.ln()
        } else {
            c0 * bulk_potential * (1.0 - arg * scaling) / (1.0 - grading)
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn junction_depletion_charge_and_capacitance(
        junction_voltage: Value,
        bottom_zero_bias_cap: Value,
        sidewall_zero_bias_cap: Value,
        bulk_potential: Value,
        bottom_grading: Value,
        sidewall_grading: Value,
        forward_cap_coeff: Value,
    ) -> (Value, Value) {
        let bottom_zero_bias_cap = bottom_zero_bias_cap.max(0.0);
        let sidewall_zero_bias_cap = sidewall_zero_bias_cap.max(0.0);
        if bottom_zero_bias_cap == 0.0 && sidewall_zero_bias_cap == 0.0 {
            return (0.0, 0.0);
        }

        let bulk_potential = bulk_potential.max(1e-12);
        let forward_cap_coeff = forward_cap_coeff.clamp(0.0, 0.999_999_999_999);
        let depletion_corner = forward_cap_coeff * bulk_potential;

        if junction_voltage < depletion_corner {
            let arg = (1.0 - junction_voltage / bulk_potential).max(1e-18);
            let bottom_scale = Self::junction_depletion_scaling(arg, bottom_grading);
            let sidewall_scale = Self::junction_depletion_scaling(arg, sidewall_grading);
            let charge = Self::junction_depletion_charge_term(
                bottom_zero_bias_cap,
                bulk_potential,
                bottom_grading,
                arg,
                bottom_scale,
            ) + Self::junction_depletion_charge_term(
                sidewall_zero_bias_cap,
                bulk_potential,
                sidewall_grading,
                arg,
                sidewall_scale,
            );
            let capacitance = (bottom_zero_bias_cap * bottom_scale
                + sidewall_zero_bias_cap * sidewall_scale)
                .max(0.0);
            return (charge, capacitance);
        }

        let arg = (1.0 - forward_cap_coeff).max(1e-18);
        let bottom_scale = Self::junction_depletion_scaling(arg, bottom_grading);
        let sidewall_scale = Self::junction_depletion_scaling(arg, sidewall_grading);
        let f2 = bottom_zero_bias_cap
            * (1.0 - forward_cap_coeff * (1.0 + bottom_grading))
            * bottom_scale
            / arg
            + sidewall_zero_bias_cap
                * (1.0 - forward_cap_coeff * (1.0 + sidewall_grading))
                * sidewall_scale
                / arg;
        let f3 = bottom_zero_bias_cap * bottom_grading * bottom_scale / arg / bulk_potential
            + sidewall_zero_bias_cap * sidewall_grading * sidewall_scale / arg / bulk_potential;
        let edge_charge = Self::junction_depletion_charge_term(
            bottom_zero_bias_cap,
            bulk_potential,
            bottom_grading,
            arg,
            bottom_scale,
        ) + Self::junction_depletion_charge_term(
            sidewall_zero_bias_cap,
            bulk_potential,
            sidewall_grading,
            arg,
            sidewall_scale,
        );
        let f4 =
            edge_charge - 0.5 * f3 * depletion_corner * depletion_corner - depletion_corner * f2;
        let charge = f4 + junction_voltage * (f2 + 0.5 * junction_voltage * f3);
        let capacitance = (f2 + junction_voltage * f3).max(0.0);
        (charge, capacitance)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_diode_nodes(&self) -> (NodeId, NodeId) {
        match self.mos_type {
            MosType::Nmos => (self.node_bulk, self.node_source),
            MosType::Pmos => (self.node_source, self.node_bulk),
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_diode_nodes(&self) -> (NodeId, NodeId) {
        match self.mos_type {
            MosType::Nmos => (self.node_bulk, self.node_drain),
            MosType::Pmos => (self.node_drain, self.node_bulk),
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_diode_voltage(&self, vbs: Value) -> Value {
        match self.mos_type {
            MosType::Nmos => vbs,
            MosType::Pmos => -vbs,
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_diode_voltage(
        &self,
        vds: Value,
        vbs: Value,
    ) -> Value {
        let vbd = vbs - vds;
        match self.mos_type {
            MosType::Nmos => vbd,
            MosType::Pmos => -vbd,
        }
    }

    #[inline]
    pub(crate) fn body_source_charge_nodes(&self) -> (NodeId, NodeId) {
        self.body_source_diode_nodes()
    }

    #[inline]
    pub(crate) fn body_drain_charge_nodes(&self) -> (NodeId, NodeId) {
        self.body_drain_diode_nodes()
    }

    #[inline]
    pub(crate) fn body_source_charge_branch_voltage(&self, vbs: Value) -> Value {
        self.body_source_diode_voltage(vbs)
    }

    #[inline]
    pub(crate) fn body_drain_charge_branch_voltage(&self, vds: Value, vbs: Value) -> Value {
        self.body_drain_diode_voltage(vds, vbs)
    }

    /// Structural presence mask for the source/drain depletion-charge branches.
    ///
    /// Classic MOS models commonly omit CJ/CJSW and explicit CBD/CBS.  In
    /// that case these branches are identically zero for every bias and can be
    /// left out of the transient hot path.  Bit 0 denotes source-body charge;
    /// bit 1 denotes drain-body charge.
    #[inline]
    pub(crate) fn body_junction_charge_mask(&self) -> u8 {
        let source_active = self.source_zero_bias_bottom_junction_capacitance() > 0.0
            || self.source_zero_bias_sidewall_junction_capacitance() > 0.0;
        let drain_active = self.drain_zero_bias_bottom_junction_capacitance() > 0.0
            || self.drain_zero_bias_sidewall_junction_capacitance() > 0.0;
        u8::from(source_active) | (u8::from(drain_active) << 1)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_junction_current_and_conductance(
        &self,
        vbs: Value,
    ) -> (Value, Value) {
        let vd = self.body_source_diode_voltage(vbs);
        let isat = self.effective_body_junction_saturation_current(self.source_area);
        self.junction_diode_current_and_conductance(isat, vd, self.junction_gmin)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_junction_current_and_conductance(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value) {
        let vd = self.body_drain_diode_voltage(vds, vbs);
        let isat = self.effective_body_junction_saturation_current(self.drain_area);
        self.junction_diode_current_and_conductance(isat, vd, self.junction_gmin)
    }

    #[inline]
    pub(crate) fn body_source_junction_charge_and_capacitance_at(
        &self,
        vbs: Value,
    ) -> (Value, Value) {
        Self::junction_depletion_charge_and_capacitance(
            self.body_source_diode_voltage(vbs),
            self.source_zero_bias_bottom_junction_capacitance(),
            self.source_zero_bias_sidewall_junction_capacitance(),
            self.pb,
            self.mj,
            self.mjsw,
            self.fc,
        )
    }

    #[inline]
    pub(crate) fn body_drain_junction_charge_and_capacitance_at(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value) {
        Self::junction_depletion_charge_and_capacitance(
            self.body_drain_diode_voltage(vds, vbs),
            self.drain_zero_bias_bottom_junction_capacitance(),
            self.drain_zero_bias_sidewall_junction_capacitance(),
            self.pb,
            self.mj,
            self.mjsw,
            self.fc,
        )
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_junction_linearization(
        &self,
        vbs: Value,
    ) -> (NodeId, NodeId, Value, Value) {
        let (anode, cathode) = self.body_source_diode_nodes();
        let vd = self.body_source_diode_voltage(vbs);
        let (id, gd) = self.body_source_junction_current_and_conductance(vbs);
        let ieq = id - gd * vd;
        (anode, cathode, gd, ieq)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_junction_linearization_cached(
        &self,
        vbs: Value,
        cache_matches: bool,
    ) -> (NodeId, NodeId, Value, Value) {
        if !cache_matches {
            return self.body_source_junction_linearization(vbs);
        }
        let (anode, cathode) = self.body_source_diode_nodes();
        let vd = self.body_source_diode_voltage(vbs);
        (anode, cathode, self.gbs, self.ibs - self.gbs * vd)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_junction_linearization(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (NodeId, NodeId, Value, Value) {
        let (anode, cathode) = self.body_drain_diode_nodes();
        let vd = self.body_drain_diode_voltage(vds, vbs);
        let (id, gd) = self.body_drain_junction_current_and_conductance(vds, vbs);
        let ieq = id - gd * vd;
        (anode, cathode, gd, ieq)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_junction_linearization_cached(
        &self,
        vds: Value,
        vbs: Value,
        cache_matches: bool,
    ) -> (NodeId, NodeId, Value, Value) {
        if !cache_matches {
            return self.body_drain_junction_linearization(vds, vbs);
        }
        let (anode, cathode) = self.body_drain_diode_nodes();
        let vd = self.body_drain_diode_voltage(vds, vbs);
        (anode, cathode, self.gbd, self.ibd - self.gbd * vd)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn stamp_diode_linearization(
        matrix: &mut impl MatrixStamper,
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if gd == 0.0 && ieq == 0.0 {
            return;
        }

        matrix.stamp(anode, anode, gd);
        matrix.stamp(anode, cathode, -gd);
        matrix.stamp(cathode, anode, -gd);
        matrix.stamp(cathode, cathode, gd);
        matrix.stamp_rhs(anode, -ieq);
        matrix.stamp_rhs(cathode, ieq);
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn stamp_diode_linearization_direct(
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if gd == 0.0 && ieq == 0.0 {
            return;
        }

        if anode > 0 {
            matrix.add(anode - 1, anode - 1, gd);
            if cathode > 0 {
                matrix.add(anode - 1, cathode - 1, -gd);
            }
            rhs[anode - 1] -= ieq;
        }
        if cathode > 0 {
            if anode > 0 {
                matrix.add(cathode - 1, anode - 1, -gd);
            }
            matrix.add(cathode - 1, cathode - 1, gd);
            rhs[cathode - 1] += ieq;
        }
    }

    /// Stamp a source-body junction through its prelinked diagonal when the
    /// bulk is ground. Non-grounded bulks retain the general sparse path.
    #[inline]
    pub(in crate::device::mosfet::mosfet) fn stamp_body_source_linearization_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if self.node_bulk != 0 {
            Self::stamp_diode_linearization_direct(matrix, rhs, anode, cathode, gd, ieq);
            return;
        }
        if gd == 0.0 && ieq == 0.0 {
            return;
        }
        if let Some(index) = self.indices.ss {
            matrix.stamp_direct(index, gd);
        }
        if anode > 0 {
            rhs[anode - 1] -= ieq;
        }
        if cathode > 0 {
            rhs[cathode - 1] += ieq;
        }
    }

    /// Stamp a drain-body junction through its prelinked diagonal when the
    /// bulk is ground. Non-grounded bulks retain the general sparse path.
    #[inline]
    pub(in crate::device::mosfet::mosfet) fn stamp_body_drain_linearization_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if self.node_bulk != 0 {
            Self::stamp_diode_linearization_direct(matrix, rhs, anode, cathode, gd, ieq);
            return;
        }
        if gd == 0.0 && ieq == 0.0 {
            return;
        }
        if let Some(index) = self.indices.dd {
            matrix.stamp_direct(index, gd);
        }
        if anode > 0 {
            rhs[anode - 1] -= ieq;
        }
        if cathode > 0 {
            rhs[cathode - 1] += ieq;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: Value, expected: Value, label: &str) {
        let tol = 1.0e-12 * actual.abs().max(expected.abs()).max(1.0);
        assert!(
            (actual - expected).abs() <= tol,
            "{label}: actual={actual:.12e} expected={expected:.12e} tol={tol:.12e}"
        );
    }

    #[test]
    fn xyce_classic_reverse_body_junction_linearizes_from_zero_bias() {
        let mut mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0).with_level(6);
        mos.set_body_junction_model(MosBodyJunctionModel::XyceClassicLinearizedReverse);

        let isat = 1.0e-14;
        let gmin = 1.0e-12;
        let reverse_voltage = -2.62;
        let expected_conductance = isat / mos.body_junction_thermal_voltage() + gmin;

        assert_close(
            mos.junction_diode_current(isat, reverse_voltage, gmin),
            expected_conductance * reverse_voltage,
            "Xyce reverse current",
        );
        assert_close(
            mos.junction_diode_conductance(isat, reverse_voltage, gmin),
            expected_conductance,
            "Xyce reverse conductance",
        );
    }

    #[test]
    fn ngspice_reverse_body_junction_keeps_deep_reverse_clamp() {
        let mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0).with_level(6);

        let isat = 1.0e-14;
        let gmin = 1.0e-12;
        let reverse_voltage = -2.62;

        assert_close(
            mos.junction_diode_current(isat, reverse_voltage, gmin),
            gmin * reverse_voltage - isat,
            "ngspice reverse current",
        );
        assert_close(
            mos.junction_diode_conductance(isat, reverse_voltage, gmin),
            gmin,
            "ngspice reverse conductance",
        );
    }

    #[test]
    fn fused_body_junction_evaluation_matches_separate_laws_bit_exactly() {
        let mut ngspice = Mosfet::new_nmos("mn".to_string(), 1, 2, 3, 0).with_level(1);
        let mut xyce = ngspice.clone();
        xyce.set_body_junction_model(MosBodyJunctionModel::XyceClassicLinearizedReverse);
        ngspice.vt = 0.026_001;
        xyce.vt = ngspice.vt;

        for mos in [&ngspice, &xyce] {
            for isat in [0.0, 1.0e-14, 2.5e-9, Value::NAN] {
                for voltage in [-2.62, -0.1, -0.078_003, -1.0e-9, 0.0, 0.1, 0.9] {
                    let gmin = 1.0e-12;
                    let (current, conductance) =
                        mos.junction_diode_current_and_conductance(isat, voltage, gmin);
                    assert_eq!(
                        current.to_bits(),
                        mos.junction_diode_current(isat, voltage, gmin).to_bits(),
                        "current mismatch for isat={isat:e}, voltage={voltage:e}"
                    );
                    assert_eq!(
                        conductance.to_bits(),
                        mos.junction_diode_conductance(isat, voltage, gmin)
                            .to_bits(),
                        "conductance mismatch for isat={isat:e}, voltage={voltage:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn body_junction_charge_mask_tracks_only_structurally_active_branches() {
        let mut mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0);
        assert_eq!(mos.body_junction_charge_mask(), 0);

        mos.cj = 2.0e-4;
        mos.source_area = 1.0e-12;
        assert_eq!(mos.body_junction_charge_mask(), 1);

        mos.drain_area = 2.0e-12;
        assert_eq!(mos.body_junction_charge_mask(), 3);

        mos.cj = 0.0;
        mos.source_area = 0.0;
        mos.drain_area = 0.0;
        mos.source_bulk_cap_zero_bias = Some(3.0e-15);
        assert_eq!(mos.body_junction_charge_mask(), 1);

        mos.source_bulk_cap_zero_bias = None;
        mos.cjsw = 5.0e-10;
        mos.drain_perimeter = 4.0e-6;
        assert_eq!(mos.body_junction_charge_mask(), 2);
    }

    #[test]
    fn grounded_bulk_linked_junction_stamps_match_general_path_exactly() {
        use crate::device::NonlinearDevice;

        let candidate = [1.2, 1.8];
        for mut mos in [
            Mosfet::new_nmos("mn".to_string(), 1, 2, 0, 0),
            Mosfet::new_pmos("mp".to_string(), 1, 2, 0, 0),
        ] {
            let triplets: Vec<_> = (0..2)
                .flat_map(|row| (0..2).map(move |col| (row, col, 0.0)))
                .collect();
            let mut linked = StaticMatrix::from_triplets(2, 2, &triplets).expect("linked matrix");
            let mut general = StaticMatrix::from_triplets(2, 2, &triplets).expect("general matrix");
            mos.link(&linked);
            mos.update(&candidate);
            let (source_anode, source_cathode, gbs, ieq_bs) =
                mos.body_source_junction_linearization_cached(mos.eval_vbs, true);
            let (drain_anode, drain_cathode, gbd, ieq_bd) =
                mos.body_drain_junction_linearization_cached(mos.eval_vds, mos.eval_vbs, true);
            let mut linked_rhs = vec![0.0; 2];
            let mut general_rhs = vec![0.0; 2];

            mos.stamp_body_source_linearization_direct(
                &mut linked,
                &mut linked_rhs,
                source_anode,
                source_cathode,
                gbs,
                ieq_bs,
            );
            mos.stamp_body_drain_linearization_direct(
                &mut linked,
                &mut linked_rhs,
                drain_anode,
                drain_cathode,
                gbd,
                ieq_bd,
            );
            Mosfet::stamp_diode_linearization_direct(
                &mut general,
                &mut general_rhs,
                source_anode,
                source_cathode,
                gbs,
                ieq_bs,
            );
            Mosfet::stamp_diode_linearization_direct(
                &mut general,
                &mut general_rhs,
                drain_anode,
                drain_cathode,
                gbd,
                ieq_bd,
            );

            assert_eq!(linked_rhs, general_rhs);
            assert_eq!(linked.values_mut(), general.values_mut());
        }
    }
}
