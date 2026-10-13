//! The B3SOIDD stamps must be the derivative of the residual they stamp.
//!
//! The residual of one instance is its share of the assembled transient
//! system, `r(x) = A(x)·x - b(x)`: the static stamp plus the charge companion
//! with backward-Euler gain `ag0` against a frozen history,
//! `cq = ag0·(q(x) - q_ref)`. Newton contracts quadratically only when the
//! stamped `A(x)` is `dr/dx`, so every stamped entry is compared against a
//! difference quotient of `r` itself.

use super::eval::B3SoiDdCharge;
use super::{B3SoiDd, SoiCompanionCurrents};
use crate::device::traits::{MatrixStamper, NonlinearDevice};
use crate::{Engine, Netlist, NodeId, Value};

const N1_CARD: &str = include_str!("../../../../../../../tests/ngspice/bsim3soidd/nmosdd.mod");
const P1_CARD: &str = include_str!("../../../../../../../tests/ngspice/bsim3soidd/pmosdd.mod");

/// Dense capture of one instance's stamps over every circuit node.
struct Assembly {
    n: usize,
    a: Vec<Value>,
    b: Vec<Value>,
}

impl Assembly {
    fn new(n: usize) -> Self {
        Self {
            n,
            a: vec![0.0; n * n],
            b: vec![0.0; n],
        }
    }

    fn entry(&self, row: NodeId, col: NodeId) -> Value {
        self.a[(row - 1) * self.n + (col - 1)]
    }

    /// `r_i = sum_k A_ik x_k - b_i` and the magnitude of the terms summed,
    /// `sum_k |A_ik x_k| + |b_i|`, which bounds the rounding in `r_i`.
    fn residual(&self, x: &[Value], row: NodeId) -> (Value, Value) {
        let i = row - 1;
        let mut sum = -self.b[i];
        let mut scale = self.b[i].abs();
        for (k, xk) in x.iter().enumerate() {
            let term = self.a[i * self.n + k] * xk;
            sum += term;
            scale += term.abs();
        }
        (sum, scale)
    }
}

impl MatrixStamper for Assembly {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        self.a[(row - 1) * self.n + (col - 1)] += value;
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        self.b[index - 1] += value;
    }
}

/// Linearize the instance exactly at `x` (no Newton limiting) and capture its
/// static and charge-companion stamps.
fn assemble(dev: &mut B3SoiDd, x: &[Value], ag0: Value, q_ref: &B3SoiDdCharge) -> Assembly {
    dev.update_static_linearization(x);
    let mut assembly = Assembly::new(x.len());
    dev.stamp_nonlinear(x, &mut assembly, &mut []);
    if ag0 > 0.0 {
        let q = dev.charge_at(x);
        let cq = |now: Value, then: Value| ag0 * (now - then);
        dev.stamp_charge_companion(
            &q,
            ag0,
            SoiCompanionCurrents {
                cqg: cq(q.qg, q_ref.qg),
                cqb: cq(q.qb, q_ref.qb),
                cqd: cq(q.qd, q_ref.qd),
                cqe: cq(q.qe, q_ref.qe),
                cqth: cq(q.qth, q_ref.qth),
            },
            x,
            &mut assembly,
        );
    }
    assembly
}

/// One bias point: absolute node voltages `[d, g, s, e, body]`. A body
/// contact sits at the body voltage, so no current flows in a tie resistor.
struct BiasPoint(&'static str, [Value; 5]);

struct Instance {
    dev: B3SoiDd,
    n: usize,
    /// Device nodes (row/column labels) in stamp order.
    nodes: Vec<(&'static str, NodeId)>,
}

/// Build one instance from a model card: a floating body (`d g s e`), an
/// ideal body tie (`d g s e p`), or a tie through `RBODY`.
fn instance(card: &str, polarity: &str, body: &str) -> Instance {
    let (terminals, card) = match body {
        "floating" => ("d g s e", card.to_string()),
        "tied" => ("d g s e p", card.to_string()),
        "resistive" => ("d g s e p", card.replace("RBODY = 0.0", "RBODY = 200.0")),
        other => panic!("unknown body case {other}"),
    };
    let model = if polarity == "n" { "n1" } else { "p1" };
    let deck = format!(
        "* B3SOIDD derivative check\n\
         vd d 0 0\nvg g 0 0\nvs s 0 0\nve e 0 0\nvp p 0 0\n\
         m1 {terminals} {model} w=10u l=0.25u\n{card}\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("derivative-check deck parses");
    let mut circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("derivative-check circuit builds");
    let n = circuit.num_nodes();
    let dev = circuit.b3soi.devices.remove(0);
    dev.set_dc_mode(false);
    let mut nodes = vec![
        ("D", dev.node_drain),
        ("G", dev.node_gate),
        ("S", dev.node_source),
        ("E", dev.node_e),
        ("B", dev.node_body),
    ];
    if body == "resistive" {
        nodes.push(("P", dev.node_p));
    }
    Instance { dev, n, nodes }
}

fn solution(inst: &Instance, point: &BiasPoint) -> Vec<Value> {
    let [vd, vg, vs, ve, vb] = point.1;
    let dev = &inst.dev;
    let mut x = vec![0.0; inst.n];
    for (node, volts) in [
        (dev.node_drain, vd),
        (dev.node_gate, vg),
        (dev.node_source, vs),
        (dev.node_e, ve),
        (dev.node_p, vb),
        (dev.node_body, vb),
    ] {
        if node != 0 {
            x[node - 1] = volts;
        }
    }
    x
}

struct Mismatch {
    row: &'static str,
    col: &'static str,
    stamped: Value,
    fd: Value,
    tol: Value,
}

/// Difference step in volts.
const H: Value = 1.0e-6;
/// Floor on the relative rounding of one evaluation of a residual row,
/// against the magnitude of the terms it sums: 2^-46, 64 ulps.
const EPS_EVAL: Value = 1.421_085_471_520_200_4e-14;

thread_local! {
    /// `tol / |stamped|` of every non-negligible entry checked on this thread,
    /// reported so a passing run shows how tight the check was.
    static RELATIVE_TOLERANCES: std::cell::RefCell<Vec<Value>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Compare every stamped entry against a difference quotient `D(h)` of the
/// residual.
///
/// The quotient is central, `(r(x+h) - r(x-h)) / 2h`, except for the drain and
/// source columns of a device at (or within the probe of) vds = 0. There the
/// normal/inverse mode select makes the residual a function of
/// `min(vd, vs)`-referenced voltages with a kink, so the stamp is the
/// derivative of the mode it selected, and the quotient is the second-order
/// one-sided `(-3r(x) + 4r(x+sh) - r(x+2sh)) / 2h` stepping into that mode.
/// At vds exactly 0 the reference also switches the parasitic-BJT currents
/// off (`Vds == 0.0`, b3soiddld.c:2393), so the static residual jumps there
/// and has no drain or source derivative; those two columns check the charge
/// companion alone, `A(ag0) - A(0)` against the quotient of `r(ag0) - r(0)`.
///
/// The tolerance is the error of the quotient itself:
/// - truncation `|D(h) - D(2h)|`, three times the leading `h^2` term;
/// - rounding: the evaluation noise of the residual, measured as the fourth
///   difference `|r(-2h) - 4r(-h) + 6r(0) - 4r(h) + r(2h)|` of the five
///   samples (whose own `h^4 r''''` is far below it), floored at
///   `EPS_EVAL * S_i`, `S_i` the summed term magnitude of row `i`; divided by
///   `h`, and by `h/4` for the larger one-sided weights;
/// - the stamped entry's own rounding, `EPS_EVAL` times its row's largest entry.
fn check(inst: &mut Instance, x: &[Value], ag0: Value) -> Vec<Mismatch> {
    let base_q = {
        inst.dev.update_static_linearization(x);
        inst.dev.charge_at(x)
    };
    let base = assemble(&mut inst.dev, x, ag0, &base_q);
    let base_static = assemble(&mut inst.dev, x, 0.0, &base_q);
    let mtype = inst.dev.mtype;
    let vds = mtype * (x[inst.dev.node_drain - 1] - x[inst.dev.node_source - 1]);
    // The terminal step that keeps the device in its selected mode.
    let into_mode = if vds >= 0.0 { mtype } else { -mtype };
    let nodes = inst.nodes.clone();
    let mut out = Vec::new();
    for &(col_label, col) in &nodes {
        if col == 0 {
            continue;
        }
        let drain_or_source = col == inst.dev.node_drain || col == inst.dev.node_source;
        let dynamic_only = vds == 0.0 && drain_or_source;
        if dynamic_only && ag0 == 0.0 {
            continue;
        }
        let side = if vds.abs() > 4.0 * H || !drain_or_source {
            None
        } else if col == inst.dev.node_drain {
            Some(into_mode)
        } else {
            Some(-into_mode)
        };
        let mut probe = |delta: Value| {
            let mut xp = x.to_vec();
            xp[col - 1] += delta;
            let full = assemble(&mut inst.dev, &xp, ag0, &base_q);
            let static_part = dynamic_only.then(|| assemble(&mut inst.dev, &xp, 0.0, &base_q));
            nodes
                .iter()
                .map(|&(_, row)| {
                    if row == 0 {
                        return (0.0, 0.0);
                    }
                    let (r, s) = full.residual(&xp, row);
                    match &static_part {
                        Some(asm) => {
                            let (r0, s0) = asm.residual(&xp, row);
                            (r - r0, s + s0)
                        }
                        None => (r, s),
                    }
                })
                .collect::<Vec<_>>()
        };
        // Five samples per column: centred on x, or stepping into the mode.
        let (offsets, one_sided) = match side {
            None => ([-2.0, -1.0, 0.0, 1.0, 2.0].map(|k| k * H), false),
            Some(s) => ([0.0, 1.0, 2.0, 3.0, 4.0].map(|k| k * s * H), true),
        };
        let samples = offsets.map(&mut probe);
        for (r, &(row_label, row)) in nodes.iter().enumerate() {
            if row == 0 {
                continue;
            }
            let f = samples.each_ref().map(|sample| sample[r].0);
            let scale = samples
                .iter()
                .map(|sample| sample[r].1)
                .fold(0.0, Value::max);
            let noise = (f[0] - 4.0 * f[1] + 6.0 * f[2] - 4.0 * f[3] + f[4])
                .abs()
                .max(EPS_EVAL * scale);
            let (d1, d2, rounding) = if one_sided {
                let s = offsets[1] / H;
                (
                    s * (-3.0 * f[0] + 4.0 * f[1] - f[2]) / (2.0 * H),
                    s * (-3.0 * f[0] + 4.0 * f[2] - f[4]) / (4.0 * H),
                    4.0 * noise / H,
                )
            } else {
                (
                    (f[3] - f[1]) / (2.0 * H),
                    (f[4] - f[0]) / (4.0 * H),
                    noise / H,
                )
            };
            let row_max = nodes
                .iter()
                .filter(|(_, c)| *c != 0)
                .map(|&(_, c)| base.entry(row, c).abs())
                .fold(0.0, Value::max);
            let tol = (d1 - d2).abs() + rounding + EPS_EVAL * row_max;
            let stamped = if dynamic_only {
                base.entry(row, col) - base_static.entry(row, col)
            } else {
                base.entry(row, col)
            };
            if stamped.abs() > 1e-3 * row_max {
                RELATIVE_TOLERANCES.with(|t| t.borrow_mut().push(tol / stamped.abs()));
            }
            if (stamped - d1).abs() > tol {
                out.push(Mismatch {
                    row: row_label,
                    col: col_label,
                    stamped,
                    fd: d1,
                    tol,
                });
            }
        }
    }
    // Leave the instance linearized at the base point.
    inst.dev.update_static_linearization(x);
    out
}

/// The operating point of b3soi_native's vds = 0 boundary deck: drain,
/// source and back gate at 1.25 V, gate at 0, floating body.
const BOUNDARY_DECK_OP: BiasPoint = BiasPoint(
    "vds=0 boundary deck op",
    [1.25, 0.0, 1.25, 1.25, 1.251_203_190_83],
);

/// `[d, g, s, e, body]` for the N1 card.
const N_POINTS: &[BiasPoint] = &[
    BOUNDARY_DECK_OP,
    BiasPoint("vds=0 gate off", [1.25, 0.0, 1.25, 1.25, 1.34]),
    BiasPoint("vds=0 gate on", [1.25, 2.5, 1.25, 1.25, 1.30]),
    BiasPoint("vds=+1mV gate off", [1.251, 0.0, 1.25, 1.25, 1.34]),
    BiasPoint("vds=-1mV gate off", [1.249, 0.0, 1.25, 1.25, 1.34]),
    BiasPoint("vds=+1mV gate on", [1.251, 2.5, 1.25, 1.25, 1.30]),
    BiasPoint("vds=-1mV gate on", [1.249, 2.5, 1.25, 1.25, 1.30]),
    BiasPoint("forward subthreshold", [1.5, 0.0, 0.0, 0.0, 0.09]),
    BiasPoint("forward moderate", [1.5, 0.6, 0.0, 0.0, 0.2]),
    BiasPoint("forward strong saturation", [1.5, 2.5, 0.0, 0.0, 0.4]),
    BiasPoint("forward strong linear", [0.1, 2.5, 0.0, 0.0, 0.05]),
    BiasPoint("reverse subthreshold", [0.0, 0.0, 1.5, 0.0, 0.09]),
    BiasPoint("reverse strong saturation", [0.0, 2.5, 1.5, 0.0, 0.4]),
    BiasPoint("reverse body bias", [1.5, 1.0, 0.0, 0.0, -0.3]),
];

/// `[d, g, s, e, body]` for the P1 card.
const P_POINTS: &[BiasPoint] = &[
    BiasPoint("vds=0 gate off", [1.25, 2.5, 1.25, 1.25, 1.16]),
    BiasPoint("vds=0 gate on", [1.25, 0.0, 1.25, 1.25, 1.20]),
    BiasPoint("forward subthreshold", [1.0, 2.5, 2.5, 1.25, 2.41]),
    BiasPoint("forward strong saturation", [1.0, 0.0, 2.5, 1.25, 2.3]),
    BiasPoint("reverse strong saturation", [2.5, 0.0, 1.0, 1.25, 2.3]),
];

/// Backward-Euler gains: the static stamp alone, then steps of 1 ns, 10 ps,
/// and the 0.17 ps first step of the vds = 0 boundary deck.
const AG0: &[(&str, Value)] = &[
    ("static", 0.0),
    ("dt=1e-9", 1.0e9),
    ("dt=1e-11", 1.0e11),
    ("dt=1.7e-13", 1.0 / 1.7e-13),
];

/// The card in the requested dialect: as written (ngspice LEVEL=56), or as
/// Xyce's LEVEL=10 SOIMOD=1, which routes the same card to the Xyce B3SOIDD.
fn dialect_card(card: &str, xyce: bool) -> String {
    if !xyce {
        return card.to_string();
    }
    let xyce_card = card.replace("Level= 56", "Level= 10 SOIMOD = 1");
    assert_ne!(xyce_card, card, "the card names LEVEL 56");
    xyce_card
}

fn sweep(
    family: &str,
    card: &str,
    polarity: &str,
    points: &[BiasPoint],
    xyce: bool,
) -> (usize, Vec<String>) {
    let mut checked = 0;
    let mut lines = Vec::new();
    let card = dialect_card(card, xyce);
    for capmod in ["3", "2"] {
        let card = card.replace("CAPMOD = 3", &format!("CAPMOD = {capmod}"));
        for body in ["floating", "tied", "resistive"] {
            let mut inst = instance(&card, polarity, body);
            for point in points {
                let x = solution(&inst, point);
                for &(dt_label, ag0) in AG0 {
                    checked += 1;
                    for m in check(&mut inst, &x, ag0) {
                        lines.push(format!(
                            "{family} capmod{capmod} {body:<9} {:<26} {dt_label:<10} dF[{}]/dx[{}]: \
                             stamped {:>13.6e} FD {:>13.6e} (err {:.2e}, tol {:.2e})",
                            point.0,
                            m.row,
                            m.col,
                            m.stamped,
                            m.fd,
                            (m.stamped - m.fd).abs(),
                            m.tol,
                        ));
                    }
                }
            }
        }
    }
    (checked, lines)
}

fn check_dialect(xyce: bool) {
    let sweeps = [
        sweep("nmos", N1_CARD, "n", N_POINTS, xyce),
        sweep("pmos", P1_CARD, "p", P_POINTS, xyce),
    ];
    let checked: usize = sweeps.iter().map(|(n, _)| n).sum();
    let lines: Vec<&str> = sweeps
        .iter()
        .flat_map(|(_, l)| l.iter().map(String::as_str))
        .collect();
    for line in &lines {
        println!("{line}");
    }
    let mut rel = RELATIVE_TOLERANCES.with(|t| t.take());
    rel.sort_by(Value::total_cmp);
    if let Some(&max) = rel.last() {
        let at = |q: Value| rel[((rel.len() - 1) as Value * q) as usize];
        println!(
            "{} entries; tolerance / |entry|: median {:.1e}, 90% {:.1e}, 99% {:.1e}, max {max:.1e}",
            rel.len(),
            at(0.5),
            at(0.9),
            at(0.99),
        );
    }
    println!("{} mismatched entries over {checked} checks", lines.len());
    assert!(
        lines.is_empty(),
        "{} stamped entries are not the derivative of the residual over {checked} checks:\n{}",
        lines.len(),
        lines.join("\n")
    );
}

#[test]
fn stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
    check_dialect(false);
}

/// The same check on Xyce's B3SOIDD (LEVEL=10 SOIMOD=1): its charges are
/// Xyce's, and its stamps their derivatives even where Xyce's own
/// capacitances leave terms out.
#[test]
fn xyce_stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
    check_dialect(true);
}

/// The boundary deck's transient stalled because its CAPMOD=3 charges carried
/// ~1e-8 relative rounding noise below threshold, which the integration gain
/// turns into nA of residual. Along every direction the charges must now be
/// smooth to rounding: second differences over 1 nV steps no larger than 64
/// ulps of the largest node charge.
#[test]
fn boundary_deck_charges_are_smooth_to_rounding() {
    let mut inst = instance(N1_CARD, "n", "floating");
    let x = solution(&inst, &BOUNDARY_DECK_OP);
    for (label, node) in [
        ("body", inst.dev.node_body),
        ("back gate", inst.dev.node_e),
        ("gate", inst.dev.node_gate),
    ] {
        let charges: Vec<[Value; 4]> = (-6..=6)
            .map(|k| {
                let mut xp = x.clone();
                xp[node - 1] += Value::from(k) * 1.0e-9;
                inst.dev.update_static_linearization(&xp);
                let q = inst.dev.charge_at(&xp);
                [q.qg, q.qb, q.qd, q.qe]
            })
            .collect();
        let largest = charges[6].iter().fold(0.0, |m: Value, q| m.max(q.abs()));
        for (charge, name) in ["qg", "qb", "qd", "qe"].iter().enumerate() {
            let roughness = charges
                .windows(3)
                .map(|w| (w[2][charge] - 2.0 * w[1][charge] + w[0][charge]).abs())
                .fold(0.0, Value::max);
            assert!(
                roughness <= EPS_EVAL * largest,
                "{name} along the {label}: second difference {roughness:.3e} C over 1 nV \
                 exceeds 64 ulps of {largest:.3e} C"
            );
        }
    }
}
