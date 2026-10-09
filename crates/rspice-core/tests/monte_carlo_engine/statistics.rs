use rspice_core::analysis::monte_carlo::VariableStatistics;

#[test]
fn means_preserve_small_remainders_after_extreme_cancellation() {
    for large in [1e308, f64::MAX, 1e16] {
        for residual in [
            1e-308,
            f64::MIN_POSITIVE,
            1.0,
            f64::from_bits(3),
            f64::from_bits(1),
        ] {
            for sign in [-1.0, 1.0] {
                let residual = sign * residual;
                for samples in [
                    [large, residual, -large],
                    [large, -large, residual],
                    [residual, large, -large],
                    [residual, -large, large],
                    [-large, large, residual],
                    [-large, residual, large],
                ] {
                    let statistics = VariableStatistics::from_samples("out", samples.to_vec(), 4);
                    // The two large terms cancel exactly; only the final division rounds.
                    assert_eq!(
                        statistics.mean.to_bits(),
                        (residual / 3.0).to_bits(),
                        "{samples:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn exact_means_keep_large_constants_and_small_spreads_representable() {
    for sign in [-1.0, 1.0] {
        let constant = VariableStatistics::from_samples("constant", vec![sign * f64::MAX; 4], 4);
        assert_eq!(constant.mean, sign * f64::MAX);
        assert_eq!(constant.std_dev, 0.0);
        let close =
            VariableStatistics::from_samples("close", vec![sign * 1e16, sign * (1e16 + 2.0)], 2);
        assert_eq!(close.mean, sign * 1e16);
        assert!((close.std_dev - 2.0_f64.sqrt()).abs() < 1e-15);
    }
}
