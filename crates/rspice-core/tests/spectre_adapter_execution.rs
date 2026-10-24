//! Spectre model-library adaptations that are proved by running the engine.
//!
//! The adapter itself lives in the `netlist` layer, which sits far below
//! `engine`. Its own unit tests therefore assert on the adapted text, the
//! reported source line, and the refusal message — never on a simulation.
//! The properties below need a solved circuit to mean anything: that the
//! circuit builder independently refuses a six-terminal BSIMSOI instance the
//! adapter also refuses, that a lowered BSIMSOI model actually reaches the
//! native LEVEL=10 PD route, and that a Spectre scale factor sets the value a
//! device is solved with. Driving an `Engine` from inside `netlist` would
//! invert the layer order (see `tests/module_layering.rs`), so they are
//! integration tests, entered through the crate's public surface.

use std::path::{Path, PathBuf};

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::library::adapt_spectre_model_library;
use rspice_core::netlist::Netlist;

fn deck_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rspice_spectre_execution_{name}.scs"))
}

/// The current each named source drives into its load at the operating point.
fn source_currents(name: &str, source: &str, sources: &[&str]) -> Vec<f64> {
    let deck = Netlist::parse_with_path(source, &deck_path(name))
        .unwrap_or_else(|error| panic!("Spectre deck '{name}' must parse: {error}"));
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&deck)
        .unwrap_or_else(|error| panic!("Spectre deck '{name}' must solve: {error}"));
    sources
        .iter()
        .map(|source| {
            result
                .branch_current_named(source)
                .unwrap_or_else(|| panic!("no branch current for {source} in '{name}'"))
        })
        .collect()
}

fn assert_current(label: &str, measured: f64, resistance: f64) {
    // One volt across the resistor, so the source sinks 1/R. The solver also
    // keeps its floor conductance (`ConvergenceConfig::gmin_target`, 1e-15 S)
    // on the node, which the allowance below covers and nothing else does: a
    // misread factor is off by powers of a thousand.
    let expected = -1.0 / resistance;
    assert!(
        (measured - expected).abs() <= 1e-12 * expected.abs() + 2e-15,
        "{label}: I = {measured:e} A, but 1 V across {resistance:e} ohm draws {expected:e} A"
    );
}

/// Each Spectre scale factor, and each SPICE suffix in a `simulator
/// lang=spice` region of the same file, sets the resistance a 1 V source is
/// solved against. The resistances are written so that every factor lands
/// between 1 mohm and a few Gohm, and each is checked against 1/R worked out
/// by hand from its decimal value.
#[test]
fn every_scale_factor_sets_the_resistance_in_its_own_language_region() {
    let spectre = [
        ("T", "0.0015T", 1.5e9),
        ("G", "2.2G", 2.2e9),
        ("M", "1M", 1e6),
        ("K", "4.7K", 4.7e3),
        ("k", "3.3k", 3.3e3),
        ("_", "47_", 47.0),
        ("%", "250%", 2.5),
        ("c", "680c", 6.8),
        ("m", "820m", 0.82),
        ("u", "1500u", 1.5e-3),
        ("n", "2700000n", 2.7e-3),
        ("p", "3900000000p", 3.9e-3),
        ("f", "5600000000000f", 5.6e-3),
        ("a", "6800000000000000a", 6.8e-3),
    ];
    let spice = [
        ("T", "0.0015T", 1.5e9),
        ("G", "2.2G", 2.2e9),
        ("MEG", "1MEG", 1e6),
        ("meg", "4.7meg", 4.7e6),
        ("K", "4.7K", 4.7e3),
        ("k", "3.3k", 3.3e3),
        ("M", "820M", 0.82),
        ("m", "680m", 0.68),
        ("U", "1500U", 1.5e-3),
        ("N", "2700000N", 2.7e-3),
        ("P", "3900000000P", 3.9e-3),
        ("F", "5600000000000F", 5.6e-3),
    ];
    let mut deck = vec!["simulator lang=spectre".to_owned()];
    for (index, (_, literal, _)) in spectre.iter().enumerate() {
        deck.push(format!("v{index} (n{index} 0) vsource dc=1"));
        deck.push(format!("r{index} (n{index} 0) resistor r={literal}"));
    }
    deck.push("simulator lang=spice".to_owned());
    for (index, (_, literal, _)) in spice.iter().enumerate() {
        deck.push(format!("Vs{index} s{index} 0 1"));
        deck.push(format!("Rs{index} s{index} 0 {literal}"));
    }
    deck.push("simulator lang=spectre".to_owned());
    deck.push("op1 dc".to_owned());

    let names = (0..spectre.len())
        .map(|index| format!("Vv{index}"))
        .chain((0..spice.len()).map(|index| format!("Vs{index}")))
        .collect::<Vec<_>>();
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    let currents = source_currents("scale-factors", &(deck.join("\n") + "\n"), &names);
    for (index, (factor, literal, resistance)) in spectre.iter().enumerate() {
        assert_current(
            &format!("Spectre factor '{factor}' in r={literal}"),
            currents[index],
            *resistance,
        );
    }
    for (index, (suffix, literal, resistance)) in spice.iter().enumerate() {
        assert_current(
            &format!("SPICE suffix '{suffix}' in {literal}"),
            currents[spectre.len() + index],
            *resistance,
        );
    }
}

/// The deck that showed `M` read as milli: `r=1M` drew a kiloampere.
#[test]
fn spectre_mega_reaches_instances_parameters_expressions_and_subckts() {
    let source = "simulator lang=spectre\n\
                  parameters rr=1M gg=2\n\
                  v1 (a 0) vsource dc=1\n\
                  r1 (a 0) resistor r=1M\n\
                  v2 (b 0) vsource dc=1\n\
                  r2 (b 0) resistor r=rr\n\
                  v3 (c 0) vsource dc=1\n\
                  r3 (c 0) resistor r=1m\n\
                  v4 (d 0) vsource dc=1\n\
                  r4 (d 0) resistor r=rr*gg\n\
                  parameters r6=gg*rr+1M\n\
                  v6 (f 0) vsource dc=1\n\
                  r6 (f 0) resistor r=r6\n\
                  subckt load (p)\n\
                  parameters rv=1k\n\
                  rl (p 0) resistor r=rv\n\
                  ends load\n\
                  v5 (e 0) vsource dc=1\n\
                  x5 (e) load rv=4M\n\
                  v7 (g 0) vsource dc=1\n\
                  x7 (g) load rv=rr*gg*2\n\
                  v8 (h 0) vsource dc=1\n\
                  r8 (h 0) resistor r='1M'\n\
                  op1 dc\n";
    let currents = source_currents(
        "mega",
        source,
        &["Vv1", "Vv2", "Vv3", "Vv4", "Vv5", "Vv6", "Vv7", "Vv8"],
    );
    for (label, measured, resistance) in [
        ("r=1M", currents[0], 1e6),
        ("r=rr with rr=1M", currents[1], 1e6),
        ("r=1m", currents[2], 1e-3),
        ("r=rr*gg", currents[3], 2e6),
        ("subckt rv=4M", currents[4], 4e6),
        ("parameters r6=gg*rr+1M", currents[5], 3e6),
        ("subckt rv=rr*gg*2", currents[6], 4e6),
        (
            "r='1M', an expression to the canonical reader",
            currents[7],
            1e6,
        ),
    ] {
        assert_current(label, measured, resistance);
    }
}

/// Spectre's `a` and `A` are two nodes; read without case they would merge
/// and the deck would solve to a different circuit without a word.
#[test]
fn names_that_differ_only_in_case_are_refused_before_anything_solves() {
    let nodes = deck_path("nodecase");
    let message = Netlist::parse_with_path(
        "simulator lang=spectre\n\
         v2 (a 0) vsource dc=1\n\
         ra (a 0) resistor r=1k\n\
         v3 (A b) vsource dc=2\n\
         rb (b 0) resistor r=1k\n\
         op1 dc\n",
        &nodes,
    )
    .expect_err("a node spelled in two cases is refused")
    .to_string();
    let file = "rspice_spectre_execution_nodecase.scs";
    assert!(
        message.contains("Spectre node names 'a' (")
            && message.contains(&format!("{file}:2) and 'A' ("))
            && message.contains(&format!("{file}:4) differ only in case")),
        "{message}"
    );

    let instances = deck_path("case");
    let message = Netlist::parse_with_path(
        "simulator lang=spectre\n\
         V0 (net1 0) vsource dc=1\n\
         r0 (net4 0) resistor r=1k\n\
         R0 (net1 net4) resistor r=1k\n\
         N1 (a 0) resistor r=1k\n\
         n1 (A 0) resistor r=3k\n\
         v2 (a 0) vsource dc=1\n\
         v3 (A 0) vsource dc=2\n\
         op1 dc\n",
        &instances,
    )
    .expect_err("an instance spelled in two cases is refused")
    .to_string();
    let file = "rspice_spectre_execution_case.scs";
    assert!(
        message.contains("Spectre instance names 'r0' (")
            && message.contains(&format!("{file}:3) and 'R0' ("))
            && message.contains(&format!("{file}:4) differ only in case")),
        "{message}"
    );
}

#[test]
fn unrepresented_bsimsoi_terminal_forms_fail_at_the_instance_source_line() {
    let source = "simulator lang=spectre\nmodel nfet bsimsoi type=n version=3.2\nn1 (d g s e p b) nfet w=1u l=0.5u\n";
    let error = adapt_spectre_model_library(Path::new("bsimsoi.scs"), source)
        .expect_err("six-terminal BSIMSOI must not silently discard a node");

    assert_eq!(error.line, 3, "{error}");
    assert!(error.message.contains("has 6 nodes"), "{error}");
    assert!(error.message.contains("not yet represented"), "{error}");

    let canonical = Netlist::parse(
        "six-terminal BSIMSOI backstop\n\
         m1 d g s e p b nmod w=1u l=0.5u\n\
         vd d 0 1\n\
         vg g 0 0.8\n\
         .model nmod nmos level=10 version=3.2\n\
         .end\n",
    )
    .expect("canonical six-terminal fixture parses");
    let message = Engine::new(SimulationConfig::default())
        .run_dc_op(&canonical)
        .expect_err("the builder must independently reject unrepresented BSIMSOI nodes")
        .to_string();
    assert!(message.contains("6 terminals"), "{message}");
    assert!(message.contains("not yet represented"), "{message}");
}

#[test]
fn lowered_bsimsoi_instances_execute_on_the_native_pd_route() {
    let source = "simulator lang=spectre\nmodel nfet bsimsoi\n+ type=n\n+ version=3.2\n+ tox=8e-9\nmodel pfet bsimsoi type=p version=3.2 tox=8e-9\nvdd (vdd 0) vsource dc=1.8\nvdn (dn 0) vsource dc=1\nvdp (dp 0) vsource dc=0.8\nvgn (gn 0) vsource dc=0.8\nvgp (gp 0) vsource dc=1\nn1 (dn gn 0 0) nfet w=1u l=0.5u\np1 (dp gp vdd vdd) pfet w=1u l=0.5u\n";
    let adapted = adapt_spectre_model_library(Path::new("bsimsoi.scs"), source)
        .expect("qualified Spectre BSIMSOI fixture lowers");
    let deck = Netlist::parse(&format!(
        "native Spectre BSIMSOI route\n{adapted}.op\n.end\n"
    ))
    .expect("lowered BSIMSOI fixture parses as canonical SPICE");
    let (_, report) = Engine::new(SimulationConfig::default())
        .run_dc_op_with_report(&deck)
        .expect("lowered n/p BSIMSOI instances execute natively");

    for name in ["Mn1", "Mp1"] {
        let entry = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing operating-point entry for {name}"));
        assert_eq!(entry.device_kind, "B3SOIPD", "{name} must use LEVEL=10");
    }
}
