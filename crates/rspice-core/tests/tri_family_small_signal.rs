//! Small-signal and truncation-error behaviour of a deck that carries more
//! than one device family.
//!
//! Three questions, none of which any other file in the tree asks:
//!
//! * does a Verilog-A module that calls `$bound_step` — a transient-only
//!   statement — still answer `.ac` and `.noise` the way the same module
//!   without it does?
//! * does a deck whose XSPICE event nets carry no small signal still assemble
//!   a finite `.ac` solution, with the event-driven nodes held at their DC
//!   level?
//! * does an authored `ddt` charge carry the same truncation-error authority a
//!   native capacitor does, so tightening `reltol` tightens the grid?
//!
//! Every `#[ignore]` reason starts with its repair lane, so counting the ignore
//! attributes whose reason opens with an `R` lane id across `tri_family_*.rs`
//! counts the open lanes; an ignored case is expected to FAIL under
//! `-- --ignored` until that lane lands.
//!
//! Pinned quantities print as `TRIFAMILY <key>=<value>`; set
//! `RSPICE_TRI_FAMILY_EMIT=1` and run with `--nocapture` to re-measure them all
//! in one pass without asserting any of them. They are self-agreement
//! tripwires, not oracles.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

const EMIT_ENV: &str = "RSPICE_TRI_FAMILY_EMIT";

/// `V(x)` of the small-signal divider at 1 MHz, for the resistive module and
/// for the module that also carries a charge.
const RESISTIVE_X: (f64, f64) = (0.5, 0.0);
const CHARGED_X: (f64, f64) = (0.5, -1.5708e-3);
/// Output noise density of the same deck at 1 MHz and 300.15 K, V^2/Hz.
const OUTPUT_NOISE: f64 = 4.144e-18;

/// Accepted transient points for a native 1 pF capacitor at each reltol.
const NATIVE_CAP_POINTS: [usize; 3] = [135, 217, 598];
const RELTOLS: [&str; 3] = ["1e-2", "1e-3", "1e-5"];

//=============================================================================
// Shared helpers
//=============================================================================

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ModelFile(std::path::PathBuf);

impl ModelFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_tri_family_small_signal_{}_{}.va",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).expect("write model file");
        Self(path)
    }

    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn emitting() -> bool {
    std::env::var_os(EMIT_ENV).is_some()
}

fn pin_f64(key: &str, observed: f64, expected: f64, tolerance: f64) {
    println!("TRIFAMILY {key}={observed:.6e}");
    if emitting() {
        return;
    }
    assert!(
        (observed - expected).abs() <= tolerance,
        "{key} is {observed:e}, pinned at {expected:e} +/- {tolerance:e}; \
         re-measure with {EMIT_ENV}=1"
    );
}

fn pin_usize(key: &str, observed: usize, expected: usize) {
    println!("TRIFAMILY {key}={observed}");
    if emitting() {
        return;
    }
    assert_eq!(
        observed, expected,
        "{key} moved off its pinned value; re-measure with {EMIT_ENV}=1"
    );
}

/// A one-port Verilog-A module whose analog body is supplied by the caller.
fn one_port(body: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         module m(p, n);\n\
         \x20inout p, n; electrical p, n;\n\
         \x20parameter real r = 1k from (0:inf);\n\
         \x20parameter real c = 1p from [0:inf);\n\
         \x20analog begin\n{body}\n\
         \x20end\n\
         endmodule\n"
    )
}

const RESISTIVE: &str = " I(p,n) <+ V(p,n)/r;";
const CHARGED: &str = " I(p,n) <+ V(p,n)/r + ddt(c*V(p,n));";
const BOUND_STEP: &str = "\n $bound_step(2n);";

/// A 1 kohm source resistance into the authored one-port, probed at `x`.
fn divider_deck(model: &ModelFile) -> Netlist {
    Netlist::parse(&format!(
        "* small signal across an authored one-port\n\
         vin in 0 dc 1 ac 1\n\
         rs in x 1k\n\
         xva x 0 m\n\
         .va \"{}\" m\n\
         .end\n",
        model.path()
    ))
    .expect("divider deck parses")
}

/// `V(x)` at 1 MHz and the output noise density at the same frequency.
fn small_signal(model: &ModelFile) -> Result<((f64, f64), f64), String> {
    let deck = divider_deck(model);
    let ac = Engine::default()
        .run_ac(&deck, &[1e6])
        .map_err(|error| error.to_string())?;
    let index = ac[0]
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("x"))
        .expect("node x is in the AC result");
    let voltage = ac[0].voltages[index];
    let noise = Engine::default()
        .run_noise(&deck, 2, &[1e6], 300.15)
        .map_err(|error| error.to_string())?;
    Ok(((voltage.re, voltage.im), noise[0].output_noise_density))
}

//=============================================================================
// $bound_step and the small-signal routes
//=============================================================================

#[test]
fn authored_one_ports_answer_ac_and_noise_without_a_step_bound() {
    for (label, body, expected) in [
        ("resistive", RESISTIVE, RESISTIVE_X),
        ("charged", CHARGED, CHARGED_X),
    ] {
        let model = ModelFile::new(&one_port(body));
        let ((re, im), noise) =
            small_signal(&model).unwrap_or_else(|error| panic!("{label}: {error}"));
        pin_f64(&format!("{label}_x_re"), re, expected.0, 1e-5);
        pin_f64(
            &format!("{label}_x_im"),
            im,
            expected.1,
            (expected.1.abs() * 1e-4).max(1e-12),
        );
        pin_f64(
            &format!("{label}_output_noise"),
            noise,
            OUTPUT_NOISE,
            OUTPUT_NOISE * 1e-3,
        );
    }
}

#[test]
fn a_step_bound_does_not_change_the_small_signal_answer() {
    for (label, body) in [("resistive", RESISTIVE), ("charged", CHARGED)] {
        let plain = ModelFile::new(&one_port(body));
        let bounded = ModelFile::new(&one_port(&format!("{body}{BOUND_STEP}")));
        let reference =
            small_signal(&plain).unwrap_or_else(|error| panic!("{label} without bound: {error}"));
        let bounded = small_signal(&bounded)
            .unwrap_or_else(|error| panic!("{label} with $bound_step: {error}"));
        assert!(
            (bounded.0.0 - reference.0.0).abs() < 1e-9
                && (bounded.0.1 - reference.0.1).abs() < 1e-12,
            "{label}: $bound_step is a transient statement and must not move the \
             small-signal answer; {:?} against {:?}",
            bounded.0,
            reference.0
        );
        assert!(
            (bounded.1 - reference.1).abs() < reference.1 * 1e-6,
            "{label}: $bound_step moved the output noise density from {:e} to {:e}",
            reference.1,
            bounded.1
        );
    }
}

//=============================================================================
// XSPICE event nets under .ac
//=============================================================================

/// One deck whose event-driven half carries no small signal, together with
/// the same analog half with every bridge deleted.
struct EventNetAcDeck {
    label: &'static str,
    deck: String,
    /// A D/A output node and the DC level it holds through the sweep.
    dac_output: Option<(&'static str, f64)>,
    /// An analog node a bridge observes, and the bridge-free deck whose
    /// small-signal answer at that node the bridge must not move. An
    /// `adc_bridge` input is a pure voltage sense with no input load, so the
    /// two answers are equal, not merely close.
    unloaded: (&'static str, String),
}

/// Three decks whose event-driven half carries no small signal at all: an
/// A/D bridge on its own, a D/A bridge holding a constant digital level, and
/// the two with an inverter between them.
fn event_net_ac_decks() -> Vec<EventNetAcDeck> {
    const RESISTIVE_HALF: &str = "* the analog half on its own\n\
                                  vin in 0 dc 3.3 ac 1\n\
                                  rs in x 1k\n\
                                  rl x 0 9k\n\
                                  .end\n";
    vec![
        EventNetAcDeck {
            label: "adc alone",
            deck: "* an A/D bridge with no consumer\n\
                   vin in 0 dc 3.3 ac 1\n\
                   rs in x 1k\n\
                   rl x 0 9k\n\
                   a_adc [x] [dx] adc\n\
                   .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
                   .end\n"
                .to_string(),
            dac_output: None,
            unloaded: ("x", RESISTIVE_HALF.to_string()),
        },
        EventNetAcDeck {
            label: "constant dac",
            deck: "* a D/A bridge holding a constant digital level\n\
                   vdig d0 0 dc 3.3\n\
                   a_adc [d0] [dq] adc\n\
                   .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
                   a_dac [dq] [y] dac\n\
                   .model dac dac_bridge(out_low=0 out_high=3.3)\n\
                   ry y 0 10k\n\
                   vin in 0 dc 0 ac 1\n\
                   rs in probe 1k\n\
                   rl probe 0 1k\n\
                   .end\n"
                .to_string(),
            // The XSPICE `dac_bridge` drives an ideal voltage output through
            // its own branch unknown, so `ry` draws no divider drop and `y`
            // sits exactly on `out_high`.
            dac_output: Some(("y", 3.3)),
            unloaded: (
                "probe",
                "* the untouched analog island on its own\n\
                 vin in 0 dc 0 ac 1\n\
                 rs in probe 1k\n\
                 rl probe 0 1k\n\
                 .end\n"
                    .to_string(),
            ),
        },
        EventNetAcDeck {
            label: "adc through an inverter into a dac",
            deck: "* an event path from an analog node back to one\n\
                   vin in 0 dc 3.3 ac 1\n\
                   rs in x 1k\n\
                   rl x 0 9k\n\
                   a_adc [x] [d1] adc\n\
                   .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
                   a_inv d1 d2 inv\n\
                   .model inv d_inverter\n\
                   a_dac [d2] [y] dac\n\
                   .model dac dac_bridge(out_low=0 out_high=3.3)\n\
                   ry y 0 10k\n\
                   .end\n"
                .to_string(),
            dac_output: Some(("y", 0.0)),
            unloaded: ("x", RESISTIVE_HALF.to_string()),
        },
    ]
}

/// `V(node)` at 1 MHz.
fn ac_response(label: &str, deck: &str, node: &str) -> num_complex::Complex64 {
    let netlist = Netlist::parse(deck).expect("event net deck parses");
    let ac = Engine::default()
        .run_ac(&netlist, &[1e6])
        .unwrap_or_else(|error| panic!("{label}: {error}"));
    let index = ac[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("{label}: no node {node} in {:?}", ac[0].node_names));
    ac[0].voltages[index]
}

#[test]
fn event_driven_nodes_hold_their_dc_level_through_an_ac_sweep() {
    for EventNetAcDeck {
        label,
        deck,
        dac_output,
        unloaded,
    } in event_net_ac_decks()
    {
        let netlist = Netlist::parse(&deck).expect("event net deck parses");
        let ac = Engine::default()
            .run_ac(&netlist, &[1e6])
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert!(
            ac[0]
                .voltages
                .iter()
                .all(|voltage| voltage.re.is_finite() && voltage.im.is_finite()),
            "{label}: the small-signal solution has a non-finite entry: {:?}",
            ac[0].voltages
        );

        // The bridges observe the analog half without loading it.
        let (sensed, bridge_free) = unloaded;
        let observed = ac_response(label, &deck, sensed);
        let reference = ac_response(label, &bridge_free, sensed);
        assert!(
            (observed - reference).norm() < 1e-12,
            "{label}: a bridge must not load {sensed}; {observed:?} against {reference:?} \
             without the bridges"
        );

        let Some((node, dc_level)) = dac_output else {
            continue;
        };
        let index = ac[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("{label}: no node {node} in {:?}", ac[0].node_names));
        let response = ac[0].voltages[index];
        assert!(
            response.norm() < 1e-12,
            "{label}: an event-driven node carries no small signal, got {response:?}"
        );
        let operating_point = Engine::default()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{label} operating point: {error}"));
        let dc_index = operating_point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("{label}: no node {node} at the operating point"));
        let solved = operating_point.node_voltages[dc_index];
        assert!(
            (solved - dc_level).abs() < 1e-3,
            "{label}: {node} must hold its DC level {dc_level}, got {solved}"
        );
    }
}

//=============================================================================
// Truncation-error authority of an authored charge
//=============================================================================

/// The same RC step response with the capacitance supplied natively or by an
/// authored `ddt`, swept over three reltols.
fn charge_step_points(authored: Option<&ModelFile>) -> [usize; 3] {
    let mut points = [0usize; 3];
    for (slot, reltol) in points.iter_mut().zip(RELTOLS) {
        let (element, include) = match authored {
            Some(model) => (
                "x1 out 0 m r=1e12 c=1p".to_string(),
                format!(".va \"{}\" m\n", model.path()),
            ),
            None => ("c1 out 0 1p".to_string(), String::new()),
        };
        let deck = format!(
            "* truncation error of a 1 pF charge\n\
             .options reltol={reltol}\n\
             v1 in 0 pulse(0 1 0 1n 1n 10n 20n)\n\
             r1 in out 1k\n\
             {element}\n\
             {include}.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("charge deck parses");
        *slot = Engine::default()
            .run_tran(&netlist, 100e-9, 5e-9)
            .unwrap_or_else(|error| panic!("reltol {reltol}: {error}"))
            .time
            .len();
    }
    points
}

#[test]
fn a_native_capacitor_tightens_its_grid_as_reltol_tightens() {
    let points = charge_step_points(None);
    for (index, reltol) in RELTOLS.iter().enumerate() {
        pin_usize(
            &format!("native_cap_points_reltol_{reltol}"),
            points[index],
            NATIVE_CAP_POINTS[index],
        );
    }
    assert!(
        points[0] < points[1] && points[1] < points[2],
        "a tighter reltol must accept more points, got {points:?}"
    );
}

#[test]
fn an_authored_charge_carries_the_same_truncation_authority_as_a_native_one() {
    let model = ModelFile::new(&one_port(CHARGED));
    let authored = charge_step_points(Some(&model));
    let native = charge_step_points(None);
    assert!(
        authored[0] < authored[1] && authored[1] < authored[2],
        "a tighter reltol must accept more points, got {authored:?}"
    );
    for (index, reltol) in RELTOLS.iter().enumerate() {
        let (a, n) = (authored[index] as f64, native[index] as f64);
        assert!(
            (a - n).abs() <= 0.2 * n,
            "reltol {reltol}: an authored ddt charge must control the step like a \
             native capacitor; {} points against {}",
            authored[index],
            native[index]
        );
    }
}
