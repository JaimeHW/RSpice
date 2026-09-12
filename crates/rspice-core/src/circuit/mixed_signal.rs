//! Circuit-wide mixed Verilog-AMS trials and acceptance.
//!
//! The circuit owns one linked HDL process/driver/event state. Each mixed
//! model retains its analog equations, electrical A/D and D/A adapters, and a
//! local view of resolved digital signals. A numerical trial samples every
//! analog probe before due HDL execution, then publishes all A/D decisions
//! before dependent processes run. Only after shared settlement do analog
//! equations consume the digital values and stamp their contributions.
//!
//! The four entry points below — a Newton probe, an operating point, the
//! inspection of a converged candidate and an accepted timepoint — are four
//! callers of one protocol. [`crate::circuit::scheduler::TrialKind`] is the
//! whole of what separates them; the trial itself owns the rollback, so a
//! speculative exit restores every model view, analog candidate, process
//! resumption, driver and queue together. Acceptance validates every model
//! before promoting any participant. A CircuitData clone contains both the
//! accepted shared runtime and its views; an individual model checkpoint cannot
//! represent the shared process state.
//!
//! The shared runtime's next event joins the transient breakpoint list. Analog
//! electrical boundaries remain physical loads. Enrolled XSPICE event drivers
//! participate in the same numerical probe and acceptance barrier. Static
//! history observes the settled candidate without replaying digital events.

#[cfg(test)]
#[path = "mixed_signal/coupled_tests.rs"]
mod coupled_tests;

use super::external_models::{XspiceCompanionPolicy, XspiceDigitalBindings};
use super::scheduler::{Candidate, MAX_BOUNDARY_SETTLE_PASSES, TrialKind, named, shared_error};
use crate::circuit::CircuitData;
use crate::{SimulationError, Value};

use crate::xspice::verilog::{BoundaryBitSource, BoundaryBus, MixedSignalHost};

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
        // Read from the parked ledgers rather than from the instances: the
        // trial that recorded these samples handed them back to the scheduler
        // when it closed, which is what keeps a rolled-back probe from erasing
        // them. An empty vector is a circuit no trial has yet opened.
        let mut context = self
            .mixed_signal_hosts
            .iter()
            .zip(&self.scheduler.ledgers)
            .filter_map(|(host, ledger)| host.rejected_probe_activity(ledger, time));
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
    pub(crate) fn add_mixed_signal_host(
        &mut self,
        mut host: MixedSignalHost,
    ) -> Result<(), SimulationError> {
        if self.scheduler.mixed_digital_coordinator.is_some() {
            return Err(crate::ElaborationError::new(
                crate::ElaborationErrorKind::Internal,
                "mixed instances cannot be added after circuit digital elaboration",
            )
            .instance(host.instance_name())
            .into());
        }
        host.set_simulation_parameters(self.generated_simulation_parameters);
        self.mixed_signal_hosts.push(host);
        Ok(())
    }

    pub(crate) fn finalize_mixed_digital(
        &mut self,
        event_nodes: &std::collections::BTreeSet<usize>,
        control: &dyn rspice_veriloga::PipelineControl,
    ) -> Result<(), SimulationError> {
        if !self.mixed_signal_hosts.is_empty() && self.scheduler.mixed_digital_coordinator.is_none()
        {
            // Elaboration can refuse a connection after creating the linked
            // runtime. Preserve the previous owners until every attachment has
            // validated; a partial link must not leave standalone models as views.
            let rollback = self.mixed_signal_hosts.clone();
            let linked = (|| {
                let mut coordinator = crate::xspice::verilog::MixedDigitalCoordinator::enroll(
                    &mut self.mixed_signal_hosts,
                    event_nodes,
                    control,
                )
                .map_err(shared_error)?;
                let bindings = XspiceDigitalBindings::enroll_circuit(self, &mut coordinator)
                    .map_err(|error| shared_error(error.into()))?;
                Ok::<_, SimulationError>((coordinator, bindings))
            })();
            let (coordinator, bindings) = match linked {
                Ok(linked) => linked,
                Err(error) => {
                    self.mixed_signal_hosts = rollback;
                    return Err(error);
                }
            };
            self.scheduler.mixed_digital_coordinator = Some(coordinator);
            self.scheduler.mixed_xspice_bindings = bindings.map(std::sync::Arc::new);
            for &node in event_nodes {
                self.net_kinds.register(node, super::NetKind::Digital);
            }
        }
        Ok(())
    }

    /// Check the existing electrical-boundary adapter against the complete
    /// event topology, including instances created after a mixed module.
    /// Direct event bindings require the circuit scheduling coordinator; the
    /// current host must not accidentally reach them through an analog bridge.
    pub(crate) fn validate_mixed_event_connections(
        &self,
        event_nodes: &std::collections::BTreeSet<usize>,
    ) -> Result<(), SimulationError> {
        for host in &self.mixed_signal_hosts {
            for (signal, node) in host.boundary_connections() {
                let kind = self.net_kinds.kind(node);
                if !kind.is_discrete()
                    || (kind == super::NetKind::Digital && event_nodes.contains(&node))
                {
                    continue;
                }
                let node_names = self.node_names_sorted();
                let node_name = node_names
                    .get(node - 1)
                    .map(String::as_str)
                    .unwrap_or("<unnamed>");
                return Err(crate::ElaborationError::new(
                    crate::ElaborationErrorKind::PortDiscipline,
                    format!(
                        "connects its discrete port '{signal}' to node '{node_name}', \
                         which carries {} event-driven XSPICE values. This connection requires \
                         an explicit shared conversion contract for continuous loading or unlike \
                         event domains; direct shared resolution currently requires a digital-only net",
                        kind.description(),
                    ),
                )
                .instance(host.instance_name())
                .into());
            }
        }
        Ok(())
    }

    /// Begin digital execution only after the engine has delivered every
    /// model's analog initialization effects and ruled out a requested exit.
    pub(crate) fn start_mixed_digital_execution(&mut self) -> Result<(), SimulationError> {
        self.finalize_mixed_digital(
            &std::collections::BTreeSet::new(),
            &rspice_veriloga::NoPipelineControl,
        )?;
        for host in &mut self.mixed_signal_hosts {
            let started = host.start_digital_execution();
            named(host, started)?;
        }
        if let Some(digital) = &mut self.scheduler.mixed_digital_coordinator {
            digital.start().map_err(shared_error)?;
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
    ///
    /// # What is missing, and why `.op` is not an exemption
    ///
    /// What every caller here wants is narrower than "a mixed module", so the
    /// refusal names it: each of them sweeps, linearizes or shoots around a
    /// discrete state that has *settled*, and settling one is a capability this
    /// build does not have. `.op` does not need it — it answers at the state a
    /// module's initial blocks left, which exists before anything has settled
    /// and before any time has passed — so `.op` does not call this at all.
    /// That is why the answer is a capability refusal rather than a circuit
    /// error: the deck is understood and the analysis is the gap.
    pub(crate) fn ensure_no_mixed_signal_hosts(
        &self,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        let Some(host) = self.mixed_signal_hosts.first() else {
            return Ok(());
        };
        Err(SimulationError::unsupported_capability(
            "analysis.mixed_signal.discrete_state",
            format!(
                "mixed Verilog-AMS instance '{}' cannot take part in {analysis}: the module's \
                 discrete half is executed by a transient event interleave, which has no \
                 small-signal or steady-state form. `.tran` runs a mixed module and `.op` \
                 solves one at the state its initial blocks left; an analysis around a \
                 settled discrete state is not available yet",
                host.instance_name()
            ),
        ))
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
        companion: XspiceCompanionPolicy<'_>,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), SimulationError> {
        self.stamp_mixed_trial(
            matrix,
            rhs,
            Candidate {
                time,
                dt,
                analysis_step: Some((initial_step, final_step)),
            },
            TrialKind::Probe,
            voltages,
            companion,
        )
    }

    fn stamp_mixed_trial(
        &mut self,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        candidate: Candidate,
        kind: TrialKind,
        voltages: &[Value],
        companion: XspiceCompanionPolicy<'_>,
    ) -> Result<(), SimulationError> {
        if self.mixed_signal_hosts.is_empty() {
            return Ok(());
        }
        // Only HDL equations and their physical D/A bridges receive this
        // weight. Enrolled XSPICE stamps apply their own policy.
        let weight = if companion.xyce_one_step_order2 {
            0.5
        } else {
            1.0
        };
        let mut trial = self.open_trial(candidate, kind, companion, None)?;
        let stamped = (|| {
            trial.settle(voltages)?;
            if trial.has_code_models() {
                // The same settled code-model candidate supplies its stamps.
                trial
                    .circuit_mut()
                    .stamp_xspice(matrix, rhs)
                    .map_err(SimulationError::from)?;
            }
            trial.stamp(matrix, rhs, voltages, weight)
        })();
        trial.finish(stamped)
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
    /// Stamp every mixed module into an operating-point system at `time`.
    ///
    /// The one assembly two callers share: the transient's t = 0 startup, and
    /// `.op` on a mixed deck, which is that startup with no advance after it.
    /// Both reach it through `Engine::try_stamp_operating_point_devices`.
    ///
    /// The trial it opens is a probe with a zero interval, so nothing here
    /// commits: the shared queue is drained only to this time's tick, and an
    /// activation scheduled past it stays pending on the queue. At t = 0 that
    /// is the whole of "no time advance" — a `#delay` or an `@(posedge)` a
    /// module's initial block scheduled for a later tick is still pending when
    /// the operating point is published, and is discarded with the circuit.
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
            Candidate {
                time,
                dt: 0.0,
                analysis_step: None,
            },
            TrialKind::OperatingPoint,
            solution,
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
        )
    }

    /// Settle and validate every mixed host before native acceptance work.
    /// The caller owns the XSPICE/resource and projected-solution rollback.
    /// Its finish callback must complete all remaining fallible preparation
    /// before promotion; after it succeeds, these HDL commits are infallible.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn accept_mixed_transient_with<T>(
        &mut self,
        time: Value,
        dt: Value,
        solution: &mut [Value],
        companion: XspiceCompanionPolicy<'_>,
        initial_step: bool,
        final_step: bool,
        resources: Option<&crate::xspice::ResourceTransaction>,
        capture_static_history: bool,
        projected: &mut Vec<(usize, Value)>,
        finish: impl FnOnce(
            &mut CircuitData,
            &mut [Value],
            &[(usize, Value)],
            Option<&[Value]>,
        ) -> Result<T, SimulationError>,
    ) -> Result<(bool, T), SimulationError> {
        if self.scheduler.mixed_xspice_bindings.is_some() && resources.is_none() {
            return Err(SimulationError::Circuit(
                "shared acceptance requires its resource transaction".into(),
            ));
        }
        let mut trial = self.open_trial(
            Candidate {
                time,
                dt,
                analysis_step: Some((initial_step, final_step)),
            },
            TrialKind::Acceptance,
            companion,
            resources,
        )?;
        if trial.has_code_models() {
            let mut projected_quiet = false;
            let mut wave = None;
            let mut moved: Vec<usize> = Vec::new();
            for pass in 0..MAX_BOUNDARY_SETTLE_PASSES {
                wave = trial.settle_projection_pass(solution, pass, &moved, wave.take())?;
                let num_nodes = trial.circuit_mut().num_nodes();
                let (updates, refusal) = trial.circuit_mut().project_xspice_voltage_outputs(
                    solution,
                    num_nodes,
                    Some(time),
                );
                let updates = match refusal {
                    Ok(()) => updates,
                    Err(error) => {
                        projected.extend(updates);
                        return Err(SimulationError::from(error));
                    }
                };
                if updates.is_empty() {
                    projected_quiet = true;
                    break;
                }
                moved.clear();
                moved.extend(updates.iter().map(|(index, _)| index + 1));
                projected.extend(updates);
            }
            if !projected_quiet {
                return Err(SimulationError::Circuit(format!(
                    "mixed candidate voltage projection did not settle at t={time:.16e}s"
                )));
            }
        } else {
            trial.settle(solution)?;
        }
        let discontinuity = trial.evaluate(solution)?;
        // Reserve every HDL candidate before native state can be promoted.
        // Dropping these reservations on a callback error unwinds all hosts.
        let static_history = capture_static_history
            .then(|| trial.static_residual(solution))
            .transpose()?;
        let mut prepared = trial.prepare()?;
        let result = finish(
            prepared.circuit_mut(),
            solution,
            projected,
            static_history.as_deref(),
        )?;
        prepared.commit();
        Ok((discontinuity, result))
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
        self.validate_nonmixed_model_acceptance()
            .map_err(SimulationError::Circuit)?;
        let mut discontinuity = self.veriloga_discontinuity_rising();
        if !self.mixed_signal_hosts.is_empty() {
            let mut trial = self.open_trial(
                Candidate {
                    time,
                    dt,
                    analysis_step: Some((initial_step, final_step)),
                },
                TrialKind::Acceptance,
                XspiceCompanionPolicy {
                    coefficients,
                    xyce_one_step_order2: false,
                },
                None,
            )?;
            trial.settle(voltages)?;
            discontinuity |= trial.evaluate(voltages)?;
            trial.prepare()?.commit();
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
        companion: XspiceCompanionPolicy<'_>,
        minimum_timestep: Value,
        initial_step: bool,
        final_step: bool,
        kind: rspice_veriloga_runtime::AnalogTaskKind,
        validate_acceptance: bool,
        consume: &mut dyn FnMut(rspice_veriloga_runtime::AnalogTaskEvent<'_>),
    ) -> Result<(Option<Value>, bool), SimulationError> {
        if self.mixed_signal_hosts.is_empty() {
            return Ok((None, false));
        }
        let mut trial = self.open_trial(
            Candidate {
                time,
                dt,
                analysis_step: Some((initial_step, final_step)),
            },
            TrialKind::Inspection,
            companion,
            None,
        )?;
        let inspected = (|| {
            let mut discontinuity = false;
            trial.settle(voltages)?;
            // An A/D crossing this settled trial placed strictly inside its
            // own interval is a root the solver is asked to land on.
            let mut refinement = trial.interior_root(minimum_timestep)?;
            if refinement.is_some() {
                return Ok((refinement, false));
            }
            for host in trial.hosts_mut() {
                let inspected = (|| {
                    host.stamp(voltages, |_, _, _| {}, |_, _| {})?;
                    discontinuity |= host.candidate_discontinuity();
                    let target = host
                        .analog_device()
                        .try_transient_event_refinement_time()
                        .map_err(|error| crate::xspice::verilog::MixedSignalError::Analog {
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
                        .map_err(|error| crate::xspice::verilog::MixedSignalError::Analog {
                            detail: error.to_string(),
                        })?
                    {
                        consume(event);
                    }
                    Ok(())
                })();
                named(host, inspected)?;
            }
            Ok((refinement, discontinuity))
        })();
        trial.finish(inspected)
    }

    /// Append every mixed module's committed boundary values to a digital
    /// snapshot, one value per deck node.
    ///
    /// Written into the same vector `fill_xspice_digital_snapshot` fills, and
    /// sorted with it, so `TransientResult::record_digital_snapshot` stays the
    /// single writer of the digital trace channel. One node carries one bit,
    /// because a bridge carries one bit: a vector boundary port is bridged as
    /// one net per conductor, so the deck node a bit landed on is what the
    /// snapshot records, and the declaration that says which of them were one
    /// word rides beside the traces rather than inside them.
    ///
    /// # Which bit, when a node carries more than one
    ///
    /// A deck node joining one module's discrete output to another's discrete
    /// input has two bridges on it — x1's D/A and x2's A/D — and each publishes
    /// its own bit. They are not the same claim. The D/A bit is what was put on
    /// the net; the A/D bit is what one reader made of the voltage that
    /// resulted, and it lags by however long the node takes to cross that
    /// reader's threshold, which an analog load can stretch over many accepted
    /// timepoints. Publishing both put two opposite values on one node at one
    /// instant and the trace recorded a zero-width glitch for every accepted
    /// point in the lag.
    ///
    /// So the driver wins: a net's value is what its driver drove. A reader's
    /// sample is that instance's own input and has no channel of its own here —
    /// [`crate::analysis::transient::DigitalTrace`] is keyed by deck node, not
    /// by instance port — so it is not published rather than published as the
    /// net. A node with no driver keeps its reader's bit, which is the only
    /// claim anyone has made about it and the value an A/D-only boundary has
    /// always traced.
    ///
    /// Ties inside one rank — two drivers on a net, or two readers of an analog
    /// node with different thresholds — resolve to the first in enumeration
    /// order, which is host registration order and then bridge declaration
    /// order. Deterministic rather than arbitrary; a net with two disagreeing
    /// drivers has no digital value to report and its analog voltage is the
    /// answer.
    pub(crate) fn append_mixed_digital_snapshot(
        &self,
        snapshot: &mut Vec<(crate::circuit::NodeId, crate::xspice::DigitalValue)>,
    ) {
        use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};
        use rspice_veriloga::four_state::FourStateBit;

        /// Precedence of a claim about a node, lowest first.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        enum Claim {
            /// The shared runtime already resolved every contribution to this
            /// event net, so there is nothing left to choose between.
            Resolved,
            /// A D/A bridge drives the node.
            Driven,
            /// An A/D bridge reads it.
            Sampled,
        }

        let mut claims: Vec<(crate::circuit::NodeId, Claim, DigitalValue)> = Vec::new();
        if let Some(digital) = &self.scheduler.mixed_digital_coordinator {
            claims.extend(
                digital
                    .event_values()
                    .map(|(node, value)| (node, Claim::Resolved, value)),
            );
        }
        for host in &self.mixed_signal_hosts {
            host.boundary_digital_values(|node, bit, source| {
                if node == 0 {
                    return;
                }
                let state = match bit {
                    FourStateBit::Zero => DigitalState::Zero,
                    FourStateBit::One => DigitalState::One,
                    FourStateBit::Unknown => DigitalState::Unknown,
                    FourStateBit::HighImpedance => DigitalState::HighZ,
                };
                let claim = match source {
                    BoundaryBitSource::Driven => Claim::Driven,
                    BoundaryBitSource::Sampled => Claim::Sampled,
                };
                claims.push((
                    node,
                    claim,
                    DigitalValue {
                        state,
                        strength: DigitalStrength::Strong,
                    },
                ));
            });
        }
        // Stable, so equal-rank claims keep enumeration order and the winner is
        // a function of the circuit rather than of the sort's pivot choices.
        claims.sort_by_key(|&(node, claim, _)| (node, claim));
        claims.dedup_by_key(|&mut (node, ..)| node);
        snapshot.extend(claims.into_iter().map(|(node, _, value)| (node, value)));
    }
}
