use rspice_core::Netlist;
use rspice_core::expr::parse_expression_strict;
use rspice_core::netlist::expr::parse_expression;

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
