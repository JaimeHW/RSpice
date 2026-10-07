//! Compact coordinates for table-driven frequency results.
use crate::analysis::{AcResult, NoiseResult};
use crate::{ModelFinish, Value};

/// Resolved meaning of a frequency-table column. Declared parameters take
/// precedence over identically named devices; names here are canonical uppercase.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrequencyDataTarget {
    Frequency,
    Parameter(String),
    DeviceParameter {
        device_name: String,
        parameter_name: String,
    },
}

/// An authored column and its values aligned with the returned analysis points.
#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyDataColumn {
    pub name: String,
    pub target: FrequencyDataTarget,
    pub values: Vec<Value>,
}

/// One table run, retaining source order even for repeated or decreasing frequencies.
///
/// Coordinates contain only accepted rows. A model-requested finish may return a
/// prefix, identified by `finish` and `requested_rows`; cancellation and failures
/// return an error instead of publishing a partial table. No row netlists are held.
#[derive(Debug, Clone)]
pub struct FrequencyDataResult<T> {
    pub table_name: String,
    pub columns: Vec<FrequencyDataColumn>,
    pub points: Vec<T>,
    pub requested_rows: usize,
    pub finish: Option<ModelFinish>,
}

impl<T> FrequencyDataResult<T> {
    pub(super) fn value_count(&self, count: impl Fn(&T) -> usize) -> usize {
        self.points.iter().fold(
            self.columns
                .iter()
                .fold(0usize, |n, c| n.saturating_add(c.values.len())),
            |n, point| n.saturating_add(count(point)),
        )
    }
}

impl FrequencyDataResult<AcResult> {
    /// Numeric storage, counting each complex component and table coordinate.
    pub fn retained_value_count(&self) -> usize {
        self.value_count(AcResult::retained_value_count)
    }
}

impl FrequencyDataResult<NoiseResult> {
    /// Numeric storage, including spectra, contributions and table coordinates.
    pub fn retained_value_count(&self) -> usize {
        self.value_count(NoiseResult::retained_value_count)
    }
}
