//! Eager declarations must be validated at the selected physical temperature.
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn eager_parameters_reconcile_before_binding_devices() {
    for expression in ["{58/(TEMP-27)}", "58 / (TEMP-27)"] {
        let source = format!(
            "Eager parameter\n.param amplitude={expression} copy=amplitude ambient=85\n.options temp={{ambient}}\nV1 out 0 {{amplitude}}\nR1 out 0 1k\n.end\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.params.get("amplitude"), Some(1.0));
        assert_eq!(netlist.params.get("copy"), Some(1.0));
        assert_eq!(netlist.options.temp, Some(85.0));
        let voltage = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((voltage - 1.0).abs() < 1e-12, "{voltage}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn eager_parameters_respect_scalar_and_swept_temperature_precedence() {
    for cards in [
        ".options temp=27\n.temp 85",
        ".temp 85\n.options temp=27",
        ".temp {ambient}\n.param ambient={base}\n.param base=85",
        ".temp 35\n.temp 55 85\n.options temp={ambient}\n.param ambient=85",
    ] {
        let source =
            format!("Temperature precedence\n.param scale={{1/(TEMP-27)}}\n{cards}\n.end\n");
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.params.get("scale"), Some(1.0 / 58.0));
        assert_eq!(netlist.options.temp, Some(85.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn invalid_eager_declarations_survive_redefinition_and_later_errors() {
    for later in [
        ".param scale=1\n.temp 85",
        ".param scale=1\n.temp {missing}\n.temp 85",
        ".if {scale}\n.temp 85\n.endif",
    ] {
        let source =
            format!("Invalid declaration\n.param scale={{1/(TEMP-TEMP)}}\n{later}\n.end\n");
        let error = Netlist::parse(&source).unwrap_err();
        assert!(error.to_string().contains("Division by zero"), "{error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_parameter_completion_can_discover_a_later_temperature() {
    let netlist = Netlist::parse("Forward completion\n.param scale={later/(TEMP-27)}\n.param later=58\n.temp {ambient}\n.param ambient={base}\n.param base=85\n.end\n").unwrap();
    assert_eq!(netlist.params.get("scale"), Some(1.0));
    assert_eq!(netlist.options.temp, Some(85.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nominal_temperature_propagates_before_eager_values_are_published() {
    let netlist = Netlist::parse("Nominal temperature\n.param scale={1/(TNOM-27)} observed={TEMP}\n.param nominal=55\n.options tnom={nominal} temp={TNOM+30}\n.end\n").unwrap();
    assert_eq!(netlist.params.get("scale"), Some(1.0 / 28.0));
    assert_eq!(netlist.params.get("observed"), Some(85.0));
    assert_eq!(netlist.options.tnom, Some(55.0));
    assert_eq!(netlist.options.temp, Some(85.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_hints_cannot_invalidate_eager_parameters() {
    let netlist = Netlist::parse(
        "Inactive hint\n.param scale={1/(TEMP-85)}\n.if 0\n.options temp=85\n.endif\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.params.get("scale"), Some(-1.0 / 58.0));
    assert_eq!(netlist.options.temp, None);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn successful_replay_keeps_eager_statistical_draw_order() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Eager statistics\n.options seed=37\n.param scale={aunif(1,.1)+1/(TEMP-27)} ambient={85+aunif(0,1)}\n.options temp={ambient}\n.param tail={aunif(0,1)}\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let amplitude = eval_expression("aunif(1,.1)", &expected).unwrap();
    let ambient = 85.0 + eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.options.temp, Some(ambient));
    assert_eq!(
        netlist.params.get("scale"),
        Some(amplitude + 1.0 / (ambient - 27.0))
    );
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
fn invalid_final_values_are_not_hidden_by_overwriting_the_binding() {
    let error =
        Netlist::parse("Invalid final value\n.param scale={1/(TEMP-85)} scale=1\n.temp 85\n.end\n")
            .unwrap_err();
    assert!(error.to_string().contains("Division by zero"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_source_completion_reconciles_temperature() {
    let netlist = Netlist::parse("Source completion\nV1 out 0 {amplitude/(TEMP-27)}\nR1 out 0 1k\n.param amplitude=58\n.temp {ambient}\n.param ambient=85\n.end\n").unwrap();
    let voltage = Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .try_voltage_named("out")
        .unwrap();
    assert!((voltage - 1.0).abs() < 1e-12, "{voltage}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn parameter_error_keeps_the_included_declaration_origin() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-provisional-parameter/root.cir"
    } else {
        "/rspice-provisional-parameter/root.cir"
    });
    let child = root.with_file_name("parameters.inc");
    let source = "Included parameter\n.include parameters.inc\n.temp 85\n.end\n";
    let include = "* declaration\n.param scale={1/(TEMP-85)}\n+ scale=1\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "parameters.inc".into(),
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
    assert!(error.to_string().contains("parameters.inc:2:"), "{error}");
    assert!(error.to_string().contains("Division by zero"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn resource_errors_are_not_replaced_by_retained_parameter_errors() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    let mut options = NetlistParseOptions::default();
    options.resource_limits.max_analysis_points = 2;
    let error = Netlist::parse_with_options(
        "Resource priority\n.param scale={1/(TEMP-27)}\n.data samples x\n1\n2\n3\n.enddata\n.temp 85\n.end\n",
        options,
    )
    .unwrap_err();
    assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn evaluated_function_domain_errors_reconcile_without_rebinding_functions() {
    let netlist = Netlist::parse("Function domain\n.func factor(x) {mod(sqrt(x-50),2)}\n.param scale={factor(TEMP)}\n.func factor(x) {x+1000}\n.temp 85\n.end\n").unwrap();
    assert_eq!(netlist.params.get("scale"), Some(35.0_f64.sqrt() % 2.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_model_completion_can_discover_temperature() {
    let netlist = Netlist::parse("Model completion\n.param at={TEMP}\n.model rm R (R={base/(at-27)})\n.param base=58\n.temp {ambient}\n.param ambient=85\n.end\n").unwrap();
    assert!(
        netlist.models[0]
            .params
            .iter()
            .any(|(name, value)| name == "R" && *value == 1.0)
    );
}
