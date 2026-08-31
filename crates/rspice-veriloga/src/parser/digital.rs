//! Productions for the discrete (IEEE 1364-2005) half of Verilog-AMS.
//!
//! These are methods on the one [`Parser`], not a second parser: Verilog-AMS
//! is a superset of Verilog, so a digital process's expressions are the same
//! expressions an analog block writes, read by the same precedence climb. Only
//! the statement and declaration grammars differ, and only those live here.
//!
//! Everything this wave does not implement is refused **by name** at the
//! keyword that opens it, never skipped. A construct that parses is a
//! construct a later wave has to execute, so accepting one speculatively would
//! turn a compile error into a silently wrong device.

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use smol_str::SmolStr;

impl Parser<'_> {
    // ------------------------------------------------------------------
    // Shared declaration fragments
    // ------------------------------------------------------------------

    /// Read an optional `signed` / `unsigned` qualifier.
    ///
    /// Both words were usable as ordinary names before the digital grammar
    /// existed, and a Verilog-A source may still declare `real signed;`. One is
    /// read as a qualifier only where a declaration can continue past it, so
    /// `input signed;` still declares a port called `signed` while
    /// `input signed [7:0] bus;` declares a signed vector.
    pub(super) fn parse_signedness(&mut self) -> Signedness {
        let qualifier = match self.current().kind {
            TokenKind::Signed => Signedness::Signed,
            TokenKind::Unsigned => Signedness::Unsigned,
            _ => return Signedness::Unsigned,
        };
        if matches!(
            self.tokens.get(self.pos + 1).map(|token| token.kind),
            Some(TokenKind::Semicolon | TokenKind::Comma | TokenKind::Assign_)
        ) {
            return Signedness::Unsigned;
        }
        self.advance();
        qualifier
    }

    /// Read an optional packed range `[msb:lsb]`.
    pub(super) fn parse_optional_vector_range(
        &mut self,
    ) -> Result<Option<VectorRange>, ParseError> {
        if !self.check(TokenKind::LBracket) {
            return Ok(None);
        }
        let start = self.current_span();
        self.advance();
        let msb = self.parse_expression()?;
        // An indexed part-select opener here would be `+:` / `-:`, which this
        // wave does not implement; the `:` requirement refuses it by position.
        self.expect(TokenKind::Colon)?;
        let lsb = self.parse_expression()?;
        self.expect(TokenKind::RBracket)?;
        Ok(Some(VectorRange {
            msb,
            lsb,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Read the comma-separated names of a net or variable declaration,
    /// each with optional unpacked dimensions and an optional initializer.
    fn parse_digital_decl_items(&mut self) -> Result<Vec<DigitalDeclItem>, ParseError> {
        let mut items = Vec::new();
        loop {
            let item_start = self.current_span();
            let name: SmolStr = self.expect_identifier("declared name")?.into();

            let mut dimensions = Vec::new();
            while self.match_token(TokenKind::LBracket) {
                let dimension_start = self.previous_span();
                let low = self.parse_expression()?;
                self.expect(TokenKind::Colon)?;
                let high = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                dimensions.push(ArrayDimension {
                    start: low,
                    end: high,
                    span: dimension_start.extend(self.previous_span()),
                });
            }

            let init = if self.match_token(TokenKind::Assign_) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            items.push(DigitalDeclItem {
                name,
                dimensions,
                init,
                span: item_start.extend(self.previous_span()),
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(items)
    }

    /// `wire [signed] [range] name [= expr] [, ...] ;`
    pub(super) fn parse_digital_net_decl(&mut self) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Wire)?;
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        let items = self.parse_digital_decl_items()?;
        Ok(DigitalNetDecl {
            kind: DigitalNetKind::Wire,
            signedness,
            range,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// `reg [signed] [range] name [unpacked dims] [, ...] ;`
    pub(super) fn parse_digital_variable_decl(
        &mut self,
    ) -> Result<DigitalVariableDecl, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Reg)?;
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        let items = self.parse_digital_decl_items()?;
        Ok(DigitalVariableDecl {
            kind: DigitalVariableKind::Reg,
            signedness,
            range,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// `assign [#delay] lvalue = expression;`
    pub(super) fn parse_continuous_assign(&mut self) -> Result<ContinuousAssign, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Assign)?;
        let delay = if self.match_token(TokenKind::Hash) {
            Some(self.parse_delay_value()?)
        } else {
            None
        };
        let target = self.parse_digital_lvalue()?;
        // IEEE 1364-2005 section 6.1: a continuous assignment is a driver on a
        // net and uses `=`. `<=` is the procedural nonblocking assignment and
        // exists only inside a process, so say that rather than reporting a
        // token mismatch.
        if self.check(TokenKind::Le) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "continuous assignment".to_string(),
                    found: "`<=`; a nonblocking assignment is only legal inside \
                            an `always` or `initial` process, and a continuous \
                            `assign` uses `=`"
                        .to_string(),
                },
                self.current_span(),
            ));
        }
        self.expect(TokenKind::Assign_)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(ContinuousAssign {
            target,
            value,
            delay,
            span: start.extend(self.previous_span()),
        })
    }

    // ------------------------------------------------------------------
    // Processes
    // ------------------------------------------------------------------

    /// `always statement` / `initial statement`
    pub(super) fn parse_digital_process(
        &mut self,
        kind: DigitalProcessKind,
    ) -> Result<DigitalProcess, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `always` / `initial`
        let id = DigitalProcessId(self.next_process_id);
        self.next_process_id += 1;
        let body = self.parse_digital_statement()?;
        Ok(DigitalProcess {
            id,
            kind,
            body,
            span: start.extend(self.previous_span()),
        })
    }

    // ------------------------------------------------------------------
    // Statements
    // ------------------------------------------------------------------

    /// One procedural statement.
    pub(super) fn parse_digital_statement(&mut self) -> Result<DigitalStatement, ParseError> {
        // Attribute instances may decorate any statement; they carry no
        // meaning for this compiler and are consumed, as in an analog block.
        if self.check(TokenKind::LParen) && self.peek_is(TokenKind::Star) {
            self.parse_attributes()?;
        }

        let start = self.current_span();
        match self.current().kind {
            TokenKind::Begin => self.parse_digital_block(),
            TokenKind::If => self.parse_digital_if(),
            TokenKind::Case => self.parse_digital_case(CaseKind::Exact),
            TokenKind::Casez => self.parse_digital_case(CaseKind::WildcardZ),
            TokenKind::Casex => self.parse_digital_case(CaseKind::WildcardXZ),
            TokenKind::For => self.parse_digital_for(),
            TokenKind::While => self.parse_digital_while(),
            TokenKind::Repeat => self.parse_digital_repeat(),
            TokenKind::Forever => self.parse_digital_forever(),
            TokenKind::At | TokenKind::Hash => self.parse_digital_timing_statement(),
            TokenKind::Semicolon => {
                self.advance();
                Ok(DigitalStatement::Null(start))
            }
            TokenKind::Identifier | TokenKind::EscapedIdentifier | TokenKind::LBrace => {
                self.parse_digital_assignment()
            }
            // Every procedural statement outside this wave's scope — system
            // tasks, `fork`/`join`, `wait`, `disable`, `force`, `assign` as a
            // procedural statement — stops here on its own keyword.
            _ => Err(self.unsupported_digital_statement()),
        }
    }

    /// `begin [: name] declarations statements end`
    fn parse_digital_block(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `begin`

        let name = if self.match_token(TokenKind::Colon) {
            Some(self.expect_identifier("block name")?.into())
        } else {
            None
        };

        let mut variables = Vec::new();
        let mut digital_variables = Vec::new();
        let mut statements = Vec::new();
        while !self.check(TokenKind::End) && !self.at_end() {
            self.parse_attributes()?;
            match self.current().kind {
                TokenKind::Real | TokenKind::Integer | TokenKind::String => {
                    variables.push(self.parse_variable_decl()?);
                }
                TokenKind::Reg => {
                    digital_variables.push(self.parse_digital_variable_decl()?);
                }
                _ => statements.push(self.parse_digital_statement()?),
            }
        }

        self.expect(TokenKind::End)?;
        Ok(DigitalStatement::Block(DigitalBlock {
            name,
            variables,
            digital_variables,
            statements,
            span: start.extend(self.previous_span()),
        }))
    }

    fn parse_digital_if(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `if`
        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let then_branch = Box::new(self.parse_digital_statement()?);
        let else_branch = if self.match_token(TokenKind::Else) {
            Some(Box::new(self.parse_digital_statement()?))
        } else {
            None
        };
        Ok(DigitalStatement::Conditional(DigitalConditional {
            condition,
            then_branch,
            else_branch,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `case (selector) [labels: stmt]* [default[:] stmt] endcase`
    fn parse_digital_case(&mut self, kind: CaseKind) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `case` / `casez` / `casex`
        self.expect(TokenKind::LParen)?;
        let selector = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let mut items = Vec::new();
        let mut default: Option<Box<DigitalStatement>> = None;
        while !self.check(TokenKind::Endcase) && !self.at_end() {
            if self.check(TokenKind::Default) {
                let default_span = self.current_span();
                self.advance();
                // The colon after `default` is optional (IEEE 1364-2005
                // section 9.5), as it already is for an analog case.
                self.match_token(TokenKind::Colon);
                if default.is_some() {
                    return Err(ParseError::new(
                        ParseErrorKind::UnsupportedConstruct {
                            context: "case statement".to_string(),
                            found: "a second `default` arm; IEEE 1364-2005 \
                                    section 9.5 permits at most one"
                                .to_string(),
                        },
                        default_span,
                    ));
                }
                default = Some(Box::new(self.parse_digital_statement()?));
                continue;
            }

            let item_start = self.current_span();
            let mut labels = Vec::new();
            loop {
                labels.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Colon)?;
            let statement = Box::new(self.parse_digital_statement()?);
            items.push(DigitalCaseItem {
                labels,
                statement,
                span: item_start.extend(self.previous_span()),
            });
        }

        self.expect(TokenKind::Endcase)?;
        Ok(DigitalStatement::Case(DigitalCase {
            kind,
            selector,
            items,
            default,
            span: start.extend(self.previous_span()),
        }))
    }

    fn parse_digital_for(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `for`
        self.expect(TokenKind::LParen)?;
        let init = Box::new(self.parse_digital_assignment_expression()?);
        self.expect(TokenKind::Semicolon)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        let update = Box::new(self.parse_digital_assignment_expression()?);
        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_digital_statement()?);
        Ok(DigitalStatement::For(DigitalFor {
            init,
            condition,
            update,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    fn parse_digital_while(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `while`
        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_digital_statement()?);
        Ok(DigitalStatement::While(DigitalWhile {
            condition,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    fn parse_digital_repeat(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `repeat`
        self.expect(TokenKind::LParen)?;
        let count = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_digital_statement()?);
        Ok(DigitalStatement::Repeat(DigitalRepeat {
            count,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    fn parse_digital_forever(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume `forever`
        let body = Box::new(self.parse_digital_statement()?);
        Ok(DigitalStatement::Forever(DigitalForever {
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `@(...) [statement]` or `#delay [statement]`
    fn parse_digital_timing_statement(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        let control = self.parse_timing_control()?;
        // A bare `@(posedge clk);` suspends and does nothing, which is a legal
        // and useful statement; the null statement is retained as `None` so a
        // later pass does not have to distinguish it from `begin end`.
        let statement = if self.match_token(TokenKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_digital_statement()?))
        };
        Ok(DigitalStatement::Timing(DigitalTiming {
            control,
            statement,
            span: start.extend(self.previous_span()),
        }))
    }

    /// `@(...)`, `@*`, `@(*)`, or `#delay`.
    fn parse_timing_control(&mut self) -> Result<TimingControl, ParseError> {
        if self.match_token(TokenKind::Hash) {
            let start = self.previous_span();
            let value = self.parse_delay_value()?;
            return Ok(TimingControl::Delay(DelayControl {
                value,
                span: start.extend(self.previous_span()),
            }));
        }
        let start = self.current_span();
        self.expect(TokenKind::At)?;

        // `@*` (IEEE 1364-2005 section 9.7.5). The parenthesized spelling
        // `@(*)` is handled below; neither can be confused with an attribute
        // instance, because an attribute requires a name after `(*`.
        if self.match_token(TokenKind::Star) {
            return Ok(TimingControl::Event(EventControl {
                sensitivity: Sensitivity::Implicit,
                span: start.extend(self.previous_span()),
            }));
        }

        self.expect(TokenKind::LParen)?;
        if self.check(TokenKind::Star) && self.peek_is(TokenKind::RParen) {
            self.advance();
            self.advance();
            return Ok(TimingControl::Event(EventControl {
                sensitivity: Sensitivity::Implicit,
                span: start.extend(self.previous_span()),
            }));
        }
        if self.check(TokenKind::RParen) {
            return Err(ParseError::new(
                ParseErrorKind::InvalidEventExpression(
                    "an event control needs at least one term; write `@*` for \
                     an implicit sensitivity list"
                        .to_string(),
                ),
                start.extend(self.current_span()),
            ));
        }

        let mut terms = Vec::new();
        loop {
            terms.push(self.parse_event_term()?);
            // IEEE 1364-2005 section 9.7.4 makes `,` and `or` synonyms in an
            // event expression. `or` is not a reserved word, so it arrives as
            // an identifier.
            if self.match_token(TokenKind::Comma) {
                continue;
            }
            if self.check(TokenKind::Identifier) && self.current().text.as_deref() == Some("or") {
                self.advance();
                continue;
            }
            break;
        }
        self.expect(TokenKind::RParen)?;

        Ok(TimingControl::Event(EventControl {
            sensitivity: Sensitivity::Explicit(terms),
            span: start.extend(self.previous_span()),
        }))
    }

    /// `[posedge|negedge] expression`
    fn parse_event_term(&mut self) -> Result<EventTerm, ParseError> {
        let start = self.current_span();
        let edge = match self.current().kind {
            TokenKind::Posedge => {
                self.advance();
                Some(EdgeKind::Posedge)
            }
            TokenKind::Negedge => {
                self.advance();
                Some(EdgeKind::Negedge)
            }
            _ => None,
        };
        let signal = self.parse_expression()?;
        Ok(EventTerm {
            edge,
            signal,
            span: start.extend(self.previous_span()),
        })
    }

    /// The operand of `#`. A parenthesized min:typ:max delay is not part of
    /// this wave and is refused by name rather than read as its typical value.
    fn parse_delay_value(&mut self) -> Result<Expression, ParseError> {
        if self.check(TokenKind::LParen) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "delay control".to_string(),
                    found: "a parenthesized min:typ:max delay; write a single \
                            delay expression instead"
                        .to_string(),
                },
                self.current_span(),
            ));
        }
        self.parse_primary()
    }

    // ------------------------------------------------------------------
    // Assignments
    // ------------------------------------------------------------------

    /// `lvalue = expr;` or `lvalue <= expr;`
    fn parse_digital_assignment(&mut self) -> Result<DigitalStatement, ParseError> {
        let assignment_start = self.current_span();
        let target = self.parse_digital_lvalue()?;
        let nonblocking = if self.match_token(TokenKind::Le) {
            true
        } else {
            self.expect(TokenKind::Assign_)?;
            false
        };
        // Intra-assignment timing control (IEEE 1364-2005 section 9.2.2).
        let timing = if self.check(TokenKind::At) || self.check(TokenKind::Hash) {
            Some(self.parse_timing_control()?)
        } else {
            None
        };
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        let assign = DigitalAssign {
            target,
            timing,
            value,
            span: assignment_start.extend(self.previous_span()),
        };
        Ok(if nonblocking {
            DigitalStatement::NonblockingAssign(assign)
        } else {
            DigitalStatement::BlockingAssign(assign)
        })
    }

    /// The `lvalue = expr` form used by a `for` header, which carries no
    /// terminating semicolon and no timing control.
    fn parse_digital_assignment_expression(&mut self) -> Result<DigitalAssign, ParseError> {
        let start = self.current_span();
        let target = self.parse_digital_lvalue()?;
        self.expect(TokenKind::Assign_)?;
        let value = self.parse_expression()?;
        Ok(DigitalAssign {
            target,
            timing: None,
            value,
            span: start.extend(self.previous_span()),
        })
    }

    /// An assignment target: a name, a bit-select, a part-select, or a
    /// concatenation of those.
    pub(super) fn parse_digital_lvalue(&mut self) -> Result<DigitalLValue, ParseError> {
        let start = self.current_span();
        if self.match_token(TokenKind::LBrace) {
            let mut elements = Vec::new();
            loop {
                elements.push(self.parse_digital_lvalue()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RBrace)?;
            return Ok(DigitalLValue::Concat {
                elements,
                span: start.extend(self.previous_span()),
            });
        }

        let name: SmolStr = self.expect_identifier("assignment target")?.into();
        if !self.match_token(TokenKind::LBracket) {
            return Ok(DigitalLValue::Identifier {
                name,
                span: start.extend(self.previous_span()),
            });
        }

        let first = self.parse_expression()?;
        if self.match_token(TokenKind::Colon) {
            let lsb = self.parse_expression()?;
            self.expect(TokenKind::RBracket)?;
            return Ok(DigitalLValue::PartSelect {
                name,
                msb: Box::new(first),
                lsb: Box::new(lsb),
                span: start.extend(self.previous_span()),
            });
        }
        self.expect(TokenKind::RBracket)?;
        Ok(DigitalLValue::BitSelect {
            name,
            index: Box::new(first),
            span: start.extend(self.previous_span()),
        })
    }

    /// Refuse the procedural statement under the cursor by name.
    ///
    /// Its body is left unconsumed, exactly as the module-item refusal does:
    /// skipping it would only move the reported position away from the
    /// construct the author has to remove.
    fn unsupported_digital_statement(&self) -> ParseError {
        let keyword = self
            .current()
            .text
            .clone()
            .unwrap_or_else(|| format!("{:?}", self.current().kind).to_lowercase());
        ParseError::new(
            ParseErrorKind::UnsupportedAmsConstruct { keyword },
            self.current_span(),
        )
    }
}
