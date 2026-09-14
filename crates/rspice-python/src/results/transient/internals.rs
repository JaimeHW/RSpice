//! Helper bodies behind the transient result's `#[pymethods]` block.
//!
//! PyO3 permits one `#[pymethods]` block per type, so every Python-facing
//! signature and its docstring lives in `mod.rs` and everything longer than a
//! delegation lives here, as a second inherent impl. `engine/internals.rs` is
//! the same split for the same reason.
//!
//! Every helper here is shared by several of those signatures: the column
//! layout is what `export_columns`, `to_csv` and the two raw exporters agree
//! on, the raw bytes are what `to_raw` and `write_raw` both publish, the
//! Fourier evaluation is what `fourier`, `fourier_of` and `fourier_current`
//! all reach, and the authored-output projection is `saved_signals`.

use super::*;

/// The samples one export column carries, or `None` when the run did not
/// retain that column at all.
///
/// A column the run did not retain is not a variable of zero points: it is a
/// variable this export does not carry. The core's own flattened projection
/// skips it for exactly that reason, and the two exporters have to agree or a
/// rawfile of a mixed deck is refused by its own length check while the CSV
/// twin grows a column of NaN. An event-only net's voltage and an unsaved
/// node's voltage are both this case; the event net reaches the reader through
/// its trace. `sweeping` is false only for a result with no axis, where an
/// empty column is the whole run rather than an absent channel.
fn retained_column(column: Option<&Vec<f64>>, sweeping: bool) -> Option<&[f64]> {
    match column {
        Some(values) if values.is_empty() && sweeping => None,
        Some(values) => Some(values.as_slice()),
        None => Some(&[][..]),
    }
}

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
        let sweeping = !self.inner.time.is_empty();
        for (index, name) in self.inner.node_names.iter().enumerate() {
            let Some(values) = retained_column(self.inner.voltages.get(index), sweeping) else {
                continue;
            };
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(real(values));
        }
        for (index, name) in self.inner.branch_names.iter().enumerate() {
            let Some(values) = retained_column(self.inner.branch_currents.get(index), sweeping)
            else {
                continue;
            };
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(real(values));
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

    /// The SPICE raw bytes `to_raw` returns and `write_raw` writes.
    ///
    /// One body rather than two: the file and the in-memory bytes are the same
    /// export, and two copies of the title default, the timestamp assignment
    /// and the format parse are two chances for them to stop being.
    pub(super) fn raw_file_bytes(
        &self,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<Vec<u8>> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice transient analysis"));
        plot.timestamp = timestamp.map(str::to_string);
        raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )
    }

    /// The columns a deck's own output cards select, in authored order.
    pub(super) fn authored_signals(
        &self,
        netlist: &crate::netlist::PyNetlist,
    ) -> PyResult<Vec<crate::results::PyProjectedSignal>> {
        let inventory = rspice_core::execution::transient_projection_signals(&self.inner)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        let projection = rspice_core::execution::SignalProjection::from_netlist(&netlist.inner)
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        let ordered = projection
            .ordered_transient_columns(
                &netlist.inner,
                &self.inner,
                netlist.resource_limits,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        crate::results::projection::project_real(
            &netlist.inner,
            rspice_core::execution::AnalysisResultKind::Transient,
            "TRAN",
            &self.inner.time,
            inventory,
            rspice_core::analysis::measure_signals::transient_signal_map(&self.inner),
            ordered,
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::engine::{DigitalTrace, DigitalTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    /// Two analog points, one retained node, one event-only net whose voltage
    /// column the run left empty, and one branch the deck did not save.
    fn mixed_result() -> PyTransientResult {
        PyTransientResult::new(TransientResult {
            current_impulses: None,
            time: vec![0.0, 1.0e-9],
            step_sizes: vec![0.0, 1.0e-9],
            voltages: vec![vec![0.0, 1.0], Vec::new()],
            branch_currents: vec![vec![0.0, -1.0e-3], Vec::new()],
            num_nodes: 2,
            node_names: vec!["out".to_string(), "d".to_string()],
            branch_names: vec!["V1".to_string(), "V2".to_string()],
            digital_traces: vec![DigitalTrace {
                node_name: "d".to_string(),
                points: vec![DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue {
                        state: DigitalState::One,
                        strength: DigitalStrength::Strong,
                    },
                }],
            }],
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        })
    }

    /// An unretained column is a column this export does not carry, not a
    /// variable of zero points.
    ///
    /// Emitting it made `to_raw()`/`write_raw()` refuse the whole export —
    /// "variable 'V(d)' has 0 points but the sweep axis has 2" — on every
    /// mixed deck, and made `to_csv()` publish a `V(d)` column of NaN. The
    /// core's own flattened projection skips such a column, so the two
    /// exporters of one run have to agree.
    #[test]
    fn an_unretained_column_is_absent_from_every_export_rather_than_empty() {
        let result = mixed_result();

        let columns = result.export_columns();
        assert_eq!(columns, vec!["time", "V(out)", "I(V1)"], "{columns:?}");

        let csv = result.to_csv().expect("a csv of retained columns renders");
        let header = csv.lines().next().expect("the csv has a header");
        assert!(!header.contains("V(d)"), "{header}");
        assert!(!csv.contains("NaN") && !csv.contains("nan"), "{csv}");

        let raw = result
            .raw_file_bytes("ascii", Some("t"), Some("stamp"))
            .expect("the raw export is no longer refused by its own length check");
        let text = String::from_utf8(raw).expect("an ascii rawfile is text");
        assert!(!text.contains("V(d)"), "{text}");
        assert!(text.contains("V(out)"), "{text}");
    }

    /// `signal("v(d)")` refuses the same net as `voltage_waveform("d")`, with
    /// the same typed failure, so the binding raises one exception type for
    /// one refusal instead of a KeyError and a ValueError for the same net.
    #[test]
    fn a_probe_spelling_of_an_event_only_net_refuses_like_the_named_accessor() {
        let result = mixed_result();
        let expected = ResultAccessError::EventOnlyNode {
            name: "d".to_string(),
            kind: rspice_core::analysis::transient::EventOnlyNetKind::Digital,
            surface: EventTraceSurface::Result,
        };

        assert_eq!(result.event_only_probe_refusal("v(d)"), Some(expected));
        // The same net through the spelling an author is likelier to write.
        assert!(
            matches!(
                result.event_only_probe_refusal(" V(D) "),
                Some(ResultAccessError::EventOnlyNode { .. })
            ),
            "case and surrounding space must not change the answer"
        );
        assert_eq!(result.event_only_probe_refusal("v(out)"), None);
        assert_eq!(result.event_only_probe_refusal("i(V1)"), None);
    }
}
