//! Stamping the bias-independent part of the MNA system.
//!
//! Covers the contributions whose coefficients do not depend on the solution:
//! conductances, branch KCL and voltage rows, coupled inductors, and
//! transmission-line ports. Each stamp comes in a triplet form used while the
//! sparsity pattern is still being discovered and a `_direct` form used once
//! the pattern is frozen and every entry has a known position, which is what
//! makes repeated stamping in the Newton loop cheap.

use super::*;

impl CircuitData {
    #[inline]
    fn stamp_global_shunt_direct(&self, matrix: &mut StaticMatrix) {
        if self.global_shunt_conductance == 0.0 {
            return;
        }
        for index in 0..self.num_nodes {
            if !self.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, self.global_shunt_conductance);
            }
        }
    }

    #[inline]
    fn stamp_global_shunt(&self, matrix: &mut TripletMatrix) {
        if self.global_shunt_conductance == 0.0 {
            return;
        }
        for index in 0..self.num_nodes {
            if !self.is_non_electrical_state_matrix_index(index) {
                matrix.push(index, index, self.global_shunt_conductance);
            }
        }
    }

    /// Provide a finite operating-point constraint for private LEVEL=1 Core
    /// states.  The transient companion replaces this identity with the
    /// physical hidden magnetization equation at every Newton assembly.
    #[inline]
    fn stamp_xyce_core_hidden_state_identity_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        for binding in &self.jiles_atherton_inductors {
            let m_var_scaling = binding.device.xyce_core_m_var_scaling();
            let r_var_scaling = binding.device.xyce_core_r_var_scaling();
            if let Some(slot) = binding.hidden_m_slot {
                let index = self.get_hidden_state_matrix_index(slot);
                matrix.add(index - 1, index - 1, 1.0);
                if index <= rhs.len() {
                    rhs[index - 1] += binding.device.magnetization() / m_var_scaling;
                }
            }
            if let Some(rate_slot) = binding.hidden_r_slot {
                let rate_index = self.get_hidden_state_matrix_index(rate_slot);
                matrix.add(rate_index - 1, rate_index - 1, 1.0);
                if rate_index <= rhs.len() {
                    rhs[rate_index - 1] +=
                        binding.device.xyce_core_level1_rate_debug() / r_var_scaling;
                }
            }
        }
        for group in &self.xyce_core_groups {
            let m_var_scaling = group.device.xyce_core_m_var_scaling();
            let r_var_scaling = group.device.xyce_core_r_var_scaling();
            if let Some(slot) = group.hidden_m_slot {
                let index = self.get_hidden_state_matrix_index(slot);
                matrix.add(index - 1, index - 1, 1.0);
                if index <= rhs.len() {
                    rhs[index - 1] += group.device.magnetization() / m_var_scaling;
                }
            }
            if let Some(rate_slot) = group.hidden_r_slot {
                let rate_index = self.get_hidden_state_matrix_index(rate_slot);
                matrix.add(rate_index - 1, rate_index - 1, 1.0);
                if rate_index <= rhs.len() {
                    rhs[rate_index - 1] +=
                        group.device.xyce_core_level1_rate_debug() / r_var_scaling;
                }
            }
        }
    }

    #[inline]
    fn stamp_xyce_core_hidden_state_identity(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for binding in &self.jiles_atherton_inductors {
            let m_var_scaling = binding.device.xyce_core_m_var_scaling();
            let r_var_scaling = binding.device.xyce_core_r_var_scaling();
            if let Some(slot) = binding.hidden_m_slot {
                let index = self.get_hidden_state_matrix_index(slot);
                matrix.push(index - 1, index - 1, 1.0);
                if index <= rhs.len() {
                    rhs[index - 1] += binding.device.magnetization() / m_var_scaling;
                }
            }
            if let Some(rate_slot) = binding.hidden_r_slot {
                let rate_index = self.get_hidden_state_matrix_index(rate_slot);
                matrix.push(rate_index - 1, rate_index - 1, 1.0);
                if rate_index <= rhs.len() {
                    rhs[rate_index - 1] +=
                        binding.device.xyce_core_level1_rate_debug() / r_var_scaling;
                }
            }
        }
        for group in &self.xyce_core_groups {
            let m_var_scaling = group.device.xyce_core_m_var_scaling();
            let r_var_scaling = group.device.xyce_core_r_var_scaling();
            if let Some(slot) = group.hidden_m_slot {
                let index = self.get_hidden_state_matrix_index(slot);
                matrix.push(index - 1, index - 1, 1.0);
                if index <= rhs.len() {
                    rhs[index - 1] += group.device.magnetization() / m_var_scaling;
                }
            }
            if let Some(rate_slot) = group.hidden_r_slot {
                let rate_index = self.get_hidden_state_matrix_index(rate_slot);
                matrix.push(rate_index - 1, rate_index - 1, 1.0);
                if rate_index <= rhs.len() {
                    rhs[rate_index - 1] +=
                        group.device.xyce_core_level1_rate_debug() / r_var_scaling;
                }
            }
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_tline_port_direct(
        matrix: &mut StaticMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        g: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -g);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -g);
            }
            matrix.add(node_neg - 1, node_neg - 1, g);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_tline_port_triplet(
        matrix: &mut TripletMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        g: Value,
    ) {
        if node_pos > 0 {
            matrix.push(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.push(node_pos - 1, node_neg - 1, -g);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.push(node_neg - 1, node_pos - 1, -g);
            }
            matrix.push(node_neg - 1, node_neg - 1, g);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_tlines_dc_direct(&self, matrix: &mut StaticMatrix) {
        for tl in &self.tlines {
            if tl.has_txl_runtime() {
                self.stamp_txl_dc_direct(matrix, tl);
                continue;
            }
            if tl.is_zero_length_pass_through() {
                self.stamp_zero_length_branch_dc_direct(matrix, tl);
                continue;
            }
            if tl.rg_branch_ordinals().is_some() {
                self.stamp_rg_branch_direct(matrix, tl);
                continue;
            }
            if tl.ltra_branch_ordinals().is_some() {
                self.stamp_ltra_branch_dc_direct(matrix, tl);
                continue;
            }
            let g_series = tl.dc_series_conductance();
            // DC fallback: couple near/far conductors through equivalent series path.
            // This preserves operating-point continuity across the line and avoids
            // nonphysical port-to-ground shunts.
            Self::stamp_tline_port_direct(matrix, tl.node1_pos, tl.node2_pos, g_series);
            Self::stamp_tline_port_direct(matrix, tl.node1_neg, tl.node2_neg, g_series);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_coupled_tlines_dc_direct(&self, matrix: &mut StaticMatrix) {
        for tline in &self.coupled_tlines {
            if tline.native_branch_ordinals().is_some() {
                self.stamp_grounded_cpl_branch_dc_direct(matrix, tline);
                continue;
            }

            for conductor in 0..tline.conductors() {
                let g_series = 1.0 / tline.dc_series_resistance(conductor);
                Self::stamp_tline_port_direct(
                    matrix,
                    tline.near_nodes[conductor],
                    tline.far_nodes[conductor],
                    g_series,
                );
            }
        }
    }

    #[inline]
    fn stamp_grounded_cpl_branch_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        tline: &crate::device::CoupledTransmissionLine,
    ) {
        let Some(branches) = tline.native_branch_ordinals() else {
            return;
        };

        for conductor in 0..tline.conductors() {
            let Some((b1_ordinal, b2_ordinal)) = branches.conductor(conductor) else {
                continue;
            };
            let b1 = self.get_branch_matrix_index(b1_ordinal);
            let b2 = self.get_branch_matrix_index(b2_ordinal);
            let r_series = tline.dc_series_resistance(conductor);

            Self::stamp_branch_kcl_direct(matrix, tline.near_nodes[conductor], 0, b1, 1.0);
            Self::stamp_branch_kcl_direct(matrix, tline.far_nodes[conductor], 0, b2, 1.0);
            matrix.add(b1 - 1, b1 - 1, 1.0);
            matrix.add(b1 - 1, b2 - 1, 1.0);
            Self::stamp_branch_voltage_row_direct(
                matrix,
                b2,
                tline.near_nodes[conductor],
                0,
                tline.far_nodes[conductor],
                0,
                1.0,
                -1.0,
            );
            matrix.add(b2 - 1, b1 - 1, -r_series);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_tlines_dc(&self, matrix: &mut TripletMatrix) {
        for tl in &self.tlines {
            if tl.has_txl_runtime() {
                self.stamp_txl_dc(matrix, tl);
                continue;
            }
            if tl.is_zero_length_pass_through() {
                self.stamp_zero_length_branch_dc(matrix, tl);
                continue;
            }
            if tl.rg_branch_ordinals().is_some() {
                self.stamp_rg_branch_triplet(matrix, tl);
                continue;
            }
            if tl.ltra_branch_ordinals().is_some() {
                self.stamp_ltra_branch_dc(matrix, tl);
                continue;
            }
            let g_series = tl.dc_series_conductance();
            Self::stamp_tline_port_triplet(matrix, tl.node1_pos, tl.node2_pos, g_series);
            Self::stamp_tline_port_triplet(matrix, tl.node1_neg, tl.node2_neg, g_series);
        }
    }

    #[inline]
    fn stamp_txl_dc_direct(&self, matrix: &mut StaticMatrix, tl: &crate::device::TransmissionLine) {
        let Some((br1_ordinal, br2_ordinal)) = tl.txl_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);
        let r_series = tl.dc_series_resistance();

        Self::stamp_branch_kcl_direct(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_direct(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.add(br1 - 1, br1 - 1, 1.0);
        matrix.add(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_direct(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
        matrix.add(br2 - 1, br1 - 1, -r_series);
    }

    #[inline]
    fn stamp_ltra_branch_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let Some((br1_ordinal, br2_ordinal)) = tl.ltra_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);
        let r_series = tl.dc_series_resistance();

        Self::stamp_branch_kcl_direct(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_direct(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.add(br1 - 1, br1 - 1, 1.0);
        matrix.add(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_direct(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
        matrix.add(br2 - 1, br1 - 1, -r_series);
    }

    /// Stamp the exact memoryless RG two-port on its reserved branch rows.
    ///
    /// With `A = cosh(theta)`, `B = Z0*sinh(theta)` and `C = sinh(theta)/Z0`
    /// for `theta = len*sqrt(R*G)`, and with both branch currents defined as
    /// entering the device at their positive terminals (ngspice `ltraload.c`
    /// `LTRA_MOD_RG`, Xyce `N_DEV_LTRA`):
    ///
    /// ```text
    /// row ibr1:  V1 - A*V2 + B*I2 = 0
    /// row ibr2:  I1 + A*I2 - C*V2 = 0
    /// KCL:       node1 += I1, node2 += I2
    /// ```
    ///
    /// The coefficients are real and frequency independent, so this identical
    /// stamp is the DC, transient and small-signal load. RSpice omits
    /// ngspice's `(1 + GMIN)` scaling of `B` and `C`: that is a matrix
    /// conditioning hack on a nonsingular two-port, and applying it would make
    /// the physical answer depend on a solver option.
    #[inline]
    fn stamp_rg_branch_direct(
        &self,
        matrix: &mut StaticMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let (Some((br1_ordinal, br2_ordinal)), Some(two_port)) =
            (tl.rg_branch_ordinals(), tl.ltra_rg_two_port())
        else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);

        Self::stamp_branch_kcl_direct(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_direct(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        Self::stamp_branch_voltage_row_direct(
            matrix,
            br1,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -two_port.cosh_theta,
        );
        matrix.add(br1 - 1, br2 - 1, two_port.transfer_impedance);
        Self::stamp_branch_voltage_row_direct(
            matrix,
            br2,
            tl.node2_pos,
            tl.node2_neg,
            0,
            0,
            -two_port.transfer_admittance,
            0.0,
        );
        matrix.add(br2 - 1, br1 - 1, 1.0);
        matrix.add(br2 - 1, br2 - 1, two_port.cosh_theta);
    }

    #[inline]
    fn stamp_rg_branch_triplet(
        &self,
        matrix: &mut TripletMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let (Some((br1_ordinal, br2_ordinal)), Some(two_port)) =
            (tl.rg_branch_ordinals(), tl.ltra_rg_two_port())
        else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);

        Self::stamp_branch_kcl_triplet(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_triplet(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        Self::stamp_branch_voltage_row_triplet(
            matrix,
            br1,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -two_port.cosh_theta,
        );
        matrix.push(br1 - 1, br2 - 1, two_port.transfer_impedance);
        Self::stamp_branch_voltage_row_triplet(
            matrix,
            br2,
            tl.node2_pos,
            tl.node2_neg,
            0,
            0,
            -two_port.transfer_admittance,
            0.0,
        );
        matrix.push(br2 - 1, br1 - 1, 1.0);
        matrix.push(br2 - 1, br2 - 1, two_port.cosh_theta);
    }

    #[inline]
    fn stamp_zero_length_branch_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let Some((br1_ordinal, br2_ordinal)) = tl.zero_length_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);

        // An RC/RG LTRA line with LEN=0 is exactly an ideal through
        // connection: I1 + I2 = 0 and V1 - V2 = 0.  Keep these as explicit
        // MNA branch equations so the short remains exact at every scale.
        Self::stamp_branch_kcl_direct(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_direct(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.add(br1 - 1, br1 - 1, 1.0);
        matrix.add(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_direct(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
    }

    #[inline]
    fn stamp_txl_dc(&self, matrix: &mut TripletMatrix, tl: &crate::device::TransmissionLine) {
        let Some((br1_ordinal, br2_ordinal)) = tl.txl_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);
        let r_series = tl.dc_series_resistance();

        Self::stamp_branch_kcl_triplet(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_triplet(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.push(br1 - 1, br1 - 1, 1.0);
        matrix.push(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_triplet(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
        matrix.push(br2 - 1, br1 - 1, -r_series);
    }

    #[inline]
    fn stamp_ltra_branch_dc(
        &self,
        matrix: &mut TripletMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let Some((br1_ordinal, br2_ordinal)) = tl.ltra_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);
        let r_series = tl.dc_series_resistance();

        Self::stamp_branch_kcl_triplet(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_triplet(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.push(br1 - 1, br1 - 1, 1.0);
        matrix.push(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_triplet(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
        matrix.push(br2 - 1, br1 - 1, -r_series);
    }

    #[inline]
    fn stamp_zero_length_branch_dc(
        &self,
        matrix: &mut TripletMatrix,
        tl: &crate::device::TransmissionLine,
    ) {
        let Some((br1_ordinal, br2_ordinal)) = tl.zero_length_branch_ordinals() else {
            return;
        };
        let br1 = self.get_branch_matrix_index(br1_ordinal);
        let br2 = self.get_branch_matrix_index(br2_ordinal);

        Self::stamp_branch_kcl_triplet(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_branch_kcl_triplet(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.push(br1 - 1, br1 - 1, 1.0);
        matrix.push(br1 - 1, br2 - 1, 1.0);
        Self::stamp_branch_voltage_row_triplet(
            matrix,
            br2,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            1.0,
            -1.0,
        );
    }

    #[inline]
    fn stamp_branch_kcl_direct(
        matrix: &mut StaticMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        branch: usize,
        coeff: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, branch - 1, coeff);
        }
        if node_neg > 0 {
            matrix.add(node_neg - 1, branch - 1, -coeff);
        }
    }

    #[inline]
    fn stamp_branch_kcl_triplet(
        matrix: &mut TripletMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        branch: usize,
        coeff: Value,
    ) {
        if node_pos > 0 {
            matrix.push(node_pos - 1, branch - 1, coeff);
        }
        if node_neg > 0 {
            matrix.push(node_neg - 1, branch - 1, -coeff);
        }
    }

    #[inline]
    fn stamp_branch_voltage_row_direct(
        matrix: &mut StaticMatrix,
        row: usize,
        p1: NodeId,
        n1: NodeId,
        p2: NodeId,
        n2: NodeId,
        c1: Value,
        c2: Value,
    ) {
        if p1 > 0 {
            matrix.add(row - 1, p1 - 1, c1);
        }
        if n1 > 0 {
            matrix.add(row - 1, n1 - 1, -c1);
        }
        if p2 > 0 {
            matrix.add(row - 1, p2 - 1, c2);
        }
        if n2 > 0 {
            matrix.add(row - 1, n2 - 1, -c2);
        }
    }

    #[inline]
    fn stamp_branch_voltage_row_triplet(
        matrix: &mut TripletMatrix,
        row: usize,
        p1: NodeId,
        n1: NodeId,
        p2: NodeId,
        n2: NodeId,
        c1: Value,
        c2: Value,
    ) {
        if p1 > 0 {
            matrix.push(row - 1, p1 - 1, c1);
        }
        if n1 > 0 {
            matrix.push(row - 1, n1 - 1, -c1);
        }
        if p2 > 0 {
            matrix.push(row - 1, p2 - 1, c2);
        }
        if n2 > 0 {
            matrix.push(row - 1, n2 - 1, -c2);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_coupled_tlines_dc(&self, matrix: &mut TripletMatrix) {
        for tline in &self.coupled_tlines {
            if tline.native_branch_ordinals().is_some() {
                self.stamp_grounded_cpl_branch_dc(matrix, tline);
                continue;
            }

            for conductor in 0..tline.conductors() {
                let g_series = 1.0 / tline.dc_series_resistance(conductor);
                Self::stamp_tline_port_triplet(
                    matrix,
                    tline.near_nodes[conductor],
                    tline.far_nodes[conductor],
                    g_series,
                );
            }
        }
    }

    #[inline]
    fn stamp_grounded_cpl_branch_dc(
        &self,
        matrix: &mut TripletMatrix,
        tline: &crate::device::CoupledTransmissionLine,
    ) {
        let Some(branches) = tline.native_branch_ordinals() else {
            return;
        };

        for conductor in 0..tline.conductors() {
            let Some((b1_ordinal, b2_ordinal)) = branches.conductor(conductor) else {
                continue;
            };
            let b1 = self.get_branch_matrix_index(b1_ordinal);
            let b2 = self.get_branch_matrix_index(b2_ordinal);
            let r_series = tline.dc_series_resistance(conductor);

            Self::stamp_branch_kcl_triplet(matrix, tline.near_nodes[conductor], 0, b1, 1.0);
            Self::stamp_branch_kcl_triplet(matrix, tline.far_nodes[conductor], 0, b2, 1.0);
            matrix.push(b1 - 1, b1 - 1, 1.0);
            matrix.push(b1 - 1, b2 - 1, 1.0);
            Self::stamp_branch_voltage_row_triplet(
                matrix,
                b2,
                tline.near_nodes[conductor],
                0,
                tline.far_nodes[conductor],
                0,
                1.0,
                -1.0,
            );
            matrix.push(b2 - 1, b1 - 1, -r_series);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_coupled_inductors_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            binding.device.stamp_dc_short(&mut stamper, &mut []);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_coupled_inductors_dc(
        &self,
        matrix: &mut TripletMatrix,
        rhs: &mut [Value],
    ) {
        for binding in &self.coupled_inductor_pairs {
            let br1 = self.get_branch_matrix_index(binding.branch1_ordinal);
            let br2 = self.get_branch_matrix_index(binding.branch2_ordinal);
            let device = &binding.device;

            if device.node1_pos > 0 {
                matrix.push(br1 - 1, device.node1_pos - 1, 1.0);
                matrix.push(device.node1_pos - 1, br1 - 1, 1.0);
            }
            if device.node1_neg > 0 {
                matrix.push(br1 - 1, device.node1_neg - 1, -1.0);
                matrix.push(device.node1_neg - 1, br1 - 1, -1.0);
            }
            if device.node2_pos > 0 {
                matrix.push(br2 - 1, device.node2_pos - 1, 1.0);
                matrix.push(device.node2_pos - 1, br2 - 1, 1.0);
            }
            if device.node2_neg > 0 {
                matrix.push(br2 - 1, device.node2_neg - 1, -1.0);
                matrix.push(device.node2_neg - 1, br2 - 1, -1.0);
            }

            rhs[br1 - 1] = 0.0;
            rhs[br2 - 1] = 0.0;
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_multi_winding_transformers_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding.device.stamp_dc_short(&mut stamper, &mut []);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_multi_winding_transformers_dc(
        &self,
        matrix: &mut TripletMatrix,
        rhs: &mut [Value],
    ) {
        for binding in &self.multi_winding_transformers {
            for (winding_idx, &(pos, neg)) in binding.device.nodes.iter().enumerate() {
                let br = self.get_branch_matrix_index(binding.branch_ordinals[winding_idx]);
                if pos > 0 {
                    matrix.push(br - 1, pos - 1, 1.0);
                    matrix.push(pos - 1, br - 1, 1.0);
                }
                if neg > 0 {
                    matrix.push(br - 1, neg - 1, -1.0);
                    matrix.push(neg - 1, br - 1, -1.0);
                }
                rhs[br - 1] = 0.0;
            }
        }
    }

    /// Stamp all linear devices for DC analysis using O(1) direct stamping
    pub fn stamp_dc_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.stamp_global_shunt_direct(matrix);
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.resistor_branches
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.capacitors
            .stamp_ic_operating_point_direct(matrix, rhs, num_nodes);
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.voltage_sources
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all(rhs);

        // Stamp controlled sources
        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);

        // Transmission-line DC fallback: couple near/far conductors via series path.
        self.stamp_tlines_dc_direct(matrix);
        self.stamp_coupled_tlines_dc_direct(matrix);
        self.stamp_xyce_core_hidden_state_identity_direct(matrix, rhs);
    }

    /// Stamp linear devices for transient Newton iterations.
    ///
    /// This intentionally excludes transmission-line DC fallback conductances:
    /// transient delay behavior is handled by dedicated tline companion stamps.
    /// It also excludes inductors, coupled pairs, and multi-winding
    /// transformers: their transient behavior is owned entirely by the
    /// companion stamps. Stamping their DC shorts here as well doubled the
    /// branch incidence (`2*v - r_eq*i = -v_eq`), which silently corrupted
    /// every transient solve containing an inductor.
    fn stamp_transient_linear_base_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.stamp_global_shunt_direct(matrix);
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.resistor_branches
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.voltage_sources
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all(rhs);

        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
    }

    pub fn stamp_transient_linear_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.stamp_transient_linear_base_direct(matrix, rhs);
    }

    /// Stamp the linear part of the t=0 transient operating point.
    ///
    /// Time-varying independent sources are evaluated at the requested
    /// transient time, while transmission lines use their DC fallback
    /// conductance so their far ends start from the correct operating point
    /// before delayed-wave companions take over for t > 0. Inductors without
    /// `IC=` are DC shorts here, while `IC=` inductors are constrained to the
    /// requested branch current using Xyce's transient-start operating-point
    /// semantics. Grouped Xyce Core windings use deterministic branch-current
    /// seeds for this startup solve so a zero-valued source in parallel with a
    /// Core winding does not create duplicate ideal-voltage rows. For t > 0,
    /// magnetic companion stamps take over and these operating-point stamps
    /// must NOT be applied.
    pub fn stamp_transient_operating_point_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        self.stamp_transient_linear_base_direct(matrix, rhs);
        self.capacitors
            .stamp_ic_operating_point_direct(matrix, rhs, self.num_nodes);
        let grouped_indices = self
            .xyce_core_groups
            .iter()
            .flat_map(|group| group.windings.iter().map(|winding| winding.inductor_index))
            .collect::<std::collections::HashSet<_>>();
        self.inductors.stamp_transient_operating_point_direct_where(
            matrix,
            rhs,
            self.num_nodes,
            |index| grouped_indices.contains(&index),
        );
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.stamp_tlines_dc_direct(matrix);
        self.stamp_coupled_tlines_dc_direct(matrix);
        self.stamp_xyce_core_hidden_state_identity_direct(matrix, rhs);
    }

    /// Stamp a deterministic transient-start seed when the ordinary t=0
    /// inductor-short operating point is singular.
    ///
    /// Inductors are fixed to `IC=` or zero branch current. Other linear
    /// devices, controlled sources, source values at t=0, and transmission-line
    /// DC fallbacks keep the same semantics as the ordinary transient operating
    /// point. This path is intentionally not used when the ordinary DC-short
    /// solve succeeds, because resistor-fed inductors must retain their solved
    /// operating-point current.
    pub fn stamp_transient_current_seed_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        self.stamp_transient_linear_base_direct(matrix, rhs);
        self.capacitors
            .stamp_ic_operating_point_direct(matrix, rhs, self.num_nodes);

        // Preserve one independent ideal-voltage constraint per connected
        // component.  Pinning every inductor current removes the node
        // connectivity that makes the startup system solvable; retaining a
        // spanning forest keeps the physical node voltages constrained while
        // replacing only cycle edges with deterministic current gauges.
        let mut parent = (0..=self.num_nodes).collect::<Vec<_>>();
        fn find(parent: &mut [usize], mut node: usize) -> usize {
            while parent[node] != node {
                parent[node] = parent[parent[node]];
                node = parent[node];
            }
            node
        }
        let mut union = |positive: NodeId, negative: NodeId| {
            let positive = find(&mut parent, positive);
            let negative = find(&mut parent, negative);
            if positive != negative {
                parent[positive] = negative;
            }
        };

        for (&positive, &negative) in self
            .voltage_sources
            .node_pos
            .iter()
            .zip(&self.voltage_sources.node_neg)
        {
            union(positive, negative);
        }
        for source in &self.behavioral_sources.voltage_sources {
            union(source.node_pos, source.node_neg);
        }
        for (&positive, &negative) in self.vcvs.node_pos.iter().zip(&self.vcvs.node_neg) {
            union(positive, negative);
        }
        for (&positive, &negative) in self.ccvs.node_pos.iter().zip(&self.ccvs.node_neg) {
            union(positive, negative);
        }

        let mut seed_current = vec![false; self.inductors.len()];
        let mut has_inductor_cycle = false;
        for (index, (&positive, &negative)) in self
            .inductors
            .node_pos
            .iter()
            .zip(&self.inductors.node_neg)
            .enumerate()
        {
            if self.inductors.ic[index].is_some() {
                seed_current[index] = true;
                continue;
            }
            let positive_root = find(&mut parent, positive);
            let negative_root = find(&mut parent, negative);
            if positive_root == negative_root {
                seed_current[index] = true;
                has_inductor_cycle = true;
            } else {
                parent[positive_root] = negative_root;
            }
        }
        if !has_inductor_cycle {
            seed_current.fill(true);
        }
        self.inductors.stamp_transient_current_seed_direct_where(
            matrix,
            rhs,
            self.num_nodes,
            |index| seed_current[index],
        );
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.stamp_tlines_dc_direct(matrix);
        self.stamp_coupled_tlines_dc_direct(matrix);
        self.stamp_xyce_core_hidden_state_identity_direct(matrix, rhs);
    }

    /// Stamp all devices with scaled source values (for source stepping)
    pub fn stamp_dc_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
    ) {
        self.stamp_global_shunt_direct(matrix);
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.resistor_branches
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.capacitors
            .stamp_ic_operating_point_direct(matrix, rhs, num_nodes);
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.voltage_sources
            .stamp_all_direct_scaled(matrix, rhs, scale, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all_scaled(rhs, scale);
        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.stamp_tlines_dc_direct(matrix);
        self.stamp_coupled_tlines_dc_direct(matrix);
        self.stamp_xyce_core_hidden_state_identity_direct(matrix, rhs);
    }

    /// Stamp all linear devices for DC analysis
    pub fn stamp_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;
        self.stamp_global_shunt(matrix);
        self.resistors.stamp_all(matrix);
        self.resistor_branches.stamp_all(matrix, rhs, num_nodes);
        self.capacitors
            .stamp_ic_operating_point(matrix, rhs, num_nodes);
        self.inductors.stamp_dc_short(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc(matrix, rhs);
        self.stamp_multi_winding_transformers_dc(matrix, rhs);
        self.voltage_sources.stamp_all(matrix, rhs);
        self.current_sources.stamp_all(rhs);
        self.vcvs.stamp_all(matrix, num_nodes);
        self.vccs.stamp_all(matrix);
        self.cccs.stamp_all(matrix, num_nodes);
        self.ccvs.stamp_all(matrix, num_nodes);
        self.stamp_tlines_dc(matrix);
        self.stamp_coupled_tlines_dc(matrix);
        self.stamp_xyce_core_hidden_state_identity(matrix, rhs);
    }
}
