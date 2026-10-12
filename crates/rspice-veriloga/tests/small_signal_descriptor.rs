//! Exact `G + sC` descriptors of Verilog-A small-signal responses.
//!
//! The descriptor is decisive only against the complex evaluator it must
//! reproduce: reducing its states out at `s = jω` gives an admittance, and
//! that admittance must equal `try_stamp_small_signal_complex` at every
//! frequency. Each row below checks five frequencies to 1e-10 relative and
//! the state count against the operator's analytic order. The same check
//! holds a noise realization to the per-frequency injection gains.
//!
//! Every assertion here holds on the interpreter and on the native route;
//! the exact-descriptor rows state bit for bit what both must produce.

mod support;

use num_complex::Complex64;
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::vm::{
    DescriptorEntry, DescriptorIndex, DescriptorStateKind, SmallSignalDescriptor, VmError,
};
use std::collections::{BTreeMap, BTreeSet};
use support::DeviceFixture;

const FREQUENCIES: [f64; 5] = [0.01, 0.3, 1.0, 3.0, 50.0];
const TOLERANCE: f64 = 1e-10;

type Admittance = BTreeMap<(usize, usize), Complex64>;

fn module(ports: &str, declarations: &str, body: &str) -> String {
    format!(
        "`include \"disciplines.vams\"
         module dut({ports}); inout {ports}; electrical {ports}; {declarations}
         analog begin {body} end endmodule"
    )
}

/// One device under test: its source, its node mapping and its bias.
struct Fixture {
    source: String,
    nodes: &'static [usize],
    internal: &'static [usize],
    branches: &'static [usize],
    bias: &'static [f64],
}

impl Fixture {
    fn two_terminal(body: &str) -> Self {
        Self {
            source: module("a, b", "real x, y, q;", body),
            nodes: &[1, 2],
            internal: &[],
            branches: &[],
            bias: &[0.25, -0.5],
        }
    }

    fn device(&self, analysis: u8) -> VerilogADevice {
        let fixture = DeviceFixture::compile(&self.source);
        let mut device = fixture
            .try_device("X1", self.nodes)
            .unwrap_or_else(|error| panic!("{}: {error}", self.source));
        if !self.internal.is_empty() {
            device.set_internal_node_indices(self.internal);
        }
        if !self.branches.is_empty() {
            device.set_branch_current_indices(self.branches);
        }
        device.update_all_voltages(self.bias);
        device.try_evaluate().expect("operating point");
        device.set_analysis_type(analysis);
        device
    }
}

fn complex_admittance(device: &mut VerilogADevice, bias: &[f64], frequency: f64) -> Admittance {
    let mut admittance = Admittance::new();
    device
        .try_stamp_small_signal_complex(bias, frequency, |row, col, re, im| {
            *admittance.entry((row, col)).or_default() += Complex64::new(re, im);
        })
        .expect("complex small-signal stamp");
    admittance
}

/// Solve `a · x = b` for every column of `b` by Gaussian elimination with
/// partial pivoting.
fn solve(mut a: Vec<Vec<Complex64>>, mut b: Vec<Vec<Complex64>>) -> Vec<Vec<Complex64>> {
    let n = a.len();
    for k in 0..n {
        let pivot = (k..n)
            .max_by(|&i, &j| a[i][k].norm().total_cmp(&a[j][k].norm()))
            .expect("non-empty");
        assert!(a[pivot][k].norm() > 0.0, "singular state block");
        a.swap(k, pivot);
        b.swap(k, pivot);
        let (a_done, a_rest) = a.split_at_mut(k + 1);
        let (b_done, b_rest) = b.split_at_mut(k + 1);
        let (pivot_row, pivot_rhs) = (&a_done[k], &b_done[k]);
        for (row, rhs) in a_rest.iter_mut().zip(b_rest) {
            let factor = row[k] / pivot_row[k];
            for (value, pivot) in row.iter_mut().zip(pivot_row).skip(k) {
                *value -= factor * pivot;
            }
            for (value, pivot) in rhs.iter_mut().zip(pivot_rhs) {
                *value -= factor * pivot;
            }
        }
    }
    for k in (0..n).rev() {
        let (b_open, b_solved) = b.split_at_mut(k + 1);
        let rhs = &mut b_open[k];
        for (coefficient, solved) in a[k][k + 1..].iter().zip(b_solved.iter()) {
            for (value, known) in rhs.iter_mut().zip(solved) {
                *value -= coefficient * known;
            }
        }
        for value in rhs.iter_mut() {
            *value /= a[k][k];
        }
    }
    b
}

/// `Y(jω) = M_cc − M_cs · M_ss⁻¹ · M_sc` with `M = G + jωC`, over the rows
/// and columns indexed by `outer` (circuit unknowns, or injections and the
/// noise input).
fn reduce(
    descriptor: &SmallSignalDescriptor,
    frequency: f64,
) -> BTreeMap<(DescriptorIndex, DescriptorIndex), Complex64> {
    let s = Complex64::new(0.0, std::f64::consts::TAU * frequency);
    let states = descriptor.states.len();
    let mut outer = BTreeSet::new();
    for entry in &descriptor.entries {
        for index in [entry.row, entry.col] {
            if !matches!(index, DescriptorIndex::State(_)) {
                outer.insert(index);
            }
        }
    }
    let outer: Vec<_> = outer.into_iter().collect();
    let position = |index: DescriptorIndex| outer.iter().position(|&other| other == index);
    let zero = Complex64::new(0.0, 0.0);
    let mut cc = vec![vec![zero; outer.len()]; outer.len()];
    let mut cs = vec![vec![zero; states]; outer.len()];
    let mut sc = vec![vec![zero; outer.len()]; states];
    let mut ss = vec![vec![zero; states]; states];
    for entry in &descriptor.entries {
        let value = Complex64::new(entry.g, 0.0) + s * entry.c;
        match (entry.row, entry.col) {
            (DescriptorIndex::State(i), DescriptorIndex::State(j)) => ss[i][j] += value,
            (DescriptorIndex::State(i), col) => sc[i][position(col).unwrap()] += value,
            (row, DescriptorIndex::State(j)) => cs[position(row).unwrap()][j] += value,
            (row, col) => cc[position(row).unwrap()][position(col).unwrap()] += value,
        }
    }
    let x = if states == 0 {
        Vec::new()
    } else {
        solve(ss, sc)
    };
    let mut reduced = BTreeMap::new();
    for (i, &row) in outer.iter().enumerate() {
        for (j, &col) in outer.iter().enumerate() {
            let mut value = cc[i][j];
            for (k, x_row) in x.iter().enumerate() {
                value -= cs[i][k] * x_row[j];
            }
            if value != zero {
                reduced.insert((row, col), value);
            }
        }
    }
    reduced
}

fn circuit(reduced: BTreeMap<(DescriptorIndex, DescriptorIndex), Complex64>) -> Admittance {
    reduced
        .into_iter()
        .map(|((row, col), value)| match (row, col) {
            (DescriptorIndex::Circuit(row), DescriptorIndex::Circuit(col)) => ((row, col), value),
            other => panic!("a circuit descriptor reduced to {other:?}"),
        })
        .collect()
}

/// The worst entry-wise relative difference, over every entry either side
/// has.
fn worst_relative_error<K: Ord + Copy + std::fmt::Debug>(
    expected: &BTreeMap<K, Complex64>,
    actual: &BTreeMap<K, Complex64>,
) -> f64 {
    let keys: BTreeSet<K> = expected.keys().chain(actual.keys()).copied().collect();
    keys.into_iter()
        .map(|key| {
            let a = expected.get(&key).copied().unwrap_or_default();
            let b = actual.get(&key).copied().unwrap_or_default();
            let scale = a.norm().max(b.norm());
            if scale == 0.0 {
                0.0
            } else {
                (a - b).norm() / scale
            }
        })
        .fold(0.0, f64::max)
}

/// Reduce the descriptor at every test frequency and hold it to the complex
/// stamp, and its state count to `order`. Returns the descriptor.
fn self_check(label: &str, fixture: &Fixture, order: usize) -> SmallSignalDescriptor {
    let mut device = fixture.device(1);
    let descriptor = device
        .try_small_signal_descriptor(fixture.bias)
        .unwrap_or_else(|error| panic!("{label}: descriptor refused: {error}"));
    assert_eq!(
        descriptor.states.len(),
        order,
        "{label}: state count {:?}",
        descriptor.states
    );
    let mut worst = 0.0_f64;
    for frequency in FREQUENCIES {
        let expected = complex_admittance(&mut device, fixture.bias, frequency);
        let actual = circuit(reduce(&descriptor, frequency));
        let error = worst_relative_error(&expected, &actual);
        assert!(
            error <= TOLERANCE,
            "{label} at {frequency} Hz: relative error {error:e}\n complex {expected:?}\n descriptor {actual:?}\n {descriptor:#?}"
        );
        worst = worst.max(error);
    }
    eprintln!("{label}: {order} states, worst relative error {worst:e}");
    // A second evaluation is the same descriptor: nothing advanced.
    assert_eq!(
        device.try_small_signal_descriptor(fixture.bias).unwrap(),
        descriptor,
        "{label}: descriptor is not a pure function of the bias"
    );
    descriptor
}

#[test]
fn laplace_nd_reduces_to_the_complex_stamp() {
    self_check(
        "laplace_nd",
        &Fixture::two_terminal(
            "I(a,b) <+ V(a,b)/2 + laplace_nd(V(a,b), '{1.0, 0.5}, '{1.0, 0.25, 0.01});",
        ),
        2,
    );
}

#[test]
fn laplace_np_reduces_to_the_complex_stamp() {
    self_check(
        "laplace_np",
        &Fixture::two_terminal("I(a,b) <+ laplace_np(V(a,b), '{1.0, 0.5}, '{-4.0, 0.0});"),
        1,
    );
}

#[test]
fn laplace_zd_reduces_to_the_complex_stamp() {
    self_check(
        "laplace_zd",
        &Fixture::two_terminal("I(a,b) <+ laplace_zd(V(a,b), '{-2.0, 0.0}, '{1.0, 0.25});"),
        1,
    );
}

#[test]
fn laplace_zp_reduces_to_the_complex_stamp() {
    self_check(
        "laplace_zp",
        &Fixture::two_terminal(
            "I(a,b) <+ laplace_zp(V(a,b), '{-1.0, 1.0, -1.0, -1.0}, '{-2.0, 2.0, -2.0, -2.0});",
        ),
        2,
    );
}

#[test]
fn idt_reduces_to_the_complex_stamp() {
    self_check(
        "idt",
        &Fixture::two_terminal("I(a,b) <+ V(a,b)/2 + idt(V(a,b), 0.5);"),
        1,
    );
}

#[test]
fn laplace_of_laplace_reduces_to_the_complex_stamp() {
    self_check(
        "laplace(laplace(V))",
        &Fixture::two_terminal(
            "I(a,b) <+ laplace_nd(laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1}), '{1.0, 0.5}, '{1.0, 0.25});",
        ),
        2,
    );
}

/// The direct nesting is interpreter-only: the native JIT refuses a
/// `laplace` operand of `ddt` when it builds the device, before any
/// small-signal question is asked. The same composition through a variable
/// is native-buildable and runs on both routes.
#[test]
fn ddt_of_laplace_reduces_to_the_complex_stamp() {
    #[cfg(not(feature = "native"))]
    self_check(
        "ddt(laplace(V))",
        &Fixture::two_terminal("I(a,b) <+ V(a,b) + ddt(laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1}));"),
        1,
    );
    self_check(
        "ddt(y), y = laplace(V)",
        &Fixture::two_terminal(
            "y = laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1}); I(a,b) <+ V(a,b) + ddt(y);",
        ),
        1,
    );
}

#[test]
fn laplace_of_ddt_reduces_to_the_complex_stamp() {
    self_check(
        "laplace(ddt(V))",
        &Fixture::two_terminal("I(a,b) <+ V(a,b) + laplace_nd(ddt(V(a,b)), '{1.0}, '{1.0, 0.1});"),
        1,
    );
}

#[test]
fn ddt_of_ddt_reduces_to_the_complex_stamp() {
    self_check(
        "ddt(ddt(V))",
        &Fixture::two_terminal("I(a,b) <+ V(a,b) + 0.01*ddt(ddt(V(a,b)));"),
        1,
    );
}

#[test]
fn state_kinds_name_the_operator_each_state_realizes() {
    let kinds = |body| self_check(body, &Fixture::two_terminal(body), 1).states;
    assert_eq!(
        kinds("I(a,b) <+ laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1});"),
        [DescriptorStateKind::Laplace { filter: 0 }]
    );
    assert!(matches!(
        kinds("I(a,b) <+ V(a,b) + idt(V(a,b), 0.0);")[..],
        [DescriptorStateKind::Idt { .. }]
    ));
    assert_eq!(
        kinds("I(a,b) <+ V(a,b) + ddt(ddt(V(a,b)));"),
        [DescriptorStateKind::Auxiliary]
    );
    // A ddt whose input carries no s is the C matrix, not a state.
    self_check(
        "ddt(V)",
        &Fixture::two_terminal("I(a,b) <+ V(a,b) + ddt(2*V(a,b));"),
        0,
    );
    // An operator no unknown drives has a zero small-signal response, and
    // no state: a kept integrator would put a false pole at s = 0.
    self_check(
        "operators of constants",
        &Fixture::two_terminal(
            "I(a,b) <+ V(a,b) + idt(1.0, 0.0) + laplace_nd(2.0, '{1.0}, '{1.0, 0.1});",
        ),
        0,
    );
    // An idt without an initial condition is already a circuit unknown: the
    // front end gives it an implicit integrator node, whose C entry carries
    // the pole.
    self_check(
        "implicit idt(V)",
        &Fixture {
            source: module("a, b", "", "I(a,b) <+ V(a,b) + idt(V(a,b));"),
            nodes: &[1, 2],
            internal: &[3],
            branches: &[],
            bias: &[0.25, -0.5, 0.125],
        },
        0,
    );
}

/// Cuts reached through assignments take their column from the derivative
/// shadow they compute; bias-dependent gains, internal nodes and branch
/// currents enter with the complex stamp's multiplicity and signs.
#[test]
fn assigned_nested_and_bias_dependent_operators_reduce_to_the_complex_stamp() {
    for (label, body, order) in [
        (
            "assigned bias-dependent laplace",
            "y = laplace_nd(V(a,b)*V(a,b), '{1.0, 0.5}, '{1.0, 0.25}); I(a,b) <+ y;",
            1,
        ),
        (
            "assigned idt",
            "q = idt(V(a,b)*V(a,b), 0.0); I(a,b) <+ V(a,b) + q/2;",
            1,
        ),
        (
            "assigned ddt(ddt)",
            "x = ddt(ddt(V(a,b))); I(a,b) <+ V(a,b) + 0.01*x;",
            1,
        ),
        (
            "ddt of an assigned ddt",
            "x = ddt(V(a,b)); I(a,b) <+ V(a,b) + 0.01*ddt(x);",
            1,
        ),
        (
            "one filter read twice",
            "y = laplace_nd(V(a), '{1.0}, '{1.0, 0.1}); I(a,b) <+ y; I(b) <+ 3*y*V(b);",
            1,
        ),
        (
            "filter weighted by a bias",
            "I(a,b) <+ V(b)*laplace_nd(V(a), '{1.0, 0.2}, '{1.0, 0.1});",
            1,
        ),
    ] {
        self_check(label, &Fixture::two_terminal(body), order);
    }

    self_check(
        "laplace on an internal node",
        &Fixture {
            source: module(
                "a, b",
                "electrical x;",
                "I(x) <+ V(x) - laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1}); I(a,b) <+ V(x)/2;",
            ),
            nodes: &[1, 2],
            internal: &[3],
            branches: &[],
            bias: &[0.25, -0.5, 0.75],
        },
        1,
    );
    self_check(
        "laplace driving a potential contribution",
        &Fixture {
            source: module(
                "i, o",
                "",
                "V(o) <+ laplace_nd(V(i), '{2.0}, '{1.0, 0.1}) + 0.5*idt(I(o), 0.0);",
            ),
            nodes: &[1, 2],
            internal: &[],
            branches: &[3],
            bias: &[0.25, 0.5, 1e-3],
        },
        2,
    );
}

#[test]
fn non_rational_operators_are_named_blockers_and_refused() {
    for (operator, body) in [
        ("absdelay", "I(a,b) <+ V(a,b) + absdelay(V(a,b), 1e-3);"),
        (
            "zi_*",
            "I(a,b) <+ V(a,b) + zi_zp(V(a,b), '{0.0, 0.0}, '{0.5, 0.0}, 1.0e-6);",
        ),
        ("idtmod", "I(a,b) <+ V(a,b) + idtmod(V(a,b), 0.0, 1.0);"),
    ] {
        let fixture = Fixture::two_terminal(body);
        let mut device = fixture.device(1);
        assert_eq!(device.small_signal_descriptor_blockers(), [operator]);
        assert_eq!(
            device.try_small_signal_descriptor(fixture.bias),
            Err(VmError::NonRationalOperator(operator)),
            "{body}"
        );
        // The complex evaluator still answers these at one frequency.
        complex_admittance(&mut device, fixture.bias, 1.0);
    }

    let rational = Fixture::two_terminal(
        "I(a,b) <+ laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1}) + idt(V(a,b), 0.0) + ddt(V(a,b));",
    );
    let device = rational.device(1);
    assert!(device.small_signal_descriptor_blockers().is_empty());
    assert_eq!(device.non_rational_analog_operators(), ["laplace", "idt"]);
}

#[test]
fn a_descriptor_needs_a_small_signal_analysis() {
    let fixture = Fixture::two_terminal("I(a,b) <+ laplace_nd(V(a,b), '{1.0}, '{1.0, 0.1});");
    let mut device = fixture.device(0);
    assert!(matches!(
        device.try_small_signal_descriptor(fixture.bias),
        Err(VmError::InvalidRuntimeConfiguration(_))
    ));
}

/// Reduce each realization at every test frequency and hold its gains to
/// `try_noise_processes_at_frequency`. Returns the realizations.
fn noise_self_check(
    label: &str,
    fixture: &Fixture,
    orders: &[usize],
) -> Vec<rspice_veriloga::vm::NoiseProcessRealization> {
    let mut device = fixture.device(3);
    let realizations = device
        .try_noise_process_realizations(fixture.bias)
        .unwrap_or_else(|error| panic!("{label}: realization refused: {error}"));
    assert_eq!(
        realizations
            .iter()
            .map(|realization| realization.descriptor.states.len())
            .collect::<Vec<_>>(),
        orders,
        "{label}: {realizations:#?}"
    );
    let mut worst = 0.0_f64;
    for frequency in FREQUENCIES {
        let processes = device
            .try_noise_processes_at_frequency(fixture.bias, frequency)
            .expect("per-frequency noise");
        assert_eq!(
            processes
                .iter()
                .map(|process| process.process_id)
                .collect::<Vec<_>>(),
            realizations
                .iter()
                .map(|realization| realization.process_id)
                .collect::<Vec<_>>(),
            "{label}: the realizations cover the processes the noise analysis reports"
        );
        for (process, realization) in processes.iter().zip(&realizations) {
            assert_eq!(
                process
                    .injections
                    .iter()
                    .map(|injection| (injection.node_pos, injection.node_neg))
                    .collect::<Vec<_>>(),
                realization.injections,
                "{label}"
            );
            let expected: BTreeMap<usize, Complex64> = process
                .injections
                .iter()
                .map(|injection| injection.gain)
                .enumerate()
                .filter(|(_, gain)| *gain != Complex64::default())
                .collect();
            let actual: BTreeMap<usize, Complex64> = reduce(&realization.descriptor, frequency)
                .into_iter()
                .map(|((row, col), gain)| match (row, col) {
                    (DescriptorIndex::Circuit(injection), DescriptorIndex::NoiseInput) => {
                        (injection, gain)
                    }
                    other => panic!("{label}: a realization reduced to {other:?}"),
                })
                .collect();
            let error = worst_relative_error(&expected, &actual);
            assert!(
                error <= TOLERANCE,
                "{label} at {frequency} Hz: relative error {error:e}\n complex {expected:?}\n realization {actual:?}"
            );
            worst = worst.max(error);
        }
    }
    eprintln!("{label}: orders {orders:?}, worst relative error {worst:e}");
    realizations
}

#[test]
fn shaped_noise_realizations_reproduce_the_complex_noise_gains() {
    noise_self_check(
        "laplace_np(white_noise(1))",
        &Fixture::two_terminal(
            "I(a,b) <+ V(a,b)/1k + laplace_np(white_noise(1), '{1.0}, '{-1.0e1, 0.0});",
        ),
        &[1],
    );
    noise_self_check(
        "white noise read directly and through a filter",
        &Fixture::two_terminal(
            "y = white_noise(1.0, \"shared\"); I(a,b) <+ V(a,b) + y; I(a,b) <+ laplace_nd(y, '{1.0}, '{1.0, 0.1});",
        ),
        &[1],
    );
    noise_self_check(
        "idt-shaped and static processes",
        &Fixture::two_terminal(
            "I(a,b) <+ V(a,b) + idt(white_noise(2.0, \"integrated\"), 0.0) + white_noise(3.0, \"flat\");",
        ),
        &[1, 0],
    );
    noise_self_check(
        "ddt(ddt)-shaped process",
        &Fixture::two_terminal(
            "I(a,b) <+ V(a,b) + 0.01*ddt(ddt(laplace_nd(white_noise(1.0), '{1.0}, '{1.0, 0.1})));",
        ),
        &[2],
    );
}

/// What both routes must produce, bit for bit: every coefficient below is
/// exact in binary64, so a route that evaluates the operating point
/// differently, or orders the cut states differently, fails here.
#[test]
fn interpreter_and_native_routes_build_the_same_descriptor() {
    let entry = |row, col, g, c| DescriptorEntry { row, col, g, c };
    let (a, b, z) = (
        DescriptorIndex::Circuit(0),
        DescriptorIndex::Circuit(1),
        DescriptorIndex::State(0),
    );

    // (1 + s/2)/(1 + s/4) of V(a,b)² at V(a,b) = 0.75: the realization is
    // A = -4, B = 1, C = -4, D = 2, and the input slope is 1.5.
    let fixture = Fixture::two_terminal(
        "y = laplace_nd(V(a,b)*V(a,b), '{1.0, 0.5}, '{1.0, 0.25}); I(a,b) <+ y;",
    );
    let descriptor = fixture
        .device(1)
        .try_small_signal_descriptor(fixture.bias)
        .unwrap();
    assert_eq!(
        descriptor,
        SmallSignalDescriptor {
            states: vec![DescriptorStateKind::Laplace { filter: 0 }],
            entries: vec![
                entry(a, a, 3.0, 0.0),
                entry(a, b, -3.0, 0.0),
                entry(a, z, -4.0, 0.0),
                entry(b, a, -3.0, 0.0),
                entry(b, b, 3.0, 0.0),
                entry(b, z, 4.0, 0.0),
                entry(z, a, -1.5, 0.0),
                entry(z, b, 1.5, 0.0),
                entry(z, z, 4.0, 1.0),
            ],
        }
    );

    // s·u with u = s·V(a,b): one auxiliary state.
    let fixture = Fixture::two_terminal("I(a,b) <+ 0.5*ddt(ddt(V(a,b)));");
    let descriptor = fixture
        .device(1)
        .try_small_signal_descriptor(fixture.bias)
        .unwrap();
    assert_eq!(
        descriptor,
        SmallSignalDescriptor {
            states: vec![DescriptorStateKind::Auxiliary],
            entries: vec![
                entry(a, z, 0.0, 0.5),
                entry(b, z, 0.0, -0.5),
                entry(z, a, 0.0, -1.0),
                entry(z, b, 0.0, 1.0),
                entry(z, z, 1.0, 0.0),
            ],
        }
    );

    // 1/(1 + s/4) of a unit white source, injected against KCL.
    let fixture = Fixture::two_terminal(
        "I(a,b) <+ V(a,b) + laplace_nd(white_noise(1.0), '{1.0}, '{1.0, 0.25});",
    );
    let realizations = fixture
        .device(3)
        .try_noise_process_realizations(fixture.bias)
        .unwrap();
    assert_eq!(
        realizations,
        [rspice_veriloga::vm::NoiseProcessRealization {
            process_id: 0,
            descriptor: SmallSignalDescriptor {
                states: vec![DescriptorStateKind::Laplace { filter: 0 }],
                entries: vec![
                    entry(a, z, -4.0, 0.0),
                    entry(z, z, 4.0, 1.0),
                    entry(z, DescriptorIndex::NoiseInput, -1.0, 0.0),
                ],
            },
            injections: vec![(1, 2)],
        }]
    );
}
