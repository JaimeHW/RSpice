//! VBIC transient hidden-state snapshot helpers.

use super::*;
use crate::device::semiconductor::VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT;
use crate::device::{BjtType, NonlinearConvergenceCriteria, NonlinearDevice};

/// The integration state a VBIC charge solve steps from: the companion
/// coefficients and the step they were derived for, plus the two accepted
/// charge samples and the companion current that the integrator differences
/// against. A solve that had four of these and not the fifth would be
/// integrating against a step it was not built for, so they travel together.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct VbicChargeStep<'a> {
    pub coeff: &'a CompanionCoefficients,
    pub dt: Value,
    pub q_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
    pub q_prev_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
    pub cq_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
}

/// What the previous two accepted steps left for the VBIC predictor: the
/// internal state at each, the linear branch state at each, and the step size
/// that separated them. A predictor given a subset of these would extrapolate
/// from a history it cannot date.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct VbicPredictorHistory<'a> {
    pub internal_prev: Option<&'a [Value; BJT_INTERNAL_STATE_DIM]>,
    pub internal_prev_prev: Option<&'a [Value; BJT_INTERNAL_STATE_DIM]>,
    pub linear_prev: Option<&'a VbicPredictorLinearBranchState>,
    pub linear_prev_prev: Option<&'a VbicPredictorLinearBranchState>,
    pub previous_dt: Value,
}

/// The partitioned companion system a legacy BJT branch stamps into: the four
/// conductance blocks of the internal/external split and the two equivalent
/// current vectors that go with them. A branch always writes the block its
/// terminals fall in and the vector on that side, so the six are one
/// destination, not six.
pub(in crate::engine::transient) struct VbicCompanionSystem<'a> {
    pub g_ii: &'a mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ie: &'a mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ei: &'a mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub g_ee: &'a mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub z_i: &'a mut [Value; BJT_INTERNAL_STATE_DIM],
    pub z_e: &'a mut [Value; BJT_EXTERNAL_STATE_DIM],
}

/// The four external terminal voltages of a BJT: collector, base, emitter and
/// substrate. They were threaded as four bare `Value`s through every VBIC
/// dynamic entry point, where transposing base and emitter typechecks and
/// silently reverses the junction.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct BjtExternalBias {
    pub vc: Value,
    pub vb: Value,
    pub ve: Value,
    pub vs: Value,
}

/// The tolerances a cached VBIC snapshot is judged reusable against.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct VbicSnapshotTolerances {
    pub voltage_abstol: Value,
    pub reltol: Value,
}

/// The two-step internal history a VBIC predictor seeds from.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct VbicSeedHistory<'a> {
    pub internal_prev: Option<&'a [Value; BJT_INTERNAL_STATE_DIM]>,
    pub internal_prev_prev: Option<&'a [Value; BJT_INTERNAL_STATE_DIM]>,
    pub linear_prev: Option<&'a VbicPredictorLinearBranchState>,
    pub linear_prev_prev: Option<&'a VbicPredictorLinearBranchState>,
}

/// Where a VBIC internal-state improvement currently stands.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct VbicInternalStateProgress {
    pub current_internal: [Value; BJT_INTERNAL_STATE_DIM],
    pub current_residual_norm: Value,
    pub current_residual_objective: Value,
    pub target_internal: [Value; BJT_INTERNAL_STATE_DIM],
    pub envelope_reference: [Value; BJT_INTERNAL_STATE_DIM],
}

mod continuation;
mod convergence;
mod linearization;
mod snapshot_solve;
mod state_evaluation;
