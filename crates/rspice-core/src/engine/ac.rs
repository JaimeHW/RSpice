//! AC Small-Signal Analysis
//!
//! Linearizes the circuit at the DC operating point, then performs
//! frequency-domain analysis at each specified frequency. Supports
//! parallel frequency sweeps when the `parallel` feature is enabled.

#![allow(clippy::needless_range_loop)]

use super::{Engine, SimulationError};
use crate::analysis::ac::AcResult;
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};
use crate::device::{MatrixStamper, NonlinearDevice};
use crate::solver::{ComplexMatrix, StaticMatrix};
use crate::{CircuitData, Complex64, Netlist, NodeId, Value};
use std::f64::consts::PI;

const BJT_DELAY_XF1_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 2;
const BJT_DELAY_XF2_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 1;

struct AcImagStamper<'a> {
    matrix: &'a mut ComplexMatrix,
}

impl MatrixStamper for AcImagStamper<'_> {
    #[inline]
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add_imag(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
        // Small-signal AC uses only dQ/dx matrix terms.
    }
}

impl Engine {
    #[inline]
    fn ac_node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn stamp_complex_four_terminal(
        matrix: &mut ComplexMatrix,
        row_pos: usize,
        row_neg: usize,
        col_pos: usize,
        col_neg: usize,
        y: Complex64,
    ) {
        if row_pos > 0 {
            if col_pos > 0 {
                matrix.add(row_pos - 1, col_pos - 1, y);
            }
            if col_neg > 0 {
                matrix.add(row_pos - 1, col_neg - 1, -y);
            }
        }
        if row_neg > 0 {
            if col_pos > 0 {
                matrix.add(row_neg - 1, col_pos - 1, -y);
            }
            if col_neg > 0 {
                matrix.add(row_neg - 1, col_neg - 1, y);
            }
        }
    }

    #[inline]
    /// Exact LTRA small-signal branch load (ngspice ltraacld.c):
    /// `Y0(s)*V1 - I1 = e^{-lambda*l}*(Y0(s)*V2 + I2)` and symmetrically for
    /// port 2, with `Y0 = sqrt((G+sC)/(R+sL))`, `lambda*l =
    /// sqrt((G+sCtot)(R+sLtot))` in total quantities (G = 0 for the native
    /// kernel). Stamped on the branch rows the native topology reserves.
    fn stamp_ltra_branch_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        br1: NodeId,
        br2: NodeId,
        omega: Value,
    ) -> bool {
        let Some((ltot, ctot, rtot)) = tline.ltra_ac_total_rlc() else {
            return false;
        };

        let s_c = Complex64::new(0.0, omega * ctot);
        let z_series = Complex64::new(rtot, omega * ltot);
        let y0 = (s_c / z_series).sqrt();
        let lambda_l = (s_c * z_series).sqrt();
        let explambda = (-lambda_l).exp();
        let y0exp = y0 * explambda;
        if !(y0.re.is_finite() && y0.im.is_finite() && explambda.re.is_finite()) {
            return false;
        }

        let mut add = |row: NodeId, col: NodeId, value: Complex64| {
            if row > 0 && col > 0 {
                matrix.add_real(row - 1, col - 1, value.re);
                matrix.add_imag(row - 1, col - 1, value.im);
            }
        };
        let one = Complex64::new(1.0, 0.0);

        for &(br, (pos_self, neg_self), (pos_far, neg_far), far_br) in &[
            (
                br1,
                (tline.node1_pos, tline.node1_neg),
                (tline.node2_pos, tline.node2_neg),
                br2,
            ),
            (
                br2,
                (tline.node2_pos, tline.node2_neg),
                (tline.node1_pos, tline.node1_neg),
                br1,
            ),
        ] {
            add(br, pos_self, y0);
            add(br, neg_self, -y0);
            add(br, br, -one);
            add(br, pos_far, -y0exp);
            add(br, neg_far, y0exp);
            add(br, far_br, -explambda);
        }
        add(tline.node1_pos, br1, one);
        add(tline.node1_neg, br1, -one);
        add(tline.node2_pos, br2, one);
        add(tline.node2_neg, br2, -one);
        true
    }

    /// Native TXL small-signal load. ngspice registers the regular TXLload
    /// as DEVacLoad, and the AC driver runs it under MODEDC, so the oracle
    /// semantic is the DC resistive two-port: `I1 + I2 = 0` and
    /// `V1 - V2 - R*len*I1 = 0` on the reserved branch rows.
    fn stamp_txl_branch_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        br1: NodeId,
        br2: NodeId,
    ) {
        let r_series = tline.dc_series_resistance();
        let mut add = |row: NodeId, col: NodeId, value: Value| {
            if row > 0 && col > 0 {
                matrix.add_real(row - 1, col - 1, value);
            }
        };

        add(tline.node1_pos, br1, 1.0);
        add(tline.node1_neg, br1, -1.0);
        add(tline.node2_pos, br2, 1.0);
        add(tline.node2_neg, br2, -1.0);

        add(br1, br1, 1.0);
        add(br1, br2, 1.0);

        add(br2, tline.node1_pos, 1.0);
        add(br2, tline.node1_neg, -1.0);
        add(br2, tline.node2_pos, -1.0);
        add(br2, tline.node2_neg, 1.0);
        add(br2, br1, -r_series);
    }

    fn stamp_transmission_line_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        omega: Value,
    ) {
        // Distributed-line Y-parameters:
        // Y11 = Y22 = Y0 * coth(gamma)
        // Y12 = Y21 = -Y0 * csch(gamma)
        // where gamma = alpha + j*omega*td (dimensionless over one delay length).
        let y0 = 1.0 / tline.z0;
        let attenuation = tline.attenuation().clamp(1e-12, 1.0);
        let alpha = (-attenuation.ln()).max(1e-12); // avoid exact lossless singular poles
        let gamma = Complex64::new(alpha, omega * tline.td);
        let sinh_gamma = gamma.sinh();

        let (y11, y12) = if sinh_gamma.norm() < 1e-12 {
            // Series expansion around gamma=0 for numerical stability.
            let inv_gamma = Complex64::new(1.0, 0.0) / gamma;
            let coth_gamma = inv_gamma + gamma / 3.0;
            let csch_gamma = inv_gamma - gamma / 6.0;
            (
                Complex64::new(y0, 0.0) * coth_gamma,
                -Complex64::new(y0, 0.0) * csch_gamma,
            )
        } else {
            let cosh_gamma = gamma.cosh();
            (
                Complex64::new(y0, 0.0) * (cosh_gamma / sinh_gamma),
                -Complex64::new(y0, 0.0) / sinh_gamma,
            )
        };
        let y21 = y12;
        let y22 = y11;

        // Stamp differential 2-port:
        // i1 = y11*v1 + y12*v2
        // i2 = y21*v1 + y22*v2
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node1_pos,
            tline.node1_neg,
            y11,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node2_pos,
            tline.node2_neg,
            y12,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node1_pos,
            tline.node1_neg,
            y21,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node2_pos,
            tline.node2_neg,
            y22,
        );
    }

    #[inline]
    fn stamp_imag_two_terminal(
        matrix: &mut ComplexMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        susceptance: Value,
    ) {
        if node_pos > 0 {
            matrix.add_imag(node_pos - 1, node_pos - 1, susceptance);
            if node_neg > 0 {
                matrix.add_imag(node_pos - 1, node_neg - 1, -susceptance);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add_imag(node_neg - 1, node_pos - 1, -susceptance);
            }
            matrix.add_imag(node_neg - 1, node_neg - 1, susceptance);
        }
    }

    #[inline]
    fn stamp_xspice_ac_control_partial(
        matrix: &mut ComplexMatrix,
        row: usize,
        connection: &crate::xspice::PortConnection,
        partial: Complex64,
        sign: Value,
        num_nodes: usize,
    ) {
        let signed = partial * sign;
        match connection {
            crate::xspice::PortConnection::Analog(node) => {
                if *node > 0 {
                    matrix.add(row, *node - 1, signed);
                }
            }
            crate::xspice::PortConnection::Differential(pos, neg) => {
                if *pos > 0 {
                    matrix.add(row, *pos - 1, signed);
                }
                if *neg > 0 {
                    matrix.add(row, *neg - 1, -signed);
                }
            }
            crate::xspice::PortConnection::CurrentProbe { branch_ordinal, .. }
            | crate::xspice::PortConnection::BranchCurrent { branch_ordinal }
            | crate::xspice::PortConnection::Hybrid { branch_ordinal, .. } => {
                if *branch_ordinal > 0 {
                    matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                }
            }
            crate::xspice::PortConnection::NamedBranchCurrent {
                branch_ordinal: Some(branch_ordinal),
                ..
            } => {
                if *branch_ordinal > 0 {
                    matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                }
            }
            _ => {}
        }
    }

    fn stamp_xspice_ac_vector_control_partial(
        matrix: &mut ComplexMatrix,
        row: usize,
        connection: &crate::xspice::PortConnection,
        index: usize,
        partial: Complex64,
        sign: Value,
        num_nodes: usize,
    ) {
        let signed = partial * sign;
        match connection {
            crate::xspice::PortConnection::AnalogVector(nodes) => {
                if let Some(node) = nodes.get(index)
                    && *node > 0
                {
                    matrix.add(row, *node - 1, signed);
                }
            }
            crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                let Some(element) = elements.get(index) else {
                    return;
                };
                match element {
                    crate::xspice::AnalogInputConnection::Node(node) => {
                        if *node > 0 {
                            matrix.add(row, *node - 1, signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::Differential(pos, neg) => {
                        if *pos > 0 {
                            matrix.add(row, *pos - 1, signed);
                        }
                        if *neg > 0 {
                            matrix.add(row, *neg - 1, -signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::CurrentProbe {
                        branch_ordinal, ..
                    }
                    | crate::xspice::AnalogInputConnection::BranchCurrent { branch_ordinal }
                    | crate::xspice::AnalogInputConnection::Hybrid { branch_ordinal, .. } => {
                        if *branch_ordinal > 0 {
                            matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::NamedBranchCurrent {
                        branch_ordinal: Some(branch_ordinal),
                        ..
                    } => {
                        if *branch_ordinal > 0 {
                            matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn is_nonzero_finite_complex(value: Complex64) -> bool {
        value != Complex64::new(0.0, 0.0) && value.re.is_finite() && value.im.is_finite()
    }

    fn stamp_xspice_ac_current_probe(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    ) {
        if branch_ordinal == 0 {
            return;
        }
        let br = circuit.get_branch_matrix_index(branch_ordinal);
        let br_idx = br - 1;
        if pos > 0 {
            ac_matrix.add_real(br_idx, pos - 1, 1.0);
            ac_matrix.add_real(pos - 1, br_idx, 1.0);
        }
        if neg > 0 {
            ac_matrix.add_real(br_idx, neg - 1, -1.0);
            ac_matrix.add_real(neg - 1, br_idx, -1.0);
        }
    }

    fn stamp_xspice_small_signal_ac(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        frequency_hz: Value,
    ) {
        let num_nodes = circuit.num_nodes();
        for instance in &circuit.xspice_instances {
            let ports = instance.ports();
            for (pos, neg, branch_ordinal) in instance.current_probe_branches() {
                Self::stamp_xspice_ac_current_probe(circuit, ac_matrix, pos, neg, branch_ordinal);
            }
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                let Some(port) = ports.get(port_idx) else {
                    continue;
                };
                if !matches!(
                    port.direction,
                    crate::xspice::PortDirection::Out | crate::xspice::PortDirection::InOut
                ) {
                    continue;
                }

                if let crate::xspice::PortConnection::CurrentOutput { pos, neg } = connection {
                    for (control_port, partial) in
                        instance.output_input_ac_partials(&port.name, frequency_hz)
                    {
                        if !Self::is_nonzero_finite_complex(partial) {
                            continue;
                        }
                        let Some(control_connection) = instance.connection(&control_port) else {
                            continue;
                        };
                        if *pos > 0 {
                            Self::stamp_xspice_ac_control_partial(
                                ac_matrix,
                                *pos - 1,
                                control_connection,
                                partial,
                                1.0,
                                num_nodes,
                            );
                        }
                        if *neg > 0 {
                            Self::stamp_xspice_ac_control_partial(
                                ac_matrix,
                                *neg - 1,
                                control_connection,
                                partial,
                                -1.0,
                                num_nodes,
                            );
                        }
                    }
                    for (control_port, index, partial) in
                        instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                    {
                        if !Self::is_nonzero_finite_complex(partial) {
                            continue;
                        }
                        let Some(control_connection) = instance.connection(&control_port) else {
                            continue;
                        };
                        if *pos > 0 {
                            Self::stamp_xspice_ac_vector_control_partial(
                                ac_matrix,
                                *pos - 1,
                                control_connection,
                                index,
                                partial,
                                1.0,
                                num_nodes,
                            );
                        }
                        if *neg > 0 {
                            Self::stamp_xspice_ac_vector_control_partial(
                                ac_matrix,
                                *neg - 1,
                                control_connection,
                                index,
                                partial,
                                -1.0,
                                num_nodes,
                            );
                        }
                    }
                    continue;
                }

                match port.default_type {
                    crate::xspice::PortType::Voltage
                    | crate::xspice::PortType::DifferentialVoltage => {
                        let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) else {
                            continue;
                        };
                        let br = circuit.get_branch_matrix_index(branch_ordinal);
                        let br_idx = br - 1;

                        match connection {
                            crate::xspice::PortConnection::Analog(node) => {
                                if *node > 0 {
                                    ac_matrix.add_real(br_idx, *node - 1, 1.0);
                                    ac_matrix.add_real(*node - 1, br_idx, 1.0);
                                }
                            }
                            crate::xspice::PortConnection::Differential(pos, neg) => {
                                if *pos > 0 {
                                    ac_matrix.add_real(br_idx, *pos - 1, 1.0);
                                    ac_matrix.add_real(*pos - 1, br_idx, 1.0);
                                }
                                if *neg > 0 {
                                    ac_matrix.add_real(br_idx, *neg - 1, -1.0);
                                    ac_matrix.add_real(*neg - 1, br_idx, -1.0);
                                }
                            }
                            _ => continue,
                        }

                        for (control_port, partial) in
                            instance.output_input_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            if let Some(control_connection) = instance.connection(&control_port) {
                                Self::stamp_xspice_ac_control_partial(
                                    ac_matrix,
                                    br_idx,
                                    control_connection,
                                    partial,
                                    -1.0,
                                    num_nodes,
                                );
                            }
                        }
                        for (control_port, index, partial) in
                            instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            if let Some(control_connection) = instance.connection(&control_port) {
                                Self::stamp_xspice_ac_vector_control_partial(
                                    ac_matrix,
                                    br_idx,
                                    control_connection,
                                    index,
                                    partial,
                                    -1.0,
                                    num_nodes,
                                );
                            }
                        }
                    }
                    crate::xspice::PortType::Current
                    | crate::xspice::PortType::DifferentialCurrent
                    | crate::xspice::PortType::Conductance
                    | crate::xspice::PortType::DifferentialConductance => {
                        for (control_port, partial) in
                            instance.output_input_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            let Some(control_connection) = instance.connection(&control_port)
                            else {
                                continue;
                            };
                            match connection {
                                crate::xspice::PortConnection::Analog(node) => {
                                    if *node > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *node - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::Differential(pos, neg) => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::CurrentOutput { pos, neg } => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                        for (control_port, index, partial) in
                            instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            let Some(control_connection) = instance.connection(&control_port)
                            else {
                                continue;
                            };
                            match connection {
                                crate::xspice::PortConnection::Analog(node) => {
                                    if *node > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *node - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::Differential(pos, neg)
                                | crate::xspice::PortConnection::CurrentOutput { pos, neg } => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn solve_small_dense_complex_system<const N: usize>(
        matrix: &[[Complex64; N]; N],
        rhs: &[Complex64; N],
        dim: usize,
    ) -> Option<[Complex64; N]> {
        if dim == 0 {
            return Some([Complex64::new(0.0, 0.0); N]);
        }

        let mut a = *matrix;
        let mut b = *rhs;

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = a[pivot][pivot].norm();
            for row in (pivot + 1)..dim {
                let value = a[row][pivot].norm();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                a.swap(pivot, best);
                b.swap(pivot, best);
            }

            let pivot_value = a[pivot][pivot];
            for row in (pivot + 1)..dim {
                let factor = a[row][pivot] / pivot_value;
                a[row][pivot] = Complex64::new(0.0, 0.0);
                for col in (pivot + 1)..dim {
                    a[row][col] -= factor * a[pivot][col];
                }
                b[row] -= factor * b[pivot];
            }
        }

        let mut x = [Complex64::new(0.0, 0.0); N];
        for row in (0..dim).rev() {
            let mut sum = b[row];
            for col in (row + 1)..dim {
                sum -= a[row][col] * x[col];
            }
            let diag = a[row][row];
            if diag.norm() < 1e-18 {
                return None;
            }
            x[row] = sum / diag;
        }

        Some(x)
    }

    fn stamp_vbic_bjt_dynamic_ac(
        matrix: &mut ComplexMatrix,
        bjt: &crate::device::Bjt,
        op_voltages: &[Value],
        omega: Value,
        include_delay_branches: bool,
    ) {
        if !bjt.uses_vbic_dynamic_charges() {
            return;
        }

        if bjt.vbic_mna_promoted() {
            // Promoted VBIC: the internal states are matrix unknowns, so each
            // charge branch stamps jw*C directly on its own nodes alongside
            // the promoted static real part - no dense Schur reduction.
            let (branches, _, _) = bjt.vbic_mna_charge_state_at_solution(op_voltages);
            let external_nodes = [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ];
            for (branch_idx, branch) in branches.iter().enumerate() {
                if !branch.is_active() {
                    continue;
                }
                if !include_delay_branches
                    && (branch_idx == BJT_DELAY_XF1_BRANCH_INDEX
                        || branch_idx == BJT_DELAY_XF2_BRANCH_INDEX)
                {
                    // Without the xf charges the algebraic xf rows pin vxf2
                    // to Itzf and the delayed-transport correction vanishes
                    // (the pre-xf reduced behavior). ngspice-46 keeps these
                    // charges in AC (vbicacld.c XQxf stamps), so every
                    // production caller passes true; the reduced mode
                    // remains for descriptor-based callers that add charge
                    // terms themselves.
                    continue;
                }

                let mut stamp_row = |row: NodeId, sign: Value| {
                    if row == 0 {
                        return;
                    }
                    for col in 0..BJT_INTERNAL_STATE_DIM {
                        let c = branch.d_internal[col];
                        let col_node = bjt.vbic_internal_node(col);
                        if c != 0.0 && col_node > 0 {
                            matrix.add_imag(row - 1, col_node - 1, sign * omega * c);
                        }
                    }
                    for col in 0..BJT_EXTERNAL_STATE_DIM {
                        let c = branch.d_external[col];
                        let col_node = external_nodes[col];
                        if c != 0.0 && col_node > 0 {
                            matrix.add_imag(row - 1, col_node - 1, sign * omega * c);
                        }
                    }
                };

                let pos = branch
                    .pos_internal
                    .map(|idx| bjt.vbic_internal_node(idx))
                    .or_else(|| branch.pos_external.map(|idx| external_nodes[idx]));
                let neg = branch
                    .neg_internal
                    .map(|idx| bjt.vbic_internal_node(idx))
                    .or_else(|| branch.neg_external.map(|idx| external_nodes[idx]));
                if let Some(row) = pos {
                    stamp_row(row, 1.0);
                }
                if let Some(row) = neg {
                    stamp_row(row, -1.0);
                }
            }
            return;
        }

        let [vc, vb, ve, vs] = [
            Self::ac_node_voltage(op_voltages, bjt.node_collector),
            Self::ac_node_voltage(op_voltages, bjt.node_base),
            Self::ac_node_voltage(op_voltages, bjt.node_emitter),
            Self::ac_node_voltage(op_voltages, bjt.node_substrate),
        ];
        let snapshot: BjtChargeSnapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut has_dynamic_charge = false;
        for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
            if !branch.is_active() {
                continue;
            }
            if !include_delay_branches
                && (branch_idx == BJT_DELAY_XF1_BRANCH_INDEX
                    || branch_idx == BJT_DELAY_XF2_BRANCH_INDEX)
            {
                // Reduced mode without the xf companion charges (see the
                // promoted arm above); ngspice-46 includes them in AC.
                continue;
            }
            branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
            has_dynamic_charge = true;
        }
        if !has_dynamic_charge {
            return;
        }

        let s = Complex64::new(0.0, omega);
        let mut internal =
            [[Complex64::new(0.0, 0.0); BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                internal[row][col] =
                    Complex64::new(snapshot.reduction.g_ii[row][col], 0.0) + s * c_ii[row][col];
            }
        }

        let mut y_total =
            [[Complex64::new(0.0, 0.0); BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            let mut rhs = [Complex64::new(0.0, 0.0); BJT_INTERNAL_STATE_DIM];
            for row in 0..BJT_INTERNAL_STATE_DIM {
                rhs[row] =
                    -(Complex64::new(snapshot.reduction.g_ie[row][col], 0.0) + s * c_ie[row][col]);
            }

            let Some(solution) =
                Self::solve_small_dense_complex_system(&internal, &rhs, BJT_INTERNAL_STATE_DIM)
            else {
                return;
            };

            for row in 0..BJT_EXTERNAL_STATE_DIM {
                let mut value =
                    Complex64::new(snapshot.reduction.g_ee[row][col], 0.0) + s * c_ee[row][col];
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    value += (Complex64::new(snapshot.reduction.g_ei[row][idx], 0.0)
                        + s * c_ei[row][idx])
                        * solution[idx];
                }
                y_total[row][col] = value;
            }
        }

        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let delta =
                    y_total[row][col] - Complex64::new(snapshot.reduction.g_reduced[row][col], 0.0);
                if delta.norm() > 0.0 && nodes[row] > 0 && nodes[col] > 0 {
                    matrix.add(nodes[row] - 1, nodes[col] - 1, delta);
                }
            }
        }
    }

    #[inline]
    fn stamp_nonlinear_small_signal_real(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        frequency_hz: Value,
    ) -> Result<(), SimulationError> {
        struct AcRealStamper<'a> {
            matrix: &'a mut ComplexMatrix,
        }

        impl MatrixStamper for AcRealStamper<'_> {
            #[inline]
            fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
                if row > 0 && col > 0 {
                    self.matrix.add_real(row - 1, col - 1, value);
                }
            }

            #[inline]
            fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
                // AC uses only Jacobian matrix terms from nonlinear devices.
            }
        }

        let mut stamper = AcRealStamper { matrix };
        let mut rhs_dummy: [Value; 0] = [];
        circuit
            .diodes
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        for bjt in &circuit.bjts.devices {
            bjt.stamp_small_signal_ac(op_voltages, &mut stamper);
        }
        circuit
            .mosfets
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi_fd
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi_pd
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // BSIM3: the DC linearization at the operating point is the real
        // part of the small-signal admittance (b3acld.c stamps the same
        // gm/gds/gmbs/gbd/gbs/substrate-current groups as the DC load).
        circuit
            .bsim3v3
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // BSIM4: identical discipline (b4acld.c repeats the DC
        // conductance groups, GIDL/GISL included, on the real axis).
        circuit
            .bsim4v8
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // EKV26 native AC uses the DC current Jacobian for real small-signal
        // conductances and the intrinsic terminal-charge Jacobian below.
        circuit
            .ekv26s
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // EKV3 uses its ekv3_rf external DC derivatives as the low-frequency
        // real small-signal term; the VANOISE fixture applies cancellation and
        // frequency shaping in `stamp_nonlinear_capacitances`.
        circuit
            .ekv3s
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .vdmoses
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        for jfet in &circuit.jfets {
            jfet.stamp_small_signal_ac(op_voltages, frequency_hz, &mut stamper);
        }
        for sw in &circuit.vswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.iswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.generic_switches {
            sw.stamp_current_conductance(&mut stamper);
        }
        #[cfg(feature = "veriloga")]
        {
            let omega = 2.0 * std::f64::consts::PI * frequency_hz;
            for device in circuit.veriloga_devices().iter() {
                // AC linearization uses Jacobian terms at the DC operating
                // point. Verilog-A device stamping exposes the Jacobian
                // through matrix callbacks.
                let mut cloned = device.clone();
                cloned.set_analysis_type(1);
                let device_name = cloned.name.to_string();
                cloned
                    .try_stamp(
                        op_voltages,
                        |row, col, value| matrix.add_real(row, col, value),
                        |_index, _value| {},
                    )
                    .map_err(|err| {
                        SimulationError::Circuit(format!(
                            "Verilog-A device '{device_name}' AC stamping failed: {err}"
                        ))
                    })?;
                let device_name = cloned.name.to_string();
                // Reactive (ddt charge/flux) part: jw * dQ/dx
                cloned
                    .try_stamp_reactive(op_voltages, |row, col, charge_deriv| {
                        matrix.add_imag(row, col, omega * charge_deriv);
                    })
                    .map_err(|err| {
                        SimulationError::Circuit(format!(
                            "Verilog-A device '{device_name}' AC reactive stamping failed: {err}"
                        ))
                    })?;
            }
        }
        #[cfg(feature = "veriloga-builtins")]
        if circuit.has_generated_veriloga_devices() {
            let omega = 2.0 * std::f64::consts::PI * frequency_hz;
            let mut generated = circuit.generated_veriloga_devices().clone();
            let num_nodes = circuit.num_nodes();
            generated.set_timepoint(
                0.0,
                0.0,
                crate::device::veriloga_generated::GeneratedDdtCoefficients::inactive(),
            );
            generated.stamp_ac_real_all(matrix, op_voltages, num_nodes);
            generated.stamp_reactive_all(matrix, op_voltages, num_nodes, omega);
        }
        Ok(())
    }

    #[inline]
    fn stamp_nonlinear_capacitances(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        // Diode junction capacitance Cj(Vd) + diffusion capacitance.
        for diode in &circuit.diodes.devices {
            let va = Self::ac_node_voltage(op_voltages, diode.node_anode);
            let vc = Self::ac_node_voltage(op_voltages, diode.node_cathode);
            let c = diode.junction_capacitance(va - vc);
            if c.is_finite() && c > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    diode.node_anode,
                    diode.node_cathode,
                    omega * c,
                );
            }
        }

        // BJT base-emitter and base-collector depletion/diffusion capacitances.
        for bjt in &circuit.bjts.devices {
            if bjt.uses_vbic_dynamic_charges() {
                continue;
            }
            let vc = Self::ac_node_voltage(op_voltages, bjt.node_collector);
            let vb = Self::ac_node_voltage(op_voltages, bjt.node_base);
            let ve = Self::ac_node_voltage(op_voltages, bjt.node_emitter);
            let vs = Self::ac_node_voltage(op_voltages, bjt.node_substrate);
            let (legacy_vbe, legacy_vbc, legacy_vcs) =
                bjt.legacy_charge_branch_voltages(vc, vb, ve, vs);
            let charges = bjt.legacy_transient_charge_state(legacy_vbe, legacy_vbc, legacy_vcs);
            let cbe = charges.capbe;
            let cbc = charges.capbc;

            if cbe.is_finite() && cbe > 0.0 {
                Self::stamp_imag_two_terminal(matrix, bjt.node_base, bjt.node_emitter, omega * cbe);
            }
            if cbc.is_finite() && cbc > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    bjt.node_base,
                    bjt.node_collector,
                    omega * cbc,
                );
            }
            if charges.capcs.is_finite() && charges.capcs > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    bjt.node_collector,
                    bjt.node_substrate,
                    omega * charges.capcs,
                );
            }
        }

        // JFET gate-source and gate-drain depletion capacitances.
        for jfet in &circuit.jfets {
            let vd = Self::ac_node_voltage(op_voltages, jfet.drain);
            let vg = Self::ac_node_voltage(op_voltages, jfet.gate);
            let vs = Self::ac_node_voltage(op_voltages, jfet.source);
            let (cgs, cgd, cds) =
                jfet.ac_capacitances(vg - vs, vg - vd, jfet.analysis_temperature());

            if cgs.is_finite() && cgs > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.source, omega * cgs);
            }
            if cgd.is_finite() && cgd > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.drain, omega * cgd);
            }
            if cds.is_finite() && cds > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.drain, jfet.source, omega * cds);
            }
        }

        // VDMOS gate, drain-source, and body-junction capacitances at the operating point.
        for vdmos in &circuit.vdmoses.devices {
            if vdmos.xyce_level18 {
                continue;
            }
            let drain = vdmos.drain_int.unwrap_or(vdmos.drain);
            let source = vdmos.source_int.unwrap_or(vdmos.source);
            let vd = Self::ac_node_voltage(op_voltages, drain);
            let vg = Self::ac_node_voltage(op_voltages, vdmos.gate);
            let vs = Self::ac_node_voltage(op_voltages, source);
            let (cgs, cgd, cds) = vdmos.capacitances(vg - vs, vd - vs);
            let cgb = vdmos.gate_bulk_capacitance();
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(op_voltages);
            let (_, cbs) = vdmos.body_source_charge_and_capacitance_at(vbs);
            let (_, cbd) = vdmos.body_drain_charge_and_capacitance_at(vbd);
            let d1_vds = vdmos.d1_charge_branch_voltage_at(op_voltages);
            let (_, cd1) = vdmos.d1_charge_and_capacitance_at(d1_vds);

            if cgs.is_finite() && cgs > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, source, omega * cgs);
            }
            if cgd.is_finite() && cgd > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, drain, omega * cgd);
            }
            if cgb.is_finite() && cgb > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, vdmos.bulk, omega * cgb);
            }
            if cds.is_finite() && cds > 0.0 {
                Self::stamp_imag_two_terminal(matrix, drain, source, omega * cds);
            }
            if cbs.is_finite() && cbs > 0.0 {
                let (pos, neg) = vdmos.body_source_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cbs);
            }
            if cbd.is_finite() && cbd > 0.0 {
                let (pos, neg) = vdmos.body_drain_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cbd);
            }
            if cd1.is_finite() && cd1 > 0.0 {
                let (pos, neg) = vdmos.d1_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cd1);
            }
        }
    }

    #[inline]
    fn stamp_bsim3_ac_nqs_corrections(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        if omega == 0.0 || circuit.bsim3v3.is_empty() {
            return;
        }
        for dev in &circuit.bsim3v3.devices {
            if !dev.uses_ac_nqs() {
                continue;
            }
            let (charge, mode) = dev.charge_at(op_voltages);
            dev.stamp_ac_nqs_correction(&charge, mode, omega, |row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_bsim4_ac_nqs_corrections(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        if omega == 0.0 || circuit.bsim4v8.is_empty() {
            return;
        }
        for dev in &circuit.bsim4v8.devices {
            if !dev.uses_ac_nqs() {
                continue;
            }
            let (charge, mode) = dev.charge_at(op_voltages);
            dev.stamp_ac_nqs_correction(&charge, mode, omega, |row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_bsim4_trnqs_ac_charge_node_anchors(matrix: &mut ComplexMatrix, circuit: &CircuitData) {
        if circuit.bsim4v8.is_empty() {
            return;
        }
        for dev in &circuit.bsim4v8.devices {
            dev.stamp_trnqs_ac_charge_node_anchor_delta(|row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_imag_matrix_entry(matrix: &mut ComplexMatrix, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            matrix.add_imag(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_jfet_ac_imag_feedback(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        frequency_hz: Value,
    ) {
        for jfet in &circuit.jfets {
            let Some((xgm, xgds)) =
                jfet.ac_imag_feedback_terms_at_frequency(op_voltages, frequency_hz)
            else {
                continue;
            };
            let xgm = if xgm.is_finite() { xgm } else { 0.0 };
            let xgds = if xgds.is_finite() { xgds } else { 0.0 };

            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.drain, xgds);
            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.gate, xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.source, -xgds - xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.drain, -xgds);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.gate, -xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.source, xgds + xgm);
        }
    }

    /// Refill a complex AC workspace in place for one frequency. The
    /// workspace keeps its sparsity pattern and shared symbolic
    /// factorization across calls, so a sweep pays the structure cost once
    /// instead of once per point.
    fn try_fill_small_signal_ac_matrix_with_vbic_delay_mode(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        op_voltages: &[Value],
        omega: Value,
        include_vbic_dynamic_stamp: bool,
        include_vbic_delay_branches: bool,
    ) -> Result<(), SimulationError> {
        let has_nonlinear = circuit.has_nonlinear_devices();
        let size = circuit.matrix_size();
        let frequency_hz = omega / (2.0 * PI);
        ac_matrix.clear_values();

        // Stamp resistors (real conductance)
        for (r_idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let g = circuit.resistors.small_signal_conductance(r_idx);

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                ac_matrix.add_real(stamp.pp.row - 1, stamp.pp.col - 1, g);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                ac_matrix.add_real(stamp.pn.row - 1, stamp.pn.col - 1, -g);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                ac_matrix.add_real(stamp.np.row - 1, stamp.np.col - 1, -g);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                ac_matrix.add_real(stamp.nn.row - 1, stamp.nn.col - 1, g);
            }
        }

        // Stamp transmission lines. Native LTRA/TXL lines carry branch
        // unknowns whose rows only the branch-form loads can fill -- the
        // nodal Y-parameter stamp would land on absent matrix cells and
        // leave the branch equations singular (dead far port).
        for tline in &circuit.tlines {
            if let Some((br1, br2)) = tline.ltra_branch_matrix_indices()
                && Self::stamp_ltra_branch_ac(ac_matrix, tline, br1, br2, omega)
            {
                continue;
            }
            if let Some((br1, br2)) = tline.txl_branch_matrix_indices() {
                Self::stamp_txl_branch_ac(ac_matrix, tline, br1, br2);
                continue;
            }
            Self::stamp_transmission_line_ac(ac_matrix, tline, omega);
        }

        // Nonlinear device Jacobian (real part) evaluated at DC operating point.
        if has_nonlinear {
            Self::stamp_nonlinear_small_signal_real(ac_matrix, circuit, op_voltages, frequency_hz)?;
            if include_vbic_dynamic_stamp {
                for bjt in &circuit.bjts.devices {
                    Self::stamp_vbic_bjt_dynamic_ac(
                        ac_matrix,
                        bjt,
                        op_voltages,
                        omega,
                        include_vbic_delay_branches,
                    );
                }
            }
        }

        // Stamp capacitors: jωC
        for (i, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let c = circuit
                .capacitors
                .capacitances
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let jwc = omega * c;

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                ac_matrix.add_imag(stamp.pp.row - 1, stamp.pp.col - 1, jwc);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                ac_matrix.add_imag(stamp.pn.row - 1, stamp.pn.col - 1, -jwc);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                ac_matrix.add_imag(stamp.np.row - 1, stamp.np.col - 1, -jwc);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                ac_matrix.add_imag(stamp.nn.row - 1, stamp.nn.col - 1, jwc);
            }
        }

        // Nonlinear semiconductor junction capacitances at the operating point.
        if has_nonlinear {
            Self::stamp_nonlinear_capacitances(ac_matrix, circuit, op_voltages, omega);
            Self::stamp_jfet_ac_imag_feedback(ac_matrix, circuit, op_voltages, frequency_hz);
        }

        // Stamp MOSFET capacitances: jωCgs, jωCgd, jωCgb (Meyer model)
        for mos in &circuit.mosfets.devices {
            let (cgs, cgd, cgb) = mos.ac_capacitances();
            let ng = mos.node_gate;
            let nd = mos.node_drain;
            let ns = mos.node_source;
            let nb = mos.node_bulk;

            let jwcgs = omega * cgs;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgs);
            }
            if ng > 0 && ns > 0 {
                ac_matrix.add_imag(ng - 1, ns - 1, -jwcgs);
            }
            if ns > 0 && ng > 0 {
                ac_matrix.add_imag(ns - 1, ng - 1, -jwcgs);
            }
            if ns > 0 {
                ac_matrix.add_imag(ns - 1, ns - 1, jwcgs);
            }

            let jwcgd = omega * cgd;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgd);
            }
            if ng > 0 && nd > 0 {
                ac_matrix.add_imag(ng - 1, nd - 1, -jwcgd);
            }
            if nd > 0 && ng > 0 {
                ac_matrix.add_imag(nd - 1, ng - 1, -jwcgd);
            }
            if nd > 0 {
                ac_matrix.add_imag(nd - 1, nd - 1, jwcgd);
            }

            let jwcgb = omega * cgb;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgb);
            }
            if ng > 0 && nb > 0 {
                ac_matrix.add_imag(ng - 1, nb - 1, -jwcgb);
            }
            if nb > 0 && ng > 0 {
                ac_matrix.add_imag(nb - 1, ng - 1, -jwcgb);
            }
            if nb > 0 {
                ac_matrix.add_imag(nb - 1, nb - 1, jwcgb);
            }

            let (_vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(op_voltages);
            let (_, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
            if cbs.is_finite() && cbs > 0.0 {
                let (pos, neg) = mos.body_source_charge_nodes();
                Self::stamp_imag_two_terminal(ac_matrix, pos, neg, omega * cbs);
            }

            let (_, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);
            if cbd.is_finite() && cbd > 0.0 {
                let (pos, neg) = mos.body_drain_charge_nodes();
                Self::stamp_imag_two_terminal(ac_matrix, pos, neg, omega * cbd);
            }
        }

        // B3SOI/BSIM3/BSIM4 coupled capacitance matrices: the mode-assembled
        // gc** blocks evaluated at the operating point, times jw — exactly the
        // xc*** entries of each model's AC load path (nqsMod = 0 for BSIM4).
        if omega != 0.0
            && (!circuit.b3soi.is_empty()
                || !circuit.b3soi_fd.is_empty()
                || !circuit.b3soi_pd.is_empty()
                || !circuit.bsim3v3.is_empty()
                || !circuit.bsim4v8.is_empty()
                || !circuit.ekv26s.is_empty()
                || !circuit.ekv3s.is_empty())
        {
            let mut stamper = AcImagStamper { matrix: ac_matrix };
            for dev in &circuit.b3soi.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.b3soi_fd.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.b3soi_pd.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.bsim3v3.devices {
                let (charge, mode) = dev.charge_at(op_voltages);
                let gc = crate::device::Bsim3v3Device::charge_matrix(&charge, mode);
                dev.stamp_charge_matrix(&gc, omega, &mut stamper);
            }
            for dev in &circuit.bsim4v8.devices {
                let (charge, mode) = dev.charge_at(op_voltages);
                dev.stamp_ac_charge_matrix(&charge, mode, omega, &mut stamper);
            }
            for dev in &circuit.ekv26s.devices {
                dev.stamp_ac_quasi_static_charge_matrix(op_voltages, omega, &mut stamper);
            }
        }
        if omega != 0.0 && !circuit.ekv3s.is_empty() {
            let frequency_hz = omega / (2.0 * PI);
            for dev in &circuit.ekv3s.devices {
                dev.stamp_ac_transadmittance_delta(frequency_hz, |row, col, value| {
                    ac_matrix.add_real(row - 1, col - 1, value);
                });
            }
        }
        Self::stamp_bsim3_ac_nqs_corrections(ac_matrix, circuit, op_voltages, omega);
        Self::stamp_bsim4_ac_nqs_corrections(ac_matrix, circuit, op_voltages, omega);
        Self::stamp_bsim4_trnqs_ac_charge_node_anchors(ac_matrix, circuit);

        // Voltage sources for AC (MNA branch equations)
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
        }

        // Branch-form resistors for AC:
        // V(np)-V(nn)-R_ac*I = 0.
        for i in 0..circuit.resistor_branches.len() {
            let np = circuit.resistor_branches.node_pos[i];
            let nn = circuit.resistor_branches.node_neg[i];
            let br_ordinal = circuit.resistor_branches.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let resistance = circuit.resistor_branches.small_signal_resistances[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_real(br - 1, br - 1, -resistance);
        }

        // Inductors for AC:
        // V(np)-V(nn)-jωL*I = 0
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let br_ordinal = circuit.inductors.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let l = circuit.inductors.inductances[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_imag(br - 1, br - 1, -omega * l);
        }

        // Mutual coupling (K elements) for AC: the standalone inductors above
        // carry the self terms; each pair adds the -jwM cross terms.
        for binding in &circuit.coupled_inductor_pairs {
            let br1 = circuit.get_branch_matrix_index(binding.branch1_ordinal);
            let br2 = circuit.get_branch_matrix_index(binding.branch2_ordinal);
            let m = binding.device.m;
            ac_matrix.add_imag(br1 - 1, br2 - 1, -omega * m);
            ac_matrix.add_imag(br2 - 1, br1 - 1, -omega * m);
        }

        // Controlled sources: VCVS
        for i in 0..circuit.vcvs.len() {
            let np = circuit.vcvs.node_pos[i];
            let nn = circuit.vcvs.node_neg[i];
            let cp = circuit.vcvs.ctrl_pos[i];
            let cn = circuit.vcvs.ctrl_neg[i];
            let br_ordinal = circuit.vcvs.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let gain = circuit.vcvs.gains[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            if cp > 0 {
                ac_matrix.add_real(br - 1, cp - 1, -gain);
            }
            if cn > 0 {
                ac_matrix.add_real(br - 1, cn - 1, gain);
            }
        }

        // Controlled sources: VCCS
        for i in 0..circuit.vccs.len() {
            let np = circuit.vccs.node_pos[i];
            let nn = circuit.vccs.node_neg[i];
            let cp = circuit.vccs.ctrl_pos[i];
            let cn = circuit.vccs.ctrl_neg[i];
            let gm = circuit.vccs.transconductances[i];

            if np > 0 && cp > 0 {
                ac_matrix.add_real(np - 1, cp - 1, gm);
            }
            if np > 0 && cn > 0 {
                ac_matrix.add_real(np - 1, cn - 1, -gm);
            }
            if nn > 0 && cp > 0 {
                ac_matrix.add_real(nn - 1, cp - 1, -gm);
            }
            if nn > 0 && cn > 0 {
                ac_matrix.add_real(nn - 1, cn - 1, gm);
            }
        }

        // Controlled sources: CCCS
        for i in 0..circuit.cccs.len() {
            let np = circuit.cccs.node_pos[i];
            let nn = circuit.cccs.node_neg[i];
            let ctrl_branch_ordinal = circuit.cccs.ctrl_branch[i];
            let gain = circuit.cccs.gains[i];
            if ctrl_branch_ordinal == 0 {
                continue;
            }
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

            if np > 0 {
                ac_matrix.add_real(np - 1, cb - 1, gain);
            }
            if nn > 0 {
                ac_matrix.add_real(nn - 1, cb - 1, -gain);
            }
        }

        // Controlled sources: CCVS
        for i in 0..circuit.ccvs.len() {
            let np = circuit.ccvs.node_pos[i];
            let nn = circuit.ccvs.node_neg[i];
            let br_ordinal = circuit.ccvs.branch_indices[i];
            let ctrl_branch_ordinal = circuit.ccvs.ctrl_branch[i];
            let rm = circuit.ccvs.transresistances[i];
            if br_ordinal == 0 || ctrl_branch_ordinal == 0 {
                continue;
            }
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_real(br - 1, cb - 1, -rm);
        }

        // Behavioral sources: small-signal linearization at the DC
        // operating point. Partials are frequency-independent and were
        // cached by `prepare_behavioral_small_signal` after the DC solve;
        // sign conventions mirror the DC stamps exactly.
        for source in &circuit.behavioral_sources.voltage_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            let br = circuit.get_branch_matrix_index(source.branch_ordinal);

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            // Branch row: V(np) - V(nn) - Σ (df/dx)·x = 0
            for (global_idx, df) in source.linearized_partials() {
                if df != 0.0 {
                    ac_matrix.add_real(br - 1, global_idx, -df);
                }
            }
        }
        for source in &circuit.behavioral_sources.current_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            // KCL rows: I flows np -> nn, linearized I ≈ Σ (df/dx)·x.
            for (global_idx, df) in source.linearized_partials() {
                if df == 0.0 {
                    continue;
                }
                if np > 0 {
                    ac_matrix.add_real(np - 1, global_idx, df);
                }
                if nn > 0 {
                    ac_matrix.add_real(nn - 1, global_idx, -df);
                }
            }
        }

        Self::stamp_xspice_small_signal_ac(circuit, ac_matrix, frequency_hz);

        // Add small diagonal for numerical stability
        for i in 0..size {
            ac_matrix.add_real(i, i, 1e-15);
        }
        Ok(())
    }

    fn fill_small_signal_ac_matrix_with_vbic_delay_mode(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        op_voltages: &[Value],
        omega: Value,
        include_vbic_dynamic_stamp: bool,
        include_vbic_delay_branches: bool,
    ) {
        if let Err(err) = Self::try_fill_small_signal_ac_matrix_with_vbic_delay_mode(
            circuit,
            ac_matrix,
            op_voltages,
            omega,
            include_vbic_dynamic_stamp,
            include_vbic_delay_branches,
        ) {
            panic!("{err}");
        }
    }

    pub(super) fn build_small_signal_ac_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> ComplexMatrix {
        Self::try_build_small_signal_ac_matrix(circuit, matrix, op_voltages, omega)
            .unwrap_or_else(|err| panic!("{err}"))
    }

    pub(super) fn try_build_small_signal_ac_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> Result<ComplexMatrix, SimulationError> {
        // ngspice-46 includes the VBIC excess-phase network in small-signal
        // analysis: vbicacld.c stamps the full Ixf static coupling and the
        // cqxf1/cqxf2 charges (times omega) onto the xf rows. The delayed
        // transport therefore shapes AC and noise transfers above ~1/TD,
        // and the official binary fails the pre-xf 2005 AC tables by over
        // 1 dB at 10 GHz on the CEamp deck.
        let mut ac_matrix = ComplexMatrix::from_real_structure(matrix);
        Self::try_fill_small_signal_ac_matrix_with_vbic_delay_mode(
            circuit,
            &mut ac_matrix,
            op_voltages,
            omega,
            true,
            true,
        )?;
        Ok(ac_matrix)
    }

    pub(super) fn build_small_signal_pz_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> ComplexMatrix {
        // PZ descriptor construction handles VBIC hidden dynamic states
        // explicitly in `engine/advanced/mod.rs`, so keep the base AC
        // linearization free of frequency-dependent VBIC companion reduction.
        let mut ac_matrix = ComplexMatrix::from_real_structure(matrix);
        Self::fill_small_signal_ac_matrix_with_vbic_delay_mode(
            circuit,
            &mut ac_matrix,
            op_voltages,
            omega,
            false,
            true,
        );
        ac_matrix
    }

    fn build_ac_excitation_rhs(circuit: &CircuitData) -> Vec<Complex64> {
        let size = circuit.matrix_size();
        let mut rhs = vec![Complex64::new(0.0, 0.0); size];

        // Independent voltage sources with AC specification.
        for i in 0..circuit.voltage_sources.len() {
            let ac_mag = circuit.voltage_sources.ac_magnitudes[i];
            let ac_phase = circuit.voltage_sources.ac_phases[i];

            if ac_mag.abs() <= 1e-15 {
                continue;
            }

            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            rhs[br - 1] = Complex64::from_polar(ac_mag, ac_phase);
        }

        // Independent current sources with AC specification.
        for i in 0..circuit.current_sources.len() {
            let ac_mag = circuit.current_sources.ac_magnitudes[i];
            let ac_phase = circuit.current_sources.ac_phases[i];
            if ac_mag.abs() <= 1e-15 {
                continue;
            }

            let i_ac = Complex64::from_polar(ac_mag, ac_phase);
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];

            if np > 0 {
                rhs[np - 1] -= i_ac;
            }
            if nn > 0 {
                rhs[nn - 1] += i_ac;
            }
        }

        rhs
    }

    /// Run AC small-signal analysis
    ///
    /// Linearizes circuit at DC operating point, then solves at each frequency.
    /// When the `parallel` feature is enabled and there are many frequency points,
    /// the frequency sweep is parallelized for better performance.
    pub fn run_ac(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
    ) -> Result<Vec<AcResult>, SimulationError> {
        validate_ac_frequencies(frequencies)?;
        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        if circuit.num_nodes() == 0 && circuit.num_branches() == 0 {
            return Ok(frequencies
                .iter()
                .map(|&frequency| AcResult {
                    frequency,
                    node_names: Vec::new(),
                    branch_names: Vec::new(),
                    voltages: Vec::new(),
                    currents: Vec::new(),
                })
                .collect());
        }
        // Coupled multiconductor lines have no small-signal load (ngspice's
        // CPL registers none and its AC solve fails with a singular matrix);
        // refuse explicitly instead of returning silently dead ports.
        if !circuit.coupled_tlines.is_empty() {
            return Err(SimulationError::Circuit(
                "AC analysis does not support coupled multiconductor (CPL) transmission lines"
                    .to_string(),
            ));
        }
        Self::ensure_supported_ac_dynamic_charges(&circuit)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point
        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            // Align stateful nonlinear models (limited junction voltages, operating region)
            // with the final converged operating-point solution before AC linearization.
            for dev in &circuit.b3soi.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_fd.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_pd.devices {
                dev.begin_timestep_iteration();
            }
            circuit.update_nonlinear(&dc_solution);
        }
        // Cache behavioral-source partials at the operating point so the
        // (immutable, per-frequency) small-signal assembly can stamp them.
        circuit.prepare_behavioral_small_signal(&dc_solution);

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        // Closure to solve at a single frequency. Takes the circuit as a
        // parameter so the parallel path below can hand each worker its own
        // clone (device-evaluation caches are Cell-based and not Sync).
        let solve_at_freq = |circuit: &CircuitData,
                             ac_matrix: &mut ComplexMatrix,
                             freq: Value|
         -> Result<AcResult, SimulationError> {
            let omega = 2.0 * PI * freq;
            Self::try_fill_small_signal_ac_matrix_with_vbic_delay_mode(
                circuit,
                ac_matrix,
                &dc_solution,
                omega,
                true,
                true,
            )?;
            let rhs = Self::build_ac_excitation_rhs(circuit);
            let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;

            Ok(AcResult {
                frequency: freq,
                node_names: node_names.clone(),
                branch_names: branch_names.clone(),
                voltages: solution[..num_nodes].to_vec(),
                currents: if size > num_nodes {
                    solution[num_nodes..].to_vec()
                } else {
                    vec![]
                },
            })
        };

        // Parallel sweep: every frequency point shares the same operating
        // point and matrix structure, so points are fully independent.
        // CircuitData is not Sync (Cell-based device-eval caches), so each
        // worker owns an independent clone paired with one contiguous chunk
        // of the sweep — no shared state, no locks, and chunk order
        // preserves output ordering. The caches are pure memoization, so
        // per-point results are identical to the sequential path.
        #[cfg(feature = "parallel")]
        if frequencies.len() >= 10 {
            use rayon::prelude::*;

            let workers = rayon::current_num_threads().clamp(1, frequencies.len());
            let chunk_len = frequencies.len().div_ceil(workers);
            let work: Vec<(CircuitData, &[Value])> = frequencies
                .chunks(chunk_len)
                .map(|chunk| (circuit.clone(), chunk))
                .collect();
            let chunk_results: Result<Vec<Vec<AcResult>>, SimulationError> = work
                .into_par_iter()
                .map(|(worker_circuit, chunk)| {
                    let mut workspace = ComplexMatrix::from_real_structure(&matrix);
                    chunk
                        .iter()
                        .map(|&freq| solve_at_freq(&worker_circuit, &mut workspace, freq))
                        .collect()
                })
                .collect();
            return chunk_results.map(|chunks| chunks.into_iter().flatten().collect());
        }

        let mut workspace = ComplexMatrix::from_real_structure(&matrix);
        frequencies
            .iter()
            .map(|&freq| solve_at_freq(&circuit, &mut workspace, freq))
            .collect()
    }
}

fn validate_ac_frequencies(frequencies: &[Value]) -> Result<(), SimulationError> {
    if frequencies.is_empty() {
        return Err(SimulationError::Circuit(
            "AC analysis requires at least one frequency point".to_string(),
        ));
    }

    if let Some((index, frequency)) = frequencies
        .iter()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || **frequency < 0.0)
    {
        return Err(SimulationError::Circuit(format!(
            "AC frequency at index {index} must be finite and non-negative, got {frequency}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ac_deck() -> Netlist {
        Netlist::parse(
            "AC deck\n\
             V1 in 0 DC 0 AC 1\n\
             R1 in out 1k\n\
             C1 out 0 1u\n\
             .end\n",
        )
        .expect("deck parses")
    }

    #[test]
    fn ac_rejects_empty_or_invalid_frequency_grid() {
        let netlist = ac_deck();
        let engine = Engine::default();

        let err = engine
            .run_ac(&netlist, &[])
            .expect_err("empty AC sweep must not report success");
        assert!(
            err.to_string().contains("frequency"),
            "unexpected error: {err}"
        );

        for freq in [f64::NAN, f64::INFINITY, -1.0] {
            let err = engine
                .run_ac(&netlist, &[freq])
                .expect_err("invalid AC frequency must not enter the solver");
            assert!(
                err.to_string().contains("finite") || err.to_string().contains("non-negative"),
                "unexpected error for freq={freq:?}: {err}"
            );
        }
    }
}
