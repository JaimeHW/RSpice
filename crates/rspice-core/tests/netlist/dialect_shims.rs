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

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = parse(deck);
    let op = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("op solves");
    op.node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(node))
        .map(|i| op.node_voltages[i])
        .unwrap_or_else(|| panic!("node {node} missing"))
}

#[test]
fn ako_base_may_follow_the_derived_card() {
    let netlist = parse(
        "ako forward\n\
         .model q1 ako:later bf=250\n\
         .model later npn bf=100 vaf=80\n\
         .end\n",
    );
    let derived = model(&netlist, "q1");
    assert!(derived.model_type.eq_ignore_ascii_case("npn"));
    assert_eq!(param(derived, "bf"), 250.0);
    assert_eq!(param(derived, "vaf"), 80.0);

    // A chain written derived-first binds base-first: DC keeps its own RS
    // on top of DB's IS and DA's N.
    let chained = op_voltage(
        "ako chain\n\
         V1 a 0 1\n\
         R1 a b 1k\n\
         D1 b 0 DC\n\
         .model DC AKO:DB D(RS=10)\n\
         .model DB AKO:DA D(IS=1e-12)\n\
         .model DA D(IS=1e-14 N=2)\n\
         .op\n\
         .end\n",
        "b",
    );
    let direct = op_voltage(
        "direct\n\
         V1 a 0 1\n\
         R1 a b 1k\n\
         D1 b 0 DC\n\
         .model DC D(IS=1e-12 N=2 RS=10)\n\
         .op\n\
         .end\n",
        "b",
    );
    assert_eq!(chained, direct);
    // QSPICE 2026 batch: V(b) = 0.9329171736615484.
    assert!((chained - 0.932_917_173_7).abs() < 2e-6, "V(b) = {chained}");
}

#[test]
fn ako_base_resolves_in_the_card_s_subcircuit_first() {
    // The derived card precedes its local base, and a top-level model of the
    // same name exists: the subcircuit's own model is the base.
    let scoped = op_voltage(
        "ako scope\n\
         V1 a 0 1\n\
         R1 a b 1k\n\
         X1 b 0 sub\n\
         .subckt sub p n\n\
         D1 p n DB\n\
         .model DB AKO:DA D(IS=1e-12)\n\
         .model DA D(IS=1e-14 N=2)\n\
         .ends\n\
         .model DA D(IS=1e-14 N=1)\n\
         .op\n\
         .end\n",
        "b",
    );
    // LTspice 26.1.1 batch (QSPICE does not find subcircuit-local AKO bases):
    // V(b) = 0.9325385160036630, the IS=1e-12 N=2 diode.
    assert!((scoped - 0.932_538_516).abs() < 2e-6, "V(b) = {scoped}");

    // A local derived card falls back to a top-level base defined later.
    let global = op_voltage(
        "ako global base\n\
         V1 a 0 1\n\
         R1 a b 1k\n\
         X1 b 0 sub\n\
         .subckt sub p n\n\
         D1 p n DB\n\
         .model DB AKO:DA D(IS=1e-12)\n\
         .ends\n\
         .model DA D(IS=1e-14 N=2)\n\
         .op\n\
         .end\n",
        "b",
    );
    assert_eq!(global, scoped);
}

#[test]
fn ako_cycle_and_missing_base_are_rejected() {
    let cycle = Netlist::parse(
        "ako cycle\n\
         D1 b 0 DB\n\
         .model DB AKO:DA D(IS=1e-12)\n\
         .model DA AKO:DB D(N=2)\n\
         .end\n",
    )
    .expect_err("an AKO cycle must fail");
    assert!(
        cycle.to_string().contains("inherits from itself"),
        "{cycle}"
    );

    let missing = Netlist::parse(
        "ako missing\n\
         .model q1 ako:nowhere npn bf=250\n\
         .end\n",
    )
    .expect_err("an undefined AKO base must fail");
    assert!(
        missing.to_string().to_lowercase().contains("nowhere"),
        "{missing}"
    );
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
