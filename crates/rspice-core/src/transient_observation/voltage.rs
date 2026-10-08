//! Voltage actions retain their own units and node identity.
use super::*;

/// One instantaneous voltage action, separate from finite voltage samples.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VoltageImpulsePoint {
    /// Exact accepted event time in seconds.
    pub time: Value,
    /// Integrated node-to-ground voltage in volt-seconds.
    pub volt_seconds: Value,
}

/// Singular node-to-ground voltage in one accepted run segment.
///
/// Differential observations subtract the two node histories. A resumed run
/// retains only newly accepted actions, without replaying its checkpoint seam.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VoltageImpulseTrace {
    pub node_name: String,
    /// Empty complete histories prove absence; missing histories do not.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub complete: bool,
    /// Nonzero voltage actions in strictly increasing event-time order.
    pub points: Vec<VoltageImpulsePoint>,
    /// Higher derivatives, ordered by (time, order), without duplicate pairs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derivatives: Vec<VoltageImpulseDerivative>,
}

impl VoltageImpulseTrace {
    /// Numeric scalar count, excluding the node identity and coverage flag.
    pub fn numeric_value_count(&self) -> usize {
        self.points
            .len()
            .saturating_mul(2)
            .saturating_add(self.derivatives.len().saturating_mul(3))
    }

    /// Whether a singular action belongs to the interval `(start, stop]`.
    pub fn has_impulses_in_window(&self, start: Value, stop: Value) -> bool {
        self.points
            .iter()
            .any(|point| point.time > start && point.time <= stop)
            || self
                .derivatives
                .iter()
                .any(|point| point.time > start && point.time <= stop)
    }

    /// Validate units' finite representation, ordering, coverage and extent.
    pub fn validate(&self, start: Value, stop: Value) -> Result<(), String> {
        if self.node_name.trim().is_empty() {
            return Err("voltage impulse has an empty node identity".into());
        }
        validate_impulse_series(
            &format!("voltage impulse trace 'V({})'", self.node_name),
            self.complete,
            self.points
                .iter()
                .map(|point| (point.time, point.volt_seconds)),
            &self.derivatives,
            start,
            stop,
        )
    }
}

pub(crate) fn validate_voltage_impulse_traces<'a>(
    traces: Option<&[VoltageImpulseTrace]>,
    start: Option<Value>,
    stop: Option<Value>,
    nodes: impl Iterator<Item = &'a str>,
) -> Result<(), String> {
    let Some(traces) = traces.filter(|traces| !traces.is_empty()) else {
        return Ok(());
    };
    let (Some(start), Some(stop)) = (start, stop) else {
        return Err("voltage impulses require a nonempty time extent".into());
    };
    let nodes = nodes.map(str::to_ascii_lowercase).collect::<HashSet<_>>();
    let mut names = HashSet::with_capacity(traces.len());
    for trace in traces {
        trace.validate(start, stop)?;
        let name = trace.node_name.to_ascii_lowercase();
        if !nodes.contains(&name) {
            return Err(format!(
                "unknown voltage impulse node '{}'",
                trace.node_name
            ));
        }
        if !names.insert(name) {
            return Err(format!(
                "duplicate voltage impulse node '{}'",
                trace.node_name
            ));
        }
    }
    Ok(())
}

pub(crate) fn voltage_impulse_value_count(traces: Option<&[VoltageImpulseTrace]>) -> usize {
    traces.into_iter().flatten().fold(0usize, |count, trace| {
        count
            .saturating_add(trace.numeric_value_count())
            .saturating_add(1)
            .saturating_add(trace.node_name.len().div_ceil(std::mem::size_of::<Value>()))
    })
}
