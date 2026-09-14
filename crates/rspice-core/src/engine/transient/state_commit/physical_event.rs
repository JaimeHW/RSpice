//! Accepted finite device histories at an instantaneous charge/flux event.
//! The incoming electrical limit is supplied by its independent solve. No
//! impulse is divided by an integration timestep to construct these values.

use super::*;
use crate::circuit::SourceTimeSide;
use charge_event::circuit::{EventPhase, PreparedEventCircuit};

pub(in crate::engine::transient) struct PhysicalEventStep<'a> {
    pub incoming: &'a [Value],
    pub time: Value,
    pub dt: Value,
    /// Physical input events identified by the event owner, including equal
    /// value corners. Ordinary accepted interpolation knots are not events.
    pub phase_events: &'a [bool],
}

#[must_use]
pub(in crate::engine::transient) struct PreparedPhysicalEvent {
    state: charge_event::ChargeEventState,
    source_branches: Vec<usize>,
    phase_current_couplings: Vec<Option<charge_event::CurrentJumpCoupling>>,
    time: Value,
    dt: Value,
    pub(super) bjt: PreparedBjtHistory,
    pub(super) capacitors: Vec<CapacitorAcceptedState>,
    left_limits: Vec<Option<Value>>,
    phase_anchors: Vec<Option<(usize, Value, Value)>>,
}

fn phase_anchors(history: &BjtTransientHistory) -> Vec<Option<(usize, Value, Value)>> {
    history
        .phase
        .iter()
        .map(|phase| {
            phase.as_ref().and_then(|phase| {
                phase
                    .accepted_samples()
                    .next_back()
                    .map(|(time, value)| (phase.accepted_sample_count(), time, value))
            })
        })
        .collect()
}

fn failure(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("physical event acceptance: {}", message.into()))
}

fn sum(terms: impl Iterator<Item = (Value, Value)> + Clone) -> Result<Value, SimulationError> {
    let value = rspice_veriloga_runtime::arithmetic::sum_products(terms)
        .map_err(|_| failure("unrepresentable finite device current"))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(failure("nonfinite device current"))
    }
}

fn rate(rates: &[Option<Value>], node: usize) -> Result<Value, SimulationError> {
    if node == 0 {
        return Ok(0.0);
    }
    rates
        .get(node - 1)
        .copied()
        .flatten()
        .filter(|value| value.is_finite())
        .ok_or_else(|| failure("device storage requires an unresolved coordinate rate"))
}

impl PreparedPhysicalEvent {
    /// Per-device delayed-current incidence in the successfully solved event
    /// topology. None denotes a device without GP delay. This is a structural
    /// input for propagation classification, not the order of this event;
    /// simultaneous source changes and model regularity still matter.
    pub(in crate::engine::transient) fn phase_current_couplings(
        &self,
    ) -> &[Option<charge_event::CurrentJumpCoupling>] {
        &self.phase_current_couplings
    }

    pub(in crate::engine::transient) fn state(&self) -> &charge_event::ChargeEventState {
        &self.state
    }

    /// Integrated current in coulombs, bound to its original MNA coordinate.
    pub(in crate::engine::transient) fn impulses(
        &self,
    ) -> impl ExactSizeIterator<Item = (usize, Value)> + '_ {
        self.source_branches
            .iter()
            .copied()
            .zip(self.state.source_impulses.iter().copied())
    }

    pub(in crate::engine::transient) fn phase_context(&self) -> bjt::BjtPhaseContext<'_> {
        bjt::BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&self.left_limits),
        }
    }

    pub(super) fn validate(
        &self,
        circuit: &crate::CircuitData,
        history: &BjtTransientHistory,
        step: AcceptedReactiveStep<'_>,
        context: bjt::BjtPhaseContext<'_>,
    ) -> Result<(), SimulationError> {
        context.bind(history)?;
        if self.time.to_bits() != step.accepted_time.to_bits()
            || self.dt.to_bits() != step.dt.to_bits()
            || self.state.solution.len() != step.accepted_solution.len()
            || !self
                .state
                .solution
                .iter()
                .zip(step.accepted_solution)
                .all(|(a, b)| a.to_bits() == b.to_bits())
            || self.bjt.values.len() != circuit.bjts.len()
            || self.capacitors.len() != circuit.capacitors.len()
            || context.incoming_arrival
            || context.input_left_limits != Some(self.left_limits.as_slice())
            || self.phase_anchors != phase_anchors(history)
        {
            return Err(failure(
                "prepared event does not match the final solution, time or history context",
            ));
        }
        // Revalidate retained phase ownership at the joint acceptance barrier.
        // An intervening history advance cannot consume a stale preparation.
        for (index, value) in self.bjt.values.iter().enumerate() {
            if let Some(sample) = value.phase_sample {
                let history = history.phase[index]
                    .as_ref()
                    .ok_or_else(|| failure("missing prepared phase history"))?;
                let delay = history
                    .small_signal_delay(self.time, sample.current, sample.delay, None)
                    .map_err(failure)?;
                if delay.to_bits() != sample.delay.to_bits() {
                    return Err(failure("prepared phase delay has changed"));
                }
                if let Some(left) = sample.left_limit {
                    history.validate_discontinuity(
                        self.time,
                        left,
                        sample.current,
                        sample.delay,
                        None,
                    )
                } else {
                    history.validate_sample(self.time, sample.current, sample.delay, None)
                }
                .map_err(failure)?;
            }
        }
        Ok(())
    }
}

impl Engine {
    pub(in crate::engine::transient) fn prepare_physical_event(
        &self,
        circuit: &crate::CircuitData,
        history: &BjtTransientHistory,
        step: PhysicalEventStep<'_>,
        options: &charge_event::EventOptions,
        flux_tolerance: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPhysicalEvent, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !step.dt.is_finite()
            || step.dt <= 0.0
            || step.phase_events.len() != circuit.bjts.len()
            || history.phase.len() != circuit.bjts.len()
        {
            return Err(failure(
                "invalid incoming interval or physical event population",
            ));
        }
        bjt::BjtPhaseContext::default().bind(history)?;
        let mut sampler = PreparedEventCircuit::new(circuit, flux_tolerance, options, abort)?;
        let inputs = sampler.forward_inputs(step.incoming, abort)?;
        let phases: Vec<_> = history
            .phase
            .iter()
            .zip(&inputs)
            .map(|(history, input)| match (history, input) {
                (Some(history), Some(endpoint)) => Ok(Some(EventPhase {
                    history,
                    endpoint: *endpoint,
                })),
                (None, None) => Ok(None),
                _ => Err(failure(
                    "transport history differs from the prepared model population",
                )),
            })
            .collect::<Result<_, _>>()?;
        let incoming = sampler.sample(
            step.time,
            SourceTimeSide::LeftLimit,
            step.incoming,
            &phases,
            options,
            abort,
        )?;
        let topology = sampler.topology(step.time, SourceTimeSide::RightLimit, options, abort)?;
        let state = topology.solve(
            step.incoming,
            incoming.charge_values(),
            options,
            abort,
            |solution, abort| {
                sampler.sample(
                    step.time,
                    SourceTimeSide::RightLimit,
                    solution,
                    &phases,
                    options,
                    abort,
                )
            },
        )?;
        let mut phase_current_couplings = Vec::with_capacity(sampler.models().len());
        for model in sampler.models() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let coupling = if model.legacy_excess_phase_delay() == 0.0 {
                None
            } else {
                let (p, n) = model.legacy_forward_transport_nodes().ok_or_else(|| {
                    failure(format!(
                        "BJT '{}' has no prepared GP current port",
                        model.name
                    ))
                })?;
                Some(topology.current_jump_coupling(p, n)?)
            };
            phase_current_couplings.push(coupling);
        }
        let mut capacitors = Vec::with_capacity(circuit.capacitors.len());
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let p = stamp.pp.row;
            let n = stamp.nn.row;
            let c = circuit.capacitors.capacitances[index];
            capacitors.push(CapacitorAcceptedState {
                voltage: Self::differential_voltage(&state.solution, p, n),
                current: sum([
                    (rate(&state.coordinate_rates, p)?, c),
                    (rate(&state.coordinate_rates, n)?, -c),
                ]
                .into_iter())?,
            });
        }
        let mut values = Vec::with_capacity(circuit.bjts.len());
        let mut left_limits = Vec::with_capacity(circuit.bjts.len());
        for (index, model) in sampler.models().iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let solution = &state.solution;
            let (branches, internal, external) = model.mna_charge_state_at_solution(solution);
            let internal_rates: Vec<_> = (0..BJT_INTERNAL_STATE_DIM)
                .map(|node| rate(&state.coordinate_rates, model.mna_internal_node(node)))
                .collect::<Result<_, _>>()?;
            let terminals = [
                model.node_collector,
                model.node_base,
                model.node_emitter,
                model.node_substrate,
            ];
            let external_rates: Vec<_> = terminals
                .iter()
                .map(|&node| rate(&state.coordinate_rates, node))
                .collect::<Result<_, _>>()?;
            let mut charges = branches.map(|branch| branch.charge);
            let mut currents = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
            for (current, branch) in currents.iter_mut().zip(&branches) {
                *current = sum(branch
                    .d_internal
                    .iter()
                    .copied()
                    .zip(internal_rates.iter().copied())
                    .chain(
                        branch
                            .d_external
                            .iter()
                            .copied()
                            .zip(external_rates.iter().copied()),
                    ))?;
            }
            if let Some(charge) = model.legacy_external_bc_charge(solution) {
                charges[BJT_QBCX_BRANCH_INDEX] = charge.charge;
                currents[BJT_QBCX_BRANCH_INDEX] = sum([
                    (
                        rate(&state.coordinate_rates, charge.nodes[0])?,
                        charge.capacitance,
                    ),
                    (
                        rate(&state.coordinate_rates, charge.nodes[1])?,
                        -charge.capacitance,
                    ),
                ]
                .into_iter())?;
            }
            let mut terminal = model.mna_terminal_currents_at_solution(solution);
            if let Some(phase) = phases[index] {
                let correction = bjt::BjtPhaseTrial {
                    history: phase.history,
                    time: step.time,
                    left_limit: Some(phase.endpoint),
                    incoming_arrival: false,
                }
                .correction(model, &internal)
                .map_err(failure)?;
                if let Some(i) = correction.pos_external {
                    terminal[i] += correction.current;
                }
                if let Some(i) = correction.neg_external {
                    terminal[i] -= correction.current;
                }
            }
            for (branch, current) in branches.iter().zip(currents) {
                if let Some(i) = branch.pos_external {
                    terminal[i] += current;
                }
                if let Some(i) = branch.neg_external {
                    terminal[i] -= current;
                }
            }
            let selected = step.phase_events[index];
            let phase_sample = if let Some(phase) = phases[index] {
                let forward = model
                    .legacy_forward_transport_branch(&internal)
                    .ok_or_else(|| failure("missing GP input equation"))?
                    .current;
                if !selected && forward != phase.endpoint {
                    return Err(failure(format!(
                        "BJT '{}' changed input without a physical event record",
                        model.name
                    )));
                }
                let left_limit = selected.then_some(phase.endpoint);
                let delay = model.legacy_excess_phase_delay();
                if let Some(left) = left_limit {
                    phase
                        .history
                        .validate_discontinuity(step.time, left, forward, delay, None)
                } else {
                    phase
                        .history
                        .validate_sample(step.time, forward, delay, None)
                }
                .map_err(failure)?;
                left_limits.push(left_limit);
                Some(AcceptedBjtPhaseSample {
                    current: forward,
                    delay,
                    left_limit,
                })
            } else {
                if selected {
                    return Err(failure(
                        "physical phase event attached to a model without delay",
                    ));
                }
                left_limits.push(None);
                None
            };
            let linear = (!circuit.bjts.devices[index].mna_promoted())
                .then(|| Self::bjt_predictor_linear_branch_state(model, external, internal));
            let voltages = model.mna_junction_voltages(solution);
            if !charges
                .iter()
                .chain(&currents)
                .chain(&internal)
                .chain(&voltages)
                .chain(&terminal)
                .all(|value| value.is_finite())
            {
                return Err(failure(format!(
                    "BJT '{}' has nonfinite physical event history",
                    model.name
                )));
            }
            values.push(AcceptedBjtValues {
                phase_sample,
                charges,
                currents,
                internal,
                linear,
                voltages,
                lead_currents: Some(terminal),
            });
        }
        let bjt = PreparedBjtHistory {
            values,
            dt: step.dt,
            accepted_time: step.time,
        };
        if let Some(control) = bjt.phase_step_control(
            circuit,
            history,
            self.transient_lte_reltol(),
            self.current_abstol(),
        )? {
            control.ensure_acceptable(circuit)?;
        }
        Ok(PreparedPhysicalEvent {
            state,
            source_branches: topology.source_branches().collect(),
            phase_current_couplings,
            time: step.time,
            dt: step.dt,
            bjt,
            capacitors,
            left_limits,
            phase_anchors: phase_anchors(history),
        })
    }
}

#[cfg(test)]
mod tests;
