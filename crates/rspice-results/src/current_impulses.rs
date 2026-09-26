//! Exact signed charge events retained separately from finite waveforms.

use rspice_core::{CurrentImpulseOwner, CurrentImpulseTrace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurrentImpulseHistoryEvidence {
    pub start_time_s: f64,
    pub stop_time_s: f64,
    /// False when the frontend could not retain every delivered observation.
    /// Per-current physical coverage remains on each trace independently.
    pub delivery_complete: bool,
    pub traces: Vec<CurrentImpulseTrace>,
}

impl CurrentImpulseHistoryEvidence {
    pub fn validate(&self) -> Result<(), String> {
        if !self.start_time_s.is_finite()
            || !self.stop_time_s.is_finite()
            || self.start_time_s < 0.0
            || self.stop_time_s < self.start_time_s
        {
            return Err("current impulses have an invalid retained time extent".into());
        }
        let mut owners = std::collections::HashSet::new();
        for trace in &self.traces {
            trace.validate(self.start_time_s, self.stop_time_s)?;
            let owner = match &trace.owner {
                CurrentImpulseOwner::Branch { branch_name } => {
                    (false, branch_name.to_ascii_lowercase(), String::new())
                }
                CurrentImpulseOwner::DeviceLead {
                    device_name,
                    parameter,
                } => (
                    true,
                    device_name.to_ascii_lowercase(),
                    parameter.to_ascii_lowercase(),
                ),
            };
            if !owners.insert(owner) {
                return Err(format!("duplicate current impulse owner '{}'", trace.owner));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> CurrentImpulseHistoryEvidence {
        CurrentImpulseHistoryEvidence {
            start_time_s: 0.0,
            stop_time_s: 1.0,
            delivery_complete: true,
            traces: vec![CurrentImpulseTrace {
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "V1".into(),
                },
                complete: true,
                points: vec![rspice_core::CurrentImpulsePoint {
                    time: 0.3,
                    charge_coulombs: -0.002,
                }],
            }],
        }
    }

    #[test]
    fn current_impulse_history_rejects_invalid_observations() {
        for change in 0..5 {
            let mut history = fixture();
            match change {
                0 => history.traces[0].points[0].charge_coulombs = f64::NAN,
                1 => history.traces[0].points[0].time = 2.0,
                2 => history.traces.push(history.traces[0].clone()),
                3 => history.traces[0].points[0].charge_coulombs = 0.0,
                _ => history.stop_time_s = -1.0,
            }
            assert!(history.validate().is_err());
        }
    }
}
