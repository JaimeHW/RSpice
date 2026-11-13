//! Failed provisional option fields must not hide later physical temperatures.
use rspice_core::Netlist;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn later_fields_on_the_same_option_card_select_temperature() {
    let netlist = Netlist::parse("Option fields\n.options reltol={1/(TEMP-27)} temp={ambient} tnom={nominal}\n.param ambient=85 nominal=35\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(35.0));
    assert_eq!(netlist.options.reltol, Some(1.0 / 58.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_failed_temperature_field_can_discover_its_later_nominal_dependency() {
    let netlist = Netlist::parse(
        "Nominal dependency\n.options temp={1/(TNOM-27)} tnom={nominal}\n.param nominal=55\n.end\n",
    )
    .unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
    assert_eq!(netlist.options.tnom, Some(55.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn numerical_validation_and_implicit_scalar_fields_reconcile_temperature() {
    let netlist = Netlist::parse("Validated fields\n.options reltol {TEMP-50}, itl1={TEMP-50}, temp {ambient}\n.param ambient=85\n.end\n").unwrap();
    assert_eq!(netlist.options.reltol, Some(35.0));
    assert_eq!(netlist.options.itl1, Some(35));
    assert_eq!(netlist.options.temp, Some(85.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_packaged_fields_keep_later_package_and_temperature_assignments() {
    let netlist = Netlist::parse("Packages\n.options TIMEINT RELTOL={1/(TEMP-27)} DEVICE TEMP={ambient} TNOM={nominal}\n.param ambient=85 nominal=35\n.end\n").unwrap();
    assert_eq!(netlist.options.timeint_reltol, Some(1.0 / 58.0));
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(35.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_option_errors_preserve_later_parent_temperature_bindings() {
    let netlist = Netlist::parse("Scoped option fields\n.subckt parent p\n.subckt child q\n.options reltol={1/(TEMP-27)} temp={ambient} tnom={nominal}\n.ends\n.param ambient=85 nominal=35\n.ends\n.end\n").unwrap();
    assert_eq!(netlist.options.reltol, Some(1.0 / 58.0));
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(35.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_time_vectors_reconcile_without_losing_later_fields() {
    for (package, field) in [("TIMEINT", "BREAKPOINTS"), ("OUTPUT", "OUTPUTTIMEPOINTS")] {
        let source = format!(
            "Vector field\n.options {package} {field}={{1/(TEMP-27)}},2 DEVICE TEMP={{ambient}}\n.param ambient=85\n.end\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let values = if package == "TIMEINT" {
            &netlist.options.timeint_breakpoints
        } else {
            &netlist.options.output_time_points
        };
        assert_eq!(values.as_slice(), [1.0 / 58.0, 2.0]);
        assert_eq!(netlist.options.temp, Some(85.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_output_intervals_keep_the_schedule_and_following_fields() {
    for (body, initial, time, interval) in [
        ("{1/(TEMP-27)} 10 2 20 4", 1.0 / 58.0, 10.0, 2.0),
        ("1 {TEMP-50} 2 40 4", 1.0, 35.0, 2.0),
        ("1 10 {1/(TEMP-27)} 20 4", 1.0, 10.0, 1.0 / 58.0),
    ] {
        let netlist = Netlist::parse(&format!("Output schedule\n.options OUTPUT INITIAL_INTERVAL={body} SNAPSHOTS=1 DEVICE TEMP={{ambient}}\n.param ambient=85\n.end\n")).unwrap();
        let schedule = netlist.options.output_interval_schedule.unwrap();
        assert_eq!(schedule.initial_interval, initial);
        assert_eq!(schedule.intervals.len(), 2);
        assert_eq!(schedule.intervals[0].time, time);
        assert_eq!(schedule.intervals[0].interval, interval);
        assert_eq!(netlist.options.output_snapshots, Some(true));
        assert_eq!(netlist.options.temp, Some(85.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_restart_tail_is_validated_on_a_fresh_temperature_pass() {
    let netlist = Netlist::parse("Restart schedule\n.options RESTART INITIAL_INTERVAL=1 {TEMP-50} {1/(TEMP-27)}\n.temp 85\n.end\n").unwrap();
    let restart = netlist.options.restart.unwrap();
    assert_eq!(restart.initial_interval, Some(1.0));
    assert_eq!(restart.intervals.len(), 1);
    assert_eq!(restart.intervals[0].time, 35.0);
    assert_eq!(restart.intervals[0].interval, 1.0 / 58.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn invalid_fields_cannot_be_erased_by_later_assignments() {
    for fields in [
        "reltol={1/(TEMP-TEMP)} reltol=1 temp=85",
        "reltol={TEMP-100} reltol=1 temp=85",
        "reltol= temp=85",
        "OUTPUT INITIAL_INTERVAL=1 10 0 DEVICE TEMP=85",
    ] {
        assert!(
            Netlist::parse(&format!("Invalid fields\n.options {fields}\n.end\n")).is_err(),
            "{fields}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn option_overlay_keeps_its_materialized_context_and_atomic_error_contract() {
    use rspice_core::netlist::{
        ParamContext, SimulationOptions, simulation_options_with_overrides,
    };
    let current = SimulationOptions {
        abstol: Some(1e-12),
        ..Default::default()
    };
    let result = simulation_options_with_overrides(
        "abstol=1e-9 reltol={1/(TEMP-27)} temp=85",
        &ParamContext::new(),
        &current,
        100,
        &rspice_core::NoAbort,
    );
    assert!(result.is_err());
    assert_eq!(current.abstol, Some(1e-12));
    assert_eq!(current.temp, None);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_option_replay_keeps_eager_statistical_order() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Option draws\n.options seed=37 reltol={aunif(1,.1)+1/(TEMP-27)} temp={85+aunif(0,1)}\n.param next={aunif(0,1)}\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let factor = eval_expression("aunif(1,.1)", &expected).unwrap();
    let temperature = 85.0 + eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.options.temp, Some(temperature));
    assert_eq!(
        netlist.options.reltol,
        Some(factor + 1.0 / (temperature - 27.0))
    );
    assert_eq!(
        netlist.params.get("next"),
        Some(eval_expression("aunif(0,1)", &expected).unwrap())
    );
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scalar_temperature_directives_keep_precedence_after_option_errors() {
    for body in [
        ".options reltol={1/(TEMP-27)} temp=27\n.temp 85",
        ".temp 85\n.options reltol={1/(TEMP-27)} temp=27",
        ".options reltol={1/(TEMP-27)} temp={ambient}\n.temp 35\n.temp 55 85\n.param ambient=85",
    ] {
        let netlist = Netlist::parse(&format!("Directive precedence\n{body}\n.end\n")).unwrap();
        assert_eq!(netlist.options.temp, Some(85.0));
        assert_eq!(netlist.options.reltol, Some(1.0 / 58.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_temperature_hints_do_not_invalidate_active_options() {
    let netlist = Netlist::parse("Inactive option\n.options reltol={1/(85-TEMP)}\n.if 0\n.options temp=85 reltol={missing}\n.endif\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, None);
    assert_eq!(netlist.options.reltol, Some(1.0 / 58.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_later_option_resource_error_remains_terminal() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    let mut options = NetlistParseOptions::default();
    options.resource_limits.max_analysis_points = 2;
    let error = Netlist::parse_with_options("Option resources\n.options reltol={1/(TEMP-27)} OUTPUT OUTPUTTIMEPOINTS=0,1,2 DEVICE TEMP=85\n.end\n", options).unwrap_err();
    assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_option_errors_keep_the_included_card_origin() {
    use rspice_core::netlist::{
        ParseError, ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-provisional-options/root.cir"
    } else {
        "/rspice-provisional-options/root.cir"
    });
    let child = root.with_file_name("options.inc");
    let source = "Option origin\n.include options.inc\n.temp 85\n.end\n";
    let include = "* options\n.options reltol={TEMP-100}\n+ reltol=1\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "options.inc".into(),
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
            &error,
            ParseWithAbortError::Parse(ParseError::Syntax { line: 2, .. })
        ),
        "{error}"
    );
    assert!(error.to_string().contains("options.inc:2:"), "{error}");
}
