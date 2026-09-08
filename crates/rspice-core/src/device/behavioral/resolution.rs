//! Bounded numerical resolution of time-only source waveforms.
//!
//! These points supplement physical events; the engine still qualifies the
//! solved circuit on a refined mesh. Source programs and interval arithmetic
//! are shared with the ordinary behavioral evaluator and feature collector.

use super::*;
use crate::abort_signal::AbortSignal;
use crate::device::NonlinearConvergenceCriteria;
use crate::expr::{TimeEnclosure, TimeInterval, compile_time_expression};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::BTreeSet;

#[derive(PartialEq, Eq)]
enum Resolution {
    Qualified,
    NeedsSubdivision,
    Discontinuous,
}

impl BehavioralSources {
    pub(crate) fn needs_time_resolution(&self, period: Value) -> bool {
        self.voltage_sources.iter().any(|source| {
            super::periodicity::needs_time_features(
                &source.ast,
                period,
                &source.periodicity_context(),
            )
        }) || self.current_sources.iter().any(|source| {
            super::periodicity::needs_time_features(
                &source.ast,
                period,
                &source.periodicity_context(),
            )
        })
    }

    pub(crate) fn refine_time_mesh(
        &self,
        times: &[Value],
        criteria: NonlinearConvergenceCriteria,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, BehavioralBreakpointError> {
        if abort.is_aborted() {
            return Err(BehavioralBreakpointError::Aborted);
        }
        let stop = times.last().copied().unwrap_or(0.0);
        if times.len() < 2 || times.first() != Some(&0.0) || !stop.is_finite() || stop <= 0.0 {
            return Err(BehavioralBreakpointError::Invalid(
                "source resolution requires an increasing finite time mesh starting at zero",
            ));
        }
        for (index, pair) in times.windows(2).enumerate() {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(BehavioralBreakpointError::Aborted);
            }
            if pair[0] >= pair[1] || !pair[0].is_finite() {
                return Err(BehavioralBreakpointError::Invalid(
                    "source resolution requires an increasing finite time mesh starting at zero",
                ));
            }
        }
        let mut additions = BTreeSet::new();
        let mut pending = Vec::new();
        let mut operations: usize = 0;
        let sources = self
            .voltage_sources
            .iter()
            .map(|source| {
                (
                    &source.ast,
                    source.periodicity_context(),
                    criteria.voltage_tolerance(),
                )
            })
            .chain(self.current_sources.iter().map(|source| {
                (
                    &source.ast,
                    source.periodicity_context(),
                    criteria.current_tolerance(),
                )
            }));
        for (expression, context, absolute) in sources {
            if !super::periodicity::needs_time_features(expression, stop, &context) {
                continue;
            }
            let program = compile_time_expression(expression, &context);
            let Some(mut bounds) = TimeEnclosure::new(&program, stop) else {
                continue;
            };
            let mut vm = Vm::new();
            let mut charge = || {
                if abort.is_aborted() {
                    return Err(BehavioralBreakpointError::Aborted);
                }
                operations = operations.saturating_add(program.instructions.len());
                if operations > 16_000_000 {
                    return Err(BehavioralBreakpointError::Invalid(
                        "source resolution exceeds its 16000000-instruction work limit",
                    ));
                }
                Ok(())
            };
            let mut resolved =
                |interval: TimeInterval| -> Result<Resolution, BehavioralBreakpointError> {
                    charge()?;
                    let Some(domain) = bounds
                        .evaluate_centered(interval, &context)
                        .filter(|domain| domain.value.is_finite())
                    else {
                        return Ok(Resolution::NeedsSubdivision);
                    };
                    let value = domain.value;
                    let minimum = if value.contains(0.0) {
                        0.0
                    } else {
                        value.lower.abs().min(value.upper.abs())
                    };
                    let scale = value.lower.abs().max(value.upper.abs()).max(absolute);
                    let tolerance =
                        absolute / scale + criteria.relative_tolerance() * (minimum / scale);
                    if domain.interpolation_error((interval.upper - interval.lower) / stop) / scale
                        <= tolerance
                    {
                        return Ok(Resolution::Qualified);
                    }
                    if !domain.continuous {
                        // Outward rounding at a branch boundary can include its
                        // other side. Cover all interior VM timestamps by bounds,
                        // then authenticate the two excluded endpoints directly.
                        let lower = interval.lower.next_up();
                        let upper = interval.upper.next_down();
                        if lower <= upper {
                            charge()?;
                            if let Some(interior) = bounds
                                .evaluate(TimeInterval { lower, upper }, &context)
                                .filter(|bounds| {
                                    bounds.value.is_finite()
                                        && bounds.value.lower == bounds.value.upper
                                })
                            {
                                let mut same = true;
                                for time in [interval.lower, interval.upper] {
                                    charge()?;
                                    same &= vm.execute(&program, &Context { time, ..context })
                                        == interior.value.lower;
                                }
                                if same {
                                    return Ok(Resolution::Qualified);
                                }
                            }
                        }
                    }
                    Ok(if domain.continuous {
                        Resolution::NeedsSubdivision
                    } else {
                        Resolution::Discontinuous
                    })
                };
            if resolved(TimeInterval {
                lower: 0.0,
                upper: stop,
            })? == Resolution::Qualified
            {
                continue;
            }
            for pair in times.windows(2) {
                pending.push(TimeInterval {
                    lower: pair[0],
                    upper: pair[1],
                });
                while let Some(interval) = pending.pop() {
                    let resolution = resolved(interval)?;
                    if resolution == Resolution::Qualified {
                        continue;
                    }
                    let midpoint = interval.lower + 0.5 * (interval.upper - interval.lower);
                    if midpoint == interval.lower || midpoint == interval.upper {
                        if resolution == Resolution::Discontinuous {
                            // Existing adjacent VM clocks remain distinct.
                            // The engine's solved alternate-method traversal
                            // owns qualification at this time precision floor.
                            continue;
                        }
                        return Err(BehavioralBreakpointError::Invalid(
                            "a source waveform or its domain cannot be resolved at the available time precision",
                        ));
                    }
                    if !additions.contains(&midpoint.to_bits()) {
                        let count = times
                            .len()
                            .saturating_add(additions.len())
                            .saturating_add(1);
                        ResourceLimitError::ensure(
                            ResourceKind::AnalysisPoints,
                            count - 1,
                            limits.max_analysis_points,
                        )?;
                        // Input, additions and eventual output coexist, as
                        // does the bounded depth-first subdivision scratch.
                        ResourceLimitError::ensure(
                            ResourceKind::ResultValues,
                            count
                                .saturating_mul(2)
                                .saturating_add(pending.len().saturating_add(2).saturating_mul(2)),
                            limits.max_result_values,
                        )?;
                        additions.insert(midpoint.to_bits());
                    }
                    pending.push(TimeInterval {
                        lower: midpoint,
                        upper: interval.upper,
                    });
                    pending.push(TimeInterval {
                        lower: interval.lower,
                        upper: midpoint,
                    });
                }
            }
        }
        if additions.is_empty() {
            return Ok(None);
        }
        let mut result = Vec::new();
        result
            .try_reserve_exact(times.len().saturating_add(additions.len()))
            .map_err(|_| {
                BehavioralBreakpointError::Invalid("source resolution mesh allocation failed")
            })?;
        let mut additions = additions.into_iter().map(Value::from_bits).peekable();
        for (index, &time) in times.iter().enumerate() {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(BehavioralBreakpointError::Aborted);
            }
            while let Some(point) = additions.next_if(|&point| point < time) {
                if result.len().is_multiple_of(256) && abort.is_aborted() {
                    return Err(BehavioralBreakpointError::Aborted);
                }
                result.push(point);
            }
            result.push(time);
        }
        Ok(Some(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    fn source(expression: &str) -> BehavioralSources {
        let mut sources = BehavioralSources::new();
        sources
            .voltage_sources
            .push(BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap());
        sources
    }

    #[test]
    fn narrow_source_resolution_preserves_clocks_and_bounds_vm_interpolation() {
        for stop in [1e-30, 1e-6, 1e300] {
            for gain in [-1e-200_f64, 1.0, 1e200] {
                let expression = format!(
                    "{gain:e}*exp(-1000000*(cos(8*pi*time/{stop:e}+0.1)+0.5*cos(16*pi*time/{stop:e}+0.2)-0.25)^2)"
                );
                let mut sources = source(&expression);
                let mut original = (0..=32)
                    .map(|index| stop * (index as Value / 32.0))
                    .collect::<Vec<_>>();
                original.push(0.173 * stop);
                original.sort_by(Value::total_cmp);
                let criteria = NonlinearConvergenceCriteria::new(gain.abs() * 1e-6, 1e-12, 1e-3);
                let refined = sources
                    .refine_time_mesh(&original, criteria, &ResourceLimits::default(), &NoAbort)
                    .unwrap()
                    .unwrap();
                assert!(
                    refined.len() < 12_000,
                    "{expression}: {} points",
                    refined.len()
                );
                assert!(original.iter().all(|time| {
                    refined
                        .binary_search_by(|value| value.total_cmp(time))
                        .is_ok()
                }));
                for pair in refined.windows(2) {
                    assert!(pair[0] < pair[1]);
                    let left = sources.voltage_sources[0].evaluate(&[], pair[0]).unwrap() / gain;
                    let right = sources.voltage_sources[0].evaluate(&[], pair[1]).unwrap() / gain;
                    for sample in 1..8 {
                        let fraction = sample as Value / 8.0;
                        let time = pair[0] + fraction * (pair[1] - pair[0]);
                        let actual = sources.voltage_sources[0].evaluate(&[], time).unwrap() / gain;
                        let linear = left + fraction * (right - left);
                        assert!(
                            (actual - linear).abs() <= 1e-6 + 1e-3 * actual.abs(),
                            "{expression}, t={time:e}: error={:e}, actual={actual:e}",
                            (actual - linear).abs()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn source_resolution_preserves_adjacent_jumps_and_refuses_resource_or_domain_failures() {
        let original = [0.0, 0.5_f64.next_down(), 0.5, 1.0];
        let sources = source("exp(pwrs(time-0.5,0))");
        assert!(
            sources
                .refine_time_mesh(
                    &original,
                    NonlinearConvergenceCriteria::default(),
                    &ResourceLimits::default(),
                    &NoAbort
                )
                .unwrap()
                .is_none()
        );
        let sources = source("exp(-1000000*(cos(8*pi*time)-0.25)^2)");
        for limits in [
            ResourceLimits {
                max_analysis_points: 2,
                ..ResourceLimits::default()
            },
            ResourceLimits {
                max_result_values: 4,
                ..ResourceLimits::default()
            },
        ] {
            assert!(matches!(
                sources.refine_time_mesh(
                    &[0.0, 1.0],
                    NonlinearConvergenceCriteria::default(),
                    &limits,
                    &NoAbort
                ),
                Err(BehavioralBreakpointError::Resource(_))
            ));
        }
        assert!(matches!(
            sources.refine_time_mesh(
                &[0.0, 1.0],
                NonlinearConvergenceCriteria::default(),
                &ResourceLimits::default(),
                &CountingAbort::new(10)
            ),
            Err(BehavioralBreakpointError::Aborted)
        ));
        assert!(matches!(
            source("exp(1/(time-0.5))").refine_time_mesh(
                &original,
                NonlinearConvergenceCriteria::default(),
                &ResourceLimits::default(),
                &NoAbort
            ),
            Err(BehavioralBreakpointError::Invalid(_))
        ));
    }

    #[test]
    fn current_source_resolution_uses_current_units_and_unions_multiple_sources() {
        let shape = "exp(-10000*(cos(8*pi*time)-0.25)^2)";
        let original = [0.0, 0.25, 0.5, 0.75, 1.0];
        let mut sources = BehavioralSources::new();
        sources.current_sources.push(
            BehavioralCurrentSource::new("B1".to_owned(), 0, 1, &format!("1e-6*{shape}")).unwrap(),
        );
        let criteria = NonlinearConvergenceCriteria::default();
        let first = sources
            .refine_time_mesh(&original, criteria, &ResourceLimits::default(), &NoAbort)
            .unwrap()
            .unwrap();
        assert!(
            first.len() > 100,
            "current tolerance must resolve microampere peaks"
        );
        sources
            .current_sources
            .push(sources.current_sources[0].clone());
        let limits = ResourceLimits {
            max_analysis_points: first.len() - 1,
            ..ResourceLimits::default()
        };
        let repeated = sources
            .refine_time_mesh(&original, criteria, &limits, &NoAbort)
            .unwrap()
            .unwrap();
        assert_eq!(
            first, repeated,
            "identical sources share numerical clocks and their point budget"
        );
        for pair in repeated.windows(2) {
            let left = sources.current_sources[0].evaluate(&[], pair[0]).unwrap();
            let right = sources.current_sources[0].evaluate(&[], pair[1]).unwrap();
            let actual = sources.current_sources[0]
                .evaluate(&[], pair[0] + 0.5 * (pair[1] - pair[0]))
                .unwrap();
            assert!(
                (actual - 0.5 * (left + right)).abs()
                    <= criteria.current_abs + criteria.rel * actual.abs()
            );
        }
    }

    #[test]
    fn absolute_multiple_root_sources_resolve_with_the_existing_work_budget() {
        use crate::numerics::integration::BreakpointManager;

        for cycles in [1, 64] {
            let sources = source(&format!(
                "exp(-1000000*(abs(cos(2*pi*{cycles}*time+0.1))+0.5*abs(cos(2*(2*pi*{cycles}*time+0.1)))-0.75)^2)"
            ));
            let limits = ResourceLimits::default();
            let mut original = BreakpointManager::new_with_tolerance(Value::from_bits(1));
            original.extend((0..=4 * cycles).map(|index| index as Value / (4 * cycles) as Value));
            sources
                .collect_transient_breakpoints(
                    1.0,
                    &mut original,
                    &NoAbort,
                    limits.max_analysis_points,
                    true,
                )
                .unwrap();
            let times = original.times();
            let refined = sources
                .refine_time_mesh(
                    times,
                    NonlinearConvergenceCriteria::default(),
                    &limits,
                    &NoAbort,
                )
                .unwrap_or_else(|error| {
                    panic!("{cycles} cycles, {} input clocks: {error}", times.len())
                })
                .unwrap();
            assert!(times.iter().all(|time| {
                refined
                    .binary_search_by(|value| value.total_cmp(time))
                    .is_ok()
            }));
        }
    }

    #[test]
    fn continuous_sub_ulp_peaks_are_not_qualified_as_ideal_jumps() {
        let original = [0.0, 0.5_f64.next_down(), 0.5, 1.0];
        let result = source("exp(-1e34*(time-0.5)^2)").refine_time_mesh(
            &original,
            NonlinearConvergenceCriteria::default(),
            &ResourceLimits::default(),
            &NoAbort,
        );
        assert!(
            matches!(result, Err(BehavioralBreakpointError::Invalid(_))),
            "a smooth peak narrower than representable time must report precision exhaustion"
        );
    }
}
