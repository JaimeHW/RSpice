//! Laplace Transform / S-Domain Transfer Function Support
//!
//! Implements s-domain transfer function evaluation for behavioral sources.
//! LAPLACE expressions in SPICE allow specifying frequency response directly.
//!
//! # Syntax
//! ```text
//! B1 out 0 V=LAPLACE(V(in)) {1/(1+s*R*C)}
//! B2 out 0 V=LAPLACE(V(in), s/100k, 1)  ; First-order high-pass, DC gain=1
//! ```
//!
//! # Theory
//! For transient analysis, we use bilinear (Tustin) transform:
//!   s = (2/T) * (1 - z^-1) / (1 + z^-1)
//!
//! For AC analysis at frequency f, we simply evaluate at s = j*2*pi*f.

use crate::Value;
use num_complex::Complex64;
use std::collections::VecDeque;

//=============================================================================
// Transfer Function Representation
//=============================================================================

/// Represents an s-domain transfer function H(s) = N(s) / D(s)
///
/// Numerator and denominator are represented as polynomial coefficients
/// in ascending order of s: [a0, a1, a2, ...] = a0 + a1*s + a2*s² + ...
#[derive(Debug, Clone)]
pub struct TransferFunction {
    /// Numerator coefficients [n0, n1, n2, ...] for n0 + n1*s + n2*s² + ...
    pub numerator: Vec<Value>,
    /// Denominator coefficients [d0, d1, d2, ...] for d0 + d1*s + d2*s² + ...
    pub denominator: Vec<Value>,
}

impl Default for TransferFunction {
    fn default() -> Self {
        Self {
            numerator: vec![1.0],
            denominator: vec![1.0],
        }
    }
}

impl TransferFunction {
    /// Create a new transfer function H(s) = N(s) / D(s)
    pub fn new(numerator: Vec<Value>, denominator: Vec<Value>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Create integrator: H(s) = k / s
    pub fn integrator(k: Value) -> Self {
        Self {
            numerator: vec![k],
            denominator: vec![0.0, 1.0],
        }
    }

    /// Create differentiator: H(s) = k * s
    pub fn differentiator(k: Value) -> Self {
        Self {
            numerator: vec![0.0, k],
            denominator: vec![1.0],
        }
    }

    /// Evaluate H(s) at given complex frequency
    pub fn evaluate(&self, s: Complex64) -> Complex64 {
        let num = Self::eval_poly(&self.numerator, s);
        let den = Self::eval_poly(&self.denominator, s);

        if den.norm() < 1e-30 {
            // Avoid division by zero
            Complex64::new(1e30, 0.0)
        } else {
            num / den
        }
    }

    /// Evaluate polynomial at complex value
    #[inline]
    fn eval_poly(coeffs: &[Value], s: Complex64) -> Complex64 {
        let mut result = Complex64::new(0.0, 0.0);
        let mut s_power = Complex64::new(1.0, 0.0);

        for &coeff in coeffs {
            result += s_power * coeff;
            s_power *= s;
        }

        result
    }

    /// Get magnitude response at frequency f (Hz)
    pub fn magnitude_at(&self, freq: Value) -> Value {
        let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * freq);
        self.evaluate(s).norm()
    }

    /// Get phase response at frequency f (Hz) in radians
    pub fn phase_at(&self, freq: Value) -> Value {
        let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * freq);
        self.evaluate(s).arg()
    }

    /// Get order of the transfer function (max of num/den degree)
    pub fn order(&self) -> usize {
        self.numerator.len().max(self.denominator.len()) - 1
    }
}

//=============================================================================
// Discrete-Time Filter for Transient Analysis
//=============================================================================

/// Discrete-time filter implementing a transfer function using bilinear transform
///
/// For transient simulation, we convert the s-domain transfer function to
/// a discrete-time difference equation using the bilinear (Tustin) transform.
#[derive(Debug, Clone)]
pub struct DiscreteFilter {
    /// Sample period (seconds)
    sample_period: Value,
    /// Numerator coefficients in z-domain
    b: Vec<Value>,
    /// Denominator coefficients in z-domain (a`[0]` normalized to 1)
    a: Vec<Value>,
    /// Input history (most recent first)
    x_history: VecDeque<Value>,
    /// Output history (most recent first)
    y_history: VecDeque<Value>,
}

impl DiscreteFilter {
    /// Process a single input sample and return the output
    pub fn process(&mut self, x: Value) -> Value {
        // Shift in new input
        self.x_history.pop_back();
        self.x_history.push_front(x);

        // Compute output: y[n] = sum(b[k]*x[n-k]) - sum(a[k]*y[n-k])
        let mut y = 0.0;

        // Numerator contribution (feedforward)
        for (k, &bk) in self.b.iter().enumerate() {
            if let Some(&xk) = self.x_history.get(k) {
                y += bk * xk;
            }
        }

        // Denominator contribution (feedback), skip a[0] which is 1
        for (k, &ak) in self.a.iter().enumerate().skip(1) {
            if let Some(&yk) = self.y_history.get(k - 1) {
                y -= ak * yk;
            }
        }

        // Store output
        self.y_history.pop_back();
        self.y_history.push_front(y);

        y
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        for x in self.x_history.iter_mut() {
            *x = 0.0;
        }
        for y in self.y_history.iter_mut() {
            *y = 0.0;
        }
    }

    /// Get the sample period
    pub fn sample_period(&self) -> Value {
        self.sample_period
    }
}

//=============================================================================
// Tests
//=============================================================================
