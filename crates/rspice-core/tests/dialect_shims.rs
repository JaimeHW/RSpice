//! PSpice/LTspice dialect shims: `AKO:` model inheritance and
//! LTspice-style passive parasitics (`Rser`/`Rpar`/`Cpar`).

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::ElementKind;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

fn model<'a>(netlist: &'a Netlist, name: &str) -> &'a rspice_core::netlist::ModelDef {
    netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("model {name} missing"))
}

fn param(m: &rspice_core::netlist::ModelDef, key: &str) -> f64 {
    m.params
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(key))
        .unwrap_or_else(|| panic!("param {key} missing on {}", m.name))
        .1
}

#[test]
fn ako_inherits_type_and_params_with_overrides() {
    let netlist = parse(
        "ako\n\
         .model npnbase npn bf=100 is=1e-15 vaf=80\n\
         .model q1 ako:npnbase npn bf=250\n\
         .model q2 ako: npnbase (is=2e-15)\n\
         V1 a 0 1\n\
         R1 a 0 1k\n\
         .op\n\
         .end\n",
    );

    let derived = model(&netlist, "q1");
    assert!(derived.model_type.eq_ignore_ascii_case("npn"));
    assert_eq!(param(derived, "bf"), 250.0); // override wins
    assert_eq!(param(derived, "is"), 1e-15); // inherited
    assert_eq!(param(derived, "vaf"), 80.0); // inherited

    // Split `AKO: base` spelling, type omitted -> inherited.
    let derived2 = model(&netlist, "q2");
    assert!(derived2.model_type.eq_ignore_ascii_case("npn"));
    assert_eq!(param(derived2, "is"), 2e-15);
    assert_eq!(param(derived2, "bf"), 100.0);
}

#[test]
fn ako_requires_base_defined_earlier() {
    let err = Netlist::parse(
        "ako forward\n\
         .model q1 ako:notyet npn bf=250\n\
         .model notyet npn bf=100\n\
         .end\n",
    )
    .expect_err("forward AKO reference must fail");
    assert!(err.to_string().to_lowercase().contains("notyet"));
}

#[test]
fn ako_type_mismatch_is_rejected() {
    let err = Netlist::parse(
        "ako mismatch\n\
         .model base npn bf=100\n\
         .model bad ako:base pnp bf=50\n\
         .end\n",
    )
    .expect_err("type mismatch must fail");
    assert!(err.to_string().to_lowercase().contains("type"));
}

#[test]
fn inductor_rser_synthesizes_a_series_resistor() {
    let netlist = parse(
        "lt parasitics\n\
         V1 in 0 DC 1\n\
         L1 in out 10u Rser=2\n\
         R2 out 0 8\n\
         .op\n\
         .end\n",
    );

    // The inductor's positive terminal moved onto the internal node.
    let l1 = netlist
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("L1"))
        .expect("L1");
    assert_eq!(l1.nodes[0], "L1#SER");
    assert!(netlist.elements.iter().any(|e| e.name == "RL1#SER"
        && matches!(e.kind, ElementKind::Resistor { value, .. } if value == 2.0)));
    // Rser must not linger as an instance parameter.
    if let ElementKind::Inductor {
        instance_params, ..
    } = &l1.kind
    {
        assert!(instance_params.is_empty());
    }

    // DC: inductor shorts, so the divider is 2 / (2 + 8).
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("op solves");
    let out = op
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .map(|i| op.node_voltages[i])
        .expect("out node");
    assert!((out - 0.8).abs() < 1e-9, "v(out) = {out}");
}

#[test]
fn capacitor_rser_and_resistor_cpar_expand() {
    let netlist = parse(
        "esr\n\
         V1 a 0 DC 1\n\
         C1 a b 100n Rser=0.05\n\
         R1 b 0 1k Cpar=3p\n\
         .op\n\
         .end\n",
    );
    assert!(netlist.elements.iter().any(|e| e.name == "RC1#SER"));
    assert!(
        netlist
            .elements
            .iter()
            .any(|e| e.name == "CR1#PAR"
                && matches!(e.kind, ElementKind::Capacitor { value, .. } if (value - 3e-12).abs() < 1e-24))
    );
    // The parallel cap sits across R1's own terminals.
    let cpar = netlist
        .elements
        .iter()
        .find(|e| e.name == "CR1#PAR")
        .expect("CR1#PAR");
    assert_eq!(cpar.nodes, vec!["B".to_string(), "0".to_string()]);
}
