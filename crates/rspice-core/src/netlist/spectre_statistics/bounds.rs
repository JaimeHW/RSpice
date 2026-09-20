//! Conditional statistical populations with stable keyed rejection streams.

use super::*;

/// Bounds apply to the parameter value after this scope's draw. Correlations
/// describe the unconditioned population; conditioning can change its moments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectreVariationBounds {
    pub lower: Option<String>,
    pub upper: Option<String>,
    /// Symmetric Gaussian standard deviations, in log space for lognormal.
    pub sigma_cutoff: Option<String>,
    /// Per-trial/per-instance rejection limit. Correlated groups use the
    /// smallest limit among their bounded members.
    pub max_attempts: u32,
}
impl Default for SpectreVariationBounds {
    fn default() -> Self {
        Self {
            lower: None,
            upper: None,
            sigma_cutoff: None,
            max_attempts: 10_000,
        }
    }
}
#[derive(Debug, Clone)]
pub(super) struct ResolvedBounds {
    lower: Value,
    upper: Value,
    pub max_attempts: u32,
}
impl ResolvedBounds {
    pub fn contains(&self, value: Value) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }
}
impl SpectreVariationBounds {
    pub(super) fn validate(&self, source: &SpectreVariation) -> Result<(), SpectreStatisticsError> {
        if [&self.lower, &self.upper, &self.sigma_cutoff]
            .into_iter()
            .flatten()
            .any(|expression| expression.trim().is_empty())
        {
            return Err(invalid(
                source.line,
                "Bound expressions must not be empty".into(),
            ));
        }
        if self.max_attempts == 0 || self.max_attempts > 1_000_000 {
            return Err(invalid(
                source.line,
                "Sampling attempt limit must be in 1..=1000000".into(),
            ));
        }
        if self.lower.is_none() && self.upper.is_none() && self.sigma_cutoff.is_none() {
            return Err(invalid(
                source.line,
                "A bounded variation requires a lower/upper bound or sigma cutoff".into(),
            ));
        }
        if self.sigma_cutoff.is_some() && source.distribution == SpectreDistribution::Uniform {
            return Err(invalid(
                source.line,
                "Uniform variations use lower/upper bounds, not Gaussian sigma truncation".into(),
            ));
        }
        Ok(())
    }
    pub(super) fn resolve(
        &self,
        source: &SpectreVariation,
        nominal: Value,
        spread: Value,
        params: &ParamContext,
    ) -> Result<ResolvedBounds, SpectreStatisticsError> {
        self.validate(source)?;
        if !spread.is_finite() {
            return Err(invalid(
                source.line,
                "Statistical spread must be finite".into(),
            ));
        }
        let mut lower = self
            .lower
            .as_ref()
            .map(|value| evaluate_finite(value, params))
            .transpose()?
            .unwrap_or(Value::NEG_INFINITY);
        let mut upper = self
            .upper
            .as_ref()
            .map(|value| evaluate_finite(value, params))
            .transpose()?
            .unwrap_or(Value::INFINITY);
        if let Some(cutoff) = &self.sigma_cutoff {
            let cutoff = evaluate_finite(cutoff, params)?;
            if cutoff <= 0.0 {
                return Err(invalid(source.line, "Sigma cutoff must be positive".into()));
            }
            let distance = cutoff * spread;
            let (lo, hi) = if distance == 0.0 {
                (nominal, nominal)
            } else {
                match source.distribution {
                    SpectreDistribution::Lognormal => (
                        libm::exp(libm::log(nominal) - distance),
                        libm::exp(libm::log(nominal) + distance),
                    ),
                    _ => (nominal - distance, nominal + distance),
                }
            };
            lower = lower.max(lo);
            upper = upper.min(hi);
        }
        match source.distribution {
            SpectreDistribution::Uniform => {
                lower = lower.max(nominal - spread);
                upper = upper.min(nominal + spread);
            }
            SpectreDistribution::Lognormal => lower = lower.max(0.0),
            _ => {}
        }
        if lower > upper
            || (spread > 0.0 && lower >= upper)
            || (spread == 0.0 && !(lower <= nominal && nominal <= upper))
        {
            return Err(invalid(
                source.line,
                format!(
                    "Variation '{}' bounds have no probability mass in the requested distribution",
                    source.parameter
                ),
            ));
        }
        Ok(ResolvedBounds {
            lower,
            upper,
            max_attempts: self.max_attempts,
        })
    }
}

pub(super) fn retry_stream(stream: u64, attempt: u32) -> u64 {
    // Attempt zero is precisely the historical unbounded sampler.
    if attempt == 0 {
        stream
    } else {
        hash_combine(
            hash_combine(stream, 0x424f_554e_4453_0000),
            u64::from(attempt),
        )
    }
}
pub(super) fn exhausted(source: &SpectreVariation, attempts: u32) -> SpectreStatisticsError {
    invalid(
        source.line,
        format!(
            "Bounded statistical group containing '{}' exhausted {attempts} sampling attempts; widen the bounds, check correlated bounds for compatibility, or increase the attempt limit",
            source.parameter
        ),
    )
}
pub(super) fn bounded_groups(target: &[Vec<Value>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; target.len()];
    let mut groups = Vec::new();
    for first in 0..target.len() {
        if visited[first] {
            continue;
        }
        let mut group = vec![first];
        visited[first] = true;
        let mut cursor = 0;
        while cursor < group.len() {
            let index = group[cursor];
            for (next, seen) in visited.iter_mut().enumerate() {
                if !*seen && target[index][next] != 0.0 {
                    *seen = true;
                    group.push(next);
                }
            }
            cursor += 1;
        }
        group.sort_unstable();
        groups.push(group);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    fn row(name: &str, distribution: SpectreDistribution) -> SpectreVariation {
        SpectreVariation {
            line: 1,
            scope: SpectreVariationScope::Process,
            parameter: name.into(),
            distribution,
            spread: if distribution == SpectreDistribution::Uniform {
                SpectreSpread::HalfRange("1".into())
            } else {
                SpectreSpread::StandardDeviation("1".into())
            },
            percent: false,
            bounds: None,
        }
    }
    #[test]
    fn bounded_populations_preserve_joint_draws_replay_and_conditional_moments() {
        let mut params = ParamContext::new();
        params.set("X", 0.0);
        params.set("Y", 0.0);
        params.set("Z", 0.0);
        let mut gaussian = row("X", SpectreDistribution::Gaussian);
        gaussian.bounds = Some(SpectreVariationBounds {
            sigma_cutoff: Some("1".into()),
            ..Default::default()
        });
        let mut plan = SpectreStatisticsPlan {
            variations: vec![gaussian.clone()],
            correlations: vec![],
        };
        let decoded = SpectreStatisticsPlan::decode_internal(&plan.encode_internal()).unwrap();
        assert_eq!(decoded, plan);
        assert!(plan.encode_internal().starts_with("S2~"));
        let mut values = Vec::new();
        for index in 0..4096 {
            let coordinate = SpectreStatisticalCoordinate {
                seed: u64::MAX,
                monte_carlo_run: index,
                ..Default::default()
            };
            let value = plan.sample_process(&params, &coordinate).unwrap()["X"];
            assert!(value > -1.0 && value < 1.0, "no clipping at the bounds");
            if index == 37 {
                assert_eq!(
                    value,
                    decoded.sample_process(&params, &coordinate).unwrap()["X"]
                );
            }
            values.push(value);
        }
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;
        // Analytic variance of N(0,1) conditioned on -1 < X < 1.
        let density = libm::exp(-0.5) / libm::sqrt(2.0 * std::f64::consts::PI);
        let expected = 1.0 - 2.0 * density / libm::erf(1.0 / libm::sqrt(2.0));
        assert!(
            mean.abs() < 0.025 && (variance - expected).abs() < 0.02,
            "{mean} {variance} {expected}"
        );
        let mut partner = gaussian.clone();
        partner.parameter = "Y".into();
        partner.bounds = Some(SpectreVariationBounds {
            lower: Some("0.2".into()),
            upper: Some("0.8".into()),
            ..Default::default()
        });
        plan.variations
            .extend([partner, row("Z", SpectreDistribution::Gaussian)]);
        plan.correlations.push(SpectreCorrelation {
            line: 1,
            scope: SpectreVariationScope::Process,
            parameters: vec!["X".into(), "Y".into()],
            coefficient: "1".into(),
        });
        let mut unbounded = plan.clone();
        for variation in &mut unbounded.variations {
            variation.bounds = None;
        }
        assert!(unbounded.encode_internal().starts_with("S1~"));
        for index in 0..32 {
            let coordinate = SpectreStatisticalCoordinate {
                seed: 123,
                monte_carlo_run: index,
                ..Default::default()
            };
            let draw = plan.sample_process(&params, &coordinate).unwrap();
            assert_eq!(draw["X"], draw["Y"]);
            assert!((0.2..=0.8).contains(&draw["X"]));
            assert_eq!(
                draw["Z"],
                unbounded.sample_process(&params, &coordinate).unwrap()["Z"]
            );
        }
        // A singular correlated population cannot satisfy incompatible bounds.
        plan.variations[0].bounds = Some(SpectreVariationBounds {
            lower: Some("-1".into()),
            upper: Some("-0.2".into()),
            max_attempts: 8,
            ..Default::default()
        });
        assert!(
            plan.sample_process(&params, &Default::default())
                .unwrap_err()
                .to_string()
                .contains("exhausted 8")
        );
        assert!(
            plan.scope_standard_deviations(
                SpectreVariationScope::Process,
                &params,
                &BTreeMap::new()
            )
            .is_err()
        );
    }
    #[test]
    fn bounds_cover_uniform_lognormal_mismatch_and_invalid_populations() {
        let mut params = ParamContext::new();
        params.set("X", 2.0);
        for distribution in [SpectreDistribution::Uniform, SpectreDistribution::Lognormal] {
            let mut variation = row("X", distribution);
            variation.scope = SpectreVariationScope::Mismatch;
            variation.bounds = Some(SpectreVariationBounds {
                lower: Some("1.9".into()),
                upper: Some("2.1".into()),
                sigma_cutoff: (distribution == SpectreDistribution::Lognormal)
                    .then(|| "0.1".into()),
                ..Default::default()
            });
            let mut plan = SpectreStatisticsPlan {
                variations: vec![variation],
                correlations: vec![],
            };
            for index in 0..32 {
                let coordinate = SpectreStatisticalCoordinate {
                    seed: 42,
                    monte_carlo_run: index,
                    ..Default::default()
                };
                let draw = plan
                    .sample_mismatch(&params, &BTreeMap::new(), "R1", &coordinate)
                    .unwrap()["X"];
                assert!((1.9..=2.1).contains(&draw));
                assert_eq!(
                    draw,
                    plan.sample_mismatch(&params, &BTreeMap::new(), "r1", &coordinate)
                        .unwrap()["X"]
                );
                assert_ne!(
                    draw,
                    plan.sample_mismatch(&params, &BTreeMap::new(), "R2", &coordinate)
                        .unwrap()["X"]
                );
            }
            plan.variations[0].spread = if distribution == SpectreDistribution::Uniform {
                SpectreSpread::HalfRange("0".into())
            } else {
                SpectreSpread::StandardDeviation("0".into())
            };
            assert_eq!(
                plan.sample_mismatch(&params, &BTreeMap::new(), "R1", &Default::default())
                    .unwrap()["X"],
                2.0
            );
            plan.variations[0].bounds.as_mut().unwrap().upper = Some("1".into());
            assert!(
                plan.sample_mismatch(&params, &BTreeMap::new(), "R1", &Default::default())
                    .is_err()
            );
        }
        let mut uniform = row("X", SpectreDistribution::Uniform);
        uniform.bounds = Some(SpectreVariationBounds {
            lower: Some("4".into()),
            ..Default::default()
        });
        let mut plan = SpectreStatisticsPlan {
            variations: vec![uniform],
            correlations: vec![],
        };
        assert!(
            plan.sample_process(&params, &Default::default())
                .unwrap_err()
                .to_string()
                .contains("no probability mass")
        );
        plan.variations[0].bounds.as_mut().unwrap().lower = Some("NaN".into());
        assert!(plan.sample_process(&params, &Default::default()).is_err());
        plan.variations[0].bounds.as_mut().unwrap().lower = Some("".into());
        assert!(
            plan.to_parser_directive()
                .unwrap_err()
                .to_string()
                .contains("empty")
        );
        plan.variations[0].bounds.as_mut().unwrap().lower = Some("1".into());
        plan.variations[0].bounds.as_mut().unwrap().max_attempts = 0;
        assert!(plan.to_parser_directive().is_err());
    }
}
