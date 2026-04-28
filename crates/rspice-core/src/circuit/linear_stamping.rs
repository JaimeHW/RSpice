use super::*;

impl CircuitData {
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
    pub(in crate::circuit) fn stamp_tlines_dc(&self, matrix: &mut TripletMatrix) {
        for tl in &self.tlines {
            let g_series = tl.dc_series_conductance();
            Self::stamp_tline_port_triplet(matrix, tl.node1_pos, tl.node2_pos, g_series);
            Self::stamp_tline_port_triplet(matrix, tl.node1_neg, tl.node2_neg, g_series);
        }
    }

    #[inline]
    pub(in crate::circuit) fn stamp_coupled_tlines_dc(&self, matrix: &mut TripletMatrix) {
        for tline in &self.coupled_tlines {
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
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
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
    }

    /// Stamp linear devices for transient Newton iterations.
    ///
    /// This intentionally excludes transmission-line DC fallback conductances:
    /// transient delay behavior is handled by dedicated tline companion stamps.
    pub fn stamp_transient_linear_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
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

    /// Stamp the linear part of the t=0 transient operating point.
    ///
    /// Time-varying independent sources are evaluated at the requested
    /// transient time, while transmission lines use their DC fallback
    /// conductance so their far ends start from the correct operating point
    /// before delayed-wave companions take over for t > 0.
    pub fn stamp_transient_operating_point_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        self.stamp_transient_linear_direct(matrix, rhs);
        self.stamp_tlines_dc_direct(matrix);
        self.stamp_coupled_tlines_dc_direct(matrix);
    }

    /// Stamp all devices with scaled source values (for source stepping)
    pub fn stamp_dc_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
    ) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
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
    }

    /// Stamp all linear devices for DC analysis
    pub fn stamp_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;
        self.resistors.stamp_all(matrix);
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
    }
}
