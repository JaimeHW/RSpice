use rspice_core::Netlist;
use rspice_core::expr::parse_expression_strict;
use rspice_core::netlist::expr::parse_expression;

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
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
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
