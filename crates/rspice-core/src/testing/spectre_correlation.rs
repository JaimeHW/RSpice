//! Spectre correlation helpers and tolerance policy.
//!
//! This module provides reusable comparison primitives for validating
//! RSpice outputs against Spectre-generated golden references.

use crate::Value;

/// Practical default tolerances for Spectre correlation gates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationTolerancePolicy {
    /// Absolute tolerance for scalar/DC values.
    pub scalar_abs: Value,
    /// Relative tolerance for scalar/DC values.
    pub scalar_rel: Value,
    /// Absolute tolerance for point-wise waveform comparisons.
    pub waveform_abs: Value,
    /// Relative tolerance for point-wise waveform comparisons.
    pub waveform_rel: Value,
    /// Maximum allowed RMS relative error for waveform comparisons.
    pub waveform_rms_rel: Value,
}

impl CorrelationTolerancePolicy {
    /// A strict-by-default policy suitable for release gating.
    pub fn release_default() -> Self {
        Self {
            scalar_abs: 1e-9,
            scalar_rel: 1e-4,
            waveform_abs: 5e-9,
            waveform_rel: 2e-3,
            waveform_rms_rel: 5e-4,
        }
    }

    /// Validate tolerance values are finite and non-negative.
    pub fn validate(&self) -> Result<(), String> {
        let checks = [
            ("scalar_abs", self.scalar_abs),
            ("scalar_rel", self.scalar_rel),
            ("waveform_abs", self.waveform_abs),
            ("waveform_rel", self.waveform_rel),
            ("waveform_rms_rel", self.waveform_rms_rel),
        ];
        for (name, value) in checks {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("invalid tolerance {}={}", name, value));
            }
        }
        Ok(())
    }
}

impl Default for CorrelationTolerancePolicy {
    fn default() -> Self {
        Self::release_default()
    }
}

/// Scalar value comparison summary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarComparison {
    pub reference: Value,
    pub candidate: Value,
    pub abs_error: Value,
    pub rel_error: Value,
    pub within_limits: bool,
}

/// Waveform comparison summary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaveformComparison {
    pub samples: usize,
    pub max_abs_error: Value,
    pub max_rel_error: Value,
    pub rms_error: Value,
    pub rms_rel_error: Value,
    pub within_limits: bool,
}

/// Compare two scalar values against the provided tolerances.
pub fn compare_scalar(
    reference: Value,
    candidate: Value,
    abs_tol: Value,
    rel_tol: Value,
) -> Result<ScalarComparison, String> {
    if !reference.is_finite() || !candidate.is_finite() {
        return Err("scalar inputs must be finite".to_string());
    }
    if !abs_tol.is_finite() || abs_tol < 0.0 || !rel_tol.is_finite() || rel_tol < 0.0 {
        return Err("scalar tolerances must be finite and non-negative".to_string());
    }

    let abs_error = (candidate - reference).abs();
    let rel_error = abs_error / reference.abs().max(1e-30);
    Ok(ScalarComparison {
        reference,
        candidate,
        abs_error,
        rel_error,
        within_limits: abs_error <= abs_tol || rel_error <= rel_tol,
    })
}

/// Compare two waveforms with linear interpolation onto reference X samples.
///
/// `reference_x` and `reference_y` define the golden waveform.
/// `candidate_x` and `candidate_y` define the waveform under test.
pub fn compare_waveform(
    reference_x: &[Value],
    reference_y: &[Value],
    candidate_x: &[Value],
    candidate_y: &[Value],
    abs_tol: Value,
    rel_tol: Value,
    rms_rel_tol: Value,
) -> Result<WaveformComparison, String> {
    if reference_x.is_empty() || reference_y.is_empty() {
        return Err("reference waveform is empty".to_string());
    }
    if candidate_x.is_empty() || candidate_y.is_empty() {
        return Err("candidate waveform is empty".to_string());
    }
    if reference_x.len() != reference_y.len() {
        return Err("reference waveform x/y length mismatch".to_string());
    }
    if candidate_x.len() != candidate_y.len() {
        return Err("candidate waveform x/y length mismatch".to_string());
    }
    if !abs_tol.is_finite()
        || abs_tol < 0.0
        || !rel_tol.is_finite()
        || rel_tol < 0.0
        || !rms_rel_tol.is_finite()
        || rms_rel_tol < 0.0
    {
        return Err("waveform tolerances must be finite and non-negative".to_string());
    }
    if !is_strictly_increasing(reference_x) {
        return Err("reference x-axis must be strictly increasing".to_string());
    }
    if !is_strictly_increasing(candidate_x) {
        return Err("candidate x-axis must be strictly increasing".to_string());
    }

    let mut max_abs_error: Value = 0.0;
    let mut max_rel_error: Value = 0.0;
    let mut squared_error_sum: Value = 0.0;
    let mut squared_rel_error_sum: Value = 0.0;
    let mut all_points_within_limits = true;

    for (&x, &reference_value) in reference_x.iter().zip(reference_y.iter()) {
        let candidate_value = interpolate_linear(candidate_x, candidate_y, x)
            .ok_or_else(|| format!("candidate waveform does not cover x={}", x))?;
        let abs_error = (candidate_value - reference_value).abs();
        let rel_error = abs_error / reference_value.abs().max(1e-30);
        max_abs_error = max_abs_error.max(abs_error);
        max_rel_error = max_rel_error.max(rel_error);
        squared_error_sum += abs_error * abs_error;
        squared_rel_error_sum += rel_error * rel_error;
        if !(abs_error <= abs_tol || rel_error <= rel_tol) {
            all_points_within_limits = false;
        }
    }

    let samples = reference_x.len();
    let rms_error = (squared_error_sum / samples as Value).sqrt();
    let rms_rel_error = (squared_rel_error_sum / samples as Value).sqrt();
    let within_limits = all_points_within_limits && rms_rel_error <= rms_rel_tol;

    Ok(WaveformComparison {
        samples,
        max_abs_error,
        max_rel_error,
        rms_error,
        rms_rel_error,
        within_limits,
    })
}

fn is_strictly_increasing(values: &[Value]) -> bool {
    values.windows(2).all(|w| w[1] > w[0])
}

fn interpolate_linear(x: &[Value], y: &[Value], target_x: Value) -> Option<Value> {
    if target_x < x[0] || target_x > x[x.len() - 1] {
        return None;
    }
    let mut lo = 0usize;
    let mut hi = x.len() - 1;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if x[mid] <= target_x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let x0 = x[lo];
    let x1 = x[hi];
    let y0 = y[lo];
    let y1 = y[hi];
    let span = x1 - x0;
    if span <= 0.0 {
        return None;
    }
    let t = (target_x - x0) / span;
    Some(y0 + t * (y1 - y0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_policy_validates() {
        let policy = CorrelationTolerancePolicy::release_default();
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_compare_scalar_within_rel_tolerance() {
        let cmp = compare_scalar(1.0, 1.00005, 1e-8, 1e-4).expect("scalar comparison should work");
        assert!(cmp.within_limits);
        assert!(cmp.abs_error > 1e-8);
        assert!(cmp.rel_error <= 1e-4);
    }

    #[test]
    fn test_compare_scalar_outside_tolerance() {
        let cmp = compare_scalar(1.0, 1.01, 1e-5, 1e-4).expect("scalar comparison should work");
        assert!(!cmp.within_limits);
    }

    #[test]
    fn test_compare_waveform_with_interpolation() {
        let reference_x = vec![0.0, 0.5, 1.0];
        let reference_y = vec![0.0, 1.0, 0.0];
        let candidate_x = vec![0.0, 0.45, 0.55, 1.0];
        let candidate_y = vec![0.0, 0.9, 0.9, 0.0];
        let cmp = compare_waveform(
            &reference_x,
            &reference_y,
            &candidate_x,
            &candidate_y,
            0.11,
            0.11,
            0.11,
        )
        .expect("waveform comparison should work");
        assert!(cmp.within_limits);
        assert!(cmp.max_abs_error <= 0.11);
    }

    #[test]
    fn test_compare_waveform_detects_mismatch() {
        let reference_x = vec![0.0, 1.0, 2.0];
        let reference_y = vec![1.0, 2.0, 3.0];
        let candidate_x = vec![0.0, 1.0, 2.0];
        let candidate_y = vec![1.0, 2.5, 4.0];
        let cmp = compare_waveform(
            &reference_x,
            &reference_y,
            &candidate_x,
            &candidate_y,
            0.1,
            0.02,
            0.02,
        )
        .expect("waveform comparison should work");
        assert!(!cmp.within_limits);
        assert!(cmp.max_rel_error > 0.02);
    }

    #[test]
    fn test_compare_waveform_rejects_non_monotonic_axis() {
        let err = compare_waveform(
            &[0.0, 1.0, 0.5],
            &[0.0, 1.0, 2.0],
            &[0.0, 1.0, 2.0],
            &[0.0, 1.0, 2.0],
            0.1,
            0.1,
            0.1,
        )
        .expect_err("non-monotonic reference x-axis should fail");
        assert!(err.contains("strictly increasing"));
    }

    #[test]
    fn test_compare_waveform_rejects_out_of_range_reference_points() {
        let err = compare_waveform(
            &[0.0, 1.0, 2.0],
            &[0.0, 1.0, 2.0],
            &[0.1, 1.0, 2.0],
            &[0.0, 1.0, 2.0],
            0.1,
            0.1,
            0.1,
        )
        .expect_err("missing candidate coverage should fail");
        assert!(err.contains("does not cover"));
    }
}
