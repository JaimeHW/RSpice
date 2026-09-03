//! Executing a deck's own analysis directives.
//!
//! `Engine.run()` is the automated-verification entry point: it runs whatever
//! `.op`, `.dc`, `.tran`, `.ac`, `.sp`, `.noise`, `.tf`, `.stb`, `.pz`, `.mc`,
//! `.hb`, `.pss`, `.pac`, `.pnoise`, `.envelope`, `.step`, `.temp`, `.sens`,
//! and `.four` cards the netlist carries, then evaluates its `.MEAS`
//! statements against the results.
//!
//! Three ordering rules are load-bearing:
//!
//! - `.four` is deferred until after the directive loop, so a `.four` card may
//!   precede the `.tran` it measures. Decks written by hand routinely do.
//! - A measurement whose analysis never ran is recorded as *not evaluated*
//!   rather than omitted, so a CI gate fails loudly instead of quietly
//!   checking nothing.
//! - `.pac`, `.pnoise`, and `.envelope` consume the periodic operating point
//!   of the specific upstream instance `DeckPlan` bound them to, retained here
//!   under that instance's canonical identity. "Whichever periodic analysis
//!   ran most recently" is a different, wrong answer in any deck that carries
//!   more than one.
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

#[derive(Clone)]
struct ExecutionContext {
    analysis_id: Option<String>,
    coordinate: Option<PyRunCoordinate>,
    /// Canonical identity of the periodic large-signal analysis this card
    /// linearizes around, as `DeckPlan` bound it.
    ///
    /// `.PAC`, `.PNOISE` and `.ENVELOPE` consume a specific upstream instance,
    /// not "whichever periodic analysis ran most recently". Carrying the plan's
    /// binding here is what keeps the executed pairing identical to the planned
    /// one.
    upstream_analysis_id: Option<String>,
}

/// A converged periodic large-signal state retained for its dependents.
enum PeriodicOperatingPoint {
    Shooting(Box<rspice_core::engine::PssOperatingPoint>),
    HarmonicBalance(Box<rspice_core::engine::HbOperatingPoint>),
}

struct PlannedDirectiveRun<'a> {
    coordinate: Option<&'a PyRunCoordinate>,
    analyses: &'a [rspice_core::execution::PlannedAnalysis],
}

struct PendingFourier {
    fundamental: f64,
    outputs: Vec<String>,
    num_harmonics: usize,
    analysis_id: String,
    coordinate: Option<PyRunCoordinate>,
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

    fn append(&mut self, mut other: Self) {
        if other.last.is_some() {
            self.last = other.last.take();
        }
        self.all.append(&mut other.all);
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
    /// Identity of the transient trajectory stored in `tran.last`.
    tran_context: Option<ExecutionContext>,
    ac: LastAndAll<Py<PyAcResult>>,
    noise: LastAndAll<Vec<PyNoiseResult>>,
    distortion: LastAndAll<Py<PyDistortionResult>>,
    hb: LastAndAll<PyHbResult>,
    pss: LastAndAll<PyPssResult>,
    pac: LastAndAll<PyPacResult>,
    pnoise: LastAndAll<PyPeriodicNoiseResult>,
    envelope: LastAndAll<Py<PyEnvelopeResult>>,
    /// Converged periodic operating points, keyed by the canonical identity of
    /// the analysis that produced them, so a dependent card consumes exactly
    /// the upstream the plan bound it to.
    periodic_operating_points: std::collections::BTreeMap<String, PeriodicOperatingPoint>,
    s_parameters: LastAndAll<PySParameterResult>,
    /// Retained separately from `noise` because `.MEAS` evaluates against
    /// core's result type, not the Python projection.
    noise_core: Option<Vec<rspice_core::analysis::NoiseResult>>,
    tf: LastAndAll<PyTransferFunctionResult>,
    stb: LastAndAll<PyStbResult>,
    pz: LastAndAll<PyPoleZeroResult>,
    monte_carlo: LastAndAll<PyMonteCarloResult>,
    step_result: Option<PyDcSweepResult>,
    temperature: Option<PyDcSweepResult>,
    sensitivity: LastAndAll<PySensitivityResult>,
    sensitivity_ac: LastAndAll<PyAcSensitivityResult>,
    fourier: Vec<PyFourierResult>,
    /// `(fundamental, outputs, harmonics)` per `.four` card, evaluated after
    /// the loop.
    pending_fourier: Vec<PendingFourier>,
}

impl DirectiveOutcomes {
    /// Merge one coordinate-local execution into the deck-wide report without
    /// letting a result from an earlier coordinate satisfy a failed analysis
    /// in a later coordinate.
    fn append(&mut self, mut other: Self) {
        self.records.append(&mut other.records);
        self.op.append(other.op);
        self.dc.append(other.dc);
        self.tran.append(other.tran);
        replace_if_some(&mut self.tran_context, other.tran_context);
        self.ac.append(other.ac);
        self.noise.append(other.noise);
        self.distortion.append(other.distortion);
        self.hb.append(other.hb);
        self.pss.append(other.pss);
        self.pac.append(other.pac);
        self.pnoise.append(other.pnoise);
        self.envelope.append(other.envelope);
        // Operating points are deliberately not merged across coordinates: a
        // dependent card must linearize around the carrier solved at its own
        // coordinate, never one retained from an earlier one.
        self.s_parameters.append(other.s_parameters);
        replace_if_some(&mut self.noise_core, other.noise_core);
        self.tf.append(other.tf);
        self.stb.append(other.stb);
        self.pz.append(other.pz);
        self.monte_carlo.append(other.monte_carlo);
        replace_if_some(&mut self.step_result, other.step_result);
        replace_if_some(&mut self.temperature, other.temperature);
        self.sensitivity.append(other.sensitivity);
        self.sensitivity_ac.append(other.sensitivity_ac);
        self.fourier.append(&mut other.fourier);
        self.pending_fourier.append(&mut other.pending_fourier);
    }
}

fn replace_if_some<T>(target: &mut Option<T>, incoming: Option<T>) {
    if incoming.is_some() {
        *target = incoming;
    }
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

    let plan = run_interruptible(py, &py_engine.active_runs, |abort| {
        rspice_core::execution::DeckPlan::from_netlist_with_abort(
            net,
            &netlist.resource_limits,
            abort,
        )
        .map_err(deck_plan_simulation_error)
    })?;
    let mut measurements = if plan.axes().is_empty() {
        run_directives(
            py_engine,
            py,
            netlist,
            &net.analyses,
            continue_on_error,
            PlannedDirectiveRun {
                coordinate: None,
                analyses: plan.analyses(),
            },
            &mut out,
        )?;
        evaluate_pending_fourier(py, &mut out)?;
        let mut measurements = evaluate_measurements(py, net, &out);
        set_measurement_execution_context(&mut measurements, &plan, None);
        measurements
    } else {
        run_axis_plan(py_engine, py, netlist, &plan, continue_on_error, &mut out)?
    };

    measurements.shrink_to_fit();
    Ok(into_report(out, measurements))
}

fn run_directives(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analyses: &[AnalysisCommand],
    continue_on_error: bool,
    planned_run: PlannedDirectiveRun<'_>,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    let mut planned = planned_run.analyses.iter();
    let mut next_fourier_ordinal = 1usize;

    for analysis in analyses {
        if matches!(
            analysis,
            AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. }
        ) {
            continue;
        }
        let (analysis_id, upstream_analysis_id) =
            if matches!(analysis, AnalysisCommand::Four { .. }) {
                let analysis_id = format!("four-{next_fourier_ordinal:03}");
                next_fourier_ordinal += 1;
                (Some(analysis_id), None)
            } else {
                match planned.next() {
                    Some(planned) => (
                        Some(planned.id().tag()),
                        planned.request().upstream().map(|id| id.tag()),
                    ),
                    None => (None, None),
                }
            };
        let context =
            (analysis_id.is_some() || planned_run.coordinate.is_some()).then(|| ExecutionContext {
                analysis_id,
                coordinate: planned_run.coordinate.cloned(),
                upstream_analysis_id,
            });
        execute_directive(
            py_engine,
            py,
            netlist,
            analysis,
            context.as_ref(),
            continue_on_error,
            out,
        )?;
    }
    Ok(())
}

fn run_materialized_directives(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analyses: &[rspice_core::execution::MaterializedAnalysis],
    coordinate: &PyRunCoordinate,
    continue_on_error: bool,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    for analysis in analyses {
        let command = analysis.command().ok_or_else(|| {
            crate::errors::SimulationError::new_err(format!(
                "canonical materializer omitted the command for {}",
                analysis.id()
            ))
        })?;
        let context = ExecutionContext {
            analysis_id: Some(analysis.output_namespace().analysis_component()),
            coordinate: Some(coordinate.clone()),
            upstream_analysis_id: analysis.planned().request().upstream().map(|id| id.tag()),
        };
        execute_directive(
            py_engine,
            py,
            netlist,
            command,
            Some(&context),
            continue_on_error,
            out,
        )?;
    }

    // FOUR is intentionally absent from `MaterializedAnalysis`: it remains
    // attached to the coordinate-local netlist and is evaluated against that
    // coordinate's final transient after every physical analysis has run.
    let mut next_fourier_ordinal = 1usize;
    for command in &netlist.inner.analyses {
        if matches!(command, AnalysisCommand::Four { .. }) {
            let context = ExecutionContext {
                analysis_id: Some(format!("four-{next_fourier_ordinal:03}")),
                coordinate: Some(coordinate.clone()),
                upstream_analysis_id: None,
            };
            next_fourier_ordinal += 1;
            execute_directive(
                py_engine,
                py,
                netlist,
                command,
                Some(&context),
                continue_on_error,
                out,
            )?;
        }
    }
    Ok(())
}

fn execute_directive(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analysis: &AnalysisCommand,
    context: Option<&ExecutionContext>,
    continue_on_error: bool,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    let records_before = out.records.len();
    if let Err(error) = execute(py_engine, py, netlist, analysis, context, out) {
        if !continue_on_error {
            return Err(error);
        }
        // Drop any partial records the failed directive pushed so the report
        // never claims a half-executed analysis succeeded.
        out.records.truncate(records_before);
        let mut record = PyAnalysisRecord::skipped(
            analysis_record_kind(analysis),
            describe_analysis(analysis),
            &crate::errors::describe_pyerr(py, &error),
        );
        if let Some(context) = context {
            record.set_execution_context(context.analysis_id.clone(), context.coordinate.clone());
        }
        out.records.push(record);
    } else if let Some(context) = context {
        for record in &mut out.records[records_before..] {
            record.set_execution_context(context.analysis_id.clone(), context.coordinate.clone());
        }
        if matches!(analysis, AnalysisCommand::Tran { .. }) {
            out.tran_context = Some(context.clone());
        }
    }
    Ok(())
}

fn run_axis_plan(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    plan: &rspice_core::execution::DeckPlan,
    continue_on_error: bool,
    out: &mut DirectiveOutcomes,
) -> PyResult<Vec<PyMeasurement>> {
    let core_engine = py_engine.engine_for_netlist(&netlist.inner);
    let materializer = run_interruptible(py, &py_engine.active_runs, |abort| {
        core_engine
            .prepare_deck_plan_materializer_with_abort(&netlist.inner, plan, abort)
            .map_err(materialized_run_simulation_error)
    })?;
    let implicit_op = plan.analyses().len() == 1
        && plan.analyses()[0].id().kind() == rspice_core::execution::AnalysisKind::ImplicitOp;
    let compatibility_axis = implicit_op.then(|| single_legacy_axis(plan)).flatten();
    let mut compatibility_operating_points = Vec::new();
    let mut compatibility_complete = compatibility_axis.is_some();
    let mut measurements = Vec::new();

    for run_index in 0..materializer.len() {
        let materialized_run = run_interruptible(py, &py_engine.active_runs, |abort| {
            materializer
                .materialize_run_with_abort(run_index, abort)
                .map_err(materialized_run_simulation_error)
        })?;
        let (core_coordinate, materialized_netlist, _topology, materialized_analyses) =
            materialized_run.into_parts();
        if core_coordinate.ordinal() != run_index {
            return Err(crate::errors::SimulationError::new_err(
                "axis materializer returned a mismatched run index",
            ));
        }
        let coordinate = PyRunCoordinate::from_core(&core_coordinate);
        let compatibility_value = compatibility_axis
            .as_ref()
            .and_then(|_| legacy_coordinate_value(&core_coordinate));
        let materialized = PyNetlist {
            inner: materialized_netlist,
            resource_limits: netlist.resource_limits,
        };
        let mut coordinate_out = DirectiveOutcomes::default();

        if implicit_op {
            let [implicit_analysis] = materialized_analyses.as_slice() else {
                return Err(crate::errors::SimulationError::new_err(format!(
                    "canonical materializer returned {} analyses for an implicit operating point; expected exactly one",
                    materialized_analyses.len()
                )));
            };
            if implicit_analysis.command().is_some() {
                return Err(crate::errors::SimulationError::new_err(
                    "canonical materializer attached an authored command to implicit OP",
                ));
            }
            let context = ExecutionContext {
                analysis_id: Some(implicit_analysis.output_namespace().analysis_component()),
                coordinate: Some(coordinate.clone()),
                upstream_analysis_id: None,
            };
            match py_engine.run_dc_op(py, &materialized) {
                Ok(result) => {
                    if let Some(value) = compatibility_value {
                        compatibility_operating_points.push((value, result.inner.clone()));
                    } else {
                        compatibility_complete = false;
                    }
                    let handle = Py::new(py, result)?;
                    coordinate_out
                        .op
                        .push_with(handle, |handle| handle.clone_ref(py));
                    let mut record = PyAnalysisRecord::executed("op", ".op (implicit)".to_string());
                    record.set_execution_context(context.analysis_id, context.coordinate);
                    coordinate_out.records.push(record);
                }
                Err(error) if continue_on_error => {
                    compatibility_complete = false;
                    let mut record = PyAnalysisRecord::skipped(
                        "op",
                        ".op (implicit)".to_string(),
                        &crate::errors::describe_pyerr(py, &error),
                    );
                    record.set_execution_context(context.analysis_id, context.coordinate);
                    coordinate_out.records.push(record);
                }
                Err(error) => return Err(error),
            }
        } else {
            run_materialized_directives(
                py_engine,
                py,
                &materialized,
                &materialized_analyses,
                &coordinate,
                continue_on_error,
                &mut coordinate_out,
            )?;
        }

        evaluate_pending_fourier(py, &mut coordinate_out)?;
        let mut coordinate_measurements =
            evaluate_measurements(py, &materialized.inner, &coordinate_out);
        set_measurement_execution_context(&mut coordinate_measurements, plan, Some(&coordinate));
        measurements.append(&mut coordinate_measurements);
        out.append(coordinate_out);
    }

    if compatibility_complete
        && compatibility_operating_points.len() == materializer.len()
        && let Some((kind, name)) = compatibility_axis
    {
        let result = PyDcSweepResult::new_named(compatibility_operating_points, &name);
        match kind {
            rspice_core::execution::AxisKind::Temperature => out.temperature = Some(result),
            rspice_core::execution::AxisKind::Data | rspice_core::execution::AxisKind::Step => {
                out.step_result = Some(result)
            }
            rspice_core::execution::AxisKind::Alter => {}
            _ => {}
        }
    }
    Ok(measurements)
}

fn materialized_run_simulation_error(
    error: rspice_core::execution::MaterializedRunError,
) -> rspice_core::engine::SimulationError {
    match error {
        rspice_core::execution::MaterializedRunError::Aborted => {
            rspice_core::engine::SimulationError::Aborted
        }
        rspice_core::execution::MaterializedRunError::DeckPlan(error) => {
            deck_plan_simulation_error(error)
        }
        rspice_core::execution::MaterializedRunError::Simulation(error) => error,
        other => rspice_core::engine::SimulationError::Circuit(format!(
            "canonical deck materialization failed: {other}"
        )),
    }
}

fn deck_plan_simulation_error(
    error: rspice_core::execution::DeckPlanError,
) -> rspice_core::engine::SimulationError {
    match error {
        rspice_core::execution::DeckPlanError::Aborted => {
            rspice_core::engine::SimulationError::Aborted
        }
        rspice_core::execution::DeckPlanError::ResourceLimit(error) => error.into(),
        other => rspice_core::engine::SimulationError::Circuit(other.to_string()),
    }
}

fn single_legacy_axis(
    plan: &rspice_core::execution::DeckPlan,
) -> Option<(rspice_core::execution::AxisKind, String)> {
    let axis = plan.axes().first().filter(|_| plan.axes().len() == 1)?;
    let name = match axis.step_target() {
        Some(rspice_core::execution::StepAxisTarget::Parameter { name }) => name.clone(),
        Some(rspice_core::execution::StepAxisTarget::Device {
            name,
            parameter: Some(parameter),
        }) => format!("{name}:{parameter}"),
        Some(rspice_core::execution::StepAxisTarget::Device {
            name,
            parameter: None,
        }) => name.clone(),
        Some(rspice_core::execution::StepAxisTarget::Model { name, parameter }) => {
            format!("{name}:{parameter}")
        }
        Some(rspice_core::execution::StepAxisTarget::Temperature) => "TEMP".to_string(),
        Some(_) => axis.name().to_string(),
        None if axis.kind() == rspice_core::execution::AxisKind::Temperature => "TEMP".to_string(),
        None => axis.name().to_string(),
    };
    Some((axis.kind(), name))
}

fn legacy_coordinate_value(coordinate: &rspice_core::execution::RunCoordinate) -> Option<f64> {
    let assignment = coordinate.assignments().first()?;
    match assignment.value() {
        rspice_core::execution::RunAxisValue::Numeric(value) => Some(*value),
        rspice_core::execution::RunAxisValue::DataRow(_) => Some(assignment.value_index() as f64),
        rspice_core::execution::RunAxisValue::AlterVariant { .. } => None,
        _ => None,
    }
}

fn set_measurement_execution_context(
    measurements: &mut [PyMeasurement],
    plan: &rspice_core::execution::DeckPlan,
    coordinate: Option<&PyRunCoordinate>,
) {
    for measurement in measurements {
        let kind = match measurement.analysis.as_str() {
            analysis if analysis.eq_ignore_ascii_case("TRAN") => {
                Some(rspice_core::execution::AnalysisKind::Tran)
            }
            analysis if analysis.eq_ignore_ascii_case("DC") => {
                Some(rspice_core::execution::AnalysisKind::Dc)
            }
            analysis if analysis.eq_ignore_ascii_case("AC") => {
                Some(rspice_core::execution::AnalysisKind::Ac)
            }
            analysis if analysis.eq_ignore_ascii_case("NOISE") => {
                Some(rspice_core::execution::AnalysisKind::Noise)
            }
            _ => None,
        };
        let analysis_id = kind.and_then(|kind| {
            plan.analyses()
                .iter()
                .rev()
                .find(|analysis| analysis.id().kind() == kind)
                .map(|analysis| analysis.id().tag())
        });
        measurement.set_execution_context(analysis_id, coordinate.cloned());
    }
}

/// Assemble the report, collapsing each `LastAndAll` into the pair it stores.
fn into_report(out: DirectiveOutcomes, measurements: Vec<PyMeasurement>) -> PyRunReport {
    let (op, all_op) = out.op.into_parts();
    let (dc, all_dc) = out.dc.into_parts();
    let (tran, all_tran) = out.tran.into_parts();
    let (ac, all_ac) = out.ac.into_parts();
    let (noise, all_noise) = out.noise.into_parts();
    let (distortion, all_distortion) = out.distortion.into_parts();
    let (hb, all_hb) = out.hb.into_parts();
    let (pss, all_pss) = out.pss.into_parts();
    let (pac, all_pac) = out.pac.into_parts();
    let (pnoise, all_pnoise) = out.pnoise.into_parts();
    let (envelope, all_envelope) = out.envelope.into_parts();
    let (s_parameters, all_s_parameters) = out.s_parameters.into_parts();
    let (tf, all_tf) = out.tf.into_parts();
    let (stb, all_stb) = out.stb.into_parts();
    let (pz, all_pz) = out.pz.into_parts();
    let (monte_carlo, all_monte_carlo) = out.monte_carlo.into_parts();
    let (sensitivity, all_sensitivity) = out.sensitivity.into_parts();
    let (sensitivity_ac, all_sensitivity_ac) = out.sensitivity_ac.into_parts();

    PyRunReport {
        op,
        dc,
        tran,
        ac,
        distortion,
        hb,
        pss,
        pac,
        pnoise,
        envelope,
        s_parameters,
        noise,
        tf,
        stb,
        pz,
        monte_carlo,
        step: out.step_result,
        temperature: out.temperature,
        sensitivity,
        sensitivity_ac,
        fourier: out.fourier,
        records: out.records,
        measurements,
        all_op,
        all_dc,
        all_tran,
        all_ac,
        all_noise,
        all_distortion,
        all_hb,
        all_pss,
        all_pac,
        all_pnoise,
        all_envelope,
        all_s_parameters,
        all_tf,
        all_stb,
        all_pz,
        all_monte_carlo,
        all_sensitivity,
        all_sensitivity_ac,
    }
}

/// Run one directive, recording what it produced.
fn execute(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analysis: &AnalysisCommand,
    context: Option<&ExecutionContext>,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    let net = &netlist.inner;
    let max_analysis_points = || {
        py_engine
            .engine_for_netlist(net)
            .config()
            .resource_limits
            .max_analysis_points
    };

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
            let resolved = rspice_core::execution::resolve_transient_maximum_step(
                *step, *stop, *start, *max_step,
            )
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
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
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
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
            out.hb.push(PyHbResult::from_core(&result));
            // Retained under this instance's canonical identity so a bound
            // `.PAC`, `.PNOISE`, or `.ENVELOPE` consumes this exact carrier.
            if let Some(id) = context.and_then(|context| context.analysis_id.clone()) {
                out.periodic_operating_points.insert(
                    id,
                    PeriodicOperatingPoint::HarmonicBalance(Box::new(result.operating_point)),
                );
            }
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
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            let result = py_engine.distortion_impl(py, netlist, frequencies, *f2_over_f1)?;
            let handle = Py::new(py, result)?;
            out.distortion
                .push_with(handle, |handle| handle.clone_ref(py));
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
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            out.s_parameters.push(py_engine.sparameter_impl(
                py,
                netlist,
                frequencies,
                *do_noise,
            )?);
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
                py,
                &engine,
                net,
                &NodeIdentifier::Name(output_node.clone()),
                "noise output",
            )?;
            let output_neg = match reference_node {
                Some(reference) => Some(py_engine.resolve_node(
                    py,
                    &engine,
                    net,
                    &NodeIdentifier::Name(reference.clone()),
                    "noise reference",
                )?),
                None => None,
            };
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
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
            out.tf.push(result);
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
            out.stb.push(result);
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
            out.pz.push(result);
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
            out.monte_carlo.push(result);
            out.records.push(PyAnalysisRecord::executed(
                "mc",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => {
            return Err(crate::errors::SimulationError::new_err(
                "run-axis directives must be executed through the canonical deck materializer",
            ));
        }
        AnalysisCommand::Pss(card) => {
            let config = rspice_core::analysis::PssConfig::from(card.as_ref());
            let harmonics = config.num_harmonics;
            let engine = py_engine.engine_for_netlist(net);
            // The operating point carries the analysis result, so solving it
            // once yields both the published result and the exact orbit a
            // dependent `.PAC`/`.PNOISE` linearizes around.
            let operating_point = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_pss_operating_point_with_abort(net, config, abort)
            })?;
            out.pss.push(PyPssResult::from_core(
                operating_point.analysis(),
                harmonics,
            ));
            if let Some(id) = context.and_then(|context| context.analysis_id.clone()) {
                out.periodic_operating_points.insert(
                    id,
                    PeriodicOperatingPoint::Shooting(Box::new(operating_point)),
                );
            }
            out.records.push(PyAnalysisRecord::executed(
                "pss",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Pac(card) => {
            let config = rspice_core::analysis::PacConfig::from(card.as_ref());
            config.validate().map_err(|message| {
                crate::errors::value_error(format!("invalid .PAC card: {message}"))
            })?;
            let engine = py_engine.engine_for_netlist(net);
            let result = match upstream_operating_point(out, context, ".PAC")? {
                PeriodicOperatingPoint::Shooting(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pac_from_pss_with_abort(net, config, point, abort)
                    })?
                }
                PeriodicOperatingPoint::HarmonicBalance(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pac_from_hb_with_abort(net, config, point, abort)
                    })?
                }
            };
            out.pac.push(PyPacResult::from_core(&result));
            out.records.push(PyAnalysisRecord::executed(
                "pac",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Pnoise(card) => {
            let offsets = sweep_frequencies(
                card.sweep.variation,
                card.sweep.points,
                card.sweep.start_freq,
                card.sweep.stop_freq,
                max_analysis_points(),
            )?;
            if offsets.iter().any(|offset| *offset <= 0.0) {
                return Err(crate::errors::value_error(
                    ".PNOISE offset frequencies must be strictly positive",
                ));
            }
            let engine = py_engine.engine_for_netlist(net);
            let reference = card.reference_node.as_deref();
            let source = card.input_source.as_deref();
            let result = match upstream_operating_point(out, context, ".PNOISE")? {
                PeriodicOperatingPoint::Shooting(point) if point.config().is_autonomous() => {
                    // An autonomous carrier's noise is oscillator phase noise,
                    // a different result family with a different unit basis.
                    // The engine computes it, but this deck route publishes
                    // only driven periodic noise, so folding it in here would
                    // mislabel phase noise as sideband noise.
                    return Err(crate::errors::not_implemented_error(
                        "Engine.run executes .PNOISE around a driven carrier only; the .PSS it is \
                         bound to is autonomous, whose noise is oscillator phase noise — call \
                         run_oscillator_noise for that result family",
                    ));
                }
                PeriodicOperatingPoint::Shooting(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pnoise_from_pss_with_abort(
                            net,
                            &offsets,
                            &card.output_node,
                            reference,
                            source,
                            card.max_sideband,
                            point,
                            abort,
                        )
                    })?
                }
                PeriodicOperatingPoint::HarmonicBalance(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pnoise_from_hb_with_abort(
                            net,
                            &offsets,
                            &card.output_node,
                            reference,
                            source,
                            card.max_sideband,
                            point,
                            abort,
                        )
                    })?
                }
            };
            out.pnoise.push(PyPeriodicNoiseResult::from_core(&result));
            out.records.push(PyAnalysisRecord::executed(
                "pnoise",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Envelope(card) => {
            // The envelope continues the carrier the bound `.HB` defined, so
            // its spectral configuration comes from that instance rather than
            // being re-derived from the deck.
            let config = match upstream_operating_point(out, context, ".ENVELOPE")? {
                PeriodicOperatingPoint::HarmonicBalance(point) => point.config().clone(),
                PeriodicOperatingPoint::Shooting(_) => {
                    return Err(crate::errors::value_error(
                        ".ENVELOPE continues a harmonic-balance carrier, but the analysis it is \
                         bound to is a shooting PSS",
                    ));
                }
            };
            let frozen = card.frozen_sources.clone();
            let engine = py_engine.engine_for_netlist(net);
            let result = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_envelope_with_abort(
                    net,
                    config,
                    &frozen,
                    card.duration,
                    card.max_step,
                    abort,
                )
            })?;
            let handle = Py::new(py, PyEnvelopeResult::from_core(py, &result)?)?;
            out.envelope
                .push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "envelope",
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
                let frequencies = sweep_frequencies(
                    sweep.variation,
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq,
                    max_analysis_points(),
                )?;
                let output = NodeIdentifier::Name(output_node.clone());
                let reference = reference_node
                    .as_ref()
                    .map(|name| NodeIdentifier::Name(name.clone()));
                out.sensitivity_ac
                    .push(py_engine.sensitivity_ac_complete_impl(
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
                out.sensitivity.push(py_engine.sensitivity_dc_complete_impl(
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
            let context = context.expect(".FOUR directives always receive a stable identity");
            out.pending_fourier.push(PendingFourier {
                fundamental: *fundamental,
                outputs: outputs.clone(),
                num_harmonics: *num_harmonics,
                analysis_id: context.analysis_id.clone().expect("checked above"),
                coordinate: context.coordinate.clone(),
            });
        }
    }
    Ok(())
}

/// The periodic operating point the deck plan bound this card to.
///
/// The binding is the plan's, not "whichever periodic analysis ran last": a
/// deck with two `.PSS` cards and a `.PAC` between them must linearize around
/// the one the planner selected, and a `.PAC` whose upstream failed must fail
/// rather than silently reuse an earlier orbit.
fn upstream_operating_point<'a>(
    out: &'a DirectiveOutcomes,
    context: Option<&ExecutionContext>,
    card: &'static str,
) -> PyResult<&'a PeriodicOperatingPoint> {
    let id = context
        .and_then(|context| context.upstream_analysis_id.clone())
        .ok_or_else(|| {
            crate::errors::SimulationError::new_err(format!(
                "{card} was executed without the upstream periodic analysis its deck plan bound it to"
            ))
        })?;
    out.periodic_operating_points.get(&id).ok_or_else(|| {
        crate::errors::SimulationError::new_err(format!(
            "{card} linearizes around {id}, which produced no periodic operating point in this run"
        ))
    })
}

/// Evaluate deferred `.four` cards against the transient result.
fn evaluate_pending_fourier(py: Python<'_>, out: &mut DirectiveOutcomes) -> PyResult<()> {
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
