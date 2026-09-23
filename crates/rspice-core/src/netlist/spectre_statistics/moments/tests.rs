use super::*;
use crate::abort_signal::NoAbort;

fn variation(name: &str, distribution: SpectreDistribution, spread: f64) -> SpectreVariation {
    SpectreVariation {
        line: 1,
        scope: SpectreVariationScope::Process,
        parameter: name.into(),
        distribution,
        spread: if distribution == SpectreDistribution::Uniform {
            SpectreSpread::HalfRange(spread.to_string())
        } else {
            SpectreSpread::StandardDeviation(spread.to_string())
        },
        percent: false,
        bounds: None,
    }
}
fn bounded(mut variation: SpectreVariation, lower: f64, upper: f64) -> SpectreVariation {
    variation.bounds = Some(SpectreVariationBounds {
        lower: Some(lower.to_string()),
        upper: Some(upper.to_string()),
        ..Default::default()
    });
    variation
}
fn correlation(left: &str, right: &str, value: f64) -> SpectreCorrelation {
    SpectreCorrelation {
        line: 1,
        scope: SpectreVariationScope::Process,
        parameters: vec![left.into(), right.into()],
        coefficient: value.to_string(),
    }
}
fn evaluate(plan: &SpectreStatisticsPlan, params: &ParamContext) -> SpectreScopeMoments {
    plan.scope_moments_with_abort(
        SpectreVariationScope::Process,
        params,
        &BTreeMap::new(),
        StatisticalMomentOptions::default(),
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap()
}
fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(0.01),
        "{actual} vs {expected}"
    );
}

#[test]
fn bounded_moments_condition_unbounded_correlated_partners() {
    let mut params = ParamContext::new();
    for name in ["A", "B", "C"] {
        params.set(name, 0.0);
    }
    let plan = SpectreStatisticsPlan {
        variations: vec![
            bounded(
                variation("A", SpectreDistribution::Gaussian, 1.0),
                -1.0,
                1.0,
            ),
            variation("B", SpectreDistribution::Gaussian, 2.0),
            variation("C", SpectreDistribution::Gaussian, 3.0),
        ],
        correlations: vec![
            correlation("A", "B", 0.6),
            correlation("A", "C", -0.3),
            correlation("B", "C", 0.2),
        ],
    };
    let result = evaluate(&plan, &params);
    // Cov(Y|a<X<b) = Cov(Y) + (Var(X|a<X<b)-1) Cov(Y,X)Cov(X,Y).
    let density = libm::exp(-0.5) / libm::sqrt(2.0 * std::f64::consts::PI);
    let variance = 1.0 - 2.0 * density / libm::erf(1.0 / std::f64::consts::SQRT_2);
    let target = [[1.0, 0.6, -0.3], [0.6, 1.0, 0.2], [-0.3, 0.2, 1.0]];
    let covariance = (0..3)
        .map(|a| {
            (0..3)
                .map(|b| target[a][b] + (variance - 1.0) * target[a][0] * target[b][0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for a in 0..3 {
        close(
            result.sigmas[a].standard_deviation,
            (a + 1) as f64 * covariance[a][a].sqrt(),
            2e-3,
        );
        for b in 0..3 {
            close(
                result.correlation.as_ref().unwrap().values()[a][b],
                covariance[a][b] / (covariance[a][a] * covariance[b][b]).sqrt(),
                4e-3,
            );
        }
    }
    assert!(result.evaluated_points > 0);
    assert!(result.relative_error_estimate <= 1e-3);
    let replay = evaluate(&plan, &params);
    assert_eq!(replay.sigmas, result.sigmas);
    assert_eq!(replay.correlation, result.correlation);
}

#[test]
fn bounded_moments_match_mixed_copula_density_integrals_and_singular_constraints() {
    let mut params = ParamContext::new();
    params.set("U", 2.0);
    params.set("L", 2.0);
    let rho = 0.4_f64;
    let sigma = 0.3_f64;
    let target = 3.0_f64.sqrt() * (2.0 * cdf(rho * sigma / 2.0_f64.sqrt()) - 1.0)
        / libm::expm1(sigma * sigma).sqrt();
    let plan = SpectreStatisticsPlan {
        variations: vec![
            bounded(
                variation("U", SpectreDistribution::Uniform, 1.0),
                1.0 + 2.0 * cdf(-0.5),
                1.0 + 2.0 * cdf(1.0),
            ),
            bounded(
                variation("L", SpectreDistribution::Lognormal, sigma),
                2.0 * libm::exp(-sigma),
                2.0 * libm::exp(0.7 * sigma),
            ),
        ],
        correlations: vec![correlation("U", "L", target)],
    };
    // Independent oracle: direct Simpson integration of the bivariate normal
    // density over a rectangle, without the production conditional transform.
    let mut raw = [0.0; 6];
    for i in 0..=120 {
        let x = -0.5 + 1.5 * i as f64 / 120.0;
        let wi = if i == 0 || i == 120 {
            1.0
        } else if i % 2 == 0 {
            2.0
        } else {
            4.0
        };
        for j in 0..=120 {
            let y = -1.0 + 1.7 * j as f64 / 120.0;
            let wj = if j == 0 || j == 120 {
                1.0
            } else if j % 2 == 0 {
                2.0
            } else {
                4.0
            };
            let weight = wi
                * wj
                * libm::exp(-(x * x - 2.0 * rho * x * y + y * y) / (2.0 * (1.0 - rho * rho)));
            let u = 1.0 + 2.0 * cdf(x);
            let l = 2.0 * libm::exp(sigma * y);
            for (sum, value) in raw.iter_mut().zip([1.0, u, l, u * u, l * l, u * l]) {
                *sum += weight * value;
            }
        }
    }
    let mean_u = raw[1] / raw[0];
    let mean_l = raw[2] / raw[0];
    let var_u = raw[3] / raw[0] - mean_u * mean_u;
    let var_l = raw[4] / raw[0] - mean_l * mean_l;
    let covariance = raw[5] / raw[0] - mean_u * mean_l;
    let result = evaluate(&plan, &params);
    close(result.sigmas[0].standard_deviation, var_l.sqrt(), 2e-3);
    close(result.sigmas[1].standard_deviation, var_u.sqrt(), 2e-3);
    close(
        result.correlation.as_ref().unwrap().values()[0][1],
        covariance / (var_l * var_u).sqrt(),
        4e-3,
    );

    params.set("A", 0.0);
    params.set("B", 0.0);
    let mut singular = SpectreStatisticsPlan {
        variations: vec![
            bounded(
                variation("A", SpectreDistribution::Gaussian, 1.0),
                -0.5,
                1.0,
            ),
            bounded(
                variation("B", SpectreDistribution::Gaussian, 1.0),
                -0.7,
                0.3,
            ),
        ],
        correlations: vec![correlation("A", "B", -1.0)],
    };
    let result = evaluate(&singular, &params);
    let marginal = evaluate(
        &SpectreStatisticsPlan {
            variations: vec![bounded(
                variation("A", SpectreDistribution::Gaussian, 1.0),
                -0.3,
                0.7,
            )],
            correlations: vec![],
        },
        &params,
    );
    close(
        result.sigmas[0].standard_deviation,
        marginal.sigmas[0].standard_deviation,
        1e-12,
    );
    assert_eq!(result.correlation.unwrap().values()[0][1], -1.0);
    singular.variations[1].bounds.as_mut().unwrap().lower = Some("2".into());
    singular.variations[1].bounds.as_mut().unwrap().upper = Some("3".into());
    assert!(
        singular
            .scope_moments_with_abort(
                SpectreVariationScope::Process,
                &params,
                &BTreeMap::new(),
                StatisticalMomentOptions::default(),
                ResourceLimits::default(),
                &NoAbort
            )
            .err()
            .unwrap()
            .to_string()
            .contains("no joint probability mass")
    );
}

#[test]
fn bounded_moments_handle_tails_limits_and_cancellation() {
    let mut params = ParamContext::new();
    params.set("A", 0.0);
    let plan = SpectreStatisticsPlan {
        variations: vec![bounded(
            variation("A", SpectreDistribution::Gaussian, 1.0),
            10.0,
            11.0,
        )],
        correlations: vec![],
    };
    // Tail oracle integrates exp(-(x²-100)/2) using offsets from10 to avoid
    // subtracting almost equal raw second moments near100.
    let mut raw = [0.0; 3];
    for i in 0..=4096 {
        let offset = i as f64 / 4096.0;
        let weight = (if i == 0 || i == 4096 {
            1.0
        } else if i % 2 == 0 {
            2.0
        } else {
            4.0
        }) * libm::exp(-10.0 * offset - 0.5 * offset * offset);
        for (sum, value) in raw.iter_mut().zip([1.0, offset, offset * offset]) {
            *sum += weight * value;
        }
    }
    let expected = (raw[2] / raw[0] - (raw[1] / raw[0]).powi(2)).sqrt();
    close(
        evaluate(&plan, &params).sigmas[0].standard_deviation,
        expected,
        2e-3,
    );
    let error = plan
        .scope_moments_with_abort(
            SpectreVariationScope::Process,
            &params,
            &BTreeMap::new(),
            StatisticalMomentOptions {
                relative_tolerance: 1e-12,
                max_points: 1024,
            },
            ResourceLimits::default(),
            &NoAbort,
        )
        .err()
        .unwrap();
    assert!(error.to_string().contains("did not converge"));
    struct Cancel(std::sync::atomic::AtomicUsize);
    impl AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 5
        }
    }
    assert!(matches!(
        plan.scope_moments_with_abort(
            SpectreVariationScope::Process,
            &params,
            &BTreeMap::new(),
            StatisticalMomentOptions::default(),
            ResourceLimits::default(),
            &Cancel(std::sync::atomic::AtomicUsize::new(0))
        ),
        Err(SpectreStatisticsError::Aborted)
    ));
    let limits = ResourceLimits {
        max_analysis_points: 512,
        ..Default::default()
    };
    assert!(
        plan.scope_moments_with_abort(
            SpectreVariationScope::Process,
            &params,
            &BTreeMap::new(),
            StatisticalMomentOptions::default(),
            limits,
            &NoAbort
        )
        .is_err()
    );
}
