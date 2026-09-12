//! Circuit-wide mixed Verilog-AMS trials and acceptance.
//!
//! The circuit owns one linked HDL process/driver/event state. Each mixed
//! model retains its analog equations, electrical A/D and D/A adapters, and a
//! local view of resolved digital signals. A numerical trial samples every
//! analog probe before due HDL execution, then publishes all A/D decisions
//! before dependent processes run. Only after shared settlement do analog
//! equations consume the digital values and stamp their contributions.
//!
//! A group guard restores every model view and analog candidate on refusal,
//! while the shared digital guard restores process resumptions, drivers and
//! queues. Acceptance validates every model before promoting any participant.
//! A CircuitData clone contains both the accepted shared runtime and its views;
//! an individual model checkpoint cannot represent the shared process state.
//!
//! The shared runtime's next event joins the transient breakpoint list. Analog
//! electrical boundaries remain physical loads. Enrolled XSPICE event drivers
//! participate in the same numerical probe and acceptance barrier. Static
//! history observes the settled candidate without replaying digital events.

#[cfg(test)]
#[path = "mixed_signal/coupled_tests.rs"]
mod coupled_tests;

use super::external_models::{
    VerilogACompanionRules, XspiceAcceptanceRollback, XspiceCompanionPolicy, XspiceDigitalBindings,
    XspiceDigitalParticipant,
};
use crate::circuit::CircuitData;
use crate::xspice::verilog::host::DigitalActiveParticipant;
use crate::{SimulationError, Value};

use crate::xspice::verilog::{BoundaryBitSource, BoundaryBus, MixedSignalError, MixedSignalHost};

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
    companion: XspiceCompanionPolicy<'_>,
) -> Result<VerilogACompanionRules, SimulationError> {
    VerilogACompanionRules::from_policy(dt, companion).map_err(|error| {
        SimulationError::Circuit(format!(
            "mixed Verilog-AMS modules cannot advance to t={time:.16e}s: {error}"
        ))
    })
}

fn mixed_error(instance: &str, error: MixedSignalError) -> SimulationError {
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
fn named<T>(
    host: &MixedSignalHost,
    result: Result<T, MixedSignalError>,
) -> Result<T, SimulationError> {
    result.map_err(|error| mixed_error(host.instance_name(), error))
}

/// Own all instance trials together. Every speculative exit restores all views
/// and analog candidates; the shared digital trial independently restores queues.
struct MixedHostTrialGroup<'a> {
    hosts: &'a mut [MixedSignalHost],
    active: bool,
}

impl<'a> MixedHostTrialGroup<'a> {
    fn begin(
        hosts: &'a mut [MixedSignalHost],
        time: Value,
        dt: Value,
        integration: VerilogACompanionRules,
        analysis_step: Option<(bool, bool)>,
        probe: bool,
    ) -> Result<Self, SimulationError> {
        if let Some(host) = hosts.iter().find(|host| host.trial_active()) {
            return Err(mixed_error(
                host.instance_name(),
                MixedSignalError::TrialProtocol {
                    detail: "a circuit trial cannot overlap an existing model trial".into(),
                },
            ));
        }
        let group = Self {
            hosts,
            active: true,
        };
        for host in group.hosts.iter_mut() {
            let (initial, final_step) = analysis_step.unwrap_or_else(|| host.analysis_step());
            let started = host.begin_trial_with_integration_rules(
                time,
                dt,
                integration.derivative,
                integration.state,
                initial,
                final_step,
                probe,
            );
            named(host, started)?;
        }
        Ok(group)
    }

    fn settle(
        &mut self,
        digital: &mut crate::xspice::verilog::SharedDigitalTrial<'_>,
        voltages: &[Value],
    ) -> Result<(), SimulationError> {
        self.settle_with(digital, voltages, None)
    }

    fn settle_with(
        &mut self,
        digital: &mut crate::xspice::verilog::SharedDigitalTrial<'_>,
        voltages: &[Value],
        mut participant: Option<&mut dyn DigitalActiveParticipant>,
    ) -> Result<(), SimulationError> {
        match &mut participant {
            Some(participant) => {
                digital.advance_with(self.hosts, voltages, Some(&mut **participant))
            }
            None => digital.advance(self.hosts, voltages),
        }
        .map_err(shared_error)?;
        digital.synchronize(self.hosts).map_err(shared_error)?;
        // After the synchronize, because an enrolled instance reads its
        // boundary through a view and that is what refreshes one: asking
        // before it compares the view against itself and always answers no.
        //
        // The coordinator ran the wheel for every enrolled instance at once,
        // so whether that run moved something the analog equations read — a
        // D/A output, or a discrete variable an analog block references — is a
        // question about the whole circuit rather than about any one instance:
        // one instance's bridge and another's A/D input can share a deck node,
        // and one instance's variable steers the current it pushes into a node
        // any other may sense. Ask every instance whether its own digital half
        // moved one, and report the disjunction to all of them before any of
        // their bridges are sampled.
        let mut fed_back = false;
        for host in self.hosts.iter() {
            fed_back |= named(host, host.digital_feedback_since_trial_start())?;
        }
        if fed_back {
            for host in self.hosts.iter_mut() {
                host.note_shared_digital_feedback();
            }
        }
        for _ in 0..MAX_BOUNDARY_SETTLE_PASSES {
            let mut moved = false;
            for host in self.hosts.iter_mut() {
                let settled = host.settle_analog_bridges(voltages);
                moved |= named(host, settled)?;
            }
            // All A/D decisions are published before any dependent HDL process
            // runs. Analog equations read the resulting bank only after quiet.
            moved |= match &mut participant {
                Some(participant) => {
                    digital.publish_adc_with(self.hosts, voltages, Some(&mut **participant))
                }
                None => digital.publish_adc(self.hosts, voltages),
            }
            .map_err(shared_error)?;
            moved |= digital.synchronize(self.hosts).map_err(shared_error)?;
            if !moved {
                return Ok(());
            }
        }
        let host = &self.hosts[0];
        Err(mixed_error(
            host.instance_name(),
            host.boundary_settle_oscillation(MAX_BOUNDARY_SETTLE_PASSES),
        ))
    }

    fn static_residual(&mut self, solution: &[Value]) -> Result<Vec<Value>, SimulationError> {
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

    /// Tell every instance that the circuit's shared queue had an activation
    /// of its own due at this trial's tick.
    ///
    /// Called on the acceptance paths only, because acceptance is where the
    /// answer is used: a boundary move at a timepoint a process was due to run
    /// at is the schedule's, and the accepted-flip ceiling does not count it.
    /// An enrolled instance holds no queue and cannot read this for itself.
    fn note_scheduled_activation(&mut self) {
        for host in self.hosts.iter_mut() {
            host.note_scheduled_activation();
        }
    }

    fn prepare(
        &mut self,
    ) -> Result<Vec<crate::xspice::verilog::PreparedMixedAcceptance<'_>>, SimulationError> {
        let mut prepared = Vec::with_capacity(self.hosts.len());
        for host in self.hosts.iter_mut() {
            prepared.push(
                host.prepare_circuit_acceptance()
                    .map_err(|(instance, error)| mixed_error(&instance, error))?,
            );
        }
        Ok(prepared)
    }

    fn promote(&mut self) -> Result<(), SimulationError> {
        for candidate in self.prepare()? {
            candidate.commit();
        }
        self.active = false;
        Ok(())
    }
}

impl Drop for MixedHostTrialGroup<'_> {
    fn drop(&mut self) {
        if self.active {
            for host in self.hosts.iter_mut() {
                if host.trial_active() {
                    let _ = host.reject_trial();
                }
            }
        }
    }
}

fn shared_error(error: MixedSignalError) -> SimulationError {
    SimulationError::Circuit(format!("mixed circuit digital execution: {error}"))
}

/// Temporarily split the circuit's digital owner/views from its code-model
/// storage. All exits put the owners back. XSPICE probes retain state across
/// Active waves and restore their COW model/queue images and external resources
/// only after the entire mixed evaluation, including stamps, has finished.
/// Accepted candidates instead use the engine's outer XSPICE/resource journal.
struct MixedCircuitOwner<'a> {
    circuit: &'a mut CircuitData,
    coordinator: Option<crate::xspice::verilog::MixedDigitalCoordinator>,
    hosts: Vec<MixedSignalHost>,
    xspice: Option<XspiceAcceptanceRollback>,
}
impl<'a> MixedCircuitOwner<'a> {
    fn begin(circuit: &'a mut CircuitData, capture_xspice: bool) -> Result<Self, SimulationError> {
        let coordinator = circuit.mixed_digital_coordinator.take().ok_or_else(|| {
            SimulationError::Circuit(
                "mixed circuit digital execution has not been elaborated".into(),
            )
        })?;
        let hosts = std::mem::take(&mut circuit.mixed_signal_hosts);
        let xspice = (capture_xspice && circuit.has_coupled_event_nets())
            .then(|| circuit.capture_xspice_acceptance());
        Ok(Self {
            circuit,
            coordinator: Some(coordinator),
            hosts,
            xspice,
        })
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
impl Drop for MixedCircuitOwner<'_> {
    fn drop(&mut self) {
        // An explicit finish reports restoration errors. Unwinding still
        // restores every context/queue; a resource failure latches the circuit.
        if let Err(error) = self.restore_xspice() {
            log::error!("{error}");
        }
        self.circuit.mixed_digital_coordinator = self.coordinator.take();
        self.circuit.mixed_signal_hosts = std::mem::take(&mut self.hosts);
    }
}

impl CircuitData {
    fn with_mixed_probe<T>(
        &mut self,
        evaluate: impl FnOnce(
            &mut CircuitData,
            &mut crate::xspice::verilog::MixedDigitalCoordinator,
            &mut [MixedSignalHost],
            Option<&crate::xspice::ResourceTransaction>,
        ) -> Result<T, SimulationError>,
    ) -> Result<T, SimulationError> {
        let mut probe = MixedCircuitOwner::begin(self, true)?;
        let result = evaluate(
            probe.circuit,
            probe.coordinator.as_mut().expect("owned coordinator"),
            &mut probe.hosts,
            probe.xspice.as_ref().map(|snapshot| snapshot.resources()),
        );
        match (result, probe.restore_xspice()) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
            (Err(error), Err(restore)) => {
                Err(SimulationError::Circuit(format!("{error}; {restore}")))
            }
        }
    }

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
    pub(crate) fn add_mixed_signal_host(
        &mut self,
        mut host: MixedSignalHost,
    ) -> Result<(), SimulationError> {
        if self.mixed_digital_coordinator.is_some() {
            return Err(SimulationError::Circuit(
                "mixed instances cannot be added after circuit digital elaboration".into(),
            ));
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
        if !self.mixed_signal_hosts.is_empty() && self.mixed_digital_coordinator.is_none() {
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
            self.mixed_digital_coordinator = Some(coordinator);
            self.mixed_xspice_bindings = bindings.map(std::sync::Arc::new);
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
                return Err(SimulationError::Circuit(format!(
                    "mixed Verilog-AMS instance '{}' connects its discrete port '{}' to node '{}', \
                     which carries {} event-driven XSPICE values. This connection requires \
                     an explicit shared conversion contract for continuous loading or unlike \
                     event domains; direct shared resolution currently requires a digital-only net",
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
        self.finalize_mixed_digital(
            &std::collections::BTreeSet::new(),
            &rspice_veriloga::NoPipelineControl,
        )?;
        for host in &mut self.mixed_signal_hosts {
            let started = host.start_digital_execution();
            named(host, started)?;
        }
        if let Some(digital) = &mut self.mixed_digital_coordinator {
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
        companion: XspiceCompanionPolicy<'_>,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), SimulationError> {
        self.stamp_mixed_trial(
            matrix,
            rhs,
            time,
            dt,
            voltages,
            companion,
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
        companion: XspiceCompanionPolicy<'_>,
        analysis_step: Option<(bool, bool)>,
    ) -> Result<(), SimulationError> {
        if self.mixed_signal_hosts.is_empty() {
            return Ok(());
        }
        let integration = mixed_integration_coefficients(time, dt, companion)?;
        let bindings = self.mixed_xspice_bindings.clone();
        self.with_mixed_probe(|circuit, coordinator, hosts, resources| {
            let mut digital = coordinator.begin_trial(time, true).map_err(shared_error)?;
            let mut group = MixedHostTrialGroup::begin(hosts, time, dt, integration, analysis_step, true)?;
            if let Some(bindings) = &bindings {
                let mut participant = XspiceDigitalParticipant::new(circuit, bindings, voltages, time, dt,
                    if analysis_step.is_some() { crate::xspice::AnalysisType::Transient }
                    else { crate::xspice::AnalysisType::DcOp },
                    crate::xspice::EvaluationPhase::CircuitTrial, companion, resources);
                group.settle_with(&mut digital, voltages, Some(&mut participant))?;
                // The same settled code-model candidate supplies its stamps.
                // Dropping the participant ends its borrow, not the trial.
                drop(participant);
                circuit
                    .stamp_xspice(matrix, rhs)
                    .map_err(SimulationError::from)?;
            } else {
                group.settle(&mut digital, voltages)?;
            }
            // Only HDL equations and their physical D/A bridges receive this
            // weight. Enrolled XSPICE stamps above already apply their policy.
            let weight = if companion.xyce_one_step_order2 { 0.5 } else { 1.0 };
            for host in group.hosts.iter_mut() {
                let stamped = host.stamp_trial(voltages, |row, col, value| {
                    if matrix.get_index(row, col).is_some() { matrix.add(row, col, weight * value); }
                    else { log::debug!("mixed Verilog-AMS stamp ({row}, {col}) missing from matrix topology"); }
                }, |row, value| {
                    if let Some(slot) = rhs.get_mut(row) { *slot += weight * value; }
                });
                named(host, stamped)?;
            }
            Ok(())
        })
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
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
            None,
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
        let integration = mixed_integration_coefficients(time, dt, companion)?;
        let bindings = self.mixed_xspice_bindings.clone();
        if bindings.is_some() && resources.is_none() {
            return Err(SimulationError::Circuit(
                "shared acceptance requires its resource transaction".into(),
            ));
        }
        let mut owner = MixedCircuitOwner::begin(self, false)?;
        let mut digital = owner
            .coordinator
            .as_mut()
            .expect("owned coordinator")
            .begin_trial(time, false)
            .map_err(shared_error)?;
        let mut group = MixedHostTrialGroup::begin(
            &mut owner.hosts,
            time,
            dt,
            integration,
            Some((initial_step, final_step)),
            false,
        )?;
        if digital.opened_on_scheduled_activation() {
            group.note_scheduled_activation();
        }
        if let Some(bindings) = &bindings {
            let mut projected_quiet = false;
            // Carried across the projection passes, and the reason an accepted
            // step dispatches each instance once: a pass that re-settles the
            // boundary after moving the solution resumes the candidate's own
            // Active wave rather than opening a new one, so it evaluates the
            // instances whose inputs the projection invalidated instead of
            // every instance in the circuit. Re-running the rest would not ask
            // the same question twice — it would run their `AcceptedStep`
            // effects twice, and a model that requests a breakpoint, writes a
            // transactional resource or advances external state does not undo
            // the first one.
            let mut wave = None;
            let mut moved: Vec<usize> = Vec::new();
            for pass in 0..MAX_BOUNDARY_SETTLE_PASSES {
                let mut participant = if pass == 0 {
                    XspiceDigitalParticipant::new(
                        owner.circuit,
                        bindings,
                        solution,
                        time,
                        dt,
                        crate::xspice::AnalysisType::Transient,
                        crate::xspice::EvaluationPhase::AcceptedStep,
                        companion,
                        resources,
                    )
                } else {
                    XspiceDigitalParticipant::resume(
                        owner.circuit,
                        bindings,
                        solution,
                        time,
                        dt,
                        crate::xspice::AnalysisType::Transient,
                        crate::xspice::EvaluationPhase::AcceptedStep,
                        companion,
                        resources,
                        wave.take(),
                        &moved,
                    )
                };
                group.settle_with(&mut digital, solution, Some(&mut participant))?;
                wave = participant.into_wave();
                let num_nodes = owner.circuit.num_nodes();
                let (updates, refusal) =
                    owner
                        .circuit
                        .project_xspice_voltage_outputs(solution, num_nodes, Some(time));
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
            group.settle(&mut digital, solution)?;
        }
        let mut discontinuity = false;
        for host in group.hosts.iter_mut() {
            let stamped = host.stamp(solution, |_, _, _| {}, |_, _| {});
            named(host, stamped)?;
            discontinuity |= host.candidate_discontinuity();
        }
        // Reserve every HDL candidate before native state can be promoted.
        // Dropping these reservations on a callback error unwinds all hosts.
        let static_history = capture_static_history
            .then(|| group.static_residual(solution))
            .transpose()?;
        let prepared = group.prepare()?;
        let result = finish(
            owner.circuit,
            solution,
            projected,
            static_history.as_deref(),
        )?;
        for candidate in prepared {
            candidate.commit();
        }
        group.active = false;
        digital.commit();
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
        let integration = mixed_integration_coefficients(
            time,
            dt,
            XspiceCompanionPolicy {
                coefficients,
                xyce_one_step_order2: false,
            },
        )?;
        self.validate_nonmixed_model_acceptance()
            .map_err(SimulationError::Circuit)?;
        let mut discontinuity = self.veriloga_discontinuity_rising();
        if !self.mixed_signal_hosts.is_empty() {
            let mut digital = self
                .mixed_digital_coordinator
                .as_mut()
                .ok_or_else(|| {
                    SimulationError::Circuit(
                        "mixed circuit digital execution has not been elaborated".into(),
                    )
                })?
                .begin_trial(time, false)
                .map_err(shared_error)?;
            let mut group = MixedHostTrialGroup::begin(
                &mut self.mixed_signal_hosts,
                time,
                dt,
                integration,
                Some((initial_step, final_step)),
                false,
            )?;
            if digital.opened_on_scheduled_activation() {
                group.note_scheduled_activation();
            }
            group.settle(&mut digital, voltages)?;
            for host in group.hosts.iter_mut() {
                let stamped = host.stamp(voltages, |_, _, _| {}, |_, _| {});
                named(host, stamped)?;
                discontinuity |= host.candidate_discontinuity();
            }
            group.promote()?;
            digital.commit();
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
        let integration = mixed_integration_coefficients(time, dt, companion)?;
        let bindings = self.mixed_xspice_bindings.clone();
        self.with_mixed_probe(|circuit, coordinator, hosts, resources| {
            let mut refinement: Option<Value> = None;
            let mut discontinuity = false;
            let mut digital = coordinator.begin_trial(time, false).map_err(shared_error)?;
            let mut group = MixedHostTrialGroup::begin(
                hosts,
                time,
                dt,
                integration,
                Some((initial_step, final_step)),
                false,
            )?;
            if let Some(bindings) = &bindings {
                let mut participant = XspiceDigitalParticipant::new(
                    circuit,
                    bindings,
                    voltages,
                    time,
                    dt,
                    crate::xspice::AnalysisType::Transient,
                    crate::xspice::EvaluationPhase::CircuitTrial,
                    companion,
                    resources,
                );
                group.settle_with(&mut digital, voltages, Some(&mut participant))?;
            } else {
                group.settle(&mut digital, voltages)?;
            }
            // An A/D crossing this settled trial placed strictly inside its
            // own interval is a root the solver is asked to land on. It is
            // read from the trial rather than predicted before one was opened,
            // because whether a D/A bridge moved in this interval — the fact
            // that decides whether the crossing is interior at all — is only
            // established by running the discrete half. The trial rolls back
            // either way, so nothing this settle published survives the
            // refusal.
            for host in group.hosts.iter() {
                if let Some(target) =
                    named(host, host.trial_boundary_refinement_time(minimum_timestep))?
                {
                    refinement =
                        Some(refinement.map_or(target, |current: Value| current.min(target)));
                }
            }
            if refinement.is_some() {
                return Ok((refinement, false));
            }
            for host in group.hosts.iter_mut() {
                let inspected = (|| {
                    host.stamp(voltages, |_, _, _| {}, |_, _| {})?;
                    discontinuity |= host.candidate_discontinuity();
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
                        })?
                    {
                        consume(event);
                    }
                    Ok(())
                })();
                named(host, inspected)?;
            }
            Ok((refinement, discontinuity))
        })
    }

    /// Publish the transient stepper's hard minimum timestep to every mixed
    /// module, and to the coupled code-model queue beside them.
    ///
    /// A module's digital half schedules on its own declared precision, which
    /// can be finer than any interval the analog solver is allowed to advance
    /// by. Without this the two halves disagree about what "stepped past an
    /// activation" means, and a schedule the analog side merely cannot resolve
    /// is reported as a lost breakpoint. With it, such an activation keeps its
    /// exact digital tick and coalesces onto the next analog timepoint.
    ///
    /// One floor, both kernels. A code model sharing an event net with a mixed
    /// module schedules on the same picosecond-and-finer grid — ngspice clamps
    /// a gate delay at 1 ps, which is a tenth of the minimum a one-second
    /// maximum timestep leaves the solver — and
    /// `engine::transient::accepted_veriloga_event_time` lands its events by
    /// the same contract as an HDL tick. So its own guard needs the same
    /// interval to measure against, and gets it here rather than from a second
    /// knob: see `circuit::external_models::coupled`'s `event_was_reachable`.
    pub(crate) fn set_mixed_analog_step_floor(&mut self, floor: Value) {
        if let Some(digital) = self.mixed_digital_coordinator.as_mut() {
            digital.set_analog_step_floor(floor);
        }
        for host in &mut self.mixed_signal_hosts {
            host.set_analog_step_floor(floor);
        }
        self.xspice_analog_step_floor = if floor.is_finite() && floor > 0.0 {
            floor
        } else {
            0.0
        };
    }

    /// Earliest scheduled digital activation across every mixed module.
    pub(crate) fn next_mixed_event_time(&self) -> Result<Option<Value>, SimulationError> {
        self.mixed_digital_coordinator
            .as_ref()
            .map(|digital| {
                digital
                    .next_event_time()
                    .map(|next| next.map(|(_, seconds)| seconds))
                    .map_err(shared_error)
            })
            .transpose()
            .map(Option::flatten)
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
        if let Some(digital) = &self.mixed_digital_coordinator {
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
