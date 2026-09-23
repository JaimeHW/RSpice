//! Exact event views over the same waveform snapshots used by trial solves.
use super::*;
use rspice_veriloga_runtime::transport_delay::DelayEventOrder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopeSourceEvent<'a> {
    pub time: Value,
    pub source: &'a str,
    pub left_value: Value,
    pub right_value: Value,
    /// Lower bound on the first derivative that can jump. Zero permits a
    /// value jump; one certifies equal values but permits a slope change.
    /// None retains unknown continuity rather than inferring it from samples.
    pub derivative_order_lower_bound: Option<u32>,
}

impl PreparedSpectralEnvelope {
    /// Next exact modulation event strictly after `time`, within this run's
    /// stop time. Adjacent representable timestamps are never tolerance-merged.
    pub fn next_source_event_after(&self, time: Value) -> Result<Option<Value>, SimulationError> {
        self.validate_time(time)?;
        self.source_events.next_after(time, self.config.stop_time)
    }

    /// Borrowed, allocation-free views of every source owning this exact
    /// timestamp. RF carrier clocks are absent unless explicitly selected as
    /// slow sources. No consumed cursor is changed by querying this schedule.
    pub fn source_events_at(
        &self,
        time: Value,
    ) -> Result<impl Iterator<Item = EnvelopeSourceEvent<'_>>, SimulationError> {
        self.validate_time(time)?;
        Ok(self.source_events.at(time)?.iter().map(|event| {
            let (name, left_value, right_value) = match event.owner {
                PhysicalSourceOwner::Voltage(index) => {
                    let source = &self.circuit.voltage_sources;
                    (
                        source.names[index].as_str(),
                        source.transient_value_at_on_side(
                            index,
                            event.time,
                            SourceTimeSide::LeftLimit,
                        ),
                        source.transient_value_at_on_side(
                            index,
                            event.time,
                            SourceTimeSide::RightLimit,
                        ),
                    )
                }
                PhysicalSourceOwner::Current(index) => {
                    let source = &self.circuit.current_sources;
                    (
                        source.names[index].as_str(),
                        source.value_at_time_on_side(index, event.time, SourceTimeSide::LeftLimit),
                        source.value_at_time_on_side(index, event.time, SourceTimeSide::RightLimit),
                    )
                }
            };
            EnvelopeSourceEvent {
                time: event.time,
                source: name,
                left_value,
                right_value,
                derivative_order_lower_bound: match event.order {
                    DelayEventOrder::AtLeast(order) => Some(order),
                    DelayEventOrder::Unknown => None,
                },
            }
        }))
    }
}
