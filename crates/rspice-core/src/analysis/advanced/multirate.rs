//! Multi-Rate Transient Analysis
//!
//! Efficient simulation of circuits with widely varying time constants,

//! critical for mixed-signal designs where slow analog signals interact
//! with fast digital or RF blocks.
//!
//! # Theory
//!
//! Multi-rate methods partition the circuit into subcircuits with different
//! natural frequencies and simulate each with an appropriate timestep:
//!
//! - **Fast blocks**: RF oscillators, PLLs, switching regulators
//! - **Slow blocks**: Control loops, thermal dynamics, bias circuits
//!
//! The blocks are synchronized at interface nodes using:
//! - Latency (delayed coupling)
//! - Waveform relaxation
//! - Subcycling within larger timesteps
//!
//! # Algorithm
//!
//! 1. Partition circuit based on activity/bandwidth
//! 2. Assign each partition a rate class
//! 3. Simulate fast partitions with small timesteps (subcycling)
//! 4. Synchronize at interface points
//! 5. Advance slow partitions with larger timesteps

use crate::Value;
use std::collections::HashMap;

//=============================================================================
// Rate Class
//=============================================================================

/// Rate class for circuit partitioning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RateClass {
    /// Very fast signals (RF, clock, switching)
    Fast,
    /// Normal speed signals (general analog)
    #[default]
    Normal,
    /// Slow signals (DC bias, thermal, control loops)
    Slow,
    /// Very slow signals (temperature drift, aging)
    VerySlow,
}

impl RateClass {
    /// Get typical timestep multiplier relative to base timestep
    pub fn timestep_multiplier(&self) -> Value {
        match self {
            RateClass::Fast => 0.1,
            RateClass::Normal => 1.0,
            RateClass::Slow => 10.0,
            RateClass::VerySlow => 100.0,
        }
    }

    /// Get typical bandwidth for this rate class
    pub fn bandwidth_threshold(&self) -> Value {
        match self {
            RateClass::Fast => 1e9,     // > 1 GHz
            RateClass::Normal => 100e6, // 100 MHz - 1 GHz
            RateClass::Slow => 1e6,     // 1 MHz - 100 MHz
            RateClass::VerySlow => 1e3, // < 1 MHz
        }
    }
}

//=============================================================================
// Circuit Partition
//=============================================================================

/// A partition of the circuit with its own rate class
#[derive(Debug, Clone)]
pub struct CircuitPartition {
    /// Partition identifier
    pub id: usize,
    /// Rate class for this partition
    pub rate_class: RateClass,
    /// Node indices belonging to this partition
    pub nodes: Vec<usize>,
    /// Interface nodes (shared with other partitions)
    pub interface_nodes: Vec<usize>,
    /// Current timestep for this partition
    pub timestep: Value,
    /// Steps taken since last synchronization
    pub subcycle_count: usize,
    /// Latency delay (time offset from global time)
    pub latency: Value,
}

impl CircuitPartition {
    /// Create new partition
    pub fn new(id: usize, rate_class: RateClass) -> Self {
        Self {
            id,
            rate_class,
            nodes: Vec::new(),
            interface_nodes: Vec::new(),
            timestep: 1e-9 * rate_class.timestep_multiplier(),
            subcycle_count: 0,
            latency: 0.0,
        }
    }

    /// Add a node to this partition
    pub fn add_node(&mut self, node: usize) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    /// Mark a node as an interface node
    pub fn add_interface_node(&mut self, node: usize) {
        if !self.interface_nodes.contains(&node) {
            self.interface_nodes.push(node);
        }
    }

    /// Check if this partition needs to synchronize
    pub fn needs_sync(&self, global_time: Value, sync_interval: Value) -> bool {
        let local_time = global_time - self.latency;
        (local_time / sync_interval).fract() < 1e-12
    }

    /// Number of subcycles to reach sync point
    pub fn subcycles_to_sync(&self, sync_interval: Value) -> usize {
        (sync_interval / self.timestep).ceil() as usize
    }
}

//=============================================================================
// Multi-Rate Configuration
//=============================================================================

/// Configuration for multi-rate transient analysis
#[derive(Debug, Clone)]
pub struct MultiRateConfig {
    /// Base timestep (for Normal rate class)
    pub base_timestep: Value,
    /// Maximum timestep (absolute limit)
    pub max_timestep: Value,
    /// Minimum timestep (absolute limit)
    pub min_timestep: Value,
    /// Synchronization interval (global time points where all partitions sync)
    pub sync_interval: Value,
    /// Start time
    pub t_start: Value,
    /// Stop time
    pub t_stop: Value,
    /// Relative tolerance
    pub rel_tol: Value,
    /// Absolute tolerance
    pub abs_tol: Value,
    /// Maximum subcycles per sync interval
    pub max_subcycles: usize,
    /// Enable automatic partitioning
    pub auto_partition: bool,
    /// Maximum latency allowed (affects accuracy)
    pub max_latency: Value,
}

impl Default for MultiRateConfig {
    fn default() -> Self {
        Self {
            base_timestep: 1e-9,
            max_timestep: 1e-6,
            min_timestep: 1e-12,
            sync_interval: 10e-9,
            t_start: 0.0,
            t_stop: 1e-6,
            rel_tol: 1e-3,
            abs_tol: 1e-9,
            max_subcycles: 1000,
            auto_partition: true,
            max_latency: 10e-9,
        }
    }
}

impl MultiRateConfig {
    /// Create config for specific application
    pub fn for_mixed_signal(digital_freq: Value, analog_bw: Value) -> Self {
        let digital_period = 1.0 / digital_freq;
        let analog_timestep = 1.0 / (10.0 * analog_bw);

        Self {
            base_timestep: analog_timestep,
            min_timestep: digital_period / 100.0,
            sync_interval: digital_period * 10.0,
            ..Default::default()
        }
    }

    /// Set time range
    pub fn with_time_range(mut self, t_start: Value, t_stop: Value) -> Self {
        self.t_start = t_start;
        self.t_stop = t_stop;
        self
    }

    /// Set tolerances
    pub fn with_tolerances(mut self, rel_tol: Value, abs_tol: Value) -> Self {
        self.rel_tol = rel_tol;
        self.abs_tol = abs_tol;
        self
    }
}

//=============================================================================
// Interface History
//=============================================================================

/// Recorded history at interface nodes for interpolation
#[derive(Debug, Clone)]
pub struct InterfaceHistory {
    /// Node index
    pub node: usize,
    /// Time points
    pub time: Vec<Value>,
    /// Voltage values at time points
    pub voltage: Vec<Value>,
    /// Maximum history length
    pub max_length: usize,
}

impl InterfaceHistory {
    /// Create new interface history
    pub fn new(node: usize, max_length: usize) -> Self {
        Self {
            node,
            time: Vec::with_capacity(max_length),
            voltage: Vec::with_capacity(max_length),
            max_length,
        }
    }

    /// Record a new value
    pub fn record(&mut self, t: Value, v: Value) {
        if self.time.len() >= self.max_length {
            self.time.remove(0);
            self.voltage.remove(0);
        }
        self.time.push(t);
        self.voltage.push(v);
    }

    /// Interpolate voltage at time t
    pub fn interpolate(&self, t: Value) -> Option<Value> {
        if self.time.is_empty() {
            return None;
        }

        // Clamp to available range
        if t <= self.time[0] {
            return Some(self.voltage[0]);
        }
        if t >= *self.time.last().unwrap() {
            return Some(*self.voltage.last().unwrap());
        }

        // Find interval
        for i in 0..self.time.len() - 1 {
            if self.time[i] <= t && t < self.time[i + 1] {
                let alpha = (t - self.time[i]) / (self.time[i + 1] - self.time[i]);
                return Some(self.voltage[i] * (1.0 - alpha) + self.voltage[i + 1] * alpha);
            }
        }

        Some(*self.voltage.last().unwrap())
    }

    /// Get most recent value
    pub fn latest(&self) -> Option<Value> {
        self.voltage.last().copied()
    }

    /// Get most recent time
    pub fn latest_time(&self) -> Option<Value> {
        self.time.last().copied()
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.time.clear();
        self.voltage.clear();
    }
}

//=============================================================================
// Multi-Rate State
//=============================================================================

/// State of multi-rate simulation
#[derive(Debug, Clone)]
pub struct MultiRateState {
    /// Current global time
    pub time: Value,
    /// State for each partition
    pub partition_states: Vec<PartitionState>,
    /// Interface histories
    pub interface_histories: HashMap<usize, InterfaceHistory>,
    /// Total simulation steps
    pub total_steps: usize,
    /// Synchronization points reached
    pub sync_points: usize,
    /// Rejected steps (across all partitions)
    pub rejected_steps: usize,
}

/// State for a single partition
#[derive(Debug, Clone)]
pub struct PartitionState {
    /// Partition ID
    pub partition_id: usize,
    /// Local time (may differ from global due to latency)
    pub local_time: Value,
    /// Current solution vector for this partition's nodes
    pub solution: Vec<Value>,
    /// Previous solution (for LTE estimation)
    pub prev_solution: Vec<Value>,
    /// Current timestep
    pub timestep: Value,
    /// Steps taken
    pub steps: usize,
    /// Subcycles since last sync
    pub subcycles: usize,
}

impl PartitionState {
    /// Create new partition state
    pub fn new(partition_id: usize, num_nodes: usize) -> Self {
        Self {
            partition_id,
            local_time: 0.0,
            solution: vec![0.0; num_nodes],
            prev_solution: vec![0.0; num_nodes],
            timestep: 1e-9,
            steps: 0,
            subcycles: 0,
        }
    }

    /// Accept a step
    pub fn accept_step(&mut self, new_solution: Vec<Value>, dt: Value) {
        self.prev_solution = std::mem::replace(&mut self.solution, new_solution);
        self.local_time += dt;
        self.timestep = dt;
        self.steps += 1;
        self.subcycles += 1;
    }

    /// Reset subcycle counter (at sync points)
    pub fn reset_subcycles(&mut self) {
        self.subcycles = 0;
    }
}

impl MultiRateState {
    /// Create new state
    pub fn new(partitions: &[CircuitPartition]) -> Self {
        let partition_states = partitions
            .iter()
            .map(|p| PartitionState::new(p.id, p.nodes.len()))
            .collect();

        let mut interface_histories = HashMap::new();
        for partition in partitions {
            for &node in &partition.interface_nodes {
                interface_histories
                    .entry(node)
                    .or_insert_with(|| InterfaceHistory::new(node, 100));
            }
        }

        Self {
            time: 0.0,
            partition_states,
            interface_histories,
            total_steps: 0,
            sync_points: 0,
            rejected_steps: 0,
        }
    }

    /// Advance global time to sync point
    pub fn advance_to_sync(&mut self, sync_time: Value) {
        self.time = sync_time;
        self.sync_points += 1;
        for ps in &mut self.partition_states {
            ps.reset_subcycles();
        }
    }

    /// Record interface value
    pub fn record_interface(&mut self, node: usize, t: Value, v: Value) {
        if let Some(history) = self.interface_histories.get_mut(&node) {
            history.record(t, v);
        }
    }

    /// Get interpolated interface value
    pub fn get_interface(&self, node: usize, t: Value) -> Option<Value> {
        self.interface_histories.get(&node)?.interpolate(t)
    }
}

//=============================================================================
// Multi-Rate Result
//=============================================================================

/// Result of multi-rate transient simulation
#[derive(Debug, Clone)]
pub struct MultiRateResult {
    /// Configuration used
    pub config: MultiRateConfig,
    /// Time points (global synchronization times)
    pub time: Vec<Value>,
    /// Solution at each sync point (node_idx -> voltage vector)
    pub voltages: HashMap<usize, Vec<Value>>,
    /// Partition timing statistics
    pub partition_stats: Vec<PartitionStats>,
    /// Total simulation time (wall clock seconds)
    pub simulation_time_seconds: Value,
    /// Speedup factor vs single-rate
    pub speedup_factor: Value,
}

/// Statistics for a partition
#[derive(Debug, Clone)]
pub struct PartitionStats {
    /// Partition ID
    pub partition_id: usize,
    /// Rate class
    pub rate_class: RateClass,
    /// Average timestep used
    pub avg_timestep: Value,
    /// Total steps taken
    pub total_steps: usize,
    /// Number of subcycles
    pub total_subcycles: usize,
}

impl MultiRateResult {
    /// Create new result
    pub fn new(config: MultiRateConfig) -> Self {
        Self {
            config,
            time: Vec::new(),
            voltages: HashMap::new(),
            partition_stats: Vec::new(),
            simulation_time_seconds: 0.0,
            speedup_factor: 1.0,
        }
    }

    /// Record a sync point
    pub fn record_sync_point(&mut self, t: Value, node_voltages: &[(usize, Value)]) {
        self.time.push(t);
        for &(node, voltage) in node_voltages {
            self.voltages.entry(node).or_default().push(voltage);
        }
    }

    /// Get voltage for a node
    pub fn get_voltage(&self, node: usize) -> Option<&[Value]> {
        self.voltages.get(&node).map(|v| v.as_slice())
    }

    /// Compute speedup estimate
    pub fn compute_speedup(&mut self) {
        let duration = self.config.t_stop - self.config.t_start;

        // Single-rate would need smallest timestep throughout
        let single_rate_steps = (duration / self.config.min_timestep) as usize;

        // Multi-rate total steps
        let multi_rate_steps: usize = self.partition_stats.iter().map(|p| p.total_steps).sum();

        if multi_rate_steps > 0 {
            self.speedup_factor = single_rate_steps as Value / multi_rate_steps as Value;
        }
    }
}

//=============================================================================
// Multi-Rate Solver
//=============================================================================

/// Multi-rate transient solver
#[derive(Debug)]
pub struct MultiRateSolver {
    /// Configuration
    config: MultiRateConfig,
    /// Circuit partitions
    partitions: Vec<CircuitPartition>,
    /// Current state
    state: MultiRateState,
    /// Recorded results
    result: MultiRateResult,
}

impl MultiRateSolver {
    /// Create new multi-rate solver
    pub fn new(config: MultiRateConfig, partitions: Vec<CircuitPartition>) -> Self {
        let state = MultiRateState::new(&partitions);
        let result = MultiRateResult::new(config.clone());

        Self {
            config,
            partitions,
            state,
            result,
        }
    }

    /// Create with automatic single partition (for compatibility)
    pub fn single_rate(config: MultiRateConfig, num_nodes: usize) -> Self {
        let mut partition = CircuitPartition::new(0, RateClass::Normal);
        for i in 0..num_nodes {
            partition.add_node(i);
        }
        Self::new(config, vec![partition])
    }

    /// Get current global time
    pub fn current_time(&self) -> Value {
        self.state.time
    }

    /// Check if simulation is complete
    pub fn is_complete(&self) -> bool {
        self.state.time >= self.config.t_stop
    }

    /// Perform subcycling for a single partition
    ///
    /// # Arguments
    /// * `partition_idx` - Which partition to step
    /// * `compute_derivative` - Function computing dv/dt for nodes
    ///
    /// # Returns
    /// Number of subcycles taken
    pub fn subcycle_partition<F>(&mut self, partition_idx: usize, compute_derivative: F) -> usize
    where
        F: Fn(&[Value], Value) -> Vec<Value>,
    {
        let target_time = self.state.time + self.config.sync_interval;
        let max_subcycles = self.config.max_subcycles;
        let interface_nodes: Vec<usize> = self.partitions[partition_idx].interface_nodes.clone();
        let partition_nodes: Vec<usize> = self.partitions[partition_idx].nodes.clone();
        let mut subcycles = 0;

        while self.state.partition_states[partition_idx].local_time < target_time
            && subcycles < max_subcycles
        {
            let ps = &mut self.state.partition_states[partition_idx];
            let dt = ps.timestep.min(target_time - ps.local_time);

            if dt <= 0.0 {
                break;
            }

            // Compute derivative
            let deriv = compute_derivative(&ps.solution, ps.local_time);

            // Forward Euler step (could be enhanced with better integration)
            let new_solution: Vec<Value> = ps
                .solution
                .iter()
                .zip(deriv.iter())
                .map(|(&v, &dv)| v + dv * dt)
                .collect();

            // Accept step (simplified - real implementation would have error control)
            ps.accept_step(new_solution, dt);
            subcycles += 1;

            // Collect interface values to record (avoid borrowing ps while accessing self.state)
            let local_time = self.state.partition_states[partition_idx].local_time;
            let mut interface_updates: Vec<(usize, Value)> = Vec::new();
            for &node in &interface_nodes {
                if let Some(local_idx) = partition_nodes.iter().position(|&n| n == node) {
                    let voltage = self.state.partition_states[partition_idx].solution[local_idx];
                    interface_updates.push((node, voltage));
                }
            }

            // Now record interface values
            for (node, voltage) in interface_updates {
                self.state.record_interface(node, local_time, voltage);
            }
        }

        self.state.total_steps += subcycles;
        subcycles
    }

    /// Perform one synchronization step
    ///
    /// # Arguments
    /// * `compute_derivatives` - Functions computing dv/dt for each partition
    pub fn sync_step<F>(&mut self, compute_derivatives: &[F])
    where
        F: Fn(&[Value], Value) -> Vec<Value>,
    {
        // Subcycle each partition
        for (i, f) in compute_derivatives.iter().enumerate() {
            if i < self.partitions.len() {
                self.subcycle_partition(i, f);
            }
        }

        // Advance to sync point
        let sync_time = self.state.time + self.config.sync_interval;
        self.state.advance_to_sync(sync_time);

        // Record result at sync point
        let mut node_voltages = Vec::new();
        for (p_idx, partition) in self.partitions.iter().enumerate() {
            for (local_idx, &node) in partition.nodes.iter().enumerate() {
                let voltage = self.state.partition_states[p_idx].solution[local_idx];
                node_voltages.push((node, voltage));
            }
        }
        self.result.record_sync_point(sync_time, &node_voltages);
    }

    /// Run full simulation
    pub fn run<F>(&mut self, compute_derivatives: &[F]) -> MultiRateResult
    where
        F: Fn(&[Value], Value) -> Vec<Value>,
    {
        // Record initial state
        let mut initial_voltages = Vec::new();
        for (p_idx, partition) in self.partitions.iter().enumerate() {
            for (local_idx, &node) in partition.nodes.iter().enumerate() {
                let voltage = self.state.partition_states[p_idx].solution[local_idx];
                initial_voltages.push((node, voltage));
            }
        }
        self.result
            .record_sync_point(self.state.time, &initial_voltages);

        // Main simulation loop
        while !self.is_complete() {
            self.sync_step(compute_derivatives);
        }

        // Compute statistics
        for (i, partition) in self.partitions.iter().enumerate() {
            let ps = &self.state.partition_states[i];
            self.result.partition_stats.push(PartitionStats {
                partition_id: partition.id,
                rate_class: partition.rate_class,
                avg_timestep: (self.state.time - self.config.t_start) / ps.steps as Value,
                total_steps: ps.steps,
                total_subcycles: ps.subcycles,
            });
        }

        self.result.compute_speedup();
        self.result.clone()
    }

    /// Get state reference
    pub fn state(&self) -> &MultiRateState {
        &self.state
    }

    /// Get partition info
    pub fn partitions(&self) -> &[CircuitPartition] {
        &self.partitions
    }
}

//=============================================================================
// Activity Detection for Auto-Partitioning
//=============================================================================

/// Activity metrics for a node (used for automatic partitioning)
#[derive(Debug, Clone)]
pub struct NodeActivity {
    /// Node index
    pub node: usize,
    /// Estimated bandwidth (frequency of activity)
    pub bandwidth: Value,
    /// RMS slew rate
    pub slew_rate: Value,
    /// DC bias level
    pub dc_level: Value,
    /// Suggested rate class
    pub rate_class: RateClass,
}

impl NodeActivity {
    /// Analyze waveform to determine activity
    pub fn from_waveform(node: usize, time: &[Value], values: &[Value]) -> Self {
        if time.len() < 2 || values.len() < 2 {
            return Self {
                node,
                bandwidth: 0.0,
                slew_rate: 0.0,
                dc_level: values.first().copied().unwrap_or(0.0),
                rate_class: RateClass::VerySlow,
            };
        }

        // Compute DC level
        let dc_level = values.iter().sum::<Value>() / values.len() as Value;

        // Compute slew rate
        let mut max_slew = 0.0_f64;
        for i in 1..time.len() {
            let dt = time[i] - time[i - 1];
            if dt > 0.0 {
                let slew = ((values[i] - values[i - 1]) / dt).abs();
                max_slew = max_slew.max(slew);
            }
        }

        // Estimate bandwidth from slew rate and signal amplitude
        let amplitude = values
            .iter()
            .fold(0.0_f64, |max, &v| max.max((v - dc_level).abs()));
        let bandwidth = if amplitude > 1e-12 {
            max_slew / (2.0 * std::f64::consts::PI * amplitude)
        } else {
            0.0
        };

        // Determine rate class (boundaries are inclusive for faster class)
        let rate_class = if bandwidth >= 1e9 {
            RateClass::Fast
        } else if bandwidth >= 100e6 {
            RateClass::Normal
        } else if bandwidth >= 1e6 {
            RateClass::Slow
        } else {
            RateClass::VerySlow
        };

        Self {
            node,
            bandwidth,
            slew_rate: max_slew,
            dc_level,
            rate_class,
        }
    }

    pub fn with_bandwidth(node: usize, bandwidth: Value) -> Self {
        // Boundaries are inclusive for faster class (commercial-grade behavior)
        let rate_class = if bandwidth >= 1e9 {
            RateClass::Fast
        } else if bandwidth >= 100e6 {
            RateClass::Normal
        } else if bandwidth >= 1e6 {
            RateClass::Slow
        } else {
            RateClass::VerySlow
        };

        Self {
            node,
            bandwidth,
            slew_rate: 0.0,
            dc_level: 0.0,
            rate_class,
        }
    }
}

/// Automatic partitioner based on activity analysis
pub struct AutoPartitioner;

impl AutoPartitioner {
    /// Create partitions from node activities
    pub fn partition(activities: &[NodeActivity]) -> Vec<CircuitPartition> {
        let mut partitions: HashMap<RateClass, CircuitPartition> = HashMap::new();
        let mut next_id = 0;

        for activity in activities {
            let partition = partitions.entry(activity.rate_class).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                CircuitPartition::new(id, activity.rate_class)
            });
            partition.add_node(activity.node);
        }

        // Convert to vec, sorted by rate class (fastest first)
        let mut result: Vec<_> = partitions.into_values().collect();
        result.sort_by_key(|p| match p.rate_class {
            RateClass::Fast => 0,
            RateClass::Normal => 1,
            RateClass::Slow => 2,
            RateClass::VerySlow => 3,
        });

        result
    }
}

//=============================================================================
// Tests
//=============================================================================
