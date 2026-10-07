//! Admission before extending retained measurement and run-report collections.

use super::{MeasurementReport, SimulationReport};
use crate::cli::CliError;
use std::sync::atomic::{AtomicUsize, Ordering};

/// One report collection's numeric payload. Parallel batches share this counter
/// before returning outcomes to their collector, so admission precedes retention.
pub(crate) struct ReportValueBudget {
    retained: AtomicUsize,
    limit: usize,
}

impl ReportValueBudget {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            retained: AtomicUsize::new(0),
            limit,
        }
    }

    /// A concrete report also retains one duration, even without measurements.
    pub(crate) fn for_report(limit: usize) -> Result<Self, CliError> {
        let budget = Self::new(limit);
        budget.admit_values(1)?;
        Ok(budget)
    }

    fn admit_values(&self, additional: usize) -> Result<(), CliError> {
        self.retained
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |retained| {
                retained
                    .checked_add(additional)
                    .filter(|count| *count <= self.limit)
            })
            .map(|_| ())
            .map_err(|retained| CliError::CoreSimulationError {
                source: rspice_core::SimulationError::ResourceLimit(
                    rspice_core::ResourceLimitError {
                        resource: rspice_core::ResourceKind::ResultValues,
                        requested: retained.saturating_add(additional),
                        limit: self.limit,
                    },
                ),
                analysis: Some("Report retention".into()),
            })
    }

    pub(crate) fn admit_measurements(&self, rows: &[MeasurementReport]) -> Result<(), CliError> {
        self.admit_values(rows.iter().fold(0usize, |count, row| {
            count.saturating_add(row.retained_value_count())
        }))
    }

    pub(crate) fn admit_reports(&self, reports: &[SimulationReport]) -> Result<(), CliError> {
        self.admit_values(reports.iter().fold(0usize, |count, report| {
            report
                .measurements
                .iter()
                .fold(count.saturating_add(1), |count, row| {
                    count.saturating_add(row.retained_value_count())
                })
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_admission_does_not_consume_capacity() {
        let budget = ReportValueBudget::new(5);
        budget.admit_values(3).unwrap();
        assert!(budget.admit_values(3).is_err());
        budget.admit_values(2).unwrap();
        assert!(budget.admit_values(1).is_err());
    }

    #[test]
    fn parallel_admission_cannot_exceed_the_shared_limit() {
        let budget = ReportValueBudget::new(200);
        let accepted = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| budget.admit_values(100).is_ok()))
                .collect();
            workers
                .into_iter()
                .map(|worker| worker.join().unwrap())
                .filter(|accepted| *accepted)
                .count()
        });
        assert_eq!(accepted, 2);
        assert!(budget.admit_values(1).is_err());
    }
}
