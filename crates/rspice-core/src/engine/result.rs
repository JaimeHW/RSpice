//! Public time-domain result types.

use crate::analysis::waveform::TransientResultCompressed;
use crate::xspice::DigitalValue;
use crate::{NodeId, Value};
use std::collections::HashMap;

/// One accepted digital event sample for a named XSPICE digital node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitalTracePoint {
    /// Event time in seconds.
    pub time: Value,
    /// Digital value at this event time.
    pub value: DigitalValue,
}

/// Accepted digital event history for one XSPICE digital node.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalTrace {
    /// Original netlist node name.
    pub node_name: String,
    /// Committed event samples for this node.
    pub points: Vec<DigitalTracePoint>,
}

/// One accepted real-valued event sample for a named XSPICE real node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealTracePoint {
    /// Event time in seconds.
    pub time: Value,
    /// Real event-node value at this event time.
    pub value: Value,
}

/// Accepted real-valued event history for one XSPICE real node.
#[derive(Debug, Clone, PartialEq)]
pub struct RealTrace {
    /// Original netlist node name.
    pub node_name: String,
    /// Committed event samples for this node.
    pub points: Vec<RealTracePoint>,
}

/// Accepted operating-point parameter history for one device.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientDeviceOpTrace {
    /// Original netlist device name.
    pub device_name: String,
    /// Device operating-point parameter name, such as `vbs` or `gm`.
    pub parameter: String,
    /// One value per accepted transient sample.
    pub values: Vec<Value>,
}

/// One typed non-solution device store waveform, such as a compact-model
/// internal resistance. Store traces are deliberately separate from voltage
/// nodes so units, matrix topology, compression, and UI labeling remain sound.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientStoreTrace {
    /// Canonical Xyce store name, for example `YMEMRISTOR!MR1:R`.
    pub name: String,
    /// One value per accepted transient sample.
    pub values: Vec<Value>,
}

/// Result of transient analysis - time-domain waveforms
#[derive(Debug, Clone)]
pub struct TransientResult {
    /// Time points
    pub time: Vec<Value>,
    /// Voltage waveforms: [node_index][time_index]
    pub voltages: Vec<Vec<Value>>,
    /// Branch current waveforms: [branch_index][time_index]
    pub branch_currents: Vec<Vec<Value>>,
    /// Number of nodes
    pub num_nodes: usize,
    /// Node names from the netlist (maps index to original name like "N001", "out", etc.)
    /// Index 0 corresponds to voltages[0], which is node 1 (not ground)
    pub node_names: Vec<String>,
    /// Branch names aligned with `branch_currents`
    pub branch_names: Vec<String>,
    /// XSPICE digital node histories captured at accepted transient points.
    pub digital_traces: Vec<DigitalTrace>,
    /// XSPICE real-valued event node histories captured at accepted transient points.
    pub real_traces: Vec<RealTrace>,
    /// Device operating-point values captured at accepted transient points.
    pub device_op_traces: Vec<TransientDeviceOpTrace>,
    /// Typed device store outputs captured at accepted transient points.
    pub store_traces: Vec<TransientStoreTrace>,
}

impl TransientResult {
    /// Append one accepted device operating-point snapshot. Missing parameters
    /// are padded with NaN so every stored trace remains aligned with `time`.
    pub(crate) fn record_device_op_sample(&mut self, report: crate::circuit::DeviceOpReport) {
        if self.time.is_empty() {
            return;
        }

        let sample_index = self.time.len() - 1;
        let mut seen = vec![false; self.device_op_traces.len()];

        for entry in report.entries {
            for (parameter, value) in entry.params {
                let trace_index = self
                    .device_op_traces
                    .iter()
                    .position(|trace| {
                        trace.device_name.eq_ignore_ascii_case(&entry.name)
                            && trace.parameter.eq_ignore_ascii_case(parameter)
                    })
                    .unwrap_or_else(|| {
                        let trace_index = self.device_op_traces.len();
                        self.device_op_traces.push(TransientDeviceOpTrace {
                            device_name: entry.name.clone(),
                            parameter: parameter.to_string(),
                            values: vec![Value::NAN; sample_index],
                        });
                        seen.push(false);
                        trace_index
                    });

                let trace = &mut self.device_op_traces[trace_index];
                if trace.values.len() < sample_index {
                    trace.values.resize(sample_index, Value::NAN);
                }
                if trace.values.len() == sample_index {
                    trace.values.push(value);
                } else if let Some(slot) = trace.values.get_mut(sample_index) {
                    *slot = value;
                }
                seen[trace_index] = true;
            }
        }

        for (trace_index, trace) in self.device_op_traces.iter_mut().enumerate() {
            if seen.get(trace_index).copied().unwrap_or(false) {
                continue;
            }
            if trace.values.len() < sample_index {
                trace.values.resize(sample_index, Value::NAN);
            }
            if trace.values.len() == sample_index {
                trace.values.push(Value::NAN);
            }
        }
    }

    /// Append committed XSPICE digital values at an accepted transient time.
    pub(crate) fn record_digital_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(NodeId, DigitalValue)],
        trace_indices: &mut HashMap<NodeId, usize>,
    ) {
        for &(node_id, value) in snapshot {
            let Some(node_name) = node_id
                .checked_sub(1)
                .and_then(|index| self.node_names.get(index))
            else {
                continue;
            };
            let trace_idx = match trace_indices.get(&node_id).copied() {
                Some(index) => index,
                None => {
                    let index = self.digital_traces.len();
                    self.digital_traces.push(DigitalTrace {
                        node_name: node_name.clone(),
                        points: Vec::new(),
                    });
                    trace_indices.insert(node_id, index);
                    index
                }
            };

            let trace = &mut self.digital_traces[trace_idx];
            if trace
                .points
                .last()
                .is_some_and(|point| point.time == time && point.value == value)
            {
                continue;
            }
            if trace
                .points
                .last()
                .is_some_and(|point| point.value == value)
            {
                continue;
            }
            trace.points.push(DigitalTracePoint { time, value });
        }
    }

    /// Get the complete digital event trace for a named XSPICE digital node.
    pub fn digital_trace_named(&self, name: &str) -> Option<&[DigitalTracePoint]> {
        self.digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| trace.points.as_slice())
    }

    /// Append committed XSPICE real event values at an accepted transient time.
    pub(crate) fn record_real_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(NodeId, Value)],
        trace_indices: &mut HashMap<NodeId, usize>,
    ) {
        for &(node_id, value) in snapshot {
            let Some(node_name) = node_id
                .checked_sub(1)
                .and_then(|index| self.node_names.get(index))
            else {
                continue;
            };
            let trace_idx = match trace_indices.get(&node_id).copied() {
                Some(index) => index,
                None => {
                    let index = self.real_traces.len();
                    self.real_traces.push(RealTrace {
                        node_name: node_name.clone(),
                        points: Vec::new(),
                    });
                    trace_indices.insert(node_id, index);
                    index
                }
            };

            let trace = &mut self.real_traces[trace_idx];
            if trace
                .points
                .last()
                .is_some_and(|point| point.time == time && point.value == value)
            {
                continue;
            }
            if trace
                .points
                .last()
                .is_some_and(|point| point.value == value)
            {
                continue;
            }
            trace.points.push(RealTracePoint { time, value });
        }
    }

    /// Get the complete real-valued event trace for a named XSPICE real node.
    pub fn real_trace_named(&self, name: &str) -> Option<&[RealTracePoint]> {
        self.real_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| trace.points.as_slice())
    }

    /// Get voltage at a node at a specific time index.
    ///
    /// Panics for invalid non-ground node IDs or time indices. Use
    /// [`Self::try_voltage_at`] for checked access.
    #[track_caller]
    pub fn voltage_at(&self, node: usize, time_index: usize) -> Value {
        if node == 0 {
            return 0.0;
        }

        self.try_voltage_at(node, time_index).unwrap_or_else(|| {
            panic!(
                "node {} / time index {} out of range for TransientResult with {} nodes and {} samples",
                node,
                time_index,
                self.num_nodes,
                self.time.len()
            )
        })
    }

    /// Get voltage at a node at a specific time index, returning `None` when
    /// the node or index is invalid.
    pub fn try_voltage_at(&self, node: usize, time_index: usize) -> Option<Value> {
        if node == 0 || node > self.num_nodes {
            return None;
        }
        self.voltages
            .get(node - 1)
            .and_then(|v| v.get(time_index))
            .copied()
    }

    /// Resolve a node name to a 1-based node index, treating common ground
    /// aliases as node 0.
    pub fn node_index_named(&self, name: &str) -> Option<usize> {
        if Self::is_ground_name(name) {
            return Some(0);
        }

        self.node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .map(|idx| idx + 1)
    }

    /// Get the complete voltage waveform for a node.
    ///
    /// Panics when `node` is invalid. Use [`Self::try_voltage_waveform`] for
    /// checked access.
    #[track_caller]
    pub fn voltage_waveform(&self, node: usize) -> &[Value] {
        self.try_voltage_waveform(node).unwrap_or_else(|| {
            panic!(
                "node {} out of range for TransientResult with {} nodes",
                node, self.num_nodes
            )
        })
    }

    /// Get the complete voltage waveform for a node, returning `None` when the
    /// node is invalid.
    pub fn try_voltage_waveform(&self, node: usize) -> Option<&[Value]> {
        if node == 0 || node > self.num_nodes {
            return None;
        }
        self.voltages.get(node - 1).map(|v| v.as_slice())
    }

    /// Get the complete voltage waveform for a named node, returning `None`
    /// when the name does not resolve to a non-ground node.
    pub fn try_voltage_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let node = self.node_index_named(name)?;
        if node == 0 {
            return None;
        }
        self.try_voltage_waveform(node)
    }

    /// Get the complete current waveform for a named branch, returning `None`
    /// when the branch name does not resolve.
    pub fn try_branch_current_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let branch_idx = self
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        self.branch_currents
            .get(branch_idx)
            .map(|waveform| waveform.as_slice())
    }

    /// Get the complete operating-point parameter waveform for a named device.
    pub fn try_device_op_waveform_named(
        &self,
        device_name: &str,
        parameter: &str,
    ) -> Option<&[Value]> {
        self.device_op_traces
            .iter()
            .find(|trace| {
                trace.device_name.eq_ignore_ascii_case(device_name)
                    && trace.parameter.eq_ignore_ascii_case(parameter)
            })
            .map(|trace| trace.values.as_slice())
    }

    /// Get a typed device-store waveform by its canonical store name.
    pub fn try_store_waveform_named(&self, name: &str) -> Option<&[Value]> {
        self.store_traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case(name))
            .map(|trace| trace.values.as_slice())
    }

    /// Get voltage at a named node and time index, returning `None` when the
    /// name or time index is invalid.
    pub fn try_voltage_at_named(&self, name: &str, time_index: usize) -> Option<Value> {
        let node = self.node_index_named(name)?;
        if node == 0 {
            return self.time.get(time_index).map(|_| 0.0);
        }
        self.try_voltage_at(node, time_index)
    }

    /// Get number of time points
    pub fn num_points(&self) -> usize {
        self.time.len()
    }

    fn is_ground_name(name: &str) -> bool {
        crate::compat::ground::is_spice_ground_name(name)
    }
}

impl From<TransientResultCompressed> for TransientResult {
    fn from(compressed: TransientResultCompressed) -> Self {
        let node_names = if compressed.node_names.len() == compressed.num_nodes {
            compressed.node_names
        } else {
            (1..=compressed.num_nodes).map(|i| i.to_string()).collect()
        };
        Self {
            time: compressed.time,
            voltages: compressed.voltages,
            branch_currents: Vec::new(),
            num_nodes: compressed.num_nodes,
            node_names,
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: compressed.store_traces,
        }
    }
}
