//! One trial protocol for every discrete lane of a circuit.
//!
//! A Newton probe, the inspection of a converged candidate, an operating point
//! and an accepted timepoint used to be four bodies that opened the same three
//! objects in the same order and settled the same boundary with four slightly
//! different spellings of one loop. They are one body here, and what separates
//! them is [`TrialKind`] alone: whether the trial may ever commit, whether it
//! owns the code-model rollback image, and which analysis and evaluation phase
//! the code models are told they are running in.
//!
//! # What a trial owns
//!
//! Opening one takes the shared HDL wheel out of the scheduler and the mixed
//! instances out of the circuit, exactly as the probe owner it replaces did,
//! and for the same reason: the code-model lane evaluates against the rest of
//! [`CircuitData`] while the wheel and the instances are being settled, so the
//! three have to be borrowable at once. Taking the instances out is also what
//! keeps the engine's `finish` callback — which runs between
//! [`Trial::prepare`] and [`Prepared::commit`] and reaches
//! `accept_model_transient_timestep` — from opening a second mixed trial inside
//! the first: with the instances out of the circuit, that path sees no mixed
//! module and promotes only the analog devices.
//!
//! # What a trial gives back
//!
//! Every exit is [`Drop`]. It rejects any instance trial still open, hands the
//! wheel's pre-trial image back to the coordinator, restores the code-model
//! image a speculative kind captured, and returns the wheel and the instances
//! to the circuit. [`Trial::finish`] is the same sequence with the restore's
//! own refusal *reported* rather than logged, which is what the speculative
//! entry points want: a probe whose code-model resources cannot be put back has
//! left the circuit latched, and that is not something to discover later.

use std::sync::Arc;

use crate::circuit::CircuitData;
use crate::circuit::external_models::{
    VerilogACompanionRules, XspiceAcceptanceRollback, XspiceActiveWave, XspiceCompanionPolicy,
    XspiceDigitalBindings, XspiceDigitalParticipant,
};
use crate::numerics::integration::CompanionCoefficients;
use crate::xspice::ResourceTransaction;
use crate::xspice::verilog::host::DigitalActiveParticipant;
use crate::xspice::verilog::{
    MixedDigitalCoordinator, MixedSignalError, MixedSignalHost, PreparedMixedAcceptance,
    SharedTrialCursor,
};
use crate::{SimulationError, Value};

use super::{shared_error, swap_candidate_ledgers};

/// How many times one trial may re-settle its boundary before the engine gives
/// up on it.
///
/// The host has its own ceiling — the scheduler's delta-cycle cap, ten thousand
/// by default — but that one measures the depth of a *digital* settling and is
/// sized for it. A boundary that will not quiet is a different failure: two
/// bridges driving each other across the domain wall, which either resolves in
/// a couple of passes or never. Capping it here keeps a Newton iteration from
/// paying ten thousand digital settles to learn that.
pub(in crate::circuit) const MAX_BOUNDARY_SETTLE_PASSES: u32 = 64;

/// Name a refusal by the instance it came from.
pub(in crate::circuit) fn mixed_error(instance: &str, error: MixedSignalError) -> SimulationError {
    let message = format!("mixed Verilog-AMS instance '{instance}': {error}");
    // The analog half's non-finite trial reaches the Newton loops classified,
    // so a mixed host retries the same domain edge a plain analog instance
    // retries. Every other mixed failure is structural and ends the run —
    // including a non-finite value at an ACCEPTED point, which only
    // `MixedSignalHost::stamp_trial` can ever produce this variant for.
    match error {
        MixedSignalError::AnalogNonFinite { .. } => SimulationError::from(
            crate::device::StampError::nonfinite_trial(instance, message),
        ),
        _ => SimulationError::Circuit(message),
    }
}

/// Name the host a refusal came from, after the borrow that produced it ended.
///
/// The obvious spelling — `map_err(|error| mixed_error(host.instance_name(),
/// error))` on a call that already holds `host` mutably — does not borrow-check,
/// and the obvious repair was to copy the name into a `String` first. That copy
/// was taken *per Newton evaluation* on all three of the driver's paths, for a
/// diagnostic that almost never gets built. Passing the finished `Result` in
/// instead spends nothing on the path that succeeds: the mutable borrow ends
/// with the call, so the name can simply be read.
#[inline]
pub(in crate::circuit) fn named<T>(
    host: &MixedSignalHost,
    result: Result<T, MixedSignalError>,
) -> Result<T, SimulationError> {
    result.map_err(|error| mixed_error(host.instance_name(), error))
}

/// The timepoint a trial is opened at.
#[derive(Clone, Copy, Debug)]
pub(in crate::circuit) struct Candidate {
    /// The endpoint the continuous half has a solution at.
    pub time: Value,
    /// The interval behind it. Zero is the operating point and carries no
    /// companion rule.
    pub dt: Value,
    /// `(initial_step, final_step)` as the transient stepper sees them, or
    /// `None` at the operating point, where each instance answers for itself:
    /// the one assembly two callers share is reached with no step to describe.
    pub analysis_step: Option<(bool, bool)>,
}

/// Which of the four questions a trial is being opened to answer.
///
/// The settle body is one body; this is the whole of what differs between the
/// four, and each arm below says which rule it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::circuit) enum TrialKind {
    /// One Newton evaluation. Never commits, and rolls back before the caller
    /// sees it.
    Probe,
    /// The converged candidate, re-settled so its interior roots and its
    /// analog tasks can be read. Never commits: the engine either lands on a
    /// root it reports or accepts the point through a fresh trial.
    Inspection,
    /// The timepoint the integrator is advancing over. The one kind
    /// [`Trial::prepare`] answers for.
    Acceptance,
    /// The transient's t = 0 startup, and `.op` on a mixed deck, which is that
    /// startup with no advance after it: a probe with a zero interval, the
    /// wheel drained no further than the tick the operating point itself is on
    /// (tick 0), the instances' `initial` blocks executed and one settle.
    OperatingPoint,
}

impl TrialKind {
    /// Whether the instances and the wheel are told this trial can never be
    /// committed.
    ///
    /// A probe trial delivers a residual at a timepoint the solver is not
    /// committing, and `MixedSignalHost::accept_trial` refuses one by name, so
    /// no route to committing it exists to be taken by mistake. The inspection
    /// is *not* a probe in that sense: it is opened on the converged candidate
    /// under the same monotonicity rule an acceptance is, and it is rolled
    /// back by its owner rather than by its flag.
    const fn speculative(self) -> bool {
        matches!(self, Self::Probe | Self::OperatingPoint)
    }

    /// Whether the trial captures and restores the code-model image itself.
    ///
    /// Every kind but the acceptance does: an accepted candidate rides the
    /// engine's own outer XSPICE/resource journal, which is the one that has to
    /// survive the `finish` callback.
    const fn captures_xspice(self) -> bool {
        !matches!(self, Self::Acceptance)
    }

    /// Whether [`Trial::prepare`] may be asked for a reservation.
    const fn may_commit(self) -> bool {
        matches!(self, Self::Acceptance)
    }

    /// What the code models are told the run is doing.
    const fn analysis(self) -> crate::xspice::AnalysisType {
        match self {
            Self::OperatingPoint => crate::xspice::AnalysisType::DcOp,
            _ => crate::xspice::AnalysisType::Transient,
        }
    }

    /// Which evaluation phase a code model sees. Only the acceptance runs the
    /// `AcceptedStep` effects a model may not undo.
    const fn phase(self) -> crate::xspice::EvaluationPhase {
        match self {
            Self::Acceptance => crate::xspice::EvaluationPhase::AcceptedStep,
            _ => crate::xspice::EvaluationPhase::CircuitTrial,
        }
    }

    const fn describe(self) -> &'static str {
        match self {
            Self::Probe => "numerical probe",
            Self::Inspection => "candidate inspection",
            Self::Acceptance => "acceptance",
            Self::OperatingPoint => "operating-point",
        }
    }
}

/// One candidate, opened on the circuit's discrete lanes.
///
/// Drop is rollback. See the module documentation for what it owns and what it
/// gives back.
pub(in crate::circuit) struct Trial<'a> {
    circuit: &'a mut CircuitData,
    /// The shared wheel, taken out of the scheduler for the life of the trial.
    coordinator: Option<MixedDigitalCoordinator>,
    /// The mixed instances, taken out of the circuit for the life of the trial.
    hosts: Vec<MixedSignalHost>,
    /// This trial's place on the wheel. `None` once committed or rolled back.
    cursor: Option<SharedTrialCursor>,
    /// The code-model image a speculative kind restores itself.
    xspice: Option<XspiceAcceptanceRollback>,
    /// The engine's own resource transaction, on the acceptance path.
    outer_resources: Option<&'a ResourceTransaction>,
    /// Present exactly when a code model shares an event net with a mixed
    /// module, which is what makes the code-model lane part of this settle.
    bindings: Option<Arc<XspiceDigitalBindings>>,
    coefficients: CompanionCoefficients,
    xyce_one_step_order2: bool,
    kind: TrialKind,
    candidate: Candidate,
    /// Whether the instances still hold trials a refusal has to reject.
    hosts_active: bool,
}

impl CircuitData {
    /// Open one candidate on every discrete lane.
    ///
    /// Fallible up to and including the instances' own `begin`; from the first
    /// line of this function every exit restores what it took, because the
    /// [`Trial`] is constructed before anything that can refuse.
    pub(in crate::circuit) fn open_trial<'a>(
        &'a mut self,
        candidate: Candidate,
        kind: TrialKind,
        companion: XspiceCompanionPolicy<'_>,
        outer_resources: Option<&'a ResourceTransaction>,
    ) -> Result<Trial<'a>, SimulationError> {
        let integration = mixed_integration_coefficients(candidate.time, candidate.dt, companion)?;
        let bindings = self.scheduler.mixed_xspice_bindings.clone();
        let coordinator = self
            .scheduler
            .mixed_digital_coordinator
            .take()
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "mixed circuit digital execution has not been elaborated".into(),
                )
            })?;
        let mut hosts = std::mem::take(&mut self.mixed_signal_hosts);
        // Lend every instance the candidate ledger the scheduler parked for it.
        // The swap leaves the instances' place-holders in the parked vector,
        // and the trial's `Drop` swaps them back.
        self.scheduler
            .ledgers
            .resize_with(hosts.len(), Default::default);
        swap_candidate_ledgers(&mut hosts, &mut self.scheduler.ledgers);
        let xspice = (kind.captures_xspice() && self.has_coupled_event_nets())
            .then(|| self.capture_xspice_acceptance());
        let mut trial = Trial {
            circuit: self,
            coordinator: Some(coordinator),
            hosts,
            cursor: None,
            xspice,
            outer_resources,
            bindings,
            coefficients: *companion.coefficients,
            xyce_one_step_order2: companion.xyce_one_step_order2,
            kind,
            candidate,
            hosts_active: false,
        };
        trial.begin(integration)?;
        Ok(trial)
    }
}

impl Trial<'_> {
    /// Open the wheel and every instance at this candidate.
    fn begin(&mut self, integration: VerilogACompanionRules) -> Result<(), SimulationError> {
        let cursor = self
            .coordinator
            .as_mut()
            .expect("an open trial owns its coordinator")
            .open_trial(self.candidate.time, self.kind.speculative())
            .map_err(shared_error)?;
        let scheduled_activation = cursor.opened_on_scheduled_activation();
        self.cursor = Some(cursor);
        if let Some(host) = self.hosts.iter().find(|host| host.trial_active()) {
            return Err(mixed_error(
                host.instance_name(),
                MixedSignalError::TrialProtocol {
                    detail: "a circuit trial cannot overlap an existing model trial".into(),
                },
            ));
        }
        self.hosts_active = true;
        for host in self.hosts.iter_mut() {
            let (initial, final_step) = self
                .candidate
                .analysis_step
                .unwrap_or_else(|| host.analysis_step());
            let started = host.begin_trial_with_integration_rules(
                self.candidate.time,
                self.candidate.dt,
                integration.derivative,
                integration.state,
                initial,
                final_step,
                self.kind.speculative(),
            );
            named(host, started)?;
        }
        // Told to the instances only where the answer is used: a boundary move
        // at a timepoint a process was due to run at is the schedule's, and the
        // accepted-flip ceiling does not count it. An enrolled instance holds
        // no queue and cannot read this for itself.
        if self.kind.may_commit() && scheduled_activation {
            for host in self.hosts.iter_mut() {
                host.note_scheduled_activation();
            }
        }
        Ok(())
    }

    /// Whether the code-model lane takes part in this trial's settle.
    pub(in crate::circuit) fn has_code_models(&self) -> bool {
        self.bindings.is_some()
    }

    /// The circuit the trial was opened on, with the wheel and the instances
    /// still held out of it.
    pub(in crate::circuit) fn circuit_mut(&mut self) -> &mut CircuitData {
        &mut *self.circuit
    }

    /// The instances this trial holds, in circuit registration order.
    pub(in crate::circuit) fn hosts_mut(&mut self) -> &mut [MixedSignalHost] {
        &mut self.hosts
    }

    /// What this trial's settle published, as a record two kinds can be
    /// compared by: the shared wheel's published tick, and per instance and
    /// A/D bridge whether this trial moved it and the instant it crossed at.
    ///
    /// Test-only. Nothing in the engine reads a settle back — the publications
    /// are consumed inside it — so this exists for the one pin that asserts the
    /// four kinds share one settle body.
    #[cfg(test)]
    #[allow(clippy::type_complexity)]
    pub(in crate::circuit) fn settle_record(&self) -> (u64, Vec<Vec<(bool, Option<Value>)>>) {
        (
            self.cursor
                .as_ref()
                .map_or(0, SharedTrialCursor::published_tick),
            self.hosts
                .iter()
                .map(MixedSignalHost::trial_boundary_record)
                .collect(),
        )
    }

    /// Settle this candidate to quiet, opening the code-model lane when the
    /// circuit is coupled.
    ///
    /// The one settle of a probe, an inspection, an operating point and an
    /// uncoupled acceptance; the coupled acceptance reaches the same body once
    /// per projection pass through [`Self::settle_projection_pass`].
    pub(in crate::circuit) fn settle(&mut self, solution: &[Value]) -> Result<(), SimulationError> {
        let Some(bindings) = self.bindings.clone() else {
            let Self {
                coordinator,
                cursor,
                hosts,
                ..
            } = self;
            return settle_lanes(
                coordinator.as_mut().expect("an open trial owns its wheel"),
                cursor.as_mut().expect("an open trial owns its cursor"),
                hosts,
                solution,
                None,
            );
        };
        let Self {
            circuit,
            coordinator,
            cursor,
            hosts,
            xspice,
            outer_resources,
            coefficients,
            xyce_one_step_order2,
            kind,
            candidate,
            ..
        } = self;
        let resources = match xspice.as_ref() {
            Some(snapshot) => Some(snapshot.resources()),
            None => *outer_resources,
        };
        let mut participant = XspiceDigitalParticipant::new(
            circuit,
            &bindings,
            solution,
            candidate.time,
            candidate.dt,
            kind.analysis(),
            kind.phase(),
            XspiceCompanionPolicy {
                coefficients: &*coefficients,
                xyce_one_step_order2: *xyce_one_step_order2,
            },
            resources,
        );
        settle_lanes(
            coordinator.as_mut().expect("an open trial owns its wheel"),
            cursor.as_mut().expect("an open trial owns its cursor"),
            hosts,
            solution,
            Some(&mut participant),
        )
    }

    /// One pass of the accepted step's voltage projection.
    ///
    /// The code-model wave is carried across the passes rather than reopened,
    /// and that is the reason an accepted step dispatches each instance once: a
    /// pass that re-settles the boundary after moving the solution resumes the
    /// candidate's own Active wave, so it evaluates the instances whose inputs
    /// the projection invalidated instead of every instance in the circuit.
    /// Re-running the rest would not ask the same question twice — it would run
    /// their `AcceptedStep` effects twice, and a model that requests a
    /// breakpoint, writes a transactional resource or advances external state
    /// does not undo the first one.
    pub(in crate::circuit) fn settle_projection_pass(
        &mut self,
        solution: &[Value],
        pass: u32,
        moved: &[usize],
        wave: Option<XspiceActiveWave>,
    ) -> Result<Option<XspiceActiveWave>, SimulationError> {
        let bindings = self
            .bindings
            .clone()
            .expect("a projection pass belongs to a coupled acceptance");
        let Self {
            circuit,
            coordinator,
            cursor,
            hosts,
            outer_resources,
            coefficients,
            xyce_one_step_order2,
            kind,
            candidate,
            ..
        } = self;
        let companion = XspiceCompanionPolicy {
            coefficients: &*coefficients,
            xyce_one_step_order2: *xyce_one_step_order2,
        };
        let mut participant = if pass == 0 {
            XspiceDigitalParticipant::new(
                circuit,
                &bindings,
                solution,
                candidate.time,
                candidate.dt,
                kind.analysis(),
                kind.phase(),
                companion,
                *outer_resources,
            )
        } else {
            XspiceDigitalParticipant::resume(
                circuit,
                &bindings,
                solution,
                candidate.time,
                candidate.dt,
                kind.analysis(),
                kind.phase(),
                companion,
                *outer_resources,
                wave,
                moved,
            )
        };
        settle_lanes(
            coordinator.as_mut().expect("an open trial owns its wheel"),
            cursor.as_mut().expect("an open trial owns its cursor"),
            hosts,
            solution,
            Some(&mut participant),
        )?;
        Ok(participant.into_wave())
    }

    /// Stamp every instance's continuous half and its physical D/A bridges into
    /// the assembly this trial was opened for.
    ///
    /// Only HDL equations and their physical D/A bridges receive `weight`.
    /// Enrolled XSPICE stamps have already applied their own policy.
    pub(in crate::circuit) fn stamp(
        &mut self,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        weight: Value,
    ) -> Result<(), SimulationError> {
        for host in self.hosts.iter_mut() {
            let stamped = host.stamp_trial(
                solution,
                |row, col, value| {
                    if matrix.get_index(row, col).is_some() {
                        matrix.add(row, col, weight * value);
                    } else {
                        log::debug!(
                            "mixed Verilog-AMS stamp ({row}, {col}) missing from matrix topology"
                        );
                    }
                },
                |row, value| {
                    if let Some(slot) = rhs.get_mut(row) {
                        *slot += weight * value;
                    }
                },
            );
            named(host, stamped)?;
        }
        Ok(())
    }

    /// Evaluate every instance against the settled candidate without stamping
    /// it anywhere, and report whether any analog half newly raised
    /// `$discontinuity`.
    pub(in crate::circuit) fn evaluate(
        &mut self,
        solution: &[Value],
    ) -> Result<bool, SimulationError> {
        let mut discontinuity = false;
        for host in self.hosts.iter_mut() {
            let stamped = host.stamp(solution, |_, _, _| {}, |_, _| {});
            named(host, stamped)?;
            discontinuity |= host.candidate_discontinuity();
        }
        Ok(discontinuity)
    }

    /// The earliest A/D crossing this settled trial placed strictly inside its
    /// own interval, across every instance. The engine lands on it.
    ///
    /// Read from the trial rather than predicted before one was opened, because
    /// whether a D/A bridge moved in this interval — the fact that decides
    /// whether a crossing is interior at all — is only established by running
    /// the discrete half. The trial rolls back either way, so nothing this
    /// settle published survives the refusal.
    ///
    /// The window a crossing has to fall inside to *be* the endpoint is
    /// `endpoint_root_window`, derived in the one place it is written; each
    /// instance applies it to its own accepted time, which is the far end of
    /// the interval its own crossings were interpolated in.
    pub(in crate::circuit) fn interior_root(
        &self,
        minimum_timestep: Value,
    ) -> Result<Option<Value>, SimulationError> {
        let mut earliest: Option<Value> = None;
        for host in self.hosts.iter() {
            if let Some(target) =
                named(host, host.trial_boundary_refinement_time(minimum_timestep))?
            {
                earliest = Some(earliest.map_or(target, |current: Value| current.min(target)));
            }
        }
        Ok(earliest)
    }

    /// The static residual every instance contributes at the settled
    /// candidate, as one circuit-sized vector.
    pub(in crate::circuit) fn static_residual(
        &mut self,
        solution: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        let mut action = vec![0.0; solution.len()];
        let mut rhs = vec![0.0; solution.len()];
        for host in self.hosts.iter_mut() {
            let (mut bad_matrix, mut bad_rhs) = (false, false);
            let stamped = host.stamp_static_dae(
                solution,
                |row, col, value| {
                    if let (Some(slot), Some(voltage)) = (action.get_mut(row), solution.get(col)) {
                        *slot += value * voltage;
                    } else {
                        bad_matrix = true;
                    }
                },
                |row, value| {
                    if let Some(slot) = rhs.get_mut(row) {
                        *slot += value;
                    } else {
                        bad_rhs = true;
                    }
                },
            );
            named(host, stamped)?;
            if bad_matrix || bad_rhs {
                return Err(SimulationError::Circuit(format!(
                    "mixed Verilog-AMS instance '{}' static history stamp exceeds the circuit topology",
                    host.instance_name()
                )));
            }
        }
        for (value, rhs) in action.iter_mut().zip(rhs) {
            *value -= rhs;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(
                    "mixed static residual is not finite".into(),
                ));
            }
        }
        Ok(action)
    }

    /// Validate every lane and reserve this candidate.
    ///
    /// Validate-all-then-commit: nothing here promotes anything, so a later
    /// instance's refusal cannot leave an earlier one committed. Dropping the
    /// reservation unwinds every instance it holds and the trial's own `Drop`
    /// then puts the wheel back.
    pub(in crate::circuit) fn prepare(&mut self) -> Result<Prepared<'_>, SimulationError> {
        if !self.kind.may_commit() {
            return Err(SimulationError::Circuit(format!(
                "a {} trial cannot be prepared for acceptance; it exists so a candidate can be \
                 assembled or inspected at a timepoint the solver is not committing",
                self.kind.describe()
            )));
        }
        let Self {
            circuit,
            coordinator,
            cursor,
            hosts,
            hosts_active,
            ..
        } = self;
        let mut reservations = Vec::with_capacity(hosts.len());
        for host in hosts.iter_mut() {
            reservations.push(
                host.prepare_circuit_acceptance()
                    .map_err(|(instance, error)| mixed_error(&instance, error))?,
            );
        }
        Ok(Prepared {
            circuit,
            coordinator,
            cursor,
            hosts_active,
            reservations,
        })
    }

    /// Close a speculative trial, reporting a code-model image that could not
    /// be put back.
    ///
    /// The refusal is reported rather than logged because a circuit whose
    /// transactional resources did not restore is latched: the next candidate
    /// would be solved against state a rolled-back probe left behind.
    pub(in crate::circuit) fn finish<T>(
        mut self,
        result: Result<T, SimulationError>,
    ) -> Result<T, SimulationError> {
        self.unwind();
        match (result, self.restore_xspice()) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
            (Err(error), Err(restore)) => {
                Err(SimulationError::Circuit(format!("{error}; {restore}")))
            }
        }
    }

    /// Reject every instance trial still open and hand the wheel's image back.
    /// Idempotent, so [`Self::finish`] and [`Drop`] can both call it.
    fn unwind(&mut self) {
        if self.hosts_active {
            for host in self.hosts.iter_mut() {
                if host.trial_active() {
                    let _ = host.reject_trial();
                }
            }
            self.hosts_active = false;
        }
        if let (Some(coordinator), Some(cursor)) = (self.coordinator.as_mut(), self.cursor.take()) {
            coordinator.rollback_trial(cursor);
        }
    }

    fn restore_xspice(&mut self) -> Result<(), SimulationError> {
        if let Some(snapshot) = self.xspice.take() {
            self.circuit
                .restore_xspice_acceptance(snapshot)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        Ok(())
    }
}

impl Drop for Trial<'_> {
    fn drop(&mut self) {
        self.unwind();
        // An explicit finish reports restoration errors. Unwinding still
        // restores every context/queue; a resource failure latches the circuit.
        if let Err(error) = self.restore_xspice() {
            log::error!("{error}");
        }
        let mut ledgers = std::mem::take(&mut self.circuit.scheduler.ledgers);
        swap_candidate_ledgers(&mut self.hosts, &mut ledgers);
        self.circuit.scheduler.ledgers = ledgers;
        self.circuit.scheduler.mixed_digital_coordinator = self.coordinator.take();
        self.circuit.mixed_signal_hosts = std::mem::take(&mut self.hosts);
    }
}

/// A validated candidate, reserved on every lane and not yet promoted.
///
/// Holds disjoint borrows of the trial's own fields rather than the trial, so
/// the engine's `finish` callback can still reach the circuit between the
/// validation and the promotion — which is exactly where it has to run.
pub(in crate::circuit) struct Prepared<'t> {
    circuit: &'t mut CircuitData,
    coordinator: &'t mut Option<MixedDigitalCoordinator>,
    cursor: &'t mut Option<SharedTrialCursor>,
    hosts_active: &'t mut bool,
    reservations: Vec<PreparedMixedAcceptance<'t>>,
}

impl Prepared<'_> {
    /// The circuit, for the caller's remaining fallible preparation.
    pub(in crate::circuit) fn circuit_mut(&mut self) -> &mut CircuitData {
        &mut *self.circuit
    }

    /// Promote every lane. Infallible by construction: everything that could
    /// refuse refused in [`Trial::prepare`].
    pub(in crate::circuit) fn commit(self) {
        for reservation in self.reservations {
            reservation.commit();
        }
        *self.hosts_active = false;
        if let (Some(coordinator), Some(cursor)) = (self.coordinator.as_mut(), self.cursor.take()) {
            coordinator.commit_trial(cursor);
        }
    }
}

/// Convert the engine's companion coefficients into the runtime's integration
/// coefficients.
///
/// The same arithmetic `prepare_veriloga_timepoint` does, because a mixed
/// module's continuous half is integrated by the same runtime as an analog
/// instance's and must be handed the same numbers — including the same refusal
/// of an interval the companion rule cannot represent. A zero interval is the
/// operating point and carries no rule.
fn mixed_integration_coefficients(
    time: Value,
    dt: Value,
    companion: XspiceCompanionPolicy<'_>,
) -> Result<VerilogACompanionRules, SimulationError> {
    VerilogACompanionRules::from_policy(dt, companion).map_err(|error| {
        SimulationError::Circuit(format!(
            "mixed Verilog-AMS modules cannot advance to t={time:.16e}s: {error}"
        ))
    })
}

/// The settle, written once.
///
/// Advance every lane to the trial's floored tick, sample every instance's
/// bridges against the candidate solution, publish the A/D decisions in
/// ascending crossing order and run the participants to quiet.
fn settle_lanes(
    coordinator: &mut MixedDigitalCoordinator,
    cursor: &mut SharedTrialCursor,
    hosts: &mut [MixedSignalHost],
    solution: &[Value],
    mut participant: Option<&mut dyn DigitalActiveParticipant>,
) -> Result<(), SimulationError> {
    match &mut participant {
        Some(participant) => {
            coordinator.advance_with(cursor, hosts, solution, Some(&mut **participant))
        }
        None => coordinator.advance(cursor, hosts, solution),
    }
    .map_err(shared_error)?;
    coordinator.synchronize(hosts).map_err(shared_error)?;
    for _ in 0..MAX_BOUNDARY_SETTLE_PASSES {
        // The coordinator ran the wheel for every enrolled instance at
        // once, so whether that run moved something the analog equations
        // read — a D/A output, or a discrete variable an analog block
        // references — is a question about the whole circuit rather than
        // about any one instance: one instance's bridge and another's A/D
        // input can share a deck node, and one instance's variable steers
        // the current it pushes into a node any other may sense. Ask every
        // instance whether its own digital half moved one, and report the
        // disjunction to all of them before any of their bridges are
        // sampled.
        //
        // Asked before every pass rather than once before the first,
        // because the write that matters is usually made by a process
        // *this trial's own publications* woke, which happens inside a
        // pass. [`MixedSignalHost::settle_analog_bridges`] asks the same
        // question of the instance's own store at the top of every pass,
        // for the same reason; an instance that holds no analog equation
        // of its own — a sampler whose only port is an A/D input — has no
        // store of its own to answer it with, and the circuit's answer is
        // the only one it can ever get.
        //
        // After a synchronize either way, because an enrolled instance
        // reads its boundary through a view and that is what refreshes
        // one: asking before it compares the view against itself and
        // always answers no. The first pass is preceded by the one above,
        // every later pass by the loop's own.
        let mut fed_back = false;
        for host in hosts.iter() {
            fed_back |= named(host, host.digital_feedback_since_trial_start())?;
        }
        if fed_back {
            for host in hosts.iter_mut() {
                host.note_shared_digital_feedback();
            }
        }
        let mut moved = false;
        for host in hosts.iter_mut() {
            let settled = host.settle_analog_bridges(solution);
            moved |= named(host, settled)?;
        }
        // All A/D decisions are published before any dependent HDL process
        // runs. Analog equations read the resulting bank only after quiet.
        moved |= match &mut participant {
            Some(participant) => {
                coordinator.publish_adc_with(cursor, hosts, solution, Some(&mut **participant))
            }
            None => coordinator.publish_adc(cursor, hosts, solution),
        }
        .map_err(shared_error)?;
        moved |= coordinator.synchronize(hosts).map_err(shared_error)?;
        if !moved {
            return Ok(());
        }
    }
    let host = &hosts[0];
    Err(mixed_error(
        host.instance_name(),
        host.boundary_settle_oscillation(MAX_BOUNDARY_SETTLE_PASSES),
    ))
}
