//! Capability refusals reach the engine's public API as typed refusals.
//!
//! "RSpice understood this deck and declines to run it" is categorically
//! different from "this deck is wrong", and every frontend reports the two
//! differently: an unsupported analysis/device pair is a capability gap to
//! record against the roadmap, while a malformed deck is the author's to fix.
//!
//! These tests drive real decks through the public entry points rather than
//! constructing errors, because the value of the taxonomy is that the refusal
//! survives the whole path from the parser or the elaborator out to the caller.

use rspice_core::analysis::HbConfig;
use rspice_core::netlist::ParseError;
use rspice_core::{Engine, Netlist, SimulationError, SimulationErrorCategory, SimulationErrorCode};

/// The capability token a refusal published, or a panic naming what came back.
fn capability_token(error: &SimulationError) -> &'static str {
    let descriptor = error.descriptor();
    assert_eq!(
        descriptor.code,
        SimulationErrorCode::UnsupportedCapability,
        "expected a capability refusal, got {error}"
    );
    assert_eq!(descriptor.category, SimulationErrorCategory::Capability);
    match error {
        SimulationError::UnsupportedCapability(refusal) => refusal.capability,
        other => panic!("descriptor and variant disagreed: {other}"),
    }
}

fn parse(source: &str) -> Netlist {
    Netlist::parse(source).expect("deck parses")
}

/// Resolve a deck node to the solver index the analysis entry points take.
fn node(netlist: &Netlist, name: &str) -> usize {
    Engine::default()
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

#[test]
fn harmonic_balance_refuses_a_device_it_cannot_stamp() {
    // Exact HB does not represent the complete Gummel-Poon equations, so a
    // BJT is a device HB understands and declines rather than a bad card.
    let netlist = parse(
        "hb capability\n\
         V1 in 0 SIN(0 0.5 1e6)\n\
         Vcc vcc 0 5\n\
         R1 in b 1k\n\
         Rc vcc c 1k\n\
         Q1 c b 0 npnmod\n\
         .model npnmod NPN IS=1e-16 BF=100\n\
         .end\n",
    );
    let error = Engine::default()
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
        .expect_err("HB must refuse a device it has no stamp for");
    let token = capability_token(&error);
    assert!(
        token.starts_with("analysis.hb."),
        "HB refusals must be namespaced under their analysis: {token}"
    );
}

#[test]
fn driven_pss_refuses_a_source_waveform_it_cannot_authenticate() {
    let netlist = parse(
        "pss capability\n\
         V1 in 0 PWL(0 0 1u 1 2u 0)\n\
         R1 in out 1k\n\
         C1 out 0 1n\n\
         .end\n",
    );
    let error = Engine::default()
        .validate_periodic_source_contract(&netlist, &["V1".to_owned()], 1.0e6)
        .expect_err("a PWL drive has no authenticated period");
    assert_eq!(
        capability_token(&error),
        "analysis.pss.driven_source_waveform"
    );
}

#[test]
fn pole_zero_refuses_transmission_lines() {
    let netlist = parse(
        "pz capability\n\
         V1 in 0 1\n\
         T1 in 0 out 0 Z0=50 TD=1n\n\
         R1 out 0 50\n\
         .end\n",
    );
    let output = node(&netlist, "out");
    let input = node(&netlist, "in");
    let error = Engine::default()
        .run_pz(&netlist, input, output)
        .expect_err("pole-zero has no distributed-line descriptor");
    assert_eq!(
        capability_token(&error),
        "analysis.pz.device.transmission_line"
    );
}

#[test]
fn noise_refuses_a_charge_model_it_does_not_implement() {
    // A fractional BSIM4 CVCHARGEMOD is a valid authored selection whose
    // charge equations this build does not implement. DC still solves; every
    // charge-based analysis, noise included, declines it.
    let netlist = parse(
        "noise capability\n\
         .model NCH NMOS LEVEL=14 VERSION=4.8 CVCHARGEMOD=1.5\n\
         M1 d g 0 0 NCH W=1u L=1u\n\
         Vg g 0 DC 0.8 AC 1\n\
         Vdd dd 0 DC 1.2\n\
         Rd dd d 1k\n\
         .end\n",
    );
    // Node 1 is the first non-ground node; the deck is refused during
    // elaboration, before any index could be resolved from a built circuit.
    let error = Engine::default()
        .run_noise(&netlist, 1, &[1.0e6], 300.15)
        .expect_err("an unimplemented charge model cannot produce noise");
    assert_eq!(capability_token(&error), "device.bsim4.cvchargemod");
}

#[test]
fn finite_length_rg_ltra_is_refused_before_any_solve() {
    let netlist = parse(
        "ltra capability\n\
         V1 in 0 1\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=0 C=0 LEN=1\n\
         Rl out 0 50\n\
         .end\n",
    );
    let error = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("finite-length RG has no native stamp");
    assert_eq!(capability_token(&error), "device.ltra.rg_finite_length");
}

#[test]
fn xyce_y_device_families_are_refused_by_the_grammar_with_their_span() {
    let error = Netlist::parse(
        "y capability\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         YDELAY delay1 2 0 1 0 TD=10N\n\
         .op\n\
         .end\n",
    )
    .expect_err("an unlowerable Y-device family must not parse");

    let ParseError::UnsupportedCapability {
        origin,
        capability,
        detail,
    } = &error
    else {
        panic!("Y-device refusals must be capability refusals, got {error}");
    };
    assert_eq!(*capability, "netlist.xyce.ydevice.no_model_program");
    assert_eq!(origin.line, 4, "the refusal must name the authoring line");
    assert!(
        detail.contains("YDELAY"),
        "the refusal must name the keyword: {detail}"
    );
}

#[test]
fn a_capability_refusal_keeps_its_category_across_the_parse_boundary() {
    // The parser and the elaborator raise refusals through different error
    // types. Both must arrive at a caller as the same category, otherwise a
    // frontend has to know which stage refused in order to report it.
    let netlist = parse(
        "y capability in elaboration\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .end\n",
    );
    let elaborated = Engine::default().run_dc_op(&netlist);
    assert!(elaborated.is_ok(), "control deck must run");

    let refused = Netlist::parse(
        "y capability\n\
         V1 in 0 1\n\
         YNEURON n1 a b neuronmod\n\
         .op\n\
         .end\n",
    )
    .expect_err("neuron families are owned by a separate effort");
    assert!(
        matches!(refused, ParseError::UnsupportedCapability { .. }),
        "got {refused}"
    );
}
