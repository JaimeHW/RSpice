//! Attaching externally defined devices to a [`CircuitData`].
//!
//! The integration surface for models the core does not implement itself:
//! XSPICE code-model instances, JIT-compiled Verilog-A devices, and the
//! checked-in generated Verilog-A built-ins. Each family needs the same
//! things — instance registration, matrix and RHS contributions folded in
//! only where the device actually has entries, and timestep acceptance that
//! propagates the device's own breakpoint requests back to the integrator.

#[cfg(all(test, feature = "veriloga"))]
#[path = "external_models/coupled_tests.rs"]
mod coupled_tests;

#[cfg(feature = "veriloga")]
mod coupled;
#[cfg(feature = "veriloga")]
pub(crate) use coupled::{XspiceDigitalBindings, XspiceDigitalParticipant};

use super::*;
use crate::xspice::{
    EventInputKind, ResourceTransaction, XspiceEventInputs, XspiceInstanceCheckpoint,
};
#[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
use std::collections::BTreeMap;
use std::collections::HashMap;

/// A resumable XSPICE Active settle at one physical candidate. Pending
/// dispatch flags stay with the circuit; transition observations and source
/// samples remain here across yields to another event participant.
#[derive(Clone)]
pub(crate) struct XspiceActiveWave {
    time: Value,
    timestep: Value,
    analysis: crate::xspice::AnalysisType,
    phase: crate::xspice::EvaluationPhase,
    coefficients: crate::numerics::integration::CompanionCoefficients,
    xyce_one_step_order2: bool,
    current_source_values: Vec<Value>,
    analog_transitions: HashMap<(NodeId, NodeId), crate::xspice::AnalogTransition>,
    pass: usize,
}

impl XspiceActiveWave {
    /// Open this wave past its blanket opening dispatch.
    ///
    /// The opening pass of a wave evaluates every registered instance, because
    /// a wave normally opens at a timepoint no instance has been evaluated at.
    /// A candidate that re-settles — the accepted step's voltage-projection
    /// loop, which re-runs the boundary after moving the solution — is not
    /// that: every instance has already run at this timepoint, and running
    /// them again is not a second look at the same question but a second
    /// execution of the same accepted-phase side effects. Past the opening
    /// pass the wave dispatches only what an event or a moved analog input
    /// marked, which is the set whose answer can actually have changed.
    #[cfg(feature = "veriloga")]
    pub(crate) fn skip_opening_dispatch(&mut self) {
        self.pass = self.pass.max(1);
    }

    /// Keep the transitions a wave published when a re-settling candidate
    /// reopens it at another physical time.
    ///
    /// A wave that opens past its opening dispatch runs only the instances an
    /// event or the projection marked. Everything else keeps the answer it
    /// gave in the wave being replaced -- including the analog transitions its
    /// readers take out of this map -- so dropping them would leave an
    /// unmarked instance's transition invisible to a marked reader that had
    /// seen it a moment earlier. An entry the new wave publishes describes
    /// this physical time and wins.
    #[cfg(feature = "veriloga")]
    pub(crate) fn inherit_analog_transitions(&mut self, previous: &Self) {
        for (key, transition) in &previous.analog_transitions {
            self.analog_transitions.entry(*key).or_insert(*transition);
        }
    }
}

/// Copy-on-write event/model state staged by circuit probe/acceptance transactions.
/// Dispatch topology is immutable, and per-evaluation scratch is recomputed.
pub(crate) struct XspiceAcceptanceRollback {
    instances: Vec<SharedXspiceInstance>,
    values: SharedXspiceEventValues,
    queue: SharedXspiceEventQueue,
    error: Option<String>,
    resources: ResourceTransaction,
}

impl XspiceAcceptanceRollback {
    pub(crate) fn resources(&self) -> &ResourceTransaction {
        &self.resources
    }
}

/// Everything one XSPICE trial evaluation can write, and nothing else.
///
/// A Newton trial stamp evaluates code models at a point the solver may
/// discard, so what it wrote has to be put back. The set it writes is this
/// triple: the instances and their contexts, the six resolved-value maps, and
/// the event scheduler. Each of the three is copy-on-write, so capturing them
/// is three reference-count bumps and costs a real copy only where the trial
/// actually writes.
///
/// The trial used to undo itself with a whole-circuit device image
/// ([`CircuitData::nonlinear_state_snapshot`]) instead — every capacitor,
/// inductor, diode, BJT, MOSFET table, behavioural source and Verilog-A
/// device, cloned outright once per Newton iteration of every step of every
/// deck holding a code model, to restore three fields none of those devices
/// appear in. The trial cannot reach them: it early-returns unless XSPICE
/// evaluation is independent of the shared event nets, and the only thing it
/// calls between capture and restore is the code-model evaluation and the
/// XSPICE matrix stamp.
///
/// `xspice_evaluation_error` is deliberately not part of this: the image the
/// device snapshot restored did not carry it either, so an evaluation failure
/// inside a trial stays reported rather than being swallowed with the state.
pub(crate) struct XspiceTrialState {
    instances: Vec<SharedXspiceInstance>,
    values: SharedXspiceEventValues,
    queue: SharedXspiceEventQueue,
}

/// Accepted Verilog-A state carried between circuits rebuilt for adjacent DC
/// sweep points.
///
/// The checkpoint payloads contain only accepted operator/event history.  The
/// accompanying keys deliberately use semantic terminal names rather than
/// raw node IDs, and omit numeric parameter values, so an otherwise identical
/// circuit rebuilt after a TEMP or parameter substitution can continue the
/// same public DC analysis.  Restore validates the complete key set before it
/// applies any payload, so changed topology, model provenance, or instance
/// cardinality fails closed.
#[derive(Debug, Clone, Default)]
pub(crate) struct VerilogADcAcceptedStateCarrier {
    #[cfg(feature = "veriloga")]
    runtime: Vec<RuntimeVerilogADcAcceptedState>,
    #[cfg(feature = "veriloga-builtins-base")]
    generated: Vec<GeneratedVerilogADcAcceptedState>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RuntimeVerilogADcStateKey {
    instance_name: String,
    model_name: String,
    source_digest: String,
    terminals: Vec<String>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct RuntimeVerilogADcAcceptedState {
    key: RuntimeVerilogADcStateKey,
    checkpoint: crate::device::veriloga::VerilogADeviceCheckpoint,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GeneratedVerilogADcStateKey {
    instance_name: String,
    model_name: String,
    model_identity: String,
    terminals: Vec<String>,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone)]
struct GeneratedVerilogADcAcceptedState {
    key: GeneratedVerilogADcStateKey,
    checkpoint: crate::device::veriloga_builtins::GeneratedVerilogAInstanceCheckpoint,
}

/// Execute every XSPICE event due at or before `time` and fold the result into
/// the resolved event-node values.
///
/// Ordering, supersession and settling belong to the scheduler kernel; what
/// happens here is what an executed event *means* to the analog side: it
/// updates one driver, and the node's value is whatever its drivers resolve
/// to. `Ok(true)` means something an instance can see changed.
///
/// The queue and the value maps arrive as their shared handles rather than as
/// mutable borrows of their contents, and the emptiness check runs before
/// either is unshared. That ordering is the whole point: this is the only
/// writer of the six maps and it runs at every settle pass of every Newton
/// iteration, so taking the mutable view first would copy the event world for
/// every quiet analog step and defeat the sharing the rollback snapshot
/// depends on.
#[cfg(test)]
fn apply_xspice_events_at_or_before(
    event_values: &mut crate::xspice::SharedXspiceEventValues,
    event_queue: &mut crate::xspice::SharedXspiceEventQueue,
    touched_digital_nodes: &mut Vec<NodeId>,
    touched_real_nodes: &mut Vec<NodeId>,
    time: Value,
) -> crate::xspice::CmResult<bool> {
    apply_xspice_events_with_resolver(
        event_values,
        event_queue,
        touched_digital_nodes,
        touched_real_nodes,
        time,
        &mut LocalDigitalResolver,
    )
}

/// An event drain publishes original drivers through the owner of a shared net.
/// Local XSPICE nets retain their ordinary resolver. Returned values are input
/// observations only; the circuit never feeds them back as output contributions.
pub(super) trait XspiceDigitalResolver {
    fn owns(&self, node: NodeId) -> bool;
    fn publish(
        &mut self,
        drivers: &[(
            crate::xspice::event_scheduler::EventTarget,
            crate::xspice::DigitalValue,
        )],
        resolved: &mut Vec<(NodeId, crate::xspice::DigitalValue)>,
    ) -> crate::xspice::CmResult<()>;
}

struct LocalDigitalResolver;
impl XspiceDigitalResolver for LocalDigitalResolver {
    fn owns(&self, _node: NodeId) -> bool {
        false
    }
    fn publish(
        &mut self,
        drivers: &[(
            crate::xspice::event_scheduler::EventTarget,
            crate::xspice::DigitalValue,
        )],
        _resolved: &mut Vec<(NodeId, crate::xspice::DigitalValue)>,
    ) -> crate::xspice::CmResult<()> {
        debug_assert!(drivers.is_empty());
        Ok(())
    }
}

fn apply_xspice_events_with_resolver(
    event_values: &mut crate::xspice::SharedXspiceEventValues,
    event_queue: &mut crate::xspice::SharedXspiceEventQueue,
    touched_digital_nodes: &mut Vec<NodeId>,
    touched_real_nodes: &mut Vec<NodeId>,
    time: Value,
    resolver: &mut impl XspiceDigitalResolver,
) -> crate::xspice::CmResult<bool> {
    let mut changed = false;
    touched_digital_nodes.clear();
    touched_real_nodes.clear();
    // Shared borrow: nothing is copied for a step with no due event, which is
    // every Newton iteration after the first and every purely analog step.
    if !event_queue.has_event_at_or_before(time) {
        return Ok(false);
    }
    let event_queue = event_queue.make_mut();
    let crate::xspice::XspiceEventValues {
        digital_values,
        digital_drivers,
        digital_event_times,
        real_values,
        real_drivers,
        real_event_times,
    } = event_values.make_mut();
    let mut shared_drivers = Vec::new();
    event_queue
        .run_due_events(time, |event| {
            let node_id = event.node_id;
            let event_time = event.time;
            let driver_key = (event.instance, event.port_name, event.driver_index);
            match event.value {
                crate::xspice::EventValue::Digital(value) => {
                    if resolver.owns(node_id) {
                        shared_drivers.push((
                            crate::xspice::event_scheduler::EventTarget {
                                node_id,
                                instance: driver_key.0.clone(),
                                port_name: driver_key.1.clone(),
                                driver_index: driver_key.2,
                            },
                            value,
                        ));
                    }
                    digital_drivers
                        .entry(node_id)
                        .or_default()
                        .insert(driver_key, value);
                    let previous_time = digital_event_times.insert(node_id, event_time);
                    changed |= previous_time != Some(event_time);
                    touched_digital_nodes.push(node_id);
                }
                crate::xspice::EventValue::Real(value) => {
                    real_drivers
                        .entry(node_id)
                        .or_default()
                        .insert(driver_key, value);
                    let previous_time = real_event_times.insert(node_id, event_time);
                    changed |= previous_time != Some(event_time);
                    touched_real_nodes.push(node_id);
                }
            }
        })
        .map_err(|error| {
            crate::xspice::CmError::EvaluationError(xspice_event_settling_message(time, &error))
        })?;
    if touched_digital_nodes.len() > 1 {
        touched_digital_nodes.sort_unstable();
        touched_digital_nodes.dedup();
    }
    if touched_real_nodes.len() > 1 {
        touched_real_nodes.sort_unstable();
        touched_real_nodes.dedup();
    }
    for &node_id in touched_digital_nodes.iter() {
        if resolver.owns(node_id) {
            continue;
        }
        let resolved = digital_drivers
            .get(&node_id)
            .and_then(|drivers| {
                let mut values = drivers.values();
                let first = *values.next()?;
                Some(values.fold(first, |resolved, value| resolved.resolve(value)))
            })
            .unwrap_or_default();
        let previous_value = digital_values.insert(node_id, resolved);
        changed |= previous_value != Some(resolved);
    }
    if !shared_drivers.is_empty() {
        let mut resolved = Vec::new();
        resolver.publish(&shared_drivers, &mut resolved)?;
        for (node, value) in resolved {
            changed |= digital_values.insert(node, value) != Some(value);
        }
    }
    for &node_id in touched_real_nodes.iter() {
        let resolved = real_drivers
            .get(&node_id)
            .map(|drivers| drivers.values().copied().sum())
            .unwrap_or(0.0);
        let previous_value = real_values.insert(node_id, resolved);
        changed |= previous_value != Some(resolved);
    }
    Ok(changed)
}

/// Mark the fan-out of both touched-node lists as having dirty event inputs.
///
/// Every drain is followed by this, the failing ones included: `run_due_events`
/// can report an oscillation after it has already executed events, and the
/// nodes those events moved are in the lists whether or not the call returned
/// `Ok`.
fn mark_drained_fanout_dirty(
    dispatch: &super::xspice_dispatch::XspiceEventDispatch,
    instances: &mut [crate::xspice::SharedXspiceInstance],
    touched_digital_nodes: &[NodeId],
    touched_real_nodes: &[NodeId],
) {
    dispatch.mark_fanout_dirty(instances, EventInputKind::Digital, touched_digital_nodes);
    dispatch.mark_fanout_dirty(instances, EventInputKind::Real, touched_real_nodes);
}

/// Register each event connection's value domain. Analog connections retain
/// their electrical meaning; the auto-bridge planner handles domain boundaries.
fn mark_xspice_event_connection_nets(
    kinds: &mut NetKinds,
    connection: &crate::xspice::PortConnection,
) {
    use crate::xspice::PortConnection;
    match connection {
        PortConnection::Digital(node) | PortConnection::DigitalInverted(node) => {
            kinds.register(*node, NetKind::Digital);
        }
        PortConnection::Real(node) => kinds.register(*node, NetKind::Real),
        PortConnection::DigitalVector(vector) => {
            for &node in vector {
                kinds.register(node, NetKind::Digital);
            }
        }
        PortConnection::RealVector(vector) => {
            for &node in vector {
                kinds.register(node, NetKind::Real);
            }
        }
        PortConnection::DigitalVectorMapped(vector) => {
            for connection in vector {
                kinds.register(connection.node, NetKind::Digital);
            }
        }
        _ => {}
    }
}

/// Render a scheduler settling failure as the analysis-level diagnostic.
///
/// The old bounded relaxation could only say which nodes were still changing
/// when it gave up. The scheduler counts activations per driver, so what comes
/// out names the instances and ports that would not quiet, busiest first,
/// which is what identifies a zero-delay loop.
fn xspice_event_settling_message(
    time: Value,
    error: &crate::xspice::event_scheduler::SchedulerError,
) -> String {
    use crate::xspice::event_scheduler::{OscillationCause, SchedulerError};

    let SchedulerError::Oscillation(diagnostic) = error else {
        return format!("XSPICE event scheduling failed at time {time:e}: {error}");
    };
    let ceiling = match diagnostic.cause {
        OscillationCause::DeltaCycleLimit => {
            format!("{} delta cycles", diagnostic.delta_cycle_limit)
        }
        OscillationCause::EventLimit => format!("{} events", diagnostic.event_limit),
    };
    let drivers: Vec<String> = diagnostic
        .entities
        .iter()
        .map(|(target, count)| {
            format!(
                "{}.{} fired {count} times",
                target.instance, target.port_name
            )
        })
        .collect();
    let busiest = if drivers.is_empty() {
        "no driver fired".to_string()
    } else {
        drivers.join(", ")
    };
    format!(
        "XSPICE event network did not settle at time {time:e} within {ceiling} \
         after {} delta cycles and {} events ({busiest})",
        diagnostic.delta_cycles, diagnostic.events_executed
    )
}

/// How a code model's reactive contribution is turned into a companion stamp:
/// the integration coefficients for the step, and whether Xyce's second-order
/// one-step start-up applies to them. The flag selects which of the
/// coefficients are read, so neither means anything without the other.
#[derive(Clone, Copy)]
pub(crate) struct XspiceCompanionPolicy<'a> {
    pub coefficients: &'a crate::numerics::integration::CompanionCoefficients,
    pub xyce_one_step_order2: bool,
}

/// The common analog companion contract for runtime and mixed HDL models.
#[cfg(feature = "veriloga")]
#[derive(Clone, Copy)]
pub(crate) struct VerilogACompanionRules {
    pub derivative: rspice_veriloga::vm::IntegrationCoefficients,
    pub state: rspice_veriloga::vm::IntegrationCoefficients,
}

#[cfg(feature = "veriloga")]
impl VerilogACompanionRules {
    pub(crate) fn from_policy(
        dt: Value,
        policy: XspiceCompanionPolicy<'_>,
    ) -> Result<Self, String> {
        // OneStep2 assembles (Q-Qprev)/dt + (F+Fprev)/2. Its ddt
        // stamp is doubled BE before the external half-weight; internal
        // integrators and filters instead solve their full trapezoidal rule.
        let coefficients = if policy.xyce_one_step_order2 {
            crate::numerics::integration::CompanionCoefficients::backward_euler()
        } else {
            *policy.coefficients
        };
        let derivative: rspice_veriloga::vm::IntegrationCoefficients =
            rspice_veriloga_runtime::GeneratedDdtCoefficients::from_companion_values_with_derivative_scale(
                coefficients.coeff_g, coefficients.coeff_v_n,
                coefficients.coeff_v_n_minus_1, coefficients.needs_two_history,
                coefficients.coeff_i_n, dt,
            ).map_err(|error| error.to_string())?
                .scaled(if policy.xyce_one_step_order2 { 2.0 } else { 1.0 }).into();
        let state = if policy.xyce_one_step_order2 && derivative.active {
            rspice_veriloga::vm::IntegrationCoefficients {
                previous_derivative_scale: 1.0,
                ..derivative
            }
        } else {
            derivative
        };
        derivative.validate().map_err(|error| error.to_string())?;
        state.validate().map_err(|error| error.to_string())?;
        Ok(Self { derivative, state })
    }
}

impl CircuitData {
    //=========================================================================
    // XSPICE Code Model Interface
    //=========================================================================

    /// Check if circuit has any XSPICE code model instances
    #[inline]
    pub fn has_xspice_devices(&self) -> bool {
        !self.xspice_instances.is_empty()
    }

    pub(crate) fn has_coupled_event_nets(&self) -> bool {
        #[cfg(feature = "veriloga")]
        {
            self.mixed_xspice_bindings.is_some()
        }
        #[cfg(not(feature = "veriloga"))]
        {
            false
        }
    }

    /// Coupled event nets are evaluated with the mixed circuit trial.
    pub(crate) fn has_independent_xspice_evaluation(&self) -> bool {
        self.has_xspice_devices() && !self.has_coupled_event_nets()
    }

    /// Add an XSPICE code model instance and update derived circuit metadata.
    pub(crate) fn add_xspice_instance(&mut self, instance: XspiceInstance) {
        self.xspice_resource_failure
            .get_or_insert_with(|| Arc::new(std::sync::OnceLock::new()));
        self.xspice_has_event_driven_devices |= instance
            .ports()
            .iter()
            .any(|port| port.default_type.is_event_driven());
        instance.for_each_event_load_contribution(|node, load| {
            *self.xspice_event_loads.entry(node).or_insert(0.0) += load;
        });
        for connection in instance.connections() {
            mark_xspice_event_connection_nets(&mut self.net_kinds, connection);
        }
        self.xspice_instances
            .push(crate::xspice::SharedXspiceInstance::new(instance));
        self.invalidate_xspice_event_dispatch();
    }

    /// Check if any XSPICE instance participates in event-driven scheduling.
    #[inline]
    pub fn has_xspice_event_driven_devices(&self) -> bool {
        self.xspice_has_event_driven_devices
    }

    /// Whether the analog solution can be stepped by a digital edge rather
    /// than by an independent source.
    ///
    /// An XSPICE event-driven code model does that through its D/A bridges,
    /// and a mixed Verilog-AMS host does the same thing through its own
    /// Thevenin D/A bridges: at a scheduled edge the bridge's source moves
    /// discontinuously, and `max_expected_source_delta` — which sees only the
    /// analog independent sources — predicts none of it. Any recovery guard
    /// that reasons "the sources are quiet, so a large solution move is
    /// nonphysical" must therefore be disarmed for both, not just for XSPICE.
    #[inline]
    pub(crate) fn has_event_driven_boundaries(&self) -> bool {
        if self.has_xspice_event_driven_devices() {
            return true;
        }
        #[cfg(feature = "veriloga")]
        {
            self.has_mixed_signal_hosts()
        }
        #[cfg(not(feature = "veriloga"))]
        {
            false
        }
    }

    /// Rebuild event kinds from the already-renumbered XSPICE connections and
    /// circuit HDL graph. Ground has no event kind.
    pub(crate) fn rebuild_net_kinds(&mut self) {
        self.net_kinds = NetKinds::default();
        for instance in &self.xspice_instances {
            for connection in instance.connections() {
                mark_xspice_event_connection_nets(&mut self.net_kinds, connection);
            }
        }
        #[cfg(feature = "veriloga")]
        if let Some(digital) = &self.mixed_digital_coordinator {
            for node in digital.event_nodes() {
                self.net_kinds.register(node, NetKind::Digital);
            }
        }
        // Renumbered connections move the nets the sensitivity map is keyed
        // by, so it is rebuilt from the same replay this cache is.
        self.invalidate_xspice_event_dispatch();
    }

    /// Whether a node identity has an event-value representation. Its
    /// electrical equations, when present, still belong to the analog solver.
    /// Ground and unknown nodes have no registered event representation.
    #[inline]
    pub(crate) fn is_discrete_net(&self, node: NodeId) -> bool {
        self.net_kinds.kind(node).is_discrete()
    }

    /// Zero-based MNA rows that also serve as event-node identities.
    pub(crate) fn xspice_event_node_matrix_rows(&self) -> impl Iterator<Item = usize> + '_ {
        self.net_kinds.discrete_nodes().map(|node| node - 1)
    }

    /// Set transient run context on all XSPICE instances.
    pub(crate) fn set_xspice_transient_context(&mut self, tstep: Value, tstop: Value) {
        let tstep = (tstep.is_finite() && tstep > 0.0).then_some(tstep);
        let tstop = (tstop.is_finite() && tstop >= 0.0).then_some(tstop);
        for instance in &mut self.xspice_instances {
            instance.make_mut().set_transient_run_context(tstep, tstop);
        }
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn has_veriloga_devices(&self) -> bool {
        !self.veriloga_devices.is_empty()
    }

    #[cfg(feature = "veriloga")]
    pub fn add_veriloga_device(&mut self, mut device: crate::device::veriloga::VerilogADevice) {
        device.set_simulation_parameters(self.generated_simulation_parameters);
        self.non_electrical_state_nodes
            .extend(device.non_electrical_node_indices());
        self.veriloga_devices.add(device);
    }

    /// Continuation settings belong to the solver and survive device rollback.
    #[cfg(feature = "veriloga")]
    pub(crate) fn sync_veriloga_simulation_parameters(&mut self) {
        for device in self.veriloga_devices.iter_mut() {
            device.set_simulation_parameters(self.generated_simulation_parameters);
        }
        for host in &mut self.mixed_signal_hosts {
            host.set_simulation_parameters(self.generated_simulation_parameters);
        }
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn veriloga_devices(&self) -> &crate::device::veriloga::VerilogADevices {
        &self.veriloga_devices
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn veriloga_devices_mut(&mut self) -> &mut crate::device::veriloga::VerilogADevices {
        &mut self.veriloga_devices
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[inline]
    pub fn has_generated_veriloga_devices(&self) -> bool {
        !self.generated_veriloga_devices.is_empty()
    }

    #[inline]
    pub(crate) fn has_any_veriloga_devices(&self) -> bool {
        #[cfg(feature = "veriloga")]
        if !self.veriloga_devices.is_empty() || !self.mixed_signal_hosts.is_empty() {
            return true;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if !self.generated_veriloga_devices.is_empty() {
            return true;
        }
        false
    }

    /// Smallest interval supported by every installed Verilog-A execution path.
    #[inline]
    pub(crate) fn veriloga_integration_timestep_floor(&self) -> Value {
        #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
        if self.has_any_veriloga_devices() {
            return rspice_veriloga_runtime::GENERATED_DDT_TIMESTEP_FLOOR;
        }
        0.0
    }

    /// Whether any model supplies its own initial equilibrium nodeset.
    pub(crate) fn veriloga_requires_nodeset_phase(&self) -> bool {
        #[cfg(feature = "veriloga")]
        if self
            .veriloga_devices
            .iter()
            .any(|device| device.requires_nodeset_phase())
            || self
                .mixed_signal_hosts
                .iter()
                .any(|host| host.requires_nodeset_phase())
        {
            return true;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if self.generated_veriloga_devices.requires_nodeset_phase() {
            return true;
        }
        false
    }

    /// Whether every Verilog-A instance can participate in Xyce OneStep's
    /// order-two F/Q split without changing its model equations.
    ///
    /// Runtime and mixed models carry the full-source equation proof and use
    /// distinct derivative/internal-state companions. Generated models retain
    /// their own compiler-proven capability, including their IDT restriction.
    /// An unqualified model selects ordinary trapezoidal/Gear companions.
    pub(crate) fn veriloga_one_step_dae_split_safe(&self) -> bool {
        #[cfg(feature = "veriloga")]
        if self
            .veriloga_devices
            .iter()
            .any(|device| !device.one_step_dae_split_safe())
            || self
                .mixed_signal_hosts
                .iter()
                .any(|host| !host.one_step_dae_split_safe())
        {
            return false;
        }

        #[cfg(feature = "veriloga-builtins-base")]
        if !self
            .generated_veriloga_devices
            .all_one_step_dae_split_safe()
        {
            return false;
        }

        true
    }

    #[cfg(feature = "veriloga-builtins-base")]
    pub fn add_generated_veriloga_device(
        &mut self,
        device: crate::device::veriloga_builtins::BuiltinVerilogAInstance,
    ) {
        self.non_electrical_state_nodes
            .extend(device.non_electrical_node_indices());
        self.generated_veriloga_devices.add(device);
    }

    #[cfg(feature = "veriloga-builtins-base")]
    pub(crate) fn generated_veriloga_devices(
        &self,
    ) -> &crate::device::veriloga_builtins::BuiltinVerilogADevices {
        &self.generated_veriloga_devices
    }

    #[cfg(feature = "veriloga-builtins-base")]
    pub(crate) fn generated_veriloga_devices_mut(
        &mut self,
    ) -> &mut crate::device::veriloga_builtins::BuiltinVerilogADevices {
        &mut self.generated_veriloga_devices
    }

    /// Evaluate all XSPICE code model instances
    ///
    /// This calls each XspiceInstance::evaluate() with the current simulation
    /// state, updating internal context and computing output contributions.
    ///
    /// # Arguments
    /// * `time` - Current simulation time
    /// * `voltages` - Current MNA solution vector, with node voltages followed by branch currents
    pub fn evaluate_xspice(&mut self, time: Value, voltages: &[Value]) {
        if let Err(e) = self.try_evaluate_xspice(time, voltages) {
            self.warn_xspice_evaluation(time, &e);
        }
    }

    /// Log one XSPICE evaluation failure, naming the iterate it happened at,
    /// and not again while the same failure repeats.
    ///
    /// Decision (R1.14): a code model that cannot evaluate at a trial point
    /// stays a warning rather than becoming a rejectable iterate. ngspice
    /// ignores it — `cm_` functions have no way to refuse a point, the
    /// dispatch returns `void`, and `stamp_xspice` has no failure channel at
    /// all — and Spectre has no XSPICE to be compared with. Turning it into a
    /// rejection would make every code model that writes a NaN on one pass cut
    /// the timestep, which is a behaviour change with no reference to check
    /// against. The repetition is the part that was wrong: the Newton loop
    /// evaluates XSPICE once per iteration and wrote this line every time, so
    /// a failing timepoint produced dozens of identical lines. The error text
    /// already names the instance (`"{instance}: {error}"`, the wave stepper);
    /// this adds the trial iterate and drops the repeats.
    fn warn_xspice_evaluation(&mut self, time: Value, error: &crate::xspice::CmError) {
        let line = format!("XSPICE evaluation error at the trial iterate t={time:.6e} s: {error}");
        if self.xspice_evaluation_warning.as_deref() == Some(line.as_str()) {
            return;
        }
        log::warn!("{line}");
        self.xspice_evaluation_warning = Some(line);
    }

    /// Fallible XSPICE evaluation for callers that must not report success
    /// after a model fails.
    pub fn try_evaluate_xspice(
        &mut self,
        time: Value,
        voltages: &[Value],
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_analysis(
            time,
            0.0,
            voltages,
            crate::xspice::AnalysisType::Transient,
        )
    }

    /// Fallible XSPICE evaluation for transient with explicit timestep.
    pub fn try_evaluate_xspice_with_timestep(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_analysis(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
        )
    }

    /// Evaluate all XSPICE code model instances for the requested analysis type.
    pub fn evaluate_xspice_with_analysis(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        analysis: crate::xspice::AnalysisType,
    ) {
        if let Err(e) = self.try_evaluate_xspice_with_analysis(time, timestep, voltages, analysis) {
            self.warn_xspice_evaluation(time, &e);
        }
    }

    /// Fallible XSPICE evaluation for the requested analysis type.
    pub fn try_evaluate_xspice_with_analysis(
        &mut self,
        time: Value,
        timestep: Value,
        solution: &[Value],
        analysis: crate::xspice::AnalysisType,
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_analysis_phase(
            time,
            timestep,
            solution,
            analysis,
            crate::xspice::EvaluationPhase::DirectEvaluation,
        )
    }

    fn try_evaluate_xspice_with_analysis_phase(
        &mut self,
        time: Value,
        timestep: Value,
        solution: &[Value],
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_analysis_phase_and_coefficients(
            time,
            timestep,
            solution,
            analysis,
            phase,
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
        )
    }

    fn try_evaluate_xspice_with_analysis_phase_and_coefficients(
        &mut self,
        time: Value,
        timestep: Value,
        solution: &[Value],
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion: XspiceCompanionPolicy<'_>,
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_resources(
            time, timestep, solution, analysis, phase, companion, None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn try_evaluate_xspice_with_resources(
        &mut self,
        time: Value,
        timestep: Value,
        solution: &[Value],
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion: XspiceCompanionPolicy<'_>,
        resources: Option<&ResourceTransaction>,
    ) -> crate::xspice::CmResult<()> {
        if self.has_coupled_event_nets() {
            return Err(crate::xspice::CmError::EvaluationError(
                "enrolled XSPICE drivers require the joint circuit candidate".into(),
            ));
        }
        let mut wave = self.begin_xspice_active_wave(time, timestep, analysis, phase, companion)?;
        while self.step_xspice_active_wave(&mut wave, solution, resources)? {}
        Ok(())
    }

    pub(crate) fn begin_xspice_active_wave(
        &mut self,
        time: Value,
        timestep: Value,
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion: XspiceCompanionPolicy<'_>,
    ) -> crate::xspice::CmResult<XspiceActiveWave> {
        if let Some(error) = self
            .xspice_resource_failure
            .as_ref()
            .and_then(|failure| failure.get())
        {
            return Err(crate::xspice::CmError::EvaluationError(error.clone()));
        }
        self.ensure_xspice_event_dispatch();
        let count = self.xspice_instances.len();
        self.xspice_dispatch_pending.clear();
        self.xspice_dispatch_pending.resize(count, false);
        self.xspice_dispatch_next_pending.clear();
        self.xspice_dispatch_next_pending.resize(count, false);
        Ok(XspiceActiveWave {
            time,
            timestep,
            analysis,
            phase,
            coefficients: *companion.coefficients,
            xyce_one_step_order2: companion.xyce_one_step_order2,
            current_source_values: self.current_sources.values_at_time(time),
            analog_transitions: HashMap::new(),
            pass: 0,
        })
    }

    /// Owe an evaluation to every instance reading a voltage from `nodes`.
    ///
    /// The accepted step's voltage projection writes model outputs back into
    /// the solution and then re-settles the boundary. Whatever reads one of
    /// the rows it moved has a stale input and must run again; everything else
    /// would recompute the same answer from the same inputs, and running it
    /// again would execute its accepted-phase side effects a second time.
    #[cfg(feature = "veriloga")]
    pub(crate) fn record_xspice_analog_input_dispatch(&mut self, nodes: &[NodeId]) {
        if nodes.is_empty() {
            return;
        }
        // Whoever opened the wave these marks belong to sized the pending
        // vector from the instance count, and that count is fixed once the
        // circuit is built, so the two agree on every path that reaches here.
        // They have to: a short vector cannot be indexed by instance, and
        // returning instead would accept the candidate with the stale analog
        // inputs the projection just invalidated -- no error, no log, no
        // symptom but a wrong answer. Say it where a debug build can hear it,
        // and keep the release build's harmless refusal to index out of range.
        debug_assert_eq!(
            self.xspice_dispatch_pending.len(),
            self.xspice_instances.len(),
            "XSPICE analog-input dispatch marks dropped: {} pending slot(s) for {} instance(s)",
            self.xspice_dispatch_pending.len(),
            self.xspice_instances.len()
        );
        if self.xspice_dispatch_pending.len() != self.xspice_instances.len() {
            return;
        }
        self.ensure_xspice_event_dispatch();
        let dispatch = self
            .xspice_event_dispatch
            .as_ref()
            .expect("prepared event dispatch");
        dispatch.record_analog_fanout_pending(&mut self.xspice_dispatch_pending, nodes);
    }

    /// Publish an already-resolved shared-net observation bank. These values
    /// are not XSPICE output contributions. Mark both the persistent input
    /// signature and this active wave's pending fanout before resuming it.
    ///
    /// Shared-net observation is the mixed HDL path, so this follows the
    /// `coupled` module that drives it rather than being dead without it.
    #[cfg(feature = "veriloga")]
    pub(crate) fn observe_xspice_shared_digital_inputs(
        &mut self,
        wave: &XspiceActiveWave,
        values: &[(NodeId, crate::xspice::DigitalValue)],
    ) {
        self.xspice_touched_digital_nodes.clear();
        for &(node, value) in values {
            if node == 0 {
                continue;
            }
            if self.xspice_event_values.digital_values.get(&node) == Some(&value)
                && self.xspice_event_values.digital_event_times.get(&node) == Some(&wave.time)
            {
                continue;
            }
            let observed = self.xspice_event_values.make_mut();
            observed.digital_values.insert(node, value);
            observed.digital_event_times.insert(node, wave.time);
            self.xspice_touched_digital_nodes.push(node);
        }
        let dispatch = self
            .xspice_event_dispatch
            .as_ref()
            .expect("prepared event wave");
        dispatch.mark_fanout_dirty(
            &mut self.xspice_instances,
            EventInputKind::Digital,
            &self.xspice_touched_digital_nodes,
        );
        dispatch.record_fanout_pending(
            &mut self.xspice_dispatch_pending,
            EventInputKind::Digital,
            &self.xspice_touched_digital_nodes,
        );
    }

    /// Execute one due-event/dirty-fanout wave, then yield to the circuit.
    /// True requests another Active wave; only false permits later HDL regions.
    pub(crate) fn step_xspice_active_wave(
        &mut self,
        wave: &mut XspiceActiveWave,
        solution: &[Value],
        resources: Option<&ResourceTransaction>,
    ) -> crate::xspice::CmResult<bool> {
        self.step_xspice_active_wave_with_resolver(
            wave,
            solution,
            resources,
            &mut LocalDigitalResolver,
        )
    }

    fn step_xspice_active_wave_with_resolver(
        &mut self,
        wave: &mut XspiceActiveWave,
        solution: &[Value],
        resources: Option<&ResourceTransaction>,
        resolver: &mut impl XspiceDigitalResolver,
    ) -> crate::xspice::CmResult<bool> {
        let time = wave.time;
        let timestep = wave.timestep;
        let analysis = wave.analysis;
        let phase = wave.phase;
        let companion_coefficients = wave.coefficients;
        let xyce_one_step_order2 = wave.xyce_one_step_order2;
        let current_source_values = &wave.current_source_values;
        let analog_transitions = &mut wave.analog_transitions;
        let pass = wave.pass;
        let num_nodes = self.num_nodes;
        let event_loads = &self.xspice_event_loads;
        let dispatch = self
            .xspice_event_dispatch
            .as_ref()
            .expect("prepared event wave");
        // Non-transient evaluations run every model body, as before. Only
        // transient event-only models can use dirty-input dispatch.
        let dirty_dispatch_applies = analysis == crate::xspice::AnalysisType::Transient;
        // Shared handles, not mutable views of their contents: a pass that
        // drains nothing and schedules nothing must leave the event world
        // still shared with the rollback snapshot.
        let event_values = &mut self.xspice_event_values;
        let event_queue = &mut self.xspice_event_queue;
        let touched_digital_nodes = &mut self.xspice_touched_digital_nodes;
        let touched_real_nodes = &mut self.xspice_touched_real_nodes;
        let instances = &mut self.xspice_instances;
        let pending = &mut self.xspice_dispatch_pending;
        let next_pending = &mut self.xspice_dispatch_next_pending;
        next_pending.fill(false);
        let mut changed = match apply_xspice_events_with_resolver(
            event_values,
            event_queue,
            touched_digital_nodes,
            touched_real_nodes,
            time,
            resolver,
        ) {
            Ok(changed) => changed,
            Err(error) => {
                // A drain that fails mid-slot has already applied the
                // events it executed, so the flags are owed before the
                // error leaves. Only the diagnostic path continues from
                // here, and it must not read a stale flag.
                mark_drained_fanout_dirty(
                    dispatch,
                    instances,
                    touched_digital_nodes,
                    touched_real_nodes,
                );
                let message = match &error {
                    crate::xspice::CmError::EvaluationError(message) => message.clone(),
                    _ => error.to_string(),
                };
                if self.xspice_evaluation_error.is_none() {
                    self.xspice_evaluation_error = Some(message.clone());
                }
                return Err(error);
            }
        };
        // What the drain just touched is owed an evaluation in *this*
        // pass, which is what the node-list dispatch this replaced did.
        mark_drained_fanout_dirty(
            dispatch,
            instances,
            touched_digital_nodes,
            touched_real_nodes,
        );
        dispatch.record_fanout_pending(pending, EventInputKind::Digital, touched_digital_nodes);
        dispatch.record_fanout_pending(pending, EventInputKind::Real, touched_real_nodes);
        if pass == 0 {
            // The opening pass dispatches every instance, exactly as it
            // always has. Narrowing it is the quiet-input check below, and
            // only that check: an instance with no input ports at all —
            // `d_pullup`, `d_pulldown` — is reached by no net's fan-out
            // and would otherwise never run even once.
            pending.fill(true);
        }

        for index in 0..instances.len() {
            if !pending[index] {
                continue;
            }
            pending[index] = false;
            // Read-only until the skip check has had its say: an instance
            // the dispatch skips must stay shared with the rollback
            // snapshot, or the copy this whole arrangement defers happens
            // anyway.
            let instance = &instances[index];
            if dirty_dispatch_applies
                && dispatch.is_dirty_dispatched(index)
                && !instance.event_inputs_dirty()
            {
                #[cfg(debug_assertions)]
                instance.debug_assert_event_inputs_quiet(
                    solution,
                    num_nodes,
                    XspiceEventInputs {
                        digital_values: &event_values.digital_values,
                        digital_event_times: &event_values.digital_event_times,
                        event_total_loads: event_loads,
                        real_values: &event_values.real_values,
                        real_event_times: &event_values.real_event_times,
                    },
                    current_source_values,
                    analog_transitions,
                    analysis,
                );
                continue;
            }

            // Past the skip check, so this instance is one the dispatch
            // did not avoid. Counted here rather than at `evaluate` so the
            // tally is of settle-loop work, not of every path that happens
            // to call a model.
            crate::xspice::settle_cost::note_instance_evaluation();
            let instance = instances[index].make_mut();
            instance.set_transient_companion_coefficients(companion_coefficients);
            instance.set_xyce_one_step_order2(xyce_one_step_order2);
            if let Err(e) = instance.update_inputs_with_analog_transitions(
                solution,
                num_nodes,
                XspiceEventInputs {
                    digital_values: &event_values.digital_values,
                    digital_event_times: &event_values.digital_event_times,
                    event_total_loads: event_loads,
                    real_values: &event_values.real_values,
                    real_event_times: &event_values.real_event_times,
                },
                current_source_values,
                analog_transitions,
            ) {
                let message = format!("{}: {}", instance.name, e);
                if self.xspice_evaluation_error.is_none() {
                    self.xspice_evaluation_error = Some(message.clone());
                }
                return Err(crate::xspice::CmError::EvaluationError(message));
            }

            if let Err(e) = instance.evaluate_with_resource_transaction(
                time, timestep, analysis, phase, resources, index,
            ) {
                let message = format!("{}: {}", instance.name, e);
                if self.xspice_evaluation_error.is_none() {
                    self.xspice_evaluation_error = Some(message.clone());
                }
                return Err(crate::xspice::CmError::EvaluationError(message));
            }
            for (key, transition) in instance.analog_output_transitions() {
                analog_transitions.insert(key, transition);
            }

            // Asking first keeps the sweep off the copy-on-write path for
            // every instance whose evaluation queued no output, which on a
            // settling gate-level design is nearly all of them. The drain
            // this replaces would have moved an empty pending list into
            // the scheduler and copied it to do so.
            if instance.has_pending_events() {
                instance.schedule_events(event_queue.make_mut(), time);
            }
            let instance_changed = match apply_xspice_events_with_resolver(
                event_values,
                event_queue,
                touched_digital_nodes,
                touched_real_nodes,
                time,
                resolver,
            ) {
                Ok(changed) => changed,
                Err(error) => {
                    mark_drained_fanout_dirty(
                        dispatch,
                        instances,
                        touched_digital_nodes,
                        touched_real_nodes,
                    );
                    let message = match &error {
                        crate::xspice::CmError::EvaluationError(message) => message.clone(),
                        _ => error.to_string(),
                    };
                    if self.xspice_evaluation_error.is_none() {
                        self.xspice_evaluation_error = Some(message.clone());
                    }
                    return Err(error);
                }
            };
            // A driver reached these nets whether or not the resolved
            // value moved, so the persistent flags are owed either way.
            mark_drained_fanout_dirty(
                dispatch,
                instances,
                touched_digital_nodes,
                touched_real_nodes,
            );
            if instance_changed {
                changed = true;
                // Owed to the *next* pass, not this one: an instance
                // already walked past keeps its place in registration
                // order rather than being revisited out of turn. Gating
                // this on `instance_changed` is what the node-list
                // dispatch it replaces did.
                dispatch.record_fanout_pending(
                    next_pending,
                    EventInputKind::Digital,
                    touched_digital_nodes,
                );
                dispatch.record_fanout_pending(
                    next_pending,
                    EventInputKind::Real,
                    touched_real_nodes,
                );
            }
        }

        wave.pass += 1;
        if !changed {
            return Ok(false);
        }

        // Something moved, so another delta cycle is owed. The scheduler
        // is what decides the network has stopped converging.
        //
        // Unguarded on purpose: reaching here means a drain returned
        // `true`, which it can only do past its emptiness check, so the
        // scheduler is already unshared and this mutable view copies
        // nothing.
        if let Err(error) = event_queue.make_mut().note_delta_cycle(time) {
            let message = xspice_event_settling_message(time, &error);
            if self.xspice_evaluation_error.is_none() {
                self.xspice_evaluation_error = Some(message.clone());
            }
            return Err(crate::xspice::CmError::EvaluationError(message));
        }
        std::mem::swap(
            &mut self.xspice_dispatch_pending,
            &mut self.xspice_dispatch_next_pending,
        );
        Ok(true)
    }

    /// Return a permanent resource failure, or consume the first recoverable
    /// XSPICE evaluation error recorded during this analysis.
    pub(crate) fn take_xspice_evaluation_error(&mut self) -> Option<String> {
        self.xspice_resource_failure
            .as_ref()
            .and_then(|failure| failure.get())
            .cloned()
            .or_else(|| self.xspice_evaluation_error.take())
    }

    /// Fill a reusable snapshot of committed event-driven digital node values.
    ///
    /// XSPICE's resolved event nodes and the boundary nets of any instantiated
    /// mixed Verilog-AMS module go into one vector, because they answer one
    /// question — what four-state value did this net hold at this accepted
    /// timepoint — and because `TransientResult::record_digital_snapshot` is
    /// the single writer of the digital trace channel. A circuit with no mixed
    /// module appends nothing, so the snapshot is the one it always was.
    pub(crate) fn fill_xspice_digital_snapshot(
        &self,
        snapshot: &mut Vec<(NodeId, crate::xspice::DigitalValue)>,
    ) {
        snapshot.clear();
        snapshot.extend(self.xspice_event_values.digital_values.iter().filter_map(
            |(&node_id, &value)| {
                if node_id == 0 {
                    return None;
                }
                #[cfg(feature = "veriloga")]
                if self
                    .mixed_xspice_bindings
                    .as_ref()
                    .is_some_and(|bindings| bindings.contains_node(node_id))
                {
                    return None;
                }
                Some((node_id, value))
            },
        ));
        #[cfg(feature = "veriloga")]
        self.append_mixed_digital_snapshot(snapshot);
        // Stable and then deduplicated, so the snapshot is a function from node
        // to value: one node, one committed value, whichever half of the
        // circuit published it. XSPICE's own event nodes and the mixed
        // boundaries are disjoint by construction — a net shared with an HDL
        // port is filtered out above and republished by the coordinator — so
        // the deduplication removes nothing today. It is here because
        // `record_digital_snapshot` writes a trace point per entry, and a
        // second entry for a node it has already written at this timepoint is
        // a zero-width glitch rather than an event.
        snapshot.sort_by_key(|(node_id, _)| *node_id);
        snapshot.dedup_by_key(|(node_id, _)| *node_id);
    }

    /// Fill a reusable snapshot of committed XSPICE real event-node values.
    pub(crate) fn fill_xspice_real_snapshot(&self, snapshot: &mut Vec<(NodeId, Value)>) {
        snapshot.clear();
        snapshot.extend(
            self.xspice_event_values
                .real_values
                .iter()
                .filter_map(|(&node_id, &value)| (node_id > 0).then_some((node_id, value))),
        );
        snapshot.sort_unstable_by_key(|(node_id, _)| *node_id);
    }

    /// Time of the next pending XSPICE digital event, if any.
    pub(crate) fn next_xspice_event_time(&self) -> Option<Value> {
        self.xspice_event_queue.next_event_time()
    }

    /// Drain absolute transient breakpoint requests emitted by XSPICE models.
    #[cfg(test)]
    pub(crate) fn take_xspice_requested_breakpoints(&mut self) -> Vec<Value> {
        let mut breakpoints = Vec::new();
        self.drain_xspice_requested_breakpoints(|time| breakpoints.push(time));
        breakpoints
    }

    /// Drain XSPICE breakpoint requests directly into a caller-provided sink.
    pub(crate) fn drain_xspice_requested_breakpoints<F>(&mut self, mut sink: F)
    where
        F: FnMut(Value),
    {
        for instance in &mut self.xspice_instances {
            // Asking first keeps the sweep off the copy-on-write path for
            // every instance that requested nothing, which on a gate-level
            // design is all but a handful.
            if !instance.has_requested_breakpoints() {
                continue;
            }
            for time in instance.make_mut().drain_requested_breakpoints() {
                sink(time);
            }
        }
    }

    /// Evaluate and stamp XSPICE for a transient solver trial without committing
    /// code-model state. The matrix/RHS receive the trial contributions, while
    /// digital queues, event timestamps, and model contexts are restored before
    /// the solver decides whether the step is accepted.
    pub(crate) fn stamp_xspice_transient_trial(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) {
        self.stamp_xspice_transient_trial_with_coefficients(
            matrix,
            rhs,
            time,
            timestep,
            voltages,
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
        );
    }

    /// Evaluate and stamp a transient trial using the engine's companion
    /// coefficients.  Trial state is still restored before returning.
    pub(crate) fn stamp_xspice_transient_trial_with_coefficients(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        timestep: Value,
        voltages: &[Value],
        companion: XspiceCompanionPolicy<'_>,
    ) {
        if !self.has_independent_xspice_evaluation() {
            return;
        }
        let XspiceCompanionPolicy {
            coefficients,
            xyce_one_step_order2,
        } = companion;
        let snapshot = self.capture_xspice_trial_state();
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase_and_coefficients(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::RollbackableProbe,
            XspiceCompanionPolicy {
                coefficients,
                xyce_one_step_order2,
            },
        ) {
            self.warn_xspice_evaluation(time, &e);
        }
        self.stamp_xspice(matrix, rhs);
        self.restore_xspice_trial_state(snapshot);
    }

    /// Evaluate XSPICE for an accepted transient timepoint without advancing
    /// committed model state.  The accepted-phase evaluation queues the
    /// static output stamps consumed by Xyce's OneStep history snapshot; the
    /// caller must invoke [`Self::accept_xspice_timestep`] after that snapshot
    /// has been captured.
    pub(crate) fn evaluate_xspice_transient_timestep_with_coefficients(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        companion: XspiceCompanionPolicy<'_>,
        resources: Option<&ResourceTransaction>,
    ) -> crate::xspice::CmResult<()> {
        self.try_evaluate_xspice_with_resources(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::AcceptedStep,
            companion,
            resources,
        )
    }

    /// Capture the state a trial evaluation can write. See [`XspiceTrialState`].
    pub(crate) fn capture_xspice_trial_state(&self) -> XspiceTrialState {
        XspiceTrialState {
            instances: self.xspice_instances.clone(),
            values: self.xspice_event_values.clone(),
            queue: self.xspice_event_queue.clone(),
        }
    }

    /// Put back what a trial evaluation wrote.
    pub(crate) fn restore_xspice_trial_state(&mut self, trial: XspiceTrialState) {
        self.xspice_instances = trial.instances;
        self.xspice_event_values = trial.values;
        self.xspice_event_queue = trial.queue;
    }

    pub(crate) fn capture_xspice_acceptance(&self) -> XspiceAcceptanceRollback {
        XspiceAcceptanceRollback {
            instances: self.xspice_instances.clone(),
            values: self.xspice_event_values.clone(),
            queue: self.xspice_event_queue.clone(),
            error: self.xspice_evaluation_error.clone(),
            resources: ResourceTransaction::default(),
        }
    }

    pub(crate) fn restore_xspice_acceptance(
        &mut self,
        rollback: XspiceAcceptanceRollback,
    ) -> crate::xspice::CmResult<()> {
        self.xspice_instances = rollback.instances;
        self.xspice_event_values = rollback.values;
        self.xspice_event_queue = rollback.queue;
        self.xspice_evaluation_error = rollback.error;
        let failures = rollback.resources.rollback();
        if failures.is_empty() {
            return Ok(());
        }
        let details: Vec<_> = failures
            .into_iter()
            .map(|failure| {
                let owner = self
                    .xspice_instances
                    .get(failure.owner)
                    .map(|instance| instance.name.as_str())
                    .unwrap_or("unknown instance");
                format!("{owner} resource '{}': {}", failure.key, failure.detail)
            })
            .collect();
        let error = format!("XSPICE resource rollback failed: {}", details.join("; "));
        let failure = self
            .xspice_resource_failure
            .get_or_insert_with(|| Arc::new(std::sync::OnceLock::new()));
        let _ = failure.set(error.clone());
        Err(crate::xspice::CmError::EvaluationError(error))
    }

    /// Standalone XSPICE acceptance has the same failure contract as the joint
    /// circuit barrier: restore model state and pending events before reporting.
    #[cfg(test)]
    pub(crate) fn accept_xspice_transient_timestep_with_coefficients(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        companion: XspiceCompanionPolicy<'_>,
    ) -> crate::xspice::CmResult<()> {
        let rollback = self.capture_xspice_acceptance();
        if let Err(error) = self.evaluate_xspice_transient_timestep_with_coefficients(
            time,
            timestep,
            voltages,
            companion,
            Some(rollback.resources()),
        ) {
            if let Err(restore) = self.restore_xspice_acceptance(rollback) {
                return Err(crate::xspice::CmError::EvaluationError(format!(
                    "{error}; {restore}"
                )));
            }
            return Err(error);
        }
        self.accept_xspice_timestep();
        rollback.resources().commit();
        Ok(())
    }

    /// Stamp XSPICE analog contributions into matrix and RHS
    ///
    /// After evaluation, analog code models produce conductance and current
    /// contributions that must be stamped into the MNA system.
    pub fn stamp_xspice(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;

        #[inline]
        fn add_matrix_if_present(matrix: &mut StaticMatrix, row: usize, col: usize, value: Value) {
            if matrix.get_index(row, col).is_some() {
                matrix.add(row, col, value);
            } else {
                log::debug!(
                    "XSPICE stamp ({}, {}) missing from matrix topology",
                    row,
                    col
                );
            }
        }

        #[inline]
        fn add_rhs_if_present(rhs: &mut [Value], index: usize, value: Value) {
            if let Some(entry) = rhs.get_mut(index) {
                *entry += value;
            } else {
                log::debug!("XSPICE RHS stamp {} outside RHS size {}", index, rhs.len());
            }
        }

        #[inline]
        fn stamp_current_output_source(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
        ) {
            if pos > 0 {
                let pos_row = pos - 1;
                add_matrix_if_present(matrix, pos_row, pos_row, conductance);
                if neg > 0 {
                    add_matrix_if_present(matrix, pos_row, neg - 1, -conductance);
                }
                add_rhs_if_present(rhs, pos_row, -current);
            }
            if neg > 0 {
                let neg_row = neg - 1;
                if pos > 0 {
                    add_matrix_if_present(matrix, neg_row, pos - 1, -conductance);
                }
                add_matrix_if_present(matrix, neg_row, neg_row, conductance);
                add_rhs_if_present(rhs, neg_row, current);
            }
        }

        #[inline]
        fn current_output_self_conductance(
            port: &crate::xspice::PortSpec,
            conductance: Value,
        ) -> Value {
            match port.default_type {
                crate::xspice::PortType::Current
                | crate::xspice::PortType::DifferentialCurrent
                | crate::xspice::PortType::Conductance
                | crate::xspice::PortType::DifferentialConductance => conductance,
                _ => 0.0,
            }
        }

        #[inline]
        fn stamp_legacy_nodal_output(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            connection: &crate::xspice::PortConnection,
            conductance: Value,
            current: Value,
        ) {
            match connection {
                crate::xspice::PortConnection::Analog(node) => {
                    if *node > 0 {
                        let row = *node - 1;
                        add_matrix_if_present(matrix, row, row, conductance);
                        add_rhs_if_present(rhs, row, current);
                    }
                }
                crate::xspice::PortConnection::Differential(pos, neg) => {
                    if *pos > 0 {
                        let pos_row = *pos - 1;
                        add_matrix_if_present(matrix, pos_row, pos_row, conductance);
                        if *neg > 0 {
                            add_matrix_if_present(matrix, pos_row, *neg - 1, -conductance);
                        }
                        add_rhs_if_present(rhs, pos_row, current);
                    }
                    if *neg > 0 {
                        let neg_row = *neg - 1;
                        if *pos > 0 {
                            add_matrix_if_present(matrix, neg_row, *pos - 1, -conductance);
                        }
                        add_matrix_if_present(matrix, neg_row, neg_row, conductance);
                        add_rhs_if_present(rhs, neg_row, -current);
                    }
                }
                _ => {}
            }
        }

        #[inline]
        fn stamp_voltage_control_partial(
            matrix: &mut StaticMatrix,
            branch_row: usize,
            connection: &crate::xspice::PortConnection,
            partial: Value,
            num_nodes: usize,
        ) {
            match connection {
                crate::xspice::PortConnection::Analog(node) => {
                    if *node > 0 {
                        add_matrix_if_present(matrix, branch_row, *node - 1, -partial);
                    }
                }
                crate::xspice::PortConnection::Differential(pos, neg) => {
                    if *pos > 0 {
                        add_matrix_if_present(matrix, branch_row, *pos - 1, -partial);
                    }
                    if *neg > 0 {
                        add_matrix_if_present(matrix, branch_row, *neg - 1, partial);
                    }
                }
                crate::xspice::PortConnection::CurrentProbe { branch_ordinal, .. }
                | crate::xspice::PortConnection::BranchCurrent { branch_ordinal }
                | crate::xspice::PortConnection::Hybrid { branch_ordinal, .. } => {
                    add_matrix_if_present(
                        matrix,
                        branch_row,
                        num_nodes + *branch_ordinal - 1,
                        -partial,
                    );
                }
                crate::xspice::PortConnection::NamedBranchCurrent {
                    branch_ordinal: Some(branch_ordinal),
                    ..
                } => {
                    add_matrix_if_present(
                        matrix,
                        branch_row,
                        num_nodes + *branch_ordinal - 1,
                        -partial,
                    );
                }
                _ => {}
            }
        }

        #[inline]
        fn stamp_voltage_control_vector_partial(
            matrix: &mut StaticMatrix,
            branch_row: usize,
            connection: &crate::xspice::PortConnection,
            index: usize,
            partial: Value,
            num_nodes: usize,
        ) {
            match connection {
                crate::xspice::PortConnection::AnalogVector(nodes) => {
                    if let Some(node) = nodes.get(index)
                        && *node > 0
                    {
                        add_matrix_if_present(matrix, branch_row, *node - 1, -partial);
                    }
                }
                crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                    if let Some(element) = elements.get(index) {
                        if let Some(branch_ordinal) = element.branch_ordinal() {
                            add_matrix_if_present(
                                matrix,
                                branch_row,
                                num_nodes + branch_ordinal - 1,
                                -partial,
                            );
                        } else if let Some(node) = element.primary_node()
                            && node > 0
                        {
                            add_matrix_if_present(matrix, branch_row, node - 1, -partial);
                        }
                    }
                }
                _ => {}
            }
        }

        #[inline]
        fn stamp_current_control_column(
            matrix: &mut StaticMatrix,
            pos: usize,
            neg: usize,
            col: usize,
            partial: Value,
        ) {
            if pos > 0 {
                add_matrix_if_present(matrix, pos - 1, col, partial);
            }
            if neg > 0 {
                add_matrix_if_present(matrix, neg - 1, col, -partial);
            }
        }

        #[inline]
        fn stamp_current_control_partial(
            matrix: &mut StaticMatrix,
            pos: usize,
            neg: usize,
            connection: &crate::xspice::PortConnection,
            partial: Value,
            num_nodes: usize,
        ) {
            match connection {
                crate::xspice::PortConnection::Analog(node) => {
                    if *node > 0 {
                        stamp_current_control_column(matrix, pos, neg, *node - 1, partial);
                    }
                }
                crate::xspice::PortConnection::Differential(ctrl_pos, ctrl_neg) => {
                    if *ctrl_pos > 0 {
                        stamp_current_control_column(matrix, pos, neg, *ctrl_pos - 1, partial);
                    }
                    if *ctrl_neg > 0 {
                        stamp_current_control_column(matrix, pos, neg, *ctrl_neg - 1, -partial);
                    }
                }
                crate::xspice::PortConnection::CurrentProbe { branch_ordinal, .. }
                | crate::xspice::PortConnection::BranchCurrent { branch_ordinal }
                | crate::xspice::PortConnection::Hybrid { branch_ordinal, .. } => {
                    stamp_current_control_column(
                        matrix,
                        pos,
                        neg,
                        num_nodes + *branch_ordinal - 1,
                        partial,
                    );
                }
                crate::xspice::PortConnection::NamedBranchCurrent {
                    branch_ordinal: Some(branch_ordinal),
                    ..
                } => {
                    stamp_current_control_column(
                        matrix,
                        pos,
                        neg,
                        num_nodes + *branch_ordinal - 1,
                        partial,
                    );
                }
                _ => {}
            }
        }

        #[inline]
        fn stamp_current_control_vector_partial(
            matrix: &mut StaticMatrix,
            pos: usize,
            neg: usize,
            connection: &crate::xspice::PortConnection,
            index: usize,
            partial: Value,
            num_nodes: usize,
        ) {
            match connection {
                crate::xspice::PortConnection::AnalogVector(nodes) => {
                    if let Some(node) = nodes.get(index)
                        && *node > 0
                    {
                        stamp_current_control_column(matrix, pos, neg, *node - 1, partial);
                    }
                }
                crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                    if let Some(element) = elements.get(index) {
                        if let Some(branch_ordinal) = element.branch_ordinal() {
                            stamp_current_control_column(
                                matrix,
                                pos,
                                neg,
                                num_nodes + branch_ordinal - 1,
                                partial,
                            );
                        } else if let Some(node) = element.primary_node()
                            && node > 0
                        {
                            stamp_current_control_column(matrix, pos, neg, node - 1, partial);
                        }
                    }
                }
                _ => {}
            }
        }

        #[inline]
        fn stamp_current_probe(
            matrix: &mut StaticMatrix,
            pos: usize,
            neg: usize,
            branch_ordinal: usize,
            num_nodes: usize,
        ) {
            let branch = num_nodes + branch_ordinal;
            let branch_row = branch - 1;
            if pos > 0 {
                add_matrix_if_present(matrix, branch_row, pos - 1, 1.0);
                add_matrix_if_present(matrix, pos - 1, branch_row, 1.0);
            }
            if neg > 0 {
                add_matrix_if_present(matrix, branch_row, neg - 1, -1.0);
                add_matrix_if_present(matrix, neg - 1, branch_row, -1.0);
            }
        }

        /// Where one XSPICE output port writes: the MNA system, the instance
        /// and port declaration that name the rows, and the node count that
        /// separates node rows from branch rows.
        struct XspiceOutputStamp<'a, 'b> {
            matrix: &'a mut StaticMatrix,
            rhs: &'a mut [Value],
            instance: &'b crate::xspice::XspiceInstance,
            port: &'b crate::xspice::PortSpec,
            num_nodes: usize,
        }

        fn stamp_current_output_port(
            stamp: XspiceOutputStamp<'_, '_>,
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
        ) {
            let XspiceOutputStamp {
                matrix,
                rhs,
                instance,
                port,
                num_nodes,
            } = stamp;
            let mut equivalent_current = current;
            for (control_port, partial) in instance.output_input_partials(&port.name) {
                if !partial.is_finite() {
                    continue;
                }
                equivalent_current -= partial * instance.analog_input_value(&control_port);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_current_control_partial(
                        matrix,
                        pos,
                        neg,
                        control_connection,
                        partial,
                        num_nodes,
                    );
                }
            }
            for (control_port, index, partial) in instance.output_input_vector_partials(&port.name)
            {
                if !partial.is_finite() {
                    continue;
                }
                equivalent_current -=
                    partial * instance.analog_vector_input_value(&control_port, index);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_current_control_vector_partial(
                        matrix,
                        pos,
                        neg,
                        control_connection,
                        index,
                        partial,
                        num_nodes,
                    );
                }
            }
            stamp_current_output_source(
                matrix,
                rhs,
                pos,
                neg,
                current_output_self_conductance(port, conductance),
                equivalent_current,
            );
        }

        fn stamp_current_vector_output_port(
            stamp: XspiceOutputStamp<'_, '_>,
            output_index: usize,
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
        ) {
            let XspiceOutputStamp {
                matrix,
                rhs,
                instance,
                port,
                num_nodes,
            } = stamp;
            let mut equivalent_current = current;
            for (control_port, partial) in
                instance.output_vector_input_partials(&port.name, output_index)
            {
                if !partial.is_finite() {
                    continue;
                }
                equivalent_current -= partial * instance.analog_input_value(&control_port);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_current_control_partial(
                        matrix,
                        pos,
                        neg,
                        control_connection,
                        partial,
                        num_nodes,
                    );
                }
            }
            for (control_port, index, partial) in
                instance.output_vector_input_vector_partials(&port.name, output_index)
            {
                if !partial.is_finite() {
                    continue;
                }
                equivalent_current -=
                    partial * instance.analog_vector_input_value(&control_port, index);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_current_control_vector_partial(
                        matrix,
                        pos,
                        neg,
                        control_connection,
                        index,
                        partial,
                        num_nodes,
                    );
                }
            }
            stamp_current_output_source(
                matrix,
                rhs,
                pos,
                neg,
                current_output_self_conductance(port, conductance),
                equivalent_current,
            );
        }

        #[inline]
        fn stamp_voltage_output_branch(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            branch_ordinal: usize,
            pos: usize,
            neg: usize,
            value: Value,
            num_nodes: usize,
        ) {
            let br_mna = num_nodes + branch_ordinal;
            let br = br_mna - 1;
            if br >= rhs.len() {
                log::debug!("XSPICE branch row {} outside RHS size {}", br, rhs.len());
                return;
            }
            if pos > 0 {
                let pos_row = pos - 1;
                add_matrix_if_present(matrix, br, pos_row, 1.0);
                add_matrix_if_present(matrix, pos_row, br, 1.0);
            }
            if neg > 0 {
                let neg_row = neg - 1;
                add_matrix_if_present(matrix, br, neg_row, -1.0);
                add_matrix_if_present(matrix, neg_row, br, -1.0);
            }
            add_rhs_if_present(rhs, br, value);
        }

        fn stamp_vector_voltage_output_branch(
            stamp: XspiceOutputStamp<'_, '_>,
            output_index: usize,
            branch_ordinal: usize,
            pos: usize,
            neg: usize,
            value: Value,
        ) {
            let XspiceOutputStamp {
                matrix,
                rhs,
                instance,
                port,
                num_nodes,
            } = stamp;
            let br_mna = num_nodes + branch_ordinal;
            let br = br_mna - 1;
            if br >= rhs.len() {
                log::debug!("XSPICE branch row {} outside RHS size {}", br, rhs.len());
                return;
            }

            let mut branch_rhs = value;
            for (control_port, partial) in
                instance.output_vector_input_partials(&port.name, output_index)
            {
                if !partial.is_finite() {
                    continue;
                }
                branch_rhs -= partial * instance.analog_input_value(&control_port);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_voltage_control_partial(
                        matrix,
                        br,
                        control_connection,
                        partial,
                        num_nodes,
                    );
                }
            }
            for (control_port, index, partial) in
                instance.output_vector_input_vector_partials(&port.name, output_index)
            {
                if !partial.is_finite() {
                    continue;
                }
                branch_rhs -= partial * instance.analog_vector_input_value(&control_port, index);
                if let Some(control_connection) = instance.connection(&control_port) {
                    stamp_voltage_control_vector_partial(
                        matrix,
                        br,
                        control_connection,
                        index,
                        partial,
                        num_nodes,
                    );
                }
            }

            stamp_voltage_output_branch(
                matrix,
                rhs,
                branch_ordinal,
                pos,
                neg,
                branch_rhs,
                num_nodes,
            );
        }

        for slot in &mut self.xspice_instances {
            let instance = &**slot;
            for (pos, neg, branch_ordinal) in instance.current_probe_branches() {
                stamp_current_probe(matrix, pos, neg, branch_ordinal, num_nodes);
            }

            let ports = instance.ports();
            // Get contributions from each output port
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                if instance.has_analog_vector_contributions(port_idx) {
                    let Some(port) = ports.get(port_idx) else {
                        continue;
                    };
                    match connection {
                        crate::xspice::PortConnection::AnalogVector(nodes) => {
                            for (index, node) in nodes.iter().copied().enumerate() {
                                let (conductance, current) =
                                    instance.analog_vector_contribution_at(port_idx, index);
                                match port.default_type {
                                    crate::xspice::PortType::Voltage
                                    | crate::xspice::PortType::DifferentialVoltage
                                    | crate::xspice::PortType::Hybrid
                                    | crate::xspice::PortType::DifferentialHybrid => {
                                        if let Some(branch_ordinal) =
                                            instance.branch_vector_output_ordinal(port_idx, index)
                                        {
                                            stamp_vector_voltage_output_branch(
                                                XspiceOutputStamp {
                                                    matrix,
                                                    rhs,
                                                    instance,
                                                    port,
                                                    num_nodes,
                                                },
                                                index,
                                                branch_ordinal,
                                                node,
                                                0,
                                                current,
                                            );
                                        }
                                    }
                                    crate::xspice::PortType::Current
                                    | crate::xspice::PortType::DifferentialCurrent
                                    | crate::xspice::PortType::Conductance
                                    | crate::xspice::PortType::DifferentialConductance => {
                                        stamp_current_vector_output_port(
                                            XspiceOutputStamp {
                                                matrix,
                                                rhs,
                                                instance,
                                                port,
                                                num_nodes,
                                            },
                                            index,
                                            node,
                                            0,
                                            conductance,
                                            current,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                            for (index, element) in elements.iter().enumerate() {
                                let (conductance, current) =
                                    instance.analog_vector_contribution_at(port_idx, index);
                                match element {
                                    crate::xspice::AnalogInputConnection::Node(node) => {
                                        match port.default_type {
                                            crate::xspice::PortType::Voltage
                                            | crate::xspice::PortType::DifferentialVoltage
                                            | crate::xspice::PortType::Hybrid
                                            | crate::xspice::PortType::DifferentialHybrid => {
                                                if let Some(branch_ordinal) = instance
                                                    .branch_vector_output_ordinal(port_idx, index)
                                                {
                                                    stamp_vector_voltage_output_branch(
                                                        XspiceOutputStamp {
                                                            matrix,
                                                            rhs,
                                                            instance,
                                                            port,
                                                            num_nodes,
                                                        },
                                                        index,
                                                        branch_ordinal,
                                                        *node,
                                                        0,
                                                        current,
                                                    );
                                                }
                                            }
                                            crate::xspice::PortType::Current
                                            | crate::xspice::PortType::DifferentialCurrent
                                            | crate::xspice::PortType::Conductance
                                            | crate::xspice::PortType::DifferentialConductance => {
                                                stamp_current_vector_output_port(
                                                    XspiceOutputStamp {
                                                        matrix,
                                                        rhs,
                                                        instance,
                                                        port,
                                                        num_nodes,
                                                    },
                                                    index,
                                                    *node,
                                                    0,
                                                    conductance,
                                                    current,
                                                );
                                            }
                                            _ => {}
                                        }
                                    }
                                    crate::xspice::AnalogInputConnection::Differential(
                                        pos,
                                        neg,
                                    )
                                    | crate::xspice::AnalogInputConnection::Hybrid {
                                        pos,
                                        neg,
                                        ..
                                    } => match port.default_type {
                                        crate::xspice::PortType::Voltage
                                        | crate::xspice::PortType::DifferentialVoltage
                                        | crate::xspice::PortType::Hybrid
                                        | crate::xspice::PortType::DifferentialHybrid => {
                                            if let Some(branch_ordinal) = instance
                                                .branch_vector_output_ordinal(port_idx, index)
                                            {
                                                stamp_vector_voltage_output_branch(
                                                    XspiceOutputStamp {
                                                        matrix,
                                                        rhs,
                                                        instance,
                                                        port,
                                                        num_nodes,
                                                    },
                                                    index,
                                                    branch_ordinal,
                                                    *pos,
                                                    *neg,
                                                    current,
                                                );
                                            }
                                        }
                                        crate::xspice::PortType::Current
                                        | crate::xspice::PortType::DifferentialCurrent
                                        | crate::xspice::PortType::Conductance
                                        | crate::xspice::PortType::DifferentialConductance => {
                                            stamp_current_vector_output_port(
                                                XspiceOutputStamp {
                                                    matrix,
                                                    rhs,
                                                    instance,
                                                    port,
                                                    num_nodes,
                                                },
                                                index,
                                                *pos,
                                                *neg,
                                                conductance,
                                                current,
                                            );
                                        }
                                        _ => {}
                                    },
                                    crate::xspice::AnalogInputConnection::CurrentOutput {
                                        pos,
                                        neg,
                                    } => {
                                        stamp_current_vector_output_port(
                                            XspiceOutputStamp {
                                                matrix,
                                                rhs,
                                                instance,
                                                port,
                                                num_nodes,
                                            },
                                            index,
                                            *pos,
                                            *neg,
                                            conductance,
                                            current,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
                if let Some((conductance, current)) = instance.get_analog_contribution(port_idx) {
                    let Some(port) = ports.get(port_idx) else {
                        continue;
                    };
                    if let crate::xspice::PortConnection::CurrentOutput { pos, neg } = connection {
                        stamp_current_output_port(
                            XspiceOutputStamp {
                                matrix,
                                rhs,
                                instance,
                                port,
                                num_nodes,
                            },
                            *pos,
                            *neg,
                            conductance,
                            current,
                        );
                        continue;
                    }
                    match port.default_type {
                        crate::xspice::PortType::Voltage
                        | crate::xspice::PortType::DifferentialVoltage
                        | crate::xspice::PortType::Hybrid
                        | crate::xspice::PortType::DifferentialHybrid => {
                            if let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) {
                                let br_mna = num_nodes + branch_ordinal;
                                let br = br_mna - 1;
                                if br >= rhs.len() {
                                    log::debug!(
                                        "XSPICE branch row {} outside RHS size {}",
                                        br,
                                        rhs.len()
                                    );
                                    continue;
                                }
                                let mut branch_rhs = current;
                                for (control_port, partial) in
                                    instance.output_input_partials(&port.name)
                                {
                                    if !partial.is_finite() {
                                        continue;
                                    }
                                    branch_rhs -=
                                        partial * instance.analog_input_value(&control_port);
                                    if let Some(control_connection) =
                                        instance.connection(&control_port)
                                    {
                                        stamp_voltage_control_partial(
                                            matrix,
                                            br,
                                            control_connection,
                                            partial,
                                            num_nodes,
                                        );
                                    }
                                }
                                for (control_port, index, partial) in
                                    instance.output_input_vector_partials(&port.name)
                                {
                                    if !partial.is_finite() {
                                        continue;
                                    }
                                    branch_rhs -= partial
                                        * instance.analog_vector_input_value(&control_port, index);
                                    if let Some(control_connection) =
                                        instance.connection(&control_port)
                                    {
                                        stamp_voltage_control_vector_partial(
                                            matrix,
                                            br,
                                            control_connection,
                                            index,
                                            partial,
                                            num_nodes,
                                        );
                                    }
                                }
                                match connection {
                                    crate::xspice::PortConnection::Analog(node) => {
                                        if *node > 0 {
                                            let node_row = *node - 1;
                                            add_matrix_if_present(matrix, br, node_row, 1.0);
                                            add_matrix_if_present(matrix, node_row, br, 1.0);
                                        }
                                        add_rhs_if_present(rhs, br, branch_rhs);
                                    }
                                    crate::xspice::PortConnection::Differential(pos, neg) => {
                                        if *pos > 0 {
                                            let pos_row = *pos - 1;
                                            add_matrix_if_present(matrix, br, pos_row, 1.0);
                                            add_matrix_if_present(matrix, pos_row, br, 1.0);
                                        }
                                        if *neg > 0 {
                                            let neg_row = *neg - 1;
                                            add_matrix_if_present(matrix, br, neg_row, -1.0);
                                            add_matrix_if_present(matrix, neg_row, br, -1.0);
                                        }
                                        add_rhs_if_present(rhs, br, branch_rhs);
                                    }
                                    crate::xspice::PortConnection::Hybrid { pos, neg, .. } => {
                                        stamp_voltage_output_branch(
                                            matrix,
                                            rhs,
                                            branch_ordinal,
                                            *pos,
                                            *neg,
                                            branch_rhs,
                                            num_nodes,
                                        );
                                    }
                                    _ => {
                                        stamp_legacy_nodal_output(
                                            matrix,
                                            rhs,
                                            connection,
                                            conductance,
                                            current,
                                        );
                                    }
                                }
                            } else {
                                // Fallback for misconfigured instances: preserve behavior.
                                stamp_legacy_nodal_output(
                                    matrix,
                                    rhs,
                                    connection,
                                    conductance,
                                    current,
                                );
                            }
                        }
                        crate::xspice::PortType::Current
                        | crate::xspice::PortType::DifferentialCurrent
                        | crate::xspice::PortType::Conductance
                        | crate::xspice::PortType::DifferentialConductance => {
                            let (pos, neg) = match connection {
                                crate::xspice::PortConnection::Analog(node) => (*node, 0),
                                crate::xspice::PortConnection::Differential(pos, neg) => {
                                    (*pos, *neg)
                                }
                                _ => continue,
                            };
                            stamp_current_output_port(
                                XspiceOutputStamp {
                                    matrix,
                                    rhs,
                                    instance,
                                    port,
                                    num_nodes,
                                },
                                pos,
                                neg,
                                conductance,
                                current,
                            );
                        }
                        _ => {}
                    }
                }
            }

            // Drain any explicit matrix/RHS stamps queued by the code model.
            // The condition is the last read through the shared view, so the
            // mutable view below is free to copy — and only does so for an
            // instance that queued something.
            if instance.has_deferred_contributions() {
                let instance = slot.make_mut();
                for (row, col, value) in instance.take_deferred_stamps() {
                    add_matrix_if_present(matrix, row, col, value);
                }
                for (node, value) in instance.take_deferred_rhs() {
                    add_rhs_if_present(rhs, node, value);
                }
            }
        }
    }

    /// Add the static-only residual retained by the last accepted XSPICE
    /// evaluation to a probe RHS.  Applying queued stamps directly to the
    /// residual avoids rebuilding matrix topology for every accepted step.
    pub(crate) fn stamp_xspice_static_residual(&mut self, solution: &[Value], rhs: &mut [Value]) {
        for slot in &mut self.xspice_instances {
            if !slot.has_static_deferred_contributions() {
                continue;
            }
            let instance = slot.make_mut();
            for (row, col, value) in instance.take_static_deferred_stamps() {
                if let (Some(rhs_value), Some(solution_value)) =
                    (rhs.get_mut(row), solution.get(col))
                {
                    // The probe residual is A*x-b.  Subtracting the static
                    // contribution from b therefore adds it to the residual.
                    *rhs_value -= value * solution_value;
                }
            }
            for (node, value) in instance.take_static_deferred_rhs() {
                if let Some(entry) = rhs.get_mut(node) {
                    *entry += value;
                }
            }
        }
    }

    /// Accept current timestep for all XSPICE instances
    ///
    /// Called after a successful timestep to commit state changes.
    pub fn accept_xspice_timestep(&mut self) {
        for instance in &mut self.xspice_instances {
            // A gate the settle pass skipped carries the time and state its
            // last evaluation left, which the accept after that evaluation
            // already advanced; re-advancing writes the same bytes back and is
            // not worth a copy.
            if instance.accept_timestep_is_noop() {
                continue;
            }
            instance.make_mut().accept_timestep();
        }
    }

    /// Project evaluated ideal XSPICE voltage outputs into the candidate
    /// solution vector after event-driven models settle. The returned writes
    /// restore the original candidate when replayed in reverse order.
    pub(crate) fn project_xspice_voltage_outputs(
        &self,
        solution: &mut [Value],
        num_nodes: usize,
    ) -> Vec<(usize, Value)> {
        #[inline]
        fn set_node(
            solution: &mut [Value],
            node: usize,
            value: Value,
            rollback: &mut Vec<(usize, Value)>,
        ) {
            if node > 0
                && let Some(slot) = solution.get_mut(node - 1)
                && *slot != value
            {
                rollback.push((node - 1, *slot));
                *slot = value;
            }
        }

        #[inline]
        fn node_value(solution: &[Value], node: usize) -> Option<Value> {
            if node == 0 {
                Some(0.0)
            } else {
                solution.get(node - 1).copied()
            }
        }

        #[inline]
        fn project_voltage_pair(
            solution: &mut [Value],
            pos: usize,
            neg: usize,
            value: Value,
            rollback: &mut Vec<(usize, Value)>,
        ) {
            if !value.is_finite() {
                return;
            }
            match (pos, neg) {
                (0, 0) => {}
                (node, 0) => set_node(solution, node, value, rollback),
                (0, node) => set_node(solution, node, -value, rollback),
                (pos, neg) => {
                    let Some(pos_value) = node_value(solution, pos) else {
                        return;
                    };
                    let Some(neg_value) = node_value(solution, neg) else {
                        return;
                    };
                    let correction = value - (pos_value - neg_value);
                    set_node(solution, pos, pos_value + 0.5 * correction, rollback);
                    set_node(solution, neg, neg_value - 0.5 * correction, rollback);
                }
            }
        }

        let mut rollback = Vec::new();
        if num_nodes == 0 || solution.is_empty() {
            return rollback;
        }

        for instance in &self.xspice_instances {
            let ports = instance.ports();
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                let Some(port) = ports.get(port_idx) else {
                    continue;
                };
                if !matches!(
                    port.default_type,
                    crate::xspice::PortType::Voltage
                        | crate::xspice::PortType::DifferentialVoltage
                        | crate::xspice::PortType::Hybrid
                        | crate::xspice::PortType::DifferentialHybrid
                ) {
                    continue;
                }

                if port.is_vector {
                    if !instance.has_analog_vector_contributions(port_idx) {
                        continue;
                    }
                    match connection {
                        crate::xspice::PortConnection::AnalogVector(nodes) => {
                            for (index, node) in nodes.iter().copied().enumerate() {
                                if instance
                                    .branch_vector_output_ordinal(port_idx, index)
                                    .is_none()
                                {
                                    continue;
                                }
                                let (_, value) =
                                    instance.analog_vector_contribution_at(port_idx, index);
                                project_voltage_pair(solution, node, 0, value, &mut rollback);
                            }
                        }
                        crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                            for (index, element) in elements.iter().enumerate() {
                                if instance
                                    .branch_vector_output_ordinal(port_idx, index)
                                    .is_none()
                                {
                                    continue;
                                }
                                let (_, value) =
                                    instance.analog_vector_contribution_at(port_idx, index);
                                match element {
                                    crate::xspice::AnalogInputConnection::Node(node) => {
                                        project_voltage_pair(
                                            solution,
                                            *node,
                                            0,
                                            value,
                                            &mut rollback,
                                        );
                                    }
                                    crate::xspice::AnalogInputConnection::Differential(
                                        pos,
                                        neg,
                                    )
                                    | crate::xspice::AnalogInputConnection::Hybrid {
                                        pos,
                                        neg,
                                        ..
                                    } => {
                                        project_voltage_pair(
                                            solution,
                                            *pos,
                                            *neg,
                                            value,
                                            &mut rollback,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                if instance.branch_ordinal_at(port_idx).is_none() {
                    continue;
                }
                let Some((_, value)) = instance.get_analog_contribution(port_idx) else {
                    continue;
                };
                match connection {
                    crate::xspice::PortConnection::Analog(node) => {
                        project_voltage_pair(solution, *node, 0, value, &mut rollback);
                    }
                    crate::xspice::PortConnection::Differential(pos, neg)
                    | crate::xspice::PortConnection::Hybrid { pos, neg, .. } => {
                        project_voltage_pair(solution, *pos, *neg, value, &mut rollback);
                    }
                    _ => {}
                }
            }
        }
        rollback
    }

    /// Prepare build-time generated Verilog-A devices for a transient timepoint.
    ///
    /// Refuses an interval the `ddt` companion rule cannot represent. Silently
    /// dropping the reactive term instead would leave every generated device
    /// stamping only its static branch at that step, which is a different
    /// circuit — and one whose native equivalents (a capacitor's `C/dt`
    /// companion) remain exact at the same interval.
    #[cfg(feature = "veriloga-builtins-base")]
    pub fn prepare_generated_veriloga_timepoint(
        &mut self,
        time: Value,
        dt: Value,
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        dynamic_residual_scale: Value,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        let ddt_coefficients =
            crate::device::veriloga_builtins::GeneratedDdtCoefficients::from_companion_values_with_derivative_scale(
                    coefficients.coeff_g,
                    coefficients.coeff_v_n,
                    coefficients.coeff_v_n_minus_1,
                    coefficients.needs_two_history,
                    coefficients.coeff_i_n,
                    dt,
                )
            .map_err(|error| {
                format!("Verilog-A generated devices cannot advance to t={time:.16e}s: {error}")
            })?
            .scaled(dynamic_residual_scale);
        self.generated_veriloga_devices
            .set_timepoint(time, dt, ddt_coefficients);
        self.generated_veriloga_devices
            .set_analysis_step(initial_step, final_step);
        Ok(())
    }

    /// Re-evaluate generated Verilog-A devices at an exact accepted solution
    /// without disturbing the live Newton matrix. Static-probe evaluation
    /// recomputes DDT/IDT candidates, event-controlled variables, and terminal-
    /// current caches at that reported bias point while leaving the matrix and
    /// right-hand side used by Newton untouched.
    #[cfg(feature = "veriloga-builtins-base")]
    pub(crate) fn evaluate_generated_veriloga_timepoint(
        &mut self,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Result<(), String> {
        let num_nodes = self.num_nodes;
        let simparams = self.generated_simulation_parameters;
        matrix.with_probe_values(|probe, rhs| {
            self.generated_veriloga_devices
                .stamp_all_with_task_recording(
                    probe,
                    rhs,
                    solution,
                    num_nodes,
                    crate::device::veriloga_builtins::GeneratedEvaluationRequest {
                        analysis: crate::device::veriloga_builtins::GeneratedAnalysisKind::Tran,
                        simparams,
                        evaluation_mode:
                            crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
                    },
                    true,
                )
                .map_err(|error| error.to_string())
        })
    }

    #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
    fn veriloga_dc_terminal_identity(&self, node: NodeId) -> Result<String, String> {
        if node == 0 {
            return Ok("0".to_string());
        }
        let mut aliases = self
            .node_map
            .iter()
            .filter(|&(_name, &candidate)| candidate == node)
            .map(|(name, &_candidate)| name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        aliases.sort_unstable();
        aliases.dedup();
        if aliases.is_empty() {
            return Err(format!(
                "Verilog-A DC state references unknown circuit node ID {node}"
            ));
        }
        Ok(aliases.join("\u{1f}"))
    }

    #[cfg(feature = "veriloga")]
    fn runtime_veriloga_dc_state_key(
        &self,
        device: &crate::device::veriloga::VerilogADevice,
    ) -> Result<RuntimeVerilogADcStateKey, String> {
        let terminals = (0..device.num_terminals())
            .map(|terminal| self.veriloga_dc_terminal_identity(device.node_for_terminal(terminal)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RuntimeVerilogADcStateKey {
            instance_name: device.name.to_ascii_lowercase(),
            model_name: device.model_name().to_string(),
            source_digest: device.source_digest().to_string(),
            terminals,
        })
    }

    #[cfg(feature = "veriloga-builtins-base")]
    fn generated_veriloga_dc_state_key(
        &self,
        device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
        checkpoint: &crate::device::veriloga_builtins::GeneratedVerilogAInstanceCheckpoint,
    ) -> Result<GeneratedVerilogADcStateKey, String> {
        let descriptor = crate::device::veriloga_builtins::generated_veriloga_model_descriptor(
            checkpoint.model_name.as_str(),
        )
        .ok_or_else(|| {
            format!(
                "generated Verilog-A instance '{}' references unavailable model '{}'",
                checkpoint.instance_name, checkpoint.model_name
            )
        })?;
        let terminals = device
            .nodes
            .get(..descriptor.terminals.len())
            .ok_or_else(|| {
                format!(
                    "generated Verilog-A instance '{}' ({}) has {} node mappings for {} external terminals",
                    checkpoint.instance_name,
                    checkpoint.model_name,
                    device.nodes.len(),
                    descriptor.terminals.len()
                )
            })?
            .iter()
            .map(|&node| self.veriloga_dc_terminal_identity(node))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(GeneratedVerilogADcStateKey {
            instance_name: checkpoint.instance_name.to_ascii_lowercase(),
            model_name: checkpoint.model_name.clone(),
            model_identity: checkpoint.model_identity.clone(),
            terminals,
        })
    }

    /// Capture accepted Verilog-A history after a public DC sweep point.
    /// Speculative DDT/IDT or event candidates make checkpoint capture fail,
    /// so this carrier can never accidentally retain a rejected Newton trial.
    pub(crate) fn capture_veriloga_dc_accepted_state(
        &self,
    ) -> Result<VerilogADcAcceptedStateCarrier, String> {
        let mut carrier = VerilogADcAcceptedStateCarrier::default();
        #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
        let _ = &mut carrier;

        #[cfg(feature = "veriloga")]
        {
            let checkpoints = self.veriloga_devices.checkpoint_states()?;
            if checkpoints.len() != self.veriloga_devices.len() {
                return Err("runtime Verilog-A DC state capture shape mismatch".to_string());
            }
            carrier.runtime = self
                .veriloga_devices
                .iter()
                .zip(checkpoints)
                .map(|(device, checkpoint)| {
                    Ok(RuntimeVerilogADcAcceptedState {
                        key: self.runtime_veriloga_dc_state_key(device)?,
                        checkpoint,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            carrier
                .runtime
                .sort_by(|left, right| left.key.cmp(&right.key));
            if carrier
                .runtime
                .windows(2)
                .any(|pair| pair[0].key == pair[1].key)
            {
                return Err(
                    "runtime Verilog-A DC state has duplicate case-insensitive instance identity"
                        .to_string(),
                );
            }
        }

        #[cfg(feature = "veriloga-builtins-base")]
        {
            let checkpoints = self
                .generated_veriloga_devices
                .accepted_checkpoint_states()?;
            if checkpoints.len() != self.generated_veriloga_devices.len() {
                return Err("generated Verilog-A DC state capture shape mismatch".to_string());
            }
            carrier.generated = self
                .generated_veriloga_devices
                .iter()
                .zip(checkpoints)
                .map(|(device, checkpoint)| {
                    Ok(GeneratedVerilogADcAcceptedState {
                        key: self.generated_veriloga_dc_state_key(device, &checkpoint)?,
                        checkpoint,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            carrier
                .generated
                .sort_by(|left, right| left.key.cmp(&right.key));
            if carrier
                .generated
                .windows(2)
                .any(|pair| pair[0].key == pair[1].key)
            {
                return Err(
                    "generated Verilog-A DC state has duplicate case-insensitive instance identity"
                        .to_string(),
                );
            }
        }

        Ok(carrier)
    }

    /// Restore accepted Verilog-A history into a freshly rebuilt DC circuit.
    ///
    /// Matching is independent of parameter values and raw node numbering,
    /// but exact over backend, case-folded instance name, model provenance,
    /// and semantic external-terminal topology.  The target backend validates
    /// every persistent-state shape before the first state is installed.
    pub(crate) fn restore_veriloga_dc_accepted_state(
        &mut self,
        _carrier: &VerilogADcAcceptedStateCarrier,
    ) -> Result<(), String> {
        #[cfg(feature = "veriloga")]
        let runtime = {
            let mut captured = _carrier
                .runtime
                .iter()
                .map(|state| (&state.key, &state.checkpoint))
                .collect::<BTreeMap<_, _>>();
            if captured.len() != _carrier.runtime.len() {
                return Err(
                    "runtime Verilog-A DC state carrier contains duplicate instance identity"
                        .to_string(),
                );
            }
            let mut normalized = Vec::with_capacity(self.veriloga_devices.len());
            for device in self.veriloga_devices.iter() {
                let key = self.runtime_veriloga_dc_state_key(device)?;
                let source = captured.remove(&key).ok_or_else(|| {
                    format!(
                        "runtime Verilog-A DC state has no matching accepted instance '{}'",
                        device.name
                    )
                })?;
                let target = device
                    .prepare_analysis_continuation(source)
                    .map_err(|error| error.to_string())?;
                normalized.push(target);
            }
            if let Some((key, _)) = captured.first_key_value() {
                return Err(format!(
                    "runtime Verilog-A DC state contains unmatched accepted instance '{}'",
                    key.instance_name
                ));
            }
            normalized
        };

        #[cfg(feature = "veriloga-builtins-base")]
        let generated = {
            let target_templates = self.generated_veriloga_devices.checkpoint_states();
            let mut captured = _carrier
                .generated
                .iter()
                .map(|state| (&state.key, &state.checkpoint))
                .collect::<BTreeMap<_, _>>();
            if captured.len() != _carrier.generated.len() {
                return Err(
                    "generated Verilog-A DC state carrier contains duplicate instance identity"
                        .to_string(),
                );
            }
            let mut normalized = Vec::with_capacity(target_templates.len());
            for (device, mut target) in self
                .generated_veriloga_devices
                .iter()
                .zip(target_templates.into_iter())
            {
                let key = self.generated_veriloga_dc_state_key(device, &target)?;
                let source = captured.remove(&key).ok_or_else(|| {
                    format!(
                        "generated Verilog-A DC state has no matching accepted instance '{}'",
                        target.instance_name
                    )
                })?;
                target.state.clone_from(&source.state);
                target
                    .terminal_currents
                    .clone_from(&source.terminal_currents);
                normalized.push(target);
            }
            if let Some((key, _)) = captured.first_key_value() {
                return Err(format!(
                    "generated Verilog-A DC state contains unmatched accepted instance '{}'",
                    key.instance_name
                ));
            }
            self.generated_veriloga_devices
                .validate_checkpoint_states(&normalized)?;
            normalized
        };

        // Generated context validation can still fail. Its collection restores
        // its own rollback image on error; install the prepared runtime devices
        // only after the generated restore succeeds. Runtime static activation
        // was refreshed on the staged devices and cannot fail during install.
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .restore_analysis_continuation_states(
                &generated,
                self.generated_simulation_parameters,
                self.num_nodes,
            )?;
        #[cfg(feature = "veriloga")]
        self.veriloga_devices
            .install_prepared_analysis_continuation(runtime);
        Ok(())
    }

    pub(crate) fn generated_veriloga_checkpoint_states(
        &self,
    ) -> Result<Vec<crate::device::veriloga_builtins::GeneratedVerilogAInstanceCheckpoint>, String>
    {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices.accepted_checkpoint_states()
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            Ok(Vec::new())
        }
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn runtime_veriloga_checkpoint_states(
        &self,
    ) -> Result<Vec<crate::device::veriloga::VerilogADeviceCheckpoint>, String> {
        self.veriloga_devices.checkpoint_states()
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn validate_runtime_veriloga_checkpoint_states(
        &self,
        states: &[crate::device::veriloga::VerilogADeviceCheckpoint],
        state_available: bool,
    ) -> Result<(), String> {
        if !state_available {
            if !states.is_empty() {
                return Err(
                    "runtime Verilog-A checkpoint state is present without availability provenance"
                        .into(),
                );
            }
            if !self.veriloga_devices.is_empty() {
                return Err(
                    "legacy transient checkpoint does not contain runtime-compiled Verilog-A accepted state; re-run the transient from t=0"
                        .into(),
                );
            }
            return Ok(());
        }
        self.veriloga_devices.validate_checkpoint_states(states)
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn restore_runtime_veriloga_checkpoint_states(
        &mut self,
        states: &[crate::device::veriloga::VerilogADeviceCheckpoint],
        state_available: bool,
    ) -> Result<(), String> {
        self.validate_runtime_veriloga_checkpoint_states(states, state_available)?;
        self.veriloga_devices.restore_checkpoint_states(states)
    }

    pub(crate) fn restore_generated_veriloga_checkpoint_states(
        &mut self,
        states: &[crate::device::veriloga_builtins::GeneratedVerilogAInstanceCheckpoint],
        state_available: bool,
    ) -> Result<(), String> {
        self.validate_generated_veriloga_checkpoint_states(states, state_available)?;

        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices
                .restore_checkpoint_states(states)
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            debug_assert!(states.is_empty());
            Ok(())
        }
    }

    pub(crate) fn validate_generated_veriloga_checkpoint_states(
        &self,
        states: &[crate::device::veriloga_builtins::GeneratedVerilogAInstanceCheckpoint],
        state_available: bool,
    ) -> Result<(), String> {
        if !state_available {
            if !states.is_empty() {
                return Err(
                    "generated Verilog-A checkpoint state is present without availability provenance"
                        .to_string(),
                );
            }
            #[cfg(feature = "veriloga-builtins-base")]
            if !self.generated_veriloga_devices.is_empty() {
                return Err(
                    "legacy transient checkpoint does not contain generated Verilog-A persistent state"
                        .to_string(),
                );
            }
            return Ok(());
        }

        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices
                .validate_checkpoint_states(states)
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            if states.is_empty() {
                Ok(())
            } else {
                Err("this build cannot restore generated Verilog-A checkpoint state".to_string())
            }
        }
    }

    pub(crate) fn begin_veriloga_analysis(&mut self, analysis: u8) -> Result<(), String> {
        self.begin_veriloga_analysis_in_phase(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
        )
    }

    /// Start a fresh analysis across runtime, generated, and mixed models. Publish
    /// the staged collections only after every instance initializes, keeping
    /// the previous trajectory intact if any initializer fails.
    pub(crate) fn begin_veriloga_analysis_in_phase(
        &mut self,
        analysis: u8,
        phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
    ) -> Result<(), String> {
        let _ = phase;
        if analysis > 4 {
            return Err(format!(
                "Verilog-A analysis must be 0=dc, 1=ac, 2=tran, 3=noise, or 4=ic, got {analysis}"
            ));
        }
        #[cfg(feature = "veriloga")]
        let mut runtime = self.veriloga_devices.clone();
        #[cfg(feature = "veriloga")]
        for device in runtime.iter_mut() {
            let instance = device.name.clone();
            device
                .try_begin_analysis_in_phase(analysis, phase)
                .map_err(|error| {
                    format!("Verilog-A device '{instance}' analysis begin failed: {error}")
                })?;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        let generated = {
            use crate::device::veriloga_builtins::GeneratedAnalysisKind;
            let kind = match analysis {
                0 => GeneratedAnalysisKind::Dc,
                1 => GeneratedAnalysisKind::Ac,
                2 => GeneratedAnalysisKind::Tran,
                3 => GeneratedAnalysisKind::Noise,
                4 => GeneratedAnalysisKind::Ic,
                _ => unreachable!("validated Verilog-A analysis"),
            };
            let mut generated = self.generated_veriloga_devices.clone();
            generated.set_analysis_phase(phase);
            generated
                .begin_analysis(kind, self.generated_simulation_parameters, self.num_nodes)
                .map_err(|error| format!("generated Verilog-A analysis begin failed: {error}"))?;
            generated
        };
        #[cfg(feature = "veriloga")]
        {
            let mut mixed = self.mixed_signal_hosts.clone();
            for host in &mut mixed {
                host.begin_analog_analysis_in_phase(analysis, phase)
                    .map_err(|error| {
                        format!(
                            "mixed Verilog-AMS instance '{}' analysis begin failed: {error}",
                            host.instance_name()
                        )
                    })?;
            }
            self.veriloga_devices = runtime;
            self.mixed_signal_hosts = mixed;
            self.mixed_digital_coordinator = self
                .mixed_digital_coordinator
                .as_ref()
                .map(|digital| digital.fresh());
        }
        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices = generated;
        }
        Ok(())
    }

    /// Begin a fresh DC analysis for every Verilog-A instance.
    pub(crate) fn begin_veriloga_dc_analysis(&mut self) -> Result<(), String> {
        self.begin_veriloga_equilibrium_analysis(0)
    }

    /// Change a solver phase without starting a new analysis or resetting state.
    pub(crate) fn set_veriloga_analysis_phase(
        &mut self,
        phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
    ) -> Result<(), String> {
        let _ = phase;
        #[cfg(feature = "veriloga")]
        {
            // A guard refresh can fail. Publish the phase for all instances
            // together so the caller can recover without a partly switched
            // circuit, including mixed hosts' solver-input copies.
            let mut devices = self.veriloga_devices.clone();
            let mut hosts = self.mixed_signal_hosts.clone();
            for device in devices.iter_mut() {
                device.try_set_analysis_phase(phase).map_err(|error| {
                    format!(
                        "Verilog-A device '{}' analysis phase setup failed: {error}",
                        device.name
                    )
                })?;
            }
            for host in &mut hosts {
                host.set_analog_analysis_phase(phase).map_err(|error| {
                    format!(
                        "mixed Verilog-AMS instance '{}' analysis phase setup failed: {error}",
                        host.instance_name()
                    )
                })?;
            }
            self.veriloga_devices = devices;
            self.mixed_signal_hosts = hosts;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices.set_analysis_phase(phase);
        Ok(())
    }

    /// Begin an equilibrium analysis whose equations are solved by the DC
    /// operating-point machinery but whose physical Verilog-A analysis is
    /// DC, AC, noise, or forced initial-condition evaluation.
    pub(crate) fn begin_veriloga_equilibrium_analysis(
        &mut self,
        analysis: u8,
    ) -> Result<(), String> {
        if !matches!(analysis, 0 | 1 | 3 | 4) {
            return Err(format!(
                "equilibrium Verilog-A analysis must be 0=dc, 1=ac, 3=noise, or 4=ic, got {analysis}"
            ));
        }
        self.begin_veriloga_analysis_in_phase(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Equilibrium,
        )?;
        self.set_veriloga_equilibrium_analysis_override(analysis);
        Ok(())
    }

    /// Configure a rebuilt analysis point without running declaration or
    /// analog initial programs. Accepted state is installed immediately after.
    pub(crate) fn prepare_veriloga_analysis_continuation(
        &mut self,
        analysis: u8,
    ) -> Result<(), String> {
        self.prepare_veriloga_equilibrium_analysis_point(analysis, false, false)?;
        self.set_veriloga_equilibrium_analysis_override(analysis);
        Ok(())
    }

    fn set_veriloga_equilibrium_analysis_override(&mut self, analysis: u8) {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let generated_analysis = match analysis {
                0 => crate::device::veriloga_builtins::GeneratedAnalysisKind::Dc,
                1 => crate::device::veriloga_builtins::GeneratedAnalysisKind::Ac,
                3 => crate::device::veriloga_builtins::GeneratedAnalysisKind::Noise,
                4 => crate::device::veriloga_builtins::GeneratedAnalysisKind::Ic,
                _ => unreachable!("validated equilibrium analysis"),
            };
            self.generated_veriloga_devices
                .set_operating_point_analysis_override(Some(generated_analysis));
        }
        let _ = analysis;
    }

    /// Prepare one public DC operating point. DC sweeps keep time fixed at
    /// zero, use inactive integration, and expose lifecycle flags only on
    /// public points (never on solver-owned continuation points).
    pub(crate) fn prepare_veriloga_dc_analysis_point(
        &mut self,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        self.prepare_veriloga_equilibrium_analysis_point(0, initial_step, final_step)
    }

    /// Prepare one point of a DC-assembled equilibrium solve while preserving
    /// the physical DC/AC/noise identity visible to Verilog-A predicates.
    pub(crate) fn prepare_veriloga_equilibrium_analysis_point(
        &mut self,
        analysis: u8,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        self.prepare_veriloga_analysis_point(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Equilibrium,
            initial_step,
            final_step,
        )
    }

    fn prepare_veriloga_analysis_point(
        &mut self,
        analysis: u8,
        phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        if !matches!(analysis, 0 | 1 | 3 | 4) {
            return Err(format!(
                "equilibrium Verilog-A analysis must be 0=dc, 1=ac, 3=noise, or 4=ic, got {analysis}"
            ));
        }
        // Both arguments remain part of the lifecycle contract even in a
        // feature-minimal build where neither Verilog-A backend is compiled.
        let _ = (phase, initial_step, final_step);
        #[cfg(feature = "veriloga")]
        for device in self.veriloga_devices.iter_mut() {
            let instance = device.name.clone();
            device.try_set_analysis_type(analysis).map_err(|error| {
                format!("Verilog-A device '{instance}' equilibrium analysis setup failed: {error}")
            })?;
            device.try_set_analysis_phase(phase).map_err(|error| {
                format!("Verilog-A device '{instance}' analysis phase setup failed: {error}")
            })?;
            device.try_set_time(0.0).map_err(|error| {
                format!("Verilog-A device '{instance}' equilibrium time setup failed: {error}")
            })?;
            device.try_set_timestep(0.0).map_err(|error| {
                format!("Verilog-A device '{instance}' equilibrium timestep setup failed: {error}")
            })?;
            device
                .try_set_analysis_step(initial_step, final_step)
                .map_err(|error| {
                    format!(
                        "Verilog-A device '{instance}' equilibrium analysis-step setup failed: {error}"
                    )
                })?;
        }

        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices.set_analysis_phase(phase);
            self.generated_veriloga_devices.set_timepoint(
                0.0,
                0.0,
                crate::device::veriloga_builtins::GeneratedDdtCoefficients::inactive(),
            );
            self.generated_veriloga_devices
                .set_analysis_step(initial_step, final_step);
        }
        Ok(())
    }

    /// Finish an AC/noise equilibrium operating point before frequency-domain
    /// work begins. The accepted state remains committed, while lifecycle
    /// flags and the generated-model DC-stamp analysis override are cleared.
    pub(crate) fn finish_veriloga_equilibrium_operating_point(
        &mut self,
        analysis: u8,
    ) -> Result<(), String> {
        self.prepare_veriloga_analysis_point(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
            false,
            false,
        )?;
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .set_operating_point_analysis_override(None);
        Ok(())
    }

    /// Prepare one frequency-domain AC/noise point. The operating-point
    /// initial state is already accepted before a sweep reaches this surface,
    /// so frequency points can expose only the final boundary.
    pub(crate) fn prepare_veriloga_frequency_analysis_point(
        &mut self,
        analysis: u8,
        final_step: bool,
    ) -> Result<(), String> {
        if !matches!(analysis, 1 | 3) {
            return Err(format!(
                "frequency-domain Verilog-A analysis must be 1=ac or 3=noise, got {analysis}"
            ));
        }
        self.prepare_veriloga_analysis_point(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
            false,
            final_step,
        )
    }

    /// Atomically accept the exact public analysis point across both
    /// runtime-compiled and build-time generated Verilog-A instances.
    /// Validation completes for every instance before the first mutation.
    pub(crate) fn accept_veriloga_analysis_point(&mut self) -> Result<(), String> {
        self.validate_nonmixed_model_acceptance()?;

        #[cfg(feature = "veriloga")]
        self.veriloga_devices.apply_validated_timestep_acceptance();
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .apply_validated_state_acceptance();
        Ok(())
    }

    /// Evaluate every runtime-compiled Verilog-A device against one final
    /// solver solution without adding another matrix/RHS contribution. This
    /// refreshes ordered assignments and speculative state immediately before
    /// the timepoint is accepted.
    #[cfg(feature = "veriloga")]
    pub(crate) fn evaluate_veriloga_timepoint(&mut self, solution: &[Value]) -> Result<(), String> {
        for device in self.veriloga_devices.iter_mut() {
            let instance = device.name.clone();
            device
                .try_stamp(solution, |_, _, _| {}, |_, _| {})
                .map_err(|error| {
                    format!(
                        "Verilog-A device '{instance}' final timepoint evaluation failed: {error}"
                    )
                })?;
        }
        Ok(())
    }

    /// Prepare runtime-compiled Verilog-A devices for a transient timepoint.
    ///
    /// A zero interval is the t=0 static evaluation and carries no companion
    /// rule. Any other interval the rule cannot represent is refused rather
    /// than reduced to inactive coefficients: a module evaluated with its
    /// `ddt` contributions dropped is a different circuit, and the caller
    /// would see a converged, plausible, wrong solution instead of a failure.
    #[cfg(feature = "veriloga")]
    pub fn prepare_veriloga_timepoint(
        &mut self,
        time: Value,
        dt: Value,
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        self.prepare_veriloga_timepoint_with_policy(
            time,
            dt,
            XspiceCompanionPolicy {
                coefficients,
                xyce_one_step_order2: false,
            },
            initial_step,
            final_step,
        )
    }

    /// Prepare both companion families from the circuit's equation policy.
    #[cfg(feature = "veriloga")]
    pub(crate) fn prepare_veriloga_timepoint_with_policy(
        &mut self,
        time: Value,
        dt: Value,
        companion: XspiceCompanionPolicy<'_>,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        let integration = VerilogACompanionRules::from_policy(dt, companion).map_err(|error| {
            format!("Verilog-A devices cannot advance to t={time:.16e}s: {error}")
        })?;
        for device in self.veriloga_devices.iter_mut() {
            let instance = device.name.clone();
            device.try_set_analysis_type(2).map_err(|error| {
                format!("Verilog-A device '{instance}' transient analysis setup failed: {error}")
            })?;
            device
                .try_set_analysis_step(initial_step, final_step)
                .map_err(|error| {
                    format!(
                        "Verilog-A device '{instance}' transient analysis-step setup failed: {error}"
                    )
                })?;
            device.try_set_time(time).map_err(|error| {
                format!("Verilog-A device '{instance}' transient time setup failed: {error}")
            })?;
            device.try_set_timestep(dt).map_err(|error| {
                format!("Verilog-A device '{instance}' transient timestep setup failed: {error}")
            })?;
            device
                .try_set_integration_rules(integration.derivative, integration.state)
                .map_err(|error| {
                    format!(
                        "Verilog-A device '{instance}' transient integration setup failed: {error}"
                    )
                })?;
        }
        for host in &mut self.mixed_signal_hosts {
            host.set_analysis_step(initial_step, final_step)
                .map_err(|error| {
                    format!(
                        "mixed Verilog-AMS instance '{}' analysis-step setup failed: {error}",
                        host.instance_name()
                    )
                })?;
        }
        Ok(())
    }

    /// Atomically commit runtime-compiled and build-time generated Verilog-A
    /// state after an accepted timestep.
    ///
    /// Returns whether any device newly raised `$discontinuity` at this
    /// step (a rising edge against the previous accepted step), so the
    /// stepper can place a fine restart without a level-true region
    /// pinning tiny steps forever.
    pub(crate) fn accept_all_veriloga_timestep(&mut self) -> Result<bool, String> {
        self.validate_nonmixed_model_acceptance()?;
        let discontinuity = self.veriloga_discontinuity_rising();
        #[cfg(feature = "veriloga")]
        self.veriloga_devices.apply_validated_timestep_acceptance();
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .apply_validated_state_acceptance();
        Ok(discontinuity)
    }

    /// One acceptance entry point for generated, runtime and mixed HDL models.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn accept_model_transient_timestep(
        &mut self,
        time: Value,
        dt: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<bool, crate::SimulationError> {
        #[cfg(feature = "veriloga")]
        if self.has_mixed_signal_hosts() {
            return self.accept_mixed_and_analog_transient_timestep(
                time,
                dt,
                voltages,
                coefficients,
                initial_step,
                final_step,
            );
        }
        let _ = (time, dt, voltages, coefficients, initial_step, final_step);
        self.accept_all_veriloga_timestep()
            .map_err(crate::SimulationError::Circuit)
    }

    /// Candidate discontinuities must reach LTE control before acceptance.
    pub(crate) fn veriloga_discontinuity_rising(&self) -> bool {
        #[cfg(feature = "veriloga")]
        let discontinuity = self
            .veriloga_devices
            .iter()
            .any(rspice_veriloga::device::VerilogADevice::discontinuity_rising);
        #[cfg(not(feature = "veriloga"))]
        let discontinuity = false;
        #[cfg(feature = "veriloga-builtins-base")]
        let discontinuity = discontinuity || self.generated_veriloga_devices.discontinuity_rising();

        discontinuity
    }

    /// Tightest `$bound_step` request across Verilog-A devices at the
    /// latest evaluation (None when nothing bounds the next step)
    pub(crate) fn veriloga_timestep_bound(&self) -> Result<Option<Value>, String> {
        #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
        return Ok(None);

        #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
        {
            let mut tightest: Option<Value> = None;
            #[cfg(feature = "veriloga")]
            for device in self.veriloga_analog_devices() {
                let instance = &device.name;
                let Some(bound) = device.try_transient_bound_step().map_err(|error| {
                    format!("Verilog-A device '{instance}' timestep bound failed: {error}")
                })?
                else {
                    continue;
                };
                tightest = Some(tightest.map_or(bound, |current| current.min(bound)));
            }
            #[cfg(feature = "veriloga-builtins-base")]
            if let Some(bound) = self.generated_veriloga_devices.transient_step_bound()? {
                tightest = Some(tightest.map_or(bound, |current| current.min(bound)));
            }
            Ok(tightest)
        }
    }

    #[cfg(feature = "veriloga")]
    fn veriloga_analog_devices(
        &self,
    ) -> impl Iterator<Item = &rspice_veriloga::device::VerilogADevice> {
        self.veriloga_devices.iter().chain(
            self.mixed_signal_hosts
                .iter()
                .map(|host| host.analog_device()),
        )
    }

    /// Earliest interior `cross`/`above` root requested by the latest final
    /// Verilog-A evaluation. The transient engine must reject the current
    /// endpoint before any device or integration history is committed.
    pub(crate) fn veriloga_event_refinement_time(&self) -> Result<Option<Value>, String> {
        #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
        return Ok(None);

        #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
        {
            let mut earliest: Option<Value> = None;
            #[cfg(feature = "veriloga")]
            for device in self.veriloga_devices.iter() {
                let instance = &device.name;
                let Some(target) =
                    device
                        .try_transient_event_refinement_time()
                        .map_err(|error| {
                            format!(
                                "Verilog-A device '{instance}' event refinement failed: {error}"
                            )
                        })?
                else {
                    continue;
                };
                earliest = Some(earliest.map_or(target, |current| current.min(target)));
            }
            #[cfg(feature = "veriloga-builtins-base")]
            if let Some(target) = self
                .generated_veriloga_devices
                .transient_event_refinement_time()
            {
                earliest = Some(earliest.map_or(target, |current| current.min(target)));
            }
            Ok(earliest)
        }
    }

    /// Earliest exact absolute event requested by accepted Verilog-A state.
    /// Runtime timer, sampled-filter, and slew corners participate alongside
    /// generated-model timers so locked grids and checkpoint resume preserve
    /// the authoritative target rather than reconstructing it from a relative
    /// step bound.
    pub(crate) fn veriloga_transient_event_time(
        &self,
        accepted_time: Value,
    ) -> Result<Option<Value>, String> {
        if !accepted_time.is_finite() || accepted_time < 0.0 {
            return Err(format!(
                "Verilog-A event scheduling received invalid accepted time {accepted_time}"
            ));
        }

        #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
        let mut earliest: Option<Value> = None;
        #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
        let earliest: Option<Value> = None;
        #[cfg(feature = "veriloga")]
        for device in self.veriloga_analog_devices() {
            let instance = &device.name;
            let Some(target) = device.try_transient_event_time().map_err(|error| {
                format!("Verilog-A device '{instance}' event scheduling failed: {error}")
            })?
            else {
                continue;
            };
            if !target.is_finite() || target <= accepted_time {
                return Err(format!(
                    "Verilog-A device '{instance}' event {target} is not strictly after accepted time {accepted_time}"
                ));
            }
            earliest = Some(earliest.map_or(target, |current| current.min(target)));
        }

        #[cfg(feature = "veriloga-builtins-base")]
        if let Some(target) = self
            .generated_veriloga_devices
            .transient_timer_event_time()?
        {
            if !target.is_finite() || target <= accepted_time {
                return Err(format!(
                    "generated Verilog-A timer event {target} is not strictly after accepted time {accepted_time}"
                ));
            }
            earliest = Some(earliest.map_or(target, |current| current.min(target)));
        }

        Ok(earliest)
    }

    /// The earliest activation any Verilog-A or mixed Verilog-AMS instance has
    /// scheduled strictly after `accepted_time`, and the instance that owns it.
    ///
    /// Unlike [`Self::veriloga_transient_event_time`], which answers the
    /// stepper's question "is there an exact event target I must land on", this
    /// answers "is anything scheduled at all, and by whom". The two differ for
    /// a module whose discrete half runs free: its activations reach the solver
    /// as runtime breakpoints collected from the digital queue rather than as
    /// an analog event target, so the event-target query is empty while the
    /// queue is not. Both are folded here, and the mixed instance is named by
    /// the deck name its discrete half is registered under.
    ///
    /// The instance is itself optional: enrolled mixed instances schedule into
    /// one shared process queue, and while that queue can say which process an
    /// activation belongs to — and therefore which module — it also holds
    /// wakeups that belong to no module at all. A pure code-model event net is
    /// deliberately not folded in either: this query exists to attribute a
    /// schedule to a module, and no answer is better than a guessed one.
    ///
    /// Called once per accepted transient point, so it allocates nothing and
    /// borrows the instance name rather than rendering it.
    pub(crate) fn veriloga_scheduled_activation(
        &self,
        accepted_time: Value,
    ) -> Option<(Option<&str>, Value)> {
        #[cfg(feature = "veriloga")]
        let mut owner: Option<(Option<&str>, Value)> = None;
        #[cfg(not(feature = "veriloga"))]
        let owner: Option<(Option<&str>, Value)> = None;
        #[cfg(feature = "veriloga")]
        {
            let analog = self.veriloga_devices.iter().flat_map(|device| {
                device
                    .try_transient_event_time()
                    .ok()
                    .flatten()
                    .map(|target| (Some(device.name.as_str()), target))
            });
            let mixed = self.mixed_signal_hosts.iter().flat_map(|host| {
                let instance = host.instance_name();
                let analog_target = host
                    .analog_device()
                    .try_transient_event_time()
                    .ok()
                    .flatten();
                // A standalone host runs its own queue. An enrolled one is a
                // value view, and answers `None` here; its activations are the
                // coordinator's, folded in below.
                let digital_target = host.next_event_time().ok().flatten();
                [analog_target, digital_target]
                    .into_iter()
                    .flatten()
                    .map(move |target| (Some(instance), target))
            });
            // The shared process queue every enrolled instance schedules into.
            // It is one queue for all of them, but the kernel still knows which
            // process drew the earliest event, and the linker knows which
            // instance owns that process — so a deck with several mixed
            // modules is attributed as precisely as a deck with one. The
            // single-host fallback stands in for the wakeups that belong to no
            // process of the design.
            let shared = self
                .mixed_digital_coordinator
                .as_ref()
                .and_then(|coordinator| coordinator.next_event_time().ok().flatten())
                .map(|(instance, target)| {
                    let instance = instance.or(match self.mixed_signal_hosts.as_slice() {
                        [only] => Some(only.instance_name()),
                        _ => None,
                    });
                    (instance, target)
                });
            for (instance, target) in analog.chain(mixed).chain(shared) {
                if !target.is_finite() || target <= accepted_time {
                    continue;
                }
                if owner
                    .as_ref()
                    .is_none_or(|(_, earliest)| target < *earliest)
                {
                    owner = Some((instance, target));
                }
            }
        }
        #[cfg(not(feature = "veriloga"))]
        let _ = accepted_time;
        owner
    }

    /// Every mixed Verilog-AMS instance in the circuit, in circuit order.
    ///
    /// What a diagnostic falls back to when
    /// [`Self::veriloga_scheduled_activation`] reports a schedule it cannot
    /// attribute: the set of modules one of which must own it is a better
    /// subject than "something". It allocates, and is called only on the way
    /// to a report or a refusal.
    pub(crate) fn mixed_digital_instance_names(&self) -> Vec<&str> {
        #[cfg(feature = "veriloga")]
        let names: Vec<&str> = self
            .mixed_signal_hosts
            .iter()
            .map(|host| host.instance_name())
            .collect();
        #[cfg(not(feature = "veriloga"))]
        let names: Vec<&str> = Vec::new();
        names
    }

    /// Check if all XSPICE instances have converged
    pub fn xspice_converged(&self, tolerance: Value) -> bool {
        self.xspice_instances
            .iter()
            .all(|inst| inst.is_converged(tolerance))
    }

    /// XSPICE instance-level reasons that prevent transient checkpoint resume.
    pub(crate) fn xspice_checkpoint_resume_blockers(&self) -> Vec<String> {
        self.xspice_instances
            .iter()
            .filter_map(|instance| instance.checkpoint_resume_blocker())
            .collect()
    }

    /// Serializable XSPICE instance state for transient checkpoint files.
    pub(crate) fn xspice_checkpoint_instance_states(&self) -> Vec<XspiceInstanceCheckpoint> {
        self.xspice_instances
            .iter()
            .map(|instance| instance.checkpoint_state())
            .collect()
    }

    /// Restore XSPICE instance state from a transient checkpoint.
    pub(crate) fn restore_xspice_checkpoint_instance_states(
        &mut self,
        checkpoints: &[XspiceInstanceCheckpoint],
    ) -> Result<(), String> {
        self.validate_xspice_checkpoint_instance_states(checkpoints)?;
        for (instance, checkpoint) in self.xspice_instances.iter_mut().zip(checkpoints) {
            instance.make_mut().restore_checkpoint_state(checkpoint)?;
        }
        Ok(())
    }

    pub(crate) fn validate_xspice_checkpoint_instance_states(
        &self,
        checkpoints: &[XspiceInstanceCheckpoint],
    ) -> Result<(), String> {
        if checkpoints.is_empty() && !self.xspice_instances.is_empty() {
            let blockers = self.xspice_checkpoint_resume_blockers();
            if !blockers.is_empty() {
                return Err(format!(
                    "legacy checkpoint did not carry serialized XSPICE instance state, \
                     and the target circuit contains unsupported XSPICE state: {}",
                    blockers.join("; ")
                ));
            }

            let context_state_present = self.xspice_instances.iter().any(|instance| {
                let checkpoint = instance.checkpoint_state();
                !checkpoint.context.state.is_empty()
                    || !checkpoint.context.state_prev.is_empty()
                    || !checkpoint.context.int_state.is_empty()
            });
            if context_state_present {
                return Err(
                    "legacy checkpoint did not carry serialized XSPICE context state".to_string(),
                );
            }
            return Ok(());
        }

        if self.xspice_instances.len() != checkpoints.len() {
            return Err(format!(
                "checkpoint XSPICE shape mismatch: {} instance state(s) captured, \
                 circuit has {} instance(s)",
                checkpoints.len(),
                self.xspice_instances.len()
            ));
        }

        for (instance, checkpoint) in self.xspice_instances.iter().zip(checkpoints) {
            instance.validate_checkpoint_state(checkpoint)?;
        }
        Ok(())
    }

    /// Zero-based node-voltage entries excluded from generic transient LTE
    /// because an XSPICE model owns explicit step-history semantics there.
    pub(crate) fn xspice_transient_voltage_lte_excluded_nodes(&self) -> Vec<usize> {
        let mut nodes = Vec::new();
        for instance in &self.xspice_instances {
            nodes.extend(instance.transient_voltage_lte_excluded_nodes());
        }
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::StaticMatrix;
    use crate::xspice::{
        AnalysisType, CmContext, CmError, CmResult, CodeModel, DigitalValue, EvaluationPhase,
        EventValue, ParamSpec, PortConnection, PortDirection, PortSpec, PortType, XspiceInstance,
    };
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::{Arc, Mutex};

    #[cfg(feature = "veriloga")]
    #[test]
    fn simparam_gmin_updates_survive_nonlinear_device_rollback() {
        use crate::device::veriloga::{Compiler, VerilogADevice};
        let runtime=Compiler::default().compile_runtime("module query(p); inout p; electrical p; analog I(p)<+$simparam(\"gmin\",7.0)*V(p); endmodule",None).unwrap();
        let mut circuit = CircuitData::new();
        let node = circuit.get_or_create_node("p");
        let device = VerilogADevice::try_new_with_canonical_ir(
            "QUERY",
            runtime.model,
            &runtime.canonical_ir,
            &[node],
        )
        .unwrap();
        circuit.add_veriloga_device(device);
        let mut snapshot = Some(circuit.nonlinear_state_snapshot());
        for gmin in [0.0, 1e-9, 2e-6] {
            circuit.set_semiconductor_junction_gmin(gmin);
            if let Some(previous) = snapshot.take() {
                circuit.restore_nonlinear_state(previous);
            }
            let device = circuit.veriloga_devices.get_mut(0).unwrap();
            device.update_voltages(&[3.0]);
            assert_eq!(device.try_evaluate().unwrap(), [3.0 * gmin]);
            assert_eq!(
                device
                    .try_compute_jacobian()
                    .unwrap()
                    .first()
                    .map_or(0.0, |entry| entry.value),
                gmin
            );
        }
    }

    #[cfg(feature = "veriloga")]
    #[test]
    fn failed_analysis_phase_guard_preserves_every_runtime_instance() {
        use crate::device::veriloga::{Compiler, VerilogADevice};
        use rspice_veriloga_runtime::AnalogAnalysisPhase;
        let source = r#"module atomic_phase(p,n);
inout p,n; electrical p,n;
parameter integer fail=0;
analog if (sqrt(1-2*fail*analysis("nodeset")))
  I(p,n)<+(1+analysis("nodeset"))*V(p,n);
endmodule"#;
        let compiler = Compiler::default();
        let model = compiler.compile(source).unwrap();
        let canonical = compiler.compile_canonical_ir(source).unwrap();
        let mut circuit = CircuitData::new();
        let node = circuit.get_or_create_node("p");
        for name in ["first", "second"] {
            let mut device = VerilogADevice::try_new_with_canonical_ir(
                name,
                model.clone(),
                &canonical,
                &[node, 0],
            )
            .unwrap();
            if name == "second" {
                device.try_set_parameter("fail", 1.0).unwrap();
            }
            circuit.add_veriloga_device(device);
        }
        circuit.begin_veriloga_equilibrium_analysis(0).unwrap();
        let error = circuit
            .set_veriloga_analysis_phase(AnalogAnalysisPhase::Nodeset)
            .unwrap_err();
        assert!(error.contains("second"), "{error}");
        for device in circuit.veriloga_devices.iter_mut() {
            device.try_stamp(&[3.0], |_, _, _| {}, |_, _| {}).unwrap();
            assert_eq!(device.try_evaluate().unwrap()[0], 3.0, "{}", device.name);
        }
    }

    #[cfg(feature = "veriloga")]
    #[test]
    fn failed_analysis_initialization_preserves_every_runtime_instance() {
        use crate::device::veriloga::{Compiler, VerilogADevice};
        let source = r#"module atomic_initial(p,n);
inout p,n; electrical p,n;
parameter integer fail=0;
real scale;
analog initial if (fail && analysis("tran")) scale=sqrt(-1); else scale=2;
analog I(p,n)<+scale*V(p,n)+ddt(V(p,n));
endmodule"#;
        let compiler = Compiler::default();
        let model = compiler.compile(source).unwrap();
        let canonical = compiler.compile_canonical_ir(source).unwrap();
        let mut circuit = CircuitData::new();
        let node = circuit.get_or_create_node("p");
        for name in ["first", "second"] {
            let mut device = VerilogADevice::try_new_with_canonical_ir(
                name,
                model.clone(),
                &canonical,
                &[node, 0],
            )
            .unwrap();
            if name == "second" {
                device.try_set_parameter("fail", 1.0).unwrap();
            }
            circuit.add_veriloga_device(device);
        }
        circuit.begin_veriloga_analysis(0).unwrap();
        for device in circuit.veriloga_devices.iter_mut() {
            device.try_stamp(&[3.0], |_, _, _| {}, |_, _| {}).unwrap();
            device.try_advance_state().unwrap();
        }
        let previous = circuit.veriloga_devices.checkpoint_states().unwrap();
        let error = circuit.begin_veriloga_analysis(2).unwrap_err();
        assert!(error.contains("second"), "{error}");
        assert_eq!(
            circuit.veriloga_devices.checkpoint_states().unwrap(),
            previous
        );
        circuit
            .veriloga_devices
            .get_mut(1)
            .unwrap()
            .try_set_parameter("fail", 0.0)
            .unwrap();
        circuit.begin_veriloga_analysis(2).unwrap();
        for device in circuit.veriloga_devices.iter_mut() {
            assert_eq!(device.try_evaluate().unwrap()[0], 6.0);
        }
    }

    #[cfg(feature = "veriloga")]
    #[test]
    fn failed_continuation_static_guard_preserves_all_runtime_instances() {
        use crate::device::veriloga::{Compiler, VerilogADevice};
        let source = r#"module continued_guard(p,n);
inout p,n; electrical p,n;
real scale;
analog initial scale=2;
analog if (sqrt(302.0-$temperature)) I(p,n)<+scale*V(p,n);
endmodule"#;
        let compiler = Compiler::default();
        let model = compiler.compile(source).unwrap();
        let canonical = compiler.compile_canonical_ir(source).unwrap();
        let build = |temperatures: [f64; 2]| {
            let mut circuit = CircuitData::new();
            let node = circuit.get_or_create_node("p");
            for (name, temperature) in ["first", "second"].into_iter().zip(temperatures) {
                let mut device = VerilogADevice::try_new_with_canonical_ir(
                    name,
                    model.clone(),
                    &canonical,
                    &[node, 0],
                )
                .unwrap();
                device.try_set_temperature(temperature).unwrap();
                circuit.add_veriloga_device(device);
            }
            circuit
        };
        let mut original = build([300.0; 2]);
        original.begin_veriloga_equilibrium_analysis(0).unwrap();
        let accepted = original.capture_veriloga_dc_accepted_state().unwrap();
        let mut rebuilt = build([301.0, 303.0]);
        rebuilt.prepare_veriloga_analysis_continuation(0).unwrap();
        let values = |circuit: &CircuitData| {
            circuit
                .veriloga_devices
                .iter()
                .map(|device| {
                    device
                        .variables()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        };
        let before = values(&rebuilt);
        let error = rebuilt
            .restore_veriloga_dc_accepted_state(&accepted)
            .unwrap_err();
        assert!(error.contains("static condition"), "{error}");
        assert_eq!(
            values(&rebuilt),
            before,
            "a failing later instance must not install the earlier one"
        );
    }

    #[cfg(feature = "veriloga-model-vbic13")]
    fn generated_dc_carrier_circuit(
        instance_name: &str,
        terminal_names: &[&str],
        add_unrelated_node_first: bool,
        temperature: Value,
    ) -> CircuitData {
        let descriptor =
            crate::device::veriloga_builtins::generated_veriloga_model_descriptor("vbic13")
                .expect("VBIC 1.3 generated descriptor");
        assert_eq!(terminal_names.len(), descriptor.terminals.len());
        let mut circuit = CircuitData::new();
        if add_unrelated_node_first {
            circuit.get_or_create_node("unrelated");
        }
        let mut nodes = terminal_names
            .iter()
            .map(|name| circuit.get_or_create_node(name))
            .collect::<Vec<_>>();
        nodes.resize(descriptor.total_node_count, 0);
        let instance = crate::device::veriloga_builtins::BuiltinVerilogAInstance::standalone(
            "vbic13",
            instance_name,
            nodes,
            vec![0; descriptor.branch_count],
            temperature,
            &[],
        )
        .expect("generated VBIC carrier instance");
        circuit.add_generated_veriloga_device(instance);
        circuit
    }

    #[cfg(feature = "veriloga-model-vbic13")]
    #[test]
    fn generated_dc_carrier_uses_semantic_topology_not_raw_node_ids() {
        let source = generated_dc_carrier_circuit("QSTATE", &["c", "b", "e"], false, 300.15);
        let mut carrier = source
            .capture_veriloga_dc_accepted_state()
            .expect("capture generated accepted state");
        carrier.generated[0].checkpoint.state.event_variables[0] = 7.0;

        let mut renumbered = generated_dc_carrier_circuit("qstate", &["c", "b", "e"], true, 350.15);
        renumbered
            .restore_veriloga_dc_accepted_state(&carrier)
            .expect("case-folded instance and semantically identical terminals restore");
        let restored = renumbered
            .capture_veriloga_dc_accepted_state()
            .expect("recapture restored generated state");
        assert_eq!(
            restored.generated[0].checkpoint.state.event_variables[0],
            7.0
        );

        let mut rewired =
            generated_dc_carrier_circuit("QSTATE", &["other", "b", "e"], false, 300.15);
        let error = rewired
            .restore_veriloga_dc_accepted_state(&carrier)
            .expect_err("changed external topology must fail closed");
        assert!(error.contains("no matching accepted instance"), "{error}");
    }

    #[test]
    fn apply_xspice_events_resolves_touched_nodes_from_node_driver_maps() {
        let mut event_values = SharedXspiceEventValues::default();
        let mut queue = SharedXspiceEventQueue::new();
        let event_queue = queue.make_mut();
        let mut touched_digital_nodes = Vec::with_capacity(4);
        let mut touched_real_nodes = Vec::with_capacity(4);
        let digital_capacity = touched_digital_nodes.capacity();
        let real_capacity = touched_real_nodes.capacity();

        event_queue.schedule(
            1.0e-9,
            1,
            "out_a",
            "d_a",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        event_queue.schedule(
            1.0e-9,
            1,
            "out_b",
            "d_b",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        event_queue.schedule(1.0e-9, 2, "out_a", "r_a", 0, EventValue::Real(1.0));
        event_queue.schedule(1.0e-9, 2, "out_b", "r_b", 0, EventValue::Real(2.0));

        assert!(
            apply_xspice_events_at_or_before(
                &mut event_values,
                &mut queue,
                &mut touched_digital_nodes,
                &mut touched_real_nodes,
                1.0e-9,
            )
            .expect("a queue nothing feeds back into settles")
        );

        assert_eq!(touched_digital_nodes, vec![1]);
        assert_eq!(touched_real_nodes, vec![2]);
        assert_eq!(touched_digital_nodes.capacity(), digital_capacity);
        assert_eq!(touched_real_nodes.capacity(), real_capacity);
        assert_eq!(
            event_values.digital_drivers.get(&1).map(HashMap::len),
            Some(2)
        );
        assert_eq!(event_values.real_drivers.get(&2).map(HashMap::len), Some(2));
        assert_eq!(
            event_values.digital_values.get(&1).copied(),
            Some(DigitalValue::one())
        );
        assert_eq!(
            event_values.digital_event_times.get(&1).copied(),
            Some(1.0e-9)
        );
        assert_eq!(event_values.real_values.get(&2).copied(), Some(3.0));
        assert_eq!(event_values.real_event_times.get(&2).copied(), Some(1.0e-9));
        assert!(queue.is_empty());

        assert!(
            !apply_xspice_events_at_or_before(
                &mut event_values,
                &mut queue,
                &mut touched_digital_nodes,
                &mut touched_real_nodes,
                1.0e-9,
            )
            .expect("an empty queue settles")
        );

        assert!(touched_digital_nodes.is_empty());
        assert!(touched_real_nodes.is_empty());
        assert_eq!(touched_digital_nodes.capacity(), digital_capacity);
        assert_eq!(touched_real_nodes.capacity(), real_capacity);
    }

    #[test]
    fn apply_xspice_events_preserves_vector_driver_elements_on_same_node() {
        let mut event_values = SharedXspiceEventValues::default();
        let mut queue = SharedXspiceEventQueue::new();
        let event_queue = queue.make_mut();
        let mut touched_digital_nodes = Vec::new();
        let mut touched_real_nodes = Vec::new();

        event_queue.schedule(
            1.0e-9,
            1,
            "out",
            "vector_driver",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        event_queue.schedule(
            1.0e-9,
            1,
            "out",
            "vector_driver",
            1,
            EventValue::Digital(DigitalValue::one()),
        );
        event_queue.schedule(
            1.0e-9,
            2,
            "real_out",
            "real_vector_driver",
            0,
            EventValue::Real(1.0),
        );
        event_queue.schedule(
            1.0e-9,
            2,
            "real_out",
            "real_vector_driver",
            1,
            EventValue::Real(2.0),
        );

        assert!(
            apply_xspice_events_at_or_before(
                &mut event_values,
                &mut queue,
                &mut touched_digital_nodes,
                &mut touched_real_nodes,
                1.0e-9,
            )
            .expect("a queue nothing feeds back into settles")
        );

        assert_eq!(
            event_values.digital_drivers.get(&1).map(HashMap::len),
            Some(2)
        );
        assert_eq!(event_values.real_drivers.get(&2).map(HashMap::len), Some(2));
        assert_eq!(
            event_values.digital_values.get(&1).copied(),
            Some(DigitalValue::one())
        );
        assert_eq!(event_values.real_values.get(&2).copied(), Some(3.0));
    }

    struct OutputModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    impl OutputModel {
        fn new(port_type: PortType) -> Self {
            Self {
                ports: vec![PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::Out,
                    default_type: port_type,
                    allowed_types: vec![port_type],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: String::new(),
                }],
                params: Vec::new(),
            }
        }
    }

    impl CodeModel for OutputModel {
        fn name(&self) -> &str {
            "output_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            ctx.set_output("out", 1.0);
            Ok(())
        }
    }

    struct ControlledVoltageModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    struct VectorControlledVoltageModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    struct VectorOutputControlledVoltageModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    impl ControlledVoltageModel {
        fn new() -> Self {
            Self {
                ports: vec![
                    PortSpec::input("in", PortType::Voltage),
                    PortSpec::output("out", PortType::Voltage),
                ],
                params: Vec::new(),
            }
        }
    }

    impl VectorControlledVoltageModel {
        fn new() -> Self {
            Self {
                ports: vec![
                    PortSpec::vector_input("in", PortType::Voltage),
                    PortSpec::output("out", PortType::Voltage),
                ],
                params: Vec::new(),
            }
        }
    }

    impl VectorOutputControlledVoltageModel {
        fn new() -> Self {
            Self {
                ports: vec![
                    PortSpec::vector_input("in", PortType::Voltage),
                    PortSpec::vector_output("out", PortType::Voltage),
                ],
                params: Vec::new(),
            }
        }
    }

    impl CodeModel for ControlledVoltageModel {
        fn name(&self) -> &str {
            "controlled_voltage_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            let input = ctx.input("in");
            ctx.set_output_with_partial("out", 2.0 * input + 1.0, 2.0);
            Ok(())
        }

        fn output_input_partials(
            &self,
            ctx: &CmContext,
            output_port: &str,
        ) -> Vec<(String, Value)> {
            if output_port.eq_ignore_ascii_case("out") {
                vec![("in".to_string(), ctx.partial("out"))]
            } else {
                Vec::new()
            }
        }
    }

    impl CodeModel for VectorControlledVoltageModel {
        fn name(&self) -> &str {
            "vector_controlled_voltage_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            let input = ctx.input_vector("in").get(1).copied().unwrap_or(0.0);
            ctx.set_output_with_partial("out", 3.0 * input + 1.0, 0.0);
            Ok(())
        }

        fn output_input_vector_partials(
            &self,
            _ctx: &CmContext,
            output_port: &str,
        ) -> Vec<(String, usize, Value)> {
            if output_port.eq_ignore_ascii_case("out") {
                vec![("in".to_string(), 1, 3.0)]
            } else {
                Vec::new()
            }
        }
    }

    impl CodeModel for VectorOutputControlledVoltageModel {
        fn name(&self) -> &str {
            "vector_output_controlled_voltage_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            let inputs = ctx.input_vector("in");
            let in0 = inputs.first().copied().unwrap_or(0.0);
            let in1 = inputs.get(1).copied().unwrap_or(0.0);
            ctx.set_output_vector("out", vec![2.0 * in0 + 1.0, 3.0 * in1 + 1.0])?;
            Ok(())
        }

        fn output_vector_input_vector_partials(
            &self,
            _ctx: &CmContext,
            output_port: &str,
            output_index: usize,
        ) -> Vec<(String, usize, Value)> {
            if !output_port.eq_ignore_ascii_case("out") {
                return Vec::new();
            }
            match output_index {
                0 => vec![("in".to_string(), 0, 2.0)],
                1 => vec![("in".to_string(), 1, 3.0)],
                _ => Vec::new(),
            }
        }
    }

    struct FailingModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    struct BreakpointModel {
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    struct FeedbackInverterModel;

    struct EventOutputProbeModel {
        calls: Arc<Mutex<usize>>,
    }

    struct PhaseProbeModel {
        seen_phases: Arc<Mutex<Vec<EvaluationPhase>>>,
    }

    impl BreakpointModel {
        fn new() -> Self {
            Self {
                ports: vec![PortSpec::output("out", PortType::Voltage)],
                params: Vec::new(),
            }
        }
    }

    impl FailingModel {
        fn new() -> Self {
            Self {
                ports: vec![PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::Out,
                    default_type: PortType::Current,
                    allowed_types: vec![PortType::Current],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: String::new(),
                }],
                params: Vec::new(),
            }
        }
    }

    impl PhaseProbeModel {
        fn new(seen_phases: Arc<Mutex<Vec<EvaluationPhase>>>) -> Self {
            Self { seen_phases }
        }
    }

    impl EventOutputProbeModel {
        fn new(calls: Arc<Mutex<usize>>) -> Self {
            Self { calls }
        }
    }

    impl CodeModel for FailingModel {
        fn name(&self) -> &str {
            "failing_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Err(CmError::EvaluationError("intentional failure".to_string()))
        }
    }

    impl CodeModel for BreakpointModel {
        fn name(&self) -> &str {
            "breakpoint_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            ctx.set_output("out", 0.0);
            ctx.request_breakpoint(ctx.time + 1.0e-9);
            Ok(())
        }
    }

    impl CodeModel for FeedbackInverterModel {
        fn name(&self) -> &str {
            "feedback_inverter_model"
        }

        fn ports(&self) -> &[PortSpec] {
            use std::sync::OnceLock;
            static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
            PORTS.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Digital),
                    PortSpec::output("out", PortType::Digital),
                ]
            })
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            let value = ctx.input_digital("in").unwrap_or_default().invert();
            ctx.set_output_digital("out", value, 0.0);
            Ok(())
        }
    }

    impl CodeModel for EventOutputProbeModel {
        fn name(&self) -> &str {
            "event_output_probe_model"
        }

        fn ports(&self) -> &[PortSpec] {
            use std::sync::OnceLock;
            static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
            PORTS.get_or_init(|| vec![PortSpec::output("out", PortType::Digital)])
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            *self
                .calls
                .lock()
                .expect("event output probe lock must not be poisoned") += 1;
            ctx.set_output_digital("out", DigitalValue::one(), 0.0);
            Ok(())
        }
    }

    impl CodeModel for PhaseProbeModel {
        fn name(&self) -> &str {
            "phase_probe_model"
        }

        fn ports(&self) -> &[PortSpec] {
            use std::sync::OnceLock;
            static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
            PORTS.get_or_init(|| vec![PortSpec::output("out", PortType::Voltage)])
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            self.seen_phases
                .lock()
                .expect("phase probe lock must not be poisoned")
                .push(ctx.evaluation_phase());
            ctx.set_output("out", 0.0);
            Ok(())
        }
    }

    fn output_instance(port_type: PortType, connection: PortConnection) -> XspiceInstance {
        XspiceInstance::new(
            "Aout",
            Arc::new(OutputModel::new(port_type)),
            vec![connection],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("output instance should construct")
    }

    fn failing_instance() -> XspiceInstance {
        XspiceInstance::new(
            "Afail",
            Arc::new(FailingModel::new()),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("failing instance should construct")
    }

    fn breakpoint_instance() -> XspiceInstance {
        XspiceInstance::new(
            "Abreak",
            Arc::new(BreakpointModel::new()),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("breakpoint instance should construct")
    }

    fn feedback_inverter_instance() -> XspiceInstance {
        XspiceInstance::new(
            "Afb",
            Arc::new(FeedbackInverterModel),
            vec![PortConnection::Digital(1), PortConnection::Digital(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("feedback inverter instance should construct")
    }

    fn event_output_probe_instance(calls: Arc<Mutex<usize>>) -> XspiceInstance {
        XspiceInstance::new(
            "Aprobe",
            Arc::new(EventOutputProbeModel::new(calls)),
            vec![PortConnection::Digital(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("event output probe instance should construct")
    }

    fn phase_probe_instance(seen_phases: Arc<Mutex<Vec<EvaluationPhase>>>) -> XspiceInstance {
        XspiceInstance::new(
            "Aphase",
            Arc::new(PhaseProbeModel::new(seen_phases)),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("phase probe instance should construct")
    }

    #[test]
    fn add_xspice_instance_caches_event_driven_presence() {
        let mut circuit = CircuitData::new();
        assert!(!circuit.has_xspice_event_driven_devices());

        circuit.add_xspice_instance(output_instance(
            PortType::Voltage,
            PortConnection::Analog(1),
        ));
        assert!(!circuit.has_xspice_event_driven_devices());
        assert!(
            !circuit.has_event_driven_boundaries(),
            "an analog-only code model does not step the solution at a digital edge"
        );

        circuit.add_xspice_instance(output_instance(
            PortType::Digital,
            PortConnection::Digital(2),
        ));
        assert!(circuit.has_xspice_event_driven_devices());
        assert!(
            circuit.has_event_driven_boundaries(),
            "an event-driven code model is the XSPICE half of the recovery-guard classification"
        );
        assert_eq!(
            circuit.xspice_event_node_matrix_rows().collect::<Vec<_>>(),
            vec![1]
        );
    }

    #[test]
    fn fill_xspice_digital_snapshot_reuses_buffer_and_sorts_nodes() {
        let mut circuit = CircuitData::new();
        let event_values = circuit.xspice_event_values.make_mut();
        event_values.digital_values.insert(3, DigitalValue::one());
        event_values.digital_values.insert(1, DigitalValue::zero());

        let mut snapshot = Vec::with_capacity(8);
        let capacity = snapshot.capacity();
        circuit.fill_xspice_digital_snapshot(&mut snapshot);

        assert_eq!(
            snapshot,
            vec![(1, DigitalValue::zero()), (3, DigitalValue::one())]
        );
        assert_eq!(snapshot.capacity(), capacity);
    }

    #[test]
    fn evaluate_xspice_reports_model_errors_without_stamping_stale_outputs() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.add_xspice_instance(failing_instance());

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0], AnalysisType::Transient);
        let err = circuit
            .take_xspice_evaluation_error()
            .expect("legacy engine-facing XSPICE evaluation must record model errors");

        assert!(err.contains("Afail"));
        assert!(err.contains("intentional failure"));
    }

    #[test]
    fn evaluate_xspice_reports_nonsettling_zero_delay_event_network() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("out");
        circuit.add_xspice_instance(feedback_inverter_instance());
        assert!(circuit.has_xspice_event_driven_devices());

        let result =
            circuit.try_evaluate_xspice_with_analysis(0.0, 1e-9, &[], AnalysisType::Transient);
        assert!(
            result.is_err(),
            "expected event iteration failure, got {result:?}"
        );
        let err = circuit
            .take_xspice_evaluation_error()
            .expect("non-settling XSPICE event networks must be reported");

        assert!(err.contains("XSPICE event network did not settle"), "{err}");
        assert!(
            err.contains("delta cycles"),
            "the ceiling that tripped must be named: {err}"
        );
        assert!(
            err.contains("Afb.out fired"),
            "the diagnostic must name the driver that would not quiet: {err}"
        );
    }

    #[test]
    fn event_scheduler_does_not_refire_output_only_models_after_own_output_change() {
        let calls = Arc::new(Mutex::new(0usize));
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("out");
        circuit.add_xspice_instance(event_output_probe_instance(Arc::clone(&calls)));

        circuit
            .try_evaluate_xspice_with_analysis(0.0, 1e-9, &[], AnalysisType::Transient)
            .expect("output-only event model should settle after seeding its output");

        assert_eq!(
            *calls
                .lock()
                .expect("event output probe lock must not be poisoned"),
            1,
            "output-only event model must not be re-fired by its own output node"
        );
    }

    #[test]
    fn stamp_xspice_skips_out_of_range_current_output_without_panicking() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.add_xspice_instance(output_instance(
            PortType::Current,
            PortConnection::Analog(2),
        ));
        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0], AnalysisType::Transient);

        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];

        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.stamp_xspice(&mut matrix, &mut rhs);
        }));

        result.expect("out-of-range XSPICE output node must not panic");
        assert_eq!(rhs, vec![0.0]);
    }

    #[test]
    fn accepted_xspice_timestep_exposes_requested_breakpoints() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.add_xspice_instance(breakpoint_instance());

        circuit
            .accept_xspice_transient_timestep_with_coefficients(
                2.0e-9,
                1.0e-9,
                &[0.0],
                XspiceCompanionPolicy {
                    coefficients:
                        &crate::numerics::integration::CompanionCoefficients::backward_euler(),
                    xyce_one_step_order2: false,
                },
            )
            .unwrap();

        let breakpoints = circuit.take_xspice_requested_breakpoints();
        assert_eq!(breakpoints.len(), 1);
        assert!((breakpoints[0] - 3.0e-9).abs() < 1.0e-21);
        assert!(circuit.take_xspice_requested_breakpoints().is_empty());
    }

    #[test]
    fn transient_trial_and_acceptance_expose_xspice_evaluation_phase() {
        let seen_phases = Arc::new(Mutex::new(Vec::new()));
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.add_xspice_instance(phase_probe_instance(Arc::clone(&seen_phases)));
        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];

        circuit.stamp_xspice_transient_trial(&mut matrix, &mut rhs, 1.0e-9, 1.0e-9, &[0.0]);
        circuit
            .accept_xspice_transient_timestep_with_coefficients(
                1.0e-9,
                1.0e-9,
                &[0.0],
                XspiceCompanionPolicy {
                    coefficients:
                        &crate::numerics::integration::CompanionCoefficients::backward_euler(),
                    xyce_one_step_order2: false,
                },
            )
            .unwrap();

        assert_eq!(
            *seen_phases
                .lock()
                .expect("phase probe lock must not be poisoned"),
            vec![
                EvaluationPhase::RollbackableProbe,
                EvaluationPhase::AcceptedStep
            ],
            "code models must be able to distinguish rollbackable trial evaluation from accepted-step evaluation"
        );
    }

    #[test]
    fn stamp_xspice_skips_out_of_range_voltage_branch_without_panicking() {
        let mut instance = output_instance(PortType::Voltage, PortConnection::Analog(1));
        instance
            .set_output_branch(0, 8)
            .expect("test instance should accept branch assignment");

        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.add_xspice_instance(instance);
        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0], AnalysisType::Transient);

        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];

        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.stamp_xspice(&mut matrix, &mut rhs);
        }));

        result.expect("out-of-range XSPICE branch row must not panic");
        assert_eq!(rhs, vec![0.0]);
    }

    #[test]
    fn project_xspice_voltage_outputs_updates_accepted_solution() {
        let mut instance = output_instance(PortType::Voltage, PortConnection::Analog(1));
        let mut circuit = CircuitData::new();
        let out_node = circuit.get_or_create_node("out");
        let branch = circuit.allocate_branch_named("Aout#out");
        instance
            .set_output_branch(0, branch)
            .expect("test instance should accept branch assignment");
        circuit.add_xspice_instance(instance);

        let mut solution = vec![0.0; circuit.matrix_size()];
        circuit.evaluate_xspice_with_analysis(1.0e-9, 1.0e-9, &solution, AnalysisType::Transient);
        let _ = circuit.project_xspice_voltage_outputs(&mut solution, circuit.num_nodes);

        assert_eq!(solution[out_node - 1], 1.0);
    }

    #[test]
    fn stamp_xspice_voltage_output_linearizes_control_input_into_branch_equation() {
        let mut instance = XspiceInstance::new(
            "Actrl",
            Arc::new(ControlledVoltageModel::new()),
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("controlled voltage instance should construct");

        let mut circuit = CircuitData::new();
        let in_node = circuit.get_or_create_node("in");
        let out_node = circuit.get_or_create_node("out");
        let branch = circuit.allocate_branch_named("Actrl#out");
        instance
            .set_output_branch(1, branch)
            .expect("test instance should accept branch assignment");
        circuit.add_xspice_instance(instance);

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0, 0.0], AnalysisType::DcOp);
        let in_row = in_node - 1;
        let out_row = out_node - 1;
        let branch_row = circuit.get_branch_matrix_index(branch) - 1;
        let mut matrix = StaticMatrix::from_triplets(
            circuit.matrix_size(),
            circuit.matrix_size(),
            &[
                (in_row, in_row, 0.0),
                (out_row, branch_row, 0.0),
                (branch_row, in_row, 0.0),
                (branch_row, out_row, 0.0),
            ],
        )
        .expect("test matrix should construct");
        let mut rhs = vec![0.0; circuit.matrix_size()];
        matrix.add(in_row, in_row, 1.0);
        rhs[in_row] = 3.0;
        circuit.stamp_xspice(&mut matrix, &mut rhs);

        let solution = matrix.solve(&rhs).expect("linearized matrix solves");
        assert!(
            (solution[out_row] - 7.0).abs() < 1.0e-12,
            "controlled voltage output should solve from linearized input partial, got {:?}",
            solution
        );
    }

    #[test]
    fn stamp_xspice_explicit_current_output_ignores_voltage_output_direct_partial_conductance() {
        let mut model = ControlledVoltageModel::new();
        model.ports[1].allowed_types = vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
        ];
        let instance = XspiceInstance::new(
            "Actrl",
            Arc::new(model),
            vec![
                PortConnection::Analog(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("controlled voltage output should accept explicit current output");

        let mut circuit = CircuitData::new();
        let in_node = circuit.get_or_create_node("in");
        let out_node = circuit.get_or_create_node("out");
        circuit.add_xspice_instance(instance);

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[3.0, 0.0], AnalysisType::DcOp);
        let in_row = in_node - 1;
        let out_row = out_node - 1;
        let mut matrix = StaticMatrix::from_triplets(
            circuit.matrix_size(),
            circuit.matrix_size(),
            &[
                (in_row, in_row, 0.0),
                (out_row, out_row, 0.0),
                (out_row, in_row, 0.0),
            ],
        )
        .expect("test matrix should construct");
        let mut rhs = vec![0.0; circuit.matrix_size()];
        matrix.add(in_row, in_row, 1.0);
        rhs[in_row] = 3.0;
        matrix.add(out_row, out_row, 1.0);
        circuit.stamp_xspice(&mut matrix, &mut rhs);

        let solution = matrix.solve(&rhs).expect("linearized matrix solves");
        assert!(
            (solution[out_row] + 7.0).abs() < 1.0e-12,
            "explicit current output should stamp only controlled current, got {:?}",
            solution
        );
    }

    #[test]
    fn stamp_xspice_voltage_output_linearizes_vector_control_input_into_branch_equation() {
        let mut instance = XspiceInstance::new(
            "Avecctrl",
            Arc::new(VectorControlledVoltageModel::new()),
            vec![
                PortConnection::AnalogVector(vec![1, 2]),
                PortConnection::Analog(3),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("vector-controlled voltage instance should construct");

        let mut circuit = CircuitData::new();
        let in0_node = circuit.get_or_create_node("in0");
        let in1_node = circuit.get_or_create_node("in1");
        let out_node = circuit.get_or_create_node("out");
        let branch = circuit.allocate_branch_named("Avecctrl#out");
        instance
            .set_output_branch(1, branch)
            .expect("test instance should accept branch assignment");
        circuit.add_xspice_instance(instance);

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0, 2.0, 0.0], AnalysisType::DcOp);
        let in0_row = in0_node - 1;
        let in1_row = in1_node - 1;
        let out_row = out_node - 1;
        let branch_row = circuit.get_branch_matrix_index(branch) - 1;
        let mut matrix = StaticMatrix::from_triplets(
            circuit.matrix_size(),
            circuit.matrix_size(),
            &[
                (in0_row, in0_row, 0.0),
                (in1_row, in1_row, 0.0),
                (out_row, branch_row, 0.0),
                (branch_row, in1_row, 0.0),
                (branch_row, out_row, 0.0),
            ],
        )
        .expect("test matrix should construct");
        let mut rhs = vec![0.0; circuit.matrix_size()];
        matrix.add(in0_row, in0_row, 1.0);
        matrix.add(in1_row, in1_row, 1.0);
        rhs[in1_row] = 2.0;
        circuit.stamp_xspice(&mut matrix, &mut rhs);

        let solution = matrix.solve(&rhs).expect("linearized matrix solves");
        assert!(
            (solution[out_row] - 7.0).abs() < 1.0e-12,
            "controlled voltage output should solve from linearized vector input partial, got {:?}",
            solution
        );
    }

    #[test]
    fn stamp_xspice_vector_voltage_output_linearizes_each_output_element() {
        let mut instance = XspiceInstance::new(
            "Avecout",
            Arc::new(VectorOutputControlledVoltageModel::new()),
            vec![
                PortConnection::AnalogVector(vec![1, 2]),
                PortConnection::AnalogVector(vec![3, 4]),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("vector-output controlled voltage instance should construct");

        let mut circuit = CircuitData::new();
        let in0_node = circuit.get_or_create_node("in0");
        let in1_node = circuit.get_or_create_node("in1");
        let out0_node = circuit.get_or_create_node("out0");
        let out1_node = circuit.get_or_create_node("out1");
        let out0_branch = circuit.allocate_branch_named("Avecout#out[0]");
        let out1_branch = circuit.allocate_branch_named("Avecout#out[1]");
        instance
            .set_output_vector_branch(1, 0, out0_branch)
            .expect("test instance should accept out[0] branch assignment");
        instance
            .set_output_vector_branch(1, 1, out1_branch)
            .expect("test instance should accept out[1] branch assignment");
        circuit.add_xspice_instance(instance);

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[1.0, 2.0, 0.0, 0.0], AnalysisType::DcOp);
        let in0_row = in0_node - 1;
        let in1_row = in1_node - 1;
        let out0_row = out0_node - 1;
        let out1_row = out1_node - 1;
        let out0_branch_row = circuit.get_branch_matrix_index(out0_branch) - 1;
        let out1_branch_row = circuit.get_branch_matrix_index(out1_branch) - 1;
        let mut matrix = StaticMatrix::from_triplets(
            circuit.matrix_size(),
            circuit.matrix_size(),
            &[
                (in0_row, in0_row, 0.0),
                (in1_row, in1_row, 0.0),
                (out0_row, out0_branch_row, 0.0),
                (out1_row, out1_branch_row, 0.0),
                (out0_branch_row, in0_row, 0.0),
                (out1_branch_row, in1_row, 0.0),
                (out0_branch_row, out0_row, 0.0),
                (out1_branch_row, out1_row, 0.0),
            ],
        )
        .expect("test matrix should construct");
        let mut rhs = vec![0.0; circuit.matrix_size()];
        matrix.add(in0_row, in0_row, 1.0);
        matrix.add(in1_row, in1_row, 1.0);
        rhs[in0_row] = 1.0;
        rhs[in1_row] = 2.0;
        circuit.stamp_xspice(&mut matrix, &mut rhs);

        let solution = matrix.solve(&rhs).expect("linearized matrix solves");
        assert!(
            (solution[out0_row] - 3.0).abs() < 1.0e-12,
            "out[0] should solve from its own vector input partial, got {:?}",
            solution
        );
        assert!(
            (solution[out1_row] - 7.0).abs() < 1.0e-12,
            "out[1] should solve from its own vector input partial, got {:?}",
            solution
        );
    }

    //=========================================================================
    // Rollback exactness for the shared instance list
    //=========================================================================
    //
    // Sharing an instance with its rollback snapshot is only sound if the
    // restored deck is the captured deck. These check something stronger than
    // equal values: that every restored instance is the *same allocation* the
    // capture took. Identity implies equality across all forty-odd context
    // fields at once, and unlike a field-by-field comparison it cannot fall out
    // of date when a field is added.
    //
    // The captured snapshot stays alive until each assertion runs, so no
    // captured address can be recycled underneath one.

    /// A gate whose output is a pure function of its digital input, i.e. one
    /// the settle dispatch may skip while its input net is quiet.
    struct QuietGateModel;

    impl CodeModel for QuietGateModel {
        fn name(&self) -> &str {
            "quiet_gate_model"
        }

        fn ports(&self) -> &[PortSpec] {
            use std::sync::OnceLock;
            static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
            PORTS.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Digital),
                    PortSpec::output("out", PortType::Digital),
                ]
            })
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }
    }

    fn quiet_gate_instance(name: &str, input: NodeId, output: NodeId) -> XspiceInstance {
        XspiceInstance::new(
            name,
            Arc::new(QuietGateModel),
            vec![
                PortConnection::Digital(input),
                PortConnection::Digital(output),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("quiet gate instance should construct")
    }

    /// Event-pure gates alongside a model that moves on time rather than on
    /// input events, which is the asymmetry the guards have to survive.
    fn mixed_event_and_time_driven_circuit() -> CircuitData {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.get_or_create_node("n2");
        circuit.get_or_create_node("n3");
        circuit.add_xspice_instance(quiet_gate_instance("Ag0", 1, 2));
        circuit.add_xspice_instance(quiet_gate_instance("Ag1", 2, 3));
        // Requests a breakpoint and drives an analog output on every
        // evaluation, with no event input to go quiet.
        circuit.add_xspice_instance(breakpoint_instance());
        circuit.add_xspice_instance(quiet_gate_instance("Ag2", 3, 1));
        circuit
    }

    fn instance_addresses(circuit: &CircuitData) -> Vec<*const XspiceInstance> {
        circuit
            .xspice_instances
            .iter()
            .map(|instance| std::ptr::from_ref(&**instance))
            .collect()
    }

    /// Drive one instance the way the settle loop does, which is what moves its
    /// context time and leaves it owing an advance.
    fn evaluate_instance_at(circuit: &mut CircuitData, index: usize, time: Value) {
        circuit.xspice_instances[index]
            .make_mut()
            .evaluate(
                time,
                1.0e-9,
                AnalysisType::Transient,
                EvaluationPhase::RollbackableProbe,
            )
            .expect("the test models evaluate cleanly");
    }

    fn instance_time(circuit: &CircuitData, index: usize) -> Value {
        circuit.xspice_instances[index]
            .checkpoint_state()
            .context
            .time
    }

    #[test]
    fn a_restored_attempt_returns_the_exact_instance_allocations_it_captured() {
        let mut circuit = mixed_event_and_time_driven_circuit();
        let captured = circuit.transient_trial_state_snapshot();
        let at_capture = instance_addresses(&circuit);

        // A rejected attempt writes through some instances and not others.
        evaluate_instance_at(&mut circuit, 0, 8.0e-9);
        circuit.xspice_instances[2].make_mut().accept_timestep();

        let during_attempt = instance_addresses(&circuit);
        assert_ne!(
            during_attempt[0], at_capture[0],
            "a write must copy away from the captured allocation, or the \
             snapshot is observing the attempt and this test proves nothing"
        );
        assert_ne!(during_attempt[2], at_capture[2]);
        assert_eq!(
            during_attempt[1], at_capture[1],
            "an instance nothing wrote to must still be shared with the capture"
        );
        assert_eq!(during_attempt[3], at_capture[3]);

        circuit.restore_nonlinear_state(captured);

        assert_eq!(
            instance_addresses(&circuit),
            at_capture,
            "every instance — written during the attempt or not — must come \
             back as the allocation the capture took"
        );
        assert_eq!(
            instance_time(&circuit, 0),
            0.0,
            "the written instance must read as it did before the attempt"
        );
    }

    #[test]
    fn repeated_reject_cycles_and_an_interleaved_merit_checkpoint_do_not_drift() {
        let mut circuit = mixed_event_and_time_driven_circuit();
        let at_step_start = instance_addresses(&circuit);

        for round in 0..5usize {
            let attempt = circuit.transient_trial_state_snapshot();

            // Rotate which instances the attempt disturbs, so no single
            // instance is always the untouched one.
            let first = round % circuit.xspice_instances.len();
            let attempt_time = 1.0e-9 * (round + 1) as Value;
            evaluate_instance_at(&mut circuit, first, attempt_time);

            // A merit checkpoint nested inside the attempt, taken and rolled
            // back the way a line search does.
            let merit = circuit.nonlinear_state_snapshot();
            let second = (round + 2) % circuit.xspice_instances.len();
            evaluate_instance_at(&mut circuit, second, 9.0e-8);
            circuit.restore_nonlinear_state(merit);
            assert_eq!(
                instance_time(&circuit, second),
                if second == first { attempt_time } else { 0.0 },
                "round {round}: a rejected merit probe must restore the \
                 attempt's state, not the step's"
            );

            circuit.restore_nonlinear_state(attempt);
            assert_eq!(
                instance_addresses(&circuit),
                at_step_start,
                "round {round}: repeated attempt/reject cycles must keep \
                 landing on the same allocations, with no drift"
            );
        }
    }

    #[test]
    fn refreshing_a_reused_snapshot_recaptures_the_live_instances() {
        let mut circuit = mixed_event_and_time_driven_circuit();
        let mut reused = circuit.transient_trial_state_snapshot();

        // An accepted step moves the deck on; the reused buffer must follow it
        // rather than keep pinning the state it first captured.
        evaluate_instance_at(&mut circuit, 1, 3.0e-9);
        circuit.refresh_transient_trial_state_snapshot(&mut reused);
        let after_refresh = instance_addresses(&circuit);

        // A later attempt is rejected off that refreshed base.
        evaluate_instance_at(&mut circuit, 1, 5.0e-9);
        circuit.restore_nonlinear_state(reused);

        assert_eq!(
            instance_addresses(&circuit),
            after_refresh,
            "a refreshed snapshot must roll back to the state it was \
             refreshed from, not to the state it was first captured from"
        );
        assert_eq!(
            instance_time(&circuit, 1),
            3.0e-9,
            "the refresh is the new rollback base"
        );
    }

    #[test]
    fn a_time_driven_model_is_never_treated_as_quiet() {
        let mut circuit = mixed_event_and_time_driven_circuit();
        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];
        circuit.stamp_xspice_transient_trial(
            &mut matrix,
            &mut rhs,
            1.0e-9,
            1.0e-9,
            &[0.0, 0.0, 0.0],
        );
        circuit
            .accept_xspice_transient_timestep_with_coefficients(
                2.0e-9,
                1.0e-9,
                &[0.0, 0.0, 0.0],
                XspiceCompanionPolicy {
                    coefficients:
                        &crate::numerics::integration::CompanionCoefficients::backward_euler(),
                    xyce_one_step_order2: false,
                },
            )
            .unwrap();

        // The breakpoint sweep skips an instance whose queue is empty. A model
        // that requests one on every evaluation must not be swept past.
        let breakpoints = circuit.take_xspice_requested_breakpoints();
        assert_eq!(
            breakpoints.len(),
            1,
            "the time-driven model's breakpoint request must survive a sweep \
             that skips the quiet gates around it"
        );
        assert!((breakpoints[0] - 3.0e-9).abs() < 1.0e-21);

        // Its accept moved it, so the accept sweep must not have judged it a
        // no-op either.
        assert!(
            circuit.xspice_instances[2].accept_timestep_is_noop(),
            "having been advanced, a further advance would now be a no-op"
        );
    }

    #[test]
    fn the_accept_sweep_only_skips_an_advance_that_would_write_nothing() {
        let mut circuit = mixed_event_and_time_driven_circuit();

        // Freshly built: nothing has evaluated, so nothing is owed an advance.
        for instance in &circuit.xspice_instances {
            assert!(
                instance.accept_timestep_is_noop(),
                "a freshly built instance has nothing to advance"
            );
        }

        // Move an instance's time forward the way an evaluation does, and the
        // guard must stop claiming the advance is free.
        evaluate_instance_at(&mut circuit, 0, 7.0e-9);
        assert!(
            !circuit.xspice_instances[0].accept_timestep_is_noop(),
            "an instance whose time moved is owed an advance"
        );

        // Hold a capture across the sweep, so an instance the sweep writes to
        // has to copy away from it and an instance it skips does not. Without
        // a live snapshot every instance is unshared and the sweep would
        // write in place either way, proving nothing.
        let captured = circuit.transient_trial_state_snapshot();
        let at_capture = instance_addresses(&circuit);

        circuit.accept_xspice_timestep();
        assert_ne!(
            std::ptr::from_ref(&*circuit.xspice_instances[0]),
            at_capture[0],
            "the owed advance must actually have been performed"
        );
        assert_eq!(
            std::ptr::from_ref(&*circuit.xspice_instances[1]),
            at_capture[1],
            "an instance owed nothing must be left untouched by the sweep"
        );
        assert!(
            circuit.xspice_instances[0].accept_timestep_is_noop(),
            "and once performed, the same advance is a no-op again"
        );

        // The skipped instances are still the captured allocations, so the
        // sweep costs one copy rather than four.
        circuit.restore_nonlinear_state(captured);
        assert_eq!(instance_addresses(&circuit), at_capture);
        assert_eq!(
            instance_time(&circuit, 0),
            7.0e-9,
            "and the accept is rolled back with everything else"
        );
    }

    //=========================================================================
    // Rollback exactness for the shared event world
    //=========================================================================
    //
    // D5 clause 1 says a rejected step rolls the event world back completely,
    // and the mechanism is that the value maps and the scheduler ride inside
    // `NonlinearDeviceStateSnapshot`. Sharing them behind an `Arc` must not
    // weaken that: the restored world has to be the captured world, value for
    // value and event for event.
    //
    // These check both halves. Identity — the restored handles are the
    // captured allocations — and content against an eager deep clone taken
    // before the attempt, which is the image the copy-at-capture code
    // produced. The scheduler has no `PartialEq`, so its content oracle is its
    // `Debug` rendering, which walks every kernel field: the future tier, all
    // four slot queues, the sequence counter, `current_tick`, the started
    // flag, the per-slot delta and event counts, the activation counts, and
    // the per-driver supersede index. All of them are `BTreeMap`s, so the
    // rendering is ordered and comparing it is exact rather than incidental.

    /// Both event-world allocations the circuit currently points at.
    fn event_world_addresses(
        circuit: &CircuitData,
    ) -> (
        *const crate::xspice::XspiceEventValues,
        *const crate::xspice::XspiceEventScheduler,
    ) {
        (
            std::ptr::from_ref(&*circuit.xspice_event_values),
            std::ptr::from_ref(&*circuit.xspice_event_queue),
        )
    }

    /// The captured world's content, as an eager deep copy plus the
    /// scheduler's full structural rendering.
    fn event_world_image(circuit: &CircuitData) -> (crate::xspice::XspiceEventValues, String) {
        (
            (*circuit.xspice_event_values).clone(),
            format!("{:?}", *circuit.xspice_event_queue),
        )
    }

    /// Two chained event models that actually drive the queue: a probe that
    /// emits a digital one on every evaluation, and an inverter that answers
    /// it, so a settle pass schedules, drains, resolves and supersedes.
    fn event_active_circuit() -> CircuitData {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.get_or_create_node("n2");
        circuit.add_xspice_instance(event_output_probe_instance(Arc::new(Mutex::new(0))));
        circuit.add_xspice_instance(feedback_inverter_instance());
        circuit
    }

    /// Run the settle loop the way a transient trial does.
    fn settle_at(circuit: &mut CircuitData, time: Value) {
        circuit
            .try_evaluate_xspice_with_analysis(time, 1.0e-9, &[0.0, 0.0], AnalysisType::Transient)
            .expect("the event models settle");
    }

    #[test]
    fn a_restored_attempt_returns_the_exact_event_world_it_captured() {
        let mut circuit = event_active_circuit();

        // Settle one timepoint first, so the capture is taken mid-run rather
        // than from a virgin world: values are resolved, drivers are indexed,
        // and the scheduler's tick and sequence counter have both moved.
        settle_at(&mut circuit, 1.0e-9);

        let expected = event_world_image(&circuit);
        let captured = circuit.transient_trial_state_snapshot();
        let at_capture = event_world_addresses(&circuit);

        // The rejected attempt: a later timepoint that drains what the first
        // one queued, schedules and supersedes more, and rewrites the resolved
        // node values.
        settle_at(&mut circuit, 3.0e-9);
        circuit.xspice_event_queue.make_mut().schedule(
            9.0e-9,
            2,
            "out",
            "Aprobe",
            0,
            EventValue::Digital(DigitalValue::zero()),
        );

        assert_ne!(
            event_world_addresses(&circuit),
            at_capture,
            "the attempt must have copied away from the captured world, or \
             the snapshot is observing it and this test proves nothing"
        );
        assert_ne!(
            event_world_image(&circuit).1,
            expected.1,
            "and it must have moved the scheduler, not merely touched it"
        );

        circuit.restore_nonlinear_state(captured);

        assert_eq!(
            event_world_addresses(&circuit),
            at_capture,
            "a rejected step must come back to the allocations the capture took"
        );
        let restored = event_world_image(&circuit);
        assert_eq!(
            restored.0, expected.0,
            "every resolved value, driver entry and event time must read as it \
             did before the attempt"
        );
        assert_eq!(
            restored.1, expected.1,
            "and the scheduler must be the captured scheduler in every kernel \
             field: pending events, slot, sequence counter, open tick and \
             supersede index"
        );
    }

    #[test]
    fn repeated_event_reject_cycles_including_a_backwards_bound_do_not_drift() {
        let mut circuit = event_active_circuit();
        settle_at(&mut circuit, 1.0e-9);

        // A reused snapshot buffer, refreshed per attempt, which is what the
        // transient loop actually holds.
        let mut attempt = circuit.transient_trial_state_snapshot();

        // Bounds that step forward and then back, the way a rejected step
        // retries at a smaller timepoint. A backwards bound opens a fresh due
        // slot (D5 clause 4), so it moves scheduler state a forward one does
        // not.
        for (round, time) in [4.0e-9, 8.0e-9, 6.0e-9, 2.0e-9, 7.0e-9]
            .into_iter()
            .enumerate()
        {
            circuit.refresh_transient_trial_state_snapshot(&mut attempt);
            let expected = event_world_image(&circuit);
            let at_capture = event_world_addresses(&circuit);

            settle_at(&mut circuit, time);
            circuit.xspice_event_queue.make_mut().schedule(
                time + 5.0e-9,
                1,
                "out",
                "Aprobe",
                0,
                EventValue::Digital(DigitalValue::zero()),
            );

            circuit.restore_nonlinear_state(attempt);
            assert_eq!(
                event_world_addresses(&circuit),
                at_capture,
                "round {round}: repeated attempt/reject cycles must keep \
                 landing on the captured allocations, with no drift"
            );
            let restored = event_world_image(&circuit);
            assert_eq!(restored.0, expected.0, "round {round}: resolved values");
            assert_eq!(restored.1, expected.1, "round {round}: scheduler image");

            attempt = circuit.transient_trial_state_snapshot();
        }
    }

    #[test]
    fn a_quiet_step_leaves_the_event_world_shared_with_its_snapshot() {
        let mut circuit = mixed_event_and_time_driven_circuit();

        // Hold a capture across the step, so a write would have to copy away
        // from it. Without a live snapshot the world is unshared and every
        // write lands in place, which would prove nothing.
        let captured = circuit.transient_trial_state_snapshot();
        let at_capture = event_world_addresses(&circuit);

        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];
        circuit.stamp_xspice_transient_trial(
            &mut matrix,
            &mut rhs,
            1.0e-9,
            1.0e-9,
            &[0.0, 0.0, 0.0],
        );
        circuit
            .accept_xspice_transient_timestep_with_coefficients(
                2.0e-9,
                1.0e-9,
                &[0.0, 0.0, 0.0],
                XspiceCompanionPolicy {
                    coefficients:
                        &crate::numerics::integration::CompanionCoefficients::backward_euler(),
                    xyce_one_step_order2: false,
                },
            )
            .unwrap();

        // The pass genuinely ran: the time-driven model evaluated and queued a
        // breakpoint request. Without that, an untouched world would prove
        // only that nothing happened at all.
        assert_eq!(
            circuit.take_xspice_requested_breakpoints().len(),
            1,
            "the settle pass must actually have evaluated the instances"
        );
        assert_eq!(
            event_world_addresses(&circuit),
            at_capture,
            "a step whose gates emit nothing and whose queue holds nothing due \
             must copy neither the value maps nor the scheduler: the drain's \
             emptiness check and the scheduling sweep's pending-output check \
             are what keep it off the copy-on-write path"
        );

        drop(captured);
    }

    #[test]
    fn a_pending_but_not_yet_due_event_does_not_copy_the_event_world() {
        // A queue that is *not* empty, holding one event dated past the
        // timepoint being settled. The drain's guard is
        // `has_event_at_or_before`, not `is_empty`: a design whose next gate
        // output is still several analog steps away must not pay a copy at
        // every step in between.
        let mut circuit = mixed_event_and_time_driven_circuit();
        circuit.xspice_event_queue.make_mut().schedule(
            5.0e-7,
            2,
            "out",
            "Ag0",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        assert!(!circuit.xspice_event_queue.is_empty());

        let captured = circuit.transient_trial_state_snapshot();
        let at_capture = event_world_addresses(&circuit);

        settle_at(&mut circuit, 1.0e-9);

        assert_eq!(
            event_world_addresses(&circuit),
            at_capture,
            "an event dated beyond the settle bound is not due, so nothing may \
             be drained and nothing may be copied"
        );
        assert_eq!(
            circuit.xspice_event_queue.len(),
            1,
            "and it must still be pending for the step that reaches its time"
        );

        drop(captured);
    }
}
