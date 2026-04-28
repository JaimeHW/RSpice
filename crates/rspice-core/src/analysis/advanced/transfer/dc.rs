use super::*;

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
