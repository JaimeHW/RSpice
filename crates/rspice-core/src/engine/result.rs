//! Public time-domain result types.

use crate::Value;
use crate::analysis::waveform::TransientResultCompressed;
use crate::xspice::DigitalValue;

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
}

impl TransientResult {
    /// Append committed XSPICE digital values at an accepted transient time.
    pub(crate) fn record_digital_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(String, DigitalValue)],
    ) {
        for (node_name, value) in snapshot {
            let trace_idx = self
                .digital_traces
                .iter()
                .position(|trace| trace.node_name.eq_ignore_ascii_case(node_name))
                .unwrap_or_else(|| {
                    self.digital_traces.push(DigitalTrace {
                        node_name: node_name.clone(),
                        points: Vec::new(),
                    });
                    self.digital_traces.len() - 1
                });

            let trace = &mut self.digital_traces[trace_idx];
            if trace
                .points
                .last()
                .is_some_and(|point| point.time == time && point.value == *value)
            {
                continue;
            }
            if trace
                .points
                .last()
                .is_some_and(|point| point.value == *value)
            {
                continue;
            }
            trace.points.push(DigitalTracePoint {
                time,
                value: *value,
            });
        }
    }

    /// Get the complete digital event trace for a named XSPICE digital node.
    pub fn digital_trace_named(&self, name: &str) -> Option<&[DigitalTracePoint]> {
        self.digital_traces
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
        matches!(name, "0") || name.eq_ignore_ascii_case("gnd")
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
        }
    }
}
