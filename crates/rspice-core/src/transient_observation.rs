//! Sparse physical observations shared by results and live sample hooks.

use crate::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// One current impulse, integrated over its instantaneous event.
///
/// Its charge is not a finite current sample. Never divide it by a timestep
/// or interpolate it onto a waveform grid.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurrentImpulsePoint {
    /// Exact accepted event time in seconds.
    pub time: Value,
    /// Signed integrated current in coulombs, in the named branch's ordinary
    /// current orientation.
    pub charge_coulombs: Value,
}

/// Newly accepted current impulses for one named branch in a run segment.
///
/// Fresh startup can contribute an impulse at zero. A checkpoint resume
/// starts from its accepted outgoing state and does not replay a past
/// impulse at the seam. These sparse events are preserved independently of
/// analog waveform retention and compression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurrentImpulseTrace {
    /// Canonical branch name, with the same orientation as its current trace.
    pub branch_name: String,
    /// Nonzero finite charge impulses in strictly increasing time order.
    pub points: Vec<CurrentImpulsePoint>,
}

impl CurrentImpulseTrace {
    /// Validate the trace against its result's time extent.
    pub(crate) fn validate(&self, start: Value, stop: Value) -> Result<(), String> {
        if self.branch_name.trim().is_empty() {
            return Err("current impulse trace has no branch name".into());
        }
        if self.points.is_empty() {
            return Err("current impulse trace has no impulses".into());
        }
        if !start.is_finite() || !stop.is_finite() || start < 0.0 || stop < start {
            return Err("current impulse trace has an invalid time extent".into());
        }
        let mut previous = None;
        for point in &self.points {
            if !point.time.is_finite()
                || point.time < start
                || point.time > stop
                || previous.is_some_and(|time| point.time <= time)
                || !point.charge_coulombs.is_finite()
                || point.charge_coulombs == 0.0
            {
                return Err(format!(
                    "current impulse trace '{}' has an invalid time or charge",
                    self.branch_name
                ));
            }
            previous = Some(point.time);
        }
        Ok(())
    }
}

/// Validate sparse branch ownership without conflating unavailable history
/// with an available empty event set.
pub(crate) fn validate_current_impulse_traces<'a>(
    traces: Option<&[CurrentImpulseTrace]>,
    start: Option<Value>,
    stop: Option<Value>,
    branches: impl Iterator<Item = &'a str>,
) -> Result<(), String> {
    let Some(traces) = traces else { return Ok(()) };
    if traces.is_empty() {
        return Ok(());
    }
    let (Some(start), Some(stop)) = (start, stop) else {
        return Err("current impulses require a nonempty time extent".into());
    };
    let branches = branches
        .map(str::to_ascii_lowercase)
        .collect::<HashSet<_>>();
    let mut names = HashSet::with_capacity(traces.len());
    for trace in traces {
        trace.validate(start, stop)?;
        if !branches.contains(&trace.branch_name.to_ascii_lowercase()) {
            return Err(format!(
                "unknown current impulse branch '{}'",
                trace.branch_name
            ));
        }
        if !names.insert(trace.branch_name.to_ascii_lowercase()) {
            return Err(format!(
                "duplicate current impulse branch '{}'",
                trace.branch_name
            ));
        }
    }
    Ok(())
}

pub(crate) fn current_impulse_value_count(traces: Option<&[CurrentImpulseTrace]>) -> usize {
    traces.into_iter().flatten().fold(0usize, |count, trace| {
        count
            .saturating_add(trace.points.len().saturating_mul(2))
            .saturating_add(
                trace
                    .branch_name
                    .len()
                    .div_ceil(std::mem::size_of::<Value>()),
            )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trace() -> CurrentImpulseTrace {
        CurrentImpulseTrace {
            branch_name: "Vdrive".into(),
            points: vec![
                CurrentImpulsePoint {
                    time: 0.0,
                    charge_coulombs: -1e-12,
                },
                CurrentImpulsePoint {
                    time: 2.0,
                    charge_coulombs: Value::from_bits(1),
                },
            ],
        }
    }

    #[test]
    fn impulse_observation_rejects_invalid_history_without_clipping_tiny_charges() {
        let valid = trace();
        valid.validate(0.0, 2.0).unwrap();
        for time in [Value::NAN, Value::INFINITY, -1.0, 0.0, 3.0] {
            let mut invalid = valid.clone();
            invalid.points[1].time = time;
            assert!(invalid.validate(0.0, 2.0).is_err(), "time {time}");
        }
        for charge in [Value::NAN, Value::INFINITY, -Value::INFINITY, 0.0, -0.0] {
            let mut invalid = valid.clone();
            invalid.points[1].charge_coulombs = charge;
            assert!(invalid.validate(0.0, 2.0).is_err(), "charge {charge}");
        }
        let mut empty = valid.clone();
        empty.points.clear();
        assert!(empty.validate(0.0, 2.0).is_err());
        empty = valid.clone();
        empty.branch_name = " ".into();
        assert!(empty.validate(0.0, 2.0).is_err());
        for (start, stop) in [
            (Value::NAN, 2.0),
            (0.0, Value::INFINITY),
            (2.0, 1.0),
            (-1.0, 2.0),
        ] {
            assert!(valid.validate(start, stop).is_err());
        }
    }

    #[test]
    fn impulse_observation_validates_named_ownership_and_counts_retained_values() {
        let traces = vec![trace()];
        let validate = |traces: &[CurrentImpulseTrace], names: &[&str]| {
            validate_current_impulse_traces(
                Some(traces),
                Some(0.0),
                Some(2.0),
                names.iter().copied(),
            )
        };
        validate(&traces, &["vDRIVE"]).unwrap();
        assert!(validate(&traces, &["other"]).is_err());
        let mut duplicate = traces[0].clone();
        duplicate.branch_name = "vDRIVE".into();
        assert!(validate(&[traces[0].clone(), duplicate], &["Vdrive"]).is_err());
        assert!(
            validate_current_impulse_traces(Some(&traces), None, None, ["Vdrive"].into_iter())
                .is_err()
        );
        assert_eq!(current_impulse_value_count(Some(&traces)), 5);
        assert_eq!(current_impulse_value_count(None), 0);
        validate_current_impulse_traces(Some(&[]), None, None, [].into_iter()).unwrap();
    }
}
