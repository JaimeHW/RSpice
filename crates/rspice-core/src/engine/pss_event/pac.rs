//! Periodic small-signal analysis around an event orbit, in the time domain.
//!
//! An orbit event shooting solved has no harmonic-balance projection to
//! linearize: its switches change inside the period. PAC (and PXF, which
//! reads its conversion matrix) runs on the orbit itself instead.
//!
//! - **The orbit.** The retained coordinates are written into the image the
//!   solve's own stabilization and boundary reproduce, and one period is
//!   traversed; it must close within the transient tolerances the orbit was
//!   certified against.
//! - **The linearized period.** At every accepted point of that period the
//!   circuit's small-signal operator `Y(jω) = G + jωC` is read at `ω = 1`.
//!   One step of the period is
//!   `(C_k/h + θG_k)·x_k = (C_{k−1}/h − (1−θ)G_{k−1})·x_{k−1} + b·(θu_k + (1−θ)u_{k−1})`,
//!   with `θ = ½` (trapezoidal), and `θ = 1` on an algebraic row and on the
//!   step a switch changes in, as the transient restarts its integration
//!   there.
//! - **The periodic response.** For `u(t) = e^{jωt}` the response satisfies
//!   `x(t₀ + T) = e^{jωT}·x(t₀)`. A sweep from rest gives `r(ω)`, the
//!   recurrence's monodromy `M` gives `x(t₀) = (e^{jωT}I − M)⁻¹·r(ω)`, and a
//!   second sweep gives `x(t_k)`. Sideband `m` is
//!   `Y_m = (1/T)∫x(t)·e^{−j(ω+mω₀)t}dt` over absolute time, by the
//!   trapezoidal rule on the mesh; an input at sideband `m'` is the same
//!   solve at `ω + m'ω₀`.
//! - **Scope.** A switch whose threshold crossing moves with the small signal
//!   would add an impulse at its change; that term is not formed, and such
//!   an orbit is refused by name, as is one with runtime Verilog-A or a
//!   small-signal stamp that is not linear in frequency.
//! - **Certification.** The sweep's endpoints and its largest response are
//!   recomputed on the bisected mesh and must agree within the analysis's
//!   relative tolerance and the run's voltage tolerance.

use super::basis::CoordinateUnit;
use super::newton::Orbit;
use super::{DISCRETE_ITERATIONS, EVENT_CARRIER, MESH_REFINEMENTS, boundary};
use crate::abort_signal::AbortSignal;
use crate::analysis::pac::{PacConfig, PacResult};
use crate::engine::ac::SmallSignalAnalysisKind;
use crate::engine::pss::PssDenseLu;
use crate::engine::transient::state_capture::{AcceptedPointObserver, CapturedTransientState};
use crate::engine::{
    Engine, PacAnalysisResult, PssOperatingPoint, SimulationError, UnsupportedCapabilityError,
};
use crate::identity::AnalysisKind;
use crate::solver::{ComplexMatrix, StaticMatrix};
use crate::{CircuitData, Complex64, Netlist, Value};

const SMALL_SIGNAL_MESH_UNCERTIFIED: &str = "analysis.periodic.small_signal_mesh_uncertified";

fn refusal(token: &'static str, detail: String) -> SimulationError {
    UnsupportedCapabilityError::new(token, detail)
        .for_analysis_kind(AnalysisKind::Pac)
        .into()
}

fn carrier_refusal(detail: String) -> SimulationError {
    refusal(EVENT_CARRIER, detail)
}

/// The small-signal operator at one accepted point of the period.
struct Operator {
    time: Value,
    /// `G`, row-major.
    g: Vec<Value>,
    /// `C`, row-major.
    c: Vec<Value>,
    /// Every voltage and then current switch's state.
    switches: Vec<u64>,
}

/// One step of the linearized period.
struct Step {
    a: PssDenseLu,
    /// `B`, row-major.
    b: Vec<Value>,
    /// Each row's `θ`: the weight of the input at the step's end.
    theta: Vec<Value>,
    /// The switches that change in this step.
    changes: Vec<usize>,
}

/// The linearized period of an event orbit.
struct LinearPeriod {
    n: usize,
    period: Value,
    /// Every accepted point's absolute time from the start of the period
    /// that holds the boundary.
    phases: Vec<Value>,
    steps: Vec<Step>,
    /// `M`, row-major.
    monodromy: Vec<Value>,
    excitation: Vec<Value>,
}

/// One input frequency's periodic response.
struct Response {
    /// `x(t_k)` at every accepted point.
    trajectory: Vec<Vec<Complex64>>,
}

impl Engine {
    /// PAC around an orbit event shooting solved.
    pub(in crate::engine) fn run_event_pac(
        &self,
        netlist: &Netlist,
        mut config: PacConfig,
        operating_point: &PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        operating_point.authenticate_event_orbit_for_reuse(
            netlist,
            &self.config,
            operating_point.config(),
        )?;
        let input_name = config
            .input_source
            .clone()
            .ok_or_else(|| SimulationError::Circuit("PAC requires an input source".to_string()))?;
        config.fundamental_freq = operating_point.analysis().result.frequency;
        config
            .validate()
            .map_err(|e| SimulationError::Circuit(format!("Invalid PAC config: {e}")))?;
        let frequencies = config
            .try_frequency_points_with_abort(abort)
            .map_err(|error| error.into_simulation_error("Invalid PAC frequency sweep"))?;
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "PAC frequency sweep produced no points".to_string(),
            ));
        }
        let pss_config = operating_point.config();
        let prepared = self.prepare_event_orbit(netlist, pss_config, None, abort)?;
        if prepared.basis.names() != operating_point.shooting_state_basis() {
            return Err(carrier_refusal(format!(
                "PAC cannot reproduce the event orbit: its coordinates at the reproduced \
                 boundary are {:?}, the retained orbit's {:?}",
                prepared.basis.names(),
                operating_point.shooting_state_basis()
            )));
        }
        let circuit = &prepared.image.circuit;
        #[cfg(feature = "veriloga")]
        if !circuit.veriloga_devices().is_empty() {
            return Err(carrier_refusal(
                "PAC on an event orbit linearizes switches and the native devices between \
                 their changes; a runtime Verilog-A module's operator around its events is \
                 not formed"
                    .to_string(),
            ));
        }
        let excitation = input_excitation(circuit, &input_name)?;
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        let num_nodes = circuit.num_nodes();
        let output = output_selection(&config, netlist, &node_names)?;
        let structure = self.build_matrix(circuit)?;
        let orbit = prepared.orbit(netlist, pss_config, abort);
        let z = operating_point.shooting_state();
        let (start, mesh) = reproduce_boundary(&orbit, &prepared.image, z, &prepared.mesh)?;
        let reltol = config.reltol;
        let abstol = prepared.mesh_tolerance.1;
        let probes = probe_frequencies(&frequencies);
        let mut coarse_mesh = boundary::bisect(&mesh, start.time);
        let mut coarse = orbit.linear_period(&start, z, &coarse_mesh, &structure, &excitation)?;
        let mut refinements = 1;
        let fine = loop {
            refinements += 1;
            let fine_mesh = boundary::bisect(&coarse_mesh, start.time);
            let fine = orbit.linear_period(&start, z, &fine_mesh, &structure, &excitation)?;
            let worst = probes
                .iter()
                .map(|&frequency| {
                    let omega = std::f64::consts::TAU * frequency;
                    let a = coarse.sidebands(&coarse.respond(omega)?, omega, &config);
                    let b = fine.sidebands(&fine.respond(omega)?, omega, &config);
                    Ok(deviation(&a, &b, num_nodes, reltol, abstol))
                })
                .collect::<Result<Vec<_>, SimulationError>>()?
                .into_iter()
                .fold((0.0, 0), |worst, (value, node)| {
                    if value > worst.0 {
                        (value, node)
                    } else {
                        worst
                    }
                });
            if worst.0 <= 1.0 {
                break fine;
            }
            if refinements > MESH_REFINEMENTS {
                return Err(refusal(
                    SMALL_SIGNAL_MESH_UNCERTIFIED,
                    format!(
                        "PAC on the event orbit cannot certify its mesh: after {refinements} \
                         bisections the response at V({}) still moves by {:.3e} times its \
                         tolerance",
                        node_names.get(worst.1).map_or("?", String::as_str),
                        worst.0
                    ),
                ));
            }
            coarse = fine;
            coarse_mesh = fine_mesh;
        };
        fine.refuse_moving_crossings(circuit, &frequencies)?;
        let mut result = if output.is_some() {
            PacResult::new(
                config.fundamental_freq,
                frequencies.clone(),
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names.clone(),
            )
        } else {
            PacResult::new_without_conversion_matrix(
                config.fundamental_freq,
                frequencies.clone(),
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names.clone(),
            )
        }
        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        result.set_input_source(&input_name);
        if let Some(name) = &config.output_node {
            result.set_output_node(name);
        }
        result.pac_magnitude = config.pac_magnitude;
        result.include_dc = config.include_dc;
        let fundamental = std::f64::consts::TAU * config.fundamental_freq;
        for (index, &frequency) in frequencies.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let omega = std::f64::consts::TAU * frequency;
            let spectra = fine.sidebands(&fine.respond(omega)?, omega, &config);
            for (k, values) in (config.sideband_min..=config.sideband_max).zip(&spectra) {
                let data = result.get_sideband_data_mut(index, k).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "PAC result is missing frequency {index}, sideband {k}"
                    ))
                })?;
                for (node, value) in values.iter().take(num_nodes).enumerate() {
                    data.set_voltage(node, *value)
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                }
                for (branch, value) in values
                    .iter()
                    .skip(num_nodes)
                    .take(branch_names.len())
                    .enumerate()
                {
                    data.set_current(branch, *value)
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                }
            }
            let Some((out, reference)) = output else {
                continue;
            };
            let read = |values: &[Complex64]| {
                values[out] - reference.map_or(Complex64::new(0.0, 0.0), |r| values[r])
            };
            for input in config.sideband_min..=config.sideband_max {
                let shifted = omega + Value::from(input) * fundamental;
                let response = if input == 0 {
                    None
                } else {
                    Some(fine.respond(shifted)?)
                };
                for output_sideband in config.sideband_min..=config.sideband_max {
                    let value = if input == 0 {
                        read(&spectra[(output_sideband - config.sideband_min) as usize])
                    } else {
                        let response = response.as_ref().expect("solved above");
                        read(&fine.sideband(response, shifted, output_sideband - input, &config))
                    };
                    result
                        .conversion_matrix
                        .set(index, output_sideband, input, value)
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                }
            }
        }
        result.set_convergence_info(
            operating_point.analysis().iterations.max(1),
            operating_point.analysis().final_residual,
        );
        Ok(PacAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            converged: true,
        })
    }
}

/// The unit small-signal excitation of the named independent source: one
/// volt on a voltage source's branch equation, one ampere from a current
/// source's positive terminal to its negative.
fn input_excitation(circuit: &CircuitData, name: &str) -> Result<Vec<Value>, SimulationError> {
    let name = name.trim();
    let mut excitation = vec![0.0; circuit.matrix_size()];
    if let Some(index) = circuit
        .voltage_sources
        .names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
    {
        let row = circuit
            .get_branch_matrix_index(circuit.voltage_sources.branch_indices[index])
            .checked_sub(1)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PAC input voltage source '{name}' has no equation row"
                ))
            })?;
        excitation[row] = 1.0;
        return Ok(excitation);
    }
    if let Some(index) = circuit
        .current_sources
        .names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
    {
        let (positive, negative) = (
            circuit.current_sources.node_pos[index],
            circuit.current_sources.node_neg[index],
        );
        if positive > 0 {
            excitation[positive - 1] -= 1.0;
        }
        if negative > 0 {
            excitation[negative - 1] += 1.0;
        }
        return Ok(excitation);
    }
    Err(SimulationError::Circuit(format!(
        "PAC input source '{name}' not found among independent sources"
    )))
}

/// The conversion matrix's output node and reference, as unknown indices.
fn output_selection(
    config: &PacConfig,
    netlist: &Netlist,
    node_names: &[String],
) -> Result<Option<(usize, Option<usize>)>, SimulationError> {
    let Some(name) = config.output_node.as_deref() else {
        return Ok(None);
    };
    let find = |name: &str| {
        node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PAC output node '{name}' not found in circuit nodes"
                ))
            })
    };
    if netlist.ground_policy().is_ground(name) {
        return Err(SimulationError::Circuit(
            "PAC output node cannot be ground".to_owned(),
        ));
    }
    let output = find(name)?;
    let reference = match config.output_ref.as_deref().map(str::trim) {
        Some(reference)
            if !reference.is_empty() && !netlist.ground_policy().is_ground(reference) =>
        {
            Some(find(reference)?)
        }
        _ => None,
    };
    if reference == Some(output) {
        return Err(SimulationError::Circuit(
            "PAC output node and reference node must be different".to_owned(),
        ));
    }
    Ok(Some((output, reference)))
}

/// The sweep's endpoints and its geometric middle, where the bisected mesh
/// is compared.
fn probe_frequencies(frequencies: &[Value]) -> Vec<Value> {
    let mut probes = vec![frequencies[0]];
    if frequencies.len() > 2 {
        probes.push(frequencies[frequencies.len() / 2]);
    }
    if frequencies.len() > 1 {
        probes.push(frequencies[frequencies.len() - 1]);
    }
    probes
}

/// The largest disagreement of two spectra's node voltages, in units of
/// `max(reltol·|y|, abstol)`, and the node it is at.
fn deviation(
    coarse: &[Vec<Complex64>],
    fine: &[Vec<Complex64>],
    num_nodes: usize,
    reltol: Value,
    abstol: Value,
) -> (Value, usize) {
    let mut worst = (0.0, 0);
    for (a, b) in coarse.iter().zip(fine) {
        for (node, (a, b)) in a.iter().zip(b).take(num_nodes).enumerate() {
            let tolerance = (reltol * b.norm()).max(abstol);
            let value = (a - b).norm() / tolerance;
            if value > worst.0 {
                worst = (value, node);
            }
        }
    }
    worst
}

/// The boundary image with the retained coordinates written in, and its
/// mesh, after checking that one period from it closes; the discrete state is
/// carried a period on while it does not come back.
fn reproduce_boundary(
    orbit: &Orbit<'_>,
    image: &CapturedTransientState,
    z: &[Value],
    mesh: &[Value],
) -> Result<(CapturedTransientState, Vec<Value>), SimulationError> {
    let mut image = image.clone();
    let mut mesh = mesh.to_vec();
    let period = orbit.config.period();
    let (reltol, abstol) = orbit.mesh_tolerance;
    for _ in 0..=DISCRETE_ITERATIONS {
        let traversal = orbit.traverse(&image, z, &mesh)?;
        if orbit.discrete_state(&image)? == orbit.discrete_state(&traversal.end)? {
            let mut worst = (0.0, 0);
            for (index, (start, end)) in z.iter().zip(&traversal.z_end).enumerate() {
                if orbit.basis.unit(index) != CoordinateUnit::Volt {
                    continue;
                }
                let tolerance = reltol * start.abs().max(end.abs()) + abstol;
                let value = (end - start).abs() / tolerance;
                if value > worst.0 {
                    worst = (value, index);
                }
            }
            if worst.0 > 1.0 {
                return Err(carrier_refusal(format!(
                    "PAC cannot reproduce the event orbit: from its retained coordinates \
                     {} misses its start after one period by {:.3e} times the transient \
                     tolerance",
                    orbit.basis.name(worst.1),
                    worst.0
                )));
            }
            return Ok((orbit.basis.write(&image, z)?, mesh));
        }
        image = traversal.end;
        mesh = boundary::shifted(&mesh, period);
    }
    Err(carrier_refusal(
        "PAC cannot reproduce the event orbit: its discrete state does not come back at the \
         reproduced boundary"
            .to_string(),
    ))
}

/// `Y(jω)` at `ω = 1` around `solution`, split into `G` and `C`.
fn operator(
    time: Value,
    circuit: &CircuitData,
    solution: &[Value],
    structure: &StaticMatrix,
    matrix: &mut ComplexMatrix,
    omega: Value,
) -> Result<Operator, SimulationError> {
    let mut circuit = circuit.clone();
    circuit.link_indices(structure);
    Engine::prepare_small_signal_state(&mut circuit, solution)?;
    circuit
        .prepare_behavioral_small_signal_at_frequency(solution, omega / std::f64::consts::TAU)
        .map_err(SimulationError::Circuit)?;
    Engine::try_fill_small_signal_matrix_with_vbic_delay_mode(
        &circuit,
        matrix,
        solution,
        omega,
        SmallSignalAnalysisKind::Ac,
        true,
        true,
    )?;
    let n = circuit.matrix_size();
    let mut g = vec![0.0; n * n];
    let mut c = vec![0.0; n * n];
    matrix.for_each_stored(|row, col, value| {
        g[row * n + col] += value.re;
        c[row * n + col] += value.im;
    });
    if omega != 1.0 {
        c.iter_mut().for_each(|entry| *entry /= omega);
    }
    let switches = circuit
        .vswitches
        .iter()
        .map(|switch| switch.state() as u64)
        .chain(circuit.iswitches.iter().map(|switch| switch.state() as u64))
        .collect();
    Ok(Operator {
        time,
        g,
        c,
        switches,
    })
}

impl Orbit<'_> {
    /// One period from `image` with `z` written over `mesh`, linearized at
    /// every accepted point.
    fn linear_period(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        mesh: &[Value],
        structure: &StaticMatrix,
        excitation: &[Value],
    ) -> Result<LinearPeriod, SimulationError> {
        let start = self.basis.write(image, z)?;
        let mut matrix = ComplexMatrix::from_real_structure(structure);
        let first = operator(
            start.time,
            &start.circuit,
            &start.solution,
            structure,
            &mut matrix,
            1.0,
        )?;
        // A stamp that is not `G + jωC` cannot be split at one frequency.
        let doubled = operator(
            start.time,
            &start.circuit,
            &start.solution,
            structure,
            &mut matrix,
            2.0,
        )?;
        let scale = first
            .g
            .iter()
            .chain(&first.c)
            .fold(0.0, |m: Value, v| m.max(v.abs()));
        if first
            .g
            .iter()
            .zip(&doubled.g)
            .chain(first.c.iter().zip(&doubled.c))
            .any(|(a, b)| (a - b).abs() > 1.0e-9 * a.abs().max(b.abs()).max(1.0e-12 * scale))
        {
            return Err(carrier_refusal(
                "PAC on an event orbit splits the small-signal operator into G + jωC, and a \
                 device in this circuit stamps a response that is not linear in frequency"
                    .to_string(),
            ));
        }
        let operators = std::cell::RefCell::new(vec![first]);
        let observer = |time: Value, circuit: &CircuitData, solution: &[Value]| {
            let mut matrix = ComplexMatrix::from_real_structure(structure);
            let point = operator(time, circuit, solution, structure, &mut matrix, 1.0)?;
            let mut operators = operators.borrow_mut();
            // The segment offers its start, and an instant it accepts twice
            // keeps its later state, which the next step starts from.
            match operators.last_mut() {
                Some(last) if time <= last.time => *last = point,
                _ => operators.push(point),
            }
            Ok(())
        };
        let observer: &AcceptedPointObserver<'_> = &observer;
        self.engine.run_tran_segment_from_image(
            self.netlist,
            &start,
            (mesh, None),
            self.max_step,
            Some(observer),
            self.abort,
        )?;
        let operators = operators.into_inner();
        let period = self.config.period();
        let t0 = start.time;
        let end = operators.last().map_or(t0, |point| point.time);
        if (end - t0 - period).abs() > 1.0e-9 * period {
            return Err(SimulationError::Circuit(format!(
                "PAC's linearized period ends at {end:.9e} s, not one period after its \
                 boundary at {t0:.9e} s"
            )));
        }
        LinearPeriod::new(&operators, excitation.to_vec(), period, t0)
    }
}

impl LinearPeriod {
    fn new(
        operators: &[Operator],
        excitation: Vec<Value>,
        period: Value,
        t0: Value,
    ) -> Result<Self, SimulationError> {
        let n = excitation.len();
        let origin = (t0 / period).floor() * period;
        let phases = operators.iter().map(|point| point.time - origin).collect();
        let mut steps = Vec::with_capacity(operators.len().saturating_sub(1));
        for pair in operators.windows(2) {
            let (before, after) = (&pair[0], &pair[1]);
            let h = after.time - before.time;
            if h <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "PAC's linearized period has a step of {h:e} s at {:.9e} s",
                    after.time
                )));
            }
            let changes: Vec<usize> = before
                .switches
                .iter()
                .zip(&after.switches)
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .map(|(index, _)| index)
                .collect();
            let theta: Vec<Value> = (0..n)
                .map(|row| {
                    let row_range = row * n..(row + 1) * n;
                    let algebraic = before.c[row_range.clone()].iter().all(|v| *v == 0.0)
                        && after.c[row_range].iter().all(|v| *v == 0.0);
                    if algebraic || !changes.is_empty() {
                        1.0
                    } else {
                        0.5
                    }
                })
                .collect();
            let a: Vec<Vec<Value>> = (0..n)
                .map(|row| {
                    (row * n..(row + 1) * n)
                        .map(|index| after.c[index] / h + theta[row] * after.g[index])
                        .collect()
                })
                .collect();
            let b: Vec<Value> = (0..n * n)
                .map(|index| before.c[index] / h - (1.0 - theta[index / n]) * before.g[index])
                .collect();
            let a = PssDenseLu::factor(&a).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PAC's linearized step at {:.9e} s is singular",
                    after.time
                ))
            })?;
            steps.push(Step {
                a,
                b,
                theta,
                changes,
            });
        }
        let mut period_map = Self {
            n,
            period,
            phases,
            steps,
            monodromy: vec![0.0; n * n],
            excitation,
        };
        let mut monodromy = vec![0.0; n * n];
        for col in 0..n {
            let mut x = vec![0.0; n];
            x[col] = 1.0;
            for step in &period_map.steps {
                x = step.a.solve(&multiply(&step.b, &x, n));
            }
            for (row, value) in x.into_iter().enumerate() {
                monodromy[row * n + col] = value;
            }
        }
        period_map.monodromy = monodromy;
        Ok(period_map)
    }

    /// One sweep of the recurrence from `x0` under `u(t) = e^{jωt}`.
    fn sweep(&self, x0: Vec<Complex64>, omega: Value, keep: bool) -> Vec<Vec<Complex64>> {
        let n = self.n;
        let input = |phase: Value| Complex64::from_polar(1.0, omega * phase);
        let mut trajectory = Vec::with_capacity(if keep { self.phases.len() } else { 1 });
        let mut x = x0;
        for (index, step) in self.steps.iter().enumerate() {
            let (u0, u1) = (input(self.phases[index]), input(self.phases[index + 1]));
            let mut re = vec![0.0; n];
            let mut im = vec![0.0; n];
            for row in 0..n {
                let mut sum: Complex64 = x
                    .iter()
                    .zip(&step.b[row * n..(row + 1) * n])
                    .map(|(value, entry)| value * entry)
                    .sum();
                let theta = step.theta[row];
                sum += self.excitation[row] * (u1 * theta + u0 * (1.0 - theta));
                re[row] = sum.re;
                im[row] = sum.im;
            }
            let (re, im) = (step.a.solve(&re), step.a.solve(&im));
            let next: Vec<Complex64> = re
                .into_iter()
                .zip(im)
                .map(|(re, im)| Complex64::new(re, im))
                .collect();
            if keep {
                trajectory.push(std::mem::replace(&mut x, next));
            } else {
                x = next;
            }
        }
        trajectory.push(x);
        trajectory
    }

    /// The periodic response to `u(t) = e^{jωt}`.
    fn respond(&self, omega: Value) -> Result<Response, SimulationError> {
        let n = self.n;
        let rest = vec![Complex64::new(0.0, 0.0); n];
        let r = self
            .sweep(rest, omega, false)
            .pop()
            .expect("a sweep ends somewhere");
        let rotation = Complex64::from_polar(1.0, omega * self.period);
        let mut system: Vec<Vec<Complex64>> = (0..n)
            .map(|row| {
                (0..n)
                    .map(|col| {
                        let identity = if row == col {
                            rotation
                        } else {
                            Complex64::new(0.0, 0.0)
                        };
                        identity - self.monodromy[row * n + col]
                    })
                    .collect()
            })
            .collect();
        let x0 = solve_complex(&mut system, r).ok_or_else(|| {
            carrier_refusal(format!(
                "PAC on the event orbit is singular at {:.6e} Hz: e^(jωT) is a Floquet \
                 multiplier of the orbit",
                omega / std::f64::consts::TAU
            ))
        })?;
        Ok(Response {
            trajectory: self.sweep(x0, omega, true),
        })
    }

    /// `Y_m` of every unknown for `m` in the analysis's sideband range.
    fn sidebands(
        &self,
        response: &Response,
        omega: Value,
        config: &PacConfig,
    ) -> Vec<Vec<Complex64>> {
        (config.sideband_min..=config.sideband_max)
            .map(|m| self.sideband(response, omega, m, config))
            .collect()
    }

    /// `Y_m = (1/T)∫x(t)·e^{−j(ω+mω₀)t}dt`, trapezoidal on the mesh.
    fn sideband(
        &self,
        response: &Response,
        omega: Value,
        m: i32,
        config: &PacConfig,
    ) -> Vec<Complex64> {
        let fundamental = std::f64::consts::TAU * config.fundamental_freq;
        let frequency = omega + Value::from(m) * fundamental;
        let mut sum = vec![Complex64::new(0.0, 0.0); self.n];
        let weight = |phase: Value| Complex64::from_polar(1.0, -frequency * phase);
        for index in 1..self.phases.len() {
            let (t0, t1) = (self.phases[index - 1], self.phases[index]);
            let half = 0.5 * (t1 - t0);
            let (w0, w1) = (weight(t0) * half, weight(t1) * half);
            let (x0, x1) = (&response.trajectory[index - 1], &response.trajectory[index]);
            for (entry, (a, b)) in sum.iter_mut().zip(x0.iter().zip(x1)) {
                *entry += a * w0 + b * w1;
            }
        }
        sum.iter_mut().for_each(|entry| *entry /= self.period);
        sum
    }

    /// Refuse an orbit with a switch whose control carries small signal at
    /// its change: its crossing time moves, and the impulse that adds is not
    /// formed.
    fn refuse_moving_crossings(
        &self,
        circuit: &CircuitData,
        frequencies: &[Value],
    ) -> Result<(), SimulationError> {
        let changes: Vec<(usize, usize)> = self
            .steps
            .iter()
            .enumerate()
            .flat_map(|(index, step)| step.changes.iter().map(move |&switch| (index + 1, switch)))
            .collect();
        if changes.is_empty() {
            return Ok(());
        }
        let control = |switch: usize, x: &[Complex64]| -> Complex64 {
            let node = |node: usize| {
                if node == 0 {
                    Complex64::new(0.0, 0.0)
                } else {
                    x[node - 1]
                }
            };
            if let Some(voltage) = circuit.vswitches.get(switch) {
                node(voltage.ctrl_pos) - node(voltage.ctrl_neg)
            } else {
                let current = &circuit.iswitches[switch - circuit.vswitches.len()];
                current
                    .ctrl_branch
                    .filter(|branch| *branch > 0 && *branch <= x.len())
                    .map_or(Complex64::new(0.0, 0.0), |branch| x[branch - 1])
            }
        };
        for &frequency in [frequencies[0], frequencies[frequencies.len() - 1]].iter() {
            let response = self.respond(std::f64::consts::TAU * frequency)?;
            let scale = response
                .trajectory
                .iter()
                .flatten()
                .fold(0.0, |m: Value, v| m.max(v.norm()));
            for &(point, switch) in &changes {
                let value = control(switch, &response.trajectory[point]).norm();
                if value > 1.0e-9 * scale.max(1.0) {
                    let name = circuit
                        .vswitches
                        .get(switch)
                        .map(|s| s.name.clone())
                        .or_else(|| {
                            circuit
                                .iswitches
                                .get(switch - circuit.vswitches.len())
                                .map(|s| s.name.clone())
                        })
                        .unwrap_or_default();
                    return Err(carrier_refusal(format!(
                        "PAC on the event orbit refuses switch '{name}': its control carries \
                         {value:.3e} of small signal where it changes at {:.6e} s, so its \
                         crossing moves with the input and the impulse that adds is not formed",
                        self.phases[point].rem_euclid(self.period)
                    )));
                }
            }
        }
        Ok(())
    }
}

fn multiply(matrix: &[Value], x: &[Value], n: usize) -> Vec<Value> {
    (0..n)
        .map(|row| {
            matrix[row * n..(row + 1) * n]
                .iter()
                .zip(x)
                .map(|(a, b)| a * b)
                .sum()
        })
        .collect()
}

/// Gaussian elimination with partial pivoting on a small dense complex
/// system; `None` when it is singular.
fn solve_complex(system: &mut [Vec<Complex64>], mut rhs: Vec<Complex64>) -> Option<Vec<Complex64>> {
    let n = rhs.len();
    let scale = system
        .iter()
        .flatten()
        .fold(0.0, |m: Value, v| m.max(v.norm()));
    for k in 0..n {
        let pivot = (k..n).max_by(|&a, &b| system[a][k].norm().total_cmp(&system[b][k].norm()))?;
        if system[pivot][k].norm() <= 1.0e-14 * scale.max(Value::MIN_POSITIVE) {
            return None;
        }
        system.swap(k, pivot);
        rhs.swap(k, pivot);
        for row in k + 1..n {
            let factor = system[row][k] / system[k][k];
            if factor == Complex64::new(0.0, 0.0) {
                continue;
            }
            let (upper, lower) = system.split_at_mut(row);
            for (target, value) in lower[0][k..].iter_mut().zip(&upper[k][k..]) {
                *target -= factor * value;
            }
            let value = rhs[k];
            rhs[row] -= factor * value;
        }
    }
    for row in (0..n).rev() {
        let mut value = rhs[row];
        for col in row + 1..n {
            value -= system[row][col] * rhs[col];
        }
        rhs[row] = value / system[row][row];
    }
    Some(rhs)
}
