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

// The module is split by what each part decides: `axes` materializes a
// `.STEP`/`.TEMP` plan into run coordinates, `execute` is the single match
// from an authored card to an engine call, and `post_process` evaluates
// `.FOUR` and `.MEAS` against what ran.
mod axes;
mod execute;
mod post_process;

use axes::{run_axis_plan, set_measurement_execution_context};
use execute::execute;
use post_process::{evaluate_measurements, evaluate_pending_fourier};

/// Results for one analysis kind, where a deck may carry several such cards.
///
/// The singular accessor keeps the last result, which is the documented
/// contract; `all` keeps every one in deck order. Holding both in one type
/// stops the two from drifting apart, which is easy when they are two
/// independent locals updated by hand at each of a dozen call sites.
pub(super) struct LastAndAll<T> {
    last: Option<T>,
    all: Vec<T>,
}

/// Where a `.TRAN` execution's startup contract comes from.
///
/// A deck's own card states it: `UIC` or an operating-point start. A
/// convenience call authors no card, so the contract is whatever the deck's
/// `.TRAN` cards declare — which is core's rule, asked of core rather than
/// restated here.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TransientStartup {
    /// The executed card's own `UIC` flag.
    Card,
    /// No card was authored; the engine infers the deck's contract.
    DeckInferred,
}

#[derive(Clone)]
struct ExecutionContext {
    analysis_id: Option<String>,
    coordinate: Option<PyRunCoordinate>,
    /// Startup contract for a `.TRAN` executed under this context.
    transient_startup: TransientStartup,
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
    transient_startup: TransientStartup,
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

    /// The one result a single-analysis execution produced.
    ///
    /// A convenience method asks for exactly one analysis, so anything other
    /// than exactly one result is a defect in this module rather than in the
    /// caller's circuit, and is reported as such instead of being silently
    /// narrowed to "the last one".
    pub(super) fn into_single(self, analysis: &str) -> PyResult<T> {
        let produced = self.all.len();
        match (self.last, produced) {
            (Some(value), 1) => Ok(value),
            (_, produced) => Err(crate::errors::SimulationError::new_err(format!(
                "the {analysis} request produced {produced} results; expected exactly one"
            ))),
        }
    }
}

impl<T: Clone> LastAndAll<T> {
    fn push(&mut self, value: T) {
        self.push_with(value, T::clone);
    }
}

/// Everything the directive loop accumulates.
#[derive(Default)]
pub(super) struct DirectiveOutcomes {
    records: Vec<PyAnalysisRecord>,
    pub(super) op: LastAndAll<Py<PySimulationResult>>,
    pub(super) dc: LastAndAll<Py<PyDcSweepResult>>,
    pub(super) tran: LastAndAll<Py<PyTransientResult>>,
    /// Identity of the transient trajectory stored in `tran.last`.
    tran_context: Option<ExecutionContext>,
    pub(super) ac: LastAndAll<Py<PyAcResult>>,
    noise: LastAndAll<Vec<PyNoiseResult>>,
    pub(super) distortion: LastAndAll<Py<PyDistortionResult>>,
    hb: LastAndAll<PyHbResult>,
    pub(super) pss: LastAndAll<PyPssResult>,
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
    pub(super) tf: LastAndAll<PyTransferFunctionResult>,
    pub(super) stb: LastAndAll<PyStbResult>,
    pub(super) pz: LastAndAll<PyPoleZeroResult>,
    pub(super) monte_carlo: LastAndAll<PyMonteCarloResult>,
    step_result: Option<PyDcSweepResult>,
    temperature: Option<PyDcSweepResult>,
    pub(super) sensitivity: LastAndAll<PySensitivityResult>,
    pub(super) sensitivity_ac: LastAndAll<PyAcSensitivityResult>,
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
                transient_startup: TransientStartup::Card,
            },
            &mut out,
        )?;
        evaluate_pending_fourier(py, net, &mut out)?;
        let mut measurements = evaluate_measurements(py, net, &out);
        set_measurement_execution_context(&mut measurements, &plan, None);
        measurements
    } else {
        run_axis_plan(py_engine, py, netlist, &plan, continue_on_error, &mut out)?
    };

    measurements.shrink_to_fit();
    Ok(into_report(out, measurements))
}

/// Execute exactly one analysis through the path `Engine.run` uses.
///
/// Every convenience method is a thin constructor over this: it validates its
/// Python arguments only by translating them into the authored card a deck
/// would carry, then hands that card to this executor. The card is planned by
/// core's own `DeckPlan`, so the call receives the same canonical
/// `AnalysisRequest` identity the deck route assigns, and `run_tran(...)`
/// cannot come to mean something different from `run(deck with .TRAN ...)`.
///
/// `continue_on_error` is deliberately false: a direct call has no report to
/// record a skipped directive in, so its failure is the call's failure.
fn run_one(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    command: AnalysisCommand,
    transient_startup: TransientStartup,
) -> PyResult<DirectiveOutcomes> {
    let mut planning_netlist = rspice_core::Netlist::default();
    planning_netlist.analyses = vec![command.clone()];
    // Planning one authored card reads only that card, so it is bounded work
    // that holds the GIL for a constant time; the analysis it plans still runs
    // on the interruptible worker.
    let plan = rspice_core::execution::DeckPlan::from_netlist_with_abort(
        &planning_netlist,
        &netlist.resource_limits,
        &rspice_core::NoAbort,
    )
    .map_err(|error| crate::errors::simulation_error_to_pyerr(deck_plan_simulation_error(error)))?;

    let mut out = DirectiveOutcomes::default();
    run_directives(
        py_engine,
        py,
        netlist,
        std::slice::from_ref(&command),
        false,
        PlannedDirectiveRun {
            coordinate: None,
            analyses: plan.analyses(),
            transient_startup,
        },
        &mut out,
    )?;
    Ok(out)
}

/// Execute one authored card whose startup contract the card itself states.
pub(super) fn run_one_card(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    command: AnalysisCommand,
) -> PyResult<DirectiveOutcomes> {
    run_one(py_engine, py, netlist, command, TransientStartup::Card)
}

/// Execute one synthesized `.TRAN` whose startup contract the deck states.
///
/// `run_tran` authors no card, so its `uic` field carries no authority; the
/// engine's own rule over the deck's `.TRAN` cards decides instead.
pub(super) fn run_one_uncarded_transient(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    command: AnalysisCommand,
) -> PyResult<DirectiveOutcomes> {
    run_one(
        py_engine,
        py,
        netlist,
        command,
        TransientStartup::DeckInferred,
    )
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
                transient_startup: planned_run.transient_startup,
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
