//! The period map's exact Jacobian, by the variational recurrence of the
//! accepted steps.
//!
//! A traversal integrates `f(x) + dQ(x)/dt + s(t) = 0`. The step of length
//! `h` to `t_k`, on the companion coefficients `(g, v₁, v₂, i₁)` the
//! transient integrated it with, solves `f(x_k) + Q̇_k + s(t_k) = 0` with
//! `Q̇_k = (g·Q_k − v₁·Q_{k−1} − v₂·Q_{k−2})/h − i₁·Q̇_{k−1}`. Its derivative at
//! the accepted solution is the recurrence `Φ = ∂Π/∂z` follows:
//!
//! - `(g/h·C_k + G_k)·δx_k = (v₁·δQ_{k−1} + v₂·δQ_{k−2})/h + i₁·δQ̇_{k−1}`,
//! - `δQ_k = C_k·δx_k` and `δQ̇_k = g/h·δQ_k − (the right-hand side)`,
//!
//! where `G_k` and `C_k` are the small-signal operator's two parts at the
//! accepted point, read in place as event PAC reads them, and each step's
//! matrix is factored once by the sparse solver the transient uses. A
//! segment leaves its image as from a breakpoint: every reactive history is
//! taken from the image's solution, `δQ_0 = δQ_{−1} = C_0·δz`, and the charge
//! derivative history is the image's own, `δQ̇_0 = 0`. A coordinate no charge
//! reads (an algebraic node, a source branch) maps to zero.
//!
//! **Saltation.** An ideal switch changes where its control crosses the
//! threshold, and that instant `τ` moves with the state. It moves by
//! `δτ = −∇g·δx_τ / ġ`, with `ġ` the control's rate over the step that
//! crossed; over `δτ` the charges follow the old conductance instead of the
//! new, so `δQ_τ += (f_new − f_old)(x_τ)·δτ`: the switch's conductance step
//! times its voltage, into those of its two nodes that hold a charge. A clock-driven switch's control is
//! fixed by its source, `∇g·δx_τ = 0`, and its saltation vanishes.
//!
//! **Scope.** The recurrence is the map's exact derivative where every
//! charge the circuit stores is one a segment takes from its starting
//! solution and the small-signal operator linearizes what the transient
//! integrates on the step's companion coefficients. Capacitors, inductors,
//! their couplings and the native semiconductors' junction, diffusion and
//! gate charges are: a run's start evaluates each of them at the solution it
//! starts from, so a segment from an iterate integrates the iterate's
//! charges. A device whose state is a coordinate of its own or that the
//! start leaves as the image held it (a Verilog-A module's integrator slots,
//! retained variables and events; a code model; a core, a memristor, a
//! thermal resistor; the B3SOI FD/PD and EKV3 models), a stamp that is not
//! `G + jωC`, the Xyce dialect's one-step integration, or a step that arrives
//! without its coefficients keeps the finite-difference columns, and a
//! recurrence the orbit's seeded directions do not confirm gives way to
//! them. A switch a state controls is one: the transient's split step ends
//! just past its crossing, and that step's trapezoid carries only part of the
//! saltation into the discrete map the directions measure.

use std::cell::RefCell;

use super::newton::Orbit;
use super::pac::Linearizer;
use crate::engine::SimulationError;
use crate::engine::transient::state_capture::{
    AcceptedPointObserver, AcceptedStep, CapturedTransientState,
};
use crate::solver::{FactorizationRequest, NumericFactorizationPolicy, StaticMatrix};
use crate::{CircuitData, Value};

/// How many columns go through the steps together.
const COLUMNS_PER_BLOCK: usize = 32;

/// An ideal switch as the recurrence reads it.
struct SwitchSite {
    /// The control: node voltages `(positive, negative)`, or a branch
    /// current at a solution index.
    control: SwitchControl,
    /// The switched branch's nodes, matrix rows counted from one.
    nodes: (usize, usize),
    /// `1/RON` and `1/ROFF`.
    conductances: (Value, Value),
}

#[derive(Clone, Copy)]
enum SwitchControl {
    Voltage(usize, usize),
    Current(usize),
}

/// What an accepted point holds of one switch.
#[derive(Clone, Copy)]
struct SwitchSample {
    state: u64,
    control: Value,
    conductance: Value,
    across: Value,
}

/// One accepted point of the linearized traversal.
struct Point {
    time: Value,
    g: Vec<Value>,
    c: Vec<Value>,
    step: Option<AcceptedStep>,
    switches: Vec<SwitchSample>,
}

/// The saltation at one switch change: `δQ += jump·(−∇g·δx / ġ)`.
struct Saltation {
    gradient: Vec<(usize, Value)>,
    rate: Value,
    jump: Vec<(usize, Value)>,
}

/// One step of the recurrence.
struct Step {
    a: RefCell<StaticMatrix>,
    /// `C_k` on the structure's stored positions.
    c: Vec<Value>,
    /// `g/h`, `v₁/h`, `v₂/h` and `i₁`.
    weights: (Value, Value, Value, Value),
    saltations: Vec<Saltation>,
}

fn row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

fn read(solution: &[Value], node: usize) -> Value {
    row(node).map_or(0.0, |index| {
        solution.get(index).copied().unwrap_or(Value::NAN)
    })
}

impl SwitchSite {
    fn sites(circuit: &CircuitData) -> Vec<Self> {
        let voltage = circuit.vswitches.iter().map(|switch| Self {
            control: SwitchControl::Voltage(switch.ctrl_pos, switch.ctrl_neg),
            nodes: (switch.node_pos, switch.node_neg),
            conductances: (1.0 / switch.ron, 1.0 / switch.roff),
        });
        let current = circuit.iswitches.iter().map(|switch| Self {
            control: SwitchControl::Current(switch.ctrl_branch.unwrap_or(0)),
            nodes: (switch.node_pos, switch.node_neg),
            conductances: (1.0 / switch.ron, 1.0 / switch.roff),
        });
        voltage.chain(current).collect()
    }

    fn control(&self, solution: &[Value]) -> Value {
        match self.control {
            SwitchControl::Voltage(positive, negative) => {
                read(solution, positive) - read(solution, negative)
            }
            SwitchControl::Current(index) => read(solution, index),
        }
    }

    fn gradient(&self) -> Vec<(usize, Value)> {
        match self.control {
            SwitchControl::Voltage(positive, negative) => [(positive, 1.0), (negative, -1.0)]
                .into_iter()
                .filter_map(|(node, sign)| row(node).map(|index| (index, sign)))
                .collect(),
            SwitchControl::Current(index) => {
                row(index).map(|index| (index, 1.0)).into_iter().collect()
            }
        }
    }

    /// Whether `from → to` is the ideal step between `1/ROFF` and `1/RON`.
    fn is_ideal_change(&self, from: &SwitchSample, to: &SwitchSample) -> bool {
        let (on, off) = self.conductances;
        let close = |a: Value, b: Value| (a - b).abs() <= 1.0e-12 * a.abs().max(b.abs());
        from.state != to.state
            && ((close(from.conductance, off) && close(to.conductance, on))
                || (close(from.conductance, on) && close(to.conductance, off)))
    }
}

/// The sample of every switch at a point, in [`SwitchSite::sites`] order.
fn sample_switches(
    circuit: &CircuitData,
    sites: &[SwitchSite],
    solution: &[Value],
) -> Vec<SwitchSample> {
    let mut samples = Vec::with_capacity(sites.len());
    let mut conductances = circuit
        .vswitches
        .iter()
        .map(|switch| {
            (
                switch.state() as u64,
                switch.small_signal_conductance(solution),
            )
        })
        .chain(circuit.iswitches.iter().map(|switch| {
            (
                switch.state() as u64,
                switch.small_signal_conductance(solution),
            )
        }));
    for site in sites {
        let (state, conductance) = conductances.next().unwrap_or((0, 0.0));
        samples.push(SwitchSample {
            state,
            control: site.control(solution),
            conductance,
            across: read(solution, site.nodes.0) - read(solution, site.nodes.1),
        });
    }
    samples
}

impl Orbit<'_> {
    /// Whether the recurrence is the exact derivative of this circuit's
    /// traversals, before any is linearized.
    pub(super) fn admits_variational(&self, image: &CapturedTransientState) -> bool {
        self.engine.config.spice_dialect != crate::SpiceDialect::Xyce
            && self.basis.dimension() == image.solution.len()
            && image.circuit.segment_takes_every_charge_from_its_start()
    }

    /// `Φ` at `z` by the variational recurrence of one traversal over `mesh`,
    /// or `None` where it is not the map's exact derivative.
    pub(super) fn variational_jacobian(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        mesh: &[Value],
    ) -> Result<Option<Vec<Vec<Value>>>, SimulationError> {
        if !self.admits_variational(image) {
            return Ok(None);
        }
        let start = self.basis.write(image, z)?;
        let structure = self.engine.build_matrix(&start.circuit)?;
        let Some(points) = self.linearized_points(&start, mesh, &structure)? else {
            return Ok(None);
        };
        let sites = SwitchSite::sites(&start.circuit);
        let n = start.solution.len();
        let Some(steps) = Self::recurrence_steps(n, &points, &structure, &sites)? else {
            return Ok(None);
        };
        let positions: Vec<(usize, usize)> = structure.stored_positions().collect();
        Ok(Some(propagate(n, &positions, &points[0].c, &steps)?))
    }

    /// One traversal from `start`, linearized at every accepted point, or
    /// `None` when its stamps are not `G + jωC`.
    fn linearized_points(
        &self,
        start: &CapturedTransientState,
        mesh: &[Value],
        structure: &StaticMatrix,
    ) -> Result<Option<Vec<Point>>, SimulationError> {
        let sites = SwitchSite::sites(&start.circuit);
        let mut linearizer = Linearizer::new(&start.circuit, structure, None);
        let first = linearizer.operator(start.time, &start.circuit, &start.solution, 1.0, false)?;
        let doubled =
            linearizer.operator(start.time, &start.circuit, &start.solution, 2.0, false)?;
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
            return Ok(None);
        }
        let first = Point {
            time: start.time,
            switches: sample_switches(&start.circuit, &sites, &start.solution),
            g: first.g,
            c: first.c,
            step: None,
        };
        let linearizer = RefCell::new(linearizer);
        let points = RefCell::new(vec![first]);
        let observer =
            |time: Value, circuit: &CircuitData, solution: &[Value], step: Option<AcceptedStep>| {
                let operator = linearizer
                    .borrow_mut()
                    .operator(time, circuit, solution, 1.0, false)?;
                let point = Point {
                    time,
                    switches: sample_switches(circuit, &sites, solution),
                    g: operator.g,
                    c: operator.c,
                    step,
                };
                let mut points = points.borrow_mut();
                // The segment offers its start, and an instant it accepts twice
                // keeps its later state, which the next step starts from.
                // Its step is the first acceptance's, which moved the time.
                match points.last_mut() {
                    Some(last) if time <= last.time => {
                        let step = match point.step {
                            Some(step) if step.step > 0.0 => Some(step),
                            _ => last.step,
                        };
                        *last = Point { step, ..point };
                    }
                    _ => points.push(point),
                }
                Ok(())
            };
        let observer: &AcceptedPointObserver<'_> = &observer;
        self.engine.run_tran_segment_from_image(
            self.netlist,
            start,
            (mesh, None),
            self.max_step,
            Some(observer),
            self.abort,
        )?;
        let points = points.into_inner();
        if points.len() < 2 || points[1..].iter().any(|point| point.step.is_none()) {
            return Ok(None);
        }
        Ok(Some(points))
    }

    /// Each step's factored matrix, charge operator, weights and
    /// saltations, or `None` for a step the recurrence cannot state.
    fn recurrence_steps(
        n: usize,
        points: &[Point],
        structure: &StaticMatrix,
        sites: &[SwitchSite],
    ) -> Result<Option<Vec<Step>>, SimulationError> {
        let positions: Vec<(usize, usize)> = structure.stored_positions().collect();
        let mut options = structure.solver_options();
        options.numeric_factorization = NumericFactorizationPolicy::ReusePivotSequence;
        let pattern: Vec<(usize, usize, Value)> = positions
            .iter()
            .map(|&(row, col)| (row, col, 0.0))
            .collect();
        let template =
            StaticMatrix::from_triplets_with_options(n, n, &pattern, options).map_err(|error| {
                SimulationError::Circuit(format!(
                    "the period map's recurrence has no step matrix: {error}"
                ))
            })?;
        if !template.stored_positions().eq(positions.iter().copied()) {
            return Ok(None);
        }
        let mut steps = Vec::with_capacity(points.len() - 1);
        for pair in points.windows(2) {
            let (before, after) = (&pair[0], &pair[1]);
            let Some(step) = after.step else {
                return Ok(None);
            };
            let h = step.step;
            let coefficients = step.coefficients;
            if h.is_nan()
                || h <= 0.0
                || (after.time - before.time - h).abs()
                    > 1.0e-9 * h.max(after.time.abs() * Value::EPSILON)
            {
                return Ok(None);
            }
            let weights = (
                coefficients.coeff_g / h,
                coefficients.coeff_v_n / h,
                coefficients.coeff_v_n_minus_1 / h,
                coefficients.coeff_i_n,
            );
            let mut a = template.clone_structure();
            for ((value, g), c) in a.values_mut().iter_mut().zip(&after.g).zip(&after.c) {
                *value = weights.0 * c + g;
            }
            // A charge moves only on a row that holds one.
            let mut holds_charge = vec![false; n];
            for (&(row, _), c) in positions.iter().zip(&after.c) {
                holds_charge[row] |= *c != 0.0;
            }
            let mut saltations = Vec::new();
            for (site, (from, to)) in sites
                .iter()
                .zip(before.switches.iter().zip(&after.switches))
            {
                if !site.is_ideal_change(from, to) {
                    continue;
                }
                let gradient = site.gradient();
                if gradient.is_empty() {
                    continue;
                }
                let rate = (to.control - from.control) / h;
                if !rate.is_finite() || rate == 0.0 {
                    return Ok(None);
                }
                let step = (to.conductance - from.conductance) * to.across;
                let jump = [(site.nodes.0, step), (site.nodes.1, -step)]
                    .into_iter()
                    .filter_map(|(node, value)| row(node).map(|index| (index, value)))
                    .filter(|&(index, _)| holds_charge[index])
                    .collect();
                saltations.push(Saltation {
                    gradient,
                    rate,
                    jump,
                });
            }
            steps.push(Step {
                a: RefCell::new(a),
                c: after.c.clone(),
                weights,
                saltations,
            });
        }
        Ok(Some(steps))
    }
}

/// `y += M·x` for the operator `M` on `positions`.
fn multiply(positions: &[(usize, usize)], values: &[Value], x: &[Value], y: &mut [Value]) {
    for (&(row, col), &value) in positions.iter().zip(values) {
        if value != 0.0 {
            y[row] += value * x[col];
        }
    }
}

/// Every column of `Φ` through the steps, a block at a time.
fn propagate(
    n: usize,
    positions: &[(usize, usize)],
    c0: &[Value],
    steps: &[Step],
) -> Result<Vec<Vec<Value>>, SimulationError> {
    let mut read = vec![false; n];
    for (&(_, col), value) in positions.iter().zip(c0) {
        read[col] |= *value != 0.0;
    }
    let columns: Vec<usize> = (0..n).filter(|&col| read[col]).collect();
    let mut phi = vec![vec![0.0; n]; n];
    let mut charge = Vec::new();
    let mut earlier = Vec::new();
    let mut rate = Vec::new();
    let mut rhs = Vec::new();
    let mut solved = Vec::new();
    for chunk in columns.chunks(COLUMNS_PER_BLOCK) {
        let count = chunk.len();
        charge.clear();
        charge.resize(n * count, 0.0);
        for (j, &col) in chunk.iter().enumerate() {
            let mut unit = vec![0.0; n];
            unit[col] = 1.0;
            multiply(positions, c0, &unit, &mut charge[j * n..(j + 1) * n]);
        }
        earlier.clone_from(&charge);
        rate.clear();
        rate.resize(n * count, 0.0);
        solved.clear();
        solved.resize(n * count, 0.0);
        for (index, step) in steps.iter().enumerate() {
            let (g, v1, v2, i1) = step.weights;
            rhs.clear();
            rhs.extend(
                charge
                    .iter()
                    .zip(&earlier)
                    .zip(&rate)
                    .map(|((q1, q2), qdot)| v1 * q1 + v2 * q2 + i1 * qdot),
            );
            step.a
                .borrow_mut()
                .solve_many_into_with_factorization(
                    &rhs,
                    count,
                    &mut solved,
                    FactorizationRequest::Automatic,
                )
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "the period map's recurrence does not solve at its step {index}: {error}"
                    ))
                })?;
            std::mem::swap(&mut earlier, &mut charge);
            charge.fill(0.0);
            for j in 0..count {
                let range = j * n..(j + 1) * n;
                multiply(
                    positions,
                    &step.c,
                    &solved[range.clone()],
                    &mut charge[range.clone()],
                );
                for ((qdot, q), b) in rate[range.clone()]
                    .iter_mut()
                    .zip(&charge[range.clone()])
                    .zip(&rhs[range])
                {
                    *qdot = g * q - b;
                }
                // The switch's instant moves the charges, not the derivative
                // history the step left.
                for saltation in &step.saltations {
                    let moved: Value = saltation
                        .gradient
                        .iter()
                        .map(|&(row, weight)| weight * solved[j * n + row])
                        .sum();
                    let delay = -moved / saltation.rate;
                    for &(row, value) in &saltation.jump {
                        charge[j * n + row] += value * delay;
                    }
                }
            }
        }
        for (j, &col) in chunk.iter().enumerate() {
            for (row, values) in phi.iter_mut().enumerate() {
                values[col] = solved[j * n + row];
            }
        }
    }
    Ok(phi)
}
