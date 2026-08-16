//! Public time-domain result types.

use crate::engine::waveform::TransientResultCompressed;
use crate::xspice::DigitalValue;
use crate::{NodeId, Value};
use std::collections::HashMap;

/// Exact sample selection used by transient output writers.
///
/// The solver result always retains its complete accepted history. This view
/// selects output rows without changing integration history, measurements,
/// Fourier analysis, checkpoint continuation, or compression inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientOutputProjection {
    source_len: usize,
    indices: Vec<usize>,
}

impl TransientOutputProjection {
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    pub fn project(&self, values: &[Value]) -> Result<Vec<Value>, String> {
        if values.len() != self.source_len {
            return Err(format!(
                "transient output series has {} samples, expected {}",
                values.len(),
                self.source_len
            ));
        }
        Ok(self.indices.iter().map(|index| values[*index]).collect())
    }
}

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
    /// Accepted integration interval for each time point. The initial sample
    /// at `time[0]` has a zero interval; subsequent entries are the exact
    /// timestep used to produce the corresponding sample. Keeping this
    /// alongside the rounded absolute times makes deterministic grid replay
    /// preserve the producing run's integration coefficients.
    pub step_sizes: Vec<Value>,
    /// Voltage waveforms, indexed `[node_index][time_index]`. A waveform is
    /// empty when output projection deliberately did not retain that node;
    /// every non-empty waveform is aligned with `time`.
    pub voltages: Vec<Vec<Value>>,
    /// Branch current waveforms, indexed `[branch_index][time_index]`. A
    /// waveform is empty when output projection deliberately did not retain
    /// that branch; every non-empty waveform is aligned with `time`.
    pub branch_currents: Vec<Vec<Value>>,
    /// Number of nodes
    pub num_nodes: usize,
    /// Node names from the netlist (maps index to original name like "N001", "out", etc.)
    /// Index 0 corresponds to `voltages[0]`, which is node 1 (not ground)
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
    /// Borrow the columns an abort signal's sample hook is allowed to see.
    ///
    /// The hook sits at the bottom of the crate and must not name a driver
    /// result type, so the driver narrows itself on the way out: waveform
    /// columns, their names, and the accepted times, and nothing else.
    pub(crate) fn observable_sample(&self) -> crate::abort_signal::TransientSample<'_> {
        crate::abort_signal::TransientSample {
            time: &self.time,
            node_names: &self.node_names,
            node_voltages: &self.voltages,
            branch_names: &self.branch_names,
            branch_currents: &self.branch_currents,
        }
    }

    /// Build the Xyce transient output view for a requested schedule.
    ///
    /// With no `OUTPUTTIMEPOINTS`, every accepted sample at or after TSTART
    /// is selected. With a schedule, only exact accepted requested points in
    /// the output horizon plus TSTOP are selected. Requested points are solver
    /// breakpoints, so a missing exact point is a scheduling error rather than
    /// an invitation to interpolate.
    pub fn output_projection(
        &self,
        output_time_points: &[Value],
        start_time: Value,
        stop_time: Value,
    ) -> Result<TransientOutputProjection, String> {
        if !start_time.is_finite()
            || !stop_time.is_finite()
            || start_time < 0.0
            || stop_time < start_time
        {
            return Err(format!(
                "invalid transient output window [{start_time}, {stop_time}]"
            ));
        }
        if self.time.is_empty() {
            return Err("transient result has no accepted samples".to_string());
        }
        if let Some(invalid) = output_time_points
            .iter()
            .find(|time| !time.is_finite() || **time < 0.0)
        {
            return Err(format!(
                "transient output schedule contains invalid time {invalid}"
            ));
        }
        for (index, &time) in self.time.iter().enumerate() {
            if !time.is_finite() {
                return Err(format!("transient result time[{index}] is not finite"));
            }
            if index > 0 && time <= self.time[index - 1] {
                return Err(format!(
                    "transient result time grid is not strictly increasing at index {index}"
                ));
            }
        }

        let indices = if output_time_points.is_empty() {
            let first = self.time.partition_point(|time| *time < start_time);
            let last = self.time.partition_point(|time| *time <= stop_time);
            (first..last).collect()
        } else {
            let mut requested = output_time_points
                .iter()
                .copied()
                .filter(|time| *time >= start_time && *time <= stop_time)
                .collect::<Vec<_>>();
            requested.push(stop_time);
            requested.sort_by(Value::total_cmp);
            requested.dedup_by(|left, right| left.to_bits() == right.to_bits());

            let mut indices = Vec::with_capacity(requested.len());
            for requested_time in requested {
                let index = self
                    .time
                    .binary_search_by(|time| time.total_cmp(&requested_time))
                    .map_err(|_| {
                        format!(
                            "transient result did not land on requested output time {requested_time}"
                        )
                    })?;
                indices.push(index);
            }
            indices
        };

        if indices.is_empty() {
            return Err("transient output projection selected no samples".to_string());
        }
        Ok(TransientOutputProjection {
            source_len: self.time.len(),
            indices,
        })
    }

    /// Append one accepted device operating-point snapshot. Missing parameters
    /// are padded with NaN so every stored trace remains aligned with `time`.
    pub(crate) fn record_device_op_sample(
        &mut self,
        report: crate::circuit::DeviceOpReport,
    ) -> usize {
        if self.time.is_empty() {
            return 0;
        }

        let sample_index = self.time.len() - 1;
        let mut seen = vec![false; self.device_op_traces.len()];
        let mut added_values = 0usize;

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
                        added_values = added_values.saturating_add(sample_index);
                        seen.push(false);
                        trace_index
                    });

                let trace = &mut self.device_op_traces[trace_index];
                if trace.values.len() < sample_index {
                    added_values = added_values.saturating_add(sample_index - trace.values.len());
                    trace.values.resize(sample_index, Value::NAN);
                }
                if trace.values.len() == sample_index {
                    trace.values.push(value);
                    added_values = added_values.saturating_add(1);
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
                added_values = added_values.saturating_add(sample_index - trace.values.len());
                trace.values.resize(sample_index, Value::NAN);
            }
            if trace.values.len() == sample_index {
                trace.values.push(Value::NAN);
                added_values = added_values.saturating_add(1);
            }
        }
        added_values
    }

    /// Append committed XSPICE digital values at an accepted transient time.
    pub(crate) fn record_digital_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(NodeId, DigitalValue)],
        trace_indices: &mut HashMap<NodeId, usize>,
        retained_nodes: &[bool],
    ) -> usize {
        let mut added_values = 0usize;
        for &(node_id, value) in snapshot {
            let Some(node_index) = node_id.checked_sub(1) else {
                continue;
            };
            if !retained_nodes.get(node_index).copied().unwrap_or(false) {
                continue;
            }
            let Some(node_name) = self.node_names.get(node_index) else {
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
            added_values = added_values.saturating_add(2);
        }
        added_values
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
        retained_nodes: &[bool],
    ) -> usize {
        let mut added_values = 0usize;
        for &(node_id, value) in snapshot {
            let Some(node_index) = node_id.checked_sub(1) else {
                continue;
            };
            if !retained_nodes.get(node_index).copied().unwrap_or(false) {
                continue;
            }
            let Some(node_name) = self.node_names.get(node_index) else {
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
            added_values = added_values.saturating_add(2);
        }
        added_values
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
        crate::naming::is_spice_ground_name(name)
    }
}

impl From<TransientResultCompressed> for TransientResult {
    fn from(compressed: TransientResultCompressed) -> Self {
        let node_names = if compressed.node_names.len() == compressed.num_nodes {
            compressed.node_names
        } else {
            (1..=compressed.num_nodes).map(|i| i.to_string()).collect()
        };
        let step_size_count = compressed.time.len();
        Self {
            time: compressed.time,
            step_sizes: vec![0.0; step_size_count],
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

#[cfg(test)]
mod tests {
    use super::*;

    fn result_on_grid(time: Vec<Value>) -> TransientResult {
        let values = time.iter().map(|time| 2.0 * time).collect::<Vec<_>>();
        TransientResult {
            step_sizes: vec![0.0; time.len()],
            time,
            voltages: vec![values],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        }
    }

    #[test]
    fn transient_output_projection_preserves_full_history_and_selects_schedule() {
        let result = result_on_grid(vec![0.0, 0.5, 1.0, 1.5, 2.0]);

        let all = result
            .output_projection(&[], 0.5, 1.5)
            .expect("ordinary output projection succeeds");
        assert_eq!(all.indices(), &[1, 2, 3]);

        let scheduled = result
            .output_projection(&[0.0, 1.0], 0.5, 2.0)
            .expect("scheduled output projection succeeds");
        assert_eq!(scheduled.indices(), &[2, 4]);
        assert_eq!(
            scheduled
                .project(&result.voltages[0])
                .expect("aligned waveform projects"),
            vec![2.0, 4.0]
        );
        assert_eq!(result.time.len(), 5, "projection must not mutate history");
    }

    #[test]
    fn transient_output_projection_requires_exact_accepted_schedule_points() {
        let result = result_on_grid(vec![0.0, 1.0, 2.0]);
        let error = result
            .output_projection(&[1.5], 0.0, 2.0)
            .expect_err("output points are solver stops, not interpolation requests");
        assert!(error.contains("did not land"));

        let projection = result
            .output_projection(&[0.0, 2.0], 0.0, 2.0)
            .expect("explicit zero and deduplicated stop are valid");
        assert_eq!(projection.indices(), &[0, 2]);
        assert!(projection.project(&[1.0]).is_err());
    }

    #[test]
    fn transient_output_projection_rejects_invalid_result_grids() {
        for time in [vec![], vec![0.0, 0.0], vec![0.0, Value::NAN]] {
            let result = result_on_grid(time);
            assert!(result.output_projection(&[], 0.0, 1.0).is_err());
        }

        let result = result_on_grid(vec![0.0, 1.0]);
        for schedule in [vec![-1.0], vec![Value::NAN], vec![Value::INFINITY]] {
            assert!(result.output_projection(&schedule, 0.0, 1.0).is_err());
        }
    }
}
