//! Native Spectre analog statements: what the adapter lowers, and what it
//! refuses by name.
//!
//! The adapter answers every refusal from one construct inventory, so a
//! recognized-but-unsupported construct says why and an unrecognized one says
//! it is unknown. These tests drive that from the outside: through lowered
//! netlists, executed results, and refusal text.

use std::path::PathBuf;

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::library::adapt_spectre_model_library;
use rspice_core::netlist::{AnalysisCommand, ElementKind, Netlist};

fn spectre_root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rspice_spectre_statements_{name}.scs"))
}

fn adapt(name: &str, source: &str) -> Netlist {
    Netlist::parse_with_path(source, &spectre_root(name))
        .unwrap_or_else(|error| panic!("Spectre source '{name}' must lower: {error}"))
}

fn refuse(name: &str, source: &str) -> String {
    Netlist::parse_with_path(source, &spectre_root(name))
        .err()
        .unwrap_or_else(|| panic!("Spectre source '{name}' must be refused"))
        .to_string()
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/xyce/Netlists/XDM/SPECTRE")
        .canonicalize()
        .expect("bundled Spectre corpus is present")
}

fn corpus_sources() -> Vec<(PathBuf, String)> {
    let mut sources = Vec::new();
    let mut pending = vec![corpus_root()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("corpus directory is readable") {
            let path = entry.expect("corpus entry is readable").path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let is_spectre = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("spectre"));
            if !is_spectre {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("corpus deck is UTF-8");
            sources.push((path, source));
        }
    }
    sources
}

/// The refusal text that means a construct had no entry in the inventory.
const UNKNOWN: &str = "unknown native Spectre";

#[test]
fn every_construct_in_the_bundled_corpus_is_classified() {
    let sources = corpus_sources();
    assert!(
        sources.len() >= 8,
        "the Spectre corpus sweep found only {} decks; the scan is broken",
        sources.len()
    );

    for (path, source) in sources {
        // The adapter must not change a source's line count, whatever it
        // decides about the statements inside it.
        if let Ok(adapted) = adapt_spectre_model_library(&path, &source) {
            assert_eq!(
                adapted.lines().count(),
                source.lines().count(),
                "{} changed line count",
                path.display()
            );
            continue;
        }
        let message = adapt_spectre_model_library(&path, &source)
            .expect_err("checked immediately above")
            .to_string();
        assert!(
            !message.contains(UNKNOWN),
            "{} contains a Spectre construct with no inventory entry: {message}",
            path.display()
        );
    }
}

#[test]
fn unknown_constructs_are_refused_as_unknown_in_every_namespace() {
    let master = refuse(
        "unknown-master",
        "simulator lang=spectre\nX1 (a b) frobnicate gain=2\n",
    );
    assert!(
        master.contains("unknown native Spectre instance master 'frobnicate'"),
        "{master}"
    );

    let family = refuse(
        "unknown-family",
        "simulator lang=spectre\nmodel mystery frobnicate gain=2\n",
    );
    assert!(
        family.contains("unknown native Spectre model family 'frobnicate'"),
        "{family}"
    );
}

#[test]
fn recognized_but_unsupported_constructs_say_why() {
    for (name, source, expected) in [
        (
            "ccvs",
            "simulator lang=spectre\nH1 (out 0 in 0) ccvs rm=100\n",
            "zero-volt probe",
        ),
        (
            "cccs",
            "simulator lang=spectre\nF1 (out 0 in 0) cccs gain=2\n",
            "zero-volt probe",
        ),
        (
            "sp",
            "simulator lang=spectre\nsp1 sp start=1 stop=1G dec=10\n",
            "no port mapping is defined",
        ),
        (
            "pss",
            "simulator lang=spectre\npss1 pss fund=1G harms=5\n",
            "periodic/RF analysis-card work package",
        ),
        (
            "options",
            "simulator lang=spectre\nmyopts options reltol=1e-3\n",
            "would change numerical results without saying so",
        ),
        (
            "altergroup",
            "simulator lang=spectre\naltergroup corner\n",
            "no ALTER variant axis",
        ),
    ] {
        let message = refuse(name, source);
        assert!(
            message.contains(expected) && !message.contains(UNKNOWN),
            "'{name}' must be refused with its recorded reason, got: {message}"
        );
    }
}

#[test]
fn dependent_and_behavioral_sources_lower_to_typed_elements() {
    let netlist = adapt(
        "dependent-sources",
        "simulator lang=spectre\n\
         V1 (in 0) vsource dc=1 type=dc\n\
         R1 (in 0) resistor r=1k\n\
         E1 (o1 0 in 0) vcvs gain=2\n\
         G1 (o2 0 in 0) vccs gm=1m\n\
         B1 (o3 0) bsource v=\"3*V(in)\"\n\
         RA (o1 0) resistor r=1k\n\
         RB (o2 0) resistor r=1k\n\
         RC (o3 0) resistor r=1k\n\
         op1 dc\n",
    );

    let kind = |name: &str| {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} is lowered"))
            .kind
            .clone()
    };
    assert!(matches!(kind("EE1"), ElementKind::Vcvs { .. }));
    assert!(matches!(kind("GG1"), ElementKind::Vccs { .. }));
    assert!(matches!(kind("BB1"), ElementKind::BehavioralVoltage { .. }));
    assert!(
        netlist
            .analyses
            .iter()
            .any(|analysis| matches!(analysis, AnalysisCommand::Op))
    );
}

/// `E1 (out 0) vcvs` has only two of its four terminals, and `vcvs` with no
/// gain has no value: both are grammar errors of the master, not of some
/// other card.
#[test]
fn malformed_dependent_and_behavioral_sources_fail_at_their_own_grammar() {
    for (name, source, expected) in [
        (
            "vcvs-arity",
            "simulator lang=spectre\nE1 (out 0) vcvs gain=2\n",
            "requires exactly four nodes",
        ),
        (
            "vcvs-gain",
            "simulator lang=spectre\nE1 (out 0 in 0) vcvs\n",
            "requires gain=",
        ),
        (
            "vccs-extra",
            "simulator lang=spectre\nG1 (out 0 in 0) vccs gm=1m bogus=3\n",
            "unsupported parameters: bogus",
        ),
        (
            "bsource-both",
            "simulator lang=spectre\nB1 (out 0) bsource v=\"1\" i=\"1\"\n",
            "declares both v= and i=",
        ),
        (
            "bsource-neither",
            "simulator lang=spectre\nB1 (out 0) bsource\n",
            "requires v= or i=",
        ),
        (
            "bsource-arity",
            "simulator lang=spectre\nB1 (out 0 extra) bsource v=\"1\"\n",
            "requires exactly two nodes",
        ),
    ] {
        let message = refuse(name, source);
        assert!(
            message.contains(expected),
            "'{name}' returned the wrong diagnostic: {message}"
        );
    }
}

#[test]
fn analyses_lower_to_canonical_cards() {
    let netlist = adapt(
        "analyses",
        "simulator lang=spectre\n\
         V1 (in 0) vsource dc=0 mag=1 type=dc\n\
         R1 (in out) resistor r=1k\n\
         C1 (out 0) capacitor c=1u\n\
         tran1 tran stop=1m step=10u\n\
         ac1 ac start=1 stop=1k dec=5\n\
         dc1 dc dev=V1 start=0 stop=1 step=0.25\n\
         noise1 (out 0) noise start=1 stop=1k dec=5 iprobe=V1\n\
         sweep1 sweep param=gain start=1 stop=3 step=1\n",
    );

    let mut saw_tran = false;
    let mut saw_ac = false;
    let mut saw_dc = false;
    let mut saw_noise = false;
    let mut saw_step = false;
    for analysis in &netlist.analyses {
        match analysis {
            AnalysisCommand::Tran { .. } => saw_tran = true,
            AnalysisCommand::Ac { .. } => saw_ac = true,
            AnalysisCommand::Dc { .. } => saw_dc = true,
            AnalysisCommand::Noise { .. } => saw_noise = true,
            AnalysisCommand::Step(_) => saw_step = true,
            _ => {}
        }
    }
    assert!(saw_tran, "tran lowers to .TRAN");
    assert!(saw_ac, "ac lowers to .AC");
    assert!(saw_dc, "dc dev= lowers to .DC");
    assert!(saw_noise, "noise lowers to .NOISE");
    assert!(saw_step, "sweep param= lowers to .STEP");
}

#[test]
fn malformed_and_ambiguous_analyses_fail_closed() {
    for (name, source, expected) in [
        (
            "tran-no-stop",
            "simulator lang=spectre\ntran1 tran step=1u\n",
            "requires stop=",
        ),
        (
            "tran-solver-control",
            "simulator lang=spectre\ntran1 tran stop=1m errpreset=moderate\n",
            "unsupported parameters: errpreset",
        ),
        (
            "tran-nodes",
            "simulator lang=spectre\ntran1 (a b) tran stop=1m\n",
            "does not take a node list",
        ),
        (
            "dc-unknown-device",
            "simulator lang=spectre\ndc1 dc dev=nosuch start=0 stop=1 step=1\n",
            "not an independent source declared in this source",
        ),
        (
            "dc-param",
            "simulator lang=spectre\ndc1 dc param=gain start=0 stop=1 step=1\n",
            "author .STEP for a parameter sweep",
        ),
        (
            "ac-density",
            "simulator lang=spectre\nac1 ac start=1 stop=1k\n",
            "requires dec= or lin=",
        ),
        (
            "ac-ambiguous-density",
            "simulator lang=spectre\nac1 ac start=1 stop=1k dec=5 lin=10\n",
            "declares both dec= and lin=",
        ),
        (
            "noise-probe-output",
            "simulator lang=spectre\nnoise1 noise start=1 stop=1k dec=5 oprobe=V1 iprobe=V1\n",
            "must name its output node pair",
        ),
        (
            "sweep-dev",
            "simulator lang=spectre\nsweep1 sweep dev=V1 start=0 stop=1 step=1\n",
            "requires param=",
        ),
    ] {
        let message = refuse(name, source);
        assert!(
            message.contains(expected),
            "'{name}' returned the wrong diagnostic: {message}"
        );
    }
}

#[test]
fn save_lowers_named_nets_and_refuses_selectors_it_cannot_express() {
    let netlist = adapt(
        "save-nets",
        "simulator lang=spectre\n\
         V1 (in 0) vsource dc=1 type=dc\n\
         R1 (in out) resistor r=1k\n\
         R2 (out 0) resistor r=1k\n\
         save in out\n\
         op1 dc\n",
    );
    assert!(
        !netlist.saves.is_empty(),
        "a Spectre save statement must reach the canonical save set"
    );

    for (name, source, expected) in [
        (
            "save-terminal",
            "simulator lang=spectre\nR1 (a b) resistor r=1k\nsave R1:1\n",
            "terminal current",
        ),
        (
            "save-option",
            "simulator lang=spectre\nsave depth=2\n",
            "selects an output scope rather than a signal",
        ),
        (
            "save-empty",
            "simulator lang=spectre\nsave\n",
            "names no signals",
        ),
    ] {
        let message = refuse(name, source);
        assert!(
            message.contains(expected),
            "'{name}' returned the wrong diagnostic: {message}"
        );
    }
}

/// A lowered Spectre source and the SPICE card it lowers to must be the same
/// circuit, not merely a similar one.
#[test]
fn lowered_dependent_sources_match_their_spice_equivalents_at_the_operating_point() {
    let spectre = adapt(
        "equivalence-op",
        "simulator lang=spectre\n\
         V1 (in 0) vsource dc=0.75 type=dc\n\
         RS (in 0) resistor r=1k\n\
         E1 (o1 0 in 0) vcvs gain=2\n\
         RA (o1 0) resistor r=1k\n\
         G1 (o2 0 in 0) vccs gm=1m\n\
         RB (o2 0) resistor r=1k\n\
         B1 (o3 0) bsource v=\"3*V(in)\"\n\
         RC (o3 0) resistor r=1k\n\
         op1 dc\n",
    );
    let spice = Netlist::parse(
        "equivalent SPICE deck\n\
         VV1 in 0 0.75\n\
         RRS in 0 1k\n\
         EE1 o1 0 in 0 2\n\
         RRA o1 0 1k\n\
         GG1 o2 0 in 0 1m\n\
         RRB o2 0 1k\n\
         BB1 o3 0 V={3*V(in)}\n\
         RRC o3 0 1k\n\
         .op\n\
         .end\n",
    )
    .expect("equivalent SPICE deck parses");

    let engine = engine();
    let from_spectre = engine.run_dc_op(&spectre).expect("Spectre OP converges");
    let from_spice = engine.run_dc_op(&spice).expect("SPICE OP converges");
    for node in ["in", "o1", "o2", "o3"] {
        let lowered = from_spectre
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("V({node}) from the Spectre deck"));
        let authored = from_spice
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("V({node}) from the SPICE deck"));
        assert_eq!(
            lowered.to_bits(),
            authored.to_bits(),
            "V({node}) differs: {lowered} vs {authored}"
        );
    }
}

#[test]
fn lowered_transient_and_ac_analyses_match_their_spice_equivalents() {
    let spectre = adapt(
        "equivalence-tran-ac",
        "simulator lang=spectre\n\
         V1 (in 0) vsource type=sine ampl=1 freq=1K mag=1\n\
         R1 (in mid) resistor r=1k\n\
         C1 (mid 0) capacitor c=1u\n\
         G1 (out 0 mid 0) vccs gm=1m\n\
         RL (out 0) resistor r=1k\n\
         tran1 tran stop=2m step=20u\n",
    );
    let spice = Netlist::parse(
        "equivalent SPICE deck\n\
         VV1 in 0 SIN(0 1 1K 0 0 0) AC 1\n\
         RR1 in mid 1k\n\
         CC1 mid 0 1u\n\
         GG1 out 0 mid 0 1m\n\
         RRL out 0 1k\n\
         .TRAN 20u 2m\n\
         .end\n",
    )
    .expect("equivalent SPICE deck parses");

    let engine = engine();
    let lowered = engine
        .run_tran(&spectre, 2e-3, 20e-6)
        .expect("Spectre transient runs");
    let authored = engine
        .run_tran(&spice, 2e-3, 20e-6)
        .expect("SPICE transient runs");
    assert_eq!(
        lowered.time.len(),
        authored.time.len(),
        "the two decks must produce the same time grid"
    );
    let index = lowered
        .node_names
        .iter()
        .position(|name: &String| name.eq_ignore_ascii_case("out"))
        .expect("out is a solved node");
    let authored_index = authored
        .node_names
        .iter()
        .position(|name: &String| name.eq_ignore_ascii_case("out"))
        .expect("out is a solved node");
    assert_eq!(
        lowered.voltage_waveform(index),
        authored.voltage_waveform(authored_index),
        "the lowered vccs must drive the same waveform as the authored G card"
    );

    let ac_point = |netlist: &Netlist| {
        let point = engine
            .run_ac(netlist, &[1.0e3])
            .expect("AC point solves")
            .pop()
            .expect("one requested AC point");
        let index = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out is an AC node");
        point.voltages[index]
    };
    let lowered_ac = ac_point(&spectre);
    let authored_ac = ac_point(&spice);
    assert_eq!(
        (lowered_ac.re.to_bits(), lowered_ac.im.to_bits()),
        (authored_ac.re.to_bits(), authored_ac.im.to_bits()),
        "the lowered AC source magnitude must match the authored AC card"
    );
}
