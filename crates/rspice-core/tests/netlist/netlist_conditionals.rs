//! End-to-end tests for `.if` / `.elseif` / `.else` / `.endif` netlist
//! conditionals (ngspice numparam semantics: conditions evaluate against
//! the parameters known at that point in the deck).

use rspice_core::netlist::{ElementKind, Netlist};

fn resistor_names(netlist: &Netlist) -> Vec<String> {
    netlist
        .elements
        .iter()
        .filter(|e| matches!(e.kind, ElementKind::Resistor { .. }))
        .map(|e| e.name.to_uppercase())
        .collect()
}

#[test]
fn if_selects_the_true_branch() {
    let deck = "\
* basic conditional
.param sel=1
.if (sel==1)
r_taken 1 0 1k
.else
r_skipped 1 0 2k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_TAKEN"]);
}

#[test]
fn else_branch_fires_when_condition_is_false() {
    let deck = "\
* else branch
.param sel=0
.if (sel==1)
r_skipped 1 0 1k
.else
r_taken 1 0 2k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_TAKEN"]);
}

#[test]
fn elseif_chain_selects_exactly_one_branch() {
    let deck = "\
* elseif chain
.param corner=2
.if (corner==1)
r_ff 1 0 1k
.elseif (corner==2)
r_tt 1 0 2k
.elseif (corner==3)
r_ss 1 0 3k
.else
r_default 1 0 4k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_TT"]);
}

#[test]
fn later_true_conditions_do_not_reopen_a_taken_chain() {
    let deck = "\
* first true branch wins
.param x=1
.if (x==1)
r_first 1 0 1k
.elseif (x>=1)
r_second 1 0 2k
.else
r_third 1 0 3k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_FIRST"]);
}

#[test]
fn nested_conditionals_compose() {
    let deck = "\
* nesting
.param outer=1 inner=0
.if (outer==1)
r_outer 1 0 1k
.if (inner==1)
r_inner_taken 2 0 1k
.else
r_inner_else 2 0 2k
.endif
.elseif (outer==2)
r_unreached 3 0 3k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_OUTER", "R_INNER_ELSE"]);
}

#[test]
fn nested_blocks_inside_false_branches_stay_suppressed() {
    // The inner `.if (1)` is inside a false outer branch: it must be
    // tracked for nesting but can never activate, and its condition may
    // reference parameters that do not exist.
    let deck = "\
* suppressed nesting
.param sel=0
.if (sel==1)
.if (never_defined_param > 0)
r_ghost 1 0 1k
.endif
r_also_ghost 1 0 1k
.endif
r_real 1 0 2k
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_REAL"]);
}

#[test]
fn params_in_false_branches_are_not_defined() {
    let deck = "\
* param suppression
.param sel=0
.if (sel==1)
.param ghost=42
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(netlist.params.get("ghost"), None);
}

#[test]
fn models_and_subcircuits_obey_conditionals() {
    // The sky130-style corner pattern: pick a model card by parameter.
    let deck = "\
* conditional model selection
.param fast=0
.if (fast==1)
.model nfet nmos (vto=0.4)
.else
.model nfet nmos (vto=0.7)
.endif
m1 d g 0 0 nfet
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(netlist.models.len(), 1, "exactly one model card survives");
    let vto = netlist.models[0]
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("vto"))
        .map(|(_, v)| *v);
    assert_eq!(vto, Some(0.7));
}

#[test]
fn conditions_use_parameters_defined_earlier() {
    let deck = "\
* expression conditions without parentheses
.param vdd=3.3
.if vdd>3
r_hv 1 0 1k
.endif
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_HV"]);
}

#[test]
fn continuation_lines_inside_false_branches_are_skipped() {
    let deck = "\
* continuations in dead branch
.param sel=0
.if (sel==1)
r_ghost 1 0
+ 1k
.endif
r_real 1 0 2k
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(resistor_names(&netlist), vec!["R_REAL"]);
}

#[test]
fn structural_errors_are_rejected() {
    let unbalanced = "* unbalanced\n.if (1)\nr1 1 0 1k\n.end\n";
    assert!(Netlist::parse(unbalanced).is_err(), "missing .endif");

    let stray_endif = "* stray\n.endif\n.end\n";
    assert!(Netlist::parse(stray_endif).is_err(), "stray .endif");

    let stray_else = "* stray\n.else\n.end\n";
    assert!(Netlist::parse(stray_else).is_err(), "stray .else");

    let elseif_after_else = "\
* misordered
.if (0)
.else
.elseif (1)
.endif
.end
";
    assert!(
        Netlist::parse(elseif_after_else).is_err(),
        ".elseif after .else"
    );

    let double_else = "\
* double else
.if (0)
.else
.else
.endif
.end
";
    assert!(Netlist::parse(double_else).is_err(), "duplicate .else");

    let undefined_in_active = "\
* undefined param in live condition
.if (no_such_param==1)
r1 1 0 1k
.endif
.end
";
    assert!(
        Netlist::parse(undefined_in_active).is_err(),
        "undefined parameter in an active condition must error"
    );
}

#[test]
fn conditionals_compose_with_statistical_seeding() {
    // Conditional selection of a seeded statistical parameter: the pieces
    // must work together since PDK decks combine them freely.
    let deck = "\
* combined
.options seed=9
.param mc=1
.if (mc==1)
.param rval={agauss(1k,50,1)}
.else
.param rval=1k
.endif
.end
";
    let first = Netlist::parse(deck).expect("parse");
    let rval = first.params.get("rval").expect("rval defined");
    assert!((rval - 1000.0).abs() < 250.0, "draw {rval} implausible");
    assert_ne!(rval, 1000.0, "mc branch must actually draw");

    let replay = Netlist::parse(deck).expect("parse");
    assert_eq!(first.params.get("rval"), replay.params.get("rval"));
}
