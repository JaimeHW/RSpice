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

#[derive(Debug, Clone)]
pub struct NetlistEnvelopeEvent {
    pub state: NetlistEnvelopeState,
    /// Coulombs in original MNA coordinates; non-source rows are empty.
    pub current_impulses: Vec<Vec<Complex64>>,
    /// Slow coordinate derivatives; ideal-source rows are empty.
    pub slow_rates: Vec<Vec<Complex64>>,
}

impl PreparedSpectralEnvelope {
    pub(super) fn require_outgoing_event(
        &self,
        state: &NetlistEnvelopeState,
    ) -> Result<(), SimulationError> {
        if state.source_side == EnvelopeSourceSide::LeftLimit
            && self.source_events_at(state.time())?.next().is_some()
        {
            return Err(invalid(
                "incoming source event must be transitioned before advancing",
            ));
        }
        Ok(())
    }

    /// Project an incoming modulation event while retaining charge/flux history,
    /// then reconstruct the outgoing finite currents and slow rates separately.
    pub fn transition_event_with_abort(
        &mut self,
        previous: &NetlistEnvelopeState,
        config: &SpectralEnvelopeEventConfig,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeEvent, SimulationError> {
        check_abort(abort)?;
        self.validate_state(previous)?;
        if previous.source_side != EnvelopeSourceSide::LeftLimit
            || self.source_events_at(previous.time())?.next().is_none()
        {
            return Err(invalid(
                "transition requires the incoming side of an exact modulation event",
            ));
        }
        let unknowns = self.carrier_sources.len();
        let mut limits = source_workspace_limits(unknowns, self.grid(), &self.limits)
            .map_err(numerical_error)?;
        let retained = unknowns
            .saturating_mul(16)
            .saturating_add(1)
            .saturating_add(unknowns.saturating_mul(self.grid().len()).saturating_mul(2));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            retained,
            limits.max_result_values,
        )?;
        limits.max_result_values -= retained;
        let rows = event_topology::prepare(&self.circuit, unknowns, abort)?;
        let sources = self
            .sources_at(previous.time(), EnvelopeSourceSide::RightLimit, abort)
            .map_err(numerical_error)?;
        let mut rates = vec![vec![Complex64::ZERO; self.grid().len()]; unknowns];
        let dc = self.grid().dc_index();
        for (source, name) in self.slow_sources.iter().zip(self.modulation_sources()) {
            check_abort(abort)?;
            let slope = if source.voltage {
                self.circuit.voltage_sources.time_derivative_at_on_side(
                    source.index,
                    previous.time(),
                    1,
                    SourceTimeSide::RightLimit,
                )
            } else {
                self.circuit.current_sources.time_derivative_at_on_side(
                    source.index,
                    previous.time(),
                    1,
                    SourceTimeSide::RightLimit,
                )
            }
            .filter(|value| value.is_finite())
            .ok_or_else(|| {
                invalid(format!(
                    "modulation source '{name}' has no finite outgoing slope"
                ))
            })?;
            for &(row, sign) in &source.rows {
                rates[row][dc].re += sign * slope;
                if !rates[row][dc].re.is_finite() {
                    return Err(invalid("outgoing source slopes overflowed"));
                }
            }
        }
        let event = self
            .solver
            .transition_spectral_envelope_with_abort(
                &previous.numerical,
                &rows,
                config,
                &sources,
                &rates,
                &limits,
                abort,
            )
            .map_err(numerical_error)?;
        Ok(NetlistEnvelopeEvent {
            state: self.bind(event.state, EnvelopeSourceSide::RightLimit),
            current_impulses: event.current_impulses,
            slow_rates: event.slow_rates,
        })
    }

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
