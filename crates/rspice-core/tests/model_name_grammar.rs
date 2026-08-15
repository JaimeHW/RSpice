//! Model-name and annotation grammar accepted by real vendor libraries.
//!
//! Discrete-part libraries name models far more freely than an identifier
//! allows. InterFET's `standard.jft` — the reference source these cases were
//! pinned against — opens with
//!
//! ```text
//! .MODEL 2N2608-NSC PJF VTO=-2.5 ... MFG=NSC
//! ```
//!
//! which exercises both a hyphen-joined manufacturer suffix in the model name
//! and the `MFG=` string annotation. ngspice 46 accepts names of this shape;
//! before these cases were pinned, RSpice rejected the file on its first line.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn run_op(deck: &str) -> rspice_core::solver::SimulationResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges")
}

#[test]
fn hyphenated_model_name_defines_and_resolves() {
    let result = run_op(
        "* hyphenated model name\n\
         V1 a 0 0.7\n\
         D1 a 0 MYD-X\n\
         .model MYD-X D(IS=1e-14 N=1)\n\
         .op\n\
         .end\n",
    );
    let current = result
        .branch_current_named("v1")
        .expect("source branch current present");
    // Forward-biased at 0.7 V with IS=1e-14, N=1: the exact value belongs to
    // the diode tests; here it only has to be a real forward current, proving
    // the instance bound to the hyphenated card rather than to a default.
    assert!(
        current < -1e-6,
        "expected forward conduction through MYD-X, got I(V1)={current:e}"
    );
}

#[test]
fn slash_qualified_model_name_resolves() {
    // `LM741/NS` and `BC547A/PLP` style names appear throughout the vendored
    // community collections.
    let result = run_op(
        "* slash-qualified model name\n\
         V1 a 0 0.7\n\
         D1 a 0 DPART/MFR\n\
         .model DPART/MFR D(IS=1e-14 N=1)\n\
         .op\n\
         .end\n",
    );
    assert!(
        result
            .branch_current_named("v1")
            .expect("source branch current present")
            < -1e-6
    );
}

#[test]
fn model_name_does_not_swallow_its_type_across_whitespace() {
    // Pieces join only where they touch in the source. A space before the type
    // token is a hard boundary, so this must still parse as name `FOO`, type
    // `D` — not as a model named `FOOD` with no type.
    let netlist = Netlist::parse(
        "* whitespace boundary\n\
         V1 a 0 0.7\n\
         D1 a 0 FOO\n\
         .model FOO D(IS=1e-14)\n\
         .op\n\
         .end\n",
    )
    .expect("deck parses");
    assert!(
        netlist.models.iter().any(|m| m.name == "FOO"),
        "expected a model named FOO, found {:?}",
        netlist.models.iter().map(|m| &m.name).collect::<Vec<_>>()
    );
}

#[test]
fn mfg_annotation_is_accepted_and_retained() {
    // MFG= names the manufacturer and carries no electrical meaning. PSpice and
    // LTspice both document it; discrete libraries annotate cards of every
    // device type with it.
    let netlist = Netlist::parse(
        "* manufacturer annotation\n\
         V1 d 0 5\n\
         V2 g 0 0\n\
         J1 d g 0 JPART\n\
         .model JPART NJF VTO=-3 BETA=1.3m LAMBDA=2.3m MFG=VISHAY\n\
         .op\n\
         .end\n",
    )
    .expect("deck with MFG annotation parses");

    let model = netlist
        .models
        .iter()
        .find(|m| m.name == "JPART")
        .expect("JPART model present");
    let mfg = model
        .string_params
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("MFG"))
        .map(|(_, value)| value.as_str());
    assert_eq!(
        mfg,
        Some("VISHAY"),
        "MFG should be retained as a string parameter, got {:?}",
        model.string_params
    );
}

#[test]
fn interfet_style_card_parses_and_solves() {
    // The exact shape of the first line of InterFET's standard.jft: hyphenated
    // name, extended JFET parameter set, MFG annotation.
    let result = run_op(
        "* interfet-style jfet card\n\
         V1 d 0 5\n\
         V2 g 0 0\n\
         J1 d g 0 2N3819-VSH\n\
         .MODEL 2N3819-VSH NJF VTO=-3 BETA=1.3m LAMBDA=2.3m RD=1 RS=1 \
         CGS=2.4p CGD=1.6p PB=1 IS=33.6f FC=0.5 N=1 XTI=3 MFG=VISHAY\n\
         .op\n\
         .end\n",
    );
    let drain_current = -result
        .branch_current_named("v1")
        .expect("source branch current present");
    // VTO=-3, BETA=1.3m at Vgs=0 gives Idss = BETA*VTO^2 = 11.7 mA before the
    // 2 ohm of RD+RS degeneration; bound it loosely enough that this stays a
    // grammar test rather than a device-model test.
    assert!(
        (5e-3..20e-3).contains(&drain_current),
        "expected a plausible Idss for 2N3819, got {drain_current:e} A"
    );
}
