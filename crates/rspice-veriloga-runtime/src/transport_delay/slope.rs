//! Regular physical slopes of the reconstructed fixed-delay trajectory.

use super::*;

/// Side of a physical time boundary, excluding any jump impulse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelayTimeSide {
    Incoming,
    Outgoing,
}

impl DelayBuffer {
    /// Read the regular slope of the reconstructed delayed trajectory.
    ///
    /// `endpoint` closes the incoming history interval at `time`. At a local
    /// input jump, pass the independently solved input left limit, not the
    /// outgoing input. An already accepted endpoint uses its stored left
    /// limit. This call neither stages nor changes any sample.
    ///
    /// This is a trajectory slope, not a partial derivative of the Newton
    /// interpolation formula as its speculative endpoint time moves. It is
    /// also distinct from sensitivity to the frozen delay operand, which is
    /// zero. A variable or zero delay needs the owning input trajectory and
    /// is outside this fixed, strictly positive-delay contract.
    /// As with value reads, the first accepted fixed definition replaces
    /// subsequent values of the authored `delay` expression.
    ///
    /// At a represented arrival group the incoming slope ends at its first
    /// event's left limit. The outgoing slope uses the compensated physical
    /// target, which can already be beyond the last event in that group.
    pub fn fixed_trajectory_slope(
        &self,
        time: f64,
        endpoint: f64,
        delay: f64,
        side: DelayTimeSide,
    ) -> Result<f64, String> {
        self.validate_runtime(time, endpoint, delay, None)?;
        let (configuration, effective_delay, _) = self.resolve_configuration(delay, None)?;
        if !matches!(configuration, DelayConfiguration::Fixed { .. }) || effective_delay <= 0.0 {
            return Err("transport trajectory slope requires a positive fixed delay".into());
        }
        if self.samples.back().is_some_and(|sample| time < sample.0) {
            return Err("transport slope time precedes the latest accepted sample".into());
        }
        if self.samples.is_empty() {
            return if time == 0.0 {
                Ok(0.0)
            } else {
                Err("transport slope requires an accepted time-zero anchor".into())
            };
        }
        let incoming = side == DelayTimeSide::Incoming;
        if time < effective_delay || (incoming && time == effective_delay) {
            return Ok(0.0);
        }
        if incoming && let Some((at, left)) = self.arriving_event(time, effective_delay)? {
            if at == 0.0 {
                return Ok(0.0);
            }
            let upper = self.samples.partition_point(|sample| sample.0 < at);
            let previous = upper
                .checked_sub(1)
                .and_then(|index| self.samples.get(index));
            return previous
                .ok_or_else(|| {
                    "incoming transport slope requires a retained predecessor".to_owned()
                })
                .and_then(|&previous| regular_slope(previous, (at, left)));
        }

        let target = DelayTarget::new(time, effective_delay);
        let upper = self.samples.partition_point(|sample| {
            target.at_or_after(sample.0)
                && (!incoming || target.high != sample.0 || target.low != 0.0)
        });
        let left = upper
            .checked_sub(1)
            .and_then(|index| self.samples.get(index));
        let right = self
            .samples
            .get(upper)
            .map_or((time, endpoint), |&(at, value)| {
                (at, self.left_value(at).unwrap_or(value))
            });
        left.ok_or_else(|| "transport slope requires a retained interpolation bracket".to_owned())
            .and_then(|&left| regular_slope(left, right))
    }
}

fn regular_slope(left: (f64, f64), right: (f64, f64)) -> Result<f64, String> {
    if right.0 <= left.0 {
        return Err("transport slope interval must be strictly positive".into());
    }
    let result = sum_products_ratio(
        [(right.1, 1.0), (left.1, -1.0)].into_iter(),
        [(right.0, 1.0), (left.0, -1.0)].into_iter(),
    )
    .map_err(|error| format!("transport history slope is not representable: {error:?}"))?;
    if !result.is_finite() {
        return Err("transport history slope is not representable".into());
    }
    Ok(result)
}
