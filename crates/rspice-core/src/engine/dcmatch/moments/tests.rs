use super::*;
use crate::abort_signal::CountingAbort;

fn entry(index: usize, value: Value) -> DcMatchContributor {
    contributor(
        DcMatchScope::Mismatch,
        "B1".into(),
        format!("P{index}"),
        1.0,
        value,
    )
    .unwrap()
}

fn card() -> DcMatchCard {
    DcMatchCard {
        contributor_limit: 0,
        sigma_multiplier: 1.0,
        ..DcMatchCard::voltage_probe("OUT")
    }
}

fn correlations(matrix: Option<Vec<Vec<Value>>>) -> ScopeCorrelations {
    ScopeCorrelations {
        mismatch: matrix.map(|matrix| ScopeCorrelation {
            statements: 1,
            order: (0..matrix.len()).map(|index| format!("P{index}")).collect(),
            matrix: SpectreCorrelationMatrix::new(matrix).unwrap(),
        }),
        process: None,
    }
}

#[test]
fn finite_sigmas_and_shares_do_not_require_representable_variances() {
    for scale in [f64::from_bits(1), 1e-200, 1.0, 1e200, f64::MAX / 8.0] {
        let mut entries = vec![entry(0, 3.0 * scale), entry(1, 4.0 * scale)];
        // Independent process and mismatch scopes also combine before the root.
        entries[1].scope = DcMatchScope::Process;
        let result = assemble(
            &card(),
            "V(OUT)".into(),
            0.0,
            entries,
            &correlations(None),
            &NoAbort,
        )
        .unwrap();
        assert!(
            (result.sigma_total / (5.0 * scale) - 1.0).abs() < 2e-15,
            "{scale}: {result:?}"
        );
        assert_eq!(result.sigma_mismatch, 3.0 * scale);
        assert_eq!(result.sigma_process, 4.0 * scale);
        for entry in result.contributors {
            let expected = if entry.parameter == "P0" {
                9.0 / 25.0
            } else {
                16.0 / 25.0
            };
            assert!((entry.share - expected).abs() < 2e-15, "{scale}: {entry:?}");
        }
    }
}

#[test]
fn correlated_covariance_preserves_scale_and_cancellation_between_rows() {
    for scale in [1e-200, 1.0, 1e200] {
        let result = assemble(
            &card(),
            "V(OUT)".into(),
            0.0,
            vec![entry(0, 3.0 * scale), entry(1, 4.0 * scale)],
            &correlations(Some(vec![vec![1.0, 0.5], vec![0.5, 1.0]])),
            &NoAbort,
        )
        .unwrap();
        assert!((result.sigma_total / (37.0_f64.sqrt() * scale) - 1.0).abs() < 2e-15);
        for entry in result.contributors {
            let expected = if entry.parameter == "P0" {
                15.0 / 37.0
            } else {
                22.0 / 37.0
            };
            assert!((entry.share - expected).abs() < 2e-15, "{scale}: {entry:?}");
        }
    }
    let matrix = correlations(Some(vec![vec![1.0; 3]; 3]));
    for values in [
        [1e100, -1e100, 1e-20],
        [1e-20, 1e100, -1e100],
        [-1e100, 1e-20, 1e100],
    ] {
        let result = assemble(
            &card(),
            "V(OUT)".into(),
            0.0,
            values
                .into_iter()
                .enumerate()
                .map(|(index, value)| entry(index, value))
                .collect(),
            &matrix,
            &NoAbort,
        )
        .unwrap();
        assert!(
            (result.sigma_total / 1e-20 - 1.0).abs() < 2e-15,
            "{values:?}: {result:?}"
        );
        assert!(result.contributors.iter().any(|entry| entry.share < -1e119));
        assert!(result.contributors.iter().any(|entry| entry.share > 1e119));
    }
}

#[test]
fn independent_instances_do_not_cancel_and_true_zero_variance_stays_zero() {
    let correlation = correlations(Some(vec![vec![1.0; 2]; 2]));
    let entries = vec![entry(0, 1e200), entry(1, -1e200)];
    let cancelled = assemble(
        &card(),
        "V(OUT)".into(),
        0.0,
        entries.clone(),
        &correlation,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(cancelled.sigma_total, 0.0);
    assert!(cancelled.sigma_total.is_sign_positive());
    assert!(
        cancelled
            .contributors
            .iter()
            .all(|entry| entry.share == 0.0)
    );
    let mut separate = entries;
    separate[1].instance = "B2".into();
    let result = assemble(
        &card(),
        "V(OUT)".into(),
        0.0,
        separate,
        &correlation,
        &NoAbort,
    )
    .unwrap();
    assert!((result.sigma_total / (std::f64::consts::SQRT_2 * 1e200) - 1.0).abs() < 2e-15);
    assert!(result.contributors.iter().all(|entry| entry.share == 0.5));
}

#[test]
fn derivatives_recover_overflowing_spans_and_unrepresentable_outputs_fail_explicitly() {
    assert_eq!(central_difference(1e308, -1e308, 1e308).unwrap(), 1.0);
    assert_eq!(central_difference(1e-300, -1e-300, 1e-300).unwrap(), 1.0);
    assert!(central_difference(1e300, -1e300, 1e-300).is_err());
    assert!(central_difference(1e-300, -1e-300, 1e300).is_err());
    assert!(central_difference(f64::NAN, 0.0, 1.0).is_err());
    for scale in [1e-200, 1e200] {
        assert!(
            contributor(
                DcMatchScope::Mismatch,
                "B1".into(),
                "P".into(),
                scale,
                scale
            )
            .is_err()
        );
    }
    let result = assemble(
        &card(),
        "V(OUT)".into(),
        0.0,
        vec![entry(0, f64::MAX), entry(1, f64::MAX)],
        &correlations(None),
        &NoAbort,
    );
    assert!(result.is_err());
    let mut quote = card();
    quote.sigma_multiplier = 2.0;
    assert!(
        assemble(
            &quote,
            "V(OUT)".into(),
            0.0,
            vec![entry(0, f64::MAX)],
            &correlations(None),
            &NoAbort
        )
        .is_err()
    );
    // The total is finite but the smaller contributor's true share is not.
    assert!(
        assemble(
            &card(),
            "V(OUT)".into(),
            0.0,
            vec![entry(0, 1e200), entry(1, 1e-200)],
            &correlations(None),
            &NoAbort
        )
        .is_err()
    );
}

#[test]
fn covariance_accumulation_observes_mid_pass_cancellation() {
    let entries = (0..100).map(|index| entry(index, 1.0)).collect();
    let abort = CountingAbort::new(20);
    let result = assemble(
        &card(),
        "V(OUT)".into(),
        0.0,
        entries,
        &correlations(None),
        &abort,
    );
    assert!(matches!(result, Err(SimulationError::Aborted)));
}
