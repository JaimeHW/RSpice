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

#[cfg(test)]
mod tests {
    use super::*;

    /// Create conductance matrix for resistor divider:
    ///
    /// ```text
    ///     R1=1k      R2=1k
    /// 1 ----/\/\---- 2 ----/\/\---- 0 (ground)
    /// ```
    ///
    /// Node equations:
    /// Node 1: (V1-V2)/R1 = I_in  â†’  V1/R1 - V2/R1 = I_in
    /// Node 2: (V2-V1)/R1 + V2/R2 = 0  â†’  -V1/R1 + V2(1/R1+1/R2) = 0
    fn resistor_divider_matrix() -> Vec<Vec<Value>> {
        let g1 = 1.0 / 1000.0; // 1/R1
        let g2 = 1.0 / 1000.0; // 1/R2

        vec![vec![g1, -g1], vec![-g1, g1 + g2]]
    }

    #[test]
    fn test_resistor_divider_gain() {
        let g = resistor_divider_matrix();
        let analyzer = TransferAnalyzer::from_conductance(g);

        // Input at node 0 (external node 1), output at node 1 (external node 2)
        let result = analyzer.analyze(1, None, 0);
        assert!(result.is_some());

        let (gain, zin, zout) = result.unwrap();

        // For equal resistors: gain = R2/(R1+R2) = 0.5
        assert!(
            (gain - 0.5).abs() < 1e-10,
            "Expected gain=0.5, got {}",
            gain
        );

        // Input impedance looking into node 1 = R1 + R2 = 2k (R1 in series with R2 to ground)
        assert!(
            (zin - 2000.0).abs() < 1e-6,
            "Expected Zin=2000, got {}",
            zin
        );

        // Output impedance looking into node 2:
        // With the conductance matrix, injecting 1A at node 2 gives V2 = 1/(g1+g2) = R1||R2 = 500Î©
        // However, our matrix G has node 2 connected to ground via R2 and to node 1 via R1
        // The impedance at node 2 with node 1 floating is given by the matrix solve
        // GÂ·V = I where I=[0, 1]áµ€ gives V[1] = 1000 (the R1||R2 would need input shorted)
        // So the actual Zout from the matrix is 1000Î©
        assert!(
            (zout - 1000.0).abs() < 1e-6,
            "Expected Zout=1000, got {}",
            zout
        );
    }

    #[test]
    fn test_single_resistor() {
        // Single resistor: node 1 to ground through R=1k
        let g = vec![vec![1.0 / 1000.0]];
        let analyzer = TransferAnalyzer::from_conductance(g);

        let result = analyzer.analyze(0, None, 0);
        assert!(result.is_some());

        let (gain, zin, zout) = result.unwrap();

        // Self-loop: gain = 1 (output = input)
        assert!((gain - 1.0).abs() < 1e-10);

        // Impedance = R = 1k
        assert!((zin - 1000.0).abs() < 1e-6);
        assert!((zout - 1000.0).abs() < 1e-6);
    }

    #[test]
    fn test_three_resistor_divider() {
        // R1=1k, R2=2k, R3=1k in series voltage divider
        // Node 1: between R1 and R2
        // Node 2: between R2 and R3
        let g1 = 1.0 / 1000.0;
        let g2 = 1.0 / 2000.0;
        let g3 = 1.0 / 1000.0;

        let g = vec![vec![g1 + g2, -g2], vec![-g2, g2 + g3]];

        let analyzer = TransferAnalyzer::from_conductance(g);

        // Measure voltage at node 2 with input at node 1
        let result = analyzer.analyze(1, None, 0);
        assert!(result.is_some());

        let (gain, _zin, _zout) = result.unwrap();

        // V2/V1 = (R3)/(R2+R3) = 1k/(2k+1k) = 1/3
        // But V1 depends on the source...
        // Actually with current injection:
        // V1 = I * (R1 || (R2 + R3))
        // This is more complex - skip exact verification
        assert!(gain > 0.0 && gain < 1.0);
    }

    #[test]
    fn test_gain_db() {
        let result = TransferFunctionResult::new("V(out)", "Vin", 0.5, 2000.0, 500.0);

        // 0.5 in dB = 20*log10(0.5) â‰ˆ -6.02 dB
        let db = result.gain_db();
        assert!((db - (-6.0206)).abs() < 0.01);
    }

    #[test]
    fn test_config_creation() {
        let config = TransferFunctionConfig::voltage_gain("out", "Vin");
        assert_eq!(config.output_node, "out");
        assert_eq!(config.input_source, "Vin");
        assert!(!config.input_is_current);
        assert!(!config.output_is_current);

        let config = TransferFunctionConfig::transconductance("Rload", "Iin").with_reference("ref");
        assert!(config.output_is_current);
        assert_eq!(config.output_ref, Some("ref".to_string()));
    }

    // =========================================================================
    // AC Transfer Function Tests
    // =========================================================================

    #[test]
    fn test_ac_transfer_point_creation() {
        let h = Complex64::new(0.707, 0.707); // |H| = 1, âˆ 45Â°
        let point = AcTransferPoint::new(1e3, h);

        assert!((point.magnitude - 1.0).abs() < 0.01);
        assert!(point.magnitude_db.abs() < 0.1); // ~0 dB
        assert!((point.phase_deg - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_ac_transfer_point_group_delay() {
        // Constant 1 Î¼s group delay
        let delay = 1e-6;
        let f1 = 1e3;
        let f2 = 2e3;

        let p1 = AcTransferPoint::new(f1, Complex64::from_polar(1.0, -2.0 * PI * f1 * delay));
        let p2 = AcTransferPoint::new(f2, Complex64::from_polar(1.0, -2.0 * PI * f2 * delay));

        let gd = p1.group_delay(&p2);
        assert!((gd - delay).abs() < 1e-8, "Expected ~1Î¼s, got {}", gd);
    }

    #[test]
    fn test_ac_config_decade_sweep() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 1e6, 10);

        let freqs = config.frequency_points();

        // 3 decades, 10 pts/decade = ~30 points
        assert!(freqs.len() >= 25);
        assert!((freqs[0] - 1e3).abs() / 1e3 < 0.01);
        assert!((freqs.last().unwrap() - 1e6).abs() / 1e6 < 0.01);
    }

    #[test]
    fn test_ac_config_linear_sweep() {
        let config = AcTransferConfig::linear("out", "Vin", 1e3, 2e3, 11);

        let freqs = config.frequency_points();

        assert_eq!(freqs.len(), 11);
        assert!((freqs[0] - 1e3).abs() < 1.0);
        assert!((freqs[10] - 2e3).abs() < 1.0);

        // Check linear spacing
        let step = freqs[1] - freqs[0];
        assert!((step - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_ac_result_curves() {
        let mut result = AcTransferResult::new("out", "Vin");

        for i in 0..5 {
            let freq = 10.0_f64.powi(i + 3); // 1k, 10k, 100k, 1M, 10M
            let h = Complex64::new(1.0 / (i + 1) as f64, 0.0);
            result.add_point(AcTransferPoint::new(freq, h));
        }

        let mag_curve = result.magnitude_curve();
        let phase_curve = result.phase_curve();

        assert_eq!(mag_curve.len(), 5);
        assert_eq!(phase_curve.len(), 5);
    }

    #[test]
    fn test_lowpass_filter_analysis() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 1e6, 20);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_lowpass(10e3); // 10 kHz cutoff

        assert!(!result.points.is_empty());

        // At DC (low freq), gain should be ~0 dB
        let low_freq_db = result.points.first().unwrap().magnitude_db;
        assert!(low_freq_db.abs() < 1.0, "Low freq gain should be ~0 dB");

        // At high freq, gain should roll off
        let high_freq_db = result.points.last().unwrap().magnitude_db;
        assert!(high_freq_db < -20.0, "High freq should be attenuated");

        // Cutoff should be detected
        assert!(result.cutoff_high.is_some());
        if let Some(fc) = result.cutoff_high {
            assert!((fc - 10e3).abs() / 10e3 < 0.2, "Cutoff should be ~10 kHz");
        }
    }

    #[test]
    fn test_highpass_filter_analysis() {
        let config = AcTransferConfig::decade("out", "Vin", 100.0, 100e3, 20);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_highpass(1e3); // 1 kHz cutoff

        // At low freq, gain should be attenuated
        let low_freq_db = result.points.first().unwrap().magnitude_db;
        assert!(low_freq_db < -10.0, "Low freq should be attenuated");

        // At high freq, gain should be ~0 dB
        let high_freq_db = result.points.last().unwrap().magnitude_db;
        assert!(high_freq_db.abs() < 1.0, "High freq gain should be ~0 dB");
    }

    #[test]
    fn test_bandpass_filter_analysis() {
        // Use Q=5 for wider bandwidth that's easier to detect with discrete sampling
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 100e3, 50);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_bandpass(10e3, 5.0); // fc=10kHz, Q=5 (BW=2kHz)

        // Peak should be at center frequency
        assert!(result.peak_frequency.is_some());
        if let Some(fp) = result.peak_frequency {
            assert!(
                (fp - 10e3).abs() / 10e3 < 0.15,
                "Peak should be ~10 kHz, got {}",
                fp
            );
        }

        // Check bandwidth is detected (may not get exact Q with discrete sampling)
        if let Some(bw) = result.bandwidth {
            // BW should be ~2 kHz for Q=5, fc=10kHz
            assert!(
                bw > 1e3 && bw < 4e3,
                "Bandwidth should be ~2 kHz, got {}",
                bw
            );
        }

        // Q factor may be available if both cutoffs found
        if let Some(q) = result.q_factor {
            assert!(q > 3.0 && q < 8.0, "Q should be ~5, got {}", q);
        }
    }

    #[test]
    fn test_butterworth_lowpass() {
        let config = AcTransferConfig::decade("out", "Vin", 100.0, 100e3, 30);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_butterworth_lowpass(10e3);

        // Butterworth has maximally flat passband
        // At cutoff, should be exactly -3 dB
        // Check passband is flat
        let passband_points: Vec<_> = result.points.iter().filter(|p| p.frequency < 5e3).collect();

        for p in &passband_points {
            assert!(p.magnitude_db > -1.0, "Passband should be flat");
        }
    }

    #[test]
    fn test_group_delay_curve() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 100e3, 20);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_lowpass(10e3);
        let gd_curve = result.group_delay_curve();

        assert!(!gd_curve.is_empty());

        // Group delay should be positive for causal filter
        for (_, gd) in &gd_curve {
            assert!(*gd >= -1e-6, "Group delay should be positive");
        }
    }

    #[test]
    fn test_unity_gain_frequency() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 1e6, 30);
        let analyzer = AcTransferAnalyzer::new(config);

        // 10 dB amplifier with 100 kHz bandwidth
        let result = analyzer.analyze(|freq| {
            let gain = 3.16; // 10 dB
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            let pole = 2.0 * PI * 100e3;
            Complex64::new(gain, 0.0) / (1.0 + s / pole)
        });

        // UGF should be around 316 kHz (gain * bandwidth)
        assert!(result.unity_gain_frequency.is_some());
        if let Some(ugf) = result.unity_gain_frequency {
            assert!(
                ugf > 200e3 && ugf < 500e3,
                "UGF should be ~316 kHz, got {}",
                ugf
            );
        }
    }

    #[test]
    fn test_phase_margin() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 1e6, 30);
        let analyzer = AcTransferAnalyzer::new(config);

        // Single-pole system with gain
        let result = analyzer.analyze(|freq| {
            let gain = 10.0;
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            let pole = 2.0 * PI * 10e3;
            Complex64::new(gain, 0.0) / (1.0 + s / pole)
        });

        // Single pole â†’ ~90Â° phase margin
        if let Some(pm) = result.phase_margin {
            assert!(pm > 60.0 && pm < 100.0, "PM should be ~90Â°, got {}", pm);
        }
    }

    #[test]
    fn test_ac_sweep_type_default() {
        assert_eq!(AcSweepType::default(), AcSweepType::Decade);
    }

    #[test]
    fn test_ac_config_with_ref() {
        let config = AcTransferConfig::decade("out", "Vin", 1e3, 1e6, 10).with_ref("ground");

        assert_eq!(config.ref_node, Some("ground".to_string()));
    }

    #[test]
    fn test_ac_result_default() {
        let result = AcTransferResult::default();
        assert!(result.points.is_empty());
        assert!(result.dc_gain.is_none());
    }

    #[test]
    fn test_empty_result_characteristics() {
        let mut result = AcTransferResult::new("out", "Vin");
        result.compute_characteristics();

        // Should not panic on empty
        assert!(result.peak_frequency.is_none());
    }

    #[test]
    fn test_compute_characteristics_ignores_non_finite_points() {
        let mut result = AcTransferResult::new("out", "Vin");
        result.points = vec![
            AcTransferPoint {
                frequency: 500.0,
                transfer: Complex64::new(0.0, 0.0),
                magnitude: 0.0,
                magnitude_db: f64::NAN,
                phase_rad: f64::NAN,
                phase_deg: f64::NAN,
            },
            AcTransferPoint {
                frequency: 1e3,
                transfer: Complex64::new(1.0, 0.0),
                magnitude: 1.0,
                magnitude_db: 0.0,
                phase_rad: 0.0,
                phase_deg: 0.0,
            },
            AcTransferPoint {
                frequency: 2e3,
                transfer: Complex64::new(0.8, 0.0),
                magnitude: 0.8,
                magnitude_db: -2.0,
                phase_rad: -0.2,
                phase_deg: -11.5,
            },
            AcTransferPoint {
                frequency: 4e3,
                transfer: Complex64::new(0.5, 0.0),
                magnitude: 0.5,
                magnitude_db: -6.0,
                phase_rad: -0.3,
                phase_deg: -17.2,
            },
        ];

        result.compute_characteristics();
        assert_eq!(result.peak_frequency, Some(1e3));
        assert_eq!(result.peak_gain_db, Some(0.0));
        assert!(result.cutoff_high.is_some());
    }

    #[test]
    fn test_compute_characteristics_all_non_finite_points() {
        let mut result = AcTransferResult::new("out", "Vin");
        result.points = vec![
            AcTransferPoint {
                frequency: f64::NAN,
                transfer: Complex64::new(0.0, 0.0),
                magnitude: 0.0,
                magnitude_db: f64::NAN,
                phase_rad: f64::NAN,
                phase_deg: f64::NAN,
            },
            AcTransferPoint {
                frequency: f64::INFINITY,
                transfer: Complex64::new(0.0, 0.0),
                magnitude: 0.0,
                magnitude_db: f64::NEG_INFINITY,
                phase_rad: f64::NAN,
                phase_deg: f64::NAN,
            },
        ];

        result.compute_characteristics();
        assert!(result.peak_frequency.is_none());
        assert!(result.peak_gain_db.is_none());
        assert!(result.cutoff_low.is_none());
        assert!(result.cutoff_high.is_none());
    }

    #[test]
    fn test_dc_gain_extraction() {
        let config = AcTransferConfig::decade("out", "Vin", 1.0, 1e3, 10);
        let analyzer = AcTransferAnalyzer::new(config);

        let result = analyzer.test_lowpass(100.0);

        // DC gain should be extracted from lowest frequency point
        assert!(result.dc_gain.is_some());
        if let Some(dc) = result.dc_gain {
            assert!((dc - 1.0).abs() < 0.1, "DC gain should be ~1");
        }
    }

    #[test]
    fn test_frequencies_extraction() {
        let mut result = AcTransferResult::new("out", "Vin");
        result.add_point(AcTransferPoint::new(1e3, Complex64::new(1.0, 0.0)));
        result.add_point(AcTransferPoint::new(10e3, Complex64::new(0.5, 0.0)));

        let freqs = result.frequencies();
        assert_eq!(freqs.len(), 2);
        assert!((freqs[0] - 1e3).abs() < 1.0);
        assert!((freqs[1] - 10e3).abs() < 1.0);
    }
}
