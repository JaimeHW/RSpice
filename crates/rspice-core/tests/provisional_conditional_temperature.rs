//! Unresolved conditional decisions must be confirmed at selected temperatures.
use rspice_core::Netlist;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_if_reaches_a_later_scalar_temperature() {
    let netlist = Netlist::parse(
        "Provisional conditional\n.if (1/(TEMP-27))\n.param selected=1\n.else\n.param selected=2\n.endif\n.temp 85\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.params.get("selected"), Some(1.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_elseif_reaches_forward_nominal_options() {
    let netlist = Netlist::parse(
        "Provisional elseif\n.if 0\n.param selected=1\n.elseif (1/(TNOM-27)>0)\n.param selected=2\n.else\n.param selected=3\n.endif\n.options tnom={nominal}\n.param nominal=55\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.tnom, Some(55.0));
    assert_eq!(netlist.params.get("selected"), Some(2.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_failed_decision_can_resolve_false_and_select_only_else() {
    let netlist = Netlist::parse("Resolved false\n.if (1/(TEMP-27)<0)\n.param selected=1\n.elseif 0\n.param selected=2\n.else\n.param selected=3\n.endif\n.temp 85\n.end\n").unwrap();
    assert_eq!(netlist.params.get("selected"), Some(3.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nested_and_scoped_decisions_reach_completed_parent_temperatures() {
    let netlist = Netlist::parse(
        "Scoped decision\n.subckt parent p\n.param offset=27\n.subckt child q\n.if 1\n.if (1/(TEMP-offset)>0)\nRchosen q 0 1k\n.elseif missing\nRwrong q 0 2k\n.endif\n.endif\n.options temp={ambient}\n.ends\n.param ambient=85\n.ends\nX1 out parent\nI1 0 out 1m\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    let child = netlist
        .subcircuits
        .iter()
        .find(|s| s.name.ends_with("child"))
        .unwrap();
    assert_eq!(child.elements.len(), 1);
    assert!(child.elements[0].name.eq_ignore_ascii_case("Rchosen"));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unresolved_chains_do_not_select_else_to_find_a_temperature() {
    let netlist = Netlist::parse(
        "No guessed else\n.if (1/(TEMP-27)>0)\n.param selected=1\n.elseif missing\n.param ambient=45\n.else\n.param ambient=65\n.endif\n.param ambient=85\n.options temp={ambient}\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.params.get("ambient"), Some(85.0));
    assert_eq!(netlist.params.get("selected"), Some(1.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn branch_effects_publish_only_after_the_condition_is_confirmed() {
    let netlist = Netlist::parse(
        "Branch effects\n.if (1/(TEMP-27)>0)\n.data chosen x\n{TEMP}\n.enddata\n.model rm R(R=1)\n.subckt chosen p\nR1 p 0 1k\n.ends\n.else\n.data wrong x\n{missing}\n.enddata\n.veriloga missing.va\n.model wrong R(R=2)\n.subckt wrong p\nR1 p 0 2k\n.ends\n.endif\n.temp 85\n.end\n",
    ).unwrap();
    assert_eq!(netlist.data_tables.len(), 1);
    assert_eq!(netlist.data_tables[0].name, "chosen");
    assert_eq!(netlist.data_tables[0].rows, vec![vec![85.0]]);
    assert_eq!(netlist.models.len(), 1);
    assert_eq!(netlist.subcircuits.len(), 1);
    assert!(netlist.veriloga_includes.is_empty());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lazy_temperature_selection_can_remove_an_initial_undefined_parameter_error() {
    let netlist = Netlist::parse(
        "Lazy condition\n.if if(TEMP==27,missing,1)\n.param selected=1\n.endif\n.temp 85\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.params.get("selected"), Some(1.0));
    assert!(Netlist::parse("No forward condition binding\n.if missing\n.param selected=1\n.endif\n.param missing=1\n.temp 85\n.end\n").is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_literal_hints_cannot_invalidate_the_default_temperature() {
    let netlist = Netlist::parse("Inactive hint\n.if 0\n.options temp=85\n.endif\n.if (1/(TEMP-85)<0)\n.param selected=1\n.endif\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, None);
    assert_eq!(netlist.params.get("selected"), Some(1.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn conditional_replay_keeps_draws_in_their_authored_phases() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Conditional samples\n.options seed=37\n.if (aunif(.5,.1)+1/(TEMP-27)>0)\n.param chosen={aunif(0,1)}\n.elseif aunif(0,1)\n.param wrong=1\n.else\n.param wrong=2\n.endif\n.param ambient={85+aunif(0,1)}\n.options temp={ambient}\n.param tail={aunif(0,1)}\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let _condition = eval_expression("aunif(.5,.1)", &expected).unwrap();
    assert_eq!(
        netlist.params.get("chosen"),
        Some(eval_expression("aunif(0,1)", &expected).unwrap())
    );
    assert_eq!(netlist.params.get("wrong"), None);
    let temperature = 85.0 + eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.options.temp, Some(temperature));
    assert_eq!(netlist.params.get("ambient"), Some(temperature));
    assert_eq!(
        netlist.params.get("tail"),
        Some(eval_expression("aunif(0,1)", &expected).unwrap())
    );
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn active_resource_limits_remain_terminal_during_recovery() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    for source in [
        "Later limit\n.if (1/(TEMP-27))\n.param selected=1\n.endif\n.data live x\n1\n2\n3\n.enddata\n.temp 85\n.end\n",
        "Recovered limit\n.if (1/(TEMP-27))\n.data live x\n1\n2\n3\n.enddata\n.endif\n.temp 85\n.end\n",
    ] {
        let mut options = NetlistParseOptions::default();
        options.resource_limits.max_analysis_points = 2;
        let error = Netlist::parse_with_options(source, options).unwrap_err();
        assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stable_errors_structural_errors_and_circular_selection_still_fail() {
    for source in [
        "Stable failure\n.if (1/(TEMP-TEMP))\n.param selected=1\n.else\n.param selected=2\n.endif\n.temp 85\n.end\n",
        "Missing endif\n.if (1/(TEMP-27))\n.param selected=1\n.temp 85\n.end\n",
        "Duplicate else\n.if (1/(TEMP-27))\n.else\n.else\n.endif\n.temp 85\n.end\n",
        "Misordered elseif\n.if (1/(TEMP-27))\n.else\n.elseif 1\n.endif\n.temp 85\n.end\n",
        "Empty condition\n.if\n.endif\n.temp 85\n.end\n",
        "Circular selection\n.if (1/(TEMP-27))\n.options temp=27\n.endif\n.end\n",
    ] {
        assert!(Netlist::parse(source).is_err(), "{source}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn selected_resistor_branch_controls_the_dc_solution() {
    let netlist = Netlist::parse("Conditional DC\nI1 0 out 1m\n.if (1/(TEMP-27)>0)\nR1 out 0 1k\n.else\nR1 out 0 2k\n.endif\n.temp 85\n.end\n").unwrap();
    let voltage = rspice_core::Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .try_voltage_named("out")
        .unwrap();
    // Allow the solver's default conductance regularization.
    assert!((voltage - 1.0).abs() < 1e-10, "{voltage}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_conditional_errors_keep_included_continuation_origins() {
    use rspice_core::netlist::{
        ParseError, ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-conditional-temperature/root.cir"
    } else {
        "/rspice-conditional-temperature/root.cir"
    });
    let child = root.with_file_name("conditional.inc");
    let source = "Conditional origin\n.include conditional.inc\n.temp 85\n.end\n";
    let include = "* continued decision\n.if (1/(TEMP-\n+ TEMP))\n.param selected=1\n.endif\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "conditional.inc".into(),
            target: child,
        }],
    )
    .unwrap();
    let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &root,
        bundle,
        Default::default(),
        &rspice_core::NoAbort,
    )
    .unwrap_err();
    assert!(
        matches!(
            error,
            ParseWithAbortError::Parse(ParseError::Syntax { line: 2, .. })
        ),
        "{error}"
    );
    assert!(error.to_string().contains("conditional.inc:2:"), "{error}");
    assert!(error.to_string().contains("Division by zero"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn conditional_functions_keep_the_definition_at_the_authored_card() {
    let netlist = Netlist::parse("Conditional function\n.func choose(x) {if(x==27,missing,1)}\n.if choose(TEMP)\n.param selected=1\n.endif\n.func choose(x) {0}\n.temp 85\n.end\n").unwrap();
    assert_eq!(netlist.params.get("selected"), Some(1.0));
}
