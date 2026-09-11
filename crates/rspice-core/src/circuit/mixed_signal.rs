//! Where a mixed Verilog-AMS module's running host lives, and how the transient
//! solver drives it.
//!
//! [`MixedSignalHost`](crate::xspice::verilog::MixedSignalHost) owns the
//! interleave — the digital time wheel, the A/D and D/A bridges, and the
//! transactional trial. This module is the two ends of the wire between that
//! host and the engine: storage beside the XSPICE instances that answer the
//! same questions, and the four solver-facing operations the transient stepper
//! calls.
//!
//! # The bracket, and why it is XSPICE's
//!
//! `stamp_xspice_transient_trial_with_coefficients` snapshots, evaluates,
//! stamps, and restores — all inside one call — so a Newton iteration sees an
//! XSPICE model's contribution without any of its state having moved. The mixed
//! host is driven the same way, with its own transaction standing in for the
//! snapshot:
//!
//! * [`CircuitData::stamp_mixed_transient_trial`] opens a *probe* trial,
//!   settles the boundary, stamps, and rolls the whole trial back. Nothing a
//!   Newton iteration or a rejected step *committed* survives it, which is D5
//!   clause 1 at the level the engine can check: a rejected timepoint moved no
//!   accepted state in either domain, so a run that rejects and a run that does
//!   not produce the same accepted trajectory bit for bit. What it leaves
//!   behind in the analog device is candidate state — the same candidate state
//!   an ordinary `VerilogADevice` carries between the Newton iterations of one
//!   timepoint, recomputed from the accepted record by the next evaluation. See
//!   `MixedSignalHost::analog`.
//! * [`CircuitData::accept_mixed_and_analog_transient_timestep`] opens the one trial that
//!   *is* committable, evaluates the module against the solution the engine
//!   kept, settles the boundary to quiet, and commits both domains atomically.
//!
//! Circuit-facing calls return only committed or restored hosts. Exclusive
//! acceptance reservations span participants inside one call; dropping them
//! restores speculative state. Their mutable borrows prevent a circuit clone
//! or another evaluation from observing a partially prepared barrier.
//!
//! # The breakpoint
//!
//! [`CircuitData::next_mixed_event_time`] is the host's next scheduled digital
//! activation in seconds, and it joins `next_xspice_event_time` in the runtime
//! breakpoint list the stepper replaces after every accepted point. That is D5
//! clause 2 through the existing seam: the step controller stops bit-exactly on
//! a digital event because the event is a breakpoint, not because a parallel
//! mechanism clamped the step behind its back.

use crate::circuit::CircuitData;
use crate::{SimulationError, Value};

use crate::xspice::verilog::{BoundaryBus, MixedSignalError, MixedSignalHost};

/// How many times one trial may re-settle its boundary before the engine gives
/// up on it.
///
/// The host has its own ceiling — the scheduler's delta-cycle cap, ten thousand
/// by default — but that one measures the depth of a *digital* settling and is
/// sized for it. A boundary that will not quiet is a different failure: two
/// bridges driving each other across the domain wall, which either resolves in
/// a couple of passes or never. Capping it here keeps a Newton iteration from
/// paying ten thousand digital settles to learn that.
const MAX_BOUNDARY_SETTLE_PASSES: u32 = 64;

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
    coefficients: &crate::numerics::integration::CompanionCoefficients,
) -> Result<rspice_veriloga::vm::IntegrationCoefficients, SimulationError> {
    rspice_veriloga_runtime::GeneratedDdtCoefficients::from_companion_values_with_derivative_scale(
        coefficients.coeff_g,
        coefficients.coeff_v_n,
        coefficients.coeff_v_n_minus_1,
        coefficients.needs_two_history,
        coefficients.coeff_i_n,
        dt,
    )
    .map(Into::into)
    .map_err(|error| {
        SimulationError::Circuit(format!(
            "mixed Verilog-AMS modules cannot advance to t={time:.16e}s: {error}"
        ))
    })
}

fn mixed_error(instance: &str, error: MixedSignalError) -> SimulationError {
    SimulationError::Circuit(format!("mixed Verilog-AMS instance '{instance}': {error}"))
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
fn named<T>(
    host: &MixedSignalHost,
    result: Result<T, MixedSignalError>,
) -> Result<T, SimulationError> {
    result.map_err(|error| mixed_error(host.instance_name(), error))
}

/// Repeat the boundary settle until it reports itself quiet.
///
/// A settle that moved a D/A input owes the analog solver another Newton pass
/// at this timestamp, and the host will refuse to commit until one reports the
/// boundary quiet. Inside one trial the loop is the driver's job, which is what
/// `MixedSignalHost::settle_analog_bridges`'s own documentation says.
///
/// The refusal is left unnamed for the caller to name, so that a settle inside
/// a longer chain does not have to know how the chain reports its instance.
fn settle_to_quiet(host: &mut MixedSignalHost, voltages: &[Value]) -> Result<(), MixedSignalError> {
    for _ in 0..MAX_BOUNDARY_SETTLE_PASSES {
        if !host.settle_analog_bridges(voltages)? {
            return Ok(());
        }
    }
    // Named participants rather than a sentence about bridges in general. The
    // host builds the diagnostic because the host is what knows which nets
    // moved; this loop knows only that they kept moving.
    Err(host.boundary_settle_oscillation(MAX_BOUNDARY_SETTLE_PASSES))
}

impl CircuitData {
    /// Enrich a terminal solver failure without changing convergence recovery
    /// or accepting any speculative boundary state.
    pub(crate) fn annotate_mixed_convergence_failure(
        &self,
        error: SimulationError,
        time: Value,
    ) -> SimulationError {
        if !matches!(error, SimulationError::ConvergenceFailed(_)) {
            return error;
        }
        let mut context = self
            .mixed_signal_hosts
            .iter()
            .filter_map(|host| host.rejected_probe_activity(time));
        let Some(first) = context.next() else {
            return error;
        };
        let mut detail = format!("{error}; {first}");
        // Bound the rendered context even when a design has many instances.
        for item in context.by_ref().take(7) {
            detail.push_str("; ");
            detail.push_str(&item);
        }
        if context.next().is_some() {
            detail.push_str("; additional switching instances omitted");
        }
        SimulationError::Circuit(detail)
    }

    /// Whether any mixed Verilog-AMS module is instantiated.
    #[inline]
    pub(crate) fn has_mixed_signal_hosts(&self) -> bool {
        !self.mixed_signal_hosts.is_empty()
    }

    /// Register one elaborated mixed module.
    pub(crate) fn add_mixed_signal_host(&mut self, mut host: MixedSignalHost) {
        host.set_simulation_parameters(self.generated_simulation_parameters);
        self.mixed_signal_hosts.push(host);
    }

    /// Check the existing electrical-boundary adapter against the complete
    /// event topology, including instances created after a mixed module.
    /// Direct event bindings require the circuit scheduling coordinator; the
    /// current host must not accidentally reach them through an analog bridge.
    pub(crate) fn validate_mixed_event_connections(&self) -> Result<(), SimulationError> {
        for host in &self.mixed_signal_hosts {
            for (signal, node) in host.boundary_connections() {
                let kind = self.net_kinds.kind(node);
                if !kind.is_discrete() {
                    continue;
                }
                let node_names = self.node_names_sorted();
                let node_name = node_names
                    .get(node - 1)
                    .map(String::as_str)
                    .unwrap_or("<unnamed>");
                return Err(SimulationError::Circuit(format!(
                    "mixed Verilog-AMS instance '{}' connects its discrete port '{}' to node '{}', \
                     which carries {} event-driven XSPICE values. Direct event connections \
                     require a shared circuit event scheduler; this instance currently uses \
                     an electrical boundary",
                    host.instance_name(),
                    signal,
                    node_name,
                    kind.description(),
                )));
            }
        }
        Ok(())
    }

    /// Begin digital execution only after the engine has delivered every
    /// model's analog initialization effects and ruled out a requested exit.
    pub(crate) fn start_mixed_digital_execution(&mut self) -> Result<(), SimulationError> {
        for host in &mut self.mixed_signal_hosts {
            let started = host.start_digital_execution();
            named(host, started)?;
        }
        Ok(())
    }

    /// Every circuit node the mixed modules' contributions can reach.
    pub(crate) fn mixed_signal_coupled_nodes(&self) -> impl Iterator<Item = Vec<usize>> + '_ {
        self.mixed_signal_hosts
            .iter()
            .map(MixedSignalHost::coupled_nodes)
    }

    /// Every bus the mixed modules' vector boundary ports declare over deck
    /// nodes, in instantiation order.
    ///
    /// Wiring data the builder recorded, handed on unchanged: this says which
    /// deck nodes are one word, and naming them is the caller's job because
    /// only the analysis holds the run's node-name table.
    pub(crate) fn mixed_signal_boundary_buses(&self) -> impl Iterator<Item = &BoundaryBus> + '_ {
        self.mixed_signal_hosts
            .iter()
            .flat_map(MixedSignalHost::boundary_buses)
    }

    /// Refuse an analysis this route does not implement.
    ///
    /// The mixed host executes a *transient* interleave: its digital half is a
    /// time wheel and its bridges are sampled between accepted timepoints.
    /// There is no small-signal linearization of a process, so an AC, noise,
    /// or harmonic-balance assembly has nothing to ask it for — and assembling
    /// one anyway would silently omit the module's continuous half too, which
    /// is a plausible answer to a question the deck did not ask.
    pub(crate) fn ensure_no_mixed_signal_hosts(
        &self,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        let Some(host) = self.mixed_signal_hosts.first() else {
            return Ok(());
        };
        Err(SimulationError::Circuit(format!(
            "mixed Verilog-AMS instance '{}' cannot take part in {analysis}: the module's \
             discrete half is executed by a transient event interleave, which has no \
             small-signal or steady-state form. Only `.tran` runs a mixed module",
            host.instance_name()
        )))
    }

    /// Stamp every mixed module for one Newton evaluation, committing nothing.
    ///
    /// The trial is a probe from end to end: it is opened, settled, stamped and
    /// rolled back inside this call, so the module's accepted state on return
    /// is exactly its accepted state on entry. See this module's documentation
    /// for why that is the whole of the rollback contract at deck level.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stamp_mixed_transient_trial(
        &mut self,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        dt: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), SimulationError> {
        self.stamp_mixed_trial(
            matrix,
            rhs,
            time,
            dt,
            voltages,
            coefficients,
            Some((initial_step, final_step)),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn stamp_mixed_trial(
        &mut self,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        dt: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        analysis_step: Option<(bool, bool)>,
    ) -> Result<(), SimulationError> {
        let integration = mixed_integration_coefficients(time, dt, coefficients)?;
        for host in &mut self.mixed_signal_hosts {
            let (initial_step, final_step) = analysis_step.unwrap_or_else(|| host.analysis_step());
            let started = host.begin_probe_trial(time, dt, integration, initial_step, final_step);
            named(host, started)?;
            let stamped = settle_to_quiet(host, voltages).and_then(|()| {
                host.stamp(
                    voltages,
                    |row, col, value| {
                        if matrix.get_index(row, col).is_some() {
                            matrix.add(row, col, value);
                        } else {
                            log::debug!(
                                "mixed Verilog-AMS stamp ({row}, {col}) missing from matrix topology"
                            );
                        }
                    },
                    |row, value| {
                        if let Some(slot) = rhs.get_mut(row) {
                            *slot += value;
                        }
                    },
                )
            });
            // The rollback runs whether or not the stamp succeeded, so a
            // refused evaluation leaves no half-advanced module behind for the
            // next call to inherit.
            let rolled_back = host.reject_trial();
            named(host, stamped)?;
            named(host, rolled_back)?;
        }
        Ok(())
    }

    /// Stamp every mixed module into an operating-point assembly.
    ///
    /// The interleave has no separate DC form and does not need one: a mixed
    /// module's continuous half is a set of equations that a zero timestep
    /// makes memoryless, and its D/A levels come from whatever the discrete
    /// half's `initial` blocks and continuous drivers settled to at time zero —
    /// which is IEEE 1364-2005's own answer to what a design holds before the
    /// first event. The trial is a probe like every other, so an operating
    /// point that is solved several times over does not advance the module once.
    pub(crate) fn stamp_mixed_operating_point(
        &mut self,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) -> Result<(), SimulationError> {
        self.stamp_mixed_trial(
            matrix,
            rhs,
            time,
            0.0,
            solution,
            &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            None,
        )
    }

    /// Validate every mixed and analog model, then promote all their states.
    /// A later participant cannot leave an earlier participant committed.
    ///
    /// The analog half is evaluated once more against the solution the engine
    /// kept — the same thing `evaluate_veriloga_timepoint` does for an analog
    /// instance, and for the same reason: the integrator commits the state of
    /// the last evaluation, so that evaluation has to be the accepted one. The
    /// boundary is then settled to quiet and both domains commit together.
    /// Returns whether an analog half newly raised `$discontinuity`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn accept_mixed_and_analog_transient_timestep(
        &mut self,
        time: Value,
        dt: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<bool, SimulationError> {
        let integration = mixed_integration_coefficients(time, dt, coefficients)?;
        self.validate_nonmixed_model_acceptance()
            .map_err(SimulationError::Circuit)?;
        let mut discontinuity = self.veriloga_discontinuity_rising();
        let mut prepared = Vec::with_capacity(self.mixed_signal_hosts.len());
        for host in &mut self.mixed_signal_hosts {
            let started = host.begin_trial(time, dt, integration, initial_step, final_step);
            named(host, started)?;
            let evaluated = settle_to_quiet(host, voltages)
                .and_then(|()| host.stamp(voltages, |_, _, _| {}, |_, _| {}));
            if let Err(error) = evaluated {
                let failure = mixed_error(host.instance_name(), error);
                if host.trial_active() {
                    let _ = host.reject_trial();
                }
                return Err(failure);
            }
            discontinuity |= host.analog_device().discontinuity_rising();
            prepared.push(
                host.prepare_circuit_acceptance()
                    .map_err(|(instance, error)| mixed_error(&instance, error))?,
            );
        }
        // Every fallible evaluation and acceptance check is finished. Earlier
        // reservations roll back automatically if a later host refused above.
        for candidate in prepared {
            candidate.commit();
        }
        self.veriloga_devices.apply_validated_timestep_acceptance();
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .apply_validated_state_acceptance();
        Ok(discontinuity)
    }

    /// Inspect control calls at the actual candidate solution. Numerical
    /// probes may have used other voltages, and their journals are speculative.
    /// After all roots are resolved, `validate_acceptance` checks every host
    /// on a copy before final-step equations replace a finishing candidate.
    /// Ordinary numerical candidates require no copies here.
    /// Returns the earliest refinement time and any candidate discontinuity.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn visit_mixed_transient_candidate_task(
        &mut self,
        time: Value,
        dt: Value,
        voltages: &[Value],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
        kind: rspice_veriloga_runtime::AnalogTaskKind,
        validate_acceptance: bool,
        consume: &mut dyn FnMut(rspice_veriloga_runtime::AnalogTaskEvent<'_>),
    ) -> Result<(Option<Value>, bool), SimulationError> {
        let integration = mixed_integration_coefficients(time, dt, coefficients)?;
        let mut refinement: Option<Value> = None;
        let mut discontinuity = false;
        for host in &mut self.mixed_signal_hosts {
            let started = host.begin_trial(time, dt, integration, initial_step, final_step);
            named(host, started)?;
            let inspected = (|| {
                settle_to_quiet(host, voltages)?;
                host.stamp(voltages, |_, _, _| {}, |_, _| {})?;
                discontinuity |= host.analog_device().discontinuity_rising();
                let target = host
                    .analog_device()
                    .try_transient_event_refinement_time()
                    .map_err(|error| MixedSignalError::Analog {
                        detail: error.to_string(),
                    })?;
                if let Some(target) = target {
                    refinement = Some(refinement.map_or(target, |current| current.min(target)));
                } else if validate_acceptance {
                    let mut accepted = host.clone();
                    accepted.accept_trial()?;
                } else if let Some(event) = host
                    .analog_device()
                    .first_candidate_analog_task(kind)
                    .map_err(|error| MixedSignalError::Analog {
                    detail: error.to_string(),
                })? {
                    consume(event);
                }
                Ok(())
            })();
            let rolled_back = host.reject_trial();
            named(host, inspected)?;
            named(host, rolled_back)?;
        }
        Ok((refinement, discontinuity))
    }

    /// Earliest scheduled digital activation across every mixed module.
    pub(crate) fn next_mixed_event_time(&self) -> Result<Option<Value>, SimulationError> {
        let mut earliest: Option<Value> = None;
        for host in &self.mixed_signal_hosts {
            let Some(time) = host
                .next_event_time()
                .map_err(|error| mixed_error(host.instance_name(), error))?
            else {
                continue;
            };
            earliest = Some(earliest.map_or(time, |current: Value| current.min(time)));
        }
        Ok(earliest)
    }

    /// Append every mixed module's committed boundary values to a digital
    /// snapshot.
    ///
    /// Written into the same vector `fill_xspice_digital_snapshot` fills, and
    /// sorted with it, so `TransientResult::record_digital_snapshot` stays the
    /// single writer of the digital trace channel. One node carries one bit,
    /// because a bridge carries one bit: a vector boundary port is bridged as
    /// one net per conductor, so the deck node a bit landed on is what the
    /// snapshot records, and the declaration that says which of them were one
    /// word rides beside the traces rather than inside them.
    pub(crate) fn append_mixed_digital_snapshot(
        &self,
        snapshot: &mut Vec<(crate::circuit::NodeId, crate::xspice::DigitalValue)>,
    ) {
        use rspice_veriloga::four_state::FourStateBit;

        for host in &self.mixed_signal_hosts {
            host.boundary_digital_values(|node, bit| {
                if node == 0 {
                    return;
                }
                let state = match bit {
                    FourStateBit::Zero => crate::xspice::DigitalState::Zero,
                    FourStateBit::One => crate::xspice::DigitalState::One,
                    FourStateBit::Unknown => crate::xspice::DigitalState::Unknown,
                    FourStateBit::HighImpedance => crate::xspice::DigitalState::HighZ,
                };
                snapshot.push((
                    node,
                    crate::xspice::DigitalValue {
                        state,
                        strength: crate::xspice::DigitalStrength::Strong,
                    },
                ));
            });
        }
    }
}
