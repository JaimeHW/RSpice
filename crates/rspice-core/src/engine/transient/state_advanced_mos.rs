//! Advanced MOS, SOI, and EKV transient charge-history helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn initialize_b3soi_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> B3SoiTransientHistory {
        let n = circuit.b3soi.len();
        let mut h = B3SoiTransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qe_prev: Vec::with_capacity(n),
            qe_prev_prev: Vec::with_capacity(n),
            qe_prev_prev_prev: Vec::with_capacity(n),
            cqe_prev: Vec::with_capacity(n),
            qth_prev: Vec::with_capacity(n),
            qth_prev_prev: Vec::with_capacity(n),
            qth_prev_prev_prev: Vec::with_capacity(n),
            cqth_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        let mut seed = |qg: Value, qb: Value, qd: Value, qe: Value, qth: Value| {
            h.qg_prev.push(qg);
            h.qg_prev_prev.push(qg);
            h.qg_prev_prev_prev.push(qg);
            h.cqg_prev.push(0.0);
            h.qb_prev.push(qb);
            h.qb_prev_prev.push(qb);
            h.qb_prev_prev_prev.push(qb);
            h.cqb_prev.push(0.0);
            h.qd_prev.push(qd);
            h.qd_prev_prev.push(qd);
            h.qd_prev_prev_prev.push(qd);
            h.cqd_prev.push(0.0);
            h.qe_prev.push(qe);
            h.qe_prev_prev.push(qe);
            h.qe_prev_prev_prev.push(qe);
            h.cqe_prev.push(0.0);
            h.qth_prev.push(qth);
            h.qth_prev_prev.push(qth);
            h.qth_prev_prev_prev.push(qth);
            h.cqth_prev.push(0.0);
        };
        // The history is indexed DD devices first, then FD, then PD; the
        // stamp/commit/truncation walks use the same concatenated order.
        // `DEBUG=-1` devices keep an (all-zero) slot so the indexing stays
        // aligned, but contribute no charges.
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        h
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn b3soi_reseed_history_slot(
        history: &mut B3SoiTransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
        qe: Value,
        qth: Value,
    ) {
        history.qg_prev[idx] = qg;
        history.qg_prev_prev[idx] = qg;
        history.qg_prev_prev_prev[idx] = qg;
        history.cqg_prev[idx] = 0.0;
        history.qb_prev[idx] = qb;
        history.qb_prev_prev[idx] = qb;
        history.qb_prev_prev_prev[idx] = qb;
        history.cqb_prev[idx] = 0.0;
        history.qd_prev[idx] = qd;
        history.qd_prev_prev[idx] = qd;
        history.qd_prev_prev_prev[idx] = qd;
        history.cqd_prev[idx] = 0.0;
        history.qe_prev[idx] = qe;
        history.qe_prev_prev[idx] = qe;
        history.qe_prev_prev_prev[idx] = qe;
        history.cqe_prev[idx] = 0.0;
        history.qth_prev[idx] = qth;
        history.qth_prev_prev[idx] = qth;
        history.qth_prev_prev_prev[idx] = qth;
        history.cqth_prev[idx] = 0.0;
    }

    #[inline]
    pub(super) fn reseed_b3soi_first_transient_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
        history: &mut B3SoiTransientHistory,
    ) {
        if !circuit.has_b3soi_devices() {
            return;
        }

        let expected_len = circuit.b3soi.devices.len()
            + circuit.b3soi_fd.devices.len()
            + circuit.b3soi_pd.devices.len();
        debug_assert_eq!(history.qg_prev.len(), expected_len);

        // Xyce's B3SOI loader resets the current charge state on
        // `initTranFlag_ && newtonIter == 0`, so the first transient solve
        // leaves the operating point without a synthetic DC-to-transient
        // charge derivative. Reseed only the charge/current slots; the
        // accepted timestep history remains owned by the transient controller.
        let mut idx = 0;
        for dev in &circuit.b3soi.devices {
            let (qg, qb, qd, qe, qth) = if dev.charges_suppressed() {
                (0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let c = dev.charge_at(solution);
                (c.qg, c.qb, c.qd, c.qe, c.qth)
            };
            Self::b3soi_reseed_history_slot(history, idx, qg, qb, qd, qe, qth);
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            let (qg, qb, qd, qe, qth) = if dev.charges_suppressed() {
                (0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let c = dev.charge_at(solution);
                (c.qg, c.qb, c.qd, c.qe, c.qth)
            };
            Self::b3soi_reseed_history_slot(history, idx, qg, qb, qd, qe, qth);
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            let (qg, qb, qd, qe, qth) = if dev.charges_suppressed() {
                (0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let c = dev.charge_at(solution);
                (c.qg, c.qb, c.qd, c.qe, c.qth)
            };
            Self::b3soi_reseed_history_slot(history, idx, qg, qb, qd, qe, qth);
            idx += 1;
        }
    }

    /// Integrate one SOI device's node charges with the engine
    /// coefficient and its per-charge history slot, yielding the equivalent
    /// charge currents `(cqg, cqb, cqd, cqe, cqth)`.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn b3soi_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
        qe: Value,
        qth: Value,
    ) -> (Value, Value, Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
            cq(
                qe,
                history.qe_prev[idx],
                history.qe_prev_prev[idx],
                history.cqe_prev[idx],
            ),
            cq(
                qth,
                history.qth_prev[idx],
                history.qth_prev_prev[idx],
                history.cqth_prev[idx],
            ),
        )
    }

    /// Commit one SOI device's accepted charges and integrated currents into
    /// its history slot.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn b3soi_commit_history_slot(
        history: &mut B3SoiTransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
        qe: Value,
        qth: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
        cqth: Value,
    ) {
        history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
        history.qg_prev_prev[idx] = history.qg_prev[idx];
        history.qg_prev[idx] = qg;
        history.cqg_prev[idx] = cqg;
        history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
        history.qb_prev_prev[idx] = history.qb_prev[idx];
        history.qb_prev[idx] = qb;
        history.cqb_prev[idx] = cqb;
        history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
        history.qd_prev_prev[idx] = history.qd_prev[idx];
        history.qd_prev[idx] = qd;
        history.cqd_prev[idx] = cqd;
        history.qe_prev_prev_prev[idx] = history.qe_prev_prev[idx];
        history.qe_prev_prev[idx] = history.qe_prev[idx];
        history.qe_prev[idx] = qe;
        history.cqe_prev[idx] = cqe;
        history.qth_prev_prev_prev[idx] = history.qth_prev_prev[idx];
        history.qth_prev_prev[idx] = history.qth_prev[idx];
        history.qth_prev[idx] = qth;
        history.cqth_prev[idx] = cqth;
    }

    /// Stamp the B3SOI transient charge companion for every SOI instance.
    ///
    /// Integrates each coupled node charge with the engine's
    /// integration coefficient `ag0` and the per-charge history, then stamps the
    /// coupled `gc**·ag0` capacitance matrix plus the `ceqq*` equivalent charge
    /// currents (B3SOI charge load). Active DD/FD/PD self-heating also stamps
    /// the thermal `qth` companion onto the temperature node.
    #[inline]
    pub(super) fn stamp_b3soi_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
    ) {
        if !circuit.has_b3soi_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        let mut idx = 0;
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
    }

    /// Commit the SOI (DD/FD/PD) charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_b3soi_history(
        circuit: &mut crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut B3SoiTransientHistory,
    ) {
        if !circuit.has_b3soi_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut idx = 0;
        for dev in &mut circuit.b3soi.devices {
            dev.commit_accepted_transient_state();
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn initialize_bsim3_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> Bsim3TransientHistory {
        let n = circuit.bsim3v3.len();
        let mut h = Bsim3TransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qcheq_prev: Vec::with_capacity(n),
            qcheq_prev_prev: Vec::with_capacity(n),
            qcheq_prev_prev_prev: Vec::with_capacity(n),
            cqcheq_prev: Vec::with_capacity(n),
            qcdump_prev: Vec::with_capacity(n),
            qcdump_prev_prev: Vec::with_capacity(n),
            qcdump_prev_prev_prev: Vec::with_capacity(n),
            cqcdump_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        // Flat seed at the accepted point with zeroed charge currents, the
        // MODEINITTRAN state copy of b3ld.c:2818-2829. States stay
        // per-device (ngspice applies `m` only at stamp time; CKTterr sees
        // the unscaled CKTstate charges).
        for dev in &circuit.bsim3v3.devices {
            let (c, _mode) = dev.charge_at(solution);
            for (q, slots) in [
                (
                    c.qg_state(),
                    [
                        &mut h.qg_prev,
                        &mut h.qg_prev_prev,
                        &mut h.qg_prev_prev_prev,
                    ],
                ),
                (
                    c.qb_state(),
                    [
                        &mut h.qb_prev,
                        &mut h.qb_prev_prev,
                        &mut h.qb_prev_prev_prev,
                    ],
                ),
                (
                    c.qd_state(),
                    [
                        &mut h.qd_prev,
                        &mut h.qd_prev_prev,
                        &mut h.qd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            h.cqg_prev.push(0.0);
            h.cqb_prev.push(0.0);
            h.cqd_prev.push(0.0);
            let qcheq = if dev.uses_trnqs() { c.qcheq } else { 0.0 };
            for slot in [
                &mut h.qcheq_prev,
                &mut h.qcheq_prev_prev,
                &mut h.qcheq_prev_prev_prev,
            ] {
                slot.push(qcheq);
            }
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(solution)
            } else {
                0.0
            };
            for slot in [
                &mut h.qcdump_prev,
                &mut h.qcdump_prev_prev,
                &mut h.qcdump_prev_prev_prev,
            ] {
                slot.push(qcdump);
            }
            h.cqcheq_prev.push(0.0);
            h.cqcdump_prev.push(0.0);
        }
        h
    }

    /// Integrate one BSIM3 device's three composite node charges with the
    /// engine coefficient and its history slot, yielding the equivalent
    /// charge currents `(cqg, cqb, cqd)` (ngspice `NIintegrate` on
    /// `BSIM3qg`/`BSIM3qb`/`BSIM3qd`).
    #[inline]
    fn bsim3_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
    ) -> (Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
        )
    }

    #[inline]
    fn bsim3_trnqs_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
        idx: usize,
        qcheq: Value,
        qcdump: Value,
    ) -> (Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qcheq,
                history.qcheq_prev[idx],
                history.qcheq_prev_prev[idx],
                history.cqcheq_prev[idx],
            ),
            cq(
                qcdump,
                history.qcdump_prev[idx],
                history.qcdump_prev_prev[idx],
                history.cqcdump_prev[idx],
            ),
        )
    }

    /// Stamp the BSIM3 transient charge companion for every instance: the
    /// mode-assembled `gc**·ag0` capacitance matrix plus the `ceqq*`
    /// equivalent charge currents. `NQSMOD=1` also stamps the hidden
    /// charge-deficit row from b3ld.c.
    #[inline]
    pub(super) fn stamp_bsim3_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
    ) {
        if !circuit.has_bsim3v3_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (charge, mode) = dev.charge_at(voltages);
            let (qg, qb, qd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge)
            } else {
                (charge.qg_state(), charge.qb_state(), charge.qd_state())
            };
            let (cqg, cqb, cqd) = Self::bsim3_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qb,
                qd,
            );
            // The history carries per-device charges; the device stamp
            // applies the parallel multiplier itself (b3ld.c: m * ceqq*).
            if dev.uses_trnqs() {
                let qcdump = dev.trnqs_qcdump_state(voltages);
                let (cqcheq, cqcdump) = Self::bsim3_trnqs_companion_currents(
                    effective_method,
                    trap_order,
                    dt,
                    history,
                    idx,
                    charge.qcheq,
                    qcdump,
                );
                dev.stamp_trnqs_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    cqcheq,
                    cqcdump,
                    voltages,
                    &mut stamper,
                );
            } else {
                dev.stamp_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    voltages,
                    &mut stamper,
                );
            }
        }
    }

    /// Commit the BSIM3 charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_bsim3_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut Bsim3TransientHistory,
    ) {
        if !circuit.has_bsim3v3_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (charge, _mode) = dev.charge_at(voltages);
            let (qg, qb, qd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge)
            } else {
                (charge.qg_state(), charge.qb_state(), charge.qd_state())
            };
            let (cqg, cqb, cqd) = Self::bsim3_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qb,
                qd,
            );
            history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
            history.qg_prev_prev[idx] = history.qg_prev[idx];
            history.qg_prev[idx] = qg;
            history.cqg_prev[idx] = cqg;
            history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
            history.qb_prev_prev[idx] = history.qb_prev[idx];
            history.qb_prev[idx] = qb;
            history.cqb_prev[idx] = cqb;
            history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
            history.qd_prev_prev[idx] = history.qd_prev[idx];
            history.qd_prev[idx] = qd;
            history.cqd_prev[idx] = cqd;
            let qcheq = if dev.uses_trnqs() { charge.qcheq } else { 0.0 };
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(voltages)
            } else {
                0.0
            };
            let (cqcheq, cqcdump) = Self::bsim3_trnqs_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qcheq,
                qcdump,
            );
            history.qcheq_prev_prev_prev[idx] = history.qcheq_prev_prev[idx];
            history.qcheq_prev_prev[idx] = history.qcheq_prev[idx];
            history.qcheq_prev[idx] = qcheq;
            history.cqcheq_prev[idx] = cqcheq;
            history.qcdump_prev_prev_prev[idx] = history.qcdump_prev_prev[idx];
            history.qcdump_prev_prev[idx] = history.qcdump_prev[idx];
            history.qcdump_prev[idx] = qcdump;
            history.cqcdump_prev[idx] = cqcdump;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn initialize_bsim4_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> Bsim4TransientHistory {
        let n = circuit.bsim4v8.len();
        let mut h = Bsim4TransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qgmid_prev: Vec::with_capacity(n),
            qgmid_prev_prev: Vec::with_capacity(n),
            qgmid_prev_prev_prev: Vec::with_capacity(n),
            cqgmid_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            qbs_prev_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            qbd_prev_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            qcheq_prev: Vec::with_capacity(n),
            qcheq_prev_prev: Vec::with_capacity(n),
            qcheq_prev_prev_prev: Vec::with_capacity(n),
            cqcheq_prev: Vec::with_capacity(n),
            qcdump_prev: Vec::with_capacity(n),
            qcdump_prev_prev: Vec::with_capacity(n),
            qcdump_prev_prev_prev: Vec::with_capacity(n),
            cqcdump_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        // Flat seed at the accepted point with zeroed charge currents, the
        // MODEINITTRAN state copy of b4ld.c:4611-4628. States stay
        // per-device (ngspice applies `m` only at stamp time; CKTterr sees
        // the unscaled CKTstate charges).
        for dev in &circuit.bsim4v8.devices {
            let (c, _mode) = dev.charge_at(solution);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&c, solution)
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
            for (q, slots) in [
                (
                    qg,
                    [
                        &mut h.qg_prev,
                        &mut h.qg_prev_prev,
                        &mut h.qg_prev_prev_prev,
                    ],
                ),
                (
                    qb,
                    [
                        &mut h.qb_prev,
                        &mut h.qb_prev_prev,
                        &mut h.qb_prev_prev_prev,
                    ],
                ),
                (
                    qd,
                    [
                        &mut h.qd_prev,
                        &mut h.qd_prev_prev,
                        &mut h.qd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            for slot in [
                &mut h.qgmid_prev,
                &mut h.qgmid_prev_prev,
                &mut h.qgmid_prev_prev_prev,
            ] {
                slot.push(qgmid);
            }
            for (q, slots) in [
                (
                    qbs,
                    [
                        &mut h.qbs_prev,
                        &mut h.qbs_prev_prev,
                        &mut h.qbs_prev_prev_prev,
                    ],
                ),
                (
                    qbd,
                    [
                        &mut h.qbd_prev,
                        &mut h.qbd_prev_prev,
                        &mut h.qbd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            h.cqg_prev.push(0.0);
            h.cqgmid_prev.push(0.0);
            h.cqb_prev.push(0.0);
            h.cqd_prev.push(0.0);
            h.cqbs_prev.push(0.0);
            h.cqbd_prev.push(0.0);
            for slot in [
                &mut h.qcheq_prev,
                &mut h.qcheq_prev_prev,
                &mut h.qcheq_prev_prev_prev,
            ] {
                slot.push(c.qchqs);
            }
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(solution)
            } else {
                0.0
            };
            for slot in [
                &mut h.qcdump_prev,
                &mut h.qcdump_prev_prev,
                &mut h.qcdump_prev_prev_prev,
            ] {
                slot.push(qcdump);
            }
            h.cqcheq_prev.push(0.0);
            h.cqcdump_prev.push(0.0);
        }
        h
    }

    /// Integrate one BSIM4 device's charge states with the
    /// engine coefficient and its history slot, yielding the equivalent
    /// charge currents (ngspice `NIintegrate` on `BSIM4qg`/`BSIM4qb`/
    /// `BSIM4qd`, plus `qbs`/`qbd` when `rbodyMod > 0`; b4ld.c:4630-4649).
    #[inline]
    fn bsim4_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
        idx: usize,
        qg: Value,
        qgmid: Value,
        qb: Value,
        qd: Value,
        qbs: Value,
        qbd: Value,
    ) -> (Value, Value, Value, Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qgmid,
                history.qgmid_prev[idx],
                history.qgmid_prev_prev[idx],
                history.cqgmid_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
            cq(
                qbs,
                history.qbs_prev[idx],
                history.qbs_prev_prev[idx],
                history.cqbs_prev[idx],
            ),
            cq(
                qbd,
                history.qbd_prev[idx],
                history.qbd_prev_prev[idx],
                history.cqbd_prev[idx],
            ),
        )
    }

    #[inline]
    fn bsim4_trnqs_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
        idx: usize,
        qcheq: Value,
        qcdump: Value,
    ) -> (Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qcheq,
                history.qcheq_prev[idx],
                history.qcheq_prev_prev[idx],
                history.cqcheq_prev[idx],
            ),
            cq(
                qcdump,
                history.qcdump_prev[idx],
                history.qcdump_prev_prev[idx],
                history.cqcdump_prev[idx],
            ),
        )
    }

    /// Stamp the BSIM4 transient charge companion for every instance: the
    /// mode-assembled `gc**·ag0` capacitance matrix plus the `ceqq*`
    /// equivalent charge currents (b4ld.c charge load, trnqsMod = 0).
    #[inline]
    pub(super) fn stamp_bsim4_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
    ) {
        if !circuit.has_bsim4v8_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        for (idx, dev) in circuit.bsim4v8.devices.iter().enumerate() {
            let (charge, mode) = dev.charge_at(voltages);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge, voltages)
            } else {
                (
                    charge.qg_state(),
                    charge.qgmid_state(),
                    charge.qb_state_for_rbody(rbody),
                    charge.qd_state(),
                    charge.qbs,
                    charge.qbd,
                )
            };
            let (cqg, cqgmid, cqb, cqd, cqbs, cqbd) = Self::bsim4_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qgmid,
                qb,
                qd,
                qbs,
                qbd,
            );
            // The history carries per-device charges; the device stamp
            // applies the parallel multiplier itself (b4ld.c: mult_q * ceqq*).
            if dev.uses_trnqs() {
                let qcdump = dev.trnqs_qcdump_state(voltages);
                let (cqcheq, cqcdump) = Self::bsim4_trnqs_companion_currents(
                    effective_method,
                    trap_order,
                    dt,
                    history,
                    idx,
                    charge.qchqs,
                    qcdump,
                );
                dev.stamp_trnqs_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    cqbs,
                    cqbd,
                    cqcheq,
                    cqcdump,
                    voltages,
                    &mut stamper,
                );
            } else {
                dev.stamp_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqgmid,
                    cqb,
                    cqd,
                    cqbs,
                    cqbd,
                    voltages,
                    &mut stamper,
                );
            }
        }
    }

    /// Commit the BSIM4 charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_bsim4_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut Bsim4TransientHistory,
    ) {
        if !circuit.has_bsim4v8_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, dev) in circuit.bsim4v8.devices.iter().enumerate() {
            let (charge, _mode) = dev.charge_at(voltages);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge, voltages)
            } else {
                (
                    charge.qg_state(),
                    charge.qgmid_state(),
                    charge.qb_state_for_rbody(rbody),
                    charge.qd_state(),
                    charge.qbs,
                    charge.qbd,
                )
            };
            let (cqg, cqgmid, cqb, cqd, cqbs, cqbd) = Self::bsim4_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qgmid,
                qb,
                qd,
                qbs,
                qbd,
            );
            history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
            history.qg_prev_prev[idx] = history.qg_prev[idx];
            history.qg_prev[idx] = qg;
            history.cqg_prev[idx] = cqg;
            history.qgmid_prev_prev_prev[idx] = history.qgmid_prev_prev[idx];
            history.qgmid_prev_prev[idx] = history.qgmid_prev[idx];
            history.qgmid_prev[idx] = qgmid;
            history.cqgmid_prev[idx] = cqgmid;
            history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
            history.qb_prev_prev[idx] = history.qb_prev[idx];
            history.qb_prev[idx] = qb;
            history.cqb_prev[idx] = cqb;
            history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
            history.qd_prev_prev[idx] = history.qd_prev[idx];
            history.qd_prev[idx] = qd;
            history.cqd_prev[idx] = cqd;
            history.qbs_prev_prev_prev[idx] = history.qbs_prev_prev[idx];
            history.qbs_prev_prev[idx] = history.qbs_prev[idx];
            history.qbs_prev[idx] = qbs;
            history.cqbs_prev[idx] = cqbs;
            history.qbd_prev_prev_prev[idx] = history.qbd_prev_prev[idx];
            history.qbd_prev_prev[idx] = history.qbd_prev[idx];
            history.qbd_prev[idx] = qbd;
            history.cqbd_prev[idx] = cqbd;
            let qcheq = charge.qchqs;
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(voltages)
            } else {
                0.0
            };
            let (cqcheq, cqcdump) = Self::bsim4_trnqs_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qcheq,
                qcdump,
            );
            history.qcheq_prev_prev_prev[idx] = history.qcheq_prev_prev[idx];
            history.qcheq_prev_prev[idx] = history.qcheq_prev[idx];
            history.qcheq_prev[idx] = qcheq;
            history.cqcheq_prev[idx] = cqcheq;
            history.qcdump_prev_prev_prev[idx] = history.qcdump_prev_prev[idx];
            history.qcdump_prev_prev[idx] = history.qcdump_prev[idx];
            history.qcdump_prev[idx] = qcdump;
            history.cqcdump_prev[idx] = cqcdump;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn initialize_ekv26_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> Ekv26TransientHistory {
        let n = circuit.ekv26s.devices.len();
        let mut h = Ekv26TransientHistory {
            q_prev: Vec::with_capacity(n),
            q_prev_prev: Vec::with_capacity(n),
            q_prev_prev_prev: Vec::with_capacity(n),
            cq_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        for dev in &circuit.ekv26s.devices {
            let q = dev.dynamic_charge_vector_at_solution(solution);
            h.q_prev.push(q);
            h.q_prev_prev.push(q);
            h.q_prev_prev_prev.push(q);
            h.cq_prev.push([0.0; EKV26_DYNAMIC_CHARGE_COUNT]);
        }
        h
    }

    #[inline]
    fn ekv26_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Ekv26TransientHistory,
        idx: usize,
        q_curr: &[Value; EKV26_DYNAMIC_CHARGE_COUNT],
    ) -> [Value; EKV26_DYNAMIC_CHARGE_COUNT] {
        let mut cq = [0.0; EKV26_DYNAMIC_CHARGE_COUNT];
        for row in 0..EKV26_DYNAMIC_CHARGE_COUNT {
            cq[row] = Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q_curr[row],
                history.q_prev[idx][row],
                history.q_prev_prev[idx][row],
                history.cq_prev[idx][row],
            );
        }
        cq
    }

    #[inline]
    fn ekv26_history_currents(
        ag0: Value,
        q_curr: &[Value; EKV26_DYNAMIC_CHARGE_COUNT],
        cq_curr: &[Value; EKV26_DYNAMIC_CHARGE_COUNT],
    ) -> [Value; EKV26_DYNAMIC_CHARGE_COUNT] {
        let mut history_currents = [0.0; EKV26_DYNAMIC_CHARGE_COUNT];
        for row in 0..EKV26_DYNAMIC_CHARGE_COUNT {
            history_currents[row] = cq_curr[row] - ag0 * q_curr[row];
        }
        history_currents
    }

    #[inline]
    pub(super) fn stamp_ekv26_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Ekv26TransientHistory,
    ) {
        if circuit.ekv26s.is_empty() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        for (idx, dev) in circuit.ekv26s.devices.iter().enumerate() {
            let q = dev.dynamic_charge_vector_at_solution(voltages);
            let cq =
                Self::ekv26_companion_currents(effective_method, trap_order, dt, history, idx, &q);
            let history_currents = Self::ekv26_history_currents(ag0, &q, &cq);
            dev.stamp_dynamic_companion(voltages, ag0, &history_currents, &mut stamper);
        }
    }

    #[inline]
    pub(super) fn update_ekv26_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut Ekv26TransientHistory,
    ) {
        if circuit.ekv26s.is_empty() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, dev) in circuit.ekv26s.devices.iter().enumerate() {
            let q = dev.dynamic_charge_vector_at_solution(voltages);
            let cq =
                Self::ekv26_companion_currents(effective_method, trap_order, dt, history, idx, &q);
            history.q_prev_prev_prev[idx] = history.q_prev_prev[idx];
            history.q_prev_prev[idx] = history.q_prev[idx];
            history.q_prev[idx] = q;
            history.cq_prev[idx] = cq;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }
}
