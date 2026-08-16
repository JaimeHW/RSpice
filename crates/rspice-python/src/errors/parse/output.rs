//! Attribute projections for the output-validation family: unresolved symbols
//! named by `.PRINT`/`.PLOT`/`.SAVE` and malformed output expressions.

use super::ParseErrorAttributes;

pub(super) fn output_symbol_validation_attributes(
    error: &rspice_core::netlist::OutputSymbolValidationError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("undefined_output_symbols");
    attributes.category = Some("output_symbol_validation");
    let count = error.unresolved.len();
    attributes.detail = Some(format!(
        "{count} unresolved output symbol{}",
        if count == 1 { "" } else { "s" }
    ));
    attributes.unresolved_output_symbols = Some(error.unresolved.iter().map(Into::into).collect());
    if let Some(first) = error.unresolved.first() {
        attributes.set_primary(&first.origin);
    }
    attributes
}

pub(super) fn output_expression_validation_attributes(
    error: &rspice_core::netlist::OutputExpressionValidationError,
) -> ParseErrorAttributes {
    use rspice_core::netlist::OutputExpressionIssue;

    let mut attributes = ParseErrorAttributes::new("invalid_output_expression");
    attributes.category = Some("output_expression_validation");
    attributes.set_primary(&error.origin);
    attributes.expression = Some(error.expression.clone());
    attributes.output_directive = Some(error.directive.to_string());
    attributes.reason = Some(error.issue.reason());
    match &error.issue {
        OutputExpressionIssue::UnknownFunction { function } => {
            attributes.kind = "unknown_output_function";
            attributes.function_name = Some(function.clone());
        }
        OutputExpressionIssue::UnresolvedIdentifier { identifier } => {
            attributes.kind = "unresolved_output_identifier";
            attributes.identifier_name = Some(identifier.clone());
        }
        OutputExpressionIssue::InvalidAccessor { operator, detail } => {
            attributes.kind = "invalid_output_accessor";
            attributes.operator_name = Some(operator.clone());
            attributes.detail = Some(detail.clone());
        }
        OutputExpressionIssue::UnresolvedDeviceParameter { device, parameter } => {
            attributes.kind = "unresolved_output_device_parameter";
            attributes.device = Some(device.clone());
            attributes.parameter_name = Some(parameter.clone());
        }
        OutputExpressionIssue::Syntax { detail } => {
            attributes.kind = "invalid_output_expression_syntax";
            attributes.detail = Some(detail.clone());
        }
    }
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        NetlistSourceLocation, OutputDirectiveKind, OutputExpressionIssue,
        OutputExpressionValidationError, OutputSymbolKind, OutputSymbolValidationError,
        UnresolvedOutputSymbol,
    };

    #[test]
    fn output_expression_validation_exposes_structured_python_attributes() {
        let error = OutputExpressionValidationError {
            directive: OutputDirectiveKind::Print,
            origin: NetlistSourceLocation {
                path: Some(std::path::PathBuf::from("bad_function.cir")),
                line: 9,
            },
            expression: "fabs(v(1))".into(),
            issue: OutputExpressionIssue::UnknownFunction {
                function: "FABS".into(),
            },
        };
        let attributes = output_expression_validation_attributes(&error);
        assert_eq!(attributes.kind, "unknown_output_function");
        assert_eq!(attributes.category, Some("output_expression_validation"));
        assert_eq!(attributes.line, Some(9));
        assert_eq!(attributes.expression.as_deref(), Some("fabs(v(1))"));
        assert_eq!(attributes.output_directive.as_deref(), Some(".PRINT"));
        assert_eq!(attributes.function_name.as_deref(), Some("FABS"));
        assert_eq!(attributes.identifier_name, None);
        assert_eq!(
            attributes.reason.as_deref(),
            Some("unknown function 'FABS'")
        );
    }

    #[test]
    fn output_expression_error_is_declared_by_the_public_type_stub() {
        let stub = include_str!("../../../rspice.pyi");
        for declaration in [
            "\"unknown_output_function\"",
            "\"unresolved_output_identifier\"",
            "\"invalid_output_accessor\"",
            "\"unresolved_output_device_parameter\"",
            "\"invalid_output_expression_syntax\"",
            "\"output_expression_validation\"",
            "output_directive: str | None",
            "operator_name: str | None",
            "function_name: str | None",
            "identifier_name: str | None",
        ] {
            assert!(
                stub.contains(declaration),
                "Python type stub omitted {declaration}"
            );
        }
    }

    #[test]
    fn output_symbol_validation_preserves_order_repetitions_and_provenance() {
        let item = |operator: &str, symbol: &str, kind| UnresolvedOutputSymbol {
            directive: OutputDirectiveKind::Print,
            origin: NetlistSourceLocation::in_file("invalid.cir", 17),
            operator: operator.into(),
            symbol: symbol.into(),
            kind,
        };
        let error = OutputSymbolValidationError {
            unresolved: vec![
                item("I", "RBogo", OutputSymbolKind::Device),
                item("VP", "bogo9", OutputSymbolKind::Node),
                item("VM", "bogo9", OutputSymbolKind::Node),
            ],
        };
        let attributes = output_symbol_validation_attributes(&error);

        assert_eq!(attributes.kind, "undefined_output_symbols");
        assert_eq!(attributes.category, Some("output_symbol_validation"));
        assert_eq!(attributes.line, Some(17));
        assert_eq!(attributes.source.as_deref(), Some("invalid.cir"));
        assert_eq!(
            attributes.detail.as_deref(),
            Some("3 unresolved output symbols")
        );
        let unresolved = attributes
            .unresolved_output_symbols
            .expect("aggregate items are exposed");
        assert_eq!(
            unresolved
                .iter()
                .map(|item| {
                    (
                        item.directive.as_str(),
                        item.operator.as_str(),
                        item.symbol.as_str(),
                        item.kind.as_str(),
                        item.line,
                        item.source.as_deref(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                ("print", "I", "RBogo", "device", 17, Some("invalid.cir")),
                ("print", "VP", "bogo9", "node", 17, Some("invalid.cir")),
                ("print", "VM", "bogo9", "node", 17, Some("invalid.cir")),
            ]
        );
    }
}
