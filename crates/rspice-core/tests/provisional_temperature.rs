//! Invalid provisional temperatures must not prevent physical option selection.
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn analysis_values_bind_after_parameter_selected_temperature() {
    let netlist = Netlist::parse("Provisional analysis\n.ac lin 1 {1/(TEMP-27)} {1/(TEMP-27)}\n.param ambient=85\n.options temp={ambient}\n.end\n").unwrap();
    let AnalysisCommand::Ac {
        start_freq,
        stop_freq,
        ..
    } = netlist.analyses[0]
    else {
        panic!("AC")
    };
    assert_eq!((start_freq, stop_freq), (1.0 / 58.0, 1.0 / 58.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_source_values_bind_after_temperature_selection() {
    let netlist = Netlist::parse("Provisional source\nV1 out 0 {amplitude/(TEMP-27)}\nR1 out 0 1k\n.param amplitude=58 ambient=85\n.options temp={ambient}\n.end\n").unwrap();
    let voltage = Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .try_voltage_named("out")
        .unwrap();
    assert!((voltage - 1.0).abs() < 1e-12, "{voltage}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scalar_temperature_directives_keep_precedence_through_failed_completion() {
    for cards in [
        ".options temp=27\n.temp 85\n",
        ".temp 85\n.options temp=27\n",
        ".temp 35\n.temp {ambient}\n.param ambient=85\n",
        ".temp 35\n.temp 55 85\n.param ambient=85\n.options temp={ambient}\n",
    ] {
        let netlist = Netlist::parse(&format!(
            "Temperature directive\n.ac lin 1 {{1/(TEMP-27)}} {{1/(TEMP-27)}}\n{cards}.end\n"
        ))
        .unwrap();
        assert_eq!(netlist.options.temp, Some(85.0));
        let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
            panic!("AC")
        };
        assert_eq!(start_freq, 1.0 / 58.0);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_temperature_directives_complete_after_an_invalid_provisional_operand() {
    let netlist = Netlist::parse("Scoped temperature\n.subckt parent p\n.subckt child q\n.ac lin 1 {1/(TEMP-27)} {1/(TEMP-27)}\n.temp {ambient}\n.ends\n.param ambient=85\n.ends\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
        panic!("AC")
    };
    assert_eq!(start_freq, 1.0 / 58.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn completion_replay_reconciles_sampled_analysis_and_temperature_operands() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Sampled temperature\n.options seed=37\n.ac lin 1 {aunif(1,.1)+1/(TEMP-27)} 100\n.temp {85+aunif(0,1)+later}\n.dc V1 {aunif(0,1)+later} 100 1\n.param later=0\n.end\n").unwrap();
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let amplitude = eval_expression("aunif(1,.1)", &reference).unwrap();
    let temperature = 85.0 + eval_expression("aunif(0,1)", &reference).unwrap();
    assert_eq!(netlist.options.temp, Some(temperature));
    let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
        panic!("AC")
    };
    assert_eq!(start_freq, amplitude + 1.0 / (temperature - 27.0));
    let AnalysisCommand::Dc { start, .. } = netlist.analyses[2] else {
        panic!("DC")
    };
    assert_eq!(start, eval_expression("aunif(0,1)", &reference).unwrap());
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &reference).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn an_inactive_literal_hint_cannot_make_a_valid_analysis_fail() {
    let netlist = Netlist::parse(
        "Inactive hint\n.ac lin 1 {50-TEMP} 100\n.if 0\n.options temp=85\n.endif\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.options.temp, None);
    let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
        panic!("AC")
    };
    assert_eq!(start_freq, 23.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn numerical_validation_uses_the_reconciled_temperature_and_captured_function() {
    let netlist = Netlist::parse("Temperature validation\n.func frequency(x) {x-50}\n.ac lin 1 {frequency(TEMP)} 100\n.func frequency(x) {x+1000}\n.param ambient=85\n.options temp={ambient}\n.end\n").unwrap();
    let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
        panic!("AC")
    };
    assert_eq!(start_freq, 35.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stable_invalid_expressions_and_changing_temperature_selections_are_rejected() {
    for (cards, message) in [
        (
            ".param ambient=27\n.options temp={ambient}",
            "Division by zero",
        ),
        (".temp {missing}\n.temp 85", "MISSING"),
        (".options temp={TEMP+1}", "TEMP/TNOM selection changes"),
    ] {
        let source =
            format!("Invalid final temperature\n.ac lin 1 {{1/(TEMP-27)}} 100\n{cards}\n.end\n");
        let error = Netlist::parse(&source).unwrap_err();
        assert!(
            error
                .to_string()
                .to_ascii_uppercase()
                .contains(&message.to_ascii_uppercase()),
            "{error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn typed_errors_keep_their_physical_origin_after_temperature_replay() {
    use rspice_core::netlist::{
        AnalysisCard, ParseError, ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-provisional-temperature/root.cir"
    } else {
        "/rspice-provisional-temperature/root.cir"
    });
    let child = root.with_file_name("child.inc");
    let source = "Included failure\n.include child.inc\n.temp 85\n.end\n";
    let include = ".pss FUND=1k HARMS={1/(TEMP-27)}\n";
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
    let ParseWithAbortError::Parse(ParseError::AnalysisCard(error)) = error else {
        panic!("{error}")
    };
    assert_eq!(error.card, AnalysisCard::Pss);
    assert_eq!(error.source_location().line, 1);
    assert!(error.to_string().contains("child.inc:1:"), "{error}");
}
