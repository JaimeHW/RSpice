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
use crate::canonical_ir::digital::{DigitalDriveStrength, DigitalStrengthLevel};
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;

/// What follows `name[index]` when another bracket opens: a packed bit or
/// constant part of an array element, or an indexed part-select.
pub(super) enum PackedSelection {
    Array(ArraySelectExpr),
    Indexed(IndexedPartSelectExpr),
}

/// The operator an IEEE 1800-2017 section 11.4.1 compound assignment applies.
/// `>>>` is a [`DigitalExpr`] rather than a [`BinaryOp`], so it has its own arm.
#[derive(Debug, Clone, Copy)]
enum CompoundOperator {
    Binary(BinaryOp),
    ArithmeticShiftRight,
}

/// The gate and switch primitives of IEEE 1364-2005 section 7 this compiler
/// accepts.
///
/// The eight combinational gates, the three-state gates, the MOS switches,
/// the pull gates and the bidirectional switches: every primitive of the
/// section.
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
    Bufif0,
    Bufif1,
    Notif0,
    Notif1,
    Nmos,
    Pmos,
    Rnmos,
    Rpmos,
    Cmos,
    Rcmos,
    Pullup,
    Pulldown,
    Tran,
    Rtran,
    Tranif0,
    Tranif1,
    Rtranif0,
    Rtranif1,
}

/// How a primitive's terminals become drivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateFamily {
    /// The eight gates of sections 7.3 and 7.4.
    Logic,
    /// Section 7.5: `(out, data, control)`.
    Tristate { invert: bool, active_low: bool },
    /// Section 7.6: `(out, data, control)`.
    Mos { active_low: bool, resistive: bool },
    /// Section 7.7: `(out, data, ncontrol, pcontrol)`, an `nmos` and a `pmos`
    /// sharing their data and output.
    Cmos { resistive: bool },
    /// Section 7.8: `(out)`, driving the constant at pull strength.
    Pull { one: bool },
    /// Section 7.6: `(inout, inout)` for `tran`, and `(inout, inout,
    /// control)` for `tranif0`/`tranif1`, whose control is active low for the
    /// `0` form. Each terminal is driven from the other, which the store
    /// resolves as one switch island rather than as two drivers.
    Bidirectional {
        control: Option<bool>,
        resistive: bool,
    },
}

impl GatePrimitive {
    /// The gate a name in module-instance position denotes, if it denotes one.
    ///
    /// Recognized by name because every one of these is a reserved word of IEEE
    /// 1364-2005 annex B, so a module cannot legally be called one. This lexer
    /// does not reserve them — they were identifiers before there was a digital
    /// grammar — which is why the check is here rather than on a token kind.
    pub(super) fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|primitive| primitive.keyword() == name)
    }

    const ALL: [Self; 26] = [
        Self::And,
        Self::Nand,
        Self::Or,
        Self::Nor,
        Self::Xor,
        Self::Xnor,
        Self::Buf,
        Self::Not,
        Self::Bufif0,
        Self::Bufif1,
        Self::Notif0,
        Self::Notif1,
        Self::Nmos,
        Self::Pmos,
        Self::Rnmos,
        Self::Rpmos,
        Self::Cmos,
        Self::Rcmos,
        Self::Pullup,
        Self::Pulldown,
        Self::Tran,
        Self::Rtran,
        Self::Tranif0,
        Self::Tranif1,
        Self::Rtranif0,
        Self::Rtranif1,
    ];

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
            Self::Bufif0 => "bufif0",
            Self::Bufif1 => "bufif1",
            Self::Notif0 => "notif0",
            Self::Notif1 => "notif1",
            Self::Nmos => "nmos",
            Self::Pmos => "pmos",
            Self::Rnmos => "rnmos",
            Self::Rpmos => "rpmos",
            Self::Cmos => "cmos",
            Self::Rcmos => "rcmos",
            Self::Pullup => "pullup",
            Self::Pulldown => "pulldown",
            Self::Tran => "tran",
            Self::Rtran => "rtran",
            Self::Tranif0 => "tranif0",
            Self::Tranif1 => "tranif1",
            Self::Rtranif0 => "rtranif0",
            Self::Rtranif1 => "rtranif1",
        }
    }

    const fn family(self) -> GateFamily {
        use GateFamily::{Bidirectional, Cmos, Logic, Mos, Pull, Tristate};
        match self {
            Self::And
            | Self::Nand
            | Self::Or
            | Self::Nor
            | Self::Xor
            | Self::Xnor
            | Self::Buf
            | Self::Not => Logic,
            Self::Bufif0 => Tristate {
                invert: false,
                active_low: true,
            },
            Self::Bufif1 => Tristate {
                invert: false,
                active_low: false,
            },
            Self::Notif0 => Tristate {
                invert: true,
                active_low: true,
            },
            Self::Notif1 => Tristate {
                invert: true,
                active_low: false,
            },
            Self::Nmos => Mos {
                active_low: false,
                resistive: false,
            },
            Self::Pmos => Mos {
                active_low: true,
                resistive: false,
            },
            Self::Rnmos => Mos {
                active_low: false,
                resistive: true,
            },
            Self::Rpmos => Mos {
                active_low: true,
                resistive: true,
            },
            Self::Cmos => Cmos { resistive: false },
            Self::Rcmos => Cmos { resistive: true },
            Self::Pullup => Pull { one: true },
            Self::Pulldown => Pull { one: false },
            Self::Tran | Self::Rtran => Bidirectional {
                control: None,
                resistive: matches!(self, Self::Rtran),
            },
            Self::Tranif0 | Self::Rtranif0 => Bidirectional {
                control: Some(true),
                resistive: matches!(self, Self::Rtranif0),
            },
            Self::Tranif1 | Self::Rtranif1 => Bidirectional {
                control: Some(false),
                resistive: matches!(self, Self::Rtranif1),
            },
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
    /// never applied; naming AND for every gate outside the eight keeps the
    /// function total rather than adding an unreachable arm.
    const fn combining_op(self) -> BinaryOp {
        match self {
            Self::Or | Self::Nor => BinaryOp::BitOr,
            Self::Xor | Self::Xnor => BinaryOp::BitXor,
            _ => BinaryOp::BitAnd,
        }
    }

    /// Whether the combined value is inverted on the way out.
    const fn inverts(self) -> bool {
        matches!(self, Self::Nand | Self::Nor | Self::Xnor | Self::Not)
    }

    /// The strength a primitive drives at when the source names none: `pull`
    /// for a pull gate (section 7.8), and strong for every other gate.
    const fn implicit_strength(self) -> Option<DigitalDriveStrength> {
        match self.family() {
            GateFamily::Pull { .. } => Some(DigitalDriveStrength {
                zero: DigitalStrengthLevel::Pull,
                one: DigitalStrengthLevel::Pull,
            }),
            _ => None,
        }
    }

    /// The terminal counts the primitive can be written with: at least two for
    /// the eight logic gates (an output and an input), exactly the section's
    /// count for the switches and three-state gates, and at least one for a
    /// pull gate, which pulls every terminal it names as Icarus Verilog's
    /// does (A.3.1 writes one).
    const fn terminal_counts(self) -> (usize, Option<usize>) {
        match self.family() {
            GateFamily::Logic => (2, None),
            GateFamily::Tristate { .. } | GateFamily::Mos { .. } => (3, Some(3)),
            GateFamily::Cmos { .. } => (4, Some(4)),
            GateFamily::Pull { .. } => (1, None),
            GateFamily::Bidirectional { control: None, .. } => (2, Some(2)),
            GateFamily::Bidirectional { .. } => (3, Some(3)),
        }
    }
}

/// A drive-strength keyword (IEEE 1364-2005 A.2.2.2): whether it is a `1`
/// strength, and its level.
pub(super) fn strength_level(text: &str) -> Option<(bool, DigitalStrengthLevel)> {
    use DigitalStrengthLevel::{HighZ, Pull, Strong, Supply, Weak};
    Some(match text {
        "supply0" => (false, Supply),
        "strong0" => (false, Strong),
        "pull0" => (false, Pull),
        "weak0" => (false, Weak),
        "highz0" => (false, HighZ),
        "supply1" => (true, Supply),
        "strong1" => (true, Strong),
        "pull1" => (true, Pull),
        "weak1" => (true, Weak),
        "highz1" => (true, HighZ),
        _ => return None,
    })
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
        // A declared range is `[msb:lsb]`; `+:` / `-:` select, they declare nothing.
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
    pub(super) fn parse_digital_decl_items(&mut self) -> Result<Vec<DigitalDeclItem>, ParseError> {
        let mut items = Vec::new();
        if self.check(TokenKind::Hash) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "net declaration".to_string(),
                    found: "a net delay (IEEE 1364-2005 section 6.1.3)".to_string(),
                },
                self.current_span(),
            ));
        }
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

    /// `wire [discipline] [signed] [range] name [= expr] [, ...] ;`
    ///
    /// and Verilog-AMS LRM 2.4 Syntax 3-8's real net,
    /// `wreal [discipline] [range] name [= expr] [, ...] ;`, which differs only
    /// in the keyword and the real-valued signal type.
    pub(super) fn parse_digital_net_decl(
        &mut self,
        module: &mut Module,
    ) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        if self.check(TokenKind::Wreal) {
            return self.parse_wreal_net_decl(module);
        }
        // IEEE 1364-2005 section 4.6.3: `trireg` stays a reserved digital
        // keyword, so it is read here rather than as a `wire` spelling.
        let trireg = self.digital_keyword_is("trireg");
        let mut kind = self
            .current()
            .text
            .as_deref()
            .and_then(DigitalNetKind::from_keyword)
            .unwrap_or(DigitalNetKind::Wire);
        if trireg {
            self.advance();
        } else {
            self.expect(TokenKind::Wire)?;
        }
        // `trireg (large) t;` (section 4.4.1): the charge strength.
        let charge = if trireg && self.check(TokenKind::LParen) {
            self.parse_charge_strength()?
        } else {
            None
        };
        // `wire (weak1, weak0) w = a;` (section 6.1.4): the strength of the
        // declaration assignment.
        let strength = if charge.is_none()
            && self.check(TokenKind::LParen)
            && self.strength_specification_follows()
        {
            Some(self.parse_drive_strength("net declaration")?)
        } else {
            None
        };
        // `wire real r;` is a SystemVerilog real-valued net (IEEE 1800-2017
        // section 6.7.1), which Verilog-AMS spells `wreal`.
        if !trireg && self.check(TokenKind::Real) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "net data type".to_string(),
                    found: "`wire real`; declare a real-valued net as `wreal` (Verilog-AMS \
                            LRM 2.4 Syntax 3-8)"
                        .to_string(),
                },
                self.current_span(),
            ));
        }
        let discipline = self.parse_discrete_discipline(true)?;
        // IEEE 1364-2005 section 4.3.2: `vectored` and `scalared` say whether
        // a tool may expand a vector net into bits. Neither changes what the
        // net holds or how it resolves, so the keyword is read past.
        let expansion = (self.digital_keyword_is("vectored")
            || self.digital_keyword_is("scalared"))
        .then(|| {
            let keyword = self.current().text.clone().unwrap_or_default();
            self.advance();
            keyword
        });
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        if let Some(keyword) = expansion
            && range.is_none()
        {
            return Err(self.error(ParseErrorKind::UnexpectedToken(format!(
                "`{keyword}` declares a vector net, so a range must follow it (IEEE 1364-2005 \
                 section 4.3.2)"
            ))));
        }
        // `wire #5 w;` (section 6.1.3): a net delay, or for a declaration
        // assignment, the delay of its driver.
        let delay = if trireg {
            let (delay, decay) = if self.check(TokenKind::Hash) {
                self.parse_trireg_delay()?
            } else {
                (None, None)
            };
            kind = DigitalNetKind::Trireg {
                charge: charge.unwrap_or(DigitalStrengthLevel::Medium),
                decay,
            };
            delay
        } else if self.match_token(TokenKind::Hash) {
            Some(self.parse_delay3(3, "net declaration")?)
        } else {
            None
        };
        let mut items = self.parse_digital_decl_items()?;
        Self::retain_discrete_discipline(
            module,
            discipline,
            &range,
            &items,
            start.extend(self.previous_span()),
        );
        // A strength or a delay belongs to the declaration assignment, which
        // section 6.1.2 makes a continuous assignment; it becomes one here so
        // that it carries them. A net declared without one has a net delay.
        if strength.is_some() || delay.is_some() {
            for item in &mut items {
                let Some(value) = item.init.take() else {
                    if let Some(delay) = &delay {
                        module.net_delays.push(NetDelay {
                            name: item.name.clone(),
                            delay: delay.clone(),
                        });
                    }
                    continue;
                };
                module.continuous_assigns.push(ContinuousAssign {
                    target: DigitalLValue::Identifier {
                        name: item.name.clone(),
                        span: item.span,
                    },
                    value,
                    delay: delay.clone(),
                    strength,
                    gate: None,
                    span: item.span,
                });
            }
        }
        Ok(DigitalNetDecl {
            kind,
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
    fn parse_wreal_net_decl(&mut self, module: &mut Module) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        let resolution = self.wreal_resolution();
        self.advance(); // consume the net-type keyword
        let discipline = self.parse_discrete_discipline(false)?;
        let range = self.parse_optional_vector_range()?;
        let items = self.parse_digital_decl_items()?;
        Self::retain_discrete_discipline(
            module,
            discipline,
            &range,
            &items,
            start.extend(self.previous_span()),
        );
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

    /// Distinguish an optional discipline from the declared net name. A range
    /// after a name can instead be an unpacked dimension, so look through it
    /// and require a following declaration name before consuming a discipline.
    pub(super) fn parse_discrete_discipline(
        &mut self,
        signedness_follows: bool,
    ) -> Result<Option<SmolStr>, ParseError> {
        if signedness_follows
            && matches!(self.current().kind, TokenKind::Signed | TokenKind::Unsigned)
        {
            return Ok(None);
        }
        let mut next = self.pos + 1;
        if self
            .tokens
            .get(next)
            .is_some_and(|token| token.kind == TokenKind::LBracket)
        {
            let mut depth = 0usize;
            for token in &self.tokens[next..] {
                next += 1;
                match token.kind {
                    TokenKind::LBracket => depth += 1,
                    TokenKind::RBracket => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        let next_kind = self.tokens.get(next).map(|token| token.kind);
        let followed_by_declaration = matches!(
            next_kind,
            Some(TokenKind::Identifier | TokenKind::EscapedIdentifier)
        ) || (signedness_follows
            && matches!(next_kind, Some(TokenKind::Signed | TokenKind::Unsigned)));
        if !((matches!(
            self.current().kind,
            TokenKind::Identifier | TokenKind::EscapedIdentifier
        ) || self.is_discipline_keyword())
            && followed_by_declaration)
        {
            return Ok(None);
        }
        Ok(Some(self.expect_identifier("discipline")?.into()))
    }

    /// Inline and separate discipline declarations share the same AST contract.
    /// Semantic analysis owns name lookup, domain checks, and conflict detection.
    fn retain_discrete_discipline(
        module: &mut Module,
        discipline: Option<SmolStr>,
        range: &Option<VectorRange>,
        items: &[DigitalDeclItem],
        span: Span,
    ) {
        if let Some(discipline) = discipline {
            module.nets.push(NetDecl {
                dimensions: items
                    .iter()
                    .filter(|item| !item.dimensions.is_empty())
                    .map(|item| (item.name.clone(), item.dimensions.clone()))
                    .collect(),
                discipline: Some(discipline),
                range: range.clone(),
                names: items.iter().map(|item| item.name.clone()).collect(),
                is_ground: false,
                is_internal: true,
                attributes: Vec::new(),
                span,
            });
        }
    }

    /// `reg [signed] [range] name [unpacked dims] [, ...] ;`
    pub(super) fn parse_digital_variable_decl(
        &mut self,
        module: Option<&mut Module>,
    ) -> Result<DigitalVariableDecl, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Reg)?;
        let discipline = if module.is_some() {
            self.parse_discrete_discipline(true)?
        } else {
            None
        };
        let signedness = self.parse_signedness();
        let range = self.parse_optional_vector_range()?;
        let items = self.parse_digital_decl_items()?;
        if let Some(module) = module {
            Self::retain_discrete_discipline(
                module,
                discipline,
                &range,
                &items,
                start.extend(self.previous_span()),
            );
        }
        Ok(DigitalVariableDecl {
            kind: DigitalVariableKind::Reg,
            signedness,
            range,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// `assign [strength] [#delay] lvalue = expression [, lvalue = expression]* ;`
    ///
    /// IEEE 1364-2005 A.6.1: one strength and delay apply to every assignment
    /// of the list.
    pub(super) fn parse_continuous_assign(&mut self) -> Result<Vec<ContinuousAssign>, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Assign)?;
        // `assign (strong1, weak0) y = a;` (section 6.1.4).
        let strength = if self.check(TokenKind::LParen) && self.strength_specification_follows() {
            Some(self.parse_drive_strength("continuous assignment")?)
        } else {
            None
        };
        let delay = if self.match_token(TokenKind::Hash) {
            Some(self.parse_delay3(3, "continuous assignment")?)
        } else {
            None
        };
        let mut assignments = Vec::new();
        loop {
            let target = self.parse_digital_lvalue()?;
            // IEEE 1364-2005 section 6.1: a continuous assignment is a driver
            // on a net and uses `=`. `<=` is the procedural nonblocking
            // assignment and exists only inside a process, so say that rather
            // than reporting a token mismatch.
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
            assignments.push(ContinuousAssign {
                target,
                value,
                delay: delay.clone(),
                strength,
                gate: None,
                span: start,
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        let span = start.extend(self.previous_span());
        for assignment in &mut assignments {
            assignment.span = span;
        }
        Ok(assignments)
    }

    /// `( strength0 , strength1 )` in either order (IEEE 1364-2005 A.2.2.2).
    ///
    /// Section 6.1.4 forbids `(highz0, highz1)` and `(highz1, highz0)`, and a
    /// pair must name one strength for each value.
    pub(super) fn parse_drive_strength(
        &mut self,
        context: &str,
    ) -> Result<DigitalDriveStrength, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::LParen)?;
        let first = self.strength_keyword(context)?;
        self.expect(TokenKind::Comma)?;
        let second = self.strength_keyword(context)?;
        self.expect(TokenKind::RParen)?;
        let span = start.extend(self.previous_span());
        let (zero, one) = match (first, second) {
            ((false, zero), (true, one)) | ((true, one), (false, zero)) => (zero, one),
            _ => {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: context.to_string(),
                        found: "a drive strength that does not name one strength for 0 and one \
                                for 1 (IEEE 1364-2005 section 6.1.4)"
                            .to_string(),
                    },
                    span,
                ));
            }
        };
        if zero == DigitalStrengthLevel::HighZ && one == DigitalStrengthLevel::HighZ {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: context.to_string(),
                    found: "`(highz0, highz1)`; IEEE 1364-2005 section 6.1.4 makes a driver \
                            that drives neither value illegal"
                        .to_string(),
                },
                span,
            ));
        }
        Ok(DigitalDriveStrength { zero, one })
    }

    /// One strength keyword: whether it is a `1` strength, and its level.
    pub(super) fn strength_keyword(
        &mut self,
        context: &str,
    ) -> Result<(bool, DigitalStrengthLevel), ParseError> {
        let parsed = self.current().text.as_deref().and_then(strength_level);
        match parsed {
            Some(parsed) => {
                self.advance();
                Ok(parsed)
            }
            None => Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: context.to_string(),
                    found: "a word that is not a drive strength (supply, strong, pull, weak or \
                            highz, with 0 or 1)"
                        .to_string(),
                },
                self.current_span(),
            )),
        }
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
    /// The gate's own delay is carried onto the assignment, where it is a
    /// delay on that driver like an `assign`'s (section 7.14).
    pub(super) fn parse_gate_instantiation(
        &mut self,
        gate: GatePrimitive,
        module: &mut Module,
    ) -> Result<(), ParseError> {
        let start = self.current_span();
        self.advance(); // consume the gate keyword
        let context = format!("`{}` gate", gate.keyword());

        // A drive strength `(strong1, weak0)` (section 7.8) applies to every
        // instance of the declaration. A pull gate may name only the strength
        // of the value it drives, and a switch takes none (A.3.1).
        let strength = if self.check(TokenKind::LParen) && self.strength_specification_follows() {
            Some(match gate.family() {
                GateFamily::Pull { one } => self.parse_pull_strength(one, &context)?,
                GateFamily::Mos { .. }
                | GateFamily::Cmos { .. }
                | GateFamily::Bidirectional { .. } => {
                    return Err(ParseError::new(
                        ParseErrorKind::UnsupportedConstruct {
                            context,
                            found: "a drive strength; IEEE 1364-2005 A.3.1 gives a switch \
                                    none, because it passes the strength of its input"
                                .to_string(),
                        },
                        self.current_span(),
                    ));
                }
                _ => self.parse_drive_strength(&context)?,
            })
        } else {
            None
        };

        // A.3.1: three delays for a three-state gate and a MOS or CMOS
        // switch, two for a logic gate and a controlled bidirectional switch
        // (turn-on and turn-off), none for a pull gate or a `tran`.
        let delay = if self.match_token(TokenKind::Hash) {
            let most = match gate.family() {
                GateFamily::Tristate { .. } | GateFamily::Mos { .. } | GateFamily::Cmos { .. } => 3,
                GateFamily::Logic
                | GateFamily::Bidirectional {
                    control: Some(_), ..
                } => 2,
                GateFamily::Pull { .. } | GateFamily::Bidirectional { control: None, .. } => 0,
            };
            Some(self.parse_delay3(most, &context)?)
        } else {
            None
        };

        let mut assignments = Vec::new();
        let mut arrays = Vec::new();
        loop {
            // The instance name is optional (section 7.1), so a `(` here opens
            // the terminal list of an unnamed instance.
            if !self.check(TokenKind::LParen) {
                self.expect_identifier("gate instance name")?;
            }
            // An array of instances, `not n[3:0] (y, v);` (section 7.1.5).
            let range = if self.match_token(TokenKind::LBracket) {
                let left = self.parse_expression()?;
                self.expect(TokenKind::Colon)?;
                let right = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                Some((left, right))
            } else {
                None
            };
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
            match range {
                Some((left, right)) => {
                    // Checked here, where the instance is, so the expanded
                    // array cannot fail on a count the source got wrong.
                    Self::gate_assignments(gate, terminals.clone(), None, span)?;
                    arrays.push(GateInstanceArray {
                        primitive: gate.keyword().into(),
                        left,
                        right,
                        terminals,
                        delay: delay.clone(),
                        strength: None,
                        span,
                    });
                }
                None => assignments.extend(Self::gate_assignments(
                    gate,
                    terminals,
                    delay.clone(),
                    span,
                )?),
            }

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;

        let declaration = start.extend(self.previous_span());
        let strength = strength.or_else(|| gate.implicit_strength());
        for assignment in &mut assignments {
            assignment.span = declaration;
            assignment.strength = strength;
        }
        for array in &mut arrays {
            array.span = declaration;
            array.strength = strength;
        }
        module.continuous_assigns.extend(assignments);
        module.gate_arrays.extend(arrays);
        Ok(())
    }

    /// A pull gate's strength (A.3.1): a pair, or the strength of the one
    /// value it drives.
    fn parse_pull_strength(
        &mut self,
        one: bool,
        context: &str,
    ) -> Result<DigitalDriveStrength, ParseError> {
        let follows_pair = self
            .tokens
            .get(self.pos + 2)
            .is_some_and(|token| token.kind == TokenKind::Comma);
        if follows_pair {
            return self.parse_drive_strength(context);
        }
        let start = self.current_span();
        self.expect(TokenKind::LParen)?;
        let (is_one, level) = self.strength_keyword(context)?;
        self.expect(TokenKind::RParen)?;
        if is_one != one {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: context.to_string(),
                    found: format!(
                        "a strength for {}; IEEE 1364-2005 A.3.1 lets a pull gate name only the \
                         strength of the value it drives",
                        if is_one { "1" } else { "0" }
                    ),
                },
                start.extend(self.previous_span()),
            ));
        }
        Ok(DigitalDriveStrength {
            zero: level,
            one: level,
        })
    }

    /// Whether the parenthesis under the cursor opens a drive-strength
    /// specification rather than a terminal list.
    ///
    /// A strength spec is two strength keywords separated by a comma, and every
    /// one of those words is an identifier to this lexer. Looking for the first
    /// one is enough: no terminal expression can begin with `supply0`.
    pub(super) fn strength_specification_follows(&self) -> bool {
        self.tokens
            .get(self.pos + 1)
            .and_then(|token| token.text.as_deref())
            .is_some_and(|text| strength_level(text).is_some())
    }

    /// `(small)`, `(medium)` or `(large)` after `trireg` (IEEE 1364-2005
    /// section 4.4.1); `None`, consuming nothing, when the parenthesis opens
    /// something else.
    fn parse_charge_strength(&mut self) -> Result<Option<DigitalStrengthLevel>, ParseError> {
        let level = match self
            .tokens
            .get(self.pos + 1)
            .and_then(|token| token.text.as_deref())
        {
            Some("small") => DigitalStrengthLevel::Small,
            Some("medium") => DigitalStrengthLevel::Medium,
            Some("large") => DigitalStrengthLevel::Large,
            _ => return Ok(None),
        };
        self.expect(TokenKind::LParen)?;
        self.advance();
        self.expect(TokenKind::RParen)?;
        Ok(Some(level))
    }

    /// A `trireg`'s `#delay`, `#(rise, fall)` or `#(rise, fall, decay)`
    /// (IEEE 1364-2005 sections 7.14 and 7.14.2).
    ///
    /// The third delay is the charge decay time, accepted only as a positive
    /// number, which is what a scheduled decay needs. The rise and fall
    /// delays are the net's delay (section 6.1.3), returned unless both are
    /// zero.
    fn parse_trireg_delay(&mut self) -> Result<(Option<Delay3>, Option<ChargeDecay>), ParseError> {
        self.expect(TokenKind::Hash)?;
        let mut delay = self.parse_delay3(3, "`trireg` declaration")?;
        let span = delay.span;
        let refuse = |found: &str| {
            Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "`trireg` declaration".to_string(),
                    found: found.to_string(),
                },
                span,
            ))
        };
        // A decimal number, not a based, sized or scaled one.
        let number = |delay: &Expression| match delay {
            Expression::Number(number)
                if number.raw.chars().all(|ch| {
                    ch.is_ascii_digit() || matches!(ch, '.' | '_' | 'e' | 'E' | '+' | '-')
                }) =>
            {
                Some(number.value)
            }
            _ => None,
        };
        let decay = match delay.values.get(2) {
            None => None,
            Some(value) => match number(value).and_then(ChargeDecay::new) {
                Some(decay) => Some(decay),
                None => {
                    return refuse(
                        "a charge decay time that is not a positive decimal number \
                         (IEEE 1364-2005 section 7.14.2)",
                    );
                }
            },
        };
        delay.values.truncate(2);
        let transitions = delay
            .values
            .iter()
            .any(|value| number(value) != Some(0.0))
            .then_some(delay);
        Ok((transitions, decay))
    }

    /// Turn one gate instance's terminal list into its driving assignments.
    pub(super) fn gate_assignments(
        gate: GatePrimitive,
        terminals: Vec<Expression>,
        delay: Option<Delay3>,
        span: crate::source::Span,
    ) -> Result<Vec<ContinuousAssign>, ParseError> {
        let (minimum, maximum) = gate.terminal_counts();
        if terminals.len() < minimum || maximum.is_some_and(|maximum| terminals.len() > maximum) {
            let required = match maximum {
                Some(exact) => format!("exactly {exact}"),
                None => format!("at least {minimum}"),
            };
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: format!("`{}` gate", gate.keyword()),
                    found: format!(
                        "{} terminal(s); IEEE 1364-2005 section 7 requires {required}",
                        terminals.len()
                    ),
                },
                span,
            ));
        }
        let assign = |output: &Expression, value: Expression, gated| {
            Ok(ContinuousAssign {
                target: Self::terminal_as_lvalue(output, gate)?,
                value,
                delay: delay.clone(),
                strength: None,
                gate: gated,
                span,
            })
        };
        // `{enable, data}`, the operand pair a gated driver writes.
        let pair = |enable: Expression, data: Expression| {
            Expression::ArrayLiteral(ArrayLiteralExpr {
                elements: vec![
                    ArrayLiteralElement::Value(enable),
                    ArrayLiteralElement::Value(data),
                ],
                assignment_pattern: false,
                span,
            })
        };
        let invert = |operand: &Expression, invert: bool| {
            if invert {
                Expression::Unary(UnaryExpr {
                    op: UnaryOp::BitNot,
                    operand: Box::new(operand.clone()),
                    span,
                })
            } else {
                operand.clone()
            }
        };
        use crate::canonical_ir::digital::DigitalGateKind;
        match gate.family() {
            GateFamily::Logic => {}
            // An inverted control is still `x` for `x` and `z` (table 5-13),
            // which is all the gates read of it.
            GateFamily::Tristate {
                invert: not,
                active_low,
            } => {
                let value = pair(
                    invert(&terminals[2], active_low),
                    invert(&terminals[1], not),
                );
                return Ok(vec![assign(
                    &terminals[0],
                    value,
                    Some(DigitalGateKind::Tristate),
                )?]);
            }
            GateFamily::Mos {
                active_low,
                resistive,
            } => {
                let value = pair(invert(&terminals[2], active_low), terminals[1].clone());
                let kind = DigitalGateKind::Mos { resistive };
                return Ok(vec![assign(&terminals[0], value, Some(kind))?]);
            }
            GateFamily::Cmos { resistive } => {
                let kind = Some(DigitalGateKind::Mos { resistive });
                let n = pair(terminals[2].clone(), terminals[1].clone());
                let p = pair(invert(&terminals[3], true), terminals[1].clone());
                return Ok(vec![
                    assign(&terminals[0], n, kind)?,
                    assign(&terminals[0], p, kind)?,
                ]);
            }
            GateFamily::Pull { one } => {
                let value = Self::gate_constant(gate, one, span)?;
                return terminals
                    .iter()
                    .map(|terminal| assign(terminal, value.clone(), None))
                    .collect();
            }
            // Each terminal is driven from the other under the control, a
            // `tran` being always on. The store reads these two drivers as one
            // switch between two net bits and resolves the island they join,
            // so neither drives the value it wrote.
            GateFamily::Bidirectional { control, resistive } => {
                let enable = match control {
                    Some(active_low) => invert(&terminals[2], active_low),
                    None => Self::gate_constant(gate, true, span)?,
                };
                let kind = Some(DigitalGateKind::Tran { resistive });
                return Ok(vec![
                    assign(
                        &terminals[0],
                        pair(enable.clone(), terminals[1].clone()),
                        kind,
                    )?,
                    assign(&terminals[1], pair(enable, terminals[0].clone()), kind)?,
                ]);
            }
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
        } else if gate.is_buffer() {
            // Section 7.4's `buf` drives `x` for a `z` input, as every other
            // gate does; an exclusive OR with `1'b0`, read as the source
            // literal reads, is the identity that does so.
            value = Expression::Binary(BinaryExpr {
                op: BinaryOp::BitXor,
                left: Box::new(value),
                right: Box::new(Expression::Number(NumberLit {
                    value: 0.0,
                    raw: "1'b0".into(),
                    span,
                })),
                span,
            });
        }

        outputs
            .into_iter()
            .map(|output| assign(output, value.clone(), None))
            .collect()
    }

    /// The one-bit constant `1'b1` or `1'b0` a gate drives or is enabled by.
    fn gate_constant(
        gate: GatePrimitive,
        one: bool,
        span: crate::source::Span,
    ) -> Result<Expression, ParseError> {
        let spelling = if one { "1'b1" } else { "1'b0" };
        let value = crate::four_state::decode(spelling).map_err(|message| {
            ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: format!("`{}` gate", gate.keyword()),
                    found: format!("its constant did not decode: {message}"),
                },
                span,
            )
        })?;
        Ok(Expression::Digital(DigitalExpr::FourState(FourStateLit {
            value,
            span,
        })))
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
            // A constant part select, as an array of instances shares a
            // terminal out, and as a bidirectional switch's other terminal is.
            Expression::Digital(DigitalExpr::PartSelect(select)) => Ok(DigitalLValue::PartSelect {
                name: select.name.clone(),
                msb: select.msb.clone(),
                lsb: select.lsb.clone(),
                span: select.span,
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
        self.nested(
            super::Nesting::Statement,
            Self::parse_digital_statement_level,
        )
    }

    /// One procedural statement, a level below the statement that contains it.
    fn parse_digital_statement_level(&mut self) -> Result<DigitalStatement, ParseError> {
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
            TokenKind::SystemIdentifier if self.is_root_reference() => {
                self.parse_digital_assignment()
            }
            TokenKind::SystemIdentifier => self.parse_system_task_enable(),
            TokenKind::Fork => self.parse_digital_fork(),
            TokenKind::Disable => self.parse_digital_disable(),
            TokenKind::AmsDigital if self.digital_keyword_is("wait") => self.parse_digital_wait(),
            TokenKind::Minus if self.event_trigger_follows() => self.parse_event_trigger(),
            TokenKind::Identifier | TokenKind::EscapedIdentifier => {
                match self.try_parse_task_enable()? {
                    Some(enable) => Ok(enable),
                    None => self.parse_digital_assignment(),
                }
            }
            TokenKind::LBrace => self.parse_digital_assignment(),
            // IEEE 1800-2017 section 11.4.2's prefix `++i;` / `--i;`, which
            // both reference simulators accept in a `.v` source.
            TokenKind::Plus | TokenKind::Minus if self.step_operator_follows() => {
                let assign = self.parse_prefix_step()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(DigitalStatement::BlockingAssign(assign))
            }
            TokenKind::Force => self.parse_procedural_continuous(ProceduralContinuousKind::Force),
            TokenKind::Release => {
                self.parse_procedural_continuous(ProceduralContinuousKind::Release)
            }
            TokenKind::Assign => self.parse_procedural_continuous(ProceduralContinuousKind::Assign),
            TokenKind::Deassign => {
                self.parse_procedural_continuous(ProceduralContinuousKind::Deassign)
            }
            _ => Err(self.unsupported_digital_statement()),
        }
    }

    /// `force lvalue = expression;`, `release lvalue;`, `assign lvalue =
    /// expression;` or `deassign lvalue;` (IEEE 1364-2005 section 9.3).
    fn parse_procedural_continuous(
        &mut self,
        kind: ProceduralContinuousKind,
    ) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        self.advance();
        let target = self.parse_digital_lvalue()?;
        let value = if kind.installs() {
            self.expect(TokenKind::Assign_)?;
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(DigitalStatement::ProceduralContinuous(
            DigitalProceduralContinuous {
                kind,
                target,
                value,
                span: start.extend(self.previous_span()),
            },
        ))
    }

    /// `$name [ ( [arg] {, [arg]} ) ] ;` — a system task enable (IEEE
    /// 1364-2005 section 10.2.2 / 17).
    ///
    /// Every name is accepted here; which ones exist, and with what
    /// arguments, is the analyzer's question, so an unknown task is refused
    /// with its name rather than as a token mismatch.
    fn parse_system_task_enable(&mut self) -> Result<DigitalStatement, ParseError> {
        let start = self.current_span();
        let name: SmolStr = self.current().text.clone().unwrap_or_default().into();
        self.advance();
        let args = if self.check(TokenKind::LParen) {
            self.parse_arg_list()?
        } else {
            Vec::new()
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(DigitalStatement::SystemTask(SystemFunction {
            name,
            args,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Whether the cursor and the next token spell `++` or `--` with nothing
    /// between them. The lexer has no increment token — `a++b` is `a + +b` in
    /// a Verilog-A expression — so the pair is recognized here, only where a
    /// statement can begin or end with one.
    fn step_operator_follows(&self) -> bool {
        let current = self.current();
        let Some(next) = self.tokens.get(self.pos + 1) else {
            return false;
        };
        matches!(current.kind, TokenKind::Plus | TokenKind::Minus)
            && next.kind == current.kind
            && current.span.end == next.span.start
    }

    /// The operator of an IEEE 1800-2017 section 11.4.1 compound assignment
    /// whose operator token is under the cursor, when the `=` that completes
    /// it follows immediately.
    fn compound_assignment_operator(&self) -> Option<CompoundOperator> {
        let current = self.current();
        let next = self.tokens.get(self.pos + 1)?;
        if next.kind != TokenKind::Assign_ || current.span.end != next.span.start {
            return None;
        }
        Some(CompoundOperator::Binary(match current.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Percent => BinaryOp::Mod,
            TokenKind::BitAnd => BinaryOp::BitAnd,
            TokenKind::BitOr => BinaryOp::BitOr,
            TokenKind::BitXor => BinaryOp::BitXor,
            // `<<<=` lexes as the same token as `<<=`, and section 11.4.1
            // makes them the same operation.
            TokenKind::Shl => BinaryOp::Shl,
            TokenKind::Shr => BinaryOp::Shr,
            TokenKind::ArithShr => return Some(CompoundOperator::ArithmeticShiftRight),
            _ => return None,
        }))
    }

    /// `++lvalue` / `--lvalue`, as the blocking assignment it abbreviates.
    fn parse_prefix_step(&mut self) -> Result<DigitalAssign, ParseError> {
        let start = self.current_span();
        let op = if self.check(TokenKind::Plus) {
            BinaryOp::Add
        } else {
            BinaryOp::Sub
        };
        self.advance();
        self.advance();
        let target = self.parse_digital_lvalue()?;
        Ok(Self::stepped_assignment(
            target,
            op,
            start.extend(self.previous_span()),
        ))
    }

    /// `lvalue = lvalue op 1`, the meaning IEEE 1800-2017 section 11.4.2 gives
    /// an increment or decrement used as a statement.
    fn stepped_assignment(target: DigitalLValue, op: BinaryOp, span: Span) -> DigitalAssign {
        let one = Expression::Number(NumberLit {
            value: 1.0,
            raw: "1".into(),
            span,
        });
        Self::compound_assignment(target, CompoundOperator::Binary(op), one, span)
    }

    /// `lvalue op= value` as `lvalue = lvalue op (value)` (IEEE 1800-2017
    /// section 11.4.1). The target's index expressions are read twice; the
    /// digital subset has no side-effecting expression an index could hold
    /// except a system function, which section 11.4.1 leaves undefined too.
    fn compound_assignment(
        target: DigitalLValue,
        op: CompoundOperator,
        value: Expression,
        span: Span,
    ) -> DigitalAssign {
        let read = Box::new(Self::lvalue_as_expression(&target));
        let value = Box::new(value);
        DigitalAssign {
            target,
            timing: None,
            value: match op {
                CompoundOperator::Binary(op) => Expression::Binary(BinaryExpr {
                    op,
                    left: read,
                    right: value,
                    span,
                }),
                CompoundOperator::ArithmeticShiftRight => {
                    Expression::Digital(DigitalExpr::ArithmeticShiftRight(ArithmeticShiftExpr {
                        left: read,
                        right: value,
                        span,
                    }))
                }
            },
            span,
        }
    }

    /// The expression that reads exactly what an assignment target writes.
    fn lvalue_as_expression(target: &DigitalLValue) -> Expression {
        match target {
            DigitalLValue::Identifier { name, span } => Expression::Identifier(Identifier {
                name: name.clone(),
                span: *span,
            }),
            DigitalLValue::BitSelect { name, index, span } => {
                Expression::ArrayAccess(ArrayAccessExpr {
                    normalized: false,
                    packed: None,
                    discrete_validity: None,
                    array: name.clone(),
                    index: index.clone(),
                    span: *span,
                })
            }
            DigitalLValue::PartSelect {
                name,
                msb,
                lsb,
                span,
            } => Expression::Digital(DigitalExpr::PartSelect(PartSelectExpr {
                name: name.clone(),
                msb: msb.clone(),
                lsb: lsb.clone(),
                span: *span,
            })),
            DigitalLValue::ArraySelect(select) => {
                Expression::Digital(DigitalExpr::ArraySelect(select.clone()))
            }
            DigitalLValue::IndexedPartSelect(select) => {
                Expression::Digital(DigitalExpr::IndexedPartSelect(select.clone()))
            }
            DigitalLValue::Concat { elements, span } => {
                Expression::ArrayLiteral(ArrayLiteralExpr {
                    elements: elements
                        .iter()
                        .map(|element| {
                            ArrayLiteralElement::Value(Self::lvalue_as_expression(element))
                        })
                        .collect(),
                    assignment_pattern: false,
                    span: *span,
                })
            }
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
                    digital_variables.push(self.parse_digital_variable_decl(None)?);
                }
                _ if self.keyword_declaration_follows() => {
                    self.parse_keyword_declaration(&mut variables, &mut digital_variables)?;
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
        if self.match_token(TokenKind::Repeat) {
            let start = self.previous_span();
            self.expect(TokenKind::LParen)?;
            let count = self.parse_expression()?;
            self.expect(TokenKind::RParen)?;
            // A repeat intra-assignment control must be followed by one event
            // control; it cannot wrap a delay or another repeat.
            if !self.check(TokenKind::At) {
                return Err(self.unsupported_digital_statement());
            }
            let TimingControl::Event(mut event) = self.parse_timing_control()? else {
                unreachable!("checked @");
            };
            event.repeat = Some(count);
            event.span = start.extend(event.span);
            return Ok(TimingControl::Event(event));
        }
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
                repeat: None,
                sensitivity: Sensitivity::Implicit,
                span: start.extend(self.previous_span()),
            }));
        }

        // `@name` without parentheses: one level-sensitive term, typically a
        // named event (IEEE 1364-2005 section 9.7.3).
        if self.check(TokenKind::Identifier) || self.check(TokenKind::EscapedIdentifier) {
            let term_start = self.current_span();
            let name = self.parse_reference_name("event name")?;
            let span = term_start.extend(self.previous_span());
            return Ok(TimingControl::Event(EventControl {
                repeat: None,
                sensitivity: Sensitivity::Explicit(vec![EventTerm {
                    edge: None,
                    signal: Expression::Identifier(Identifier { name, span }),
                    span,
                }]),
                span: start.extend(self.previous_span()),
            }));
        }
        self.expect(TokenKind::LParen)?;
        if self.check(TokenKind::Star) && self.peek_is(TokenKind::RParen) {
            self.advance();
            self.advance();
            return Ok(TimingControl::Event(EventControl {
                repeat: None,
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
            repeat: None,
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

    /// A delay value or a parenthesized delay expression, which may be a
    /// `min : typ : max` expression (IEEE 1364-2005 section 5.3).
    pub(super) fn parse_delay_value(&mut self) -> Result<Expression, ParseError> {
        if self.check(TokenKind::LParen) {
            self.advance();
            let expression = self.parse_mintypmax_expression()?;
            if self.check(TokenKind::Comma) {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: "delay control".to_string(),
                        found: "separate rise, fall and turn-off delays (IEEE 1364-2005 section \
                                7.14); write a single delay expression instead"
                            .to_string(),
                    },
                    self.current_span(),
                ));
            }
            self.expect(TokenKind::RParen)?;
            return Ok(expression);
        }
        self.parse_primary()
    }

    /// The delay after a gate's, a continuous assignment's or a net's `#`:
    /// a value, or a parenthesized list of at most `most` of them (IEEE
    /// 1364-2005 A.2.2.3). `what` names the construct for a refusal.
    pub(super) fn parse_delay3(&mut self, most: usize, what: &str) -> Result<Delay3, ParseError> {
        let start = self.current_span();
        let mut values = Vec::new();
        if self.match_token(TokenKind::LParen) {
            loop {
                values.push(self.parse_mintypmax_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
        } else {
            values.push(self.parse_primary()?);
        }
        let span = start.extend(self.previous_span());
        if values.len() > most {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: what.to_string(),
                    found: match most {
                        0 => "a delay; IEEE 1364-2005 section 7.14 gives this primitive none"
                            .to_string(),
                        _ => format!(
                            "{} delays; IEEE 1364-2005 section 7.14 gives it at most {most}",
                            values.len()
                        ),
                    },
                },
                span,
            ));
        }
        Ok(Delay3 { values, span })
    }

    // ------------------------------------------------------------------
    // Assignments
    // ------------------------------------------------------------------

    /// `lvalue = expr;` or `lvalue <= expr;`, and the IEEE 1800-2017 section
    /// 11.4 statement forms `lvalue op= expr;`, `lvalue++;` and `lvalue--;`.
    fn parse_digital_assignment(&mut self) -> Result<DigitalStatement, ParseError> {
        let assignment_start = self.current_span();
        let target = self.parse_digital_lvalue()?;
        if let Some(assign) = self.parse_assignment_operator_tail(&target, assignment_start)? {
            self.expect(TokenKind::Semicolon)?;
            return Ok(DigitalStatement::BlockingAssign(assign));
        }
        let nonblocking = if self.match_token(TokenKind::Le) {
            true
        } else {
            self.expect(TokenKind::Assign_)?;
            false
        };
        // Intra-assignment timing control (IEEE 1364-2005 section 9.2.2).
        let timing = if self.check(TokenKind::At)
            || self.check(TokenKind::Hash)
            || self.check(TokenKind::Repeat)
        {
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

    /// After an assignment target: a compound operator (`op=`) with its
    /// right-hand side, or a postfix `++`/`--`, as the blocking assignment it
    /// abbreviates. `None` leaves the cursor on a plain `=` or `<=`.
    ///
    /// A compound assignment takes no intra-assignment timing control (IEEE
    /// 1800-2017 section 11.4.1 has no such form), so one is a syntax error
    /// here rather than a delay silently attached to a read-modify-write.
    fn parse_assignment_operator_tail(
        &mut self,
        target: &DigitalLValue,
        start: Span,
    ) -> Result<Option<DigitalAssign>, ParseError> {
        if self.step_operator_follows() {
            let op = if self.check(TokenKind::Plus) {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            self.advance();
            self.advance();
            return Ok(Some(Self::stepped_assignment(
                target.clone(),
                op,
                start.extend(self.previous_span()),
            )));
        }
        let Some(op) = self.compound_assignment_operator() else {
            return Ok(None);
        };
        self.advance();
        self.advance();
        if self.check(TokenKind::Hash) || self.check(TokenKind::At) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "compound assignment".to_string(),
                    found: "an intra-assignment timing control; IEEE 1800-2017 section 11.4.1 \
                            gives `op=` no timing form, so write `x = #d x op y;`"
                        .to_string(),
                },
                self.current_span(),
            ));
        }
        let value = self.parse_expression()?;
        Ok(Some(Self::compound_assignment(
            target.clone(),
            op,
            value,
            start.extend(self.previous_span()),
        )))
    }

    /// The `lvalue = expr` form used by a `for` header, which carries no
    /// terminating semicolon and no timing control. The IEEE 1800-2017
    /// section 11.4 forms `i++`, `++i`, `i--`, `--i` and `i op= e` are
    /// accepted too, as both reference simulators do in a `.v` source.
    fn parse_digital_assignment_expression(&mut self) -> Result<DigitalAssign, ParseError> {
        let start = self.current_span();
        if self.step_operator_follows() {
            return self.parse_prefix_step();
        }
        let target = self.parse_digital_lvalue()?;
        if let Some(assign) = self.parse_assignment_operator_tail(&target, start)? {
            return Ok(assign);
        }
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

        let name = self.parse_reference_name("assignment target")?;
        if !self.match_token(TokenKind::LBracket) {
            return Ok(DigitalLValue::Identifier {
                name,
                span: start.extend(self.previous_span()),
            });
        }

        let first = self.parse_expression()?;
        if let Some((ascending, width)) = self.parse_indexed_part_width()? {
            return Ok(DigitalLValue::IndexedPartSelect(IndexedPartSelectExpr {
                name,
                indices: Vec::new(),
                base: Box::new(first),
                width: Box::new(width),
                ascending,
                span: start.extend(self.previous_span()),
            }));
        }
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
        if self.check(TokenKind::LBracket) {
            return Ok(match self.parse_array_packed_select(name, first, start)? {
                PackedSelection::Array(select) => DigitalLValue::ArraySelect(select),
                PackedSelection::Indexed(select) => DigitalLValue::IndexedPartSelect(select),
            });
        }
        Ok(DigitalLValue::BitSelect {
            name,
            index: Box::new(first),
            span: start.extend(self.previous_span()),
        })
    }

    pub(super) fn parse_array_packed_select(
        &mut self,
        name: SmolStr,
        index: Expression,
        start: crate::Span,
    ) -> Result<PackedSelection, ParseError> {
        let mut additional_indices = Vec::new();
        let select = loop {
            self.expect(TokenKind::LBracket)?;
            let first = self.parse_expression()?;
            if let Some((ascending, width)) = self.parse_indexed_part_width()? {
                let mut indices = vec![index];
                indices.append(&mut additional_indices);
                return Ok(PackedSelection::Indexed(IndexedPartSelectExpr {
                    name,
                    indices,
                    base: Box::new(first),
                    width: Box::new(width),
                    ascending,
                    span: start.extend(self.previous_span()),
                }));
            }
            if self.match_token(TokenKind::Colon) {
                let lsb = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                break PackedSelect::Part {
                    msb: Box::new(first),
                    lsb: Box::new(lsb),
                };
            }
            self.expect(TokenKind::RBracket)?;
            if self.check(TokenKind::LBracket) {
                additional_indices.push(first);
            } else {
                break PackedSelect::Bit(Box::new(first));
            }
        };
        Ok(PackedSelection::Array(ArraySelectExpr {
            name,
            index: Box::new(index),
            additional_indices,
            select,
            span: start.extend(self.previous_span()),
        }))
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
