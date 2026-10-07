//! Resolved parameter values aligned with accepted analysis rows.
//!
//! Only bindings that differ from the original deck require storage. Capturing
//! values at execution time avoids replaying parameter expressions or random
//! draws when a frontend later evaluates measurements.

use crate::abort_signal::AbortSignal;
use crate::netlist::ParamContext;
use crate::{ComplexValue, SimulationError};
use std::collections::BTreeMap;

/// Compact resolved parameter overrides aligned with accepted analysis points.
#[derive(Debug, Clone, Default)]
pub(crate) struct MeasureParameterSeries {
    rows: usize,
    columns: BTreeMap<String, Vec<Option<ComplexValue>>>,
}

/// Parameter environment used while preparing a measurement waveform.
#[derive(Clone, Copy)]
pub(crate) struct MeasureParameters<'a> {
    pub base: &'a ParamContext,
    pub rows: Option<&'a MeasureParameterSeries>,
}

impl<'a> From<&'a ParamContext> for MeasureParameters<'a> {
    fn from(base: &'a ParamContext) -> Self {
        Self { base, rows: None }
    }
}

pub(crate) struct PendingParameterRow {
    values: Vec<(String, Option<ComplexValue>, Option<ComplexValue>)>,
    pub(crate) additional_values: usize,
}

fn same(left: Option<ComplexValue>, right: Option<ComplexValue>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            left.re.to_bits() == right.re.to_bits() && left.im.to_bits() == right.im.to_bits()
        }
        (None, None) => true,
        _ => false,
    }
}

impl MeasureParameterSeries {
    /// Number of accepted rows represented by this snapshot.
    pub(crate) fn point_count(&self) -> usize {
        self.rows
    }

    /// Number of retained real/imaginary scalar slots.
    pub(crate) fn retained_value_count(&self) -> usize {
        self.columns.values().fold(0usize, |count, values| {
            count.saturating_add(values.len().saturating_mul(2))
        })
    }

    /// Resolve an override at the exact accepted row. An absent column uses
    /// the original deck; an absent value in an existing column is undefined.
    pub(crate) fn resolve(&self, name: &str, row: usize) -> Result<Option<ComplexValue>, String> {
        if row >= self.point_count() {
            return Err(format!(
                "parameter '{name}' is unavailable at analysis row {row}"
            ));
        }
        let Some(column) = self.columns.get(&name.to_uppercase()) else {
            return Ok(None);
        };
        column
            .get(row)
            .copied()
            .flatten()
            .map(Some)
            .ok_or_else(|| format!("parameter '{name}' is unavailable at analysis row {row}"))
    }

    pub(crate) fn prepare_row(
        &self,
        original: &ParamContext,
        current: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<PendingParameterRow, SimulationError> {
        super::super::measure::continuous::poll(abort, 0)?;
        let mut pending = PendingParameterRow {
            values: Vec::new(),
            additional_values: 0,
        };
        for (index, name) in self.columns.keys().enumerate() {
            super::super::measure::continuous::poll(abort, index)?;
            pending.values.push((
                name.clone(),
                original.get_complex(name),
                current.get_complex(name),
            ));
            pending.additional_values = pending.additional_values.saturating_add(2);
        }
        let mut names = std::collections::BTreeSet::new();
        for (index, (name, _)) in original
            .resolved_numeric_parameters()
            .chain(current.resolved_numeric_parameters())
            .enumerate()
        {
            super::super::measure::continuous::poll(abort, index)?;
            names.insert(name);
        }
        for (index, name) in names.into_iter().enumerate() {
            super::super::measure::continuous::poll(abort, index)?;
            let baseline = original.get_complex(name);
            let value = current.get_complex(name);
            if !self.columns.contains_key(name) && !same(baseline, value) {
                pending.values.push((name.to_owned(), baseline, value));
                pending.additional_values = pending
                    .additional_values
                    .saturating_add(self.rows.saturating_add(1).saturating_mul(2));
            }
        }
        Ok(pending)
    }

    /// Commit only after the row solver accepted a result and the caller
    /// admitted `additional_values` against its aggregate result budget.
    pub(crate) fn commit_row(
        &mut self,
        pending: PendingParameterRow,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (index, (name, baseline, value)) in pending.values.into_iter().enumerate() {
            super::super::measure::continuous::poll(abort, index)?;
            let column = self.columns.entry(name).or_default();
            let additional = self.rows.saturating_add(1).saturating_sub(column.len());
            column
                .try_reserve(additional)
                .map_err(|source| SimulationError::Allocation {
                    object: "measurement parameter series",
                    source,
                })?;
            while column.len() < self.rows {
                super::super::measure::continuous::poll(abort, column.len())?;
                column.push(baseline);
            }
            column.push(value);
        }
        self.rows += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAbort;

    #[test]
    fn missing_binding_does_not_fall_back_to_original_value() {
        let mut original = ParamContext::new();
        original.set("P", 1.0);
        let mut current = original.clone();
        current.set_string("P", "changed type");
        let mut series = MeasureParameterSeries::default();
        let pending = series.prepare_row(&original, &current, &NoAbort).unwrap();
        assert_eq!(pending.additional_values, 2);
        series.commit_row(pending, &NoAbort).unwrap();
        assert!(series.resolve("p", 0).is_err());
        assert_eq!(series.resolve("unmodified", 0).unwrap(), None);
    }

    #[test]
    fn parameter_capture_honors_cancellation_even_without_bindings() {
        let context = ParamContext::new();
        let abort = crate::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            MeasureParameterSeries::default().prepare_row(&context, &context, &abort),
            Err(SimulationError::Aborted)
        ));
    }

    #[test]
    fn unmodified_bindings_still_require_an_accepted_row() {
        let mut original = ParamContext::new();
        original.set("P", 1.0);
        let mut series = MeasureParameterSeries::default();
        assert!(series.resolve("P", 0).is_err());
        let pending = series.prepare_row(&original, &original, &NoAbort).unwrap();
        series.commit_row(pending, &NoAbort).unwrap();
        assert_eq!(series.resolve("P", 0).unwrap(), None);
        assert!(series.resolve("P", 1).is_err());
    }
}
