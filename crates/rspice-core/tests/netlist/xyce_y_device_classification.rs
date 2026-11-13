//! Every `Y` line is classified before it is parsed.
//!
//! Xyce's `Y` namespace holds keyword-style devices (`Y<type> <name> ...`)
//! while historical SPICE uses `Y<name>` for a lossy transmission line. The
//! parser owns a closed table of Xyce's registered device types; a keyword in
//! that table is lowered or refused by family, and only a token outside it may
//! be read as a transmission line. These tests hold that contract from the
//! outside: through parse results and refusal text, never through the table.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_core::netlist::{ElementKind, Netlist};

fn deck(line: &str) -> String {
    format!("* Y-device classification fixture\n{line}\n.op\n.end\n")
}

fn parse_error(line: &str) -> String {
    Netlist::parse(&deck(line))
        .expect_err("line must be refused")
        .to_string()
}

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/xyce/Netlists")
        .canonicalize()
        .expect("bundled Xyce netlist corpus is present")
}

/// Every `Y` line the bundled Xyce corpus contains, keyed by its leading
/// token, with one representative source line per token.
fn corpus_y_lines() -> BTreeMap<String, (PathBuf, String)> {
    let mut found = BTreeMap::new();
    let mut pending = vec![corpus_root()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory).expect("corpus directory is readable");
        for entry in entries {
            let entry = entry.expect("corpus directory entry is readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let is_netlist = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| {
                    extension.eq_ignore_ascii_case("cir")
                        || extension.eq_ignore_ascii_case("net")
                        || extension.eq_ignore_ascii_case("sp")
                });
            if !is_netlist {
                continue;
            }
            let Ok(source) = std::fs::read_to_string(&path) else {
                continue;
            };
            for raw in source.lines() {
                let trimmed = raw.trim();
                let mut characters = trimmed.chars();
                if !characters
                    .next()
                    .is_some_and(|first| first == 'Y' || first == 'y')
                {
                    continue;
                }
                if !characters
                    .next()
                    .is_some_and(|second| second.is_ascii_alphanumeric() || second == '_')
                {
                    continue;
                }
                let token = trimmed
                    .split(|character: char| character.is_whitespace() || character == ',')
                    .next()
                    .unwrap_or_default();
                if token.len() < 2 {
                    continue;
                }
                found
                    .entry(token.to_ascii_uppercase())
                    .or_insert_with(|| (path.clone(), trimmed.to_owned()));
            }
        }
    }
    found
}

/// Text the parser emits only when a leading `Y` token matched no entry in the
/// Xyce device table and had to be re-read as a transmission line.
const UNCLASSIFIED: &str = "is not a recognized Xyce Y-device keyword";

#[test]
fn no_corpus_y_line_is_unclassified_or_lowered_to_a_transmission_line() {
    let lines = corpus_y_lines();
    assert!(
        lines.len() > 20,
        "the corpus sweep found only {} distinct Y tokens; the scan is broken",
        lines.len()
    );

    for (token, (path, line)) in lines {
        let display = path.display();
        match Netlist::parse(&deck(&line)) {
            Ok(netlist) => {
                let transmission_line = netlist
                    .elements
                    .iter()
                    .find(|element| matches!(element.kind, ElementKind::TransmissionLine { .. }));
                assert!(
                    transmission_line.is_none(),
                    "{token} from {display} silently lowered to a transmission line: {line}"
                );
            }
            Err(error) => {
                let message = error.to_string();
                assert!(
                    !message.contains(UNCLASSIFIED),
                    "{token} from {display} has no entry in the Xyce Y-device table: {message}"
                );
            }
        }
    }
}

#[test]
fn ibis_buffer_is_refused_by_family_not_by_transmission_line_parameter() {
    let message = parse_error("YIBIS buffer1 1 0 2 0 FILE=at16245.ibs Model=AT16245_IN");
    assert!(
        message.contains("unsupported capability")
            && message.contains("IBIS buffer")
            && message.contains("YIBIS"),
        "{message}"
    );
    assert!(
        !message
            .to_ascii_lowercase()
            .contains("transmission-line parameter"),
        "the IBIS refusal must not blame transmission-line parameters: {message}"
    );
}

#[test]
fn unknown_y_keyword_is_refused_instead_of_shifting_transmission_line_nodes() {
    let message = parse_error("YFOO buffer1 1 0 2 0 FILE=whatever.ibs");
    assert!(
        message.contains(UNCLASSIFIED) && message.contains("YFOO"),
        "{message}"
    );
}

#[test]
fn ordinary_y_line_transmission_line_still_parses() {
    let netlist = Netlist::parse(&deck("Y1 in 0 out 0 Z0=50 TD=1n")).expect("legacy Y-line parses");
    let element = netlist
        .elements
        .iter()
        .find(|element| element.name == "Y1")
        .expect("legacy Y-line keeps its authored name");
    assert!(matches!(element.kind, ElementKind::TransmissionLine { .. }));
}

#[test]
fn analog_families_without_a_model_program_name_the_family_and_the_capability() {
    for (line, family) in [
        ("YLIN L1 1 0 2 0 LINMOD", "linear N-port"),
        (
            "YPDE d1 1 0 DIODE na=1.0e15 nd=1.0e15",
            "TCAD device-equation",
        ),
        ("YDELAY delay1 2 0 1 0 TD=10N", "ideal delay line"),
        (
            "YTRANSLINE line1 1 2 testLine len=10 lumps=5000",
            "lumped transmission line",
        ),
        (
            "YROM ROM1 1 out BASE_FILENAME=mor.cir",
            "reduced-order model",
        ),
        ("YACC acc1 acc vel pos v0=0 x0=0.4", "accelerated-mass"),
        (
            "YAWL awl1 in awl1_out T=1 UL=0.5 LL=-0.5",
            "anti-windup limiter",
        ),
        ("YNEURON n1 a b neuronmod", "neuron membrane model"),
        ("YNEURONPOP np1 a b np", "neuron population"),
        ("YSYNAPSE s1 a b synmod", "neuron synapse"),
        ("YRXN R1 BAS GND rxn1", "chemical reaction network"),
    ] {
        let message = parse_error(line);
        assert!(
            message.contains("unsupported capability") && message.contains(family),
            "expected the {family} refusal for `{line}`, got: {message}"
        );
    }
}

#[test]
fn power_grid_short_and_long_spellings_name_the_same_family() {
    let short = parse_error("YPGBR BR1 b1 b2 v1 v2 AT=PQP R=0.05 X=0.1 B=0.05");
    let long = parse_error("YPOWERGRIDBRANCH BR1 b1 b2 v1 v2 AT=PQP R=0.05 X=0.1 B=0.05");
    assert!(short.contains("power-grid branch"), "{short}");
    assert!(long.contains("power-grid branch"), "{long}");
    assert!(short.contains("YPGBR"), "{short}");
    assert!(long.contains("YPOWERGRIDBRANCH"), "{long}");
}

#[test]
fn digital_and_mixed_signal_families_name_the_owning_effort() {
    for (line, family) in [
        ("YADC adc1 1 0 2 0 ADCMOD", "analog-to-digital converter"),
        ("YDAC dac1 1 0 2 0 DACMOD", "digital-to-analog converter"),
        ("YBUF b1 in out DMOD", "digital buffer"),
        ("YJKFF ff1 j k clk q qb DMOD", "digital JK flip-flop"),
        ("YTFF ff1 t clk q qb DMOD", "digital T flip-flop"),
        ("YDLTCH l1 d en q qb DMOD", "digital D latch"),
        ("YADD a1 in1 in2 out DMOD", "digital adder"),
    ] {
        let message = parse_error(line);
        assert!(
            message.contains("separate digital/mixed-signal effort") && message.contains(family),
            "expected the digital-ownership refusal for `{line}`, got: {message}"
        );
    }
}

#[test]
fn external_coupling_families_name_the_verilog_a_effort() {
    for line in [
        "YEXT y1 1 2 externcode=xyce netlist=resInner.cir",
        "YGENEXT R1 1 1a DPARAMS={NAME=R VALUE=1K}",
        "YVSRC VCC 1 0 Voltage=5V",
        "YRLC2 rlc1 1 0 R=1kohm L=1mH C=1pf",
    ] {
        let message = parse_error(line);
        assert!(
            message.contains("Verilog-A/external-coupling effort"),
            "expected the external-coupling refusal for `{line}`, got: {message}"
        );
    }
}

#[test]
fn internally_rewritten_mutual_inductors_point_at_the_canonical_card() {
    for line in ["YMIL K1 L1 L2 0.5", "YMIN K1 L1 L2 1 CORE"] {
        let message = parse_error(line);
        assert!(
            message.contains("mutual inductor") && message.contains("'K'"),
            "expected the mutual-inductor redirect for `{line}`, got: {message}"
        );
    }
}

#[test]
fn memristor_keyword_still_lowers_to_the_native_namespaced_device() {
    let source = "* native memristor\n\
                  .model mrm1 memristor level=2 ron=50 roff=1k\n\
                  YMEMRISTOR mr1 in 0 mrm1 ivrelation=1\n\
                  V1 in 0 1\n\
                  .op\n\
                  .end\n";
    let netlist = Netlist::parse(source).expect("YMEMRISTOR lowers natively");
    let element = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("YMEMRISTOR!MR1"))
        .expect("the memristor keeps Xyce's type-qualified identity");
    assert!(matches!(element.kind, ElementKind::XyceMemristor { .. }));
}

#[test]
fn bridged_digital_gates_keep_their_xspice_lowering_under_both_spellings() {
    for (line, instance) in [
        ("YNAND N1 in_a in_b out DMOD", "N1"),
        ("YNXOR X1 in_a in_b out DMOD", "X1"),
        ("YXNOR X2 in_a in_b out DMOD", "X2"),
        ("YINV I1 in out DMOD", "I1"),
        ("YNOT I2 in out DMOD", "I2"),
    ] {
        let netlist = Netlist::parse(&deck(line)).unwrap_or_else(|error| {
            panic!("`{line}` must keep its XSPICE digital-bridge lowering: {error}")
        });
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == instance)
            .unwrap_or_else(|| panic!("`{line}` must produce instance {instance}"));
        assert!(
            matches!(element.kind, ElementKind::Xspice { .. }),
            "`{line}` must lower to an XSPICE instance"
        );
    }
}

#[test]
fn rail_referenced_inverter_form_is_accepted_for_both_keywords() {
    for line in [
        "YNOT I1 dgnd dpwr in out DMOD",
        "YINV I1 dgnd dpwr in out DMOD",
    ] {
        let netlist = Netlist::parse(&deck(line))
            .unwrap_or_else(|error| panic!("`{line}` must parse: {error}"));
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "I1")
            .unwrap_or_else(|| panic!("`{line}` must produce instance I1"));
        let ElementKind::Xspice { model, .. } = &element.kind else {
            panic!("`{line}` must lower to an XSPICE instance");
        };
        assert_eq!(model, "d_inverter", "{line}");
    }
}

#[test]
fn malformed_keyword_device_lines_report_their_own_grammar() {
    // A classified keyword must never fall back to the transmission-line
    // reader, even when its own arguments are wrong.
    let message = parse_error("YMEMRISTOR mr1 in");
    assert!(!message.contains(UNCLASSIFIED), "{message}");
    assert!(
        !message.to_ascii_lowercase().contains("z0"),
        "a malformed memristor must not be diagnosed as a transmission line: {message}"
    );
}

#[test]
fn corpus_root_is_the_vendored_harness_directory() {
    // Guards the sweep above against silently scanning nothing if the corpus
    // is relocated.
    assert!(corpus_root().join("YLIN").is_dir(), "YLIN corpus directory");
    assert!(
        Path::new(&corpus_root()).join("IBIS/ibis.cir").is_file(),
        "IBIS corpus deck"
    );
}
