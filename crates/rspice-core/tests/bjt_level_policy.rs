//! Build-time policy for BJT model levels.
//!
//! Native GP (including ngspice LEVEL=2) and VBIC decks should keep running,
//! but advanced BJT families without native implementations must not be
//! silently evaluated as VBIC or legacy GP.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{ElementKind, Netlist};

const VBIC13_TEST_VBBE: f64 = 2.0;
const VBIC13_TEST_NBBE: f64 = 5.0;
const VBIC13_TEST_IBBE: f64 = 1e-9;
const VBIC13_TEST_VBE: f64 = -2.4;

fn op_deck(model_line: &str) -> String {
    format!(
        "* bjt level policy\n\
         vc c 0 dc 1.0\n\
         vb b 0 dc 0.7\n\
         q1 c b 0 qmod\n\
         {model_line}\n\
         .op\n\
         .end\n"
    )
}

fn inert_vbic13_params() -> &'static str {
    "VBBE=0 NBBE=1 IBBE=1e-6 TVBBE1=0 TVBBE2=0 TNBBE=0 EBBE=0"
}

fn active_vbic13_wbe1_breakdown_deck(level: i32, vbe: f64) -> String {
    active_vbic13_wbe1_breakdown_deck_with_extra(level, vbe, "")
}

fn active_vbic13_wbe1_breakdown_deck_with_options(
    level: i32,
    vbe: f64,
    options: &str,
    extra_params: &str,
) -> String {
    let options = if options.is_empty() {
        String::new()
    } else {
        format!(" {options}")
    };
    format!(
        "* VBIC13 reverse B-E breakdown, WBE=1 Xyce/ngspice agreement path\n\
         .options gmin=0 reltol=1e-9 abstol=1e-15 vntol=1e-12{options}\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbe}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL={level} IS=1e-30 ISRR=0 IBEI=0 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX=0 RBI=0 RE=0 WBE=1 VBBE={VBIC13_TEST_VBBE} \
         NBBE={VBIC13_TEST_NBBE} IBBE={VBIC13_TEST_IBBE} {extra_params})\n\
         .op\n\
         .end\n"
    )
}

fn active_vbic13_wbe1_breakdown_deck_with_extra(
    level: i32,
    vbe: f64,
    extra_params: &str,
) -> String {
    active_vbic13_wbe1_breakdown_deck_with_options(level, vbe, "", extra_params)
}

fn vbic13_wbe1_reverse_be_source_current(vbe: f64) -> f64 {
    vbic13_wbe1_reverse_be_source_current_at(
        vbe,
        rspice_core::constants::TEMP_REFERENCE,
        0.0,
        0.0,
        0.0,
    )
}

fn vbic13_wbe1_reverse_be_source_current_at(
    vbe: f64,
    temp_k: f64,
    tvbbe1: f64,
    tvbbe2: f64,
    tnbbe: f64,
) -> f64 {
    let delta_t = temp_k - rspice_core::constants::TEMP_REFERENCE;
    let vt = rspice_core::analysis::temperature::thermal_voltage(temp_k);
    let vbbe = VBIC13_TEST_VBBE * (1.0 + delta_t * (tvbbe1 + delta_t * tvbbe2));
    let nbbe = VBIC13_TEST_NBBE * (1.0 + delta_t * tnbbe);
    let exponent = (-vbbe - vbe) / (nbbe * vt);
    let ebbe = (-vbbe / (nbbe * vt)).exp();
    VBIC13_TEST_IBBE * (exponent.exp() - ebbe)
}

fn xyce_vbic_oracle_deck(level: i32) -> String {
    xyce_vbic_oracle_deck_with_extra_params(level, "")
}

fn xyce_vbic_oracle_deck_with_extra_params(level: i32, extra_params: &str) -> String {
    let substrate = if level == 12 {
        "vs s 0 dc 0\nq1 c b e s qmod\n"
    } else {
        "q1 c b e qmod\n"
    };
    format!(
        "* Xyce 7.10 VBIC LEVEL={level} operating-point oracle\n\
         vc c 0 dc 1.2\n\
         vb b 0 dc 0.78\n\
         ve e 0 dc 0\n\
         {substrate}\
         .model qmod npn level={level} IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 \
         ISP=1e-15 RCX=1 RCI=2 RBX=1 RBI=2 RE=1 RS=0 RBP=1 VEF=10 VER=4 IKF=2e-3 \
         ITF=8e-2 XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 \
         CJCP=4e-13 VO=2 GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 \
         TR=100e-12 TD=0 RTH=0 GMIN=0.0 {extra_params}\n\
         .op\n\
         .end\n"
    )
}

fn run(deck: &str) -> Result<(), String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn branch_current(deck: &str, branch: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("op converges");
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing {branch} branch in {:?}", result.branch_names))
}

fn op_result(deck: &str) -> rspice_core::solver::SimulationResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("op converges")
}

fn voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing {node} node in {:?}", result.node_names))
}

fn assert_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64) {
    let rel = (got - expected).abs() / expected.abs().max(1e-30);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.9e}, expected {expected:.9e}, rel {rel:.3e} > {rel_tol:.3e}"
    );
}

fn build(deck: &str) -> Result<rspice_core::CircuitData, String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .map_err(|err| err.to_string())
}

fn ac_branch_current(deck: &str, branch: &str, frequency: f64) -> rspice_core::Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("AC converges")
        .pop()
        .expect("one AC point");
    let index = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing {branch} branch in {:?}", result.branch_names));
    result.currents[index]
}

#[test]
fn five_terminal_bjt_syntax_keeps_extra_nodes_and_model_name() {
    let deck = "* five-terminal BJT syntax\n\
                q1 coll base emit subs aux qmod\n\
                .model qmod NPN (IS=1e-16 BF=100)\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");

    assert_eq!(
        q1.nodes,
        vec![
            "COLL".to_string(),
            "BASE".to_string(),
            "EMIT".to_string(),
            "SUBS".to_string(),
            "AUX".to_string()
        ]
    );
    match &q1.kind {
        ElementKind::Bjt { model, .. } => assert!(
            model.eq_ignore_ascii_case("qmod"),
            "expected BJT model name qmod, got {model}"
        ),
        other => panic!("expected BJT element, got {other:?}"),
    }
}

#[test]
fn bracketed_bjt_substrate_node_matches_unbracketed_ngspice_form() {
    let unbracketed = xyce_vbic_oracle_deck(12);
    let bracketed = unbracketed.replace("q1 c b e s qmod", "q1 c b e [s] qmod");
    let netlist = Netlist::parse(&bracketed).expect("bracketed substrate BJT deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");
    assert_eq!(
        q1.nodes,
        vec![
            "C".to_string(),
            "B".to_string(),
            "E".to_string(),
            "S".to_string()
        ]
    );

    for branch in ["vc", "vb", "ve"] {
        assert_rel_close(
            &format!("bracketed BJT substrate {branch} current"),
            branch_current(&bracketed, branch),
            branch_current(&unbracketed, branch),
            1.0e-12,
        );
    }
}

#[test]
fn bracketed_bjt_ground_substrate_is_accepted_as_ground_node() {
    let deck = xyce_vbic_oracle_deck(11).replace("q1 c b e qmod", "q1 c b e [0] qmod");
    let netlist = Netlist::parse(&deck).expect("bracketed ground substrate BJT deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");
    assert_eq!(
        q1.nodes,
        vec![
            "C".to_string(),
            "B".to_string(),
            "E".to_string(),
            "0".to_string()
        ]
    );
    run(&deck).expect("bracketed ground-substrate BJT deck runs");
}

#[test]
fn cmc_bjt_levels_fail_closed_as_veriloga_codegen_targets() {
    for (level, family) in [
        (8, "HICUM"),
        (230, "HICUM"),
        (234, "HICUM"),
        (504, "MEXTRAM"),
        (505, "MEXTRAM"),
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} VERSION=2.40 IS=1e-16 BF=100)"
        ));
        let message = run(&deck).expect_err("CMC BJT level must fail closed");

        assert!(
            message.contains(&format!("LEVEL={level}")) && message.contains(family),
            "LEVEL={level} error should identify {family}: {message}"
        );
        assert!(
            message.contains("Verilog-A-to-Rust") && message.contains("codegen"),
            "LEVEL={level} error should describe the codegen path: {message}"
        );
        assert!(
            message.contains("must not fall back"),
            "LEVEL={level} error should forbid fallback routing: {message}"
        );
    }
}

#[test]
fn unsupported_advanced_bjt_level_is_rejected() {
    let level = 235;
    let deck = op_deck(&format!(
        ".model qmod NPN (LEVEL={level} VERSION=2.34 IS=1e-16 BF=100)"
    ));
    let message = run(&deck).expect_err(&format!("unsupported LEVEL={level} must not run as VBIC"));
    assert!(
        message.contains("BJT"),
        "error identifies the device family: {message}"
    );
    assert!(
        message.contains(&format!("LEVEL={level}")),
        "error names the unsupported level: {message}"
    );
    assert!(
        message.contains("unsupported") || message.contains("no native implementation"),
        "error explains the unsupported routing: {message}"
    );
    assert!(
        message.contains("generated from Verilog-A") || message.contains("no native"),
        "error should point to future generated support instead of fallback routing: {message}"
    );
}

#[test]
fn xyce_hbt_x_level23_fails_closed_by_name() {
    let deck = op_deck(".model qmod NPN (LEVEL=23)");
    let message = run(&deck).expect_err("Xyce HBT_X LEVEL=23 must fail closed by name");

    assert!(
        message.contains("Xyce")
            && message.contains("HBT_X")
            && message.contains("LEVEL=23")
            && message.contains("native"),
        "HBT_X error should identify the unsupported Xyce native model family: {message}"
    );
    assert!(
        message.contains("Verilog-A"),
        "HBT_X error should make the no-Verilog-A-fallback rule explicit: {message}"
    );
}

#[test]
fn ngspice_bjt_level2_runs_as_legacy_gummel_poon_alias() {
    let deck = op_deck(".model qmod NPN (LEVEL=2 IS=1e-16 BF=100 VAF=50)");
    run(&deck).expect("ngspice BJT LEVEL=2 alias must run on the native GP path");

    // ngspice 46 maps LEVEL=2 to its legacy BJT device for this card.
    assert_rel_close(
        "LEVEL=2 I(VC)",
        branch_current(&deck, "vc"),
        -5.704_37e-5,
        1.0e-5,
    );
    assert_rel_close(
        "LEVEL=2 I(VB)",
        branch_current(&deck, "vb"),
        -5.670_35e-7,
        1.2e-5,
    );
}

#[test]
fn level1_gummel_poon_bias_dependent_transit_time_parameters_stay_legacy() {
    let deck_for = |transit_params: &str| {
        format!(
            "* SPICE Gummel-Poon bias-dependent forward transit time\n\
             vc c 0 dc 5\n\
             vb b 0 dc 0.7 ac 1\n\
             q1 c b 0 qmod\n\
             .model qmod NPN (IS=1e-16 BF=100 ISE=1e-16 NE=1.5 \
             VAF=50 TF=1n {transit_params})\n\
             .op\n\
             .end\n"
        )
    };
    let ordinary = deck_for("");
    let bias_dependent = deck_for("XTF=3 VTF=10 ITF=0");

    // XTF/VTF/ITF alter only stored charge. An implicit level-1 card carrying
    // them must retain the same Gummel-Poon DC transport and base-current model.
    for branch in ["vc", "vb"] {
        assert_rel_close(
            &format!("level-1 GP {branch} DC current"),
            branch_current(&bias_dependent, branch),
            branch_current(&ordinary, branch),
            1.0e-12,
        );
    }

    // The parameters must still participate in the legacy dynamic-charge
    // equation: with ITF=0, XTF scales TF by the VBC-dependent factor.
    let frequency = 1.0e8;
    let ordinary_input = ac_branch_current(&ordinary, "vb", frequency);
    let bias_dependent_input = ac_branch_current(&bias_dependent, "vb", frequency);
    assert!(
        bias_dependent_input.im.abs() > ordinary_input.im.abs() * 1.5,
        "XTF/VTF/ITF should increase the level-1 GP input charge current: \
         ordinary={ordinary_input:?}, bias-dependent={bias_dependent_input:?}"
    );
}

#[test]
fn ngspice_lpnp_model_type_routes_to_native_lateral_pnp() {
    let deck_for = |model_type: &str| {
        format!(
            "* ngspice {model_type} legacy BJT alias\n\
             vc c 0 dc -1.0\n\
             vb b 0 dc -0.7\n\
             q1 c b 0 qmod\n\
             .model qmod {model_type} (IS=1e-16 BF=100 VAF=50)\n\
             .op\n\
             .end\n"
        )
    };
    let lpnp = deck_for("LPNP");
    let pnp = deck_for("PNP");

    // ngspice rewrites LPNP to PNP with lateral substrate semantics; native
    // PNP already defaults to the same lateral topology in RSpice.
    assert_rel_close(
        "LPNP I(VC)",
        branch_current(&lpnp, "vc"),
        branch_current(&pnp, "vc"),
        1.0e-12,
    );
    assert_rel_close(
        "LPNP I(VB)",
        branch_current(&lpnp, "vb"),
        branch_current(&pnp, "vb"),
        1.0e-12,
    );
}

#[test]
fn ngspice_lpnp_alias_rejects_nonlegacy_bjt_levels_until_oracled() {
    let deck = op_deck(".model qmod LPNP (LEVEL=4 IS=1e-16 IBEI=1e-18 IBCI=1e-18)");
    let message =
        run(&deck).expect_err("LPNP must not silently select non-legacy native BJT levels");

    assert!(
        message.contains("LPNP") && message.contains("LEVEL=4") && message.contains("legacy"),
        "LPNP advanced-level error should name the legacy alias boundary: {message}"
    );
}

#[test]
fn unresolved_bjt_level_selector_fails_closed() {
    let deck = op_deck(".model qmod NPN (LEVEL={native_level} IS=1e-16 BF=100)");
    let message =
        run(&deck).expect_err("unresolved BJT LEVEL selector must not fall back to legacy GP");

    assert!(
        message.contains("BJT") && message.contains("LEVEL"),
        "error should identify the BJT LEVEL selector: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite numeric literal"),
        "error should explain unresolved selectors must be finite numeric literals: {message}"
    );
}

#[test]
fn non_numeric_bjt_level_selector_fails_closed() {
    let deck = op_deck(".model qmod NPN (LEVEL=\"504\" IS=1e-16 BF=100)");
    let message =
        run(&deck).expect_err("non-numeric BJT LEVEL selector must not fall back to legacy GP");

    assert!(
        message.contains("BJT") && message.contains("LEVEL"),
        "error should identify the BJT LEVEL selector: {message}"
    );
    assert!(
        message.contains("non-numeric") && message.contains("finite numeric literal"),
        "error should explain string selectors must be finite numeric literals: {message}"
    );
}

#[test]
fn vbic_level11_rejects_unresolved_native_model_params() {
    let deck = op_deck(".model qmod NPN (LEVEL=11 IS={is_native} IBEI=1e-18 IBCI=1e-18 RCX=1)");
    let message = run(&deck).expect_err("unresolved VBIC model parameter must fail closed");

    assert!(
        message.contains("VBIC") && message.contains("IS"),
        "error should identify the unresolved VBIC model parameter: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite numeric literal"),
        "error should explain native VBIC model parameters must be numeric: {message}"
    );
}

#[test]
fn vbic_level4_still_runs_natively() {
    let deck = op_deck(".model qmod NPN (LEVEL=4 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1)");
    run(&deck).expect("LEVEL=4 remains the native VBIC selector");
}

#[test]
fn xyce_vbic_levels_11_and_12_run_natively() {
    for level in [11, 12] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2)"
        ));
        run(&deck).unwrap_or_else(|err| panic!("Xyce VBIC LEVEL={level} must run natively: {err}"));
    }
}

#[test]
fn xyce_vbic_levels_11_and_12_use_vbic_internal_topology() {
    for level in [11, 12] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 RE=1)"
        ));
        let circuit = build(&deck)
            .unwrap_or_else(|err| panic!("Xyce VBIC LEVEL={level} circuit builds: {err}"));
        for node in [
            "q1.__cx.internal",
            "q1.__ci.internal",
            "q1.__bx.internal",
            "q1.__bi.internal",
            "q1.__ei.internal",
        ] {
            assert!(
                circuit.get_node_by_name(node).is_some(),
                "LEVEL={level} should allocate VBIC internal node {node}"
            );
        }
    }
}

#[test]
fn ngspice_vbic_level9_uses_vbic_internal_topology() {
    let deck = op_deck(
        ".model qmod NPN (LEVEL=9 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 RE=1)",
    );
    let circuit = build(&deck).expect("ngspice VBIC LEVEL=9 alias circuit builds natively");
    for node in [
        "q1.__cx.internal",
        "q1.__ci.internal",
        "q1.__bx.internal",
        "q1.__bi.internal",
        "q1.__ei.internal",
    ] {
        assert!(
            circuit.get_node_by_name(node).is_some(),
            "LEVEL=9 should allocate VBIC internal node {node}"
        );
    }
}

#[test]
fn xyce_vbic_levels_11_12_and_13_match_xyce710_dc_op_oracle() {
    // XyceNF 7.10, same decks with `.print dc V(c) V(b) V(e) I(vc) I(vb) I(ve)`.
    for level in [11, 12, 13] {
        let deck = xyce_vbic_oracle_deck(level);
        assert_rel_close(
            &format!("LEVEL={level} I(VC)"),
            branch_current(&deck, "vc"),
            -7.468_978_34e-4,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} I(VB)"),
            branch_current(&deck, "vb"),
            -1.213_920_45e-5,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} I(VE)"),
            branch_current(&deck, "ve"),
            7.590_370_38e-4,
            1.0e-4,
        );
    }
}

#[test]
fn inert_vbic13_zero_breakdown_params_run_on_native_vbic_levels() {
    for level in [4, 9, 11, 12, 13] {
        let base = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2)"
        ));
        let inert = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 {})",
            inert_vbic13_params()
        ));

        run(&inert).unwrap_or_else(|err| {
            panic!("inert VBIC13 zero-breakdown params should run on LEVEL={level}: {err}")
        });
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 I(VC)"),
            branch_current(&inert, "vc"),
            branch_current(&base, "vc"),
            1.0e-10,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 I(VB)"),
            branch_current(&inert, "vb"),
            branch_current(&base, "vb"),
            1.0e-10,
        );
    }

    for level in [11, 12, 13] {
        let deck = xyce_vbic_oracle_deck_with_extra_params(level, inert_vbic13_params());
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VC)"),
            branch_current(&deck, "vc"),
            -7.468_978_34e-4,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VB)"),
            branch_current(&deck, "vb"),
            -1.213_920_45e-5,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VE)"),
            branch_current(&deck, "ve"),
            7.590_370_38e-4,
            1.0e-4,
        );
    }
}

#[test]
fn active_vbic13_wbe1_reverse_be_breakdown_matches_xyce_ngspice_equation() {
    let expected = vbic13_wbe1_reverse_be_source_current(VBIC13_TEST_VBE);
    for level in [4, 11, 12, 13] {
        let deck = active_vbic13_wbe1_breakdown_deck(level, VBIC13_TEST_VBE);
        assert_rel_close(
            &format!("LEVEL={level} active VBIC13 WBE=1 I(VB)"),
            branch_current(&deck, "vb"),
            expected,
            2.0e-5,
        );
    }
}

#[test]
fn vbic13_ebbe_parameter_is_recomputed_from_vbbe_nbbe() {
    let base = active_vbic13_wbe1_breakdown_deck(11, VBIC13_TEST_VBE);
    let with_legacy_ebbe =
        active_vbic13_wbe1_breakdown_deck_with_extra(11, VBIC13_TEST_VBE, "EBBE=0.25");

    assert_rel_close(
        "finite user EBBE should not perturb recomputed VBIC13 breakdown",
        branch_current(&with_legacy_ebbe, "vb"),
        branch_current(&base, "vb"),
        1.0e-12,
    );
}

fn active_vbic13_wbe_split_breakdown_deck(wbe: f64, rbx: f64, rbi: f64, vbase: f64) -> String {
    format!(
        "* VBIC13 reverse B-E breakdown, Xyce-primary WBE split path\n\
         .options gmin=0 reltol=1e-10 abstol=1e-15 vntol=1e-12\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbase}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL=11 IS=1e-30 ISRR=0 IBEI=0 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX={rbx} RBI={rbi} RE=0 RBP=0 WBE={wbe} \
         VBBE={VBIC13_TEST_VBBE} NBBE={VBIC13_TEST_NBBE} IBBE={VBIC13_TEST_IBBE})\n\
         .op\n\
         .end\n"
    )
}

fn vbic13_reverse_be_breakdown_source_current(vbe: f64) -> f64 {
    vbic13_wbe1_reverse_be_source_current(vbe)
}

fn solve_xyce_split_vbi(vbase: f64, wbe: f64, rbx: f64, rbi: f64) -> f64 {
    let resistance = rbx + wbe * rbi;
    let mut lo = vbase;
    let mut hi = vbase + 0.5;
    for _ in 0..160 {
        let mid = 0.5 * (lo + hi);
        let residual = mid - vbase - resistance * vbic13_reverse_be_breakdown_source_current(mid);
        if residual > 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

fn ordinary_vbic_wbe_split_deck(wbe: f64, rbx: f64, rbi: f64, vbase: f64) -> String {
    format!(
        "* VBIC ordinary B-E current, Xyce WBE split path\n\
         .options gmin=0 reltol=1e-10 abstol=1e-15 vntol=1e-12\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbase}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL=11 IS=1e-30 ISRR=0 IBEI=1e-18 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX={rbx} RBI={rbi} RE=0 RBP=0 WBE={wbe} VBBE=0)\n\
         .op\n\
         .end\n"
    )
}

fn vbic_ordinary_ibe_current(vbe: f64) -> (f64, f64) {
    let vt =
        rspice_core::analysis::temperature::thermal_voltage(rspice_core::constants::TEMP_REFERENCE);
    let exp_vbe = (vbe / vt).exp();
    (1.0e-18 * (exp_vbe - 1.0), 1.0e-18 * exp_vbe / vt)
}

fn solve_xyce_ordinary_wbe_split(vbase: f64, wbe: f64, rbx: f64, rbi: f64) -> (f64, f64, f64) {
    let mut vbx = vbase;
    let mut vbi = vbase;
    for _ in 0..80 {
        let (ibe_bx, gbe_bx) = vbic_ordinary_ibe_current(vbx);
        let (ibe_bi, gbe_bi) = vbic_ordinary_ibe_current(vbi);
        let f1 = (vbase - vbx) / rbx - wbe * ibe_bi - (1.0 - wbe) * ibe_bx;
        let f2 = (vbx - vbi) / rbi - wbe * ibe_bi;
        if f1.abs().max(f2.abs()) < 1.0e-18 {
            break;
        }

        let a = -1.0 / rbx - (1.0 - wbe) * gbe_bx;
        let b = -wbe * gbe_bi;
        let c = 1.0 / rbi;
        let d = -1.0 / rbi - wbe * gbe_bi;
        let det = a * d - b * c;
        assert!(
            det.abs() > 1.0e-30,
            "ordinary WBE split oracle Jacobian singular"
        );
        let delta_vbx = (-f1 * d + b * f2) / det;
        let delta_vbi = (c * f1 - a * f2) / det;
        vbx += delta_vbx.clamp(-0.05, 0.05);
        vbi += delta_vbi.clamp(-0.05, 0.05);
    }

    let (ibe_bx, _) = vbic_ordinary_ibe_current(vbx);
    let (ibe_bi, _) = vbic_ordinary_ibe_current(vbi);
    let total = wbe * ibe_bi + (1.0 - wbe) * ibe_bx;
    (vbx, vbi, total)
}

#[test]
fn active_vbic13_wbe_split_collapsed_topology_preserves_total_breakdown_current() {
    let deck = active_vbic13_wbe_split_breakdown_deck(0.5, 0.0, 0.0, VBIC13_TEST_VBE);

    assert_rel_close(
        "active VBIC13 split WBE collapsed I(VB)",
        branch_current(&deck, "vb"),
        vbic13_wbe1_reverse_be_source_current(VBIC13_TEST_VBE),
        2.0e-5,
    );
}

#[test]
fn ordinary_vbic_wbe_split_uses_vbx_for_extrinsic_be_current() {
    let vbase = 0.62;
    let wbe = 0.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = ordinary_vbic_wbe_split_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let (expected_vbx, expected_vbi, expected_current) =
        solve_xyce_ordinary_wbe_split(vbase, wbe, rbx, rbi);

    assert_rel_close(
        "ordinary VBIC WBE split I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        -expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "ordinary VBIC WBE split bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "ordinary VBIC WBE split bi voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
    assert!(
        (expected_vbx - expected_vbi).abs() > 1.0e-3,
        "test must separate bx from bi enough to prove ordinary WBE split placement"
    );
}

#[test]
fn ordinary_vbic_wbe_uses_literal_out_of_range_value_like_xyce() {
    let vbase = 0.62;
    let wbe = 1.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = ordinary_vbic_wbe_split_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let (expected_vbx, expected_vbi, expected_current) =
        solve_xyce_ordinary_wbe_split(vbase, wbe, rbx, rbi);

    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        -expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE bi voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
}

#[test]
fn active_vbic13_wbe_split_uses_bx_to_ei_injection_controlled_by_vbi() {
    let vbase = -2.5;
    let wbe = 0.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = active_vbic13_wbe_split_breakdown_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let expected_vbi = solve_xyce_split_vbi(vbase, wbe, rbx, rbi);
    let expected_current = vbic13_reverse_be_breakdown_source_current(expected_vbi);
    let expected_vbx = vbase + rbx * expected_current;

    assert_rel_close(
        "active VBIC13 split WBE non-collapsed I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "active VBIC13 split WBE bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "active VBIC13 split WBE bi control voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
    assert!(
        (voltage(&result, "q1.__bi.internal") - voltage(&result, "q1.__bx.internal")).abs()
            > 1.0e-3,
        "test must separate bx from bi enough to prove control-node placement"
    );
}

#[test]
fn active_vbic13_temperature_coefficients_follow_xyce_ngspice_formula() {
    let temp_c = 75.0;
    let temp_k = temp_c + 273.15;
    let vbe = -2.5;
    let tvbbe1 = 1.0e-3;
    let tvbbe2 = 1.0e-6;
    let tnbbe = 5.0e-4;
    let deck = active_vbic13_wbe1_breakdown_deck_with_options(
        11,
        vbe,
        &format!("temp={temp_c}"),
        &format!("TVBBE1={tvbbe1} TVBBE2={tvbbe2} TNBBE={tnbbe}"),
    );

    assert_rel_close(
        "active VBIC13 temperature-scaled I(VB)",
        branch_current(&deck, "vb"),
        vbic13_wbe1_reverse_be_source_current_at(vbe, temp_k, tvbbe1, tvbbe2, tnbbe),
        2.0e-5,
    );
}

#[test]
fn vbic13_specific_parameters_reject_fail_closed_when_not_inert_or_not_native_vbic() {
    for level_prefix in ["", "LEVEL=1"] {
        let separator = if level_prefix.is_empty() { "" } else { " " };
        let deck = op_deck(&format!(
            ".model qmod NPN ({level_prefix}{separator}IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
            inert_vbic13_params()
        ));
        let message =
            run(&deck).expect_err("VBIC13-only params must not be accepted on legacy GP cards");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        for param in ["VBBE", "NBBE", "IBBE", "TVBBE1", "TVBBE2", "TNBBE", "EBBE"] {
            assert!(
                message.contains(param),
                "error lists unsupported VBIC13 parameter {param}: {message}"
            );
        }
        assert!(
            message.contains("not implemented") || message.contains("unsupported"),
            "error explains fail-closed routing: {message}"
        );
        assert!(
            !message.contains("selects native VBIC"),
            "error should not imply legacy/no-level cards selected native VBIC: {message}"
        );
    }

    for (param, value) in [
        ("VBBE", "-1.0"),
        ("NBBE", "0"),
        ("NBBE", "-1"),
        ("IBBE", "0"),
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {param}={value})"
        ));
        let message =
            run(&deck).expect_err("invalid VBIC13 reverse B-E breakdown params must reject");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        assert!(
            message.contains(param),
            "error names the active unsupported parameter {param}: {message}"
        );
        assert!(
            message.contains("breakdown") || message.contains("not implemented"),
            "error explains active physics is unavailable: {message}"
        );
    }

    let deck = op_deck(&format!(
        ".model qmod NPN (LEVEL=234 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
        inert_vbic13_params()
    ));
    let message = run(&deck).expect_err("VBIC13 params must reject on unsupported BJT levels");
    assert!(
        message.contains("LEVEL=4")
            && message.contains("LEVEL=9")
            && message.contains("LEVEL=11")
            && message.contains("LEVEL=12")
            && message.contains("LEVEL=13"),
        "error should name the native VBIC-only acceptance scope: {message}"
    );
}

#[test]
fn vbic13_specific_parameters_reject_when_not_finite_numeric_literals() {
    for extra in [
        "VBBE={VBREAK}",
        "IBBE={IBBE_SCALE}",
        "VBBE=\"active\"",
        "NBBE=\"1\"",
        "VBBE={1/0}",
        "VBBE=2 NBBE=5 IBBE=1e-9 WBE={SPLIT}",
        "VBBE=2 NBBE=5 IBBE=1e-9 WBE=\"0.5\"",
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {extra})"
        ));
        let message =
            run(&deck).expect_err("VBIC13 params must be finite numeric literals to be accepted");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        assert!(
            message.contains("finite")
                || message.contains("numeric")
                || message.contains("unresolved"),
            "error explains finite numeric requirement: {message}"
        );
    }
}

#[test]
fn vbic13_inert_params_require_exact_native_vbic_level_selectors() {
    for level in [
        "4.0000000005",
        "11.0000000005",
        "12.0000000005",
        "13.0000000005",
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
            inert_vbic13_params()
        ));
        let message =
            run(&deck).expect_err("near-integer BJT levels must not select native VBIC routing");
        assert!(
            message.contains("VBIC13") || message.contains("LEVEL"),
            "error explains rejected selector or VBIC13 routing: {message}"
        );
    }
}

#[test]
fn legacy_bjt_levels_still_run() {
    for model_line in [
        ".model qmod NPN (IS=1e-16 BF=100)",
        ".model qmod NPN (LEVEL=0 IS=1e-16 BF=100)",
        ".model qmod NPN (LEVEL=1 IS=1e-16 BF=100)",
    ] {
        let deck = op_deck(model_line);
        run(&deck).unwrap_or_else(|err| panic!("{model_line} must remain legacy GP: {err}"));
    }
}
