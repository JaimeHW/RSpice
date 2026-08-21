use super::*;

#[derive(Debug, Clone)]
pub struct Port {
    /// Port number (1-indexed)
    pub number: usize,
    /// Positive terminal node
    pub node_pos: String,
    /// Negative terminal node (usually ground)
    pub node_neg: String,
    /// Reference impedance (typically 50Ω)
    pub z0: Value,
}

impl Port {
    /// Create a single-ended port (referenced to ground)
    pub fn single_ended(number: usize, node: &str, z0: Value) -> Self {
        Self {
            number,
            node_pos: node.to_string(),
            node_neg: "0".to_string(),
            z0,
        }
    }

    /// Create a differential port
    pub fn differential(number: usize, node_pos: &str, node_neg: &str, z0: Value) -> Self {
        Self {
            number,
            node_pos: node_pos.to_string(),
            node_neg: node_neg.to_string(),
            z0,
        }
    }
}

//=============================================================================
// S-Matrix
//=============================================================================

/// S-parameter matrix at a single frequency
#[derive(Debug, Clone)]
pub struct SMatrix {
    /// Frequency in Hz
    pub frequency: Value,
    /// Angular frequency ω = 2πf
    pub omega: Value,
    /// S-parameter data `[row][col]` = S[row+1, col+1]
    data: Vec<Vec<Complex64>>,
    /// Number of ports
    num_ports: usize,
}

impl SMatrix {
    /// Create empty S-matrix for N ports
    pub fn new(frequency: Value, num_ports: usize) -> Self {
        Self {
            frequency,
            omega: 2.0 * PI * frequency,
            data: vec![vec![Complex64::ZERO; num_ports]; num_ports],
            num_ports,
        }
    }

    /// Set S-parameter value (1-indexed)
    pub fn set(&mut self, row: usize, col: usize, value: Complex64) {
        if row >= 1 && row <= self.num_ports && col >= 1 && col <= self.num_ports {
            self.data[row - 1][col - 1] = value;
        }
    }

    /// Get S-parameter value (1-indexed)
    pub fn get(&self, row: usize, col: usize) -> Complex64 {
        if row >= 1 && row <= self.num_ports && col >= 1 && col <= self.num_ports {
            self.data[row - 1][col - 1]
        } else {
            Complex64::ZERO
        }
    }

    /// Get S11 (input reflection)
    pub fn s11(&self) -> Complex64 {
        self.get(1, 1)
    }

    /// Get S21 (forward transmission)
    pub fn s21(&self) -> Complex64 {
        self.get(2, 1)
    }

    /// Get S12 (reverse transmission)
    pub fn s12(&self) -> Complex64 {
        self.get(1, 2)
    }

    /// Get S22 (output reflection)
    pub fn s22(&self) -> Complex64 {
        self.get(2, 2)
    }
}

//=============================================================================
// S-Parameter Result
//=============================================================================

/// Complete S-parameter analysis result
#[derive(Debug, Clone)]
pub struct SParameterResult {
    /// Reference impedance for all ports
    pub z0: Value,
    /// Number of ports
    pub num_ports: usize,
    /// Port definitions
    pub ports: Vec<Port>,
    /// S-matrices at each frequency point
    pub data: Vec<SMatrix>,
}

impl SParameterResult {
    /// Create empty result
    pub fn new(z0: Value, ports: Vec<Port>) -> Self {
        let num_ports = ports.len();
        Self {
            z0,
            num_ports,
            ports,
            data: Vec::new(),
        }
    }

    /// Add S-matrix for a frequency point
    pub fn add(&mut self, matrix: SMatrix) {
        self.data.push(matrix);
    }

    /// Get frequency points
    pub fn frequencies(&self) -> Vec<Value> {
        self.data.iter().map(|s| s.frequency).collect()
    }

    /// Get VSWR from S11
    /// VSWR = (1 + |S11|) / (1 - |S11|)
    pub fn vswr(&self) -> Vec<Value> {
        self.data
            .iter()
            .map(|s| {
                let gamma = s.s11().norm();
                if gamma >= 1.0 {
                    f64::INFINITY
                } else {
                    (1.0 + gamma) / (1.0 - gamma)
                }
            })
            .collect()
    }
}

//=============================================================================
// S-Parameter Analyzer
//=============================================================================
