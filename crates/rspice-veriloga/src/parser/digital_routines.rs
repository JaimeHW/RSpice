//! Tasks, functions and the procedural thread statements (IEEE 1364-2005
//! sections 9.7.3, 9.7.6, 9.8.2, 10.2-10.4): `task`/`function` declarations,
//! task enables, `fork`/`join`, `disable`, `wait`, `-> event`, and the
//! `time`, `realtime` and `event` declarations.

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;

impl Parser<'_> {
    /// Whether the cursor is the reserved digital keyword `text`.
    ///
    /// `time`, `realtime`, `event` and `wait` lex as reserved digital words;
    /// the parser tells them apart by their spelling.
    pub(super) fn digital_keyword_is(&self, text: &str) -> bool {
        self.check(TokenKind::AmsDigital) && self.current().text.as_deref() == Some(text)
    }

    /// `time name, ...;` (an unsigned 64-bit variable, IEEE 1364-2005 section
    /// 4.8) or `event name, ...;` (section 9.7.3).
    pub(super) fn parse_keyword_variable_decl(
        &mut self,
    ) -> Result<DigitalVariableDecl, ParseError> {
        let start = self.current_span();
        let time = self.digital_keyword_is("time");
        self.advance();
        let (kind, range) = if time {
            (DigitalVariableKind::Reg, Some(time_range(start)))
        } else {
            (DigitalVariableKind::Event, None)
        };
        let mut items = self.parse_digital_decl_items()?;
        if kind == DigitalVariableKind::Event {
            // An event's storage starts at a known level so that the first
            // trigger is a change.
            for item in &mut items {
                item.init.get_or_insert_with(|| {
                    Expression::Number(NumberLit {
                        value: 0.0,
                        raw: "0".into(),
                        span: item.span,
                    })
                });
            }
        }
        Ok(DigitalVariableDecl {
            kind,
            signedness: Signedness::Unsigned,
            range,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// Whether the cursor starts a block-item declaration that is not `reg`,
    /// `integer`, `real` or `string`: `time`, `realtime` or `event`.
    pub(super) fn keyword_declaration_follows(&self) -> bool {
        ["time", "realtime", "event"]
            .iter()
            .any(|keyword| self.digital_keyword_is(keyword))
    }

    /// One `time`, `realtime` or `event` block-item declaration, into the
    /// list it belongs to.
    pub(super) fn parse_keyword_declaration(
        &mut self,
        variables: &mut Vec<VariableDecl>,
        digital_variables: &mut Vec<DigitalVariableDecl>,
    ) -> Result<(), ParseError> {
        if self.digital_keyword_is("realtime") {
            variables.push(self.parse_variable_decl()?);
        } else if self.digital_keyword_is("event") {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "event".to_string(),
                    found: "a named event declared inside a task, function or block; declare \
                            it in the module"
                        .to_string(),
                },
                self.current_span(),
            ));
        } else {
            digital_variables.push(self.parse_keyword_variable_decl()?);
        }
        Ok(())
    }

    /// `task [automatic] name [(ports)]; items statement endtask` or
    /// `function [automatic] [type] name [(ports)]; items statement
    /// endfunction` (IEEE 1364-2005 sections 10.2.1 and 10.4.1).
    pub(super) fn parse_digital_routine(&mut self) -> Result<DigitalRoutine, ParseError> {
        let start = self.current_span();
        let is_task = self.check(TokenKind::Task);
        self.advance();
        let automatic = self.check(TokenKind::Identifier)
            && self.current().text.as_deref() == Some("automatic")
            && !self.peek_is(TokenKind::Semicolon)
            && !self.peek_is(TokenKind::LParen);
        if automatic {
            self.advance();
        }
        let kind = if is_task {
            DigitalRoutineKind::Task
        } else {
            DigitalRoutineKind::Function(Box::new(self.parse_function_result_type()?))
        };
        let name: SmolStr = self
            .expect_identifier(if is_task {
                "task name"
            } else {
                "function name"
            })?
            .into();
        let mut arguments = Vec::new();
        if self.match_token(TokenKind::LParen) {
            if !self.check(TokenKind::RParen) {
                self.parse_routine_port_list(&mut arguments)?;
            }
            self.expect(TokenKind::RParen)?;
        }
        self.expect(TokenKind::Semicolon)?;

        let mut variables = Vec::new();
        let mut digital_variables = Vec::new();
        loop {
            self.parse_attributes()?;
            match self.current().kind {
                TokenKind::Input | TokenKind::Output | TokenKind::Inout => {
                    self.parse_routine_argument_declaration(&mut arguments)?;
                }
                TokenKind::Reg => {
                    digital_variables.push(self.parse_digital_variable_decl(None)?);
                }
                TokenKind::Real | TokenKind::Integer | TokenKind::String => {
                    variables.push(self.parse_variable_decl()?);
                }
                TokenKind::Parameter | TokenKind::Localparam => {
                    return Err(ParseError::new(
                        ParseErrorKind::UnsupportedConstruct {
                            context: if is_task { "task" } else { "function" }.to_string(),
                            found: "a parameter declared inside a task or function; \
                                    declare it in the module"
                                .to_string(),
                        },
                        self.current_span(),
                    ));
                }
                _ if self.keyword_declaration_follows() => {
                    self.parse_keyword_declaration(&mut variables, &mut digital_variables)?;
                }
                _ => break,
            }
        }
        type_arguments(&mut arguments, &mut variables, &mut digital_variables);

        let end = if is_task {
            TokenKind::Endtask
        } else {
            TokenKind::Endfunction
        };
        let body = if self.check(end) {
            DigitalStatement::Null(self.current_span())
        } else {
            self.parse_digital_statement()?
        };
        self.expect(end)?;
        Ok(DigitalRoutine {
            kind,
            automatic,
            name,
            arguments,
            variables,
            digital_variables,
            body,
            span: start.extend(self.previous_span()),
        })
    }

    /// A function's result: `[signed] [range]`, `integer`, `real`,
    /// `realtime` or `time` (IEEE 1364-2005 section 10.4.1).
    fn parse_function_result_type(&mut self) -> Result<DigitalRoutineType, ParseError> {
        if let Some(ty) = self.parse_keyword_value_type() {
            return Ok(ty);
        }
        // IEEE 1800's `function reg [7:0] f`, which both reference
        // simulators accept in a `.v` source.
        self.match_token(TokenKind::Reg);
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        Ok(DigitalRoutineType {
            kind: DigitalRoutineValueKind::Reg,
            signedness,
            range,
        })
    }

    /// `integer`, `real`, `realtime` or `time` as a routine value type.
    fn parse_keyword_value_type(&mut self) -> Option<DigitalRoutineType> {
        let (kind, signedness) = if self.check(TokenKind::Integer) {
            (DigitalRoutineValueKind::Integer, Signedness::Signed)
        } else if self.check(TokenKind::Real) || self.digital_keyword_is("realtime") {
            (DigitalRoutineValueKind::Real, Signedness::Signed)
        } else if self.digital_keyword_is("time") {
            (DigitalRoutineValueKind::Time, Signedness::Unsigned)
        } else {
            return None;
        };
        self.advance();
        Some(DigitalRoutineType {
            kind,
            signedness,
            range: None,
        })
    }

    /// An argument's type after its direction: `[reg] [signed] [range]` or a
    /// keyword type.
    fn parse_routine_argument_type(&mut self) -> Result<DigitalRoutineType, ParseError> {
        if let Some(ty) = self.parse_keyword_value_type() {
            return Ok(ty);
        }
        self.match_token(TokenKind::Reg);
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        Ok(DigitalRoutineType {
            kind: DigitalRoutineValueKind::Reg,
            signedness,
            range,
        })
    }

    fn routine_direction(&self) -> Option<PortDirection> {
        Some(match self.current().kind {
            TokenKind::Input => PortDirection::Input,
            TokenKind::Output => PortDirection::Output,
            TokenKind::Inout => PortDirection::Inout,
            _ => return None,
        })
    }

    /// `(input [7:0] a, b, output c)`: a name without a direction continues
    /// the previous one (IEEE 1364-2005 section 10.2.1).
    fn parse_routine_port_list(
        &mut self,
        arguments: &mut Vec<DigitalRoutineArgument>,
    ) -> Result<(), ParseError> {
        let mut current: Option<(PortDirection, DigitalRoutineType)> = None;
        loop {
            self.parse_attributes()?;
            if let Some(direction) = self.routine_direction() {
                self.advance();
                current = Some((direction, self.parse_routine_argument_type()?));
            } else if current.is_none()
                || self.check(TokenKind::Reg)
                || self.check(TokenKind::Integer)
                || self.check(TokenKind::Real)
                || self.digital_keyword_is("time")
                || self.digital_keyword_is("realtime")
            {
                // IEEE 1364-2005 section 10.2.1 (syntax 10-1): every port
                // declaration starts with its direction; only a further name
                // continues the previous one. Defaulting to `input`, or
                // restating a type alone, is IEEE 1800.
                return Err(ParseError::new(
                    ParseErrorKind::Expected {
                        expected: "`input`, `output` or `inout` before a routine argument \
                                   declaration (IEEE 1364-2005 section 10.2.1)"
                            .to_string(),
                        found: format!("{:?}", self.current().kind),
                    },
                    self.current_span(),
                ));
            }
            let span = self.current_span();
            let name: SmolStr = self.expect_identifier("argument name")?.into();
            let (direction, ty) = current.clone().expect("set above");
            arguments.push(DigitalRoutineArgument {
                direction,
                ty,
                name,
                span,
            });
            if !self.match_token(TokenKind::Comma) {
                return Ok(());
            }
        }
    }

    /// `input [7:0] a, b;` as a routine item declaration.
    fn parse_routine_argument_declaration(
        &mut self,
        arguments: &mut Vec<DigitalRoutineArgument>,
    ) -> Result<(), ParseError> {
        let direction = self.routine_direction().expect("caller checked");
        self.advance();
        let ty = self.parse_routine_argument_type()?;
        loop {
            let span = self.current_span();
            let name: SmolStr = self.expect_identifier("argument name")?.into();
            arguments.push(DigitalRoutineArgument {
                direction,
                ty: ty.clone(),
                name,
                span,
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    /// `fork [: name] declarations statements join` (IEEE 1364-2005 section
    /// 9.8.2). IEEE 1800's `join_any` and `join_none` are refused by name.
    pub(super) fn parse_digital_fork(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `fork`
        let name = if self.match_token(TokenKind::Colon) {
            Some(self.expect_identifier("block name")?.into())
        } else {
            None
        };
        let mut variables = Vec::new();
        let mut digital_variables = Vec::new();
        let mut statements = Vec::new();
        while !self.check(TokenKind::Join) && !self.at_end() {
            if self.check(TokenKind::Identifier)
                && matches!(
                    self.current().text.as_deref(),
                    Some("join_any" | "join_none")
                )
            {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: "fork".to_string(),
                        found: format!(
                            "`{}`, an IEEE 1800 fork form; IEEE 1364 `fork` ends with `join`",
                            self.current().text.as_deref().unwrap_or_default()
                        ),
                    },
                    self.current_span(),
                ));
            }
            self.parse_attributes()?;
            let declaration = matches!(
                self.current().kind,
                TokenKind::Real | TokenKind::Integer | TokenKind::String | TokenKind::Reg
            ) || self.keyword_declaration_follows();
            if declaration && name.is_none() {
                // IEEE 1364-2005 section 9.8.2 (syntax 9-14): only a named
                // block declares; IEEE 1800 relaxed this.
                return Err(ParseError::new(
                    ParseErrorKind::Expected {
                        expected: "a statement: an unnamed `fork` block declares nothing \
                                   (IEEE 1364-2005 section 9.8.2)"
                            .to_string(),
                        found: format!("{:?}", self.current().kind),
                    },
                    self.current_span(),
                ));
            }
            match self.current().kind {
                TokenKind::Real | TokenKind::Integer | TokenKind::String => {
                    variables.push(self.parse_variable_decl()?);
                }
                TokenKind::Reg => {
                    digital_variables.push(self.parse_digital_variable_decl(None)?);
                }
                _ if self.keyword_declaration_follows() => {
                    self.parse_keyword_declaration(&mut variables, &mut digital_variables)?;
                }
                _ => statements.push(self.parse_digital_statement()?),
            }
        }
        self.expect(TokenKind::Join)?;
        Ok(DigitalStatement::Fork(DigitalBlock {
            name,
            variables,
            digital_variables,
            statements,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `disable name;` (IEEE 1364-2005 section 10.3).
    pub(super) fn parse_digital_disable(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `disable`
        if self.check(TokenKind::Fork) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "disable".to_string(),
                    found: "`disable fork`, an IEEE 1800 statement".to_string(),
                },
                self.current_span(),
            ));
        }
        let target = self.parse_reference_name("block or task name")?;
        self.expect(TokenKind::Semicolon)?;
        Ok(DigitalStatement::Disable(DigitalDisable {
            target,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `wait (condition) statement_or_null` (IEEE 1364-2005 section 9.7.6).
    pub(super) fn parse_digital_wait(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `wait`
        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let statement = if self.match_token(TokenKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_digital_statement()?))
        };
        Ok(DigitalStatement::Wait(DigitalWaitStatement {
            condition,
            statement,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Whether the cursor spells `->` with nothing between the two tokens.
    pub(super) fn event_trigger_follows(&self) -> bool {
        let current = self.current();
        let Some(next) = self.tokens.get(self.pos + 1) else {
            return false;
        };
        current.kind == TokenKind::Minus
            && next.kind == TokenKind::Gt
            && current.span.end == next.span.start
    }

    /// `-> event;` (IEEE 1364-2005 section 9.7.3).
    pub(super) fn parse_event_trigger(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // `-`
        self.advance(); // `>`
        let target = self.parse_digital_lvalue()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(DigitalStatement::EventTrigger(DigitalEventTrigger {
            target,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `name;` or `name(args);`, told from an assignment by what follows the
    /// name. Returns `None` with the cursor restored when it is not one.
    pub(super) fn try_parse_task_enable(&mut self) -> Result<Option<DigitalStatement>, ParseError> {
        let saved = self.pos;
        let start = self.current_span();
        let Ok(name) = self.parse_reference_name("task name") else {
            self.pos = saved;
            return Ok(None);
        };
        let args = if self.check(TokenKind::Semicolon) {
            Vec::new()
        } else if self.check(TokenKind::LParen) {
            self.parse_arg_list()?
        } else {
            self.pos = saved;
            return Ok(None);
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Some(DigitalStatement::TaskEnable(DigitalTaskEnable {
            name,
            args,
            span: start.extend(self.previous_span()),
        })))
    }
}

/// `input x; real x;`: a variable declaration that names an argument declared
/// without a type gives the argument its type (IEEE 1364-2005 section 12.3.3,
/// which section 10.2.1 applies to routine ports), rather than declaring a
/// second `x`.
fn type_arguments(
    arguments: &mut [DigitalRoutineArgument],
    variables: &mut Vec<VariableDecl>,
    digital_variables: &mut Vec<DigitalVariableDecl>,
) {
    // A port is typed once. A `reg` redeclaration repeats the port's range
    // exactly; any other type needs a port declared with none. Anything else
    // stays a second declaration, which analysis reports as a duplicate.
    let mut typed = std::collections::HashSet::new();
    let mut untyped = |arguments: &mut [DigitalRoutineArgument],
                       name: &SmolStr,
                       reg_range: Option<Option<&VectorRange>>|
     -> Option<usize> {
        if typed.contains(name) {
            return None;
        }
        let index = arguments.iter().position(|argument| {
            argument.name == *name
                && argument.ty.kind == DigitalRoutineValueKind::Reg
                && match reg_range {
                    Some(range) => same_range(argument.ty.range.as_ref(), range),
                    None => argument.ty.range.is_none() && !argument.ty.signedness.is_signed(),
                }
        })?;
        typed.insert(name.clone());
        Some(index)
    };
    for declaration in variables.iter_mut() {
        let kind = match declaration.var_type {
            VarType::Real => DigitalRoutineValueKind::Real,
            VarType::Integer => DigitalRoutineValueKind::Integer,
            VarType::String => continue,
        };
        declaration.items.retain(|item| {
            if !item.dimensions.is_empty() {
                return true;
            }
            let Some(index) = untyped(arguments, &item.name, None) else {
                return true;
            };
            arguments[index].ty = DigitalRoutineType {
                kind,
                signedness: Signedness::Signed,
                range: None,
            };
            false
        });
    }
    variables.retain(|declaration| !declaration.items.is_empty());
    for declaration in digital_variables.iter_mut() {
        let kind = match declaration.kind {
            DigitalVariableKind::Reg => DigitalRoutineValueKind::Reg,
            DigitalVariableKind::Integer => DigitalRoutineValueKind::Integer,
            DigitalVariableKind::Real => DigitalRoutineValueKind::Real,
            _ => continue,
        };
        // `time` arrives as `reg [63:0]` whose range is the keyword itself
        // (see `time_range`); it types like `integer`, not like `reg`.
        let time = declaration
            .range
            .as_ref()
            .is_some_and(|range| range.span == range.msb.span());
        let reg_range = (kind == DigitalRoutineValueKind::Reg && !time)
            .then_some(declaration.range.as_ref());
        declaration.items.retain(|item| {
            if !item.dimensions.is_empty() || item.init.is_some() {
                return true;
            }
            let Some(index) = untyped(arguments, &item.name, reg_range) else {
                return true;
            };
            arguments[index].ty = DigitalRoutineType {
                kind: if time {
                    DigitalRoutineValueKind::Time
                } else {
                    kind
                },
                signedness: declaration.signedness,
                range: declaration.range.clone(),
            };
            false
        });
    }
    digital_variables.retain(|declaration| !declaration.items.is_empty());
}

/// Whether two declared ranges are the same, as far as their literal bounds
/// show; a bound written as an expression is taken as the same.
fn same_range(port: Option<&VectorRange>, redeclared: Option<&VectorRange>) -> bool {
    let literal = |expression: &Expression| match expression {
        Expression::Number(number) => Some(number.value),
        _ => None,
    };
    match (port, redeclared) {
        (None, None) => true,
        (Some(port), Some(redeclared)) => [
            (&port.msb, &redeclared.msb),
            (&port.lsb, &redeclared.lsb),
        ]
        .into_iter()
        .all(|(a, b)| match (literal(a), literal(b)) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }),
        _ => false,
    }
}

/// `[63:0]`, the range a `time` variable is declared over.
fn time_range(span: Span) -> VectorRange {
    let number = |value: f64, raw: &str| {
        Expression::Number(NumberLit {
            value,
            raw: raw.into(),
            span,
        })
    };
    VectorRange {
        msb: number(63.0, "63"),
        lsb: number(0.0, "0"),
        span,
    }
}
