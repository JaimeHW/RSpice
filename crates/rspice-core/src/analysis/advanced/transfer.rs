//! Transfer Function Analysis (.TF)
//!
//! Computes DC small-signal transfer characteristics:
//! - Transfer gain: V(output)/V(input) or I(output)/I(input)
//! - Input impedance: Zin = V(input)/I(input)
//! - Output impedance: Zout (Thevenin equivalent at output)
//!
//! # Algorithm
//!
//! 1. Solve DC operating point to linearize nonlinear devices
//! 2. Build small-signal conductance matrix G
//! 3. **Gain**: Apply unit input, measure output
//! 4. **Zin**: Zin = Vinput / Iinput (from source current)
//! 5. **Zout**: Zero input, apply test current at output
//!
//! # Example
//!
//! ```ignore
//! .TF V(out) Vin
//! ```

#![allow(clippy::needless_range_loop)]
use crate::Value;

/// Result of transfer function analysis
#[derive(Debug, Clone)]
pub struct TransferFunctionResult {
    /// Output variable (e.g., "V(out)")
    pub output: String,
    /// Input source name (e.g., "Vin")
    pub input: String,
    /// Transfer gain (output/input ratio)
    pub gain: Value,
    /// Input impedance in Ohms
    pub input_impedance: Value,
    /// Output impedance in Ohms (Thevenin equivalent)
    pub output_impedance: Value,
}

impl TransferFunctionResult {
    /// Create a new transfer function result
    pub fn new(output: &str, input: &str, gain: Value, zin: Value, zout: Value) -> Self {
        Self {
            output: output.to_string(),
            input: input.to_string(),
            gain,
            input_impedance: zin,
            output_impedance: zout,
        }
    }

    /// Get gain in decibels
    pub fn gain_db(&self) -> Value {
        20.0 * self.gain.abs().log10()
    }
}

/// Configuration for transfer function analysis
#[derive(Debug, Clone)]
pub struct TransferFunctionConfig {
    /// Output node or variable (e.g., "out" or "V(out)")
    pub output_node: String,
    /// Reference node for output (None = ground)
    pub output_ref: Option<String>,
    /// Input source name
    pub input_source: String,
    /// Whether input is current source (vs voltage source)
    pub input_is_current: bool,
    /// Whether output is current (vs voltage)
    pub output_is_current: bool,
    /// Current measurement element (if output_is_current)
    pub output_element: Option<String>,
}

impl TransferFunctionConfig {
    /// Create config for voltage-to-voltage transfer function
    ///
    /// Example: `.TF V(out) Vin`
    pub fn voltage_gain(output_node: &str, input_source: &str) -> Self {
        Self {
            output_node: output_node.to_string(),
            output_ref: None,
            input_source: input_source.to_string(),
            input_is_current: false,
            output_is_current: false,
            output_element: None,
        }
    }

    /// Create config for voltage-to-current transfer function
    ///
    /// Example: `.TF I(Rload) Vin`
    pub fn transconductance(output_element: &str, input_source: &str) -> Self {
        Self {
            output_node: String::new(),
            output_ref: None,
            input_source: input_source.to_string(),
            input_is_current: false,
            output_is_current: true,
            output_element: Some(output_element.to_string()),
        }
    }

    /// Create config with reference node for differential output
    pub fn with_reference(mut self, ref_node: &str) -> Self {
        self.output_ref = Some(ref_node.to_string());
        self
    }
}

/// Transfer function analyzer
///
/// Operates on a linearized circuit (small-signal model at DC operating point)
pub struct TransferAnalyzer {
    /// Number of nodes (excluding ground)
    num_nodes: usize,
    /// Number of voltage source branches
    #[allow(dead_code)]
    num_branches: usize,
    /// Conductance matrix (linearized at OP)
    g_matrix: Vec<Vec<Value>>,
    /// Node name to index mapping
    node_map: std::collections::HashMap<String, usize>,
    /// Source name to branch index mapping  
    source_map: std::collections::HashMap<String, usize>,
}

impl TransferAnalyzer {
    /// Create analyzer from linearized circuit matrices
    pub fn new(
        num_nodes: usize,
        num_branches: usize,
        g_matrix: Vec<Vec<Value>>,
        node_map: std::collections::HashMap<String, usize>,
        source_map: std::collections::HashMap<String, usize>,
    ) -> Self {
        Self {
            num_nodes,
            num_branches,
            g_matrix,
            node_map,
            source_map,
        }
    }

    /// Create from a simple conductance matrix (for testing)
    pub fn from_conductance(g: Vec<Vec<Value>>) -> Self {
        let n = g.len();
        Self {
            num_nodes: n,
            num_branches: 0,
            g_matrix: g,
            node_map: std::collections::HashMap::new(),
            source_map: std::collections::HashMap::new(),
        }
    }

    /// Get node index (1-indexed externally, 0-indexed internally)
    fn node_index(&self, node: &str) -> Option<usize> {
        // Try direct lookup
        if let Some(&idx) = self.node_map.get(node) {
            return Some(idx);
        }
        // Try parsing as number
        if let Ok(n) = node.parse::<usize>()
            && n > 0
            && n <= self.num_nodes
        {
            return Some(n - 1);
        }
        None
    }

    /// Solve Gx = b using Gaussian elimination with partial pivoting
    fn solve(&self, b: &[Value]) -> Option<Vec<Value>> {
        let n = self.g_matrix.len();
        if n == 0 || b.len() != n {
            return None;
        }

        // Augmented matrix [G | b]
        let mut aug: Vec<Vec<Value>> = self
            .g_matrix
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut new_row = row.clone();
                new_row.push(bi);
                new_row
            })
            .collect();

        // Forward elimination with partial pivoting
        for k in 0..n {
            // Find pivot
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for i in (k + 1)..n {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
                    max_row = i;
                }
            }

            if max_val < 1e-15 {
                return None; // Singular matrix
            }

            // Swap rows
            if max_row != k {
                aug.swap(k, max_row);
            }

            // Eliminate column
            let pivot = aug[k][k];
            for i in (k + 1)..n {
                let factor = aug[i][k] / pivot;
                aug[i][k] = 0.0;
                for j in (k + 1)..=n {
                    aug[i][j] -= factor * aug[k][j];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Some(x)
    }

    /// Compute transfer function
    ///
    /// # Arguments
    /// * `output_node` - Output node index (0-indexed)
    /// * `ref_node` - Reference node index (None = ground)
    /// * `input_node` - Input injection node index
    ///
    /// # Returns
    /// (gain, input_impedance, output_impedance)
    pub fn analyze(
        &self,
        output_node: usize,
        ref_node: Option<usize>,
        input_node: usize,
    ) -> Option<(Value, Value, Value)> {
        let n = self.num_nodes;

        if output_node >= n || input_node >= n {
            return None;
        }

        // Step 1: Compute gain
        // Inject 1A current at input node, measure voltage at output
        let mut b_gain = vec![0.0; n];
        b_gain[input_node] = 1.0;

        let v_gain = self.solve(&b_gain)?;

        let v_out = match ref_node {
            Some(r) if r < n => v_gain[output_node] - v_gain[r],
            _ => v_gain[output_node],
        };
        let v_in = v_gain[input_node];

        // Gain = V(out) / V(in) when I(in) = 1A
        // Since we inject 1A, V(in) is the input impedance
        // and V(out) is the transimpedance
        // For voltage source input: gain = V(out)/V(in)
        let gain = if v_in.abs() > 1e-15 {
            v_out / v_in
        } else {
            0.0
        };

        // Input impedance = V(in) / I(in) = V(in) / 1A = V(in)
        let z_in = v_in;

        // Step 2: Compute output impedance
        // Zero input, inject 1A at output, measure voltage
        let mut b_out = vec![0.0; n];
        b_out[output_node] = 1.0;
        if let Some(r) = ref_node
            && r < n
        {
            b_out[r] = -1.0;
        }

        let v_out_test = self.solve(&b_out)?;

        // Output impedance = V(out) / I(out) = V(out) / 1A
        let z_out = match ref_node {
            Some(r) if r < n => v_out_test[output_node] - v_out_test[r],
            _ => v_out_test[output_node],
        };

        Some((gain, z_in, z_out))
    }

    /// Analyze with node names
    pub fn analyze_named(&self, config: &TransferFunctionConfig) -> Option<TransferFunctionResult> {
        let output_idx = self.node_index(&config.output_node)?;
        let ref_idx = config.output_ref.as_ref().and_then(|r| self.node_index(r));

        // For now, assume input source is at a single node
        // In full implementation, would look up branch index
        let input_idx = self
            .source_map
            .get(&config.input_source)
            .copied()
            .or_else(|| self.node_index(&config.input_source))?;

        let (gain, zin, zout) = self.analyze(output_idx, ref_idx, input_idx)?;

        Some(TransferFunctionResult::new(
            &config.output_node,
            &config.input_source,
            gain,
            zin,
            zout,
        ))
    }
}

//=============================================================================
// AC Transfer Function (XF Analysis)
//=============================================================================

use num_complex::Complex64;
use std::f64::consts::PI;

/// Single frequency point in AC transfer function
#[derive(Debug, Clone)]
pub struct AcTransferPoint {
    /// Frequency (Hz)
    pub frequency: Value,

    /// Complex transfer function H(jÏ‰)
    pub transfer: Complex64,

    /// Magnitude (linear)
    pub magnitude: Value,

    /// Magnitude in dB
    pub magnitude_db: Value,

    /// Phase in radians
    pub phase_rad: Value,

    /// Phase in degrees
    pub phase_deg: Value,
}

impl AcTransferPoint {
    /// Create from frequency and complex transfer function
    pub fn new(frequency: Value, transfer: Complex64) -> Self {
        let magnitude = transfer.norm();
        Self {
            frequency,
            transfer,
            magnitude,
            magnitude_db: 20.0 * magnitude.log10(),
            phase_rad: transfer.arg(),
            phase_deg: transfer.arg() * 180.0 / PI,
        }
    }

    /// Group delay contribution between this point and next
    pub fn group_delay(&self, next: &AcTransferPoint) -> Value {
        let df = next.frequency - self.frequency;
        if df.abs() < 1e-15 {
            return 0.0;
        }

        let mut dphi = next.phase_rad - self.phase_rad;
        // Unwrap phase
        while dphi > PI {
            dphi -= 2.0 * PI;
        }
        while dphi < -PI {
            dphi += 2.0 * PI;
        }

        -dphi / (2.0 * PI * df)
    }
}

/// AC Transfer Function analysis result
#[derive(Debug, Clone)]
pub struct AcTransferResult {
    /// Output node/variable name
    pub output: String,

    /// Input source name
    pub input: String,

    /// Transfer function at each frequency
    pub points: Vec<AcTransferPoint>,

    /// DC gain (if available)
    pub dc_gain: Option<Value>,

    /// DC gain in dB
    pub dc_gain_db: Option<Value>,

    /// Peak gain frequency (Hz)
    pub peak_frequency: Option<Value>,

    /// Peak gain (dB)
    pub peak_gain_db: Option<Value>,

    /// -3dB cutoff frequency (low) for bandpass
    pub cutoff_low: Option<Value>,

    /// -3dB cutoff frequency (high)
    pub cutoff_high: Option<Value>,

    /// Bandwidth (Hz)
    pub bandwidth: Option<Value>,

    /// Quality factor Q (for bandpass/resonant)
    pub q_factor: Option<Value>,

    /// Unity gain frequency (Hz)
    pub unity_gain_frequency: Option<Value>,

    /// Phase margin at unity gain (degrees)
    pub phase_margin: Option<Value>,
}

impl Default for AcTransferResult {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl AcTransferResult {
    /// Create new result
    pub fn new(output: &str, input: &str) -> Self {
        Self {
            output: output.to_string(),
            input: input.to_string(),
            points: Vec::new(),
            dc_gain: None,
            dc_gain_db: None,
            peak_frequency: None,
            peak_gain_db: None,
            cutoff_low: None,
            cutoff_high: None,
            bandwidth: None,
            q_factor: None,
            unity_gain_frequency: None,
            phase_margin: None,
        }
    }

    /// Add a frequency point
    pub fn add_point(&mut self, point: AcTransferPoint) {
        self.points.push(point);
    }

    /// Get frequency vector
    pub fn frequencies(&self) -> Vec<Value> {
        self.points.iter().map(|p| p.frequency).collect()
    }

    /// Get magnitude curve (freq, dB)
    pub fn magnitude_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.frequency, p.magnitude_db))
            .collect()
    }

    /// Get phase curve (freq, degrees)
    pub fn phase_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.frequency, p.phase_deg))
            .collect()
    }

    /// Get group delay curve
    pub fn group_delay_curve(&self) -> Vec<(Value, Value)> {
        if self.points.len() < 2 {
            return Vec::new();
        }

        self.points
            .windows(2)
            .map(|w| {
                let gd = w[0].group_delay(&w[1]);
                ((w[0].frequency + w[1].frequency) / 2.0, gd)
            })
            .collect()
    }

    /// Compute filter characteristics from data
    pub fn compute_characteristics(&mut self) {
        if self.points.is_empty() {
            return;
        }

        // Find peak
        let Some((peak_idx, peak_db)) = self
            .points
            .iter()
            .enumerate()
            .filter_map(|(idx, point)| {
                point
                    .magnitude_db
                    .is_finite()
                    .then_some((idx, point.magnitude_db))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return;
        };

        self.peak_gain_db = Some(peak_db);
        self.peak_frequency = Some(self.points[peak_idx].frequency);

        // DC gain (from lowest frequency if < 100 Hz)
        if let Some(first) = self.points.first()
            && first.frequency < 100.0
        {
            self.dc_gain = Some(first.magnitude);
            self.dc_gain_db = Some(first.magnitude_db);
        }

        // Find -3dB cutoffs
        let threshold = peak_db - 3.0;

        // Low cutoff (before peak)
        self.cutoff_low = self.find_crossing_before(peak_idx, threshold);

        // High cutoff (after peak)
        self.cutoff_high = self.find_crossing_after(peak_idx, threshold);

        // Bandwidth
        if let (Some(fl), Some(fh)) = (self.cutoff_low, self.cutoff_high) {
            self.bandwidth = Some(fh - fl);

            // Q factor = f_center / bandwidth
            if let Some(fc) = self.peak_frequency {
                let bw = fh - fl;
                if bw > 0.0 {
                    self.q_factor = Some(fc / bw);
                }
            }
        }

        // Unity gain frequency and phase margin
        self.unity_gain_frequency = self.find_unity_gain_crossing();
        if let Some(ugf) = self.unity_gain_frequency {
            self.phase_margin = self.phase_at_frequency(ugf).map(|p| 180.0 + p);
        }
    }

    /// Find frequency where magnitude crosses threshold before index
    fn find_crossing_before(&self, before_idx: usize, threshold: Value) -> Option<Value> {
        for i in (1..before_idx).rev() {
            let db0 = self.points[i - 1].magnitude_db;
            let db1 = self.points[i].magnitude_db;
            let f0_raw = self.points[i - 1].frequency;
            let f1_raw = self.points[i].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 <= threshold && db1 > threshold) || (db0 >= threshold && db1 < threshold) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                // Interpolate
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (threshold - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Find frequency where magnitude crosses threshold after index
    fn find_crossing_after(&self, after_idx: usize, threshold: Value) -> Option<Value> {
        for i in after_idx..self.points.len() - 1 {
            let db0 = self.points[i].magnitude_db;
            let db1 = self.points[i + 1].magnitude_db;
            let f0_raw = self.points[i].frequency;
            let f1_raw = self.points[i + 1].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 >= threshold && db1 < threshold) || (db0 <= threshold && db1 > threshold) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                // Interpolate
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (threshold - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Find unity gain (0 dB) crossing frequency
    fn find_unity_gain_crossing(&self) -> Option<Value> {
        for i in 0..self.points.len() - 1 {
            let db0 = self.points[i].magnitude_db;
            let db1 = self.points[i + 1].magnitude_db;
            let f0_raw = self.points[i].frequency;
            let f1_raw = self.points[i + 1].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 >= 0.0 && db1 < 0.0) || (db0 <= 0.0 && db1 > 0.0) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (0.0 - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Get phase at specific frequency (interpolated)
    fn phase_at_frequency(&self, freq: Value) -> Option<Value> {
        if !freq.is_finite() || freq <= 0.0 {
            return None;
        }
        for i in 0..self.points.len() - 1 {
            if self.points[i].frequency <= freq && self.points[i + 1].frequency >= freq {
                let f0_raw = self.points[i].frequency;
                let f1_raw = self.points[i + 1].frequency;
                let p0 = self.points[i].phase_deg;
                let p1 = self.points[i + 1].phase_deg;
                if !f0_raw.is_finite()
                    || !f1_raw.is_finite()
                    || !p0.is_finite()
                    || !p1.is_finite()
                    || f0_raw <= 0.0
                    || f1_raw <= 0.0
                {
                    continue;
                }
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let denom = f1 - f0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                let alpha = (freq.log10() - f0) / denom;
                let phase = p0 + alpha * (p1 - p0);
                if phase.is_finite() {
                    return Some(phase);
                }
            }
        }
        None
    }
}

/// Configuration for AC transfer function analysis
#[derive(Debug, Clone)]
pub struct AcTransferConfig {
    /// Output node name
    pub output_node: String,

    /// Reference node (ground if None)
    pub ref_node: Option<String>,

    /// Input source name
    pub input_source: String,

    /// Start frequency (Hz)
    pub freq_start: Value,

    /// Stop frequency (Hz)
    pub freq_stop: Value,

    /// Number of points per decade (for decade sweep)
    pub points_per_decade: usize,

    /// Sweep type
    pub sweep_type: AcSweepType,
}

/// AC frequency sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcSweepType {
    /// Linear sweep
    Linear,
    /// Decade (logarithmic) sweep
    #[default]
    Decade,
    /// Octave sweep
    Octave,
}

impl AcTransferConfig {
    /// Create decade sweep configuration
    pub fn decade(
        output_node: &str,
        input_source: &str,
        freq_start: Value,
        freq_stop: Value,
        points_per_decade: usize,
    ) -> Self {
        Self {
            output_node: output_node.to_string(),
            ref_node: None,
            input_source: input_source.to_string(),
            freq_start,
            freq_stop,
            points_per_decade,
            sweep_type: AcSweepType::Decade,
        }
    }

    /// Create linear sweep configuration
    pub fn linear(
        output_node: &str,
        input_source: &str,
        freq_start: Value,
        freq_stop: Value,
        num_points: usize,
    ) -> Self {
        Self {
            output_node: output_node.to_string(),
            ref_node: None,
            input_source: input_source.to_string(),
            freq_start,
            freq_stop,
            points_per_decade: num_points, // Repurpose for total points
            sweep_type: AcSweepType::Linear,
        }
    }

    /// Set reference node
    pub fn with_ref(mut self, ref_node: &str) -> Self {
        self.ref_node = Some(ref_node.to_string());
        self
    }

    /// Generate frequency points
    pub fn frequency_points(&self) -> Vec<Value> {
        match self.sweep_type {
            AcSweepType::Linear => {
                let n = self.points_per_decade;
                if n <= 1 {
                    return vec![self.freq_start];
                }
                let step = (self.freq_stop - self.freq_start) / (n - 1) as Value;
                (0..n)
                    .map(|i| self.freq_start + i as Value * step)
                    .collect()
            }
            AcSweepType::Decade => {
                if self.freq_start <= 0.0 || self.freq_stop <= 0.0 {
                    return vec![self.freq_start.max(1e-15)];
                }
                let log_start = self.freq_start.log10();
                let log_stop = self.freq_stop.log10();
                let num_decades = log_stop - log_start;
                let total_points = (num_decades * self.points_per_decade as f64).ceil() as usize;
                let total_points = total_points.max(1);

                (0..total_points)
                    .map(|i| {
                        let log_f = log_start
                            + (log_stop - log_start) * i as f64 / (total_points - 1).max(1) as f64;
                        10.0_f64.powf(log_f)
                    })
                    .collect()
            }
            AcSweepType::Octave => {
                if self.freq_start <= 0.0 || self.freq_stop <= 0.0 {
                    return vec![self.freq_start.max(1e-15)];
                }
                let log2_start = self.freq_start.log2();
                let log2_stop = self.freq_stop.log2();
                let num_octaves = log2_stop - log2_start;
                let total_points = (num_octaves * self.points_per_decade as f64).ceil() as usize;
                let total_points = total_points.max(1);

                (0..total_points)
                    .map(|i| {
                        let log2_f = log2_start
                            + (log2_stop - log2_start) * i as f64
                                / (total_points - 1).max(1) as f64;
                        2.0_f64.powf(log2_f)
                    })
                    .collect()
            }
        }
    }
}

/// AC Transfer Function Analyzer
pub struct AcTransferAnalyzer {
    config: AcTransferConfig,
}

impl AcTransferAnalyzer {
    /// Create new analyzer
    pub fn new(config: AcTransferConfig) -> Self {
        Self { config }
    }

    /// Analyze using a transfer function evaluator
    ///
    /// The evaluator should return H(jÏ‰) for given frequency
    pub fn analyze<F>(&self, mut evaluator: F) -> AcTransferResult
    where
        F: FnMut(Value) -> Complex64,
    {
        let mut result = AcTransferResult::new(&self.config.output_node, &self.config.input_source);

        for freq in self.config.frequency_points() {
            let h = evaluator(freq);
            result.add_point(AcTransferPoint::new(freq, h));
        }

        result.compute_characteristics();
        result
    }

    /// Create a test lowpass filter transfer function
    ///
    /// H(s) = Ï‰â‚€ / (s + Ï‰â‚€) = 1 / (1 + s/Ï‰â‚€)
    pub fn test_lowpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            Complex64::new(omega_0, 0.0) / (s + Complex64::new(omega_0, 0.0))
        })
    }

    /// Create a test highpass filter transfer function
    ///
    /// H(s) = s / (s + Ï‰â‚€)
    pub fn test_highpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            s / (s + Complex64::new(omega_0, 0.0))
        })
    }

    /// Create a test bandpass filter transfer function
    ///
    /// H(s) = Ï‰â‚’/Q Â· s / (sÂ² + Ï‰â‚’/Q Â· s + Ï‰â‚’Â²)
    pub fn test_bandpass(&self, center_freq: Value, q_factor: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * center_freq;
        let omega_q = omega_0 / q_factor;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            let num = Complex64::new(omega_q, 0.0) * s;
            let denom =
                s * s + s * Complex64::new(omega_q, 0.0) + Complex64::new(omega_0 * omega_0, 0.0);
            num / denom
        })
    }

    /// Create a test two-pole lowpass (Butterworth-like)
    ///
    /// H(s) = Ï‰â‚€Â² / (sÂ² + âˆš2Â·Ï‰â‚€Â·s + Ï‰â‚€Â²)
    pub fn test_butterworth_lowpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;
        let omega_0_sq = omega_0 * omega_0;
        let sqrt2_omega_0 = 2.0_f64.sqrt() * omega_0;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            Complex64::new(omega_0_sq, 0.0)
                / (s * s + s * Complex64::new(sqrt2_omega_0, 0.0) + Complex64::new(omega_0_sq, 0.0))
        })
    }
}

//=============================================================================
// Tests
//=============================================================================

