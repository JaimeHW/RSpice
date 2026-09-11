//! Accepted-step reactive-history commit logic.

use super::*;

/// The accepted step whose reactive history is being committed: the solution
/// that was accepted, the time it lands at, the step size, and the companion
/// coefficients the devices stamped with (BSIM4's non-quasi-static branch uses
/// its own set).
#[derive(Clone, Copy)]
pub(super) struct AcceptedReactiveStep<'a> {
    pub accepted_solution: &'a [Value],
    pub accepted_time: Value,
    pub dt: Value,
    pub coeff: &'a CompanionCoefficients,
    pub bsim4_trnqs_coeff: &'a CompanionCoefficients,
}

/// The optional per-family state the commit reseeds from when the step
/// produced it: the Xyce second-order flag it was taken under, the VBIC charge
/// snapshots, the accepted capacitor and MOSFET capacitance states, the gate
/// companion charges and whether to suppress them, and the transmission-line
/// reference states.
#[derive(Clone, Copy)]
pub(super) struct AcceptedReactiveSnapshots<'a> {
    pub xyce_one_step_order2: bool,
    pub vbic_snapshots: Option<&'a [Option<BjtChargeSnapshot>]>,
    pub capacitor_accepted_states: Option<&'a [CapacitorAcceptedState]>,
    pub mosfet_caps: Option<&'a [(Value, Value, Value)]>,
    pub mosfet_gate_companion_charges: Option<&'a [MosfetGateCompanionCharges]>,
    pub suppress_gate_charge_history: bool,
    pub tline_dc_refs: &'a [(Value, Value)],
    pub coupled_tline_refs: &'a [CoupledTlineReferenceState],
}

/// The breakpoint schedule the commit may extend, with the analysis stop time
/// and the tolerances a new breakpoint is judged against.
pub(super) struct ReactiveBreakpointScheduling<'a> {
    pub breakpoints: &'a mut BreakpointManager,
    pub tstop: Value,
    pub voltage_reltol: Value,
    pub voltage_abstol: Value,
    pub current_abstol: Value,
}

/// Newly evaluated values only; older history levels stay in their SoA arrays
/// until every BJT has reconstructed a valid candidate.
struct AcceptedBjtValues {
    charges: [Value; BJT_DYNAMIC_CHARGE_COUNT],
    currents: [Value; BJT_DYNAMIC_CHARGE_COUNT],
    internal: [Value; BJT_INTERNAL_STATE_DIM],
    linear: Option<BjtPredictorLinearBranchState>,
    voltages: [Value; 3],
    lead_currents: Option<[Value; BJT_EXTERNAL_STATE_DIM]>,
}

#[must_use]
pub(super) struct PreparedBjtHistory {
    values: Vec<AcceptedBjtValues>,
    dt: Value,
}

/// Fallible native work evaluated before any accepted history is rotated.
#[must_use]
pub(super) struct PreparedReactiveHistory<'engine> {
    bjt: PreparedBjtHistory,
    behavioral: crate::device::behavioral::PreparedBehavioralStep,
    #[cfg(feature = "parallel")]
    mos_workers: Option<usize>,
    #[cfg(feature = "parallel")]
    mos_pool: Option<&'engine rayon::ThreadPool>,
    engine: std::marker::PhantomData<&'engine Engine>,
}

impl Engine {
    /// Commit all accepted JFET charge and trap histories. Trial evaluations
    /// borrow these histories; transient and shooting share the same commit.
    pub(in crate::engine) fn accept_jfet_history(
        circuit: &crate::circuit::CircuitData,
        jfet_history: &mut JfetTransientHistory,
        accepted_solution: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        suppress_gate_charge_history: bool,
    ) {
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, accepted_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, accepted_solution);
            let (vgstrap, vgdtrap, power) = jfet.jfet2_next_transient_memory(
                vgs_eval,
                vgd_eval,
                jfet_history.jfet2_vgstrap_prev[idx],
                jfet_history.jfet2_vgdtrap_prev[idx],
                jfet_history.jfet2_power_prev[idx],
                dt,
            );
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                Some((
                    jfet_history.vgs_prev[idx],
                    jfet_history.vgd_prev[idx],
                    jfet_history.qgs_prev[idx],
                    jfet_history.qgd_prev[idx],
                )),
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            jfet_history.jfet2_vgstrap_prev[idx] = vgstrap;
            jfet_history.jfet2_vgdtrap_prev[idx] = vgdtrap;
            jfet_history.jfet2_power_prev[idx] = power;
            jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
            jfet_history.vgs_prev[idx] = vgs_charge;
            jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
            jfet_history.vgd_prev[idx] = vgd_charge;
            jfet_history.vds_prev_prev[idx] = jfet_history.vds_prev[idx];
            jfet_history.vds_prev[idx] = vds_charge;
            jfet_history.accepted_cqgs[idx] = 0.0;
            jfet_history.accepted_cqgd[idx] = 0.0;
            jfet_history.accepted_cqds[idx] = 0.0;
            if !suppress_gate_charge_history {
                let (geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = if let Some(charge) = jfet2_charge {
                    nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        charge.qgs,
                        BranchChargeHistory {
                            q_prev: jfet_history.qgs_prev[idx],
                            q_prev_prev: jfet_history.qgs_prev_prev[idx],
                            cq_prev: jfet_history.cqgs_prev[idx],
                        },
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        jfet_history.vgs_prev_prev[idx],
                        BranchChargeHistory {
                            q_prev: jfet_history.qgs_prev[idx],
                            q_prev_prev: jfet_history.qgs_prev_prev[idx],
                            cq_prev: jfet_history.cqgs_prev[idx],
                        },
                    )
                };
                jfet_history.qgs_prev_prev_prev[idx] = jfet_history.qgs_prev_prev[idx];
                jfet_history.qgs_prev_prev[idx] = jfet_history.qgs_prev[idx];
                jfet_history.qgs_prev[idx] = qgs_curr;
                jfet_history.cqgs_prev[idx] = cqgs_curr;
                if cgs.is_finite() && cgs > 0.0 {
                    let raw = Self::differential_voltage(accepted_solution, jfet.gate, jfet.source);
                    jfet_history.accepted_cqgs[idx] = cqgs_curr + geq_gs * (raw - vgs_charge);
                }

                let (geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = if let Some(charge) = jfet2_charge {
                    nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        charge.qgd,
                        BranchChargeHistory {
                            q_prev: jfet_history.qgd_prev[idx],
                            q_prev_prev: jfet_history.qgd_prev_prev[idx],
                            cq_prev: jfet_history.cqgd_prev[idx],
                        },
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        jfet_history.vgd_prev_prev[idx],
                        BranchChargeHistory {
                            q_prev: jfet_history.qgd_prev[idx],
                            q_prev_prev: jfet_history.qgd_prev_prev[idx],
                            cq_prev: jfet_history.cqgd_prev[idx],
                        },
                    )
                };
                jfet_history.qgd_prev_prev_prev[idx] = jfet_history.qgd_prev_prev[idx];
                jfet_history.qgd_prev_prev[idx] = jfet_history.qgd_prev[idx];
                jfet_history.qgd_prev[idx] = qgd_curr;
                jfet_history.cqgd_prev[idx] = cqgd_curr;
                if cgd.is_finite() && cgd > 0.0 {
                    let raw = Self::differential_voltage(accepted_solution, jfet.gate, jfet.drain);
                    jfet_history.accepted_cqgd[idx] = cqgd_curr + geq_gd * (raw - vgd_charge);
                }
            }
            if cds.is_finite() && cds > 0.0 {
                let (geq_ds, _ieq_ds, qds_curr, cqds_curr) = Self::jfet_companion_terms(
                    coeff,
                    dt,
                    cds,
                    vds_charge,
                    jfet_history.vds_prev_prev[idx],
                    BranchChargeHistory {
                        q_prev: jfet_history.qds_prev[idx],
                        q_prev_prev: jfet_history.qds_prev_prev[idx],
                        cq_prev: jfet_history.cqds_prev[idx],
                    },
                );
                jfet_history.qds_prev_prev_prev[idx] = jfet_history.qds_prev_prev[idx];
                jfet_history.qds_prev_prev[idx] = jfet_history.qds_prev[idx];
                jfet_history.qds_prev[idx] = qds_curr;
                jfet_history.cqds_prev[idx] = cqds_curr;
                let raw = Self::differential_voltage(accepted_solution, jfet.drain, jfet.source);
                jfet_history.accepted_cqds[idx] = cqds_curr + geq_ds * (raw - vds_charge);
            }
        }
        jfet_history.accepted_dt_prev_prev = jfet_history.accepted_dt_prev;
        jfet_history.accepted_dt_prev = dt;
    }

    /// Prepare the complete BJT family before rotating any accepted history.
    /// Shared by ordinary transient integration and periodic traversals.
    pub(in crate::engine) fn accept_bjt_history(
        circuit: &crate::circuit::CircuitData,
        history: &mut BjtTransientHistory,
        accepted_solution: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
    ) -> Result<(), SimulationError> {
        let prepared = Self::prepare_bjt_history(
            circuit,
            history,
            accepted_solution,
            coeff,
            dt,
            vbic_snapshots,
        )?;
        Self::commit_bjt_history(history, prepared);
        Ok(())
    }

    fn prepare_bjt_history(
        circuit: &crate::circuit::CircuitData,
        history: &BjtTransientHistory,
        solution: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
    ) -> Result<PreparedBjtHistory, SimulationError> {
        let mut values = Vec::with_capacity(circuit.bjts.devices.len());
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(solution, bjt.node_collector);
            let vb = Self::node_voltage(solution, bjt.node_base);
            let ve = Self::node_voltage(solution, bjt.node_emitter);
            let vs = Self::node_voltage(solution, bjt.node_substrate);
            let external = [vc, vb, ve, vs];
            if !external.iter().all(|value| value.is_finite()) {
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' accepted terminal voltage is non-finite for dt={dt:e}",
                    bjt.name
                )));
            }
            let (charges, internal, linear, voltages, mut lead_currents) = if bjt.mna_promoted() {
                let (branches, internal, _) = bjt.mna_charge_state_at_solution(solution);
                let mut charges = branches.map(|branch| branch.charge);
                if let Some(charge) = bjt.legacy_external_bc_charge(solution) {
                    charges[BJT_QBCX_BRANCH_INDEX] = charge.charge;
                }
                let voltages = if bjt.uses_legacy_gummel_poon() {
                    bjt.mna_junction_voltages(solution)
                } else {
                    [vb - ve, vb - vc, vc - vs]
                };
                (charges, internal, None, voltages, None)
            } else {
                let cached = vbic_snapshots
                    .and_then(|cache| cache.get(idx))
                    .copied()
                    .flatten();
                let snapshot = Self::resolve_legacy_bjt_transient_snapshot(
                    bjt,
                    external,
                    BjtChargeStep {
                        coeff,
                        dt,
                        q_prev: &history.charge_q_prev[idx],
                        q_prev_prev: &history.charge_q_prev_prev[idx],
                        cq_prev: &history.charge_cq_prev[idx],
                    },
                    BjtPredictorHistory {
                        internal_prev: history.dynamic_internal_prev.get(idx),
                        linear_prev: history.dynamic_linear_prev.get(idx),
                        linear_prev_prev: history.dynamic_linear_prev_prev.get(idx),
                        previous_dt: history.accepted_dt_prev,
                    },
                    cached,
                )
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "BJT '{}' accepted private transient state did not converge for dt={dt:e}",
                        bjt.name
                    ))
                })?;
                let (vbe, vbc, vbx, vcs) =
                    Self::legacy_bjt_charge_branch_voltages_with_vbx(&snapshot);
                // Form total lead current against the old accepted Q/CQ so it
                // matches the companion that produced this electrical candidate.
                let lead_currents = Self::reduced_bjt_transient_terminal_currents(
                    bjt,
                    &snapshot,
                    BjtChargeStep {
                        coeff,
                        dt,
                        q_prev: &history.charge_q_prev[idx],
                        q_prev_prev: &history.charge_q_prev_prev[idx],
                        cq_prev: &history.charge_cq_prev[idx],
                    },
                )?;
                let legacy = bjt.legacy_transient_charge_state_with_vbx(vbe, vbc, vbx, vcs);
                let mut charges = snapshot.branches.map(|branch| branch.charge);
                charges[BJT_QBE_BRANCH_INDEX] = legacy.qbe;
                charges[BJT_QBC_BRANCH_INDEX] = legacy.qbc;
                charges[BJT_QBCX_BRANCH_INDEX] = legacy.qbx;
                if let Some(charge) = bjt.legacy_external_bc_charge(solution) {
                    charges[BJT_QBCX_BRANCH_INDEX] = charge.charge;
                }
                charges[BJT_QBCP_BRANCH_INDEX] = legacy.qcs;
                let internal = snapshot.reduction.internal_voltages;
                (
                    charges,
                    internal,
                    Some(Self::bjt_predictor_linear_branch_state(
                        bjt, external, internal,
                    )),
                    [vbe, vbc, vcs],
                    Some(lead_currents),
                )
            };
            let currents: [Value; BJT_DYNAMIC_CHARGE_COUNT] = std::array::from_fn(|branch| {
                Self::jfet_companion_ccap(
                    coeff,
                    dt,
                    charges[branch],
                    BranchChargeHistory {
                        q_prev: history.charge_q_prev[idx][branch],
                        q_prev_prev: history.charge_q_prev_prev[idx][branch],
                        cq_prev: history.charge_cq_prev[idx][branch],
                    },
                )
            });
            if bjt.uses_legacy_gummel_poon() && bjt.mna_promoted() {
                let mut terminal = bjt.mna_terminal_currents_at_solution(solution);
                let (branches, _, _) = bjt.mna_charge_state_at_solution(solution);
                for (branch, current) in branches.iter().zip(currents) {
                    if let Some(index) = branch.pos_external {
                        terminal[index] += current;
                    }
                    if let Some(index) = branch.neg_external {
                        terminal[index] -= current;
                    }
                }
                lead_currents = Some(terminal);
            }
            if !charges
                .iter()
                .chain(currents.iter())
                .chain(internal.iter())
                .chain(voltages.iter())
                .chain(lead_currents.iter().flatten())
                .all(|value| value.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' accepted transient history is non-finite for dt={dt:e}",
                    bjt.name
                )));
            }
            values.push(AcceptedBjtValues {
                charges,
                currents,
                internal,
                linear,
                voltages,
                lead_currents,
            });
        }
        Ok(PreparedBjtHistory { values, dt })
    }

    fn commit_bjt_history(history: &mut BjtTransientHistory, prepared: PreparedBjtHistory) {
        for (idx, value) in prepared.values.into_iter().enumerate() {
            history.accepted_terminal_currents[idx] = value.lead_currents;
            history.charge_q_prev_prev_prev[idx] = history.charge_q_prev_prev[idx];
            history.charge_q_prev_prev[idx] = history.charge_q_prev[idx];
            history.charge_q_prev[idx] = value.charges;
            history.charge_cq_prev[idx] = value.currents;
            history.accepted_external_bc_current[idx] = value.currents[BJT_QBCX_BRANCH_INDEX];
            history.dynamic_internal_prev_prev[idx] = history.dynamic_internal_prev[idx];
            history.dynamic_internal_prev[idx] = value.internal;
            if let Some(linear) = value.linear {
                history.dynamic_linear_prev_prev[idx] = history.dynamic_linear_prev[idx];
                history.dynamic_linear_prev[idx] = linear;
            }
            history.vbe_prev_prev[idx] = history.vbe_prev[idx];
            history.vbe_prev[idx] = value.voltages[0];
            history.vbc_prev_prev[idx] = history.vbc_prev[idx];
            history.vbc_prev[idx] = value.voltages[1];
            history.vcs_prev_prev[idx] = history.vcs_prev[idx];
            history.vcs_prev[idx] = value.voltages[2];
            let promoted = value.linear.is_none();
            history.ibe_prev[idx] = if promoted {
                0.0
            } else {
                value.currents[BJT_QBE_BRANCH_INDEX]
            };
            history.ibc_prev[idx] = if promoted {
                0.0
            } else {
                value.currents[BJT_QBC_BRANCH_INDEX]
            };
            history.ics_prev[idx] = if promoted {
                0.0
            } else {
                value.currents[BJT_QBCP_BRANCH_INDEX]
            };
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = prepared.dt;
    }

    #[inline]
    fn install_cached_mosfet_gate_companion_charges(
        charges: &MosfetGateCompanionCharges,
        qgs: &mut Value,
        cqgs: &mut Value,
        qgd: &mut Value,
        cqgd: &mut Value,
        qgb: &mut Value,
        cqgb: &mut Value,
    ) {
        (*qgs, *cqgs) = charges[0];
        (*qgd, *cqgd) = charges[1];
        (*qgb, *cqgb) = charges[2];
    }

    /// Include the companion's linear movement when its accepted charge
    /// coordinate still differs from the solved node drop after limiting.
    fn accepted_mosfet_gate_currents(
        mos: &crate::device::Mosfet,
        solution: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        [halves, previous]: [(Value, Value, Value); 2],
        mut currents: [Value; 3],
    ) -> [Value; 3] {
        let (raw_vgs, raw_vds, raw_vbs) = mos.unlimited_branch_voltages_at(solution);
        if mos.uses_legacy_bsim() {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(solution);
            let charge = mos
                .legacy_gate_charge_at(vgs, vds, vbs)
                .expect("legacy BSIM charge");
            let movement = [raw_vgs - vgs, raw_vds - vds, raw_vbs - vbs];
            let gain = Self::jfet_companion_geq(coeff, 1.0, dt);
            for (current, c) in currents.iter_mut().zip(charge.derivatives) {
                *current += gain * (c[0] * movement[0] + c[1] * movement[1] + c[2] * movement[2]);
            }
            return currents;
        }
        let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(solution);
        let movement = [
            raw_vgs - vgs,
            (raw_vgs - raw_vds) - vgd,
            (raw_vgs - raw_vbs) - vgb,
        ];
        if movement != [0.0; 3] {
            let overlap = mos.overlap_capacitances();
            let caps = [
                halves.0 + previous.0 + overlap.0,
                halves.1 + previous.1 + overlap.1,
                halves.2 + previous.2 + overlap.2,
            ];
            for branch in 0..3 {
                currents[branch] +=
                    Self::jfet_companion_geq(coeff, caps[branch], dt) * movement[branch];
            }
        }
        currents
    }

    pub(super) fn prepare_reactive_history<'engine>(
        &'engine self,
        circuit: &mut crate::circuit::CircuitData,
        step: AcceptedReactiveStep<'_>,
        histories: &TransientDeviceHistories<'_>,
        snapshots: AcceptedReactiveSnapshots<'_>,
    ) -> Result<PreparedReactiveHistory<'engine>, SimulationError> {
        #[cfg(feature = "parallel")]
        let mos_workers = self.classic_mos_parallel_worker_count(circuit.mosfets.devices.len());
        #[cfg(feature = "parallel")]
        let mos_pool = if mos_workers.is_some() {
            let instance_count = circuit.mosfets.devices.len();
            let mosfet_history = &*histories.mosfet;
            let mosfet_caps = snapshots.mosfet_caps;
            let mosfet_gate_companion_charges = snapshots
                .mosfet_gate_companion_charges
                .filter(|charges| charges.len() == instance_count);
            let history_shapes_match = [
                &mosfet_history.vgs_prev,
                &mosfet_history.vgs_prev_prev,
                &mosfet_history.capgs_prev_half,
                &mosfet_history.qgs_prev,
                &mosfet_history.qgs_prev_prev,
                &mosfet_history.qgs_prev_prev_prev,
                &mosfet_history.cqgs_prev,
                &mosfet_history.vgd_prev,
                &mosfet_history.vgd_prev_prev,
                &mosfet_history.capgd_prev_half,
                &mosfet_history.qgd_prev,
                &mosfet_history.qgd_prev_prev,
                &mosfet_history.qgd_prev_prev_prev,
                &mosfet_history.cqgd_prev,
                &mosfet_history.vgb_prev,
                &mosfet_history.vgb_prev_prev,
                &mosfet_history.capgb_prev_half,
                &mosfet_history.qgb_prev,
                &mosfet_history.qgb_prev_prev,
                &mosfet_history.qgb_prev_prev_prev,
                &mosfet_history.cqgb_prev,
                &mosfet_history.vbs_j_prev,
                &mosfet_history.vbs_j_prev_prev,
                &mosfet_history.qbs_prev,
                &mosfet_history.qbs_prev_prev,
                &mosfet_history.cqbs_prev,
                &mosfet_history.vbd_j_prev,
                &mosfet_history.vbd_j_prev_prev,
                &mosfet_history.qbd_prev,
                &mosfet_history.qbd_prev_prev,
                &mosfet_history.cqbd_prev,
            ]
            .into_iter()
            .all(|history| history.len() == instance_count);
            let caps_shape_matches =
                mosfet_caps.is_none_or(|capacitances| capacitances.len() == instance_count);
            let gate_charges_shape_matches =
                mosfet_gate_companion_charges.is_none_or(|charges| charges.len() == instance_count);
            if !history_shapes_match
                || !caps_shape_matches
                || !gate_charges_shape_matches
                || mosfet_history.accepted_displacement_currents.len() != instance_count
            {
                return Err(SimulationError::Circuit(
                    "classic-MOS transient history shape does not match the device population"
                        .to_string(),
                ));
            }
            self.prepare_classic_mos_parallel()?
        } else {
            None
        };
        let bjt = Self::prepare_bjt_history(
            circuit,
            histories.bjt,
            step.accepted_solution,
            step.coeff,
            step.dt,
            snapshots.vbic_snapshots,
        )?;
        let behavioral = circuit
            .behavioral_sources
            .prepare_transient_step(step.accepted_solution, step.accepted_time)
            .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        Ok(PreparedReactiveHistory {
            bjt,
            behavioral,
            #[cfg(feature = "parallel")]
            mos_workers,
            #[cfg(feature = "parallel")]
            mos_pool,
            engine: std::marker::PhantomData,
        })
    }

    #[cfg(test)]
    pub(super) fn update_reactive_history(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        step: AcceptedReactiveStep<'_>,
        histories: TransientDeviceHistories<'_>,
        snapshots: AcceptedReactiveSnapshots<'_>,
        scheduling: ReactiveBreakpointScheduling<'_>,
        sink: DynamicBreakpointSink<'_>,
    ) -> Result<(), SimulationError> {
        let prepared = self.prepare_reactive_history(circuit, step, &histories, snapshots)?;
        self.commit_reactive_history(
            circuit, step, histories, snapshots, scheduling, sink, prepared,
        );
        Ok(())
    }

    /// Consume preparation for this unchanged solution, integration policy and
    /// topology, on the preparing execution context. No fallible model work runs.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn commit_reactive_history(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        step: AcceptedReactiveStep<'_>,
        histories: TransientDeviceHistories<'_>,
        snapshots: AcceptedReactiveSnapshots<'_>,
        scheduling: ReactiveBreakpointScheduling<'_>,
        sink: DynamicBreakpointSink<'_>,
        prepared: PreparedReactiveHistory<'_>,
    ) {
        let DynamicBreakpointSink {
            dynamic_breakpoints_added,
            warned_dynamic_breakpoint_cap,
            pending_dynamic_breakpoints,
        } = sink;
        let ReactiveBreakpointScheduling {
            breakpoints,
            tstop,
            voltage_reltol,
            voltage_abstol,
            current_abstol,
        } = scheduling;
        let AcceptedReactiveSnapshots {
            xyce_one_step_order2,
            vbic_snapshots: _,
            capacitor_accepted_states,
            mosfet_caps,
            mosfet_gate_companion_charges,
            suppress_gate_charge_history,
            tline_dc_refs,
            coupled_tline_refs,
        } = snapshots;
        let TransientDeviceHistories {
            bjt: bjt_history,
            jfet: jfet_history,
            diode: diode_history,
            mosfet: mosfet_history,
            vdmos: vdmos_history,
            b3soi: b3soi_history,
            bsim3: bsim3_history,
            bsim4: bsim4_history,
            ekv26: ekv26_history,
        } = histories;
        let AcceptedReactiveStep {
            accepted_solution,
            accepted_time,
            dt,
            coeff,
            bsim4_trnqs_coeff,
        } = step;
        let num_nodes = circuit.num_nodes();
        let capacitor_accepted_states = capacitor_accepted_states
            .filter(|states| states.len() == circuit.capacitors.stamps.len());
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit
                .capacitors
                .value_expressions
                .get(cap_idx)
                .and_then(Option::as_ref)
                .is_some()
            {
                continue;
            }
            let (v_new, i_new) = if let Some(states) = capacitor_accepted_states {
                let state = states[cap_idx];
                (state.voltage, state.current)
            } else {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = Self::differential_voltage(accepted_solution, np, nn);

                // An IC capacitor's MNA branch is its physical lead current
                // and is numerically authoritative. Ordinary Norton
                // companions have no branch, so reconstruct those from OLD
                // history before rotating it.
                let i_new =
                    if let Some(branch_ordinal) = circuit.capacitors.ic_branch_indices[cap_idx] {
                        accepted_solution[num_nodes + branch_ordinal - 1]
                    } else {
                        let geq = coeff.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
                        let ieq = coeff.capacitor_ieq(
                            circuit.capacitors.capacitances[cap_idx],
                            dt,
                            circuit.capacitors.v_prev[cap_idx],
                            circuit.capacitors.v_prev_prev[cap_idx],
                            circuit.capacitors.i_prev[cap_idx],
                        );
                        geq * v_new - ieq
                    };
                (v_new, i_new)
            };

            let v_old = circuit.capacitors.v_prev[cap_idx];
            circuit.capacitors.v_prev_prev_prev[cap_idx] = circuit.capacitors.v_prev_prev[cap_idx];
            circuit.capacitors.v_prev_prev[cap_idx] = v_old;
            circuit.capacitors.v_prev[cap_idx] = v_new;
            circuit.capacitors.i_prev[cap_idx] = i_new;
        }
        circuit
            .capacitors
            .update_solution_dependent_state_with_coefficients(
                accepted_solution,
                accepted_time,
                dt,
                coeff,
                num_nodes,
            );

        for l_idx in 0..circuit.inductors.names.len() {
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_new = accepted_solution[br_idx];
                circuit.inductors.i_prev_prev_prev[l_idx] = circuit.inductors.i_prev_prev[l_idx];
                circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                circuit.inductors.i_prev[l_idx] = i_new;

                let np = circuit.inductors.node_pos[l_idx];
                let nn = circuit.inductors.node_neg[l_idx];
                let v_new = Self::differential_voltage(accepted_solution, np, nn);
                circuit.inductors.v_prev[l_idx] = v_new;
            }
        }
        circuit.update_coupled_inductor_pair_state(accepted_solution);
        circuit.update_multi_winding_transformer_state(accepted_solution);
        circuit.refresh_jiles_atherton_inductances(accepted_solution);
        circuit.commit_xyce_core_inductances(accepted_solution, dt, xyce_one_step_order2);
        circuit.commit_accepted_nonlinear_state();
        circuit
            .behavioral_sources
            .commit_transient_step(prepared.behavioral);

        // Update transmission-line delayed-wave history from the accepted state.
        for (idx, tl) in circuit.tlines.iter_mut().enumerate() {
            let previous_forward = tl.launched_forward_wave();
            let previous_backward = tl.launched_backward_wave();
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
            if tl.is_memoryless_two_port() {
                continue;
            }
            if let Some((br1, br2)) = tl.txl_branch_matrix_indices() {
                let i1 = accepted_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = accepted_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.accept_txl_history(accepted_time, v1, i1, v2, i2);
                continue;
            }
            if let Some((br1, br2)) = tl.ltra_branch_matrix_indices() {
                let i1 = accepted_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = accepted_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.update_history(accepted_time, v1, i1, v2, i2);
                if !tl.is_distributed_rc()
                    && let Some(arrival) =
                        tl.ltra_derivative_breakpoint_arrival(voltage_reltol, current_abstol)
                {
                    Self::schedule_dynamic_tline_breakpoint(
                        breakpoints,
                        arrival,
                        tstop,
                        DynamicBreakpointSink {
                            dynamic_breakpoints_added,
                            warned_dynamic_breakpoint_cap,
                            pending_dynamic_breakpoints,
                        },
                    );
                }
                tl.compact_ltra_history_if_straight();
                continue;
            }
            let (_v1_ref, _v2_ref) = tline_dc_refs.get(idx).copied().unwrap_or((0.0, 0.0));
            let response = tl.transient_port_response(accepted_time);
            let (i1_actual, i2_actual) = response.port_currents(v1, v2);
            tl.update_history(accepted_time, v1, i1_actual, v2, i2_actual);
            if tl.has_distributed_rlgc() {
                if let Some(arrival) =
                    tl.ltra_derivative_breakpoint_arrival(voltage_reltol, current_abstol)
                {
                    Self::schedule_dynamic_tline_breakpoint(
                        breakpoints,
                        arrival,
                        tstop,
                        DynamicBreakpointSink {
                            dynamic_breakpoints_added,
                            warned_dynamic_breakpoint_cap,
                            pending_dynamic_breakpoints,
                        },
                    );
                }
                tl.compact_ltra_history_if_straight();
            } else {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    TlineArrivalEvent {
                        event_time: accepted_time,
                        delay: tl.delay(),
                        tstop,
                    },
                    TlineWaveChange {
                        previous_wave: previous_forward,
                        current_wave: tl.launched_forward_wave(),
                        reltol: voltage_reltol,
                        abstol: voltage_abstol,
                    },
                    DynamicBreakpointSink {
                        dynamic_breakpoints_added,
                        warned_dynamic_breakpoint_cap,
                        pending_dynamic_breakpoints,
                    },
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    TlineArrivalEvent {
                        event_time: accepted_time,
                        delay: tl.delay(),
                        tstop,
                    },
                    TlineWaveChange {
                        previous_wave: previous_backward,
                        current_wave: tl.launched_backward_wave(),
                        reltol: voltage_reltol,
                        abstol: voltage_abstol,
                    },
                    DynamicBreakpointSink {
                        dynamic_breakpoints_added,
                        warned_dynamic_breakpoint_cap,
                        pending_dynamic_breakpoints,
                    },
                );
            }
        }

        for (idx, tl) in circuit.coupled_tlines.iter_mut().enumerate() {
            // Native (ngspice-faithful) lines advance their convolution state
            // from the accepted port voltages and branch currents, then skip the
            // modal Norton bookkeeping entirely (their transient response comes
            // from the branch-current convolution stamp).
            if tl.uses_native_runtime() {
                let near_physical = Self::differential_port_voltages(
                    accepted_solution,
                    &tl.near_nodes,
                    tl.near_ref,
                );
                let far_physical =
                    Self::differential_port_voltages(accepted_solution, &tl.far_nodes, tl.far_ref);
                let conductors = tl.conductors();
                let mut near_i = vec![0.0; conductors];
                let mut far_i = vec![0.0; conductors];
                if let Some(branches) = tl.native_branch_matrix_indices() {
                    for c in 0..conductors {
                        if let Some((b1, b2)) = branches.conductor(c) {
                            near_i[c] = accepted_solution.get(b1 - 1).copied().unwrap_or(0.0);
                            far_i[c] = accepted_solution.get(b2 - 1).copied().unwrap_or(0.0);
                        }
                    }
                }
                tl.native_commit_accepted(
                    accepted_time,
                    &near_physical,
                    &far_physical,
                    &near_i,
                    &far_i,
                );
                continue;
            }

            let previous_mode_launches = tl.launched_modal_waves().collect::<Vec<_>>();
            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let near_physical =
                Self::differential_port_voltages(accepted_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(accepted_solution, &tl.far_nodes, tl.far_ref);
            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let incoming_near = tl.incoming_near_modal(accepted_time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(accepted_time, &refs.near_modal);
            let near_currents = tl.port_currents(&near_physical, &incoming_near);
            let far_currents = tl.port_currents(&far_physical, &incoming_far);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                accepted_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            for (
                (delay, previous_forward, previous_backward),
                (_, current_forward, current_backward),
            ) in previous_mode_launches
                .into_iter()
                .zip(tl.launched_modal_waves())
            {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    TlineArrivalEvent {
                        event_time: accepted_time,
                        delay,
                        tstop,
                    },
                    TlineWaveChange {
                        previous_wave: previous_forward,
                        current_wave: current_forward,
                        reltol: voltage_reltol,
                        abstol: voltage_abstol,
                    },
                    DynamicBreakpointSink {
                        dynamic_breakpoints_added,
                        warned_dynamic_breakpoint_cap,
                        pending_dynamic_breakpoints,
                    },
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    TlineArrivalEvent {
                        event_time: accepted_time,
                        delay,
                        tstop,
                    },
                    TlineWaveChange {
                        previous_wave: previous_backward,
                        current_wave: current_backward,
                        reltol: voltage_reltol,
                        abstol: voltage_abstol,
                    },
                    DynamicBreakpointSink {
                        dynamic_breakpoints_added,
                        warned_dynamic_breakpoint_cap,
                        pending_dynamic_breakpoints,
                    },
                );
            }
        }

        Self::commit_bjt_history(bjt_history, prepared.bjt);

        Self::accept_jfet_history(
            circuit,
            jfet_history,
            accepted_solution,
            coeff,
            dt,
            suppress_gate_charge_history,
        );

        for (idx, diode) in circuit.diodes.devices.iter().enumerate() {
            let vd =
                Self::differential_voltage(accepted_solution, diode.node_anode, diode.node_cathode);
            let (qd, _) = diode.junction_charge_and_capacitance(vd);
            diode_history.accept_branch(idx, vd, qd, coeff, dt);
        }
        diode_history.finish_step(dt);

        // Rotate whole accepted-state generations once, as ngspice rotates
        // CKTstate pointers, instead of copying two history levels for every
        // instance and branch. The old `prev_prev` buffers become scratch for
        // the new accepted values; arithmetic below still reads the identical
        // old `prev` and `prev_prev` generations.
        mosfet_history.rotate_gate_generations(suppress_gate_charge_history);
        // Preserve the oldest body-charge generation before serial or parallel
        // acceptance shifts the two body histories. BSIM terminal-charge LTE
        // includes these junctions in its Gear2 divided differences.
        if !mosfet_history.qbs_prev_prev_prev.is_empty() {
            mosfet_history
                .qbs_prev_prev_prev
                .clone_from(&mosfet_history.qbs_prev_prev);
            mosfet_history
                .qbd_prev_prev_prev
                .clone_from(&mosfet_history.qbd_prev_prev);
        }
        let mosfet_gate_companion_charges = mosfet_gate_companion_charges
            .filter(|charges| charges.len() == circuit.mosfets.devices.len());

        // Every instance owns one disjoint element in each history vector, so
        // the model arithmetic can run in parallel without reductions or a
        // change in floating-point operation order. Rayon MultiZip stops at
        // the shortest input; reject a broken internal shape explicitly so a
        // release build can never commit only a prefix of device history.
        #[cfg(feature = "parallel")]
        let mosfet_history_updated_in_parallel = {
            use rayon::prelude::*;

            let instance_count = circuit.mosfets.devices.len();
            if let Some(worker_count) = prepared.mos_workers {
                let chunk_size = instance_count.div_ceil(worker_count).max(1);
                let devices = circuit.mosfets.devices.as_slice();
                let vgs_prev_prev = mosfet_history.vgs_prev_prev.as_slice();
                let qgs_prev_prev = mosfet_history.qgs_prev_prev.as_slice();
                let qgs_prev_prev_prev = mosfet_history.qgs_prev_prev_prev.as_slice();
                let vgd_prev_prev = mosfet_history.vgd_prev_prev.as_slice();
                let qgd_prev_prev = mosfet_history.qgd_prev_prev.as_slice();
                let qgd_prev_prev_prev = mosfet_history.qgd_prev_prev_prev.as_slice();
                let vgb_prev_prev = mosfet_history.vgb_prev_prev.as_slice();
                let qgb_prev_prev = mosfet_history.qgb_prev_prev.as_slice();
                let qgb_prev_prev_prev = mosfet_history.qgb_prev_prev_prev.as_slice();

                let gate_outputs = (
                    mosfet_history.vgs_prev.as_mut_slice(),
                    mosfet_history.capgs_prev_half.as_mut_slice(),
                    mosfet_history.qgs_prev.as_mut_slice(),
                    mosfet_history.cqgs_prev.as_mut_slice(),
                    mosfet_history.vgd_prev.as_mut_slice(),
                    mosfet_history.capgd_prev_half.as_mut_slice(),
                    mosfet_history.qgd_prev.as_mut_slice(),
                    mosfet_history.cqgd_prev.as_mut_slice(),
                    mosfet_history.vgb_prev.as_mut_slice(),
                    mosfet_history.capgb_prev_half.as_mut_slice(),
                    mosfet_history.qgb_prev.as_mut_slice(),
                    mosfet_history.cqgb_prev.as_mut_slice(),
                )
                    .into_par_iter();
                let body_outputs = (
                    mosfet_history.vbs_j_prev.as_mut_slice(),
                    mosfet_history.vbs_j_prev_prev.as_mut_slice(),
                    mosfet_history.qbs_prev.as_mut_slice(),
                    mosfet_history.qbs_prev_prev.as_mut_slice(),
                    mosfet_history.cqbs_prev.as_mut_slice(),
                    mosfet_history.vbd_j_prev.as_mut_slice(),
                    mosfet_history.vbd_j_prev_prev.as_mut_slice(),
                    mosfet_history.qbd_prev.as_mut_slice(),
                    mosfet_history.qbd_prev_prev.as_mut_slice(),
                    mosfet_history.cqbd_prev.as_mut_slice(),
                    mosfet_history.accepted_displacement_currents.as_mut_slice(),
                )
                    .into_par_iter();

                let operation = || {
                    gate_outputs
                        .zip(body_outputs)
                        .with_min_len(chunk_size)
                        .enumerate()
                        .for_each(
                            |(
                                idx,
                                (
                                    (
                                        vgs_out,
                                        capgs_out,
                                        qgs_out,
                                        cqgs_out,
                                        vgd_out,
                                        capgd_out,
                                        qgd_out,
                                        cqgd_out,
                                        vgb_out,
                                        capgb_out,
                                        qgb_out,
                                        cqgb_out,
                                    ),
                                    (
                                        vbs_j_out,
                                        vbs_j_prev_out,
                                        qbs_out,
                                        qbs_prev_out,
                                        cqbs_out,
                                        vbd_j_out,
                                        vbd_j_prev_out,
                                        qbd_out,
                                        qbd_prev_out,
                                        cqbd_out,
                                        displacement_out,
                                    ),
                                ),
                            )| {
                                let mos = &devices[idx];
                                *displacement_out = [0.0; 5];
                                let (_, raw_vds, raw_vbs) =
                                    mos.unlimited_branch_voltages_at(accepted_solution);
                                let (vgs, vds, vbs) =
                                    mos.eval_branch_voltages_at(accepted_solution);
                                let vgd = vgs - vds;
                                let vgb = vgs - vbs;
                                let (cgs_half, cgd_half, cgb_half) = match mosfet_caps {
                                    Some(cache) => cache[idx],
                                    None => mos.transient_capacitance_halves_at(vgs, vds, vbs),
                                };
                                let previous_cap_halves = (*capgs_out, *capgd_out, *capgb_out);
                                *vgs_out = vgs;
                                *capgs_out = cgs_half;
                                *vgd_out = vgd;
                                *capgd_out = cgd_half;
                                *vgb_out = vgb;
                                *capgb_out = cgb_half;
                                if !suppress_gate_charge_history {
                                    let exact_charges =
                                        mos.legacy_gate_charge_at(vgs, vds, vbs).map(|charge| {
                                            Self::integrate_mosfet_gate_charges(
                                                charge.charges,
                                                coeff,
                                                dt,
                                                [
                                                    BranchChargeHistory {
                                                        q_prev: qgs_prev_prev[idx],
                                                        q_prev_prev: qgs_prev_prev_prev[idx],
                                                        cq_prev: *cqgs_out,
                                                    },
                                                    BranchChargeHistory {
                                                        q_prev: qgd_prev_prev[idx],
                                                        q_prev_prev: qgd_prev_prev_prev[idx],
                                                        cq_prev: *cqgd_out,
                                                    },
                                                    BranchChargeHistory {
                                                        q_prev: qgb_prev_prev[idx],
                                                        q_prev_prev: qgb_prev_prev_prev[idx],
                                                        cq_prev: *cqgb_out,
                                                    },
                                                ],
                                            )
                                        });
                                    if let Some(charges) = exact_charges.as_ref().or_else(|| {
                                        mosfet_gate_companion_charges.map(|charges| &charges[idx])
                                    }) {
                                        Self::install_cached_mosfet_gate_companion_charges(
                                            charges, qgs_out, cqgs_out, qgd_out, cqgd_out, qgb_out,
                                            cqgb_out,
                                        );
                                    } else {
                                        let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
                                        let cgs = cgs_half + previous_cap_halves.0 + cgs_ov;
                                        let cgd = cgd_half + previous_cap_halves.1 + cgd_ov;
                                        let cgb = cgb_half + previous_cap_halves.2 + cgb_ov;
                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgs,
                                                vgs,
                                                vgs_prev_prev[idx],
                                                BranchChargeHistory {
                                                    q_prev: qgs_prev_prev[idx],
                                                    q_prev_prev: qgs_prev_prev_prev[idx],
                                                    cq_prev: *cqgs_out,
                                                },
                                            );
                                        *qgs_out = q_curr;
                                        *cqgs_out = cq_curr;

                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgd,
                                                vgd,
                                                vgd_prev_prev[idx],
                                                BranchChargeHistory {
                                                    q_prev: qgd_prev_prev[idx],
                                                    q_prev_prev: qgd_prev_prev_prev[idx],
                                                    cq_prev: *cqgd_out,
                                                },
                                            );
                                        *qgd_out = q_curr;
                                        *cqgd_out = cq_curr;

                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgb,
                                                vgb,
                                                vgb_prev_prev[idx],
                                                BranchChargeHistory {
                                                    q_prev: qgb_prev_prev[idx],
                                                    q_prev_prev: qgb_prev_prev_prev[idx],
                                                    cq_prev: *cqgb_out,
                                                },
                                            );
                                        *qgb_out = q_curr;
                                        *cqgb_out = cq_curr;
                                    }
                                }

                                if !suppress_gate_charge_history {
                                    displacement_out[..3].copy_from_slice(
                                        &Self::accepted_mosfet_gate_currents(
                                            mos,
                                            accepted_solution,
                                            coeff,
                                            dt,
                                            [(cgs_half, cgd_half, cgb_half), previous_cap_halves],
                                            [*cqgs_out, *cqgd_out, *cqgb_out],
                                        ),
                                    );
                                }

                                let body_charge_mask = mos.body_junction_charge_mask();
                                if body_charge_mask & 1 != 0 {
                                    let vbs_j = mos.body_source_charge_branch_voltage(vbs);
                                    let (q_exact, capacitance) =
                                        mos.body_source_junction_charge_and_capacitance_at(vbs);
                                    let (geq, _ieq, q_curr, cq_curr) =
                                        nonlinear_charge_companion_terms(
                                            coeff,
                                            dt,
                                            capacitance,
                                            vbs_j,
                                            q_exact,
                                            BranchChargeHistory {
                                                q_prev: *qbs_out,
                                                q_prev_prev: *qbs_prev_out,
                                                cq_prev: *cqbs_out,
                                            },
                                        );
                                    *vbs_j_prev_out = *vbs_j_out;
                                    *vbs_j_out = vbs_j;
                                    *qbs_prev_out = *qbs_out;
                                    *qbs_out = q_curr;
                                    *cqbs_out = cq_curr;
                                    displacement_out[3] = cq_curr
                                        + geq
                                            * (mos.body_source_charge_branch_voltage(raw_vbs)
                                                - vbs_j);
                                }

                                if body_charge_mask & 2 != 0 {
                                    let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
                                    let (q_exact, capacitance) =
                                        mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
                                    let (geq, _ieq, q_curr, cq_curr) =
                                        nonlinear_charge_companion_terms(
                                            coeff,
                                            dt,
                                            capacitance,
                                            vbd_j,
                                            q_exact,
                                            BranchChargeHistory {
                                                q_prev: *qbd_out,
                                                q_prev_prev: *qbd_prev_out,
                                                cq_prev: *cqbd_out,
                                            },
                                        );
                                    *vbd_j_prev_out = *vbd_j_out;
                                    *vbd_j_out = vbd_j;
                                    *qbd_prev_out = *qbd_out;
                                    *qbd_out = q_curr;
                                    *cqbd_out = cq_curr;
                                    displacement_out[4] = cq_curr
                                        + geq
                                            * (mos.body_drain_charge_branch_voltage(
                                                raw_vds, raw_vbs,
                                            ) - vbd_j);
                                }
                            },
                        );
                };
                match prepared.mos_pool {
                    Some(pool) => pool.install(operation),
                    None => operation(),
                }
                true
            } else {
                false
            }
        };
        #[cfg(not(feature = "parallel"))]
        let mosfet_history_updated_in_parallel = false;

        let serial_mosfet_devices = if mosfet_history_updated_in_parallel {
            &[][..]
        } else {
            circuit.mosfets.devices.as_slice()
        };
        for (idx, mos) in serial_mosfet_devices.iter().enumerate() {
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
        mosfet_history.accepted_dt_prev_prev = mosfet_history.accepted_dt_prev;
        mosfet_history.accepted_dt_prev = dt;

        for (idx, vdmos) in circuit.vdmoses.devices.iter().enumerate() {
            let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(accepted_solution);
            let vd1 = vdmos.d1_charge_branch_voltage_at(accepted_solution);
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(accepted_solution);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (qbs_exact, cbs) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
            let (qbd_exact, cbd) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
            let (qd1_exact, cd1) = vdmos.d1_charge_and_capacitance_at(vd1);

            vdmos_history.vgs_prev_prev[idx] = vdmos_history.vgs_prev[idx];
            vdmos_history.vgs_prev[idx] = vgs;
            let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgs,
                vgs,
                vdmos_history.vgs_prev_prev[idx],
                BranchChargeHistory {
                    q_prev: vdmos_history.qgs_prev[idx],
                    q_prev_prev: vdmos_history.qgs_prev_prev[idx],
                    cq_prev: vdmos_history.cqgs_prev[idx],
                },
            );
            vdmos_history.qgs_prev_prev_prev[idx] = vdmos_history.qgs_prev_prev[idx];
            vdmos_history.qgs_prev_prev[idx] = vdmos_history.qgs_prev[idx];
            vdmos_history.qgs_prev[idx] = qgs_curr;
            vdmos_history.cqgs_prev[idx] = cqgs_curr;

            vdmos_history.vgd_prev_prev[idx] = vdmos_history.vgd_prev[idx];
            vdmos_history.vgd_prev[idx] = vgd;
            let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgd,
                vgd,
                vdmos_history.vgd_prev_prev[idx],
                BranchChargeHistory {
                    q_prev: vdmos_history.qgd_prev[idx],
                    q_prev_prev: vdmos_history.qgd_prev_prev[idx],
                    cq_prev: vdmos_history.cqgd_prev[idx],
                },
            );
            vdmos_history.qgd_prev_prev_prev[idx] = vdmos_history.qgd_prev_prev[idx];
            vdmos_history.qgd_prev_prev[idx] = vdmos_history.qgd_prev[idx];
            vdmos_history.qgd_prev[idx] = qgd_curr;
            vdmos_history.cqgd_prev[idx] = cqgd_curr;

            vdmos_history.vgb_prev_prev[idx] = vdmos_history.vgb_prev[idx];
            vdmos_history.vgb_prev[idx] = vgb;
            let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgb,
                vgb,
                vdmos_history.vgb_prev_prev[idx],
                BranchChargeHistory {
                    q_prev: vdmos_history.qgb_prev[idx],
                    q_prev_prev: vdmos_history.qgb_prev_prev[idx],
                    cq_prev: vdmos_history.cqgb_prev[idx],
                },
            );
            vdmos_history.qgb_prev_prev_prev[idx] = vdmos_history.qgb_prev_prev[idx];
            vdmos_history.qgb_prev_prev[idx] = vdmos_history.qgb_prev[idx];
            vdmos_history.qgb_prev[idx] = qgb_curr;
            vdmos_history.cqgb_prev[idx] = cqgb_curr;

            vdmos_history.vds_prev_prev[idx] = vdmos_history.vds_prev[idx];
            vdmos_history.vds_prev[idx] = vds;
            let (_geq_ds, _ieq_ds, qds_curr, cqds_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cds,
                vds,
                vdmos_history.vds_prev_prev[idx],
                BranchChargeHistory {
                    q_prev: vdmos_history.qds_prev[idx],
                    q_prev_prev: vdmos_history.qds_prev_prev[idx],
                    cq_prev: vdmos_history.cqds_prev[idx],
                },
            );
            vdmos_history.qds_prev_prev_prev[idx] = vdmos_history.qds_prev_prev[idx];
            vdmos_history.qds_prev_prev[idx] = vdmos_history.qds_prev[idx];
            vdmos_history.qds_prev[idx] = qds_curr;
            vdmos_history.cqds_prev[idx] = cqds_curr;

            vdmos_history.vbs_prev_prev[idx] = vdmos_history.vbs_prev[idx];
            vdmos_history.vbs_prev[idx] = vbs;
            let (_geq_bs, _ieq_bs, qbs_curr, cqbs_curr) = nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbs,
                vbs,
                qbs_exact,
                BranchChargeHistory {
                    q_prev: vdmos_history.qbs_prev[idx],
                    q_prev_prev: vdmos_history.qbs_prev_prev[idx],
                    cq_prev: vdmos_history.cqbs_prev[idx],
                },
            );
            vdmos_history.qbs_prev_prev_prev[idx] = vdmos_history.qbs_prev_prev[idx];
            vdmos_history.qbs_prev_prev[idx] = vdmos_history.qbs_prev[idx];
            vdmos_history.qbs_prev[idx] = qbs_curr;
            vdmos_history.cqbs_prev[idx] = cqbs_curr;

            vdmos_history.vbd_prev_prev[idx] = vdmos_history.vbd_prev[idx];
            vdmos_history.vbd_prev[idx] = vbd;
            let (_geq_bd, _ieq_bd, qbd_curr, cqbd_curr) = nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbd,
                vbd,
                qbd_exact,
                BranchChargeHistory {
                    q_prev: vdmos_history.qbd_prev[idx],
                    q_prev_prev: vdmos_history.qbd_prev_prev[idx],
                    cq_prev: vdmos_history.cqbd_prev[idx],
                },
            );
            vdmos_history.qbd_prev_prev_prev[idx] = vdmos_history.qbd_prev_prev[idx];
            vdmos_history.qbd_prev_prev[idx] = vdmos_history.qbd_prev[idx];
            vdmos_history.qbd_prev[idx] = qbd_curr;
            vdmos_history.cqbd_prev[idx] = cqbd_curr;

            vdmos_history.vd1_prev_prev[idx] = vdmos_history.vd1_prev[idx];
            vdmos_history.vd1_prev[idx] = vd1;
            let (_geq_d1, _ieq_d1, qd1_curr, cqd1_curr) = nonlinear_charge_companion_terms(
                coeff,
                dt,
                cd1,
                vd1,
                qd1_exact,
                BranchChargeHistory {
                    q_prev: vdmos_history.qd1_prev[idx],
                    q_prev_prev: vdmos_history.qd1_prev_prev[idx],
                    cq_prev: vdmos_history.cqd1_prev[idx],
                },
            );
            vdmos_history.qd1_prev_prev_prev[idx] = vdmos_history.qd1_prev_prev[idx];
            vdmos_history.qd1_prev_prev[idx] = vdmos_history.qd1_prev[idx];
            vdmos_history.qd1_prev[idx] = qd1_curr;
            vdmos_history.cqd1_prev[idx] = cqd1_curr;
        }
        vdmos_history.accepted_dt_prev_prev = vdmos_history.accepted_dt_prev;
        vdmos_history.accepted_dt_prev = dt;

        Self::update_b3soi_history(circuit, accepted_solution, coeff, dt, b3soi_history);
        Self::update_bsim3_history(circuit, accepted_solution, coeff, dt, bsim3_history);
        Self::update_bsim4_history(
            circuit,
            accepted_solution,
            Bsim4CompanionStep {
                coeff,
                trnqs_coeff: bsim4_trnqs_coeff,
                dt,
            },
            bsim4_history,
        );
        Self::update_ekv26_history(circuit, accepted_solution, coeff, dt, ekv26_history);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_mosfet_gate_companion_charges_install_bit_exactly() {
        let charges = [
            (1.25e-15, -2.5e-6),
            (-3.75e-15, 4.5e-6),
            (5.5e-15, -6.25e-6),
        ];
        let mut qgs = Value::NAN;
        let mut cqgs = Value::NAN;
        let mut qgd = Value::NAN;
        let mut cqgd = Value::NAN;
        let mut qgb = Value::NAN;
        let mut cqgb = Value::NAN;

        Engine::install_cached_mosfet_gate_companion_charges(
            &charges, &mut qgs, &mut cqgs, &mut qgd, &mut cqgd, &mut qgb, &mut cqgb,
        );

        assert_eq!(
            [qgs, cqgs, qgd, cqgd, qgb, cqgb].map(Value::to_bits),
            [
                charges[0].0,
                charges[0].1,
                charges[1].0,
                charges[1].1,
                charges[2].0,
                charges[2].1,
            ]
            .map(Value::to_bits)
        );
    }
    fn native_candidate(
        engine: &Engine,
        circuit: &mut crate::CircuitData,
        bjt: &mut BjtTransientHistory,
        solution: &[Value],
        time: Value,
    ) -> Result<(), SimulationError> {
        let coeff = CompanionCoefficients::backward_euler();
        engine.update_reactive_history(
            circuit,
            AcceptedReactiveStep {
                accepted_solution: solution,
                accepted_time: time,
                dt: 1e-9,
                coeff: &coeff,
                bsim4_trnqs_coeff: &coeff,
            },
            TransientDeviceHistories {
                bjt,
                jfet: &mut JfetTransientHistory::default(),
                diode: &mut DiodeTransientHistory::default(),
                mosfet: &mut MosfetTransientHistory::default(),
                vdmos: &mut VdmosTransientHistory::default(),
                b3soi: &mut B3SoiTransientHistory::default(),
                bsim3: &mut Bsim3TransientHistory::default(),
                bsim4: &mut Bsim4TransientHistory::default(),
                ekv26: &mut Ekv26TransientHistory::default(),
            },
            AcceptedReactiveSnapshots {
                xyce_one_step_order2: false,
                vbic_snapshots: None,
                capacitor_accepted_states: None,
                mosfet_caps: None,
                mosfet_gate_companion_charges: None,
                suppress_gate_charge_history: false,
                tline_dc_refs: &[],
                coupled_tline_refs: &[],
            },
            ReactiveBreakpointScheduling {
                breakpoints: &mut BreakpointManager::new(),
                tstop: 3e-9,
                voltage_reltol: 1e-3,
                voltage_abstol: 1e-6,
                current_abstol: 1e-12,
            },
            DynamicBreakpointSink {
                dynamic_breakpoints_added: &mut 0,
                warned_dynamic_breakpoint_cap: &mut false,
                pending_dynamic_breakpoints: &mut Vec::new(),
            },
        )
    }

    #[test]
    fn native_preparation_preserves_all_earlier_histories_when_bjt_or_expression_fails() {
        let deck = Netlist::parse("native preparation\nRin in 0 1k\nRbad bad 0 1k\nBfirst integral 0 V=sdt(v(in))\nBlater out 0 I={sdt(v(in))+v(bad)}\nRout out 0 1k\nRintegral integral 0 1k\nQfirst c b1 0 qm\nQlater c b2 0 qm\nRc c 0 1k\nRb1 b1 0 1k\nRb2 b2 0 1k\nC1 in 0 1n\nL1 c 0 1n\n.model qm NPN(IS=1e-14 CJE=1p CJC=1p TF=1n RC=1 RB=1 RE=1)\n.end\n").unwrap();
        let engine = Engine::default();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let mut solution = vec![0.0; circuit.matrix_size()];
        circuit
            .behavioral_sources
            .accept_transient_step(&solution, 0.0)
            .unwrap();
        let mut bjt =
            Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
        let before = bjt.clone();
        // Snapshot every mutable passive history generation.
        let capacitor_before = format!("{:?}", circuit.capacitors);
        let inductor_before = format!("{:?}", circuit.inductors);
        // Native series resistances can put the actual junction terminals on
        // generated nodes; inject the failure at the BJT's bound base node.
        let b1 = circuit.bjts.devices[0].node_base - 1;
        let b2 = circuit.bjts.devices[1].node_base - 1;
        let input = circuit.get_node_by_name("in").unwrap() - 1;
        let bad = circuit.get_node_by_name("bad").unwrap() - 1;
        solution[b1] = 0.6;
        solution[b2] = Value::NAN;
        solution[input] = 10.0;
        let branch = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
        solution[branch] = 0.5;
        let error = native_candidate(&engine, &mut circuit, &mut bjt, &solution, 1e-9).unwrap_err();
        assert!(
            error.to_string().to_ascii_lowercase().contains("qlater"),
            "{error}"
        );
        assert_eq!(bjt, before);
        assert_eq!(format!("{:?}", circuit.capacitors), capacitor_before);
        assert_eq!(format!("{:?}", circuit.inductors), inductor_before);

        solution[b2] = 0.5;
        solution[bad] = Value::NAN;
        let error = native_candidate(&engine, &mut circuit, &mut bjt, &solution, 1e-9).unwrap_err();
        assert!(
            error.to_string().to_ascii_lowercase().contains("blater"),
            "{error}"
        );
        assert_eq!(
            bjt, before,
            "prepared BJT values must not leak through a later expression failure"
        );
        assert_eq!(format!("{:?}", circuit.capacitors), capacitor_before);
        assert_eq!(format!("{:?}", circuit.inductors), inductor_before);

        // Also exercise the standalone family entry point: its first voltage
        // source cannot commit SDT before its later current source is validated.
        circuit
            .behavioral_sources
            .accept_transient_step(&solution, 1e-9)
            .unwrap_err();
        solution[bad] = 0.0;
        solution[input] = 0.0;
        native_candidate(&engine, &mut circuit, &mut bjt, &solution, 2e-9).unwrap();
        assert_ne!(bjt, before);
        assert_eq!(bjt.accepted_dt_prev, 1e-9);
        assert_eq!(circuit.inductors.i_prev, [0.5]);
        let integral = circuit.behavioral_sources.voltage_sources[0]
            .evaluate(&solution, 2e-9)
            .unwrap();
        assert_eq!(
            integral, 0.0,
            "a refused 10 V trial must contribute no area to accepted SDT state"
        );
        let integral = circuit.behavioral_sources.current_sources[0]
            .evaluate(&solution, 2e-9)
            .unwrap();
        assert_eq!(integral, 0.0);
    }
}
