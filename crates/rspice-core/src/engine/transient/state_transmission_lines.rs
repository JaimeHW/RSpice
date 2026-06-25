//! Transmission-line transient companion and reference-history helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn stamp_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        _tline_dc_refs: &[(Value, Value)],
    ) {
        for tl in &circuit.tlines {
            if tl.has_txl_runtime() {
                if let Some(stamp) = tl.txl_transient_stamp(time) {
                    Self::stamp_txl_branch_runtime(matrix, rhs, tl, stamp);
                }
            } else if tl.ltra_branch_matrix_indices().is_some() {
                let response = tl.transient_port_response(time);
                Self::stamp_ltra_branch_runtime(matrix, rhs, tl, response);
            } else {
                let response = tl.transient_port_response(time);
                Self::stamp_tline_two_port(matrix, rhs, tl, response);
            }
        }
    }

    #[inline]
    pub(super) fn stamp_coupled_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        dt: Value,
        coupled_tline_refs: &[CoupledTlineReferenceState],
    ) {
        for (idx, tl) in circuit.coupled_tlines.iter().enumerate() {
            if tl.uses_native_runtime() {
                Self::stamp_native_cpl_branch_companions(matrix, rhs, tl, time, dt);
                continue;
            }

            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let incoming_near = tl.incoming_near_modal(time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(time, &refs.near_modal);
            let eq_near = tl.port_equivalent_current(&incoming_near);
            let eq_far = tl.port_equivalent_current(&incoming_far);

            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.near_nodes,
                tl.near_ref,
                tl.port_admittance(),
                &eq_near,
            );
            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.far_nodes,
                tl.far_ref,
                tl.port_admittance(),
                &eq_far,
            );
        }
    }

    /// Stamp the ngspice-faithful CPL branch-current convolution equations.
    ///
    /// Mirrors the matrix pointers written by `CPLload` (cplload.c) for the
    /// transient (non-MODEDC) path:
    /// - KCL: `pos[m]` row gets `+Ibr1[m]`, `far[m]` row gets `+Ibr2[m]`.
    /// - Branch1 row m: `-Ibr1[m] + sum_p aten_h1[m][p] Vpos[p]
    ///   - sum_p f3[m][p] Vfar[p] - sum_p f2[m][p] Ibr2[p] = ff[m]`.
    /// - Branch2 row m: `-Ibr2[m] + sum_p aten_h1[m][p] Vfar[p]
    ///   - sum_p f3[m][p] Vpos[p] - sum_p f2[m][p] Ibr1[p] = gg[m]`.
    ///
    /// When the native step plan is unavailable (e.g. degenerate dt before the
    /// runtime is seeded) the branch rows are anchored with `-Ibr=0` so the
    /// matrix stays non-singular.
    #[inline]
    fn stamp_native_cpl_branch_companions(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::CoupledTransmissionLine,
        time: Value,
        dt: Value,
    ) {
        let Some(branches) = tl.native_branch_matrix_indices() else {
            return;
        };
        let conductors = tl.conductors();

        // KCL coupling and branch-diagonal anchors are always present.
        for conductor in 0..conductors {
            let Some((b1, b2)) = branches.conductor(conductor) else {
                continue;
            };
            let near = tl.near_nodes[conductor];
            let far = tl.far_nodes[conductor];
            if near > 0 {
                matrix.add(near - 1, b1 - 1, 1.0);
            }
            if far > 0 {
                matrix.add(far - 1, b2 - 1, 1.0);
            }
            matrix.add(b1 - 1, b1 - 1, -1.0);
            matrix.add(b2 - 1, b2 - 1, -1.0);
        }

        let Some(plan) = tl.native_step_plan(time, dt) else {
            return;
        };

        for m in 0..conductors {
            let Some((b1_m, b2_m)) = branches.conductor(m) else {
                continue;
            };
            for p in 0..conductors {
                let Some((b1_p, b2_p)) = branches.conductor(p) else {
                    continue;
                };
                let near_p = tl.near_nodes[p];
                let far_p = tl.far_nodes[p];

                let aten_h1 = plan.aten_h1[m][p];
                if aten_h1 != 0.0 {
                    if near_p > 0 {
                        matrix.add(b1_m - 1, near_p - 1, aten_h1);
                    }
                    if far_p > 0 {
                        matrix.add(b2_m - 1, far_p - 1, aten_h1);
                    }
                }

                if plan.ext {
                    let f3 = plan.f3[m][p];
                    if f3 != 0.0 {
                        if far_p > 0 {
                            matrix.add(b1_m - 1, far_p - 1, -f3);
                        }
                        if near_p > 0 {
                            matrix.add(b2_m - 1, near_p - 1, -f3);
                        }
                    }
                    let f2 = plan.f2[m][p];
                    if f2 != 0.0 {
                        matrix.add(b1_m - 1, b2_p - 1, -f2);
                        matrix.add(b2_m - 1, b1_p - 1, -f2);
                    }
                }
            }
            rhs[b1_m - 1] += plan.ff[m];
            rhs[b2_m - 1] += plan.gg[m];
        }
    }

    #[inline]
    pub(super) fn initialize_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<(Value, Value)> {
        let mut refs = Vec::with_capacity(circuit.tlines.len());
        for tl in &mut circuit.tlines {
            tl.reset();
            let z_port = Self::tline_transient_port_impedance(tl);
            let g = 1.0 / z_port;
            let v1 = Self::differential_voltage(initial_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(initial_solution, tl.node2_pos, tl.node2_neg);
            refs.push((v1, v2));
            if let Some((br1, br2)) = tl.txl_branch_matrix_indices() {
                let i1 = initial_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = initial_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.initialize_txl_history(initial_time, v1, i1, v2, i2);
                continue;
            }
            if let Some((br1, br2)) = tl.ltra_branch_matrix_indices() {
                let i1 = initial_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = initial_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.update_history(initial_time, v1, i1, v2, i2);
                continue;
            }

            // Seed delayed-wave state from the initial OP so pre-edge steady states
            // are preserved (avoids artificial startup droop/ringing).
            // Port equations: i1 = g*(v1 - incoming1), i2 = g*(v2 - incoming2),
            // with incoming1 <- v2 and incoming2 <- v1 at t=0.
            let i1_actual = g * (v1 - v2);
            let i2_actual = g * (v2 - v1);
            let wave_scale = z_port / tl.impedance();
            tl.update_history(
                initial_time,
                v1,
                i1_actual * wave_scale,
                v2,
                i2_actual * wave_scale,
            );
        }
        refs
    }

    #[inline]
    pub(super) fn initialize_coupled_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<CoupledTlineReferenceState> {
        let mut refs = Vec::with_capacity(circuit.coupled_tlines.len());
        for tl in &mut circuit.coupled_tlines {
            tl.reset();
            let near_physical =
                Self::differential_port_voltages(initial_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(initial_solution, &tl.far_nodes, tl.far_ref);

            // Seed the ngspice-faithful native convolution runtime from the DC
            // operating point. When active, the line uses the branch-current
            // transient stamp instead of the modal Norton path below, but we
            // still populate the modal reference state to keep the per-line
            // bookkeeping uniform and harmless for native lines.
            if tl.uses_native_runtime() {
                if let Err(err) = tl.native_seed_dc(&near_physical, &far_physical) {
                    log::warn!("{err}");
                }
            }

            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let near_currents = tl.port_currents(&near_physical, &far_modal);
            let far_currents = tl.port_currents(&far_physical, &near_modal);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                initial_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            refs.push(CoupledTlineReferenceState {
                near_modal,
                far_modal,
            });
        }
        refs
    }
}
