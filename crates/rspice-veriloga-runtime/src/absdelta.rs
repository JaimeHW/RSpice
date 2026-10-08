//! Interpolated A2D observations for VAMS-2023 sections 5.10.3.4 and 8.4.6.
//!
//! An observer does not request analog breakpoints. The caller supplies an
//! accepted history and a candidate interval, merges its events with the other
//! digital work, and commits the returned state only when the coupled trial is
//! accepted. A D2A consequence invalidates the unconsumed portion of the interval.
//! No analog expression is evaluated here; all samples come from the owning
//! analog evaluation. Event enumeration is lazy and allocates no event queue.

/// One physical analog sample; time is in seconds, without HDL quantization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbsDeltaSample {
    pub time: f64,
    pub value: f64,
}

/// Evaluated analog operands. Omitted or zero tolerances select runtime defaults.
#[derive(Debug, Clone, Copy)]
pub struct AbsDeltaControls {
    pub delta: f64,
    pub time_tolerance: f64,
    pub expression_tolerance: f64,
    pub enable: f64,
}

/// Accepted observer history. Copying it is sufficient to stage or reject a trial.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AbsDeltaState {
    sample: Option<AbsDeltaSample>,
    event: Option<AbsDeltaSample>,
    enabled: bool,
    direction: i8,
    extremum: f64,
    /// A real expression change exceeded delta while time_tol suppressed delivery.
    pending: bool,
}

impl AbsDeltaState {
    pub fn last_sample(self) -> Option<AbsDeltaSample> {
        self.sample
    }

    pub fn last_event(self) -> Option<AbsDeltaSample> {
        self.event
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsDeltaReason {
    Initialization,
    Enabled,
    Delta,
    Direction,
    ValueChange,
}

/// An event and the observer history at that physical instant. The caller must
/// still validate/accept the analog solution before promoting this history.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbsDeltaEvent {
    pub sample: AbsDeltaSample,
    pub reason: AbsDeltaReason,
    pub state: AbsDeltaState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsDeltaError {
    InvalidExpression,
    InvalidTime,
    BackwardTime,
    InvalidDelta,
    InvalidTimeTolerance,
    InvalidExpressionTolerance,
    InvalidTimePrecision,
    InvalidEnable,
    UnrepresentableEvent,
}

impl std::fmt::Display for AbsDeltaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidExpression => "absdelta expression must be finite",
            Self::InvalidTime => "absdelta time must be finite and non-negative",
            Self::BackwardTime => "absdelta candidate precedes its accepted sample",
            Self::InvalidDelta => "absdelta delta must be finite and non-negative",
            Self::InvalidTimeTolerance => "absdelta time tolerance must be finite and non-negative",
            Self::InvalidExpressionTolerance => {
                "absdelta expression tolerance must be finite and non-negative"
            }
            Self::InvalidTimePrecision => "absdelta HDL time precision must be finite and positive",
            Self::InvalidEnable => "absdelta enable must be a finite 32-bit integer",
            Self::UnrepresentableEvent => {
                "absdelta event cannot advance at the available numeric precision"
            }
        })
    }
}

impl std::error::Error for AbsDeltaError {}

/// Speculative events along one linear analog interpolation interval.
///
/// Controls describe this interval; the caller splits at known control changes.
/// `initializing` is true for initialization and each DC sweep point. The HDL
/// precision bounds the effective time tolerance, but never rounds physical
/// event times. The caller applies its event budget while calling `next_event`.
#[derive(Debug, Clone)]
pub struct AbsDeltaInterval {
    state: AbsDeltaState,
    endpoint: AbsDeltaSample,
    delta: f64,
    time_tolerance: f64,
    expression_tolerance: f64,
    enabled: bool,
    immediate: Option<AbsDeltaReason>,
    done: bool,
}

impl AbsDeltaInterval {
    pub fn new(
        accepted: AbsDeltaState,
        endpoint: AbsDeltaSample,
        controls: AbsDeltaControls,
        time_precision: f64,
        initializing: bool,
    ) -> Result<Self, AbsDeltaError> {
        if !endpoint.value.is_finite() {
            return Err(AbsDeltaError::InvalidExpression);
        }
        if !nonnegative(endpoint.time) {
            return Err(AbsDeltaError::InvalidTime);
        }
        if !initializing
            && accepted
                .sample
                .is_some_and(|point| endpoint.time < point.time)
        {
            return Err(AbsDeltaError::BackwardTime);
        }
        if !nonnegative(controls.delta) {
            return Err(AbsDeltaError::InvalidDelta);
        }
        if !nonnegative(controls.time_tolerance) {
            return Err(AbsDeltaError::InvalidTimeTolerance);
        }
        if !nonnegative(controls.expression_tolerance) {
            return Err(AbsDeltaError::InvalidExpressionTolerance);
        }
        if !time_precision.is_finite() || time_precision <= 0.0 {
            return Err(AbsDeltaError::InvalidTimePrecision);
        }
        if !controls.enable.is_finite()
            || controls.enable.fract() != 0.0
            || controls.enable < i32::MIN as f64
            || controls.enable > i32::MAX as f64
        {
            return Err(AbsDeltaError::InvalidEnable);
        }
        let enabled = controls.enable != 0.0;
        let immediate = if !enabled {
            None
        } else if initializing || accepted.sample.is_none() {
            Some(AbsDeltaReason::Initialization)
        } else if !accepted.enabled {
            Some(AbsDeltaReason::Enabled)
        } else {
            None
        };
        let scale = accepted.sample.map_or(endpoint.value.abs(), |sample| {
            sample.value.abs().max(endpoint.value.abs())
        });
        let expression_tolerance = if controls.expression_tolerance > 0.0 {
            controls.expression_tolerance
        } else {
            (64.0 * f64::EPSILON * scale).max(f64::MIN_POSITIVE)
        };
        Ok(Self {
            state: accepted,
            endpoint,
            delta: controls.delta,
            time_tolerance: controls.time_tolerance.max(time_precision),
            expression_tolerance,
            enabled,
            immediate,
            done: false,
        })
    }

    /// Candidate history is available only after all interval events have been
    /// consumed. Dropping the interval leaves the accepted input unchanged.
    pub fn candidate(&self) -> Option<AbsDeltaState> {
        self.done.then_some(self.state)
    }

    pub fn next_event(&mut self) -> Result<Option<AbsDeltaEvent>, AbsDeltaError> {
        if self.done {
            return Ok(None);
        }
        if let Some(reason) = self.immediate.take() {
            self.state.direction = 0;
            self.state.extremum = self.endpoint.value;
            return Ok(Some(self.emit(self.endpoint, reason)));
        }
        let Some(start) = self.state.sample else {
            self.finish();
            return Ok(None);
        };
        if !self.enabled
            || (start.value == self.endpoint.value && (!self.state.pending || self.delta == 0.0))
        {
            self.finish();
            return Ok(None);
        }
        if self.delta == 0.0 {
            // Value-change mode observes endpoints only and ignores tolerances.
            return Ok(Some(self.emit(self.endpoint, AbsDeltaReason::ValueChange)));
        }
        let direction = if self.endpoint.value > start.value {
            1
        } else if self.endpoint.value < start.value {
            -1
        } else {
            self.state.direction
        };
        if self.state.direction == 0 {
            self.state.direction = direction;
            self.state.extremum = start.value;
        }

        let reversal = if direction != 0
            && direction != self.state.direction
            && (self.endpoint.value - self.state.extremum).abs() >= self.expression_tolerance
        {
            let threshold = self.state.extremum + direction as f64 * self.expression_tolerance;
            Some(self.at_value(threshold, start)?)
        } else {
            None
        };
        let amplitude = self.amplitude_event(start, direction)?;
        let next = match (reversal, amplitude) {
            (Some(reversal), Some(amplitude)) if amplitude.time < reversal.time => {
                Some((amplitude, AbsDeltaReason::Delta))
            }
            (Some(reversal), _) => Some((reversal, AbsDeltaReason::Direction)),
            (None, Some(amplitude)) => Some((amplitude, AbsDeltaReason::Delta)),
            (None, None) => None,
        };
        if let Some((sample, reason)) = next {
            if reason == AbsDeltaReason::Direction {
                self.state.direction = direction;
                self.state.extremum = sample.value;
            }
            return Ok(Some(self.emit(sample, reason)));
        }
        self.finish();
        Ok(None)
    }

    fn amplitude_event(
        &self,
        start: AbsDeltaSample,
        direction: i8,
    ) -> Result<Option<AbsDeltaSample>, AbsDeltaError> {
        let event = self
            .state
            .event
            .expect("enabled observer has an initial event");
        let next_time = if start.time == self.endpoint.time {
            start.time
        } else {
            start.time.next_up()
        };
        let mut earliest = (event.time + self.time_tolerance).max(next_time);
        if earliest - event.time < self.time_tolerance {
            earliest = earliest.next_up();
        }
        if earliest > self.endpoint.time {
            return Ok(None);
        }
        let first = self.at_time(earliest, start);
        if (first.value - event.value).abs() >= self.delta {
            return Ok(Some(first));
        }
        let threshold = event.value + direction as f64 * self.delta;
        if !threshold.is_finite()
            || (direction > 0 && threshold > self.endpoint.value)
            || (direction < 0 && threshold < self.endpoint.value)
        {
            return Ok(None);
        }
        if threshold == event.value {
            return Err(AbsDeltaError::UnrepresentableEvent);
        }
        let point = self.at_value(threshold, start)?;
        Ok((point.time >= earliest).then_some(point))
    }

    fn at_value(&self, value: f64, start: AbsDeltaSample) -> Result<AbsDeltaSample, AbsDeltaError> {
        if start.time == self.endpoint.time {
            return Ok(self.endpoint);
        }
        let width = self.endpoint.value - start.value;
        let fraction = if width.is_finite() {
            (value - start.value) / width
        } else {
            // Opposite finite extremes may have an overflowing difference.
            let scale = self.endpoint.value.abs().max(start.value.abs());
            (value / scale - start.value / scale)
                / (self.endpoint.value / scale - start.value / scale)
        };
        let time = start.time + fraction.clamp(0.0, 1.0) * (self.endpoint.time - start.time);
        let time = time.max(start.time.next_up());
        if !time.is_finite() || time > self.endpoint.time {
            return Err(AbsDeltaError::UnrepresentableEvent);
        }
        Ok(self.at_time(time, start))
    }

    fn at_time(&self, time: f64, start: AbsDeltaSample) -> AbsDeltaSample {
        if time >= self.endpoint.time {
            return self.endpoint;
        }
        let fraction = (time - start.time) / (self.endpoint.time - start.time);
        let value = if (self.endpoint.value - start.value).is_finite() {
            start.value + fraction * (self.endpoint.value - start.value)
        } else {
            (1.0 - fraction) * start.value + fraction * self.endpoint.value
        };
        AbsDeltaSample { time, value }
    }

    fn emit(&mut self, sample: AbsDeltaSample, reason: AbsDeltaReason) -> AbsDeltaEvent {
        self.state.sample = Some(sample);
        self.state.event = Some(sample);
        self.state.enabled = true;
        self.state.pending = false;
        self.update_extremum(sample.value);
        AbsDeltaEvent {
            sample,
            reason,
            state: self.state,
        }
    }

    fn update_extremum(&mut self, value: f64) {
        self.state.extremum = match self.state.direction {
            1 => self.state.extremum.max(value),
            -1 => self.state.extremum.min(value),
            _ => value,
        };
    }

    fn finish(&mut self) {
        self.state.pending = self.enabled
            && self.delta > 0.0
            && (self.state.pending
                || self
                    .state
                    .sample
                    .is_some_and(|sample| sample.value != self.endpoint.value))
            && self
                .state
                .event
                .is_some_and(|event| (self.endpoint.value - event.value).abs() >= self.delta);
        self.state.sample = Some(self.endpoint);
        self.state.enabled = self.enabled;
        if !self.enabled {
            self.state.direction = 0;
        }
        self.update_extremum(self.endpoint.value);
        self.done = true;
    }
}

fn nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

#[cfg(test)]
mod tests;
