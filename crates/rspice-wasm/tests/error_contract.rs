use rspice_wasm::{
    run_ac_analysis, run_ac_analysis_detailed, summarize_netlist, summarize_netlist_detailed,
};

const INVALID_OUTPUT_DECK: &str = "typed browser error\n\
V1 1 0 1\n\
.PRINT OP V(MISSING_NODE) I(MISSING_DEVICE) {V(REPEATED)} {VM(REPEATED)}\n\
.OP\n\
.END\n";

#[test]
fn output_symbol_error_preserves_stable_kind_origin_and_exact_order() {
    let error = summarize_netlist_detailed(INVALID_OUTPUT_DECK)
        .expect_err("undefined output symbols must fail strict browser inspection");

    assert_eq!(error.kind, "undefined_output_symbols");
    assert_eq!(error.category, "output_symbol_validation");
    assert_eq!(error.primary_source, None);
    assert_eq!(error.primary_line, Some(3));
    assert_eq!(error.unresolved_output_symbols.len(), 4);

    let observed = error
        .unresolved_output_symbols
        .iter()
        .map(|item| {
            (
                item.directive.as_str(),
                item.line,
                item.operator.as_str(),
                item.symbol.as_str(),
                item.symbol_kind.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![
            ("print", 3, "I", "MISSING_DEVICE", "device"),
            ("print", 3, "V", "MISSING_NODE", "node"),
            ("print", 3, "V", "REPEATED", "node"),
            ("print", 3, "VM", "REPEATED", "node"),
        ]
    );
    assert!(error.message.contains("MISSING_DEVICE"));

    let serialized = serde_json::to_value(&error).expect("error contract serializes");
    assert!(serialized.get("unresolved_output_symbols").is_some());
    assert!(serialized.get("unresolved").is_none());
}

#[test]
fn fourier_directive_uses_the_core_four_tag() {
    let error =
        summarize_netlist_detailed("typed fourier error\nV1 1 0 1\n.FOUR 1k V(MISSING)\n.END\n")
            .expect_err("undefined FOUR output node must fail validation");

    assert_eq!(error.unresolved_output_symbols.len(), 1);
    assert_eq!(error.unresolved_output_symbols[0].directive, "four");
}

#[test]
fn legacy_string_error_api_retains_the_typed_errors_message() {
    let structured = summarize_netlist_detailed(INVALID_OUTPUT_DECK)
        .expect_err("detailed API must reject undefined output symbols");
    let legacy = summarize_netlist(INVALID_OUTPUT_DECK)
        .expect_err("legacy API must reject undefined output symbols");

    assert_eq!(legacy, structured.message);
}

#[test]
fn execution_and_argument_boundaries_publish_structured_categories() {
    let parse_error = run_ac_analysis_detailed(INVALID_OUTPUT_DECK, &[1_000.0])
        .expect_err("execution must validate before circuit construction");
    assert_eq!(parse_error.kind, "undefined_output_symbols");
    assert_eq!(parse_error.primary_line, Some(3));

    let argument_error = run_ac_analysis_detailed("valid\nV1 1 0 1\n.END\n", &[])
        .expect_err("empty frequency grids are invalid");
    assert_eq!(argument_error.kind, "invalid_argument");
    assert_eq!(argument_error.category, "input_validation");
    assert_eq!(
        run_ac_analysis("valid\nV1 1 0 1\n.END\n", &[])
            .expect_err("legacy API retains invalid-grid message"),
        argument_error.message
    );
}
