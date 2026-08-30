//! Platform-neutral planning for Xyce `.OPTIONS RESTART` checkpoint jobs.

use crate::Value;
use crate::netlist::XyceRestartInterval;
use crate::resource::{ResourceKind, ResourceLimitError};
use std::collections::HashSet;
use thiserror::Error;

use super::checkpoint::TransientCheckpointEncoding;

/// Absolute time window used by Xyce 7.10 to decide whether restart output is due.
///
/// Xyce defines `MachinePrecision()` as `2 * 10^-digits10 = 2e-15`,
/// deliberately distinct from IEEE epsilon. `Transient::testRestartSaveTime`
/// treats a scheduled restart as due when `scheduled - current <= 2 *
/// MachinePrecision()`. Restart output observes accepted points and does not
/// add a solver breakpoint, so this tolerance is independent of the
/// integrator's breakpoint tolerance.
pub const XYCE_RESTART_SCHEDULE_TOLERANCE: Value = 4.0e-15;

/// Whether an accepted solver time satisfies Xyce's restart-output schedule.
#[inline]
pub fn xyce_restart_schedule_is_due(accepted_time: Value, scheduled_time: Value) -> bool {
    scheduled_time - accepted_time <= XYCE_RESTART_SCHEDULE_TOLERANCE
}

/// Failure to construct an exact, bounded Xyce restart checkpoint plan.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum XyceRestartPlanError {
    /// The initial checkpoint cadence is not finite and positive.
    #[error(".OPTIONS RESTART INITIAL_INTERVAL must be finite and positive")]
    InvalidInitialInterval,
    /// The transient stop time cannot bound a restart schedule.
    #[error(".OPTIONS RESTART requires a finite, positive .TRAN stop time")]
    InvalidStopTime,
    /// A cadence transition is unordered or outside its numeric domain.
    #[error(
        ".OPTIONS RESTART transition {index} time must be finite, nonnegative, and strictly increasing"
    )]
    InvalidTransitionTime { index: usize },
    /// A cadence transition requests an invalid interval.
    #[error(".OPTIONS RESTART transition {index} interval must be finite and positive")]
    InvalidTransitionInterval { index: usize },
    /// Floating-point resolution prevents the cadence from moving forward.
    #[error(
        ".OPTIONS RESTART cadence cannot advance beyond {current:.17e}s; increase the interval or reduce the simulated time scale"
    )]
    CadenceDoesNotAdvance { current: Value },
    /// The nominal schedule exceeds its caller-provided analysis-point budget.
    #[error(
        ".OPTIONS RESTART schedule exceeds the configured analysis-point limit of {}",
        .source.limit
    )]
    AnalysisPointLimit { source: ResourceLimitError },
    /// Xyce's six-significant-digit suffix maps two nominal times to one name.
    #[error(
        ".OPTIONS RESTART filename precision maps more than one checkpoint to '{logical_name}'; choose a wider checkpoint interval or shorter stop time"
    )]
    LogicalNameCollision { logical_name: String },
    /// A checkpoint time cannot be represented by Xyce's finite-time naming contract.
    #[error(".OPTIONS RESTART checkpoint time must be finite, found {time}")]
    NonfiniteCheckpointTime { time: Value },
}

/// Exact nominal schedule, names, and encoding for one Xyce restart output job.
///
/// The job prefix is deliberately opaque here. Frontends remain responsible
/// for validating whether a logical name is safe on their filesystem or
/// storage backend. Construction validates every numeric input, enforces the
/// supplied point budget, and refuses Xyce `%g` filename collisions before a
/// simulation starts.
#[derive(Debug, Clone, PartialEq)]
pub struct XyceRestartJobPlan {
    job: String,
    nominal_times: Vec<Value>,
    name_suffixes: Vec<String>,
    encoding: TransientCheckpointEncoding,
}

impl XyceRestartJobPlan {
    /// Construct one bounded checkpoint-output plan from parsed Xyce options.
    pub fn new(
        job: impl Into<String>,
        initial_interval: Value,
        intervals: &[XyceRestartInterval],
        tstop: Value,
        pack: Option<bool>,
        max_points: usize,
    ) -> Result<Self, XyceRestartPlanError> {
        if !initial_interval.is_finite() || initial_interval <= 0.0 {
            return Err(XyceRestartPlanError::InvalidInitialInterval);
        }
        if !tstop.is_finite() || tstop <= 0.0 {
            return Err(XyceRestartPlanError::InvalidStopTime);
        }

        let mut previous_transition = None;
        for (index, transition) in intervals.iter().enumerate() {
            if !transition.time.is_finite()
                || transition.time < 0.0
                || previous_transition.is_some_and(|previous| transition.time <= previous)
            {
                return Err(XyceRestartPlanError::InvalidTransitionTime { index });
            }
            if !transition.interval.is_finite() || transition.interval <= 0.0 {
                return Err(XyceRestartPlanError::InvalidTransitionInterval { index });
            }
            previous_transition = Some(transition.time);
        }

        let mut nominal_times = Vec::new();
        push_bounded_checkpoint(&mut nominal_times, 0.0, max_points)?;
        let mut current = 0.0;
        while current < tstop {
            let next = next_restart_time(current, initial_interval, intervals)?;
            if next > tstop {
                break;
            }
            push_bounded_checkpoint(&mut nominal_times, next, max_points)?;
            current = next;
        }

        let job = job.into();
        let mut name_suffixes = Vec::with_capacity(nominal_times.len());
        let mut unique_suffixes = HashSet::with_capacity(nominal_times.len());
        for &time in &nominal_times {
            let suffix = xyce_restart_time_suffix(time)?;
            if !unique_suffixes.insert(suffix.clone()) {
                return Err(XyceRestartPlanError::LogicalNameCollision {
                    logical_name: format!("{job}{suffix}"),
                });
            }
            name_suffixes.push(suffix);
        }

        Ok(Self {
            job,
            nominal_times,
            name_suffixes,
            encoding: if pack.unwrap_or(true) {
                TransientCheckpointEncoding::Packed
            } else {
                TransientCheckpointEncoding::Unpacked
            },
        })
    }

    /// Strictly increasing nominal checkpoint times, including time zero.
    pub fn nominal_times(&self) -> &[Value] {
        &self.nominal_times
    }

    /// Portable checkpoint representation selected by Xyce's `PACK` option.
    pub fn encoding(&self) -> TransientCheckpointEncoding {
        self.encoding
    }

    /// Logical filename for an exact member of [`Self::nominal_times`].
    pub fn logical_name(&self, nominal_time: Value) -> Option<String> {
        self.nominal_times
            .binary_search_by(|time| time.total_cmp(&nominal_time))
            .ok()
            .and_then(|index| self.name_suffixes.get(index))
            .map(|suffix| format!("{}{suffix}", self.job))
    }
}

fn push_bounded_checkpoint(
    schedule: &mut Vec<Value>,
    time: Value,
    max_points: usize,
) -> Result<(), XyceRestartPlanError> {
    let requested = schedule.len().saturating_add(1);
    ResourceLimitError::ensure(ResourceKind::AnalysisPoints, requested, max_points)
        .map_err(|source| XyceRestartPlanError::AnalysisPointLimit { source })?;
    schedule.push(time);
    Ok(())
}

fn next_restart_time(
    current: Value,
    initial_interval: Value,
    intervals: &[XyceRestartInterval],
) -> Result<Value, XyceRestartPlanError> {
    let first_transition = intervals.first().map(|transition| transition.time);
    let candidate = if first_transition.is_none_or(|first| current < first) {
        let cadence = current + initial_interval;
        first_transition.map_or(cadence, |first| cadence.min(first))
    } else {
        let active_index = intervals.partition_point(|transition| transition.time <= current) - 1;
        let active = intervals[active_index];
        let steps = ((current - active.time) / active.interval).floor();
        let cadence = active.time + (steps + 1.0) * active.interval;
        intervals
            .get(active_index + 1)
            .map_or(cadence, |next| cadence.min(next.time))
    };
    if !candidate.is_finite() || candidate <= current {
        return Err(XyceRestartPlanError::CadenceDoesNotAdvance { current });
    }
    Ok(candidate)
}

fn xyce_restart_time_suffix(time: Value) -> Result<String, XyceRestartPlanError> {
    if !time.is_finite() {
        return Err(XyceRestartPlanError::NonfiniteCheckpointTime { time });
    }
    if time == 0.0 {
        return Ok(if time.is_sign_negative() {
            "-0".to_string()
        } else {
            "0".to_string()
        });
    }

    // Xyce inserts the double into a fresh C++ ostream: defaultfloat with six
    // significant digits. Select notation from the exponent after rounding.
    let scientific = format!("{time:.5e}");
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("finite Rust scientific formatting always contains an exponent");
    let exponent = exponent
        .parse::<i32>()
        .expect("Rust scientific formatting emits a numeric exponent");
    if (-4..6).contains(&exponent) {
        let decimals =
            usize::try_from(5 - exponent).expect("fixed-point exponent range is non-negative");
        let mut text = format!("{time:.decimals$}");
        while text.ends_with('0') && text.contains('.') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        Ok(text)
    } else {
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        Ok(format!("{mantissa}e{exponent:+03}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{xyce_restart_schedule_is_due, xyce_restart_time_suffix};
    use crate::Value;

    #[test]
    fn schedule_uses_xyce_absolute_machine_precision_window() {
        let scheduled: Value = 2.0e-4;
        let one_ulp_below = Value::from_bits(scheduled.to_bits() - 1);

        assert!(one_ulp_below < scheduled);
        assert!(xyce_restart_schedule_is_due(one_ulp_below, scheduled));
        assert!(xyce_restart_schedule_is_due(scheduled, scheduled));
        assert!(xyce_restart_schedule_is_due(scheduled + 1.0e-9, scheduled));
        assert!(xyce_restart_schedule_is_due(scheduled - 3.0e-15, scheduled));
        assert!(!xyce_restart_schedule_is_due(
            scheduled - 5.0e-15,
            scheduled
        ));
    }

    #[test]
    fn suffix_is_byte_exact_at_xyce_defaultfloat_boundaries() {
        let oracle = [
            (0x0000_0000_0000_0000, "0"),
            (0x8000_0000_0000_0000, "-0"),
            (0x3e35_798e_e230_8c3a, "5e-09"),
            (0x3f1a_36e2_0f35_445d, "9.99999e-05"),
            (0x3f1a_36e2_0f35_445e, "0.0001"),
            (0x3f1a_36e2_eb1c_432d, "0.0001"),
            (0x3f50_624d_4981_4abb, "0.001"),
            (0x40f8_69ff_5c28_f5c3, "100000"),
            (0x412e_847e_ffff_ffff, "999999"),
            (0x412e_847f_0000_0000, "1e+06"),
            (0x412e_8480_0000_0000, "1e+06"),
            (0xbf1a_36e2_0f35_445e, "-0.0001"),
            (0xc12e_847f_0000_0000, "-1e+06"),
            (0x54b2_49ad_2594_c37d, "1e+100"),
            (0x2b2b_ff2e_e48e_0530, "1e-100"),
            (0x0000_0000_0000_0001, "4.94066e-324"),
            (0x7fef_ffff_ffff_ffff, "1.79769e+308"),
        ];
        for (bits, expected) in oracle {
            let time = f64::from_bits(bits);
            assert_eq!(
                xyce_restart_time_suffix(time).unwrap(),
                expected,
                "Xyce defaultfloat mismatch for {time:.17e} (0x{bits:016X})"
            );
        }

        for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(xyce_restart_time_suffix(nonfinite).is_err());
        }
    }
}
