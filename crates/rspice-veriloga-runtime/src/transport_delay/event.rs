use super::*;

/// A conservative lower bound on the first discontinuous derivative of a
/// physical input event. Order zero permits a value jump; positive order
/// requires equal values. The solver, not sampled interpolation data, owns
/// the derivative/smoothness argument. Unknown never certifies smoothness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelayEventOrder {
    Unknown,
    AtLeast(u32),
}

impl DelayEventOrder {
    /// Simultaneous events retain the least-smooth contributor, with unknown
    /// provenance kept explicit. No order cutoff is imposed here.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::AtLeast(a), Self::AtLeast(b)) => Self::AtLeast(a.min(b)),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayEvent {
    pub left: f64,
    pub right: f64,
    pub order: DelayEventOrder,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayEventArrival {
    pub time: f64,
    pub order: DelayEventOrder,
}

impl DelayBuffer {
    pub(super) fn event_sample(
        &self,
        time: f64,
        event: DelayEvent,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayCandidate, String> {
        if !event.left.is_finite() {
            return Err("delay left limit must be finite".into());
        }
        let mut sample = self.direct_sample(time, event.right, delay, max_delay)?;
        sample.left_limit = Some(event.left);
        sample.event_order = match event.order {
            DelayEventOrder::Unknown => None,
            DelayEventOrder::AtLeast(order) => {
                if order > 0 && event.left != event.right {
                    return Err(
                        "positive delay event order requires equal left and right values".into(),
                    );
                }
                Some(order)
            }
        };
        self.validate_candidate_commit(sample, time)?;
        Ok(sample)
    }

    /// Preflight physical event limits and declared order without changing
    /// accepted history or an existing speculative VM candidate.
    pub fn validate_event(
        &self,
        time: f64,
        event: DelayEvent,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        self.event_sample(time, event, delay, max_delay).map(|_| ())
    }

    /// Atomically append a physical event and its explicit order. Higher
    /// derivatives are certified by the owning solver; this buffer checks
    /// value consistency and preserves the metadata without re-inferring it.
    pub fn accept_event(
        &mut self,
        time: f64,
        event: DelayEvent,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        let sample = self.event_sample(time, event, delay, max_delay)?;
        self.candidate = Some(sample);
        self.apply_validated_commit();
        Ok(())
    }

    pub fn accepted_event_orders(
        &self,
    ) -> impl DoubleEndedIterator<Item = (f64, u32)> + ExactSizeIterator + '_ {
        self.event_orders.iter().copied()
    }

    /// Next physical arrival, merging order bounds of every event assigned
    /// to that representable clock. TwoSum/TwoDiff ownership remains the same
    /// as the sided history reads. Ordinary knots create no arrival.
    pub fn next_event_after(&self, time: f64) -> Result<Option<DelayEventArrival>, String> {
        let Some(arrival) = self.next_discontinuity_after(time)? else {
            return Ok(None);
        };
        let Some(DelayConfiguration::Fixed { delay }) = self.configuration else {
            unreachable!("fixed arrival validated by next_discontinuity_after")
        };
        let previous = DelayTarget::new(time, delay);
        let target = DelayTarget::new(arrival, delay);
        let first = self
            .left_limits
            .partition_point(|record| previous.at_or_after(record.0));
        let end = self
            .left_limits
            .partition_point(|record| target.at_or_after(record.0));
        if first == end {
            return Err("delay arrival has no owned physical event".into());
        }
        let first_time = self.left_limits[first].0;
        let last_time = self.left_limits[end - 1].0;
        let order_first = self
            .event_orders
            .partition_point(|record| record.0 < first_time);
        let order_end = self
            .event_orders
            .partition_point(|record| record.0 <= last_time);
        // Known-order records are a validated subset of event records. A
        // missing member makes the whole represented group unknown, without
        // scanning it or performing a binary lookup for every event.
        let order = if order_end - order_first != end - first {
            DelayEventOrder::Unknown
        } else {
            DelayEventOrder::AtLeast(
                self.event_orders
                    .range(order_first..order_end)
                    .map(|record| record.1)
                    .min()
                    .expect("nonempty event group"),
            )
        };
        Ok(Some(DelayEventArrival {
            time: arrival,
            order,
        }))
    }

    pub(super) fn validate_event_orders(checkpoint: &DelayCheckpoint) -> Result<(), String> {
        let mut previous = None;
        for &(time, order) in &checkpoint.event_orders {
            if !time.is_finite() || previous.is_some_and(|old| time <= old) {
                return Err(
                    "delay event orders must have finite, strictly increasing times".into(),
                );
            }
            let index = checkpoint
                .left_limits
                .partition_point(|record| record.0 < time);
            let Some(&(left_time, left)) = checkpoint.left_limits.get(index) else {
                return Err("delay event order has no accepted left limit".into());
            };
            if left_time.to_bits() != time.to_bits() {
                return Err("delay event order must match an accepted left limit".into());
            }
            let index = checkpoint.samples.partition_point(|record| record.0 < time);
            let right = checkpoint.samples[index].1; // Left-limit validation already established ownership.
            if order > 0 && left != right {
                return Err(
                    "positive delay event order requires equal left and right values".into(),
                );
            }
            previous = Some(time);
        }
        Ok(())
    }
}
