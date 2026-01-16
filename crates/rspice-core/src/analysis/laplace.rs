//! Laplace Transform / S-Domain Transfer Function Support
//!
//! Implements s-domain transfer function evaluation for behavioral sources.
//! LAPLACE expressions in SPICE allow specifying frequency response directly.
//!
//! # LTspice Syntax
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
        Self { numerator, denominator }
    }

    /// Create first-order low-pass: H(s) = 1 / (1 + s*tau)
    pub fn lowpass_1st(tau: Value) -> Self {
        Self {
            numerator: vec![1.0],
            denominator: vec![1.0, tau],
        }
    }

    /// Create first-order high-pass: H(s) = s*tau / (1 + s*tau)
    pub fn highpass_1st(tau: Value) -> Self {
        Self {
            numerator: vec![0.0, tau],
            denominator: vec![1.0, tau],
        }
    }

    /// Create second-order low-pass: H(s) = wn² / (s² + 2*zeta*wn*s + wn²)
    pub fn lowpass_2nd(omega_n: Value, zeta: Value) -> Self {
        let wn2 = omega_n * omega_n;
        Self {
            numerator: vec![wn2],
            denominator: vec![wn2, 2.0 * zeta * omega_n, 1.0],
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
    /// Transfer function specification
    tf: TransferFunction,
    /// Sample period (seconds)
    sample_period: Value,
    /// Numerator coefficients in z-domain
    b: Vec<Value>,
    /// Denominator coefficients in z-domain (a[0] normalized to 1)
    a: Vec<Value>,
    /// Input history (most recent first)
    x_history: VecDeque<Value>,
    /// Output history (most recent first)
    y_history: VecDeque<Value>,
}

impl DiscreteFilter {
    /// Create a discrete filter from transfer function H(s)
    /// 
    /// Uses bilinear transform: s = (2/T) * (z-1)/(z+1)
    pub fn from_tf(tf: TransferFunction, sample_period: Value) -> Self {
        let order = tf.order();
        let (b, a) = Self::bilinear_transform(&tf, sample_period);
        
        Self {
            tf,
            sample_period,
            b,
            a,
            x_history: VecDeque::from(vec![0.0; order + 1]),
            y_history: VecDeque::from(vec![0.0; order + 1]),
        }
    }

    /// Apply bilinear (Tustin) transform to convert H(s) to H(z)
    /// 
    /// Substitutes s = (2/T) * (1-z^-1) / (1+z^-1)
    fn bilinear_transform(tf: &TransferFunction, t: Value) -> (Vec<Value>, Vec<Value>) {
        let k = 2.0 / t;
        let order = tf.order();
        
        // For simplicity, handle up to second order directly
        // Higher orders would need proper polynomial transformation
        
        match order {
            0 => {
                // Zero-order: H(s) = n0/d0 => H(z) = n0/d0
                let gain = tf.numerator.get(0).copied().unwrap_or(1.0) 
                         / tf.denominator.get(0).copied().unwrap_or(1.0);
                (vec![gain], vec![1.0])
            }
            1 => {
                // First-order: H(s) = (n0 + n1*s) / (d0 + d1*s)
                let n0 = tf.numerator.get(0).copied().unwrap_or(0.0);
                let n1 = tf.numerator.get(1).copied().unwrap_or(0.0);
                let d0 = tf.denominator.get(0).copied().unwrap_or(1.0);
                let d1 = tf.denominator.get(1).copied().unwrap_or(0.0);
                
                // s = k * (1-z^-1)/(1+z^-1) = k * (z-1)/(z+1)
                // Multiply through by (z+1) / (z+1)
                // Numerator: n0 + n1*k*(z-1)/(z+1) => (n0*(z+1) + n1*k*(z-1)) / (z+1)
                //          = ((n0+n1*k)*z + (n0-n1*k)) / (z+1)
                // Denominator: (d0*(z+1) + d1*k*(z-1)) / (z+1)
                //            = ((d0+d1*k)*z + (d0-d1*k)) / (z+1)
                
                let b0 = n0 + n1 * k;
                let b1 = n0 - n1 * k;
                let a0 = d0 + d1 * k;
                let a1 = d0 - d1 * k;
                
                // Normalize so a[0] = 1
                (vec![b0 / a0, b1 / a0], vec![1.0, a1 / a0])
            }
            _ => {
                // Second order and higher - use simplified approximation
                // For proper implementation, would need polynomial convolution
                let n0 = tf.numerator.get(0).copied().unwrap_or(0.0);
                let n1 = tf.numerator.get(1).copied().unwrap_or(0.0);
                let n2 = tf.numerator.get(2).copied().unwrap_or(0.0);
                let d0 = tf.denominator.get(0).copied().unwrap_or(1.0);
                let d1 = tf.denominator.get(1).copied().unwrap_or(0.0);
                let d2 = tf.denominator.get(2).copied().unwrap_or(0.0);
                
                let k2 = k * k;
                
                // Bilinear transform for second order
                let b0 = n0 + n1 * k + n2 * k2;
                let b1 = 2.0 * n0 - 2.0 * n2 * k2;
                let b2 = n0 - n1 * k + n2 * k2;
                
                let a0 = d0 + d1 * k + d2 * k2;
                let a1 = 2.0 * d0 - 2.0 * d2 * k2;
                let a2 = d0 - d1 * k + d2 * k2;
                
                (
                    vec![b0 / a0, b1 / a0, b2 / a0],
                    vec![1.0, a1 / a0, a2 / a0],
                )
            }
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    
    fn approx_eq(a: Value, b: Value, tol: Value) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_lowpass_dc_gain() {
        let tf = TransferFunction::lowpass_1st(1e-3); // 1ms time constant
        
        // DC gain should be 1
        let dc = tf.magnitude_at(0.0);
        assert!(approx_eq(dc, 1.0, 1e-10));
    }

    #[test]
    fn test_lowpass_corner_frequency() {
        let tau = 1e-3; // 1ms
        let fc = 1.0 / (2.0 * std::f64::consts::PI * tau); // ~159 Hz
        let tf = TransferFunction::lowpass_1st(tau);
        
        // At corner frequency, gain should be -3dB ≈ 0.707
        let mag = tf.magnitude_at(fc);
        assert!(approx_eq(mag, 0.707, 0.01));
    }

    #[test]
    fn test_highpass_dc_rejection() {
        let tf = TransferFunction::highpass_1st(1e-3);
        
        // DC gain should be 0
        let dc = tf.magnitude_at(0.0);
        assert!(dc < 1e-10);
    }

    #[test]
    fn test_discrete_filter_step() {
        let tf = TransferFunction::lowpass_1st(0.01); // 10ms time constant
        let mut filt = DiscreteFilter::from_tf(tf, 0.001); // 1ms sample period
        
        // Apply step input
        let mut output = 0.0;
        for _ in 0..100 {
            output = filt.process(1.0);
        }
        
        // After ~10 time constants, output should approach 1
        assert!(output > 0.99);
    }

    #[test]
    fn test_integrator() {
        let tf = TransferFunction::integrator(1.0);
        
        // |H(jw)| = 1/w, so at w=1 rad/s (f ≈ 0.159 Hz), magnitude = 1
        let freq = 1.0 / (2.0 * std::f64::consts::PI);
        let mag = tf.magnitude_at(freq);
        assert!(approx_eq(mag, 1.0, 0.1));
    }

    #[test]
    fn test_second_order_lowpass() {
        let wn = 2.0 * std::f64::consts::PI * 100.0; // 100 Hz natural freq
        let zeta = 0.707; // Butterworth (maximally flat)
        let tf = TransferFunction::lowpass_2nd(wn, zeta);
        
        // DC gain should be 1
        let dc = tf.magnitude_at(0.0);
        assert!(approx_eq(dc, 1.0, 1e-6));
        
        // At wn, gain should be 1/(2*zeta) for second order
        let mag_wn = tf.magnitude_at(100.0);
        assert!(approx_eq(mag_wn, 1.0 / (2.0 * zeta), 0.1));
    }
}
