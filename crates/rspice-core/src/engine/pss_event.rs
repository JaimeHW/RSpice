//! Event shooting: the periodic steady state of a switched analog circuit,
//! solved on the transient engine itself.
//!
//! The period map is a transient segment, not a second integrator. A live
//! image of the circuit is taken at a quiet boundary `t0` in the last
//! stabilization period; the shooting coordinates `z` are written into it;
//! the transient engine runs over a locked mesh for one period, cutting the
//! step where an ideal switch crosses its threshold and refining a
//! Verilog-A event as a run does; and the image at `t0 + T` is read back.
//! Hysteretic switches and runtime Verilog-A modules with events, retained
//! variables or integrators take part through the state their own image
//! carries, with no second state-map implementation for any of them.
//!
//! Native shooting keeps every circuit it admits. A circuit it refuses only
//! for a switch's hysteresis state or a runtime Verilog-A module's state
//! comes here; a running HDL host, XSPICE models and every other family keep
//! native shooting's refusal.
//!
//! - [`basis`]: `z`, the continuous coordinates, and `σ`, the discrete state
//!   held fixed by the inner iteration.
//! - [`boundary`]: the quiet boundary and the nominal mesh.
//! - [`newton`]: the period map and the inner Newton iteration on `z`.
//! - [`certify`]: the mesh, linearization and event-margin checks an orbit
//!   passes before it is published.
//! - [`pac`]: PAC, and PXF through it, on the orbit in the time domain.

mod basis;
mod boundary;
mod certify;
mod newton;
mod pac;

use crate::abort_signal::AbortSignal;
use crate::analysis::pss::{PeriodicWaveform, PssResult};
use crate::analysis::{FloquetOrbitKind, FloquetSpectrumEvidence, PssConfig};
use crate::engine::transient::state_capture::{AcceptedPointObserver, CapturedTransientState};
use crate::engine::{Engine, PssAnalysisResult, SimulationError, UnsupportedCapabilityError};
use crate::identity::AnalysisKind;
use crate::{CircuitData, Netlist, Value};
use basis::ShootingBasis;
use newton::Orbit;

const DISCRETE_STATE_NOT_PERIODIC: &str = "analysis.pss.discrete_state_not_periodic";
const NO_QUIET_BOUNDARY: &str = "analysis.pss.no_quiet_boundary";
const EVENT_GRAZING: &str = "analysis.pss.event_grazing";
const EVENT_ORDER_AMBIGUOUS: &str = "analysis.pss.event_order_ambiguous";
const EVENT_STRUCTURE_UNSTABLE: &str = "analysis.pss.event_structure_unstable";
const OPAQUE_MODEL_STATE: &str = "analysis.pss.opaque_model_state";
const LINEARIZATION_UNCERTIFIED: &str = "analysis.pss.linearization_uncertified";
const MESH_UNCERTIFIED: &str = "analysis.pss.mesh_uncertified";
const EVENT_AUTONOMOUS: &str = "analysis.pss.event_autonomous";
/// A periodic small-signal analysis asked of an orbit event shooting solved.
pub(in crate::engine) const EVENT_CARRIER: &str = "analysis.periodic.event_carrier";
/// Every coordinate name of an event orbit's retained state starts with
/// this, so a retained operating point names the engine that solved it.
pub(in crate::engine) const EVENT_BASIS_PREFIX: &str = "event:";

/// How many instants of the last stabilization period are offered as the
/// boundary.
const BOUNDARY_CANDIDATES: usize = 16;
/// How many times the mesh is bisected before an orbit that keeps moving
/// with it is refused.
const MESH_REFINEMENTS: usize = 3;
/// How many periods the outer iteration advances the discrete state before
/// it refuses an orbit whose discrete state does not come back.
const DISCRETE_ITERATIONS: usize = 8;
/// The Newton tolerances of every step of a traversal, relative and as a
/// fraction of the run's absolute tolerances. The period map must be a
/// smooth function of the coordinates for its Newton iteration and its
/// finite-difference columns: a step left at the transient's own tolerance
/// (1e-3 relative, 1 uV) moves by as much as a column's perturbation.
const TRAVERSAL_RELTOL: Value = 1.0e-6;
const TRAVERSAL_ABSTOL_FRACTION: Value = 1.0e-3;

/// A solved and certified event orbit.
#[derive(Debug)]
pub(in crate::engine) struct EventPss {
    pub(in crate::engine) analysis: PssAnalysisResult,
    /// The coordinate names, each led by [`EVENT_BASIS_PREFIX`].
    pub(in crate::engine) basis: Vec<String>,
    /// The converged coordinates at the boundary.
    pub(in crate::engine) state: Vec<Value>,
    pub(in crate::engine) certification: EventCertification,
}

/// What certified an event orbit.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::engine) struct EventCertification {
    /// The boundary's phase in the period.
    pub(in crate::engine) boundary_phase: Value,
    /// Periods the outer iteration advanced the discrete state.
    pub(in crate::engine) discrete_iterations: usize,
    /// Times the mesh was bisected.
    pub(in crate::engine) mesh_refinements: usize,
    /// The orbit's deviation from its bisection, in transient tolerances.
    pub(in crate::engine) mesh_deviation: Value,
    /// The worst relative miss of `Φ` along a seeded direction.
    pub(in crate::engine) linearization_miss: Value,
    /// The smallest gap between state-dependent events of different sources.
    pub(in crate::engine) min_event_gap: Option<Value>,
    /// The orbit's events: the quantity and its phase in the period.
    pub(in crate::engine) events: Vec<(String, Value)>,
}

/// The typed refusals event shooting gives.
struct Refusal;

impl Refusal {
    fn refusal(token: &'static str, detail: String) -> SimulationError {
        UnsupportedCapabilityError::new(token, detail)
            .for_analysis_kind(AnalysisKind::Pss)
            .into()
    }

    fn grazing_under_perturbation(coordinate: &str) -> SimulationError {
        Self::refusal(
            EVENT_GRAZING,
            format!(
                "PSS event shooting cannot linearize the period map along {coordinate}: every \
                 perturbation of it changes the orbit's events, so an event grazes its threshold"
            ),
        )
    }

    fn grazing(quantity: &str, time: Value, slope: Value) -> SimulationError {
        Self::refusal(
            EVENT_GRAZING,
            format!(
                "PSS event shooting refuses the orbit: {quantity} grazes its threshold at \
                 t={time:.6e} s (rate {slope:.3e}/s), where the period map is not \
                 differentiable"
            ),
        )
    }

    fn order_ambiguous(
        first: (&str, Value),
        second: (&str, Value),
        guard: Value,
    ) -> SimulationError {
        Self::refusal(
            EVENT_ORDER_AMBIGUOUS,
            format!(
                "PSS event shooting refuses the orbit: {} at t={:.6e} s and {} at t={:.6e} s are \
                 closer than the {guard:.3e} s guard, so their order is not determined",
                first.0, first.1, second.0, second.1
            ),
        )
    }

    fn unstable(named: &str, held: usize, changed: usize) -> SimulationError {
        Self::refusal(
            EVENT_STRUCTURE_UNSTABLE,
            format!(
                "PSS event shooting cannot keep the orbit's events: the Newton iteration moves \
                 {named} (the held orbit has {held} events, the corrected one {changed})"
            ),
        )
    }

    fn linearization(coordinate: &str, miss: Value) -> SimulationError {
        Self::refusal(
            LINEARIZATION_UNCERTIFIED,
            format!(
                "PSS event shooting cannot certify the period map's linearization: along a \
                 seeded direction it misses the map by {miss:.3e} (relative) at {coordinate}"
            ),
        )
    }

    fn mesh(worst: &str, deviation: Value, refinements: usize) -> SimulationError {
        Self::refusal(
            MESH_UNCERTIFIED,
            format!(
                "PSS event shooting cannot certify the orbit's mesh: after {refinements} \
                 bisections {worst} still moves by {deviation:.3e} times the transient \
                 tolerance"
            ),
        )
    }

    fn no_quiet_boundary(what: &str) -> SimulationError {
        Self::refusal(
            NO_QUIET_BOUNDARY,
            format!(
                "PSS event shooting finds no quiet boundary in the last stabilization period: \
                 {what} is busy at every candidate instant"
            ),
        )
    }

    fn not_periodic(quantity: &str, repeats: Option<usize>, fundamental: Value) -> SimulationError {
        let detail = match repeats {
            Some(k) if k > 1 => format!(
                "PSS event shooting refuses the orbit: the discrete state repeats every {k} \
                 periods ({quantity} differs after one); use fund={:.6e} Hz",
                fundamental / k as Value
            ),
            _ => format!(
                "PSS event shooting refuses the orbit: {quantity} does not come back after one \
                 period of the converged map"
            ),
        };
        Self::refusal(DISCRETE_STATE_NOT_PERIODIC, detail)
    }
}

/// Whether native shooting refuses this circuit only for state event
/// shooting carries: a hysteretic switch, or a runtime Verilog-A module's
/// events, retained variables or integrators.
pub(in crate::engine) fn takes_event_shooting(circuit: &CircuitData) -> bool {
    !circuit.vswitches.is_empty()
        || !circuit.iswitches.is_empty()
        || crate::engine::analysis_capability::native_shooting_refuses_runtime_veriloga(circuit)
}

/// The event-shooting preflight for a circuit [`takes_event_shooting`]
/// answered: native shooting's refusal for every family event shooting does
/// not take, and by name each piece of state no coordinate carries.
pub(in crate::engine) fn admission_refusal(
    circuit: &CircuitData,
    netlist: &Netlist,
    config: &PssConfig,
) -> Option<SimulationError> {
    if let Some(refusal) =
        crate::engine::analysis_capability::event_shooting_capability_refusal(circuit)
    {
        return Some(refusal.into());
    }
    use crate::engine::periodic_capability::{PeriodicDeviceFamily as F, pss_state_gaps};
    if let Some(gap) = pss_state_gaps(circuit)
        .into_iter()
        .find(|gap| !matches!(gap.family, F::VoltageSwitch | F::CurrentSwitch))
    {
        return Some(Refusal::refusal(
            OPAQUE_MODEL_STATE,
            format!(
                "PSS event shooting cannot carry the state of {}: {}",
                gap.family.label(),
                gap.detail
            ),
        ));
    }
    if netlist.options.transient_noise.is_some() {
        return Some(Refusal::refusal(
            OPAQUE_MODEL_STATE,
            "PSS event shooting cannot carry transient noise: its streams have no periodic state"
                .to_string(),
        ));
    }
    if config.is_autonomous() {
        return Some(Refusal::refusal(
            EVENT_AUTONOMOUS,
            "PSS event shooting solves driven orbits; an oscillator whose switch or Verilog-A \
             state native shooting cannot carry has no autonomous solve"
                .to_string(),
        ));
    }
    opaque_veriloga_refusal(circuit)
}

/// A runtime Verilog-A instance whose operator state is a history no
/// shooting coordinate carries.
#[cfg(feature = "veriloga")]
fn opaque_veriloga_refusal(circuit: &CircuitData) -> Option<SimulationError> {
    if let Some(refusal) =
        crate::engine::analysis_capability::event_shooting_operator_refusal(circuit)
    {
        return Some(refusal.into());
    }
    let states = circuit.runtime_veriloga_checkpoint_states().ok()?;
    for (checkpoint, device) in states.iter().zip(circuit.veriloga_devices().iter()) {
        let accepted = &checkpoint.accepted;
        let operator = if !accepted.delay_buffers.is_empty() {
            "absdelay"
        } else if !accepted.laplace_filters.is_empty() {
            "a laplace_* filter"
        } else if !accepted.zi_filters.is_empty() {
            "a zi_* filter"
        } else if !accepted.idtmod_origins.is_empty() {
            "idtmod"
        } else {
            continue;
        };
        return Some(
            UnsupportedCapabilityError::new(
                OPAQUE_MODEL_STATE,
                format!(
                    "PSS event shooting cannot carry the state of Verilog-A instance '{}' \
                     (module '{}'): its {operator} history is not a shooting coordinate",
                    device.name,
                    device.model_name()
                ),
            )
            .for_analysis_kind(AnalysisKind::Pss)
            .for_instance(device.name.to_string())
            .for_module(device.model_name().to_owned())
            .into(),
        );
    }
    None
}

#[cfg(not(feature = "veriloga"))]
fn opaque_veriloga_refusal(_circuit: &CircuitData) -> Option<SimulationError> {
    None
}

/// A stabilized circuit at its quiet boundary: everything the solve and a
/// small-signal analysis that reproduces a retained orbit share.
struct Prepared {
    /// The engine every traversal runs on, with tightened Newton tolerances.
    traversals: Engine,
    /// The run's own relative and absolute voltage tolerances.
    mesh_tolerance: (Value, Value),
    period: Value,
    max_step: Value,
    guard: Value,
    basis: ShootingBasis,
    /// The live image at the boundary.
    image: CapturedTransientState,
    /// The nominal mesh of the period from the boundary.
    mesh: Vec<Value>,
}

impl Prepared {
    fn orbit<'a>(
        &'a self,
        netlist: &'a Netlist,
        config: &'a PssConfig,
        abort: &'a dyn AbortSignal,
    ) -> Orbit<'a> {
        Orbit {
            engine: &self.traversals,
            netlist,
            basis: &self.basis,
            config,
            max_step: self.max_step,
            guard: self.guard,
            mesh_tolerance: self.mesh_tolerance,
            abort,
        }
    }
}

impl Engine {
    /// Solve a driven periodic steady state by event shooting.
    pub(in crate::engine) fn run_event_pss(
        &self,
        netlist: &Netlist,
        config: &PssConfig,
        dc_seed: Option<&crate::engine::PssDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<EventPss, SimulationError> {
        let prepared = self.prepare_event_orbit(netlist, config, dc_seed, abort)?;
        let orbit = prepared.orbit(netlist, config, abort);
        let (image, solved, mesh, discrete_iterations) = orbit.solve_discrete(
            prepared.image.clone(),
            prepared.mesh.clone(),
            prepared.period,
        )?;
        let certified = orbit.certify(&image, solved, mesh)?;
        certified.publish(
            &prepared.basis,
            &image,
            config,
            prepared.period,
            discrete_iterations,
        )
    }

    /// Stabilize the circuit, choose its quiet boundary and lay out the
    /// orbit's coordinates and nominal mesh there.
    fn prepare_event_orbit(
        &self,
        netlist: &Netlist,
        config: &PssConfig,
        dc_seed: Option<&crate::engine::PssDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<Prepared, SimulationError> {
        config
            .validate()
            .map_err(crate::engine::PssError::InvalidConfig)?;
        let resolved = self.resolved_for_netlist(netlist);
        let traversals = {
            let mut config = resolved.config.clone();
            let newton = &mut config.convergence_config;
            newton.voltage_reltol = newton.voltage_reltol.min(TRAVERSAL_RELTOL);
            newton.voltage_abstol = resolved.voltage_abstol() * TRAVERSAL_ABSTOL_FRACTION;
            newton.current_abstol = resolved.current_abstol() * TRAVERSAL_ABSTOL_FRACTION;
            Engine::new_with_resolved_config(config)
        };
        let period = config.period();
        let points = config.points_per_period.max(2);
        let max_step = period / points as Value;
        let tstab = config.effective_tstab().max(2.0 * period);
        let candidates: Vec<Value> = (0..BOUNDARY_CANDIDATES)
            .map(|k| tstab - period + (k as Value + 0.5) * period / BOUNDARY_CANDIDATES as Value)
            .collect();
        let history_start = tstab - (DISCRETE_ITERATIONS as Value + 2.0) * period;
        let recorded = std::cell::RefCell::new(Vec::new());
        let observer = |time: Value, circuit: &CircuitData, _solution: &[Value]| {
            if time >= history_start {
                recorded.borrow_mut().push(basis::probe(time, circuit)?);
            }
            Ok(())
        };
        let observer: &AcceptedPointObserver<'_> = &observer;
        // The run goes on one period past the last candidate, so the window
        // from any of them is accepted instants of its own.
        let (stabilization, images) = resolved.run_tran_observed(
            netlist,
            (tstab + period, max_step),
            dc_seed,
            &candidates,
            observer,
            abort,
        )?;
        let probes = recorded.into_inner();
        let last = images.last().ok_or_else(|| {
            SimulationError::Circuit("PSS stabilization captured no boundary image".to_string())
        })?;
        let provisional = ShootingBasis::of(last)?;
        let history = boundary::events(&provisional, &probes);
        let guard = 1.0e-6 * period;
        let candidate_probes: Vec<(Value, &basis::Probe)> = images
            .iter()
            .filter_map(|image| {
                probes
                    .iter()
                    .rev()
                    .find(|probe| probe.time == image.time)
                    .map(|probe| (image.time, probe))
            })
            .collect();
        let recent: Vec<_> = history
            .iter()
            .filter(|event| event.time >= tstab - 2.0 * period)
            .cloned()
            .collect();
        let choice = boundary::choose(&candidate_probes, &recent, period, guard).map_err(
            |busy| match busy {
                boundary::NoQuietBoundary::Ramping(instance) => Refusal::no_quiet_boundary(
                    &format!("a transition ramp of Verilog-A instance '{instance}'"),
                ),
                boundary::NoQuietBoundary::Busy(quantity) => {
                    Refusal::no_quiet_boundary(provisional.discrete_name(quantity))
                }
            },
        )?;
        let boundary_time = candidate_probes[choice.candidate].0;
        let image = images
            .iter()
            .find(|image| image.time == boundary_time)
            .cloned()
            .ok_or_else(|| SimulationError::Circuit("PSS lost its boundary image".to_string()))?;
        let basis = ShootingBasis::of(&image)?;
        // The stabilization's own history already shows a discrete state
        // that repeats only after several periods.
        let fingerprint_at = |time: Value| {
            probes
                .iter()
                .rev()
                .find(|probe| probe.time <= time + guard)
                .map(|probe| basis.fingerprint(probe))
        };
        if let Some(here) = fingerprint_at(boundary_time) {
            let earlier: Vec<_> = (1..=DISCRETE_ITERATIONS)
                .map_while(|k| {
                    let time = boundary_time - k as Value * period;
                    (time >= history_start)
                        .then(|| fingerprint_at(time))
                        .flatten()
                })
                .collect();
            if let Some(first) = earlier.first()
                && first != &here
                && let Some(repeat) = earlier.iter().position(|fingerprint| fingerprint == &here)
            {
                let quantity = here
                    .first_difference(first)
                    .map_or("the discrete state", |index| basis.discrete_name(index));
                return Err(Refusal::not_periodic(
                    quantity,
                    Some(repeat + 1),
                    1.0 / period,
                ));
            }
        }
        let excluded: Vec<Value> = history
            .iter()
            .filter(|event| event.state_dependent)
            .map(|event| event.time)
            .collect();
        let mesh = boundary::nominal_mesh(&stabilization.time, boundary_time, period, &excluded);
        Ok(Prepared {
            mesh_tolerance: (resolved.voltage_reltol(), resolved.voltage_abstol()),
            traversals,
            period,
            max_step,
            guard,
            basis,
            image,
            mesh,
        })
    }
}

/// A certified orbit, ready to publish.
struct Certified {
    solved: newton::Solved,
    refinements: usize,
    deviation: Value,
    linearization_miss: Value,
    min_event_gap: Option<Value>,
    multipliers: Vec<num_complex::Complex64>,
    evidence: FloquetSpectrumEvidence,
}

impl Orbit<'_> {
    /// The outer iteration: solve with the discrete state held, then ask
    /// whether it came back; if not, continue from the state one period on.
    fn solve_discrete(
        &self,
        image: CapturedTransientState,
        mesh: Vec<Value>,
        period: Value,
    ) -> Result<(CapturedTransientState, newton::Solved, Vec<Value>, usize), SimulationError> {
        let mut image = image;
        let mut mesh = mesh;
        let mut reuse = None;
        let mut seen = Vec::new();
        for pass in 0..=DISCRETE_ITERATIONS {
            let solved = self.solve(&image, &mesh, reuse.take())?;
            let start = self.discrete_state(&image)?;
            let end = self.discrete_state(&solved.traversal.end)?;
            if start == end {
                return Ok((image, solved, mesh, pass));
            }
            if seen.is_empty() {
                seen.push(start.clone());
            }
            seen.push(end.clone());
            let quantity = start
                .first_difference(&end)
                .map_or("the discrete state", |index| {
                    self.basis.discrete_name(index)
                });
            if let Some(repeat) = seen
                .iter()
                .rev()
                .skip(1)
                .position(|fingerprint| fingerprint == &end)
                && repeat > 0
            {
                return Err(Refusal::not_periodic(
                    quantity,
                    Some(repeat + 1),
                    1.0 / period,
                ));
            }
            if pass == DISCRETE_ITERATIONS {
                return Err(Refusal::not_periodic(quantity, None, 1.0 / period));
            }
            reuse = Some(solved.jacobian.clone());
            mesh = boundary::shifted(&mesh, period);
            image = solved.traversal.end;
        }
        unreachable!("the outer iteration returns from its last pass")
    }

    /// Mesh bisection, linearization and event margins.
    fn certify(
        &self,
        image: &CapturedTransientState,
        solved: newton::Solved,
        mesh: Vec<Value>,
    ) -> Result<Certified, SimulationError> {
        let t0 = image.time;
        let mut iterations = solved.iterations;
        let mut coarse = solved;
        let mut coarse_mesh = mesh;
        let mut refinements = 0;
        let (fine, fine_mesh, deviation) = loop {
            refinements += 1;
            let fine_mesh = boundary::bisect(&coarse_mesh, t0);
            let refined = self.basis.write(image, &coarse.z)?;
            let fine = self.solve(&refined, &fine_mesh, Some(coarse.jacobian.clone()))?;
            iterations += fine.iterations;
            if newton::signature(&fine.traversal.events)
                != newton::signature(&coarse.traversal.events)
            {
                // An orbit that grazes a threshold gains or loses its events
                // with the mesh: that is the refusal's name.
                self.certify_margins(&coarse.traversal)?;
                self.certify_margins(&fine.traversal)?;
                return Err(Refusal::unstable(
                    "the events of the refined mesh",
                    coarse.traversal.events.len(),
                    fine.traversal.events.len(),
                ));
            }
            let deviation = self.mesh_deviation(&coarse.traversal, &fine.traversal);
            if deviation.deviation <= 1.0 {
                break (fine, fine_mesh, deviation.deviation);
            }
            if refinements >= MESH_REFINEMENTS {
                return Err(Refusal::mesh(
                    &deviation.worst,
                    deviation.deviation,
                    refinements,
                ));
            }
            coarse = fine;
            coarse_mesh = fine_mesh;
        };
        let start = self.basis.write(image, &fine.z)?;
        let phi = fine.jacobian.clone();
        let linearization_miss =
            self.certify_linearization(&start, &fine.z, &fine.traversal, &fine_mesh, &phi)?;
        let (min_event_gap, _) = self.certify_margins(&fine.traversal)?;
        let (multipliers, evidence) = if phi.is_empty() {
            (Vec::new(), FloquetSpectrumEvidence::NoDynamicModes)
        } else {
            crate::analysis::pss::ShootingNewtonSolver::new(self.config.tolerance)
                .compute_floquet_spectrum_with_abort(&phi, self.abort)
                .map_err(|error| match error {
                    crate::analysis::pss::FloquetSpectrumError::Aborted => SimulationError::Aborted,
                    crate::analysis::pss::FloquetSpectrumError::Numerical(message) => {
                        SimulationError::Circuit(format!(
                            "PSS Floquet spectrum qualification failed: {message}"
                        ))
                    }
                })?
        };
        let mut fine = fine;
        fine.iterations = iterations;
        Ok(Certified {
            solved: fine,
            refinements,
            deviation,
            linearization_miss,
            min_event_gap,
            multipliers,
            evidence,
        })
    }
}

impl Certified {
    /// The published orbit: its waveform on `[0, T]` in absolute time modulo
    /// the period, and its certification.
    fn publish(
        self,
        basis: &ShootingBasis,
        image: &CapturedTransientState,
        config: &PssConfig,
        period: Value,
        discrete_iterations: usize,
    ) -> Result<EventPss, SimulationError> {
        let solved = self.solved;
        let waveform = &solved.traversal.result;
        let t0 = image.time;
        let end = t0 + period;
        let phase_zero = boundary::phase_zero_before(end, period);
        let phase_zero = if phase_zero > t0 && phase_zero <= end {
            phase_zero
        } else {
            t0
        };
        // Samples from the period boundary to the window's end come first,
        // then those from the window's start back round to the boundary.
        let mut order: Vec<(Value, usize)> = Vec::with_capacity(waveform.time.len() + 1);
        for (index, &time) in waveform.time.iter().enumerate() {
            if time >= phase_zero {
                order.push((time - phase_zero, index));
            }
        }
        for (index, &time) in waveform.time.iter().enumerate() {
            if time > t0 && time <= phase_zero {
                order.push((time - phase_zero + period, index));
            }
        }
        order.sort_by(|a, b| a.0.total_cmp(&b.0));
        order.dedup_by(|later, earlier| later.0 - earlier.0 <= period * 1.0e-12);
        if let Some(first) = order.first_mut() {
            first.0 = 0.0;
        }
        match order.last_mut() {
            Some(last) if (last.0 - period).abs() <= period * 1.0e-9 => last.0 = period,
            _ => {
                if let Some(&(_, index)) = order.first() {
                    order.push((period, index));
                }
            }
        }
        let mut result = PssResult::new(period, waveform.node_names.len(), order.len());
        result.time = order.iter().map(|(time, _)| *time).collect();
        result.iterations = solved.iterations;
        result.residual_norm = solved.residual * config.tolerance;
        result.node_names = waveform.node_names.clone();
        result.branch_names = waveform.branch_names.clone();
        let sampled = |trace: &Vec<Value>| {
            PeriodicWaveform::from_values(order.iter().map(|&(_, index)| trace[index]).collect())
        };
        result.waveforms = waveform.voltages.iter().map(sampled).collect();
        result.branch_waveforms = waveform.branch_currents.iter().map(sampled).collect();
        result.set_floquet_spectrum(
            self.multipliers.clone(),
            self.evidence,
            FloquetOrbitKind::Driven,
        );
        let is_stable = result.is_stable();
        let events = solved
            .traversal
            .events
            .iter()
            .map(|event| {
                (
                    basis.discrete_name(event.quantity).to_owned(),
                    event.time.rem_euclid(period),
                )
            })
            .collect();
        let certification = EventCertification {
            boundary_phase: t0.rem_euclid(period),
            discrete_iterations,
            mesh_refinements: self.refinements,
            mesh_deviation: self.deviation,
            linearization_miss: self.linearization_miss,
            min_event_gap: self.min_event_gap,
            events,
        };
        let analysis = PssAnalysisResult {
            result,
            iterations: solved.iterations,
            final_residual: solved.residual * config.tolerance,
            period,
            monodromy: solved.jacobian,
            floquet_multipliers: self.multipliers,
            is_stable,
        };
        Ok(EventPss {
            analysis,
            basis: basis.names().to_vec(),
            state: solved.z,
            certification,
        })
    }
}

#[cfg(test)]
mod tests;
