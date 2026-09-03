#![allow(clippy::needless_range_loop)]

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::pole_zero::{
    Matrix, PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult,
};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};
use crate::{CircuitData, Netlist, Value};

impl Engine {
    /// Reduce a sparse `G + sC` descriptor to a dense state-space model whose
    /// dimension is the number of dynamic states, not the full MNA size.
    ///
    /// Algebraic variables are eliminated with the production sparse LU:
    /// `G_eff = Gdd - Gda * Gaa^-1 * Gad`. The same factorization reduces the
    /// input/output vectors, then `Cdd` is solved in one batched operation for
    /// both `A` and `B`. A singular partition returns `None` and the caller
    /// retains the generalized dense descriptor fallback.
    fn try_sparse_pz_state_space(
        g_descriptor: &crate::solver::ComplexMatrix,
        c_descriptor: &crate::solver::ComplexMatrix,
        config: &PoleZeroConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Option<PoleZeroResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let n = g_descriptor.nrows;
        if n == 0 || g_descriptor.ncols != n || c_descriptor.nrows != n || c_descriptor.ncols != n {
            return Ok(None);
        }

        let mut dynamic_mask = vec![false; n];
        c_descriptor.for_each_stored(|row, col, value| {
            // Descriptor structure is scale independent. Even a very small
            // nonzero capacitance/inductance coefficient represents a real
            // dynamic state and may create a correspondingly large finite
            // pole; an absolute or matrix-relative cutoff erases it.
            if value.im != 0.0 {
                dynamic_mask[row] = true;
                dynamic_mask[col] = true;
            }
        });
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let dynamic = dynamic_mask
            .iter()
            .enumerate()
            .filter_map(|(index, &is_dynamic)| is_dynamic.then_some(index))
            .collect::<Vec<_>>();
        if dynamic.is_empty() {
            return Ok(None);
        }
        let algebraic = dynamic_mask
            .iter()
            .enumerate()
            .filter_map(|(index, &is_dynamic)| (!is_dynamic).then_some(index))
            .collect::<Vec<_>>();
        let dynamic_count = dynamic.len();
        let algebraic_count = algebraic.len();
        let mut dynamic_map = vec![usize::MAX; n];
        let mut algebraic_map = vec![usize::MAX; n];
        for (reduced, &original) in dynamic.iter().enumerate() {
            if reduced.is_multiple_of(256) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            dynamic_map[original] = reduced;
        }
        for (reduced, &original) in algebraic.iter().enumerate() {
            if reduced.is_multiple_of(256) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            algebraic_map[original] = reduced;
        }

        let mut input = vec![0.0; n];
        if config.input_is_current {
            if config.input_pos >= n {
                return Ok(None);
            }
            input[config.input_pos] += 1.0;
            if let Some(negative) = config.input_neg {
                if negative >= n {
                    return Ok(None);
                }
                input[negative] -= 1.0;
            }
        } else if let Some(branch) = config.input_voltage_branch {
            if branch >= n {
                return Ok(None);
            }
            input[branch] = config.input_voltage_gain;
        } else {
            // A synthesized voltage-source branch changes the descriptor
            // topology; the ordinary analyzer owns that exact augmentation.
            return Ok(None);
        }
        let mut output = vec![0.0; n];
        if config.output_pos >= n {
            return Ok(None);
        }
        output[config.output_pos] += 1.0;
        if let Some(negative) = config.output_neg {
            if negative >= n {
                return Ok(None);
            }
            output[negative] -= 1.0;
        }

        let mut c_dd = Matrix::zeros(dynamic_count, dynamic_count);
        c_descriptor.for_each_stored(|row, col, value| {
            let reduced_row = dynamic_map[row];
            let reduced_col = dynamic_map[col];
            if reduced_row != usize::MAX && reduced_col != usize::MAX && value.im != 0.0 {
                c_dd.add(reduced_row, reduced_col, value.im);
            }
        });

        let mut g_dd = Matrix::zeros(dynamic_count, dynamic_count);
        let mut g_da = vec![Vec::<(usize, Value)>::new(); dynamic_count];
        let mut g_ad = vec![0.0; algebraic_count.saturating_mul(dynamic_count)];
        let mut g_aa_triplets = Vec::new();
        g_descriptor.for_each_stored(|row, col, value| {
            let value = value.re;
            if value == 0.0 {
                return;
            }
            match (dynamic_map[row], dynamic_map[col]) {
                (reduced_row, reduced_col)
                    if reduced_row != usize::MAX && reduced_col != usize::MAX =>
                {
                    g_dd.add(reduced_row, reduced_col, value);
                }
                (reduced_row, _) if reduced_row != usize::MAX => {
                    g_da[reduced_row].push((algebraic_map[col], value));
                }
                (_, reduced_col) if reduced_col != usize::MAX => {
                    let reduced_row = algebraic_map[row];
                    g_ad[reduced_col * algebraic_count + reduced_row] += value;
                }
                _ => g_aa_triplets.push((algebraic_map[row], algebraic_map[col], value)),
            }
        });

        let b_d = dynamic
            .iter()
            .map(|&index| input[index])
            .collect::<Vec<_>>();
        let b_a = algebraic
            .iter()
            .map(|&index| input[index])
            .collect::<Vec<_>>();
        let l_d = dynamic
            .iter()
            .map(|&index| output[index])
            .collect::<Vec<_>>();
        let l_a = algebraic
            .iter()
            .map(|&index| output[index])
            .collect::<Vec<_>>();

        let (g_aa_inv_g_ad, g_aa_inv_b_a) = if algebraic_count == 0 {
            (Vec::new(), Vec::new())
        } else {
            for index in 0..algebraic_count {
                g_aa_triplets.push((index, index, 0.0));
            }
            let mut g_aa = match crate::solver::StaticMatrix::from_triplets(
                algebraic_count,
                algebraic_count,
                &g_aa_triplets,
            ) {
                Ok(matrix) => matrix,
                Err(_) => return Ok(None),
            };
            let mut rhs = g_ad;
            rhs.extend_from_slice(&b_a);
            let mut solved = Vec::new();
            if g_aa
                .solve_many_into(&rhs, dynamic_count + 1, &mut solved)
                .is_err()
            {
                return Ok(None);
            }
            let b_offset = dynamic_count * algebraic_count;
            (solved[..b_offset].to_vec(), solved[b_offset..].to_vec())
        };

        let mut g_eff = g_dd;
        let b_direct = b_d.clone();
        let mut b_eff = b_d;
        let mut c_eff = l_d;
        let mut d_eff = 0.0;
        if algebraic_count > 0 {
            for row in 0..dynamic_count {
                for &(algebraic_index, conductance) in &g_da[row] {
                    b_eff[row] -= conductance * g_aa_inv_b_a[algebraic_index];
                    for col in 0..dynamic_count {
                        let correction =
                            conductance * g_aa_inv_g_ad[col * algebraic_count + algebraic_index];
                        g_eff.add(row, col, -correction);
                    }
                }
            }
            for (algebraic_index, &weight) in l_a.iter().enumerate() {
                if weight == 0.0 {
                    continue;
                }
                d_eff += weight * g_aa_inv_b_a[algebraic_index];
                for col in 0..dynamic_count {
                    c_eff[col] -= weight * g_aa_inv_g_ad[col * algebraic_count + algebraic_index];
                }
            }
        }

        // An ideal voltage source can clamp a current-input node. Sparse LU
        // then leaves only roundoff in the reduced dynamic drive, which must
        // not turn an identically-zero transfer into a qualified empty zero
        // set. Detect cancellation relative to the elimination operations and
        // let the independent dense descriptor path issue TransferExtraction.
        if config.compute_zeros && config.input_is_current {
            let solved_input_norm = g_aa_inv_b_a
                .iter()
                .map(|value| value.abs())
                .fold(0.0_f64, Value::max);
            let cancellation_tolerance = 128.0 * (n.max(1) as Value) * Value::EPSILON;
            let effectively_zero = |value: Value, scale: Value| {
                if scale == 0.0 {
                    value == 0.0
                } else {
                    value.abs() <= cancellation_tolerance * scale
                }
            };
            let dynamic_drive_is_zero = b_eff.iter().enumerate().all(|(row, value)| {
                let elimination_scale = g_da[row]
                    .iter()
                    .map(|(_, conductance)| conductance.abs() * solved_input_norm)
                    .sum::<Value>();
                effectively_zero(*value, b_direct[row].abs() + elimination_scale)
            });
            let direct_scale = l_a
                .iter()
                .map(|weight| weight.abs() * solved_input_norm)
                .sum::<Value>();
            if dynamic_drive_is_zero && effectively_zero(d_eff, direct_scale) {
                return Ok(None);
            }
        }

        let mut c_triplets = Vec::new();
        for row in 0..dynamic_count {
            for col in 0..dynamic_count {
                let value = c_dd.get(row, col);
                if value != 0.0 || row == col {
                    c_triplets.push((row, col, value));
                }
            }
        }
        let mut c_sparse = match crate::solver::StaticMatrix::from_triplets(
            dynamic_count,
            dynamic_count,
            &c_triplets,
        ) {
            Ok(matrix) => matrix,
            Err(_) => return Ok(None),
        };
        let mut rhs = Vec::with_capacity(dynamic_count * (dynamic_count + 1));
        for col in 0..dynamic_count {
            for row in 0..dynamic_count {
                rhs.push(g_eff.get(row, col));
            }
        }
        rhs.extend_from_slice(&b_eff);
        let mut solved = Vec::new();
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if c_sparse
            .solve_many_into(&rhs, dynamic_count + 1, &mut solved)
            .is_err()
        {
            return Ok(None);
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let mut a = Matrix::zeros(dynamic_count, dynamic_count);
        for col in 0..dynamic_count {
            for row in 0..dynamic_count {
                a.set(row, col, -solved[col * dynamic_count + row]);
            }
        }
        let b = solved[dynamic_count * dynamic_count..].to_vec();
        let result = match PoleZeroAnalyzer::analyze_state_space_with_abort(
            a,
            b,
            c_eff,
            d_eff,
            config,
            &format!("node{}", config.input_pos),
            &format!("node{}", config.output_pos),
            abort,
        ) {
            Ok(result) => result,
            // Sparse reduction is an optimization. Its numerical path may
            // fail while the unreduced generalized descriptor remains
            // regular, so let the caller try that independent exact path.
            Err(
                PoleZeroAnalysisError::EigenvalueFailure { .. }
                | PoleZeroAnalysisError::NonFiniteEigenvalue { .. }
                | PoleZeroAnalysisError::NumericalQualification { .. }
                | PoleZeroAnalysisError::IncompleteSpectrum { .. }
                | PoleZeroAnalysisError::InvalidSystem(_)
                | PoleZeroAnalysisError::TransferExtraction(_),
            ) => return Ok(None),
            Err(PoleZeroAnalysisError::Aborted) => return Err(SimulationError::Aborted),
            Err(error) => {
                return Err(SimulationError::Solver(
                    crate::solver::SolverError::InvalidCircuit(format!(
                        "pole-zero sparse reduction failed: {error}"
                    )),
                ));
            }
        };
        Ok(Some(result))
    }

    #[inline]
    pub(in crate::engine) fn optional_system_index(node_id: usize) -> Option<usize> {
        if node_id == 0 {
            None
        } else {
            Some(node_id - 1)
        }
    }

    #[inline]
    pub(in crate::engine) fn ac_linearization_node_voltage(
        voltages: &[Value],
        node: usize,
    ) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    /// Refuse an analysis a mixed Verilog-AMS module cannot take part in.
    ///
    /// Placed beside the XSPICE MIF boundary warning because it answers the
    /// same shape of question at the same point of every small-signal and
    /// steady-state analysis: what does this circuit contain that this analysis
    /// cannot represent. The difference is that a code model still linearizes
    /// and a process does not, so this refuses where that one warns.
    pub(in crate::engine) fn ensure_no_mixed_signal_analysis(
        circuit: &CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        #[cfg(feature = "veriloga")]
        circuit.ensure_no_mixed_signal_hosts(analysis)?;
        #[cfg(not(feature = "veriloga"))]
        let _ = (circuit, analysis);
        Ok(())
    }

    pub(in crate::engine) fn warn_xspice_mif_analysis_boundary(
        circuit: &CircuitData,
        analysis: &str,
        detail: &str,
    ) {
        if circuit.has_xspice_devices() {
            log::warn!(
                "{analysis} analysis: {detail}; XSPICE code models participate through RSpice's built-in small-signal/runtime adapters, but ngspice-style dynamic MIF analysis hooks are not available"
            );
        }
    }

    /// Refuse a circuit whose dynamic state cannot be exported as an explicit
    /// finite state in a rational `G + sC` descriptor.
    ///
    /// Pole-zero extraction is the consumer of the charge/dynamic-state
    /// capability: a delay line's descriptor is irrational, and an AC-NQS BSIM
    /// charge deficit needs a hidden state the descriptor pair has no room
    /// for. Both answers come from the declaration table rather than from a
    /// list maintained here.
    fn ensure_supported_pz_dynamic_state_descriptors(
        circuit: &CircuitData,
    ) -> Result<(), SimulationError> {
        let gaps = crate::engine::periodic_capability::dynamic_state_descriptor_gaps(circuit);
        match crate::engine::periodic_capability::summarize(&gaps) {
            None => Ok(()),
            Some(summary) => Err(SimulationError::unsupported_capability(
                "analysis.pz.device",
                format!("Pole-zero analysis does not yet support {summary}"),
            )),
        }
    }

    pub(in crate::engine) fn descriptor_expand_square(
        g_matrix: &mut Matrix,
        c_matrix: &mut Matrix,
        extra_states: usize,
    ) -> usize {
        let (n, _) = g_matrix.dims();
        let mut g_expanded = Matrix::zeros(n + extra_states, n + extra_states);
        let mut c_expanded = Matrix::zeros(n + extra_states, n + extra_states);
        for row in 0..n {
            for col in 0..n {
                g_expanded.set(row, col, g_matrix.get(row, col));
                c_expanded.set(row, col, c_matrix.get(row, col));
            }
        }
        *g_matrix = g_expanded;
        *c_matrix = c_expanded;
        n
    }

    pub(in crate::engine) fn stamp_vbic_pz_descriptor_states(
        circuit: &CircuitData,
        op_voltages: &[Value],
        g_matrix: &mut Matrix,
        c_matrix: &mut Matrix,
    ) {
        for bjt in &circuit.bjts.devices {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            if bjt.vbic_mna_promoted() {
                // Promoted VBIC: the internal states are matrix unknowns and
                // the static linearization is already in the base G matrix,
                // so only the charge derivatives join the C descriptor, on
                // their actual system rows/columns.
                let (branches, _, _) = bjt.vbic_mna_charge_state_at_solution(op_voltages);
                let external_nodes = [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ];
                for branch in branches.iter() {
                    if !branch.is_active() {
                        continue;
                    }
                    // The qxf delay charges join the C descriptor like every
                    // other branch: ngspice-46 keeps the excess-phase network
                    // in small-signal analysis (vbicacld.c XQxf stamps), and
                    // its CEamp pole set carries the corresponding xf Bessel
                    // pair at (-3 +- j*sqrt(3))/(2*TD).
                    let mut stamp_row = |row: crate::NodeId, sign: Value| {
                        let Some(row_idx) = Self::optional_system_index(row) else {
                            return;
                        };
                        for col in 0..BJT_INTERNAL_STATE_DIM {
                            let c = branch.d_internal[col];
                            if c != 0.0
                                && let Some(col_idx) =
                                    Self::optional_system_index(bjt.vbic_internal_node(col))
                            {
                                c_matrix.add(row_idx, col_idx, sign * c);
                            }
                        }
                        for col in 0..BJT_EXTERNAL_STATE_DIM {
                            let c = branch.d_external[col];
                            if c != 0.0
                                && let Some(col_idx) =
                                    Self::optional_system_index(external_nodes[col])
                            {
                                c_matrix.add(row_idx, col_idx, sign * c);
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
                continue;
            }

            let vc = Self::ac_linearization_node_voltage(op_voltages, bjt.node_collector);
            let vb = Self::ac_linearization_node_voltage(op_voltages, bjt.node_base);
            let ve = Self::ac_linearization_node_voltage(op_voltages, bjt.node_emitter);
            let vs = Self::ac_linearization_node_voltage(op_voltages, bjt.node_substrate);
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
                if branch_idx + 2 >= BJT_DYNAMIC_CHARGE_COUNT {
                    // ngspice small-signal parity excludes VBIC excess-phase TD
                    // companion charges from linearized frequency-domain matrices.
                    continue;
                }
                branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
                has_dynamic_charge = true;
            }
            if !has_dynamic_charge {
                continue;
            }

            let internal_start =
                Self::descriptor_expand_square(g_matrix, c_matrix, BJT_INTERNAL_STATE_DIM);
            let external_nodes = [
                Self::optional_system_index(bjt.node_collector),
                Self::optional_system_index(bjt.node_base),
                Self::optional_system_index(bjt.node_emitter),
                Self::optional_system_index(bjt.node_substrate),
            ];

            for ext_row in 0..BJT_EXTERNAL_STATE_DIM {
                let Some(row_idx) = external_nodes[ext_row] else {
                    continue;
                };

                for ext_col in 0..BJT_EXTERNAL_STATE_DIM {
                    let Some(col_idx) = external_nodes[ext_col] else {
                        continue;
                    };
                    g_matrix.add(
                        row_idx,
                        col_idx,
                        snapshot.reduction.g_ee[ext_row][ext_col]
                            - snapshot.reduction.g_reduced[ext_row][ext_col],
                    );
                    c_matrix.add(row_idx, col_idx, -c_ee[ext_row][ext_col]);
                }

                for int_col in 0..BJT_INTERNAL_STATE_DIM {
                    let col_idx = internal_start + int_col;
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ei[ext_row][int_col]);
                    c_matrix.add(row_idx, col_idx, -c_ei[ext_row][int_col]);
                }
            }

            for int_row in 0..BJT_INTERNAL_STATE_DIM {
                let row_idx = internal_start + int_row;

                for ext_col in 0..BJT_EXTERNAL_STATE_DIM {
                    let Some(col_idx) = external_nodes[ext_col] else {
                        continue;
                    };
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ie[int_row][ext_col]);
                    c_matrix.add(row_idx, col_idx, -c_ie[int_row][ext_col]);
                }

                for int_col in 0..BJT_INTERNAL_STATE_DIM {
                    let col_idx = internal_start + int_col;
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ii[int_row][int_col]);
                    c_matrix.add(row_idx, col_idx, -c_ii[int_row][int_col]);
                }
            }
        }
    }

    /// Run pole-zero analysis
    ///
    /// Finds poles and zeros of the transfer function from input to output node.
    /// Uses the MNA formulation: (G + s·C)·V = I
    pub fn run_pz(
        &self,
        netlist: &Netlist,
        input_node: usize,
        output_node: usize,
    ) -> Result<PoleZeroResult, SimulationError> {
        self.run_pz_with_abort(netlist, input_node, output_node, &NoAbort)
    }

    /// Cancellable form of [`Self::run_pz`].
    pub(crate) fn run_pz_with_abort(
        &self,
        netlist: &Netlist,
        input_node: usize,
        output_node: usize,
        abort: &dyn AbortSignal,
    ) -> Result<PoleZeroResult, SimulationError> {
        self.run_pz_ports_with_abort(
            netlist,
            input_node,
            None,
            output_node,
            None,
            true,
            true,
            true,
            abort,
        )
    }

    /// Run pole-zero analysis with explicit differential ports and mode control.
    pub fn run_pz_ports(
        &self,
        netlist: &Netlist,
        input_pos: usize,
        input_neg: Option<usize>,
        output_pos: usize,
        output_neg: Option<usize>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
    ) -> Result<PoleZeroResult, SimulationError> {
        self.run_pz_ports_with_abort(
            netlist,
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            input_is_current,
            compute_poles,
            compute_zeros,
            &NoAbort,
        )
    }

    /// Cancellable form of [`Self::run_pz_ports`].
    #[allow(clippy::too_many_arguments)]
    pub fn run_pz_ports_with_abort(
        &self,
        netlist: &Netlist,
        input_pos: usize,
        input_neg: Option<usize>,
        output_pos: usize,
        output_neg: Option<usize>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
        abort: &dyn AbortSignal,
    ) -> Result<PoleZeroResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut circuit = self.build_circuit_with_abort(netlist, abort)?;
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "Pole-zero",
            "using the AC linearization path because ngspice MIF code models do not provide DEVpzLoad hooks",
        );
        Self::ensure_no_mixed_signal_analysis(&circuit, "pole-zero analysis")?;
        Self::ensure_supported_dynamic_charges(&circuit, "Pole-zero")?;
        Self::ensure_supported_pz_dynamic_state_descriptors(&circuit)?;
        let num_nodes = circuit.num_nodes();

        let validate_node = |node: usize, label: &str| -> Result<(), SimulationError> {
            if node > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "Invalid node for PZ analysis: {}={} (max={})",
                    label, node, num_nodes
                )));
            }
            Ok(())
        };

        validate_node(input_pos, "input_pos")?;
        if let Some(node) = input_neg {
            validate_node(node, "input_neg")?;
        }
        validate_node(output_pos, "output_pos")?;
        if let Some(node) = output_neg {
            validate_node(node, "output_neg")?;
        }

        if input_pos == 0 {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for PZ analysis: input_pos={} (must be non-ground)",
                input_pos
            )));
        }
        if output_pos == 0 {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for PZ analysis: output_pos={} (must be non-ground)",
                output_pos
            )));
        }
        if input_neg == Some(input_pos) {
            return Err(SimulationError::Circuit(
                "Invalid PZ input port: input_pos and input_neg cannot be the same".to_string(),
            ));
        }
        if output_neg == Some(output_pos) {
            return Err(SimulationError::Circuit(
                "Invalid PZ output port: output_pos and output_neg cannot be the same".to_string(),
            ));
        }

        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution =
            self.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?;
        abort.observe_progress(0.25);
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        let matrix_size = circuit.matrix_size();
        self.ensure_result_shape(matrix_size, matrix_size.saturating_mul(8).saturating_add(1))?;

        // Reuse the AC linearization path so pole-zero analysis sees the same
        // nonlinear small-signal conductances and capacitances as AC analysis.
        let g_descriptor =
            Self::try_build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 0.0)?;
        let c_descriptor =
            Self::try_build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 1.0)?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        abort.observe_progress(0.5);
        let input_neg_node = input_neg.unwrap_or(0);
        let matches_requested_input_port = |np: usize, nn: usize| {
            (np == input_pos && nn == input_neg_node) || (nn == input_pos && np == input_neg_node)
        };
        let matches_input_voltage_port =
            |np: usize, nn: usize| !input_is_current && matches_requested_input_port(np, nn);
        let mut input_voltage_branch = None;
        let mut input_voltage_gain = 1.0;

        // Stamp independent voltage sources into G (MNA branch equations).
        // If a deck already contains an ideal source on the requested voltage
        // input port, use that branch directly as the excitation variable
        // instead of synthesizing a parallel source later.
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;

            if input_is_current && compute_zeros && matches_requested_input_port(np, nn) {
                return Err(SimulationError::Solver(
                    crate::solver::SolverError::InvalidCircuit(
                        "pole-zero transfer extraction failed: the requested current input is parallel to an independent ideal voltage source"
                            .to_string(),
                    ),
                ));
            }

            if matches_input_voltage_port(np, nn) {
                if input_voltage_branch.replace(br).is_some() {
                    return Err(SimulationError::Circuit(
                        "Multiple independent voltage sources drive the requested PZ input port"
                            .to_string(),
                    ));
                }
                input_voltage_gain = if np == input_pos && nn == input_neg_node {
                    1.0
                } else {
                    -1.0
                };
            }
        }

        let mut config = PoleZeroConfig::poles_and_zeros(input_pos - 1, output_pos - 1);
        config.input_neg = input_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.output_neg = output_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.input_is_current = input_is_current;
        config.input_voltage_branch = input_voltage_branch;
        config.input_voltage_gain = input_voltage_gain;
        config.compute_poles = compute_poles;
        config.compute_zeros = compute_zeros;

        // VBIC can introduce descriptor states that are not represented in
        // the frozen AC matrix. All other native small-signal devices expose
        // their complete G/C topology there and are eligible for sparse
        // algebraic elimination before dense eigenvalue work.
        let has_external_vbic_descriptor_states = circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.uses_vbic_dynamic_charges());
        if !has_external_vbic_descriptor_states
            && let Some(result) =
                Self::try_sparse_pz_state_space(&g_descriptor, &c_descriptor, &config, abort)?
        {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            self.ensure_result_values(
                result
                    .poles
                    .len()
                    .saturating_add(result.zeros.len())
                    .saturating_mul(2)
                    .saturating_add(2),
            )?;
            abort.observe_progress(1.0);
            return Ok(result);
        }

        let mut g_matrix = Matrix::from_dense(g_descriptor.to_dense_real());
        let mut c_matrix = Matrix::from_dense(c_descriptor.to_dense_imag());
        Self::stamp_vbic_pz_descriptor_states(&circuit, &dc_solution, &mut g_matrix, &mut c_matrix);
        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);

        let result = analyzer
            .analyze_with_abort(&config, abort)
            .map_err(|error| match error {
                PoleZeroAnalysisError::Aborted => SimulationError::Aborted,
                error => SimulationError::Solver(crate::solver::SolverError::InvalidCircuit(
                    format!("pole-zero extraction failed: {error}"),
                )),
            })?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_result_values(
            result
                .poles
                .len()
                .saturating_add(result.zeros.len())
                .saturating_mul(2)
                .saturating_add(2),
        )?;
        abort.observe_progress(1.0);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_roots_close(actual: &[crate::Complex64], expected: &[crate::Complex64]) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "actual={actual:#?}, expected={expected:#?}"
        );
        for (actual, expected) in actual.iter().zip(expected) {
            let scale = actual.norm().max(expected.norm()).max(1.0);
            assert!(
                (actual.re - expected.re).abs() <= 1.0e-8 * scale
                    && (actual.im - expected.im).abs() <= 1.0e-8 * scale,
                "actual={actual}, expected={expected}"
            );
        }
    }

    #[test]
    fn sparse_pz_reduction_eliminates_algebraic_mna_nodes() {
        let netlist = Netlist::parse(
            "Sparse PZ algebraic reduction\n\
             R1 in out 1k\n\
             R2 in 0 1k\n\
             C1 out 0 1u\n\
             .END\n",
        )
        .expect("PZ deck parses");
        let result = Engine::default()
            .run_pz(&netlist, 1, 2)
            .expect("sparse PZ succeeds");

        assert_eq!(result.poles.len(), 1, "{:#?}", result.poles);
        let pole = result.poles[0];
        assert!(pole.im.abs() <= 1.0e-10, "unexpected complex pole {pole}");
        assert!(
            (pole.re + 500.0).abs() <= 1.0e-8,
            "expected -500 rad/s, got {pole}"
        );
        assert!(
            result.zeros.is_empty(),
            "unexpected zeros: {:#?}",
            result.zeros
        );
        let dc_gain = result.dc_gain.expect("finite DC transimpedance");
        assert!(
            (dc_gain - 1_000.0).abs() <= 1.0e-8,
            "expected 1 kohm DC transimpedance, got {dc_gain}"
        );
    }

    #[test]
    fn sparse_pz_state_space_matches_dense_descriptor_poles_zeros_and_gain() {
        let structure = crate::solver::StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .expect("full two-state structure");
        let mut g_sparse = crate::solver::ComplexMatrix::from_real_structure(&structure);
        let mut c_sparse = crate::solver::ComplexMatrix::from_real_structure(&structure);
        let g = [[1.5e-3, -1.0e-3], [-1.0e-3, 4.0e-3 / 3.0]];
        let c = [[1.2e-6, -1.0e-6], [-1.0e-6, 1.4e-6]];
        for row in 0..2 {
            for col in 0..2 {
                g_sparse.add_real(row, col, g[row][col]);
                c_sparse.add_imag(row, col, c[row][col]);
            }
        }
        let config = PoleZeroConfig::poles_and_zeros(0, 1);
        let sparse = Engine::try_sparse_pz_state_space(&g_sparse, &c_sparse, &config, &NoAbort)
            .expect("sparse reduction does not error")
            .expect("descriptor is reducible");
        let dense = PoleZeroAnalyzer::new(
            Matrix::from_dense(g.iter().map(|row| row.to_vec()).collect()),
            Matrix::from_dense(c.iter().map(|row| row.to_vec()).collect()),
        )
        .analyze(&config)
        .expect("dense descriptor extraction");

        assert_roots_close(&sparse.poles, &dense.poles);
        assert_roots_close(&sparse.zeros, &dense.zeros);
        let sparse_gain = sparse.dc_gain.expect("sparse finite DC gain");
        let dense_gain = dense.dc_gain.expect("dense finite DC gain");
        assert!(
            (sparse_gain - dense_gain).abs()
                <= 1.0e-10 * sparse_gain.abs().max(dense_gain.abs()).max(1.0),
            "sparse gain={sparse_gain}, dense gain={dense_gain}"
        );
    }

    #[test]
    fn sparse_pz_mixed_scale_capacitances_match_dense_spectrum() {
        // Both diagonal entries are genuine dynamic states despite their very
        // different scales. The sparse topology census must use structural
        // nonzero membership and retain the 2e-18 entry just as the dense
        // descriptor path does.
        let structure = crate::solver::StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .expect("full two-state structure");
        let mut g_sparse = crate::solver::ComplexMatrix::from_real_structure(&structure);
        let mut c_sparse = crate::solver::ComplexMatrix::from_real_structure(&structure);
        let g = [[1.0e-3, 0.0], [0.0, 2.0e-3]];
        let c = [[1.0e-6, 0.0], [0.0, 2.0e-18]];
        for row in 0..2 {
            for col in 0..2 {
                g_sparse.add_real(row, col, g[row][col]);
                c_sparse.add_imag(row, col, c[row][col]);
            }
        }

        let mut config = PoleZeroConfig::poles_and_zeros(0, 1);
        config.compute_zeros = false;
        let sparse = Engine::try_sparse_pz_state_space(&g_sparse, &c_sparse, &config, &NoAbort)
            .expect("sparse reduction does not error")
            .expect("both mixed-scale capacitances remain dynamic");
        let dense = PoleZeroAnalyzer::new(
            Matrix::from_dense(g.iter().map(|row| row.to_vec()).collect()),
            Matrix::from_dense(c.iter().map(|row| row.to_vec()).collect()),
        )
        .analyze(&config)
        .expect("dense mixed-scale descriptor extraction");

        assert_roots_close(&sparse.poles, &dense.poles);
        assert_root_present_for_mixed_scale(&sparse.poles, -1.0e3);
        assert_root_present_for_mixed_scale(&sparse.poles, -1.0e15);
    }

    fn assert_root_present_for_mixed_scale(actual: &[crate::Complex64], expected: Value) {
        assert!(
            actual.iter().any(|root| {
                root.im.abs() <= 1.0e-8 * expected.abs().max(1.0)
                    && (root.re - expected).abs() <= 1.0e-8 * expected.abs().max(1.0)
            }),
            "missing expected pole {expected:.6e}; actual={actual:#?}"
        );
    }
}
