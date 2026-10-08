//! Singular node-voltage observations, independent of finite waveforms.
use rspice_core::VoltageImpulseTrace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoltageImpulseHistoryEvidence {
    pub start_time_s: f64,
    pub stop_time_s: f64,
    /// Delivery completeness is independent of each node's physical coverage.
    pub delivery_complete: bool,
    pub traces: Vec<VoltageImpulseTrace>,
}

impl VoltageImpulseHistoryEvidence {
    pub fn validate(&self) -> Result<(), String> {
        if !self.start_time_s.is_finite()
            || !self.stop_time_s.is_finite()
            || self.start_time_s < 0.0
            || self.stop_time_s < self.start_time_s
        {
            return Err("voltage impulses have an invalid retained time extent".into());
        }
        let mut nodes = std::collections::HashSet::new();
        for trace in &self.traces {
            trace.validate(self.start_time_s, self.stop_time_s)?;
            if !nodes.insert(trace.node_name.to_ascii_lowercase()) {
                return Err(format!(
                    "duplicate voltage impulse node '{}'",
                    trace.node_name
                ));
            }
        }
        Ok(())
    }
}
