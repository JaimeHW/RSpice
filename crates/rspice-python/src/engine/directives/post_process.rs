//! Post-processing evaluated against results the directive loop produced.
//!
//! `.FOUR` and `.MEAS` are deferred until every physical analysis in a
//! coordinate has run, so a card may precede the analysis it measures, and a
//! measurement whose analysis never ran is recorded as not evaluated rather
//! than dropped.

use super::*;

/// Evaluate deferred `.four` cards against the transient result.
pub(super) fn evaluate_pending_fourier(
    py: Python<'_>,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    // .FOUR needs a transient result; evaluate after the loop so a
    // .four directive may precede its .tran in the deck.
    for pending in std::mem::take(&mut out.pending_fourier) {
        let PendingFourier {
            fundamental,
            outputs,
            num_harmonics,
            analysis_id,
            coordinate,
        } = pending;
        let records_before = out.records.len();
        let parent_analysis_id = out
            .tran_context
            .as_ref()
            .and_then(|context| context.analysis_id.clone());
        let parent_coordinate = out
            .tran_context
            .as_ref()
            .and_then(|context| context.coordinate.clone())
            .or_else(|| coordinate.clone());
        match out.tran.last() {
            Some(tran_obj) => {
                let tran_ref = tran_obj.borrow(py);
                // Borrowed, not copied, across the worker's GIL release:
                // `TransientResult` exposes no mutating method, so nothing
                // Python can call meanwhile invalidates this grid.
                let time = tran_ref.inner.time.as_slice();
                for output in &outputs {
                    // `.four` addresses node voltages, differential node
                    // pairs, and branch currents alike.
                    let waveform = crate::signal::parse_signal_spec(output)
                        .map_err(crate::errors::value_error)
                        .and_then(|spec| tran_ref.signal_waveform(&spec));
                    match waveform {
                        Ok(waveform) => {
                            let analysis = rspice_core::analysis::FourierAnalysis::new(
                                rspice_core::analysis::FourierConfig::new(fundamental)
                                    .with_harmonics(num_harmonics),
                            );
                            // Qualification and transformation of a long
                            // waveform is unbounded work, so it runs on the
                            // interruptible worker. A cancellation is the one
                            // outcome that is not this output's own problem:
                            // it propagates instead of being recorded as a
                            // skipped directive.
                            let qualified =
                                crate::abort::run_interruptible_unregistered(py, |abort| {
                                    match analysis.analyze_with_abort(time, &waveform, abort) {
                                        Err(
                                            rspice_core::analysis::fourier::FourierError::Aborted,
                                        ) => Err(rspice_core::SimulationError::Aborted),
                                        outcome => Ok(outcome),
                                    }
                                })?;
                            match qualified {
                                Ok(result) => {
                                    out.fourier.push(PyFourierResult::from_core_with_provenance(
                                        &result,
                                        output.clone(),
                                        analysis_id.clone(),
                                        parent_analysis_id.clone(),
                                        parent_coordinate.clone(),
                                    ));
                                    out.records.push(PyAnalysisRecord::executed(
                                        "four",
                                        format!(".four {fundamental} {output}"),
                                    ));
                                }
                                Err(error) => {
                                    out.records.push(PyAnalysisRecord::skipped(
                                        "four",
                                        format!(".four {fundamental} {output}"),
                                        &format!(
                                            "Fourier output `{output}` could not be analyzed: {error}"
                                        ),
                                    ));
                                }
                            }
                        }
                        Err(err) => {
                            out.records.push(PyAnalysisRecord::skipped(
                                "four",
                                format!(".four {fundamental} {output}"),
                                &crate::errors::describe_pyerr(py, &err),
                            ));
                        }
                    }
                }
            }
            None => {
                out.records.push(PyAnalysisRecord::skipped(
                    "four",
                    format!(".four {fundamental} {}", outputs.join(" ")),
                    "requires a .tran analysis in the netlist",
                ));
            }
        }
        for record in &mut out.records[records_before..] {
            record.set_execution_context(Some(analysis_id.clone()), coordinate.clone());
            record.set_parent_analysis_id(parent_analysis_id.clone());
        }
    }
    Ok(())
}

/// Evaluate the deck's `.MEAS` statements against whatever ran.
pub(super) fn evaluate_measurements(
    py: Python<'_>,
    net: &rspice_core::Netlist,
    out: &DirectiveOutcomes,
) -> Vec<PyMeasurement> {
    // Evaluate measurements; report unevaluated ones as failures so CI
    // cannot silently skip checks.
    let mut measurements = Vec::new();
    match out.tran.last() {
        Some(tran_obj) => {
            let tran_ref = tran_obj.borrow(py);
            measurements.extend(measure::evaluate_tran_measurements(net, &tran_ref.inner));
        }
        None => measurements.extend(measure::unevaluated_measurements(
            net,
            "TRAN",
            "requires a .tran analysis in the netlist",
        )),
    }
    match out.dc.last() {
        Some(dc_obj) => {
            let dc_ref = dc_obj.borrow(py);
            measurements.extend(measure::evaluate_dc_measurements(net, &dc_ref.results));
        }
        None => measurements.extend(measure::unevaluated_measurements(
            net,
            "DC",
            "requires a .dc analysis in the netlist",
        )),
    }
    match out.ac.last() {
        Some(ac_obj) => {
            let ac_ref = ac_obj.borrow(py);
            measurements.extend(measure::evaluate_ac_measurements(net, &ac_ref.results));
        }
        None => measurements.extend(measure::unevaluated_measurements(
            net,
            "AC",
            "requires a .ac analysis in the netlist",
        )),
    }
    match &out.noise_core {
        Some(noise_results) => {
            measurements.extend(measure::evaluate_noise_measurements(net, noise_results));
        }
        None => measurements.extend(measure::unevaluated_measurements(
            net,
            "NOISE",
            "requires a .noise analysis in the netlist",
        )),
    }
    measurements
}
