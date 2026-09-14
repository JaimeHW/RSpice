//! BJT transient charge linearization and private Gummel-Poon state solves.

use super::*;
use crate::device::BjtType;

impl Engine {
    pub(in crate::engine) fn initialize_bjt_phase_history(
        circuit: &crate::circuit::CircuitData,
        history: &mut BjtTransientHistory,
    ) -> Result<(), SimulationError> {
        let mut phase = Vec::with_capacity(circuit.bjts.devices.len());
        for (index, bjt) in circuit.bjts.devices.iter().enumerate() {
            let delay = bjt.legacy_excess_phase_delay();
            if delay == 0.0 {
                phase.push(None);
                continue;
            }
            let forward = bjt
                .legacy_forward_transport_branch(&history.dynamic_internal_prev[index])
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "BJT '{}' has no GP forward transport",
                        bjt.name
                    ))
                })?;
            let mut buffer = rspice_veriloga_runtime::transport_delay::DelayBuffer::new(4);
            buffer
                .accept_sample(0.0, forward.current, delay, None)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "BJT '{}' phase initialization: {error}",
                        bjt.name
                    ))
                })?;
            phase.push(Some(buffer));
        }
        history.phase = phase;
        Ok(())
    }

    /// GP PTF currently has an AC/noise operator but no transient history.
    /// Refuse before startup effects, integration, or checkpoint publication.
    /// Remove this admission boundary only with delay residuals, accepted
    /// history, error control, and restoration implemented together.
    pub(in crate::engine) fn ensure_bjt_transient_phase_support(
        circuit: &crate::circuit::CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (index, bjt) in circuit.bjts.devices.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let delay = bjt.legacy_excess_phase_delay();
            if delay != 0.0 {
                return Err(SimulationError::unsupported_capability(
                    "analysis.tran.bjt_excess_phase",
                    format!(
                        "BJT '{}': transient GP PTF excess phase (nominal delay {delay:.17e} s) is not implemented; forward-transport delay history, timestep error control, and restart state are unavailable",
                        bjt.name
                    ),
                ));
            }
        }
        Ok(())
    }
}

impl BjtTransientHistory {
    pub(in crate::engine::transient) fn phase_trial(
        &self,
        index: usize,
        time: Value,
    ) -> Option<BjtPhaseTrial<'_>> {
        self.phase[index].as_ref().map(|history| BjtPhaseTrial {
            history,
            time,
            left_limit: None,
            incoming_arrival: false,
        })
    }
}

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
    pub phase: Option<BjtPhaseTrial<'a>>,
}

/// Accepted transport memory read by one speculative electrical timepoint.
/// The nonlinear solve cannot append, replace, or prune that memory.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct BjtPhaseTrial<'a> {
    pub history: &'a rspice_veriloga_runtime::transport_delay::DelayBuffer,
    pub time: Value,
    /// Forward transport from a separately solved incoming electrical state.
    /// Right-side Newton probes hold this value fixed through every private
    /// solve, reduction and promoted stamp. It is never a mutable history or
    /// a value inferred from the current right-side iterate.
    pub left_limit: Option<Value>,
    /// Select the incoming side of delayed events during the left equation
    /// solve. An incoming solve has no held outgoing input endpoint.
    pub incoming_arrival: bool,
}

impl BjtPhaseTrial<'_> {
    /// The previous accepted phase current is a physical static DAE term.
    /// OneStep's half-current history must include it even though the generic
    /// static device sampler has no access to the engine's transport memory.
    fn accepted_correction_current(self, bjt: &crate::device::Bjt) -> Result<Value, String> {
        let (time, forward) = self
            .history
            .accepted_samples()
            .next_back()
            .ok_or("GP phase has no accepted forward-current sample")?;
        Ok(self
            .history
            .difference_with_coefficients(time, forward, bjt.legacy_excess_phase_delay(), None)?
            .output)
    }

    pub(in crate::engine::transient) fn stamp_promoted(
        self,
        bjt: &crate::device::Bjt,
        stamper: &mut impl crate::device::MatrixStamper,
        xyce_one_step_order2: bool,
    ) -> Result<(), String> {
        let (_, internal, _) = bjt.mna_charge_state();
        let mut correction = self.correction(bjt, &internal)?;
        let weight = if xyce_one_step_order2 {
            correction.current =
                0.5 * correction.current + 0.5 * self.accepted_correction_current(bjt)?;
            for derivative in correction
                .d_internal
                .iter_mut()
                .chain(&mut correction.d_external)
            {
                *derivative *= 0.5;
            }
            0.5
        } else {
            1.0
        };
        bjt.stamp_legacy_mna_phase_correction(stamper, &correction, weight)
    }

    pub(in crate::engine::transient) fn correction(
        self,
        bjt: &crate::device::Bjt,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Result<crate::device::semiconductor::BjtCurrentBranch, String> {
        if self.incoming_arrival && self.left_limit.is_some() {
            return Err("incoming GP phase trial cannot hold an outgoing input endpoint".into());
        }
        let mut branch = bjt
            .legacy_forward_transport_branch(internal)
            .ok_or_else(|| {
                "GP transport history was attached to a different BJT model".to_owned()
            })?;
        let delay = bjt.legacy_excess_phase_delay();
        if !delay.is_finite() || delay <= 0.0 {
            return Err("GP transport requires a finite positive nominal phase delay".into());
        }
        let retained_delay =
            self.history
                .small_signal_delay(self.time, branch.current, delay, None)?;
        if retained_delay.to_bits() != delay.to_bits() {
            return Err("GP transport history belongs to a different nominal phase delay".into());
        }
        let evaluation = if self.incoming_arrival {
            self.history
                .difference_before_arrival(self.time, branch.current, delay, None)?
        } else if let Some(left) = self.left_limit {
            self.history.difference_at_discontinuity(
                self.time,
                left,
                branch.current,
                delay,
                None,
            )?
        } else {
            self.history
                .difference_with_coefficients(self.time, branch.current, delay, None)?
        };
        branch.current = evaluation.output;
        for derivative in branch.d_internal.iter_mut().chain(&mut branch.d_external) {
            *derivative = evaluation.apply_input_derivative(*derivative)?;
        }
        Ok(branch)
    }
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

pub(in crate::engine::transient) mod arrival;
mod event_context;
pub(in crate::engine::transient) mod interpolation;
pub(in crate::engine) use event_context::BjtPhaseContext;
mod linearization;
mod snapshot;

#[cfg(test)]
mod phase_tests;
