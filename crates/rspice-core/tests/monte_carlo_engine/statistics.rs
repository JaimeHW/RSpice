use rspice_core::analysis::monte_carlo::{
    MeanConfidenceInterval, MeanConfidenceMethod, VariableStatistics,
};
use rspice_core::analysis::{MonteCarloConfig, MonteCarloRunner};

fn student_interval(samples: [f64; 2], confidence_pct: f64) -> MeanConfidenceInterval {
    let mut config = MonteCarloConfig::new(2).with_seed(1);
    config.histogram_bins = 1;
    config.confidence_pct = confidence_pct;
    let mut samples = samples.into_iter();
    MonteCarloRunner::new(config)
        .run::<_, ()>(|_| {
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                samples.next().unwrap(),
            )]))
        })
        .unwrap()
        .variables["out"]
        .mean_confidence
        .unwrap()
}

#[test]
fn student_limits_survive_overflowing_sample_deviation() {
    let interval = student_interval([-f64::MAX, f64::MAX], 20.0);
    let MeanConfidenceInterval::Available { lower, upper } = interval else {
        panic!("representable interval was lost: {interval:?}");
    };
    // For two symmetric samples, SE is MAX, even though s = sqrt(2) * MAX
    // exceeds binary64. The df=1 critical value is tan(pi * 0.2 / 2).
    let width = f64::MAX * (std::f64::consts::PI * 0.1).tan();
    assert!((lower / -width - 1.0).abs() < 2e-15);
    assert!((upper / width - 1.0).abs() < 2e-15);
}

#[test]
fn student_limits_preserve_subnormal_centers_and_widths() {
    let tiny = f64::from_bits(1);
    // The exact center is half a subnormal ulp; SE is also half an ulp.
    // t(1, .975) = 12.706204736..., giving endpoints -5.853... and
    // 6.853... ulps, which round to -6 and 7 only at publication.
    assert_eq!(
        student_interval([0.0, tiny], 95.0),
        MeanConfidenceInterval::Available {
            lower: -f64::from_bits(6),
            upper: f64::from_bits(7),
        }
    );
}

#[test]
fn student_limits_retain_center_offsets_between_adjacent_samples() {
    for value in [1.0_f64, 1e16, 1e300] {
        let adjacent = f64::from_bits(value.to_bits() + 1);
        for samples in [[value, adjacent], [-adjacent, -value]] {
            // The true mean is midway between adjacent values. A central
            // 20% interval straddles that midpoint and rounds to both values.
            assert_eq!(
                student_interval(samples, 20.0),
                MeanConfidenceInterval::Available {
                    lower: samples[0],
                    upper: samples[1],
                }
            );
        }
    }
}

#[test]
fn student_cancellation_stops_during_moments_without_publishing() {
    let mut config = MonteCarloConfig::new(128).with_seed(1);
    config.histogram_bins = 1;
    let mut samples = [1e308, 1e-308, -1e308].into_iter().cycle();
    let mut result = MonteCarloRunner::new(config)
        .run::<_, ()>(|_| {
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                samples.next().unwrap(),
            )]))
        })
        .unwrap();
    let census = rspice_core::abort_signal::CountingAbort::new(usize::MAX);
    let limits = rspice_core::resource::ResourceLimits::default();
    result
        .compute_mean_confidence(20.0, MeanConfidenceMethod::StudentT, limits, &census)
        .unwrap();
    let previous_method = result.confidence;
    let previous_interval = result.variables["out"].mean_confidence;
    for threshold in 0..census.count() {
        let abort = rspice_core::abort_signal::CountingAbort::new(threshold);
        let error = result
            .compute_mean_confidence(20.0, MeanConfidenceMethod::StudentT, limits, &abort)
            .unwrap_err();
        assert!(matches!(error, rspice_core::SimulationError::Aborted));
        assert_eq!(abort.polls_after_abort(), 0, "threshold {threshold}");
        assert_eq!(result.confidence, previous_method);
        assert_eq!(result.variables["out"].mean_confidence, previous_interval);
    }
}

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

#[test]
fn bootstrap_preserves_small_means_after_extreme_cancellation() {
    for residual in [-1e-308, 1e-308] {
        let mut config = MonteCarloConfig::new(3).with_seed(1);
        config.histogram_bins = 1;
        config.confidence_pct = 1.0;
        config.confidence_method = MeanConfidenceMethod::PercentileBootstrap {
            resamples: 1024,
            seed: 42,
        };
        let mut samples = [1e308, residual, -1e308].into_iter();
        let result = MonteCarloRunner::new(config)
            .run::<_, ()>(|_| {
                Ok(std::collections::HashMap::from([(
                    "out".to_owned(),
                    samples.next().unwrap(),
                )]))
            })
            .unwrap();
        // Both central ranks (506.385 and 516.615) fall inside the 247
        // resamples containing one of each value: the mean is residual / 3.
        assert_eq!(
            result.variables["out"].mean_confidence,
            Some(MeanConfidenceInterval::Available {
                lower: residual / 3.0,
                upper: residual / 3.0,
            })
        );
    }
}

#[test]
fn bootstrap_sample_buffer_is_budgeted_before_simulation() {
    let mut config = MonteCarloConfig::new(64).with_seed(1);
    config.histogram_bins = 1;
    config.resource_limits.max_result_values = 80;
    config.confidence_method = MeanConfidenceMethod::PercentileBootstrap {
        resamples: 32,
        seed: 42,
    };
    let error = MonteCarloRunner::new(config)
        .run::<_, ()>(|_| panic!("the bootstrap scratch budget must be checked before simulation"))
        .unwrap_err();
    let rspice_core::SimulationError::ResourceLimit(error) = error else {
        panic!("expected a resource error, got {error}");
    };
    assert_eq!(
        error.resource,
        rspice_core::resource::ResourceKind::ResultValues
    );
    assert_eq!(error.requested, 96);
}

#[test]
fn bootstrap_budget_includes_live_results_and_preserves_previous_estimates() {
    let mut config = MonteCarloConfig::new(3).with_seed(1);
    config.histogram_bins = 1;
    let mut samples = [0.0, 1.0, 2.0].into_iter();
    let mut result = MonteCarloRunner::new(config)
        .run::<_, ()>(|_| {
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                samples.next().unwrap(),
            )]))
        })
        .unwrap();
    let previous_method = result.confidence;
    let previous_interval = result.variables["out"].mean_confidence;
    let method = MeanConfidenceMethod::PercentileBootstrap {
        resamples: 16,
        seed: 42,
    };
    let mut limits = rspice_core::resource::ResourceLimits::default();
    // 16 retained values + 16 resample means + one reusable three-sample buffer.
    limits.max_result_values = 34;
    let error = result
        .compute_mean_confidence(95.0, method, limits, &rspice_core::NoAbort)
        .unwrap_err();
    let rspice_core::SimulationError::ResourceLimit(error) = error else {
        panic!("expected a resource error, got {error}");
    };
    assert_eq!(error.requested, 35);
    assert_eq!(result.confidence, previous_method);
    assert_eq!(result.variables["out"].mean_confidence, previous_interval);
    limits.max_result_values = 35;
    result
        .compute_mean_confidence(95.0, method, limits, &rspice_core::NoAbort)
        .unwrap();
    assert_eq!(result.confidence.unwrap().method, method);
    assert!(matches!(
        result.variables["out"].mean_confidence,
        Some(MeanConfidenceInterval::Available { .. })
    ));
}

#[test]
fn bootstrap_cancellation_stops_during_exact_summation_without_publishing() {
    let mut config = MonteCarloConfig::new(128).with_seed(1);
    config.histogram_bins = 1;
    let mut samples = [1e308, 1e-308, -1e308].into_iter().cycle();
    let mut result = MonteCarloRunner::new(config)
        .run::<_, ()>(|_| {
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                samples.next().unwrap(),
            )]))
        })
        .unwrap();
    let previous_method = result.confidence;
    let previous_interval = result.variables["out"].mean_confidence;
    // Exercise admission, sampling, and both exact-summation passes. Every
    // cancellation must return immediately and leave the old report intact.
    for threshold in 0..16 {
        let abort = rspice_core::abort_signal::CountingAbort::new(threshold);
        let error = result
            .compute_mean_confidence(
                95.0,
                MeanConfidenceMethod::PercentileBootstrap {
                    resamples: 4,
                    seed: 42,
                },
                rspice_core::resource::ResourceLimits::default(),
                &abort,
            )
            .unwrap_err();
        assert!(matches!(error, rspice_core::SimulationError::Aborted));
        assert_eq!(abort.polls_after_abort(), 0, "threshold {threshold}");
        assert_eq!(result.confidence, previous_method);
        assert_eq!(result.variables["out"].mean_confidence, previous_interval);
    }
}
