//! Temperature options bind forward dependencies in their declaration scope.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{AnalysisCommand, NetlistParseOptions};
use rspice_core::{Engine, Netlist};

const DIALECTS: [ExpressionDialect; 2] = [ExpressionDialect::Ngspice, ExpressionDialect::Xyce];

fn parse(source: &str, expression_dialect: ExpressionDialect) -> Netlist {
    Netlist::parse_with_options(
        source,
        NetlistParseOptions {
            expression_dialect,
            ..Default::default()
        },
    )
    .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn child_options_bind_completed_parent_root_and_header_declarations() {
    for dialect in DIALECTS {
        for body in [
            ".subckt parent p\n.subckt child q\n.options temp=+ambient tnom=-nominal\n.ends\n.param ambient={base+5} nominal=10\n.param base=80\n.ends\n",
            ".subckt parent p\n.subckt child q\n.options temp={ambient} tnom=-nominal\n.ends\n.ends\n.param ambient={base+5} nominal=10\n.param base=80\n",
            ".subckt parent p params: ambient={base+5} nominal={later}\n.subckt child q\n.options temp={ambient} tnom=-nominal\n.ends\n.param base=80 later=10\n.ends\n",
        ] {
            let netlist = parse(&format!("Parent temperatures\n{body}.end\n"), dialect);
            assert_eq!(netlist.options.temp, Some(85.0), "{body}");
            assert_eq!(netlist.options.tnom, Some(-10.0), "{body}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn parent_graphs_wait_for_final_definitions_and_ignore_child_overrides() {
    for dialect in DIALECTS {
        for before in [
            ".param ambient={base}\n",
            ".param ambient={base}\n.param base=80\n",
            ".param ambient={base}\n.param base={ambient}\n",
        ] {
            let source = format!(
                "Parent ownership\n.subckt parent p\n{before}.subckt child q\n.param base=999\n.param local={{ambient}}\n.options temp={{local}} tnom={{ambient}}\n.ends\n.param base=85\n.ends\n.end\n"
            );
            let netlist = parse(&source, dialect);
            assert_eq!(netlist.options.temp, Some(85.0), "{source}");
            assert_eq!(netlist.options.tnom, Some(85.0), "{source}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn delayed_options_share_parent_samples_with_parent_options_and_analyses() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Parent samples\n.options seed=37\n.subckt parent p\n.param shared={base+aunif(0,1)}\n.subckt child q\n.options temp={aunif(0,1)+shared}\n.ac lin 1 {shared} {shared}\n.param child_draw={aunif(0,1)}\n.ends\n.param parent_draw={aunif(0,1)}\n.param base=85\n.options tnom={shared}\n.ends\n.param last_draw={aunif(0,1)}\n.end\n",
            dialect,
        );
        let mut reference = ParamContext::new();
        reference.set_expression_dialect(dialect);
        reference.set_random_seed(37);
        let draw = || eval_expression("aunif(0,1)", &reference).unwrap();
        let _child = draw();
        let _parent = draw();
        let operand = draw();
        let shared = 85.0 + draw();
        assert_eq!(netlist.options.temp, Some(operand + shared));
        assert_eq!(netlist.options.tnom, Some(shared));
        assert_eq!(netlist.params.get("last_draw"), Some(draw()));
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            draw()
        );
        let AnalysisCommand::Ac {
            start_freq,
            stop_freq,
            ..
        } = netlist.analyses[0]
        else {
            panic!("AC")
        };
        assert_eq!((start_freq, stop_freq), (shared, shared));
        assert_eq!(netlist.params.get("shared"), None);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_owned_samples_materialize_once_across_closed_siblings_and_root_options() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Root sample\n.options seed=37\n.param shared={base+aunif(0,1)}\n.subckt child p\n.options temp={shared}\n.ends\n.subckt unused p\n.param shared=999\n.ends\n.subckt sibling p\n.options tnom={shared}\n.ac lin 1 {shared} {shared}\n.ends\n.param base=85\n.options temp={shared}\n.end\n",
            dialect,
        );
        let mut reference = ParamContext::new();
        reference.set_expression_dialect(dialect);
        reference.set_random_seed(37);
        let shared = 85.0 + eval_expression("aunif(0,1)", &reference).unwrap();
        assert_eq!(netlist.options.temp, Some(shared));
        assert_eq!(netlist.options.tnom, Some(shared));
        assert_eq!(netlist.params.get("shared"), Some(shared));
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
        let AnalysisCommand::Ac {
            start_freq,
            stop_freq,
            ..
        } = netlist.analyses[0]
        else {
            panic!("AC")
        };
        assert_eq!((start_freq, stop_freq), (shared, shared));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn completed_local_scopes_keep_their_original_sampling_phase() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Local phase\n.options seed=37\n.subckt parent p\n.subckt child q\n.options temp={local+aunif(0,1)}\n.param local=85\n.ends\n.param after={aunif(0,1)}\n.options tnom={after}\n.ends\n.param last_draw={aunif(0,1)}\n.end\n",
            dialect,
        );
        let mut reference = ParamContext::new();
        reference.set_expression_dialect(dialect);
        reference.set_random_seed(37);
        let draw = || eval_expression("aunif(0,1)", &reference).unwrap();
        assert_eq!(netlist.options.temp, Some(85.0 + draw()));
        assert_eq!(netlist.options.tnom, Some(draw()));
        assert_eq!(netlist.params.get("last_draw"), Some(draw()));
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            draw()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn delayed_options_keep_captured_functions_complex_values_and_lazy_branches() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Captured scope\n.subckt parent p\n.param offset=5 z={sqrt(-1)}\n.func adjust(x) {x+offset+img(z)}\n.subckt child q\n.param unused={instance_only+missing}\n.options temp={adjust(if(1,ambient,unused))}\n.ends\n.param ambient=81 offset=100 z={sqrt(-4)}\n.func adjust(x) {x+1000}\n.ends\n.end\n",
            dialect,
        );
        assert_eq!(netlist.options.temp, Some(85.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn delayed_parent_temperature_reconciles_earlier_values_and_physical_dc() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Physical temperature\n.param observed={TEMP}\nI1 0 out 1m\nR1 out 0 {TEMP}\n.subckt parent p\n.subckt child q\n.options temp={ambient}\n.ends\n.param ambient=85\n.ends\n.end\n",
            dialect,
        );
        assert_eq!(netlist.options.temp, Some(85.0));
        assert_eq!(netlist.params.get("observed"), Some(85.0));
        let voltage = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((voltage - 0.085).abs() < 1e-12, "{voltage}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sibling_definitions_never_complete_another_scope_or_skip_invalid_assignments() {
    for dialect in DIALECTS {
        for (tail, expected) in [
            (
                ".subckt sibling p\n.param ambient=85\n.ends\n",
                "Undefined parameter: AMBIENT",
            ),
            (".param ambient=-274\n", "absolute zero"),
            (
                ".param ambient={other}\n.param other={ambient}\n",
                "cyclic parameter dependency",
            ),
        ] {
            let source = format!(
                "Invalid delayed scope\n.subckt child p\n.options temp={{ambient}}\n.ends\n.options temp=65\n{tail}.end\n"
            );
            let error = Netlist::parse_with_options(
                &source,
                NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap_err();
            assert!(error.to_string().contains(expected), "{error}");
            assert!(
                matches!(
                    error,
                    rspice_core::netlist::ParseError::Syntax { line: 3, .. }
                ),
                "{error}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn delayed_included_assignment_errors_keep_physical_card_location() {
    use rspice_core::netlist::{
        ParseError, ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-parent-temperature/root.cir"
    } else {
        "/rspice-parent-temperature/root.cir"
    });
    let child = root.with_file_name("child.inc");
    let source = "Included delayed option\n.include child.inc\n.options temp=65\n.param ambient=-274\n.end\n";
    let include =
        ".subckt parent p\n.subckt child q\n.options reltol=1e-5\n+ temp={ambient}\n.ends\n.ends\n";
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
        NetlistParseOptions::default(),
        &rspice_core::NoAbort,
    )
    .unwrap_err();
    assert!(
        matches!(
            &error,
            ParseWithAbortError::Parse(ParseError::Syntax { line: 3, .. })
        ),
        "{error}"
    );
    assert!(error.to_string().contains("child.inc:3:"), "{error}");
}
