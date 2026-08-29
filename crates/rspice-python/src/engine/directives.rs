//! Executing a deck's own analysis directives.
//!
//! `Engine.run()` is the automated-verification entry point: it runs whatever
//! `.op`, `.dc`, `.tran`, `.ac`, `.sp`, `.noise`, `.tf`, `.stb`, `.pz`, `.mc`,
//! `.step`, `.temp`, `.sens`, and `.four` cards the netlist carries, then
//! evaluates its `.MEAS` statements against the results.
//!
//! Two ordering rules are load-bearing:
//!
//! - `.four` is deferred until after the directive loop, so a `.four` card may
//!   precede the `.tran` it measures. Decks written by hand routinely do.
//! - A measurement whose analysis never ran is recorded as *not evaluated*
//!   rather than omitted, so a CI gate fails loudly instead of quietly
//!   checking nothing.
//!
//! Each directive runs inside its own fallible scope. When one fails and
//! `continue_on_error` is set, the records it had already pushed are rolled
//! back before a single skipped record replaces them, so a report can never
//! describe a half-executed analysis as successful.

use super::*;

/// Results for one analysis kind, where a deck may carry several such cards.
///
/// The singular accessor keeps the last result, which is the documented
/// contract; `all` keeps every one in deck order. Holding both in one type
/// stops the two from drifting apart, which is easy when they are two
/// independent locals updated by hand at each of a dozen call sites.
struct LastAndAll<T> {
    last: Option<T>,
    all: Vec<T>,
}

impl<T> Default for LastAndAll<T> {
    fn default() -> Self {
        Self {
            last: None,
            all: Vec::new(),
        }
    }
}

impl<T> LastAndAll<T> {
    /// Record a result as both the latest and a member of the full list.
    ///
    /// The caller supplies `clone` because duplicating a `Py<T>` handle needs
    /// a GIL token, which this container has no access to.
    fn push_with(&mut self, value: T, clone: impl FnOnce(&T) -> T) {
        self.all.push(clone(&value));
        self.last = Some(value);
    }

    fn last(&self) -> Option<&T> {
        self.last.as_ref()
    }

    /// `(last, all)` in the shape `RunReport` stores.
    fn into_parts(self) -> (Option<T>, Vec<T>) {
        (self.last, self.all)
    }
}

impl<T: Clone> LastAndAll<T> {
    fn push(&mut self, value: T) {
        self.push_with(value, T::clone);
    }
}

/// Everything the directive loop accumulates.
#[derive(Default)]
struct DirectiveOutcomes {
    records: Vec<PyAnalysisRecord>,
    op: LastAndAll<Py<PySimulationResult>>,
    dc: LastAndAll<Py<PyDcSweepResult>>,
    tran: LastAndAll<Py<PyTransientResult>>,
    ac: LastAndAll<Py<PyAcResult>>,
    noise: LastAndAll<Vec<PyNoiseResult>>,
    distortion: Option<Py<PyDistortionResult>>,
    hb: Option<PyHbResult>,
    s_parameters: Option<PySParameterResult>,
    /// Retained separately from `noise` because `.MEAS` evaluates against
    /// core's result type, not the Python projection.
    noise_core: Option<Vec<rspice_core::analysis::NoiseResult>>,
    tf: Option<PyTransferFunctionResult>,
    stb: Option<PyStbResult>,
    pz: Option<PyPoleZeroResult>,
    monte_carlo: Option<PyMonteCarloResult>,
    step_result: Option<PyDcSweepResult>,
    temperature: Option<PyDcSweepResult>,
    sensitivity: Option<PySensitivityResult>,
    sensitivity_ac: Option<PyAcSensitivityResult>,
    fourier: Vec<PyFourierResult>,
    /// `(fundamental, outputs, harmonics)` per `.four` card, evaluated after
    /// the loop.
    pending_fourier: Vec<(f64, Vec<String>, usize)>,
}

/// Run every analysis directive the deck declares, then its `.MEAS` statements.
pub(super) fn run(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    continue_on_error: bool,
) -> PyResult<PyRunReport> {
    let net = &netlist.inner;
    let mut out = DirectiveOutcomes::default();

    for analysis in &net.analyses {
        let records_before = out.records.len();
        if let Err(error) = execute(py_engine, py, netlist, analysis, &mut out) {
            if !continue_on_error {
                return Err(error);
            }
            // Drop any partial records the failed directive pushed so the
            // report never claims a half-executed analysis succeeded.
            out.records.truncate(records_before);
            out.records.push(PyAnalysisRecord::skipped(
                analysis_record_kind(analysis),
                describe_analysis(analysis),
                &crate::errors::describe_pyerr(py, &error),
            ));
        }
    }

    evaluate_pending_fourier(py, &mut out);
    let measurements = evaluate_measurements(py, net, &out);
    Ok(into_report(out, measurements))
}

/// Assemble the report, collapsing each `LastAndAll` into the pair it stores.
fn into_report(out: DirectiveOutcomes, measurements: Vec<PyMeasurement>) -> PyRunReport {
    let (op, all_op) = out.op.into_parts();
    let (dc, all_dc) = out.dc.into_parts();
    let (tran, all_tran) = out.tran.into_parts();
    let (ac, all_ac) = out.ac.into_parts();
    let (noise, all_noise) = out.noise.into_parts();

    PyRunReport {
        op,
        dc,
        tran,
        ac,
        distortion: out.distortion,
        hb: out.hb,
        s_parameters: out.s_parameters,
        noise,
        tf: out.tf,
        stb: out.stb,
        pz: out.pz,
        monte_carlo: out.monte_carlo,
        step: out.step_result,
        temperature: out.temperature,
        sensitivity: out.sensitivity,
        sensitivity_ac: out.sensitivity_ac,
        fourier: out.fourier,
        records: out.records,
        measurements,
        all_op,
        all_dc,
        all_tran,
        all_ac,
        all_noise,
    }
}

/// Run one directive, recording what it produced.
fn execute(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analysis: &AnalysisCommand,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    let net = &netlist.inner;

    match analysis {
        AnalysisCommand::Op => {
            let result = py_engine.run_dc_op(py, netlist)?;
            let handle = Py::new(py, result)?;
            out.op.push_with(handle, |handle| handle.clone_ref(py));
            out.records
                .push(PyAnalysisRecord::executed("op", ".op".to_string()));
        }
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => {
            let engine = py_engine.engine_for_netlist(&netlist.inner);
            let primary = DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            };
            let results = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_dc_sweep2_spec_with_report_and_abort(
                    &netlist.inner,
                    source,
                    &primary,
                    sweep2.as_ref(),
                    abort,
                )
            })?;
            let result = match sweep2 {
                Some(outer) => PyDcSweepResult::new_nested_with_reports(
                    results,
                    source,
                    &outer.source,
                    outer.spec().points(),
                )?,
                None => PyDcSweepResult::new_named_with_reports(results, source),
            };
            let handle = Py::new(py, result)?;
            out.dc.push_with(handle, |handle| handle.clone_ref(py));
            let description = describe_analysis(analysis);
            out.records
                .push(PyAnalysisRecord::executed("dc", description));
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let tstart = start.unwrap_or(0.0);
            let resolved = resolve_tran_max_step(*step, *stop, tstart, *max_step);
            let result = py_engine.tran_impl(
                py,
                netlist,
                *stop,
                resolved,
                tstart,
                Some(rspice_core::engine::TransientStartupMode::from_uic(*uic)),
            )?;
            let handle = Py::new(py, result)?;
            out.tran.push_with(handle, |handle| handle.clone_ref(py));
            let mut detail = format!(".tran {step} {stop}");
            if tstart > 0.0 {
                detail.push_str(&format!(" (tstart={tstart})"));
            }
            out.records.push(PyAnalysisRecord::executed("tran", detail));
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
            let result = py_engine.ac_impl(py, netlist, frequencies)?;
            let handle = Py::new(py, result)?;
            out.ac.push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "ac",
                format!(
                    ".ac {} {points} {start_freq} {stop_freq}",
                    format!("{variation:?}").to_lowercase()
                ),
            ));
        }
        AnalysisCommand::AcData { table_name } => {
            let frequencies = ac_data_frequencies(net, table_name)?;
            let result = py_engine.ac_impl(py, netlist, frequencies)?;
            let handle = Py::new(py, result)?;
            out.ac.push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "ac_data",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Hb { frequencies } => {
            let requested_orders = if net.options.hb_num_frequencies.is_empty() {
                None
            } else {
                Some(net.options.hb_num_frequencies.as_slice())
            };
            let orders = resolve_hb_harmonic_orders(
                frequencies.len(),
                requested_orders,
                ".OPTIONS HBINT NUMFREQ",
            )?;
            let mut config = hb_config_from_tones(frequencies, &orders, None)?;
            // Xyce's explicit single-tone NUMFREQ contract uses the
            // minimal bilateral 2*N+1 collocation grid.
            if frequencies.len() == 1 && requested_orders.is_some() {
                config.collocation_points = Some(
                    orders[0]
                        .checked_mul(2)
                        .and_then(|count| count.checked_add(1))
                        .ok_or_else(|| {
                            crate::errors::value_error(
                                ".OPTIONS HBINT NUMFREQ exceeds the addressable collocation grid",
                            )
                        })?,
                );
            }
            let engine = py_engine.engine_for_netlist(net);
            let result = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_hb_with_abort(net, config, abort)
            })?;
            out.hb = Some(PyHbResult::from_core(&result));
            out.records.push(PyAnalysisRecord::executed(
                "hb",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            let frequencies = sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
            let result = py_engine.distortion_impl(py, netlist, frequencies, *f2_over_f1)?;
            out.distortion = Some(Py::new(py, result)?);
            out.records.push(PyAnalysisRecord::executed(
                "disto",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise,
        } => {
            let frequencies = sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
            out.s_parameters =
                Some(py_engine.sparameter_impl(py, netlist, frequencies, *do_noise)?);
            out.records.push(PyAnalysisRecord::executed(
                "sp",
                describe_analysis(analysis),
            ));
            if *do_noise {
                out.records.push(PyAnalysisRecord::executed(
                    "sp_noise",
                    describe_analysis(analysis),
                ));
            }
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let engine = py_engine.engine_for_netlist(net);
            let output = py_engine.resolve_node(
                &engine,
                net,
                &NodeIdentifier::Name(output_node.clone()),
                "noise output",
            )?;
            let output_neg = match reference_node {
                Some(reference) => Some(py_engine.resolve_node(
                    &engine,
                    net,
                    &NodeIdentifier::Name(reference.clone()),
                    "noise reference",
                )?),
                None => None,
            };
            let frequencies = sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
            let source = if input_source.is_empty() {
                None
            } else {
                Some(input_source.as_str())
            };
            let results = py_engine.noise_core_impl(
                py,
                netlist,
                output,
                output_neg,
                source,
                &frequencies,
                None,
            )?;
            let converted: Vec<PyNoiseResult> =
                results.iter().map(PyNoiseResult::from_core).collect();
            out.noise.push(converted);
            out.noise_core = Some(results);
            out.records.push(PyAnalysisRecord::executed(
                "noise",
                format!(".noise V({output_node}) {input_source}"),
            ));
        }
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let engine = py_engine.engine_for_netlist(net);
            let (_, results) = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_noise_data_named_with_input_source_and_abort(
                    net,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    table_name,
                    engine.config().temperature,
                    abort,
                )
            })?;
            let converted: Vec<PyNoiseResult> =
                results.iter().map(PyNoiseResult::from_core).collect();
            out.noise.push(converted);
            out.noise_core = Some(results);
            out.records.push(PyAnalysisRecord::executed(
                "noise_data",
                format!(".noise V({output_node}) {input_source} DATA={table_name}"),
            ));
        }
        AnalysisCommand::Tf {
            output_node,
            reference_node,
            output_is_current,
            input_source,
        } => {
            let result = py_engine.tf_impl(
                py,
                netlist,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
            )?;
            out.tf = Some(result);
            out.records.push(PyAnalysisRecord::executed(
                "tf",
                format!(".tf {output_node} {input_source}"),
            ));
        }
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => {
            let result = py_engine.stb_impl(
                py,
                netlist,
                probe,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            )?;
            out.stb = Some(result);
            out.records.push(PyAnalysisRecord::executed(
                "stb",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            let (compute_poles, compute_zeros) = match analysis_type {
                PoleZeroAnalysisType::PoleZero => (true, true),
                PoleZeroAnalysisType::PolesOnly => (true, false),
                PoleZeroAnalysisType::ZerosOnly => (false, true),
            };
            let result = py_engine.pz_impl(
                py,
                netlist,
                &NodeIdentifier::Name(input_pos.clone()),
                Some(&NodeIdentifier::Name(input_neg.clone())),
                &NodeIdentifier::Name(output_pos.clone()),
                Some(&NodeIdentifier::Name(output_neg.clone())),
                matches!(transfer_type, PoleZeroTransferType::Current),
                compute_poles,
                compute_zeros,
            )?;
            out.pz = Some(result);
            out.records.push(PyAnalysisRecord::executed(
                "pz",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::MonteCarlo(command) => {
            let distribution = match command.distribution {
                rspice_core::netlist::MonteCarloDistribution::Gaussian => "gaussian",
                rspice_core::netlist::MonteCarloDistribution::Uniform => "uniform",
                rspice_core::netlist::MonteCarloDistribution::WorstCase => "worst_case",
            };
            let params = (!command.params.is_empty()).then(|| command.params.clone());
            let result = py_engine.run_monte_carlo(
                py,
                netlist,
                command.runs,
                command.seed,
                distribution,
                command.relative_spread,
                params,
            )?;
            out.monte_carlo = Some(result);
            out.records.push(PyAnalysisRecord::executed(
                "mc",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Step(command) => {
            let values = command.sweep.values();
            let engine = py_engine.engine_for_netlist(net);
            let results = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_step_command_with_abort(net, command, &values, abort)
            })?;
            out.step_result = Some(PyDcSweepResult::new_named(results, &command.name));
            out.records.push(PyAnalysisRecord::executed(
                "step",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Temp { temperatures } => {
            let command = StepCommand {
                target: StepTarget::Temp,
                name: "TEMP".to_string(),
                param_name: None,
                sweep: StepSweep::List(temperatures.clone()),
            };
            let engine = py_engine.engine_for_netlist(net);
            let results = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_step_command_with_abort(net, &command, temperatures, abort)
            })?;
            out.temperature = Some(PyDcSweepResult::new_named(results, "TEMP"));
            out.records.push(PyAnalysisRecord::executed(
                "temp",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } => {
            if let Some(sweep) = ac_sweep {
                let frequencies = ac_sweep_frequencies(
                    sweep.variation,
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq,
                );
                let output = NodeIdentifier::Name(output_node.clone());
                let reference = reference_node
                    .as_ref()
                    .map(|name| NodeIdentifier::Name(name.clone()));
                out.sensitivity_ac = Some(py_engine.sensitivity_ac_complete_impl(
                    py,
                    netlist,
                    &output,
                    reference.as_ref(),
                    *output_is_current,
                    &frequencies,
                    filters,
                )?);
                out.records.push(PyAnalysisRecord::executed(
                    "sens_ac",
                    describe_analysis(analysis),
                ));
            } else {
                let output = NodeIdentifier::Name(output_node.clone());
                let reference = reference_node
                    .as_ref()
                    .map(|name| NodeIdentifier::Name(name.clone()));
                out.sensitivity = Some(py_engine.sensitivity_dc_complete_impl(
                    py,
                    netlist,
                    &output,
                    reference.as_ref(),
                    *output_is_current,
                    filters,
                )?);
                out.records.push(PyAnalysisRecord::executed(
                    "sens",
                    describe_analysis(analysis),
                ));
            }
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            out.pending_fourier
                .push((*fundamental, outputs.clone(), *num_harmonics));
        }
    }
    Ok(())
}

/// Evaluate deferred `.four` cards against the transient result.
fn evaluate_pending_fourier(py: Python<'_>, out: &mut DirectiveOutcomes) {
    // .FOUR needs a transient result; evaluate after the loop so a
    // .four directive may precede its .tran in the deck.
    for (fundamental, outputs, num_harmonics) in std::mem::take(&mut out.pending_fourier) {
        match out.tran.last() {
            Some(tran_obj) => {
                let tran_ref = tran_obj.borrow(py);
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
                            match analysis.analyze(&tran_ref.inner.time, &waveform) {
                                Ok(result) => {
                                    out.fourier.push(PyFourierResult::from_core(&result));
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
    }
}

/// Evaluate the deck's `.MEAS` statements against whatever ran.
fn evaluate_measurements(
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
