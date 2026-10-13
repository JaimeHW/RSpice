//! The resistance law of the native voltage (`S`) and current (`W`)
//! switches against their reference simulators.
//!
//! A SPICE3 `SW`/`CSW` card, and PSpice's threshold-form `VSWITCH`/`ISWITCH`
//! card that ngspice translates into one, is an ideal switch: exactly `RON`
//! or `ROFF` by state (`sw/swload.c`, `csw/cswload.c`). `SMOOTH`, an RSpice
//! extension, is the only way to give it a transition. Under Xyce a `W` card
//! is an `S` switch on `I(Vname)` and follows Xyce's cubic law between `IOFF`
//! (default 0) and `ION` (default 1 mA), `N_DEV_SW.C`.
//!
//! The reference numbers are ngspice-46 (`print`, `numdgt=17`) and Xyce 7.10
//! (`.PRINT ... PRECISION=17`) output for the decks below.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

fn engine(dialect: SpiceDialect) -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: dialect,
        ..Default::default()
    })
}

fn branch_currents(deck: &str, dialect: SpiceDialect, sources: &[&str]) -> Vec<f64> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine(dialect)
        .run_dc_op(&netlist)
        .expect("operating point solves");
    sources
        .iter()
        .map(|source| {
            let index = op
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(source))
                .unwrap_or_else(|| panic!("branch {source} missing from {:?}", op.branch_names));
            op.branch_currents[index]
        })
        .collect()
}

/// Each switch sits between a 1 V source and an ammeter, so its current is
/// its conductance, untouched by any node shunt.
const AMMETER_DECK: &str = "\
* switch currents at far controls and at the threshold
v1 in 0 1
s1 in m1 c1 0 swv
vm1 m1 0 0
vc1 c1 0 0
s2 in m2 c2 0 swv
vm2 m2 0 0
vc2 c2 0 1
w3 in m3 vs3 csw
vm3 m3 0 0
is3 0 x3 dc -1m
vs3 x3 0 0
w4 in m4 vs4 csw
vm4 m4 0 0
is4 0 x4 dc 2m
vs4 x4 0 0
s5 in m5 c5 0 swd
vm5 m5 0 0
vc5 c5 0 -5
w6 in m6 vs6 cswd
vm6 m6 0 0
is6 0 x6 dc -5
vs6 x6 0 0
s7 in m7 c7 0 swv
vm7 m7 0 0
vc7 c7 0 0.5
w8 in m8 vs8 csw
vm8 m8 0 0
is8 0 x8 dc 0.5m
vs8 x8 0 0
s9 in m9 c9 0 swh
vm9 m9 0 0
vc9 c9 0 0.5
.model swv sw vt=0.5 ron=1 roff=1e12
.model csw csw it=0.5m ron=1 roff=1e12
.model swd sw vt=0.5 ron=3
.model cswd csw it=0.5 ron=3
.model swh sw vt=0.5 vh=0.1 ron=1 roff=1e12
.end
";

/// Far from its threshold an ideal switch is exactly `RON` or `ROFF`, bit for
/// bit what ngspice-46 gives. The smooth law RSpice used before put
/// 0.9988e12 Ω on an `S` card half a volt below its threshold and about
/// 2.7e11 Ω on a `W` card 1.5 mA below it, against `ROFF = 1e12`.
///
/// At a control exactly on a zero-width threshold ngspice takes its
/// inverted-band rule, which toggles a switch entering the band, so an off
/// switch there turns on (`s7`, `w8`); a positive band holds the off state
/// (`s9`).
#[test]
fn ideal_switch_conductance_is_exactly_ngspice46() {
    // ngspice-46: i(vm1..vm9) with numdgt=17; it prints `ROFF`'s current as
    // 9.99999999999999980e-13, the double nearest 1e-12.
    let ngspice = [
        1.0e-12, 1.0, 1.0e-12, 1.0, 1.0e-12, 1.0e-12, 1.0, 1.0, 1.0e-12,
    ];
    let sources = [
        "vm1", "vm2", "vm3", "vm4", "vm5", "vm6", "vm7", "vm8", "vm9",
    ];
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let currents = branch_currents(AMMETER_DECK, dialect, &sources);
        for ((source, current), expected) in sources.iter().zip(currents).zip(ngspice) {
            assert_eq!(
                current.to_bits(),
                f64::to_bits(expected),
                "{dialect:?} {source}: {current:.17e} A against ngspice-46 {expected:.17e} A"
            );
        }
    }
}

/// `SMOOTH` keeps the smooth log-resistance transition: on its threshold the
/// switch is the geometric mean of `RON` and `ROFF`.
#[test]
fn smooth_is_the_only_way_to_a_transition() {
    let deck = "\
* SMOOTH switches on their thresholds
v1 in 0 1
s1 in m1 c1 0 sws
vm1 m1 0 0
vc1 c1 0 0.5
w2 in m2 vs2 csws
vm2 m2 0 0
is2 0 x2 dc 0.5m
vs2 x2 0 0
.model sws sw vt=0.5 ron=1 roff=1e6 smooth=0.1
.model csws csw it=0.5m ron=1 roff=1e6 smooth=1m
.end
";
    for current in branch_currents(deck, SpiceDialect::Ngspice, &["vm1", "vm2"]) {
        assert!(
            (current - 1.0e-3).abs() <= 1.0e-12,
            "a SMOOTH switch on its threshold conducts 1/sqrt(RON*ROFF): {current:e} A"
        );
    }
}

/// A Xyce `ISWITCH` card naming no `ION`/`IOFF` follows Xyce's curve between
/// its defaults 0 and 1 mA. Xyce 7.10: `I(VM1) = 1.00000000000000024e-03`
/// (0.5 mA, mid-curve: `1/sqrt(RON*ROFF)`), `I(VM2) = 8.65964323360065698e-06`
/// (0.25 mA), `I(VM3) = 1` (2 mA, fully on).
#[test]
fn xyce_current_switch_without_a_curve_takes_xyce_defaults() {
    let deck = "\
Xyce W switch with no ION/IOFF on its ISWITCH card
v1 in 0 1
w1 in m1 vs1 csw
vm1 m1 0 0
is1 0 x1 dc 0.5m
vs1 x1 0 0
w2 in m2 vs2 csw
vm2 m2 0 0
is2 0 x2 dc 0.25m
vs2 x2 0 0
w3 in m3 vs3 csw
vm3 m3 0 0
is3 0 x3 dc 2m
vs3 x3 0 0
.model csw iswitch (ron=1 roff=1e6)
.end
";
    let currents = branch_currents(deck, SpiceDialect::Xyce, &["vm1", "vm2", "vm3"]);
    for (current, expected) in
        currents
            .into_iter()
            .zip([1.000_000_000_000_000_2e-3, 8.659_643_233_600_657e-6, 1.0])
    {
        let relative = (current - expected).abs() / expected;
        assert!(
            relative <= 1.0e-12,
            "Xyce W current {current:.17e} A against Xyce 7.10 {expected:.17e} A"
        );
    }
}

fn transient_samples(
    deck: &str,
    tstop: f64,
    max_step: f64,
    nodes: &[&str],
    times: &[f64],
) -> Vec<Vec<f64>> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let tran = engine(SpiceDialect::Ngspice)
        .run_tran(&netlist, tstop, max_step)
        .expect("transient runs");
    let waves: Vec<&[f64]> = nodes
        .iter()
        .map(|node| {
            tran.try_voltage_waveform_named(node)
                .unwrap_or_else(|| panic!("node {node} missing"))
        })
        .collect();
    times
        .iter()
        .map(|&time| {
            let index = tran.time.partition_point(|&t| t < time).max(1);
            let (t0, t1) = (tran.time[index - 1], tran.time[index]);
            waves
                .iter()
                .map(|wave| {
                    wave[index - 1] + (wave[index] - wave[index - 1]) * (time - t0) / (t1 - t0)
                })
                .collect()
        })
        .collect()
}

/// The hysteresis switches of the ngspice manual's switch example (p. 92):
/// voltage switches starting on and off inside and below their band, and a
/// current switch. ngspice-46 and RSpice take the same 2514 points and cross
/// mid-level at the same instants; the samples are ngspice-46's,
/// interpolated from its output at these times.
#[test]
fn hysteresis_switches_follow_ngspice46_through_the_manual_example() {
    let deck = "\
Switch test (ngspice manual p92)
v1 1 0 DC 0.0 PWL (0 0 2e-3 2 4e-3 0)
v2 2 0 DC 0.0 PWL (0 0.9 2e-3 2 4e-3 0.4)
i3 3 0 DC 0.0 PWL (0 0 2e-3 2m 4e-3 0)
v4 4 0 DC 2.0
r3 3 33 10k
vm3 0 33 dc 0
r10 4 10 10k
r20 4 20 10k
r30 4 30 10k
r40 4 40 10k
s1 10 0 1 0 switch1 OFF
s2 20 0 2 0 switch1 OFF
s3 30 0 2 0 switch1 ON
.model switch1 sw vt=1 vh=0.2 ron=1 roff=10k
w1 40 0 vm3 wswitch1 off
.model wswitch1 csw it=1m ih=0.2m ron=1k roff=10k
.end
";
    let on = 0.000_199_980_001_999_8;
    let w_on = 0.181_818_181_818_181_8;
    let expected = [
        (0.5e-3, [1.0, 1.0, on, 1.0]),
        (1.0e-3, [1.0, on, on, 1.0]),
        (2.0e-3, [on, on, on, w_on]),
        (3.0e-3, [on, on, on, w_on]),
        (3.4e-3, [1.0, on, on, 1.0]),
        (4.5e-3, [1.0, 1.0, 1.0, 1.0]),
    ];
    let times: Vec<f64> = expected.iter().map(|(time, _)| *time).collect();
    let samples = transient_samples(deck, 5.0e-3, 2.0e-6, &["10", "20", "30", "40"], &times);
    for ((time, want), got) in expected.iter().zip(samples) {
        for ((node, want), got) in ["10", "20", "30", "40"].iter().zip(want).zip(got) {
            assert!(
                (got - want).abs() <= 1.0e-9 * want.abs().max(1.0e-3),
                "v({node}) at {time:e} s: {got:.12e} against ngspice-46 {want:.12e}"
            );
        }
    }
}

/// A default `VT = 0` switch whose control starts exactly at 0 V: ngspice-46
/// turns both on at the operating point, and after each pulse a switch whose
/// control returns to exactly 0 V takes the state opposite to the one it left
/// (`s1` on before, off after; `s2` off during its negative pulse, on after).
#[test]
fn zero_width_threshold_toggles_like_ngspice46() {
    let deck = "\
switch with default VT=0 driven from exactly 0 V
v1 in 0 1
s1 in a c1 0 swv
r1 a 0 1k
vc1 c1 0 pulse(0 1 1u 1n 1n 1u 4u)
s2 in b c2 0 swv
r2 b 0 1k
vc2 c2 0 pulse(0 -1 1u 1n 1n 1u 4u)
.model swv sw ron=1 roff=1meg
.end
";
    let on = 0.999_000_999_000_999_1;
    let off = 0.000_999_000_999_000_999;
    let expected = [(0.5e-6, [on, on]), (1.5e-6, [on, off]), (3.0e-6, [off, on])];
    let times: Vec<f64> = expected.iter().map(|(time, _)| *time).collect();
    let samples = transient_samples(deck, 4.0e-6, 1.0e-8, &["a", "b"], &times);
    for ((time, want), got) in expected.iter().zip(samples) {
        for ((node, want), got) in ["a", "b"].iter().zip(want).zip(got) {
            assert!(
                (got - want).abs() <= 1.0e-9,
                "v({node}) at {time:e} s: {got:.12e} against ngspice-46 {want:.12e}"
            );
        }
    }
}
