use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::advanced) fn optional_system_index(node_id: usize) -> Option<usize> {
        if node_id == 0 {
            None
        } else {
            Some(node_id - 1)
        }
    }

    #[inline]
    pub(in crate::engine::advanced) fn ac_linearization_node_voltage(
        voltages: &[Value],
        node: usize,
    ) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    fn ensure_supported_bsim_ac_nqs_pz_models(
        circuit: &CircuitData,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.bsim3v3.devices {
            if dev.core.model.acnqs_mod != 0 {
                return Err(SimulationError::Circuit(format!(
                    "Pole-zero analysis does not yet support BSIM3 '{}' with ACNQSMOD=1; \
                     AC-NQS is a rational charge-deficit effect and needs a hidden \
                     charge-deficit state instead of G+sC descriptor extraction",
                    dev.name
                )));
            }
        }
        for dev in &circuit.bsim4v8.devices {
            if dev.core.model.acnqs_mod != 0 {
                return Err(SimulationError::Circuit(format!(
                    "Pole-zero analysis does not yet support BSIM4 '{}' with ACNQSMOD=1; \
                     AC-NQS is a rational charge-deficit effect and needs a hidden \
                     charge-deficit state instead of G+sC descriptor extraction",
                    dev.name
                )));
            }
        }
        Ok(())
    }

    pub(in crate::engine::advanced) fn descriptor_expand_square(
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

    pub(in crate::engine::advanced) fn stamp_vbic_pz_descriptor_states(
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
        self.run_pz_ports(
            netlist,
            input_node,
            None,
            output_node,
            None,
            true,
            true,
            true,
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
        let mut circuit = self.build_circuit(netlist)?;
        Self::ensure_supported_dynamic_charges(&circuit, "Pole-zero")?;
        Self::ensure_supported_bsim_ac_nqs_pz_models(&circuit)?;
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

        if !circuit.tlines.is_empty() {
            return Err(SimulationError::Circuit(
                "Pole-zero analysis does not yet support transmission lines".to_string(),
            ));
        }

        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution = self.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }

        // Reuse the AC linearization path so pole-zero analysis sees the same
        // nonlinear small-signal conductances and capacitances as AC analysis.
        let g_descriptor = Self::build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 0.0);
        let c_descriptor = Self::build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 1.0);
        let mut g_matrix = Matrix::from_dense(g_descriptor.to_dense_real());
        let mut c_matrix = Matrix::from_dense(c_descriptor.to_dense_imag());
        Self::stamp_vbic_pz_descriptor_states(&circuit, &dc_solution, &mut g_matrix, &mut c_matrix);

        let input_neg_node = input_neg.unwrap_or(0);
        let matches_input_voltage_port = |np: usize, nn: usize| {
            !input_is_current
                && ((np == input_pos && nn == input_neg_node)
                    || (nn == input_pos && np == input_neg_node))
        };
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

        // Create analyzer and run
        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::poles_and_zeros(input_pos - 1, output_pos - 1);
        config.input_neg = input_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.output_neg = output_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.input_is_current = input_is_current;
        config.input_voltage_branch = input_voltage_branch;
        config.input_voltage_gain = input_voltage_gain;
        config.compute_poles = compute_poles;
        config.compute_zeros = compute_zeros;

        Ok(analyzer.analyze(&config))
    }
}
