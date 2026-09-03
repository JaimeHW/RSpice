//! Spectre model-library adaptations that are proved by running the engine.
//!
//! The adapter itself lives in the `netlist` layer, which sits far below
//! `engine`. Its own unit tests therefore assert on the adapted text, the
//! reported source line, and the refusal message — never on a simulation.
//! The two properties below need a solved circuit to mean anything: that the
//! circuit builder independently refuses a six-terminal BSIMSOI instance the
//! adapter also refuses, and that a lowered BSIMSOI model actually reaches the
//! native LEVEL=10 PD route. Driving an `Engine` from inside `netlist` would
//! invert the layer order (see `tests/module_layering.rs`), so they are
//! integration tests, entered through the crate's public surface.

use std::path::Path;

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::library::adapt_spectre_model_library;
use rspice_core::netlist::Netlist;

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
