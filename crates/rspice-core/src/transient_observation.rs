//! Sparse physical observations shared by results and live sample hooks.

use crate::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A current's physical identity, independent of its displayed probe spelling.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum CurrentImpulseOwner {
    /// An existing solved or derived branch-current signal.
    Branch {
        #[serde(rename = "branchName")]
        branch_name: String,
    },
    /// A device lead current in the operating-point parameter namespace.
    DeviceLead {
        #[serde(rename = "deviceName")]
        device_name: String,
        parameter: String,
    },
}

impl CurrentImpulseOwner {
    pub(crate) fn canonical(&self) -> Self {
        match self {
            Self::Branch { branch_name } => Self::Branch {
                branch_name: branch_name.to_ascii_lowercase(),
            },
            Self::DeviceLead {
                device_name,
                parameter,
            } => Self::DeviceLead {
                device_name: device_name.to_ascii_lowercase(),
                parameter: parameter.to_ascii_lowercase(),
            },
        }
    }

    pub(crate) fn value_count(&self) -> usize {
        let bytes = match self {
            Self::Branch { branch_name } => branch_name.len(),
            Self::DeviceLead {
                device_name,
                parameter,
            } => device_name.len().saturating_add(parameter.len()),
        };
        // One value for the coverage flag and owner discriminant, plus names.
        1usize.saturating_add(bytes.div_ceil(std::mem::size_of::<Value>()))
    }

    fn validate(&self) -> Result<(), String> {
        let valid = match self {
            Self::Branch { branch_name } => !branch_name.trim().is_empty(),
            Self::DeviceLead {
                device_name,
                parameter,
            } => !device_name.trim().is_empty() && !parameter.trim().is_empty(),
        };
        valid
            .then_some(())
            .ok_or_else(|| "current impulse has an empty owner identity".into())
    }
}

impl std::fmt::Display for CurrentImpulseOwner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Branch { branch_name } => write!(formatter, "I({branch_name})"),
            Self::DeviceLead {
                device_name,
                parameter,
            } => write!(formatter, "@{device_name}[{parameter}]"),
        }
    }
}

/// One current impulse, integrated over its instantaneous event.
///
/// Its charge is not a finite current sample. Never divide it by a timestep
/// or interpolate it onto a waveform grid.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurrentImpulsePoint {
    /// Exact accepted event time in seconds.
    pub time: Value,
    /// Signed integrated current in coulombs, in the named owner's ordinary
    /// current orientation.
    pub charge_coulombs: Value,
}

/// Newly accepted impulses and coverage for one current in a run segment.
///
/// Fresh startup can contribute an impulse at zero. A checkpoint resume
/// starts from its accepted outgoing state and does not replay a past
/// impulse at the seam. These sparse events are preserved independently of
/// analog waveform retention and compression.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentImpulseTrace {
    /// Canonical current owner, with its existing current orientation.
    #[serde(flatten)]
    pub owner: CurrentImpulseOwner,
    /// Whether all impulses of this current in the run segment were recorded.
    /// An empty complete trace proves absence; an omitted owner does not.
    /// Legacy records restore false, retaining data without inventing coverage.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub complete: bool,
    /// Nonzero finite charge impulses in strictly increasing time order.
    pub points: Vec<CurrentImpulsePoint>,
}

impl<'de> Deserialize<'de> for CurrentImpulseTrace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct Wire {
            branch_name: Option<String>,
            device_name: Option<String>,
            parameter: Option<String>,
            #[serde(default)]
            complete: bool,
            points: Vec<CurrentImpulsePoint>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let owner = match (wire.branch_name, wire.device_name, wire.parameter) {
            (Some(branch_name), None, None) => CurrentImpulseOwner::Branch { branch_name },
            (None, Some(device_name), Some(parameter)) => CurrentImpulseOwner::DeviceLead {
                device_name,
                parameter,
            },
            _ => {
                return Err(serde::de::Error::custom(
                    "current impulse must identify exactly one branch or device lead",
                ));
            }
        };
        Ok(Self {
            owner,
            complete: wire.complete,
            points: wire.points,
        })
    }
}

impl CurrentImpulseTrace {
    /// Validate the trace against its result's time extent.
    pub(crate) fn validate(&self, start: Value, stop: Value) -> Result<(), String> {
        self.owner.validate()?;
        if self.points.is_empty() && !self.complete {
            return Err(
                "current impulse trace has neither observations nor complete coverage".into(),
            );
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
                    self.owner
                ));
            }
            previous = Some(point.time);
        }
        Ok(())
    }
}

/// Validate typed ownership and preserve the distinction between missing
/// coverage and a complete empty history for a particular current.
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
        if let CurrentImpulseOwner::Branch { branch_name } = &trace.owner
            && !branches.contains(&branch_name.to_ascii_lowercase())
        {
            return Err(format!("unknown current impulse branch '{}'", branch_name));
        }
        if !names.insert(trace.owner.canonical()) {
            return Err(format!("duplicate current impulse owner '{}'", trace.owner));
        }
    }
    Ok(())
}

pub(crate) fn current_impulse_value_count(traces: Option<&[CurrentImpulseTrace]>) -> usize {
    traces.into_iter().flatten().fold(0usize, |count, trace| {
        count
            .saturating_add(trace.points.len().saturating_mul(2))
            .saturating_add(trace.owner.value_count())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_ownership_separates_namespaces_and_preserves_legacy_coverage() {
        let legacy = r#"{"branchName":"Q1","points":[{"time":0.0,"chargeCoulombs":-1e-12}]}"#;
        let branch: CurrentImpulseTrace = serde_json::from_str(legacy).unwrap();
        assert!(!branch.complete);
        assert!(!serde_json::to_string(&branch).unwrap().contains("complete"));
        let lead = CurrentImpulseTrace {
            owner: CurrentImpulseOwner::DeviceLead {
                device_name: "Q1".into(),
                parameter: "ic".into(),
            },
            complete: true,
            points: vec![],
        };
        let validate = |traces: &[CurrentImpulseTrace]| {
            validate_current_impulse_traces(Some(traces), Some(0.0), Some(1.0), ["Q1"].into_iter())
        };
        validate(&[branch.clone(), lead.clone()]).unwrap();
        let mut duplicate = lead.clone();
        duplicate.owner = CurrentImpulseOwner::DeviceLead {
            device_name: "q1".into(),
            parameter: "IC".into(),
        };
        assert!(validate(&[lead.clone(), duplicate]).is_err());
        let mut incomplete = lead.clone();
        incomplete.complete = false;
        assert!(validate(&[incomplete]).is_err());
        for identity in [
            r#""branchName":"Q1","deviceName":"Q1","parameter":"ic""#,
            r#""branchName":"Q1","parameter":"ic""#,
            r#""deviceName":"Q1""#,
            r#""parameter":"ic""#,
            r#""deviceName":"Q1","parameter":"ic","unknown":true"#,
        ] {
            let json = format!("{{{identity},\"complete\":true,\"points\":[]}}");
            assert!(
                serde_json::from_str::<CurrentImpulseTrace>(&json).is_err(),
                "{json}"
            );
        }
        let json = serde_json::to_string(&lead).unwrap();
        assert_eq!(
            serde_json::from_str::<CurrentImpulseTrace>(&json).unwrap(),
            lead
        );
    }

    fn trace() -> CurrentImpulseTrace {
        CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "Vdrive".into(),
            },
            complete: false,
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
        empty.owner = CurrentImpulseOwner::Branch {
            branch_name: " ".into(),
        };
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
        duplicate.owner = CurrentImpulseOwner::Branch {
            branch_name: "vDRIVE".into(),
        };
        assert!(validate(&[traces[0].clone(), duplicate], &["Vdrive"]).is_err());
        assert!(
            validate_current_impulse_traces(Some(&traces), None, None, ["Vdrive"].into_iter())
                .is_err()
        );
        assert_eq!(current_impulse_value_count(Some(&traces)), 6);
        assert_eq!(current_impulse_value_count(None), 0);
        validate_current_impulse_traces(Some(&[]), None, None, [].into_iter()).unwrap();
    }
}
