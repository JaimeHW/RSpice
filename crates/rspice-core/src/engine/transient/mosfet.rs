//! Shared serial accepted classic-MOS integration history.
use super::state::MosfetCompanionBiasSource;
use super::*;

pub(super) struct MosfetHistoryStep<'a> {
    pub solution: &'a [Value],
    pub coeff: &'a CompanionCoefficients,
    pub dt: Value,
    pub suppress_gate_charge: bool,
    pub capacitances: Option<&'a [(Value, Value, Value)]>,
    pub charges: Option<&'a [MosfetGateCompanionCharges]>,
}

impl Engine {
    pub(in crate::engine) fn stamp_mosfet_shooting_companions(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &MosfetTransientHistory,
        physical_probe: bool,
    ) {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        for (index, mos) in circuit.mosfets.devices.iter().enumerate() {
            let bias = if physical_probe {
                mos.unlimited_branch_voltages_at(voltages)
            } else {
                mos.eval_branch_voltages_at(voltages)
            };
            if let Some(charge) = mos.legacy_gate_charge_at(bias.0, bias.1, bias.2) {
                let charges = Self::integrate_mosfet_gate_charges(
                    charge.charges,
                    coeff,
                    dt,
                    [
                        BranchChargeHistory {
                            q_prev: history.qgs_prev[index],
                            q_prev_prev: history.qgs_prev_prev[index],
                            cq_prev: history.cqgs_prev[index],
                        },
                        BranchChargeHistory {
                            q_prev: history.qgd_prev[index],
                            q_prev_prev: history.qgd_prev_prev[index],
                            cq_prev: history.cqgd_prev[index],
                        },
                        BranchChargeHistory {
                            q_prev: history.qgb_prev[index],
                            q_prev_prev: history.qgb_prev_prev[index],
                            cq_prev: history.cqgb_prev[index],
                        },
                    ],
                );
                mos.stamp_legacy_gate_charge(
                    &charge,
                    Self::jfet_companion_geq(coeff, 1.0, dt),
                    charges.map(|(_, cq)| cq),
                    [bias.0, bias.1, bias.2],
                    &mut StaticMatrixChargeStamper { matrix, rhs },
                );
            }
            let (terms, _, _) = Self::mosfet_companion_branch_terms::<false>(
                mos,
                index,
                voltages,
                coeff,
                dt,
                history,
                false,
                if physical_probe {
                    MosfetCompanionBiasSource::Physical
                } else {
                    MosfetCompanionBiasSource::Solution
                },
                None,
            );
            let ports = [
                (mos.node_gate, mos.node_source),
                (mos.node_gate, mos.node_drain),
                (mos.node_gate, mos.node_bulk),
                mos.body_source_charge_nodes(),
                mos.body_drain_charge_nodes(),
            ];
            for ((pos, neg), (geq, ieq)) in ports.into_iter().zip(terms) {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &TwoTerminalStampSlots::link(matrix, pos, neg),
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    pub(super) fn accept_mosfet_histories_after_rotation(
        devices: &[crate::device::Mosfet],
        mosfet_history: &mut MosfetTransientHistory,
        step: MosfetHistoryStep<'_>,
    ) {
        let MosfetHistoryStep {
            solution: accepted_solution,
            coeff,
            dt,
            suppress_gate_charge: suppress_gate_charge_history,
            capacitances: mosfet_caps,
            charges: mosfet_gate_companion_charges,
        } = step;
        for (idx, mos) in devices.iter().enumerate() {
            let displacement = &mut mosfet_history.accepted_displacement_currents[idx];
            *displacement = [0.0; 5];
            let (_, raw_vds, raw_vbs) = mos.unlimited_branch_voltages_at(accepted_solution);
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(accepted_solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            // The truncation walk already evaluated the Meyer halves on this
            // accepted solution; reuse them when the caller captured them.
            let (cgs_half, cgd_half, cgb_half) = match mosfet_caps {
                Some(cache) => cache[idx],
                None => mos.transient_capacitance_halves_at(vgs, vds, vbs),
            };
            let previous_cap_halves = (
                mosfet_history.capgs_prev_half[idx],
                mosfet_history.capgd_prev_half[idx],
                mosfet_history.capgb_prev_half[idx],
            );
            mosfet_history.vgs_prev[idx] = vgs;
            mosfet_history.capgs_prev_half[idx] = cgs_half;
            mosfet_history.vgd_prev[idx] = vgd;
            mosfet_history.capgd_prev_half[idx] = cgd_half;
            mosfet_history.vgb_prev[idx] = vgb;
            mosfet_history.capgb_prev_half[idx] = cgb_half;
            if !suppress_gate_charge_history {
                let exact_charges = mos.legacy_gate_charge_at(vgs, vds, vbs).map(|charge| {
                    Self::integrate_mosfet_gate_charges(
                        charge.charges,
                        coeff,
                        dt,
                        [
                            BranchChargeHistory {
                                q_prev: mosfet_history.qgs_prev_prev[idx],
                                q_prev_prev: mosfet_history.qgs_prev_prev_prev[idx],
                                cq_prev: mosfet_history.cqgs_prev[idx],
                            },
                            BranchChargeHistory {
                                q_prev: mosfet_history.qgd_prev_prev[idx],
                                q_prev_prev: mosfet_history.qgd_prev_prev_prev[idx],
                                cq_prev: mosfet_history.cqgd_prev[idx],
                            },
                            BranchChargeHistory {
                                q_prev: mosfet_history.qgb_prev_prev[idx],
                                q_prev_prev: mosfet_history.qgb_prev_prev_prev[idx],
                                cq_prev: mosfet_history.cqgb_prev[idx],
                            },
                        ],
                    )
                });
                if let Some(charges) = exact_charges
                    .as_ref()
                    .or_else(|| mosfet_gate_companion_charges.map(|charges| &charges[idx]))
                {
                    Self::install_cached_mosfet_gate_companion_charges(
                        charges,
                        &mut mosfet_history.qgs_prev[idx],
                        &mut mosfet_history.cqgs_prev[idx],
                        &mut mosfet_history.qgd_prev[idx],
                        &mut mosfet_history.cqgd_prev[idx],
                        &mut mosfet_history.qgb_prev[idx],
                        &mut mosfet_history.cqgb_prev[idx],
                    );
                } else {
                    let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
                    let cgs = cgs_half + previous_cap_halves.0 + cgs_ov;
                    let cgd = cgd_half + previous_cap_halves.1 + cgd_ov;
                    let cgb = cgb_half + previous_cap_halves.2 + cgb_ov;
                    let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs,
                        mosfet_history.vgs_prev_prev[idx],
                        BranchChargeHistory {
                            q_prev: mosfet_history.qgs_prev_prev[idx],
                            q_prev_prev: mosfet_history.qgs_prev_prev_prev[idx],
                            cq_prev: mosfet_history.cqgs_prev[idx],
                        },
                    );
                    mosfet_history.qgs_prev[idx] = qgs_curr;
                    mosfet_history.cqgs_prev[idx] = cqgs_curr;

                    let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd,
                        mosfet_history.vgd_prev_prev[idx],
                        BranchChargeHistory {
                            q_prev: mosfet_history.qgd_prev_prev[idx],
                            q_prev_prev: mosfet_history.qgd_prev_prev_prev[idx],
                            cq_prev: mosfet_history.cqgd_prev[idx],
                        },
                    );
                    mosfet_history.qgd_prev[idx] = qgd_curr;
                    mosfet_history.cqgd_prev[idx] = cqgd_curr;

                    let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgb,
                        vgb,
                        mosfet_history.vgb_prev_prev[idx],
                        BranchChargeHistory {
                            q_prev: mosfet_history.qgb_prev_prev[idx],
                            q_prev_prev: mosfet_history.qgb_prev_prev_prev[idx],
                            cq_prev: mosfet_history.cqgb_prev[idx],
                        },
                    );
                    mosfet_history.qgb_prev[idx] = qgb_curr;
                    mosfet_history.cqgb_prev[idx] = cqgb_curr;
                }
            }

            if !suppress_gate_charge_history {
                displacement[..3].copy_from_slice(&Self::accepted_mosfet_gate_currents(
                    mos,
                    accepted_solution,
                    coeff,
                    dt,
                    [(cgs_half, cgd_half, cgb_half), previous_cap_halves],
                    [
                        mosfet_history.cqgs_prev[idx],
                        mosfet_history.cqgd_prev[idx],
                        mosfet_history.cqgb_prev[idx],
                    ],
                ));
            }

            let body_charge_mask = mos.body_junction_charge_mask();
            if body_charge_mask & 1 != 0 {
                let vbs_j = mos.body_source_charge_branch_voltage(vbs);
                let (qbs_exact, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs);
                let (geq_bs, _ieq_bs, qbs_curr, cqbs_curr) = nonlinear_charge_companion_terms(
                    coeff,
                    dt,
                    cbs,
                    vbs_j,
                    qbs_exact,
                    BranchChargeHistory {
                        q_prev: mosfet_history.qbs_prev[idx],
                        q_prev_prev: mosfet_history.qbs_prev_prev[idx],
                        cq_prev: mosfet_history.cqbs_prev[idx],
                    },
                );
                mosfet_history.vbs_j_prev_prev[idx] = mosfet_history.vbs_j_prev[idx];
                mosfet_history.vbs_j_prev[idx] = vbs_j;
                mosfet_history.qbs_prev_prev[idx] = mosfet_history.qbs_prev[idx];
                mosfet_history.qbs_prev[idx] = qbs_curr;
                mosfet_history.cqbs_prev[idx] = cqbs_curr;
                displacement[3] =
                    cqbs_curr + geq_bs * (mos.body_source_charge_branch_voltage(raw_vbs) - vbs_j);
            }

            if body_charge_mask & 2 != 0 {
                let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
                let (qbd_exact, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
                let (geq_bd, _ieq_bd, qbd_curr, cqbd_curr) = nonlinear_charge_companion_terms(
                    coeff,
                    dt,
                    cbd,
                    vbd_j,
                    qbd_exact,
                    BranchChargeHistory {
                        q_prev: mosfet_history.qbd_prev[idx],
                        q_prev_prev: mosfet_history.qbd_prev_prev[idx],
                        cq_prev: mosfet_history.cqbd_prev[idx],
                    },
                );
                mosfet_history.vbd_j_prev_prev[idx] = mosfet_history.vbd_j_prev[idx];
                mosfet_history.vbd_j_prev[idx] = vbd_j;
                mosfet_history.qbd_prev_prev[idx] = mosfet_history.qbd_prev[idx];
                mosfet_history.qbd_prev[idx] = qbd_curr;
                mosfet_history.cqbd_prev[idx] = cqbd_curr;
                displacement[4] = cqbd_curr
                    + geq_bd * (mos.body_drain_charge_branch_voltage(raw_vds, raw_vbs) - vbd_j);
            }
        }
    }

    pub(in crate::engine) fn accept_mosfet_shooting_history(
        circuit: &crate::circuit::CircuitData,
        history: &mut MosfetTransientHistory,
        solution: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
    ) {
        history.rotate_gate_generations(false);
        if !history.qbs_prev_prev_prev.is_empty() {
            history
                .qbs_prev_prev_prev
                .clone_from(&history.qbs_prev_prev);
            history
                .qbd_prev_prev_prev
                .clone_from(&history.qbd_prev_prev);
        }
        Self::accept_mosfet_histories_after_rotation(
            &circuit.mosfets.devices,
            history,
            MosfetHistoryStep {
                solution,
                coeff,
                dt,
                suppress_gate_charge: false,
                capacitances: None,
                charges: None,
            },
        );
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }
}
