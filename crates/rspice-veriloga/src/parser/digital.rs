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

/// The gate primitives of IEEE 1364-2005 section 7.2 this compiler accepts.
///
/// The eight combinational ones. The tristate family (`bufif0`, `notif1`, ...),
/// the MOS switches, and the pull gates are absent, and a design naming one is
/// refused as an undefined module rather than silently becoming something else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GatePrimitive {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Buf,
    Not,
}

impl GatePrimitive {
    /// The gate a name in module-instance position denotes, if it denotes one.
    ///
    /// Recognized by name because every one of these is a reserved word of IEEE
    /// 1364-2005 annex B, so a module cannot legally be called one. This lexer
    /// does not reserve them — they were identifiers before there was a digital
    /// grammar — which is why the check is here rather than on a token kind.
    pub(super) fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "and" => Self::And,
            "nand" => Self::Nand,
            "or" => Self::Or,
            "nor" => Self::Nor,
            "xor" => Self::Xor,
            "xnor" => Self::Xnor,
            "buf" => Self::Buf,
            "not" => Self::Not,
            _ => return None,
        })
    }

    pub(super) const fn keyword(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Nand => "nand",
            Self::Or => "or",
            Self::Nor => "nor",
            Self::Xor => "xor",
            Self::Xnor => "xnor",
            Self::Buf => "buf",
            Self::Not => "not",
        }
    }

    /// Whether the gate drives its *last* terminal's value onto every other
    /// one, which is what section 7.4 makes `buf` and `not` do.
    const fn is_buffer(self) -> bool {
        matches!(self, Self::Buf | Self::Not)
    }

    /// The operator that combines two inputs, before any inversion.
    ///
    /// A buffer has one input and never reaches the fold, so its operator is
    /// never applied; naming AND here keeps the function total rather than
    /// adding an unreachable arm that a later reader has to check.
    const fn combining_op(self) -> BinaryOp {
        match self {
            Self::And | Self::Nand | Self::Buf | Self::Not => BinaryOp::BitAnd,
            Self::Or | Self::Nor => BinaryOp::BitOr,
            Self::Xor | Self::Xnor => BinaryOp::BitXor,
        }
    }

    /// Whether the combined value is inverted on the way out.
    const fn inverts(self) -> bool {
        matches!(self, Self::Nand | Self::Nor | Self::Xnor | Self::Not)
    }

    /// The fewest terminals the gate can be written with.
    ///
    /// Two either way: an n-input gate needs an output and at least one input
    /// to be meaningful, and a buffer needs at least one output and its input.
    const fn minimum_terminals(self) -> usize {
        2
    }
}

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
    ///
    /// and Verilog-AMS LRM 2.4 Syntax 3-8's real net,
    /// `wreal [discipline] [range] name [= expr] [, ...] ;`, which differs only
    /// in the keyword and in the discipline the standard lets one carry.
    pub(super) fn parse_digital_net_decl(&mut self) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        if self.check(TokenKind::Wreal) {
            return self.parse_wreal_net_decl();
        }
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

    /// `wreal [discipline_identifier] [range] name [= expr] [, ...] ;`
    ///
    /// Verilog-AMS LRM 2.4 Syntax 3-8, plus the four resolved net-type
    /// spellings [`WrealResolution`] documents. Which one was written is read
    /// back off the keyword token's text, because all five share this one
    /// production and splitting them would put it in five places.
    ///
    /// The `signed` qualifier is *not* in Syntax 3-8 and is not read here: a
    /// real has no sign bit to declare, and `wreal signed;` is therefore a net
    /// called `signed`, exactly as `real signed;` has always been a variable
    /// called `signed`.
    fn parse_wreal_net_decl(&mut self) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        let resolution = self.wreal_resolution();
        self.advance(); // consume the net-type keyword
        self.parse_wreal_discipline()?;
        let range = self.parse_optional_vector_range()?;
        let items = self.parse_digital_decl_items()?;
        Ok(DigitalNetDecl {
            kind: DigitalNetKind::Wreal(resolution),
            signedness: Signedness::Unsigned,
            range,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// The resolution the real-net keyword under the cursor names.
    ///
    /// The lexer gave all five spellings one kind and kept the text, so this
    /// reads the text back. A token that somehow carries none is the plain
    /// `wreal`, which is the reading that refuses a second driver rather than
    /// combining one.
    pub(super) fn wreal_resolution(&self) -> WrealResolution {
        self.current()
            .text
            .as_deref()
            .and_then(WrealResolution::from_keyword)
            .unwrap_or(WrealResolution::Single)
    }

    /// Read the optional discipline identifier of a `wreal` declaration.
    ///
    /// Verilog-AMS LRM 2.4 Syntax 3-8 permits one, and section 3.11's Discrete
    /// Domain Rule says which ones are compatible: a real net's discipline must
    /// have a discrete domain. `ddiscrete` is the standard discrete discipline
    /// and the only one this compiler implements; any other name is refused
    /// here rather than accepted and ignored, because a discipline that was
    /// read and dropped would silently place the net in a domain nothing
    /// afterwards honours.
    ///
    /// A discipline is distinguished from the net's own name the way the port
    /// grammar distinguishes them: two adjacent identifiers, or an identifier
    /// followed by a range. `wreal w;` declares `w`; `wreal ddiscrete w;`
    /// declares `w` in `ddiscrete`.
    fn parse_wreal_discipline(&mut self) -> Result<(), ParseError> {
        let followed_by_declaration = matches!(
            self.tokens.get(self.pos + 1).map(|token| token.kind),
            Some(TokenKind::Identifier | TokenKind::EscapedIdentifier | TokenKind::LBracket)
        );
        if !(self.check(TokenKind::Identifier) && followed_by_declaration) {
            return Ok(());
        }
        let span = self.current_span();
        let name = self.expect_identifier("discipline")?;
        if name != "ddiscrete" {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "`wreal` declaration".to_string(),
                    found: format!(
                        "the discipline `{name}`; Verilog-AMS LRM 2.4 section 3.11's Discrete \
                         Domain Rule makes a real net's discipline a discrete-domain one, and \
                         `ddiscrete` is the only one this compiler implements"
                    ),
                },
                span,
            ));
        }
        Ok(())
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
    // Gate primitives
    // ------------------------------------------------------------------

    /// `gate [#delay] [name] (terminals) [, [name] (terminals)]* ;`
    ///
    /// Each instance becomes one continuous assignment per driven output, which
    /// is what a gate primitive *is*: IEEE 1364-2005 section 7.2 defines the
    /// eight of them by the truth tables of section 4.1, and section 6.1's
    /// continuous assignment is a driver evaluating those same tables. Lowering
    /// them to the construct that already exists means a design cannot get a
    /// different answer from `nand g (y, a, b)` than from `assign y = ~(a & b)`
    /// — which is exactly the disagreement a front end with two paths has two
    /// chances to produce.
    ///
    /// The gate's own delay is carried onto the assignment rather than dropped;
    /// the lowering refuses it there, by name, for the reason it refuses one on
    /// an `assign`.
    pub(super) fn parse_gate_instantiation(
        &mut self,
        gate: GatePrimitive,
    ) -> Result<Vec<ContinuousAssign>, ParseError> {
        let start = self.current_span();
        self.advance(); // consume the gate keyword

        // A drive-strength specification `(strong1, weak0)` selects between
        // strengths this compiler's one-strength resolution cannot represent,
        // so it is refused rather than ignored: ignoring it would silently
        // resolve a contention the design meant to decide.
        if self.check(TokenKind::LParen) && self.strength_specification_follows() {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: format!("`{}` gate", gate.keyword()),
                    found: "a drive-strength specification; this compiler resolves nets at one \
                            strength, so a strength that changes the resolution cannot be \
                            honoured"
                        .to_string(),
                },
                self.current_span(),
            ));
        }

        let delay = if self.match_token(TokenKind::Hash) {
            Some(self.parse_delay_value()?)
        } else {
            None
        };

        let mut assignments = Vec::new();
        loop {
            // The instance name is optional (section 7.1), so a `(` here opens
            // the terminal list of an unnamed instance.
            if !self.check(TokenKind::LParen) {
                self.expect_identifier("gate instance name")?;
            }
            let instance_start = self.current_span();
            self.expect(TokenKind::LParen)?;
            let mut terminals = Vec::new();
            loop {
                terminals.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
            let span = instance_start.extend(self.previous_span());
            assignments.extend(self.gate_assignments(gate, terminals, delay.clone(), span)?);

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;

        let declaration = start.extend(self.previous_span());
        for assignment in &mut assignments {
            assignment.span = declaration;
        }
        Ok(assignments)
    }

    /// Whether the parenthesis under the cursor opens a drive-strength
    /// specification rather than a terminal list.
    ///
    /// A strength spec is two strength keywords separated by a comma, and every
    /// one of those words is an identifier to this lexer. Looking for the first
    /// one is enough: no terminal expression can begin with `supply0`.
    fn strength_specification_follows(&self) -> bool {
        const STRENGTHS: [&str; 8] = [
            "supply0", "supply1", "strong0", "strong1", "pull0", "pull1", "weak0", "weak1",
        ];
        self.tokens
            .get(self.pos + 1)
            .and_then(|token| token.text.as_deref())
            .is_some_and(|text| STRENGTHS.contains(&text))
    }

    /// Turn one gate instance's terminal list into its driving assignments.
    fn gate_assignments(
        &self,
        gate: GatePrimitive,
        terminals: Vec<Expression>,
        delay: Option<Expression>,
        span: crate::source::Span,
    ) -> Result<Vec<ContinuousAssign>, ParseError> {
        let minimum = gate.minimum_terminals();
        if terminals.len() < minimum {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: format!("`{}` gate", gate.keyword()),
                    found: format!(
                        "{} terminal(s); IEEE 1364-2005 section 7.2 requires at least {minimum}",
                        terminals.len()
                    ),
                },
                span,
            ));
        }

        // Sections 7.3 and 7.4 split the eight gates by where the input is: an
        // n-input gate drives its first terminal from the rest, while `buf` and
        // `not` drive every terminal but the last from that last one.
        let (outputs, inputs): (Vec<&Expression>, Vec<&Expression>) = if gate.is_buffer() {
            let (outputs, input) = terminals.split_at(terminals.len() - 1);
            (outputs.iter().collect(), input.iter().collect())
        } else {
            let (output, inputs) = terminals.split_at(1);
            (output.iter().collect(), inputs.iter().collect())
        };

        let mut value = inputs[0].clone();
        for input in &inputs[1..] {
            value = Expression::Binary(BinaryExpr {
                op: gate.combining_op(),
                left: Box::new(value),
                right: Box::new((*input).clone()),
                span,
            });
        }
        if gate.inverts() {
            value = Expression::Unary(UnaryExpr {
                op: UnaryOp::BitNot,
                operand: Box::new(value),
                span,
            });
        }

        outputs
            .into_iter()
            .map(|output| {
                Ok(ContinuousAssign {
                    target: Self::terminal_as_lvalue(output, gate)?,
                    value: value.clone(),
                    delay: delay.clone(),
                    span,
                })
            })
            .collect()
    }

    /// A gate's output terminal, as an assignment target.
    fn terminal_as_lvalue(
        terminal: &Expression,
        gate: GatePrimitive,
    ) -> Result<DigitalLValue, ParseError> {
        match terminal {
            Expression::Identifier(identifier) => Ok(DigitalLValue::Identifier {
                name: identifier.name.clone(),
                span: identifier.span,
            }),
            Expression::ArrayAccess(access) => Ok(DigitalLValue::BitSelect {
                name: access.array.clone(),
                index: access.index.clone(),
                span: access.span,
            }),
            other => Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: format!("`{}` gate output", gate.keyword()),
                    found: "an expression; a gate drives a net, so its output terminal must be \
                            a name or a bit select of one"
                        .to_string(),
                },
                other.span(),
            )),
        }
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
