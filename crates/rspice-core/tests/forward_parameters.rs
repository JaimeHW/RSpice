//! Public parser regressions for deferred ordinary parameter declarations.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{
    NetlistParseOptions, ParameterRedefinitionDiagnosticPolicy, ParameterRedefinitionPolicy,
};
use rspice_core::{Engine, Netlist};

const DIALECTS: [ExpressionDialect; 2] = [ExpressionDialect::Ngspice, ExpressionDialect::Xyce];

fn options(expression_dialect: ExpressionDialect) -> NetlistParseOptions {
    NetlistParseOptions {
        expression_dialect,
        ..Default::default()
    }
}

fn next_draw(params: &ParamContext) -> f64 {
    eval_expression("aunif(0,1)", params).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_root_aliases_preserve_complex_values_and_drive_the_circuit() {
    for dialect in DIALECTS {
        for body in [
            ".param ambient={later+img(z)}\n.param later=85 z={sqrt(-1)}\n",
            ".param ambient=later\n.param later=84\n",
            ".param ambient=-later\n.param later=-84\n",
            ".param ambient=+later\n.param later=84\n",
        ] {
            let source = format!(
                "Forward root\n.options temp={{ambient}}\n{body}I1 0 out 1m\nR1 out 0 {{ambient}}\n.end\n"
            );
            let netlist = Netlist::parse_with_options(&source, options(dialect)).unwrap();
            assert_eq!(
                netlist.params.get("ambient"),
                Some(84.0),
                "{dialect:?}: {body}"
            );
            assert_eq!(netlist.options.temp, Some(84.0));
            let voltage = Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .try_voltage_named("out")
                .unwrap();
            assert!((voltage - 0.084).abs() < 1e-10, "{voltage}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_static_root_parameters_finalize_without_temperature_options() {
    for dialect in DIALECTS {
        let netlist = Netlist::parse_with_options(
            "Forward static\n.param first={second+img(z)}\n.param second=85 z={sqrt(-1)}\n.end\n",
            options(dialect),
        )
        .unwrap();
        assert_eq!(netlist.params.get("first"), Some(84.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn model_and_parameter_share_one_forward_statistical_sample() {
    for dialect in DIALECTS {
        let source = "Forward model sample\n.options seed=37\n.param value={aunif(0,1)+base}\n.model rm R (R={value})\n.param base=85\n.end\n";
        let netlist = Netlist::parse_with_options(source, options(dialect)).unwrap();
        let mut reference = ParamContext::new();
        reference.set_random_seed(37);
        let expected = 85.0 + next_draw(&reference);
        assert_eq!(netlist.params.get("value"), Some(expected));
        assert!(
            netlist.models[0]
                .params
                .iter()
                .any(|(name, value)| name == "R" && *value == expected)
        );
        assert_eq!(next_draw(&netlist.params), next_draw(&reference));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_subcircuit_parameters_resolve_in_the_local_scope() {
    for dialect in DIALECTS {
        let source = "Forward local\n.subckt cell p n\n.options temp={ambient}\n.param ambient={later+img(z)}\n.param later=85 z={sqrt(-1)}\nR1 p n {ambient}\n.ends\nI1 0 out 1m\nX1 out 0 cell\n.end\n";
        let netlist = Netlist::parse_with_options(source, options(dialect)).unwrap();
        assert_eq!(netlist.options.temp, Some(84.0));
        let voltage = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((voltage - 0.084).abs() < 1e-10, "{voltage}");
    }
}

/// ngspice-46 selects `nch.1` for `L=0.1u` and solves the same drain current
/// whether `.param LL=... WW=...` comes before the transistor or after it.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_referenced_geometry_selects_its_bin_and_sizes_the_device() {
    let cards = ".model nch.1 nmos level=1 vto=0.5 kp=100u lmin=0.05u lmax=0.2u wmin=1u wmax=100u\n\
                 .model nch.2 nmos level=1 vto=0.5 kp=200u lmin=0.2u lmax=1u wmin=1u wmax=100u\n";
    // Saturation at VGS=VDS=1: Id = KP/2 * W/L * (VGS-VTO)^2.
    for (length, bin, kp) in [(0.1e-6, "nch.1", 100e-6), (0.5e-6, "nch.2", 200e-6)] {
        let geometry = format!(".param LL={length} WW=10u\n");
        let expected = kp / 2.0 * (10e-6 / length) * 0.25;
        for (before, after) in [("", geometry.as_str()), (geometry.as_str(), "")] {
            let source = format!(
                "Forward geometry\n{before}VD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L={{LL}} W={{WW}}\n{cards}{after}.end\n"
            );
            let netlist = Netlist::parse(&source).unwrap();
            let inspection = Engine::default().inspect_model_bins(&netlist).unwrap();
            let instance = &inspection.instances[0];
            assert!(instance.selected_model.eq_ignore_ascii_case(bin), "{source}");
            let close = |value: Option<f64>, expected: f64| {
                value.is_some_and(|value| (value - expected).abs() <= 1e-12 * expected)
            };
            assert!(close(instance.length, length), "{source}");
            assert!(close(instance.width, 10e-6), "{source}");
            let drain = -Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .branch_current_named("VD")
                .unwrap();
            assert!(
                // The drain-bulk junction leaks ~1 pA beside the channel.
                (drain - expected).abs() <= 1e-6 * expected,
                "{source}: {drain} vs {expected}"
            );
        }
    }
}

/// Every top-level device family that defers an instance assignment binds the
/// forward parameter instead of falling back to its default area or
/// multiplier: the deck solves exactly as its literal twin does.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_referenced_instance_parameters_match_their_literal_twin() {
    let devices = "V1 a 0 1\nD1 a 0 dmod area={AREA}\nR1 a 0 1k m={MULT}\n\
                   Q1 a a 0 qmod area={AREA}\nM1 a a 0 0 nch L=1u W=10u m={MULT}\n\
                   .model dmod d is=1e-14\n.model qmod npn is=1e-16 bf=100\n\
                   .model nch nmos level=1 vto=0.5 kp=100u\n";
    let supply = |source: &str| {
        -Engine::default()
            .run_dc_op(&Netlist::parse(source).unwrap())
            .unwrap()
            .branch_current_named("V1")
            .unwrap()
    };
    let forward = supply(&format!(
        "Forward instance parameters\n{devices}.param AREA=2 MULT=3\n.end\n"
    ));
    let literal = supply(&format!(
        "Literal instance parameters\n{}.end\n",
        devices.replace("{AREA}", "2").replace("{MULT}", "3")
    ));
    let defaults = supply(&format!(
        "Default instance parameters\n{}.end\n",
        devices.replace("{AREA}", "1").replace("{MULT}", "1")
    ));
    assert!(
        (forward - literal).abs() <= 1e-12 * literal,
        "{forward} vs {literal}"
    );
    assert!((forward - defaults).abs() > 0.4 * defaults, "{forward}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_forward_probes_preserve_the_seeded_random_stream() {
    for dialect in DIALECTS {
        for expression in ["aunif(0,1)+base", "sample(base)", "hidden(0)"] {
            let source = format!(
                "Forward draws\n.options seed=37 temp={{ambient}} tnom={{ambient}}\n.func sample(x) {{aunif(0,1)+x}}\n.func hidden(x) {{aunif(0,1)+base+x}}\n.param ambient={{{expression}}}\n.param base=85 eager={{aunif(0,1)}}\n.end\n"
            );
            let netlist = Netlist::parse_with_options(&source, options(dialect)).unwrap();
            let mut reference = ParamContext::new();
            reference.set_random_seed(37);
            assert_eq!(netlist.params.get("eager"), Some(next_draw(&reference)));
            let ambient = 85.0 + next_draw(&reference);
            assert_eq!(netlist.params.get("ambient"), Some(ambient));
            assert_eq!(netlist.options.temp, Some(ambient));
            assert_eq!(netlist.options.tnom, Some(ambient));
            assert_eq!(next_draw(&netlist.params), next_draw(&reference));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lazy_missing_branches_and_known_functions_draw_once() {
    for dialect in DIALECTS {
        let source = "Lazy draws\n.options seed=37\n.func sample(x) {aunif(0,1)+x}\n.param lazy={1 ? aunif(0,1) : missing}\n.param known={sample(2)}\n.end\n";
        let netlist = Netlist::parse_with_options(source, options(dialect)).unwrap();
        let mut reference = ParamContext::new();
        reference.set_random_seed(37);
        assert_eq!(netlist.params.get("lazy"), Some(next_draw(&reference)));
        assert_eq!(
            netlist.params.get("known"),
            Some(2.0 + next_draw(&reference))
        );
        assert_eq!(next_draw(&netlist.params), next_draw(&reference));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_duplicates_obey_selection_and_diagnostic_policies() {
    use rspice_core::netlist::{NetlistSourceLocation, ParseError};
    let source = "Forward duplicates\n.options seed=37\n.param value={aunif(0,1)+first}\n.param value={aunif(0,1)+last}\n.param first=10 last=20\n.end\n";
    for dialect in DIALECTS {
        for (selection, base) in [
            (ParameterRedefinitionPolicy::UseFirst, 10.0),
            (ParameterRedefinitionPolicy::UseLast, 20.0),
        ] {
            let mut opts = options(dialect);
            opts.parameter_redefinition_policy = selection;
            opts.parameter_redefinition_diagnostic_policy =
                ParameterRedefinitionDiagnosticPolicy::Warning;
            let netlist = Netlist::parse_with_options(source, opts).unwrap();
            let mut reference = ParamContext::new();
            reference.set_random_seed(37);
            assert_eq!(
                netlist.params.get("value"),
                Some(base + next_draw(&reference)),
                "{dialect:?} {selection:?}: {:?}",
                netlist.params.random()
            );
            assert_eq!(next_draw(&netlist.params), next_draw(&reference));
            assert_eq!(netlist.diagnostics.len(), 1);
            assert_eq!(
                netlist.diagnostics[0].origin,
                Some(NetlistSourceLocation::in_memory(4))
            );
        }
        let mut opts = options(dialect);
        opts.parameter_redefinition_diagnostic_policy =
            ParameterRedefinitionDiagnosticPolicy::Error;
        let error = Netlist::parse_with_options(source, opts).unwrap_err();
        assert!(
            matches!(error, ParseError::ParameterRedefinition(_)),
            "{error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn selected_definition_owns_deferred_errors_and_numeric_overrides_clear_them() {
    for dialect in DIALECTS {
        let source =
            "Missing duplicate\n.param value={missing_first}\n.param value={missing_last}\n.end\n";
        for (selection, line, missing) in [
            (ParameterRedefinitionPolicy::UseFirst, 2, "MISSING_FIRST"),
            (ParameterRedefinitionPolicy::UseLast, 3, "MISSING_LAST"),
        ] {
            let mut opts = options(dialect);
            opts.parameter_redefinition_policy = selection;
            let error = Netlist::parse_with_options(source, opts)
                .unwrap_err()
                .to_string();
            assert!(
                error.contains(&format!("line {line}:")) && error.contains(missing),
                "{error}"
            );
        }
        let netlist = Netlist::parse_with_options(
            "Retired error\n.param value={missing}\n.param value=85\n.end\n",
            options(dialect),
        )
        .unwrap();
        assert_eq!(netlist.params.get("value"), Some(85.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unresolved_cycles_and_invalid_expressions_are_rejected() {
    for dialect in DIALECTS {
        for (body, message) in [
            (".param a={b}\n.param b={a}\n", "cyclic"),
            (".param a={missing}\n", "MISSING"),
            (".param a={b+}\n.param b=85\n", "line 2"),
        ] {
            let error = Netlist::parse_with_options(
                &format!("Invalid forward\n{body}.end\n"),
                options(dialect),
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains(message), "{dialect:?}: {error}");
        }
    }
    let error = Netlist::parse("Invalid division\n.param a={b/0}\n.param b=85\n.end\n")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Division by zero") && error.contains("line 2"),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ngspice_parameters_stay_static_and_xyce_runtime_parameters_stay_symbolic() {
    let netlist =
        Netlist::parse("Static forward\n.param ambient={base+TEMP}\n.param base=58\n.end\n")
            .unwrap();
    assert_eq!(netlist.params.get("ambient"), Some(85.0));
    for special in ["TIME", "FREQ"] {
        let source =
            format!("Runtime forward\n.param value={{base+{special}}}\n.param base=58\n.end\n");
        let error = Netlist::parse(&source).unwrap_err().to_string();
        assert!(
            error.contains(special) && error.contains("line 2"),
            "{error}"
        );
        let netlist =
            Netlist::parse_with_options(&source, options(ExpressionDialect::Xyce)).unwrap();
        assert!(netlist.params.get_parameter_expression("value").is_some());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deferred_include_errors_retain_physical_path_and_logical_card_origin() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::env::temp_dir().join("rspice-forward-param-root.cir");
    let child = root.with_file_name("rspice-forward-param-child.inc");
    let source = "Included parameters\n.include child.inc\n.param later=85\n.end\n";
    for dialect in DIALECTS {
        let include = ".param known=1\n+ bad={missing+later}\n";
        let bundle = SealedSourceBundle::try_new_with_edges(
            [
                (root.clone(), source.to_owned()),
                (child.clone(), include.to_owned()),
            ],
            [SealedSourceEdge {
                owner: root.clone(),
                requested_path: "child.inc".into(),
                target: child.clone(),
            }],
        )
        .unwrap();
        let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            source,
            &root,
            bundle,
            options(dialect),
            &rspice_core::NoAbort,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains(&format!("{}:1", child.display())), "{error}");
        assert!(error.contains("MISSING"), "{error}");
        assert!(!error.contains(&root.display().to_string()), "{error}");
    }
}
