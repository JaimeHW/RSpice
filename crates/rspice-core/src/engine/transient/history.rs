//! Transient solver limits and per-device accepted-state history.

use super::*;

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
/// Largest single source movement to allow on proactive nonlinear ramp tracking.
pub(super) const SOURCE_RAMP_TRACKING_DELTA: Value = 5e-2;
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
#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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
    pub(super) dynamic_internal_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    pub(super) dynamic_internal_prev_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    pub(super) dynamic_linear_prev: Vec<VbicPredictorLinearBranchState>,
    pub(super) dynamic_linear_prev_prev: Vec<VbicPredictorLinearBranchState>,
    pub(super) accepted_dt_prev: Value,
    pub(super) accepted_dt_prev_prev: Value,
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

#[derive(Debug, Clone, Copy, Default)]
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
