//! Excursion-duration qualification on linearly interpolated accepted samples.
use super::{SoARuleVerdict, SoaThresholds};
use rspice_core::{SimulationError, abort_signal::AbortSignal};
use serde::{Deserialize, Serialize};

/// Retrospective duration screening; recovery applies only between excursions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SoaDurationMode {
    #[default]
    PerExcursion,
    Cumulative {
        recovery_time_s: Option<f64>,
    },
}
impl SoaDurationMode {
    pub fn is_default(&self) -> bool {
        *self == Self::PerExcursion
    }

    pub fn validate(self, minimum_duration_s: Option<f64>) -> Result<(), String> {
        if minimum_duration_s.is_some_and(|value| !value.is_finite() || value <= 0.0) {
            return Err("SOA duration threshold must be finite and positive".into());
        }
        if let Self::Cumulative { recovery_time_s } = self {
            if minimum_duration_s.is_none() {
                return Err("Cumulative SOA exposure requires a duration threshold".into());
            }
            if recovery_time_s.is_some_and(|value| !value.is_finite() || value <= 0.0) {
                return Err("SOA recovery time must be finite and positive".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaCumulativeDurationEvidence {
    pub recovery_time_s: Option<f64>,
    pub peak_exposure_s: f64,
    pub final_exposure_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaDurationEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cumulative: Option<SoaCumulativeDurationEvidence>,
    pub minimum_duration_s: f64,
    pub total_exceedance_s: f64,
    pub longest_excursion_s: f64,
    pub qualified_excursions: u64,
    pub rejected_excursions: u64,
    pub clipped_excursions: u64,
}
impl SoaDurationEvidence {
    pub fn mode(self) -> SoaDurationMode {
        self.cumulative
            .map_or(SoaDurationMode::PerExcursion, |value| {
                SoaDurationMode::Cumulative {
                    recovery_time_s: value.recovery_time_s,
                }
            })
    }

    pub fn validate(self) -> Result<(), String> {
        self.mode().validate(Some(self.minimum_duration_s))?;
        if let Some(cumulative) = self.cumulative
            && (!cumulative.peak_exposure_s.is_finite()
                || cumulative.peak_exposure_s < 0.0
                || cumulative.peak_exposure_s > self.total_exceedance_s
                || !cumulative.final_exposure_s.is_finite()
                || cumulative.final_exposure_s < 0.0
                || cumulative.final_exposure_s > cumulative.peak_exposure_s)
        {
            return Err("SOA cumulative duration evidence is invalid".into());
        }
        if !self.minimum_duration_s.is_finite()
            || self.minimum_duration_s <= 0.0
            || !self.total_exceedance_s.is_finite()
            || self.total_exceedance_s < 0.0
            || !self.longest_excursion_s.is_finite()
            || self.longest_excursion_s < 0.0
            || self.longest_excursion_s > self.total_exceedance_s
            || self
                .qualified_excursions
                .checked_add(self.rejected_excursions)
                .is_none_or(|count| self.clipped_excursions > count)
        {
            return Err("SOA duration evidence is invalid".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum SoaLimitTrace<'a> {
    Constant(f64),
    Samples(&'a [f64]),
}
impl SoaLimitTrace<'_> {
    pub fn at(self, index: usize) -> f64 {
        match self {
            Self::Constant(value) => value,
            Self::Samples(values) => values[index],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoaExcursion {
    pub start_s: f64,
    pub end_s: f64,
    pub first_sample: usize,
    pub last_sample: usize,
    pub qualified: bool,
}

pub struct SoaDurationScan {
    pub evidence: SoaDurationEvidence,
    pub qualified_samples: Vec<bool>,
    pub excursions: Vec<SoaExcursion>,
}

/// Qualifications are retrospective: the complete excursion's width decides
/// whether its above-limit samples are violations. Observation-window edges
/// clip the width; no duration outside that window is inferred.
#[cfg(test)]
fn qualify_soa_duration(
    time: &[f64],
    stress: &[f64],
    limits: SoaLimitTrace<'_>,
    minimum_duration_s: f64,
    abort: &dyn AbortSignal,
) -> Result<SoaDurationScan, SimulationError> {
    qualify_soa_duration_with_mode(
        time,
        stress,
        limits,
        minimum_duration_s,
        SoaDurationMode::PerExcursion,
        abort,
    )
}

/// Accumulated exposure starts at zero at the observation-window boundary.
/// It increases with above-limit time, optionally decays exponentially in gaps,
/// and qualifies each whole excursion using its exposure at that excursion's end.
pub fn qualify_soa_duration_with_mode(
    time: &[f64],
    stress: &[f64],
    limits: SoaLimitTrace<'_>,
    minimum_duration_s: f64,
    mode: SoaDurationMode,
    abort: &dyn AbortSignal,
) -> Result<SoaDurationScan, SimulationError> {
    let invalid = |message: &str| SimulationError::Circuit(message.into());
    if !minimum_duration_s.is_finite() || minimum_duration_s <= 0.0 {
        return Err(rspice_core::config::SimulationConfigError::InvalidValue {
            field: "soa.minimum_duration_s",
            value: minimum_duration_s,
            requirement: "finite and positive",
        }
        .into());
    }
    mode.validate(Some(minimum_duration_s))
        .map_err(SimulationError::Circuit)?;
    if time.is_empty()
        || time.len() != stress.len()
        || matches!(limits, SoaLimitTrace::Samples(values) if values.len() != time.len())
    {
        return Err(invalid(
            "SOA duration traces have incomplete sample coverage",
        ));
    }
    for i in 0..time.len() {
        if i % 256 == 0 && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !time[i].is_finite()
            || time[i] < 0.0
            || (i > 0 && time[i] <= time[i - 1])
            || !stress[i].is_finite()
            || stress[i] < 0.0
            || !limits.at(i).is_finite()
            || limits.at(i) < 0.0
        {
            return Err(invalid(
                "SOA duration requires increasing finite times and nonnegative finite stress/limits",
            ));
        }
    }
    // Stable root fraction for opposite-signed margins, even near f64::MAX.
    let crossing = |i: usize| {
        let a = (stress[i - 1] - limits.at(i - 1)).abs();
        let b = (stress[i] - limits.at(i)).abs();
        let scale = a.max(b);
        let fraction = (a / scale) / (a / scale + b / scale);
        time[i - 1] + (time[i] - time[i - 1]) * fraction
    };
    let mut excursions = Vec::new();
    let mut open = (stress[0] > limits.at(0)).then_some((time[0], 0usize));
    for i in 1..time.len() {
        if i % 256 == 0 && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let before = stress[i - 1] > limits.at(i - 1);
        let after = stress[i] > limits.at(i);
        if !before && after {
            open = Some((crossing(i), i));
        }
        if before && !after {
            let (start_s, first_sample) = open.take().expect("open positive excursion");
            let end_s = crossing(i);
            excursions.push(SoaExcursion {
                start_s,
                end_s,
                first_sample,
                last_sample: i - 1,
                qualified: end_s - start_s >= minimum_duration_s,
            });
        }
    }
    if let Some((start_s, first_sample)) = open {
        let end_s = time[time.len() - 1];
        excursions.push(SoaExcursion {
            start_s,
            end_s,
            first_sample,
            last_sample: time.len() - 1,
            qualified: end_s - start_s >= minimum_duration_s,
        });
    }
    let mut evidence = SoaDurationEvidence {
        cumulative: match mode {
            SoaDurationMode::PerExcursion => None,
            SoaDurationMode::Cumulative { recovery_time_s } => {
                Some(SoaCumulativeDurationEvidence {
                    recovery_time_s,
                    peak_exposure_s: 0.0,
                    final_exposure_s: 0.0,
                })
            }
        },
        minimum_duration_s,
        total_exceedance_s: 0.0,
        longest_excursion_s: 0.0,
        qualified_excursions: 0,
        rejected_excursions: 0,
        clipped_excursions: 0,
    };
    let mut qualified_samples = vec![false; time.len()];
    let mut correction = 0.0;
    let mut previous_end = time[0];
    for (index, excursion) in excursions.iter_mut().enumerate() {
        if index % 256 == 0 && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let duration = excursion.end_s - excursion.start_s;
        let increment = duration - correction;
        let total = evidence.total_exceedance_s + increment;
        correction = (total - evidence.total_exceedance_s) - increment;
        evidence.total_exceedance_s = total;
        evidence.longest_excursion_s = evidence.longest_excursion_s.max(duration);
        if let Some(cumulative) = &mut evidence.cumulative {
            cumulative.final_exposure_s = if let Some(tau) = cumulative.recovery_time_s {
                let gap = excursion.start_s - previous_end;
                // Rounding cannot let recovered exposure exceed total exposure.
                (cumulative.final_exposure_s * (-gap / tau).exp() + duration).min(total)
            } else {
                total
            };
            cumulative.peak_exposure_s =
                cumulative.peak_exposure_s.max(cumulative.final_exposure_s);
            excursion.qualified = cumulative.final_exposure_s >= minimum_duration_s;
        }
        previous_end = excursion.end_s;
        if excursion.qualified {
            evidence.qualified_excursions += 1;
        } else {
            evidence.rejected_excursions += 1;
        }
        if excursion.first_sample == 0 || excursion.last_sample == time.len() - 1 {
            evidence.clipped_excursions += 1;
        }
        if excursion.qualified {
            for chunk in
                qualified_samples[excursion.first_sample..=excursion.last_sample].chunks_mut(256)
            {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                chunk.fill(true);
            }
        }
    }
    if let Some(cumulative) = &mut evidence.cumulative
        && let Some(tau) = cumulative.recovery_time_s
    {
        cumulative.final_exposure_s *= (-(time[time.len() - 1] - previous_end) / tau).exp();
    }
    evidence.validate().map_err(SimulationError::Circuit)?;
    Ok(SoaDurationScan {
        evidence,
        qualified_samples,
        excursions,
    })
}

pub fn soa_duration_verdict(
    thresholds: SoaThresholds,
    actual: f64,
    limit: f64,
    qualified: bool,
) -> SoARuleVerdict {
    let verdict = thresholds.verdict(actual, limit);
    if !qualified
        && matches!(
            verdict,
            SoARuleVerdict::Violation | SoARuleVerdict::Critical
        )
    {
        if thresholds.warning_fraction.is_some() {
            SoARuleVerdict::Warning
        } else {
            SoARuleVerdict::Pass
        }
    } else {
        verdict
    }
}

#[cfg(test)]
#[test]
fn soa_duration_cumulative_exposure_recovers_only_between_excursions() {
    use rspice_core::abort_signal::NoAbort;
    // Excursions [0.25, 1.75] and [2.5, 5.5]: widths 1.5 and 3, gap 0.75.
    let time = [0., 1., 2., 3., 4., 5., 6.];
    let stress = [0., 4., 0., 2., 2., 2., 0.];
    for (recovery_time_s, qualified) in [(None, 1), (Some(0.1), 0), (Some(3.), 1)] {
        let scan = qualify_soa_duration_with_mode(
            &time,
            &stress,
            SoaLimitTrace::Constant(1.),
            4.,
            SoaDurationMode::Cumulative { recovery_time_s },
            &NoAbort,
        )
        .unwrap();
        let evidence = scan.evidence.cumulative.unwrap();
        let peak = recovery_time_s.map_or(4.5, |tau| 3. + 1.5 * (-0.75 / tau).exp());
        let final_exposure = recovery_time_s.map_or(peak, |tau| peak * (-0.5 / tau).exp());
        assert!((evidence.peak_exposure_s - peak).abs() < 1e-14);
        assert!((evidence.final_exposure_s - final_exposure).abs() < 1e-14);
        assert_eq!(scan.evidence.qualified_excursions, qualified);
        assert_eq!(scan.evidence.rejected_excursions, 2 - qualified);
        assert!(
            !scan.qualified_samples[1],
            "later exposure must not reclassify the first pulse"
        );
        assert_eq!(scan.qualified_samples[4], qualified == 1);
    }
    // Starting inside an excursion cannot infer earlier stress, and a long gap
    // lets a later excursion fall below threshold even after a qualified one.
    let clipped = qualify_soa_duration_with_mode(
        &[5., 6., 7., 8., 20., 21.],
        &[2., 2., 0., 0., 0., 2.],
        SoaLimitTrace::Constant(1.),
        1.,
        SoaDurationMode::Cumulative {
            recovery_time_s: Some(1.),
        },
        &NoAbort,
    )
    .unwrap();
    assert_eq!(clipped.evidence.clipped_excursions, 2);
    assert!(clipped.excursions[0].qualified);
    assert!(!clipped.excursions[1].qualified);
    assert!(
        (clipped.evidence.cumulative.unwrap().final_exposure_s - (0.5 + 1.5 * (-14_f64).exp()))
            .abs()
            < 1e-14
    );
    let no_stress = qualify_soa_duration_with_mode(
        &[0., 1.],
        &[0., 0.],
        SoaLimitTrace::Constant(1.),
        1.,
        SoaDurationMode::Cumulative {
            recovery_time_s: None,
        },
        &NoAbort,
    )
    .unwrap();
    assert_eq!(no_stress.evidence.cumulative.unwrap().peak_exposure_s, 0.);
    for recovery_time_s in [Some(0.), Some(-1.), Some(f64::INFINITY), Some(f64::NAN)] {
        assert!(
            SoaDurationMode::Cumulative { recovery_time_s }
                .validate(Some(1.))
                .is_err()
        );
    }
    assert!(
        SoaDurationMode::Cumulative {
            recovery_time_s: None
        }
        .validate(None)
        .is_err()
    );
    struct Abort;
    impl AbortSignal for Abort {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        qualify_soa_duration_with_mode(
            &time,
            &stress,
            SoaLimitTrace::Constant(1.),
            4.,
            SoaDurationMode::Cumulative {
                recovery_time_s: None
            },
            &Abort,
        ),
        Err(SimulationError::Aborted)
    ));
}

#[cfg(test)]
#[test]
fn soa_duration_qualifies_interpolated_excursions_and_reclassifies_worst_point() {
    use super::{SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter};
    use rspice_core::abort_signal::NoAbort;
    let time = [0., 1., 2., 3., 4., 5., 6.];
    let stress = [0., 4., 0., 2., 2., 2., 0.];
    let scan =
        qualify_soa_duration(&time, &stress, SoaLimitTrace::Constant(1.), 2., &NoAbort).unwrap();
    assert_eq!(scan.evidence.total_exceedance_s, 4.5);
    assert_eq!(scan.evidence.longest_excursion_s, 3.0);
    assert_eq!(scan.evidence.qualified_excursions, 1);
    assert_eq!(scan.evidence.rejected_excursions, 1);
    assert_eq!(scan.evidence.clipped_excursions, 0);
    assert_eq!(
        scan.qualified_samples,
        vec![false, false, false, true, true, true, false]
    );
    assert_eq!(
        (scan.excursions[0].start_s, scan.excursions[0].end_s),
        (0.25, 1.75)
    );
    assert_eq!(
        (scan.excursions[1].start_s, scan.excursions[1].end_s),
        (2.5, 5.5)
    );
    for warning in [Some(0.9), None] {
        let mut manager = SoAManager::with_thresholds(SoaThresholds {
            warning_fraction: warning,
            ..Default::default()
        })
        .unwrap();
        manager
            .register_device(
                "M1",
                SoADefinition {
                    limits: vec![SoALimit {
                        duration_mode: Default::default(),
                        minimum_duration_s: Some(2.),
                        current_envelope: None,
                        power_derating: None,
                        voltage_basis: Default::default(),
                        parameter: SoAParameter::Vds,
                        max_value: 1.,
                        unit: "V".into(),
                        description: "Drain voltage".into(),
                    }],
                },
            )
            .unwrap();
        for (t, actual) in time.into_iter().zip(stress) {
            manager
                .check_point(
                    t,
                    &std::collections::HashMap::from([(
                        "M1".into(),
                        std::collections::HashMap::from([(SoAParameter::Vds, actual)]),
                    )]),
                )
                .unwrap();
        }
        assert_eq!(manager.evaluations().next().unwrap().worst_actual_value, 4.);
        manager.finalize_durations(&time, &NoAbort).unwrap();
        let evaluation = manager.evaluations().next().unwrap();
        assert_eq!(evaluation.worst_actual_value, 2.);
        assert_eq!(evaluation.worst_time, 3.);
        assert_eq!(evaluation.verdict, SoARuleVerdict::Critical);
        assert_eq!(evaluation.duration, Some(scan.evidence));
        assert_eq!(
            manager.violations().len(),
            if warning.is_some() { 4 } else { 3 }
        );
        let wire = crate::simulation::runner::worker_contract::WorkerSoAEvaluation::from(
            evaluation.clone(),
        );
        let wire: crate::simulation::runner::worker_contract::WorkerSoAEvaluation =
            serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
        assert_eq!(SoAEvaluation::from(wire), *evaluation);
    }
    let varying = qualify_soa_duration(
        &[0., 1., 2., 3.],
        &[0.5; 4],
        SoaLimitTrace::Samples(&[1., 0., 0., 1.]),
        2.,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(varying.evidence.longest_excursion_s, 2.);
    assert_eq!(varying.qualified_samples, vec![false, true, true, false]);
    let extreme = qualify_soa_duration(
        &[0., 1., 2.],
        &[0., f64::MAX, 0.],
        SoaLimitTrace::Samples(&[f64::MAX, 0., f64::MAX]),
        1.,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(
        (extreme.excursions[0].start_s, extreme.excursions[0].end_s),
        (0.5, 1.5)
    );
    let clipped = qualify_soa_duration(
        &[0., 1., 2.],
        &[2., 1., 2.],
        SoaLimitTrace::Constant(1.),
        1.5,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(clipped.evidence.rejected_excursions, 2);
    assert_eq!(clipped.evidence.clipped_excursions, 2);
    assert_eq!(clipped.evidence.total_exceedance_s, 2.);
    assert_eq!(clipped.evidence.longest_excursion_s, 1.);
    struct Abort;
    impl AbortSignal for Abort {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        qualify_soa_duration(&time, &stress, SoaLimitTrace::Constant(1.), 2., &Abort),
        Err(SimulationError::Aborted)
    ));
}
