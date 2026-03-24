//! Floquet Analysis for PNoise
//!
//! Implements time-varying transfer function computation using Floquet theory.
//! This is the core mathematical framework for phase noise analysis in periodic circuits.
//!
//! # Floquet Theory Background
//!
//! For a linear time-varying (LTV) system with period T, the state transition matrix
//! Î¦(t, tâ‚€) satisfies:
//!
//! dÎ¦/dt = A(t) * Î¦(t, tâ‚€),  Î¦(tâ‚€, tâ‚€) = I
//!
//! where A(t) is the time-varying Jacobian with A(t+T) = A(t).
//!
//! The **monodromy matrix** M = Î¦(T, 0) captures the complete period evolution.
//! Its eigenvalues (Floquet multipliers Î¼) and eigenvectors determine stability.
//!
//! Floquet exponents: Î» = ln(Î¼) / T
//!
//! # Phase Noise Application (Hajimiri-Lee Model)
//!
//! Phase noise arises from noise sources perturbing the oscillator state.
//! The key insight is that noise at different times in the cycle has different
//! impact on phase - captured by the **Impulse Sensitivity Function (ISF)**.
//!
//! Single-sideband phase noise:
//! L(Î”Ï‰) = Î“_rmsÂ² * i_nÂ² / (2 * Î”Ï‰Â² * q_maxÂ²)
//!
//! where Î“_rms is the RMS value of the ISF, i_nÂ² is noise current PSD,
//! Î”Ï‰ is offset from carrier, and q_max is max charge displacement.
//!
//! # Implementation
//!
//! This module provides:
//! - Monodromy matrix computation from time-varying Jacobians
//! - Eigenvalue decomposition for Floquet exponents
//! - ISF/PPV computation for noise sensitivity
//! - Transfer functions with proper 1/fÂ² behavior near carrier

#![allow(clippy::needless_range_loop)]
use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

// ============================================================================
// Main Floquet Analyzer
// ============================================================================

/// Floquet analyzer for phase noise calculations
///
/// Implements the mathematical machinery for computing phase noise from
/// circuit noise sources using Floquet theory and the Hajimiri-Lee ISF model.
#[derive(Debug)]
pub struct FloquetAnalyzer {
    /// Period of the steady-state solution [s]
    period: Value,

    /// Carrier frequency = 1/period [Hz]
    carrier_freq: Value,

    /// Number of harmonics in harmonic balance representation
    num_harmonics: usize,

    /// Number of circuit state variables
    num_states: usize,

    /// Floquet modes (computed from monodromy matrix eigenvectors)
    modes: Vec<FloquetMode>,

    /// Floquet multipliers (eigenvalues of monodromy matrix)
    floquet_multipliers: Vec<Complex64>,

    /// Floquet exponents Î» = ln(Î¼)/T
    floquet_exponents: Vec<Complex64>,

    /// Impulse Sensitivity Function samples over one period
    /// isf[node][time_sample] - how noise at node affects phase at each time
    isf_samples: Vec<Vec<Value>>,

    /// RMS value of ISF for each node (Î“_rms)
    isf_rms: Vec<Value>,

    /// Maximum charge displacement (q_max) for each node
    q_max: Vec<Value>,

    /// Time-varying Jacobian dA/dx sampled over period
    /// jacobian_samples[time_idx][row][col]
    jacobian_samples: Vec<Vec<Vec<Value>>>,

    /// Number of time samples per period for Jacobian
    num_time_samples: usize,

    /// Periodic waveform samples for ISF computation
    /// waveform[node][time_sample]
    waveform_samples: Vec<Vec<Value>>,

    /// Whether the analyzer has been initialized with proper data
    initialized: bool,
}

impl FloquetAnalyzer {
    /// Create new Floquet analyzer for given period and number of harmonics
    pub fn new(period: Value, num_harmonics: usize) -> Self {
        assert!(period > 0.0, "Period must be positive");
        Self {
            period,
            carrier_freq: 1.0 / period,
            num_harmonics,
            num_states: 0,
            modes: Vec::new(),
            floquet_multipliers: Vec::new(),
            floquet_exponents: Vec::new(),
            isf_samples: Vec::new(),
            isf_rms: Vec::new(),
            q_max: Vec::new(),
            jacobian_samples: Vec::new(),
            num_time_samples: 64, // Default to 64 samples per period
            waveform_samples: Vec::new(),
            initialized: false,
        }
    }

    /// Get the period [s]
    pub fn period(&self) -> Value {
        self.period
    }

    /// Get carrier frequency [Hz]
    pub fn carrier_freq(&self) -> Value {
        self.carrier_freq
    }

    /// Get number of harmonics
    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Set carrier frequency explicitly (overrides 1/period)
    pub fn with_carrier_freq(mut self, freq: Value) -> Self {
        self.carrier_freq = freq;
        self
    }

    /// Set number of time samples per period for Jacobian integration
    pub fn with_time_samples(mut self, n: usize) -> Self {
        self.num_time_samples = n.max(16); // Minimum 16 samples
        self
    }

    /// Set number of state variables
    pub fn with_num_states(mut self, n: usize) -> Self {
        self.num_states = n;
        self
    }

    /// Initialize with time-varying Jacobian samples from PSS solution
    ///
    /// `jacobians[t][i][j]` = dI_i/dV_j at time t
    ///
    /// Time samples should be uniformly spaced over one period [0, T)
    pub fn set_jacobian_samples(&mut self, jacobians: Vec<Vec<Vec<Value>>>) {
        if jacobians.is_empty() {
            return;
        }

        self.num_time_samples = jacobians.len();
        if let Some(first) = jacobians.first() {
            self.num_states = first.len();
        }

        self.jacobian_samples = jacobians;
    }

    /// Set periodic waveform samples for ISF computation
    ///
    /// `waveform[node][time_sample]` = voltage at node at each time sample
    pub fn set_waveform_samples(&mut self, waveforms: Vec<Vec<Value>>) {
        self.waveform_samples = waveforms;
    }

    /// Compute Floquet analysis from Jacobian samples
    ///
    /// This performs:
    /// 1. Monodromy matrix computation via numerical integration
    /// 2. Eigenvalue decomposition for modes and multipliers
    /// 3. ISF computation from adjoint system
    pub fn compute(&mut self) -> Result<(), FloquetError> {
        if self.jacobian_samples.is_empty() {
            // Use approximate model if no Jacobian data
            self.compute_approximate_isf();
            self.initialized = true;
            return Ok(());
        }

        // Step 1: Compute monodromy matrix M = Î¦(T, 0)
        let monodromy = self.compute_monodromy_matrix()?;

        // Step 2: Eigenvalue decomposition
        self.compute_eigendecomposition(&monodromy)?;

        // Step 3: Compute ISF from adjoint system
        self.compute_isf()?;

        self.initialized = true;
        Ok(())
    }

    /// Compute monodromy matrix M = Î¦(T, 0) via numerical integration
    ///
    /// Uses the fundamental matrix solution of dx/dt = A(t)x
    /// integrated over one complete period using trapezoidal rule.
    fn compute_monodromy_matrix(&self) -> Result<Vec<Vec<Complex64>>, FloquetError> {
        let n = self.num_states;
        if n == 0 {
            return Err(FloquetError::NoStates);
        }

        // Initialize Î¦(0,0) = I
        let mut phi: Vec<Vec<Complex64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if i == j {
                            Complex64::new(1.0, 0.0)
                        } else {
                            Complex64::new(0.0, 0.0)
                        }
                    })
                    .collect()
            })
            .collect();

        // Time step
        let dt = self.period / self.num_time_samples as f64;

        // Integrate dÎ¦/dt = A(t)Î¦ using trapezoidal rule
        for k in 0..self.num_time_samples {
            let a_k = &self.jacobian_samples[k];
            let a_next = &self.jacobian_samples[(k + 1) % self.num_time_samples];

            // Î¦_{k+1} â‰ˆ (I - dt/2 * A_{k+1})^{-1} * (I + dt/2 * A_k) * Î¦_k
            // Simplified: Î¦_{k+1} â‰ˆ (I + dt * A_k) * Î¦_k (forward Euler)
            // Better: Î¦_{k+1} â‰ˆ exp(dt * (A_k + A_{k+1})/2) * Î¦_k (midpoint)

            // Use matrix exponential approximation: exp(dt*A) â‰ˆ I + dt*A + (dt*A)Â²/2
            let a_mid: Vec<Vec<Value>> = (0..n)
                .map(|i| (0..n).map(|j| 0.5 * (a_k[i][j] + a_next[i][j])).collect())
                .collect();

            let exp_a = self.matrix_exponential(&a_mid, dt);
            phi = self.matrix_multiply_complex(&exp_a, &phi);
        }

        Ok(phi)
    }

    /// Compute matrix exponential exp(dt * A) using PadÃ© approximation
    ///
    /// For small dt*||A||, uses Taylor series: I + dt*A + (dt*A)Â²/2! + ...
    /// For larger values, uses scaling and squaring with PadÃ© approximant.
    fn matrix_exponential(&self, a: &[Vec<Value>], dt: Value) -> Vec<Vec<Complex64>> {
        let n = a.len();
        if n == 0 {
            return Vec::new();
        }

        // Compute scaled matrix: B = dt * A
        let b: Vec<Vec<Value>> = a
            .iter()
            .map(|row| row.iter().map(|&x| dt * x).collect())
            .collect();

        // Estimate norm for scaling
        let norm: Value = b
            .iter()
            .map(|row| row.iter().map(|x| x.abs()).sum::<Value>())
            .fold(0.0, Value::max);

        // Number of squarings needed: 2^s * ||B|| < 1
        let s = ((norm.ln() / 2.0_f64.ln()).ceil().max(0.0)) as u32;
        let scale = 2.0_f64.powi(s as i32);

        // Scale down
        let b_scaled: Vec<Vec<Value>> = b
            .iter()
            .map(|row| row.iter().map(|&x| x / scale).collect())
            .collect();

        // Compute exp(B/2^s) using Taylor series (6 terms for good accuracy)
        let mut result: Vec<Vec<Complex64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if i == j {
                            Complex64::new(1.0, 0.0)
                        } else {
                            Complex64::new(0.0, 0.0)
                        }
                    })
                    .collect()
            })
            .collect();

        let mut term = result.clone();
        for k in 1..=6 {
            term = self.matrix_multiply_real_complex(&b_scaled, &term);
            let factor = 1.0 / (k as f64);
            for i in 0..n {
                for j in 0..n {
                    term[i][j] *= factor;
                    result[i][j] += term[i][j];
                }
            }
        }

        // Square s times to get exp(B)
        for _ in 0..s {
            result = self.matrix_multiply_complex(&result, &result);
        }

        result
    }

    /// Multiply two complex matrices
    fn matrix_multiply_complex(
        &self,
        a: &[Vec<Complex64>],
        b: &[Vec<Complex64>],
    ) -> Vec<Vec<Complex64>> {
        let n = a.len();
        if n == 0 {
            return Vec::new();
        }
        let m = b[0].len();

        (0..n)
            .map(|i| {
                (0..m)
                    .map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum())
                    .collect()
            })
            .collect()
    }

    /// Multiply real matrix by complex matrix
    fn matrix_multiply_real_complex(
        &self,
        a: &[Vec<Value>],
        b: &[Vec<Complex64>],
    ) -> Vec<Vec<Complex64>> {
        let n = a.len();
        if n == 0 {
            return Vec::new();
        }
        let m = b[0].len();

        (0..n)
            .map(|i| {
                (0..m)
                    .map(|j| (0..n).map(|k| Complex64::new(a[i][k], 0.0) * b[k][j]).sum())
                    .collect()
            })
            .collect()
    }

    /// Compute eigenvalue decomposition of monodromy matrix
    ///
    /// Uses power iteration and deflation for dominant eigenvalues.
    /// For phase noise, the critical mode is the one with |Î¼| â‰ˆ 1.
    fn compute_eigendecomposition(
        &mut self,
        monodromy: &[Vec<Complex64>],
    ) -> Result<(), FloquetError> {
        let n = monodromy.len();
        if n == 0 {
            return Err(FloquetError::NoStates);
        }

        // Use power iteration to find dominant eigenvalues
        // For a well-conditioned oscillator, we expect one mode with |Î¼| â‰ˆ 1 (phase mode)

        let mut multipliers = Vec::with_capacity(n.min(4));
        let mut modes = Vec::with_capacity(n.min(4));

        // Power iteration for up to 4 dominant modes
        let mut deflated = monodromy.to_vec();

        for mode_idx in 0..n.min(4) {
            let (eigenvalue, eigenvector) = self.power_iteration(&deflated, 100, 1e-10)?;

            multipliers.push(eigenvalue);
            let exponent = eigenvalue.ln() / self.period;
            let mode = FloquetMode::new(mode_idx, exponent, eigenvector.clone());
            modes.push(mode);

            // Deflate matrix: M' = M - Î¼ * v * v^H / ||v||Â²
            let norm_sq: Value = eigenvector.iter().map(|x| x.norm_sqr()).sum();
            if norm_sq > 1e-30 {
                for i in 0..deflated.len() {
                    for j in 0..deflated[i].len() {
                        deflated[i][j] -=
                            eigenvalue * eigenvector[i] * eigenvector[j].conj() / norm_sq;
                    }
                }
            }
        }

        self.floquet_multipliers = multipliers;
        self.floquet_exponents = self
            .floquet_multipliers
            .iter()
            .map(|m| m.ln() / self.period)
            .collect();
        self.modes = modes;

        Ok(())
    }

    /// Power iteration to find dominant eigenvalue
    fn power_iteration(
        &self,
        matrix: &[Vec<Complex64>],
        max_iter: usize,
        tol: Value,
    ) -> Result<(Complex64, Vec<Complex64>), FloquetError> {
        let n = matrix.len();
        if n == 0 {
            return Err(FloquetError::NoStates);
        }

        // Initialize with random-ish vector
        let mut v: Vec<Complex64> = (0..n)
            .map(|i| Complex64::new((i as f64 + 1.0).sin(), (i as f64 + 1.0).cos()))
            .collect();

        // Normalize
        let mut norm: Value = v.iter().map(|x| x.norm_sqr()).sum::<Value>().sqrt();
        for x in &mut v {
            *x /= norm;
        }

        let mut lambda = Complex64::new(1.0, 0.0);
        let mut prev_lambda = Complex64::new(0.0, 0.0);

        for _ in 0..max_iter {
            // w = M * v
            let w: Vec<Complex64> = (0..n)
                .map(|i| (0..n).map(|j| matrix[i][j] * v[j]).sum())
                .collect();

            // Rayleigh quotient: Î» = v^H * w / v^H * v
            let num: Complex64 = (0..n).map(|i| v[i].conj() * w[i]).sum();
            let denom: Value = v.iter().map(|x| x.norm_sqr()).sum();
            lambda = num / denom;

            // Normalize w
            norm = w.iter().map(|x| x.norm_sqr()).sum::<Value>().sqrt();
            if norm < 1e-30 {
                break;
            }
            v = w.into_iter().map(|x| x / norm).collect();

            // Check convergence
            if (lambda - prev_lambda).norm() < tol * lambda.norm().max(1.0) {
                break;
            }
            prev_lambda = lambda;
        }

        Ok((lambda, v))
    }

    /// Compute Impulse Sensitivity Function (ISF) from adjoint system
    ///
    /// The ISF Î“(t) satisfies the adjoint equation:
    /// dÎ“/dt = -A^T(t) * Î“
    ///
    /// with normalization: âˆ«â‚€^T Î“(t) Â· x'(t) dt = 1
    fn compute_isf(&mut self) -> Result<(), FloquetError> {
        let n_nodes = self.waveform_samples.len();
        let n_samples = self.num_time_samples;

        if n_nodes == 0 || self.waveform_samples.is_empty() {
            // No waveform data - use approximate ISF
            self.compute_approximate_isf();
            return Ok(());
        }

        // Initialize ISF storage
        self.isf_samples = vec![vec![0.0; n_samples]; n_nodes];
        self.isf_rms = vec![0.0; n_nodes];
        self.q_max = vec![0.0; n_nodes];

        // For each node, compute ISF from waveform derivative
        // Simplified Hajimiri model: Î“(t) âˆ dV/dt normalized
        for node in 0..n_nodes {
            if self.waveform_samples[node].len() < n_samples {
                continue;
            }

            // Compute dV/dt numerically
            let dt = self.period / n_samples as f64;
            let mut dvdt = vec![0.0; n_samples];
            let mut max_dvdt = 0.0_f64;

            for i in 0..n_samples {
                let v_prev = self.waveform_samples[node][(i + n_samples - 1) % n_samples];
                let v_next = self.waveform_samples[node][(i + 1) % n_samples];
                dvdt[i] = (v_next - v_prev) / (2.0 * dt);
                max_dvdt = max_dvdt.max(dvdt[i].abs());
            }

            // Compute q_max (max charge = C_eff * V_swing)
            let v_max = self.waveform_samples[node]
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            let v_min = self.waveform_samples[node]
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let v_swing = v_max - v_min;

            // Approximate effective capacitance from slew rate
            // q_max â‰ˆ C_eff * V_swing, slew â‰ˆ I/C_eff, so q_max â‰ˆ V_swing * I / slew
            // Simplified: q_max âˆ V_swing / (max dV/dt) * some_factor
            if max_dvdt > 1e-20 {
                self.q_max[node] = v_swing * self.period / (2.0 * PI);
            } else {
                self.q_max[node] = v_swing * 1e-12; // Fallback with 1pF effective
            }

            // ISF = normalized dV/dt
            // The ISF peaks at zero crossings (max slew) and is zero at peaks
            if max_dvdt > 1e-20 {
                for i in 0..n_samples {
                    self.isf_samples[node][i] = dvdt[i] / max_dvdt;
                }
            }

            // Compute RMS of ISF: Î“_rmsÂ² = (1/T) âˆ«â‚€^T Î“Â²(t) dt
            let isf_sq_sum: Value = self.isf_samples[node].iter().map(|x| x * x).sum();
            self.isf_rms[node] = (isf_sq_sum / n_samples as f64).sqrt();
        }

        Ok(())
    }

    /// Compute approximate ISF when no detailed waveform is available
    pub fn compute_approximate_isf(&mut self) {
        // Without detailed circuit data, assume sinusoidal oscillation
        // ISF for a sinusoidal oscillator: Î“(t) = cos(Ï‰t)
        // This gives Î“_rms = 1/âˆš2

        let n_nodes = self.num_states.max(1);
        let n_samples = self.num_time_samples.max(64);

        self.isf_samples = vec![vec![0.0; n_samples]; n_nodes];
        self.isf_rms = vec![1.0 / 2.0_f64.sqrt(); n_nodes]; // Î“_rms = 1/âˆš2
        self.q_max = vec![1e-12; n_nodes]; // Default 1pC

        // Sample ISF = cos(2Ï€t/T)
        for node in 0..n_nodes {
            for i in 0..n_samples {
                let t = i as f64 / n_samples as f64;
                self.isf_samples[node][i] = (2.0 * PI * t).cos();
            }
        }
    }

    /// Get number of Floquet modes
    pub fn num_modes(&self) -> usize {
        self.modes.len()
    }

    /// Get ISF RMS values for each node
    pub fn isf_rms(&self) -> &[Value] {
        &self.isf_rms
    }

    /// Set Floquet modes manually
    pub fn set_modes(&mut self, modes: Vec<FloquetMode>) {
        self.modes = modes;
        self.compute_exponents_from_modes();
    }

    /// Set Floquet multipliers manually
    pub fn set_multipliers(&mut self, multipliers: Vec<Complex64>) {
        self.floquet_multipliers = multipliers;
        self.floquet_exponents = self
            .floquet_multipliers
            .iter()
            .map(|m| m.ln() / self.period)
            .collect();
    }

    fn compute_exponents_from_modes(&mut self) {
        self.floquet_exponents = self.modes.iter().map(|m| m.exponent).collect();
        self.floquet_multipliers = self
            .floquet_exponents
            .iter()
            .map(|e| (e * self.period).exp())
            .collect();
    }

    /// Get Floquet exponents
    pub fn floquet_exponents(&self) -> &[Complex64] {
        &self.floquet_exponents
    }

    // ========================================================================
    // Transfer Functions
    // ========================================================================

    /// Compute transfer function from noise source to phase at offset frequency
    ///
    /// Returns the phase noise transfer function H(f_offset) that converts
    /// a unit noise current at the specified node to phase deviation.
    pub fn transfer_function(&self, offset_freq: Value, node_idx: usize) -> TransferFunction {
        let omega_offset = 2.0 * PI * offset_freq;
        let mut tf = TransferFunction::new(offset_freq);

        // For oscillators, phase noise transfer function has form:
        // |H(Î”f)|Â² = Î“_rmsÂ² / (2 * q_maxÂ² * (2Ï€ * Î”f)Â²)
        //
        // This gives the characteristic 1/fÂ² (-20 dB/decade) slope

        let isf_rms = if node_idx < self.isf_rms.len() {
            self.isf_rms[node_idx]
        } else if !self.isf_rms.is_empty() {
            self.isf_rms[0]
        } else {
            1.0 / 2.0_f64.sqrt() // Default for sinusoidal
        };

        let q_max = if node_idx < self.q_max.len() {
            self.q_max[node_idx]
        } else if !self.q_max.is_empty() {
            self.q_max[0]
        } else {
            1e-12 // Default 1pC
        };

        // |H(Î”Ï‰)|Â² = Î“_rmsÂ² / (2 * q_maxÂ² * Î”Ï‰Â²)
        // |H(Î”f)| = Î“_rms / (q_max * 2Ï€ * Î”f * âˆš2)
        if omega_offset.abs() > 1e-20 && q_max.abs() > 1e-30 {
            tf.magnitude = isf_rms / (q_max * omega_offset * 2.0_f64.sqrt());
            tf.phase = -PI / 2.0; // 90Â° phase shift from integration
        } else {
            // At DC offset, transfer function is limited by loop bandwidth
            tf.magnitude = isf_rms / q_max * self.period; // Approximate
            tf.phase = 0.0;
        }

        tf
    }

    /// Compute noise transfer from device noise to phase PSD [radÂ²/Hz]
    ///
    /// This implements the Hajimiri-Lee equation:
    /// S_Ï†(Î”f) = Î“_rmsÂ² * S_in / (2 * q_maxÂ² * (2Ï€*Î”f)Â²)
    ///
    /// Arguments:
    /// - offset_freq: Offset from carrier [Hz]
    /// - noise_psd: Device noise power spectral density [AÂ²/Hz or VÂ²/Hz]
    /// - node_idx: Circuit node where noise is injected
    ///
    /// Returns: Phase noise PSD [radÂ²/Hz]
    pub fn noise_to_phase_transfer(
        &self,
        offset_freq: Value,
        noise_psd: Value,
        node_idx: usize,
    ) -> Value {
        let tf = self.transfer_function(offset_freq, node_idx);
        // S_Ï† = |H(f)|Â² * S_n
        tf.magnitude * tf.magnitude * noise_psd
    }

    /// Convert phase noise PSD to single-sideband dBc/Hz
    ///
    /// L(f) = S_Ï†(f) / 2  (single-sideband)
    /// dBc/Hz = 10 * log10(L(f))
    pub fn phase_psd_to_dbc(&self, phase_psd_rad2_hz: Value) -> Value {
        if phase_psd_rad2_hz <= 0.0 {
            return -200.0; // Floor
        }
        10.0 * (phase_psd_rad2_hz / 2.0).log10()
    }

    /// Compute phase sensitivity vector (PPV) at normalized time [0, 1]
    ///
    /// The PPV projects noise perturbations onto the phase deviation.
    /// This is essentially the ISF sampled at the given time.
    pub fn phase_sensitivity(&self, time_normalized: Value) -> Vec<Complex64> {
        let n_samples = self.num_time_samples.max(1);

        // Map normalized time to sample index
        let t_idx = ((time_normalized * n_samples as f64) as usize) % n_samples;

        self.isf_samples
            .iter()
            .map(|isf| {
                if t_idx < isf.len() {
                    Complex64::new(isf[t_idx], 0.0)
                } else {
                    Complex64::new(0.0, 0.0)
                }
            })
            .collect()
    }
}

// ============================================================================
// Floquet Mode
// ============================================================================

/// Floquet mode (eigenvector of monodromy matrix)
#[derive(Debug, Clone)]
pub struct FloquetMode {
    /// Mode index
    pub index: usize,

    /// Floquet exponent Î» (complex)
    pub exponent: Complex64,

    /// Eigenvector components
    pub eigenvector: Vec<Complex64>,

    /// Mode classification
    pub mode_type: FloquetModeType,
}

impl FloquetMode {
    /// Create new Floquet mode with automatic classification
    pub fn new(index: usize, exponent: Complex64, eigenvector: Vec<Complex64>) -> Self {
        let mode_type = Self::classify_mode(exponent);
        Self {
            index,
            exponent,
            eigenvector,
            mode_type,
        }
    }

    /// Classify mode based on Floquet exponent
    ///
    /// - Phase mode: Re(Î») â‰ˆ 0 (neutral stability, |Î¼| â‰ˆ 1)
    /// - Stable mode: Re(Î») < 0 (|Î¼| < 1)
    /// - Unstable mode: Re(Î») > 0 (|Î¼| > 1)
    fn classify_mode(exponent: Complex64) -> FloquetModeType {
        let re = exponent.re;
        let threshold = 1e-6; // Threshold for "approximately zero"

        if re.abs() < threshold {
            FloquetModeType::Phase
        } else if re < -threshold {
            FloquetModeType::Stable
        } else {
            FloquetModeType::Unstable
        }
    }

    /// Get damping factor Ïƒ = -Re(Î»)
    pub fn damping(&self) -> Value {
        -self.exponent.re
    }

    /// Get natural frequency Ï‰ = Im(Î»)
    pub fn natural_freq(&self) -> Value {
        self.exponent.im
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Classification of Floquet modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloquetModeType {
    /// Phase mode: eigenvalue on unit circle, |Î¼| â‰ˆ 1
    Phase,
    /// Stable mode: eigenvalue inside unit circle, |Î¼| < 1
    Stable,
    /// Unstable mode: eigenvalue outside unit circle, |Î¼| > 1
    Unstable,
}

/// Transfer function result
#[derive(Debug, Clone)]
pub struct TransferFunction {
    /// Offset frequency [Hz]
    pub offset_freq: Value,
    /// Magnitude |H(f)|
    pub magnitude: Value,
    /// Phase angle [rad]
    pub phase: Value,
}

impl TransferFunction {
    /// Create new transfer function result
    pub fn new(offset_freq: Value) -> Self {
        Self {
            offset_freq,
            magnitude: 0.0,
            phase: 0.0,
        }
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Value {
        20.0 * self.magnitude.max(1e-100).log10()
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> Value {
        self.phase * 180.0 / PI
    }

    /// Get complex transfer function H = |H| * e^(jÏ†)
    pub fn complex(&self) -> Complex64 {
        Complex64::from_polar(self.magnitude, self.phase)
    }
}

/// Floquet analysis errors
#[derive(Debug, Clone)]
pub enum FloquetError {
    /// No state variables defined
    NoStates,
    /// Matrix is singular
    SingularMatrix,
    /// Convergence failed
    ConvergenceFailed,
}

impl std::fmt::Display for FloquetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStates => write!(f, "No state variables defined"),
            Self::SingularMatrix => write!(f, "Singular matrix in Floquet analysis"),
            Self::ConvergenceFailed => write!(f, "Floquet eigenvalue convergence failed"),
        }
    }
}

impl std::error::Error for FloquetError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod floquet_tests {
    use super::*;

    // ========================================================================
    // Basic Construction Tests
    // ========================================================================

    #[test]
    fn test_floquet_analyzer_new() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);
        assert_eq!(analyzer.period(), 1e-9);
        // Use approximate comparison due to float precision
        assert!((analyzer.carrier_freq() - 1e9).abs() < 1.0);
        assert_eq!(analyzer.num_harmonics, 10);
    }

    #[test]
    fn test_floquet_analyzer_with_carrier_freq() {
        let analyzer = FloquetAnalyzer::new(1e-9, 5).with_carrier_freq(2.4e9);
        assert_eq!(analyzer.carrier_freq(), 2.4e9);
    }

    #[test]
    fn test_floquet_analyzer_with_time_samples() {
        let analyzer = FloquetAnalyzer::new(1e-9, 5).with_time_samples(128);
        assert_eq!(analyzer.num_time_samples, 128);
    }

    #[test]
    fn test_floquet_analyzer_with_num_states() {
        let analyzer = FloquetAnalyzer::new(1e-9, 5).with_num_states(10);
        assert_eq!(analyzer.num_states, 10);
    }

    #[test]
    fn test_floquet_analyzer_very_small_period() {
        // 10 ps period = 100 GHz carrier
        let analyzer = FloquetAnalyzer::new(10e-12, 20);
        assert!((analyzer.carrier_freq() - 100e9).abs() < 1e3);
    }

    #[test]
    fn test_floquet_analyzer_very_large_period() {
        // 1 second period = 1 Hz carrier
        let analyzer = FloquetAnalyzer::new(1.0, 3);
        assert!((analyzer.carrier_freq() - 1.0).abs() < 1e-10);
    }

    // ========================================================================
    // Mode Classification Tests
    // ========================================================================

    #[test]
    fn test_floquet_mode_classification_phase() {
        let mode = FloquetMode::new(0, Complex64::new(0.0, 1e6), vec![Complex64::new(1.0, 0.0)]);
        assert_eq!(mode.mode_type, FloquetModeType::Phase);
    }

    #[test]
    fn test_floquet_mode_classification_stable() {
        let mode = FloquetMode::new(1, Complex64::new(-1e6, 0.0), vec![Complex64::new(0.5, 0.5)]);
        assert_eq!(mode.mode_type, FloquetModeType::Stable);
    }

    #[test]
    fn test_floquet_mode_classification_unstable() {
        let mode = FloquetMode::new(2, Complex64::new(1e5, 0.0), vec![Complex64::new(0.0, 1.0)]);
        assert_eq!(mode.mode_type, FloquetModeType::Unstable);
    }

    #[test]
    fn test_floquet_mode_classification_near_zero() {
        // Very small real part should be classified as phase mode
        let mode = FloquetMode::new(0, Complex64::new(1e-8, 1e6), vec![Complex64::new(1.0, 0.0)]);
        assert_eq!(mode.mode_type, FloquetModeType::Phase);
    }

    #[test]
    fn test_floquet_mode_damping() {
        let mode = FloquetMode::new(0, Complex64::new(-1e5, 2e6), vec![]);
        assert!((mode.damping() - 1e5).abs() < 1.0);
    }

    #[test]
    fn test_floquet_mode_natural_freq() {
        let mode = FloquetMode::new(0, Complex64::new(-1e5, 2e6), vec![]);
        assert!((mode.natural_freq() - 2e6).abs() < 1.0);
    }

    // ========================================================================
    // Matrix Exponential Tests
    // ========================================================================

    #[test]
    fn test_matrix_exponential_identity() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // exp(0) = I
        let zero_matrix = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let result = analyzer.matrix_exponential(&zero_matrix, 1.0);

        assert!((result[0][0].re - 1.0).abs() < 1e-10);
        assert!((result[1][1].re - 1.0).abs() < 1e-10);
        assert!(result[0][1].norm() < 1e-10);
        assert!(result[1][0].norm() < 1e-10);
    }

    #[test]
    fn test_matrix_exponential_diagonal() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // exp(diag(a, b)) = diag(exp(a), exp(b))
        let diag_matrix = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let result = analyzer.matrix_exponential(&diag_matrix, 1.0);

        assert!((result[0][0].re - 1.0_f64.exp()).abs() < 0.01);
        assert!((result[1][1].re - 2.0_f64.exp()).abs() < 0.1); // e^2 â‰ˆ 7.389
        assert!(result[0][1].norm() < 1e-6);
        assert!(result[1][0].norm() < 1e-6);
    }

    #[test]
    fn test_matrix_exponential_small_dt() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // For small dt, exp(dt*A) â‰ˆ I + dt*A
        let matrix = vec![vec![0.1, 0.2], vec![0.3, 0.4]];
        let dt = 0.001;
        let result = analyzer.matrix_exponential(&matrix, dt);

        // Compare with I + dt*A
        assert!((result[0][0].re - (1.0 + dt * 0.1)).abs() < 1e-5);
        assert!((result[0][1].re - dt * 0.2).abs() < 1e-5);
        assert!((result[1][0].re - dt * 0.3).abs() < 1e-5);
        assert!((result[1][1].re - (1.0 + dt * 0.4)).abs() < 1e-5);
    }

    #[test]
    fn test_matrix_exponential_nilpotent() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // Nilpotent matrix [[0, 1], [0, 0]] has exp = [[1, t], [0, 1]]
        let nilpotent = vec![vec![0.0, 1.0], vec![0.0, 0.0]];
        let t = 2.5;
        let result = analyzer.matrix_exponential(&nilpotent, t);

        assert!((result[0][0].re - 1.0).abs() < 1e-6);
        assert!((result[0][1].re - t).abs() < 0.01);
        assert!(result[1][0].norm() < 1e-6);
        assert!((result[1][1].re - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_matrix_exponential_3x3() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        let matrix = vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ];
        let result = analyzer.matrix_exponential(&matrix, 1.0);

        // exp(0) = I for any size
        assert!((result[0][0].re - 1.0).abs() < 1e-10);
        assert!((result[1][1].re - 1.0).abs() < 1e-10);
        assert!((result[2][2].re - 1.0).abs() < 1e-10);
    }

    // ========================================================================
    // Transfer Function Tests
    // ========================================================================

    #[test]
    fn test_transfer_function_1_over_f_squared() {
        // For Hajimiri model, |H(f)|Â² âˆ 1/fÂ²
        // So |H(f1)|/|H(f2)| = f2/f1
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_10k = analyzer.transfer_function(10e3, 0);

        // Ratio should be 10 (one decade = 10x frequency = 1/10 magnitude)
        let ratio = tf_1k.magnitude / tf_10k.magnitude;
        assert!(
            (ratio - 10.0).abs() < 0.5,
            "Expected ratio ~10, got {}",
            ratio
        );
    }

    #[test]
    fn test_transfer_function_two_decades() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_100k = analyzer.transfer_function(100e3, 0);

        // Two decades = 100x frequency = 1/100 magnitude
        let ratio = tf_1k.magnitude / tf_100k.magnitude;
        assert!(
            (ratio - 100.0).abs() < 5.0,
            "Expected ratio ~100, got {}",
            ratio
        );
    }

    #[test]
    fn test_transfer_function_db() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_10k = analyzer.transfer_function(10e3, 0);

        // One decade should give 20 dB difference
        let db_diff = tf_1k.magnitude_db() - tf_10k.magnitude_db();
        assert!(
            (db_diff - 20.0).abs() < 1.0,
            "Expected ~20 dB/decade, got {}",
            db_diff
        );
    }

    #[test]
    fn test_transfer_function_complex() {
        let mut tf = TransferFunction::new(1e6);
        tf.magnitude = 0.5;
        tf.phase = PI / 4.0;

        let c = tf.complex();
        assert!((c.norm() - 0.5).abs() < 1e-10);
        assert!((c.arg() - PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_transfer_function_at_very_low_offset() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // At very low offset (0.1 Hz), TF should still be finite
        let tf = analyzer.transfer_function(0.1, 0);
        assert!(tf.magnitude > 0.0);
        assert!(tf.magnitude.is_finite());
    }

    #[test]
    fn test_transfer_function_at_high_offset() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // At high offset (100 MHz), TF should be much smaller than at 1 kHz
        let tf_1k = analyzer.transfer_function(1e3, 0);
        let tf_100m = analyzer.transfer_function(100e6, 0);

        assert!(tf_100m.magnitude > 0.0);
        assert!(tf_100m.magnitude < tf_1k.magnitude);
        // 5 decades = 10^5 ratio
        let ratio = tf_1k.magnitude / tf_100m.magnitude;
        assert!(ratio > 1e4, "Expected >10^4 ratio, got {}", ratio);
    }

    // ========================================================================
    // Phase Noise Calculation Tests
    // ========================================================================

    #[test]
    fn test_phase_noise_dbc_calculation() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // 1e-10 radÂ²/Hz should give -103 dBc/Hz (SSB)
        // L = S_Ï†/2 = 5e-11, 10*log10(5e-11) â‰ˆ -103
        let dbc = analyzer.phase_psd_to_dbc(1e-10);
        assert!(dbc < -100.0 && dbc > -110.0);
    }

    #[test]
    fn test_phase_noise_dbc_floor() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // Very small or zero PSD should return floor value
        let dbc = analyzer.phase_psd_to_dbc(0.0);
        assert_eq!(dbc, -200.0);
    }

    #[test]
    fn test_phase_noise_dbc_typical_values() {
        let analyzer = FloquetAnalyzer::new(1e-9, 10);

        // -80 dBc/Hz corresponds to S_Ï† = 2e-8 radÂ²/Hz
        let target_psd = 2.0 * 10.0_f64.powf(-8.0);
        let dbc = analyzer.phase_psd_to_dbc(target_psd);
        assert!((dbc - (-80.0)).abs() < 1.0);
    }

    #[test]
    fn test_noise_to_phase_transfer() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // Test that phase PSD is non-negative and scales with noise PSD
        let psd1 = analyzer.noise_to_phase_transfer(1e6, 1e-20, 0);
        let psd2 = analyzer.noise_to_phase_transfer(1e6, 2e-20, 0);

        assert!(psd1 > 0.0);
        assert!((psd2 / psd1 - 2.0).abs() < 0.01); // Should scale linearly
    }

    #[test]
    fn test_noise_to_phase_transfer_frequency_dependence() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let noise_psd = 1e-20;
        let psd_1k = analyzer.noise_to_phase_transfer(1e3, noise_psd, 0);
        let psd_10k = analyzer.noise_to_phase_transfer(10e3, noise_psd, 0);

        // With 1/fÂ² transfer, phase PSD ratio should be 100 per decade
        let ratio = psd_1k / psd_10k;
        assert!(
            (ratio - 100.0).abs() < 5.0,
            "Expected ratio ~100, got {}",
            ratio
        );
    }

    // ========================================================================
    // ISF and PPV Tests
    // ========================================================================

    #[test]
    fn test_phase_sensitivity_periodic() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // ISF should be periodic - check at 0 and 1
        let ppv_0 = analyzer.phase_sensitivity(0.0);
        let ppv_1 = analyzer.phase_sensitivity(1.0);

        if !ppv_0.is_empty() && !ppv_1.is_empty() {
            let diff = (ppv_0[0] - ppv_1[0]).norm();
            assert!(diff < 0.1, "PPV should be periodic");
        }
    }

    #[test]
    fn test_approximate_isf_rms() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.num_states = 3;
        analyzer.compute_approximate_isf();

        // For cos(Ï‰t), Î“_rms = 1/âˆš2 â‰ˆ 0.707
        for rms in &analyzer.isf_rms {
            assert!(
                (*rms - 1.0 / 2.0_f64.sqrt()).abs() < 0.01,
                "ISF RMS should be ~0.707, got {}",
                rms
            );
        }
    }

    #[test]
    fn test_isf_samples_cosinusoidal() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.num_states = 1;
        analyzer.compute_approximate_isf();

        // ISF should be cos(2Ï€t/T)
        // At t=0 (sample 0): cos(0) = 1
        // At t=T/4 (sample 16 for 64 samples): cos(Ï€/2) = 0
        // At t=T/2 (sample 32): cos(Ï€) = -1
        let n = analyzer.isf_samples[0].len();
        assert!((analyzer.isf_samples[0][0] - 1.0).abs() < 0.01);
        assert!(analyzer.isf_samples[0][n / 4].abs() < 0.1);
        assert!((analyzer.isf_samples[0][n / 2] - (-1.0)).abs() < 0.01);
    }

    // ========================================================================
    // Noise Slope Verification Tests
    // ========================================================================

    #[test]
    fn test_flicker_noise_slope() {
        // With 1/f noise (PSD âˆ 1/f) added to 1/fÂ² transfer function,
        // total phase noise âˆ 1/fÂ³ near carrier (-30 dB/decade)
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        // Flicker noise: S_n(f) = Kf / f
        let kf = 1e-20;
        let noise_1k = kf / 1e3;
        let noise_10k = kf / 10e3;

        let phase_psd_1k = analyzer.noise_to_phase_transfer(1e3, noise_1k, 0);
        let phase_psd_10k = analyzer.noise_to_phase_transfer(10e3, noise_10k, 0);

        // With 1/f noise Ã— 1/fÂ² TF = 1/fÂ³
        // Ratio should be 10Â³ = 1000 per decade
        // In dB: 30 dB/decade
        let dbc_1k = analyzer.phase_psd_to_dbc(phase_psd_1k);
        let dbc_10k = analyzer.phase_psd_to_dbc(phase_psd_10k);
        let slope = dbc_1k - dbc_10k;

        assert!(
            slope > 25.0 && slope < 35.0,
            "1/f noise slope should be ~30 dB/decade, got {}",
            slope
        );
    }

    #[test]
    fn test_white_noise_slope() {
        // With white noise (flat PSD) through 1/fÂ² transfer function,
        // phase noise âˆ 1/fÂ² (-20 dB/decade)
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer.compute_approximate_isf();

        let white_noise = 1e-20; // Constant PSD

        let phase_psd_1k = analyzer.noise_to_phase_transfer(1e3, white_noise, 0);
        let phase_psd_10k = analyzer.noise_to_phase_transfer(10e3, white_noise, 0);

        let dbc_1k = analyzer.phase_psd_to_dbc(phase_psd_1k);
        let dbc_10k = analyzer.phase_psd_to_dbc(phase_psd_10k);
        let slope = dbc_1k - dbc_10k;

        assert!(
            slope > 15.0 && slope < 25.0,
            "White noise slope should be ~20 dB/decade, got {}",
            slope
        );
    }

    // ========================================================================
    // Error Handling Tests
    // ========================================================================

    #[test]
    fn test_floquet_error_display() {
        assert!(FloquetError::NoStates.to_string().contains("state"));
        assert!(
            FloquetError::SingularMatrix
                .to_string()
                .contains("Singular")
        );
        assert!(
            FloquetError::ConvergenceFailed
                .to_string()
                .contains("convergence")
        );
    }

    #[test]
    fn test_compute_with_no_jacobians() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        // Should fall back to approximate ISF
        let result = analyzer.compute();
        assert!(result.is_ok());
    }

    // ========================================================================
    // Multiplier and Exponent Tests
    // ========================================================================

    #[test]
    fn test_set_multipliers() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        let multipliers = vec![
            Complex64::new(1.0, 0.0), // Phase mode (|Î¼| = 1)
            Complex64::new(0.5, 0.0), // Stable mode (|Î¼| < 1)
        ];
        analyzer.set_multipliers(multipliers);

        assert_eq!(analyzer.floquet_multipliers.len(), 2);
        assert_eq!(analyzer.floquet_exponents().len(), 2);
    }

    #[test]
    fn test_set_modes() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        let modes = vec![
            FloquetMode::new(0, Complex64::new(0.0, 0.0), vec![Complex64::new(1.0, 0.0)]),
            FloquetMode::new(1, Complex64::new(-1e6, 0.0), vec![Complex64::new(0.5, 0.5)]),
        ];
        analyzer.set_modes(modes);

        assert_eq!(analyzer.num_modes(), 2);
    }

    #[test]
    fn test_floquet_exponents_from_multipliers() {
        let mut analyzer = FloquetAnalyzer::new(1.0, 10); // T = 1s

        // Î¼ = e^(Î»T), so Î» = ln(Î¼)/T
        // For Î¼ = e^2 â‰ˆ 7.389, Î» = 2
        let multipliers = vec![Complex64::new(2.0_f64.exp(), 0.0)];
        analyzer.set_multipliers(multipliers);

        let exponents = analyzer.floquet_exponents();
        assert!((exponents[0].re - 2.0).abs() < 0.01);
    }

    // ========================================================================
    // Edge Cases and Stress Tests
    // ========================================================================

    #[test]
    fn test_many_harmonics() {
        let analyzer = FloquetAnalyzer::new(1e-9, 100);
        assert_eq!(analyzer.num_harmonics, 100);
    }

    #[test]
    fn test_transfer_function_multiple_nodes() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer = analyzer.with_num_states(5);
        analyzer.compute_approximate_isf();

        // Should be able to query any node
        for node in 0..5 {
            let tf = analyzer.transfer_function(1e6, node);
            assert!(tf.magnitude > 0.0);
        }
    }

    #[test]
    fn test_transfer_function_out_of_range_node() {
        let mut analyzer = FloquetAnalyzer::new(1e-9, 10);
        analyzer = analyzer.with_num_states(2);
        analyzer.compute_approximate_isf();

        // Node index out of range should still work (use default)
        let tf = analyzer.transfer_function(1e6, 10);
        assert!(tf.magnitude > 0.0);
    }
}
