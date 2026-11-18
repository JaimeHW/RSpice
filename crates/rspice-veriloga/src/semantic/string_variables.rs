//! Strings where analog code reads them as values: a string variable, and
//! the file name `$fopen` takes or the format `$fscanf` takes.
//!
//! A string variable's slot holds the index of its string in the module's
//! string table, so assigning one and passing one to `$fopen` or `$fscanf`
//! are numeric operations every executable lowering runs as it runs any
//! other. The table holds every string an assignment can store: each string
//! literal and each string parameter's value assigned to a string variable.
//! A variable never assigned holds index 0, the empty string, which is a
//! string variable's initial value. A call that reads a string variable
//! carries the table, and reads the index the variable holds when the call
//! runs (LRM 2.4 section 9.5.1 takes a string expression for the name).
//!
//! A string parameter has no slot: its value is fixed when the module is
//! elaborated (`string_parameters`), so where a call takes a string it reads
//! as the literal it closed to.

use super::{SemanticAnalyzer, SymbolKind};
use crate::analog_tasks::AnalogSystemCall;
use crate::ast::{ConditionalExpr, Expression, Identifier, NumberLit};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use crate::types::ValueType;
use smol_str::SmolStr;

/// What a string argument of a call names.
pub(super) enum StringArgument {
    /// Text fixed when the module is elaborated: a string literal or a
    /// string parameter's value.
    Fixed(String),
    /// A string variable, read when the call runs.
    Variable,
}

fn refuse(detail: String, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(detail),
        span,
    ))
}

impl SemanticAnalyzer {
    /// The string parameter or string variable `identifier` names.
    fn string_symbol(&self, identifier: &Identifier) -> Option<(SymbolKind, SmolStr)> {
        let resolved = self.resolve_substituted_name(&identifier.name);
        let symbol = self.symbols.lookup(&resolved)?;
        (symbol.value_type == ValueType::String
            && matches!(symbol.kind, SymbolKind::Parameter | SymbolKind::Variable))
        .then_some((symbol.kind, resolved))
    }

    /// The index of `string` in the module's string table, added if new.
    fn intern_string(&mut self, string: &str, span: Span) -> Expression {
        let index = match self.string_table.iter().position(|held| held == string) {
            Some(index) => index,
            None => {
                self.string_table.push(string.into());
                self.string_table.len() - 1
            }
        };
        Expression::Number(NumberLit {
            value: index as f64,
            raw: index.to_string().into(),
            span,
        })
    }

    /// What a string variable is assigned, as the index its slot holds: a
    /// string literal, a string parameter, another string variable, or a
    /// condition choosing between them.
    pub(super) fn string_variable_value(
        &mut self,
        expression: &Expression,
    ) -> CompileResult<Expression> {
        match expression {
            Expression::StringLit(literal) => Ok(self.intern_string(&literal.value, literal.span)),
            Expression::Identifier(identifier) => match self.string_symbol(identifier) {
                Some((SymbolKind::Parameter, name)) => {
                    let value = self
                        .string_parameter_values
                        .get(&name)
                        .cloned()
                        .unwrap_or_default();
                    Ok(self.intern_string(&value, identifier.span))
                }
                Some(_) => Ok(expression.clone()),
                None => Err(refuse(
                    format!(
                        "a string variable is assigned a string literal, a string parameter or \
                         another string variable, not `{}`",
                        identifier.name
                    ),
                    identifier.span,
                )),
            },
            Expression::Conditional(conditional) => Ok(Expression::Conditional(ConditionalExpr {
                condition: conditional.condition.clone(),
                then_expr: Box::new(self.string_variable_value(&conditional.then_expr)?),
                else_expr: Box::new(self.string_variable_value(&conditional.else_expr)?),
                span: conditional.span,
            })),
            other => Err(refuse(
                "a string variable is assigned a string literal, a string parameter or another \
                 string variable"
                    .into(),
                other.span(),
            )),
        }
    }

    /// What the string argument `argument` of the call `name` names; `what`
    /// is the argument, for the message.
    pub(super) fn string_argument(
        &self,
        name: &str,
        argument: &Expression,
        what: &str,
    ) -> CompileResult<StringArgument> {
        let refused = || {
            refuse(
                format!(
                    "`{name}`: {what} must be a string literal, a string parameter or a string \
                     variable"
                ),
                argument.span(),
            )
        };
        match argument {
            Expression::StringLit(literal) => Ok(StringArgument::Fixed(literal.value.to_string())),
            Expression::Identifier(identifier) => match self.string_symbol(identifier) {
                Some((SymbolKind::Parameter, name)) => Ok(StringArgument::Fixed(
                    self.string_parameter_values
                        .get(&name)
                        .map(ToString::to_string)
                        .unwrap_or_default(),
                )),
                Some(_) => Ok(StringArgument::Variable),
                None => Err(refused()),
            },
            _ => Err(refused()),
        }
    }

    /// [`Self::string_argument`] where the text must be fixed when the module
    /// is elaborated: a string literal or a string parameter.
    pub(super) fn fixed_string_argument(
        &self,
        name: &str,
        argument: &Expression,
        what: &str,
    ) -> CompileResult<String> {
        match self.string_argument(name, argument, what) {
            Ok(StringArgument::Fixed(text)) => Ok(text),
            _ => Err(refuse(
                format!("`{name}`: {what} must be a string literal or a string parameter"),
                argument.span(),
            )),
        }
    }

    /// Give every call that reads a string variable the module's string
    /// table, now that every assignment has added its strings.
    pub(super) fn close_system_call_strings<S>(&self, calls: &mut [AnalogSystemCall<S>]) {
        for call in calls {
            if call.site.text_variable.is_some() {
                call.site.strings = self.string_table.iter().map(ToString::to_string).collect();
            }
        }
    }
}
