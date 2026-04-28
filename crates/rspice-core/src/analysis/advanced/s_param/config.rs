use super::*;

#[derive(Debug, Clone)]
pub struct SParameterConfig {
    /// Start frequency (Hz)
    pub f_start: Value,
    /// Stop frequency (Hz)
    pub f_stop: Value,
    /// Number of frequency points
    pub num_points: usize,
    /// Frequency sweep type
    pub sweep_type: FrequencySweep,
    /// Reference impedance
    pub z0: Value,
}

/// Frequency sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencySweep {
    Linear,
    Decade,
    Octave,
}

impl SParameterConfig {
    /// Create linear sweep configuration
    pub fn linear(f_start: Value, f_stop: Value, num_points: usize) -> Self {
        Self {
            f_start,
            f_stop,
            num_points,
            sweep_type: FrequencySweep::Linear,
            z0: 50.0,
        }
    }

    /// Create decade sweep configuration
    pub fn decade(f_start: Value, f_stop: Value, points_per_decade: usize) -> Self {
        let decades = (f_stop / f_start).log10();
        let num_points = (decades * points_per_decade as Value).ceil() as usize;
        Self {
            f_start,
            f_stop,
            num_points,
            sweep_type: FrequencySweep::Decade,
            z0: 50.0,
        }
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: Value) -> Self {
        self.z0 = z0;
        self
    }

    /// Generate frequency points
    pub fn frequencies(&self) -> Vec<Value> {
        if self.num_points <= 1 {
            return vec![self.f_start];
        }

        match self.sweep_type {
            FrequencySweep::Linear => {
                let step = (self.f_stop - self.f_start) / (self.num_points - 1) as Value;
                (0..self.num_points)
                    .map(|i| self.f_start + (i as Value) * step)
                    .collect()
            }
            FrequencySweep::Decade | FrequencySweep::Octave => {
                let log_start = self.f_start.log10();
                let log_stop = self.f_stop.log10();
                let log_step = (log_stop - log_start) / (self.num_points - 1) as Value;
                (0..self.num_points)
                    .map(|i| 10_f64.powf(log_start + (i as Value) * log_step))
                    .collect()
            }
        }
    }
}

/// S-parameter analyzer
///
/// Computes S-parameters by exciting each port and measuring the response
pub struct SParameterAnalyzer {
    config: SParameterConfig,
    ports: Vec<Port>,
}

impl SParameterAnalyzer {
    /// Create analyzer
    pub fn new(config: SParameterConfig, ports: Vec<Port>) -> Self {
        Self { config, ports }
    }

    /// Compute S-parameters from Y-parameters (admittance matrix)
    ///
    /// S = (I - Z₀·Y) · (I + Z₀·Y)⁻¹
    ///
    /// For 2-port:
    /// S11 = ((1-Z₀·Y11)(1+Z₀·Y22) + Z₀²·Y12·Y21) / Δ
    /// S21 = -2·Z₀·Y21 / Δ
    /// where Δ = (1+Z₀·Y11)(1+Z₀·Y22) - Z₀²·Y12·Y21
    pub fn from_y_parameters(&self, y: &[[Complex; 2]; 2], frequency: Value) -> SMatrix {
        let z0 = Complex::new(self.config.z0, 0.0);
        let one = Complex::ONE;

        // Y-to-S conversion for 2-port
        let y11 = y[0][0];
        let y12 = y[0][1];
        let y21 = y[1][0];
        let y22 = y[1][1];

        // Δ = (1 + Z₀·Y11)(1 + Z₀·Y22) - Z₀²·Y12·Y21
        let z0y11 = z0 * y11;
        let z0y22 = z0 * y22;
        let z02y12y21 = z0 * z0 * y12 * y21;

        let term1 = (one + z0y11) * (one + z0y22);
        let delta = term1 - z02y12y21;

        // S11 = ((1 - Z₀·Y11)(1 + Z₀·Y22) + Z₀²·Y12·Y21) / Δ
        let s11_num = (one - z0y11) * (one + z0y22) + z02y12y21;
        let s11 = s11_num / delta;

        // S21 = -2·Z₀·Y21 / Δ
        let s21 = (z0 * y21 * (-2.0)) / delta;

        // S12 = -2·Z₀·Y12 / Δ
        let s12 = (z0 * y12 * (-2.0)) / delta;

        // S22 = ((1 + Z₀·Y11)(1 - Z₀·Y22) + Z₀²·Y12·Y21) / Δ
        let s22_num = (one + z0y11) * (one - z0y22) + z02y12y21;
        let s22 = s22_num / delta;

        let mut matrix = SMatrix::new(frequency, 2);
        matrix.set(1, 1, s11);
        matrix.set(1, 2, s12);
        matrix.set(2, 1, s21);
        matrix.set(2, 2, s22);

        matrix
    }

    /// Compute S-parameters from Z-parameters (impedance matrix)
    ///
    /// S = (Z - Z₀·I) · (Z + Z₀·I)⁻¹
    pub fn from_z_parameters(&self, z: &[[Complex; 2]; 2], frequency: Value) -> SMatrix {
        let z0 = Complex::new(self.config.z0, 0.0);

        let z11 = z[0][0];
        let z12 = z[0][1];
        let z21 = z[1][0];
        let z22 = z[1][1];

        // Δ = (Z11 + Z₀)(Z22 + Z₀) - Z12·Z21
        let delta = (z11 + z0) * (z22 + z0) - z12 * z21;

        // S11 = ((Z11 - Z₀)(Z22 + Z₀) - Z12·Z21) / Δ
        let s11 = ((z11 - z0) * (z22 + z0) - z12 * z21) / delta;

        // S21 = 2·Z₀·Z21 / Δ
        let s21 = z0 * z21 * 2.0 / delta;

        // S12 = 2·Z₀·Z12 / Δ
        let s12 = z0 * z12 * 2.0 / delta;

        // S22 = ((Z11 + Z₀)(Z22 - Z₀) - Z12·Z21) / Δ
        let s22 = ((z11 + z0) * (z22 - z0) - z12 * z21) / delta;

        let mut matrix = SMatrix::new(frequency, 2);
        matrix.set(1, 1, s11);
        matrix.set(1, 2, s12);
        matrix.set(2, 1, s21);
        matrix.set(2, 2, s22);

        matrix
    }

    /// Run S-parameter analysis with a Y-parameter solver
    ///
    /// The solver should return the Y-matrix for the given frequency
    pub fn analyze<F>(&self, mut y_solver: F) -> SParameterResult
    where
        F: FnMut(Value) -> [[Complex; 2]; 2],
    {
        let mut result = SParameterResult::new(self.config.z0, self.ports.clone());

        for frequency in self.config.frequencies() {
            let y = y_solver(frequency);
            let s_matrix = self.from_y_parameters(&y, frequency);
            result.add(s_matrix);
        }

        result
    }
}
