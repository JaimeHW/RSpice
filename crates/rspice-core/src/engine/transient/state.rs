//! Reactive companion state and transient recovery helpers.

use super::*;

/// One transient companion stamp: the circuit being stamped, the matrix and
/// right-hand side it writes into, the trial node voltages it linearizes at,
/// and the companion coefficients and step that turn a charge into a
/// conductance and an equivalent current. The coefficients are derived for the
/// step, so a stamp handed one without the other would be linearizing against
/// an integration nobody asked for.
pub(in crate::engine) struct TransientCompanionStamp<'a, 'b> {
    pub circuit: &'a crate::circuit::CircuitData,
    pub matrix: &'a mut crate::solver::StaticMatrix,
    pub rhs: &'a mut [Value],
    pub voltages: &'b [Value],
    pub coeff: &'a CompanionCoefficients,
    pub dt: Value,
}

/// Every per-family reactive history a transient step carries. A restart
/// reseeds all of them from one accepted solution, and reseeding a subset
/// would leave the families disagreeing about which step they are on.
pub(super) struct TransientDeviceHistories<'a> {
    pub bjt: &'a mut BjtTransientHistory,
    pub jfet: &'a mut JfetTransientHistory,
    pub diode: &'a mut DiodeTransientHistory,
    pub mosfet: &'a mut MosfetTransientHistory,
    pub vdmos: &'a mut VdmosTransientHistory,
    pub b3soi: &'a mut B3SoiTransientHistory,
    pub bsim3: &'a mut Bsim3TransientHistory,
    pub bsim4: &'a mut Bsim4TransientHistory,
    pub ekv26: &'a mut Ekv26TransientHistory,
}

/// Which startup a reactive history is being seeded for.
///
/// `.TRAN ... UIC` has no operating point to seed from, so ngspice replaces it
/// with a single device load under `MODEINITJCT|MODETRANOP|MODEUIC`
/// (`maths/ni/niiter.c:41-47` runs exactly one `CKTload` and returns). That is
/// the only place either reference reads a device's instance `IC=` vector, so
/// it is the only seed that may consume one.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::engine) enum ReactiveHistorySeed {
    /// The t=0 seed of a `.TRAN ... UIC` run.
    UicStartup,
    /// A solved operating point, a resumed checkpoint, or an integration
    /// restart — all of which carry a real bias the device must follow.
    SolvedBias,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum AcceptedJunctionHistoryRestart {
    /// Preserve the authoritative accepted generation at a physical breakpoint.
    Preserve,
    /// Recompute current state from the accepted solution during livelock recovery.
    Reinitialize,
}

#[derive(Clone, Copy)]
pub(super) enum MosfetCompanionBiasSource {
    /// Derive both gate and evaluated branches from the supplied solution.
    Solution,
    /// Reuse a verified update cache and form gate branches at physical bias.
    VerifiedPhysicalGate,
    /// Reuse a verified update cache and form gate branches at evaluated bias.
    VerifiedEvaluatedGate,
}

impl Engine {
    /// The node voltages one device's `UIC` history seed reads.
    ///
    /// ngspice fills every ungiven `IC` component from the node solution before
    /// the UIC load runs — `BJTgetic`, `DIOgetic`, `JFETgetic` and `MOS1getic`
    /// all copy `CKTrhs` into the ungiven slots — and the load then evaluates
    /// the instance at those junction voltages. Authoring a component therefore
    /// means exactly "this device sees that terminal difference instead of the
    /// one the solution carries", and it means it for that device alone: the
    /// values live in the instance, never in `CKTrhsOld`, so nothing else in
    /// the circuit observes them. Reproduce that by handing this device its own
    /// copy of the solution with its own terminals moved.
    ///
    /// A terminal that is ground cannot move, so each difference is imposed on
    /// whichever of its two nodes is a real unknown; an `IC` component naming
    /// two grounded terminals is a contradiction the deck has already lost and
    /// is left alone. Differences are applied in order and each reads the
    /// running values, so a shared reference terminal stays consistent.
    fn seeded_device_solution<'a>(
        solution: &'a [Value],
        seed: ReactiveHistorySeed,
        differences: &[(usize, usize, Option<Value>)],
    ) -> std::borrow::Cow<'a, [Value]> {
        if seed != ReactiveHistorySeed::UicStartup
            || differences.iter().all(|(_, _, target)| target.is_none())
        {
            return std::borrow::Cow::Borrowed(solution);
        }

        let mut seeded = solution.to_vec();
        for (pos, neg, target) in differences {
            let Some(target) = *target else {
                continue;
            };
            if !target.is_finite() {
                continue;
            }
            if *pos > 0 {
                let reference = Self::node_voltage(&seeded, *neg);
                seeded[*pos - 1] = reference + target;
            } else if *neg > 0 {
                seeded[*neg - 1] = -target;
            }
        }
        std::borrow::Cow::Owned(seeded)
    }

    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltages(
        snapshot: &BjtChargeSnapshot,
    ) -> (Value, Value, Value) {
        let (vbe, vbc, _vbx, vcs) = Self::legacy_bjt_charge_branch_voltages_with_vbx(snapshot);
        (vbe, vbc, vcs)
    }

    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltages_with_vbx(
        snapshot: &BjtChargeSnapshot,
    ) -> (Value, Value, Value, Value) {
        let internal = &snapshot.reduction.internal_voltages;
        (
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VEI_STATE_INDEX],
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX],
            Self::legacy_bjt_charge_branch_voltage(
                snapshot,
                &snapshot.branches[BJT_QBCX_BRANCH_INDEX],
            ),
            Self::legacy_bjt_charge_branch_voltage(
                snapshot,
                &snapshot.branches[BJT_QBCP_BRANCH_INDEX],
            ),
        )
    }

    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltage(
        snapshot: &BjtChargeSnapshot,
        branch: &BjtChargeBranch,
    ) -> Value {
        Self::legacy_bjt_terminal_voltage(snapshot, branch.pos_internal, branch.pos_external)
            - Self::legacy_bjt_terminal_voltage(snapshot, branch.neg_internal, branch.neg_external)
    }

    #[inline]
    fn legacy_bjt_terminal_voltage(
        snapshot: &BjtChargeSnapshot,
        internal: Option<usize>,
        external: Option<usize>,
    ) -> Value {
        if let Some(idx) = internal {
            snapshot.reduction.internal_voltages[idx]
        } else if let Some(idx) = external {
            snapshot.reduction.external_voltages[idx]
        } else {
            0.0
        }
    }

    /// Breakpoint-style integration epoch restart.
    ///
    /// Re-seeds every reactive history like transient startup (flat history,
    /// zeroed companion derivatives, maxstep-seeded dt chains) so truncation
    /// estimators stop differencing the previous integration epoch. A physical
    /// breakpoint preserves accepted junction charge and JFET traps because their
    /// limited and reduced state cannot be reconstructed exactly from the
    /// external solution; livelock recovery deliberately recomputes it to
    /// discard poisoned history. Transmission-line delay buffers are left
    /// alone because they hold genuine propagating state.
    pub(super) fn reseed_reactive_histories_for_restart(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        hinted_max_step: Value,
        accepted_junction_history_restart: AcceptedJunctionHistoryRestart,
        histories: TransientDeviceHistories<'_>,
    ) {
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
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let v = Self::differential_voltage(solution, cap.pp.row, cap.nn.row);
            circuit.capacitors.v_prev[cap_idx] = v;
            circuit.capacitors.v_prev_prev[cap_idx] = v;
            circuit.capacitors.v_prev_prev_prev[cap_idx] = v;
            circuit.capacitors.i_prev[cap_idx] = 0.0;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let v = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v;
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i = solution.get(br_idx).copied().unwrap_or(0.0);
                circuit.inductors.i_prev[l_idx] = i;
                circuit.inductors.i_prev_prev[l_idx] = i;
                circuit.inductors.i_prev_prev_prev[l_idx] = i;
            }
        }
        circuit.update_coupled_inductor_pair_state(solution);
        circuit.update_multi_winding_transformer_state(solution);

        // A restart re-seeds from a solution the run already accepted, which is
        // a real bias every device must follow; the t=0 `IC=` vectors are spent.
        let seed = ReactiveHistorySeed::SolvedBias;
        match accepted_junction_history_restart {
            AcceptedJunctionHistoryRestart::Preserve => {
                Self::flatten_bjt_and_diode_histories_for_order_one_restart(
                    bjt_history,
                    diode_history,
                    hinted_max_step,
                );
                jfet_history.normalize_for_order_one(hinted_max_step);
            }
            AcceptedJunctionHistoryRestart::Reinitialize => {
                *bjt_history = Self::initialize_bjt_history(circuit, solution, seed);
                bjt_history.accepted_dt_prev = hinted_max_step;
                bjt_history.accepted_dt_prev_prev = hinted_max_step;
                *diode_history = Self::initialize_diode_history(circuit, solution, seed);
                diode_history.accepted_dt_prev = hinted_max_step;
                diode_history.accepted_dt_prev_prev = hinted_max_step;
                *jfet_history = Self::initialize_jfet_history(circuit, solution, seed);
                jfet_history.accepted_dt_prev = hinted_max_step;
                jfet_history.accepted_dt_prev_prev = hinted_max_step;
            }
        }
        *mosfet_history = Self::initialize_mosfet_history(circuit, solution, seed);
        mosfet_history.accepted_dt_prev = hinted_max_step;
        mosfet_history.accepted_dt_prev_prev = hinted_max_step;
        *vdmos_history = Self::initialize_vdmos_history(circuit, solution);
        vdmos_history.accepted_dt_prev = hinted_max_step;
        vdmos_history.accepted_dt_prev_prev = hinted_max_step;
        *b3soi_history = Self::initialize_b3soi_history(circuit, solution);
        *bsim3_history = Self::initialize_bsim3_history(circuit, solution);
        bsim3_history.accepted_dt_prev = hinted_max_step;
        bsim3_history.accepted_dt_prev_prev = hinted_max_step;
        *bsim4_history = Self::initialize_bsim4_history(circuit, solution);
        bsim4_history.accepted_dt_prev = hinted_max_step;
        bsim4_history.accepted_dt_prev_prev = hinted_max_step;
        *ekv26_history = Self::initialize_ekv26_history(circuit, solution);
        ekv26_history.accepted_dt_prev = hinted_max_step;
        ekv26_history.accepted_dt_prev_prev = hinted_max_step;
    }

    /// Starts a new order-one integration epoch without re-evaluating accepted
    /// nonlinear device state from the node solution.
    ///
    /// BJT charge reduction can carry internal voltages and limited junction
    /// biases which are not recoverable bit-for-bit from the external solution.
    /// Keep that accepted generation authoritative, flatten every older
    /// generation onto it, and discard only derivative history. Diode charge is
    /// handled the same way so a breakpoint cannot perturb an accepted limited
    /// junction bias by recomputing it.
    #[inline]
    pub(super) fn flatten_bjt_and_diode_histories_for_order_one_restart(
        bjt_history: &mut BjtTransientHistory,
        diode_history: &mut DiodeTransientHistory,
        accepted_dt_seed: Value,
    ) {
        bjt_history.vbe_prev_prev.clone_from(&bjt_history.vbe_prev);
        bjt_history.vbc_prev_prev.clone_from(&bjt_history.vbc_prev);
        bjt_history.vcs_prev_prev.clone_from(&bjt_history.vcs_prev);
        bjt_history.ibe_prev.fill(0.0);
        bjt_history.ibc_prev.fill(0.0);
        bjt_history.ics_prev.fill(0.0);
        bjt_history
            .charge_q_prev_prev
            .clone_from(&bjt_history.charge_q_prev);
        bjt_history
            .charge_q_prev_prev_prev
            .clone_from(&bjt_history.charge_q_prev);
        bjt_history
            .charge_cq_prev
            .fill([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
        bjt_history
            .dynamic_internal_prev_prev
            .clone_from(&bjt_history.dynamic_internal_prev);
        bjt_history
            .dynamic_linear_prev_prev
            .clone_from(&bjt_history.dynamic_linear_prev);
        bjt_history.accepted_dt_prev = accepted_dt_seed;
        bjt_history.accepted_dt_prev_prev = accepted_dt_seed;

        diode_history.restart(accepted_dt_seed);
    }

    #[inline]
    pub(in crate::engine) fn initialize_bjt_history(
        circuit: &crate::circuit::CircuitData,
        solution: &[Value],
        seed: ReactiveHistorySeed,
    ) -> BjtTransientHistory {
        let n = circuit.bjts.devices.len();
        let mut history = BjtTransientHistory {
            vbe_prev: Vec::with_capacity(n),
            vbe_prev_prev: Vec::with_capacity(n),
            ibe_prev: Vec::with_capacity(n),
            vbc_prev: Vec::with_capacity(n),
            vbc_prev_prev: Vec::with_capacity(n),
            ibc_prev: Vec::with_capacity(n),
            vcs_prev: Vec::with_capacity(n),
            vcs_prev_prev: Vec::with_capacity(n),
            ics_prev: Vec::with_capacity(n),
            charge_q_prev: Vec::with_capacity(n),
            charge_q_prev_prev: Vec::with_capacity(n),
            charge_q_prev_prev_prev: Vec::with_capacity(n),
            charge_cq_prev: Vec::with_capacity(n),
            accepted_external_bc_current: vec![0.0; n],
            accepted_terminal_currents: Vec::with_capacity(n),
            dynamic_internal_prev: Vec::with_capacity(n),
            dynamic_internal_prev_prev: Vec::with_capacity(n),
            dynamic_linear_prev: Vec::with_capacity(n),
            dynamic_linear_prev_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for bjt in &circuit.bjts.devices {
            // `IC=VBE,VCE` (`bjt/bjt.c:24`, `N_DEV_BJT.C:114`) states the
            // base-emitter and collector-emitter drops this instance opens at.
            // Both references normalize junction voltages by the polarity type
            // and then assign `vbe = type·icVBE` (`bjtload.c:247-248`,
            // `N_DEV_BJT.C:2872-2873`), so the two type factors cancel and the
            // raw terminal difference is the authored value for an NPN and a
            // PNP alike.
            //
            // A promoted VBIC instance is excluded: its junction voltages are
            // matrix unknowns of their own, and `vbicload.c:238-249` assigns
            // every one of them (`Vbei`, `Vbex`, `Vbci`, `Vbcx`, `Vbep`,
            // `Vbcp` and all four resistor drops) rather than only the two
            // terminal differences a node-space seed can reach. Moving the
            // external terminals alone would leave that instance half seeded,
            // which is worse than leaving the vector unread.
            let (ic_vbe, ic_vce) = if bjt.uses_vbic_dynamic_charges() {
                (None, None)
            } else {
                bjt.transient_initial_condition().unwrap_or((None, None))
            };
            let external_base_seed = bjt
                .legacy_external_bc_charge_nodes()
                .map_or((0, 0, None), |nodes| (nodes[0], bjt.node_emitter, ic_vbe));
            let seeded = Self::seeded_device_solution(
                solution,
                seed,
                &[
                    (bjt.node_base, bjt.node_emitter, ic_vbe),
                    (bjt.node_collector, bjt.node_emitter, ic_vce),
                    // MODEUIC seeds vbx=IC(VBE)-IC(VCE), before RBM too.
                    external_base_seed,
                    (
                        bjt.node_bx,
                        bjt.node_ei,
                        if bjt.mna_promoted() { ic_vbe } else { None },
                    ),
                    (
                        bjt.node_bi,
                        bjt.node_ei,
                        if bjt.mna_promoted() { ic_vbe } else { None },
                    ),
                    (
                        bjt.node_cx,
                        bjt.node_ei,
                        if bjt.mna_promoted() { ic_vce } else { None },
                    ),
                    (
                        bjt.node_ci,
                        bjt.node_ei,
                        if bjt.mna_promoted() { ic_vce } else { None },
                    ),
                ],
            );
            let solution = seeded.as_ref();
            let vc = Self::node_voltage(solution, bjt.node_collector);
            let vb = Self::node_voltage(solution, bjt.node_base);
            let ve = Self::node_voltage(solution, bjt.node_emitter);
            let vs = Self::node_voltage(solution, bjt.node_substrate);
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;

            if bjt.mna_promoted() {
                // Promoted BJT: the internal states are part of the solved
                // operating point, so the charge history seeds directly from
                // the solution vector with no nested snapshot solve.
                let (branches, internal, _) = bjt.mna_charge_state_at_solution(solution);
                let mut charge_values = branches.map(|branch| branch.charge);
                if let Some(charge) = bjt.legacy_external_bc_charge(solution) {
                    charge_values[BJT_QBCX_BRANCH_INDEX] = charge.charge;
                }
                let [vbe, vbc, vcs] = if bjt.uses_legacy_gummel_poon() {
                    bjt.mna_junction_voltages(solution)
                } else {
                    [vbe, vbc, vcs]
                };
                history.vbe_prev.push(vbe);
                history.vbe_prev_prev.push(vbe);
                history.ibe_prev.push(0.0);
                history.vbc_prev.push(vbc);
                history.vbc_prev_prev.push(vbc);
                history.ibc_prev.push(0.0);
                history.vcs_prev.push(vcs);
                history.vcs_prev_prev.push(vcs);
                history.ics_prev.push(0.0);
                history.charge_q_prev.push(charge_values);
                history.charge_q_prev_prev.push(charge_values);
                history.charge_q_prev_prev_prev.push(charge_values);
                history.charge_cq_prev.push([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
                history.accepted_terminal_currents.push(None);
                history.dynamic_internal_prev.push(internal);
                history.dynamic_internal_prev_prev.push(internal);
                history
                    .dynamic_linear_prev
                    .push(BjtPredictorLinearBranchState::default());
                history
                    .dynamic_linear_prev_prev
                    .push(BjtPredictorLinearBranchState::default());
                continue;
            }

            let charge_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (history_vbe, history_vbc, history_vcs) =
                Self::legacy_bjt_charge_branch_voltages(&charge_snapshot);
            history.vbe_prev.push(history_vbe);
            history.vbe_prev_prev.push(history_vbe);
            history.ibe_prev.push(0.0);
            history.vbc_prev.push(history_vbc);
            history.vbc_prev_prev.push(history_vbc);
            history.ibc_prev.push(0.0);
            history.vcs_prev.push(history_vcs);
            history.vcs_prev_prev.push(history_vcs);
            history.ics_prev.push(0.0);

            let mut charge_values = charge_snapshot.branches.map(|branch| branch.charge);
            let (legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages_with_vbx(&charge_snapshot);
            let charges = bjt.legacy_transient_charge_state_with_vbx(
                legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs,
            );
            charge_values[BJT_QBE_BRANCH_INDEX] = charges.qbe;
            charge_values[BJT_QBC_BRANCH_INDEX] = charges.qbc;
            charge_values[BJT_QBCX_BRANCH_INDEX] = charges.qbx;
            if let Some(charge) = bjt.legacy_external_bc_charge(solution) {
                charge_values[BJT_QBCX_BRANCH_INDEX] = charge.charge;
            }
            charge_values[BJT_QBCP_BRANCH_INDEX] = charges.qcs;
            let predictor_linear = Self::bjt_predictor_linear_branch_state(
                bjt,
                [vc, vb, ve, vs],
                charge_snapshot.reduction.internal_voltages,
            );
            history.charge_q_prev.push(charge_values);
            history.charge_q_prev_prev.push(charge_values);
            history.charge_q_prev_prev_prev.push(charge_values);
            history.charge_cq_prev.push([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
            history.accepted_terminal_currents.push(None);
            history
                .dynamic_internal_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history
                .dynamic_internal_prev_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history.dynamic_linear_prev.push(predictor_linear);
            history.dynamic_linear_prev_prev.push(predictor_linear);
        }

        history
    }

    #[inline]
    pub(in crate::engine) fn initialize_jfet_history(
        circuit: &crate::circuit::CircuitData,
        solution: &[Value],
        seed: ReactiveHistorySeed,
    ) -> JfetTransientHistory {
        let n = circuit.jfets.len();
        let mut history = JfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vds_prev: Vec::with_capacity(n),
            vds_prev_prev: Vec::with_capacity(n),
            qds_prev: Vec::with_capacity(n),
            qds_prev_prev: Vec::with_capacity(n),
            qds_prev_prev_prev: Vec::with_capacity(n),
            cqds_prev: Vec::with_capacity(n),
            jfet2_vgstrap_prev: Vec::with_capacity(n),
            jfet2_vgdtrap_prev: Vec::with_capacity(n),
            jfet2_power_prev: Vec::with_capacity(n),
            accepted_cqgs: vec![0.0; n],
            accepted_cqgd: vec![0.0; n],
            accepted_cqds: vec![0.0; n],
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for jfet in &circuit.jfets {
            // `IC=VDS,VGS` (`jfet/jfet.c:17`, `mes/mes.c:16`); `jfetload.c:106-111`
            // and `mesload.c:113-118` open the UIC transient operating point at
            // `vds = type·icVDS`, `vgs = type·icVGS`, `vgd = vgs - vds`, so the
            // gate-drain drop follows from the pair rather than being authored.
            // Xyce registers no `IC` on either device, so a deck that carries
            // one is an ngspice deck and takes the ngspice arm.
            //
            // HFET1 and the Xyce Sydney MESFET are excluded: both take their
            // evaluated branch voltages from the instance's own internal
            // branch state rather than from the solution (see
            // `jfet_branch_voltages`), so a node-space seed would reach their
            // charge history and not their evaluation.
            let (ic_vds, ic_vgs) = if matches!(
                jfet.params.channel_model,
                crate::device::JfetChannelModel::Hfet1
                    | crate::device::JfetChannelModel::XyceSydney
            ) {
                (None, None)
            } else {
                jfet.transient_initial_condition().unwrap_or((None, None))
            };
            let seeded = Self::seeded_device_solution(
                solution,
                seed,
                &[
                    (jfet.drain, jfet.source, ic_vds),
                    (jfet.gate, jfet.source, ic_vgs),
                ],
            );
            let solution = seeded.as_ref();
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, solution);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, solution);
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                None,
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            let qgs = jfet2_charge
                .map(|charge| charge.qgs)
                .unwrap_or_else(|| cgs.max(0.0) * vgs_charge);
            let qgd = jfet2_charge
                .map(|charge| charge.qgd)
                .unwrap_or_else(|| cgd.max(0.0) * vgd_charge);
            let qds = cds.max(0.0) * vds_charge;
            let (_, _, power) =
                jfet.jfet2_next_transient_memory(vgs_eval, vgd_eval, vgs_eval, vgd_eval, 0.0, 0.0);
            history.vgs_prev.push(vgs_charge);
            history.vgs_prev_prev.push(vgs_charge);
            history.qgs_prev.push(qgs);
            history.qgs_prev_prev.push(qgs);
            history.qgs_prev_prev_prev.push(qgs);
            history.cqgs_prev.push(0.0);
            history.vgd_prev.push(vgd_charge);
            history.vgd_prev_prev.push(vgd_charge);
            history.qgd_prev.push(qgd);
            history.qgd_prev_prev.push(qgd);
            history.qgd_prev_prev_prev.push(qgd);
            history.cqgd_prev.push(0.0);
            history.vds_prev.push(vds_charge);
            history.vds_prev_prev.push(vds_charge);
            history.qds_prev.push(qds);
            history.qds_prev_prev.push(qds);
            history.qds_prev_prev_prev.push(qds);
            history.cqds_prev.push(0.0);
            history.jfet2_vgstrap_prev.push(vgs_eval);
            history.jfet2_vgdtrap_prev.push(vgd_eval);
            history.jfet2_power_prev.push(power);
        }

        history
    }

    #[inline]
    pub(super) fn refresh_jfet2_transient_linearizations(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        dt: Value,
        history: &JfetTransientHistory,
    ) {
        for (idx, jfet) in circuit.jfets.iter_mut().enumerate() {
            jfet.refresh_jfet2_transient_operating_terms(
                solution,
                history.jfet2_vgstrap_prev[idx],
                history.jfet2_vgdtrap_prev[idx],
                history.jfet2_power_prev[idx],
                dt,
            );
        }
    }

    pub(super) fn initialize_diode_history(
        circuit: &crate::circuit::CircuitData,
        solution: &[Value],
        seed: ReactiveHistorySeed,
    ) -> DiodeTransientHistory {
        DiodeTransientHistory::from_biases(circuit.diodes.devices.iter().map(|diode| {
            // The diode is the one family whose `IC` is a scalar in both
            // references (`dio/dio.c:16` declares `IF_REAL`, `N_DEV_Diode.C:79`
            // a plain `addPar`): it names the junction drop directly, and
            // `dioload.c:153-157` opens the UIC transient operating point at
            // `vd = DIOinitCond`.
            let seeded = Self::seeded_device_solution(
                solution,
                seed,
                &[(
                    diode.node_anode,
                    diode.node_cathode,
                    diode.transient_initial_condition(),
                )],
            );
            let solution = seeded.as_ref();
            let vd = Self::differential_voltage(solution, diode.node_anode, diode.node_cathode);
            let (qd, _capd) = diode.junction_charge_and_capacitance(vd);
            (vd, qd)
        }))
    }

    #[inline]
    pub(super) fn initialize_mosfet_history(
        circuit: &crate::circuit::CircuitData,
        solution: &[Value],
        seed: ReactiveHistorySeed,
    ) -> MosfetTransientHistory {
        let n = circuit.mosfets.len();
        let legacy_lte = circuit
            .mosfets
            .devices
            .iter()
            .any(|mos| mos.uses_legacy_bsim());
        let mut history = MosfetTransientHistory {
            accepted_displacement_currents: vec![[0.0; 5]; n],
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            capgs_prev_half: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            capgd_prev_half: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            capgb_prev_half: Vec::with_capacity(n),
            qgb_prev: Vec::with_capacity(n),
            qgb_prev_prev: Vec::with_capacity(n),
            qgb_prev_prev_prev: Vec::with_capacity(n),
            cqgb_prev: Vec::with_capacity(n),
            vbs_j_prev: Vec::with_capacity(n),
            vbs_j_prev_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            qbs_prev_prev_prev: Vec::with_capacity(if legacy_lte { n } else { 0 }),
            cqbs_prev: Vec::with_capacity(n),
            vbd_j_prev: Vec::with_capacity(n),
            vbd_j_prev_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            qbd_prev_prev_prev: Vec::with_capacity(if legacy_lte { n } else { 0 }),
            cqbd_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for mos in &circuit.mosfets.devices {
            // `IC=VDS,VGS,VBS` (`mos1/mos1.c:29`, `N_DEV_MOSFET1.C:139`);
            // `mos1load.c:398-400` opens the UIC transient operating point at
            // those three drops.
            let authored_ic = mos.transient_initial_condition();
            let (ic_vds, ic_vgs, ic_vbs) = authored_ic.unwrap_or((None, None, None));
            let seeded = Self::seeded_device_solution(
                solution,
                seed,
                &[
                    (mos.node_drain, mos.node_source, ic_vds),
                    (mos.node_gate, mos.node_source, ic_vgs),
                    (mos.node_bulk, mos.node_source, ic_vbs),
                ],
            );
            let solution = seeded.as_ref();
            let (vgs, vds, vbs) =
                if seed == ReactiveHistorySeed::UicStartup && authored_ic.is_some() {
                    mos.unlimited_branch_voltages_at(solution)
                } else {
                    mos.eval_branch_voltages_at(solution)
                };
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            let (cgs_half, cgd_half, cgb_half) = mos.transient_capacitance_halves_at(vgs, vds, vbs);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = 2.0 * cgs_half + cgs_ov;
            let cgd = 2.0 * cgd_half + cgd_ov;
            let cgb = 2.0 * cgb_half + cgb_ov;

            let [qgs, qgd, qgb] = mos.legacy_gate_charge_at(vgs, vds, vbs).map_or(
                [cgs.max(0.0) * vgs, cgd.max(0.0) * vgd, cgb.max(0.0) * vgb],
                |charge| charge.charges,
            );

            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.capgs_prev_half.push(cgs_half);
            history.qgs_prev.push(qgs);
            history.qgs_prev_prev.push(qgs);
            history.qgs_prev_prev_prev.push(qgs);
            history.cqgs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.capgd_prev_half.push(cgd_half);
            history.qgd_prev.push(qgd);
            history.qgd_prev_prev.push(qgd);
            history.qgd_prev_prev_prev.push(qgd);
            history.cqgd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.capgb_prev_half.push(cgb_half);
            history.qgb_prev.push(qgb);
            history.qgb_prev_prev.push(qgb);
            history.qgb_prev_prev_prev.push(qgb);
            history.cqgb_prev.push(0.0);

            let vbs_j = mos.body_source_charge_branch_voltage(vbs);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
            let (qbs, _) = mos.body_source_junction_charge_and_capacitance_at(vbs);
            let (qbd, _) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
            history.vbs_j_prev.push(vbs_j);
            history.vbs_j_prev_prev.push(vbs_j);
            history.qbs_prev.push(qbs);
            history.qbs_prev_prev.push(qbs);
            if legacy_lte {
                history.qbs_prev_prev_prev.push(qbs);
            }
            history.cqbs_prev.push(0.0);
            history.vbd_j_prev.push(vbd_j);
            history.vbd_j_prev_prev.push(vbd_j);
            history.qbd_prev.push(qbd);
            history.qbd_prev_prev.push(qbd);
            if legacy_lte {
                history.qbd_prev_prev_prev.push(qbd);
            }
            history.cqbd_prev.push(0.0);
        }

        history
    }

    #[inline]
    pub(super) fn initialize_vdmos_history(
        circuit: &crate::circuit::CircuitData,
        solution: &[Value],
    ) -> VdmosTransientHistory {
        let n = circuit.vdmoses.len();
        let mut history = VdmosTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            qgb_prev: Vec::with_capacity(n),
            qgb_prev_prev: Vec::with_capacity(n),
            qgb_prev_prev_prev: Vec::with_capacity(n),
            cqgb_prev: Vec::with_capacity(n),
            vds_prev: Vec::with_capacity(n),
            vds_prev_prev: Vec::with_capacity(n),
            qds_prev: Vec::with_capacity(n),
            qds_prev_prev: Vec::with_capacity(n),
            qds_prev_prev_prev: Vec::with_capacity(n),
            cqds_prev: Vec::with_capacity(n),
            vbs_prev: Vec::with_capacity(n),
            vbs_prev_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            qbs_prev_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            vbd_prev: Vec::with_capacity(n),
            vbd_prev_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            qbd_prev_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            vd1_prev: Vec::with_capacity(n),
            vd1_prev_prev: Vec::with_capacity(n),
            qd1_prev: Vec::with_capacity(n),
            qd1_prev_prev: Vec::with_capacity(n),
            qd1_prev_prev_prev: Vec::with_capacity(n),
            cqd1_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for vdmos in &circuit.vdmoses.devices {
            let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(solution);
            let vd1 = vdmos.d1_charge_branch_voltage_at(solution);
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(solution);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (qbs, _) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
            let (qbd, _) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
            let (qd1, _) = vdmos.d1_charge_and_capacitance_at(vd1);
            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.qgs_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev_prev.push(cgs.max(0.0) * vgs);
            history.cqgs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.qgd_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev_prev.push(cgd.max(0.0) * vgd);
            history.cqgd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.qgb_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev_prev.push(cgb.max(0.0) * vgb);
            history.cqgb_prev.push(0.0);

            history.vds_prev.push(vds);
            history.vds_prev_prev.push(vds);
            history.qds_prev.push(cds.max(0.0) * vds);
            history.qds_prev_prev.push(cds.max(0.0) * vds);
            history.qds_prev_prev_prev.push(cds.max(0.0) * vds);
            history.cqds_prev.push(0.0);

            history.vbs_prev.push(vbs);
            history.vbs_prev_prev.push(vbs);
            history.qbs_prev.push(qbs);
            history.qbs_prev_prev.push(qbs);
            history.qbs_prev_prev_prev.push(qbs);
            history.cqbs_prev.push(0.0);

            history.vbd_prev.push(vbd);
            history.vbd_prev_prev.push(vbd);
            history.qbd_prev.push(qbd);
            history.qbd_prev_prev.push(qbd);
            history.qbd_prev_prev_prev.push(qbd);
            history.cqbd_prev.push(0.0);

            history.vd1_prev.push(vd1);
            history.vd1_prev_prev.push(vd1);
            history.qd1_prev.push(qd1);
            history.qd1_prev_prev.push(qd1);
            history.qd1_prev_prev_prev.push(qd1);
            history.cqd1_prev.push(0.0);
        }

        history
    }

    #[inline]
    pub(in crate::engine) fn stamp_bjt_transient_companions(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        xyce_one_step_order2: bool,
    ) -> Result<(), SimulationError> {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        let charge_factor = Self::jfet_companion_geq(coeff, 1.0, dt);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(voltages, bjt.node_collector);
            let vb = Self::node_voltage(voltages, bjt.node_base);
            let ve = Self::node_voltage(voltages, bjt.node_emitter);
            let vs = Self::node_voltage(voltages, bjt.node_substrate);

            if charge_factor <= 0.0 {
                continue;
            }
            if let Some(charge) = bjt.legacy_external_bc_charge(voltages) {
                let current = Self::jfet_companion_ccap(
                    coeff,
                    dt,
                    charge.charge,
                    BranchChargeHistory {
                        q_prev: history.charge_q_prev[idx][BJT_QBCX_BRANCH_INDEX],
                        q_prev_prev: history.charge_q_prev_prev[idx][BJT_QBCX_BRANCH_INDEX],
                        cq_prev: history.charge_cq_prev[idx][BJT_QBCX_BRANCH_INDEX],
                    },
                );
                let conductance = charge_factor * charge.capacitance;
                let source = conductance * charge.voltage - current;
                if !conductance.is_finite() || !source.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "BJT '{}' has a nonfinite external BC companion",
                        bjt.name
                    )));
                }
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    charge.nodes[0],
                    charge.nodes[1],
                    conductance,
                    source,
                );
            }
            if bjt.mna_promoted() {
                // Promoted BJT: per-branch charge companions on the actual
                // internal nodes (ngspice NIintegrate discipline), evaluated
                // and linearized at the limited bias cached by the device
                // update for this Newton iterate.
                let (branches, internal, external) = bjt.mna_charge_state();
                let mut stamper = StaticMatrixChargeStamper {
                    matrix: &mut *matrix,
                    rhs: &mut *rhs,
                };
                for (branch_idx, branch) in branches.iter().enumerate() {
                    if !branch.is_active() {
                        continue;
                    }
                    let cq = Self::jfet_companion_ccap(
                        coeff,
                        dt,
                        branch.charge,
                        BranchChargeHistory {
                            q_prev: history.charge_q_prev[idx][branch_idx],
                            q_prev_prev: history.charge_q_prev_prev[idx][branch_idx],
                            cq_prev: history.charge_cq_prev[idx][branch_idx],
                        },
                    );
                    if !cq.is_finite()
                        || branch
                            .d_internal
                            .iter()
                            .chain(&branch.d_external)
                            .any(|value| !(charge_factor * value).is_finite())
                    {
                        return Err(SimulationError::Circuit(format!(
                            "BJT '{}' transient charge {branch_idx} is nonfinite for dt={dt:e}",
                            bjt.name,
                        )));
                    }
                    let polarity = bjt.charge_branch_polarity(branch_idx);
                    Self::stamp_vbic_mna_charge_branch(
                        &mut stamper,
                        bjt,
                        branch,
                        polarity * charge_factor,
                        polarity * cq,
                        &internal,
                        &external,
                    );
                }
                continue;
            }

            // Schur elimination must follow the OneStep static/history split.
            // The private equations are half a trapezoidal companion; reducing
            // a BE companion first and then halving only its DC stamp changes
            // the internal resistance/charge balance.
            let private_coeff = if xyce_one_step_order2 {
                CompanionCoefficients::trapezoidal()
            } else {
                *coeff
            };
            let coeff = &private_coeff;
            let cached_snapshot = vbic_snapshot_cache.get(idx).copied().flatten();
            let Some(snapshot) = Self::resolve_legacy_bjt_transient_snapshot(
                bjt,
                [vc, vb, ve, vs],
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
                cached_snapshot,
            ) else {
                vbic_snapshot_cache[idx] = None;
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' private transient state did not converge for dt={dt:e}",
                    bjt.name
                )));
            };

            if !snapshot.branches.iter().any(BjtChargeBranch::is_active) {
                vbic_snapshot_cache[idx] = None;
                continue;
            }
            let Some(linearization) = Self::assemble_legacy_bjt_transient_linearization(
                bjt,
                &snapshot,
                BjtChargeStep {
                    coeff,
                    dt,
                    q_prev: &history.charge_q_prev[idx],
                    q_prev_prev: &history.charge_q_prev_prev[idx],
                    cq_prev: &history.charge_cq_prev[idx],
                },
            ) else {
                vbic_snapshot_cache[idx] = None;
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' transient companion could not be assembled for dt={dt:e}",
                    bjt.name
                )));
            };
            let (base_static_g, base_static_i_eq) =
                Self::bjt_static_stamped_external_system(bjt, &[vc, vb, ve, vs]);
            vbic_snapshot_cache[idx] = Some(snapshot);
            let Some((mut y_total, mut reduced_i_eq)) =
                Self::reduce_bjt_transient_external_system(&linearization)
            else {
                vbic_snapshot_cache[idx] = None;
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' transient companion could not be reduced for dt={dt:e}",
                    bjt.name
                )));
            };

            if xyce_one_step_order2 {
                let previous_current =
                    history.accepted_terminal_currents[idx].ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "BJT '{}' OneStep companion requires accepted terminal-current history",
                            bjt.name
                        ))
                    })?;
                let previous_external = Self::bjt_external_from_linear_history(
                    bjt,
                    &history.dynamic_internal_prev[idx],
                    &history.dynamic_linear_prev[idx],
                );
                let [vc, vb, ve, vs] = previous_external;
                let previous_static = bjt.external_terminal_currents_at_bias(vc, vb, ve, vs);
                // At an external terminal, OneStep is half the full Trap
                // current plus half the previous total current. The global
                // history already supplies half the previous DC-reduced
                // current, so replace that term before projecting tied nodes.
                for row in 0..BJT_EXTERNAL_STATE_DIM {
                    reduced_i_eq[row] += previous_static[row] - previous_current[row];
                }
            }
            bjt.project_legacy_tied_terminal_system(&mut y_total, &mut reduced_i_eq);
            let weight = if xyce_one_step_order2 { 0.5 } else { 1.0 };
            let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
            let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
            for row in 0..BJT_EXTERNAL_STATE_DIM {
                delta_i_eq[row] = weight * (reduced_i_eq[row] - base_static_i_eq[row]);
                for col in 0..BJT_EXTERNAL_STATE_DIM {
                    delta[row][col] = weight * (y_total[row][col] - base_static_g[row][col]);
                }
            }
            let nodes = [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ];
            Self::stamp_external_reduced_system(matrix, rhs, &nodes, &delta, &delta_i_eq);
        }
        Ok(())
    }

    /// Stamp one promoted BJT charge branch as a Norton companion on its
    /// actual matrix nodes. Charge branches use the standard MNA orientation:
    /// the integrated current `cq` leaves the positive node and enters the
    /// negative node, with conductance `ag0 * dq/dv` across every coupled
    /// column and the linearization point folded into the source term.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn stamp_vbic_mna_charge_branch(
        stamper: &mut impl crate::device::MatrixStamper,
        bjt: &crate::device::Bjt,
        branch: &BjtChargeBranch,
        ag0: Value,
        cq: Value,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        let external_nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let mut source = -cq;
        for (derivative, voltage) in branch
            .d_internal
            .iter()
            .zip(internal)
            .take(BJT_INTERNAL_STATE_DIM)
        {
            source += ag0 * derivative * voltage;
        }
        for (derivative, voltage) in branch
            .d_external
            .iter()
            .zip(external)
            .take(BJT_EXTERNAL_STATE_DIM)
        {
            source += ag0 * derivative * voltage;
        }

        let mut stamp_row = |row: crate::NodeId, sign: Value| {
            if row == 0 {
                return;
            }
            for col in 0..BJT_INTERNAL_STATE_DIM {
                let g = ag0 * branch.d_internal[col];
                if g != 0.0 {
                    stamper.stamp(row, bjt.mna_internal_node(col), sign * g);
                }
            }
            for (derivative, node) in branch
                .d_external
                .iter()
                .zip(external_nodes)
                .take(BJT_EXTERNAL_STATE_DIM)
            {
                let g = ag0 * derivative;
                if g != 0.0 {
                    stamper.stamp(row, node, sign * g);
                }
            }
            stamper.stamp_rhs(row, sign * source);
        };

        let pos = branch
            .pos_internal
            .map(|idx| bjt.mna_internal_node(idx))
            .or_else(|| branch.pos_external.map(|idx| external_nodes[idx]));
        let neg = branch
            .neg_internal
            .map(|idx| bjt.mna_internal_node(idx))
            .or_else(|| branch.neg_external.map(|idx| external_nodes[idx]));
        if let Some(row) = pos {
            stamp_row(row, 1.0);
        }
        if let Some(row) = neg {
            stamp_row(row, -1.0);
        }
    }

    #[inline]
    pub(in crate::engine) fn stamp_jfet_transient_companions(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &JfetTransientHistory,
        suppress_gate_charge: bool,
    ) {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, voltages);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, voltages);
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                Some((
                    history.vgs_prev[idx],
                    history.vgd_prev[idx],
                    history.qgs_prev[idx],
                    history.qgd_prev[idx],
                )),
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;

            if !suppress_gate_charge && cgs.is_finite() && cgs > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = if let Some(charge) = jfet2_charge {
                    nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        charge.qgs,
                        BranchChargeHistory {
                            q_prev: history.qgs_prev[idx],
                            q_prev_prev: history.qgs_prev_prev[idx],
                            cq_prev: history.cqgs_prev[idx],
                        },
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        history.vgs_prev[idx],
                        BranchChargeHistory {
                            q_prev: history.qgs_prev[idx],
                            q_prev_prev: history.qgs_prev_prev[idx],
                            cq_prev: history.cqgs_prev[idx],
                        },
                    )
                };
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.source, geq, ieq);
            }

            if !suppress_gate_charge && cgd.is_finite() && cgd > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = if let Some(charge) = jfet2_charge {
                    nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        charge.qgd,
                        BranchChargeHistory {
                            q_prev: history.qgd_prev[idx],
                            q_prev_prev: history.qgd_prev_prev[idx],
                            cq_prev: history.cqgd_prev[idx],
                        },
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        history.vgd_prev[idx],
                        BranchChargeHistory {
                            q_prev: history.qgd_prev[idx],
                            q_prev_prev: history.qgd_prev_prev[idx],
                            cq_prev: history.cqgd_prev[idx],
                        },
                    )
                };
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.drain, geq, ieq);
            }

            if cds.is_finite() && cds > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    coeff,
                    dt,
                    cds,
                    vds_charge,
                    history.vds_prev[idx],
                    BranchChargeHistory {
                        q_prev: history.qds_prev[idx],
                        q_prev_prev: history.qds_prev_prev[idx],
                        cq_prev: history.cqds_prev[idx],
                    },
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.drain, jfet.source, geq, ieq);
            }
        }
    }

    /// Resolve the matrix slots for the five MOSFET charge companions
    /// (gate-source, gate-drain, gate-bulk, body-source, body-drain).
    pub(super) fn link_mosfet_companion_slots(
        circuit: &crate::circuit::CircuitData,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<[TwoTerminalStampSlots; 5]> {
        circuit
            .mosfets
            .devices
            .iter()
            .map(|mos| {
                let (bs_pos, bs_neg) = mos.body_source_charge_nodes();
                let (bd_pos, bd_neg) = mos.body_drain_charge_nodes();
                [
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_source),
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_drain),
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_bulk),
                    TwoTerminalStampSlots::link(matrix, bs_pos, bs_neg),
                    TwoTerminalStampSlots::link(matrix, bd_pos, bd_neg),
                ]
            })
            .collect()
    }

    pub(super) fn link_compact_mosfet_companion_slots(
        circuit: &crate::circuit::CircuitData,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<[CompactTwoTerminalStampSlots; 5]> {
        circuit
            .mosfets
            .devices
            .iter()
            .map(|mos| {
                let (bs_pos, bs_neg) = mos.body_source_charge_nodes();
                let (bd_pos, bd_neg) = mos.body_drain_charge_nodes();
                [
                    CompactTwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_source),
                    CompactTwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_drain),
                    CompactTwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_bulk),
                    CompactTwoTerminalStampSlots::link(matrix, bs_pos, bs_neg),
                    CompactTwoTerminalStampSlots::link(matrix, bd_pos, bd_neg),
                ]
            })
            .collect()
    }

    pub(super) fn link_vdmos_companion_slots(
        circuit: &crate::circuit::CircuitData,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<[TwoTerminalStampSlots; 7]> {
        circuit
            .vdmoses
            .devices
            .iter()
            .map(|vdmos| {
                let (gs_pos, gs_neg) = vdmos.gate_source_charge_nodes();
                let (gd_pos, gd_neg) = vdmos.gate_drain_charge_nodes();
                let (gb_pos, gb_neg) = vdmos.gate_bulk_charge_nodes();
                let (ds_pos, ds_neg) = vdmos.drain_source_charge_nodes();
                let (bs_pos, bs_neg) = vdmos.body_source_charge_nodes();
                let (bd_pos, bd_neg) = vdmos.body_drain_charge_nodes();
                let (d1_pos, d1_neg) = vdmos.d1_charge_nodes();
                [
                    TwoTerminalStampSlots::link(matrix, gs_pos, gs_neg),
                    TwoTerminalStampSlots::link(matrix, gd_pos, gd_neg),
                    TwoTerminalStampSlots::link(matrix, gb_pos, gb_neg),
                    TwoTerminalStampSlots::link(matrix, ds_pos, ds_neg),
                    TwoTerminalStampSlots::link(matrix, bs_pos, bs_neg),
                    TwoTerminalStampSlots::link(matrix, bd_pos, bd_neg),
                    TwoTerminalStampSlots::link(matrix, d1_pos, d1_neg),
                ]
            })
            .collect()
    }

    #[inline]
    pub(super) fn stamp_mosfet_transient_companions(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        use_verified_cached_bias: bool,
        slots: &[[TwoTerminalStampSlots; 5]],
        caps_cache_out: Option<&mut Vec<(Value, Value, Value)>>,
    ) {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        // NOTE (M3.2, measured 2026-06-12 on mos_array_4096): evaluating
        // these per-device terms on the rayon pool — par_chunks(256) with a
        // serial in-order apply, bit-identical to this loop at any thread
        // count — was 29% SLOWER than this serial walk (stamp 0.52s → 0.67s
        // over the run). Per-iteration term buffers plus fork/join overhead
        // exceed what ~100 ns level-1 evaluations can save even at 4096
        // devices. Parallel device evaluation only pays once a section
        // carries microsecond-scale models (VBIC/BSIM tiers) or the whole
        // iteration (companions + conduction + update) is fused into one
        // pool pass over persistent scratch. The terms helper below stays
        // pure precisely so that fused pass can be built when the model
        // tiers justify it.
        let mut caps_cache = caps_cache_out;
        if let Some(cache) = caps_cache.as_deref_mut() {
            cache.clear();
            cache.reserve(circuit.mosfets.devices.len());
        }
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            if !suppress_gate_charge && mos.uses_legacy_bsim() {
                let (vgs, vds, vbs) = mos.eval_branch_voltages_at(voltages);
                if let Some(charge) = mos.legacy_gate_charge_at(vgs, vds, vbs) {
                    let charges = Self::integrate_mosfet_gate_charges(
                        charge.charges,
                        coeff,
                        dt,
                        [
                            BranchChargeHistory {
                                q_prev: history.qgs_prev[idx],
                                q_prev_prev: history.qgs_prev_prev[idx],
                                cq_prev: history.cqgs_prev[idx],
                            },
                            BranchChargeHistory {
                                q_prev: history.qgd_prev[idx],
                                q_prev_prev: history.qgd_prev_prev[idx],
                                cq_prev: history.cqgd_prev[idx],
                            },
                            BranchChargeHistory {
                                q_prev: history.qgb_prev[idx],
                                q_prev_prev: history.qgb_prev_prev[idx],
                                cq_prev: history.cqgb_prev[idx],
                            },
                        ],
                    );
                    mos.stamp_legacy_gate_charge(
                        &charge,
                        Self::jfet_companion_geq(coeff, 1.0, dt),
                        charges.map(|(_, cq)| cq),
                        [vgs, vds, vbs],
                        &mut StaticMatrixChargeStamper { matrix, rhs },
                    );
                }
            }
            let (device_terms, _charges, caps) = Self::mosfet_companion_branch_terms::<false>(
                mos,
                idx,
                voltages,
                coeff,
                dt,
                history,
                suppress_gate_charge,
                if use_verified_cached_bias {
                    MosfetCompanionBiasSource::VerifiedPhysicalGate
                } else {
                    MosfetCompanionBiasSource::Solution
                },
                None,
            );
            if let Some(cache) = caps_cache.as_deref_mut() {
                cache.push(caps);
            }
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &slots[idx][branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    /// Apply companion terms evaluated alongside the candidate's nonlinear
    /// update. Sparse writes remain serial and deterministic; only the pure
    /// per-device arithmetic is moved onto the bounded MOS worker pool.
    #[inline]
    pub(super) fn stamp_cached_mosfet_transient_companions(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        slots: &[[TwoTerminalStampSlots; 5]],
        terms: &[MosfetCompanionBranchTerms],
    ) {
        debug_assert_eq!(slots.len(), terms.len());
        for (device_slots, device_terms) in slots.iter().zip(terms) {
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &device_slots[branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    /// Apply cached MOS companion terms to a once-validated CSC value slice.
    #[inline]
    pub(super) fn stamp_cached_mosfet_transient_companion_values(
        values: &mut [Value],
        rhs: &mut [Value],
        slots: &[[TwoTerminalStampSlots; 5]],
        terms: &[MosfetCompanionBranchTerms],
    ) {
        debug_assert_eq!(slots.len(), terms.len());
        for (device_slots, device_terms) in slots.iter().zip(terms) {
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_values(
                        values,
                        rhs,
                        &device_slots[branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    /// Compact-offset twin of [`Self::stamp_mosfet_transient_companions`].
    /// The caller supplies the one frozen-pattern token for the whole device
    /// batch. A mismatch leaves the matrix, RHS, and optional capacitance
    /// cache untouched so the caller can relink and use the checked path.
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn stamp_mosfet_transient_compact_companions_for_pattern(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        use_verified_cached_bias: bool,
        pattern: Option<crate::solver::CscPatternToken>,
        slots: &[[CompactTwoTerminalStampSlots; 5]],
        caps_cache_out: Option<&mut Vec<(Value, Value, Value)>>,
    ) -> bool {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        if circuit
            .mosfets
            .devices
            .iter()
            .any(|mos| mos.uses_legacy_bsim())
        {
            return false;
        }
        if slots.len() != circuit.mosfets.devices.len() {
            return false;
        }
        let Some(values) = pattern.and_then(|pattern| matrix.values_mut_for_pattern(pattern))
        else {
            return false;
        };
        let mut caps_cache = caps_cache_out;
        if let Some(cache) = caps_cache.as_deref_mut() {
            cache.clear();
            cache.reserve(circuit.mosfets.devices.len());
        }
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (device_terms, _charges, caps) = Self::mosfet_companion_branch_terms::<false>(
                mos,
                idx,
                voltages,
                coeff,
                dt,
                history,
                suppress_gate_charge,
                if use_verified_cached_bias {
                    MosfetCompanionBiasSource::VerifiedPhysicalGate
                } else {
                    MosfetCompanionBiasSource::Solution
                },
                None,
            );
            if let Some(cache) = caps_cache.as_deref_mut() {
                cache.push(caps);
            }
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_compact_two_terminal_companion_values(
                        values,
                        rhs,
                        &slots[idx][branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
        true
    }

    /// Compact-offset twin used only after the enclosing classic-MOS cache
    /// validates its frozen CSC pattern once for the complete batch.
    #[inline]
    pub(super) fn stamp_cached_mosfet_transient_compact_companion_values(
        values: &mut [Value],
        rhs: &mut [Value],
        slots: &[[CompactTwoTerminalStampSlots; 5]],
        terms: &[MosfetCompanionBranchTerms],
    ) {
        debug_assert_eq!(slots.len(), terms.len());
        for (device_slots, device_terms) in slots.iter().zip(terms) {
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_compact_two_terminal_companion_values(
                        values,
                        rhs,
                        &device_slots[branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    /// Charge-companion `(geq, ieq)` for one MOSFET's five reactive branches
    /// (gate-source, gate-drain, gate-bulk, body-source, body-drain) at the
    /// given iterate. Pure: no engine or device state is touched, which is
    /// what lets the transient assembly evaluate devices on the thread pool.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn mosfet_companion_branch_terms<const CAPTURE_CHARGES: bool>(
        mos: &crate::device::Mosfet,
        idx: usize,
        voltages: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        bias_source: MosfetCompanionBiasSource,
        constants: Option<&crate::device::mosfet::ClassicMosTransientConstants>,
    ) -> (
        MosfetCompanionBranchTerms,
        MosfetGateCompanionCharges,
        (Value, Value, Value),
    ) {
        let mut terms = [(0.0, 0.0); 5];
        let mut charges = [(0.0, 0.0); 3];
        let mut caps = (0.0, 0.0, 0.0);
        let ((vgs, vgd, vgb), (vgs_eval, vds_eval, vbs_eval)) = match bias_source {
            MosfetCompanionBiasSource::Solution => (
                mos.gate_charge_branch_voltages_at(voltages),
                mos.eval_branch_voltages_at(voltages),
            ),
            MosfetCompanionBiasSource::VerifiedPhysicalGate => {
                let ((vgs, vds, vbs), evaluated) = mos.verified_cached_transient_branch_voltages();
                ((vgs, vgs - vds, vgs - vbs), evaluated)
            }
            MosfetCompanionBiasSource::VerifiedEvaluatedGate => {
                let (_, evaluated) = mos.verified_cached_transient_branch_voltages();
                let (vgs, vds, vbs) = evaluated;
                ((vgs, vgs - vds, vgs - vbs), evaluated)
            }
        };

        if !suppress_gate_charge && !mos.uses_legacy_bsim() {
            let unit_geq = Self::jfet_companion_geq(coeff, 1.0, dt);
            let (cgs_half, cgd_half, cgb_half) = if let Some(constants) = constants {
                mos.transient_capacitance_halves_with_constants(
                    vgs_eval, vds_eval, vbs_eval, constants,
                )
            } else {
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval)
            };
            caps = (cgs_half, cgd_half, cgb_half);
            let (cgs_ov, cgd_ov, cgb_ov) = if let Some(constants) = constants {
                mos.overlap_capacitances_with_constants(constants)
            } else {
                mos.overlap_capacitances()
            };
            let cgs = cgs_half + history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + history.capgb_prev_half[idx] + cgb_ov;

            let (geq_gs, ieq_gs, qgs, cqgs) = Self::jfet_companion_terms_with_unit_geq(
                coeff,
                dt,
                unit_geq,
                cgs,
                vgs,
                history.vgs_prev[idx],
                BranchChargeHistory {
                    q_prev: history.qgs_prev[idx],
                    q_prev_prev: history.qgs_prev_prev[idx],
                    cq_prev: history.cqgs_prev[idx],
                },
            );
            terms[0] = (geq_gs, ieq_gs);
            if CAPTURE_CHARGES {
                charges[0] = (qgs, cqgs);
            }

            let (geq_gd, ieq_gd, qgd, cqgd) = Self::jfet_companion_terms_with_unit_geq(
                coeff,
                dt,
                unit_geq,
                cgd,
                vgd,
                history.vgd_prev[idx],
                BranchChargeHistory {
                    q_prev: history.qgd_prev[idx],
                    q_prev_prev: history.qgd_prev_prev[idx],
                    cq_prev: history.cqgd_prev[idx],
                },
            );
            terms[1] = (geq_gd, ieq_gd);
            if CAPTURE_CHARGES {
                charges[1] = (qgd, cqgd);
            }

            let (geq_gb, ieq_gb, qgb, cqgb) = Self::jfet_companion_terms_with_unit_geq(
                coeff,
                dt,
                unit_geq,
                cgb,
                vgb,
                history.vgb_prev[idx],
                BranchChargeHistory {
                    q_prev: history.qgb_prev[idx],
                    q_prev_prev: history.qgb_prev_prev[idx],
                    cq_prev: history.cqgb_prev[idx],
                },
            );
            terms[2] = (geq_gb, ieq_gb);
            if CAPTURE_CHARGES {
                charges[2] = (qgb, cqgb);
            }
        }

        let body_charge_mask = constants.map_or_else(
            || mos.body_junction_charge_mask(),
            |constants| mos.body_junction_charge_mask_with_constants(constants),
        );
        if body_charge_mask & 1 != 0 {
            let vbs_j = mos.body_source_charge_branch_voltage(vbs_eval);
            let (qbs_curr, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
            let (geq_bs, ieq_bs, _q, _cq) = nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbs,
                vbs_j,
                qbs_curr,
                BranchChargeHistory {
                    q_prev: history.qbs_prev[idx],
                    q_prev_prev: history.qbs_prev_prev[idx],
                    cq_prev: history.cqbs_prev[idx],
                },
            );
            terms[3] = (geq_bs, ieq_bs);
        }

        if body_charge_mask & 2 != 0 {
            let vbd_j = mos.body_drain_charge_branch_voltage(vds_eval, vbs_eval);
            let (qbd_curr, cbd) =
                mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);
            let (geq_bd, ieq_bd, _q, _cq) = nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbd,
                vbd_j,
                qbd_curr,
                BranchChargeHistory {
                    q_prev: history.qbd_prev[idx],
                    q_prev_prev: history.qbd_prev_prev[idx],
                    cq_prev: history.cqbd_prev[idx],
                },
            );
            terms[4] = (geq_bd, ieq_bd);
        }

        (terms, charges, caps)
    }

    pub(super) fn integrate_mosfet_gate_charges(
        charges: [Value; 3],
        coeff: &CompanionCoefficients,
        dt: Value,
        history: [BranchChargeHistory; 3],
    ) -> MosfetGateCompanionCharges {
        std::array::from_fn(|branch| {
            (
                charges[branch],
                Self::jfet_companion_ccap(coeff, dt, charges[branch], history[branch]),
            )
        })
    }

    #[inline]
    pub(super) fn stamp_vdmos_transient_companions(
        stamp: TransientCompanionStamp<'_, '_>,
        history: &VdmosTransientHistory,
        slots: &[[TwoTerminalStampSlots; 7]],
    ) {
        let TransientCompanionStamp {
            circuit,
            matrix,
            rhs,
            voltages,
            coeff,
            dt,
        } = stamp;
        for (idx, vdmos) in circuit.vdmoses.devices.iter().enumerate() {
            let terms =
                Self::vdmos_companion_branch_terms(vdmos, idx, voltages, coeff, dt, history);
            for (branch, &(geq, ieq)) in terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &slots[idx][branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    fn vdmos_companion_branch_terms(
        vdmos: &crate::device::Vdmos,
        idx: usize,
        voltages: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        history: &VdmosTransientHistory,
    ) -> [(Value, Value); 7] {
        let mut terms = [(0.0, 0.0); 7];
        let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(voltages);
        let vd1 = vdmos.d1_charge_branch_voltage_at(voltages);
        let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(voltages);
        let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
        let cgb = vdmos.gate_bulk_capacitance();
        let (qbs, cbs) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
        let (qbd, cbd) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
        let (qd1, cd1) = vdmos.d1_charge_and_capacitance_at(vd1);

        let (geq_gs, ieq_gs, _qgs, _cqgs) = Self::jfet_companion_terms(
            coeff,
            dt,
            cgs,
            vgs,
            history.vgs_prev[idx],
            BranchChargeHistory {
                q_prev: history.qgs_prev[idx],
                q_prev_prev: history.qgs_prev_prev[idx],
                cq_prev: history.cqgs_prev[idx],
            },
        );
        terms[0] = (geq_gs, ieq_gs);

        let (geq_gd, ieq_gd, _qgd, _cqgd) = Self::jfet_companion_terms(
            coeff,
            dt,
            cgd,
            vgd,
            history.vgd_prev[idx],
            BranchChargeHistory {
                q_prev: history.qgd_prev[idx],
                q_prev_prev: history.qgd_prev_prev[idx],
                cq_prev: history.cqgd_prev[idx],
            },
        );
        terms[1] = (geq_gd, ieq_gd);

        let (geq_gb, ieq_gb, _qgb, _cqgb) = Self::jfet_companion_terms(
            coeff,
            dt,
            cgb,
            vgb,
            history.vgb_prev[idx],
            BranchChargeHistory {
                q_prev: history.qgb_prev[idx],
                q_prev_prev: history.qgb_prev_prev[idx],
                cq_prev: history.cqgb_prev[idx],
            },
        );
        terms[2] = (geq_gb, ieq_gb);

        let (geq_ds, ieq_ds, _qds, _cqds) = Self::jfet_companion_terms(
            coeff,
            dt,
            cds,
            vds,
            history.vds_prev[idx],
            BranchChargeHistory {
                q_prev: history.qds_prev[idx],
                q_prev_prev: history.qds_prev_prev[idx],
                cq_prev: history.cqds_prev[idx],
            },
        );
        terms[3] = (geq_ds, ieq_ds);

        let (geq_bs, ieq_bs, _qbs, _cqbs) = nonlinear_charge_companion_terms(
            coeff,
            dt,
            cbs,
            vbs,
            qbs,
            BranchChargeHistory {
                q_prev: history.qbs_prev[idx],
                q_prev_prev: history.qbs_prev_prev[idx],
                cq_prev: history.cqbs_prev[idx],
            },
        );
        terms[4] = (geq_bs, ieq_bs);

        let (geq_bd, ieq_bd, _qbd, _cqbd) = nonlinear_charge_companion_terms(
            coeff,
            dt,
            cbd,
            vbd,
            qbd,
            BranchChargeHistory {
                q_prev: history.qbd_prev[idx],
                q_prev_prev: history.qbd_prev_prev[idx],
                cq_prev: history.cqbd_prev[idx],
            },
        );
        terms[5] = (geq_bd, ieq_bd);

        let (geq_d1, ieq_d1, _qd1, _cqd1) = nonlinear_charge_companion_terms(
            coeff,
            dt,
            cd1,
            vd1,
            qd1,
            BranchChargeHistory {
                q_prev: history.qd1_prev[idx],
                q_prev_prev: history.qd1_prev_prev[idx],
                cq_prev: history.cqd1_prev[idx],
            },
        );
        terms[6] = (geq_d1, ieq_d1);

        terms
    }
}

pub(super) type MosfetCompanionBranchTerms = [(Value, Value); 5];
pub(super) type MosfetGateCompanionCharges = [(Value, Value); 3];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_bsim_history_seeds_exact_charge_for_solved_and_uic_bias() {
        for level in [4, 5] {
            for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
                let netlist = Netlist::parse(&format!(
                    "legacy BSIM charge seed\nRD d 0 1k\nRG g 0 1k\nRS s 0 1k\nRB b 0 1k\nM1 d g s b mm W=2u L=1u AD=2p AS=3p PD=4u PS=5u IC={},{},{}\n.model mm {kind}(LEVEL={level} TOX=.03 VFB=-.7 PHI=.6 K1=.5 VBB=-5 VDD=5 CJ=1m CJSW=1n PB=.8 PBSW=.2 MJ=.5 MJSW=.5)\n.end\n",
                    p*0.2, p*1.5, p*-0.3)).unwrap();
                let engine = Engine::default();
                let circuit = engine.build_circuit(&netlist).unwrap();
                assert!(
                    !circuit.has_cacheable_classic_mos_transient_base(),
                    "a reciprocal-capacitor cache cannot stamp BSIM terminal charge"
                );
                let mos = &circuit.mosfets.devices[0];
                let expected = mos
                    .legacy_gate_charge_at(p * 1.5, p * 0.2, p * -0.3)
                    .unwrap()
                    .charges;
                for seed in [
                    ReactiveHistorySeed::UicStartup,
                    ReactiveHistorySeed::SolvedBias,
                ] {
                    let mut voltage = vec![0.0; circuit.matrix_size()];
                    if seed == ReactiveHistorySeed::SolvedBias {
                        voltage[mos.node_gate - 1] = p * 1.5;
                        voltage[mos.node_drain - 1] = p * 0.2;
                        voltage[mos.node_bulk - 1] = p * -0.3;
                    }
                    let history = Engine::initialize_mosfet_history(&circuit, &voltage, seed);
                    for (branch, (first, second, third)) in [
                        (
                            &history.qgs_prev,
                            &history.qgs_prev_prev,
                            &history.qgs_prev_prev_prev,
                        ),
                        (
                            &history.qgd_prev,
                            &history.qgd_prev_prev,
                            &history.qgd_prev_prev_prev,
                        ),
                        (
                            &history.qgb_prev,
                            &history.qgb_prev_prev,
                            &history.qgb_prev_prev_prev,
                        ),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        assert!((first[0] - expected[branch]).abs() < 1e-28);
                        assert_eq!(first, second);
                        assert_eq!(first, third);
                    }
                    for (first, second, third, bias, bottom, sidewall) in [
                        (
                            &history.qbs_prev,
                            &history.qbs_prev_prev,
                            &history.qbs_prev_prev_prev,
                            -0.3,
                            3e-15,
                            5e-15,
                        ),
                        (
                            &history.qbd_prev,
                            &history.qbd_prev_prev,
                            &history.qbd_prev_prev_prev,
                            -0.5,
                            2e-15,
                            4e-15,
                        ),
                    ] {
                        let q = [(bottom, 0.8_f64), (sidewall, 0.2_f64)]
                            .into_iter()
                            .map(|(c, phi)| 2.0 * c * bias / (1.0 + (1.0 - bias / phi).sqrt()))
                            .sum::<Value>();
                        assert!((first[0] - q).abs() < q.abs() * 1e-12);
                        assert_eq!(first, second);
                        assert_eq!(first, third);
                    }
                    assert_eq!(history.cqbs_prev, [0.0]);
                    assert_eq!(history.cqbd_prev, [0.0]);
                    assert_eq!(history.cqgs_prev, [0.0]);
                    assert_eq!(history.cqgd_prev, [0.0]);
                    assert_eq!(history.cqgb_prev, [0.0]);
                    assert_eq!(history.capgs_prev_half, [0.0]);
                    assert_eq!(history.capgd_prev_half, [0.0]);
                    assert_eq!(history.capgb_prev_half, [0.0]);
                }
            }
        }
    }

    #[test]
    fn physical_breakpoint_preserves_jfet_traps_and_accepted_charge() {
        let netlist = crate::netlist::Netlist::parse("JFET restart\nVd d 0 2\nVg g 0 -0.5\nJ1 d g 0 jm\n.model jm NJF(LEVEL=2 CGS=1n CGD=2n TAUG=1u TAUD=2u)\n.end\n").unwrap();
        let mut circuit = Engine::default().build_circuit(&netlist).unwrap();
        let solution = vec![0.0; circuit.matrix_size()];
        let mut history =
            Engine::initialize_jfet_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
        history.jfet2_vgstrap_prev[0] = -0.73;
        history.jfet2_vgdtrap_prev[0] = -2.1;
        history.jfet2_power_prev[0] = 0.031;
        history.qgs_prev[0] = 1.23e-9;
        history.qgd_prev[0] = -2.34e-9;
        history.qds_prev[0] = 3.45e-9;
        history.accepted_cqgs[0] = 0.012;
        history.accepted_cqgd[0] = -0.023;
        history.accepted_cqds[0] = 0.034;
        Engine::reseed_reactive_histories_for_restart(
            &mut circuit,
            &solution,
            1e-9,
            AcceptedJunctionHistoryRestart::Preserve,
            TransientDeviceHistories {
                bjt: &mut BjtTransientHistory::default(),
                jfet: &mut history,
                diode: &mut DiodeTransientHistory::default(),
                mosfet: &mut MosfetTransientHistory::default(),
                vdmos: &mut VdmosTransientHistory::default(),
                b3soi: &mut B3SoiTransientHistory::default(),
                bsim3: &mut Bsim3TransientHistory::default(),
                bsim4: &mut Bsim4TransientHistory::default(),
                ekv26: &mut Ekv26TransientHistory::default(),
            },
        );
        assert_eq!(history.jfet2_vgstrap_prev, [-0.73]);
        assert_eq!(history.jfet2_vgdtrap_prev, [-2.1]);
        assert_eq!(history.jfet2_power_prev, [0.031]);
        assert_eq!(history.qgs_prev, [1.23e-9]);
        assert_eq!(history.qgd_prev, [-2.34e-9]);
        assert_eq!(history.qds_prev, [3.45e-9]);
        assert_eq!(history.qgs_prev_prev_prev, history.qgs_prev);
        assert_eq!(history.cqgs_prev, [0.0]);
        assert_eq!(history.accepted_cqgs, [0.012]);
        assert_eq!(history.accepted_cqgd, [-0.023]);
        assert_eq!(history.accepted_cqds, [0.034]);
    }

    #[test]
    fn diode_charge_stamp_and_commit_preserve_zero_and_signed_slopes() {
        let netlist = Netlist::parse(
            "signed charge continuation\nR1 p 0 1k\nR2 n 0 1k\nD1 p n dm\n.model dm D\n.end\n",
        )
        .unwrap();
        let engine = Engine::default();
        let mut circuit = engine.build_circuit(&netlist).unwrap();
        // The depletion continuation with M=-1 gives Q(V)=V-V^2/2,
        // dQ/dV=1-V. Pin the resolved parameters to avoid temperature
        // adjustment in this stamp-level test of the driver's branch law.
        let diode = &mut circuit.diodes.devices[0];
        diode.cj0 = 1.0;
        diode.vj = 1.0;
        diode.m = -1.0;
        diode.fc = 0.5;
        diode.tt = 0.0;
        let (pos, neg) = (diode.node_anode - 1, diode.node_cathode - 1);
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let mut voltage = vec![0.0; circuit.matrix_size()];
        voltage[pos] = 0.5;
        let initial =
            Engine::initialize_diode_history(&circuit, &voltage, ReactiveHistorySeed::SolvedBias);
        assert_eq!(initial.qd_prev, [0.375]);
        let coeff = CompanionCoefficients::backward_euler();
        for vd in [1.0, 1.5] {
            let mut history = initial.clone();
            let mut rhs = vec![0.0; circuit.matrix_size()];
            voltage[pos] = vd;
            matrix.clear_values();
            circuit.diodes.stamp_charge_companions(
                &mut matrix,
                &mut rhs,
                &voltage,
                &coeff,
                0.25,
                &history,
                false,
            );
            let charge = vd - 0.5 * vd * vd;
            let current = (charge - 0.375) / 0.25;
            let source = (1.0 - vd) / 0.25 * vd - current;
            assert_eq!(rhs[pos], source);
            assert_eq!(rhs[neg], -source);
            assert_eq!(history, initial, "a trial must not commit charge");
            history.accept_branch(0, vd, charge, &coeff, 0.25);
            assert_eq!(history.qd_prev, [charge]);
            assert_eq!(history.cqd_prev, [current]);
        }
    }
    use crate::Netlist;

    #[test]
    fn order_one_restart_flattens_bjt_history_around_exact_accepted_state() {
        let accepted_charge: [Value; BJT_DYNAMIC_CHARGE_COUNT] =
            std::array::from_fn(|idx| 10.0 + idx as Value);
        let accepted_internal: [Value; BJT_INTERNAL_STATE_DIM] =
            std::array::from_fn(|idx| 20.0 + idx as Value);
        let accepted_terminal: [Value; BJT_EXTERNAL_STATE_DIM] =
            std::array::from_fn(|idx| 30.0 + idx as Value);
        let accepted_linear = BjtPredictorLinearBranchState {
            vrcx: 41.0,
            vrci: 42.0,
            vrbx: 43.0,
            vrbi: 44.0,
            vre: 45.0,
            vrbp: 46.0,
            vrs: 47.0,
        };
        let mut bjt_history = BjtTransientHistory {
            vbe_prev: vec![1.0],
            vbe_prev_prev: vec![-1.0],
            ibe_prev: vec![101.0],
            vbc_prev: vec![2.0],
            vbc_prev_prev: vec![-2.0],
            ibc_prev: vec![102.0],
            vcs_prev: vec![3.0],
            vcs_prev_prev: vec![-3.0],
            ics_prev: vec![103.0],
            charge_q_prev: vec![accepted_charge],
            charge_q_prev_prev: vec![[-4.0; BJT_DYNAMIC_CHARGE_COUNT]],
            charge_q_prev_prev_prev: vec![[-5.0; BJT_DYNAMIC_CHARGE_COUNT]],
            charge_cq_prev: vec![[104.0; BJT_DYNAMIC_CHARGE_COUNT]],
            accepted_external_bc_current: vec![107.0],
            accepted_terminal_currents: vec![Some(accepted_terminal)],
            dynamic_internal_prev: vec![accepted_internal],
            dynamic_internal_prev_prev: vec![[-6.0; BJT_INTERNAL_STATE_DIM]],
            dynamic_linear_prev: vec![accepted_linear],
            dynamic_linear_prev_prev: vec![BjtPredictorLinearBranchState::default()],
            accepted_dt_prev: 105.0,
            accepted_dt_prev_prev: 106.0,
        };

        Engine::flatten_bjt_and_diode_histories_for_order_one_restart(
            &mut bjt_history,
            &mut DiodeTransientHistory::default(),
            0.0,
        );

        assert_eq!(bjt_history.vbe_prev, vec![1.0]);
        assert_eq!(bjt_history.vbe_prev_prev, bjt_history.vbe_prev);
        assert_eq!(bjt_history.vbc_prev, vec![2.0]);
        assert_eq!(bjt_history.vbc_prev_prev, bjt_history.vbc_prev);
        assert_eq!(bjt_history.vcs_prev, vec![3.0]);
        assert_eq!(bjt_history.vcs_prev_prev, bjt_history.vcs_prev);
        assert_eq!(bjt_history.ibe_prev, vec![0.0]);
        assert_eq!(bjt_history.ibc_prev, vec![0.0]);
        assert_eq!(bjt_history.ics_prev, vec![0.0]);
        assert_eq!(bjt_history.charge_q_prev, vec![accepted_charge]);
        assert_eq!(bjt_history.charge_q_prev_prev, bjt_history.charge_q_prev);
        assert_eq!(
            bjt_history.charge_q_prev_prev_prev,
            bjt_history.charge_q_prev
        );
        assert_eq!(
            bjt_history.charge_cq_prev,
            vec![[0.0; BJT_DYNAMIC_CHARGE_COUNT]]
        );
        assert_eq!(
            bjt_history.accepted_terminal_currents,
            vec![Some(accepted_terminal)]
        );
        assert_eq!(bjt_history.dynamic_internal_prev, vec![accepted_internal]);
        assert_eq!(
            bjt_history.dynamic_internal_prev_prev,
            bjt_history.dynamic_internal_prev
        );
        assert_eq!(bjt_history.accepted_external_bc_current, vec![107.0]);
        assert_eq!(bjt_history.dynamic_linear_prev[0].vrcx, 41.0);
        assert_eq!(bjt_history.dynamic_linear_prev_prev[0].vrcx, 41.0);
        assert_eq!(bjt_history.dynamic_linear_prev_prev[0].vrs, 47.0);
        assert_eq!(bjt_history.accepted_dt_prev, 0.0);
        assert_eq!(bjt_history.accepted_dt_prev_prev, 0.0);
    }

    #[test]
    fn order_one_restart_flattens_diode_history_around_exact_accepted_state() {
        let mut diode_history = DiodeTransientHistory {
            vd_prev: vec![1.25],
            vd_prev_prev: vec![-1.0],
            qd_prev: vec![2.5],
            qd_prev_prev: vec![-2.0],
            qd_prev_prev_prev: vec![-3.0],
            cqd_prev: vec![99.0],
            accepted_dt_prev: 3.0,
            accepted_dt_prev_prev: 4.0,
        };

        Engine::flatten_bjt_and_diode_histories_for_order_one_restart(
            &mut BjtTransientHistory::default(),
            &mut diode_history,
            0.125,
        );

        assert_eq!(diode_history.vd_prev, vec![1.25]);
        assert_eq!(diode_history.vd_prev_prev, diode_history.vd_prev);
        assert_eq!(diode_history.qd_prev, vec![2.5]);
        assert_eq!(diode_history.qd_prev_prev, diode_history.qd_prev);
        assert_eq!(diode_history.qd_prev_prev_prev, diode_history.qd_prev);
        assert_eq!(diode_history.cqd_prev, vec![0.0]);
        assert_eq!(diode_history.accepted_dt_prev, 0.125);
        assert_eq!(diode_history.accepted_dt_prev_prev, 0.125);
    }

    #[test]
    fn pvdmos_companion_slots_follow_polarity_normalized_charge_voltages() {
        let deck = "\
PVDMOS charge slot orientation
VD d 0 -0.5
VG g 0 0
VS s 0 0
M1 d g s 0 PM W=1 L=1u
.MODEL PM PMOS LEVEL=18
+ VTO=-100
+ RD=0
+ RS=0
+ RG=0
+ CGDO=1e-11
+ CGSO=1e-11
+ CGBO=1e-11
+ CBD=0
+ CBS=0
+ D1CJO=1e-12
+ D1TT=0
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let slots = Engine::link_vdmos_companion_slots(&circuit, &matrix);
        let vdmos = &circuit.vdmoses.devices[0];
        let di = vdmos.drain_int.unwrap_or(vdmos.drain);
        let si = vdmos.source_int.unwrap_or(vdmos.source);
        let d1p = vdmos.d1_prime.unwrap_or(vdmos.source);

        let expected = [
            (si, vdmos.gate),
            (di, vdmos.gate),
            (vdmos.bulk, vdmos.gate),
            (si, di),
            (si, vdmos.bulk),
            (di, vdmos.bulk),
            (d1p, vdmos.drain),
        ];

        for (branch, (slot, expected_nodes)) in slots[0].iter().zip(expected).enumerate() {
            assert_eq!(
                (slot.pos, slot.neg),
                expected_nodes,
                "PVDMOS companion branch {branch} must be oriented with the polarity-normalized charge voltage"
            );
        }
    }
}
