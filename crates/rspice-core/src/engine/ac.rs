//! AC Small-Signal Analysis
//!
//! Linearizes the circuit at the DC operating point, then performs
//! frequency-domain analysis at each specified frequency. Supports
//! parallel frequency sweeps when the `parallel` feature is enabled.

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
                // ngspice linear AC decks treat VBIC excess-phase TD as a
                // transient-only dynamic delay state; keep AC parity by excluding
                // xf1/xf2 companion-charge contributions from small-signal stamping.
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
    ) {
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
        for jfet in &circuit.jfets {
            jfet.stamp_small_signal_ac(op_voltages, frequency_hz, &mut stamper);
        }
        for sw in &circuit.vswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.iswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            // AC linearization uses Jacobian terms at the DC operating point.
            // Verilog-A device stamping exposes Jacobian through matrix callbacks.
            let mut cloned = device.clone();
            cloned.stamp(
                op_voltages,
                |row, col, value| matrix.add_real(row, col, value),
                |_index, _value| {},
            );
        }
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
            let (cgs, cgd, cds) = jfet.ac_capacitances(vg - vs, vg - vd, jfet.params.tnom);

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
    }

    fn build_small_signal_ac_matrix_with_vbic_delay_mode(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
        include_vbic_dynamic_stamp: bool,
        include_vbic_delay_branches: bool,
    ) -> ComplexMatrix {
        let has_nonlinear = circuit.has_nonlinear_devices();
        let size = circuit.matrix_size();
        let mut ac_matrix = ComplexMatrix::from_real_structure(matrix);

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

        // Stamp transmission lines as distributed 2-port Y-parameters.
        for tline in &circuit.tlines {
            Self::stamp_transmission_line_ac(&mut ac_matrix, tline, omega);
        }

        // Nonlinear device Jacobian (real part) evaluated at DC operating point.
        if has_nonlinear {
            Self::stamp_nonlinear_small_signal_real(
                &mut ac_matrix,
                circuit,
                op_voltages,
                omega / (2.0 * PI),
            );
            if include_vbic_dynamic_stamp {
                for bjt in &circuit.bjts.devices {
                    Self::stamp_vbic_bjt_dynamic_ac(
                        &mut ac_matrix,
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
            Self::stamp_nonlinear_capacitances(&mut ac_matrix, circuit, op_voltages, omega);
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
        }

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

        // Add small diagonal for numerical stability
        for i in 0..size {
            ac_matrix.add_real(i, i, 1e-15);
        }

        ac_matrix
    }

    pub(super) fn build_small_signal_ac_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> ComplexMatrix {
        Self::build_small_signal_ac_matrix_with_vbic_delay_mode(
            circuit,
            matrix,
            op_voltages,
            omega,
            true,
            false,
        )
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
        Self::build_small_signal_ac_matrix_with_vbic_delay_mode(
            circuit,
            matrix,
            op_voltages,
            omega,
            false,
            true,
        )
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
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point
        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            // Align stateful nonlinear models (limited junction voltages, operating region)
            // with the final converged operating-point solution before AC linearization.
            circuit.update_nonlinear(&dc_solution);
        }

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        // Closure to solve at a single frequency
        let solve_at_freq = |freq: Value| -> Result<AcResult, SimulationError> {
            let omega = 2.0 * PI * freq;
            let ac_matrix =
                Self::build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega);
            let rhs = Self::build_ac_excitation_rhs(&circuit);
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

        // Use parallel iteration when feature is enabled and we have many frequencies
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;

            // Parallel threshold: use parallel for 10+ frequency points
            if frequencies.len() >= 10 {
                let results: Result<Vec<_>, _> = frequencies
                    .par_iter()
                    .map(|&freq| solve_at_freq(freq))
                    .collect();
                return results;
            }
        }

        // Sequential fallback (or when parallel feature disabled)
        frequencies
            .iter()
            .map(|&freq| solve_at_freq(freq))
            .collect()
    }
}
