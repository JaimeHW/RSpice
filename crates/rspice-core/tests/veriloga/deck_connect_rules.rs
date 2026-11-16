//! `.CONNECTRULES`: a connect-rule block written in the deck, on decks the
//! engine builds and solves.
//!
//! The block compiles through the connect compiler every other block takes,
//! so the converters it selects, their parameters and where each value came
//! from are the ones a `.vams` block with the same statements gives. What the
//! deck adds is the location: each converter, and each refusal, names the
//! deck line its statement is written on.

use rspice_core::circuit::{ConverterInsertion, ConverterKind, ConverterSource};
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};

/// The ramp's slope, volts per second: 0 to 3.3 V over 100 ns.
const SLOPE: f64 = 3.3 / 100e-9;
/// The transient's largest step. A connect-rule A/D edge is a `cross()`
/// event, dated at the crossing whatever the step (VAMS-LRM 2.4 5.10.3.1).
const MAX_STEP: f64 = 0.1e-9;

/// A ramp into an XSPICE inverter's input: the node `din` is an A/D boundary,
/// `dout` a D/A one, both converted by the deck's own block.
fn ramp_deck(statements: &str) -> String {
    format!(
        "* deck connect rules\n\
         vin din 0 pwl(0 0 100n 3.3)\n\
         rin din 0 1meg\n\
         ainv din dout inverter\n\
         .model inverter d_inverter\n\
         rload dout 0 1k\n\
         .connectrules ramp_rules\n\
         {statements}\
         .end\n"
    )
}

fn converters(deck: &str) -> Vec<ConverterInsertion> {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    Engine::default()
        .build_circuit(&netlist)
        .unwrap_or_else(|error| panic!("deck builds: {error}"))
        .converter_insertions()
        .to_vec()
}

fn build_error(deck: &str) -> String {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    match Engine::default().build_circuit(&netlist) {
        Ok(_) => panic!("the deck must be refused"),
        Err(error) => error.to_string(),
    }
}

/// The block selects the converters, each names its statement's deck line,
/// and the threshold a statement sets is where the A/D converter switches: a
/// ramp of slope `s` crosses `vth` at `t = vth / s`.
#[test]
fn a_deck_block_moves_the_a2d_switching_point_to_its_threshold() {
    for threshold in [1.2, 2.0] {
        let deck = ramp_deck(&format!(
            "+ connect a2d #(.vsup(3.3), .vthi({threshold}), .vtlo({threshold}));\n\
             + connect d2a #(.vsup(3.3)); // the driving side\n"
        ));
        let records = converters(&deck);
        let rules = ConverterSource::ConnectRule {
            block: "ramp_rules".into(),
            earlier_signature: false,
        };
        let a2d = records
            .iter()
            .find(|record| record.kind == ConverterKind::A2d)
            .expect("din is an A/D boundary");
        assert_eq!(
            (a2d.boundary.as_str(), &a2d.rules, a2d.location.as_deref()),
            ("DIN", &rules, Some("line 8"))
        );
        let vthi = a2d
            .parameters
            .iter()
            .find(|parameter| parameter.name == "vthi")
            .expect("vthi is recorded");
        assert_eq!(
            (vthi.value, &vthi.source),
            (threshold, &ConverterSource::ConnectStatement)
        );
        let d2a = records
            .iter()
            .find(|record| record.kind == ConverterKind::D2a)
            .expect("dout is a D/A boundary");
        assert_eq!(
            (d2a.boundary.as_str(), &d2a.rules, d2a.location.as_deref()),
            ("DOUT", &rules, Some("line 9"))
        );

        let netlist = Netlist::parse(&deck).expect("deck parses");
        let result = Engine::default()
            .run_tran(&netlist, 100e-9, MAX_STEP)
            .unwrap_or_else(|error| panic!("transient solves: {error}"));
        let switched = result
            .digital_trace_named("DIN")
            .expect("din has a digital side")
            .iter()
            .find(|point| point.value.state == DigitalState::One)
            .expect("the ramp crosses the threshold")
            .time;
        let crossing = threshold / SLOPE;
        assert!(
            (switched - crossing).abs() <= 1.0e-15,
            "vth = {threshold} V: switched at {switched:e} s, the ramp crosses at {crossing:e} s"
        );
    }
}

/// Every statement is checked when the deck is built, whether or not a
/// boundary selects it, and the refusal names the deck line it is written on.
#[test]
fn a_deck_block_is_refused_at_the_line_that_is_wrong() {
    let cases = [
        (
            // `bidir` bridges no boundary here; its misspelt knob is still
            // refused rather than left unheard.
            "+ connect a2d;\n+ connect d2a;\n+ connect bidir #(.vthx(1.2));\n",
            "line 10",
            "passes parameter 'vthx', which the built-in bidir does not take",
        ),
        (
            "+ connect a2d;\n+ connect d2b;\n",
            "line 9",
            "'d2b', which is not a declared connectmodule",
        ),
    ];
    for (statements, line, expected) in cases {
        let error = build_error(&ramp_deck(statements));
        assert!(
            error.contains(&format!("at {line}:")) && error.contains(expected),
            "{statements:?}: {error}"
        );
    }
    let reserved = build_error(
        &ramp_deck("+ connect a2d;\n+ connect d2a;\n").replace("ramp_rules", "rspice_mine"),
    );
    assert!(
        reserved.contains("at line 7:") && reserved.contains("reserves for its shipped rule sets"),
        "{reserved}"
    );
}

/// `.OPTIONS CONNECTRULES` selects a deck block by name beside another
/// block, as it selects a `.vams` block, and an unknown name lists the deck's
/// blocks by their `.CONNECTRULES` lines.
#[test]
fn connectrules_selects_a_deck_block_by_name() {
    let two_blocks = ramp_deck("+ connect a2d #(.vthi(1.0), .vtlo(1.0));\n+ connect d2a;\n")
        .replace(
            ".end\n",
            ".connectrules other\n+ connect a2d #(.vthi(2.0), .vtlo(2.0));\n+ connect \
             d2a;\n.options connectrules=other\n.end\n",
        );
    let a2d = converters(&two_blocks)
        .into_iter()
        .find(|record| record.kind == ConverterKind::A2d)
        .expect("din converts");
    assert_eq!(a2d.location.as_deref(), Some("line 11"));
    let unknown = build_error(&two_blocks.replace("connectrules=other", "connectrules=third"));
    assert!(
        unknown.contains("'ramp_rules' at line 7") && unknown.contains("'other' at line 10"),
        "{unknown}"
    );
}

/// A port no one-way statement fits converts through the bidirectional one,
/// as section 7.8.3.1's Figure 7-9 has a `bidir` rule beside a `d2a` rule
/// take the input ports: the input `din` gets the bidirectional converter,
/// which senses the ramp at its own threshold, and `dout` keeps `d2a`.
#[test]
fn an_input_port_only_a_bidir_rule_covers_converts_through_it() {
    let deck = ramp_deck(
        "+ connect bidir #(.vsup(3.3), .ftlo(0.7), .fthi(0.7));\n\
         + connect d2a #(.vsup(3.3));\n",
    );
    let records = converters(&deck);
    let din = records
        .iter()
        .find(|record| record.boundary == "DIN")
        .expect("din converts");
    assert_eq!(
        (
            din.kind,
            din.module.as_str(),
            din.direction,
            din.location.as_deref()
        ),
        (
            ConverterKind::Bidir,
            "bidir",
            rspice_core::xspice::PortDirection::In,
            Some("line 8")
        ),
        "{din:#?}"
    );
    let dout = records
        .iter()
        .find(|record| record.boundary == "DOUT")
        .expect("dout converts");
    assert_eq!(
        (dout.kind, dout.module.as_str()),
        (ConverterKind::D2a, "d2a")
    );

    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 100e-9, MAX_STEP)
        .unwrap_or_else(|error| panic!("transient solves: {error}"));
    let switched = result
        .digital_trace_named("DIN")
        .expect("din has a digital side")
        .iter()
        .find(|point| point.value.state == DigitalState::One)
        .expect("the ramp crosses the threshold")
        .time;
    let crossing = 0.7 * 3.3 / SLOPE;
    assert!(
        (0.0..=MAX_STEP).contains(&(switched - crossing)),
        "switched at {switched:e} s, the ramp crosses fthi at {crossing:e} s"
    );
}
