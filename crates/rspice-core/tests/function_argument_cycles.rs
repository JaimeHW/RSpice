use rspice_core::netlist::expr::{ExprError, ParamContext, eval_expression_complex};

#[test]
fn expression_argument_cycles_report_errors_without_growing_the_evaluator_stack() {
    for (name, body, expression) in [
        ("F", "IF(X<=0,1,F(X-1)+F(X-1))", "F(2)"),
        ("F", "X+F(X)", "F(1)"),
        ("G", "X+F(X)", "G(1)"),
    ] {
        let mut context = ParamContext::new();
        context.define_function("F", vec!["X".into()], "X+G(X)");
        context.define_function(name, vec!["X".into()], body);
        let error = eval_expression_complex(expression, &context).unwrap_err();
        assert!(
            matches!(&error, ExprError::InvalidArgument(message) if message.contains("cyclic user-function argument binding")),
            "{expression}: {error}"
        );
        // A failed evaluation cannot leave argument bindings on the context.
        context.define_function("F", vec!["X".into()], "X+1");
        assert_eq!(
            eval_expression_complex("F(2)", &context).unwrap(),
            3.0.into()
        );
    }
}
