use super::*;
use std::collections::HashMap;

fn apply_xspice_events_at_or_before(
    digital_values: &mut HashMap<NodeId, crate::xspice::DigitalValue>,
    digital_event_times: &mut HashMap<NodeId, Value>,
    event_queue: &mut crate::xspice::EventQueue,
    time: Value,
) -> bool {
    let mut changed = false;
    for event in event_queue.pop_events_at(time) {
        let previous_value = digital_values.insert(event.node_id, event.value);
        let previous_time = digital_event_times.insert(event.node_id, event.time);
        changed |= previous_value != Some(event.value) || previous_time != Some(event.time);
    }
    changed
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

    /// Check if any XSPICE instance participates in event-driven scheduling.
    #[inline]
    pub fn has_xspice_event_driven_devices(&self) -> bool {
        self.xspice_instances.iter().any(|instance| {
            instance
                .ports()
                .iter()
                .any(|port| port.default_type.is_event_driven())
        })
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn has_veriloga_devices(&self) -> bool {
        !self.veriloga_devices.is_empty()
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn veriloga_device_count(&self) -> usize {
        self.veriloga_devices.len()
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

    /// Evaluate all XSPICE code model instances
    ///
    /// This calls each XspiceInstance::evaluate() with the current simulation
    /// state, updating internal context and computing output contributions.
    ///
    /// # Arguments
    /// * `time` - Current simulation time
    /// * `voltages` - Current node voltage solution
    pub fn evaluate_xspice(&mut self, time: Value, voltages: &[Value]) {
        self.evaluate_xspice_with_analysis(
            time,
            0.0,
            voltages,
            crate::xspice::AnalysisType::Transient,
        );
    }

    /// Evaluate all XSPICE code model instances for transient with explicit timestep.
    pub fn evaluate_xspice_with_timestep(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) {
        self.evaluate_xspice_with_analysis(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
        );
    }

    /// Evaluate all XSPICE code model instances for the requested analysis type.
    pub fn evaluate_xspice_with_analysis(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        analysis: crate::xspice::AnalysisType,
    ) {
        let max_event_passes = if self.has_xspice_event_driven_devices() {
            self.xspice_instances.len().saturating_add(1).max(1)
        } else {
            1
        };

        for _pass in 0..max_event_passes {
            let digital_values = &mut self.xspice_digital_values;
            let digital_event_times = &mut self.xspice_digital_event_times;
            let event_queue = &mut self.xspice_event_queue;
            let mut changed = apply_xspice_events_at_or_before(
                digital_values,
                digital_event_times,
                event_queue,
                time,
            );

            for instance in &mut self.xspice_instances {
                instance.update_inputs(voltages, digital_values, digital_event_times);

                if let Err(e) = instance.evaluate(time, timestep, analysis) {
                    log::warn!("XSPICE evaluation error for {}: {}", instance.name, e);
                }

                instance.schedule_events(event_queue, time);
                changed |= apply_xspice_events_at_or_before(
                    digital_values,
                    digital_event_times,
                    event_queue,
                    time,
                );
            }

            if !changed {
                break;
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
        let snapshot = self.nonlinear_state_snapshot();
        self.evaluate_xspice_with_timestep(time, timestep, voltages);
        self.stamp_xspice(matrix, rhs);
        self.restore_nonlinear_state(snapshot);
    }

    /// Commit XSPICE state for an accepted transient timepoint.
    pub(crate) fn accept_xspice_transient_timestep(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) {
        self.evaluate_xspice_with_timestep(time, timestep, voltages);
        self.accept_xspice_timestep();
    }

    /// Stamp XSPICE analog contributions into matrix and RHS
    ///
    /// After evaluation, analog code models produce conductance and current
    /// contributions that must be stamped into the MNA system.
    pub fn stamp_xspice(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;

        #[inline]
        fn stamp_nodal_current_output(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            connection: &crate::xspice::PortConnection,
            conductance: Value,
            current: Value,
        ) {
            match connection {
                crate::xspice::PortConnection::Analog(node) => {
                    if *node > 0 {
                        matrix.add(*node - 1, *node - 1, conductance);
                        rhs[*node - 1] += current;
                    }
                }
                crate::xspice::PortConnection::Differential(pos, neg) => {
                    if *pos > 0 {
                        matrix.add(*pos - 1, *pos - 1, conductance);
                        if *neg > 0 {
                            matrix.add(*pos - 1, *neg - 1, -conductance);
                        }
                        rhs[*pos - 1] += current;
                    }
                    if *neg > 0 {
                        if *pos > 0 {
                            matrix.add(*neg - 1, *pos - 1, -conductance);
                        }
                        matrix.add(*neg - 1, *neg - 1, conductance);
                        rhs[*neg - 1] -= current;
                    }
                }
                _ => {}
            }
        }

        for instance in &mut self.xspice_instances {
            let ports = instance.ports();
            // Get contributions from each output port
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                if let Some((conductance, current)) = instance.get_analog_contribution(port_idx) {
                    let Some(port) = ports.get(port_idx) else {
                        continue;
                    };
                    match port.default_type {
                        crate::xspice::PortType::Voltage
                        | crate::xspice::PortType::DifferentialVoltage => {
                            if let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) {
                                let br_mna = num_nodes + branch_ordinal;
                                let br = br_mna - 1;
                                match connection {
                                    crate::xspice::PortConnection::Analog(node) => {
                                        if *node > 0 {
                                            matrix.add(br, *node - 1, 1.0);
                                            matrix.add(*node - 1, br, 1.0);
                                        }
                                        rhs[br] += current;
                                    }
                                    crate::xspice::PortConnection::Differential(pos, neg) => {
                                        if *pos > 0 {
                                            matrix.add(br, *pos - 1, 1.0);
                                            matrix.add(*pos - 1, br, 1.0);
                                        }
                                        if *neg > 0 {
                                            matrix.add(br, *neg - 1, -1.0);
                                            matrix.add(*neg - 1, br, -1.0);
                                        }
                                        rhs[br] += current;
                                    }
                                    _ => {
                                        stamp_nodal_current_output(
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
                                stamp_nodal_current_output(
                                    matrix,
                                    rhs,
                                    connection,
                                    conductance,
                                    current,
                                );
                            }
                        }
                        crate::xspice::PortType::Current => {
                            stamp_nodal_current_output(
                                matrix,
                                rhs,
                                connection,
                                conductance,
                                current,
                            );
                        }
                        _ => {}
                    }
                }
            }

            // Drain any explicit matrix/RHS stamps queued by the code model.
            for (row, col, value) in instance.take_deferred_stamps() {
                if row < rhs.len() && col < rhs.len() {
                    if matrix.get_index(row, col).is_some() {
                        matrix.add(row, col, value);
                    } else {
                        log::debug!(
                            "XSPICE deferred stamp ({}, {}) missing from matrix topology",
                            row,
                            col
                        );
                    }
                }
            }
            for (node, value) in instance.take_deferred_rhs() {
                if node < rhs.len() {
                    rhs[node] += value;
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

    /// Prepare Verilog-A devices for a transient timepoint evaluation
    ///
    /// Sets the simulation time, integration timestep, and analysis type so
    /// ddt/idt and event operators see transient semantics.
    #[cfg(feature = "veriloga")]
    pub fn prepare_veriloga_timepoint(&mut self, time: Value, dt: Value) {
        for device in self.veriloga_devices.iter_mut() {
            device.set_analysis_type(2);
            device.set_time(time);
            device.set_timestep(dt);
        }
    }

    /// Commit Verilog-A integrator state after an accepted timestep.
    ///
    /// Returns whether any device newly raised `$discontinuity` at this
    /// step (a rising edge against the previous accepted step), so the
    /// stepper can place a fine restart without a level-true region
    /// pinning tiny steps forever.
    #[cfg(feature = "veriloga")]
    pub fn accept_veriloga_timestep(&mut self) -> bool {
        let mut discontinuity = false;
        for device in self.veriloga_devices.iter_mut() {
            discontinuity |= device.discontinuity_rising();
            device.advance_state();
        }
        discontinuity
    }

    /// Tightest `$bound_step` request across Verilog-A devices at the
    /// latest evaluation (None when nothing bounds the next step)
    #[cfg(feature = "veriloga")]
    pub fn veriloga_timestep_bound(&self) -> Option<Value> {
        self.veriloga_devices
            .iter()
            .filter_map(|device| device.transient_bound_step())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Whether any Verilog-A device flagged `$discontinuity` at the
    /// latest evaluation
    #[cfg(feature = "veriloga")]
    pub fn veriloga_discontinuity_pending(&self) -> bool {
        self.veriloga_devices
            .iter()
            .any(|device| device.discontinuity_pending())
    }

    /// Check if all XSPICE instances have converged
    pub fn xspice_converged(&self, tolerance: Value) -> bool {
        self.xspice_instances
            .iter()
            .all(|inst| inst.is_converged(tolerance))
    }
}
