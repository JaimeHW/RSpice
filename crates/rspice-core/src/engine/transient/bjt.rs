//! BJT transient charge linearization and private Gummel-Poon state solves.

use super::*;
use crate::device::BjtType;

/// The integration state a BJT charge solve steps from: the companion
/// coefficients and the step they were derived for, plus the two accepted
/// charge samples and the companion current that the integrator differences
/// against. A solve that had four of these and not the fifth would be
/// integrating against a step it was not built for, so they travel together.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct BjtChargeStep<'a> {
    pub coeff: &'a CompanionCoefficients,
    pub dt: Value,
    pub q_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
    pub q_prev_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
    pub cq_prev: &'a [Value; BJT_DYNAMIC_CHARGE_COUNT],
}

/// Accepted state for the private BJT predictor: the latest internal nodes,
/// the previous two linear branch states, and the step size separating them.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct BjtPredictorHistory<'a> {
    pub internal_prev: Option<&'a [Value; BJT_INTERNAL_STATE_DIM]>,
    pub linear_prev: Option<&'a BjtPredictorLinearBranchState>,
    pub linear_prev_prev: Option<&'a BjtPredictorLinearBranchState>,
    pub previous_dt: Value,
}

/// The partitioned companion system a legacy BJT branch stamps into: the four
/// conductance blocks of the internal/external split and the two equivalent
/// current vectors that go with them. A branch always writes the block its
/// terminals fall in and the vector on that side, so the six are one
/// destination, not six.
pub(in crate::engine::transient) struct BjtCompanionSystem<'a> {
    pub g_ii: &'a mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ie: &'a mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ei: &'a mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub g_ee: &'a mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    pub z_i: &'a mut [Value; BJT_INTERNAL_STATE_DIM],
    pub z_e: &'a mut [Value; BJT_EXTERNAL_STATE_DIM],
}

mod linearization;
mod snapshot;
