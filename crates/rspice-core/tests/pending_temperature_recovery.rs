//! Failed deferred temperature groups discover later independent assignments.
use rspice_core::Netlist;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn pending_temperature_discovers_later_nominal_in_the_same_group() {
    let netlist = Netlist::parse(
        "Pending option completion\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=1 nominal=55\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
    assert_eq!(netlist.options.tnom, Some(55.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_closed_scope_can_reach_a_later_root_temperature_group() {
    let netlist = Netlist::parse(
        "Later root group\n.subckt child p\n.options temp={scale/(TNOM-27)}\n.param scale=1\n.ends\n.options tnom={nominal}\n.param nominal=55\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
    assert_eq!(netlist.options.tnom, Some(55.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_nominal_discovers_later_temperature_and_reconciles_eager_values() {
    let netlist = Netlist::parse(
        "Reverse dependency\n.param observed={TNOM}\n.options tnom={scale/(TEMP-27)} temp={ambient}\n.param scale=1 ambient=85\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(1.0 / 58.0));
    assert_eq!(netlist.params.get("observed"), Some(1.0 / 58.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn numeric_temperature_validation_recovers_in_both_expression_dialects() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::netlist::NetlistParseOptions;
    for expression_dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let netlist = Netlist::parse_with_options(
            "Numeric validation\n.options temp={scale*(TNOM-50)-270} tnom={nominal}\n.param scale=1 nominal=55\n.end\n",
            NetlistParseOptions { expression_dialect, ..Default::default() },
        ).unwrap();
        assert_eq!(netlist.options.temp, Some(-265.0));
        assert_eq!(netlist.options.tnom, Some(55.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn later_unfinished_ancestor_defers_the_whole_failed_group() {
    let netlist = Netlist::parse(
        "Incomplete owner\n.subckt parent p\n.subckt child q\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=1\n.ends\n.param nominal={base}\n.param base=55\n.ends\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
    assert_eq!(netlist.options.tnom, Some(55.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn candidate_precedence_uses_authored_order_not_scope_completion_order() {
    for (prefix, suffix, nominal) in [
        (
            "",
            ".options tnom={root_nominal}\n.param root_nominal=65\n",
            65.0,
        ),
        (
            ".options tnom={root_nominal}\n",
            ".param root_nominal=65\n",
            55.0,
        ),
    ] {
        let netlist = Netlist::parse(&format!(
            "Candidate precedence\n{prefix}.subckt child p\n.options temp={{scale/(TNOM-27)}} tnom={{nominal}}\n.param scale=1 nominal=55\n.ends\n{suffix}.end\n",
        )).unwrap();
        assert_eq!(netlist.options.temp, Some(1.0 / (nominal - 27.0)));
        assert_eq!(netlist.options.tnom, Some(nominal));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn later_root_and_sibling_groups_are_still_discovered_after_failed_delayed_groups() {
    for selector in [
        ".options tnom={nominal}\n",
        ".subckt sibling p\n.options tnom={nominal}\n.ends\n",
    ] {
        let netlist = Netlist::parse(&format!(
            "Several delayed groups\n.subckt child p\n.options temp={{scale/(TNOM-27)}}\n.ends\n{selector}.param scale=1 nominal=55\n.end\n",
        )).unwrap();
        assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
        assert_eq!(netlist.options.tnom, Some(55.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_groups_keep_sampled_operands_and_dependencies_at_their_original_phase() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse(
        "Failed group samples\n.options seed=37\n.options temp={scale+aunif(0,1)+1/(TNOM-27)} tnom={nominal}\n.param scale={base+aunif(0,1)} nominal={later+aunif(0,1)}\n.param base=1 later=55 eager={aunif(0,1)}\n.end\n",
    ).unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let draw = || eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.params.get("eager"), Some(draw()));
    let scale = 1.0 + draw();
    let operand = draw();
    let nominal = 55.0 + draw();
    assert_eq!(
        netlist.options.temp,
        Some(scale + operand + 1.0 / (nominal - 27.0))
    );
    assert_eq!(netlist.options.tnom, Some(nominal));
    assert_eq!(netlist.params.get("scale"), Some(scale));
    assert_eq!(netlist.params.get("nominal"), Some(nominal));
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        draw()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stable_invalid_and_superseded_assignments_never_become_successful() {
    for (value, expected) in [
        ("scale/(TNOM-TNOM)", "Division by zero"),
        ("scale*(TNOM-100)-300", "absolute zero"),
        ("missing", "Undefined parameter: MISSING"),
    ] {
        let source = format!(
            "Stable failure\n.options temp={{{value}}} temp=85 tnom={{nominal}}\n.param scale=1 nominal=55\n.end\n",
        );
        let error = Netlist::parse(&source).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        assert!(
            matches!(
                error,
                rspice_core::netlist::ParseError::Syntax { line: 2, .. }
            ),
            "{error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scalar_temp_and_sweeps_keep_their_precedence_over_recovered_options() {
    for (card, expected) in [(".temp 85", 85.0), (".temp 85 95", 1.0 / 28.0)] {
        for (prefix, suffix) in [
            (format!("{card}\n"), String::new()),
            (String::new(), format!("{card}\n")),
        ] {
            let netlist = Netlist::parse(&format!(
                "Directive precedence\n{prefix}.options temp={{scale/(TNOM-27)}} tnom={{nominal}}\n.param scale=1 nominal=55\n{suffix}.end\n",
            )).unwrap();
            assert_eq!(netlist.options.temp, Some(expected));
            assert_eq!(netlist.options.tnom, Some(55.0));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_group_does_not_hide_a_later_resource_limit() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    let mut options = NetlistParseOptions::default();
    options.resource_limits.max_analysis_points = 2;
    let error = Netlist::parse_with_options(
        "Resource limit\n.subckt child p\n.options temp={scale/(TNOM-27)}\n.param scale=1\n.ends\n.options OUTPUT OUTPUTTIMEPOINTS=1,2,3 DEVICE TNOM=55\n.end\n",
        options,
    ).unwrap_err();
    assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_group_errors_keep_the_physical_included_card_origin() {
    use rspice_core::netlist::{
        ParseError, ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-pending-temperature/root.cir"
    } else {
        "/rspice-pending-temperature/root.cir"
    });
    let child = root.with_file_name("child.inc");
    let source = "Retained group origin\n.include child.inc\n.options temp=85 tnom=65\n.end\n";
    let include = ".subckt child p\n.options temp={scale/(TNOM-TNOM)}\n+ tnom={nominal}\n.param scale=1 nominal=55\n.ends\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "child.inc".into(),
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
    assert!(error.to_string().contains("child.inc:2:"), "{error}");
    assert!(error.to_string().contains("Division by zero"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn recovered_options_drive_the_returned_circuit_and_dc_solution() {
    let netlist = Netlist::parse(
        "Physical circuit\nI1 0 out 1m\nR1 out 0 {TEMP}\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=2380 nominal=55\n.end\n",
    ).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.params.get("TEMP"), Some(85.0));
    let voltage = rspice_core::Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .try_voltage_named("out")
        .unwrap();
    assert!((voltage - 0.085).abs() < 1e-12, "{voltage}");
}
