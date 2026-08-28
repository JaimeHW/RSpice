//! Focused regression tests for descriptor pole-zero semantics.
//!
//! These tests deliberately use the public dense analyzer so failures identify
//! pole-zero math/result-contract regressions without involving netlist parsing,
//! device stamping, or the sparse engine path.

use rspice_core::Complex64;
use rspice_core::analysis::{
    PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult, PzMatrix,
};

fn poles_only(input: usize, output: usize) -> PoleZeroConfig {
    let mut config = PoleZeroConfig::poles_and_zeros(input, output);
    config.compute_zeros = false;
    config
}

fn assert_root_present(actual: &[Complex64], expected: Complex64, relative_tolerance: f64) {
    let found = actual.iter().any(|root| {
        let scale = root.norm().max(expected.norm()).max(1.0);
        (root.re - expected.re).abs() <= relative_tolerance * scale
            && (root.im - expected.im).abs() <= relative_tolerance * scale
    });
    assert!(
        found,
        "missing expected root {expected}; actual roots: {actual:#?}"
    );
}

#[test]
fn coupled_descriptor_preserves_complex_conjugate_poles() {
    // det(G + sC) = (s + 1)^2 + 4, hence s = -1 +/- j2.
    // A diagonal -Gii/Cii approximation would incorrectly return two copies
    // of -1 and therefore fail this regression.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, -2.0], vec![2.0, 1.0]]),
        PzMatrix::identity(2),
    );

    let result = analyzer
        .analyze(&poles_only(0, 1))
        .expect("coupled descriptor is regular");

    assert_eq!(result.poles.len(), 2, "{:#?}", result.poles);
    assert_root_present(&result.poles, Complex64::new(-1.0, 2.0), 1.0e-10);
    assert_root_present(&result.poles, Complex64::new(-1.0, -2.0), 1.0e-10);
}

#[test]
fn genuine_low_frequency_complex_poles_are_not_rounded_to_real() {
    let imaginary_part = 1.0e-13;
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, -imaginary_part], vec![imaginary_part, 1.0]]),
        PzMatrix::identity(2),
    );

    let result = analyzer
        .analyze(&poles_only(0, 1))
        .expect("the near-real conjugate pair is a regular spectrum");

    assert_root_present(&result.poles, Complex64::new(-1.0, imaginary_part), 1.0e-15);
    assert_root_present(
        &result.poles,
        Complex64::new(-1.0, -imaginary_part),
        1.0e-15,
    );
}

#[test]
fn regular_singular_descriptor_eliminates_algebraic_variable() {
    // x0 is dynamic and x1 is algebraic:
    //   x0' + x0 - x1 = 0
    //          -x0 + 2*x1 = 0
    // Eliminating x1 gives x0' + 0.5*x0 = 0. The zero row/column in C
    // represents an infinite generalized eigenvalue and must not create or
    // erase the single finite pole.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, -1.0], vec![-1.0, 2.0]]),
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 0.0]]),
    );

    let result = analyzer
        .analyze(&poles_only(0, 0))
        .expect("singular C with an invertible algebraic partition is regular");

    assert_eq!(result.poles.len(), 1, "{:#?}", result.poles);
    assert_root_present(&result.poles, Complex64::new(-0.5, 0.0), 1.0e-10);
}

#[test]
fn descriptor_roots_are_invariant_to_a_common_small_scale() {
    let scale = 1.0e-24;
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![2.0 * scale, -scale], vec![-scale, 2.0 * scale]]),
        PzMatrix::from_dense(vec![vec![scale, 0.0], vec![0.0, scale]]),
    );

    let result = analyzer
        .analyze(&poles_only(0, 0))
        .expect("common descriptor scaling must not change the spectrum");

    assert_eq!(result.poles.len(), 2, "{:#?}", result.poles);
    assert_root_present(&result.poles, Complex64::new(-1.0, 0.0), 1.0e-10);
    assert_root_present(&result.poles, Complex64::new(-3.0, 0.0), 1.0e-10);
}

#[test]
fn common_small_descriptor_scale_preserves_a_two_by_two_zero() {
    // H00(s) = (s + 2) / ((s + 1)(s + 3)). Scaling both G and C
    // multiplies the transfer by only a constant and cannot move its zero.
    let scale = 1.0e-24;
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![2.0 * scale, -scale], vec![-scale, 2.0 * scale]]),
        PzMatrix::from_dense(vec![vec![scale, 0.0], vec![0.0, scale]]),
    );

    let result = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 0))
        .expect("a common nonzero scale must preserve transfer zeros");

    assert_eq!(result.zeros.len(), 1, "{:#?}", result.zeros);
    assert_root_present(&result.zeros, Complex64::new(-2.0, 0.0), 1.0e-10);
}

#[test]
fn identically_zero_transfer_is_a_transfer_extraction_error() {
    // The two states are decoupled. Driving state 0 and observing state 1
    // makes H10(s) identically zero, so there is no finite numerator root set
    // to report as a successful transfer extraction.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 2.0]]),
        PzMatrix::identity(2),
    );

    let error = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 1))
        .expect_err("an identically-zero transfer must not look successful");

    assert!(
        matches!(error, PoleZeroAnalysisError::TransferExtraction(_)),
        "unexpected error: {error}"
    );
}

#[test]
fn close_but_distinct_pole_and_zero_are_both_retained() {
    // Choose trace(G)=6 and det(G)=8, so the poles are exactly -2 and -4,
    // while H00's numerator is s+2.000001. A proximity-only cancellation
    // heuristic would erase useful conditioning evidence here.
    let g22 = 2.000_001;
    let g11 = 6.0 - g22;
    let coupling_product = g11 * g22 - 8.0;
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![g11, 1.0], vec![coupling_product, g22]]),
        PzMatrix::identity(2),
    );

    let result = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 0))
        .expect("the regular near-cancellation transfer is extractable");

    assert_eq!(result.poles.len(), 2, "{:#?}", result.poles);
    assert_eq!(result.zeros.len(), 1, "{:#?}", result.zeros);
    assert_root_present(&result.poles, Complex64::new(-2.0, 0.0), 1.0e-10);
    assert_root_present(&result.zeros, Complex64::new(-2.000_001, 0.0), 1.0e-10);
    assert!(
        (result.poles[0].re - result.zeros[0].re).abs() > 5.0e-7,
        "distinct roots were numerically collapsed: poles={:#?}, zeros={:#?}",
        result.poles,
        result.zeros
    );
}

#[test]
fn requested_pole_and_zero_sets_are_independent() {
    // H00(s) = (s + 2) / ((s + 2)^2 - 1), which has one finite zero
    // at -2 and two finite poles at -1 and -3.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![2.0, -1.0], vec![-1.0, 2.0]]),
        PzMatrix::identity(2),
    );

    let all = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 0))
        .expect("coupled descriptor has poles and zeros");
    assert_eq!(all.poles.len(), 2, "{:#?}", all.poles);
    assert_eq!(all.zeros.len(), 1, "{:#?}", all.zeros);
    assert_root_present(&all.poles, Complex64::new(-1.0, 0.0), 1.0e-10);
    assert_root_present(&all.poles, Complex64::new(-3.0, 0.0), 1.0e-10);
    assert_root_present(&all.zeros, Complex64::new(-2.0, 0.0), 1.0e-10);

    let mut poles_config = PoleZeroConfig::poles_and_zeros(0, 0);
    poles_config.compute_zeros = false;
    let poles = analyzer
        .analyze(&poles_config)
        .expect("pole-only extraction");
    assert_eq!(poles.poles.len(), 2, "{:#?}", poles.poles);
    assert!(poles.zeros.is_empty(), "{:#?}", poles.zeros);

    let mut zeros_config = PoleZeroConfig::poles_and_zeros(0, 0);
    zeros_config.compute_poles = false;
    let zeros = analyzer
        .analyze(&zeros_config)
        .expect("zero-only extraction");
    assert!(zeros.poles.is_empty(), "{:#?}", zeros.poles);
    assert_eq!(zeros.zeros.len(), 1, "{:#?}", zeros.zeros);
    assert_root_present(&zeros.zeros, Complex64::new(-2.0, 0.0), 1.0e-10);
}

#[test]
fn an_empty_pole_set_is_not_evidence_of_stability() {
    // An empty set can mean that poles were not requested, were filtered, or
    // could not be computed. None of those states proves asymptotic stability.
    let result = PoleZeroResult::new("input", "output");
    assert!(
        !result.is_stable(),
        "an absent pole set must be indeterminate rather than vacuously stable"
    );
}

#[test]
fn an_irregular_descriptor_is_an_error_not_a_diagonal_estimate() {
    // The second row is 0 + s*0, so det(G+sC) is identically zero. Returning
    // only the first diagonal root (-1) would misrepresent an irregular
    // pencil as a complete first-order system.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 0.0]]),
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 0.0]]),
    );

    let error = analyzer
        .analyze(&poles_only(0, 0))
        .expect_err("an irregular descriptor cannot produce qualified poles");

    assert!(
        matches!(error, PoleZeroAnalysisError::IrregularDescriptor { .. }),
        "unexpected error: {error}"
    );
}

#[test]
fn empty_descriptors_are_rejected() {
    let analyzer = PoleZeroAnalyzer::new(PzMatrix::zeros(0, 0), PzMatrix::zeros(0, 0));
    for config in [poles_only(0, 0), PoleZeroConfig::poles_and_zeros(0, 0)] {
        let error = analyzer
            .analyze(&config)
            .expect_err("an empty descriptor has no qualified transfer or spectrum");

        assert!(
            matches!(error, PoleZeroAnalysisError::InvalidSystem(_)),
            "unexpected error: {error}"
        );
    }
}

#[test]
fn a_frequency_limit_cannot_silently_hide_a_pole() {
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 100.0]]),
        PzMatrix::identity(2),
    );
    let mut config = poles_only(0, 0);
    config.max_pole_freq = 1.0; // 2π rad/s, below the second pole magnitude.

    let error = analyzer
        .analyze(&config)
        .expect_err("a truncated spectrum must not look complete");

    assert!(
        matches!(
            error,
            PoleZeroAnalysisError::FrequencyLimitExceeded {
                quantity: "pole",
                omitted: 1,
                ..
            }
        ),
        "unexpected error: {error}"
    );
}

#[test]
fn zero_only_extraction_ignores_an_unrequested_pole_cutoff() {
    // trace(G)=101 and det(G)=100 give poles at -1 and -100, while H00 has
    // its only zero at -2. A 1 Hz reporting cutoff admits the zero but not the
    // fast pole. Since poles were not requested, that pole must not reject a
    // complete zero-only result.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![99.0, 1.0], vec![98.0, 2.0]]),
        PzMatrix::identity(2),
    );
    let mut config = PoleZeroConfig::poles_and_zeros(0, 0);
    config.compute_poles = false;
    config.max_pole_freq = 1.0;

    let result = analyzer
        .analyze(&config)
        .expect("an unrequested pole cannot invalidate zero-only extraction");

    assert!(result.poles.is_empty(), "{:#?}", result.poles);
    assert_eq!(result.zeros.len(), 1, "{:#?}", result.zeros);
    assert_root_present(&result.zeros, Complex64::new(-2.0, 0.0), 1.0e-10);
}

#[test]
fn overflowing_frequency_limit_is_rejected_fail_closed() {
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0]]),
        PzMatrix::from_dense(vec![vec![1.0]]),
    );
    let mut config = poles_only(0, 0);
    config.max_pole_freq = f64::MAX;

    let error = analyzer
        .analyze(&config)
        .expect_err("a Hz-to-rad/s overflow must not disable the cutoff");

    assert!(
        matches!(error, PoleZeroAnalysisError::InvalidSystem(_)),
        "unexpected error: {error}"
    );
}

#[test]
fn an_unavailable_dc_gain_is_not_replaced_with_unity() {
    // H(s)=1/s has no finite DC gain. The root set is still meaningful, but
    // callers must not receive a fabricated gain of one.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![0.0]]),
        PzMatrix::from_dense(vec![vec![1.0]]),
    );

    let result = analyzer
        .analyze(&poles_only(0, 0))
        .expect("the integrator descriptor is regular");

    assert_eq!(result.poles, vec![Complex64::new(0.0, 0.0)]);
    assert_eq!(result.dc_gain, None);
    assert!(!result.is_stable());
}

#[test]
fn dc_gain_rejects_a_ragged_public_matrix_without_panicking() {
    // Matrix::from_dense is public and can represent ragged input. The direct
    // DC-gain helper must validate that boundary just like full PZ analysis.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0, 0.0], vec![0.0]]),
        PzMatrix::identity(2),
    );

    let call = std::panic::catch_unwind(|| analyzer.dc_gain(0, 1));
    let gain = call.expect("dc_gain must not panic on a ragged public matrix");
    assert_eq!(gain, None);
}

#[test]
fn generalized_solver_retains_a_large_finite_root() {
    // A rank-one C forces the generalized QZ path. Its one finite root is
    // -1e14 rad/s; a small-beta heuristic used to misclassify it as infinite.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![2.0e14, 0.0], vec![0.0, 2.0e14]]),
        PzMatrix::from_dense(vec![vec![1.0, 1.0], vec![1.0, 1.0]]),
    );
    let mut config = poles_only(0, 0);
    config.max_pole_freq = 2.0e13;

    let result = analyzer
        .analyze(&config)
        .expect("large finite generalized root must remain finite");

    assert_eq!(result.poles.len(), 1, "{:#?}", result.poles);
    assert_root_present(&result.poles, Complex64::new(-1.0e14, 0.0), 1.0e-10);
}
