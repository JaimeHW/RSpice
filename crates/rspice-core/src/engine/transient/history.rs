//! Transient solver limits and per-device accepted-state history.

use super::*;
use crate::device::semiconductor::AcceptedBjtChargeSnapshotCheckpoint;

/// Maximum voltage limit for solution values (matching DC solver)
///
/// Commercial simulators like Spectre/HSPICE use similar limits to prevent
/// Newton-Raphson divergence on stiff nonlinear circuits (e.g., BJT exponential I-V).
/// This value matches the DC solver's MAX_VOLTAGE in convergence.rs for consistency.
pub(super) const MAX_VOLTAGE: Value = 1000.0;
/// Conservative magnitude limit for branch-state unknowns (currents and auxiliary
/// MNA variables). These states can legitimately exceed node-voltage scales in
/// tightly coupled passive networks, so they need a separate guardrail.
pub(super) const MAX_BRANCH_STATE_MAGNITUDE: Value = 1e12;
/// Xyce Core branch currents are raw physical currents multiplied by the
/// authored winding values in the Q matrix.  Tiny vacuum coefficients can
/// therefore require currents above the generic 1e12 safety rail even for
/// bounded one-volt sources.  Keep a finite overflow guard while preserving
/// the canonical Xyce state range.
pub(super) const MAX_XYCE_CORE_BRANCH_STATE_MAGNITUDE: Value = 1e30;
/// Maximum allowed per-iteration node update during Newton damping.
///
/// This bound controls nonlinear solve trust-region size.
pub(super) const MAX_NEWTON_ITER_DELTA_V: Value = 1e-2;
/// Initial global trust-region limit for the ngspice legacy-BJT backend.
///
/// Legacy BJTs use ngspice-style local pnjlim limiting internally; this wider
/// nodal leash lets sharp switching steps converge without removing the global
/// guardrail completely.
pub(super) const LEGACY_NGSPICE_BJT_NEWTON_ITER_DELTA_V: Value = 1.5e-2;
/// Largest node trust-region used after repeated finite Newton corrections.
///
/// Device-local junction limiting still governs semiconductor branch voltages;
/// this cap only prevents the global MNA node update limiter from turning a
/// valid large-signal transition into hundreds of identical 10 mV iterations.
pub(super) const MAX_ADAPTIVE_NEWTON_ITER_DELTA_V: Value = STARTUP_RECOVERY_DELTA_V;
/// Number of failed Newton retries at a timepoint before the global node
/// trust region is re-engaged as a rescue.
///
/// ngspice never clamps node updates globally during transient stepping, so
/// the first attempts at each timepoint run pure Newton (with device-level
/// junction limiting) to preserve ngspice waveform and timestep parity. Only
/// when a point has already failed repeatedly does the conservative damping
/// return as a robustness fallback.
pub(super) const CONSERVATIVE_LIMITING_RETRY_THRESHOLD: usize = 2;
/// Maximum allowed node update when committing force-accepted steps.
///
/// This remains tight to avoid committing nonphysical jumps into reactive history.
pub(super) const MAX_FORCE_ACCEPT_DELTA_V: Value = 5e-2;
/// Relaxed trust-region limit used only during early startup when DC OP failed and
/// transient had to begin from a linearized seed.
pub(super) const STARTUP_RECOVERY_DELTA_V: Value = 2e-1;
/// Minimum failed retries required at the effective minimum timestep before a
/// timepoint may be force-accepted.
pub(super) const MIN_RETRIES_AT_MINIMUM_TIMESTEP: usize = 1;
/// Failed Newton retries at a timepoint before the gmin-continuation rescue
/// is attempted (see `transient/rescue.rs`). The first retries stay on the
/// plain dt-cut path so ordinary stiffness keeps ngspice-parity stepping;
/// a knife edge that survives two cuts is dt-independent and goes to
/// continuation before the cut cascade can poison the charge history.
pub(super) const TRANSIENT_GMIN_RESCUE_MIN_RETRIES: usize = 2;
/// Source edge magnitude that triggers transient source-step capping.
pub(super) const SOURCE_ACTIVE_DELTA: Value = 1e-2;
/// Maximum source movement per proactive ramp step for HFET charge models.
pub(super) const HFET_SOURCE_RAMP_TRACKING_DELTA: Value = 5e-2;
/// Local ngspice `NIiter()` raises any smaller iteration limit to 100.
pub(super) const NGSPICE_NIITER_MIN_ITERATIONS: usize = 100;
/// Safety cap for synthesized transmission-line arrival breakpoints.
pub(super) const MAX_PROPAGATED_TLINE_BREAKPOINTS: usize = 200_000;
/// Safety cap for dynamically scheduled transmission-line arrival breakpoints.
pub(super) const MAX_DYNAMIC_TLINE_BREAKPOINTS: usize = 200_000;
pub(super) const VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL: Value = 1e-15;
pub(super) const VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL: Value = 1e-12;
pub(super) const BJT_VBIC_TRUNCATION_BRANCH_COUNT: usize = BJT_DYNAMIC_CHARGE_COUNT - 3;
pub(super) const BJT_VCX_STATE_INDEX: usize = 0;
pub(super) const BJT_VCI_STATE_INDEX: usize = 1;
pub(super) const BJT_VBX_STATE_INDEX: usize = 2;
pub(super) const BJT_VBI_STATE_INDEX: usize = 3;
pub(super) const BJT_VEI_STATE_INDEX: usize = 4;
pub(super) const BJT_VBP_STATE_INDEX: usize = 5;
pub(super) const BJT_VSI_STATE_INDEX: usize = 6;
pub(super) const BJT_THERMAL_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 3;
pub(super) const BJT_DELAY_XF1_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 2;
pub(super) const BJT_DELAY_XF2_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 1;
pub(super) const BJT_QBE_BRANCH_INDEX: usize = 0;
pub(super) const BJT_QBC_BRANCH_INDEX: usize = 2;
pub(super) const BJT_QBCX_BRANCH_INDEX: usize = 3;
pub(super) const BJT_QBCP_BRANCH_INDEX: usize = 7;
pub(super) const BJT_DELAY_XF1_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 2;
pub(super) const BJT_DELAY_XF2_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 1;
pub(super) const BJT_STATIC_CORE_STATE_DIM: usize = BJT_INTERNAL_STATE_DIM - 2;
pub(super) const BJT_EXT_C_INDEX: usize = 0;
pub(super) const BJT_EXT_B_INDEX: usize = 1;
pub(super) const BJT_EXT_E_INDEX: usize = 2;
pub(super) const BJT_EXT_S_INDEX: usize = 3;
pub(super) const EKV26_DYNAMIC_CHARGE_COUNT: usize = 4;

#[derive(Debug, Clone, Default)]
pub(super) struct JfetTransientHistory {
    pub(super) vgs_prev: Vec<Value>,
    pub(super) vgs_prev_prev: Vec<Value>,
    pub(super) qgs_prev: Vec<Value>,
    pub(super) qgs_prev_prev: Vec<Value>,
    pub(super) qgs_prev_prev_prev: Vec<Value>,
    pub(super) cqgs_prev: Vec<Value>,
    pub(super) vgd_prev: Vec<Value>,
    pub(super) vgd_prev_prev: Vec<Value>,
    pub(super) qgd_prev: Vec<Value>,
    pub(super) qgd_prev_prev: Vec<Value>,
    pub(super) qgd_prev_prev_prev: Vec<Value>,
    pub(super) cqgd_prev: Vec<Value>,
    pub(super) vds_prev: Vec<Value>,
    pub(super) vds_prev_prev: Vec<Value>,
    pub(super) qds_prev: Vec<Value>,
    pub(super) qds_prev_prev: Vec<Value>,
    pub(super) qds_prev_prev_prev: Vec<Value>,
    pub(super) cqds_prev: Vec<Value>,
    pub(super) jfet2_vgstrap_prev: Vec<Value>,
    pub(super) jfet2_vgdtrap_prev: Vec<Value>,
    pub(super) jfet2_power_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

/// Junction charge history for diodes (ngspice `DIOcapCharge` state):
/// the depletion+diffusion charge is integrated with the same companion
/// discipline as the JFET/MOSFET gate charges.
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct DiodeTransientHistory {
    pub(super) vd_prev: Vec<Value>,
    pub(super) vd_prev_prev: Vec<Value>,
    pub(super) qd_prev: Vec<Value>,
    pub(super) qd_prev_prev: Vec<Value>,
    pub(super) qd_prev_prev_prev: Vec<Value>,
    pub(super) cqd_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct BjtTransientHistory {
    pub(super) vbe_prev: Vec<Value>,
    pub(super) vbe_prev_prev: Vec<Value>,
    pub(super) ibe_prev: Vec<Value>,
    pub(super) vbc_prev: Vec<Value>,
    pub(super) vbc_prev_prev: Vec<Value>,
    pub(super) ibc_prev: Vec<Value>,
    pub(super) vcs_prev: Vec<Value>,
    pub(super) vcs_prev_prev: Vec<Value>,
    pub(super) ics_prev: Vec<Value>,
    pub(super) charge_q_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    pub(super) charge_q_prev_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    pub(super) charge_q_prev_prev_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    pub(super) charge_cq_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    /// Accepted total terminal currents for legacy native BJTs, in C/B/E/S
    /// order. `None` preserves the ordinary static operating-point report for
    /// device families whose transient lead-current projection is not owned
    /// by the reduced legacy companion.
    pub(super) accepted_terminal_currents: Vec<Option<[Value; BJT_EXTERNAL_STATE_DIM]>>,
    pub(super) dynamic_internal_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    pub(super) dynamic_internal_prev_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    pub(super) dynamic_linear_prev: Vec<VbicPredictorLinearBranchState>,
    pub(super) dynamic_linear_prev_prev: Vec<VbicPredictorLinearBranchState>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

pub(super) const BJT_TRANSIENT_HISTORY_RUNTIME_TAG: &str =
    "legacy-gummel-poon-transient-history-v1";
pub(super) const DIODE_TRANSIENT_HISTORY_RUNTIME_TAG: &str = "native-diode-transient-history-v1";

/// Versionable image of the accepted BJT/diode integration state owned by the
/// transient engine rather than by the device instances.
///
/// The histories remain in their runtime struct-of-arrays form. Parallel name
/// and runtime-tag vectors bind every ordinal to the exact elaborated device;
/// checkpoint wire code can serialize one row at a time without transposing
/// or duplicating the in-memory payload here.
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct AcceptedJunctionTransientHistoryCheckpoint {
    pub(super) available: bool,
    pub(super) resume_blockers: Vec<String>,
    pub(super) bjt_names: Vec<String>,
    pub(super) bjt_runtime_tags: Vec<String>,
    pub(super) bjt_history: BjtTransientHistory,
    pub(super) diode_names: Vec<String>,
    pub(super) diode_runtime_tags: Vec<String>,
    pub(super) diode_history: DiodeTransientHistory,
    pub(super) vbic_snapshot_cache: Vec<Option<AcceptedBjtChargeSnapshotCheckpoint>>,
}

impl AcceptedJunctionTransientHistoryCheckpoint {
    pub(super) fn unavailable(blocker: impl Into<String>) -> Self {
        Self {
            available: false,
            resume_blockers: vec![blocker.into()],
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TrapezoidalOrderTrial {
    pub(super) limit: Value,
    pub(super) promote: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct VbicTransientLinearization {
    pub(super) g_ii: [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub(super) g_ie: [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub(super) g_ei: [[Value; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub(super) g_ee: [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub(super) z_i: [Value; BJT_INTERNAL_STATE_DIM],
    pub(super) z_e: [Value; BJT_EXTERNAL_STATE_DIM],
}

pub(super) type VbicDynamicStateEvaluation = (
    BjtChargeSnapshot,
    VbicTransientLinearization,
    [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    [Value; BJT_INTERNAL_STATE_DIM],
    Value,
);

pub(super) type VbicBestEffortSolve = (
    BjtChargeSnapshot,
    VbicTransientLinearization,
    [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    Value,
);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct VbicPredictorLinearBranchState {
    pub(super) vrcx: Value,
    pub(super) vrci: Value,
    pub(super) vrbx: Value,
    pub(super) vrbi: Value,
    pub(super) vre: Value,
    pub(super) vrbp: Value,
    pub(super) vrs: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VbicCachedSnapshotReuse {
    SeedOnly,
    NewtonBypass,
}

fn validate_history_vector_shapes(
    kind: &str,
    expected: usize,
    fields: &[(&str, usize)],
) -> Result<(), String> {
    for &(field, actual) in fields {
        if actual != expected {
            return Err(format!(
                "{kind} transient history field '{field}' shape mismatch: captured {actual} values, runtime requires {expected}"
            ));
        }
    }
    Ok(())
}

fn validate_history_finite_values(
    field: &str,
    values: impl IntoIterator<Item = Value>,
) -> Result<(), String> {
    for (index, value) in values.into_iter().enumerate() {
        if !value.is_finite() {
            return Err(format!(
                "transient history field '{field}' contains a non-finite value at scalar index {index}"
            ));
        }
    }
    Ok(())
}

fn validate_history_dt(field: &str, value: Value) -> Result<(), String> {
    if !value.is_finite() || value < 0.0 {
        return Err(format!(
            "transient history field '{field}' must be finite and nonnegative, got {value}"
        ));
    }
    Ok(())
}

impl Engine {
    /// Capture the accepted engine-owned junction histories without mutating
    /// either the circuit or the runtime histories. A valid non-breakpoint
    /// capture preserves the optional per-BJT snapshot cache exactly.
    pub(super) fn capture_accepted_junction_transient_history_checkpoint(
        circuit: &crate::circuit::CircuitData,
        bjt_history: &BjtTransientHistory,
        diode_history: &DiodeTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
    ) -> AcceptedJunctionTransientHistoryCheckpoint {
        let mut resume_blockers = Vec::new();
        let mut encoded_snapshot_cache = Vec::with_capacity(vbic_snapshot_cache.len());
        for (index, snapshot) in vbic_snapshot_cache.iter().enumerate() {
            let encoded = match (circuit.bjts.devices.get(index), snapshot) {
                (_, None) => None,
                (Some(bjt), Some(snapshot)) => {
                    match bjt.encode_accepted_charge_snapshot_checkpoint(snapshot) {
                        Ok(checkpoint) => Some(checkpoint),
                        Err(error) => {
                            resume_blockers.push(error);
                            None
                        }
                    }
                }
                (None, Some(_)) => {
                    resume_blockers.push(format!(
                        "BJT transient snapshot cache contains ordinal {index}, but the circuit has only {} BJT instances",
                        circuit.bjts.devices.len()
                    ));
                    None
                }
            };
            encoded_snapshot_cache.push(encoded);
        }

        let mut checkpoint = AcceptedJunctionTransientHistoryCheckpoint {
            available: true,
            resume_blockers,
            bjt_names: circuit
                .bjts
                .devices
                .iter()
                .map(|bjt| bjt.name.clone())
                .collect(),
            bjt_runtime_tags: vec![
                BJT_TRANSIENT_HISTORY_RUNTIME_TAG.to_string();
                circuit.bjts.devices.len()
            ],
            bjt_history: bjt_history.clone(),
            diode_names: circuit
                .diodes
                .devices
                .iter()
                .map(|diode| diode.name.clone())
                .collect(),
            diode_runtime_tags: vec![
                DIODE_TRANSIENT_HISTORY_RUNTIME_TAG.to_string();
                circuit.diodes.devices.len()
            ],
            diode_history: diode_history.clone(),
            vbic_snapshot_cache: encoded_snapshot_cache,
        };

        if let Err(error) =
            Self::validate_accepted_junction_transient_history_payload(circuit, &checkpoint)
        {
            if !checkpoint
                .resume_blockers
                .iter()
                .any(|blocker| blocker == &error)
            {
                checkpoint.resume_blockers.push(error);
            }
        }
        checkpoint
    }

    /// Validate availability, capture blockers, identity, runtime family,
    /// fixed shapes and every numeric lane before any caller replaces live
    /// engine state.
    pub(super) fn validate_accepted_junction_transient_history_checkpoint(
        circuit: &crate::circuit::CircuitData,
        checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    ) -> Result<(), String> {
        if !checkpoint.available {
            return Err("accepted BJT/diode transient history is unavailable".to_string());
        }
        if !checkpoint.resume_blockers.is_empty() {
            return Err(format!(
                "accepted BJT/diode transient history is not resumable: {}",
                checkpoint.resume_blockers.join("; ")
            ));
        }
        Self::validate_accepted_junction_transient_history_payload(circuit, checkpoint)
    }

    fn validate_accepted_junction_transient_history_payload(
        circuit: &crate::circuit::CircuitData,
        checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    ) -> Result<(), String> {
        let bjt_count = circuit.bjts.devices.len();
        if checkpoint.bjt_names.len() != bjt_count {
            return Err(format!(
                "BJT transient history identity shape mismatch: captured {} names, runtime requires {bjt_count}",
                checkpoint.bjt_names.len()
            ));
        }
        if checkpoint.bjt_runtime_tags.len() != bjt_count {
            return Err(format!(
                "BJT transient history runtime-tag shape mismatch: captured {} tags, runtime requires {bjt_count}",
                checkpoint.bjt_runtime_tags.len()
            ));
        }
        for (index, bjt) in circuit.bjts.devices.iter().enumerate() {
            let captured_name = &checkpoint.bjt_names[index];
            if captured_name != &bjt.name {
                return Err(format!(
                    "BJT transient history instance mismatch at ordinal {index}: captured '{captured_name}', circuit has '{}'",
                    bjt.name
                ));
            }
            let captured_tag = &checkpoint.bjt_runtime_tags[index];
            if captured_tag != BJT_TRANSIENT_HISTORY_RUNTIME_TAG {
                return Err(format!(
                    "BJT '{}' transient history runtime mismatch: captured '{captured_tag}', runtime requires '{BJT_TRANSIENT_HISTORY_RUNTIME_TAG}'",
                    bjt.name
                ));
            }
            if !bjt.uses_legacy_gummel_poon() {
                return Err(format!(
                    "BJT '{}' transient history is not checkpointable; only the legacy Gummel-Poon runtime has a complete history contract",
                    bjt.name
                ));
            }
        }

        let bjt = &checkpoint.bjt_history;
        validate_history_vector_shapes(
            "BJT",
            bjt_count,
            &[
                ("vbe_prev", bjt.vbe_prev.len()),
                ("vbe_prev_prev", bjt.vbe_prev_prev.len()),
                ("ibe_prev", bjt.ibe_prev.len()),
                ("vbc_prev", bjt.vbc_prev.len()),
                ("vbc_prev_prev", bjt.vbc_prev_prev.len()),
                ("ibc_prev", bjt.ibc_prev.len()),
                ("vcs_prev", bjt.vcs_prev.len()),
                ("vcs_prev_prev", bjt.vcs_prev_prev.len()),
                ("ics_prev", bjt.ics_prev.len()),
                ("charge_q_prev", bjt.charge_q_prev.len()),
                ("charge_q_prev_prev", bjt.charge_q_prev_prev.len()),
                ("charge_q_prev_prev_prev", bjt.charge_q_prev_prev_prev.len()),
                ("charge_cq_prev", bjt.charge_cq_prev.len()),
                (
                    "accepted_terminal_currents",
                    bjt.accepted_terminal_currents.len(),
                ),
                ("dynamic_internal_prev", bjt.dynamic_internal_prev.len()),
                (
                    "dynamic_internal_prev_prev",
                    bjt.dynamic_internal_prev_prev.len(),
                ),
                ("dynamic_linear_prev", bjt.dynamic_linear_prev.len()),
                (
                    "dynamic_linear_prev_prev",
                    bjt.dynamic_linear_prev_prev.len(),
                ),
            ],
        )?;
        for (field, values) in [
            ("bjt.vbe_prev", bjt.vbe_prev.as_slice()),
            ("bjt.vbe_prev_prev", bjt.vbe_prev_prev.as_slice()),
            ("bjt.ibe_prev", bjt.ibe_prev.as_slice()),
            ("bjt.vbc_prev", bjt.vbc_prev.as_slice()),
            ("bjt.vbc_prev_prev", bjt.vbc_prev_prev.as_slice()),
            ("bjt.ibc_prev", bjt.ibc_prev.as_slice()),
            ("bjt.vcs_prev", bjt.vcs_prev.as_slice()),
            ("bjt.vcs_prev_prev", bjt.vcs_prev_prev.as_slice()),
            ("bjt.ics_prev", bjt.ics_prev.as_slice()),
        ] {
            validate_history_finite_values(field, values.iter().copied())?;
        }
        for (field, values) in [
            ("bjt.charge_q_prev", bjt.charge_q_prev.as_slice()),
            ("bjt.charge_q_prev_prev", bjt.charge_q_prev_prev.as_slice()),
            (
                "bjt.charge_q_prev_prev_prev",
                bjt.charge_q_prev_prev_prev.as_slice(),
            ),
            ("bjt.charge_cq_prev", bjt.charge_cq_prev.as_slice()),
        ] {
            validate_history_finite_values(field, values.iter().flatten().copied())?;
        }
        for (index, currents) in bjt.accepted_terminal_currents.iter().enumerate() {
            if let Some(currents) = currents {
                validate_history_finite_values(
                    &format!("bjt.accepted_terminal_currents[{index}]"),
                    currents.iter().copied(),
                )?;
            }
        }
        for (field, values) in [
            (
                "bjt.dynamic_internal_prev",
                bjt.dynamic_internal_prev.as_slice(),
            ),
            (
                "bjt.dynamic_internal_prev_prev",
                bjt.dynamic_internal_prev_prev.as_slice(),
            ),
        ] {
            validate_history_finite_values(field, values.iter().flatten().copied())?;
        }
        for (field, states) in [
            (
                "bjt.dynamic_linear_prev",
                bjt.dynamic_linear_prev.as_slice(),
            ),
            (
                "bjt.dynamic_linear_prev_prev",
                bjt.dynamic_linear_prev_prev.as_slice(),
            ),
        ] {
            validate_history_finite_values(
                field,
                states.iter().flat_map(|state| {
                    [
                        state.vrcx, state.vrci, state.vrbx, state.vrbi, state.vre, state.vrbp,
                        state.vrs,
                    ]
                }),
            )?;
        }
        validate_history_dt("bjt.accepted_dt_prev", bjt.accepted_dt_prev)?;
        validate_history_dt("bjt.accepted_dt_prev_prev", bjt.accepted_dt_prev_prev)?;

        if checkpoint.vbic_snapshot_cache.len() != bjt_count {
            return Err(format!(
                "BJT transient snapshot-cache shape mismatch: captured {} entries, runtime requires {bjt_count}",
                checkpoint.vbic_snapshot_cache.len()
            ));
        }
        for (index, snapshot) in checkpoint.vbic_snapshot_cache.iter().enumerate() {
            if let Some(snapshot) = snapshot {
                circuit.bjts.devices[index]
                    .validate_accepted_charge_snapshot_checkpoint(snapshot)?;
            }
        }

        let diode_count = circuit.diodes.devices.len();
        if checkpoint.diode_names.len() != diode_count {
            return Err(format!(
                "diode transient history identity shape mismatch: captured {} names, runtime requires {diode_count}",
                checkpoint.diode_names.len()
            ));
        }
        if checkpoint.diode_runtime_tags.len() != diode_count {
            return Err(format!(
                "diode transient history runtime-tag shape mismatch: captured {} tags, runtime requires {diode_count}",
                checkpoint.diode_runtime_tags.len()
            ));
        }
        for (index, diode) in circuit.diodes.devices.iter().enumerate() {
            let captured_name = &checkpoint.diode_names[index];
            if captured_name != &diode.name {
                return Err(format!(
                    "diode transient history instance mismatch at ordinal {index}: captured '{captured_name}', circuit has '{}'",
                    diode.name
                ));
            }
            let captured_tag = &checkpoint.diode_runtime_tags[index];
            if captured_tag != DIODE_TRANSIENT_HISTORY_RUNTIME_TAG {
                return Err(format!(
                    "diode '{}' transient history runtime mismatch: captured '{captured_tag}', runtime requires '{DIODE_TRANSIENT_HISTORY_RUNTIME_TAG}'",
                    diode.name
                ));
            }
        }

        let diode = &checkpoint.diode_history;
        validate_history_vector_shapes(
            "diode",
            diode_count,
            &[
                ("vd_prev", diode.vd_prev.len()),
                ("vd_prev_prev", diode.vd_prev_prev.len()),
                ("qd_prev", diode.qd_prev.len()),
                ("qd_prev_prev", diode.qd_prev_prev.len()),
                ("qd_prev_prev_prev", diode.qd_prev_prev_prev.len()),
                ("cqd_prev", diode.cqd_prev.len()),
            ],
        )?;
        for (field, values) in [
            ("diode.vd_prev", diode.vd_prev.as_slice()),
            ("diode.vd_prev_prev", diode.vd_prev_prev.as_slice()),
            ("diode.qd_prev", diode.qd_prev.as_slice()),
            ("diode.qd_prev_prev", diode.qd_prev_prev.as_slice()),
            (
                "diode.qd_prev_prev_prev",
                diode.qd_prev_prev_prev.as_slice(),
            ),
            ("diode.cqd_prev", diode.cqd_prev.as_slice()),
        ] {
            validate_history_finite_values(field, values.iter().copied())?;
        }
        validate_history_dt("diode.accepted_dt_prev", diode.accepted_dt_prev)?;
        validate_history_dt("diode.accepted_dt_prev_prev", diode.accepted_dt_prev_prev)?;
        Ok(())
    }

    /// Clone and normalize a validated physical-breakpoint image into a new
    /// order-one integration epoch. The authoritative current accepted state
    /// remains exact; older generations/derivatives are flattened and the
    /// trial snapshot cache is deliberately invalidated.
    pub(super) fn normalize_accepted_junction_transient_history_checkpoint_for_order_one(
        circuit: &crate::circuit::CircuitData,
        checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
        accepted_dt_seed: Value,
    ) -> Result<AcceptedJunctionTransientHistoryCheckpoint, String> {
        validate_history_dt("accepted_dt_seed", accepted_dt_seed)?;
        Self::validate_accepted_junction_transient_history_checkpoint(circuit, checkpoint)?;
        let mut normalized = checkpoint.clone();
        Self::flatten_bjt_and_diode_histories_for_order_one_restart(
            &mut normalized.bjt_history,
            &mut normalized.diode_history,
            accepted_dt_seed,
        );
        normalized.vbic_snapshot_cache.fill(None);
        Self::validate_accepted_junction_transient_history_checkpoint(circuit, &normalized)?;
        Ok(normalized)
    }

    /// Validate the complete aggregate, then clone/decode its engine-owned
    /// runtime pieces. No circuit/device state is mutated.
    pub(super) fn restore_accepted_junction_transient_history_checkpoint(
        circuit: &crate::circuit::CircuitData,
        checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    ) -> Result<
        (
            BjtTransientHistory,
            DiodeTransientHistory,
            Vec<Option<BjtChargeSnapshot>>,
        ),
        String,
    > {
        Self::validate_accepted_junction_transient_history_checkpoint(circuit, checkpoint)?;
        let mut snapshot_cache = Vec::with_capacity(checkpoint.vbic_snapshot_cache.len());
        for (index, snapshot) in checkpoint.vbic_snapshot_cache.iter().enumerate() {
            snapshot_cache.push(match snapshot {
                Some(snapshot) => Some(
                    circuit.bjts.devices[index]
                        .decode_accepted_charge_snapshot_checkpoint(snapshot)?,
                ),
                None => None,
            });
        }
        Ok((
            checkpoint.bjt_history.clone(),
            checkpoint.diode_history.clone(),
            snapshot_cache,
        ))
    }
}

/// Candidate state already evaluated by the ordinary-capacitor CKTterr walk
/// and eligible for an exact accepted-history handoff.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct CapacitorAcceptedState {
    pub(super) voltage: Value,
    pub(super) current: Value,
}

#[derive(Debug, Clone, Default)]
pub(super) struct MosfetTransientHistory {
    pub(super) vgs_prev: Vec<Value>,
    pub(super) vgs_prev_prev: Vec<Value>,
    pub(super) capgs_prev_half: Vec<Value>,
    pub(super) qgs_prev: Vec<Value>,
    pub(super) qgs_prev_prev: Vec<Value>,
    pub(super) qgs_prev_prev_prev: Vec<Value>,
    pub(super) cqgs_prev: Vec<Value>,
    pub(super) vgd_prev: Vec<Value>,
    pub(super) vgd_prev_prev: Vec<Value>,
    pub(super) capgd_prev_half: Vec<Value>,
    pub(super) qgd_prev: Vec<Value>,
    pub(super) qgd_prev_prev: Vec<Value>,
    pub(super) qgd_prev_prev_prev: Vec<Value>,
    pub(super) cqgd_prev: Vec<Value>,
    pub(super) vgb_prev: Vec<Value>,
    pub(super) vgb_prev_prev: Vec<Value>,
    pub(super) capgb_prev_half: Vec<Value>,
    pub(super) qgb_prev: Vec<Value>,
    pub(super) qgb_prev_prev: Vec<Value>,
    pub(super) qgb_prev_prev_prev: Vec<Value>,
    pub(super) cqgb_prev: Vec<Value>,
    pub(super) vbs_j_prev: Vec<Value>,
    pub(super) vbs_j_prev_prev: Vec<Value>,
    pub(super) qbs_prev: Vec<Value>,
    pub(super) qbs_prev_prev: Vec<Value>,
    pub(super) cqbs_prev: Vec<Value>,
    pub(super) vbd_j_prev: Vec<Value>,
    pub(super) vbd_j_prev_prev: Vec<Value>,
    pub(super) qbd_prev: Vec<Value>,
    pub(super) qbd_prev_prev: Vec<Value>,
    pub(super) cqbd_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

impl MosfetTransientHistory {
    /// Rotate accepted classic-MOS gate state generations in O(1). The old
    /// oldest buffers become scratch for the caller's new accepted values.
    #[inline]
    pub(super) fn rotate_gate_generations(&mut self, suppress_gate_charge_history: bool) {
        std::mem::swap(&mut self.vgs_prev, &mut self.vgs_prev_prev);
        std::mem::swap(&mut self.vgd_prev, &mut self.vgd_prev_prev);
        std::mem::swap(&mut self.vgb_prev, &mut self.vgb_prev_prev);
        if suppress_gate_charge_history {
            return;
        }

        std::mem::swap(&mut self.qgs_prev_prev, &mut self.qgs_prev_prev_prev);
        std::mem::swap(&mut self.qgs_prev, &mut self.qgs_prev_prev);
        std::mem::swap(&mut self.qgd_prev_prev, &mut self.qgd_prev_prev_prev);
        std::mem::swap(&mut self.qgd_prev, &mut self.qgd_prev_prev);
        std::mem::swap(&mut self.qgb_prev_prev, &mut self.qgb_prev_prev_prev);
        std::mem::swap(&mut self.qgb_prev, &mut self.qgb_prev_prev);
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct VdmosTransientHistory {
    pub(super) vgs_prev: Vec<Value>,
    pub(super) vgs_prev_prev: Vec<Value>,
    pub(super) qgs_prev: Vec<Value>,
    pub(super) qgs_prev_prev: Vec<Value>,
    pub(super) qgs_prev_prev_prev: Vec<Value>,
    pub(super) cqgs_prev: Vec<Value>,
    pub(super) vgd_prev: Vec<Value>,
    pub(super) vgd_prev_prev: Vec<Value>,
    pub(super) qgd_prev: Vec<Value>,
    pub(super) qgd_prev_prev: Vec<Value>,
    pub(super) qgd_prev_prev_prev: Vec<Value>,
    pub(super) cqgd_prev: Vec<Value>,
    pub(super) vgb_prev: Vec<Value>,
    pub(super) vgb_prev_prev: Vec<Value>,
    pub(super) qgb_prev: Vec<Value>,
    pub(super) qgb_prev_prev: Vec<Value>,
    pub(super) qgb_prev_prev_prev: Vec<Value>,
    pub(super) cqgb_prev: Vec<Value>,
    pub(super) vds_prev: Vec<Value>,
    pub(super) vds_prev_prev: Vec<Value>,
    pub(super) qds_prev: Vec<Value>,
    pub(super) qds_prev_prev: Vec<Value>,
    pub(super) qds_prev_prev_prev: Vec<Value>,
    pub(super) cqds_prev: Vec<Value>,
    pub(super) vbs_prev: Vec<Value>,
    pub(super) vbs_prev_prev: Vec<Value>,
    pub(super) qbs_prev: Vec<Value>,
    pub(super) qbs_prev_prev: Vec<Value>,
    pub(super) qbs_prev_prev_prev: Vec<Value>,
    pub(super) cqbs_prev: Vec<Value>,
    pub(super) vbd_prev: Vec<Value>,
    pub(super) vbd_prev_prev: Vec<Value>,
    pub(super) qbd_prev: Vec<Value>,
    pub(super) qbd_prev_prev: Vec<Value>,
    pub(super) qbd_prev_prev_prev: Vec<Value>,
    pub(super) cqbd_prev: Vec<Value>,
    pub(super) vd1_prev: Vec<Value>,
    pub(super) vd1_prev_prev: Vec<Value>,
    pub(super) qd1_prev: Vec<Value>,
    pub(super) qd1_prev_prev: Vec<Value>,
    pub(super) qd1_prev_prev_prev: Vec<Value>,
    pub(super) cqd1_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

/// Per-instance B3SOIDD (BSIMSOI level 56) charge-integration history.
///
/// The SOI charge model integrates the coupled node charges (qg/qb/qd/qe) with
/// the engine's integration coefficient, mirroring ngspice's `NIintegrate` on
/// `B3SOIDDq{g,b,d,e}`. DD and FD self-heating also integrate
/// `qth = Cth*delTemp`.
/// We keep the last two accepted charges (for Gear/Trap2 history) and the last
/// integrated charge-current `cq*` per node. The SOI body and thermal charges
/// feed LTE exactly as the gate charges do.
#[derive(Debug, Clone, Default)]
pub(super) struct B3SoiTransientHistory {
    pub(super) qg_prev: Vec<Value>,
    pub(super) qg_prev_prev: Vec<Value>,
    pub(super) qg_prev_prev_prev: Vec<Value>,
    pub(super) cqg_prev: Vec<Value>,
    pub(super) qb_prev: Vec<Value>,
    pub(super) qb_prev_prev: Vec<Value>,
    pub(super) qb_prev_prev_prev: Vec<Value>,
    pub(super) cqb_prev: Vec<Value>,
    pub(super) qd_prev: Vec<Value>,
    pub(super) qd_prev_prev: Vec<Value>,
    pub(super) qd_prev_prev_prev: Vec<Value>,
    pub(super) cqd_prev: Vec<Value>,
    pub(super) qe_prev: Vec<Value>,
    pub(super) qe_prev_prev: Vec<Value>,
    pub(super) qe_prev_prev_prev: Vec<Value>,
    pub(super) cqe_prev: Vec<Value>,
    pub(super) qth_prev: Vec<Value>,
    pub(super) qth_prev_prev: Vec<Value>,
    pub(super) qth_prev_prev_prev: Vec<Value>,
    pub(super) cqth_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

/// Per-instance BSIM3v3.3 (MOS level 8/49) charge-integration history.
///
/// Mirrors [`B3SoiTransientHistory`] over the three composite CKTstate
/// charges ngspice integrates for BSIM3 (`BSIM3qg`, `BSIM3qd = qdrn - qbd`,
/// `BSIM3qb = qbulk + qbd + qbs` — the junction depletion charges are folded
/// in, b3ld.c:2796-2801), with the last integrated charge-current `cq*` per
/// state. `b3trunc.c` runs `CKTterr` over exactly these three states.
#[derive(Debug, Clone, Default)]
pub(super) struct Bsim3TransientHistory {
    pub(super) qg_prev: Vec<Value>,
    pub(super) qg_prev_prev: Vec<Value>,
    pub(super) qg_prev_prev_prev: Vec<Value>,
    pub(super) cqg_prev: Vec<Value>,
    pub(super) qb_prev: Vec<Value>,
    pub(super) qb_prev_prev: Vec<Value>,
    pub(super) qb_prev_prev_prev: Vec<Value>,
    pub(super) cqb_prev: Vec<Value>,
    pub(super) qd_prev: Vec<Value>,
    pub(super) qd_prev_prev: Vec<Value>,
    pub(super) qd_prev_prev_prev: Vec<Value>,
    pub(super) cqd_prev: Vec<Value>,
    pub(super) qcheq_prev: Vec<Value>,
    pub(super) qcheq_prev_prev: Vec<Value>,
    pub(super) qcheq_prev_prev_prev: Vec<Value>,
    pub(super) cqcheq_prev: Vec<Value>,
    pub(super) qcdump_prev: Vec<Value>,
    pub(super) qcdump_prev_prev: Vec<Value>,
    pub(super) qcdump_prev_prev_prev: Vec<Value>,
    pub(super) cqcdump_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

/// Per-instance BSIM4 v4.8 (MOS level 14/54) charge-integration history.
///
/// The same base shape as [`Bsim3TransientHistory`]: b4ld.c integrates
/// `BSIM4qg`, `BSIM4qd`, and `BSIM4qb`. When `rbodyMod>0`, `qb` becomes the
/// intrinsic bulk charge and b4ld.c also integrates separate junction states
/// `qbs`/`qbd`; when `rgateMod=3`, it also integrates middle-gate overlap
/// charge `qgmid`. `b4trunc.c` runs `CKTterr` over those extra states too.
#[derive(Debug, Clone, Default)]
pub(super) struct Bsim4TransientHistory {
    pub(super) qg_prev: Vec<Value>,
    pub(super) qg_prev_prev: Vec<Value>,
    pub(super) qg_prev_prev_prev: Vec<Value>,
    pub(super) cqg_prev: Vec<Value>,
    pub(super) qgmid_prev: Vec<Value>,
    pub(super) qgmid_prev_prev: Vec<Value>,
    pub(super) qgmid_prev_prev_prev: Vec<Value>,
    pub(super) cqgmid_prev: Vec<Value>,
    pub(super) qb_prev: Vec<Value>,
    pub(super) qb_prev_prev: Vec<Value>,
    pub(super) qb_prev_prev_prev: Vec<Value>,
    pub(super) cqb_prev: Vec<Value>,
    pub(super) qd_prev: Vec<Value>,
    pub(super) qd_prev_prev: Vec<Value>,
    pub(super) qd_prev_prev_prev: Vec<Value>,
    pub(super) cqd_prev: Vec<Value>,
    pub(super) qbs_prev: Vec<Value>,
    pub(super) qbs_prev_prev: Vec<Value>,
    pub(super) qbs_prev_prev_prev: Vec<Value>,
    pub(super) cqbs_prev: Vec<Value>,
    pub(super) qbd_prev: Vec<Value>,
    pub(super) qbd_prev_prev: Vec<Value>,
    pub(super) qbd_prev_prev_prev: Vec<Value>,
    pub(super) cqbd_prev: Vec<Value>,
    pub(super) qcheq_prev: Vec<Value>,
    pub(super) qcheq_prev_prev: Vec<Value>,
    pub(super) qcheq_prev_prev_prev: Vec<Value>,
    pub(super) cqcheq_prev: Vec<Value>,
    pub(super) qcdump_prev: Vec<Value>,
    pub(super) qcdump_prev_prev: Vec<Value>,
    pub(super) qcdump_prev_prev_prev: Vec<Value>,
    pub(super) cqcdump_prev: Vec<Value>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

#[derive(Debug, Clone, Default)]
pub(super) struct Ekv26TransientHistory {
    pub(super) q_prev: Vec<[Value; EKV26_DYNAMIC_CHARGE_COUNT]>,
    pub(super) q_prev_prev: Vec<[Value; EKV26_DYNAMIC_CHARGE_COUNT]>,
    pub(super) q_prev_prev_prev: Vec<[Value; EKV26_DYNAMIC_CHARGE_COUNT]>,
    pub(super) cq_prev: Vec<[Value; EKV26_DYNAMIC_CHARGE_COUNT]>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
}

#[derive(Debug, Clone, Default)]
pub(super) struct CoupledTlineReferenceState {
    pub(super) near_modal: Vec<Value>,
    pub(super) far_modal: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Netlist;

    fn accepted_junction_history_fixture() -> (
        crate::circuit::CircuitData,
        BjtTransientHistory,
        DiodeTransientHistory,
        Vec<Option<BjtChargeSnapshot>>,
    ) {
        let deck = "\
accepted junction transient history fixture
VCC c 0 1
VB b 0 0.7
Q1 c b 0 QM
D1 b 0 DM
.MODEL QM NPN (CJE=1p CJC=2p)
.MODEL DM D (CJO=3p TT=1n)
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("fixture parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("fixture builds");
        let bjt_history = BjtTransientHistory {
            vbe_prev: vec![1.0],
            vbe_prev_prev: vec![2.0],
            ibe_prev: vec![3.0],
            vbc_prev: vec![4.0],
            vbc_prev_prev: vec![5.0],
            ibc_prev: vec![6.0],
            vcs_prev: vec![7.0],
            vcs_prev_prev: vec![8.0],
            ics_prev: vec![9.0],
            charge_q_prev: vec![std::array::from_fn(|index| 10.0 + index as Value)],
            charge_q_prev_prev: vec![std::array::from_fn(|index| 30.0 + index as Value)],
            charge_q_prev_prev_prev: vec![std::array::from_fn(|index| 50.0 + index as Value)],
            charge_cq_prev: vec![std::array::from_fn(|index| 70.0 + index as Value)],
            accepted_terminal_currents: vec![Some(std::array::from_fn(|index| {
                90.0 + index as Value
            }))],
            dynamic_internal_prev: vec![std::array::from_fn(|index| 100.0 + index as Value)],
            dynamic_internal_prev_prev: vec![std::array::from_fn(|index| 130.0 + index as Value)],
            dynamic_linear_prev: vec![VbicPredictorLinearBranchState {
                vrcx: 160.0,
                vrci: 161.0,
                vrbx: 162.0,
                vrbi: 163.0,
                vre: 164.0,
                vrbp: 165.0,
                vrs: 166.0,
            }],
            dynamic_linear_prev_prev: vec![VbicPredictorLinearBranchState {
                vrcx: 170.0,
                vrci: 171.0,
                vrbx: 172.0,
                vrbi: 173.0,
                vre: 174.0,
                vrbp: 175.0,
                vrs: 176.0,
            }],
            accepted_dt_prev: 0.25,
            accepted_dt_prev_prev: 0.5,
        };
        let diode_history = DiodeTransientHistory {
            vd_prev: vec![201.0],
            vd_prev_prev: vec![202.0],
            qd_prev: vec![203.0],
            qd_prev_prev: vec![204.0],
            qd_prev_prev_prev: vec![205.0],
            cqd_prev: vec![206.0],
            accepted_dt_prev: 0.75,
            accepted_dt_prev_prev: 1.0,
        };
        let bjt = &circuit.bjts.devices[0];
        let snapshot = bjt.charge_snapshot(1.0, 0.7, 0.0, 0.0);
        (circuit, bjt_history, diode_history, vec![Some(snapshot)])
    }

    #[test]
    fn accepted_junction_history_capture_and_restore_preserve_every_runtime_field() {
        let (circuit, bjt_history, diode_history, snapshot_cache) =
            accepted_junction_history_fixture();
        let checkpoint = Engine::capture_accepted_junction_transient_history_checkpoint(
            &circuit,
            &bjt_history,
            &diode_history,
            &snapshot_cache,
        );
        assert!(checkpoint.available);
        assert!(checkpoint.resume_blockers.is_empty());
        assert_eq!(
            checkpoint.bjt_names,
            vec![circuit.bjts.devices[0].name.clone()]
        );
        assert_eq!(
            checkpoint.bjt_runtime_tags,
            vec![BJT_TRANSIENT_HISTORY_RUNTIME_TAG.to_string()]
        );
        assert_eq!(
            checkpoint.diode_names,
            vec![circuit.diodes.devices[0].name.clone()]
        );
        assert_eq!(
            checkpoint.diode_runtime_tags,
            vec![DIODE_TRANSIENT_HISTORY_RUNTIME_TAG.to_string()]
        );
        Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &checkpoint)
            .expect("captured history validates");

        let (restored_bjt, restored_diode, restored_cache) =
            Engine::restore_accepted_junction_transient_history_checkpoint(&circuit, &checkpoint)
                .expect("history restores");
        assert_eq!(restored_bjt, bjt_history);
        assert_eq!(restored_diode, diode_history);
        let restored_cache_checkpoint = circuit.bjts.devices[0]
            .encode_accepted_charge_snapshot_checkpoint(
                restored_cache[0]
                    .as_ref()
                    .expect("snapshot remains present"),
            )
            .expect("restored snapshot re-encodes");
        assert_eq!(
            checkpoint.vbic_snapshot_cache[0].as_ref(),
            Some(&restored_cache_checkpoint)
        );
    }

    #[test]
    fn accepted_junction_history_order_one_normalization_reuses_flattening_contract() {
        let (circuit, bjt_history, diode_history, snapshot_cache) =
            accepted_junction_history_fixture();
        let checkpoint = Engine::capture_accepted_junction_transient_history_checkpoint(
            &circuit,
            &bjt_history,
            &diode_history,
            &snapshot_cache,
        );
        let mut expected_bjt = bjt_history.clone();
        let mut expected_diode = diode_history.clone();
        Engine::flatten_bjt_and_diode_histories_for_order_one_restart(
            &mut expected_bjt,
            &mut expected_diode,
            0.125,
        );

        let normalized =
            Engine::normalize_accepted_junction_transient_history_checkpoint_for_order_one(
                &circuit,
                &checkpoint,
                0.125,
            )
            .expect("physical-breakpoint history normalizes");
        assert_eq!(normalized.bjt_history, expected_bjt);
        assert_eq!(normalized.diode_history, expected_diode);
        assert_eq!(normalized.vbic_snapshot_cache, vec![None]);
        assert!(checkpoint.vbic_snapshot_cache[0].is_some());
        assert_eq!(
            normalized.bjt_history.accepted_terminal_currents,
            bjt_history.accepted_terminal_currents
        );
    }

    #[test]
    fn accepted_junction_history_validation_rejects_identity_shape_and_numeric_corruption() {
        let (circuit, bjt_history, diode_history, snapshot_cache) =
            accepted_junction_history_fixture();
        let checkpoint = Engine::capture_accepted_junction_transient_history_checkpoint(
            &circuit,
            &bjt_history,
            &diode_history,
            &snapshot_cache,
        );

        let mut wrong_name = checkpoint.clone();
        wrong_name.bjt_names[0].push_str("-wrong");
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &wrong_name)
                .unwrap_err()
                .contains("instance mismatch")
        );

        let mut wrong_shape = checkpoint.clone();
        wrong_shape.diode_history.qd_prev.pop();
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &wrong_shape)
                .unwrap_err()
                .contains("shape mismatch")
        );

        let mut non_finite = checkpoint.clone();
        non_finite.bjt_history.accepted_terminal_currents[0] =
            Some([Value::NAN; BJT_EXTERNAL_STATE_DIM]);
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &non_finite,)
                .unwrap_err()
                .contains("non-finite")
        );

        let mut negative_dt = checkpoint.clone();
        negative_dt.diode_history.accepted_dt_prev = -1.0;
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(
                &circuit,
                &negative_dt,
            )
            .unwrap_err()
            .contains("nonnegative")
        );

        let mut bad_cache = checkpoint.clone();
        bad_cache.vbic_snapshot_cache[0]
            .as_mut()
            .expect("snapshot exists")
            .state_values[0] = Value::INFINITY;
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &bad_cache)
                .unwrap_err()
                .contains("non-finite")
        );

        let unavailable = AcceptedJunctionTransientHistoryCheckpoint::unavailable(
            "legacy checkpoint has no transient history",
        );
        assert!(
            Engine::validate_accepted_junction_transient_history_checkpoint(
                &circuit,
                &unavailable,
            )
            .unwrap_err()
            .contains("unavailable")
        );
    }

    #[test]
    fn mosfet_gate_generation_rotation_preserves_all_three_accepted_levels() {
        let mut history = MosfetTransientHistory {
            vgs_prev: vec![1.0],
            vgs_prev_prev: vec![2.0],
            vgd_prev: vec![3.0],
            vgd_prev_prev: vec![4.0],
            vgb_prev: vec![5.0],
            vgb_prev_prev: vec![6.0],
            qgs_prev: vec![10.0],
            qgs_prev_prev: vec![20.0],
            qgs_prev_prev_prev: vec![30.0],
            qgd_prev: vec![40.0],
            qgd_prev_prev: vec![50.0],
            qgd_prev_prev_prev: vec![60.0],
            qgb_prev: vec![70.0],
            qgb_prev_prev: vec![80.0],
            qgb_prev_prev_prev: vec![90.0],
            ..Default::default()
        };

        history.rotate_gate_generations(false);

        assert_eq!((history.vgs_prev[0], history.vgs_prev_prev[0]), (2.0, 1.0));
        assert_eq!((history.vgd_prev[0], history.vgd_prev_prev[0]), (4.0, 3.0));
        assert_eq!((history.vgb_prev[0], history.vgb_prev_prev[0]), (6.0, 5.0));
        assert_eq!(
            (
                history.qgs_prev[0],
                history.qgs_prev_prev[0],
                history.qgs_prev_prev_prev[0],
            ),
            (30.0, 10.0, 20.0),
        );
        assert_eq!(
            (
                history.qgd_prev[0],
                history.qgd_prev_prev[0],
                history.qgd_prev_prev_prev[0],
            ),
            (60.0, 40.0, 50.0),
        );
        assert_eq!(
            (
                history.qgb_prev[0],
                history.qgb_prev_prev[0],
                history.qgb_prev_prev_prev[0],
            ),
            (90.0, 70.0, 80.0),
        );

        let frozen_charge = history.qgs_prev.clone();
        history.rotate_gate_generations(true);
        assert_eq!(history.qgs_prev, frozen_charge);
        assert_eq!((history.vgs_prev[0], history.vgs_prev_prev[0]), (1.0, 2.0));
    }
}
