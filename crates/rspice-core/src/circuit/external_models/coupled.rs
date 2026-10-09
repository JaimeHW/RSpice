//! XSPICE participation in the circuit-owned HDL Active region.
//! Bindings describe original code-model outputs; resolved observations never
//! become drivers. The owner retains this participant for a complete settle
//! and includes its circuit state in the same trial as the HDL host.
use super::*;
use crate::xspice::DigitalValue;
use crate::xspice::event_scheduler::EventTarget;
#[cfg(test)]
use crate::xspice::verilog::host::DigitalHost;
use crate::xspice::verilog::host::{
    DigitalActiveExchange, DigitalActiveParticipant, DigitalRunError,
};
use crate::xspice::verilog::store::{ExternalBitDriverId, ExternalNetChange, ExternalRealDriverId};
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use std::collections::{BTreeSet, VecDeque};

/// Immutable connections between physical circuit node identities and resolved
/// HDL bit groups and real signals. Driver indices match XspiceInstance::schedule_events.
#[derive(Clone, Debug)]
pub(crate) struct XspiceDigitalBindings {
    by_node: BTreeMap<NodeId, usize>,
    by_net: BTreeMap<usize, NodeId>,
    drivers: BTreeMap<EventTarget, ExternalBitDriverId>,
    /// Store driver id -> code-model owner and original port element.
    inout_observers: BTreeMap<usize, (usize, EventTarget)>,
    real_by_node: BTreeMap<NodeId, DigitalSignalId>,
    real_by_signal: BTreeMap<DigitalSignalId, NodeId>,
    real_drivers: BTreeMap<EventTarget, ExternalRealDriverId>,
}

impl XspiceDigitalBindings {
    pub(super) fn checkpoint_observations(
        &self,
        owner: &crate::xspice::verilog::MixedDigitalCoordinator,
    ) -> Result<Vec<(NodeId, crate::xspice::EventValue)>, String> {
        let mut bits = BTreeMap::new();
        let mut reals = BTreeMap::new();
        for (node, value) in owner
            .event_values()
            .filter(|(node, _)| self.by_node.contains_key(node))
        {
            if bits.insert(node, value).is_some_and(|old| old != value) {
                return Err("HDL aliases disagree at an XSPICE bit node".into());
            }
        }
        for (node, value) in owner
            .real_event_values()
            .filter(|(node, _)| self.real_by_node.contains_key(node))
        {
            if reals
                .insert(node, value)
                .is_some_and(|old: f64| old.to_bits() != value.to_bits())
            {
                return Err("HDL aliases disagree at an XSPICE real node".into());
            }
        }
        if bits.len() != self.by_node.len() || reals.len() != self.real_by_node.len() {
            return Err("restored HDL owner is missing an XSPICE shared observation".into());
        }
        Ok(bits
            .into_iter()
            .map(|(node, value)| (node, crate::xspice::EventValue::Digital(value)))
            .chain(
                reals
                    .into_iter()
                    .map(|(node, value)| (node, crate::xspice::EventValue::Real(value))),
            )
            .collect())
    }

    pub(crate) fn enroll_circuit(
        circuit: &CircuitData,
        coordinator: &mut crate::xspice::verilog::MixedDigitalCoordinator,
    ) -> Result<Option<Self>, DigitalRunError> {
        let nets: Vec<_> = coordinator.event_bindings().collect();
        let bits = Self::enroll_with(circuit, &nets, |observed, drivers, inouts| {
            let ids = coordinator.attach_external_bits(observed, drivers)?;
            coordinator.observe_other_drivers(
                &inouts.iter().map(|&index| ids[index]).collect::<Vec<_>>(),
            )?;
            Ok(ids)
        })?;
        let offered: BTreeMap<_, _> = coordinator.real_event_bindings().iter().copied().collect();
        let mut connected = BTreeSet::new();
        let mut targets = Vec::new();
        for instance in &circuit.xspice_instances {
            instance.for_each_event_input_net(|kind, node| {
                if kind == EventInputKind::Real && offered.contains_key(&node) {
                    connected.insert(node);
                }
            });
            instance.for_each_real_output_driver(|target| {
                if offered.contains_key(&target.node_id) {
                    connected.insert(target.node_id);
                    targets.push(target);
                }
            });
        }
        if connected.is_empty() {
            return Ok(bits);
        }
        let mut result = bits.unwrap_or_else(|| Self {
            by_node: BTreeMap::new(),
            by_net: BTreeMap::new(),
            drivers: BTreeMap::new(),
            inout_observers: BTreeMap::new(),
            real_by_node: BTreeMap::new(),
            real_by_signal: BTreeMap::new(),
            real_drivers: BTreeMap::new(),
        });
        result.real_by_node = connected
            .into_iter()
            .map(|node| (node, offered[&node]))
            .collect();
        result.real_by_signal = result
            .real_by_node
            .iter()
            .map(|(&node, &signal)| (signal, node))
            .collect();
        targets.sort();
        let targets: Vec<_> = targets
            .into_iter()
            .map(|target| (offered[&target.node_id], target))
            .collect();
        let ids = coordinator.attach_external_reals(
            &result.real_by_node.values().copied().collect::<Vec<_>>(),
            &targets,
        )?;
        result.real_drivers = targets
            .into_iter()
            .map(|(_, target)| target)
            .zip(ids)
            .collect();
        Ok(Some(result))
    }

    #[cfg(test)]
    pub(crate) fn enroll(
        circuit: &CircuitData,
        digital: &mut DigitalHost,
        nets: &[(NodeId, usize)],
    ) -> Result<Option<Self>, DigitalRunError> {
        Self::enroll_with(circuit, nets, |observed, drivers, inouts| {
            let ids = digital.attach_external_bits(observed, drivers)?;
            digital.observe_other_drivers(
                &inouts.iter().map(|&index| ids[index]).collect::<Vec<_>>(),
            )?;
            Ok(ids)
        })
    }

    fn enroll_with(
        circuit: &CircuitData,
        nets: &[(NodeId, usize)],
        attach: impl FnOnce(
            &[usize],
            &[(usize, EventTarget)],
            &[usize],
        ) -> Result<Vec<ExternalBitDriverId>, DigitalRunError>,
    ) -> Result<Option<Self>, DigitalRunError> {
        let mut offered = BTreeMap::new();
        let mut net_ids = BTreeSet::new();
        for &(node, net) in nets {
            if node == 0 || offered.insert(node, net).is_some() || !net_ids.insert(net) {
                return Err(external_error(format!(
                    "shared XSPICE connections require distinct non-ground circuit nodes and bit groups; node {node}, group {net}"
                )));
            }
        }
        let mut connected = BTreeSet::new();
        let mut targets = Vec::new();
        let mut inout_sources = BTreeMap::new();
        for (index, instance) in circuit.xspice_instances.iter().enumerate() {
            instance.for_each_event_input_net(|kind, node| {
                if kind == EventInputKind::Digital && offered.contains_key(&node) {
                    connected.insert(node);
                }
            });
            instance.for_each_digital_output_driver(|target| {
                if offered.contains_key(&target.node_id) {
                    connected.insert(target.node_id);
                    if instance.ports().iter().any(|port| {
                        port.name == target.port_name
                            && port.direction == crate::xspice::PortDirection::InOut
                    }) {
                        inout_sources.insert(target.clone(), index);
                    }
                    targets.push(target);
                }
            });
        }
        if connected.is_empty() {
            return Ok(None);
        }
        let by_node: BTreeMap<_, _> = connected
            .into_iter()
            .map(|node| (node, offered[&node]))
            .collect();
        let by_net = by_node.iter().map(|(&node, &net)| (net, node)).collect();
        targets.sort();
        let targets: Vec<_> = targets
            .into_iter()
            .map(|target| (by_node[&target.node_id], target))
            .collect();
        let inouts: Vec<_> = targets
            .iter()
            .enumerate()
            .filter_map(|(index, (_, target))| inout_sources.contains_key(target).then_some(index))
            .collect();
        let ids = attach(
            &by_node.values().copied().collect::<Vec<_>>(),
            &targets,
            &inouts,
        )?;
        let inout_observers = inouts
            .into_iter()
            .map(|index| {
                let target = &targets[index].1;
                (ids[index].index(), (inout_sources[target], target.clone()))
            })
            .collect();
        Ok(Some(Self {
            by_node,
            by_net,
            inout_observers,
            drivers: targets
                .into_iter()
                .map(|(_, target)| target)
                .zip(ids)
                .collect(),
            real_by_node: BTreeMap::new(),
            real_by_signal: BTreeMap::new(),
            real_drivers: BTreeMap::new(),
        }))
    }
    pub(crate) fn contains_node(&self, node: NodeId) -> bool {
        self.by_node.contains_key(&node)
    }
    pub(crate) fn contains_real_node(&self, node: NodeId) -> bool {
        self.real_by_node.contains_key(&node)
    }

    pub(crate) fn remap_nodes(&mut self, remap: impl Fn(usize) -> usize) {
        for (_, target) in self.inout_observers.values_mut() {
            target.node_id = remap(target.node_id);
        }
        self.by_node = std::mem::take(&mut self.by_node)
            .into_iter()
            .map(|(node, net)| (remap(node), net))
            .collect();
        for node in self.by_net.values_mut() {
            *node = remap(*node);
        }
        self.drivers = std::mem::take(&mut self.drivers)
            .into_iter()
            .map(|(mut target, id)| {
                target.node_id = remap(target.node_id);
                (target, id)
            })
            .collect();
        self.real_by_node = std::mem::take(&mut self.real_by_node)
            .into_iter()
            .map(|(node, signal)| (remap(node), signal))
            .collect();
        for node in self.real_by_signal.values_mut() {
            *node = remap(*node);
        }
        self.real_drivers = std::mem::take(&mut self.real_drivers)
            .into_iter()
            .map(|(mut target, id)| {
                target.node_id = remap(target.node_id);
                (target, id)
            })
            .collect();
    }
}

fn external_error(detail: impl Into<String>) -> DigitalRunError {
    DigitalRunError::ExternalExecution {
        detail: detail.into(),
    }
}
fn model_error(error: impl std::fmt::Display) -> crate::xspice::CmError {
    crate::xspice::CmError::EvaluationError(error.to_string())
}

/// Borrowed code-model side of one candidate. Creating this object does not
/// advance state. Each callback executes at most one model dispatch wave, then
/// yields so the HDL host can run newly activated processes before later regions.
pub(crate) struct XspiceDigitalParticipant<'a> {
    circuit: &'a mut CircuitData,
    bindings: &'a XspiceDigitalBindings,
    solution: &'a [Value],
    resources: Option<&'a ResourceTransaction>,
    time: Value,
    timestep: Value,
    analysis: crate::xspice::AnalysisType,
    phase: crate::xspice::EvaluationPhase,
    coefficients: crate::numerics::integration::CompanionCoefficients,
    xyce_one_step_order2: bool,
    wave: Option<XspiceActiveWave>,
    pending: VecDeque<ExternalNetChange>,
    initialized: bool,
    analog_boundaries_ready: bool,
    interpolating: bool,
    interpolation_endpoint_window: f64,
    /// Node rows an earlier pass of this same candidate moved, and with them
    /// the fact that there *was* an earlier pass. Empty and `None` for the
    /// opening pass, which is the ordinary case.
    projected: &'a [NodeId],
    resettling: bool,
}

impl<'a> XspiceDigitalParticipant<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        circuit: &'a mut CircuitData,
        bindings: &'a XspiceDigitalBindings,
        solution: &'a [Value],
        time: Value,
        timestep: Value,
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion: XspiceCompanionPolicy<'_>,
        resources: Option<&'a ResourceTransaction>,
    ) -> Self {
        Self {
            circuit,
            bindings,
            solution,
            resources,
            time,
            timestep,
            analysis,
            phase,
            coefficients: *companion.coefficients,
            xyce_one_step_order2: companion.xyce_one_step_order2,
            wave: None,
            pending: VecDeque::new(),
            initialized: false,
            analog_boundaries_ready: false,
            interpolating: false,
            interpolation_endpoint_window: 0.0,
            projected: &[],
            resettling: false,
        }
    }

    /// Continue one candidate whose solution a voltage projection moved.
    ///
    /// The wave comes back in so that the models' accumulated analog
    /// transitions and the pass counter survive the projection, and `projected`
    /// names the node rows that moved. Together they make the re-settle
    /// dispatch exactly the instances whose inputs are now stale instead of
    /// re-running every instance's accepted-phase evaluation at a timepoint it
    /// has already been evaluated at.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn resume(
        circuit: &'a mut CircuitData,
        bindings: &'a XspiceDigitalBindings,
        solution: &'a [Value],
        time: Value,
        timestep: Value,
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion: XspiceCompanionPolicy<'_>,
        resources: Option<&'a ResourceTransaction>,
        wave: Option<XspiceActiveWave>,
        projected: &'a [NodeId],
    ) -> Self {
        let mut resumed = Self::new(
            circuit, bindings, solution, time, timestep, analysis, phase, companion, resources,
        );
        // Owed before the settle begins, because a resumed wave never reopens
        // and `begin_xspice_active_wave` — the other place these marks are
        // laid down — is what clears the pending flags.
        resumed
            .circuit
            .record_xspice_analog_input_dispatch(projected);
        resumed.initialized = wave.is_some();
        resumed.wave = wave;
        resumed.projected = projected;
        resumed.resettling = true;
        resumed
    }

    /// Hand the candidate's Active wave back to the projection loop.
    pub(crate) fn into_wave(self) -> Option<XspiceActiveWave> {
        self.wave
    }

    /// Whether a queued code-model event this candidate has already passed was
    /// one the analog solver could have opened a wave at.
    ///
    /// The event's own scheduler owns its exact time and its ordering; the
    /// analog solver owes it only a timepoint at or after it. An event a whole
    /// hard-minimum step or more after the accepted analog time is reachable:
    /// the stepper had a legal interval to it and did not take it, which is
    /// the synchronization fault `settle_active` refuses. One closer than that
    /// is not reachable at all — there is no analog instant between the
    /// accepted point and it — so it is delivered at the timepoint the stepper
    /// did land on, which is where `engine::transient`'s
    /// `landed_veriloga_event_time` coalesces it (`accepted + hard_min`) and
    /// where `step_xspice_active_wave_with_resolver` drains it in queue order.
    /// Refusing that one would end a run over a schedule the analog side is
    /// simply too coarse to resolve: a 1 ps gate delay — ngspice's clamped
    /// minimum — under the 10 ps minimum a one-second maximum timestep leaves
    /// the solver.
    ///
    /// This is the mixed half's rule, on the other kernel's queue:
    /// `xspice::verilog::mixed::shared::MixedDigitalCoordinator::activation_was_reachable`
    /// and `MixedSignalHost::begin_trial_with_integration_rules`' contract
    /// state it for HDL activations, against the same floor
    /// `CircuitScheduler::set_analog_step_floor` stores for both. A zero
    /// floor means no solver has declared one — a directly driven participant,
    /// or an analysis that never set it — and then every event stepped past is
    /// a fault, as it was before either half had a floor.
    ///
    /// The accepted time is the candidate's own start, `time - timestep`:
    /// `settle_active` already refuses physical work outside
    /// `[time - timestep, time]`, so that difference is the interval the
    /// stepper chose, and there is no earlier instant for an event to have
    /// been landed on.
    fn event_was_reachable(&self, due: Value) -> bool {
        let floor = self.circuit.scheduler.analog_step_floor();
        !floor.is_finite() || floor <= 0.0 || due - (self.time - self.timestep) >= floor
    }
}

impl DigitalActiveParticipant for XspiceDigitalParticipant<'_> {
    fn prepare_interpolated_execution(
        &mut self,
        endpoint_window: f64,
    ) -> Result<(), DigitalRunError> {
        self.interpolating = true;
        self.interpolation_endpoint_window = endpoint_window;
        let sources = self.circuit.current_sources.values_at_time(self.time);
        let transitions = self
            .circuit
            .xspice_instances
            .iter()
            .flat_map(|instance| instance.analog_output_transitions())
            .collect();
        let values = &self.circuit.scheduler.xspice_event_values;
        for instance in &mut self.circuit.xspice_instances {
            if instance.has_mixed_input_thresholds() {
                instance
                    .make_mut()
                    .update_inputs_with_analog_transitions(
                        self.solution,
                        self.circuit.num_nodes,
                        XspiceEventInputs {
                            digital_values: &values.digital_values,
                            digital_event_times: &values.digital_event_times,
                            event_total_loads: &self.circuit.xspice_event_loads,
                            real_values: &values.real_values,
                            real_event_times: &values.real_event_times,
                        },
                        &sources,
                        &transitions,
                    )
                    .map_err(|error| external_error(error.to_string()))?;
            }
        }
        Ok(())
    }

    fn next_event_time(&self) -> Option<f64> {
        self.circuit.scheduler.xspice_event_queue.next_event_time()
    }

    fn analog_feedback_pending(&self) -> bool {
        self.wave.as_ref().is_some_and(|wave| wave.analog_feedback)
    }

    fn analog_threshold_sample(&self, instance: usize, port: &str, element: usize) -> Option<f64> {
        self.circuit
            .xspice_instances
            .get(instance)?
            .analog_threshold_sample(port, element)
    }

    fn analog_boundaries_ready(&mut self) -> bool {
        if self.analog_boundaries_ready {
            return false;
        }
        self.analog_boundaries_ready = true;
        let mut released = false;
        for (index, instance) in self.circuit.xspice_instances.iter().enumerate() {
            if instance.has_mixed_input_thresholds() {
                self.circuit.xspice_dispatch_pending[index] = true;
                released = true;
            }
        }
        released
    }

    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        let physical = exchange.physical_seconds();
        let physical = if self.interpolating
            && physical.is_finite()
            && physical <= self.time
            && self.time - physical <= self.interpolation_endpoint_window
        {
            self.time
        } else {
            physical
        };
        if !physical.is_finite() || physical > self.time || physical < self.time - self.timestep {
            return Err(external_error(format!(
                "XSPICE Active work at {physical:.16e}s is outside candidate interval [{:.16e}, {:.16e}]s",
                self.time - self.timestep,
                self.time,
            )));
        }
        if self.wave.as_ref().is_none_or(|wave| wave.time != physical) {
            if !self.pending.is_empty()
                || self.wave.as_ref().is_some_and(|wave| physical < wave.time)
            {
                return Err(external_error(
                    "XSPICE physical time cannot advance with pending shared-net observations or move backwards within one candidate",
                ));
            }
            // See `Self::event_was_reachable`: only an event the stepper had a
            // legal interval to and skipped anyway is a lost breakpoint.
            if let Some(due) = self.circuit.scheduler.xspice_event_queue.next_event_time()
                && due < physical
                && self.event_was_reachable(due)
                && !(self.interpolating && physical - due <= self.interpolation_endpoint_window)
            {
                return Err(external_error(format!(
                    "missed XSPICE breakpoint at {due:.16e}s before shared Active work at {physical:.16e}s"
                )));
            }
            let mut wave = self
                .circuit
                .begin_xspice_active_wave(
                    physical,
                    self.timestep - (self.time - physical),
                    self.analysis,
                    self.phase,
                    XspiceCompanionPolicy {
                        coefficients: &self.coefficients,
                        xyce_one_step_order2: self.xyce_one_step_order2,
                    },
                )
                .map_err(|error| external_error(error.to_string()))?;
            if self.interpolating && physical < self.time {
                wave.interpolated = true;
                wave.skip_opening_dispatch();
            }
            if self.resettling {
                // Opening a wave resets the pending flags, so the projection's
                // marks are laid down again behind it.
                wave.skip_opening_dispatch();
                let projected = self.projected;
                self.circuit.record_xspice_analog_input_dispatch(projected);
                // Only the marked instances run in the new wave, so what the
                // unmarked ones published has to come with it or a marked
                // reader loses a transition it could see before the reopen.
                if let Some(previous) = self.wave.as_ref() {
                    wave.inherit_analog_transitions(previous);
                }
            }
            self.wave = Some(wave);
        }
        self.pending.extend(exchange.take_event_changes());
        let wave = self.wave.as_mut().expect("prepared physical Active wave");
        wave.defer_mixed_adc = !self.analog_boundaries_ready || wave.interpolated;
        if !self.initialized {
            // An undriven shared bit starts at Z. Seed missing observation
            // entries from the state preceding the first journaled publication,
            // so an input-only XSPICE endpoint sees an explicit value too.
            let mut initial = Vec::new();
            for (&node, &net) in &self.bindings.by_node {
                if !self
                    .circuit
                    .scheduler
                    .xspice_event_values
                    .digital_values
                    .contains_key(&node)
                {
                    let value = match self.pending.iter().find_map(|change| match change {
                        ExternalNetChange::Bits(change) if change.net == net => {
                            Some(change.previous)
                        }
                        _ => None,
                    }) {
                        Some(value) => value,
                        None => exchange.read_net(net)?,
                    };
                    initial.push((node, value));
                }
            }
            self.circuit
                .observe_xspice_shared_digital_inputs(wave, &initial);
            let mut initial_real = Vec::new();
            for (&node, &signal) in &self.bindings.real_by_node {
                if !self
                    .circuit
                    .scheduler
                    .xspice_event_values
                    .real_values
                    .contains_key(&node)
                {
                    let value = self
                        .pending
                        .iter()
                        .find_map(|change| match change {
                            ExternalNetChange::Real {
                                signal: changed,
                                previous,
                                ..
                            } if *changed == signal => Some(*previous),
                            _ => None,
                        })
                        .or_else(|| exchange.read_real_signal(signal))
                        .ok_or_else(|| external_error("missing linked real signal"))?;
                    initial_real.push((node, value));
                }
            }
            self.circuit
                .observe_xspice_shared_real_inputs(wave, &initial_real);
            for (&id, (owner, target)) in &self.bindings.inout_observers {
                let value = self
                    .pending
                    .iter()
                    .find_map(|change| match change {
                        ExternalNetChange::DriverInput {
                            driver, previous, ..
                        } if driver.index() == id => Some(*previous),
                        _ => None,
                    })
                    .map(Ok)
                    .unwrap_or_else(|| {
                        exchange.read_other_drivers(self.bindings.drivers[target])
                    })?;
                self.circuit
                    .observe_xspice_shared_inout(wave, *owner, target, value);
            }
            self.initialized = true;
        }
        let mut observed = Vec::new();
        let mut observed_real = Vec::new();
        while let Some(change) = self.pending.pop_front() {
            match change {
                ExternalNetChange::Bits(change) => {
                    let node = self
                        .bindings
                        .by_net
                        .get(&change.net)
                        .copied()
                        .ok_or_else(|| {
                            external_error(format!(
                                "unbound XSPICE observation for bit group {}",
                                change.net
                            ))
                        })?;
                    observed.push((node, change.value));
                }
                ExternalNetChange::DriverInput { driver, value, .. } => {
                    let (owner, target) = self
                        .bindings
                        .inout_observers
                        .get(&driver.index())
                        .ok_or_else(|| {
                            external_error(format!("unbound inout observer {}", driver.index()))
                        })?;
                    self.circuit
                        .observe_xspice_shared_inout(wave, *owner, target, value);
                }
                ExternalNetChange::Real { signal, value, .. } => {
                    let node = self
                        .bindings
                        .real_by_signal
                        .get(&signal)
                        .copied()
                        .ok_or_else(|| {
                            external_error(format!(
                                "unbound XSPICE real observation for signal {signal}"
                            ))
                        })?;
                    observed_real.push((node, value));
                }
            }
            if self
                .pending
                .front()
                .is_none_or(|next| next.starts_publication())
            {
                break;
            }
        }
        self.circuit
            .observe_xspice_shared_digital_inputs(wave, &observed);
        self.circuit
            .observe_xspice_shared_real_inputs(wave, &observed_real);
        let mut resolver = SharedResolver {
            bindings: self.bindings,
            exchange,
        };
        let more = self
            .circuit
            .step_xspice_active_wave_with_resolver(
                wave,
                self.solution,
                self.resources,
                &mut resolver,
            )
            .map_err(|error| external_error(error.to_string()))?;
        Ok(more || !self.pending.is_empty())
    }
}

struct SharedResolver<'a, 'host> {
    bindings: &'a XspiceDigitalBindings,
    exchange: &'a mut DigitalActiveExchange<'host>,
}
impl XspiceDigitalResolver for SharedResolver<'_, '_> {
    fn owns(&self, node: NodeId) -> bool {
        self.bindings.by_node.contains_key(&node)
    }
    fn owns_real(&self, node: NodeId) -> bool {
        self.bindings.real_by_node.contains_key(&node)
    }
    fn publish(
        &mut self,
        drivers: &[(EventTarget, DigitalValue)],
        real_drivers: &[(EventTarget, Value)],
        resolved: &mut Vec<(NodeId, DigitalValue)>,
        resolved_real: &mut Vec<(NodeId, Value)>,
    ) -> crate::xspice::CmResult<()> {
        let mut bank = Vec::with_capacity(drivers.len());
        let mut nodes = BTreeSet::new();
        for (target, value) in drivers {
            let Some(&id) = self.bindings.drivers.get(target) else {
                return Err(model_error(format!(
                    "undeclared XSPICE output driver {}.{}[{}] on shared node {}",
                    target.instance, target.port_name, target.driver_index, target.node_id,
                )));
            };
            bank.push((id, *value));
            nodes.insert(target.node_id);
        }
        // Validate every original driver before publishing any vector element.
        let mut real_bank = Vec::with_capacity(real_drivers.len());
        let mut real_nodes = BTreeSet::new();
        for (target, value) in real_drivers {
            let id = self
                .bindings
                .real_drivers
                .get(target)
                .copied()
                .ok_or_else(|| {
                    model_error(format!(
                        "undeclared XSPICE real output driver {}.{}[{}]",
                        target.instance, target.port_name, target.driver_index
                    ))
                })?;
            real_bank.push((id, *value));
            real_nodes.insert(target.node_id);
        }
        self.exchange
            .drive_bank(&bank, &real_bank)
            .map_err(model_error)?;
        for node in nodes {
            resolved.push((
                node,
                self.exchange
                    .read_net(self.bindings.by_node[&node])
                    .map_err(model_error)?,
            ));
        }
        for node in real_nodes {
            let value = self
                .exchange
                .read_real_signal(self.bindings.real_by_node[&node])
                .ok_or_else(|| model_error("missing shared real net"))?;
            resolved_real.push((node, value));
        }
        Ok(())
    }
}
