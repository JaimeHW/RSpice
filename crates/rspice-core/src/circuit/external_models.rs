//! Attaching externally defined devices to a [`CircuitData`].
//!
//! The integration surface for models the core does not implement itself:
//! XSPICE code-model instances, JIT-compiled Verilog-A devices, and the
//! checked-in generated Verilog-A built-ins. Each family needs the same
//! things — instance registration, matrix and RHS contributions folded in
//! only where the device actually has entries, and timestep acceptance that
//! propagates the device's own breakpoint requests back to the integrator.

use super::*;
use crate::xspice::XspiceInstanceCheckpoint;
#[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
use std::collections::BTreeMap;
use std::collections::HashMap;

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

fn apply_xspice_events_at_or_before(
    digital_values: &mut HashMap<NodeId, crate::xspice::DigitalValue>,
    digital_drivers: &mut XspiceDigitalDrivers,
    digital_event_times: &mut HashMap<NodeId, Value>,
    real_values: &mut HashMap<NodeId, Value>,
    real_drivers: &mut XspiceRealDrivers,
    real_event_times: &mut HashMap<NodeId, Value>,
    event_queue: &mut crate::xspice::EventQueue,
    touched_digital_nodes: &mut Vec<NodeId>,
    touched_real_nodes: &mut Vec<NodeId>,
    time: Value,
) -> bool {
    let mut changed = false;
    touched_digital_nodes.clear();
    touched_real_nodes.clear();
    if !event_queue.has_event_at_or_before(time) {
        return false;
    }
    event_queue.drain_events_at(time, |event| {
        let node_id = event.node_id;
        let event_time = event.time;
        let driver_key = (event.instance, event.port_name, event.driver_index);
        match event.value {
            crate::xspice::EventValue::Digital(value) => {
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
    });
    if touched_digital_nodes.len() > 1 {
        touched_digital_nodes.sort_unstable();
        touched_digital_nodes.dedup();
    }
    if touched_real_nodes.len() > 1 {
        touched_real_nodes.sort_unstable();
        touched_real_nodes.dedup();
    }
    for &node_id in touched_digital_nodes.iter() {
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
    for &node_id in touched_real_nodes.iter() {
        let resolved = real_drivers
            .get(&node_id)
            .map(|drivers| drivers.values().copied().sum())
            .unwrap_or(0.0);
        let previous_value = real_values.insert(node_id, resolved);
        changed |= previous_value != Some(resolved);
    }
    changed
}

fn xspice_port_declares_event_type(port: &crate::xspice::PortSpec) -> bool {
    if port.allowed_types.is_empty() {
        port.default_type.is_event_driven()
    } else {
        port.allowed_types
            .iter()
            .copied()
            .any(|port_type| port_type.is_event_driven())
    }
}

fn xspice_event_output_width(connection: &crate::xspice::PortConnection) -> usize {
    match connection {
        crate::xspice::PortConnection::Digital(_)
        | crate::xspice::PortConnection::DigitalInverted(_)
        | crate::xspice::PortConnection::Real(_) => 1,
        crate::xspice::PortConnection::DigitalVector(nodes)
        | crate::xspice::PortConnection::RealVector(nodes) => nodes.len(),
        crate::xspice::PortConnection::DigitalVectorMapped(nodes) => nodes.len(),
        _ => 0,
    }
}

fn xspice_event_output_count(instances: &[crate::xspice::XspiceInstance]) -> usize {
    instances
        .iter()
        .map(|instance| {
            instance
                .ports()
                .iter()
                .zip(instance.connections())
                .filter(|(port, _)| {
                    matches!(
                        port.direction,
                        crate::xspice::PortDirection::Out | crate::xspice::PortDirection::InOut
                    ) && xspice_port_declares_event_type(port)
                })
                .map(|(_, connection)| xspice_event_output_width(connection))
                .sum::<usize>()
        })
        .sum()
}

fn extend_xspice_event_connection_nodes(
    nodes: &mut Vec<NodeId>,
    connection: &crate::xspice::PortConnection,
) {
    match connection {
        crate::xspice::PortConnection::Digital(node)
        | crate::xspice::PortConnection::DigitalInverted(node)
        | crate::xspice::PortConnection::Real(node) => nodes.push(*node),
        crate::xspice::PortConnection::DigitalVector(vector)
        | crate::xspice::PortConnection::RealVector(vector) => nodes.extend(vector),
        crate::xspice::PortConnection::DigitalVectorMapped(vector) => {
            nodes.extend(vector.iter().map(|connection| connection.node));
        }
        _ => {}
    }
}

fn xspice_event_node_summary(digital_nodes: &[NodeId], real_nodes: &[NodeId]) -> String {
    fn append_nodes(out: &mut String, label: &str, nodes: &[NodeId]) {
        if nodes.is_empty() {
            return;
        }
        if !out.is_empty() {
            out.push_str("; ");
        }
        out.push_str(label);
        out.push_str(" nodes ");
        for (index, node) in nodes.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(&node.to_string());
        }
    }

    let mut summary = String::new();
    append_nodes(&mut summary, "digital", digital_nodes);
    append_nodes(&mut summary, "real", real_nodes);
    if summary.is_empty() {
        "no event nodes changed in the last pass".to_string()
    } else {
        summary
    }
}

fn extend_xspice_event_nodes(target: &mut Vec<NodeId>, source: &[NodeId]) {
    if source.is_empty() {
        return;
    }
    target.extend_from_slice(source);
    target.sort_unstable();
    target.dedup();
}

fn xspice_connection_uses_digital_node(
    connection: &crate::xspice::PortConnection,
    changed_nodes: &[NodeId],
) -> bool {
    match connection {
        crate::xspice::PortConnection::Digital(node)
        | crate::xspice::PortConnection::DigitalInverted(node) => changed_nodes.contains(node),
        crate::xspice::PortConnection::DigitalVector(nodes) => {
            nodes.iter().any(|node| changed_nodes.contains(node))
        }
        crate::xspice::PortConnection::DigitalVectorMapped(nodes) => nodes
            .iter()
            .any(|connection| changed_nodes.contains(&connection.node)),
        _ => false,
    }
}

fn xspice_connection_uses_real_node(
    connection: &crate::xspice::PortConnection,
    changed_nodes: &[NodeId],
) -> bool {
    match connection {
        crate::xspice::PortConnection::Real(node) => changed_nodes.contains(node),
        crate::xspice::PortConnection::RealVector(nodes) => {
            nodes.iter().any(|node| changed_nodes.contains(node))
        }
        _ => false,
    }
}

fn xspice_instance_depends_on_event_nodes(
    instance: &crate::xspice::XspiceInstance,
    changed_digital_nodes: &[NodeId],
    changed_real_nodes: &[NodeId],
) -> bool {
    instance
        .ports()
        .iter()
        .zip(instance.connections())
        .any(|(port, connection)| {
            matches!(
                port.direction,
                crate::xspice::PortDirection::In | crate::xspice::PortDirection::InOut
            ) && (xspice_connection_uses_digital_node(connection, changed_digital_nodes)
                || xspice_connection_uses_real_node(connection, changed_real_nodes))
        })
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

    /// Add an XSPICE code model instance and update derived circuit metadata.
    pub(crate) fn add_xspice_instance(&mut self, instance: XspiceInstance) {
        self.xspice_has_event_driven_devices |= instance
            .ports()
            .iter()
            .any(|port| port.default_type.is_event_driven());
        instance.for_each_event_load_contribution(|node, load| {
            *self.xspice_event_loads.entry(node).or_insert(0.0) += load;
        });
        for connection in instance.connections() {
            extend_xspice_event_connection_nodes(&mut self.xspice_event_nodes, connection);
        }
        self.xspice_event_nodes.retain(|node| *node > 0);
        self.xspice_event_nodes.sort_unstable();
        self.xspice_event_nodes.dedup();
        self.xspice_instances.push(instance);
    }

    /// Check if any XSPICE instance participates in event-driven scheduling.
    #[inline]
    pub fn has_xspice_event_driven_devices(&self) -> bool {
        self.xspice_has_event_driven_devices
    }

    /// Zero-based MNA rows that also serve as XSPICE event-node identities.
    pub(crate) fn xspice_event_node_matrix_rows(&self) -> impl Iterator<Item = usize> + '_ {
        self.xspice_event_nodes.iter().map(|node| node - 1)
    }

    /// Set transient run context on all XSPICE instances.
    pub(crate) fn set_xspice_transient_context(&mut self, tstep: Value, tstop: Value) {
        let tstep = (tstep.is_finite() && tstep > 0.0).then_some(tstep);
        let tstop = (tstop.is_finite() && tstop >= 0.0).then_some(tstop);
        for instance in &mut self.xspice_instances {
            instance.set_transient_run_context(tstep, tstop);
        }
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn has_veriloga_devices(&self) -> bool {
        !self.veriloga_devices.is_empty()
    }

    #[cfg(feature = "veriloga")]
    pub fn add_veriloga_device(&mut self, device: crate::device::veriloga::VerilogADevice) {
        self.veriloga_devices.add(device);
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
        if !self.veriloga_devices.is_empty() {
            return true;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if !self.generated_veriloga_devices.is_empty() {
            return true;
        }
        false
    }

    /// Whether every Verilog-A instance can participate in Xyce OneStep's
    /// order-two F/Q split without changing its model equations.
    ///
    /// Runtime-loaded models currently expose a combined transient stamp, so
    /// they conservatively remain at order one. Generated models carry a
    /// compiler-proven capability bit that excludes `idt`, nonlinear `ddt`,
    /// and `ddt`-dependent control flow.
    pub(crate) fn veriloga_one_step_dae_split_safe(&self) -> bool {
        #[cfg(feature = "veriloga")]
        if !self.veriloga_devices.is_empty() {
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
            log::warn!("XSPICE evaluation error: {e}");
        }
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
            log::warn!("XSPICE evaluation error: {e}");
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
            crate::numerics::integration::CompanionCoefficients::backward_euler(),
            false,
        )
    }

    fn try_evaluate_xspice_with_analysis_phase_and_coefficients(
        &mut self,
        time: Value,
        timestep: Value,
        solution: &[Value],
        analysis: crate::xspice::AnalysisType,
        phase: crate::xspice::EvaluationPhase,
        companion_coefficients: crate::numerics::integration::CompanionCoefficients,
        xyce_one_step_order2: bool,
    ) -> crate::xspice::CmResult<()> {
        let current_source_values = self.current_sources.values_at_time(time);
        let max_event_passes = if self.has_xspice_event_driven_devices() {
            xspice_event_output_count(&self.xspice_instances)
                .saturating_add(1)
                .max(1)
        } else {
            1
        };
        let num_nodes = self.num_nodes;
        let event_loads = &self.xspice_event_loads;
        let mut dispatch_digital_nodes = Vec::new();
        let mut dispatch_real_nodes = Vec::new();
        let mut analog_transitions =
            HashMap::<(NodeId, NodeId), crate::xspice::AnalogTransition>::new();

        for pass in 0..max_event_passes {
            let digital_values = &mut self.xspice_digital_values;
            let digital_drivers = &mut self.xspice_digital_drivers;
            let digital_event_times = &mut self.xspice_digital_event_times;
            let real_values = &mut self.xspice_real_values;
            let real_drivers = &mut self.xspice_real_drivers;
            let real_event_times = &mut self.xspice_real_event_times;
            let event_queue = &mut self.xspice_event_queue;
            let touched_digital_nodes = &mut self.xspice_touched_digital_nodes;
            let touched_real_nodes = &mut self.xspice_touched_real_nodes;
            let mut next_dispatch_digital_nodes = Vec::new();
            let mut next_dispatch_real_nodes = Vec::new();
            let mut changed = apply_xspice_events_at_or_before(
                digital_values,
                digital_drivers,
                digital_event_times,
                real_values,
                real_drivers,
                real_event_times,
                event_queue,
                touched_digital_nodes,
                touched_real_nodes,
                time,
            );
            extend_xspice_event_nodes(&mut dispatch_digital_nodes, touched_digital_nodes);
            extend_xspice_event_nodes(&mut dispatch_real_nodes, touched_real_nodes);

            for instance in &mut self.xspice_instances {
                if pass > 0
                    && !xspice_instance_depends_on_event_nodes(
                        instance,
                        &dispatch_digital_nodes,
                        &dispatch_real_nodes,
                    )
                {
                    continue;
                }

                instance.set_transient_companion_coefficients(companion_coefficients);
                instance.set_xyce_one_step_order2(xyce_one_step_order2);
                if let Err(e) = instance.update_inputs_with_analog_transitions(
                    solution,
                    num_nodes,
                    digital_values,
                    digital_event_times,
                    event_loads,
                    real_values,
                    real_event_times,
                    &current_source_values,
                    &analog_transitions,
                ) {
                    let message = format!("{}: {}", instance.name, e);
                    if self.xspice_evaluation_error.is_none() {
                        self.xspice_evaluation_error = Some(message.clone());
                    }
                    return Err(crate::xspice::CmError::EvaluationError(message));
                }

                if let Err(e) = instance.evaluate(time, timestep, analysis, phase) {
                    let message = format!("{}: {}", instance.name, e);
                    if self.xspice_evaluation_error.is_none() {
                        self.xspice_evaluation_error = Some(message.clone());
                    }
                    return Err(crate::xspice::CmError::EvaluationError(message));
                }
                for (key, transition) in instance.analog_output_transitions() {
                    analog_transitions.insert(key, transition);
                }

                instance.schedule_events(event_queue, time);
                let instance_changed = apply_xspice_events_at_or_before(
                    digital_values,
                    digital_drivers,
                    digital_event_times,
                    real_values,
                    real_drivers,
                    real_event_times,
                    event_queue,
                    touched_digital_nodes,
                    touched_real_nodes,
                    time,
                );
                if instance_changed {
                    changed = true;
                    extend_xspice_event_nodes(
                        &mut next_dispatch_digital_nodes,
                        touched_digital_nodes,
                    );
                    extend_xspice_event_nodes(&mut next_dispatch_real_nodes, touched_real_nodes);
                }
            }

            if !changed {
                return Ok(());
            }

            if pass + 1 == max_event_passes {
                let summary_digital_nodes = if next_dispatch_digital_nodes.is_empty() {
                    dispatch_digital_nodes.as_slice()
                } else {
                    next_dispatch_digital_nodes.as_slice()
                };
                let summary_real_nodes = if next_dispatch_real_nodes.is_empty() {
                    dispatch_real_nodes.as_slice()
                } else {
                    next_dispatch_real_nodes.as_slice()
                };
                let unsettled =
                    xspice_event_node_summary(summary_digital_nodes, summary_real_nodes);
                let message = format!(
                    "XSPICE event network did not settle at time {time:e} after {max_event_passes} passes ({unsettled})"
                );
                if self.xspice_evaluation_error.is_none() {
                    self.xspice_evaluation_error = Some(message.clone());
                }
                return Err(crate::xspice::CmError::EvaluationError(message));
            }
            dispatch_digital_nodes = next_dispatch_digital_nodes;
            dispatch_real_nodes = next_dispatch_real_nodes;
        }
        Ok(())
    }

    /// Return and clear the first XSPICE evaluation error recorded during
    /// this analysis, if any.
    pub(crate) fn take_xspice_evaluation_error(&mut self) -> Option<String> {
        self.xspice_evaluation_error.take()
    }

    /// Fill a reusable snapshot of committed XSPICE digital node values.
    pub(crate) fn fill_xspice_digital_snapshot(
        &self,
        snapshot: &mut Vec<(NodeId, crate::xspice::DigitalValue)>,
    ) {
        snapshot.clear();
        snapshot.extend(
            self.xspice_digital_values
                .iter()
                .filter_map(|(&node_id, &value)| (node_id > 0).then_some((node_id, value))),
        );
        snapshot.sort_unstable_by_key(|(node_id, _)| *node_id);
    }

    /// Fill a reusable snapshot of committed XSPICE real event-node values.
    pub(crate) fn fill_xspice_real_snapshot(&self, snapshot: &mut Vec<(NodeId, Value)>) {
        snapshot.clear();
        snapshot.extend(
            self.xspice_real_values
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
            for time in instance.drain_requested_breakpoints() {
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
            &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            false,
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
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        xyce_one_step_order2: bool,
    ) {
        let snapshot = self.nonlinear_state_snapshot();
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase_and_coefficients(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::RollbackableProbe,
            *coefficients,
            xyce_one_step_order2,
        ) {
            log::warn!("XSPICE evaluation error: {e}");
        }
        self.stamp_xspice(matrix, rhs);
        self.restore_nonlinear_state(snapshot);
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
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        xyce_one_step_order2: bool,
    ) {
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase_and_coefficients(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::AcceptedStep,
            *coefficients,
            xyce_one_step_order2,
        ) {
            log::warn!("XSPICE evaluation error: {e}");
        }
    }

    /// Commit XSPICE state for an accepted transient timepoint using the
    /// integrator's selected companion coefficients.
    pub(crate) fn accept_xspice_transient_timestep_with_coefficients(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        xyce_one_step_order2: bool,
    ) {
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase_and_coefficients(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::AcceptedStep,
            *coefficients,
            xyce_one_step_order2,
        ) {
            log::warn!("XSPICE evaluation error: {e}");
        }
        self.accept_xspice_timestep();
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

        fn stamp_current_output_port(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            instance: &crate::xspice::XspiceInstance,
            port: &crate::xspice::PortSpec,
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
            num_nodes: usize,
        ) {
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
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            instance: &crate::xspice::XspiceInstance,
            port: &crate::xspice::PortSpec,
            output_index: usize,
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
            num_nodes: usize,
        ) {
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
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            instance: &crate::xspice::XspiceInstance,
            port: &crate::xspice::PortSpec,
            output_index: usize,
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

        for instance in &mut self.xspice_instances {
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
                                                matrix,
                                                rhs,
                                                instance,
                                                port,
                                                index,
                                                branch_ordinal,
                                                node,
                                                0,
                                                current,
                                                num_nodes,
                                            );
                                        }
                                    }
                                    crate::xspice::PortType::Current
                                    | crate::xspice::PortType::DifferentialCurrent
                                    | crate::xspice::PortType::Conductance
                                    | crate::xspice::PortType::DifferentialConductance => {
                                        stamp_current_vector_output_port(
                                            matrix,
                                            rhs,
                                            instance,
                                            port,
                                            index,
                                            node,
                                            0,
                                            conductance,
                                            current,
                                            num_nodes,
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
                                                        matrix,
                                                        rhs,
                                                        instance,
                                                        port,
                                                        index,
                                                        branch_ordinal,
                                                        *node,
                                                        0,
                                                        current,
                                                        num_nodes,
                                                    );
                                                }
                                            }
                                            crate::xspice::PortType::Current
                                            | crate::xspice::PortType::DifferentialCurrent
                                            | crate::xspice::PortType::Conductance
                                            | crate::xspice::PortType::DifferentialConductance => {
                                                stamp_current_vector_output_port(
                                                    matrix,
                                                    rhs,
                                                    instance,
                                                    port,
                                                    index,
                                                    *node,
                                                    0,
                                                    conductance,
                                                    current,
                                                    num_nodes,
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
                                                    matrix,
                                                    rhs,
                                                    instance,
                                                    port,
                                                    index,
                                                    branch_ordinal,
                                                    *pos,
                                                    *neg,
                                                    current,
                                                    num_nodes,
                                                );
                                            }
                                        }
                                        crate::xspice::PortType::Current
                                        | crate::xspice::PortType::DifferentialCurrent
                                        | crate::xspice::PortType::Conductance
                                        | crate::xspice::PortType::DifferentialConductance => {
                                            stamp_current_vector_output_port(
                                                matrix,
                                                rhs,
                                                instance,
                                                port,
                                                index,
                                                *pos,
                                                *neg,
                                                conductance,
                                                current,
                                                num_nodes,
                                            );
                                        }
                                        _ => {}
                                    },
                                    crate::xspice::AnalogInputConnection::CurrentOutput {
                                        pos,
                                        neg,
                                    } => {
                                        stamp_current_vector_output_port(
                                            matrix,
                                            rhs,
                                            instance,
                                            port,
                                            index,
                                            *pos,
                                            *neg,
                                            conductance,
                                            current,
                                            num_nodes,
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
                            matrix,
                            rhs,
                            instance,
                            port,
                            *pos,
                            *neg,
                            conductance,
                            current,
                            num_nodes,
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
                                matrix,
                                rhs,
                                instance,
                                port,
                                pos,
                                neg,
                                conductance,
                                current,
                                num_nodes,
                            );
                        }
                        _ => {}
                    }
                }
            }

            // Drain any explicit matrix/RHS stamps queued by the code model.
            for (row, col, value) in instance.take_deferred_stamps() {
                add_matrix_if_present(matrix, row, col, value);
            }
            for (node, value) in instance.take_deferred_rhs() {
                add_rhs_if_present(rhs, node, value);
            }
        }
    }

    /// Add the static-only residual retained by the last accepted XSPICE
    /// evaluation to a probe RHS.  Applying queued stamps directly to the
    /// residual avoids rebuilding matrix topology for every accepted step.
    pub(crate) fn stamp_xspice_static_residual(&mut self, solution: &[Value], rhs: &mut [Value]) {
        for instance in &mut self.xspice_instances {
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
            instance.accept_timestep();
        }
    }

    /// Project committed ideal XSPICE voltage outputs back into the accepted
    /// solution vector after event-driven models settle.
    pub(crate) fn project_xspice_voltage_outputs(&self, solution: &mut [Value], num_nodes: usize) {
        #[inline]
        fn set_node(solution: &mut [Value], node: usize, value: Value) {
            if node > 0
                && let Some(slot) = solution.get_mut(node - 1)
            {
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
        fn project_voltage_pair(solution: &mut [Value], pos: usize, neg: usize, value: Value) {
            if !value.is_finite() {
                return;
            }
            match (pos, neg) {
                (0, 0) => {}
                (node, 0) => set_node(solution, node, value),
                (0, node) => set_node(solution, node, -value),
                (pos, neg) => {
                    let Some(pos_value) = node_value(solution, pos) else {
                        return;
                    };
                    let Some(neg_value) = node_value(solution, neg) else {
                        return;
                    };
                    let correction = value - (pos_value - neg_value);
                    set_node(solution, pos, pos_value + 0.5 * correction);
                    set_node(solution, neg, neg_value - 0.5 * correction);
                }
            }
        }

        if num_nodes == 0 || solution.is_empty() {
            return;
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
                                project_voltage_pair(solution, node, 0, value);
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
                                        project_voltage_pair(solution, *node, 0, value);
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
                                        project_voltage_pair(solution, *pos, *neg, value);
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
                        project_voltage_pair(solution, *node, 0, value);
                    }
                    crate::xspice::PortConnection::Differential(pos, neg)
                    | crate::xspice::PortConnection::Hybrid { pos, neg, .. } => {
                        project_voltage_pair(solution, *pos, *neg, value);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Prepare build-time generated Verilog-A devices for a transient timepoint.
    #[cfg(feature = "veriloga-builtins-base")]
    pub fn prepare_generated_veriloga_timepoint(
        &mut self,
        time: Value,
        dt: Value,
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        dynamic_residual_scale: Value,
        initial_step: bool,
        final_step: bool,
    ) {
        let ddt_coefficients =
            crate::device::veriloga_builtins::GeneratedDdtCoefficients::from_companion_values_with_derivative_scale(
                    coefficients.coeff_g,
                    coefficients.coeff_v_n,
                    coefficients.coeff_v_n_minus_1,
                    coefficients.needs_two_history,
                    coefficients.coeff_i_n,
                    dt,
                )
            .scaled(dynamic_residual_scale);
        self.generated_veriloga_devices
            .set_timepoint(time, dt, ddt_coefficients);
        self.generated_veriloga_devices
            .set_analysis_step(initial_step, final_step);
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
                .stamp_all_with_mode(
                    probe,
                    rhs,
                    solution,
                    num_nodes,
                    crate::device::veriloga_builtins::GeneratedAnalysisKind::Tran,
                    simparams,
                    crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
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
            .filter_map(|(name, &candidate)| (candidate == node).then(|| name.to_ascii_lowercase()))
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
        checkpoint: &crate::device::veriloga::VerilogADeviceCheckpoint,
    ) -> Result<RuntimeVerilogADcStateKey, String> {
        let terminals = (0..device.num_terminals())
            .map(|terminal| self.veriloga_dc_terminal_identity(device.node_for_terminal(terminal)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RuntimeVerilogADcStateKey {
            instance_name: checkpoint.instance_name.to_ascii_lowercase(),
            model_name: checkpoint.model_name.to_string(),
            source_digest: checkpoint.source_digest.to_string(),
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
                        key: self.runtime_veriloga_dc_state_key(device, &checkpoint)?,
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
            let target_templates = self.veriloga_devices.checkpoint_states()?;
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
            let mut normalized = Vec::with_capacity(target_templates.len());
            for (device, mut target) in self
                .veriloga_devices
                .iter()
                .zip(target_templates.into_iter())
            {
                let key = self.runtime_veriloga_dc_state_key(device, &target)?;
                let source = captured.remove(&key).ok_or_else(|| {
                    format!(
                        "runtime Verilog-A DC state has no matching accepted instance '{}'",
                        target.instance_name
                    )
                })?;
                target.accepted.clone_from(&source.accepted);
                target.prev_discontinuity = source.prev_discontinuity;
                normalized.push(target);
            }
            if let Some((key, _)) = captured.first_key_value() {
                return Err(format!(
                    "runtime Verilog-A DC state contains unmatched accepted instance '{}'",
                    key.instance_name
                ));
            }
            self.veriloga_devices
                .validate_checkpoint_states(&normalized)?;
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

        #[cfg(feature = "veriloga")]
        self.veriloga_devices.restore_checkpoint_states(&runtime)?;
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .restore_checkpoint_states(&generated)?;
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

    /// Prepare Verilog-A devices for a transient timepoint evaluation
    ///
    /// Sets the simulation time, integration timestep, and analysis type so
    /// ddt/idt and event operators see transient semantics.
    #[cfg(feature = "veriloga")]
    pub(crate) fn begin_veriloga_analysis(&mut self, analysis: u8) -> Result<(), String> {
        for device in self.veriloga_devices.iter_mut() {
            let instance = device.name.clone();
            device.try_begin_analysis(analysis).map_err(|error| {
                format!("Verilog-A device '{instance}' analysis begin failed: {error}")
            })?;
        }
        Ok(())
    }

    /// Begin a fresh DC analysis for every runtime-compiled Verilog-A
    /// instance. Generated instances are newly constructed with the circuit;
    /// their DC analysis kind is supplied by the ordinary DC stamp path.
    pub(crate) fn begin_veriloga_dc_analysis(&mut self) -> Result<(), String> {
        self.begin_veriloga_equilibrium_analysis(0)
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
        #[cfg(feature = "veriloga")]
        self.begin_veriloga_analysis(analysis)?;
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
        Ok(())
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
        if !matches!(analysis, 0 | 1 | 3 | 4) {
            return Err(format!(
                "equilibrium Verilog-A analysis must be 0=dc, 1=ac, 3=noise, or 4=ic, got {analysis}"
            ));
        }
        // Both arguments remain part of the lifecycle contract even in a
        // feature-minimal build where neither Verilog-A backend is compiled.
        let _ = (initial_step, final_step);
        #[cfg(feature = "veriloga")]
        for device in self.veriloga_devices.iter_mut() {
            let instance = device.name.clone();
            device.try_set_analysis_type(analysis).map_err(|error| {
                format!("Verilog-A device '{instance}' equilibrium analysis setup failed: {error}")
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
        self.prepare_veriloga_equilibrium_analysis_point(analysis, false, false)?;
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
        self.prepare_veriloga_equilibrium_analysis_point(analysis, false, final_step)
    }

    /// Atomically accept the exact public analysis point across both
    /// runtime-compiled and build-time generated Verilog-A instances.
    /// Validation completes for every instance before the first mutation.
    pub(crate) fn accept_veriloga_analysis_point(&mut self) -> Result<(), String> {
        #[cfg(feature = "veriloga")]
        self.veriloga_devices.validate_timestep_acceptance()?;
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .validate_state_acceptance()?;

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

    #[cfg(feature = "veriloga")]
    pub fn prepare_veriloga_timepoint(
        &mut self,
        time: Value,
        dt: Value,
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), String> {
        const INTEGRATION_EPSILON: Value = 1.0e-20;
        let integration = if dt.is_finite() && dt.abs() > INTEGRATION_EPSILON {
            let inverse_timestep = 1.0 / dt;
            rspice_veriloga::vm::IntegrationCoefficients {
                active: true,
                derivative_scale: coefficients.coeff_g * inverse_timestep,
                previous_value_scale: coefficients.coeff_v_n * inverse_timestep,
                older_value_scale: if coefficients.needs_two_history {
                    coefficients.coeff_v_n_minus_1 * inverse_timestep
                } else {
                    0.0
                },
                previous_derivative_scale: coefficients.coeff_i_n,
            }
        } else {
            rspice_veriloga::vm::IntegrationCoefficients::inactive()
        };
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
                .try_set_integration_coefficients(integration)
                .map_err(|error| {
                    format!(
                        "Verilog-A device '{instance}' transient integration setup failed: {error}"
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
        #[cfg(feature = "veriloga")]
        self.veriloga_devices.validate_timestep_acceptance()?;
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .validate_state_acceptance()?;

        #[cfg(feature = "veriloga")]
        let discontinuity = self
            .veriloga_devices
            .iter()
            .any(rspice_veriloga::device::VerilogADevice::discontinuity_rising);
        #[cfg(not(feature = "veriloga"))]
        let discontinuity = false;

        #[cfg(feature = "veriloga")]
        self.veriloga_devices.apply_validated_timestep_acceptance();
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .apply_validated_state_acceptance();
        Ok(discontinuity)
    }

    /// Tightest `$bound_step` request across Verilog-A devices at the
    /// latest evaluation (None when nothing bounds the next step)
    #[cfg(feature = "veriloga")]
    pub(crate) fn veriloga_timestep_bound(&self) -> Result<Option<Value>, String> {
        let mut tightest: Option<Value> = None;
        for device in self.veriloga_devices.iter() {
            let instance = device.name.clone();
            let Some(bound) = device.try_transient_bound_step().map_err(|error| {
                format!("Verilog-A device '{instance}' timestep bound failed: {error}")
            })?
            else {
                continue;
            };
            tightest = Some(tightest.map_or(bound, |current| current.min(bound)));
        }
        Ok(tightest)
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
                let instance = device.name.clone();
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
        for device in self.veriloga_devices.iter() {
            let instance = device.name.clone();
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
            instance.restore_checkpoint_state(checkpoint)?;
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
        Event, EventQueue, EventValue, ParamSpec, PortConnection, PortDirection, PortSpec,
        PortType, XspiceInstance,
    };
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::{Arc, Mutex};

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
        let mut digital_values = HashMap::new();
        let mut digital_drivers = HashMap::new();
        let mut digital_event_times = HashMap::new();
        let mut real_values = HashMap::new();
        let mut real_drivers = HashMap::new();
        let mut real_event_times = HashMap::new();
        let mut event_queue = EventQueue::new();
        let mut touched_digital_nodes = Vec::with_capacity(4);
        let mut touched_real_nodes = Vec::with_capacity(4);
        let digital_capacity = touched_digital_nodes.capacity();
        let real_capacity = touched_real_nodes.capacity();

        event_queue.schedule(Event::new(
            1.0e-9,
            1,
            "out_a",
            "d_a",
            EventValue::Digital(DigitalValue::one()),
        ));
        event_queue.schedule(Event::new(
            1.0e-9,
            1,
            "out_b",
            "d_b",
            EventValue::Digital(DigitalValue::one()),
        ));
        event_queue.schedule(Event::new(1.0e-9, 2, "out_a", "r_a", EventValue::Real(1.0)));
        event_queue.schedule(Event::new(1.0e-9, 2, "out_b", "r_b", EventValue::Real(2.0)));

        assert!(apply_xspice_events_at_or_before(
            &mut digital_values,
            &mut digital_drivers,
            &mut digital_event_times,
            &mut real_values,
            &mut real_drivers,
            &mut real_event_times,
            &mut event_queue,
            &mut touched_digital_nodes,
            &mut touched_real_nodes,
            1.0e-9,
        ));

        assert_eq!(touched_digital_nodes, vec![1]);
        assert_eq!(touched_real_nodes, vec![2]);
        assert_eq!(touched_digital_nodes.capacity(), digital_capacity);
        assert_eq!(touched_real_nodes.capacity(), real_capacity);
        assert_eq!(
            digital_drivers.get(&1).map(|drivers| drivers.len()),
            Some(2)
        );
        assert_eq!(real_drivers.get(&2).map(|drivers| drivers.len()), Some(2));
        assert_eq!(digital_values.get(&1).copied(), Some(DigitalValue::one()));
        assert_eq!(digital_event_times.get(&1).copied(), Some(1.0e-9));
        assert_eq!(real_values.get(&2).copied(), Some(3.0));
        assert_eq!(real_event_times.get(&2).copied(), Some(1.0e-9));
        assert!(event_queue.is_empty());

        assert!(!apply_xspice_events_at_or_before(
            &mut digital_values,
            &mut digital_drivers,
            &mut digital_event_times,
            &mut real_values,
            &mut real_drivers,
            &mut real_event_times,
            &mut event_queue,
            &mut touched_digital_nodes,
            &mut touched_real_nodes,
            1.0e-9,
        ));

        assert!(touched_digital_nodes.is_empty());
        assert!(touched_real_nodes.is_empty());
        assert_eq!(touched_digital_nodes.capacity(), digital_capacity);
        assert_eq!(touched_real_nodes.capacity(), real_capacity);
    }

    #[test]
    fn apply_xspice_events_preserves_vector_driver_elements_on_same_node() {
        let mut digital_values = HashMap::new();
        let mut digital_drivers = HashMap::new();
        let mut digital_event_times = HashMap::new();
        let mut real_values = HashMap::new();
        let mut real_drivers = HashMap::new();
        let mut real_event_times = HashMap::new();
        let mut event_queue = EventQueue::new();
        let mut touched_digital_nodes = Vec::new();
        let mut touched_real_nodes = Vec::new();

        event_queue.schedule(Event::new_with_driver_index(
            1.0e-9,
            1,
            "out",
            "vector_driver",
            0,
            EventValue::Digital(DigitalValue::one()),
        ));
        event_queue.schedule(Event::new_with_driver_index(
            1.0e-9,
            1,
            "out",
            "vector_driver",
            1,
            EventValue::Digital(DigitalValue::one()),
        ));
        event_queue.schedule(Event::new_with_driver_index(
            1.0e-9,
            2,
            "real_out",
            "real_vector_driver",
            0,
            EventValue::Real(1.0),
        ));
        event_queue.schedule(Event::new_with_driver_index(
            1.0e-9,
            2,
            "real_out",
            "real_vector_driver",
            1,
            EventValue::Real(2.0),
        ));

        assert!(apply_xspice_events_at_or_before(
            &mut digital_values,
            &mut digital_drivers,
            &mut digital_event_times,
            &mut real_values,
            &mut real_drivers,
            &mut real_event_times,
            &mut event_queue,
            &mut touched_digital_nodes,
            &mut touched_real_nodes,
            1.0e-9,
        ));

        assert_eq!(
            digital_drivers.get(&1).map(|drivers| drivers.len()),
            Some(2)
        );
        assert_eq!(real_drivers.get(&2).map(|drivers| drivers.len()), Some(2));
        assert_eq!(digital_values.get(&1).copied(), Some(DigitalValue::one()));
        assert_eq!(real_values.get(&2).copied(), Some(3.0));
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

        circuit.add_xspice_instance(output_instance(
            PortType::Digital,
            PortConnection::Digital(2),
        ));
        assert!(circuit.has_xspice_event_driven_devices());
        assert_eq!(
            circuit.xspice_event_node_matrix_rows().collect::<Vec<_>>(),
            vec![1]
        );
    }

    #[test]
    fn fill_xspice_digital_snapshot_reuses_buffer_and_sorts_nodes() {
        let mut circuit = CircuitData::new();
        circuit.xspice_digital_values.insert(3, DigitalValue::one());
        circuit
            .xspice_digital_values
            .insert(1, DigitalValue::zero());

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
        assert_eq!(xspice_event_output_count(&circuit.xspice_instances), 1);

        let result =
            circuit.try_evaluate_xspice_with_analysis(0.0, 1e-9, &[], AnalysisType::Transient);
        assert!(
            result.is_err(),
            "expected event iteration failure, got {result:?}"
        );
        let err = circuit
            .take_xspice_evaluation_error()
            .expect("non-settling XSPICE event networks must be reported");

        assert!(err.contains("XSPICE event network did not settle"));
        assert!(err.contains("2 passes"));
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

        circuit.accept_xspice_transient_timestep_with_coefficients(
            2.0e-9,
            1.0e-9,
            &[0.0],
            &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            false,
        );

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
        circuit.accept_xspice_transient_timestep_with_coefficients(
            1.0e-9,
            1.0e-9,
            &[0.0],
            &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            false,
        );

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
        circuit.project_xspice_voltage_outputs(&mut solution, circuit.num_nodes);

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
}
