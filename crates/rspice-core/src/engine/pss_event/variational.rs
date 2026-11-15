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
//! **Saltation.** An ideal switch changes where its control `c(x)` crosses
//! its threshold. A locked step is landed a resolution short of the crossing,
//! where the switch still holds its state, and the switch changes at the
//! start of the next step, which leaves at order one over a tenth of the
//! mesh's step, before the step to the mesh's target resumes; the instant `τ`
//! the step into it ends at is the root of `c`, so it moves with the state.
//! That step, of length `h_A = τ − t_{k−1}`, ends at `x_τ = x_τ^fix + w·δτ`:
//! the derivative at a fixed end, and `w = ∂x_τ/∂τ`, which solves
//!
//! - `(g/h_A·C_τ + G_τ)·w = (Q̇_τ + i₁·Q̇_{k−1})/h_A + ḃ(τ)`,
//!
//! the step's own residual moved with its end: its charge derivative's
//! `1/h_A` and the sources' right-hand side `b`, read from the left. The
//! crossing holds `∇c·δx_τ = 0`, so `δτ = −∇c·δx_τ^fix / ∇c·w`, and
//!
//! - `δQ_τ += C_τ·w·δτ`, `δQ̇_τ += (g/h_A·C_τ·w − (Q̇_τ + i₁·Q̇_{k−1})/h_A)·δτ`.
//!
//! The step leaving `τ`, of length `h_B = t_k − τ`, moves with its start, and
//! its end follows by `s`: the order-one step's end a fixed interval after
//! `τ` moves with it, `s = 1`, one halfway to its target by `s = 1/2`, and an
//! end at the target stays, `s = 0`. Its right-hand side gains
//!
//! - `((s − 1)·(Q̇_k + i₁·Q̇_τ)/h_B + s·ḃ(t_k))·δτ`,
//!
//! its charge derivative keeps the moved end's `s·ḃ(t_k)·δτ`, and the step
//! after it, whose start moves by `s·δτ`, gains
//!
//! - `−s·(Q̇_{k+1} + i₁·Q̇_k)/h_{k+1}·δτ`.
//!
//! Those charge derivatives are the companions' own,
//! `Q̇_k = (g·ΔQ_k + v₂·ΔQ_{k−1})/h − i₁·Q̇_{k−1}`, with the charge steps
//! `ΔQ_k` taken on the mean of the two points' `C`. A clock-driven switch's
//! control is fixed by its source, `∇c·δx_τ = 0`, and its saltation
//! vanishes.
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
//! them. A saltation is exact where the charges are linear, the
//! source-driven residual is the independent sources', and the steps around
//! the crossing are one-step formulas whose coefficients do not depend on
//! the step ratio (backward Euler, the trapezoid); elsewhere it is the
//! nearest the recurrence states, and the directions decide.

use std::cell::RefCell;

use super::newton::Orbit;
use super::pac::Linearizer;
use crate::circuit::SourceTimeSide;
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
    conductance: Value,
}

/// One accepted point of the linearized traversal.
struct Point {
    time: Value,
    g: Vec<Value>,
    c: Vec<Value>,
    solution: Vec<Value>,
    step: Option<AcceptedStep>,
    switches: Vec<SwitchSample>,
}

/// A switch change at the start of a step, whose instant `τ` the step into
/// it ended at: with `δτ = −∇c·δx_τ / rate`, the step's start moves by
/// `δQ_τ += charge·δτ` and `δQ̇_τ += history·δτ`, its right-hand side by
/// `rhs·δτ` and its charge derivative by `end_rate·δτ`, and the next step's
/// right-hand side by `next_rhs·δτ`.
struct Saltation {
    gradient: Vec<(usize, Value)>,
    /// `∇c·w`.
    rate: Value,
    /// `C_τ·w`.
    charge: Vec<Value>,
    /// `g/h_A·C_τ·w − (Q̇_τ + i₁·Q̇_{k−1})/h_A`.
    history: Vec<Value>,
    /// `(s − 1)·(Q̇_k + i₁·Q̇_τ)/h_B + s·ḃ(t_k)`, with `s` how far the
    /// step's end follows its start.
    rhs: Vec<Value>,
    /// `s·ḃ(t_k)`; empty for an end that stays.
    end_rate: Vec<Value>,
    /// `−s·(Q̇_{k+1} + i₁·Q̇_k)/h_{k+1}`; empty for an end that stays.
    next_rhs: Vec<Value>,
}

/// One step of the recurrence.
struct Step {
    a: RefCell<StaticMatrix>,
    /// `C_k` on the structure's stored positions.
    c: Vec<Value>,
    /// `g/h`, `v₁/h`, `v₂/h` and `i₁`.
    weights: (Value, Value, Value, Value),
    /// The switch changes at the step's start.
    saltations: Vec<Saltation>,
}

fn row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

impl SwitchSite {
    fn sites(circuit: &CircuitData) -> Vec<Self> {
        let voltage = circuit.vswitches.iter().map(|switch| Self {
            control: SwitchControl::Voltage(switch.ctrl_pos, switch.ctrl_neg),
            conductances: (1.0 / switch.ron, 1.0 / switch.roff),
        });
        let current = circuit.iswitches.iter().map(|switch| Self {
            control: SwitchControl::Current(switch.ctrl_branch.unwrap_or(0)),
            conductances: (1.0 / switch.ron, 1.0 / switch.roff),
        });
        voltage.chain(current).collect()
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
    for _ in sites {
        let (state, conductance) = conductances.next().unwrap_or((0, 0.0));
        samples.push(SwitchSample { state, conductance });
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
        let Some(steps) = Self::recurrence_steps(&start.circuit, &points, &structure, &sites)?
        else {
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
            solution: start.solution.clone(),
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
                    solution: solution.to_vec(),
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
        circuit: &CircuitData,
        points: &[Point],
        structure: &StaticMatrix,
        sites: &[SwitchSite],
    ) -> Result<Option<Vec<Step>>, SimulationError> {
        let n = points[0].solution.len();
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
        // The companions' charge derivatives at the latest three points, and
        // the latest two charge steps, which a saltation moves with its
        // instant. A circuit with no switch has none.
        let mut derivatives = Derivatives::new(if sites.is_empty() { 0 } else { n });
        let mut steps: Vec<Step> = Vec::with_capacity(points.len() - 1);
        for (index, pair) in points.windows(2).enumerate() {
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
            derivatives.advance(&positions, before, after, weights);
            // The previous step's end followed a switch's instant, and this
            // step's start moves with it.
            if let Some(follows) = before.step.map(|into| into.follows).filter(|s| *s != 0.0)
                && let Some(previous) = steps.last_mut()
            {
                if step.follows != 0.0 {
                    return Ok(None);
                }
                let next_rhs: Vec<Value> = derivatives
                    .at(0)
                    .iter()
                    .zip(derivatives.at(1))
                    .map(|(now, then)| -follows * (now + weights.3 * then) / h)
                    .collect();
                for saltation in &mut previous.saltations {
                    saltation.next_rhs.clone_from(&next_rhs);
                }
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
                // A change on the step from the image has no step into its
                // instant: the image is where it was dated.
                let (Some(previous), false) = (index.checked_sub(1), gradient.is_empty()) else {
                    continue;
                };
                let Some(into) = before.step else {
                    return Ok(None);
                };
                let (into_g, into_i1) = (into.coefficients.coeff_g, into.coefficients.coeff_i_n);
                // `(Q̇_τ + i₁·Q̇_{k−1})/h_A`, the step into `τ`'s charge
                // derivative moved with its end.
                let reach: Vec<Value> = derivatives
                    .at(1)
                    .iter()
                    .zip(derivatives.at(2))
                    .map(|(now, then)| (now + into_i1 * then) / into.step)
                    .collect();
                let sources = source_rate(circuit, before.time, 1.0e-3 * into.step, n);
                let moved: Vec<Value> = reach.iter().zip(&sources).map(|(a, b)| a + b).collect();
                let mut w = vec![0.0; n];
                steps[previous]
                    .a
                    .borrow_mut()
                    .solve_many_into_with_factorization(
                        &moved,
                        1,
                        &mut w,
                        FactorizationRequest::Automatic,
                    )
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "the period map's recurrence does not solve the instant of \
                             its step {index}: {error}"
                        ))
                    })?;
                let rate: Value = gradient.iter().map(|&(row, weight)| weight * w[row]).sum();
                if !rate.is_finite() || rate == 0.0 {
                    return Ok(None);
                }
                let mut charge = vec![0.0; n];
                multiply(&positions, &before.c, &w, &mut charge);
                let history = charge
                    .iter()
                    .zip(&reach)
                    .map(|(q, reach)| into_g / into.step * q - reach)
                    .collect();
                let follows = step.follows;
                let end_rate: Vec<Value> = if follows == 0.0 {
                    Vec::new()
                } else {
                    source_rate(circuit, after.time, 1.0e-3 * h, n)
                        .into_iter()
                        .map(|rate| follows * rate)
                        .collect()
                };
                let rhs = derivatives
                    .at(0)
                    .iter()
                    .zip(derivatives.at(1))
                    .enumerate()
                    .map(|(row, (now, then))| {
                        (follows - 1.0) * (now + weights.3 * then) / h
                            + end_rate.get(row).copied().unwrap_or(0.0)
                    })
                    .collect();
                saltations.push(Saltation {
                    gradient,
                    rate,
                    charge,
                    history,
                    rhs,
                    end_rate,
                    next_rhs: Vec::new(),
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

/// The companions' charge derivatives along the traversal, newest first,
/// and the latest two charge steps.
struct Derivatives {
    rates: [Vec<Value>; 3],
    changes: [Vec<Value>; 2],
}

impl Derivatives {
    fn new(n: usize) -> Self {
        Self {
            rates: std::array::from_fn(|_| vec![0.0; n]),
            changes: std::array::from_fn(|_| vec![0.0; n]),
        }
    }

    /// `Q̇` at the newest point, `back` points ago.
    fn at(&self, back: usize) -> &[Value] {
        &self.rates[back]
    }

    /// Take the step from `before` to `after` on `weights`: `ΔQ_k` on the
    /// mean of the two points' `C`, and `Q̇_k = (g·ΔQ_k + v₂·ΔQ_{k−1})/h −
    /// i₁·Q̇_{k−1}`, the companion's own, since `g = v₁ + v₂`. The image's
    /// derivative history is not known and reads zero; a segment leaves its
    /// image at order one, which does not read it.
    fn advance(
        &mut self,
        positions: &[(usize, usize)],
        before: &Point,
        after: &Point,
        (g, _, v2, i1): (Value, Value, Value, Value),
    ) {
        if self.rates[0].is_empty() {
            return;
        }
        self.changes.swap(0, 1);
        self.rates.rotate_right(1);
        let difference: Vec<Value> = after
            .solution
            .iter()
            .zip(&before.solution)
            .map(|(a, b)| a - b)
            .collect();
        let [change, earlier] = &mut self.changes;
        change.fill(0.0);
        multiply(positions, &after.c, &difference, change);
        multiply(positions, &before.c, &difference, change);
        change.iter_mut().for_each(|value| *value *= 0.5);
        let [now, then, _] = &mut self.rates;
        for (((rate, change), earlier), then) in
            now.iter_mut().zip(&*change).zip(&*earlier).zip(&*then)
        {
            *rate = g * change + v2 * earlier - i1 * then;
        }
    }
}

/// `ḃ(t)`, the rate of the independent sources' right-hand side at `time`,
/// from the left: the step into `t` ends there and reads the sources' left
/// limit. The second-order backward difference over `spacing`, a thousandth
/// of that step, reaches no source corner, since a step ends at every one.
fn source_rate(circuit: &CircuitData, time: Value, spacing: Value, n: usize) -> Vec<Value> {
    let nodes = circuit.num_nodes();
    let at = |instant: Value| {
        let mut rhs = vec![0.0; n];
        circuit.voltage_sources.update_transient_rhs_on_side(
            &mut rhs,
            instant,
            |ordinal| nodes + ordinal,
            SourceTimeSide::LeftLimit,
        );
        circuit.current_sources.stamp_transient_rhs_on_side(
            &mut rhs,
            instant,
            SourceTimeSide::LeftLimit,
        );
        rhs
    };
    let (now, one, two) = (at(time), at(time - spacing), at(time - 2.0 * spacing));
    now.iter()
        .zip(&one)
        .zip(&two)
        .map(|((now, one), two)| (3.0 * now - 4.0 * one + two) / (2.0 * spacing))
        .collect()
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
    let mut delays = Vec::new();
    let mut carried = Vec::new();
    for chunk in columns.chunks(COLUMNS_PER_BLOCK) {
        let count = chunk.len();
        carried.clear();
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
            // A switch's instant at the step's start moves the charges and
            // their derivative there, and the step's own length.
            delays.clear();
            for saltation in &step.saltations {
                for j in 0..count {
                    let moved: Value = saltation
                        .gradient
                        .iter()
                        .map(|&(row, weight)| weight * solved[j * n + row])
                        .sum();
                    let delay = -moved / saltation.rate;
                    delays.push(delay);
                    let range = j * n..(j + 1) * n;
                    for ((q, qdot), (shift, history)) in charge[range.clone()]
                        .iter_mut()
                        .zip(&mut rate[range])
                        .zip(saltation.charge.iter().zip(&saltation.history))
                    {
                        *q += shift * delay;
                        *qdot += history * delay;
                    }
                }
            }
            rhs.clear();
            rhs.extend(
                charge
                    .iter()
                    .zip(&earlier)
                    .zip(&rate)
                    .map(|((q1, q2), qdot)| v1 * q1 + v2 * q2 + i1 * qdot),
            );
            for (saltation, delays) in step.saltations.iter().zip(delays.chunks(count)) {
                for (j, delay) in delays.iter().enumerate() {
                    for (b, shift) in rhs[j * n..(j + 1) * n].iter_mut().zip(&saltation.rhs) {
                        *b += shift * delay;
                    }
                }
            }
            // An instant at the previous step's start moved its end, which
            // is this step's start.
            let previous = index
                .checked_sub(1)
                .map_or(&[][..], |p| &steps[p].saltations[..]);
            for (saltation, delays) in previous.iter().zip(carried.chunks(count)) {
                for (j, delay) in delays.iter().enumerate() {
                    for (b, shift) in rhs[j * n..(j + 1) * n].iter_mut().zip(&saltation.next_rhs) {
                        *b += shift * delay;
                    }
                }
            }
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
                    .zip(&rhs[range.clone()])
                {
                    *qdot = g * q - b;
                }
                for (saltation, delays) in step.saltations.iter().zip(delays.chunks(count)) {
                    for (qdot, shift) in rate[range.clone()].iter_mut().zip(&saltation.end_rate) {
                        *qdot += shift * delays[j];
                    }
                }
            }
            std::mem::swap(&mut carried, &mut delays);
        }
        for (j, &col) in chunk.iter().enumerate() {
            for (row, values) in phi.iter_mut().enumerate() {
                values[col] = solved[j * n + row];
            }
        }
    }
    Ok(phi)
}
