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
use crate::xspice::verilog::store::{DigitalBitChange, ExternalBitDriverId};
use std::collections::{BTreeSet, VecDeque};

/// Immutable connections between physical circuit node identities and resolved
/// HDL bit groups. Driver indices match XspiceInstance::schedule_events.
#[derive(Clone, Debug)]
pub(crate) struct XspiceDigitalBindings {
    by_node: BTreeMap<NodeId, usize>,
    by_net: BTreeMap<usize, NodeId>,
    drivers: BTreeMap<EventTarget, ExternalBitDriverId>,
}

impl XspiceDigitalBindings {
    pub(crate) fn enroll_circuit(
        circuit: &CircuitData,
        coordinator: &mut crate::xspice::verilog::MixedDigitalCoordinator,
    ) -> Result<Option<Self>, DigitalRunError> {
        let nets: Vec<_> = coordinator.event_bindings().collect();
        Self::enroll_with(circuit, &nets, |observed, drivers| {
            coordinator.attach_external_bits(observed, drivers)
        })
    }

    #[cfg(test)]
    pub(crate) fn enroll(
        circuit: &CircuitData,
        digital: &mut DigitalHost,
        nets: &[(NodeId, usize)],
    ) -> Result<Option<Self>, DigitalRunError> {
        Self::enroll_with(circuit, nets, |observed, drivers| {
            digital.attach_external_bits(observed, drivers)
        })
    }

    fn enroll_with(
        circuit: &CircuitData,
        nets: &[(NodeId, usize)],
        attach: impl FnOnce(
            &[usize],
            &[(usize, EventTarget)],
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
        for instance in &circuit.xspice_instances {
            instance.for_each_event_input_net(|kind, node| {
                if kind == EventInputKind::Digital && offered.contains_key(&node) {
                    connected.insert(node);
                }
            });
            instance.for_each_digital_output_driver(|target| {
                if offered.contains_key(&target.node_id) {
                    connected.insert(target.node_id);
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
        let ids = attach(&by_node.values().copied().collect::<Vec<_>>(), &targets)?;
        Ok(Some(Self {
            by_node,
            by_net,
            drivers: targets
                .into_iter()
                .map(|(_, target)| target)
                .zip(ids)
                .collect(),
        }))
    }
    pub(crate) fn contains_node(&self, node: NodeId) -> bool {
        self.by_node.contains_key(&node)
    }

    pub(crate) fn remap_nodes(&mut self, remap: impl Fn(usize) -> usize) {
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
    pending: VecDeque<DigitalBitChange>,
    initialized: bool,
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
    /// `CircuitData::set_mixed_analog_step_floor` publishes to both. A zero
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
        let floor = self.circuit.xspice_analog_step_floor;
        !floor.is_finite() || floor <= 0.0 || due - (self.time - self.timestep) >= floor
    }
}

impl DigitalActiveParticipant for XspiceDigitalParticipant<'_> {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        let physical = exchange.physical_seconds();
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
            if let Some(due) = self.circuit.xspice_event_queue.next_event_time()
                && due < physical
                && self.event_was_reachable(due)
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
        self.pending.extend(exchange.take_changes());
        let wave = self.wave.as_mut().expect("prepared physical Active wave");
        if !self.initialized {
            // An undriven shared bit starts at Z. Seed missing observation
            // entries from the state preceding the first journaled publication,
            // so an input-only XSPICE endpoint sees an explicit value too.
            let mut initial = Vec::new();
            for (&node, &net) in &self.bindings.by_node {
                if !self
                    .circuit
                    .xspice_event_values
                    .digital_values
                    .contains_key(&node)
                {
                    let value = match self.pending.iter().find(|change| change.net == net) {
                        Some(change) => change.previous,
                        None => exchange.read_net(net)?,
                    };
                    initial.push((node, value));
                }
            }
            self.circuit
                .observe_xspice_shared_digital_inputs(wave, &initial);
            self.initialized = true;
        }
        let mut observed = Vec::new();
        while let Some(change) = self.pending.pop_front() {
            let Some(&node) = self.bindings.by_net.get(&change.net) else {
                return Err(external_error(format!(
                    "unbound XSPICE observation for bit group {}",
                    change.net
                )));
            };
            observed.push((node, change.value));
            if self
                .pending
                .front()
                .is_none_or(|next| next.starts_publication)
            {
                break;
            }
        }
        self.circuit
            .observe_xspice_shared_digital_inputs(wave, &observed);
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
    fn publish(
        &mut self,
        drivers: &[(EventTarget, DigitalValue)],
        resolved: &mut Vec<(NodeId, DigitalValue)>,
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
        self.exchange.drive_many(&bank).map_err(model_error)?;
        for node in nodes {
            resolved.push((
                node,
                self.exchange
                    .read_net(self.bindings.by_node[&node])
                    .map_err(model_error)?,
            ));
        }
        Ok(())
    }
}
