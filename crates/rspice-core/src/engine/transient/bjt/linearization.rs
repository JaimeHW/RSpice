//! BJT transient linearization and reduced-system assembly.
//! VBIC thermal dynamics use the promoted MNA charge branches.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::transient) fn assemble_legacy_bjt_transient_linearization(
        bjt: &crate::device::Bjt,
        snapshot: &crate::device::semiconductor::BjtChargeSnapshot,
        step: BjtChargeStep<'_>,
    ) -> Option<BjtTransientLinearization> {
        let BjtChargeStep {
            coeff,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        } = step;
        let charge_factor = Self::jfet_companion_geq(coeff, 1.0, dt);
        if charge_factor <= 0.0 {
            return None;
        }

        let mut g_ii = snapshot.reduction.g_ii;
        let mut g_ie = snapshot.reduction.g_ie;
        let mut g_ei = snapshot.reduction.g_ei;
        let mut g_ee = snapshot.reduction.g_ee;
        let mut z_i = snapshot.reduction.z_i_static;
        let mut z_e = snapshot.reduction.z_e_static;

        let (vbe, vbc, vbx, vcs) = Self::legacy_bjt_charge_branch_voltages_with_vbx(snapshot);
        let charges = bjt.legacy_transient_charge_state_with_vbx(vbe, vbc, vbx, vcs);
        let qbe_branch = snapshot.branches[BJT_QBE_BRANCH_INDEX];
        let qbc_branch = snapshot.branches[BJT_QBC_BRANCH_INDEX];
        let qbx_branch = snapshot.branches[BJT_QBCX_BRANCH_INDEX];
        let qcs_branch = snapshot.branches[BJT_QBCP_BRANCH_INDEX];
        let mut has_dynamic_charge = false;

        if qbe_branch.is_active() {
            let cqbe = Self::jfet_companion_ccap(
                coeff,
                dt,
                charges.qbe,
                BranchChargeHistory {
                    q_prev: q_prev[BJT_QBE_BRANCH_INDEX],
                    q_prev_prev: q_prev_prev[BJT_QBE_BRANCH_INDEX],
                    cq_prev: cq_prev[BJT_QBE_BRANCH_INDEX],
                },
            );
            let geqbe = charge_factor * charges.capbe;
            let i_eq = geqbe
                * Self::legacy_bjt_internal_branch_voltage(
                    snapshot,
                    BJT_VBI_STATE_INDEX,
                    BJT_VEI_STATE_INDEX,
                )
                - cqbe;
            Self::stamp_legacy_bjt_two_terminal_companion(
                &qbe_branch,
                BJT_VBI_STATE_INDEX,
                BJT_VEI_STATE_INDEX,
                geqbe,
                i_eq,
                BjtCompanionSystem {
                    g_ii: &mut g_ii,
                    g_ie: &mut g_ie,
                    g_ei: &mut g_ei,
                    g_ee: &mut g_ee,
                    z_i: &mut z_i,
                    z_e: &mut z_e,
                },
            );
            has_dynamic_charge = true;

            if charges.capbe_vbc != 0.0 {
                let geqcb = charge_factor * charges.capbe_vbc;
                let vbc = Self::legacy_bjt_internal_branch_voltage(
                    snapshot,
                    BJT_VBI_STATE_INDEX,
                    BJT_VCI_STATE_INDEX,
                );
                let i_eq = geqcb * vbc;
                Self::stamp_legacy_bjt_controlled_companion(
                    &qbe_branch,
                    BJT_VBI_STATE_INDEX,
                    BJT_VCI_STATE_INDEX,
                    geqcb,
                    i_eq,
                    BjtCompanionSystem {
                        g_ii: &mut g_ii,
                        g_ie: &mut g_ie,
                        g_ei: &mut g_ei,
                        g_ee: &mut g_ee,
                        z_i: &mut z_i,
                        z_e: &mut z_e,
                    },
                );
                has_dynamic_charge = true;
            }
        }

        if qbc_branch.is_active() {
            let cqbc = Self::jfet_companion_ccap(
                coeff,
                dt,
                charges.qbc,
                BranchChargeHistory {
                    q_prev: q_prev[BJT_QBC_BRANCH_INDEX],
                    q_prev_prev: q_prev_prev[BJT_QBC_BRANCH_INDEX],
                    cq_prev: cq_prev[BJT_QBC_BRANCH_INDEX],
                },
            );
            let geqbc = charge_factor * charges.capbc;
            let i_eq = geqbc
                * Self::legacy_bjt_internal_branch_voltage(
                    snapshot,
                    BJT_VBI_STATE_INDEX,
                    BJT_VCI_STATE_INDEX,
                )
                - cqbc;
            Self::stamp_legacy_bjt_two_terminal_companion(
                &qbc_branch,
                BJT_VBI_STATE_INDEX,
                BJT_VCI_STATE_INDEX,
                geqbc,
                i_eq,
                BjtCompanionSystem {
                    g_ii: &mut g_ii,
                    g_ie: &mut g_ie,
                    g_ei: &mut g_ei,
                    g_ee: &mut g_ee,
                    z_i: &mut z_i,
                    z_e: &mut z_e,
                },
            );
            has_dynamic_charge = true;
        }

        if qbx_branch.is_active() {
            let cqbx = Self::jfet_companion_ccap(
                coeff,
                dt,
                charges.qbx,
                BranchChargeHistory {
                    q_prev: q_prev[BJT_QBCX_BRANCH_INDEX],
                    q_prev_prev: q_prev_prev[BJT_QBCX_BRANCH_INDEX],
                    cq_prev: cq_prev[BJT_QBCX_BRANCH_INDEX],
                },
            );
            let geqbx = charge_factor * charges.capbx;
            let i_eq = geqbx * Self::legacy_bjt_charge_branch_voltage(snapshot, &qbx_branch) - cqbx;
            Self::stamp_legacy_bjt_branch_voltage_companion(
                &qbx_branch,
                geqbx,
                i_eq,
                BjtCompanionSystem {
                    g_ii: &mut g_ii,
                    g_ie: &mut g_ie,
                    g_ei: &mut g_ei,
                    g_ee: &mut g_ee,
                    z_i: &mut z_i,
                    z_e: &mut z_e,
                },
            );
            has_dynamic_charge = true;
        }

        if qcs_branch.is_active() {
            let cqcs = Self::jfet_companion_ccap(
                coeff,
                dt,
                charges.qcs,
                BranchChargeHistory {
                    q_prev: q_prev[BJT_QBCP_BRANCH_INDEX],
                    q_prev_prev: q_prev_prev[BJT_QBCP_BRANCH_INDEX],
                    cq_prev: cq_prev[BJT_QBCP_BRANCH_INDEX],
                },
            );
            let geqcs = charge_factor * charges.capcs;
            let i_eq = geqcs * Self::legacy_bjt_charge_branch_voltage(snapshot, &qcs_branch) - cqcs;
            Self::stamp_legacy_bjt_branch_voltage_companion(
                &qcs_branch,
                geqcs,
                i_eq,
                BjtCompanionSystem {
                    g_ii: &mut g_ii,
                    g_ie: &mut g_ie,
                    g_ei: &mut g_ei,
                    g_ee: &mut g_ee,
                    z_i: &mut z_i,
                    z_e: &mut z_e,
                },
            );
            has_dynamic_charge = true;
        }

        has_dynamic_charge.then_some(BjtTransientLinearization {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        })
    }

    #[inline]
    fn legacy_bjt_internal_branch_voltage(
        snapshot: &crate::device::semiconductor::BjtChargeSnapshot,
        pos: usize,
        neg: usize,
    ) -> Value {
        snapshot.reduction.internal_voltages[pos] - snapshot.reduction.internal_voltages[neg]
    }

    #[inline]
    fn stamp_legacy_bjt_two_terminal_companion(
        branch: &BjtChargeBranch,
        control_pos_internal: usize,
        control_neg_internal: usize,
        geq: Value,
        i_eq: Value,
        system: BjtCompanionSystem<'_>,
    ) {
        let BjtCompanionSystem {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        } = system;
        Self::stamp_legacy_bjt_controlled_companion(
            branch,
            control_pos_internal,
            control_neg_internal,
            geq,
            i_eq,
            BjtCompanionSystem {
                g_ii,
                g_ie,
                g_ei,
                g_ee,
                z_i,
                z_e,
            },
        );
    }

    #[inline]
    fn stamp_legacy_bjt_controlled_companion(
        current_branch: &BjtChargeBranch,
        control_pos_internal: usize,
        control_neg_internal: usize,
        transconductance: Value,
        i_eq: Value,
        system: BjtCompanionSystem<'_>,
    ) {
        let BjtCompanionSystem {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        } = system;
        let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        let d_external = [0.0; BJT_EXTERNAL_STATE_DIM];
        d_internal[control_pos_internal] += transconductance;
        d_internal[control_neg_internal] -= transconductance;
        Self::stamp_legacy_bjt_companion(
            current_branch,
            &d_internal,
            &d_external,
            i_eq,
            BjtCompanionSystem {
                g_ii,
                g_ie,
                g_ei,
                g_ee,
                z_i,
                z_e,
            },
        );
    }

    #[inline]
    fn stamp_legacy_bjt_branch_voltage_companion(
        branch: &BjtChargeBranch,
        transconductance: Value,
        i_eq: Value,
        system: BjtCompanionSystem<'_>,
    ) {
        let BjtCompanionSystem {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        } = system;
        let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        let mut d_external = [0.0; BJT_EXTERNAL_STATE_DIM];
        Self::add_legacy_bjt_terminal_control(
            branch.pos_internal,
            branch.pos_external,
            transconductance,
            &mut d_internal,
            &mut d_external,
        );
        Self::add_legacy_bjt_terminal_control(
            branch.neg_internal,
            branch.neg_external,
            -transconductance,
            &mut d_internal,
            &mut d_external,
        );
        Self::stamp_legacy_bjt_companion(
            branch,
            &d_internal,
            &d_external,
            i_eq,
            BjtCompanionSystem {
                g_ii,
                g_ie,
                g_ei,
                g_ee,
                z_i,
                z_e,
            },
        );
    }

    #[inline]
    fn add_legacy_bjt_terminal_control(
        internal: Option<usize>,
        external: Option<usize>,
        value: Value,
        d_internal: &mut [Value; BJT_INTERNAL_STATE_DIM],
        d_external: &mut [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        if let Some(idx) = internal {
            d_internal[idx] += value;
        } else if let Some(idx) = external {
            d_external[idx] += value;
        }
    }

    #[inline]
    fn stamp_legacy_bjt_companion(
        current_branch: &BjtChargeBranch,
        d_internal: &[Value; BJT_INTERNAL_STATE_DIM],
        d_external: &[Value; BJT_EXTERNAL_STATE_DIM],
        i_eq: Value,
        system: BjtCompanionSystem<'_>,
    ) {
        let BjtCompanionSystem {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        } = system;
        for (row_sign, row_internal, row_external) in [
            (
                1.0,
                current_branch.pos_internal,
                current_branch.pos_external,
            ),
            (
                -1.0,
                current_branch.neg_internal,
                current_branch.neg_external,
            ),
        ] {
            if let Some(row) = row_internal {
                // Hidden legacy rows are branch-balance equations; their current
                // orientation is opposite the external terminal KCL orientation.
                let row_sign = -row_sign;
                for col in 0..BJT_INTERNAL_STATE_DIM {
                    g_ii[row][col] += row_sign * d_internal[col];
                }
                for col in 0..BJT_EXTERNAL_STATE_DIM {
                    g_ie[row][col] += row_sign * d_external[col];
                }
                z_i[row] += row_sign * i_eq;
            }
            if let Some(row) = row_external {
                for col in 0..BJT_INTERNAL_STATE_DIM {
                    g_ei[row][col] += row_sign * d_internal[col];
                }
                for col in 0..BJT_EXTERNAL_STATE_DIM {
                    g_ee[row][col] += row_sign * d_external[col];
                }
                z_e[row] += row_sign * i_eq;
            }
        }
    }

    #[inline]
    pub(in crate::engine::transient) fn solve_bjt_static_core_from_linearization(
        linearization: &BjtTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let mut g_static = [[0.0; BJT_STATIC_CORE_STATE_DIM]; BJT_STATIC_CORE_STATE_DIM];
        let mut rhs_static = [0.0; BJT_STATIC_CORE_STATE_DIM];
        for (row, ((rhs, g_static_row), &charge)) in rhs_static
            .iter_mut()
            .zip(g_static.iter_mut())
            .zip(&linearization.z_i)
            .enumerate()
        {
            *rhs = charge;
            for (conductance, voltage) in linearization.g_ie[row].iter().zip(external_voltages) {
                *rhs -= conductance * voltage;
            }
            for (conductance, voltage) in linearization.g_ii[row]
                [BJT_STATIC_CORE_STATE_DIM..BJT_INTERNAL_STATE_DIM]
                .iter()
                .zip(&internal_voltages[BJT_STATIC_CORE_STATE_DIM..BJT_INTERNAL_STATE_DIM])
            {
                *rhs -= conductance * voltage;
            }
            g_static_row.copy_from_slice(&linearization.g_ii[row][..BJT_STATIC_CORE_STATE_DIM]);
        }
        let solved_static =
            crate::numerics::solve_small_dense(&g_static, &rhs_static, BJT_STATIC_CORE_STATE_DIM)?;
        let mut solved_internal = *internal_voltages;
        solved_internal[..BJT_STATIC_CORE_STATE_DIM].copy_from_slice(&solved_static);
        Some(solved_internal)
    }

    #[inline]
    pub(in crate::engine::transient) fn bjt_internal_equation_residual(
        linearization: &BjtTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut residual = [0.0; BJT_INTERNAL_STATE_DIM];
        for (((entry, &charge), internal_row), external_row) in residual
            .iter_mut()
            .zip(&linearization.z_i)
            .zip(&linearization.g_ii)
            .zip(&linearization.g_ie)
        {
            *entry = -charge;
            for (conductance, voltage) in internal_row.iter().zip(internal_voltages) {
                *entry += conductance * voltage;
            }
            for (conductance, voltage) in external_row.iter().zip(external_voltages) {
                *entry += conductance * voltage;
            }
        }
        residual
    }

    #[inline]
    pub(in crate::engine::transient) fn bjt_static_core_residual_norm(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        crate::numerics::infinity_norm(&residual[..BJT_STATIC_CORE_STATE_DIM])
    }

    #[inline]
    pub(in crate::engine::transient) fn reduce_bjt_transient_external_system(
        linearization: &BjtTransientLinearization,
    ) -> Option<(
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    )> {
        // Four terminal derivatives and the history source share one factorization.
        let mut rhs = [[0.0; BJT_EXTERNAL_STATE_DIM + 1]; BJT_INTERNAL_STATE_DIM];
        for (row, entries) in rhs.iter_mut().enumerate() {
            for (col, entry) in entries[..BJT_EXTERNAL_STATE_DIM].iter_mut().enumerate() {
                *entry = -linearization.g_ie[row][col];
            }
            entries[BJT_EXTERNAL_STATE_DIM] = linearization.z_i[row];
        }
        let solutions = crate::numerics::solve_small_dense_many(
            &linearization.g_ii,
            &rhs,
            BJT_INTERNAL_STATE_DIM,
        )?;
        let mut y_total = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            for ((total_row, external_row), internal_row) in y_total
                .iter_mut()
                .zip(&linearization.g_ee)
                .zip(&linearization.g_ei)
            {
                let mut value = external_row[col];
                for (conductance, unknown) in internal_row.iter().zip(&solutions) {
                    value += conductance * unknown[col];
                }
                if !value.is_finite() {
                    return None;
                }
                total_row[col] = value;
            }
        }

        let mut reduced_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
        for ((entry, &charge), internal_row) in reduced_i_eq
            .iter_mut()
            .zip(&linearization.z_e)
            .zip(&linearization.g_ei)
        {
            *entry = charge;
            for (conductance, unknown) in internal_row.iter().zip(&solutions) {
                *entry -= conductance * unknown[BJT_EXTERNAL_STATE_DIM];
            }
            if !entry.is_finite() {
                return None;
            }
        }

        Some((y_total, reduced_i_eq))
    }

    /// Evaluate the four accepted terminal currents from the exact reduced
    /// transient companion that owns the Newton stamp. This is deliberately
    /// not a finite difference of output samples: `Y*v - i_eq` contains the
    /// integration method's accepted `dQ/dt` history and the static `F`
    /// contribution at the same point.
    pub(in crate::engine::transient) fn reduced_bjt_transient_terminal_currents(
        bjt: &crate::device::Bjt,
        snapshot: &crate::device::semiconductor::BjtChargeSnapshot,
        step: BjtChargeStep<'_>,
    ) -> Result<[Value; BJT_EXTERNAL_STATE_DIM], SimulationError> {
        let BjtChargeStep {
            coeff,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        } = step;
        if !snapshot.branches.iter().any(BjtChargeBranch::is_active) {
            return Ok(bjt.operating_point_terminal_currents());
        }
        let linearization = Self::assemble_legacy_bjt_transient_linearization(
            bjt,
            snapshot,
            BjtChargeStep {
                coeff,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            },
        )
        .ok_or_else(|| {
            SimulationError::Circuit(format!(
                "BJT '{}' transient lead-current companion could not be assembled",
                bjt.name
            ))
        })?;
        let (admittance, source) = Self::reduce_bjt_transient_external_system(&linearization)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "BJT '{}' transient lead-current companion could not be reduced",
                    bjt.name
                ))
            })?;
        let external = snapshot.reduction.external_voltages;
        Ok(std::array::from_fn(|row| {
            admittance[row]
                .iter()
                .zip(external.iter())
                .map(|(coefficient, voltage)| coefficient * voltage)
                .sum::<Value>()
                - source[row]
        }))
    }

    #[inline]
    pub(in crate::engine::transient) fn bjt_static_stamped_external_system(
        bjt: &crate::device::Bjt,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> (
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        bjt.stamped_reduced_external_system(external[0], external[1], external[2], external[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promoted_vbic_thermal_decay_matches_backward_euler_across_scales() {
        for level in [4, 9, 11, 12, 13] {
            let substrate = if level == 11 { "" } else { " 0" };
            for m in [1.0, 1e-20, 1e-200, 1e200] {
                for rise in [30.0, 1e-14] {
                    let netlist = crate::Netlist::parse(&format!(
                        "VBIC thermal decay\nQ1 0 0 0{substrate} th vm M={m} SW_ET=0\n.model vm NPN(LEVEL={level} RTH=1000 CTH=1p SELFT=1 IS=0 IBEI=0 IBCI=0 RCI=0 RBI=0 RBP=0)\n.ic V(th)={rise}\n.end\n"
                    )).unwrap();
                    let mut config = crate::engine::SimulationConfig {
                        integration_method: IntegrationMethod::BackwardEuler,
                        ..Default::default()
                    };
                    config.convergence_config.gmin_target = 0.0;
                    config.convergence_config.junction_gmin_target = 0.0;
                    config.convergence_config.voltage_abstol = rise * 1e-10;
                    config.convergence_config.voltage_reltol = 1e-9;
                    config.convergence_config.current_abstol = rise * m * 1e-13;
                    config.convergence_config.residual_reltol = 1e-9;
                    config.transient_nonlinear_abstol = Some(rise * 1e-10);
                    config.transient_nonlinear_reltol = Some(1e-9);
                    config.transient_nonlinear_rhstol = Some(rise * m * 1e-13);
                    let engine = Engine::new(config);
                    let circuit = engine.build_circuit(&netlist).unwrap();
                    assert!(circuit.bjts.devices[0].mna_promoted());
                    let result = engine
                        .run_tran_with_startup_mode(
                            &netlist,
                            2e-9,
                            1e-10,
                            TransientStartupMode::Uic,
                        )
                        .unwrap();
                    let values = result.try_voltage_waveform_named("th").unwrap();
                    assert_eq!(result.time[0], 0.0);
                    assert_eq!(*result.time.last().unwrap(), 2e-9);
                    assert!((values[0] - rise).abs() <= rise * 1e-10);
                    let mut expected = rise;
                    for (times, &actual) in result.time.windows(2).zip(&values[1..]) {
                        // The physical RC is 1 ns for every M. Check the
                        // backward-Euler law on each actual accepted step.
                        expected /= 1.0 + (times[1] - times[0]) / 1e-9;
                        assert!(
                            (actual - expected).abs() <= rise * 2e-8,
                            "LEVEL={level} M={m:e} rise={rise:e} t={:e}: {actual:e} vs {expected:e}",
                            times[1],
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn private_bjt_residual_norm_includes_invalid_equations() {
        let mut linearization = BjtTransientLinearization {
            g_ii: [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
            g_ie: [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
            g_ei: [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
            g_ee: [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
            z_i: [0.0; BJT_INTERNAL_STATE_DIM],
            z_e: [0.0; BJT_EXTERNAL_STATE_DIM],
        };
        let external = [0.0; BJT_EXTERNAL_STATE_DIM];
        let internal = [0.0; BJT_INTERNAL_STATE_DIM];
        for lane in 0..BJT_INTERNAL_STATE_DIM {
            linearization.z_i = [0.0; BJT_INTERNAL_STATE_DIM];
            linearization.z_i[lane] = Value::NAN;
            let residual =
                Engine::bjt_internal_equation_residual(&linearization, &external, &internal);
            assert_eq!(
                Engine::bjt_static_core_residual_norm(&residual),
                if lane < BJT_STATIC_CORE_STATE_DIM {
                    Value::INFINITY
                } else {
                    0.0
                }
            );
        }
    }

    #[test]
    fn legacy_companion_preserves_history_at_nonpositive_charge_slopes() {
        for (polarity, resistance) in [(1.0, 0.0), (-1.0, 0.0), (1.0, 100.0), (-1.0, 100.0)] {
            let params = [
                ("LEVEL", 1.0),
                ("TF", 1e-9),
                ("TR", 1e-9),
                ("VAR", 0.72),
                ("RB", resistance),
                ("RBM", 0.0),
            ]
            .map(|(name, value)| (name.to_owned(), value))
            .into_iter()
            .collect();
            let mut bjt = if polarity > 0.0 {
                crate::device::Bjt::new_npn("q".into(), 1, 2, 3)
            } else {
                crate::device::Bjt::new_pnp("q".into(), 1, 2, 3)
            }
            .with_params(&params);
            bjt.set_junction_gmin(0.0);
            for bias in [0.71, -1e100] {
                let vb = polarity * bias;
                let vc = if bias > 0.0 { vb } else { 0.0 };
                let mut internal = [0.0; BJT_INTERNAL_STATE_DIM];
                internal[BJT_VCX_STATE_INDEX] = vc;
                internal[BJT_VCI_STATE_INDEX] = vc;
                internal[BJT_VBX_STATE_INDEX] = vb;
                internal[BJT_VBI_STATE_INDEX] = vb;
                let snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, 0.0, 0.0, internal);
                let q = snapshot.branches.map(|branch| branch.charge);
                let mut previous = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
                let mut previous_previous = previous;
                let mut current_previous = previous;
                for index in [BJT_QBE_BRANCH_INDEX, BJT_QBC_BRANCH_INDEX] {
                    previous[index] = polarity * 4e-12;
                    previous_previous[index] = polarity * 3e-12;
                    current_previous[index] = polarity * 2e-3;
                }
                let dt = 1e-9;
                for (coeff, factors) in [
                    (
                        CompanionCoefficients::backward_euler(),
                        [1.0, -1.0, 0.0, 0.0],
                    ),
                    (CompanionCoefficients::trapezoidal(), [2.0, -2.0, 0.0, -1.0]),
                    (CompanionCoefficients::gear2(), [1.5, -2.0, 0.5, 0.0]),
                ] {
                    let linearization = Engine::assemble_legacy_bjt_transient_linearization(
                        &bjt,
                        &snapshot,
                        BjtChargeStep {
                            coeff: &coeff,
                            dt,
                            q_prev: &previous,
                            q_prev_prev: &previous_previous,
                            cq_prev: &current_previous,
                        },
                    )
                    .expect("stored charge must retain its companion");
                    let integrated_current = |index| {
                        (factors[0] * q[index]
                            + factors[1] * previous[index]
                            + factors[2] * previous_previous[index])
                            / dt
                            + factors[3] * current_previous[index]
                    };
                    let ibe = integrated_current(BJT_QBE_BRANCH_INDEX);
                    let ibc = integrated_current(BJT_QBC_BRANCH_INDEX);
                    let base_terminal_current = if resistance > 0.0 { 0.0 } else { ibe + ibc };
                    for (row, expected) in [-ibc, base_terminal_current, -ibe, 0.0]
                        .into_iter()
                        .enumerate()
                    {
                        let base = &snapshot.reduction;
                        let actual = (0..BJT_INTERNAL_STATE_DIM)
                            .map(|col| {
                                (linearization.g_ei[row][col] - base.g_ei[row][col]) * internal[col]
                            })
                            .sum::<Value>()
                            + (0..BJT_EXTERNAL_STATE_DIM)
                                .map(|col| {
                                    (linearization.g_ee[row][col] - base.g_ee[row][col])
                                        * base.external_voltages[col]
                                })
                                .sum::<Value>()
                            - (linearization.z_e[row] - base.z_e_static[row]);
                        assert!(
                            (actual - expected).abs() < 1e-12,
                            "polarity={polarity}, bias={bias}, coeff={coeff:?}, row={row}: {actual} != {expected}"
                        );
                    }
                    if resistance > 0.0 {
                        let row = BJT_VBI_STATE_INDEX;
                        let base = &snapshot.reduction;
                        let actual = (0..BJT_INTERNAL_STATE_DIM)
                            .map(|col| {
                                (linearization.g_ii[row][col] - base.g_ii[row][col]) * internal[col]
                            })
                            .sum::<Value>()
                            + (0..BJT_EXTERNAL_STATE_DIM)
                                .map(|col| {
                                    (linearization.g_ie[row][col] - base.g_ie[row][col])
                                        * base.external_voltages[col]
                                })
                                .sum::<Value>()
                            - (linearization.z_i[row] - base.z_i_static[row]);
                        // The private base equation balances current entering
                        // the node; external MNA rows balance current leaving it.
                        assert!(
                            (actual + ibe + ibc).abs() < 1e-12,
                            "private base current has wrong orientation: {actual} versus {}",
                            -ibe - ibc
                        );
                    }
                }
            }
        }
    }
}
