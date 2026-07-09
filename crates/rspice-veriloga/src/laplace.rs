//! Laplace (s-domain) filter implementation for Verilog-A
//!
//! This module provides state-space representations for continuous-time
//! transfer functions in the s-domain, supporting both pole-zero and
//! numerator-denominator forms.
//!
//! ## Theory
//!
//! For a transfer function H(s) = N(s)/D(s), we convert to state-space form:
//!   dx/dt = A*x + B*u
//!   y = C*x + D*u
//!
//! The controllable canonical form is used for the conversion.
//!
//! ## Time Integration
//!
//! Backward Euler integration is used for numerical stability:
//!   x[n] = (I - h*A)^(-1) * (x[n-1] + h*B*u[n])
//!
//! where h is the timestep.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// Convert (re, im) root pairs into real polynomial coefficients in
/// ascending powers of s. Errors when the roots do not form conjugate pairs.
pub fn roots_to_polynomial(roots: &[(f64, f64)]) -> Result<Vec<f64>, String> {
    StateSpaceFilter::roots_to_polynomial_ascending(roots)
}

/// Complex number representation for poles and zeros
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn abs2(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.abs2();
        if denom.abs() < 1e-30 {
            return Self::new(f64::INFINITY, f64::INFINITY);
        }
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / denom,
            (self.im * rhs.re - self.re * rhs.im) / denom,
        )
    }
}

/// State-space filter representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpaceFilter {
    /// State matrix A (n x n)
    pub a: Vec<Vec<f64>>,
    /// Input matrix B (n x 1)
    pub b: Vec<f64>,
    /// Output matrix C (1 x n)
    pub c: Vec<f64>,
    /// Feedthrough coefficient D (scalar)
    pub d: f64,
    /// Candidate state vector for the in-flight timestep
    pub state: Vec<f64>,
    /// State at the last accepted timestep
    pub state_prev: Vec<f64>,
    /// System order
    pub order: usize,
    /// DC gain for normalization
    pub dc_gain: f64,
}

impl StateSpaceFilter {
    /// Create a new state-space filter from coefficients
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>, d: f64) -> Self {
        let order = b.len();
        let state = vec![0.0; order];
        let state_prev = vec![0.0; order];

        Self {
            a,
            b,
            c,
            d,
            state,
            state_prev,
            order,
            dc_gain: 1.0,
        }
    }

    /// Create from numerator and denominator polynomials (descending powers of s)
    ///
    /// H(s) = (b_n*s^n + ... + b_1*s + b_0) / (a_m*s^m + ... + a_1*s + a_0)
    pub fn from_transfer_function(numerator: &[f64], denominator: &[f64]) -> Self {
        let n = denominator.len() - 1; // Order of the system

        if n == 0 {
            // Static gain only
            let gain = if denominator[0].abs() > 1e-15 {
                numerator.first().copied().unwrap_or(1.0) / denominator[0]
            } else {
                1.0
            };
            return Self {
                a: vec![],
                b: vec![],
                c: vec![],
                d: gain,
                state: vec![],
                state_prev: vec![],
                order: 0,
                dc_gain: gain,
            };
        }

        // Normalize by leading coefficient
        let a_n = denominator[0];
        if a_n.abs() < 1e-15 {
            return Self::unity_gain();
        }

        let denom_norm: Vec<f64> = denominator.iter().map(|x| x / a_n).collect();

        // Pad numerator if shorter than denominator
        let mut num_norm = vec![0.0; denominator.len()];
        let offset = denominator.len() - numerator.len();
        for (i, &coeff) in numerator.iter().enumerate() {
            num_norm[offset + i] = coeff / a_n;
        }

        // Build controllable canonical form
        let mut a_matrix = vec![vec![0.0; n]; n];
        let mut b_vec = vec![0.0; n];
        let mut c_vec = vec![0.0; n];

        // A matrix: companion form
        for i in 0..n - 1 {
            a_matrix[i][i + 1] = 1.0;
        }
        for i in 0..n {
            a_matrix[n - 1][i] = -denom_norm[n - i];
        }

        // B vector
        b_vec[n - 1] = 1.0;

        // C vector and D scalar
        let d_scalar = num_norm[0];
        for i in 0..n {
            c_vec[i] = num_norm[n - i] - num_norm[0] * denom_norm[n - i];
        }

        // Compute DC gain: H(0) = b_0 / a_0
        let dc_gain = if denom_norm[n].abs() > 1e-15 {
            num_norm[n] / denom_norm[n]
        } else {
            1.0
        };

        Self {
            a: a_matrix,
            b: b_vec,
            c: c_vec,
            d: d_scalar,
            state: vec![0.0; n],
            state_prev: vec![0.0; n],
            order: n,
            dc_gain,
        }
    }

    /// Create from poles and zeros with gain
    ///
    /// H(s) = gain * prod(s - zeros) / prod(s - poles)
    pub fn from_poles_zeros(poles: &[Complex], zeros: &[Complex], gain: f64) -> Self {
        if poles.is_empty() && zeros.is_empty() {
            return Self {
                a: vec![],
                b: vec![],
                c: vec![],
                d: gain,
                state: vec![],
                state_prev: vec![],
                order: 0,
                dc_gain: gain,
            };
        }

        // Convert poles/zeros to polynomial coefficients
        let numerator = Self::roots_to_poly(zeros);
        let denominator = Self::roots_to_poly(poles);

        // Scale numerator by gain
        let scaled_num: Vec<f64> = numerator.iter().map(|x| x * gain).collect();

        Self::from_transfer_function(&scaled_num, &denominator)
    }

    /// Convert roots to polynomial coefficients (descending powers)
    fn roots_to_poly(roots: &[Complex]) -> Vec<f64> {
        if roots.is_empty() {
            return vec![1.0];
        }

        // Start with (s - r_0)
        let mut poly = vec![Complex::real(1.0), Complex::new(-roots[0].re, -roots[0].im)];

        // Multiply by each (s - r_i)
        for root in roots.iter().skip(1) {
            let mut new_poly = vec![Complex::real(0.0); poly.len() + 1];

            // Multiply by s
            for (i, &coeff) in poly.iter().enumerate() {
                new_poly[i].re += coeff.re;
                new_poly[i].im += coeff.im;
            }

            // Multiply by -root
            for (i, &coeff) in poly.iter().enumerate() {
                new_poly[i + 1].re -= coeff.re * root.re - coeff.im * root.im;
                new_poly[i + 1].im -= coeff.re * root.im + coeff.im * root.re;
            }

            poly = new_poly;
        }

        // Extract real parts (imaginary should be ~0 for conjugate pairs)
        poly.iter().map(|c| c.re).collect()
    }

    /// Convert (re, im) root pairs to real polynomial coefficients in
    /// ascending powers of s, validating that complex roots cancel.
    pub fn roots_to_polynomial_ascending(roots: &[(f64, f64)]) -> Result<Vec<f64>, String> {
        let complex_roots: Vec<Complex> =
            roots.iter().map(|&(re, im)| Complex::new(re, im)).collect();

        // Validate that the imaginary parts cancel (conjugate pairs)
        let mut im_sum = 0.0;
        for root in &complex_roots {
            im_sum += root.im;
        }
        if im_sum.abs() > 1e-9 {
            return Err("complex roots must come in conjugate pairs".to_string());
        }

        let mut descending = Self::roots_to_poly(&complex_roots);
        descending.reverse();
        Ok(descending)
    }

    /// Create a unity gain passthrough filter
    pub fn unity_gain() -> Self {
        Self {
            a: vec![],
            b: vec![],
            c: vec![],
            d: 1.0,
            state: vec![],
            state_prev: vec![],
            order: 0,
            dc_gain: 1.0,
        }
    }

    /// Create a first-order lowpass filter with cutoff frequency
    /// H(s) = omega_c / (s + omega_c)
    pub fn lowpass_first_order(cutoff_hz: f64) -> Self {
        let omega_c = 2.0 * PI * cutoff_hz;
        Self::from_transfer_function(&[omega_c], &[1.0, omega_c])
    }

    /// Create a second-order lowpass filter (Butterworth)
    /// H(s) = omega_n^2 / (s^2 + 2*zeta*omega_n*s + omega_n^2)
    pub fn lowpass_second_order(cutoff_hz: f64, damping: f64) -> Self {
        let omega_n = 2.0 * PI * cutoff_hz;
        let omega_n2 = omega_n * omega_n;
        Self::from_transfer_function(&[omega_n2], &[1.0, 2.0 * damping * omega_n, omega_n2])
    }

    /// Create a differentiator with time constant tau
    /// H(s) = tau*s / (tau*s + 1)
    pub fn differentiator(tau: f64) -> Self {
        Self::from_transfer_function(&[tau, 0.0], &[tau, 1.0])
    }

    /// Create an integrator with time constant tau (leaky)
    /// H(s) = 1 / (tau*s + 1)
    pub fn integrator(tau: f64) -> Self {
        Self::from_transfer_function(&[1.0], &[tau, 1.0])
    }

    /// Evaluate an in-flight Backward Euler candidate.
    ///
    /// This computes: x[n] = (I - h*A)^(-1) * (x[n-1] + h*B*u[n])
    ///                y[n] = C*x[n] + D*u[n]
    ///
    /// Repeated calls recompute from `state_prev`, which is the last accepted
    /// state. The simulator calls [`Self::commit`] only after accepting the
    /// timestep, keeping Newton reevaluations idempotent.
    pub fn step(&mut self, input: f64, timestep: f64) -> f64 {
        if self.order == 0 {
            return self.d * input;
        }

        // For small systems (order <= 2), use direct formulas
        // For larger systems, use Gauss elimination
        match self.order {
            1 => self.step_first_order(input, timestep),
            2 => self.step_second_order(input, timestep),
            _ => self.step_general(input, timestep),
        }
    }

    /// Commit the most recently evaluated candidate.
    pub fn commit(&mut self) {
        self.state_prev.copy_from_slice(&self.state);
    }

    /// First-order Backward Euler step
    fn step_first_order(&mut self, input: f64, h: f64) -> f64 {
        // dx/dt = a*x + b*u
        // (1 - h*a)*x_new = x_old + h*b*u
        let a = self.a[0][0];
        let b = self.b[0];
        let c = self.c[0];

        let denom = 1.0 - h * a;
        if denom.abs() > 1e-15 {
            self.state[0] = (self.state_prev[0] + h * b * input) / denom;
        }

        c * self.state[0] + self.d * input
    }

    /// Second-order Backward Euler step
    fn step_second_order(&mut self, input: f64, h: f64) -> f64 {
        // Build I - h*A
        let a00 = 1.0 - h * self.a[0][0];
        let a01 = -h * self.a[0][1];
        let a10 = -h * self.a[1][0];
        let a11 = 1.0 - h * self.a[1][1];

        // RHS: x_old + h*B*u
        let rhs0 = self.state_prev[0] + h * self.b[0] * input;
        let rhs1 = self.state_prev[1] + h * self.b[1] * input;

        // Solve 2x2 system using Cramer's rule
        let det = a00 * a11 - a01 * a10;
        if det.abs() > 1e-15 {
            self.state[0] = (rhs0 * a11 - rhs1 * a01) / det;
            self.state[1] = (rhs1 * a00 - rhs0 * a10) / det;
        }

        // Output: y = C*x + D*u
        self.c[0] * self.state[0] + self.c[1] * self.state[1] + self.d * input
    }

    /// General order Backward Euler step (Gauss elimination)
    fn step_general(&mut self, input: f64, h: f64) -> f64 {
        let n = self.order;

        // Build the system matrix (I - h*A)
        let mut mat = vec![vec![0.0; n]; n];
        for (i, row) in mat.iter_mut().enumerate().take(n) {
            for (j, cell) in row.iter_mut().enumerate().take(n) {
                *cell = if i == j { 1.0 } else { 0.0 };
                *cell -= h * self.a[i][j];
            }
        }

        // Build RHS: x_prev + h*B*u
        let mut rhs: Vec<f64> = self
            .state_prev
            .iter()
            .zip(self.b.iter())
            .map(|(&x_prev, &b)| x_prev + h * b * input)
            .collect();

        // Gaussian elimination with partial pivoting
        for k in 0..n {
            // Find pivot
            let mut max_row = k;
            let mut max_val = mat[k][k].abs();
            for (i, row) in mat.iter().enumerate().skip(k + 1) {
                if row[k].abs() > max_val {
                    max_val = row[k].abs();
                    max_row = i;
                }
            }

            // Swap rows
            if max_row != k {
                mat.swap(k, max_row);
                rhs.swap(k, max_row);
            }

            // Eliminate
            let pivot = mat[k][k];
            if pivot.abs() < 1e-15 {
                continue;
            }

            let pivot_row = mat[k].clone();
            for (i, row) in mat.iter_mut().enumerate().skip(k + 1) {
                let factor = row[k] / pivot;
                row[k] = 0.0;
                for (j, cell) in row.iter_mut().enumerate().skip(k + 1) {
                    *cell -= factor * pivot_row[j];
                }
                rhs[i] -= factor * rhs[k];
            }
        }

        // Back substitution
        for i in (0..n).rev() {
            let pivot = mat[i][i];
            if pivot.abs() < 1e-15 {
                self.state[i] = 0.0;
                continue;
            }
            let mut sum = rhs[i];
            for (&mat_coeff, &state_coeff) in mat[i].iter().zip(self.state.iter()).skip(i + 1) {
                sum -= mat_coeff * state_coeff;
            }
            self.state[i] = sum / pivot;
        }

        // Output: y = C*x + D*u
        let mut output = self.d * input;
        for i in 0..n {
            output += self.c[i] * self.state[i];
        }
        output
    }

    /// Get DC output (s=0) for a given input
    pub fn dc_output(&self, input: f64) -> f64 {
        self.dc_gain * input
    }

    /// Reset filter state to zero
    pub fn reset(&mut self) {
        for x in &mut self.state {
            *x = 0.0;
        }
        for x in &mut self.state_prev {
            *x = 0.0;
        }
    }

    /// Set initial state
    pub fn set_initial_state(&mut self, initial: &[f64]) {
        for (i, &val) in initial.iter().enumerate() {
            if i < self.state.len() {
                self.state[i] = val;
                self.state_prev[i] = val;
            }
        }
    }

    /// Compute frequency response at given frequency (Hz)
    pub fn frequency_response(&self, freq_hz: f64) -> (f64, f64) {
        let omega = 2.0 * PI * freq_hz;

        if self.order == 0 {
            return (self.d.abs(), if self.d >= 0.0 { 0.0 } else { PI });
        }

        // Evaluate H(jw) = C * (jwI - A)^(-1) * B + D directly from the
        // state-space form so the response stays consistent with the runtime
        // filter realization instead of relying on reconstructed polynomials.
        let state = match solve_complex_system(&self.a, &self.b, omega) {
            Some(state) => state,
            None => return (f64::INFINITY, 0.0),
        };

        let mut response = Complex::real(self.d);
        for (&c, state_value) in self.c.iter().zip(state.iter()) {
            response += Complex::real(c) * *state_value;
        }

        (response.magnitude(), response.phase())
    }
}

fn solve_complex_system(a: &[Vec<f64>], b: &[f64], omega: f64) -> Option<Vec<Complex>> {
    let n = b.len();
    if n == 0 {
        return Some(Vec::new());
    }

    let mut mat = vec![vec![Complex::real(0.0); n]; n];
    let mut rhs = vec![Complex::real(0.0); n];

    for i in 0..n {
        rhs[i] = Complex::real(b[i]);
        for j in 0..n {
            let imag = if i == j { omega } else { 0.0 };
            mat[i][j] = Complex::new(-a[i][j], imag);
        }
    }

    for k in 0..n {
        let mut pivot_row = k;
        let mut pivot_mag = mat[k][k].magnitude();
        for (row_idx, row) in mat.iter().enumerate().skip(k + 1) {
            let mag = row[k].magnitude();
            if mag > pivot_mag {
                pivot_mag = mag;
                pivot_row = row_idx;
            }
        }

        if pivot_mag < 1e-18 {
            return None;
        }

        if pivot_row != k {
            mat.swap(k, pivot_row);
            rhs.swap(k, pivot_row);
        }

        let pivot = mat[k][k];
        let pivot_row_values = mat[k].clone();
        let rhs_pivot = rhs[k];
        for row_idx in (k + 1)..n {
            let factor = mat[row_idx][k] / pivot;
            mat[row_idx][k] = Complex::real(0.0);
            for col_idx in (k + 1)..n {
                mat[row_idx][col_idx] = mat[row_idx][col_idx] - factor * pivot_row_values[col_idx];
            }
            rhs[row_idx] = rhs[row_idx] - factor * rhs_pivot;
        }
    }

    let mut solution = vec![Complex::real(0.0); n];
    for row_idx in (0..n).rev() {
        let pivot = mat[row_idx][row_idx];
        if pivot.magnitude() < 1e-18 {
            return None;
        }
        let mut sum = rhs[row_idx];
        for col_idx in (row_idx + 1)..n {
            sum = sum - mat[row_idx][col_idx] * solution[col_idx];
        }
        solution[row_idx] = sum / pivot;
    }

    Some(solution)
}

/// Laplace transform filter type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaplaceFilter {
    /// Pole-zero form: H(s) = gain * prod(s-zeros) / prod(s-poles)
    PoleZero {
        gain: f64,
        poles: Vec<Complex>,
        zeros: Vec<Complex>,
    },
    /// Numerator-denominator form: H(s) = N(s)/D(s)
    NumDen {
        numerator: Vec<f64>,
        denominator: Vec<f64>,
    },
}

impl LaplaceFilter {
    /// Convert to state-space representation
    pub fn to_state_space(&self) -> StateSpaceFilter {
        match self {
            LaplaceFilter::PoleZero { gain, poles, zeros } => {
                StateSpaceFilter::from_poles_zeros(poles, zeros, *gain)
            }
            LaplaceFilter::NumDen {
                numerator,
                denominator,
            } => StateSpaceFilter::from_transfer_function(numerator, denominator),
        }
    }

    /// Evaluate DC gain
    pub fn dc_gain(&self) -> f64 {
        match self {
            LaplaceFilter::PoleZero { gain, poles, zeros } => {
                // H(0) = gain * prod(-zeros) / prod(-poles)
                let mut dc = *gain;
                for z in zeros {
                    dc *= z.magnitude();
                }
                for p in poles {
                    let mag = p.magnitude();
                    if mag.abs() > 1e-15 {
                        dc /= mag;
                    }
                }
                dc
            }
            LaplaceFilter::NumDen {
                numerator,
                denominator,
            } => {
                // H(0) = N(0)/D(0) = numerator[last] / denominator[last]
                let n0 = numerator.last().copied().unwrap_or(1.0);
                let d0 = denominator.last().copied().unwrap_or(1.0);
                if d0.abs() > 1e-15 { n0 / d0 } else { 1.0 }
            }
        }
    }
}
