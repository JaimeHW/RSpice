//! Charge truncation and transient LTE control helpers.

#![allow(clippy::needless_range_loop)]

#[cfg(test)]
use super::state::MosfetCompanionBiasSource;
use super::*;

#[cfg(feature = "parallel")]
const CAPACITOR_TRUNCATION_ITEMS_PER_WORKER: usize = 1_024;
#[cfg(feature = "parallel")]
const CAPACITOR_TRUNCATION_MAX_WORKERS: usize = 8;

#[inline]
pub(super) fn lte_debug_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("RSPICE_LTE_DEBUG").is_some())
}

/// Per-device-family invariants for ngspice's `CKTterr` divided-difference
/// charge test. One family evaluates this same timestep/tolerance geometry for
/// every charge branch, so validate and precompute it once outside hot loops.
#[derive(Clone, Copy)]
pub(super) struct NgspiceChargeTruncationContext {
    dt: Value,
    prev_dt: Value,
    prev_prev_dt: Value,
    first_span: Value,
    second_span: Value,
    total_span: Value,
    order: u8,
    factor: Value,
    reltol: Value,
    current_abstol: Value,
    charge_abstol: Value,
    trtol: Value,
}

impl NgspiceChargeTruncationContext {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub(super) fn new(
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Self> {
        if !dt.is_finite()
            || dt <= 0.0
            || !prev_dt.is_finite()
            || prev_dt <= 0.0
            || !trtol.is_finite()
            || trtol <= 0.0
        {
            return None;
        }

        let mut order = trap_order.clamp(1, 2);
        if order >= 2 && (!prev_prev_dt.is_finite() || prev_prev_dt <= 0.0) {
            order = 1;
        }
        let first_span = dt + prev_dt;
        if !first_span.is_finite() || first_span <= 0.0 {
            return None;
        }
        let (second_span, total_span) = if order >= 2 {
            let second_span = prev_dt + prev_prev_dt;
            let total_span = dt + second_span;
            if !second_span.is_finite()
                || second_span <= 0.0
                || !total_span.is_finite()
                || total_span <= 0.0
            {
                return None;
            }
            (second_span, total_span)
        } else {
            (0.0, first_span)
        };

        Some(Self {
            dt,
            prev_dt,
            prev_prev_dt,
            first_span,
            second_span,
            total_span,
            order,
            factor: Engine::ngspice_vbic_truncation_factor(method, order),
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        })
    }

    #[inline]
    fn limit(
        &self,
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        q_prev_prev_prev: Value,
        cq_curr: Value,
        cq_prev: Value,
    ) -> Option<Value> {
        let volttol = self.current_abstol + self.reltol * cq_curr.abs().max(cq_prev.abs());
        let chargetol =
            self.reltol * q_curr.abs().max(q_prev.abs()).max(self.charge_abstol) / self.dt;
        let tol = volttol.max(chargetol);
        if !tol.is_finite() || tol <= 0.0 {
            return None;
        }

        // This is the scalar expansion of ngspice's in-place divided-
        // difference triangle. Preserve its operation order so the optimized
        // family path remains bit-identical to the reference helper.
        let d01 = (q_curr - q_prev) / self.dt;
        let d12 = (q_prev - q_prev_prev) / self.prev_dt;
        let diff = if self.order >= 2 {
            let d23 = (q_prev_prev - q_prev_prev_prev) / self.prev_prev_dt;
            let dd01 = (d01 - d12) / self.first_span;
            let dd12 = (d12 - d23) / self.second_span;
            (dd01 - dd12) / self.total_span
        } else {
            (d01 - d12) / self.first_span
        };
        let denom = self.current_abstol.max(self.factor * diff.abs());
        if !denom.is_finite() || denom <= 0.0 {
            return None;
        }

        let mut limit = self.trtol * tol / denom;
        if self.order >= 2 {
            limit = limit.sqrt();
        }
        (limit.is_finite() && limit > 0.0).then_some(limit)
    }

    /// Apply the MOS1 gate-charge CKTterr walk to q/cq values already
    /// evaluated by the accepted candidate's companion kernel. Capacitance
    /// eligibility is reconstructed exactly so even subnormal/underflow edge
    /// cases retain the canonical branch-selection semantics.
    #[inline]
    pub(super) fn mosfet_gate_limit_from_cached_charges(
        &self,
        mos: &crate::device::Mosfet,
        idx: usize,
        charges: &MosfetGateCompanionCharges,
        caps: (Value, Value, Value),
        history: &MosfetTransientHistory,
    ) -> Option<Value> {
        self.mosfet_gate_limit_from_cached_charges_and_overlap(
            idx,
            charges,
            caps,
            mos.overlap_capacitances(),
            history,
        )
    }

    #[inline]
    fn mosfet_gate_limit_from_cached_charges_and_overlap(
        &self,
        idx: usize,
        charges: &MosfetGateCompanionCharges,
        caps: (Value, Value, Value),
        overlap: (Value, Value, Value),
        history: &MosfetTransientHistory,
    ) -> Option<Value> {
        let (cgs_ov, cgd_ov, cgb_ov) = overlap;
        let capacitances = [
            caps.0 + history.capgs_prev_half[idx] + cgs_ov,
            caps.1 + history.capgd_prev_half[idx] + cgd_ov,
            caps.2 + history.capgb_prev_half[idx] + cgb_ov,
        ];
        let histories = [
            (
                history.qgs_prev[idx],
                history.qgs_prev_prev[idx],
                history.qgs_prev_prev_prev[idx],
                history.cqgs_prev[idx],
            ),
            (
                history.qgd_prev[idx],
                history.qgd_prev_prev[idx],
                history.qgd_prev_prev_prev[idx],
                history.cqgd_prev[idx],
            ),
            (
                history.qgb_prev[idx],
                history.qgb_prev_prev[idx],
                history.qgb_prev_prev_prev[idx],
                history.cqgb_prev[idx],
            ),
        ];

        let mut limit = 2.0 * self.dt;
        let mut found_branch = false;
        for branch in 0..3 {
            let capacitance = capacitances[branch];
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }
            let (q_curr, cq_curr) = charges[branch];
            let (q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) = histories[branch];
            let Some(branch_limit) = self.limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }
        found_branch.then_some(limit)
    }

    /// Reduce the device-local CKTterr limits already captured by an accepted
    /// classic-MOS candidate. Keeping this separate from candidate evaluation
    /// lets sequential workloads avoid proving LTE for candidates that the
    /// exact nonlinear residual subsequently rejects.
    #[inline]
    pub(super) fn classic_mos_gate_limit_from_cached_charges(
        &self,
        constants: &[crate::device::mosfet::ClassicMosTransientConstants],
        charges: &[MosfetGateCompanionCharges],
        caps: &[(Value, Value, Value)],
        history: &MosfetTransientHistory,
    ) -> Option<Value> {
        debug_assert_eq!(constants.len(), charges.len());
        debug_assert_eq!(constants.len(), caps.len());

        let mut limit = 2.0 * self.dt;
        let mut found_branch = false;

        // History is stored branch-major. Traverse it in that same order so
        // each CKTterr stream is contiguous and the accepted-state proof does
        // not rebuild three temporary branch arrays for every device.
        for idx in 0..constants.len() {
            let (cgs_ov, _, _) = constants[idx].overlap_capacitances();
            let capacitance = caps[idx].0 + history.capgs_prev_half[idx] + cgs_ov;
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }
            let (q_curr, cq_curr) = charges[idx][0];
            let Some(branch_limit) = self.limit(
                q_curr,
                history.qgs_prev[idx],
                history.qgs_prev_prev[idx],
                history.qgs_prev_prev_prev[idx],
                cq_curr,
                history.cqgs_prev[idx],
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        for idx in 0..constants.len() {
            let (_, cgd_ov, _) = constants[idx].overlap_capacitances();
            let capacitance = caps[idx].1 + history.capgd_prev_half[idx] + cgd_ov;
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }
            let (q_curr, cq_curr) = charges[idx][1];
            let Some(branch_limit) = self.limit(
                q_curr,
                history.qgd_prev[idx],
                history.qgd_prev_prev[idx],
                history.qgd_prev_prev_prev[idx],
                cq_curr,
                history.cqgd_prev[idx],
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        for idx in 0..constants.len() {
            let (_, _, cgb_ov) = constants[idx].overlap_capacitances();
            let capacitance = caps[idx].2 + history.capgb_prev_half[idx] + cgb_ov;
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }
            let (q_curr, cq_curr) = charges[idx][2];
            let Some(branch_limit) = self.limit(
                q_curr,
                history.qgb_prev[idx],
                history.qgb_prev_prev[idx],
                history.qgb_prev_prev_prev[idx],
                cq_curr,
                history.cqgb_prev[idx],
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        found_branch.then_some(limit)
    }
}

impl Engine {
    /// Whether ngspice's device-local CKTterr charge walks own transient
    /// timestep control for this integration front.
    ///
    /// Xyce accepted-solution LTE is deliberately exclusive: OneStep/Gear12
    /// returns only the weighted solution-correction norm, not a second
    /// compact-device charge norm.
    #[inline]
    pub(super) fn uses_ngspice_charge_truncation(lte_estimator: &LteEstimator) -> bool {
        !lte_estimator.uses_accepted_solution_reference()
    }

    #[inline]
    pub(super) fn ngspice_vbic_truncation_factor(method: IntegrationMethod, order: u8) -> Value {
        match order.max(1) {
            1 => 0.5,
            2 => match method {
                IntegrationMethod::Gear2 => 0.222_222_222_2,
                _ => 0.083_333_333_33,
            },
            _ => 0.083_333_333_33,
        }
    }

    #[inline]
    #[cfg(test)]
    pub(super) fn ngspice_charge_truncation_limit(
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        q_prev_prev_prev: Value,
        cq_curr: Value,
        cq_prev: Value,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        NgspiceChargeTruncationContext::new(
            dt,
            prev_dt,
            prev_prev_dt,
            method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?
        .limit(
            q_curr,
            q_prev,
            q_prev_prev,
            q_prev_prev_prev,
            cq_curr,
            cq_prev,
        )
    }

    #[inline]
    pub(super) fn capacitor_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
        mut accepted_states_out: Option<&mut Vec<CapacitorAcceptedState>>,
    ) -> Option<Value> {
        if let Some(states) = accepted_states_out.as_deref_mut() {
            states.clear();
            states.reserve(circuit.capacitors.stamps.len());
        }
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff =
            CompanionCoefficients::for_method_with_previous_step(effective_method, dt, prev_dt);
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            prev_dt,
            prev_prev_dt,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;
        let mut accepted_states_complete = true;

        for (idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit
                .capacitors
                .value_expressions
                .get(idx)
                .and_then(Option::as_ref)
                .is_some()
            {
                // Solution-dependent capacitors own a charge history that is
                // integrated from C(V) between accepted points. The legacy
                // static C*V truncation walk is not valid for that law; the
                // generic accepted-solution LTE estimator remains active.
                accepted_states_complete = false;
                continue;
            }
            let capacitance = circuit.capacitors.capacitances[idx];
            if !capacitance.is_finite() || capacitance <= 0.0 {
                accepted_states_complete = false;
                continue;
            }

            let voltage = Self::differential_voltage(candidate_solution, cap.pp.row, cap.nn.row);
            let q_curr = capacitance * voltage;
            let q_prev = capacitance * circuit.capacitors.v_prev[idx];
            let q_prev_prev = capacitance * circuit.capacitors.v_prev_prev[idx];
            let q_prev_prev_prev = capacitance * circuit.capacitors.v_prev_prev_prev[idx];
            let geq = coeff.capacitor_geq(capacitance, dt);
            let ieq = coeff.capacitor_ieq(
                capacitance,
                dt,
                circuit.capacitors.v_prev[idx],
                circuit.capacitors.v_prev_prev[idx],
                circuit.capacitors.i_prev[idx],
            );
            let cq_curr = geq * voltage - ieq;
            let cq_prev = circuit.capacitors.i_prev[idx];
            if let Some(states) = accepted_states_out.as_deref_mut() {
                let accepted_current =
                    if let Some(branch_ordinal) = circuit.capacitors.ic_branch_indices[idx] {
                        candidate_solution[circuit.num_nodes() + branch_ordinal - 1]
                    } else {
                        cq_curr
                    };
                states.push(CapacitorAcceptedState {
                    voltage,
                    current: accepted_current,
                });
            }

            let Some(branch_limit) = truncation.limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        if !accepted_states_complete && let Some(states) = accepted_states_out {
            states.clear();
        }

        found_branch.then_some(limit)
    }

    /// Choose a bounded parallel width only when the complete capacitor SoA
    /// is a fixed, positive charge law. This is a topology/invariant proof,
    /// not a numerical shortcut: mixed or malformed storage stays on the
    /// canonical serial path.
    #[cfg(feature = "parallel")]
    pub(super) fn capacitor_truncation_parallel_worker_count(
        &self,
        circuit: &crate::circuit::CircuitData,
    ) -> Option<usize> {
        let count = circuit.capacitors.stamps.len();
        if count <= CAPACITOR_TRUNCATION_ITEMS_PER_WORKER
            || count != circuit.capacitors.capacitances.len()
            || count != circuit.capacitors.value_expressions.len()
            || count != circuit.capacitors.ic_branch_indices.len()
            || circuit
                .capacitors
                .value_expressions
                .iter()
                .any(Option::is_some)
            || circuit
                .capacitors
                .capacitances
                .iter()
                .any(|capacitance| !capacitance.is_finite() || *capacitance <= 0.0)
        {
            return None;
        }

        let useful_workers = count.div_ceil(CAPACITOR_TRUNCATION_ITEMS_PER_WORKER);
        let worker_count = self
            .parallel_worker_count(count)
            .min(CAPACITOR_TRUNCATION_MAX_WORKERS)
            .min(useful_workers.max(1));
        (worker_count > 1).then_some(worker_count)
    }

    /// Deterministic parallel form of the ordinary-capacitor CKTterr walk.
    /// Every branch executes the exact scalar arithmetic used by the serial
    /// reference. The only reduction is `min` over finite positive limits, so
    /// task completion order cannot change the selected timestep.
    #[cfg(feature = "parallel")]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn capacitor_ngspice_truncation_limit_parallel(
        &self,
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
        worker_count: usize,
        accepted_states_out: &mut Vec<CapacitorAcceptedState>,
    ) -> Option<Value> {
        use rayon::prelude::*;

        accepted_states_out.clear();
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff =
            CompanionCoefficients::for_method_with_previous_step(effective_method, dt, prev_dt);
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            prev_dt,
            prev_prev_dt,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let count = circuit.capacitors.stamps.len();
        debug_assert!(worker_count > 1);
        debug_assert_eq!(count, circuit.capacitors.capacitances.len());
        debug_assert_eq!(count, circuit.capacitors.value_expressions.len());
        debug_assert_eq!(count, circuit.capacitors.ic_branch_indices.len());
        debug_assert!(
            circuit
                .capacitors
                .value_expressions
                .iter()
                .all(Option::is_none)
        );
        debug_assert!(
            circuit
                .capacitors
                .capacitances
                .iter()
                .all(|capacitance| capacitance.is_finite() && *capacitance > 0.0)
        );

        accepted_states_out.resize(count, CapacitorAcceptedState::default());
        let chunk_size = count.div_ceil(worker_count).max(1);
        let num_nodes = circuit.num_nodes();
        // Capture only the immutable capacitor SoA slices. `CircuitData` also
        // owns unrelated interior-mutable devices, so borrowing the whole
        // circuit would unnecessarily make this independent kernel non-Sync.
        let stamps = circuit.capacitors.stamps.as_slice();
        let capacitances = circuit.capacitors.capacitances.as_slice();
        let v_prev = circuit.capacitors.v_prev.as_slice();
        let v_prev_prev = circuit.capacitors.v_prev_prev.as_slice();
        let v_prev_prev_prev = circuit.capacitors.v_prev_prev_prev.as_slice();
        let i_prev = circuit.capacitors.i_prev.as_slice();
        let ic_branch_indices = circuit.capacitors.ic_branch_indices.as_slice();
        // Reuse the engine's existing bounded per-device pool. These kernels
        // execute at disjoint phases, so sharing it preserves the global
        // no-oversubscription contract without creating another worker set.
        let parallel_limit = self.install_classic_mos_parallel(|| {
            accepted_states_out
                .par_chunks_mut(chunk_size)
                .enumerate()
                .map(|(chunk_index, states)| {
                    let start = chunk_index * chunk_size;
                    let mut chunk_limit = None;
                    for (offset, state) in states.iter_mut().enumerate() {
                        let idx = start + offset;
                        let cap = stamps[idx];
                        let capacitance = capacitances[idx];
                        let voltage =
                            Self::differential_voltage(candidate_solution, cap.pp.row, cap.nn.row);
                        let q_curr = capacitance * voltage;
                        let q_prev = capacitance * v_prev[idx];
                        let q_prev_prev = capacitance * v_prev_prev[idx];
                        let q_prev_prev_prev = capacitance * v_prev_prev_prev[idx];
                        let geq = coeff.capacitor_geq(capacitance, dt);
                        let ieq = coeff.capacitor_ieq(
                            capacitance,
                            dt,
                            v_prev[idx],
                            v_prev_prev[idx],
                            i_prev[idx],
                        );
                        let cq_curr = geq * voltage - ieq;
                        let cq_prev = i_prev[idx];
                        let accepted_current = if let Some(branch_ordinal) = ic_branch_indices[idx]
                        {
                            candidate_solution[num_nodes + branch_ordinal - 1]
                        } else {
                            cq_curr
                        };
                        *state = CapacitorAcceptedState {
                            voltage,
                            current: accepted_current,
                        };

                        if let Some(branch_limit) = truncation.limit(
                            q_curr,
                            q_prev,
                            q_prev_prev,
                            q_prev_prev_prev,
                            cq_curr,
                            cq_prev,
                        ) {
                            chunk_limit =
                                Self::min_truncation_limit(chunk_limit, Some(branch_limit));
                        }
                    }
                    chunk_limit
                })
                .reduce(|| None, Self::min_truncation_limit)
        });

        match parallel_limit {
            Ok(limit) => limit.map(|limit| (2.0 * dt).min(limit)),
            Err(_) => Self::capacitor_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                prev_dt,
                prev_prev_dt,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
                Some(accepted_states_out),
            ),
        }
    }

    #[inline]
    pub(super) fn vbic_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let lte_debug = lte_debug_enabled();
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.vbic_mna_promoted() {
                continue;
            }

            // Promoted VBIC: the candidate solution carries the internal node
            // voltages, so the charges evaluate directly at the candidate
            // bias (ngspice VBICtrunc CKTterr over the charge states).
            let (branches, _, _) = bjt.vbic_mna_charge_state_at_solution(candidate_solution);

            for branch_idx in 0..BJT_VBIC_TRUNCATION_BRANCH_COUNT {
                let q_curr = branches[branch_idx].charge;
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr =
                    Self::jfet_companion_ccap(&coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);

                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                if branch_limit < dt && lte_debug {
                    log::warn!(
                        "BJT LTE bind: dev={idx} branch={branch_idx} q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn legacy_bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let lte_debug = lte_debug_enabled();
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(candidate_solution, bjt.node_collector);
            let vb = Self::node_voltage(candidate_solution, bjt.node_base);
            let ve = Self::node_voltage(candidate_solution, bjt.node_emitter);
            let vs = Self::node_voltage(candidate_solution, bjt.node_substrate);
            let candidate_external = [vc, vb, ve, vs];
            let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
            let snapshot_reuse_reltol = reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
            let snapshot = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                candidate_external,
                &coeff,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                history.dynamic_internal_prev.get(idx),
                history.dynamic_internal_prev_prev.get(idx),
                history.dynamic_linear_prev.get(idx),
                history.dynamic_linear_prev_prev.get(idx),
                history.accepted_dt_prev,
                vbic_snapshot_cache.get(idx).copied().flatten(),
                VbicCachedSnapshotReuse::SeedOnly,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )?;

            // Match ngspice's legacy BJT CKTterr coverage: qbe, qbc, qsub,
            // and true qbcx only when an internal collector-resistance branch
            // exists. Branch 3 is the XCJC external split charge in the
            // legacy backend, so it is integrated but not used as a separate
            // truncation limiter.
            for branch_idx in [
                BJT_QBE_BRANCH_INDEX,
                BJT_QBC_BRANCH_INDEX,
                BJT_QBCP_BRANCH_INDEX,
            ] {
                let branch = snapshot.branches[branch_idx];
                if !branch.is_active() {
                    continue;
                }
                let q_curr = branch.charge;
                if !q_curr.is_finite() {
                    continue;
                }
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr =
                    Self::jfet_companion_ccap(&coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);

                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                if branch_limit < dt && lte_debug {
                    log::warn!(
                        "legacy BJT LTE bind: dev={idx} branch={branch_idx} q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        if let Some(vbic_limit) = Self::vbic_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(vbic_limit);
            found_branch = true;
        }

        if let Some(legacy_limit) = Self::legacy_bjt_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(legacy_limit);
            found_branch = true;
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn jfet_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &JfetTransientHistory,
        suppress_gate_charge: bool,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, candidate_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, candidate_solution);
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

            for (
                is_gate_charge,
                capacitance,
                voltage,
                voltage_prev,
                q_curr_exact,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    true,
                    cgs,
                    vgs_charge,
                    history.vgs_prev[idx],
                    jfet2_charge.map(|charge| charge.qgs),
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    true,
                    cgd,
                    vgd_charge,
                    history.vgd_prev[idx],
                    jfet2_charge.map(|charge| charge.qgd),
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    false,
                    cds,
                    vgs_charge - vgd_charge,
                    history.vds_prev[idx],
                    None,
                    history.qds_prev[idx],
                    history.qds_prev_prev[idx],
                    history.qds_prev_prev_prev[idx],
                    history.cqds_prev[idx],
                ),
            ] {
                if suppress_gate_charge && is_gate_charge {
                    continue;
                }

                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = if let Some(q_exact) = q_curr_exact {
                    Self::nonlinear_charge_companion_terms(
                        &coeff,
                        dt,
                        capacitance,
                        voltage,
                        q_exact,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                    )
                } else {
                    Self::jfet_companion_terms(
                        &coeff,
                        dt,
                        capacitance,
                        voltage,
                        voltage_prev,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                    )
                };
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// ngspice `DIOtrunc` (CKTterr on the `DIOcapCharge` state): the diode
    /// junction depletion+diffusion charge drives the timestep through the
    /// same divided-difference truncation law as the other junction devices.
    #[inline]
    pub(super) fn diode_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &DiodeTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, diode) in circuit.diodes.devices.iter().enumerate() {
            let vd = Self::differential_voltage(
                candidate_solution,
                diode.node_anode,
                diode.node_cathode,
            );
            let (qd, capd) = diode.junction_charge_and_capacitance(vd);
            if !capd.is_finite() || capd <= 0.0 {
                continue;
            }

            let (_geq, _ieq, q_curr, cq_curr) = Self::nonlinear_charge_companion_terms(
                &coeff,
                dt,
                capd,
                vd,
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            );
            let Some(branch_limit) = truncation.limit(
                q_curr,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.qd_prev_prev_prev[idx],
                cq_curr,
                history.cqd_prev[idx],
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn mosfet_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
        caps_cache: Option<(&mut Vec<(Value, Value, Value)>, bool)>,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }
        let lte_debug = lte_debug_enabled();

        // Exact-residual assembly may already have evaluated the Meyer halves
        // on this candidate. Reuse them when available; otherwise capture the
        // truncation walk for the accepted-history rotation.
        let mut caps_cache = caps_cache.map(|(cache, valid)| {
            let reuse = valid && cache.len() == circuit.mosfets.devices.len();
            if !reuse {
                cache.clear();
                cache.reserve(circuit.mosfets.devices.len());
            }
            (cache, reuse)
        });

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (cgs_half, cgd_half, cgb_half) = match caps_cache.as_ref() {
                Some((cache, true)) => cache[idx],
                _ => {
                    let (vgs_eval, vds_eval, vbs_eval) =
                        mos.eval_branch_voltages_at(candidate_solution);
                    mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval)
                }
            };
            if let Some((cache, false)) = caps_cache.as_mut() {
                cache.push((cgs_half, cgd_half, cgb_half));
            }
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(candidate_solution);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();

            for (
                _branch,
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    "qgs",
                    cgs_half + history.capgs_prev_half[idx] + cgs_ov,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    "qgd",
                    cgd_half + history.capgd_prev_half[idx] + cgd_ov,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    "qgb",
                    cgb_half + history.capgb_prev_half[idx] + cgb_ov,
                    vgb,
                    history.vgb_prev[idx],
                    history.qgb_prev[idx],
                    history.qgb_prev_prev[idx],
                    history.qgb_prev_prev_prev[idx],
                    history.cqgb_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    &coeff,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                if branch_limit < dt && lte_debug {
                    log::warn!(
                        "MOS LTE bind: dev={idx} branch={_branch} cap={capacitance:.4e} v=[{voltage:.4e},{voltage_prev:.4e}] q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn vdmos_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &VdmosTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, vdmos) in circuit.vdmoses.devices.iter().enumerate() {
            let (vgs, vgd, vgb, vds) =
                vdmos.transient_charge_branch_voltages_at(candidate_solution);
            let vd1 = vdmos.d1_charge_branch_voltage_at(candidate_solution);
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(candidate_solution);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (qbs, cbs) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
            let (qbd, cbd) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
            let (qd1, cd1) = vdmos.d1_charge_and_capacitance_at(vd1);

            for (
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    cgs,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    cgd,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    cgb,
                    vgb,
                    history.vgb_prev[idx],
                    history.qgb_prev[idx],
                    history.qgb_prev_prev[idx],
                    history.qgb_prev_prev_prev[idx],
                    history.cqgb_prev[idx],
                ),
                (
                    cds,
                    vds,
                    history.vds_prev[idx],
                    history.qds_prev[idx],
                    history.qds_prev_prev[idx],
                    history.qds_prev_prev_prev[idx],
                    history.cqds_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    &coeff,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }

            for (
                capacitance,
                voltage,
                q_curr_exact,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    cbs,
                    vbs,
                    qbs,
                    history.qbs_prev[idx],
                    history.qbs_prev_prev[idx],
                    history.qbs_prev_prev_prev[idx],
                    history.cqbs_prev[idx],
                ),
                (
                    cbd,
                    vbd,
                    qbd,
                    history.qbd_prev[idx],
                    history.qbd_prev_prev[idx],
                    history.qbd_prev_prev_prev[idx],
                    history.cqbd_prev[idx],
                ),
                (
                    cd1,
                    vd1,
                    qd1,
                    history.qd1_prev[idx],
                    history.qd1_prev_prev[idx],
                    history.qd1_prev_prev_prev[idx],
                    history.cqd1_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::nonlinear_charge_companion_terms(
                    &coeff,
                    dt,
                    capacitance,
                    voltage,
                    q_curr_exact,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// LTE truncation limit for the BSIMSOI (level 56) charge states.
    ///
    /// Mirrors [`Self::mosfet_ngspice_truncation_limit`] but over the three
    /// B3SOI states that ngspice's `B3SOIDDtrunc` feeds to `CKTterr`: `qb`,
    /// `qg`, and `qd`. `qe` and DD's thermal `qth` are still integrated by the
    /// transient companion/history path, but they do not independently reduce
    /// the accepted timestep. Returns the tightest per-charge step bound, or
    /// `None` when no SOI charge is active.
    pub(super) fn b3soi_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        // The history is indexed DD devices first, then FD, then PD,
        // matching the companion stamp/commit walks.
        let mut device_charges: Vec<Option<(Value, Value, Value)>> = Vec::with_capacity(
            circuit.b3soi.devices.len()
                + circuit.b3soi_fd.devices.len()
                + circuit.b3soi_pd.devices.len(),
        );
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                device_charges.push(None);
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push(Some((c.qg, c.qb, c.qd)));
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                device_charges.push(None);
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push(Some((c.qg, c.qb, c.qd)));
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                device_charges.push(None);
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push(Some((c.qg, c.qb, c.qd)));
        }

        for (idx, charges) in device_charges.into_iter().enumerate() {
            let Some((qg, qb, qd)) = charges else {
                continue;
            };
            for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                (
                    qb,
                    history.qb_prev[idx],
                    history.qb_prev_prev[idx],
                    history.qb_prev_prev_prev[idx],
                    history.cqb_prev[idx],
                ),
                (
                    qg,
                    history.qg_prev[idx],
                    history.qg_prev_prev[idx],
                    history.qg_prev_prev_prev[idx],
                    history.cqg_prev[idx],
                ),
                (
                    qd,
                    history.qd_prev[idx],
                    history.qd_prev_prev[idx],
                    history.qd_prev_prev_prev[idx],
                    history.cqd_prev[idx],
                ),
            ] {
                // Integrated charge current at the candidate point.
                let cq_curr =
                    Self::jfet_companion_ccap(&coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// LTE truncation limit for the BSIM3v3.3 (level 8/49) charge states.
    ///
    /// Mirrors [`Self::b3soi_ngspice_truncation_limit`] over the three
    /// composite BSIM3 node charges (`qg`/`qb`/`qd`, junction depletion
    /// charges folded in) — exactly the states `b3trunc.c` feeds `CKTterr`.
    /// Returns the tightest per-charge step bound, or `None` when no charge
    /// is active.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn bsim3_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (c, _mode) = dev.charge_at(candidate_solution);
            for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                (
                    c.qg_state(),
                    history.qg_prev[idx],
                    history.qg_prev_prev[idx],
                    history.qg_prev_prev_prev[idx],
                    history.cqg_prev[idx],
                ),
                (
                    c.qb_state(),
                    history.qb_prev[idx],
                    history.qb_prev_prev[idx],
                    history.qb_prev_prev_prev[idx],
                    history.cqb_prev[idx],
                ),
                (
                    c.qd_state(),
                    history.qd_prev[idx],
                    history.qd_prev_prev[idx],
                    history.qd_prev_prev_prev[idx],
                    history.cqd_prev[idx],
                ),
            ] {
                // Integrated charge current at the candidate point.
                let cq_curr =
                    Self::jfet_companion_ccap(&coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// LTE truncation limit for the BSIM4 v4.8 (level 14/54) charge states.
    ///
    /// Mirrors [`Self::bsim3_ngspice_truncation_limit`] over BSIM4
    /// `qg`/`qb`/`qd`; when `rbodyMod > 0`, `qb` is the intrinsic bulk charge
    /// and ngspice also runs `CKTterr` for separate `qbs`/`qbd` junction
    /// states (b4trunc.c). Returns the tightest per-charge step bound, or
    /// `None` when no charge is active.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn bsim4_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, dev) in circuit.bsim4v8.devices.iter().enumerate() {
            let (c, _mode) = dev.charge_at(candidate_solution);
            let rbody = dev.rbody_enabled();
            let (qg, _qgmid, qb, qd, _qbs, _qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&c, candidate_solution)
            } else {
                (
                    c.qg_state(),
                    c.qgmid_state(),
                    c.qb_state_for_rbody(rbody),
                    c.qd_state(),
                    c.qbs,
                    c.qbd,
                )
            };
            let mut consider_charge = |q_curr: Value,
                                       q_prev: Value,
                                       q_prev_prev: Value,
                                       q_prev_prev_prev: Value,
                                       cq_prev: Value| {
                // Integrated charge current at the candidate point.
                let cq_curr =
                    Self::jfet_companion_ccap(&coeff, dt, q_curr, q_prev, q_prev_prev, cq_prev);
                let Some(branch_limit) = truncation.limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    return;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            };

            for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                (
                    qg,
                    history.qg_prev[idx],
                    history.qg_prev_prev[idx],
                    history.qg_prev_prev_prev[idx],
                    history.cqg_prev[idx],
                ),
                (
                    qb,
                    history.qb_prev[idx],
                    history.qb_prev_prev[idx],
                    history.qb_prev_prev_prev[idx],
                    history.cqb_prev[idx],
                ),
                (
                    qd,
                    history.qd_prev[idx],
                    history.qd_prev_prev[idx],
                    history.qd_prev_prev_prev[idx],
                    history.cqd_prev[idx],
                ),
            ] {
                consider_charge(q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev);
            }

            if dev.core.model.rgate_mod == 3 {
                consider_charge(
                    c.qgmid_state(),
                    history.qgmid_prev[idx],
                    history.qgmid_prev_prev[idx],
                    history.qgmid_prev_prev_prev[idx],
                    history.cqgmid_prev[idx],
                );
            }

            if rbody {
                for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                    (
                        c.qbs,
                        history.qbs_prev[idx],
                        history.qbs_prev_prev[idx],
                        history.qbs_prev_prev_prev[idx],
                        history.cqbs_prev[idx],
                    ),
                    (
                        c.qbd,
                        history.qbd_prev[idx],
                        history.qbd_prev_prev[idx],
                        history.qbd_prev_prev_prev[idx],
                        history.cqbd_prev[idx],
                    ),
                ] {
                    consider_charge(q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev);
                }
            }

            if dev.uses_trnqs() {
                consider_charge(
                    dev.trnqs_qcdump_state(candidate_solution),
                    history.qcdump_prev[idx],
                    history.qcdump_prev_prev[idx],
                    history.qcdump_prev_prev_prev[idx],
                    history.cqcdump_prev[idx],
                );
            }
        }

        found_branch.then_some(limit)
    }

    /// LTE truncation limit for native EKV 2.6 intrinsic terminal charges.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn ekv26_ngspice_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Ekv26TransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method_with_previous_step(
            effective_method,
            dt,
            history.accepted_dt_prev,
        );
        let truncation = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            effective_method,
            trap_order,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )?;
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, dev) in circuit.ekv26s.devices.iter().enumerate() {
            let q_curr = dev.dynamic_charge_vector_at_solution(candidate_solution);
            for row in 0..EKV26_DYNAMIC_CHARGE_COUNT {
                let q_prev = history.q_prev[idx][row];
                let q_prev_prev = history.q_prev_prev[idx][row];
                let q_prev_prev_prev = history.q_prev_prev_prev[idx][row];
                let cq_prev = history.cq_prev[idx][row];
                let cq_curr = Self::jfet_companion_ccap(
                    &coeff,
                    dt,
                    q_curr[row],
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = truncation.limit(
                    q_curr[row],
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// Prepare the unique, non-excluded solution indices used by the
    /// nonlinear terminal-activity guard. Circuit topology is immutable
    /// during an analysis, so the accepted-step loop should not rediscover
    /// these nodes through the large device-instance arrays.
    pub(super) fn nonlinear_terminal_solution_indices(
        circuit: &crate::circuit::CircuitData,
        excluded_solution_indices: &[bool],
    ) -> Vec<usize> {
        let mut included = vec![false; excluded_solution_indices.len()];
        let mut include = |node: usize| {
            let Some(index) = node.checked_sub(1) else {
                return;
            };
            if !excluded_solution_indices
                .get(index)
                .copied()
                .unwrap_or(true)
                && let Some(slot) = included.get_mut(index)
            {
                *slot = true;
            }
        };

        for mos in &circuit.mosfets.devices {
            include(mos.node_drain);
            include(mos.node_gate);
            include(mos.node_source);
            include(mos.node_bulk);
        }
        for bjt in &circuit.bjts.devices {
            include(bjt.node_collector);
            include(bjt.node_base);
            include(bjt.node_emitter);
            include(bjt.node_substrate);
        }
        for jfet in &circuit.jfets {
            include(jfet.drain);
            include(jfet.gate);
            include(jfet.source);
        }

        included
            .into_iter()
            .enumerate()
            .filter_map(|(index, include)| include.then_some(index))
            .collect()
    }

    /// Signal-activity step limit: rescale the candidate step so that no
    /// solved nonlinear-device terminal voltage moves more than `bound` volts
    /// in one step.
    ///
    /// Complements the polynomial charge LTE, which estimates error from
    /// divided differences of sampled charges and is therefore blind to
    /// curvature lying entirely between samples (see
    /// [`crate::constants::DEVICE_ACTIVITY_STEP_BOUND`]). Returns the
    /// proportionally reduced step when the bound is exceeded, `None` when
    /// the candidate respects it.
    pub(super) fn nonlinear_terminal_activity_limit(
        terminal_solution_indices: &[usize],
        accepted_solution: &[Value],
        candidate_solution: &[Value],
        dt: Value,
        bound: Value,
    ) -> Option<Value> {
        if !(bound.is_finite() && bound > 0.0 && dt.is_finite() && dt > 0.0) {
            return None;
        }

        let mut max_delta: Value = 0.0;
        for &solution_index in terminal_solution_indices {
            let accepted = accepted_solution
                .get(solution_index)
                .copied()
                .unwrap_or(0.0);
            let candidate = candidate_solution
                .get(solution_index)
                .copied()
                .unwrap_or(0.0);
            let delta = (candidate - accepted).abs();
            if delta.is_finite() && delta > max_delta {
                max_delta = delta;
            }
        }

        (max_delta > bound).then(|| dt * bound / max_delta)
    }

    #[inline]
    pub(super) fn min_truncation_limit(
        first: Option<Value>,
        second: Option<Value>,
    ) -> Option<Value> {
        match (first, second) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    #[inline]
    pub(super) fn ltra_candidate_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        candidate_time: Value,
    ) -> Option<Value> {
        let mut limit = Value::INFINITY;
        let mut found_line = false;

        for tl in &circuit.tlines {
            let Some((br1, br2)) = tl.ltra_branch_matrix_indices() else {
                continue;
            };

            let v1 = Self::differential_voltage(candidate_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(candidate_solution, tl.node2_pos, tl.node2_neg);
            let i1 = candidate_solution.get(br1 - 1).copied().unwrap_or(0.0);
            let i2 = candidate_solution.get(br2 - 1).copied().unwrap_or(0.0);
            let Some(line_limit) = tl
                .ltra_candidate_truncation_limit(candidate_time, v1, i1, v2, i2)
                .filter(|line_limit| line_limit.is_finite() && *line_limit > 0.0)
            else {
                continue;
            };

            limit = limit.min(line_limit);
            found_line = true;
        }

        found_line.then_some(limit)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn ngspice_device_truncation_limit(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        bjt_history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        jfet_history: &JfetTransientHistory,
        diode_history: &DiodeTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        vdmos_history: &VdmosTransientHistory,
        ekv26_history: &Ekv26TransientHistory,
        suppress_gate_charge: bool,
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let capacitor_limit = if !circuit.capacitors.is_empty() {
            Self::capacitor_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history.accepted_dt_prev,
                mosfet_history.accepted_dt_prev_prev,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
                None,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let bjt_limit = if !circuit.bjts.devices.is_empty() {
            Self::bjt_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                bjt_history,
                vbic_snapshot_cache,
                voltage_abstol,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let jfet_limit = if !circuit.jfets.is_empty() {
            Self::jfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                jfet_history,
                suppress_gate_charge,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let diode_limit = if !circuit.diodes.is_empty() {
            Self::diode_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                diode_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let mosfet_limit = if !suppress_gate_charge && !circuit.mosfets.is_empty() {
            Self::mosfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
                None,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let vdmos_limit = if !circuit.vdmoses.is_empty() {
            Self::vdmos_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                vdmos_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let ekv26_limit = if !circuit.ekv26s.is_empty() {
            Self::ekv26_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                ekv26_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        Self::min_truncation_limit(
            Self::min_truncation_limit(
                Self::min_truncation_limit(
                    Self::min_truncation_limit(
                        Self::min_truncation_limit(capacitor_limit, bjt_limit),
                        jfet_limit,
                    ),
                    diode_limit,
                ),
                mosfet_limit,
            ),
            Self::min_truncation_limit(vdmos_limit, ekv26_limit),
        )
    }

    #[inline]
    pub(super) fn should_retry_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit <= 0.9 * dt
    }

    #[inline]
    pub(super) fn should_promote_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit > 1.05 * dt
    }

    #[inline]
    pub(super) fn next_trapezoidal_order_after_accepted_step(
        current_order: u8,
        hit_breakpoint: bool,
        should_promote: bool,
    ) -> u8 {
        if hit_breakpoint {
            1
        } else if current_order >= 2 || should_promote {
            2
        } else {
            1
        }
    }

    #[inline]
    pub(super) fn trapezoidal_order_after_timestep_control_reject(current_order: u8) -> u8 {
        current_order.max(1)
    }

    #[inline]
    pub(super) fn bjt_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::CircuitData,
        bjt_truncation_limit: Option<Value>,
    ) -> bool {
        bjt_truncation_limit.is_some()
            && !circuit.bjts.devices.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.jfets.is_empty()
            && circuit.vdmoses.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && circuit.xyce_core_groups.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn jfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::CircuitData,
        jfet_truncation_limit: Option<Value>,
    ) -> bool {
        jfet_truncation_limit.is_some()
            && !circuit.jfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.vdmoses.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && circuit.xyce_core_groups.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn mosfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::CircuitData,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        mosfet_truncation_limit.is_some()
            && !circuit.mosfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.jfets.is_empty()
            && circuit.vdmoses.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && circuit.xyce_core_groups.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn ngspice_device_truncation_covers_transient_lte(
        circuit: &crate::circuit::CircuitData,
        capacitor_truncation_limit: Option<Value>,
        bjt_truncation_limit: Option<Value>,
        jfet_truncation_limit: Option<Value>,
        diode_truncation_limit: Option<Value>,
        mosfet_truncation_limit: Option<Value>,
        vdmos_truncation_limit: Option<Value>,
    ) -> bool {
        if circuit.has_xspice_devices()
            || !circuit.inductors.is_empty()
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.jiles_atherton_inductors.is_empty()
            || !circuit.xyce_core_groups.is_empty()
        {
            return false;
        }

        let capacitor_controlled =
            circuit.capacitors.is_empty() || capacitor_truncation_limit.is_some();
        let bjt_controlled = circuit.bjts.devices.is_empty() || bjt_truncation_limit.is_some();
        let jfet_controlled = circuit.jfets.is_empty() || jfet_truncation_limit.is_some();
        // Zero-charge diodes (CJO=0, TT=0) report no truncation limit; the
        // generic node-voltage estimator stays in charge for those decks.
        let diode_controlled = circuit.diodes.is_empty() || diode_truncation_limit.is_some();
        let mosfet_controlled = circuit.mosfets.is_empty() || mosfet_truncation_limit.is_some();
        let vdmos_controlled = circuit.vdmoses.is_empty() || vdmos_truncation_limit.is_some();

        capacitor_controlled
            && bjt_controlled
            && jfet_controlled
            && diode_controlled
            && mosfet_controlled
            && vdmos_controlled
    }

    #[inline]
    pub(super) fn estimate_transient_lte(
        circuit: &crate::circuit::CircuitData,
        candidate_solution: &[Value],
        predicted_solution: Option<&[Value]>,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        is_strictly_linear_transient: bool,
        voltage_lte_estimator: &LteEstimator,
        voltage_lte_excluded_nodes: &[usize],
        xyce_lte_excluded_indices: &[usize],
    ) -> (Value, bool) {
        if is_strictly_linear_transient && !voltage_lte_estimator.uses_accepted_solution_reference()
        {
            return (0.0, true);
        }

        let uses_xyce_solution_domain = voltage_lte_estimator.uses_accepted_solution_reference();
        if !uses_xyce_solution_domain {
            return voltage_lte_estimator.estimate_prefix_excluding(
                candidate_solution,
                circuit.num_nodes(),
                dt,
                voltage_lte_excluded_nodes,
            );
        }
        let error_domain_len = candidate_solution.len();
        let excluded = xyce_lte_excluded_indices;
        match predicted_solution {
            Some(predicted) => voltage_lte_estimator
                .estimate_correction_prefix_excluding_for_integration(
                    candidate_solution,
                    predicted,
                    error_domain_len,
                    dt,
                    excluded,
                    method,
                    trap_order,
                ),
            None => voltage_lte_estimator.estimate_prefix_excluding_for_integration(
                candidate_solution,
                error_domain_len,
                dt,
                excluded,
                method,
                trap_order,
            ),
        }
    }

    #[inline]
    pub(super) fn trapezoidal_order_trial_timestep_limit(
        circuit: &crate::circuit::CircuitData,
        accepted_solution: &[Value],
        method: IntegrationMethod,
        dt: Value,
        is_strictly_linear_transient: bool,
        history: &BjtTransientHistory,
        jfet_history: &JfetTransientHistory,
        diode_history: &DiodeTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        vdmos_history: &VdmosTransientHistory,
        ekv26_history: &Ekv26TransientHistory,
        voltage_lte_estimator: &LteEstimator,
        voltage_lte_excluded_nodes: &[usize],
        xyce_lte_excluded_indices: &[usize],
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<TrapezoidalOrderTrial> {
        if !matches!(
            method,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
        ) {
            return None;
        }
        if !(dt.is_finite() && dt > 0.0) {
            return None;
        }
        // Match ngspice startup behavior: keep order-1 through the first accepted
        // transient step, then run the order-2 trial truncation check. The trial
        // limit still caps the next step when it is not large enough to promote.
        if !(history.accepted_dt_prev.is_finite() && history.accepted_dt_prev > 0.0) {
            return None;
        }

        if !voltage_lte_estimator.uses_accepted_solution_reference()
            && let Some(limit) = Self::ngspice_device_truncation_limit(
                circuit,
                accepted_solution,
                method,
                2,
                dt,
                history,
                vbic_snapshot_cache,
                jfet_history,
                diode_history,
                mosfet_history,
                vdmos_history,
                ekv26_history,
                false,
                voltage_abstol,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
        {
            return Some(TrapezoidalOrderTrial {
                limit,
                promote: Self::should_promote_ngspice_charge_truncation(limit, dt),
            });
        }

        let (candidate_lte, accept) = Self::estimate_transient_lte(
            circuit,
            accepted_solution,
            None,
            dt,
            method,
            2,
            is_strictly_linear_transient,
            voltage_lte_estimator,
            voltage_lte_excluded_nodes,
            xyce_lte_excluded_indices,
        );
        if !accept {
            return None;
        }

        let candidate_scale = if is_strictly_linear_transient
            && !voltage_lte_estimator.uses_accepted_solution_reference()
        {
            1.0
        } else if voltage_lte_estimator.uses_accepted_solution_reference() {
            voltage_lte_estimator.recommend_scale_for_integration(candidate_lte, method, 2)
        } else {
            voltage_lte_estimator.recommend_scale(candidate_lte)
        };
        if candidate_scale >= 0.95 {
            Some(TrapezoidalOrderTrial {
                limit: Value::INFINITY,
                promote: true,
            })
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn lu_decompose_small_dense_real<const N: usize>(
        matrix: &[[Value; N]; N],
        dim: usize,
    ) -> Option<([[Value; N]; N], [usize; N])> {
        if dim == 0 {
            let mut pivots = [0usize; N];
            for (idx, pivot) in pivots.iter_mut().enumerate() {
                *pivot = idx;
            }
            return Some((*matrix, pivots));
        }

        let mut lu = *matrix;
        let mut pivots = [0usize; N];
        for (idx, pivot) in pivots.iter_mut().enumerate() {
            *pivot = idx;
        }

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = lu[pivot][pivot].abs();
            for row in (pivot + 1)..dim {
                let value = lu[row][pivot].abs();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                lu.swap(pivot, best);
                pivots.swap(pivot, best);
            }

            let pivot_value = lu[pivot][pivot];
            for row in (pivot + 1)..dim {
                lu[row][pivot] /= pivot_value;
                let factor = lu[row][pivot];
                for col in (pivot + 1)..dim {
                    lu[row][col] -= factor * lu[pivot][col];
                }
            }
        }

        Some((lu, pivots))
    }

    #[inline]
    pub(super) fn lu_solve_small_dense_real<const N: usize>(
        lu: &[[Value; N]; N],
        pivots: &[usize; N],
        rhs: &[Value; N],
        dim: usize,
    ) -> Option<[Value; N]> {
        if dim == 0 {
            return Some([0.0; N]);
        }

        let mut x = [0.0; N];
        for row in 0..dim {
            x[row] = rhs[pivots[row]];
            for col in 0..row {
                x[row] -= lu[row][col] * x[col];
            }
        }

        for row in (0..dim).rev() {
            for col in (row + 1)..dim {
                x[row] -= lu[row][col] * x[col];
            }
            let diag = lu[row][row];
            if diag.abs() < 1e-18 {
                return None;
            }
            x[row] /= diag;
        }

        Some(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Netlist;
    use crate::numerics::integration::TransientLteReference;

    #[allow(clippy::too_many_arguments)]
    fn reference_charge_truncation_limit(
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        q_prev_prev_prev: Value,
        cq_curr: Value,
        cq_prev: Value,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if !dt.is_finite() || dt <= 0.0 {
            return None;
        }
        let mut order = trap_order.clamp(1, 2);
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }
        if order >= 2 && (!prev_prev_dt.is_finite() || prev_prev_dt <= 0.0) {
            order = 1;
        }

        let volttol = current_abstol + reltol * cq_curr.abs().max(cq_prev.abs());
        let chargetol = reltol * q_curr.abs().max(q_prev.abs()).max(charge_abstol) / dt;
        let tol = volttol.max(chargetol);
        if !tol.is_finite() || tol <= 0.0 {
            return None;
        }

        let mut diff = [q_curr, q_prev, q_prev_prev, q_prev_prev_prev];
        let delta_old = [dt, prev_dt, prev_prev_dt];
        let mut deltmp = delta_old;
        let mut j = usize::from(order);
        loop {
            for i in 0..=j {
                let denom = deltmp[i];
                if !denom.is_finite() || denom <= 0.0 {
                    return None;
                }
                diff[i] = (diff[i] - diff[i + 1]) / denom;
            }
            if j == 0 {
                break;
            }
            j -= 1;
            for i in 0..=j {
                deltmp[i] = deltmp[i + 1] + delta_old[i];
            }
        }

        let factor = Engine::ngspice_vbic_truncation_factor(method, order);
        let denom = current_abstol.max(factor * diff[0].abs());
        if !denom.is_finite() || denom <= 0.0 || !trtol.is_finite() || trtol <= 0.0 {
            return None;
        }
        let mut limit = trtol * tol / denom;
        if order >= 2 {
            limit = limit.sqrt();
        }
        (limit.is_finite() && limit > 0.0).then_some(limit)
    }

    #[test]
    fn precomputed_charge_truncation_matches_reference_bit_exactly() {
        let methods = [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ];
        let timesteps = [
            (1.0e-9, 0.8e-9, 1.2e-9),
            (1.0e-30, 2.0e-30, 3.0e-30),
            (1.0e9, 2.0e9, 3.0e9),
            (1.0e-9, 2.0e-9, Value::NAN),
        ];
        let states = [
            (2.1e-15, 1.8e-15, 1.1e-15, 0.4e-15, 2.2e-6, 1.7e-6),
            (-3.0e-12, -2.7e-12, -2.0e-12, -1.2e-12, -4.0e-3, -3.5e-3),
            (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        ];

        for method in methods {
            for trap_order in 0..=3 {
                for (dt, prev_dt, prev_prev_dt) in timesteps {
                    for (q0, q1, q2, q3, cq0, cq1) in states {
                        let expected = reference_charge_truncation_limit(
                            q0,
                            q1,
                            q2,
                            q3,
                            cq0,
                            cq1,
                            dt,
                            prev_dt,
                            prev_prev_dt,
                            method,
                            trap_order,
                            1.0e-3,
                            1.0e-12,
                            1.0e-14,
                            7.0,
                        );
                        let actual = Engine::ngspice_charge_truncation_limit(
                            q0,
                            q1,
                            q2,
                            q3,
                            cq0,
                            cq1,
                            dt,
                            prev_dt,
                            prev_prev_dt,
                            method,
                            trap_order,
                            1.0e-3,
                            1.0e-12,
                            1.0e-14,
                            7.0,
                        );
                        assert_eq!(
                            actual.map(Value::to_bits),
                            expected.map(Value::to_bits),
                            "method={method:?} order={trap_order} dts=({dt},{prev_dt},{prev_prev_dt}) state=({q0},{q1},{q2},{q3},{cq0},{cq1})",
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn accepted_solution_lte_excludes_ngspice_device_charge_truncation() {
        let xyce = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        let native = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::PredictorLocal,
        );

        assert!(!Engine::uses_ngspice_charge_truncation(&xyce));
        assert!(Engine::uses_ngspice_charge_truncation(&native));
    }

    #[test]
    fn capacitor_truncation_exports_canonical_accepted_state_and_fails_closed() {
        let mut circuit = build_truncation_circuit(
            "Capacitor accepted-state handoff\n\
V1 n 0 0\n\
C1 n 0 2p\n\
.TRAN 1n 10n\n\
.END\n",
        );
        circuit.capacitors.v_prev[0] = 0.25;
        circuit.capacitors.v_prev_prev[0] = -0.1;
        circuit.capacitors.v_prev_prev_prev[0] = -0.2;
        circuit.capacitors.i_prev[0] = 3.0e-6;
        let mut candidate = vec![0.0; circuit.matrix_size()];
        let node = circuit.get_node_by_name("n").expect("node exists");
        candidate[node - 1] = 0.8;
        let dt = 0.7e-9;
        let prev_dt = 0.9e-9;
        let prev_prev_dt = 1.1e-9;
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let mut states = vec![CapacitorAcceptedState {
            voltage: Value::NAN,
            current: Value::NAN,
        }];

        let _ = Engine::capacitor_ngspice_truncation_limit(
            &circuit,
            &candidate,
            method,
            trap_order,
            dt,
            prev_dt,
            prev_prev_dt,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            Some(&mut states),
        );

        let coeff = CompanionCoefficients::for_method_with_previous_step(method, dt, prev_dt);
        let capacitance = circuit.capacitors.capacitances[0];
        let voltage = Engine::differential_voltage(
            &candidate,
            circuit.capacitors.stamps[0].pp.row,
            circuit.capacitors.stamps[0].nn.row,
        );
        let geq = coeff.capacitor_geq(capacitance, dt);
        let ieq = coeff.capacitor_ieq(
            capacitance,
            dt,
            circuit.capacitors.v_prev[0],
            circuit.capacitors.v_prev_prev[0],
            circuit.capacitors.i_prev[0],
        );
        assert_eq!(
            states
                .iter()
                .map(|state| (state.voltage.to_bits(), state.current.to_bits()))
                .collect::<Vec<_>>(),
            vec![(voltage.to_bits(), (geq * voltage - ieq).to_bits())],
        );

        circuit.capacitors.capacitances[0] = 0.0;
        states.push(CapacitorAcceptedState {
            voltage: 1.0,
            current: 2.0,
        });
        let _ = Engine::capacitor_ngspice_truncation_limit(
            &circuit,
            &candidate,
            method,
            trap_order,
            dt,
            prev_dt,
            prev_prev_dt,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            Some(&mut states),
        );
        assert!(states.is_empty(), "an incomplete handoff must fail closed");
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn parallel_capacitor_truncation_is_bit_identical_and_state_ordered() {
        let mut deck = String::from("Parallel capacitor truncation\nV1 n 0 0\n");
        for index in 0..64 {
            deck.push_str(&format!(
                "C{} n 0 {:.17e}\n",
                index + 1,
                (index + 1) as Value * 1.0e-12
            ));
        }
        deck.push_str(".TRAN 1n 10n\n.END\n");
        let mut circuit = build_truncation_circuit(&deck);
        for index in 0..circuit.capacitors.len() {
            let scale = (index + 1) as Value;
            circuit.capacitors.v_prev[index] = 0.25 + scale * 1.0e-4;
            circuit.capacitors.v_prev_prev[index] = -0.1 + scale * 2.0e-4;
            circuit.capacitors.v_prev_prev_prev[index] = -0.2 - scale * 1.0e-4;
            circuit.capacitors.i_prev[index] = scale * 3.0e-9;
        }
        let mut candidate = vec![0.0; circuit.matrix_size()];
        let node = circuit.get_node_by_name("n").expect("node exists");
        candidate[node - 1] = 0.8;
        let dt = 0.7e-9;
        let prev_dt = 0.9e-9;
        let prev_prev_dt = 1.1e-9;
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let mut serial_states = Vec::new();
        let serial_limit = Engine::capacitor_ngspice_truncation_limit(
            &circuit,
            &candidate,
            method,
            trap_order,
            dt,
            prev_dt,
            prev_prev_dt,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            Some(&mut serial_states),
        );
        let engine = Engine::default();
        let mut parallel_states = Vec::new();
        let parallel_limit = engine.capacitor_ngspice_truncation_limit_parallel(
            &circuit,
            &candidate,
            method,
            trap_order,
            dt,
            prev_dt,
            prev_prev_dt,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            4,
            &mut parallel_states,
        );

        assert_eq!(
            parallel_limit.map(Value::to_bits),
            serial_limit.map(Value::to_bits)
        );
        assert_eq!(parallel_states.len(), serial_states.len());
        assert!(
            parallel_states
                .iter()
                .zip(&serial_states)
                .all(|(parallel, serial)| {
                    parallel.voltage.to_bits() == serial.voltage.to_bits()
                        && parallel.current.to_bits() == serial.current.to_bits()
                })
        );
    }

    fn build_truncation_circuit(deck: &str) -> crate::circuit::CircuitData {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        engine.build_circuit(&netlist).expect("circuit builds")
    }

    #[test]
    fn family_charge_truncation_lte_shortcuts_do_not_cover_mixed_vdmos_decks() {
        let bjt_and_vdmos = "\
Mixed BJT and VDMOS LTE coverage
VC c 0 0
VB b 0 0
VD d 0 0
VG g 0 0
VS s 0 0
Q1 c b 0 QB
M1 d g s 0 VM W=1 L=1u
.MODEL QB NPN IS=1e-15 BF=100 CJE=1p
.MODEL VM NMOS LEVEL=18 VTO=100 CGDO=1e-12 CGSO=0 CGBO=0 CBD=0 CBS=0
.OP
.END
";
        let circuit = build_truncation_circuit(bjt_and_vdmos);
        assert!(
            !Engine::bjt_charge_truncation_covers_transient_lte(&circuit, Some(1.0e-9)),
            "BJT-family LTE shortcut must not hide generic LTE when a VDMOS is also present"
        );

        let jfet_and_vdmos = "\
Mixed JFET and VDMOS LTE coverage
VD d 0 0
VG g 0 0
VS s 0 0
VJD jd 0 0
VJG jg 0 0
VJS js 0 0
J1 jd jg js JM
M1 d g s 0 VM W=1 L=1u
.MODEL JM NJF LEVEL=2 BETA=1e-12 VT0=-2 LAMBDA=0 VBI=1
.MODEL VM NMOS LEVEL=18 VTO=100 CGDO=1e-12 CGSO=0 CGBO=0 CBD=0 CBS=0
.OP
.END
";
        let circuit = build_truncation_circuit(jfet_and_vdmos);
        assert!(
            !Engine::jfet_charge_truncation_covers_transient_lte(&circuit, Some(1.0e-9)),
            "JFET-family LTE shortcut must not hide generic LTE when a VDMOS is also present"
        );

        let mosfet_and_vdmos = "\
Mixed MOSFET and VDMOS LTE coverage
VD d 0 0
VG g 0 0
VS s 0 0
VMD md 0 0
VMG mg 0 0
VMS ms 0 0
M0 md mg ms 0 MM W=1u L=1u
M1 d g s 0 VM W=1 L=1u
.MODEL MM NMOS LEVEL=1 VTO=1 KP=1e-3
.MODEL VM NMOS LEVEL=18 VTO=100 CGDO=1e-12 CGSO=0 CGBO=0 CBD=0 CBS=0
.OP
.END
";
        let circuit = build_truncation_circuit(mosfet_and_vdmos);
        assert!(
            !Engine::mosfet_charge_truncation_covers_transient_lte(&circuit, Some(1.0e-9)),
            "MOSFET-family LTE shortcut must not hide generic LTE when a VDMOS is also present"
        );
    }

    #[test]
    fn mosfet_truncation_reuses_candidate_capacitances_bit_exactly() {
        let deck = "\
Classic MOS capacitance reuse
VD d 0 1
VG g 0 0.5
VS s 0 0
M1 d g s 0 NM W=10u L=1u
M2 d g s 0 NM W=17u L=2u
.MODEL NM NMOS LEVEL=1 VTO=0.7 KP=1e-3 CGSO=1e-10 CGDO=2e-10 CGBO=3e-10
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");

        let mut history = Engine::initialize_mosfet_history(&circuit, &base);
        let dt = 1.0e-9;
        history.accepted_dt_prev = dt;
        history.accepted_dt_prev_prev = dt;
        let gate = circuit.get_node_by_name("g").expect("gate node");
        let mut candidate = base.clone();
        candidate[gate - 1] += 0.25;

        let mut cache = Vec::new();
        let direct = Engine::mosfet_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            Some((&mut cache, false)),
        );
        assert_eq!(cache.len(), circuit.mosfets.devices.len());
        let captured = cache.clone();

        let reused = Engine::mosfet_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
            Some((&mut cache, true)),
        );

        assert_eq!(reused.map(Value::to_bits), direct.map(Value::to_bits));
        assert_eq!(cache, captured, "reuse must not rewrite the cache");

        let coeff = CompanionCoefficients::for_method_with_previous_step(
            IntegrationMethod::Trapezoidal,
            dt,
            history.accepted_dt_prev,
        );
        let mut charges = Vec::new();
        let mut caps = Vec::new();
        for (idx, device) in circuit.mosfets.devices.iter().enumerate() {
            let (_terms, device_charges, device_caps) = Engine::mosfet_companion_branch_terms::<true>(
                device,
                idx,
                &candidate,
                &coeff,
                dt,
                &history,
                false,
                MosfetCompanionBiasSource::Solution,
                None,
            );
            charges.push(device_charges);
            caps.push(device_caps);
        }
        let context = NgspiceChargeTruncationContext::new(
            dt,
            history.accepted_dt_prev,
            history.accepted_dt_prev_prev,
            IntegrationMethod::Trapezoidal,
            1,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
        )
        .expect("valid truncation context");
        let fused =
            circuit
                .mosfets
                .devices
                .iter()
                .enumerate()
                .fold(None, |limit, (idx, device)| {
                    Engine::min_truncation_limit(
                        limit,
                        context.mosfet_gate_limit_from_cached_charges(
                            device,
                            idx,
                            &charges[idx],
                            caps[idx],
                            &history,
                        ),
                    )
                });
        assert_eq!(fused.map(Value::to_bits), direct.map(Value::to_bits));
        let constants = circuit
            .mosfets
            .devices
            .iter()
            .map(crate::device::Mosfet::classic_transient_constants)
            .collect::<Vec<_>>();
        let batched = context
            .classic_mos_gate_limit_from_cached_charges(&constants, &charges, &caps, &history);
        assert_eq!(batched.map(Value::to_bits), direct.map(Value::to_bits));
    }

    #[test]
    fn b3soi_truncation_ignores_qth_like_ngspice_b3soiddtrunc() {
        let deck = "\
B3SOIDD qth truncation coverage
VD d 0 5
VG g 0 1.2
VS s 0 0
VE e 0 0
M1 d g s e n1 w=4u l=1u
.MODEL n1 NMOS LEVEL=56 SHMOD=1 RTH0=0.1 CTH0=1 CAPMOD=2
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");

        let mut history = Engine::initialize_b3soi_history(&circuit, &base);
        let dt = 1.0e-9;
        history.accepted_dt_prev = dt;
        history.accepted_dt_prev_prev = dt;

        let temp = circuit
            .get_node_by_name("m1.__temp.internal")
            .expect("self-heating temp node");
        let mut candidate = base.clone();
        candidate[temp - 1] = 100.0;

        let limit = Engine::b3soi_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
        )
        .expect("unchanged qg/qb/qd still yields the default 2*dt bound");

        assert!(
            (limit - 2.0 * dt).abs() <= 1.0e-18,
            "B3SOIDDtrunc should ignore qth-only changes; got limit {limit:.9e}"
        );
    }

    #[test]
    fn vdmos_charge_history_participates_in_device_truncation_limit() {
        let deck = "\
VDMOS truncation coverage
VD d 0 0
VG g 0 0
VS s 0 0
M1 d g s 0 VTRUNC W=1 L=1u
.MODEL VTRUNC NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGDO=1e-9
+ CGSO=0
+ CGBO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");

        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &base);
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &base);
        let mut diode_history = Engine::initialize_diode_history(&circuit, &base);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
        let mut vdmos_history = Engine::initialize_vdmos_history(&circuit, &base);
        let mut ekv26_history = Engine::initialize_ekv26_history(&circuit, &base);
        let dt = 1.0e-9;
        bjt_history.accepted_dt_prev = dt;
        bjt_history.accepted_dt_prev_prev = dt;
        jfet_history.accepted_dt_prev = dt;
        jfet_history.accepted_dt_prev_prev = dt;
        diode_history.accepted_dt_prev = dt;
        diode_history.accepted_dt_prev_prev = dt;
        mosfet_history.accepted_dt_prev = dt;
        mosfet_history.accepted_dt_prev_prev = dt;
        vdmos_history.accepted_dt_prev = dt;
        vdmos_history.accepted_dt_prev_prev = dt;
        ekv26_history.accepted_dt_prev = dt;
        ekv26_history.accepted_dt_prev_prev = dt;

        let gate = circuit.get_node_by_name("g").expect("gate node");
        let mut candidate = base.clone();
        candidate[gate - 1] = 1.0;

        let limit = Engine::ngspice_device_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &bjt_history,
            &[],
            &jfet_history,
            &diode_history,
            &mosfet_history,
            &vdmos_history,
            &ekv26_history,
            false,
            1.0e-9,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
        );

        assert!(
            limit.is_some_and(|limit| limit.is_finite() && limit > 0.0),
            "VDMOS-only charge deck must contribute a truncation limit, got {limit:?}"
        );
    }

    #[test]
    fn ekv26_charge_history_participates_in_device_truncation_limit() {
        let deck = "\
EKV26 truncation coverage
.OPTIONS TEMP=27
.MODEL n NMOS LEVEL=260 TNOM=27 COX=4.379e-3 XJ=22.53n VTO=570.6m TCV=1.194m \
        GAMMA=670.7m PHI=450m KP=232.1u BEX=-1.828 E0=42.216MEG UCRIT=3.146E6 \
        LAMBDA=228.3m DL=-60.86n DW=-209.7n WETA=2.001 LETA=264.6m AVTO=0
M1 d g s b n W=10u L=1u AS=0 AD=0 PS=0 PD=0
VD d 0 1
VG g 0 0.8
VS s 0 0
VB b 0 -1
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");

        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &base);
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &base);
        let mut diode_history = Engine::initialize_diode_history(&circuit, &base);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
        let mut vdmos_history = Engine::initialize_vdmos_history(&circuit, &base);
        let mut ekv26_history = Engine::initialize_ekv26_history(&circuit, &base);
        let dt = 1.0e-9;
        bjt_history.accepted_dt_prev = dt;
        bjt_history.accepted_dt_prev_prev = dt;
        jfet_history.accepted_dt_prev = dt;
        jfet_history.accepted_dt_prev_prev = dt;
        diode_history.accepted_dt_prev = dt;
        diode_history.accepted_dt_prev_prev = dt;
        mosfet_history.accepted_dt_prev = dt;
        mosfet_history.accepted_dt_prev_prev = dt;
        vdmos_history.accepted_dt_prev = dt;
        vdmos_history.accepted_dt_prev_prev = dt;
        ekv26_history.accepted_dt_prev = dt;
        ekv26_history.accepted_dt_prev_prev = dt;

        let gate = circuit.get_node_by_name("g").expect("gate node");
        let mut candidate = base.clone();
        candidate[gate - 1] += 0.1;

        let limit = Engine::ngspice_device_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &bjt_history,
            &[],
            &jfet_history,
            &diode_history,
            &mosfet_history,
            &vdmos_history,
            &ekv26_history,
            false,
            1.0e-9,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
        );

        assert!(
            limit.is_some_and(|limit| limit.is_finite() && limit > 0.0),
            "EKV26-only charge deck must contribute a truncation limit, got {limit:?}"
        );
    }

    #[test]
    fn jfet2_cds_history_participates_even_when_gate_charge_is_suppressed() {
        let deck = "\
JFET2 CDS truncation coverage
VD d 0 0
VG g 0 0
VS s 0 0
J1 d g s PS area=1
.MODEL PS NJF(level=2 beta=1e-12 vt0=-2 lambda=0 vbi=1 is=1e-18 n=1 \
              cgs=0 cgd=0 cds=1e-9)
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");

        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &base);
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &base);
        let mut diode_history = Engine::initialize_diode_history(&circuit, &base);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
        let mut vdmos_history = Engine::initialize_vdmos_history(&circuit, &base);
        let mut ekv26_history = Engine::initialize_ekv26_history(&circuit, &base);
        let dt = 1.0e-9;
        bjt_history.accepted_dt_prev = dt;
        bjt_history.accepted_dt_prev_prev = dt;
        jfet_history.accepted_dt_prev = dt;
        jfet_history.accepted_dt_prev_prev = dt;
        diode_history.accepted_dt_prev = dt;
        diode_history.accepted_dt_prev_prev = dt;
        mosfet_history.accepted_dt_prev = dt;
        mosfet_history.accepted_dt_prev_prev = dt;
        vdmos_history.accepted_dt_prev = dt;
        vdmos_history.accepted_dt_prev_prev = dt;
        ekv26_history.accepted_dt_prev = dt;
        ekv26_history.accepted_dt_prev_prev = dt;

        let drain = circuit.get_node_by_name("d").expect("drain node");
        let mut candidate = base.clone();
        candidate[drain - 1] = 1.0;

        let limit = Engine::ngspice_device_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &bjt_history,
            &[],
            &jfet_history,
            &diode_history,
            &mosfet_history,
            &vdmos_history,
            &ekv26_history,
            true,
            1.0e-9,
            1.0e-3,
            1.0e-12,
            1.0e-14,
            7.0,
        );

        assert!(
            limit.is_some_and(|limit| limit.is_finite() && limit > 0.0),
            "JFET2 CDS charge must contribute a truncation limit independently of gate-charge suppression, got {limit:?}"
        );
    }
}
