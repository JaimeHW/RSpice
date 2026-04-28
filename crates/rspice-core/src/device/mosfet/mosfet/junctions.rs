use super::*;

impl Mosfet {
    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_junction_thermal_voltage() -> Value {
        VT_REFERENCE.max(1e-12)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn junction_diode_current(
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
        let nvt = Self::body_junction_thermal_voltage();
        if v <= -3.0 * nvt {
            gmin * v - isat
        } else {
            let expv = (v / nvt).clamp(-80.0, 80.0).exp();
            isat * (expv - 1.0) + gmin * v
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn junction_diode_conductance(
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
        let nvt = Self::body_junction_thermal_voltage();
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

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_source_junction_current_and_conductance(
        &self,
        vbs: Value,
    ) -> (Value, Value) {
        let vd = self.body_source_diode_voltage(vbs);
        let isat = self.effective_body_junction_saturation_current(self.source_area);
        (
            Self::junction_diode_current(isat, vd, self.junction_gmin),
            Self::junction_diode_conductance(isat, vd, self.junction_gmin),
        )
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_drain_junction_current_and_conductance(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value) {
        let vd = self.body_drain_diode_voltage(vds, vbs);
        let isat = self.effective_body_junction_saturation_current(self.drain_area);
        (
            Self::junction_diode_current(isat, vd, self.junction_gmin),
            Self::junction_diode_conductance(isat, vd, self.junction_gmin),
        )
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
}
