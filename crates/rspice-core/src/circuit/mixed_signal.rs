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
//! electrical boundaries remain physical loads; direct XSPICE event bindings
//! still require event-driver enrollment and are explicitly refused.

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
        integration: rspice_veriloga::vm::IntegrationCoefficients,
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
            let started = if probe {
                host.begin_probe_trial(time, dt, integration, initial, final_step)
            } else {
                host.begin_trial(time, dt, integration, initial, final_step)
            };
            named(host, started)?;
        }
        Ok(group)
    }

    fn settle(
        &mut self,
        digital: &mut crate::xspice::verilog::SharedDigitalTrial<'_>,
        voltages: &[Value],
    ) -> Result<(), SimulationError> {
        digital
            .advance(self.hosts, voltages)
            .map_err(shared_error)?;
        digital.synchronize(self.hosts).map_err(shared_error)?;
        for _ in 0..MAX_BOUNDARY_SETTLE_PASSES {
            let mut moved = false;
            for host in self.hosts.iter_mut() {
                let settled = host.settle_analog_bridges(voltages);
                moved |= named(host, settled)?;
            }
            // All A/D decisions are published before any dependent HDL process
            // runs. Analog equations read the resulting bank only after quiet.
            moved |= digital.publish_adc(self.hosts).map_err(shared_error)?;
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

    fn promote(&mut self) -> Result<(), SimulationError> {
        let mut prepared = Vec::with_capacity(self.hosts.len());
        for host in self.hosts.iter_mut() {
            prepared.push(
                host.prepare_circuit_acceptance()
                    .map_err(|(instance, error)| mixed_error(&instance, error))?,
            );
        }
        for candidate in prepared {
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
        control: &dyn rspice_veriloga::PipelineControl,
    ) -> Result<(), SimulationError> {
        if !self.mixed_signal_hosts.is_empty() && self.mixed_digital_coordinator.is_none() {
            self.mixed_digital_coordinator = Some(
                crate::xspice::verilog::MixedDigitalCoordinator::enroll(
                    &mut self.mixed_signal_hosts,
                    control,
                )
                .map_err(shared_error)?,
            );
        }
        Ok(())
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
        self.finalize_mixed_digital(&rspice_veriloga::NoPipelineControl)?;
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
        if self.mixed_signal_hosts.is_empty() {
            return Ok(());
        }
        let integration = mixed_integration_coefficients(time, dt, coefficients)?;
        let mut digital = self
            .mixed_digital_coordinator
            .as_mut()
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "mixed circuit digital execution has not been elaborated".into(),
                )
            })?
            .begin_trial(time, true)
            .map_err(shared_error)?;
        let mut group = MixedHostTrialGroup::begin(
            &mut self.mixed_signal_hosts,
            time,
            dt,
            integration,
            analysis_step,
            true,
        )?;
        group.settle(&mut digital, voltages)?;
        for host in group.hosts.iter_mut() {
            let stamped = host.stamp(
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
            );
            named(host, stamped)?;
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
            group.settle(&mut digital, voltages)?;
            for host in group.hosts.iter_mut() {
                let stamped = host.stamp(voltages, |_, _, _| {}, |_, _| {});
                named(host, stamped)?;
                discontinuity |= host.analog_device().discontinuity_rising();
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
        coefficients: &crate::numerics::integration::CompanionCoefficients,
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
        let mut boundary_root: Option<Value> = None;
        for host in &self.mixed_signal_hosts {
            if let Some(target) = named(
                host,
                host.analog_boundary_refinement_time(time, voltages, minimum_timestep),
            )? {
                boundary_root = Some(boundary_root.map_or(target, |current| current.min(target)));
            }
        }
        if boundary_root.is_some() {
            return Ok((boundary_root, false));
        }
        let integration = mixed_integration_coefficients(time, dt, coefficients)?;
        let mut refinement: Option<Value> = None;
        let mut discontinuity = false;
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
        group.settle(&mut digital, voltages)?;
        for host in group.hosts.iter_mut() {
            let inspected = (|| {
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
            named(host, inspected)?;
        }
        Ok((refinement, discontinuity))
    }

    /// Earliest scheduled digital activation across every mixed module.
    pub(crate) fn next_mixed_event_time(&self) -> Result<Option<Value>, SimulationError> {
        self.mixed_digital_coordinator
            .as_ref()
            .map(|digital| digital.next_event_time().map_err(shared_error))
            .transpose()
            .map(Option::flatten)
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
