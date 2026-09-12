//! Circuit ownership of digital execution and per-model observation banks.
use super::super::host::{DigitalActiveExchange, DigitalActiveParticipant};
use super::super::store::{ExternalBitDriverId, StoreError};
use super::*;
use crate::xspice::event_scheduler::EventTarget;
use crate::xspice::event_scheduler::SchedulerError;
use rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan;
use rspice_veriloga::canonical_ir::digital_link::{
    DigitalLinkDirection, DigitalLinkInstance, DigitalLinkNet, DigitalLinkPort,
    DigitalLinkedInstance, link_digital_plans,
};

/// An analog model observes resolved values; it owns no process, driver or queue.
#[derive(Clone)]
pub(super) struct SignalView {
    plan: Arc<CanonicalDigitalPlan>,
    bits: Vec<FourStateValue>,
    reals: Vec<f64>,
}

impl SignalView {
    fn new(plan: Arc<CanonicalDigitalPlan>) -> Self {
        let bits = plan
            .signals
            .iter()
            .map(|signal| {
                FourStateValue::splat(
                    signal.width,
                    if signal.procedurally_assignable {
                        FourStateBit::Unknown
                    } else {
                        FourStateBit::HighImpedance
                    },
                )
            })
            .collect();
        Self {
            reals: vec![0.0; plan.signals.len()],
            plan,
            bits,
        }
    }

    fn check(
        &self,
        signal: DigitalSignalId,
        value: &FourStateValue,
    ) -> Result<(), DigitalRunError> {
        let declared = self
            .plan
            .signal(signal)
            .ok_or(StoreError::UndeclaredSignal(signal))?;
        if declared.kind.is_real() {
            return Err(StoreError::RealPortDrivenWithBits {
                signal,
                name: declared.name.to_string(),
            }
            .into());
        }
        if declared.width != value.width() {
            return Err(StoreError::WidthMismatch {
                signal,
                name: declared.name.to_string(),
                declared: declared.width,
                offered: value.width(),
            }
            .into());
        }
        Ok(())
    }

    fn force_many(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
    ) -> Result<(), DigitalRunError> {
        for (signal, value) in drives {
            self.check(*signal, value)?;
        }
        for (signal, value) in drives {
            self.bits[usize::from(*signal)].clone_from(value);
        }
        Ok(())
    }
}

/// Standalone hosts execute locally. Enrolled circuit models hold only values;
/// the owning circuit supplies every process activation and resolved output.
#[derive(Clone)]
pub(super) enum MixedDigital {
    Owned(DigitalHost),
    View(SignalView),
}

impl MixedDigital {
    pub(super) fn plan(&self) -> &Arc<CanonicalDigitalPlan> {
        match self {
            Self::Owned(host) => host.plan(),
            Self::View(view) => &view.plan,
        }
    }
    pub(super) fn is_view(&self) -> bool {
        matches!(self, Self::View(_))
    }
    pub(super) fn fresh(&self) -> Self {
        match self {
            Self::Owned(host) => Self::Owned(host.fresh()),
            Self::View(view) => Self::View(SignalView::new(Arc::clone(&view.plan))),
        }
    }
    pub(super) fn signal(&self, name: &str) -> Result<DigitalSignalId, DigitalRunError> {
        self.plan()
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .map(|signal| signal.id)
            .ok_or_else(|| DigitalRunError::UnknownSignal { name: name.into() })
    }
    pub(super) fn read(&self, signal: DigitalSignalId) -> Option<&FourStateValue> {
        match self {
            Self::Owned(host) => host.read(signal),
            Self::View(view) => view.bits.get(usize::from(signal)),
        }
    }
    pub(super) fn read_real(&self, signal: DigitalSignalId) -> Option<f64> {
        match self {
            Self::Owned(host) => host.read_real(signal),
            Self::View(view) => view.reals.get(usize::from(signal)).copied(),
        }
    }
    pub(super) fn is_real(&self, signal: DigitalSignalId) -> bool {
        self.plan()
            .signal(signal)
            .is_some_and(|signal| signal.kind.is_real())
    }
    pub(super) fn declared_range(&self, signal: DigitalSignalId) -> VectorBounds {
        match self {
            Self::Owned(host) => host.declared_range(signal),
            Self::View(view) => view
                .plan
                .signal(signal)
                .map_or(VectorBounds::SCALAR, |signal| signal.declared_range()),
        }
    }
    pub(super) fn next_tick(&self) -> Option<u64> {
        match self {
            Self::Owned(host) => host.next_tick(),
            Self::View(_) => None,
        }
    }
    pub(super) fn sample_analog_probes(&mut self, values: &[Option<f64>]) {
        if let Self::Owned(host) = self {
            host.sample_analog_probes(values);
        }
    }
    pub(super) fn prepare_start(&mut self) -> Result<(), DigitalRunError> {
        match self {
            Self::Owned(host) => host.prepare_start(),
            Self::View(_) => Ok(()),
        }
    }
    pub(super) fn start(&mut self) -> Result<(), DigitalRunError> {
        match self {
            Self::Owned(host) => host.start(),
            Self::View(_) => Ok(()),
        }
    }
    pub(super) fn advance_to(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        match self {
            Self::Owned(host) => host.advance_to(tick),
            Self::View(_) => Ok(()),
        }
    }
    pub(super) fn force_many(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        match self {
            Self::Owned(host) => host.force_many(drives, tick),
            Self::View(view) => view.force_many(drives),
        }
    }
    /// `clock_seconds` dates the event, `candidate_seconds` names the analog
    /// timepoint. See `DigitalHost::force_many_from_analog_at`.
    pub(super) fn force_many_from_analog_at(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
        clock_seconds: f64,
        candidate_seconds: f64,
    ) -> Result<(), DigitalRunError> {
        match self {
            Self::Owned(host) => {
                host.force_many_from_analog_ordered(drives, tick, clock_seconds, candidate_seconds)
            }
            Self::View(view) => view.force_many(drives),
        }
    }
}

/// The circuit's one HDL process/driver/queue state. Instance maps are aligned
/// with CircuitData's analog-host order, independently of linked process order.
#[derive(Clone)]
pub(crate) struct MixedDigitalCoordinator {
    digital: MixedCell<DigitalHost>,
    maps: Vec<DigitalLinkedInstance>,
    port_signals: Vec<Vec<DigitalSignalId>>,
    event_nodes: Vec<usize>,
    resolution: TimeResolution,
    enabled: bool,
    accepted_time: Option<f64>,
    /// Smallest interval the analog solver is allowed to advance by, or zero
    /// when nothing has declared one.
    ///
    /// See [`Self::set_analog_step_floor`]: this is what separates an
    /// activation the stepper skipped from one it could not have landed on.
    analog_step_floor: f64,
    probes: Vec<Option<f64>>,
    drives: Vec<(DigitalSignalId, FourStateValue)>,
    /// Every A/D bit one settle pass moved, across every enrolled instance,
    /// ordered by the instant it crossed. Scratch, refilled per pass.
    publications: Vec<AdcPublication>,
}

/// One enrolled instance's A/D bit, dated by the instant it crossed.
#[derive(Clone, Copy)]
struct AdcPublication {
    /// The interpolated threshold crossing, or the trial's endpoint for a
    /// transition a D/A move inside this interval caused.
    crossing: f64,
    /// The tick that instant's event lands on.
    tick: u64,
    /// Position in the circuit's host order.
    host: usize,
    /// Index into that host's A/D bridge table.
    bridge: usize,
    bit: FourStateBit,
}

impl fmt::Debug for MixedDigitalCoordinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MixedDigitalCoordinator")
            .field("instances", &self.maps.len())
            .field("accepted_time", &self.accepted_time)
            .finish()
    }
}

fn port_net_name(instance: &str, signal: DigitalSignalId) -> String {
    format!(
        "@port:{}:{}:{}",
        instance.len(),
        instance,
        usize::from(signal)
    )
}

impl MixedDigitalCoordinator {
    pub(crate) fn enroll(
        hosts: &mut [MixedSignalHost],
        event_nodes: &std::collections::BTreeSet<usize>,
        control: &dyn rspice_veriloga::PipelineControl,
    ) -> Result<Self, MixedSignalError> {
        for host in hosts.iter() {
            host.require_idle("link circuit digital execution")?;
            if host.digital_started || host.state.started || host.state.digital.is_view() {
                return Err(MixedSignalError::TrialProtocol {
                    detail: "digital linking requires fresh, unenrolled instances".into(),
                });
            }
        }
        let mut ports = Vec::with_capacity(hosts.len());
        let mut private_nets = Vec::new();
        for host in hosts.iter() {
            let mut directions = std::collections::BTreeMap::new();
            for (signal, positive, negative, direction) in host
                .state
                .bridges
                .adc
                .iter()
                .map(|bridge| {
                    (
                        bridge.signal,
                        bridge.positive,
                        bridge.negative,
                        DigitalLinkDirection::Input,
                    )
                })
                .chain(host.state.bridges.dac.iter().map(|bridge| {
                    (
                        bridge.signal,
                        bridge.positive,
                        bridge.negative,
                        DigitalLinkDirection::Output,
                    )
                }))
            {
                if !event_nodes.contains(&positive) {
                    continue;
                }
                if negative != 0 {
                    return Err(MixedSignalError::InvalidBridge {
                        detail: "a differential electrical bridge cannot become an event net"
                            .into(),
                    });
                }
                directions
                    .entry(signal)
                    .and_modify(|previous| {
                        if *previous != direction {
                            *previous = DigitalLinkDirection::Inout;
                        }
                    })
                    .or_insert(direction);
            }
            let instance_ports: Vec<_> = directions
                .into_iter()
                .map(|(signal, direction)| {
                    let name = host
                        .state
                        .digital
                        .plan()
                        .signal(signal)
                        .unwrap()
                        .name
                        .to_string();
                    private_nets.push(DigitalLinkNet {
                        name: port_net_name(&host.instance, signal),
                        ports: vec![(host.instance.clone(), name.clone())],
                    });
                    DigitalLinkPort {
                        name,
                        signal,
                        direction,
                    }
                })
                .collect();
            ports.push(instance_ports);
        }
        let instances: Vec<_> = hosts
            .iter()
            .zip(&ports)
            .map(|(host, ports)| DigitalLinkInstance {
                name: &host.instance,
                plan: host.state.digital.plan(),
                ports,
            })
            .collect();
        // Variable ports keep an explicit continuous connection process. The
        // bit graph below only aliases wires and cannot bypass HDL regions.
        let linked = link_digital_plans(&instances, &private_nets, control).map_err(|errors| {
            MixedSignalError::Compile {
                detail: errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; "),
            }
        })?;
        let resolution = TimeResolution::new(linked.plan.timing.precision_exponent)
            .map_err(DigitalRunError::from)?;
        let mut maps: std::collections::BTreeMap<_, _> = linked
            .instances
            .into_iter()
            .map(|map| (map.name.clone(), map))
            .collect();
        let maps: Vec<_> = hosts
            .iter()
            .map(|host| {
                maps.remove(&host.instance)
                    .expect("linker retained every instance")
            })
            .collect();
        let port_signals: Vec<_> = hosts
            .iter()
            .zip(&maps)
            .zip(&ports)
            .map(|((host, map), ports)| {
                let mut signals = map.signals.clone();
                for port in ports {
                    signals[usize::from(port.signal)] =
                        linked.signal_names[&port_net_name(&host.instance, port.signal)];
                }
                signals
            })
            .collect();
        let mut bit_nets = std::collections::BTreeMap::<
            usize,
            Vec<super::super::store::DigitalBitConnection>,
        >::new();
        for (host, signals) in hosts.iter().zip(&port_signals) {
            for (signal, bit, node) in host
                .state
                .bridges
                .adc
                .iter()
                .map(|bridge| (bridge.signal, bridge.bit, bridge.positive))
                .chain(
                    host.state
                        .bridges
                        .dac
                        .iter()
                        .map(|bridge| (bridge.signal, bridge.bit, bridge.positive)),
                )
            {
                if event_nodes.contains(&node) {
                    bit_nets.entry(node).or_default().push(
                        super::super::store::DigitalBitConnection {
                            signal: signals[usize::from(signal)],
                            bit,
                        },
                    );
                }
            }
        }
        let (event_node_ids, bit_groups): (Vec<_>, Vec<_>) = bit_nets.into_iter().unzip();
        let views: Vec<_> = hosts
            .iter()
            .map(|host| {
                MixedCell::new(MixedDigital::View(SignalView::new(Arc::clone(
                    host.state.digital.plan(),
                ))))
            })
            .collect();
        let probes = vec![None; linked.plan.analog_probes.len()];
        // Global execution honors the strictest participant's configured
        // ceilings; enrolling a model must never silently relax its limits.
        let limits = hosts
            .iter()
            .map(|host| {
                let MixedDigital::Owned(digital) = &*host.state.digital else {
                    unreachable!("validated unenrolled instance")
                };
                digital.scheduler_limits()
            })
            .reduce(|left, right| SchedulerLimits {
                max_delta_cycles_per_tick: left
                    .max_delta_cycles_per_tick
                    .min(right.max_delta_cycles_per_tick),
                max_events_per_tick: left.max_events_per_tick.min(right.max_events_per_tick),
                max_reported_oscillating_entities: left
                    .max_reported_oscillating_entities
                    .min(right.max_reported_oscillating_entities),
            })
            .unwrap_or_default();
        let mut digital = DigitalHost::from_plan(Arc::new(linked.plan), resolution, limits);
        let dependencies: Vec<_> = hosts
            .iter()
            .zip(&maps)
            .flat_map(|(host, map)| {
                host.analog_probes
                    .iter()
                    .enumerate()
                    .filter_map(move |(index, probe)| {
                        matches!(probe, AnalogProbeWiring::Variable { .. }).then(|| {
                            (
                                map.analog_probes[index],
                                host.discrete_inputs
                                    .iter()
                                    .map(|input| map.signals[usize::from(input.signal)])
                                    .collect(),
                            )
                        })
                    })
            })
            .collect();
        digital.bind_analog_variable_inputs(&dependencies)?;
        if !bit_groups.is_empty() {
            digital.connect_bits(&bit_groups)?;
        }
        for (host, view) in hosts.iter_mut().zip(views) {
            if !event_nodes.is_empty() {
                host.strip_event_boundaries(event_nodes);
            }
            host.state.digital = view;
            host.resolution = resolution;
        }
        Ok(Self {
            digital: MixedCell::new(digital),
            maps,
            port_signals,
            event_nodes: event_node_ids,
            resolution,
            enabled: false,
            accepted_time: None,
            analog_step_floor: 0.0,
            probes,
            drives: Vec::new(),
            publications: Vec::new(),
        })
    }

    pub(crate) fn fresh(&self) -> Self {
        Self {
            digital: MixedCell::new(self.digital.fresh()),
            maps: self.maps.clone(),
            port_signals: self.port_signals.clone(),
            event_nodes: self.event_nodes.clone(),
            resolution: self.resolution,
            enabled: false,
            accepted_time: None,
            analog_step_floor: self.analog_step_floor,
            probes: vec![None; self.probes.len()],
            drives: Vec::new(),
            publications: Vec::new(),
        }
    }

    /// Resolve scheduler process identities back to their circuit instances.
    /// This runs only on failure and leaves the structured diagnostic intact.
    fn execution_error(&self, mut error: DigitalRunError) -> MixedSignalError {
        if let DigitalRunError::Scheduler(SchedulerError::Oscillation(diagnostic)) = &mut error {
            for (target, _) in &mut diagnostic.entities {
                if self.digital.is_external_target(target) {
                    continue;
                }
                if let Some(owner) = self.maps.iter().find(|map| {
                    map.processes
                        .iter()
                        .chain(&map.connection_processes)
                        .any(|process| usize::from(*process) == target.node_id)
                }) {
                    target.instance = format!(
                        "mixed Verilog-AMS instance '{}' process {}",
                        owner.name, target.instance
                    );
                }
            }
        }
        error.into()
    }

    pub(crate) fn start(&mut self) -> Result<(), MixedSignalError> {
        if !self.enabled {
            self.digital.make_mut().prepare_start()?;
            self.enabled = true;
        }
        Ok(())
    }

    pub(crate) fn event_bindings(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.event_nodes
            .iter()
            .enumerate()
            .map(|(net, &node)| (node, net))
    }

    pub(crate) fn attach_external_bits(
        &mut self,
        observed: &[usize],
        drivers: &[(usize, EventTarget)],
    ) -> Result<Vec<ExternalBitDriverId>, DigitalRunError> {
        self.digital
            .make_mut()
            .attach_external_bits(observed, drivers)
    }

    pub(crate) fn remap_circuit_nodes(&mut self, remap: impl Fn(usize) -> usize) {
        debug_assert!(!self.enabled);
        self.digital.make_mut().remap_external_nodes(&remap);
        // Retain group indices: the digital bit topology refers to these slots.
        for node in &mut self.event_nodes {
            *node = remap(*node);
        }
    }

    pub(crate) fn event_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.event_nodes.iter().copied().filter(|node| *node > 0)
    }

    pub(crate) fn event_values(
        &self,
    ) -> impl Iterator<Item = (usize, crate::xspice::DigitalValue)> + '_ {
        self.event_nodes
            .iter()
            .enumerate()
            .map(|(index, &node)| {
                (
                    node,
                    self.digital
                        .connected_value(index)
                        .expect("elaborated event bit"),
                )
            })
            .filter(|(node, _)| *node > 0)
    }

    pub(crate) fn next_event_time(&self) -> Result<Option<f64>, MixedSignalError> {
        self.digital
            .next_tick()
            .map(|tick| {
                self.resolution
                    .ticks_to_seconds(tick)
                    .map_err(DigitalRunError::from)
                    .map_err(Into::into)
            })
            .transpose()
    }

    /// Declare the smallest interval the analog solver may advance by.
    ///
    /// Zero — the default — means no solver has declared one, and every
    /// activation the analog side steps past is a synchronization fault. A
    /// positive floor is the transient stepper's hard minimum timestep, and it
    /// is what makes [`Self::begin_trial`] able to tell the two cases apart.
    pub(crate) fn set_analog_step_floor(&mut self, floor: f64) {
        self.analog_step_floor = if floor.is_finite() && floor > 0.0 {
            floor
        } else {
            0.0
        };
    }

    /// Whether an activation the analog side stepped past was one it could
    /// have landed on.
    ///
    /// A digital tick more than one hard-minimum step after the accepted
    /// analog time is reachable: the stepper had a legal interval to it and
    /// did not take it, which is the missed-breakpoint fault this refuses.
    /// One closer than that is not reachable at all — there is no analog time
    /// between the accepted point and it — so refusing it would end a run over
    /// a schedule the analog side is simply too coarse to resolve, which is
    /// not what Spectre/AMS Designer does with one.
    fn activation_was_reachable(&self, scheduled_seconds: f64) -> bool {
        if self.analog_step_floor <= 0.0 {
            return true;
        }
        let accepted = self.accepted_time.unwrap_or(0.0);
        scheduled_seconds - accepted >= self.analog_step_floor
    }

    /// Open a trial at an analog time.
    ///
    /// # Contract for a schedule finer than the analog resolution
    ///
    /// The digital scheduler owns the exact tick and the ordering of every
    /// activation; the analog solver owes an activation only a timepoint at or
    /// after it. So a trial whose time is past a pending activation is opened,
    /// not refused, whenever that activation is closer to the accepted analog
    /// time than [`Self::set_analog_step_floor`]'s interval: the queue is then
    /// drained to this trial's tick in tick order, and every activation inside
    /// that window coalesces onto this one analog timepoint. The refusal is
    /// kept for an activation the stepper could have landed on, because that
    /// one is a lost breakpoint rather than a resolution limit.
    pub(crate) fn begin_trial(
        &mut self,
        time: f64,
        probe: bool,
    ) -> Result<SharedDigitalTrial<'_>, MixedSignalError> {
        if !self.enabled {
            return Err(MixedSignalError::TrialProtocol {
                detail: "circuit digital execution must start before a trial".into(),
            });
        }
        let tick = self
            .resolution
            .seconds_to_floor_ticks(time)
            .map_err(DigitalRunError::from)?;
        if !probe && self.accepted_time.is_some_and(|accepted| time <= accepted) {
            return Err(MixedSignalError::TrialProtocol {
                detail: "circuit digital acceptance must advance time".into(),
            });
        }
        if let Some(next) = self.digital.next_tick()
            && next < tick
        {
            let scheduled_seconds = self
                .resolution
                .ticks_to_seconds(next)
                .map_err(DigitalRunError::from)?;
            if self.activation_was_reachable(scheduled_seconds) {
                return Err(MixedSignalError::MissedDigitalBreakpoint {
                    scheduled_seconds,
                    trial_seconds: time,
                });
            }
        }
        let rollback = self.digital.clone();
        Ok(SharedDigitalTrial {
            coordinator: self,
            rollback: Some(rollback),
            time,
            tick,
            published_tick: tick,
            probe,
        })
    }
}

/// The shared event state rolls back on every exit except explicit acceptance.
pub(crate) struct SharedDigitalTrial<'a> {
    coordinator: &'a mut MixedDigitalCoordinator,
    rollback: Option<MixedCell<DigitalHost>>,
    time: f64,
    tick: u64,
    /// The highest tick this trial has published an A/D bank at, carried for
    /// the life of the trial rather than for one settle pass.
    ///
    /// The same running maximum `ActiveTrial::published_tick` keeps on the
    /// standalone path, and monotone for the same reason: a settle pass is one
    /// Newton iteration of this trial, the next one solves a different
    /// candidate over the same interval, and a root it finds can interpolate
    /// behind a crossing this pass already published and ran the discrete half
    /// on. Re-seeding at the trial's floored tick each pass would let the
    /// store's clock go backwards inside one trial.
    published_tick: u64,
    probe: bool,
}

struct CircuitAnalogParticipant<'a, 'p> {
    hosts: &'a mut [MixedSignalHost],
    maps: &'a [DigitalLinkedInstance],
    solution: &'a [f64],
    external: Option<&'p mut dyn DigitalActiveParticipant>,
}

impl DigitalActiveParticipant for CircuitAnalogParticipant<'_, '_> {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        match &mut self.external {
            Some(external) => external.settle_active(exchange),
            None => {
                exchange.require_standalone_execution()?;
                Ok(false)
            }
        }
    }
    fn sample_analog(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<(), DigitalRunError> {
        let requested = exchange.analog_sample_requests();
        let mut samples = Vec::new();
        for (host, map) in self.hosts.iter_mut().zip(self.maps) {
            let mut producer = AnalogModelParticipant {
                instance: &host.instance,
                analog: &mut host.analog,
                inputs: &host.discrete_inputs,
                probes: &host.analog_probes,
                prepared: &mut host.prepared_analog,
                solution: self.solution,
                signals: Some(&map.signals),
                probe_ids: Some(&map.analog_probes),
            };
            producer.sample_into(exchange, &requested, &mut samples)?;
        }
        exchange.publish_analog_variables(&samples)
    }
}

impl SharedDigitalTrial<'_> {
    /// Read every process probe before allowing any instance to run.
    pub(crate) fn advance(
        &mut self,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
    ) -> Result<(), MixedSignalError> {
        self.advance_with(hosts, solution, None)
    }

    pub(crate) fn advance_with(
        &mut self,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
        participant: Option<&mut dyn DigitalActiveParticipant>,
    ) -> Result<(), MixedSignalError> {
        let coordinator = &mut self.coordinator;
        for (host, map) in hosts.iter().zip(&coordinator.maps) {
            host.validate_solution(solution)?;
            for (probe, global) in host.analog_probes.iter().zip(&map.analog_probes) {
                coordinator.probes[usize::from(*global)] = probe.sample(solution);
            }
        }
        let has_external = participant.is_some();
        let mut participant = CircuitAnalogParticipant {
            hosts,
            maps: &coordinator.maps,
            solution,
            external: participant,
        };
        if coordinator
            .digital
            .next_tick()
            .is_some_and(|next| next <= self.tick)
        {
            let digital = coordinator.digital.make_mut();
            digital.sample_analog_probes(&coordinator.probes);
            let advanced = digital.advance_to_with(self.tick, &mut participant);
            advanced.map_err(|error| coordinator.execution_error(error))?;
        }
        if has_external {
            // An XSPICE event or analog input can be due without an HDL timer.
            // Run that physical boundary through the causal lane so quantizing
            // its reporting tick cannot consume an unrelated future timer.
            //
            // Dated at the tick *at or after* the instant, not the nearest
            // one: this is a second event kernel's time base meeting the HDL
            // wheel, and the reverse direction hands an HDL tick to XSPICE at
            // exactly the instant that tick names. The inverse of an exact map
            // is the least tick not before the instant — see this module's
            // "three time bases", which also says why nearest-tick is the
            // rejected alternative here rather than a forbidden one.
            let tick = coordinator
                .resolution
                .seconds_to_ceil_ticks(self.time)
                .map_err(DigitalRunError::from)?;
            let digital = coordinator.digital.make_mut();
            digital.sample_analog_probes(&coordinator.probes);
            let advanced =
                digital.force_many_from_analog_with(&[], tick, self.time, &mut participant);
            advanced.map_err(|error| coordinator.execution_error(error))?;
        }
        Ok(())
    }

    /// Apply all A/D decisions together, preserving analog activation provenance.
    pub(crate) fn publish_adc(
        &mut self,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
    ) -> Result<bool, MixedSignalError> {
        self.publish_adc_with(hosts, solution, None)
    }

    /// Every circuit A/D transition of this settle pass, in ascending crossing
    /// order across all enrolled instances.
    ///
    /// Collected rather than published directly because the order is a
    /// property of the *circuit*, not of any one instance: two instances whose
    /// sense nodes crossed at different instants inside one analog step are
    /// two events at two ticks, and the shared wheel has to see them in the
    /// order the analog world produced them.
    fn collect_adc_publications(
        &mut self,
        hosts: &[MixedSignalHost],
    ) -> Result<(), MixedSignalError> {
        let trial_tick = self.tick;
        let coordinator = &mut self.coordinator;
        coordinator.publications.clear();
        for (index, host) in hosts.iter().enumerate() {
            // An endpoint-dated transition names no tick of its own — see
            // `settle_into`. Rounding this trial's own timestamp here would
            // publish the shared bank one tick past the instant the
            // integrator accepted, which is the same error in the shared path.
            let endpoint_dated = host.trial.as_ref().is_none_or(|trial| trial.dac_activity);
            for (&(bridge, bit), &(_, crossing)) in
                host.scratch.bit_drives.iter().zip(&host.scratch.crossings)
            {
                let tick = if endpoint_dated {
                    trial_tick
                } else {
                    coordinator
                        .resolution
                        .seconds_to_ticks(crossing)
                        .map_err(DigitalRunError::from)?
                };
                coordinator.publications.push(AdcPublication {
                    crossing,
                    tick,
                    host: index,
                    bridge,
                    bit,
                });
            }
        }
        coordinator.publications.sort_by(|left, right| {
            left.crossing
                .total_cmp(&right.crossing)
                .then(left.host.cmp(&right.host))
                .then(left.bridge.cmp(&right.bridge))
        });
        Ok(())
    }

    pub(crate) fn publish_adc_with(
        &mut self,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
        mut participant: Option<&mut dyn DigitalActiveParticipant>,
    ) -> Result<bool, MixedSignalError> {
        self.collect_adc_publications(hosts)?;
        // The analog candidate instant, which every group publishes against:
        // the continuous half has a solution at the trial's endpoint and
        // nowhere else inside the step, however finely the discrete half dates
        // the transitions it found there.
        let candidate_seconds = self.time;
        let mut published_any = false;
        let mut group = 0;
        while group < self.coordinator.publications.len() {
            let crossing = self.coordinator.publications[group].crossing;
            let end = self.coordinator.publications[group..]
                .iter()
                .position(|entry| entry.crossing != crossing)
                .map_or(self.coordinator.publications.len(), |offset| group + offset);
            // Monotone across the trial, not merely across this pass: a
            // crossing in the lower half of a tick rounds to a slot the
            // digital world has already left, and an earlier group — of this
            // pass or of an earlier Newton iteration of this same trial — may
            // have left a later one still.
            self.published_tick = self.coordinator.publications[group..end]
                .iter()
                .map(|entry| entry.tick)
                .fold(self.published_tick, u64::max);
            let published_tick = self.published_tick;
            let coordinator = &mut self.coordinator;
            coordinator.drives.clear();
            for entry in &coordinator.publications[group..end] {
                let host = &hosts[entry.host];
                let bridge = &host.state.bridges.adc[entry.bridge];
                let global = coordinator.port_signals[entry.host][usize::from(bridge.signal)];
                // Preserve event-connected bits of a partly electrical vector,
                // and bits of it that crossed at another instant. Only
                // physical A/D decisions at *this* instant are external forces.
                match coordinator
                    .drives
                    .iter()
                    .position(|(held, _)| *held == global)
                {
                    Some(slot) => coordinator.drives[slot].1.set_bit(bridge.bit, entry.bit),
                    None => {
                        let mut value = coordinator
                            .digital
                            .read(global)
                            .expect("mapped A/D port")
                            .clone();
                        value.set_bit(bridge.bit, entry.bit);
                        coordinator.drives.push((global, value));
                    }
                }
            }
            let (held, drives) = (&coordinator.digital, &mut coordinator.drives);
            drives.retain(|(global, value)| held.read(*global) != Some(value));
            group = end;
            if coordinator.drives.is_empty() {
                continue;
            }
            let digital = coordinator.digital.make_mut();
            digital.sample_analog_probes(&coordinator.probes);
            // Reborrowed through a `match` rather than `Option::map`, which
            // would hand the closure's return the outer lifetime and so lend
            // the participant to every later group at once.
            let external: Option<&mut dyn DigitalActiveParticipant> = match &mut participant {
                Some(external) => Some(&mut **external),
                None => None,
            };
            let mut active = CircuitAnalogParticipant {
                hosts: &mut *hosts,
                maps: &coordinator.maps,
                solution,
                external,
            };
            // The crossing dates the event — it is what every process this
            // publication wakes reads as `$abstime`. The trial's own timestamp
            // stays the analog candidate instant, because that is the only
            // time the continuous half has a solution at.
            let result = digital.force_many_from_analog_at(
                &coordinator.drives,
                published_tick,
                crossing,
                candidate_seconds,
                &mut active,
            );
            result.map_err(|error| coordinator.execution_error(error))?;
            published_any = true;
        }
        Ok(published_any)
    }

    /// Refresh analog-facing values after shared settlement. No model view can
    /// advance a process or resolve a driver independently.
    pub(crate) fn synchronize(
        &mut self,
        hosts: &mut [MixedSignalHost],
    ) -> Result<bool, MixedSignalError> {
        let mut changed = false;
        for (host, map) in hosts.iter_mut().zip(&self.coordinator.maps) {
            let differs = map.signals.iter().enumerate().any(|(local, &global)| {
                let local = DigitalSignalId::from(local);
                if host.state.digital.is_real(local) {
                    host.state.digital.read_real(local).map(f64::to_bits)
                        != self.coordinator.digital.read_real(global).map(f64::to_bits)
                } else {
                    host.state.digital.read(local) != self.coordinator.digital.read(global)
                }
            });
            if !differs {
                continue;
            }
            let mut dac_moved = Vec::new();
            for (index, bridge) in host.state.bridges.dac.iter().enumerate() {
                if host
                    .state
                    .digital
                    .read(bridge.signal)
                    .map(|value| value.bit(bridge.bit))
                    != self
                        .coordinator
                        .digital
                        .read(map.signals[usize::from(bridge.signal)])
                        .map(|value| value.bit(bridge.bit))
                {
                    dac_moved.push(index);
                }
            }
            let MixedDigital::View(view) = host.state.digital.make_mut() else {
                return Err(MixedSignalError::TrialProtocol {
                    detail: "a coordinated mixed instance lost its signal view".into(),
                });
            };
            for (local, &global) in map.signals.iter().enumerate() {
                if view.plan.signals[local].kind.is_real() {
                    view.reals[local] = self
                        .coordinator
                        .digital
                        .read_real(global)
                        .expect("linked real signal");
                } else if let Some(value) = self.coordinator.digital.read(global) {
                    if view.bits[local] != *value {
                        view.bits[local].clone_from(value);
                    }
                }
            }
            if let Some(trial) = &mut host.trial {
                for index in dac_moved {
                    trial.vectors.dac_moved[index] = true;
                }
                trial.bridges_quiet = false;
            }
            changed = true;
        }
        Ok(changed)
    }

    pub(crate) fn commit(mut self) {
        assert!(
            !self.probe,
            "a numerical probe cannot commit shared digital state"
        );
        self.coordinator.accepted_time = Some(self.time);
        self.rollback = None;
    }
}

impl Drop for SharedDigitalTrial<'_> {
    fn drop(&mut self) {
        if let Some(rollback) = self.rollback.take() {
            self.coordinator.digital = rollback;
        }
    }
}
