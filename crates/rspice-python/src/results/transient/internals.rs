//! Helper bodies behind the transient result's `#[pymethods]` block.
//!
//! PyO3 permits one `#[pymethods]` block per type, so every Python-facing
//! signature and its docstring lives in `mod.rs` and everything longer than a
//! delegation lives here, as a second inherent impl. `engine/internals.rs` is
//! the same split for the same reason.
//!
//! Both helpers are shared by several of those signatures: the column layout
//! is what `export_columns`, `to_csv` and the two raw exporters agree on, and
//! the Fourier evaluation is what `fourier`, `fourier_of` and
//! `fourier_current` all reach.

use super::*;

impl PyTransientResult {
    /// Column layout shared by the CSV and raw exporters.
    pub(super) fn raw_plot(&self, title: &str) -> crate::export::RawPlot {
        use crate::export::{RawVariable, RawVariableKind};
        let real = |values: &[f64]| -> Vec<rspice_core::Complex64> {
            values
                .iter()
                .map(|value| rspice_core::Complex64::new(*value, 0.0))
                .collect()
        };

        let mut variables = vec![RawVariable {
            name: "time".to_string(),
            kind: RawVariableKind::Time,
        }];
        let mut series = vec![real(&self.inner.time)];
        for (index, name) in self.inner.node_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(real(
                self.inner
                    .voltages
                    .get(index)
                    .map_or(&[][..], Vec::as_slice),
            ));
        }
        for (index, name) in self.inner.branch_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(real(
                self.inner
                    .branch_currents
                    .get(index)
                    .map_or(&[][..], Vec::as_slice),
            ));
        }

        crate::export::RawPlot {
            title: title.to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables,
            series,
            complex: false,
            timestamp: None,
        }
    }

    /// Shared `.FOUR` evaluation for every waveform source.
    ///
    /// Qualifying and transforming a long waveform is unbounded work, so it
    /// runs on the interruptible worker: `KeyboardInterrupt` stops it and the
    /// GIL is released while it runs. A result object owns no engine, so the
    /// run is not registered with one — `Engine.cancel_all()` does not reach
    /// post-processing of an already-returned result.
    ///
    /// The sample grid is borrowed rather than copied across that release,
    /// which is sound because this class exposes no mutating method: there is
    /// no `&mut self` entry point and no `__setstate__`, so nothing Python
    /// can call while the worker runs can move the values out from under it.
    pub(super) fn fourier_of_waveform(
        &self,
        py: Python<'_>,
        output: &str,
        waveform: &[f64],
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "fundamental must be a positive finite frequency in Hz, got {fundamental}"
            )));
        }
        if num_harmonics == 0 {
            return Err(crate::errors::value_error(
                "num_harmonics must be at least 1",
            ));
        }
        let analysis =
            FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(num_harmonics));
        let time = self.inner.time.as_slice();
        let qualified = crate::abort::run_interruptible_unregistered(py, |abort| {
            match analysis.analyze_with_abort(time, waveform, abort) {
                // Cancellation is the worker's business; every other outcome
                // is this waveform's own and stays a value error below.
                Err(FourierError::Aborted) => Err(rspice_core::SimulationError::Aborted),
                outcome => Ok(outcome),
            }
        })?;
        let result = qualified.map_err(|error| {
            crate::errors::value_error(format!("Fourier waveform could not be analyzed: {error}"))
        })?;
        Ok(PyFourierResult::from_core(&result).with_output(output))
    }
}
