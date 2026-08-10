//! Reactive companion stamping helpers for transient analysis.

use super::*;

/// Pre-resolved matrix slots for one two-terminal companion branch.
///
/// `stamp_two_terminal_companion` pays four position-map hash lookups per
/// branch per Newton iteration; on device-heavy decks those lookups dominate
/// the assembly. The slots are resolved once per transient run (the matrix
/// pattern is frozen) and stamped through direct CSC indices afterwards.
#[derive(Clone, Copy, Default)]
pub(super) struct TwoTerminalStampSlots {
    pub(super) pos: usize,
    pub(super) neg: usize,
    pub(super) pp: Option<crate::solver::CscIndex>,
    pub(super) pn: Option<crate::solver::CscIndex>,
    pub(super) np: Option<crate::solver::CscIndex>,
    pub(super) nn: Option<crate::solver::CscIndex>,
}

impl TwoTerminalStampSlots {
    pub(super) fn link(
        matrix: &crate::solver::StaticMatrix,
        node_pos: usize,
        node_neg: usize,
    ) -> Self {
        let mut slots = Self {
            pos: node_pos,
            neg: node_neg,
            ..Self::default()
        };
        if node_pos > 0 {
            slots.pp = matrix.get_index(node_pos - 1, node_pos - 1);
            if node_neg > 0 {
                slots.pn = matrix.get_index(node_pos - 1, node_neg - 1);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                slots.np = matrix.get_index(node_neg - 1, node_pos - 1);
            }
            slots.nn = matrix.get_index(node_neg - 1, node_neg - 1);
        }
        slots
    }
}

/// Pattern-local twin of [`TwoTerminalStampSlots`] for batched sparse writes.
///
/// One enclosing transient context owns and validates the frozen pattern
/// token. Keeping only numeric offsets here halves the per-branch topology
/// footprint while direct and small-device paths retain fully checked
/// [`CscIndex`] values.
#[derive(Clone, Copy)]
pub(super) struct CompactTwoTerminalStampSlots {
    pub(super) pos: usize,
    pub(super) neg: usize,
    pp: usize,
    pn: usize,
    np: usize,
    nn: usize,
}

impl CompactTwoTerminalStampSlots {
    const ABSENT: usize = usize::MAX;

    #[inline]
    fn linked_offset(index: Option<crate::solver::CscIndex>) -> usize {
        index.map_or(Self::ABSENT, crate::solver::CscIndex::offset)
    }

    #[inline]
    fn offset(slot: usize) -> Option<usize> {
        (slot != Self::ABSENT).then_some(slot)
    }

    pub(super) fn link(
        matrix: &crate::solver::StaticMatrix,
        node_pos: usize,
        node_neg: usize,
    ) -> Self {
        let mut slots = Self {
            pos: node_pos,
            neg: node_neg,
            pp: Self::ABSENT,
            pn: Self::ABSENT,
            np: Self::ABSENT,
            nn: Self::ABSENT,
        };
        if node_pos > 0 {
            slots.pp = Self::linked_offset(matrix.get_index(node_pos - 1, node_pos - 1));
            if node_neg > 0 {
                slots.pn = Self::linked_offset(matrix.get_index(node_pos - 1, node_neg - 1));
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                slots.np = Self::linked_offset(matrix.get_index(node_neg - 1, node_pos - 1));
            }
            slots.nn = Self::linked_offset(matrix.get_index(node_neg - 1, node_neg - 1));
        }
        slots
    }
}

impl Engine {
    /// Batched-stamp twin of [`Engine::stamp_two_terminal_companion_direct`].
    ///
    /// `values` has already been matched to the frozen pattern that produced
    /// `slots`, so the enclosing batch pays that identity check only once.
    #[inline]
    pub(super) fn stamp_two_terminal_companion_values(
        values: &mut [Value],
        rhs: &mut [Value],
        slots: &TwoTerminalStampSlots,
        geq: Value,
        i_eq: Value,
    ) {
        if let Some(idx) = slots.pp {
            values[idx.offset()] += geq;
        }
        if let Some(idx) = slots.pn {
            values[idx.offset()] += -geq;
        }
        if let Some(idx) = slots.np {
            values[idx.offset()] += -geq;
        }
        if let Some(idx) = slots.nn {
            values[idx.offset()] += geq;
        }
        if slots.pos > 0 {
            rhs[slots.pos - 1] += i_eq;
        }
        if slots.neg > 0 {
            rhs[slots.neg - 1] -= i_eq;
        }
    }

    /// Pattern-local batched stamp using compact numeric offsets. The caller
    /// must first validate the enclosing cache's frozen pattern token against
    /// the matrix that owns `values`.
    #[inline]
    pub(super) fn stamp_compact_two_terminal_companion_values(
        values: &mut [Value],
        rhs: &mut [Value],
        slots: &CompactTwoTerminalStampSlots,
        geq: Value,
        i_eq: Value,
    ) {
        if let Some(offset) = CompactTwoTerminalStampSlots::offset(slots.pp) {
            values[offset] += geq;
        }
        if let Some(offset) = CompactTwoTerminalStampSlots::offset(slots.pn) {
            values[offset] += -geq;
        }
        if let Some(offset) = CompactTwoTerminalStampSlots::offset(slots.np) {
            values[offset] += -geq;
        }
        if let Some(offset) = CompactTwoTerminalStampSlots::offset(slots.nn) {
            values[offset] += geq;
        }
        if slots.pos > 0 {
            rhs[slots.pos - 1] += i_eq;
        }
        if slots.neg > 0 {
            rhs[slots.neg - 1] -= i_eq;
        }
    }

    /// Index-resolved twin of [`Engine::stamp_two_terminal_companion`].
    #[inline]
    pub(super) fn stamp_two_terminal_companion_direct(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        slots: &TwoTerminalStampSlots,
        geq: Value,
        i_eq: Value,
    ) {
        if let Some(idx) = slots.pp {
            matrix.stamp_direct(idx, geq);
        }
        if let Some(idx) = slots.pn {
            matrix.stamp_direct(idx, -geq);
        }
        if let Some(idx) = slots.np {
            matrix.stamp_direct(idx, -geq);
        }
        if let Some(idx) = slots.nn {
            matrix.stamp_direct(idx, geq);
        }
        if slots.pos > 0 {
            rhs[slots.pos - 1] += i_eq;
        }
        if slots.neg > 0 {
            rhs[slots.neg - 1] -= i_eq;
        }
    }
    #[inline]
    pub(super) fn tline_transient_port_impedance(tl: &crate::device::TransmissionLine) -> Value {
        // Keep the local port relation anchored to the characteristic
        // impedance; lossy model-card behavior is captured through delayed-wave
        // attenuation and history smoothing rather than by distorting the
        // immediate Z0 boundary condition.
        tl.impedance().max(1e-12)
    }

    #[inline]
    pub(super) fn stamp_tline_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        g: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -g);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -g);
            }
            matrix.add(node_neg - 1, node_neg - 1, g);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    pub(super) fn stamp_tline_cross_conductance(
        matrix: &mut crate::solver::StaticMatrix,
        node_row_pos: usize,
        node_row_neg: usize,
        node_col_pos: usize,
        node_col_neg: usize,
        g_cross: Value,
    ) {
        if g_cross == 0.0 {
            return;
        }

        if node_row_pos > 0 {
            if node_col_pos > 0 {
                matrix.add(node_row_pos - 1, node_col_pos - 1, g_cross);
            }
            if node_col_neg > 0 {
                matrix.add(node_row_pos - 1, node_col_neg - 1, -g_cross);
            }
        }
        if node_row_neg > 0 {
            if node_col_pos > 0 {
                matrix.add(node_row_neg - 1, node_col_pos - 1, -g_cross);
            }
            if node_col_neg > 0 {
                matrix.add(node_row_neg - 1, node_col_neg - 1, g_cross);
            }
        }
    }

    #[inline]
    pub(super) fn stamp_tline_two_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::TransmissionLine,
        response: crate::device::TlineTransientResponse,
    ) {
        Self::stamp_tline_port(
            matrix,
            rhs,
            tl.node1_pos,
            tl.node1_neg,
            response.self_conductance(),
            response.i_eq_port1(),
        );
        Self::stamp_tline_port(
            matrix,
            rhs,
            tl.node2_pos,
            tl.node2_neg,
            response.self_conductance(),
            response.i_eq_port2(),
        );
        Self::stamp_tline_cross_conductance(
            matrix,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            response.mutual_conductance(),
        );
        Self::stamp_tline_cross_conductance(
            matrix,
            tl.node2_pos,
            tl.node2_neg,
            tl.node1_pos,
            tl.node1_neg,
            response.mutual_conductance(),
        );
    }

    #[inline]
    pub(super) fn stamp_txl_branch_runtime(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::TransmissionLine,
        stamp: crate::device::TxlTransientStamp,
    ) {
        let Some((br1, br2)) = tl.txl_branch_matrix_indices() else {
            return;
        };

        Self::stamp_txl_branch_kcl(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_txl_branch_kcl(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);

        matrix.add(br1 - 1, br1 - 1, -1.0);
        matrix.add(br2 - 1, br2 - 1, -1.0);

        Self::stamp_txl_voltage_row(matrix, br1, tl.node1_pos, tl.node1_neg, stamp.local_coeff);
        Self::stamp_txl_voltage_row(matrix, br2, tl.node2_pos, tl.node2_neg, stamp.local_coeff);

        if stamp.delayed_voltage_coeff != 0.0 {
            Self::stamp_txl_voltage_row(
                matrix,
                br1,
                tl.node2_pos,
                tl.node2_neg,
                -stamp.delayed_voltage_coeff,
            );
            Self::stamp_txl_voltage_row(
                matrix,
                br2,
                tl.node1_pos,
                tl.node1_neg,
                -stamp.delayed_voltage_coeff,
            );
        }
        if stamp.delayed_current_coeff != 0.0 {
            matrix.add(br1 - 1, br2 - 1, -stamp.delayed_current_coeff);
            matrix.add(br2 - 1, br1 - 1, -stamp.delayed_current_coeff);
        }

        rhs[br1 - 1] = stamp.rhs1;
        rhs[br2 - 1] = stamp.rhs2;
    }

    #[inline]
    pub(super) fn stamp_ltra_branch_runtime(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::TransmissionLine,
        response: crate::device::TlineTransientResponse,
    ) {
        let Some((br1, br2)) = tl.ltra_branch_matrix_indices() else {
            return;
        };

        Self::stamp_txl_branch_kcl(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_txl_branch_kcl(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);

        matrix.add(br1 - 1, br1 - 1, -1.0);
        matrix.add(br2 - 1, br2 - 1, -1.0);

        Self::stamp_txl_voltage_row(
            matrix,
            br1,
            tl.node1_pos,
            tl.node1_neg,
            response.self_conductance(),
        );
        Self::stamp_txl_voltage_row(
            matrix,
            br2,
            tl.node2_pos,
            tl.node2_neg,
            response.self_conductance(),
        );

        if response.mutual_conductance() != 0.0 {
            Self::stamp_txl_voltage_row(
                matrix,
                br1,
                tl.node2_pos,
                tl.node2_neg,
                response.mutual_conductance(),
            );
            Self::stamp_txl_voltage_row(
                matrix,
                br2,
                tl.node1_pos,
                tl.node1_neg,
                response.mutual_conductance(),
            );
        }
        if response.mutual_current_coefficient() != 0.0 {
            matrix.add(br1 - 1, br2 - 1, response.mutual_current_coefficient());
            matrix.add(br2 - 1, br1 - 1, response.mutual_current_coefficient());
        }

        rhs[br1 - 1] = response.i_eq_port1();
        rhs[br2 - 1] = response.i_eq_port2();
    }

    #[inline]
    pub(super) fn stamp_zero_length_branch_runtime(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::TransmissionLine,
    ) {
        let Some((br1, br2)) = tl.zero_length_branch_matrix_indices() else {
            return;
        };

        // Preserve the exact ideal-through MNA equations for every transient
        // candidate: I1 + I2 = 0 and V1 - V2 = 0.  No delayed history or
        // companion conductance is introduced for LEN=0 RC/RG cards.
        Self::stamp_txl_branch_kcl(matrix, tl.node1_pos, tl.node1_neg, br1, 1.0);
        Self::stamp_txl_branch_kcl(matrix, tl.node2_pos, tl.node2_neg, br2, 1.0);
        matrix.add(br1 - 1, br1 - 1, 1.0);
        matrix.add(br1 - 1, br2 - 1, 1.0);
        Self::stamp_txl_voltage_row(matrix, br2, tl.node1_pos, tl.node1_neg, 1.0);
        Self::stamp_txl_voltage_row(matrix, br2, tl.node2_pos, tl.node2_neg, -1.0);
        rhs[br1 - 1] = 0.0;
        rhs[br2 - 1] = 0.0;
    }

    #[inline]
    pub(crate) fn stamp_tline_branch_topology(
        triplets: &mut Vec<(usize, usize, Value)>,
        tl: &crate::device::TransmissionLine,
        br1: usize,
        br2: usize,
    ) {
        for &(node, br) in &[
            (tl.node1_pos, br1),
            (tl.node1_neg, br1),
            (tl.node2_pos, br2),
            (tl.node2_neg, br2),
        ] {
            if node > 0 {
                triplets.push((node - 1, br - 1, 0.0));
            }
        }
        for &(br, node) in &[
            (br1, tl.node1_pos),
            (br1, tl.node1_neg),
            (br1, tl.node2_pos),
            (br1, tl.node2_neg),
            (br2, tl.node1_pos),
            (br2, tl.node1_neg),
            (br2, tl.node2_pos),
            (br2, tl.node2_neg),
        ] {
            if node > 0 {
                triplets.push((br - 1, node - 1, 0.0));
            }
        }
        triplets.push((br1 - 1, br1 - 1, 0.0));
        triplets.push((br1 - 1, br2 - 1, 0.0));
        triplets.push((br2 - 1, br1 - 1, 0.0));
        triplets.push((br2 - 1, br2 - 1, 0.0));
    }

    #[inline]
    fn stamp_txl_branch_kcl(
        matrix: &mut crate::solver::StaticMatrix,
        node_pos: usize,
        node_neg: usize,
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
    fn stamp_txl_voltage_row(
        matrix: &mut crate::solver::StaticMatrix,
        row: usize,
        node_pos: usize,
        node_neg: usize,
        coeff: Value,
    ) {
        if node_pos > 0 {
            matrix.add(row - 1, node_pos - 1, coeff);
        }
        if node_neg > 0 {
            matrix.add(row - 1, node_neg - 1, -coeff);
        }
    }

    #[inline]
    pub(super) fn stamp_shared_reference_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        nodes: &[usize],
        reference: usize,
        admittance: &[Vec<Value>],
        eq_currents: &[Value],
    ) {
        let row_sums: Vec<Value> = admittance
            .iter()
            .map(|row| row.iter().copied().sum())
            .collect();

        for (row_idx, &node_row) in nodes.iter().enumerate() {
            if node_row == 0 {
                continue;
            }
            for (col_idx, &node_col) in nodes.iter().enumerate() {
                if node_col > 0 {
                    matrix.add(node_row - 1, node_col - 1, admittance[row_idx][col_idx]);
                }
            }
            if reference > 0 {
                matrix.add(node_row - 1, reference - 1, -row_sums[row_idx]);
            }
            rhs[node_row - 1] += eq_currents.get(row_idx).copied().unwrap_or(0.0);
        }

        if reference > 0 {
            let mut ref_injection = 0.0;
            for (col_idx, &node_col) in nodes.iter().enumerate() {
                if node_col > 0 {
                    matrix.add(reference - 1, node_col - 1, -row_sums[col_idx]);
                }
                ref_injection -= eq_currents.get(col_idx).copied().unwrap_or(0.0);
            }
            let ref_sum: Value = row_sums.iter().copied().sum();
            matrix.add(reference - 1, reference - 1, ref_sum);
            rhs[reference - 1] += ref_injection;
        }
    }

    #[inline]
    pub(super) fn stamp_two_terminal_companion(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        geq: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, geq);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -geq);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -geq);
            }
            matrix.add(node_neg - 1, node_neg - 1, geq);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    pub(super) fn stamp_external_reduced_system(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        nodes: &[usize; BJT_EXTERNAL_STATE_DIM],
        y: &[[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        i_eq: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col > 0 {
                    matrix.add(node_row - 1, node_col - 1, y[row][col]);
                }
            }
            rhs[node_row - 1] += i_eq[row];
        }
    }

    #[inline]
    pub(super) fn jfet_branch_voltages(
        jfet: &crate::device::Jfet,
        voltages: &[Value],
    ) -> (Value, Value) {
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1 | crate::device::JfetChannelModel::XyceSydney
        ) && let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages()
        {
            return (vgs, vgd);
        }
        let vg = Self::node_voltage(voltages, jfet.gate);
        let vd = Self::node_voltage(voltages, jfet.drain);
        let vs = Self::node_voltage(voltages, jfet.source);
        (vg - vs, vg - vd)
    }

    #[inline]
    pub(super) fn jfet_charge_branch_voltages(
        jfet: &crate::device::Jfet,
        voltages: &[Value],
    ) -> (Value, Value) {
        // ngspice HFET1 keeps two voltage tracks: limited vgs/vgd for nonlinear
        // channel/capacitance evaluation, and raw vgspp/vgdpp for charge history.
        // The transient qgs/qgd update must follow the raw branch voltage so a
        // source edge injects the same charge current even when DEVfetlim limits
        // the nonlinear control voltage during Newton iterations.
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) && jfet.params.hfet_level >= 5
        {
            let vg = Self::node_voltage(voltages, jfet.gate);
            let vd = Self::node_voltage(voltages, jfet.drain);
            let vs = Self::node_voltage(voltages, jfet.source);
            return (vg - vs, vg - vd);
        }

        // MESA/HFET2-style devices keep using the limited internal branch state,
        // matching the existing level-2..4 path and its convergence behavior.
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1 | crate::device::JfetChannelModel::XyceSydney
        ) && let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages()
        {
            return (vgs, vgd);
        }

        let vg = Self::node_voltage(voltages, jfet.gate);
        let vd = Self::node_voltage(voltages, jfet.drain);
        let vs = Self::node_voltage(voltages, jfet.source);
        (vg - vs, vg - vd)
    }

    #[inline]
    pub(super) fn effective_trapezoidal_order(method: IntegrationMethod, trap_order: u8) -> u8 {
        match method {
            IntegrationMethod::BackwardEuler => 1,
            IntegrationMethod::Gear2 => trap_order.clamp(1, 2),
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => trap_order.clamp(1, 2),
        }
    }

    #[inline]
    pub(super) fn breakpoint_landing_forces_order_one(dialect: SpiceDialect) -> bool {
        dialect != SpiceDialect::Xyce
    }

    #[inline]
    pub(super) fn step_trapezoidal_order(
        method: IntegrationMethod,
        trap_order: u8,
        force_order_one: bool,
    ) -> u8 {
        if force_order_one
            && matches!(
                method,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
            )
        {
            1
        } else {
            Self::effective_trapezoidal_order(method, trap_order)
        }
    }

    #[inline]
    pub(super) fn effective_companion_method(
        method: IntegrationMethod,
        trap_order: u8,
    ) -> IntegrationMethod {
        match method {
            IntegrationMethod::Trapezoidal
            | IntegrationMethod::TrapGear
            | IntegrationMethod::Gear2
                if trap_order <= 1 =>
            {
                IntegrationMethod::BackwardEuler
            }
            _ => method,
        }
    }

    #[inline]
    pub(super) fn jfet_companion_geq(
        coeff: &CompanionCoefficients,
        capacitance: Value,
        dt: Value,
    ) -> Value {
        if !capacitance.is_finite() || capacitance <= 0.0 || !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        coeff.capacitor_geq(capacitance, dt)
    }

    #[inline]
    pub(super) fn jfet_companion_ccap(
        coeff: &CompanionCoefficients,
        dt: Value,
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        coeff.capacitor_geq(1.0, dt) * q_curr
            - coeff.capacitor_ieq(1.0, dt, q_prev, q_prev_prev, cq_prev)
    }

    #[inline]
    pub(super) fn jfet_companion_terms(
        coeff: &CompanionCoefficients,
        dt: Value,
        capacitance: Value,
        v_curr: Value,
        v_prev: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> (Value, Value, Value, Value) {
        let geq = Self::jfet_companion_geq(coeff, capacitance, dt);
        if geq == 0.0 {
            return (0.0, 0.0, q_prev, 0.0);
        }
        // Match ngspice nonlinear charge-branch transient update:
        // q(n+1) = q(n) + C(n+1) * (v(n+1) - v(n))
        let q_curr = q_prev + capacitance * (v_curr - v_prev);
        let cq_curr = Self::jfet_companion_ccap(coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        // Match ngspice load linearization contract for capacitive branches:
        //   i(v) ≈ ccap + geq * (v - v_hist) = geq * v - (geq * v_hist - ccap).
        // With our companion stamp convention (i = geq * v - i_eq), this gives:
        //   i_eq = geq * v_hist - ccap.
        // NOTE: This intentionally uses branch voltage history, not charge, because
        // q is not generally equal to C * v for voltage-dependent capacitances.
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    /// Gate-charge companion update with the unit-capacitance conductance
    /// already evaluated for this timestep. A MOS instance evaluates three
    /// independent Meyer branches with the same coefficients and `dt`; sharing
    /// this exact scalar avoids repeating identical divisions without changing
    /// any per-branch arithmetic.
    #[inline]
    pub(super) fn jfet_companion_terms_with_unit_geq(
        coeff: &CompanionCoefficients,
        dt: Value,
        unit_geq: Value,
        capacitance: Value,
        v_curr: Value,
        v_prev: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> (Value, Value, Value, Value) {
        let geq = Self::jfet_companion_geq(coeff, capacitance, dt);
        if geq == 0.0 {
            return (0.0, 0.0, q_prev, 0.0);
        }
        let q_curr = q_prev + capacitance * (v_curr - v_prev);
        let cq_curr =
            unit_geq * q_curr - coeff.capacitor_ieq(1.0, dt, q_prev, q_prev_prev, cq_prev);
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    #[inline]
    pub(super) fn nonlinear_charge_companion_terms(
        coeff: &CompanionCoefficients,
        dt: Value,
        capacitance: Value,
        v_curr: Value,
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> (Value, Value, Value, Value) {
        let geq = Self::jfet_companion_geq(coeff, capacitance, dt);
        if geq == 0.0 {
            return (0.0, 0.0, q_curr, 0.0);
        }
        let cq_curr = Self::jfet_companion_ccap(coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    #[inline]
    pub(super) fn linear_charge_history_ieq(
        coeff: &CompanionCoefficients,
        dt: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        coeff.capacitor_ieq(1.0, dt, q_prev, q_prev_prev, cq_prev)
    }

    #[inline]
    pub(super) fn predict_transient_history_value(
        previous: Value,
        previous_previous: Option<Value>,
        dt: Value,
        previous_dt: Value,
    ) -> Value {
        let Some(previous_previous) = previous_previous else {
            return previous;
        };
        if !(dt.is_finite() && dt > 0.0 && previous_dt.is_finite() && previous_dt > 0.0) {
            return previous;
        }

        let xfact = dt / previous_dt;
        let predicted = (1.0 + xfact) * previous - xfact * previous_previous;
        if predicted.is_finite() {
            predicted
        } else {
            previous
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_two_terminal_stamp_slots_contain_only_offsets() {
        let bytes = std::mem::size_of::<CompactTwoTerminalStampSlots>();
        assert!(
            bytes <= 48,
            "compact two-terminal stamp plan regressed to {bytes} bytes; keep pattern identity in the enclosing cache"
        );
    }

    #[test]
    fn xyce_jfet1_transient_charge_uses_limited_branch_state() {
        let mut jfet = crate::device::Jfet::njf("j1", 1, 2, 0).enable_xyce_jfet1_model();
        crate::device::NonlinearDevice::update(&mut jfet, &[0.0, 0.0]);
        let raw_solution = [0.0, 10.0];
        crate::device::NonlinearDevice::update(&mut jfet, &raw_solution);
        let (limited_vgs, limited_vgd, _) = jfet
            .internal_branch_state_voltages()
            .expect("second nonlinear update establishes limited JFET state");
        assert_ne!(limited_vgs.to_bits(), 10.0_f64.to_bits());
        assert_ne!(limited_vgd.to_bits(), 10.0_f64.to_bits());

        let evaluation = Engine::jfet_branch_voltages(&jfet, &raw_solution);
        let charge = Engine::jfet_charge_branch_voltages(&jfet, &raw_solution);
        assert_eq!(
            [evaluation.0.to_bits(), evaluation.1.to_bits()],
            [limited_vgs.to_bits(), limited_vgd.to_bits()]
        );
        assert_eq!(
            [charge.0.to_bits(), charge.1.to_bits()],
            [limited_vgs.to_bits(), limited_vgd.to_bits()]
        );
    }

    #[test]
    fn shared_unit_geq_companion_matches_canonical_terms_exactly() {
        let coefficient_sets = [
            CompanionCoefficients::backward_euler(),
            CompanionCoefficients::trapezoidal(),
            CompanionCoefficients::gear2(),
        ];
        for coeff in coefficient_sets {
            for dt in [1.0e-15, 2.7e-12, 0.0, Value::INFINITY] {
                let unit_geq = Engine::jfet_companion_geq(&coeff, 1.0, dt);
                for capacitance in [0.0, 1.3e-15, 4.2e-9, -1.0, Value::NAN] {
                    let canonical = Engine::jfet_companion_terms(
                        &coeff,
                        dt,
                        capacitance,
                        1.7,
                        -0.4,
                        3.1e-12,
                        -2.7e-12,
                        8.3e-6,
                    );
                    let shared = Engine::jfet_companion_terms_with_unit_geq(
                        &coeff,
                        dt,
                        unit_geq,
                        capacitance,
                        1.7,
                        -0.4,
                        3.1e-12,
                        -2.7e-12,
                        8.3e-6,
                    );
                    assert_eq!(
                        [
                            shared.0.to_bits(),
                            shared.1.to_bits(),
                            shared.2.to_bits(),
                            shared.3.to_bits(),
                        ],
                        [
                            canonical.0.to_bits(),
                            canonical.1.to_bits(),
                            canonical.2.to_bits(),
                            canonical.3.to_bits(),
                        ],
                        "shared factor differs for dt={dt}, capacitance={capacitance}, coeff={coeff:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn batched_companion_stamp_matches_validated_stamp_exactly() {
        let linked_matrix = crate::solver::StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .expect("full two-node matrix");
        let slots = TwoTerminalStampSlots::link(&linked_matrix, 1, 2);
        let compact_slots = CompactTwoTerminalStampSlots::link(&linked_matrix, 1, 2);

        let mut validated_matrix = linked_matrix.clone_structure();
        let mut validated_rhs = vec![0.0; 2];
        Engine::stamp_two_terminal_companion_direct(
            &mut validated_matrix,
            &mut validated_rhs,
            &slots,
            1.25,
            -0.75,
        );

        let mut batched_matrix = linked_matrix.clone_structure();
        let mut batched_rhs = vec![0.0; 2];
        let values = batched_matrix
            .values_mut_for_pattern(linked_matrix.pattern_token())
            .expect("clone retains the linked pattern");
        Engine::stamp_two_terminal_companion_values(values, &mut batched_rhs, &slots, 1.25, -0.75);

        assert_eq!(batched_rhs, validated_rhs);
        assert_eq!(batched_matrix.values_mut(), validated_matrix.values_mut());

        let mut compact_matrix = linked_matrix.clone_structure();
        let mut compact_rhs = vec![0.0; 2];
        let values = compact_matrix
            .values_mut_for_pattern(linked_matrix.pattern_token())
            .expect("clone retains the linked pattern");
        Engine::stamp_compact_two_terminal_companion_values(
            values,
            &mut compact_rhs,
            &compact_slots,
            1.25,
            -0.75,
        );
        assert_eq!(compact_rhs, validated_rhs);
        assert_eq!(compact_matrix.values_mut(), validated_matrix.values_mut());
    }
}
