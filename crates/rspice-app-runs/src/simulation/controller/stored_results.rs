//! What the app says about the results stores its transients write: a run its volume or its
//! plan's storage budget stopped, and a stored column a waveform could not read.

use rspice_app_state::workbench::app_state::diagnostic_store::run_notices::storage_stop_message;

use super::*;
use crate::state::{ResultsStorageBudget, ResultsStorageStop};

impl SimulationController {
    /// The stop a transient's results store names, as the run keeps it, and the store remembered
    /// as `analysis`'s of `run_sequence`, so a column of it that cannot be read is said there.
    pub(super) fn note_results_store(
        &mut self,
        result: &crate::simulation::SimulationResult,
        analysis: &str,
        run_sequence: Option<u64>,
    ) -> Option<ResultsStorageStop> {
        let receipt = result.transient_store()?;
        if let Some(run_sequence) = run_sequence {
            self.stored_segments.insert(
                receipt.path.display().to_string(),
                (run_sequence, analysis.to_owned()),
            );
        }
        let stop = receipt.stopped.as_ref()?;
        Some(ResultsStorageStop {
            analysis: analysis.to_owned(),
            at: stop.at,
            rows: receipt.rows,
            bytes: receipt.bytes,
            free: stop.free,
            folder: receipt
                .path
                .parent()
                .unwrap_or(&receipt.path)
                .display()
                .to_string(),
            budget: stop.budget.map(|reached| ResultsStorageBudget {
                budget: reached.budget,
                written: reached.written,
                point: self.current_run_set_point(),
            }),
        })
    }

    /// The Run Set point the current task solves, as a results table names it: its corner,
    /// supply and temperature, then each value the Run Set swept.
    fn current_run_set_point(&self) -> Option<String> {
        let point = self.current_provenance.as_ref()?.pvt_point()?;
        let mut label = point.label();
        for swept in point.swept_values() {
            label.push_str(&format!(" \u{b7} {} = {}", swept.name, swept.value));
        }
        Some(label)
    }

    /// Say that the volume holding the run's results store filled, or its results reached its
    /// plan's storage budget, and stopped `stop.analysis`: one line in the run's log, one finding
    /// in its batch, and the run marked so the status bar says how it ended. The rows it kept
    /// stand as its result.
    pub(super) fn report_results_storage_stop(
        &self,
        state: &mut impl RunHost,
        stop: &ResultsStorageStop,
        run_sequence: u64,
    ) {
        self.run_line(
            state.model_mut(),
            LogSeverity::Error,
            format!("{}: {}", stop.analysis, storage_stop_message(stop)),
        );
        let facts = state.slot_facts();
        let model = state.model_mut();
        model.publish_results_storage_stop(stop, run_sequence, facts);
        if let Some(run) = model.simulation.retained.run_by_sequence_mut(run_sequence) {
            run.stop_on_results_storage(stop.clone());
        }
    }

    /// Say each stored column a waveform could not read since the last frame: in the log and
    /// findings of the run that wrote it while that run is kept, and on the status bar.
    pub(super) fn report_unreadable_columns(&mut self, state: &mut impl RunHost) {
        let failures = rspice_results::waveform::take_column_read_failures();
        self.report_column_failures(state, &failures);
    }

    fn report_column_failures(
        &mut self,
        state: &mut impl RunHost,
        failures: &[rspice_results::waveform::ColumnReadFailure],
    ) {
        let Some(last) = failures.last() else {
            return;
        };
        let prompt = match failures.len() {
            1 => format!(
                "The stored samples of {} could not be read; see Problems",
                last.column
            ),
            count => format!("{count} stored waveforms could not be read; see Problems"),
        };
        let facts = state.slot_facts();
        for failure in failures {
            let model = state.model_mut();
            let owner = self
                .stored_segments
                .get(&failure.store)
                .filter(|(run, _)| model.simulation.retained.run_by_sequence(*run).is_some());
            let line = format!("Results: {failure}");
            match owner {
                Some((run_sequence, analysis)) => {
                    write_run_line(model, Some(*run_sequence), LogSeverity::Error, line);
                    model.publish_unreadable_column(
                        analysis,
                        &failure.column,
                        &failure.cause,
                        *run_sequence,
                        facts,
                    );
                }
                None => {
                    model.record(LogSeverity::Error, crate::diagnostics::LogSource::Run, line);
                }
            }
        }
        let retained = &state.model().simulation.retained;
        self.stored_segments
            .retain(|_, (run, _)| retained.run_by_sequence(*run).is_some());
        state.prompt(prompt);
    }
}

#[cfg(test)]
mod tests;
