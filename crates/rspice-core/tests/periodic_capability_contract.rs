//! Deck-level behaviour of the periodic-analysis capability declarations.
//!
//! The declaration table itself is checked family by family in the engine's
//! unit tests. These cases prove the queries over it reach the analyses: a
//! family the table admits is not refused, a family it refuses is refused
//! before any solver work, and the refusal names the missing capability rather
//! than only the device.

use rspice_core::analysis::PssConfig;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("capability fixture deck parses")
}

fn hb_error(deck: &str) -> String {
    engine()
        .run_hb(&parse(deck), HbConfig::new(F0).with_harmonics(2))
        .expect_err("this deck must be refused before exact HB solving")
        .to_string()
}

/// Whether a message is one of the capability preflights rather than a solver
/// or convergence outcome. Admission tests assert the absence of these.
fn is_capability_rejection(message: &str) -> bool {
    message.contains("HB runtime does not yet support")
        || message.contains("MNA is unavailable")
        || message.contains("Pole-zero analysis does not yet support")
        || message.contains("PSS transient continuation is unavailable")
        || message.contains("HB Envelope continuation is unavailable")
        || message.contains("cyclostationary colored-noise folding")
}

fn assert_admitted<T>(outcome: Result<T, SimulationError>, what: &str) {
    if let Err(error) = outcome {
        let message = error.to_string();
        assert!(
            !is_capability_rejection(&message),
            "{what} is declared supported and must not hit a capability preflight: {message}"
        );
    }
}

#[test]
fn harmonic_balance_admits_every_family_the_table_declares_supported() {
    let cases = [
        (
            "linear R/L/C with independent and controlled sources",
            "\
* declared-complete linear families
vin in 0 sin(0 1 1meg)
r1 in mid 1k
l1 mid out 1u
c1 out 0 100p
e1 buf 0 out 0 2.0
g1 0 sink out 0 1m
rsink sink 0 1k
i1 0 mid dc 0
.end
",
        ),
        (
            "LEVEL=1 diode inside its declared restriction",
            "\
* declared-restricted diode, inside the restriction
vin in 0 sin(0 0.4 1meg)
r1 in a 1k
d1 a 0 dmod
.model dmod d(is=1e-14 cjo=1p)
.end
",
        ),
        (
            "LEVEL=1 MOSFET inside its declared restriction",
            "\
* declared-restricted classic MOS, inside the restriction
vd d 0 dc 1
vg g 0 sin(0 0.2 1meg)
m1 d g 0 0 mmod l=1u w=10u
.model mmod nmos (level=1 vto=0.5 kp=1m)
rl d 0 10k
.end
",
        ),
        (
            "non-hysteretic voltage switch inside its declared restriction",
            "\
* declared-restricted switch, inside the restriction
v1 out 0 sin(0 1 1meg)
vc ctrl 0 dc 1
s1 out mid ctrl 0 smod
r1 mid 0 1k
.model smod sw (vt=0 vh=0 ron=1 roff=1meg)
.end
",
        ),
    ];

    for (what, deck) in cases {
        let outcome = engine().run_hb(&parse(deck), HbConfig::new(F0).with_harmonics(2));
        assert_admitted(outcome, what);
    }
}

#[test]
fn harmonic_balance_names_the_missing_residual_capability() {
    let cases = [
        (
            "\
* a Gummel-Poon BJT has no exact periodic residual
vin in 0 sin(0 0.2 1meg)
r1 in b 10k
q1 c b 0 qmod
vcc c 0 dc 5
.model qmod npn (is=1e-16 bf=100)
.end
",
            "Gummel-Poon/VBIC equations are not represented by exact HB",
        ),
        (
            "\
* a hysteretic switch has no exact periodic state evolution
v1 out 0 sin(0 1 1meg)
vc ctrl 0 dc 0
s1 out 0 ctrl 0 smod
.model smod sw (vt=0 vh=0.1 ron=1 roff=1meg)
.end
",
            "requiring hysteresis or Xyce ON/OFF curve semantics",
        ),
        (
            "\
* a current-controlled switch needs its control-branch spectrum
iin 0 out dc 0
vctrl ctrl 0 dc 0
w1 out 0 vctrl csw
.model csw iswitch (ron=1 roff=1meg ion=1 ioff=0)
r1 out 0 1k
.end
",
            "current-controlled switches requiring exact control-branch current spectra",
        ),
    ];

    for (deck, expected) in cases {
        let message = hb_error(deck);
        assert!(
            message.contains("HB runtime does not yet support"),
            "the residual/Jacobian preflight must own this rejection: {message}"
        );
        assert!(
            message.contains(expected),
            "the rejection must name the missing capability '{expected}': {message}"
        );
    }
}

#[test]
fn harmonic_balance_names_the_missing_periodic_descriptor_capability() {
    let cases = [
        (
            "\
* a behavioral source has no exact periodic MNA descriptor
vin in 0 sin(0 1 1meg)
r1 in out 1k
b1 out 0 i={v(in)*1m}
r2 out 0 1k
.end
",
            "behavioral-source equations",
        ),
        (
            "\
* a solution-dependent capacitor needs a periodic charge linearization
iin 0 out dc 0
vctrl ctrl 0 dc 0.5
c1 out 0 C={1p*(1+V(ctrl))}
r1 out 0 1k
.end
",
            "solution-dependent capacitor charge linearizations",
        ),
    ];

    for (deck, expected) in cases {
        let message = hb_error(deck);
        assert!(
            message.contains("exact HB MNA is unavailable"),
            "the periodic descriptor preflight must own this rejection: {message}"
        );
        assert!(
            message.contains(expected),
            "the rejection must name the missing capability '{expected}': {message}"
        );
    }
}

#[test]
fn pss_continuation_state_follows_the_declared_period_map_capability() {
    let supported = "\
* only ordinary R/L/C companion history crosses the period map
vin in 0 sin(0 1 1meg)
r1 in mid 1k
l1 mid out 1u
c1 out 0 100p
.end
";
    let config = PssConfig::new(F0)
        .with_harmonics(2)
        .with_points_per_period(16)
        .with_tstab_periods(0)
        .with_tolerance(1.0e-6);
    assert_admitted(
        engine().run_pss_with_continuation_state(&parse(supported), config.clone()),
        "an R/L/C deck",
    );

    let cases = [
        (
            "\
* diode charge history is outside the shooting state
vin in 0 sin(0 0.4 1meg)
r1 in out 1k
c1 out 0 100p
d1 out 0 dmod
.model dmod d(cjo=1p)
.end
",
            "diode junction/diffusion charge history",
        ),
        (
            "\
* a Xyce LEVEL=2 resistor carries an accepted temperature state
vin in 0 sin(0 1 1meg)
r1 in out rmod l=1u a=1u
c1 out 0 100p
.model rmod R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)
.end
",
            "thermal resistor accepted temperature state",
        ),
    ];
    for (deck, expected) in cases {
        let message = engine()
            .run_pss_with_continuation_state(&parse(deck), config.clone())
            .expect_err("an uncaptured period-map state must fail before the periodic solve")
            .to_string();
        assert!(
            message.contains("PSS transient continuation is unavailable"),
            "the period-map preflight must own this rejection: {message}"
        );
        assert!(
            message.contains(expected),
            "the rejection must name the uncaptured state '{expected}': {message}"
        );
    }
}

#[test]
fn hb_envelope_continuation_names_every_family_outside_the_linear_subset() {
    let supported = "\
* R/C and independent sources are the declared envelope subset
Vcarrier carrier 0 SIN(0 1 1meg)
Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)
Rcarrier carrier out 1k
Rmod mod out 2k
Cout out 0 160p
.end
";
    assert_admitted(
        engine().run_hb_envelope_continuation_state(
            &parse(supported),
            HbConfig::new(F0).with_harmonics(2),
            &["Vmod".to_string()],
        ),
        "an R/C envelope deck",
    );

    let with_inductor = supported.replace("Cout out 0 160p", "Cout out 0 160p\nLout out 0 1u");
    let message = engine()
        .run_hb_envelope_continuation_state(
            &parse(&with_inductor),
            HbConfig::new(F0).with_harmonics(2),
            &["Vmod".to_string()],
        )
        .expect_err("an inductor is outside the exact envelope initializer's subset")
        .to_string();
    assert!(
        message.contains("HB Envelope continuation is unavailable"),
        "the envelope preflight must own this rejection: {message}"
    );
    assert!(
        message.contains("inductors"),
        "the rejection must name the family outside the subset: {message}"
    );
}

#[test]
fn pole_zero_follows_the_declared_dynamic_state_descriptor_capability() {
    let supported = "\
* an R/C ladder exports every dynamic state as an explicit descriptor state
vin in 0 ac 1
r1 in out 1k
c1 out 0 100p
r2 out 0 10k
.end
";
    let netlist = parse(supported);
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    assert_admitted(engine().run_pz(&netlist, input, output), "an R/C ladder");

    let with_line = "\
* a delay line's descriptor is irrational
vin in 0 ac 1
r1 in a 50
t1 a 0 out 0 z0=50 td=1n
r2 out 0 50
.end
";
    let netlist = parse(with_line);
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("line circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let message = engine()
        .run_pz(&netlist, input, output)
        .expect_err("a transmission line has no finite explicit descriptor state")
        .to_string();
    assert!(
        message.contains("Pole-zero analysis does not yet support"),
        "the dynamic-state preflight must own this rejection: {message}"
    );
    assert!(
        message.contains("irrational") && message.contains("transmission line 'T1'"),
        "the rejection must name the missing dynamic-state capability: {message}"
    );
}

#[test]
fn pnoise_follows_the_declared_noise_source_capability() {
    let stationary = "\
* thermal noise alone is a declared-complete periodic noise source
r1 out 0 1k
.end
";
    assert_admitted(
        engine().run_pnoise(&parse(stationary), F0, &[1.0e4], "out", None, None, 0),
        "a thermal-noise-only deck",
    );

    let colored = "\
* an authored flicker coefficient modulates with the periodic current
r1 out 0 1k noisy=1
.model rmod r (kf=1e-15 af=1 ef=1)
r2 out 0 rmod 1k
.end
";
    let message = engine()
        .run_pnoise(&parse(colored), F0, &[1.0e4], "out", None, None, 0)
        .expect_err("a modulated flicker source needs cyclostationary folding")
        .to_string();
    assert!(
        message.contains("cyclostationary colored-noise folding"),
        "the noise-source preflight must own this rejection: {message}"
    );
    assert!(
        message.contains("flicker noise"),
        "the rejection must name the mechanism it cannot fold: {message}"
    );
}
