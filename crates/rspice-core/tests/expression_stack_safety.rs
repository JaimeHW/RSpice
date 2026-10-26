use rspice_core::Netlist;
use rspice_core::expr::parse_expression_strict;
use rspice_core::netlist::expr::parse_expression;

#[test]
fn oversized_poly_dimension_returns_a_diagnostic() {
    use rspice_core::netlist::expr::{ParamContext, prepare_behavioral_expression};
    let expression = format!("POLY({}) TIME 1", usize::MAX);
    let error = prepare_behavioral_expression(&expression, &ParamContext::new())
        .expect_err("impossible polynomial dimension must be rejected");
    assert!(error.contains("POLY"), "{error}");
}

#[test]
fn polynomial_expansion_is_bounded_before_expression_parsing() {
    use rspice_core::netlist::expr::{ParamContext, prepare_behavioral_expression};
    let expression = format!("POLY(1) TIME {}", "1 ".repeat(4096));
    let error = prepare_behavioral_expression(&expression, &ParamContext::new())
        .expect_err("polynomial factors must not outgrow the expression tree");
    assert!(error.contains("stack safety limit"), "{error}");

    let expression = format!("POLY(1) {} {}", "x".repeat(100_000), "1 ".repeat(32));
    let error = prepare_behavioral_expression(&expression, &ParamContext::new())
        .expect_err("large repeated operands must not outgrow the generated source budget");
    assert!(error.contains("byte limit"), "{error}");
}

#[test]
fn function_argument_duplication_respects_the_expansion_budget() {
    use rspice_core::netlist::expr::{ParamContext, prepare_behavioral_expression};
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(|| {
            let mut context = ParamContext::new();
            let repeated = vec!["X"; 32].join(",");
            context.define_function("F0", vec!["X".to_owned()], "X");
            for index in 1..8 {
                context.define_function(
                    &format!("F{index}"),
                    vec!["X".to_owned()],
                    &format!("F{}(max({repeated}))", index - 1),
                );
            }
            let error = prepare_behavioral_expression("F7(TIME)", &context)
                .expect_err("repeated arguments must not outgrow the expression budget");
            assert!(error.contains("expansion exceeded"), "{error}");
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn runtime_function_expansion_cannot_build_an_unbounded_boxed_tree() {
    use rspice_core::netlist::expr::{ParamContext, prepare_behavioral_expression};
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(|| {
            for expand_argument in [false, true] {
                let mut context = ParamContext::new();
                context.define_function("F0", vec!["X".to_owned()], "X+TIME");
                for index in 1..1000 {
                    let body = if expand_argument {
                        format!("F{}(X+TIME)", index - 1)
                    } else {
                        format!("F{}(X)+TIME", index - 1)
                    };
                    context.define_function(&format!("F{index}"), vec!["X".to_owned()], &body);
                }
                let error = prepare_behavioral_expression("F999(1)", &context)
                    .expect_err("runtime expansion must respect the retained tree bound");
                assert!(error.contains("stack safety limit"), "{error}");
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn recursive_expression_forms_return_diagnostics_on_a_desktop_sized_stack() {
    // A separate thread exercises the Windows executable's usual 1 MiB stack,
    // independently of the test runner's larger stack.
    recursive_expression_forms_return_diagnostics_within(1024 * 1024);
}

#[test]
fn recursive_expression_forms_keep_a_quarter_of_a_desktop_stack_spare() {
    // The same work a quarter MiB short of the desktop stack: the nesting
    // limits leave at least that much of a 1 MiB thread unused in an
    // unoptimized build, whatever features the build carries.
    recursive_expression_forms_return_diagnostics_within(768 * 1024);
}

fn recursive_expression_forms_return_diagnostics_within(stack_size: usize) {
    std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            let nesting = 4096;
            let expressions = [
                format!("{}1{}", "(".repeat(nesting), ")".repeat(nesting)),
                format!("{}1{}", "{".repeat(nesting), "}".repeat(nesting)),
                format!("{}1", "-".repeat(nesting)),
                format!("{}1", "+".repeat(nesting)),
                format!("{}1", "!".repeat(nesting)),
                format!("{}1{}", "sin(".repeat(nesting), ")".repeat(nesting)),
                format!("{}1", "1?1:".repeat(nesting)),
                format!("{}1", "2^-".repeat(nesting)),
                format!("{}1", "1+".repeat(nesting)),
            ];
            for expression in &expressions {
                let behavioral = parse_expression_strict(expression);
                let parameter = parse_expression(expression);
                assert!(
                    behavioral.is_err(),
                    "behavioral parser accepted excessive nesting"
                );
                assert!(
                    parameter.is_err(),
                    "parameter parser accepted excessive nesting"
                );
            }
            for deck in [
                format!(
                    "deep parameter\n.param x={}\nR1 1 0 1k\n.end\n",
                    expressions[0]
                ),
                format!("deep source\nB1 1 0 V={}\n.end\n", expressions[0]),
            ] {
                let result = Netlist::parse(&deck)
                    .map_err(|error| error.to_string())
                    .and_then(|netlist| {
                        rspice_core::Engine::default()
                            .run_dc_op(&netlist)
                            .map(|_| ())
                            .map_err(|error| error.to_string())
                    });
                let error = result.expect_err("deck must reject excessive nesting");
                assert!(error.contains("stack safety limit"), "{error}");
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn expanded_function_sums_are_not_refused_as_deep_nesting() {
    // Xyce certification deck BUG_1222_SON/bug_1222_son_1: `.PRINT {dino(...)}`
    // calls .FUNCs five deep whose bodies are flat sums of ten terms and
    // `&` chains of four comparisons. The authored nesting is shallow; the
    // expanded sums must not read back as one group per term.
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::measure_signals::evaluate_dc_output_requests_with_abort;
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;
    use rspice_core::resource::ResourceLimits;

    let netlist = Netlist::parse_with_options(
        include_str!(
            "../../../tests/xyce/Netlists/Certification_Tests/BUG_1222_SON/bug_1222_son_1.cir"
        ),
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..NetlistParseOptions::default()
        },
    )
    .expect("bug_1222_son_1 parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..SimulationConfig::default()
    });
    let sweep = engine
        .run_dc_sweep(&netlist, "V1", 1.0, 1.0, 1.0)
        .expect("bug_1222_son_1 DC sweep runs");
    let columns = evaluate_dc_output_requests_with_abort(
        &netlist,
        &sweep,
        ResourceLimits::default(),
        &NoAbort,
    )
    .expect("every authored .PRINT column evaluates");

    // Xyce 7.10, OutputData/Certification_Tests/BUG_1222_SON/bug_1222_son_1.cir.prn,
    // for V(1), V(2) and the three BETTY columns. The two DINO columns are
    // only required to evaluate here: Xyce prints 0 for both, where the
    // deck's lexically scoped .FUNC tables give 6e-9 and 9e-9.
    let expected = [
        Some(1.0),
        Some(0.0),
        None,
        None,
        Some(7.0e-9),
        Some(7.0e-9),
        Some(9.0e-9),
    ];
    assert_eq!(columns.len(), expected.len(), "{columns:?}");
    for ((name, _, values), expected) in columns.iter().zip(expected) {
        assert_eq!(values.len(), 1, "{name}");
        assert!(values[0].is_finite(), "{name}: got {}", values[0]);
        if let Some(expected) = expected {
            assert!(
                (values[0] - expected).abs() <= 1.0e-6 * expected.abs() + 1.0e-15,
                "{name}: got {}, Xyce 7.10 gives {expected}",
                values[0]
            );
        }
    }
}
