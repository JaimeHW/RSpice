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

/// A converter input found at a decision boundary of a held equilibrium.
///
/// The held discrete state is frozen for every analysis that linearizes
/// around it, so a decision that a perturbation the size of the solver's own
/// error could have flipped is worth naming: the small-signal answer is the
/// answer for *this* decision, and the other one was equally consistent.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ThresholdFinding {
    /// The converter instance whose decision is in question.
    pub(crate) converter: String,
    /// The analog net its input senses.
    pub(crate) net: String,
    /// The mixed module port the decision feeds, as `instance.port`, when a
    /// module reads it.
    pub(crate) host_port: Option<String>,
    pub(crate) kind: ThresholdFindingKind,
    /// The input the converter decided on.
    pub(crate) voltage: Value,
}

/// What inspecting a converged candidate found in the mixed modules.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct MixedCandidateInspection {
    /// The earliest root strictly inside the candidate's interval that the
    /// solver is asked to land on.
    pub(crate) refinement: Option<Value>,
    /// Whether that root is an analog block's own `cross`/`above` event, whose
    /// body runs in the solve that lands on it, rather than an A/D converter's
    /// threshold crossing, which only the discrete half reads.
    pub(crate) analog_event_root: bool,
    /// Some host's electrical change at the candidate cannot be compared with
    /// smooth history (its `candidate_discontinuity`).
    pub(crate) discontinuity: bool,
}

/// Which boundary a [`ThresholdFinding`] sits on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ThresholdFindingKind {
    /// Within Newton's own tolerance of a switching threshold.
    OnThreshold { threshold: Value },
    /// Inside an unknown or hysteresis band with no earlier decision, so the
    /// converter published X.
    UndefinedBand { low: Value, high: Value },
}

impl std::fmt::Display for ThresholdFinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "converter '{}' input on net '{}'",
            self.converter, self.net
        )?;
        if let Some(port) = &self.host_port {
            write!(f, " (feeding '{port}')")?;
        }
        match self.kind {
            ThresholdFindingKind::OnThreshold { threshold } => write!(
                f,
                " sits on its threshold {threshold:e} V at the held equilibrium \
                 (v = {:e} V); the frozen decision is one of two consistent ones",
                self.voltage
            ),
            ThresholdFindingKind::UndefinedBand { low, high } => write!(
                f,
                " lies inside its undefined band ({low:e} V, {high:e} V) with no earlier \
                 decision (v = {:e} V); the held equilibrium carries X",
                self.voltage
            ),
        }
    }
}

/// The held equilibrium of one DC sweep point, carried into the circuit the
/// next point rebuilds (design §4.3).
///
/// A sibling of the transient checkpoint's mixed image rather than one, for
/// the reasons that image cannot serve: it never leaves the analysis, so it is
/// shared copy-on-write instead of serialized; and it is a time-zero
/// equilibrium no time was ever accepted at, where the transient image is an
/// accepted timepoint. Only the discrete state travels — the wheel's store
/// and each instance's accepted decision bank. Every analog half is rebuilt
/// with the circuit at the new point's parameter values.
#[derive(Clone, Debug)]
pub(crate) struct MixedEquilibriumImage {
    coordinator: crate::xspice::verilog::CoordinatorEquilibriumImage,
    hosts: Vec<crate::xspice::verilog::HostEquilibriumImage>,
}

/// The potential of one circuit node in a solution, ground at zero.
fn node_voltage(solution: &[Value], node: usize) -> Value {
    if node == 0 {
        0.0
    } else {
        solution.get(node - 1).copied().unwrap_or(0.0)
    }
}

/// How a finding names the analog net one converter input element senses.
fn analog_input_net(
    connection: &crate::xspice::PortConnection,
    element: usize,
    node_name: &dyn Fn(usize) -> String,
) -> String {
    use crate::xspice::{AnalogInputConnection, PortConnection};
    let pair = |pos: usize, neg: usize| {
        if neg == 0 {
            node_name(pos)
        } else {
            format!("{},{}", node_name(pos), node_name(neg))
        }
    };
    match connection {
        PortConnection::Analog(node) => node_name(*node),
        PortConnection::Differential(pos, neg) => pair(*pos, *neg),
        PortConnection::AnalogVector(nodes) => nodes
            .get(element)
            .map_or_else(|| format!("element {element}"), |node| node_name(*node)),
        PortConnection::TypedAnalogVector(elements) => match elements.get(element) {
            Some(AnalogInputConnection::Node(node)) => node_name(*node),
            Some(
                AnalogInputConnection::Differential(pos, neg)
                | AnalogInputConnection::VoltageOutput { pos, neg }
                | AnalogInputConnection::CurrentOutput { pos, neg }
                | AnalogInputConnection::Hybrid { pos, neg, .. },
            ) => pair(*pos, *neg),
            _ => format!("element {element}"),
        },
        _ => format!("element {element}"),
    }
}

impl CircuitData {
    /// Enrich a terminal solver failure without changing convergence recovery
    /// or accepting any speculative boundary state.
    pub(crate) fn annotate_mixed_convergence_failure(
        &self,
        error: SimulationError,
        time: Value,
    ) -> SimulationError {
        if !matches!(
            error,
            SimulationError::ConvergenceFailed(_) | SimulationError::ContinuationFailed(_)
        ) {
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

    /// The cross-domain loop the instances' accepted boundary histories show,
    /// named by instance, when a boundary net has moved at `minimum_run`
    /// consecutive accepted points with nothing scheduled to explain any of
    /// them. `None` when no instance's history shows one.
    ///
    /// For a guard that is refusing the run for a reason of its own — the
    /// stepper pinned at its floor — and should say why when the reason is a
    /// zero-delay loop: see `MixedSignalHost::accepted_boundary_loop`.
    pub(crate) fn mixed_boundary_loop(&self, minimum_run: u32) -> Option<SimulationError> {
        self.mixed_signal_hosts.iter().find_map(|host| {
            host.accepted_boundary_loop(minimum_run)
                .and_then(|error| named::<()>(host, Err(error)).err())
        })
    }

    /// Whether any mixed Verilog-AMS module is instantiated.
    #[inline]
    pub(crate) fn has_mixed_signal_hosts(&self) -> bool {
        !self.mixed_signal_hosts.is_empty()
    }

    /// Whether the accepted point stepped a digital-to-analog drive of any
    /// instance: the solution accepted there is the drive's left limit, and
    /// the interval leaving it opens on a right limit no accepted sample
    /// holds. See `MixedSignalHost::accepted_point_stepped_a_drive`.
    pub(crate) fn accepted_point_stepped_a_mixed_drive(&self) -> bool {
        self.mixed_signal_hosts
            .iter()
            .any(MixedSignalHost::accepted_point_stepped_a_drive)
    }

    /// The edge completion each instance asks for: the end of a step a
    /// discrete value its analog block reads took at the accepted point. See
    /// `MixedSignalHost::read_step_completion`.
    pub(crate) fn mixed_read_step_completions(&self) -> impl Iterator<Item = Value> + '_ {
        self.mixed_signal_hosts
            .iter()
            .filter_map(MixedSignalHost::read_step_completion)
    }

    /// Whether some instance's analog block reads a discrete value as it
    /// stands after a timepoint's digital activity. See
    /// `MixedSignalHost::analog_reads_discrete_after_activity`.
    pub(crate) fn mixed_analog_reads_discrete_after_activity(&self) -> bool {
        self.mixed_signal_hosts
            .iter()
            .any(MixedSignalHost::analog_reads_discrete_after_activity)
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
            let sampled = self.analog_sampled_real_nodes();
            let solution_samples = self.solution_sampled_real_nodes();
            for host in &mut self.mixed_signal_hosts {
                host.declare_analog_sampled_reals(&sampled, &solution_samples);
            }
            for (node, real) in coordinator.event_domains() {
                self.net_kinds.register(
                    node,
                    if real {
                        super::NetKind::Real
                    } else {
                        super::NetKind::Digital
                    },
                );
            }
            self.scheduler.mixed_digital_coordinator = Some(coordinator);
            self.scheduler.mixed_xspice_bindings = bindings.map(std::sync::Arc::new);
        }
        Ok(())
    }

    /// Every real event net a code model drives from the analog solution: an
    /// instance that reads a node voltage and publishes a real, which is what a
    /// V-to-real converter is whether elaboration inserted it or the deck
    /// authored it.
    fn analog_sampled_real_nodes(&self) -> std::collections::BTreeSet<usize> {
        let mut nodes = std::collections::BTreeSet::new();
        for instance in &self.xspice_instances {
            let mut reads_analog = false;
            instance.for_each_analog_input_net(|_| reads_analog = true);
            if reads_analog {
                instance.for_each_real_output_driver(|target| {
                    nodes.insert(target.node_id);
                });
            }
        }
        nodes
    }

    /// The real event nets only a sample of the analog solution moves: every
    /// code model driving one reads a node voltage and no event input, so it
    /// publishes only when it is evaluated at an accepted point, and no mixed
    /// module drives it. A sampler with a clock or an enable input is left
    /// out, because what moves its output is then that input's event.
    fn solution_sampled_real_nodes(&self) -> std::collections::BTreeSet<usize> {
        let mut sampled = std::collections::BTreeSet::new();
        let mut otherwise = std::collections::BTreeSet::new();
        for instance in &self.xspice_instances {
            let mut reads_analog = false;
            instance.for_each_analog_input_net(|_| reads_analog = true);
            let mut reads_events = false;
            instance.for_each_event_input_net(|_, _| reads_events = true);
            let drivers = if reads_analog && !reads_events {
                &mut sampled
            } else {
                &mut otherwise
            };
            instance.for_each_real_output_driver(|target| {
                drivers.insert(target.node_id);
            });
        }
        for host in &self.mixed_signal_hosts {
            for (port, _) in host.direct_event_ports() {
                if port.direction
                    != rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection::Input
                {
                    otherwise.insert(port.node);
                }
            }
        }
        sampled.retain(|node| !otherwise.contains(node));
        sampled
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
            host.validate_direct_event_ports(event_nodes)
                .map_err(|error| {
                    crate::ElaborationError::new(
                        crate::ElaborationErrorKind::PortDiscipline,
                        error.to_string(),
                    )
                    .instance(host.instance_name())
                })?;
            for node in host.direct_real_nodes() {
                if matches!(
                    self.net_kinds.kind(node),
                    super::NetKind::Digital | super::NetKind::DigitalAndReal
                ) {
                    return Err(crate::ElaborationError::new(
                        crate::ElaborationErrorKind::PortDiscipline,
                        "a real-valued HDL port and a four-state XSPICE event port require explicit domain conversion",
                    ).instance(host.instance_name()).into());
                }
            }
            for (signal, node) in host.boundary_connections() {
                let kind = self.net_kinds.kind(node);
                if !kind.is_discrete()
                    || (event_nodes.contains(&node)
                        && (kind == super::NetKind::Digital
                            || (kind == super::NetKind::Real
                                && host.direct_real_nodes().any(|real| real == node))))
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
        if self.scheduler.has_equilibrium_commit() {
            return Err(crate::ElaborationError::new(
                crate::ElaborationErrorKind::Internal,
                "digital execution cannot start on a circuit that already holds a committed \
                 equilibrium; that circuit belongs to an operating point or a small-signal bias",
            )
            .into());
        }
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

    /// Stamp every mixed module for one Newton evaluation, committing nothing.
    ///
    /// The trial is a probe from end to end: it is opened, settled, stamped and
    /// rolled back inside this call, so the module's accepted state on return
    /// is exactly its accepted state on entry. See this module's documentation
    /// for why that is the whole of the rollback contract at deck level.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stamp_mixed_transient_trial(
        &mut self,
        abort: &dyn crate::abort_signal::AbortSignal,
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
            abort,
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

    #[allow(clippy::too_many_arguments)]
    fn stamp_mixed_trial(
        &mut self,
        abort: &dyn crate::abort_signal::AbortSignal,
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
            trial.settle(abort, voltages)?;
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
            &crate::abort_signal::NoAbort,
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

    /// Commit the converged operating point as the held equilibrium.
    ///
    /// The coupled time-zero fixed point the operating-point Newton solve
    /// converged to is settled once more at `solution` and kept, rather than
    /// rolled back the way every operating-point probe is: the wheel keeps its
    /// settled store, every host its settled view, A/D decisions and probe
    /// samples, every enrolled code model its settled context, and each
    /// host's analog half takes its validated acceptance. Linearization reads
    /// only this committed state, and a DC sweep's next point starts from it.
    ///
    /// A boundary that will not quiet is refused with the settle's own
    /// oscillation error. A circuit with no mixed module has nothing to
    /// commit: XSPICE code models already keep their operating-point state.
    pub(crate) fn accept_mixed_equilibrium(
        &mut self,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        if self.mixed_signal_hosts.is_empty() {
            return Ok(());
        }
        let mut trial = self.open_trial(
            Candidate {
                time: 0.0,
                dt: 0.0,
                analysis_step: None,
            },
            TrialKind::Equilibrium,
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
            None,
        )?;
        trial.settle(&crate::abort_signal::NoAbort, solution)?;
        trial.evaluate(solution)?;
        trial.commit_equilibrium()
    }

    /// The held equilibrium this circuit carries, when it carries one: a
    /// circuit with mixed modules whose time-zero settle was committed.
    pub(crate) fn mixed_equilibrium_image(&self) -> Result<Option<MixedEquilibriumImage>, String> {
        let Some(coordinator) = &self.scheduler.mixed_digital_coordinator else {
            return Ok(None);
        };
        if self.mixed_signal_hosts.is_empty() || !self.scheduler.has_equilibrium_commit() {
            return Ok(None);
        }
        Ok(Some(MixedEquilibriumImage {
            coordinator: coordinator.equilibrium_image()?,
            hosts: self
                .mixed_signal_hosts
                .iter()
                .map(MixedSignalHost::equilibrium_image)
                .collect::<Result<_, _>>()?,
        }))
    }

    /// Adopt the held equilibrium a previous DC sweep point's circuit kept,
    /// after this circuit has started digital execution.
    ///
    /// Installed whole or not at all: iff the wheel's digital structure and
    /// every instance's boundary structure are the image's. Returns the first
    /// instance whose structure differs, with this circuit left at its fresh
    /// initialization, so the caller can say which elaboration changed.
    pub(crate) fn adopt_mixed_equilibrium_image(
        &mut self,
        image: &MixedEquilibriumImage,
    ) -> Result<Option<String>, String> {
        let first_host = || {
            self.mixed_signal_hosts
                .first()
                .map_or_else(String::new, |host| host.instance_name().to_owned())
        };
        let Some(coordinator) = &self.scheduler.mixed_digital_coordinator else {
            return Ok(Some(first_host()));
        };
        if self.mixed_signal_hosts.len() != image.hosts.len() {
            return Ok(Some(first_host()));
        }
        // Validate every participant on copies before the first install.
        let mut staged_coordinator = coordinator.clone();
        if !staged_coordinator.adopt_equilibrium_image(&image.coordinator)? {
            return Ok(Some(first_host()));
        }
        let mut staged_hosts = self.mixed_signal_hosts.clone();
        for (host, host_image) in staged_hosts.iter_mut().zip(&image.hosts) {
            if !host.adopt_equilibrium_image(host_image)? {
                return Ok(Some(host.instance_name().to_owned()));
            }
        }
        self.scheduler.mixed_digital_coordinator = Some(staged_coordinator);
        self.mixed_signal_hosts = staged_hosts;
        self.scheduler.ledgers.clear();
        self.scheduler.note_equilibrium_commit();
        Ok(None)
    }

    /// Discard the analog initialization calls a freshly built module queued
    /// when a DC sweep continues on a rebuilt circuit: they were delivered at
    /// the sweep's first point, and the continuation does not run them again.
    pub(crate) fn discard_mixed_initial_analog_tasks(&mut self) -> Result<(), String> {
        for host in &mut self.mixed_signal_hosts {
            host.visit_accepted_analog_tasks(&mut |_| {})
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    /// Converter inputs that sit at a decision boundary of the held
    /// equilibrium at `solution` (S5(c)).
    ///
    /// Two findings are reported, each naming the converter, its input net and
    /// the mixed module port it feeds:
    ///
    /// * an input within Newton's own "same value" tolerance of a threshold,
    ///   `|v − θ| ≤ reltol·max(|v|, |θ|) + vntol`, where a perturbation the
    ///   size of the solver's own error could have flipped the decision;
    /// * an input inside an unknown or hysteresis band whose converter
    ///   published X because no earlier decision exists.
    ///
    /// The code models are read through their committed input samples; a
    /// module's own A/D bridges through `solution`.
    pub(crate) fn mixed_threshold_findings(
        &self,
        solution: &[Value],
        reltol: Value,
        vntol: Value,
    ) -> Vec<ThresholdFinding> {
        let node_names = self.node_names_sorted();
        let node_name = |node: usize| -> String {
            if node == 0 {
                "0".to_owned()
            } else {
                node_names
                    .get(node - 1)
                    .cloned()
                    .unwrap_or_else(|| format!("node {node}"))
            }
        };
        let on_threshold = |voltage: Value, threshold: Value| {
            (voltage - threshold).abs() <= reltol * voltage.abs().max(threshold.abs()) + vntol
        };
        let mut findings = Vec::new();
        for instance in &self.xspice_instances {
            let Ok(thresholds) = instance.analog_input_thresholds() else {
                continue;
            };
            for threshold in thresholds {
                let Some(voltage) =
                    instance.analog_threshold_sample(&threshold.port, threshold.element)
                else {
                    continue;
                };
                let net = instance
                    .connection(&threshold.port)
                    .map(|connection| analog_input_net(connection, threshold.element, &node_name))
                    .unwrap_or_else(|| threshold.port.clone());
                let output = self.converter_output_node(instance, threshold.element);
                let host_port = output.and_then(|node| self.mixed_host_port_on(node));
                let kind = if let Some(theta) = [threshold.low, threshold.high]
                    .into_iter()
                    .find(|theta| on_threshold(voltage, *theta))
                {
                    Some(ThresholdFindingKind::OnThreshold { threshold: theta })
                } else if threshold.low < voltage
                    && voltage < threshold.high
                    && output.is_some_and(|node| {
                        self.scheduler
                            .xspice_event_values
                            .digital_values
                            .get(&node)
                            .is_some_and(|value| {
                                value.state == crate::xspice::DigitalState::Unknown
                            })
                    })
                {
                    Some(ThresholdFindingKind::UndefinedBand {
                        low: threshold.low,
                        high: threshold.high,
                    })
                } else {
                    None
                };
                if let Some(kind) = kind {
                    findings.push(ThresholdFinding {
                        converter: instance.name.clone(),
                        net,
                        host_port,
                        kind,
                        voltage,
                    });
                }
            }
        }
        for host in &self.mixed_signal_hosts {
            for bridge in host.boundary_thresholds() {
                let voltage = node_voltage(solution, bridge.positive)
                    - node_voltage(solution, bridge.negative);
                let kind = if let Some(theta) = [bridge.low, bridge.high]
                    .into_iter()
                    .find(|theta| on_threshold(voltage, *theta))
                {
                    Some(ThresholdFindingKind::OnThreshold { threshold: theta })
                } else if bridge.low < voltage && voltage < bridge.high && bridge.undefined {
                    Some(ThresholdFindingKind::UndefinedBand {
                        low: bridge.low,
                        high: bridge.high,
                    })
                } else {
                    None
                };
                if let Some(kind) = kind {
                    findings.push(ThresholdFinding {
                        converter: host.instance_name().to_owned(),
                        net: if bridge.negative == 0 {
                            node_name(bridge.positive)
                        } else {
                            format!(
                                "{},{}",
                                node_name(bridge.positive),
                                node_name(bridge.negative)
                            )
                        },
                        host_port: Some(format!("{}.{}", host.instance_name(), bridge.signal)),
                        kind,
                        voltage,
                    });
                }
            }
        }
        findings
    }

    /// The digital net a converter publishes the decision of one input
    /// element on.
    fn converter_output_node(
        &self,
        instance: &crate::xspice::XspiceInstance,
        element: usize,
    ) -> Option<usize> {
        use crate::xspice::{PortConnection, PortDirection};
        let index = instance
            .ports()
            .iter()
            .position(|port| port.direction == PortDirection::Out)?;
        match instance.connection_at(index)? {
            PortConnection::Digital(node) | PortConnection::DigitalInverted(node) => {
                (element == 0).then_some(*node)
            }
            PortConnection::DigitalVector(nodes) => nodes.get(element).copied(),
            PortConnection::DigitalVectorMapped(nodes) => {
                nodes.get(element).map(|connection| connection.node)
            }
            _ => None,
        }
    }

    /// The mixed module port bound to one event net, as `instance.port`.
    fn mixed_host_port_on(&self, node: usize) -> Option<String> {
        self.mixed_signal_hosts.iter().find_map(|host| {
            host.direct_event_ports()
                .find(|(port, _)| port.node == node)
                .map(|(_, name)| format!("{}.{name}", host.instance_name()))
        })
    }

    /// Settle and validate every mixed host before native acceptance work.
    /// The caller owns the XSPICE/resource and projected-solution rollback.
    /// Its finish callback must complete all remaining fallible preparation
    /// before promotion; after it succeeds, these HDL commits are infallible.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn accept_mixed_transient_with<T>(
        &mut self,
        abort: &dyn crate::abort_signal::AbortSignal,
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
    ) -> Result<(super::ModelDiscontinuity, T), SimulationError> {
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
                wave = trial.settle_projection_pass(abort, solution, pass, &moved, wave.take())?;
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
            trial.settle(abort, solution)?;
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
            trial.settle(&crate::abort_signal::NoAbort, voltages)?;
            discontinuity |= trial.evaluate(voltages)?.restarts_integration();
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
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn visit_mixed_transient_candidate_task(
        &mut self,
        abort: &dyn crate::abort_signal::AbortSignal,
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
    ) -> Result<MixedCandidateInspection, SimulationError> {
        if self.mixed_signal_hosts.is_empty() {
            return Ok(MixedCandidateInspection::default());
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
            trial.settle(abort, voltages)?;
            // An A/D crossing this settled trial placed strictly inside its
            // own interval is a root the solver is asked to land on.
            let mut refinement = trial.interior_root(minimum_timestep)?;
            if refinement.is_some() {
                return Ok(MixedCandidateInspection {
                    refinement,
                    analog_event_root: false,
                    discontinuity: false,
                });
            }
            // A design's `$finish`, `$stop` or `$fatal` (IEEE 1364-2005
            // section 17.4.1) at this candidate ends the run on it, through the
            // same request an analog `$finish` makes. The engine already
            // breakpoints the HDL tick, so the accepted point is the tick's.
            if kind == rspice_veriloga_runtime::AnalogTaskKind::Finish
                && !validate_acceptance
                && let Some(finish) = trial
                    .coordinator()
                    .and_then(crate::xspice::verilog::MixedDigitalCoordinator::hdl_finish)
            {
                consume(rspice_veriloga_runtime::AnalogTaskEvent {
                    instance: finish.instance,
                    model: finish.module,
                    call: rspice_veriloga_runtime::AnalogTaskInvocation {
                        kind: rspice_veriloga_runtime::AnalogTaskKind::Finish,
                        site: finish.finish.site,
                        time,
                        arguments: Box::new([
                            rspice_veriloga_runtime::AnalogTaskArgument::Integer(i64::from(
                                finish.finish.level,
                            )),
                        ]),
                    },
                });
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
            // No A/D root returned above, so any root left is an analog block's.
            Ok(MixedCandidateInspection {
                refinement,
                analog_event_root: refinement.is_some(),
                discontinuity,
            })
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
