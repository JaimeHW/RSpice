//! Bind result-owned expression syntax to the canonical deck numeric reader.

use rspice_results::calculator::ast::CalculatorExpr;
use rspice_results::calculator::parser as syntax;
pub use rspice_results::calculator::parser::ParseError;
use rspice_simulation_contract::spice_value::parse_spice_value_checked;

pub struct Parser<'a>(syntax::Parser<'a>);

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(syntax::Parser::new(input, parse_spice_value_checked))
    }

    pub fn try_parse(&mut self) -> Result<CalculatorExpr, ParseError> {
        self.0.try_parse()
    }
}

pub fn try_parse(input: &str) -> Result<CalculatorExpr, ParseError> {
    syntax::try_parse(input, parse_spice_value_checked)
}

#[cfg(test)]
mod tests {
    use super::try_parse;
    use rspice_results::calculator::ast::{BinaryOp, CalculatorConstant, CalculatorExpr};

    #[test]
    fn parses_spice_suffix_numeric_literals() {
        assert_eq!(try_parse("1k").unwrap(), CalculatorExpr::Number(1.0e3));
        assert_eq!(try_parse("1MHz").unwrap(), CalculatorExpr::Number(1.0e6));
        match try_parse("2.5u").unwrap() {
            CalculatorExpr::Number(value) => assert!((value - 2.5e-6).abs() < 1.0e-18),
            other => panic!("expected numeric literal, got {other:?}"),
        }
        assert_eq!(try_parse("3meg").unwrap(), CalculatorExpr::Number(3.0e6));
    }

    #[test]
    fn numeric_probe_tokens_preserve_authored_spelling() {
        for node in ["0", "00", "001", "1k", "1e-3", "1.25", "2µ"] {
            let signal = format!("V({node})");
            assert_eq!(
                try_parse(&format!(" V ( {node} ) ")).unwrap(),
                CalculatorExpr::wave(&signal)
            );
        }
    }

    #[test]
    fn probe_arguments_are_literal_names_before_arithmetic_tokenization() {
        for name in [
            "out+",
            "out-",
            "-1",
            "n$bias",
            "n#1",
            "1e999",
            "1e",
            "2µ",
            "/X1/1e999",
        ] {
            for accessor in ["V", "I"] {
                let signal = format!("{accessor}({name})");
                assert_eq!(
                    try_parse(&format!(" {accessor} ( {name} ) / 2 ")).unwrap(),
                    CalculatorExpr::binary(
                        BinaryOp::Div,
                        CalculatorExpr::wave(&signal),
                        CalculatorExpr::Number(2.0),
                    ),
                    "{signal}",
                );
            }
        }
        assert_eq!(
            try_parse(r#"V("out name")"#).unwrap(),
            CalculatorExpr::wave("V(out name)")
        );
        for text in [
            "V()",
            "V( )",
            "V(out name)",
            "V(out,in)",
            "I(out,in)",
            "V((out))",
            "V(out",
            r#"V("out)"#,
            "1e999",
            "1e",
        ] {
            assert!(try_parse(text).is_err(), "{text}");
        }
    }

    #[test]
    fn parses_quoted_hierarchical_signal_names() {
        let expr = try_parse(r#"V("/top/out")"#).unwrap();
        assert_eq!(expr, CalculatorExpr::wave("V(/top/out)"));
    }

    #[test]
    fn calculator_lexes_prefix_slash_paths_but_keeps_infix_division() {
        for (text, expected) in [
            ("V(/X1/out)", CalculatorExpr::wave("V(/X1/out)")),
            ("/X1/out", CalculatorExpr::wave("/X1/out")),
            (
                "avg(/X1/out)",
                CalculatorExpr::func("avg", vec![CalculatorExpr::wave("/X1/out")]),
            ),
            (
                "2 * /X1/out",
                CalculatorExpr::binary(
                    BinaryOp::Mul,
                    CalculatorExpr::Number(2.0),
                    CalculatorExpr::wave("/X1/out"),
                ),
            ),
        ] {
            assert_eq!(try_parse(text).unwrap(), expected, "{text}");
        }

        for (text, left, right) in [
            ("a/b", CalculatorExpr::wave("a"), CalculatorExpr::wave("b")),
            (
                "V(a)/V(b)",
                CalculatorExpr::wave("V(a)"),
                CalculatorExpr::wave("V(b)"),
            ),
            (
                "V(/X1/out)/V(out)",
                CalculatorExpr::wave("V(/X1/out)"),
                CalculatorExpr::wave("V(out)"),
            ),
            (
                "(a+b)/2",
                CalculatorExpr::binary(
                    BinaryOp::Add,
                    CalculatorExpr::wave("a"),
                    CalculatorExpr::wave("b"),
                ),
                CalculatorExpr::Number(2.0),
            ),
        ] {
            assert_eq!(
                try_parse(text).unwrap(),
                CalculatorExpr::binary(BinaryOp::Div, left, right),
                "{text} still divides"
            );
        }
    }

    #[test]
    fn a_domain_name_followed_by_a_parenthesis_is_a_function_call() {
        // Bare, these name the sweep's own axis.
        assert_eq!(
            try_parse("freq").unwrap(),
            CalculatorExpr::Constant(CalculatorConstant::Frequency)
        );
        assert_eq!(
            try_parse("TIME").unwrap(),
            CalculatorExpr::Constant(CalculatorConstant::Time)
        );

        // Called, they are the measurement functions of the same name.
        assert_eq!(
            try_parse("freq(V(out))").unwrap(),
            CalculatorExpr::func("freq", vec![CalculatorExpr::wave("V(out)")])
        );
        assert_eq!(
            try_parse("2 * freq(V(out))").unwrap(),
            CalculatorExpr::binary(
                BinaryOp::Mul,
                CalculatorExpr::Number(2.0),
                CalculatorExpr::func("freq", vec![CalculatorExpr::wave("V(out)")]),
            )
        );
    }

    #[test]
    fn rejects_malformed_expressions_instead_of_zero_recovery() {
        for text in ["", "1+", "1 2", "1e", "V()", "avg(V(out)", "1$2"] {
            assert!(
                try_parse(text).is_err(),
                "expression {text:?} should be rejected"
            );
        }
    }
}
