use super::*;
use std::collections::HashMap;

fn apply_xspice_events_at_or_before(
    digital_values: &mut HashMap<NodeId, crate::xspice::DigitalValue>,
    digital_drivers: &mut HashMap<(NodeId, String, String), crate::xspice::DigitalValue>,
    digital_event_times: &mut HashMap<NodeId, Value>,
    real_values: &mut HashMap<NodeId, Value>,
    real_drivers: &mut HashMap<(NodeId, String, String), Value>,
    real_event_times: &mut HashMap<NodeId, Value>,
    event_queue: &mut crate::xspice::EventQueue,
    time: Value,
) -> bool {
    fn resolve_node_drivers(
        drivers: &HashMap<(NodeId, String, String), crate::xspice::DigitalValue>,
        node_id: NodeId,
    ) -> Option<crate::xspice::DigitalValue> {
        drivers
            .iter()
            .filter_map(|((driver_node, _, _), value)| (*driver_node == node_id).then_some(*value))
            .reduce(|acc, value| acc.resolve(&value))
    }

    fn resolve_real_node_drivers(
        drivers: &HashMap<(NodeId, String, String), Value>,
        node_id: NodeId,
    ) -> Option<Value> {
        let mut found = false;
        let mut sum = 0.0;
        for ((driver_node, _, _), value) in drivers {
            if *driver_node == node_id {
                found = true;
                sum += *value;
            }
        }
        found.then_some(sum)
    }

    let mut changed = false;
    for event in event_queue.pop_events_at(time) {
        let driver_key = (event.node_id, event.instance, event.port_name);
        match event.value {
            crate::xspice::EventValue::Digital(value) => {
                digital_drivers.insert(driver_key, value);
                let resolved =
                    resolve_node_drivers(digital_drivers, event.node_id).unwrap_or_default();
                let previous_value = digital_values.insert(event.node_id, resolved);
                let previous_time = digital_event_times.insert(event.node_id, event.time);
                changed |= previous_value != Some(resolved) || previous_time != Some(event.time);
            }
            crate::xspice::EventValue::Real(value) => {
                real_drivers.insert(driver_key, value);
                let resolved =
                    resolve_real_node_drivers(real_drivers, event.node_id).unwrap_or(0.0);
                let previous_value = real_values.insert(event.node_id, resolved);
                let previous_time = real_event_times.insert(event.node_id, event.time);
                changed |= previous_value != Some(resolved) || previous_time != Some(event.time);
            }
        }
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

    #[cfg(feature = "veriloga-builtins")]
    #[inline]
    pub fn has_generated_veriloga_devices(&self) -> bool {
        !self.generated_veriloga_devices.is_empty()
    }

    #[cfg(feature = "veriloga-builtins")]
    pub fn add_generated_veriloga_device(
        &mut self,
        device: crate::device::veriloga_generated::BuiltinVerilogAInstance,
    ) {
        self.generated_veriloga_devices.add(device);
    }

    #[cfg(feature = "veriloga-builtins")]
    pub(crate) fn generated_veriloga_devices(
        &self,
    ) -> &crate::device::veriloga_generated::BuiltinVerilogADevices {
        &self.generated_veriloga_devices
    }

    #[cfg(feature = "veriloga-builtins")]
    pub(crate) fn generated_veriloga_devices_mut(
        &mut self,
    ) -> &mut crate::device::veriloga_generated::BuiltinVerilogADevices {
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

    /// Evaluate all XSPICE code model instances for transient with explicit timestep.
    pub fn evaluate_xspice_with_timestep(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) {
        if let Err(e) = self.try_evaluate_xspice_with_timestep(time, timestep, voltages) {
            log::warn!("XSPICE evaluation error: {e}");
        }
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
        let max_event_passes = if self.has_xspice_event_driven_devices() {
            self.xspice_instances.len().saturating_add(1).max(1)
        } else {
            1
        };

        for _pass in 0..max_event_passes {
            let digital_values = &mut self.xspice_digital_values;
            let digital_drivers = &mut self.xspice_digital_drivers;
            let digital_event_times = &mut self.xspice_digital_event_times;
            let real_values = &mut self.xspice_real_values;
            let real_drivers = &mut self.xspice_real_drivers;
            let real_event_times = &mut self.xspice_real_event_times;
            let event_queue = &mut self.xspice_event_queue;
            let mut changed = apply_xspice_events_at_or_before(
                digital_values,
                digital_drivers,
                digital_event_times,
                real_values,
                real_drivers,
                real_event_times,
                event_queue,
                time,
            );

            for instance in &mut self.xspice_instances {
                instance.update_inputs(
                    solution,
                    self.num_nodes,
                    digital_values,
                    digital_event_times,
                    real_values,
                    real_event_times,
                );

                if let Err(e) = instance.evaluate(time, timestep, analysis, phase) {
                    let message = format!("{}: {}", instance.name, e);
                    if self.xspice_evaluation_error.is_none() {
                        self.xspice_evaluation_error = Some(message.clone());
                    }
                    return Err(crate::xspice::CmError::EvaluationError(message));
                }

                instance.schedule_events(event_queue, time);
                changed |= apply_xspice_events_at_or_before(
                    digital_values,
                    digital_drivers,
                    digital_event_times,
                    real_values,
                    real_drivers,
                    real_event_times,
                    event_queue,
                    time,
                );
            }

            if !changed {
                break;
            }
        }
        Ok(())
    }

    /// Return and clear the first XSPICE evaluation error recorded during
    /// this analysis, if any.
    pub(crate) fn take_xspice_evaluation_error(&mut self) -> Option<String> {
        self.xspice_evaluation_error.take()
    }

    /// Snapshot committed XSPICE digital node values with stable netlist names.
    pub(crate) fn xspice_digital_snapshot(&self) -> Vec<(String, crate::xspice::DigitalValue)> {
        let mut node_names: Vec<(NodeId, String)> = self
            .node_map
            .iter()
            .filter_map(|(name, &id)| (id > 0).then_some((id, name.clone())))
            .collect();
        node_names.sort_by_key(|(id, _)| *id);
        node_names.dedup_by_key(|(id, _)| *id);

        node_names
            .into_iter()
            .filter_map(|(node_id, name)| {
                self.xspice_digital_values
                    .get(&node_id)
                    .copied()
                    .map(|value| (name, value))
            })
            .collect()
    }

    /// Snapshot committed XSPICE real-valued node values with stable netlist names.
    pub(crate) fn xspice_real_snapshot(&self) -> Vec<(String, Value)> {
        let mut node_names: Vec<(NodeId, String)> = self
            .node_map
            .iter()
            .filter_map(|(name, &id)| (id > 0).then_some((id, name.clone())))
            .collect();
        node_names.sort_by_key(|(id, _)| *id);
        node_names.dedup_by_key(|(id, _)| *id);

        node_names
            .into_iter()
            .filter_map(|(node_id, name)| {
                self.xspice_real_values
                    .get(&node_id)
                    .copied()
                    .map(|value| (name, value))
            })
            .collect()
    }

    /// Time of the next pending XSPICE digital event, if any.
    pub(crate) fn next_xspice_event_time(&self) -> Option<Value> {
        self.xspice_event_queue.next_event_time()
    }

    /// Drain absolute transient breakpoint requests emitted by XSPICE models.
    pub(crate) fn take_xspice_requested_breakpoints(&mut self) -> Vec<Value> {
        let mut breakpoints = Vec::new();
        for instance in &mut self.xspice_instances {
            breakpoints.extend(instance.take_requested_breakpoints());
        }
        breakpoints
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
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::RollbackableProbe,
        ) {
            log::warn!("XSPICE evaluation error: {e}");
        }
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
        if let Err(e) = self.try_evaluate_xspice_with_analysis_phase(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
            crate::xspice::EvaluationPhase::AcceptedStep,
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
                if let Some(contributions) = instance.get_analog_vector_contributions(port_idx) {
                    let Some(port) = ports.get(port_idx) else {
                        continue;
                    };
                    match connection {
                        crate::xspice::PortConnection::AnalogVector(nodes) => {
                            for (index, node) in nodes.iter().copied().enumerate() {
                                let (conductance, current) =
                                    contributions.get(index).copied().unwrap_or((0.0, 0.0));
                                match port.default_type {
                                    crate::xspice::PortType::Voltage
                                    | crate::xspice::PortType::DifferentialVoltage => {
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
                                    crate::xspice::PortType::Current => {
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
                                    contributions.get(index).copied().unwrap_or((0.0, 0.0));
                                match element {
                                    crate::xspice::AnalogInputConnection::Node(node) => {
                                        match port.default_type {
                                            crate::xspice::PortType::Voltage
                                            | crate::xspice::PortType::DifferentialVoltage => {
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
                                            crate::xspice::PortType::Current => {
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
                                    ) => match port.default_type {
                                        crate::xspice::PortType::Voltage
                                        | crate::xspice::PortType::DifferentialVoltage => {
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
                                        crate::xspice::PortType::Current => {
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
                        crate::xspice::PortType::Current => {
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

    /// Accept current timestep for all XSPICE instances
    ///
    /// Called after a successful timestep to commit state changes.
    pub fn accept_xspice_timestep(&mut self) {
        for instance in &mut self.xspice_instances {
            instance.accept_timestep();
        }
    }

    /// Prepare build-time generated Verilog-A devices for a transient timepoint.
    #[cfg(feature = "veriloga-builtins")]
    pub fn prepare_generated_veriloga_timepoint(
        &mut self,
        time: Value,
        dt: Value,
        coefficients: &crate::analysis::CompanionCoefficients,
    ) {
        let ddt_coefficients =
            crate::device::veriloga_generated::GeneratedDdtCoefficients::from_companion(
                coefficients,
                dt,
            );
        self.generated_veriloga_devices
            .set_timepoint(time, dt, ddt_coefficients);
    }

    /// Commit build-time generated Verilog-A integrator state after acceptance.
    #[cfg(feature = "veriloga-builtins")]
    pub fn accept_generated_veriloga_timestep(&mut self) {
        self.generated_veriloga_devices.accept_timestep();
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
        AnalysisType, CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec,
        PortConnection, PortDirection, PortSpec, PortType, XspiceInstance,
    };
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::{Arc, Mutex};

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
            ctx.set_output_vector("out", vec![2.0 * in0 + 1.0, 3.0 * in1 + 1.0]);
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
    fn evaluate_xspice_reports_model_errors_without_stamping_stale_outputs() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.xspice_instances.push(failing_instance());

        circuit.evaluate_xspice_with_analysis(0.0, 1e-9, &[0.0], AnalysisType::Transient);
        let err = circuit
            .take_xspice_evaluation_error()
            .expect("legacy engine-facing XSPICE evaluation must record model errors");

        assert!(err.contains("Afail"));
        assert!(err.contains("intentional failure"));
    }

    #[test]
    fn stamp_xspice_skips_out_of_range_current_output_without_panicking() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("n1");
        circuit.xspice_instances.push(output_instance(
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
        circuit.xspice_instances.push(breakpoint_instance());

        circuit.accept_xspice_transient_timestep(2.0e-9, 1.0e-9, &[0.0]);

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
        circuit
            .xspice_instances
            .push(phase_probe_instance(Arc::clone(&seen_phases)));
        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("1x1 matrix should construct");
        let mut rhs = vec![0.0];

        circuit.stamp_xspice_transient_trial(&mut matrix, &mut rhs, 1.0e-9, 1.0e-9, &[0.0]);
        circuit.accept_xspice_transient_timestep(1.0e-9, 1.0e-9, &[0.0]);

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
        circuit.xspice_instances.push(instance);
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
        circuit.xspice_instances.push(instance);

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
        circuit.xspice_instances.push(instance);

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
        circuit.xspice_instances.push(instance);

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
        circuit.xspice_instances.push(instance);

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
