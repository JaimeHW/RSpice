use super::*;
use crate::device::MatrixStamper;

pub(in crate::engine::transient) struct EventStamp {
    pub(super) rows: Vec<Vec<(usize, Value)>>,
    pub(super) values: Vec<Value>,
    pub(super) scales: Vec<Value>,
    terms: usize,
    limit: usize,
    fault: Option<StampFault>,
}

enum StampFault {
    Index,
    Nonfinite,
    Resource(ResourceLimitError),
}

impl EventStamp {
    fn new(size: usize, options: &EventOptions) -> Self {
        Self {
            rows: vec![Vec::new(); size],
            values: vec![0.0; size],
            scales: vec![0.0; size],
            terms: 0,
            limit: options
                .limits
                .max_result_values
                .saturating_sub(size.saturating_mul(64))
                / 256,
            fault: None,
        }
    }

    fn validate(&self, size: usize) -> Result<()> {
        if let Some(failure) = &self.fault {
            return Err(match failure {
                StampFault::Index => error("physical stamp index outside the prepared matrix"),
                StampFault::Nonfinite => error("nonfinite physical stamp"),
                StampFault::Resource(failure) => (*failure).into(),
            });
        }
        if self.rows.len() != size
            || self.values.len() != size
            || self.scales.len() != size
            || !self
                .values
                .iter()
                .chain(&self.scales)
                .all(|value| value.is_finite())
        {
            return Err(error("invalid physical stamp"));
        }
        Ok(())
    }
}

impl MatrixStamper for EventStamp {
    fn stamp(&mut self, row: usize, column: usize, value: Value) {
        if self.fault.is_some() || row == 0 || column == 0 {
            return;
        }
        if row > self.rows.len() || column > self.rows.len() {
            self.fault = Some(StampFault::Index);
            return;
        }
        if !value.is_finite() {
            self.fault = Some(StampFault::Nonfinite);
            return;
        }
        if let Err(failure) = ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.terms.saturating_add(1).saturating_mul(256),
            self.limit.saturating_mul(256),
        ) {
            self.fault = Some(StampFault::Resource(failure));
            return;
        }
        self.rows[row - 1].push((column - 1, value));
        self.terms += 1;
    }
    fn stamp_rhs(&mut self, row: usize, value: Value) {
        if self.fault.is_some() || row == 0 {
            return;
        }
        if row > self.values.len() {
            self.fault = Some(StampFault::Index);
            return;
        }
        if !value.is_finite() {
            self.fault = Some(StampFault::Nonfinite);
            return;
        }
        match sum([(self.values[row - 1], 1.0), (value, -1.0)].into_iter()) {
            Ok(total) => self.values[row - 1] = total,
            Err(_) => self.fault = Some(StampFault::Nonfinite),
        }
        self.scales[row - 1] = self.scales[row - 1].max(value.abs());
    }
}

/// F and Q exclude ideal voltage-source currents and their equations.
/// The prepared owner must certify this structural independence, not infer
/// it from a zero derivative at one bias. Its history endpoint is immutable.
pub(in crate::engine::transient) struct EventSample {
    pub f: EventStamp,
    pub q: EventStamp,
    /// Explicit physical time partials at fixed event coordinates/history.
    pub f_time: Vec<Value>,
    pub q_time: Vec<Value>,
}

impl EventSample {
    pub(in crate::engine::transient) fn charge_values(&self) -> &[Value] {
        &self.q.values
    }

    pub(super) fn nonfinite(&self, size: usize) -> Result<bool> {
        // Domain-invalid Newton probes may backtrack, but a simultaneous
        // structural/resource fault must still reach the caller immediately.
        for stamp in [&self.f, &self.q] {
            match &stamp.fault {
                Some(StampFault::Index) => {
                    return Err(error("physical stamp index outside the prepared matrix"));
                }
                Some(StampFault::Resource(failure)) => return Err((*failure).into()),
                _ => {}
            }
            if stamp.rows.len() != size || stamp.values.len() != size || stamp.scales.len() != size
            {
                return Err(error("invalid physical stamp dimensions"));
            }
        }
        if self.f_time.len() != size || self.q_time.len() != size {
            return Err(error("invalid explicit-time physical dimensions"));
        }
        Ok([&self.f, &self.q]
            .into_iter()
            .any(|stamp| matches!(stamp.fault, Some(StampFault::Nonfinite)))
            || self
                .f_time
                .iter()
                .chain(&self.q_time)
                .any(|value| !value.is_finite()))
    }
    pub(super) fn new(size: usize, options: &EventOptions) -> Result<Self> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            size.saturating_mul(64),
            options.limits.max_result_values,
        )?;
        Ok(Self {
            f: EventStamp::new(size, options),
            q: EventStamp::new(size, options),
            f_time: vec![0.0; size],
            q_time: vec![0.0; size],
        })
    }
    pub(super) fn validate(&self, size: usize) -> Result<()> {
        self.f.validate(size)?;
        self.q.validate(size)?;
        if self.f_time.len() != size
            || self.q_time.len() != size
            || !self
                .f_time
                .iter()
                .chain(&self.q_time)
                .all(|value| value.is_finite())
        {
            return Err(error("invalid explicit-time physical terms"));
        }
        Ok(())
    }
}
