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
//!
//! ```text
//! x[n] = (I - h*A)^(-1) * (x[n-1] + h*B*u[n])
//! ```
//!
//! where h is the timestep.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A malformed transfer function or a Laplace evaluation that cannot be
/// represented faithfully in `f64`.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LaplaceError {
    /// The authored pole/zero or coefficient definition is invalid.
    #[error("invalid Laplace transfer function: {0}")]
    InvalidDefinition(String),
    /// A state-space system has no unique solution at the requested point.
    #[error("singular Laplace {0} system")]
    SingularSystem(&'static str),
    /// Runtime inputs or arithmetic cannot produce a finite, faithful value.
    #[error("invalid Laplace evaluation: {0}")]
    InvalidEvaluation(String),
}

/// Convert (re, im) root pairs into real polynomial coefficients in
/// ascending powers of s. Errors when the roots do not form conjugate pairs.
pub fn roots_to_polynomial(roots: &[(f64, f64)]) -> Result<Vec<f64>, String> {
    StateSpaceFilter::roots_to_polynomial_ascending(roots).map_err(|error| error.to_string())
}

/// Return the real zero-frequency gain of a pole-zero transfer function.
///
/// The root arrays contain `(real, imaginary)` pairs and must describe real
/// polynomials: every non-real root therefore needs a conjugate partner.  The
/// constant term is obtained from the same checked root expansion used to
/// construct the runtime state-space realization, so Jacobian lowering cannot
/// reinterpret the flattened real/imaginary storage as independent roots.
pub(crate) fn checked_pole_zero_dc_gain(
    gain: f64,
    zeros: &[(f64, f64)],
    poles: &[(f64, f64)],
) -> Result<f64, LaplaceError> {
    if !gain.is_finite() {
        return Err(LaplaceError::InvalidDefinition(
            "pole-zero DC gain factor must be finite".into(),
        ));
    }

    let zero_constant = checked_root_constant_term("zero", zeros)?;
    let pole_constant = checked_root_constant_term("pole", poles)?;
    let numerator = checked_definition_product(gain, zero_constant, "pole-zero DC numerator")?;
    checked_ratio(numerator, pole_constant, "pole-zero DC gain")
}

fn checked_root_constant_term(
    role: &'static str,
    roots: &[(f64, f64)],
) -> Result<f64, LaplaceError> {
    let constant = StateSpaceFilter::roots_to_polynomial_ascending(roots)?
        .first()
        .copied()
        .unwrap_or(1.0);
    if !constant.is_finite() {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{role} root DC product is non-finite"
        )));
    }
    if constant == 0.0
        && roots
            .iter()
            .all(|(real, imaginary)| *real != 0.0 || *imaginary != 0.0)
    {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{role} root DC product underflows f64"
        )));
    }
    Ok(constant)
}

const PIVOT_RELATIVE_TOLERANCE: f64 = 16.0 * f64::EPSILON;
const ROOT_RELATIVE_TOLERANCE: f64 = 64.0 * f64::EPSILON;

/// State-space filter representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpaceFilter {
    /// State matrix A (n x n)
    a: Vec<Vec<f64>>,
    /// Input matrix B (n x 1)
    b: Vec<f64>,
    /// Output matrix C (1 x n)
    c: Vec<f64>,
    /// Feedthrough coefficient D (scalar)
    d: f64,
    /// Exact checked zero-frequency action retained from the authored
    /// transfer function. Arbitrary state-space systems and legacy
    /// serialized artifacts fall back to the realization solve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    transfer_function_dc_gain: Option<f64>,
    /// Candidate state vector for the in-flight timestep
    state: Vec<f64>,
    /// State at the last accepted timestep
    state_prev: Vec<f64>,
    /// Whether `state` was successfully recreated by the current complete
    /// device evaluation pass.
    #[serde(skip)]
    candidate_valid: bool,
    /// System order
    order: usize,
}

/// Accepted dynamic state for a compiled Laplace realization. The immutable
/// realization matrices remain in the compiled device and are shape-checked
/// before these values are injected.
#[derive(Debug, Clone, PartialEq)]
pub struct LaplaceCheckpoint {
    pub state: Vec<f64>,
}

impl StateSpaceFilter {
    /// Create a new state-space filter from coefficients
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>, d: f64) -> Result<Self, LaplaceError> {
        let order = b.len();
        let state = vec![0.0; order];
        let state_prev = vec![0.0; order];

        let filter = Self {
            a,
            b,
            c,
            d,
            transfer_function_dc_gain: None,
            state,
            state_prev,
            candidate_valid: false,
            order,
        };
        filter.validate_structure()?;
        Ok(filter)
    }

    /// Create from numerator and denominator polynomials (descending powers of s).
    ///
    /// H(s) = (b_n*s^n + ... + b_1*s + b_0) / (a_m*s^m + ... + a_1*s + a_0)
    ///
    /// Improper transfer functions are refused because this state-space
    /// realization has no input-derivative channel. Silently truncating their
    /// numerator would produce a different circuit.
    pub fn from_transfer_function(
        numerator: &[f64],
        denominator: &[f64],
    ) -> Result<Self, LaplaceError> {
        validate_finite_coefficients("numerator", numerator)?;
        validate_finite_coefficients("denominator", denominator)?;

        let denominator = trim_leading_zeros(denominator);
        if denominator.is_empty() {
            return Err(LaplaceError::InvalidDefinition(
                "denominator polynomial is identically zero".into(),
            ));
        }
        let numerator = trim_leading_zeros(numerator);
        let numerator = if numerator.is_empty() {
            &[0.0][..]
        } else {
            numerator
        };
        let transfer_function_dc_gain = if denominator.last() == Some(&0.0) {
            None
        } else {
            Some(checked_ratio(
                numerator.last().copied().unwrap_or(0.0),
                denominator.last().copied().unwrap_or(0.0),
                "transfer-function DC gain",
            )?)
        };

        let numerator_degree = if numerator == [0.0] {
            0
        } else {
            numerator.len() - 1
        };
        let denominator_degree = denominator.len() - 1;
        if numerator != [0.0] && numerator_degree > denominator_degree {
            return Err(LaplaceError::InvalidDefinition(format!(
                "improper transfer function: numerator degree {numerator_degree} exceeds denominator degree {denominator_degree}; input derivatives are not supported"
            )));
        }

        if denominator_degree == 0 {
            let gain = checked_ratio(
                numerator.last().copied().unwrap_or(0.0),
                denominator[0],
                "static gain",
            )?;
            return Ok(Self {
                a: vec![],
                b: vec![],
                c: vec![],
                d: gain,
                transfer_function_dc_gain,
                state: vec![],
                state_prev: vec![],
                candidate_valid: false,
                order: 0,
            });
        }

        let leading_denominator = denominator[0];
        let denom_norm = denominator
            .iter()
            .enumerate()
            .map(|(index, coefficient)| {
                checked_ratio(
                    *coefficient,
                    leading_denominator,
                    &format!("normalized denominator coefficient {index}"),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut num_norm = vec![0.0; denominator.len()];
        let offset = denominator.len() - numerator.len();
        for (index, coefficient) in numerator.iter().enumerate() {
            num_norm[offset + index] = checked_ratio(
                *coefficient,
                leading_denominator,
                &format!("normalized numerator coefficient {index}"),
            )?;
        }

        let n = denominator_degree;
        let mut a_matrix = vec![vec![0.0; n]; n];
        let mut b_vec = vec![0.0; n];
        let mut c_vec = vec![0.0; n];

        for (i, row) in a_matrix.iter_mut().enumerate().take(n - 1) {
            row[i + 1] = 1.0;
        }
        for i in 0..n {
            a_matrix[n - 1][i] = -denom_norm[n - i];
        }
        b_vec[n - 1] = 1.0;

        let d_scalar = num_norm[0];
        for i in 0..n {
            let product = num_norm[0] * denom_norm[n - i];
            let value = num_norm[n - i] - product;
            if !product.is_finite() || !value.is_finite() {
                return Err(LaplaceError::InvalidDefinition(
                    "state-space realization overflowed while combining coefficients".into(),
                ));
            }
            c_vec[i] = value;
        }

        let filter = Self {
            a: a_matrix,
            b: b_vec,
            c: c_vec,
            d: d_scalar,
            transfer_function_dc_gain,
            state: vec![0.0; n],
            state_prev: vec![0.0; n],
            candidate_valid: false,
            order: n,
        };
        filter.validate_structure()?;
        Ok(filter)
    }

    /// Create from poles and zeros with gain
    ///
    /// H(s) = gain * prod(s - zeros) / prod(s - poles)
    pub fn from_poles_zeros(
        poles: &[Complex64],
        zeros: &[Complex64],
        gain: f64,
    ) -> Result<Self, LaplaceError> {
        if !gain.is_finite() {
            return Err(LaplaceError::InvalidDefinition(
                "pole-zero gain must be finite".into(),
            ));
        }
        if poles.is_empty() && zeros.is_empty() {
            return Ok(Self {
                a: vec![],
                b: vec![],
                c: vec![],
                d: gain,
                transfer_function_dc_gain: Some(gain),
                state: vec![],
                state_prev: vec![],
                candidate_valid: false,
                order: 0,
            });
        }

        // Convert poles/zeros to polynomial coefficients
        let numerator = Self::roots_to_poly(zeros)?;
        let denominator = Self::roots_to_poly(poles)?;

        // Scale numerator by gain
        let scaled_num = numerator
            .iter()
            .map(|coefficient| {
                let scaled = coefficient * gain;
                if scaled.is_finite() && (scaled != 0.0 || *coefficient == 0.0 || gain == 0.0) {
                    Ok(scaled)
                } else {
                    Err(LaplaceError::InvalidDefinition(
                        "pole-zero gain scaling overflowed or underflowed".into(),
                    ))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Self::from_transfer_function(&scaled_num, &denominator)
    }

    /// Convert roots to polynomial coefficients (descending powers)
    fn roots_to_poly(roots: &[Complex64]) -> Result<Vec<f64>, LaplaceError> {
        if roots.is_empty() {
            return Ok(vec![1.0]);
        }

        validate_conjugate_roots(roots)?;
        let mut poly = vec![Complex64::new(1.0, 0.0)];
        let mut magnitude_bound = vec![1.0];
        for root in roots {
            let mut new_poly = vec![Complex64::new(0.0, 0.0); poly.len() + 1];
            let mut new_bound = vec![0.0; magnitude_bound.len() + 1];
            for (i, &coeff) in poly.iter().enumerate() {
                new_poly[i] += coeff;
                new_poly[i + 1] -= coeff * *root;
                new_bound[i] += magnitude_bound[i];
                new_bound[i + 1] += magnitude_bound[i] * root.norm();
            }
            if new_poly
                .iter()
                .any(|coefficient| !coefficient.re.is_finite() || !coefficient.im.is_finite())
                || new_bound.iter().any(|bound| !bound.is_finite())
            {
                return Err(LaplaceError::InvalidDefinition(
                    "root expansion overflowed".into(),
                ));
            }
            poly = new_poly;
            magnitude_bound = new_bound;
        }

        poly.into_iter()
            .zip(magnitude_bound)
            .enumerate()
            .map(|(index, (coefficient, bound))| {
                let tolerance = ROOT_RELATIVE_TOLERANCE * bound;
                if coefficient.im.abs() > tolerance {
                    Err(LaplaceError::InvalidDefinition(format!(
                        "root expansion produced a non-real coefficient at index {index} (imaginary residual {})",
                        coefficient.im
                    )))
                } else {
                    Ok(coefficient.re)
                }
            })
            .collect()
    }

    /// Convert (re, im) root pairs to real polynomial coefficients in
    /// ascending powers of s, validating that complex roots cancel.
    pub fn roots_to_polynomial_ascending(roots: &[(f64, f64)]) -> Result<Vec<f64>, LaplaceError> {
        let complex_roots: Vec<Complex64> = roots
            .iter()
            .map(|&(re, im)| Complex64::new(re, im))
            .collect();

        let mut descending = Self::roots_to_poly(&complex_roots)?;
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
            transfer_function_dc_gain: Some(1.0),
            state: vec![],
            state_prev: vec![],
            candidate_valid: false,
            order: 0,
        }
    }

    /// Create a first-order lowpass filter with cutoff frequency
    /// H(s) = omega_c / (s + omega_c)
    pub fn lowpass_first_order(cutoff_hz: f64) -> Result<Self, LaplaceError> {
        let omega_c = 2.0 * PI * cutoff_hz;
        Self::from_transfer_function(&[omega_c], &[1.0, omega_c])
    }

    /// Create a second-order lowpass filter (Butterworth)
    /// H(s) = omega_n^2 / (s^2 + 2*zeta*omega_n*s + omega_n^2)
    pub fn lowpass_second_order(cutoff_hz: f64, damping: f64) -> Result<Self, LaplaceError> {
        let omega_n = 2.0 * PI * cutoff_hz;
        let omega_n2 = omega_n * omega_n;
        Self::from_transfer_function(&[omega_n2], &[1.0, 2.0 * damping * omega_n, omega_n2])
    }

    /// Create a differentiator with time constant tau
    /// H(s) = tau*s / (tau*s + 1)
    pub fn differentiator(tau: f64) -> Result<Self, LaplaceError> {
        Self::from_transfer_function(&[tau, 0.0], &[tau, 1.0])
    }

    /// Create an integrator with time constant tau (leaky)
    /// H(s) = 1 / (tau*s + 1)
    pub fn integrator(tau: f64) -> Result<Self, LaplaceError> {
        Self::from_transfer_function(&[1.0], &[tau, 1.0])
    }

    /// Evaluate an in-flight Backward Euler candidate.
    ///
    /// This computes:
    ///
    /// ```text
    /// x[n] = (I - h*A)^(-1) * (x[n-1] + h*B*u[n])
    /// y[n] = C*x[n] + D*u[n]
    /// ```
    ///
    /// Repeated calls recompute from `state_prev`, which is the last accepted
    /// state. The simulator calls [`Self::commit`] only after accepting the
    /// timestep, keeping Newton reevaluations idempotent.
    pub fn step(&mut self, input: f64, timestep: f64) -> Result<f64, LaplaceError> {
        // A failed reevaluation must not leave an older candidate eligible for
        // acceptance.
        self.candidate_valid = false;
        self.validate_structure()?;
        if !input.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(
                "input must be finite".into(),
            ));
        }
        if self.order == 0 {
            return checked_product(self.d, input, "static output");
        }
        if !timestep.is_finite() || timestep <= 0.0 {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "transient timestep must be finite and positive, got {timestep}"
            )));
        }

        let output = self.step_checked(input, timestep)?;
        self.candidate_valid = true;
        Ok(output)
    }

    /// Return the exact coefficient of the current input in a Backward Euler
    /// transient evaluation at `timestep`.
    ///
    /// For the realization evaluated by [`Self::step`], the coefficient is
    ///
    /// ```text
    /// D + h*C*(I - h*A)^(-1)*B
    /// ```
    ///
    /// This is intentionally distinct from the DC gain. It is a read-only
    /// calculation: neither accepted state nor an in-flight candidate is
    /// changed, so callers can use it during Newton linearization without
    /// affecting transactionality.
    pub fn backward_euler_input_gain(&self, timestep: f64) -> Result<f64, LaplaceError> {
        self.validate_structure()?;
        if self.order == 0 {
            return checked_product(self.d, 1.0, "static transient input gain");
        }
        if !timestep.is_finite() || timestep <= 0.0 {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "transient timestep must be finite and positive, got {timestep}"
            )));
        }

        let matrix = self.backward_euler_matrix(timestep)?;
        let rhs = self
            .b
            .iter()
            .map(|coefficient| timestep * coefficient)
            .collect::<Vec<_>>();
        if rhs.iter().any(|value| !value.is_finite()) {
            return Err(LaplaceError::InvalidEvaluation(
                "transient input-gain right-hand side overflowed".into(),
            ));
        }

        let state_gain = solve_real_system(matrix, rhs, "transient")?;
        checked_state_output(&self.c, &state_gain, self.d, 1.0)
    }

    /// Evaluate and publish a DC-equilibrium candidate for transient startup.
    ///
    /// The equilibrium is solved into temporary storage and becomes eligible
    /// for [`Self::commit`] only after both the solve and output evaluation
    /// complete successfully. Repeated Newton passes therefore replace the
    /// candidate, while a failed or skipped final pass cannot commit stale
    /// state. Order-zero filters remain stateless.
    pub(crate) fn dc_candidate(&mut self, input: f64) -> Result<f64, LaplaceError> {
        self.candidate_valid = false;
        let (equilibrium, output) = self.dc_equilibrium(input)?;
        if let Some(equilibrium) = equilibrium {
            self.state.copy_from_slice(&equilibrium);
            self.candidate_valid = true;
        }
        Ok(output)
    }

    /// Commit the most recently evaluated candidate.
    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.state_prev.clone_from(&self.state);
            self.candidate_valid = false;
        }
    }

    /// Start one complete device evaluation pass by invalidating any state
    /// candidate left by the preceding pass.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate_valid = false;
    }

    /// Validate the candidate that would be committed, without mutation.
    pub(crate) fn validate_commit(&self) -> Result<(), LaplaceError> {
        self.validate_structure()?;
        if self.candidate_valid && self.state.iter().any(|value| !value.is_finite()) {
            return Err(LaplaceError::InvalidEvaluation(
                "Laplace candidate state is not finite".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn checkpoint(&self) -> LaplaceCheckpoint {
        LaplaceCheckpoint {
            state: self.state_prev.clone(),
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), LaplaceError> {
        self.validate_structure()?;
        if self.candidate_valid {
            return Err(LaplaceError::InvalidEvaluation(
                "Laplace filter has an in-flight Newton candidate".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(
        &self,
        checkpoint: &LaplaceCheckpoint,
    ) -> Result<(), LaplaceError> {
        self.validate_structure()?;
        if checkpoint.state.len() != self.order {
            return Err(LaplaceError::InvalidDefinition(format!(
                "checkpoint state length is {}, expected {}",
                checkpoint.state.len(),
                self.order
            )));
        }
        if checkpoint.state.iter().any(|value| !value.is_finite()) {
            return Err(LaplaceError::InvalidDefinition(
                "checkpoint state contains a non-finite value".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &LaplaceCheckpoint) {
        self.state_prev.copy_from_slice(&checkpoint.state);
        self.state.copy_from_slice(&checkpoint.state);
        self.candidate_valid = false;
    }

    fn step_checked(&mut self, input: f64, h: f64) -> Result<f64, LaplaceError> {
        let matrix = self.backward_euler_matrix(h)?;

        let rhs = self
            .state_prev
            .iter()
            .zip(self.b.iter())
            .map(|(&previous, &b)| previous + h * b * input)
            .collect::<Vec<_>>();
        if rhs.iter().any(|value| !value.is_finite()) {
            return Err(LaplaceError::InvalidEvaluation(
                "transient right-hand side overflowed".into(),
            ));
        }

        let candidate = solve_real_system(matrix, rhs, "transient")?;
        let output = checked_state_output(&self.c, &candidate, self.d, input)?;
        self.state.copy_from_slice(&candidate);
        Ok(output)
    }

    fn backward_euler_matrix(&self, h: f64) -> Result<Vec<Vec<f64>>, LaplaceError> {
        let mut matrix = vec![vec![0.0; self.order]; self.order];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = if i == j { 1.0 } else { 0.0 };
                *cell -= h * self.a[i][j];
                if !cell.is_finite() {
                    return Err(LaplaceError::InvalidEvaluation(
                        "transient state matrix overflowed".into(),
                    ));
                }
            }
        }
        Ok(matrix)
    }

    /// Get DC output (s=0) for a given input
    pub fn dc_output(&self, input: f64) -> Result<f64, LaplaceError> {
        self.validate_structure()?;
        if !input.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(
                "input must be finite".into(),
            ));
        }
        if let Some(gain) = self.transfer_function_dc_gain {
            return checked_product(gain, input, "DC output");
        }
        self.dc_equilibrium(input).map(|(_, output)| output)
    }

    fn dc_equilibrium(&self, input: f64) -> Result<(Option<Vec<f64>>, f64), LaplaceError> {
        self.validate_structure()?;
        if !input.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(
                "input must be finite".into(),
            ));
        }
        if self.order == 0 {
            return checked_product(self.d, input, "DC output").map(|output| (None, output));
        }

        let matrix = self
            .a
            .iter()
            .map(|row| row.iter().map(|value| -*value).collect())
            .collect();
        let rhs = self.b.iter().map(|value| *value * input).collect();
        let equilibrium = solve_real_system(matrix, rhs, "DC equilibrium")?;
        let output = if let Some(gain) = self.transfer_function_dc_gain {
            checked_product(gain, input, "DC output")?
        } else {
            checked_state_output(&self.c, &equilibrium, self.d, input)?
        };
        Ok((Some(equilibrium), output))
    }

    /// Reset all accepted and speculative state to zero for a new analysis.
    /// The compiled state-space realization remains unchanged.
    pub fn reset(&mut self) {
        self.state.fill(0.0);
        self.state_prev.fill(0.0);
        self.candidate_valid = false;
    }

    /// Set initial state
    pub fn set_initial_state(&mut self, initial: &[f64]) -> Result<(), LaplaceError> {
        self.validate_structure()?;
        if initial.len() != self.order {
            return Err(LaplaceError::InvalidDefinition(format!(
                "initial-state length {} does not match filter order {}",
                initial.len(),
                self.order
            )));
        }
        if initial.iter().any(|value| !value.is_finite()) {
            return Err(LaplaceError::InvalidDefinition(
                "initial state must contain only finite values".into(),
            ));
        }
        self.state.copy_from_slice(initial);
        self.state_prev.copy_from_slice(initial);
        self.candidate_valid = false;
        Ok(())
    }

    /// Compute frequency response at given frequency (Hz)
    pub fn frequency_response(&self, freq_hz: f64) -> Result<(f64, f64), LaplaceError> {
        self.validate_structure()?;
        if !freq_hz.is_finite() || freq_hz < 0.0 {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "frequency must be finite and nonnegative, got {freq_hz}"
            )));
        }
        let omega = 2.0 * PI * freq_hz;
        if !omega.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "angular frequency overflows f64 for {freq_hz} Hz"
            )));
        }

        if self.order == 0 {
            return Ok((self.d.abs(), if self.d >= 0.0 { 0.0 } else { PI }));
        }

        // Evaluate H(jw) = C * (jwI - A)^(-1) * B + D directly from the
        // state-space form so the response stays consistent with the runtime
        // filter realization instead of relying on reconstructed polynomials.
        let state = solve_complex_system(&self.a, &self.b, omega)?;

        let mut response = Complex64::new(self.d, 0.0);
        for (&c, state_value) in self.c.iter().zip(state.iter()) {
            response += Complex64::new(c, 0.0) * *state_value;
        }

        let magnitude = response.norm();
        let phase = response.arg();
        if !magnitude.is_finite() || !phase.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(
                "frequency response is non-finite".into(),
            ));
        }
        Ok((magnitude, phase))
    }

    fn validate_structure(&self) -> Result<(), LaplaceError> {
        let dimensions_match = self.a.len() == self.order
            && self.a.iter().all(|row| row.len() == self.order)
            && self.b.len() == self.order
            && self.c.len() == self.order
            && self.state.len() == self.order
            && self.state_prev.len() == self.order;
        if !dimensions_match {
            return Err(LaplaceError::InvalidDefinition(format!(
                "state-space dimensions do not match declared order {}",
                self.order
            )));
        }
        if !self.d.is_finite()
            || self
                .transfer_function_dc_gain
                .is_some_and(|gain| !gain.is_finite())
            || self
                .a
                .iter()
                .flatten()
                .chain(self.b.iter())
                .chain(self.c.iter())
                .chain(self.state.iter())
                .chain(self.state_prev.iter())
                .any(|value| !value.is_finite())
        {
            return Err(LaplaceError::InvalidDefinition(
                "state-space matrices, states, and feedthrough must be finite".into(),
            ));
        }
        Ok(())
    }
}

fn solve_complex_system(
    a: &[Vec<f64>],
    b: &[f64],
    omega: f64,
) -> Result<Vec<Complex64>, LaplaceError> {
    let n = b.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    let mut mat = vec![vec![Complex64::new(0.0, 0.0); n]; n];
    let mut rhs = vec![Complex64::new(0.0, 0.0); n];

    for i in 0..n {
        rhs[i] = Complex64::new(b[i], 0.0);
        for j in 0..n {
            let imag = if i == j { omega } else { 0.0 };
            mat[i][j] = Complex64::new(-a[i][j], imag);
        }
    }

    for k in 0..n {
        let mut pivot_row = None;
        let mut best_ratio = 0.0;
        for (row_idx, row) in mat.iter().enumerate().skip(k) {
            let row_scale = row
                .iter()
                .skip(k)
                .map(|value| complex_scale(*value))
                .fold(0.0_f64, f64::max);
            if row_scale == 0.0 || !row_scale.is_finite() {
                continue;
            }
            let ratio = complex_scale(row[k]) / row_scale;
            if ratio > best_ratio {
                best_ratio = ratio;
                pivot_row = Some(row_idx);
            }
        }

        if best_ratio <= PIVOT_RELATIVE_TOLERANCE * (n - k) as f64 {
            return Err(LaplaceError::SingularSystem("frequency-response"));
        }

        let pivot_row = pivot_row.ok_or(LaplaceError::SingularSystem("frequency-response"))?;

        if pivot_row != k {
            mat.swap(k, pivot_row);
            rhs.swap(k, pivot_row);
        }

        let pivot = mat[k][k];
        let pivot_row_values = mat[k].clone();
        let rhs_pivot = rhs[k];
        for row_idx in (k + 1)..n {
            let factor =
                checked_complex_ratio(mat[row_idx][k], pivot, "frequency-response elimination")?;
            mat[row_idx][k] = Complex64::new(0.0, 0.0);
            for col_idx in (k + 1)..n {
                mat[row_idx][col_idx] -= factor * pivot_row_values[col_idx];
            }
            rhs[row_idx] -= factor * rhs_pivot;
            if !complex_is_finite(rhs[row_idx])
                || mat[row_idx].iter().any(|value| !complex_is_finite(*value))
            {
                return Err(LaplaceError::InvalidEvaluation(
                    "frequency-response solve overflowed".into(),
                ));
            }
        }
    }

    let mut solution = vec![Complex64::new(0.0, 0.0); n];
    for row_idx in (0..n).rev() {
        let pivot = mat[row_idx][row_idx];
        let row_scale = mat[row_idx]
            .iter()
            .skip(row_idx)
            .map(|value| complex_scale(*value))
            .fold(0.0_f64, f64::max);
        if row_scale == 0.0
            || complex_scale(pivot) / row_scale <= PIVOT_RELATIVE_TOLERANCE * (n - row_idx) as f64
        {
            return Err(LaplaceError::SingularSystem("frequency-response"));
        }
        let mut sum = rhs[row_idx];
        for col_idx in (row_idx + 1)..n {
            sum -= mat[row_idx][col_idx] * solution[col_idx];
        }
        solution[row_idx] =
            checked_complex_ratio(sum, pivot, "frequency-response back substitution")?;
    }

    Ok(solution)
}

fn validate_finite_coefficients(label: &str, coefficients: &[f64]) -> Result<(), LaplaceError> {
    for (index, coefficient) in coefficients.iter().enumerate() {
        if !coefficient.is_finite() {
            return Err(LaplaceError::InvalidDefinition(format!(
                "{label} coefficient {index} must be finite"
            )));
        }
    }
    Ok(())
}

fn trim_leading_zeros(values: &[f64]) -> &[f64] {
    let first_nonzero = values.iter().position(|value| *value != 0.0);
    first_nonzero.map_or(&[], |index| &values[index..])
}

fn checked_ratio(numerator: f64, denominator: f64, context: &str) -> Result<f64, LaplaceError> {
    if denominator == 0.0 {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} has a zero denominator"
        )));
    }
    let result = numerator / denominator;
    if !result.is_finite() {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} is outside the representable f64 range"
        )));
    }
    if result == 0.0 && numerator != 0.0 {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} underflows f64"
        )));
    }
    Ok(result)
}

fn checked_definition_product(left: f64, right: f64, context: &str) -> Result<f64, LaplaceError> {
    if !left.is_finite() || !right.is_finite() {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} contains a non-finite factor"
        )));
    }
    let result = left * right;
    if !result.is_finite() {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} is outside the representable f64 range"
        )));
    }
    if result == 0.0 && left != 0.0 && right != 0.0 {
        return Err(LaplaceError::InvalidDefinition(format!(
            "{context} underflows f64"
        )));
    }
    Ok(result)
}

fn checked_product(left: f64, right: f64, context: &str) -> Result<f64, LaplaceError> {
    let result = left * right;
    if !result.is_finite() {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} is non-finite"
        )));
    }
    if result == 0.0 && left != 0.0 && right != 0.0 {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} underflows f64"
        )));
    }
    Ok(result)
}

fn relative_match(left: f64, right: f64) -> bool {
    if left == right {
        return true;
    }
    let scale = left.abs().max(right.abs());
    scale != 0.0 && (left - right).abs() <= ROOT_RELATIVE_TOLERANCE * scale
}

fn validate_conjugate_roots(roots: &[Complex64]) -> Result<(), LaplaceError> {
    let mut paired = vec![false; roots.len()];
    for (index, root) in roots.iter().enumerate() {
        if !root.re.is_finite() || !root.im.is_finite() {
            return Err(LaplaceError::InvalidDefinition(format!(
                "root {index} must have finite real and imaginary parts"
            )));
        }
        if paired[index] {
            continue;
        }
        if root.im == 0.0 {
            paired[index] = true;
            continue;
        }
        let conjugate = roots
            .iter()
            .enumerate()
            .skip(index + 1)
            .find(|(candidate_index, candidate)| {
                !paired[*candidate_index]
                    && relative_match(root.re, candidate.re)
                    && relative_match(root.im, -candidate.im)
            })
            .map(|(candidate_index, _)| candidate_index);
        let Some(conjugate_index) = conjugate else {
            return Err(LaplaceError::InvalidDefinition(format!(
                "complex root {index} ({}, {}) has no conjugate partner",
                root.re, root.im
            )));
        };
        paired[index] = true;
        paired[conjugate_index] = true;
    }
    Ok(())
}

fn solve_real_system(
    mut matrix: Vec<Vec<f64>>,
    mut rhs: Vec<f64>,
    context: &'static str,
) -> Result<Vec<f64>, LaplaceError> {
    let n = rhs.len();
    if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} state-space dimensions are inconsistent"
        )));
    }
    if matrix
        .iter()
        .flatten()
        .chain(rhs.iter())
        .any(|value| !value.is_finite())
    {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} system contains a non-finite value"
        )));
    }

    for k in 0..n {
        let mut pivot_row = None;
        let mut best_ratio = 0.0;
        for (row_index, row) in matrix.iter().enumerate().skip(k) {
            let row_scale = row
                .iter()
                .skip(k)
                .map(|value| value.abs())
                .fold(0.0_f64, f64::max);
            if row_scale == 0.0 {
                continue;
            }
            let ratio = row[k].abs() / row_scale;
            if ratio > best_ratio {
                best_ratio = ratio;
                pivot_row = Some(row_index);
            }
        }
        if best_ratio <= PIVOT_RELATIVE_TOLERANCE * (n - k) as f64 {
            return Err(LaplaceError::SingularSystem(context));
        }
        let pivot_row = pivot_row.ok_or(LaplaceError::SingularSystem(context))?;
        if pivot_row != k {
            matrix.swap(k, pivot_row);
            rhs.swap(k, pivot_row);
        }

        let pivot = matrix[k][k];
        let pivot_values = matrix[k].clone();
        let pivot_rhs = rhs[k];
        for row_index in (k + 1)..n {
            let factor = matrix[row_index][k] / pivot;
            matrix[row_index][k] = 0.0;
            for column_index in (k + 1)..n {
                matrix[row_index][column_index] -= factor * pivot_values[column_index];
            }
            rhs[row_index] -= factor * pivot_rhs;
            if !rhs[row_index].is_finite()
                || matrix[row_index].iter().any(|value| !value.is_finite())
            {
                return Err(LaplaceError::InvalidEvaluation(format!(
                    "{context} solve overflowed"
                )));
            }
        }
    }

    let mut solution = vec![0.0; n];
    for row_index in (0..n).rev() {
        let row_scale = matrix[row_index]
            .iter()
            .skip(row_index)
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max);
        let pivot = matrix[row_index][row_index];
        if row_scale == 0.0
            || pivot.abs() / row_scale <= PIVOT_RELATIVE_TOLERANCE * (n - row_index) as f64
        {
            return Err(LaplaceError::SingularSystem(context));
        }
        let mut sum = rhs[row_index];
        for (coefficient, solved) in matrix[row_index]
            .iter()
            .zip(solution.iter())
            .skip(row_index + 1)
        {
            sum -= coefficient * solved;
        }
        solution[row_index] = sum / pivot;
        if !solution[row_index].is_finite() {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "{context} solution is non-finite"
            )));
        }
    }
    Ok(solution)
}

fn checked_state_output(
    c: &[f64],
    state: &[f64],
    feedthrough: f64,
    input: f64,
) -> Result<f64, LaplaceError> {
    let mut output = feedthrough * input;
    for (coefficient, value) in c.iter().zip(state.iter()) {
        output = coefficient.mul_add(*value, output);
    }
    if output.is_finite() {
        Ok(output)
    } else {
        Err(LaplaceError::InvalidEvaluation(
            "state-space output is non-finite".into(),
        ))
    }
}

fn complex_is_finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

fn complex_scale(value: Complex64) -> f64 {
    value.re.abs().max(value.im.abs())
}

fn checked_complex_ratio(
    numerator: Complex64,
    denominator: Complex64,
    context: &str,
) -> Result<Complex64, LaplaceError> {
    let numerator_scale = numerator.re.abs().max(numerator.im.abs());
    let denominator_scale = denominator.re.abs().max(denominator.im.abs());
    if denominator_scale == 0.0 || !denominator_scale.is_finite() || !numerator_scale.is_finite() {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} has an invalid divisor or numerator"
        )));
    }
    if numerator_scale == 0.0 {
        return Ok(Complex64::new(0.0, 0.0));
    }

    let numerator_exponent = binary_exponent(numerator_scale);
    let denominator_exponent = binary_exponent(denominator_scale);
    let numerator_normalized = Complex64::new(
        scale_by_power_of_two(numerator.re, -numerator_exponent, context)?,
        scale_by_power_of_two(numerator.im, -numerator_exponent, context)?,
    );
    let denominator_normalized = Complex64::new(
        scale_by_power_of_two(denominator.re, -denominator_exponent, context)?,
        scale_by_power_of_two(denominator.im, -denominator_exponent, context)?,
    );
    let norm = denominator_normalized.re.mul_add(
        denominator_normalized.re,
        denominator_normalized.im * denominator_normalized.im,
    );
    let real_normalized = (numerator_normalized.re * denominator_normalized.re
        + numerator_normalized.im * denominator_normalized.im)
        / norm;
    let imaginary_normalized = (numerator_normalized.im * denominator_normalized.re
        - numerator_normalized.re * denominator_normalized.im)
        / norm;
    let exponent = numerator_exponent - denominator_exponent;
    Ok(Complex64::new(
        scale_by_power_of_two(real_normalized, exponent, context)?,
        scale_by_power_of_two(imaginary_normalized, exponent, context)?,
    ))
}

fn binary_exponent(value: f64) -> i32 {
    debug_assert!(value.is_finite() && value > 0.0);
    let bits = value.to_bits();
    let encoded_exponent = ((bits >> 52) & 0x7ff) as i32;
    if encoded_exponent != 0 {
        encoded_exponent - 1023
    } else {
        let significand = bits & ((1_u64 << 52) - 1);
        let highest_bit = 63 - significand.leading_zeros() as i32;
        highest_bit - 1074
    }
}

fn scale_by_power_of_two(
    mut value: f64,
    mut exponent: i32,
    context: &str,
) -> Result<f64, LaplaceError> {
    if value == 0.0 {
        return Ok(value);
    }
    let original = value;
    while exponent > 1023 {
        value *= 2.0_f64.powi(1023);
        exponent -= 1023;
        if !value.is_finite() {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "{context} result overflows f64"
            )));
        }
    }
    while exponent < -1022 {
        value *= 2.0_f64.powi(-1022);
        exponent += 1022;
        if value == 0.0 {
            return Err(LaplaceError::InvalidEvaluation(format!(
                "{context} nonzero result underflows f64"
            )));
        }
    }
    value *= 2.0_f64.powi(exponent);
    if !value.is_finite() {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} result overflows f64"
        )));
    }
    if value == 0.0 && original != 0.0 {
        return Err(LaplaceError::InvalidEvaluation(format!(
            "{context} nonzero result underflows f64"
        )));
    }
    Ok(value)
}

/// Laplace transform filter type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaplaceFilter {
    /// Pole-zero form: H(s) = gain * prod(s-zeros) / prod(s-poles)
    PoleZero {
        gain: f64,
        poles: Vec<Complex64>,
        zeros: Vec<Complex64>,
    },
    /// Numerator-denominator form: H(s) = N(s)/D(s)
    NumDen {
        numerator: Vec<f64>,
        denominator: Vec<f64>,
    },
}

impl LaplaceFilter {
    /// Convert to state-space representation
    pub fn to_state_space(&self) -> Result<StateSpaceFilter, LaplaceError> {
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
    pub fn dc_gain(&self) -> Result<f64, LaplaceError> {
        match self {
            LaplaceFilter::PoleZero { gain, poles, zeros } => {
                let zeros = zeros
                    .iter()
                    .map(|root| (root.re, root.im))
                    .collect::<Vec<_>>();
                let poles = poles
                    .iter()
                    .map(|root| (root.re, root.im))
                    .collect::<Vec<_>>();
                checked_pole_zero_dc_gain(*gain, &zeros, &poles)
            }
            LaplaceFilter::NumDen { .. } => self.to_state_space()?.dc_output(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compile_filter(expression: &str) -> crate::CompileResult<crate::CompiledModel> {
        let source = format!(
            r#"
`include "disciplines.vams"
module laplace_validation(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ {expression};
endmodule
"#
        );
        crate::VerilogACompiler::new(crate::CompilerOptions::default()).compile(&source)
    }

    #[test]
    fn tiny_well_scaled_coefficients_and_pivots_are_supported() {
        let filter = StateSpaceFilter::from_transfer_function(&[1.0e-300], &[1.0e-300, 1.0e-300])
            .expect("common coefficient scaling is valid");
        let dc = filter.dc_output(1.0).expect("finite DC equilibrium");
        assert!((dc - 1.0).abs() <= 8.0 * f64::EPSILON);

        let solution = solve_complex_system(&[vec![-1.0e-300]], &[1.0e-300], 0.0)
            .expect("a tiny, well-scaled system is nonsingular");
        assert!((solution[0].re - 1.0).abs() <= 8.0 * f64::EPSILON);
    }

    #[test]
    fn transient_input_gain_matches_backward_euler_finite_difference() {
        let mut filter = StateSpaceFilter::integrator(1.0).expect("first-order filter");
        filter
            .set_initial_state(&[0.25])
            .expect("matching initial state");

        let timestep = 0.5;
        let gain = filter
            .backward_euler_input_gain(timestep)
            .expect("finite Backward Euler input gain");
        assert!((gain - 1.0 / 3.0).abs() <= 8.0 * f64::EPSILON);
        assert_ne!(gain, filter.dc_output(1.0).expect("finite DC gain"));

        let epsilon = 1.0e-6;
        let upper = filter
            .step(0.75 + epsilon, timestep)
            .expect("upper candidate");
        let lower = filter
            .step(0.75 - epsilon, timestep)
            .expect("lower candidate");
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - gain).abs() <= 1.0e-9);
    }

    #[test]
    fn transient_input_gain_is_read_only_and_includes_feedthrough() {
        let mut filter = StateSpaceFilter::differentiator(1.0).expect("first-order filter");
        filter
            .set_initial_state(&[0.25])
            .expect("matching initial state");
        filter.step(0.75, 0.5).expect("in-flight candidate");

        let state_before = filter.state.clone();
        let accepted_before = filter.state_prev.clone();
        let candidate_valid_before = filter.candidate_valid;
        let gain = filter
            .backward_euler_input_gain(0.5)
            .expect("finite Backward Euler input gain");

        assert!((gain - 2.0 / 3.0).abs() <= 8.0 * f64::EPSILON);
        assert_eq!(filter.state, state_before);
        assert_eq!(filter.state_prev, accepted_before);
        assert_eq!(filter.candidate_valid, candidate_valid_before);
    }

    #[test]
    fn transient_input_gain_uses_step_timestep_validation() {
        let dynamic = StateSpaceFilter::integrator(1.0).expect("first-order filter");
        for timestep in [0.0, -1.0, f64::INFINITY, f64::NAN] {
            assert!(matches!(
                dynamic.backward_euler_input_gain(timestep),
                Err(LaplaceError::InvalidEvaluation(_))
            ));
        }

        let static_filter = StateSpaceFilter::unity_gain();
        assert_eq!(
            static_filter
                .backward_euler_input_gain(f64::NAN)
                .expect("order-zero step ignores timestep"),
            1.0
        );
    }

    #[test]
    fn checked_pole_zero_dc_gain_preserves_real_root_pairs() {
        let gain = checked_pole_zero_dc_gain(3.0, &[(-2.0, 0.0)], &[(-4.0, 0.0)])
            .expect("ordinary real roots define a finite DC gain");
        assert_eq!(gain.to_bits(), 1.5_f64.to_bits());
    }

    #[test]
    fn checked_pole_zero_dc_gain_uses_conjugate_pair_magnitudes() {
        let gain = checked_pole_zero_dc_gain(
            2.0,
            &[(-1.0, 2.0), (-1.0, -2.0)],
            &[(-3.0, 4.0), (-3.0, -4.0)],
        )
        .expect("conjugate roots define real constant terms");
        assert!((gain - 0.4).abs() <= 8.0 * f64::EPSILON);
    }

    #[test]
    fn pole_zero_filter_dc_gain_matches_state_space_equilibrium() {
        for filter in [
            LaplaceFilter::PoleZero {
                gain: 3.0,
                zeros: vec![Complex64::new(-2.0, 0.0)],
                poles: vec![Complex64::new(-4.0, 0.0)],
            },
            LaplaceFilter::PoleZero {
                gain: 2.0,
                zeros: vec![Complex64::new(-1.0, 2.0), Complex64::new(-1.0, -2.0)],
                poles: vec![Complex64::new(-3.0, 4.0), Complex64::new(-3.0, -4.0)],
            },
        ] {
            let checked = filter.dc_gain().expect("checked pole-zero DC gain");
            let equilibrium = filter
                .to_state_space()
                .and_then(|state_space| state_space.dc_output(1.0))
                .expect("state-space DC equilibrium");
            assert!((checked - equilibrium).abs() <= 16.0 * f64::EPSILON);
        }
    }

    #[test]
    fn checked_pole_zero_dc_gain_rejects_invalid_root_definitions() {
        for (zeros, poles, expected) in [
            (vec![(1.0, 2.0)], vec![(-1.0, 0.0)], "no conjugate partner"),
            (
                vec![(f64::NAN, 0.0)],
                vec![(-1.0, 0.0)],
                "must have finite real and imaginary parts",
            ),
            (vec![(-1.0, 0.0)], vec![(0.0, 0.0)], "zero denominator"),
        ] {
            let error = checked_pole_zero_dc_gain(1.0, &zeros, &poles)
                .expect_err("invalid pole-zero definition must fail closed");
            assert!(
                matches!(error, LaplaceError::InvalidDefinition(_)),
                "unexpected error type: {error:?}"
            );
            assert!(
                error.to_string().contains(expected),
                "expected '{expected}' in diagnostic, got: {error}"
            );
        }
    }

    #[test]
    fn state_space_constructor_rejects_malformed_or_nonfinite_storage() {
        for result in [
            StateSpaceFilter::new(vec![], vec![1.0], vec![1.0], 0.0),
            StateSpaceFilter::new(vec![vec![0.0]], vec![1.0], vec![], 0.0),
            StateSpaceFilter::new(vec![vec![f64::NAN]], vec![1.0], vec![1.0], 0.0),
            StateSpaceFilter::new(vec![], vec![], vec![], f64::INFINITY),
        ] {
            assert!(result.is_err());
        }
    }

    #[test]
    fn legacy_serialized_dc_gain_is_ignored_in_favor_of_checked_equilibrium() {
        let filter = StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 0.0])
            .expect("transient integrator definition");
        let mut serialized = serde_json::to_value(&filter).expect("serialize state-space filter");
        assert!(serialized.get("dc_gain").is_none());
        serialized
            .as_object_mut()
            .expect("filter serializes as an object")
            .insert("dc_gain".into(), serde_json::json!(1.0));
        let restored: StateSpaceFilter =
            serde_json::from_value(serialized).expect("legacy extra field remains compatible");
        assert!(matches!(
            restored.dc_output(1.0),
            Err(LaplaceError::SingularSystem("DC equilibrium"))
        ));
    }

    #[test]
    fn serialization_never_persists_an_in_flight_candidate() {
        let mut filter = StateSpaceFilter::integrator(1.0).expect("filter definition");
        filter
            .step(2.0, 0.25)
            .expect("produce a speculative transient candidate");
        assert!(filter.candidate_valid);
        assert_ne!(filter.state, filter.state_prev);

        let serialized = serde_json::to_value(&filter).expect("serialize state-space filter");
        assert!(serialized.get("candidate_valid").is_none());
        let mut restored: StateSpaceFilter =
            serde_json::from_value(serialized).expect("deserialize state-space filter");
        assert!(!restored.candidate_valid);

        let accepted_before = restored.state_prev.clone();
        restored.commit();
        assert_eq!(restored.state_prev, accepted_before);
        restored
            .validate_checkpoint_ready()
            .expect("deserialization must not recreate an in-flight candidate");
    }

    #[test]
    fn dc_candidate_commits_only_the_final_successful_pass() {
        let mut filter = StateSpaceFilter::integrator(1.0).expect("filter definition");
        assert_eq!(filter.checkpoint().state, vec![0.0]);

        assert_eq!(filter.dc_candidate(4.0).expect("first DC candidate"), 4.0);
        assert_eq!(filter.state, vec![4.0]);
        assert_eq!(filter.checkpoint().state, vec![0.0]);

        assert_eq!(
            filter.dc_candidate(6.0).expect("replacement DC candidate"),
            6.0
        );
        filter.commit();
        assert_eq!(filter.checkpoint().state, vec![6.0]);

        filter.dc_candidate(7.0).expect("speculative DC candidate");
        filter
            .dc_candidate(f64::NAN)
            .expect_err("a failed final pass must invalidate the prior candidate");
        filter.commit();
        assert_eq!(filter.checkpoint().state, vec![6.0]);

        filter
            .dc_candidate(8.0)
            .expect("candidate before skipped pass");
        filter.begin_evaluation();
        filter.commit();
        assert_eq!(filter.checkpoint().state, vec![6.0]);

        assert_eq!(filter.dc_output(9.0).expect("read-only DC output"), 9.0);
        filter.commit();
        assert_eq!(filter.checkpoint().state, vec![6.0]);
    }

    #[test]
    fn dc_candidate_is_atomic_and_order_zero_remains_stateless() {
        let mut singular = StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 0.0])
            .expect("ideal integrator is valid for transient analysis");
        singular
            .set_initial_state(&[7.0])
            .expect("matching accepted state");
        let before = singular.state.clone();
        singular
            .dc_candidate(1.0)
            .expect_err("singular DC equilibrium must fail");
        assert_eq!(singular.state, before);
        assert!(!singular.candidate_valid);
        singular.commit();
        assert_eq!(singular.checkpoint().state, vec![7.0]);

        let mut output_overflow =
            StateSpaceFilter::new(vec![vec![-1.0]], vec![1.0], vec![f64::MAX], f64::MAX)
                .expect("finite state-space realization");
        output_overflow
            .set_initial_state(&[5.0])
            .expect("matching accepted state");
        output_overflow
            .dc_candidate(1.0)
            .expect_err("non-finite output must not publish the solved equilibrium");
        assert_eq!(output_overflow.state, vec![5.0]);
        assert!(!output_overflow.candidate_valid);

        let mut stateless = StateSpaceFilter::unity_gain();
        assert_eq!(stateless.dc_candidate(3.0).unwrap(), 3.0);
        assert!(!stateless.candidate_valid);
        stateless.commit();
        assert!(stateless.checkpoint().state.is_empty());
    }

    #[test]
    fn scaled_complex_division_handles_both_f64_extremes() {
        for value in [f64::from_bits(1), f64::MAX] {
            let quotient = checked_complex_ratio(
                Complex64::new(value, value),
                Complex64::new(value, value),
                "test division",
            )
            .expect("equal finite complex values have a representable quotient");
            assert_eq!(quotient.re.to_bits(), 1.0_f64.to_bits());
            assert_eq!(quotient.im.to_bits(), 0.0_f64.to_bits());
        }

        let error = checked_complex_ratio(
            Complex64::new(f64::from_bits(1), 0.0),
            Complex64::new(f64::MAX, 0.0),
            "test division",
        )
        .expect_err("a true nonzero quotient below f64 must fail closed");
        assert!(error.to_string().contains("underflows"));
    }

    #[test]
    fn frequency_response_handles_large_finite_systems_and_rejects_omega_overflow() {
        let filter = StateSpaceFilter::new(vec![vec![-f64::MAX]], vec![f64::MAX], vec![1.0], 0.0)
            .expect("finite extreme state-space filter");
        let finite_frequency = f64::MAX / (2.0 * PI);
        let (magnitude, phase) = filter
            .frequency_response(finite_frequency)
            .expect("scaled complex solve remains representable");
        assert!(magnitude.is_finite() && phase.is_finite());

        let error = filter
            .frequency_response(f64::MAX)
            .expect_err("2*pi*f overflow must be diagnosed");
        assert!(error.to_string().contains("angular frequency overflows"));
    }

    #[test]
    fn improper_transfer_functions_are_refused_without_panicking() {
        let result = std::panic::catch_unwind(|| {
            StateSpaceFilter::from_transfer_function(&[1.0, 2.0], &[0.5])
        });
        let error = result
            .expect("validation must not panic")
            .expect_err("improper transfer functions require input derivatives");
        assert!(error.to_string().contains("improper transfer function"));
    }

    #[test]
    fn zero_frequency_pole_reports_singular_dc_equilibrium() {
        let filter = StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 0.0])
            .expect("an integrator is valid for transient analysis");
        let error = filter
            .dc_output(1.0)
            .expect_err("an ideal integrator has no finite DC equilibrium");
        assert!(matches!(
            error,
            LaplaceError::SingularSystem("DC equilibrium")
        ));
    }

    #[test]
    fn legacy_ir_laplace_nd_derivative_retains_the_primal_site_and_definition() {
        use crate::ir::{DerivativeWrt, IrExpr, LaplaceSiteId, autodiff};

        let derivative = |numerator, denominator| {
            let site = LaplaceSiteId::from_span(crate::source::Span::dummy());
            let expression = IrExpr::LaplaceND {
                site,
                expr: Box::new(IrExpr::Voltage(0, usize::MAX)),
                numerator,
                denominator,
            };
            autodiff::simplify(autodiff::differentiate(
                &expression,
                &DerivativeWrt::Voltage(0),
            ))
        };

        let IrExpr::LaplaceNDDerivative {
            site,
            expr,
            numerator,
            denominator,
        } = derivative(vec![1.0e-310], vec![1.0e-310, 1.0])
        else {
            panic!("Laplace ND derivative must retain an explicit state-site action");
        };
        assert_eq!(site, LaplaceSiteId::from_span(crate::source::Span::dummy()));
        assert!(matches!(expr.as_ref(), IrExpr::Const(1.0)));
        assert_eq!(numerator, vec![1.0e-310]);
        assert_eq!(denominator, vec![1.0e-310, 1.0]);
    }

    #[test]
    fn legacy_ir_laplace_zp_derivative_retains_the_primal_site_and_definition() {
        use crate::ir::{DerivativeWrt, IrExpr, LaplaceSiteId, autodiff};

        let derivative = |zeros, poles| {
            let site = LaplaceSiteId::from_span(crate::source::Span::dummy());
            let expression = IrExpr::LaplaceZP {
                site,
                expr: Box::new(IrExpr::Voltage(0, usize::MAX)),
                zeros,
                poles,
                gain: 2.0,
            };
            autodiff::simplify(autodiff::differentiate(
                &expression,
                &DerivativeWrt::Voltage(0),
            ))
        };

        let zeros = vec![(-1.0, 2.0), (-1.0, -2.0)];
        let poles = vec![(-3.0, 4.0), (-3.0, -4.0)];
        let IrExpr::LaplaceZPDerivative {
            site,
            expr,
            zeros: derivative_zeros,
            poles: derivative_poles,
            gain,
        } = derivative(zeros.clone(), poles.clone())
        else {
            panic!("Laplace ZP derivative must retain an explicit state-site action");
        };
        assert_eq!(site, LaplaceSiteId::from_span(crate::source::Span::dummy()));
        assert!(matches!(expr.as_ref(), IrExpr::Const(1.0)));
        assert_eq!(derivative_zeros, zeros);
        assert_eq!(derivative_poles, poles);
        assert_eq!(gain, 2.0);
    }

    #[cfg(not(feature = "native"))]
    #[test]
    fn singular_dc_filter_reaches_typed_interpreter_error_boundary() {
        let model = compile_filter("laplace_nd(V(p, n), '{1.0}, '{0.0, 1.0})")
            .expect("ideal integrator remains available for transient analysis");
        let mut device = crate::device::VerilogADevice::try_new(
            "L_SINGULAR_DC",
            std::sync::Arc::new(model),
            &[1, 0],
        )
        .expect("device construction");
        device.update_voltages(&[1.0]);
        let error = device
            .try_evaluate()
            .expect_err("DC integrator evaluation must fail closed");
        assert!(matches!(error, crate::vm::VmError::InvalidNumericResult(_)));
        assert!(error.to_string().contains("DC equilibrium"));
    }

    #[test]
    fn singular_dynamic_solve_does_not_reuse_stale_state() {
        let mut filter = StateSpaceFilter::new(vec![vec![1.0]], vec![1.0], vec![1.0], 0.0)
            .expect("well-formed state-space filter");
        filter
            .set_initial_state(&[7.0])
            .expect("matching finite initial state");
        let error = filter
            .step(1.0, 1.0)
            .expect_err("I - hA is exactly singular");
        assert!(matches!(error, LaplaceError::SingularSystem("transient")));
        assert_eq!(filter.state, vec![7.0]);
        assert_eq!(filter.state_prev, vec![7.0]);
    }

    #[test]
    fn roots_with_only_a_cancelled_imaginary_sum_are_refused() {
        let error = roots_to_polynomial(&[(1.0, 2.0), (3.0, -2.0)])
            .expect_err("equal and opposite imaginary parts are not enough");
        assert!(error.contains("no conjugate partner"));
    }

    #[test]
    fn ordinary_laplace_calls_report_improper_shapes_without_panicking() {
        for expression in [
            "laplace_nd(V(p, n), '{1.0, 2.0}, '{0.5})",
            "laplace_zp(V(p, n), '{-1.0, 0.0, -2.0, 0.0}, '{-3.0, 0.0})",
        ] {
            let result = std::panic::catch_unwind(|| compile_filter(expression));
            let error = result
                .expect("compile validation must not panic")
                .expect_err("improper Laplace shape must fail compilation");
            assert!(
                error.to_string().contains("improper transfer function"),
                "unexpected diagnostic: {error}"
            );
        }
    }

    #[test]
    fn ordinary_laplace_zp_call_rejects_nonconjugate_roots() {
        let error =
            compile_filter("laplace_zp(V(p, n), '{1.0, 2.0, 3.0, -2.0}, '{-1.0, 0.0, -2.0, 0.0})")
                .expect_err("nonconjugate roots cannot define a real transfer function");
        assert!(
            error.to_string().contains("no conjugate partner"),
            "unexpected diagnostic: {error}"
        );
    }

    #[test]
    fn ordinary_laplace_call_accepts_tiny_common_scaling() {
        compile_filter("laplace_nd(V(p, n), '{1.0e-300}, '{1.0e-300, 1.0e-300})")
            .expect("absolute coefficient magnitude must not decide validity");
    }
}
