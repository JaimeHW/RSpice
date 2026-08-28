//! Focused regression tests for descriptor pole-zero semantics.
//!
//! These tests deliberately use the public dense analyzer so failures identify
//! pole-zero math/result-contract regressions without involving netlist parsing,
//! device stamping, or the sparse engine path.

use rspice_core::analysis::{
    PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult, PzMatrix,
    RootSetEvidence, SpectrumCertificate, StabilityVerdict,
};
use rspice_core::{Complex64, Engine, Netlist, SimulationConfig};

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
    let certificate = result
        .pole_evidence
        .certificate()
        .expect("a computed descriptor spectrum carries a certificate");
    assert_eq!(certificate.problem_order, 2);
    assert_eq!(certificate.infinite_count, 1);
    assert_eq!(certificate.finite_count(), 1);
    assert!(result.pole_evidence.is_consistent_with(&result.poles));
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
    assert!(matches!(
        poles.pole_evidence,
        RootSetEvidence::Qualified { .. }
    ));
    assert_eq!(poles.zero_evidence, RootSetEvidence::NotRequested);

    let mut zeros_config = PoleZeroConfig::poles_and_zeros(0, 0);
    zeros_config.compute_poles = false;
    let zeros = analyzer
        .analyze(&zeros_config)
        .expect("zero-only extraction");
    assert!(zeros.poles.is_empty(), "{:#?}", zeros.poles);
    assert_eq!(zeros.zeros.len(), 1, "{:#?}", zeros.zeros);
    assert_eq!(zeros.pole_evidence, RootSetEvidence::NotRequested);
    assert!(matches!(
        zeros.zero_evidence,
        RootSetEvidence::Qualified { .. }
    ));
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
    assert_eq!(result.stability_verdict(), StabilityVerdict::Indeterminate);
}

#[test]
fn a_regular_all_infinite_spectrum_is_qualified_empty() {
    // det(1 + s*0) never vanishes at finite s. The regular order-one pencil
    // therefore has exactly one infinite eigenvalue and no finite poles.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0]]),
        PzMatrix::from_dense(vec![vec![0.0]]),
    );

    let result = analyzer
        .analyze(&poles_only(0, 0))
        .expect("the constant scalar pencil is regular");

    assert!(result.poles.is_empty());
    let RootSetEvidence::QualifiedEmpty { certificate } = &result.pole_evidence else {
        panic!("expected qualified-empty evidence");
    };
    assert_eq!(certificate.problem_order, 1);
    assert_eq!(certificate.infinite_count, 1);
    assert_eq!(certificate.finite_count(), 0);
    assert_eq!(result.stability_verdict(), StabilityVerdict::Stable);
}

#[test]
fn scalar_rc_pole_and_zero_spectrum_is_qualified() {
    // H(s)=1/(1+s*1e-3) has one pole at -1000 rad/s and no finite zeros.
    // The zero calculation uses a Rosenbrock pencil with an infinite pair,
    // which must still admit a finite, normwise residual certificate.
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![vec![1.0]]),
        PzMatrix::from_dense(vec![vec![1.0e-3]]),
    );

    let result = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 0))
        .expect("a scalar RC transfer has a regular pole-zero spectrum");

    assert_eq!(result.poles, vec![Complex64::new(-1000.0, 0.0)]);
    assert!(result.zeros.is_empty());
    assert!(matches!(
        result.pole_evidence,
        RootSetEvidence::Qualified { .. }
    ));
    assert!(matches!(
        result.zero_evidence,
        RootSetEvidence::QualifiedEmpty { .. }
    ));
    assert!(result.has_consistent_root_evidence());
}

#[test]
fn ideal_voltage_source_rejects_a_parallel_current_input_transfer() {
    // The ideal source clamps node 0, so an additional current injected at
    // that same node is sunk by the source and has identically zero transfer
    // to node 1. This is not a qualified empty zero set.
    let conductance = 1.0e-3;
    let analyzer = PoleZeroAnalyzer::new(
        PzMatrix::from_dense(vec![
            vec![conductance, -conductance, 1.0],
            vec![-conductance, conductance, 0.0],
            vec![1.0, 0.0, 0.0],
        ]),
        PzMatrix::from_dense(vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 1.0e-6, 0.0],
            vec![0.0, 0.0, 0.0],
        ]),
    );

    let error = analyzer
        .analyze(&PoleZeroConfig::poles_and_zeros(0, 1))
        .expect_err("the clamped current-to-voltage transfer is identically zero");
    assert!(
        matches!(error, PoleZeroAnalysisError::TransferExtraction(_)),
        "unexpected error: {error}"
    );
}

#[test]
fn engine_rejects_current_input_at_an_ideal_voltage_source_node() {
    let netlist =
        Netlist::parse("* voltage-driven RC\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.end\n")
            .expect("RC deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let circuit = engine.build_circuit(&netlist).expect("RC circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");

    let error = engine
        .run_pz(&netlist, input, output)
        .expect_err("a current input cannot parallel an ideal voltage source");
    assert!(
        error
            .to_string()
            .to_ascii_lowercase()
            .contains("transfer extraction"),
        "unexpected error: {error}"
    );
}

#[test]
fn current_driven_rc_engine_path_accepts_its_infinite_zero_chain() {
    let netlist = Netlist::parse(
        "* current-driven RC PZ\nI1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.end\n",
    )
    .expect("RC deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let circuit = engine.build_circuit(&netlist).expect("RC circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");

    let result = engine
        .run_pz(&netlist, input, output)
        .expect("current-driven RC pole-zero solve succeeds");

    assert_eq!(result.poles.len(), 1, "{:#?}", result.poles);
    assert!(result.zeros.is_empty(), "{:#?}", result.zeros);
    assert!(matches!(
        result.zero_evidence,
        RootSetEvidence::QualifiedEmpty { .. }
    ));
    assert!(result.has_consistent_root_evidence());
}

#[test]
fn root_evidence_invariants_reject_mismatched_vectors() {
    let finite = SpectrumCertificate::exact(1, 0).expect("valid exact certificate");
    let empty = SpectrumCertificate::exact(1, 1).expect("valid exact certificate");
    let qualified = RootSetEvidence::Qualified {
        certificate: finite,
    };
    let qualified_empty = RootSetEvidence::QualifiedEmpty { certificate: empty };

    assert!(qualified.is_consistent_with(&[Complex64::new(-1.0, 0.0)]));
    assert!(!qualified.is_consistent_with(&[]));
    assert!(qualified_empty.is_consistent_with(&[]));
    assert!(!qualified_empty.is_consistent_with(&[Complex64::new(-1.0, 0.0)]));
    assert!(RootSetEvidence::from_certificate(0, finite).is_none());
    assert!(RootSetEvidence::from_certificate(1, empty).is_none());
    assert!(SpectrumCertificate::new(1, 0, 0.0, 1.0).is_none());
}

#[test]
fn approximate_and_legacy_poles_have_indeterminate_stability() {
    let strict_tolerance = 128.0 * f64::EPSILON;
    let certificate = SpectrumCertificate::new(1, 0, 1.0e-9, strict_tolerance)
        .expect("a complete spectrum below the hard residual limit is valid");
    let mut result = PoleZeroResult::new("input", "output");
    result.poles = vec![Complex64::new(-1.0, 0.0)];
    result.pole_evidence = RootSetEvidence::from_certificate(1, certificate)
        .expect("the certificate count matches the root vector");

    assert!(matches!(
        result.pole_evidence,
        RootSetEvidence::Approximate { .. }
    ));
    assert_eq!(result.stability_verdict(), StabilityVerdict::Indeterminate);

    result.pole_evidence = RootSetEvidence::LegacyUnknown;
    assert_eq!(result.stability_verdict(), StabilityVerdict::Indeterminate);
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
