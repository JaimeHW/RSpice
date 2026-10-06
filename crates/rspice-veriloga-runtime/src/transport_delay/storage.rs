//! Fallible storage operations for owners with an accepted-state barrier.

use super::*;
use std::collections::TryReserveError;

fn reserve<T>(values: &mut VecDeque<T>, additional: usize) -> Result<(), TryReserveError> {
    #[cfg(test)]
    if values.capacity().saturating_sub(values.len()) < additional {
        reservation_attempt()?;
    }
    values.try_reserve(additional)
}

fn copy_records<T: Copy>(source: &VecDeque<T>) -> Result<Vec<T>, TryReserveError> {
    let mut result = Vec::new();
    #[cfg(test)]
    if !source.is_empty() {
        reservation_attempt()?;
    }
    result.try_reserve_exact(source.len())?;
    result.extend(source.iter().copied());
    Ok(result)
}

impl DelayBuffer {
    /// Fallible counterpart of [`Self::new`]. The capacity is an allocation
    /// hint, still clamped to the per-site accepted-record ceiling.
    pub fn try_new(capacity: usize) -> Result<Self, TryReserveError> {
        let mut result = Self::new(0);
        reserve(&mut result.samples, capacity.min(MAX_DELAY_HISTORY_SAMPLES))?;
        Ok(result)
    }

    /// Reserve all storage a native accepted sample can append. Call this
    /// after validating the sample and before any device commits its state.
    /// `None` denotes an ordinary sample; an event retains a left limit and,
    /// when supplied, its derivative-order record. Only capacity may change
    /// on failure; accepted values and a staged VM candidate stay unchanged.
    pub fn try_reserve_sample(
        &mut self,
        event: Option<DelayEventOrder>,
    ) -> Result<(), TryReserveError> {
        reserve(&mut self.samples, 1)?;
        if event.is_some() {
            reserve(&mut self.left_limits, 1)?;
        }
        if matches!(event, Some(DelayEventOrder::AtLeast(_))) {
            reserve(&mut self.event_orders, 1)?;
        }
        Ok(())
    }

    /// Backing storage of retained records, including spare capacity.
    /// The inline buffer and any checkpoint copy are accounted for by owners.
    pub fn allocated_bytes(&self) -> usize {
        self.samples
            .capacity()
            .saturating_mul(std::mem::size_of::<(f64, f64)>())
            .saturating_add(
                self.left_limits
                    .capacity()
                    .saturating_mul(std::mem::size_of::<(f64, f64)>()),
            )
            .saturating_add(
                self.event_orders
                    .capacity()
                    .saturating_mul(std::mem::size_of::<(f64, u32)>()),
            )
    }

    /// Copy retained history without an infallible collection allocation.
    /// A staged candidate is preserved but is never included in a checkpoint.
    pub fn try_clone(&self) -> Result<Self, TryReserveError> {
        Ok(Self {
            samples: copy_records(&self.samples)?.into(),
            left_limits: copy_records(&self.left_limits)?.into(),
            event_orders: copy_records(&self.event_orders)?.into(),
            configuration: self.configuration,
            candidate: self.candidate,
        })
    }

    /// Capture accepted records with fallible allocation. As with
    /// [`Self::checkpoint`], a staged candidate is excluded.
    pub fn try_checkpoint(&self) -> Result<DelayCheckpoint, TryReserveError> {
        Ok(DelayCheckpoint {
            configuration: self.configuration,
            samples: copy_records(&self.samples)?,
            left_limits: copy_records(&self.left_limits)?,
            event_orders: copy_records(&self.event_orders)?,
        })
    }

    pub(super) fn reserve_restoration(
        &mut self,
        checkpoint: &DelayCheckpoint,
    ) -> Result<(), TryReserveError> {
        let samples = checkpoint.samples.len().saturating_sub(self.samples.len());
        let left = checkpoint
            .left_limits
            .len()
            .saturating_sub(self.left_limits.len());
        let orders = checkpoint
            .event_orders
            .len()
            .saturating_sub(self.event_orders.len());
        reserve(&mut self.samples, samples)?;
        reserve(&mut self.left_limits, left)?;
        reserve(&mut self.event_orders, orders)
    }
}

#[cfg(test)]
thread_local! {
    static FAIL_RESERVATION_AFTER: std::cell::Cell<Option<usize>> = const {
        std::cell::Cell::new(None)
    };
}

#[cfg(test)]
fn reservation_attempt() -> Result<(), TryReserveError> {
    FAIL_RESERVATION_AFTER.with(|remaining| {
        if let Some(count) = remaining.get() {
            if count == 0 {
                remaining.set(None);
                return Vec::<u8>::new().try_reserve(usize::MAX);
            }
            remaining.set(Some(count - 1));
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_and_copy_failures_are_values_and_preserve_the_source() {
        FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(0)));
        assert!(DelayBuffer::try_new(4).is_err());
        let mut source = DelayBuffer::try_new(4).unwrap();
        source
            .accept_event(
                0.0,
                DelayEvent {
                    left: 2.0,
                    right: 2.0,
                    order: DelayEventOrder::AtLeast(1),
                },
                10.0,
                None,
            )
            .unwrap();
        source.eval(1.0, 3.0, 10.0, None).unwrap();
        let original = source.clone();
        for failure in 0..3 {
            FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(failure)));
            assert!(source.try_clone().is_err());
            assert_eq!(source, original);
            FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(failure)));
            assert!(source.try_checkpoint().is_err());
            assert_eq!(source, original);
        }
        assert_eq!(source.try_clone().unwrap(), source);
        assert_eq!(source.try_checkpoint().unwrap(), source.checkpoint());
    }

    #[test]
    fn failed_event_append_preserves_the_prior_samples_and_staged_candidate() {
        let mut buffer = DelayBuffer::new(4);
        for time in 0..4 {
            buffer
                .accept_sample(f64::from(time), 1.0, 10.0, None)
                .unwrap();
        }
        buffer.eval(4.0, 3.0, 10.0, None).unwrap();
        for failure in 0..3 {
            let mut attempt = buffer.clone();
            FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(failure)));
            let error = attempt
                .accept_event(
                    4.0,
                    DelayEvent {
                        left: 2.0,
                        right: 2.0,
                        order: DelayEventOrder::AtLeast(1),
                    },
                    10.0,
                    None,
                )
                .unwrap_err();
            assert!(error.contains("allocation"), "{error}");
            assert_eq!(attempt, buffer);
        }
    }

    #[test]
    fn failed_event_reservation_preserves_accepted_and_candidate_state() {
        let mut buffer = DelayBuffer::try_new(0).unwrap();
        buffer.eval(0.0, 3.0, 10.0, None).unwrap();
        let before = buffer.clone();
        for failure in 0..3 {
            let mut attempt = before.clone();
            FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(failure)));
            assert!(
                attempt
                    .try_reserve_sample(Some(DelayEventOrder::AtLeast(1)))
                    .is_err()
            );
            assert_eq!(attempt, before);
        }
        buffer
            .try_reserve_sample(Some(DelayEventOrder::AtLeast(1)))
            .unwrap();
        let bytes = buffer.allocated_bytes();
        buffer
            .accept_event(
                0.0,
                DelayEvent {
                    left: 2.0,
                    right: 2.0,
                    order: DelayEventOrder::AtLeast(1),
                },
                10.0,
                None,
            )
            .unwrap();
        assert_eq!(buffer.allocated_bytes(), bytes);
        assert_eq!(buffer.try_checkpoint().unwrap(), buffer.checkpoint());
        assert_eq!(buffer.try_clone().unwrap(), buffer);
    }

    #[test]
    fn failed_restore_preserves_all_values_and_candidate() {
        let mut source = DelayBuffer::new(0);
        source
            .accept_event(
                0.0,
                DelayEvent {
                    left: 2.0,
                    right: 2.0,
                    order: DelayEventOrder::AtLeast(1),
                },
                10.0,
                None,
            )
            .unwrap();
        let checkpoint = source.checkpoint();
        let mut destination = DelayBuffer::new(0);
        destination.eval(0.0, 7.0, 20.0, None).unwrap();
        let before = destination.clone();
        FAIL_RESERVATION_AFTER.with(|remaining| remaining.set(Some(1)));
        assert!(
            destination
                .restore_checkpoint(&checkpoint)
                .unwrap_err()
                .contains("allocation")
        );
        assert_eq!(destination, before);
        destination.restore_checkpoint(&checkpoint).unwrap();
        assert_eq!(destination, source);
    }
}
