//! Conditional DATA blocks must follow the same selection as ordinary cards.
use rspice_core::netlist::{Netlist, NetlistParseOptions};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn only_the_selected_branch_publishes_data_tables() {
    let netlist = Netlist::parse(
        "Conditional tables\n.if 0\n.data skipped x\n1\n.enddata\n.else\n.data selected x\n2\n.enddata\n.endif\n.end\n",
    ).unwrap();
    assert_eq!(netlist.data_tables.len(), 1);
    assert_eq!(netlist.data_tables[0].name, "selected");
    assert_eq!(netlist.data_tables[0].rows, vec![vec![2.0]]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_table_headers_rows_and_limits_are_not_evaluated() {
    for block in [
        ".data\n.enddata\n",
        ".data unused x\n{missing}\n.enddata\n",
        ".data unused x\n1\n2\n3\n.enddata\n",
    ] {
        let mut options = NetlistParseOptions::default();
        options.resource_limits.max_analysis_points = 2;
        let netlist = Netlist::parse_with_options(
            &format!("Inactive data\n.if 0\n{block}.endif\n.end\n"),
            options,
        )
        .unwrap();
        assert!(netlist.data_tables.is_empty());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_rows_consume_no_random_draws_across_nested_branches() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse(
        "Inactive samples\n.options seed=37\n.if 0\n.if missing\n.data unused x\n{aunif(0,1)}\n.enddata\n.endif\n.elseif 0\n.data unused x\n{aunif(0,1)}\n.enddata\n.else\n.data live x\n{aunif(0,1)}\n.enddata\n.endif\n.param after={aunif(0,1)}\n.end\n",
    ).unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    assert_eq!(netlist.data_tables.len(), 1);
    assert_eq!(
        netlist.data_tables[0].rows,
        vec![vec![eval_expression("aunif(0,1)", &expected).unwrap()]]
    );
    assert_eq!(
        netlist.params.get("after"),
        Some(eval_expression("aunif(0,1)", &expected).unwrap())
    );
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn active_tables_keep_syntax_numeric_and_resource_validation() {
    use rspice_core::netlist::ParseError;
    for block in [
        ".data\n.enddata\n",
        ".data active x\n{missing}\n.enddata\n",
        ".enddata\n",
    ] {
        assert!(
            Netlist::parse(&format!(
                "Active invalid table\n.if 1\n{block}.endif\n.end\n"
            ))
            .is_err()
        );
    }
    let mut options = NetlistParseOptions::default();
    options.resource_limits.max_analysis_points = 2;
    let error = Netlist::parse_with_options(
        "Active table limit\n.if 1\n.data active x\n1\n2\n3\n.enddata\n.endif\n.end\n",
        options,
    )
    .unwrap_err();
    assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
}
