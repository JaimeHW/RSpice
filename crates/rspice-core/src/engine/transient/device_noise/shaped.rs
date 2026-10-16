//! Noise a Verilog-A module shapes itself, rendered in time.
//!
//! A process whose injection gain depends on frequency reaches the circuit
//! through `ddt`, `idt` or `laplace_*`. Its gains are a rational function of
//! `s`, realized exactly as a `G + sC` descriptor driven by the process's
//! unit input (`try_noise_process_realizations` for runtime modules and
//! mixed hosts; a generated device's gains are `real + jω·reactive` by
//! construction, a descriptor with no states). Writing the descriptor as
//!
//! ```text
//! G_SS·z + C_SS·ż + G_Su·u + C_Su·u̇ = 0           (state rows)
//! y_i = G_iS·z + C_iS·ż + G_iu·u + C_iu·u̇          (gain rows)
//! ```
//!
//! and replacing every `d/dt` by the backward difference on the noise grid,
//! `(x_n − x_{n−1})/NT`, gives the filter the circuit stamps:
//!
//! ```text
//! (G_SS + C_SS/NT)·z_n = (C_SS/NT)·z_{n−1} − (G_Su + C_Su/NT)·u_n + (C_Su/NT)·u_{n−1}
//! y_n = (G_iS + C_iS/NT)·z_n − (C_iS/NT)·z_{n−1} + (G_iu + C_iu/NT)·u_n − (C_iu/NT)·u_{n−1}
//! ```
//!
//! With `E = C_SS`, `A = −G_SS`, `B = −G_Su` and the gain rows' `C`, `D₀`,
//! `D₁`, this is `(E − NT·A)·z_n = E·z_{n−1} + NT·B·u_n` with
//! `y_n = C·z_n + D₀·u_n + D₁·(u_n − u_{n−1})/NT`; the general form also
//! carries the terms a nested `ddt` puts on a state's input or on a gain
//! row's state. Substituting one difference operator for every `s` maps the
//! whole descriptor at once, so the discrete filter is exactly
//! `H((1 − q⁻¹)/NT)`: its response follows `|H(j2πf)|` to within the
//! backward difference's `2·sin(πf·NT)/(2πf·NT)` per power of `s`, which the
//! hold's own `sinc(f·NT)` compensates for a pure integrator.
//!
//! The input `u` is the process's train scaled by its amplitude, so a
//! bias-dependent density enters the filter where the module's noise
//! function sits rather than after the shaping. The refresh at each accepted
//! point re-derives the amplitude and the gain rows. The state rows are the
//! filter's poles and input map; a bias that moves them beyond RELTOL
//! refuses the run, because the committed state was integrated under the
//! old dynamics and no rendering of the new ones continues it exactly.
//!
//! The filter is streamed: its state is a handful of values per process, and
//! each sample is computed from the committed state and the index-addressed
//! train, so nothing is materialized and no sample budget applies beyond the
//! train's own. The cost is bounded by the sample count, which the grid
//! already bounds.

use super::processes::{
    CollectedNoiseProcess, NoiseProcessRoute, ProcessRealization, RealizationFailure,
    ShapingDescriptor, ShapingEntry, ShapingIndex, runtime_noise_realizations,
};
use crate::circuit::device_noise::{ShapedOutput, ShapedStep};
use crate::numerics::DenseScalar;
use crate::{CircuitData, Complex64, Value};

/// Relative agreement a realization must show with the per-frequency gains
/// it claims to reproduce.
///
/// Both come from one operating point through one dispatch, so they differ
/// only by the elimination's rounding; a disagreement past this is a wrong
/// pairing of processes or injections, never a modeling difference.
const REALIZATION_AGREEMENT: Value = 1.0e-6;

/// The descriptor of one shaped process at one bias.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ShapedRealization {
    pub(super) descriptor: ShapingDescriptor,
}

/// Why a shaped process cannot be rendered.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ShapingRefusal {
    /// The shaping passes through `absdelay`, `zi_*` or `idtmod`.
    NonRational(&'static str),
    /// Any other failure: a defect, reported as a circuit error.
    Failed(String),
}

impl From<RealizationFailure> for ShapingRefusal {
    fn from(failure: RealizationFailure) -> Self {
        match failure {
            RealizationFailure::NonRational(name) => Self::NonRational(name),
            RealizationFailure::Failed(detail) => Self::Failed(detail),
        }
    }
}

/// The realization of every shaped process among `shaped`, in order.
///
/// `shaped` were evaluated at `frequency`; each realization is checked to
/// reproduce those gains before it is used.
pub(super) fn realize_shaped_processes(
    circuit: &CircuitData,
    solution: &[Value],
    shaped: &[&CollectedNoiseProcess],
    frequency: Value,
) -> Vec<Result<ShapedRealization, ShapingRefusal>> {
    type Realized = Result<Vec<ProcessRealization>, ShapingRefusal>;
    let mut by_instance: std::collections::HashMap<(NoiseProcessRoute, &str), Realized> =
        std::collections::HashMap::new();
    shaped
        .iter()
        .map(|process| {
            let descriptor = match process.route {
                NoiseProcessRoute::Generated => first_order_descriptor(process, frequency),
                route => {
                    let realized = by_instance
                        .entry((route, process.instance.as_str()))
                        .or_insert_with(|| {
                            runtime_noise_realizations(circuit, solution, route, &process.instance)
                                .map_err(ShapingRefusal::from)
                        });
                    let realized = realized.as_ref().map_err(Clone::clone)?;
                    let ProcessRealization {
                        descriptor,
                        injections,
                        ..
                    } = realized
                        .iter()
                        .find(|realization| realization.process_id == process.process_id)
                        .ok_or_else(|| {
                            ShapingRefusal::Failed(format!(
                                "noise process '{}' has no shaping realization",
                                process.describe()
                            ))
                        })?;
                    let endpoints: Vec<(usize, usize)> = process
                        .injections
                        .iter()
                        .map(|injection| (injection.node_pos, injection.node_neg))
                        .collect();
                    if *injections != endpoints {
                        return Err(ShapingRefusal::Failed(format!(
                            "noise process '{}' realizes injections {injections:?} but is \
                             evaluated at {endpoints:?}",
                            process.describe()
                        )));
                    }
                    descriptor.clone()
                }
            };
            check_reproduces(&descriptor, process, frequency).map_err(ShapingRefusal::Failed)?;
            Ok(ShapedRealization { descriptor })
        })
        .collect()
}

/// The stateless descriptor of a generated process: its gains are
/// `real + j2πf·reactive`, so each injection is `real + s·reactive`.
fn first_order_descriptor(process: &CollectedNoiseProcess, frequency: Value) -> ShapingDescriptor {
    let omega = std::f64::consts::TAU * frequency;
    ShapingDescriptor {
        states: 0,
        entries: process
            .injections
            .iter()
            .enumerate()
            .map(|(position, injection)| ShapingEntry {
                row: ShapingIndex::Output(position),
                col: ShapingIndex::Input,
                g: injection.gain.re,
                c: if omega > 0.0 {
                    injection.gain.im / omega
                } else {
                    0.0
                },
            })
            .collect(),
    }
}

/// Whether a generated process's gains are `real + jω·reactive` at a second
/// frequency, as the first-order descriptor assumes.
pub(super) fn is_first_order(
    at_reference: &CollectedNoiseProcess,
    reference: Value,
    other: &CollectedNoiseProcess,
    frequency: Value,
) -> bool {
    let descriptor = first_order_descriptor(at_reference, reference);
    at_reference.injections.len() == other.injections.len()
        && check_reproduces(&descriptor, other, frequency).is_ok()
}

/// Reduce a descriptor at `s = j2πf` to its injection gains.
fn reduced_gains(
    descriptor: &ShapingDescriptor,
    outputs: usize,
    frequency: Value,
) -> Result<Vec<Complex64>, String> {
    let s = Complex64::new(0.0, std::f64::consts::TAU * frequency);
    let n = descriptor.states;
    let mut states = vec![vec![Complex64::default(); n + 1]; n];
    let mut gains = vec![Complex64::default(); outputs];
    let mut couplings = vec![vec![Complex64::default(); n]; outputs];
    for entry in &descriptor.entries {
        let value = Complex64::new(entry.g, 0.0) + s * entry.c;
        match (entry.row, entry.col) {
            (ShapingIndex::State(row), ShapingIndex::State(col)) if row < n && col < n => {
                states[row][col] += value;
            }
            (ShapingIndex::State(row), ShapingIndex::Input) if row < n => {
                states[row][n] += value;
            }
            (ShapingIndex::Output(row), ShapingIndex::State(col)) if row < outputs && col < n => {
                couplings[row][col] += value;
            }
            (ShapingIndex::Output(row), ShapingIndex::Input) if row < outputs => {
                gains[row] += value;
            }
            _ => return Err(format!("shaping entry {entry:?} is outside its descriptor")),
        }
    }
    if n > 0 {
        // M_SS·x = M_Su, so the gains are M_iu − M_iS·x.
        let solved = solve_dense(states, 1)
            .ok_or_else(|| format!("the shaping states are singular at {frequency:e} Hz"))?;
        for (gain, coupling) in gains.iter_mut().zip(&couplings) {
            for (coefficient, x) in coupling.iter().zip(&solved) {
                *gain -= *coefficient * x[0];
            }
        }
    }
    Ok(gains)
}

/// Check that a descriptor reproduces a process's gains at `frequency`.
fn check_reproduces(
    descriptor: &ShapingDescriptor,
    process: &CollectedNoiseProcess,
    frequency: Value,
) -> Result<(), String> {
    let gains = reduced_gains(descriptor, process.injections.len(), frequency)?;
    let scale = process
        .injections
        .iter()
        .map(|injection| injection.gain.norm())
        .fold(0.0, Value::max);
    for (position, (realized, injection)) in gains.iter().zip(&process.injections).enumerate() {
        let apart = (*realized - injection.gain).norm();
        if apart.is_nan() || apart > REALIZATION_AGREEMENT * scale {
            return Err(format!(
                "noise process '{}' injection {position}: its shaping realization gives \
                 {realized} at {frequency:e} Hz, the evaluation {}",
                process.describe(),
                injection.gain
            ));
        }
    }
    Ok(())
}

/// The state step of a descriptor discretized by the backward difference on
/// an `nt` grid.
pub(super) fn discretized_step(
    descriptor: &ShapingDescriptor,
    nt: Value,
) -> Result<ShapedStep, String> {
    let n = descriptor.states;
    // Columns: C_SS/NT (n), then the drive and the carry.
    let mut system = vec![vec![0.0; 2 * n + 2]; n];
    for entry in &descriptor.entries {
        let ShapingIndex::State(row) = entry.row else {
            continue;
        };
        if row >= n {
            return Err(format!("shaping entry {entry:?} is outside its descriptor"));
        }
        match entry.col {
            ShapingIndex::State(col) if col < n => {
                system[row][col] += entry.g + entry.c / nt;
                system[row][n + col] += entry.c / nt;
            }
            ShapingIndex::Input => {
                system[row][2 * n] -= entry.g + entry.c / nt;
                system[row][2 * n + 1] += entry.c / nt;
            }
            _ => return Err(format!("shaping entry {entry:?} is outside its descriptor")),
        }
    }
    let mut transition = vec![0.0; n * n];
    let mut drive = vec![0.0; n];
    let mut carry = vec![0.0; n];
    if n > 0 {
        let solved = solve_dense(system, n + 2)
            .ok_or_else(|| format!("the shaping states are singular on the {nt:e} s noise grid"))?;
        for (row, values) in solved.iter().enumerate() {
            transition[row * n..(row + 1) * n].copy_from_slice(&values[..n]);
            drive[row] = values[n];
            carry[row] = values[n + 1];
        }
    }
    if transition
        .iter()
        .chain(&drive)
        .chain(&carry)
        .any(|value| !value.is_finite())
    {
        return Err(format!(
            "the shaping filter is not representable on the {nt:e} s noise grid"
        ));
    }
    Ok(ShapedStep {
        transition,
        drive,
        carry,
    })
}

/// The outputs of a descriptor discretized by the backward difference on an
/// `nt` grid, one per injection.
pub(super) fn discretized_outputs(
    descriptor: &ShapingDescriptor,
    outputs: usize,
    nt: Value,
) -> Vec<ShapedOutput> {
    let n = descriptor.states;
    let mut discrete = vec![
        ShapedOutput {
            state: vec![0.0; n],
            prior_state: vec![0.0; n],
            input: 0.0,
            prior_input: 0.0,
        };
        outputs
    ];
    for entry in &descriptor.entries {
        let ShapingIndex::Output(row) = entry.row else {
            continue;
        };
        let Some(output) = discrete.get_mut(row) else {
            continue;
        };
        match entry.col {
            ShapingIndex::State(col) if col < n => {
                output.state[col] += entry.g + entry.c / nt;
                output.prior_state[col] -= entry.c / nt;
            }
            ShapingIndex::Input => {
                output.input += entry.g + entry.c / nt;
                output.prior_input -= entry.c / nt;
            }
            _ => {}
        }
    }
    discrete
}

/// Whether the filter's own dynamics — the state rows, its poles and input
/// map — differ between two biases by more than `reltol`.
///
/// Each row is compared at its own scale, so an entry that is round-off
/// beside the row's largest coefficient cannot fail it.
pub(super) fn state_rows_moved(
    before: &ShapingDescriptor,
    after: &ShapingDescriptor,
    reltol: Value,
) -> bool {
    if before.states != after.states {
        return true;
    }
    let rows = |descriptor: &ShapingDescriptor| {
        let mut rows: std::collections::BTreeMap<(usize, ShapingIndex), (Value, Value)> =
            std::collections::BTreeMap::new();
        for entry in &descriptor.entries {
            if let ShapingIndex::State(row) = entry.row {
                let slot = rows.entry((row, entry.col)).or_default();
                slot.0 += entry.g;
                slot.1 += entry.c;
            }
        }
        rows
    };
    let (left, right) = (rows(before), rows(after));
    let mut scales = vec![(0.0_f64, 0.0_f64); before.states];
    for (&(row, _), &(g, c)) in left.iter().chain(&right) {
        scales[row].0 = scales[row].0.max(g.abs());
        scales[row].1 = scales[row].1.max(c.abs());
    }
    let keys: std::collections::BTreeSet<_> = left.keys().chain(right.keys()).collect();
    keys.into_iter().any(|key| {
        let (g0, c0) = left.get(key).copied().unwrap_or_default();
        let (g1, c1) = right.get(key).copied().unwrap_or_default();
        let (g_scale, c_scale) = scales[key.0];
        (g1 - g0).abs() > reltol * g_scale || (c1 - c0).abs() > reltol * c_scale
    })
}

/// Solve `A·X = B` for an `n × (n + columns)` augmented system, by partial
/// pivoting. `None` when the system is singular.
fn solve_dense<T: DenseScalar>(mut system: Vec<Vec<T>>, columns: usize) -> Option<Vec<Vec<T>>> {
    let n = system.len();
    for pivot in 0..n {
        let best = (pivot..n).max_by(|&left, &right| {
            system[left][pivot]
                .magnitude()
                .total_cmp(&system[right][pivot].magnitude())
        })?;
        let magnitude = system[best][pivot].magnitude();
        if magnitude == 0.0 || !magnitude.is_finite() {
            return None;
        }
        system.swap(pivot, best);
        let (above, below) = system.split_at_mut(pivot + 1);
        let pivot_row = &above[pivot];
        for row in below.iter_mut() {
            let factor = row[pivot].quotient(pivot_row[pivot]);
            for col in pivot..n + columns {
                let product = factor * pivot_row[col];
                row[col] -= product;
            }
        }
    }
    let mut solution = vec![vec![T::default(); columns]; n];
    for row in (0..n).rev() {
        for column in 0..columns {
            let mut residual = system[row][n + column];
            for col in (row + 1)..n {
                let product = system[row][col] * solution[col][column];
                residual -= product;
            }
            let value = residual.quotient(system[row][row]);
            if !value.finite() {
                return None;
            }
            solution[row][column] = value;
        }
    }
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(row: ShapingIndex, col: ShapingIndex, g: Value, c: Value) -> ShapingEntry {
        ShapingEntry { row, col, g, c }
    }

    /// `laplace_nd(u, {1}, {1, τ})` as its cut realizes it: one state with
    /// `s·z − A·z − B·u = 0`, `A = −1/τ`, `B = 1/τ`, and output `z`.
    fn first_order_lowpass(tau: Value) -> ShapingDescriptor {
        use ShapingIndex::{Input, Output, State};
        ShapingDescriptor {
            states: 1,
            entries: vec![
                entry(Output(0), State(0), 1.0, 0.0),
                entry(State(0), Input, -1.0 / tau, 0.0),
                entry(State(0), State(0), 1.0 / tau, 1.0),
            ],
        }
    }

    /// The backward-difference step of a low-pass is the exact recurrence
    /// `z_n = (z_{n−1} + (NT/τ)·u_n)/(1 + NT/τ)`.
    #[test]
    fn a_first_order_lowpass_discretizes_to_its_backward_difference() {
        let (tau, nt) = (1.0e-6, 1.0e-9);
        let step = discretized_step(&first_order_lowpass(tau), nt).expect("discretizes");
        let ratio = nt / tau;
        assert!((step.transition[0] - 1.0 / (1.0 + ratio)).abs() < 1.0e-15);
        assert!((step.drive[0] - ratio / (1.0 + ratio)).abs() < 1.0e-15);
        assert_eq!(step.carry[0], 0.0);
        let outputs = discretized_outputs(&first_order_lowpass(tau), 1, nt);
        assert_eq!(outputs[0].state, vec![1.0]);
        assert_eq!(outputs[0].prior_state, vec![0.0]);
    }

    /// The descriptor's reduction is `1/(1 + sτ)`.
    #[test]
    fn a_lowpass_reduces_to_its_transfer_function() {
        let tau = 1.0e-6;
        let frequency = 3.0e5;
        let gains = reduced_gains(&first_order_lowpass(tau), 1, frequency).expect("reduces");
        let expected =
            Complex64::new(1.0, 0.0) / Complex64::new(1.0, std::f64::consts::TAU * frequency * tau);
        assert!((gains[0] - expected).norm() < 1.0e-12);
    }

    /// A `ddt`-shaped gain, `s·k`, discretizes to the backward difference of
    /// its input and needs no state.
    #[test]
    fn a_differentiated_input_is_its_backward_difference() {
        let descriptor = ShapingDescriptor {
            states: 0,
            entries: vec![entry(
                ShapingIndex::Output(0),
                ShapingIndex::Input,
                0.0,
                2.0,
            )],
        };
        let outputs = discretized_outputs(&descriptor, 1, 0.5);
        assert_eq!(outputs[0].input, 4.0);
        assert_eq!(outputs[0].prior_input, -4.0);
        assert!(discretized_step(&descriptor, 0.5).is_ok());
    }

    fn generated_process(gains: &[Complex64]) -> CollectedNoiseProcess {
        use super::super::processes::{NoiseProcessInjection, NoiseProcessSpectrum};
        CollectedNoiseProcess {
            instance: "x1".to_string(),
            module: "gen".to_string(),
            route: NoiseProcessRoute::Generated,
            process_id: 0,
            mechanism: "D".to_string(),
            spectrum: NoiseProcessSpectrum::White,
            psd: 1.0,
            injections: gains
                .iter()
                .enumerate()
                .map(|(position, gain)| NoiseProcessInjection {
                    node_pos: position + 1,
                    node_neg: 0,
                    gain: *gain,
                })
                .collect(),
        }
    }

    /// A generated process's `real + jω·reactive` gains realize as the
    /// stateless descriptor `real + s·reactive`, which the same evaluation at
    /// another frequency confirms and a different law does not.
    #[test]
    fn a_generated_gain_realizes_as_its_first_order_form() {
        let omega = |frequency: Value| std::f64::consts::TAU * frequency;
        let at = |frequency: Value| {
            generated_process(&[
                Complex64::new(2.0, omega(frequency) * 3.0),
                Complex64::new(-1.0, 0.0),
            ])
        };
        let (reference, other) = (5.0e8, 2.0e6);
        let realized =
            realize_shaped_processes(&CircuitData::new(), &[], &[&at(reference)], reference);
        let descriptor = realized[0].as_ref().expect("realizes").descriptor.clone();
        assert_eq!(descriptor.states, 0);
        assert_eq!(descriptor.entries[0].g, 2.0);
        assert!((descriptor.entries[0].c - 3.0).abs() < 1.0e-15);
        assert_eq!(descriptor.entries[1].g, -1.0);
        assert_eq!(descriptor.entries[1].c, 0.0);
        assert!(is_first_order(&at(reference), reference, &at(other), other));
        let quadratic = generated_process(&[
            Complex64::new(2.0, omega(other).powi(2) * 3.0),
            Complex64::new(-1.0, 0.0),
        ]);
        assert!(!is_first_order(
            &at(reference),
            reference,
            &quadratic,
            other
        ));
    }

    /// The dynamics are compared row by row at the row's own scale.
    #[test]
    fn moved_state_rows_are_measured_against_the_row_scale() {
        let base = first_order_lowpass(1.0e-6);
        let mut nudged = base.clone();
        nudged.entries[1].g *= 1.0 + 1.0e-6;
        assert!(!state_rows_moved(&base, &nudged, 1.0e-3));
        let mut moved = base.clone();
        moved.entries[2].g *= 1.01;
        assert!(state_rows_moved(&base, &moved, 1.0e-3));
        let mut gain_only = base.clone();
        gain_only.entries[0].g = 5.0;
        assert!(!state_rows_moved(&base, &gain_only, 1.0e-3));
    }
}
