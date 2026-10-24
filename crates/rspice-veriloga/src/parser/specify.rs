//! `specify` blocks and `specparam` declarations (IEEE 1364-2005 section 14).
//!
//! A specparam is a constant of its module and joins its localparams. A
//! module path joins [`Module::specify_paths`]; a state-dependent path's
//! condition becomes a one-bit net the module drives from it, so the path
//! reads one net's value when its destination changes. A `PATHPULSE$`
//! specparam joins [`Module::specify_pulse_limits`] and a pulse style
//! declaration [`Module::specify_pulse_styles`] (section 14.6). A timing
//! check joins [`Module::timing_checks`] (section 15), and each delayed
//! signal it names is driven by a copy of its signal, which a module path
//! delays (section 15.8). A condition of a timing check other than one
//! signal becomes a one-bit net, as a path's does (section 15.6).

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;

/// The prefix of the net a state-dependent path's condition drives. `$` is
/// legal inside an identifier, so the names stay apart from the author's
/// only by convention, as a primitive instance's nets do.
const CONDITION_PREFIX: &str = "specify$if";

/// The prefix of the net a timing check condition other than one signal
/// drives, named as [`CONDITION_PREFIX`]'s are.
const CHECK_CONDITION_PREFIX: &str = "specify$tc";

fn refusal(context: &str, found: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::UnsupportedConstruct {
            context: context.to_string(),
            found: found.into(),
        },
        span,
    )
}

/// A terminal's bit-select as written, or `None` for a whole signal.
fn bit_text(terminal: &SpecifyTerminal) -> Option<&str> {
    match &terminal.select {
        Some(TerminalSelect::Bit { text, .. }) => Some(text),
        _ => None,
    }
}

impl Parser<'_> {
    /// `specify ... endspecify`, at its opening keyword.
    pub(super) fn parse_specify_block(&mut self, module: &mut Module) -> Result<(), ParseError> {
        self.advance(); // `specify`
        while !self.check(TokenKind::Endspecify) {
            if self.at_end() {
                return Err(self.error(ParseErrorKind::UnexpectedEof));
            }
            if self.check_identifier_text("specparam") {
                self.parse_specparam(module)?;
                continue;
            }
            let style = [
                ("pulsestyle_onevent", PulseStyle::OnEvent),
                ("pulsestyle_ondetect", PulseStyle::OnDetect),
                ("showcancelled", PulseStyle::ShowCancelled),
                ("noshowcancelled", PulseStyle::NoShowCancelled),
            ]
            .into_iter()
            .find(|(keyword, _)| self.check_identifier_text(keyword));
            if let Some((_, style)) = style {
                self.parse_pulse_style(module, style)?;
                continue;
            }
            match self.current().kind {
                TokenKind::LParen | TokenKind::If => self.parse_specify_path(module)?,
                TokenKind::Identifier if self.check_identifier_text("ifnone") => {
                    self.parse_specify_path(module)?;
                }
                TokenKind::SystemIdentifier => self.parse_timing_check(module)?,
                _ => return Err(self.unsupported_current("`specify` item")),
            }
        }
        self.advance(); // `endspecify`
        Ok(())
    }

    /// `pulsestyle_onevent`, `pulsestyle_ondetect`, `showcancelled` or
    /// `noshowcancelled` and its outputs (section 14.6.4), at the keyword.
    /// Each output's declaration comes before any module path to it.
    fn parse_pulse_style(
        &mut self,
        module: &mut Module,
        style: PulseStyle,
    ) -> Result<(), ParseError> {
        let keyword = self.current().text.clone().unwrap_or_default();
        self.advance();
        for output in self.parse_specify_terminals("pulse style output")? {
            if module
                .specify_paths
                .iter()
                .any(|path| path.destinations.iter().any(|to| to.name == output.name))
            {
                return Err(refusal(
                    "`specify` pulse style",
                    format!(
                        "`{keyword} {}` after a module path to it; IEEE 1364-2005 section \
                         14.6.4 declares an output's pulse style before its paths",
                        output.name
                    ),
                    output.span,
                ));
            }
            module.specify_pulse_styles.push(SpecifyPulseStyle {
                output: output.name,
                style,
                span: output.span,
            });
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    /// The rest of a `PATHPULSE$` specparam (section 14.6.1), after its
    /// name: `= (reject [, error])`, or as Icarus Verilog also reads it,
    /// `= reject`.
    fn parse_pulse_limit(
        &mut self,
        module: &mut Module,
        name: &str,
        start: Span,
    ) -> Result<(), ParseError> {
        let terminals = match name.strip_prefix("PATHPULSE$") {
            Some("") => None,
            Some(rest) => match rest.split_once('$') {
                Some((input, output))
                    if !input.is_empty() && !output.is_empty() && !output.contains('$') =>
                {
                    Some((SmolStr::from(input), SmolStr::from(output)))
                }
                _ => {
                    return Err(refusal(
                        "`specify` pulse limit",
                        format!(
                            "`{name}`, which names no `PATHPULSE$input$output` path (IEEE \
                             1364-2005 section 14.6.1)"
                        ),
                        start,
                    ));
                }
            },
            None => unreachable!("the caller saw the prefix"),
        };
        if self.check(TokenKind::LBracket) {
            return Err(refusal(
                "`specify` pulse limit",
                format!("`{name}` with a select of its terminal"),
                self.current_span(),
            ));
        }
        self.expect(TokenKind::Assign_)?;
        let (reject, error) = if self.match_token(TokenKind::LParen) {
            let reject = self.parse_mintypmax_expression()?;
            let error = if self.match_token(TokenKind::Comma) {
                Some(self.parse_mintypmax_expression()?)
            } else {
                None
            };
            self.expect(TokenKind::RParen)?;
            (reject, error)
        } else {
            (self.parse_mintypmax_expression()?, None)
        };
        module.specify_pulse_limits.push(SpecifyPulseLimit {
            terminals,
            reject,
            error,
            span: start.extend(self.previous_span()),
        });
        Ok(())
    }

    /// `specparam [range] name = value, ...;` (section 14.2.4.1 and A.2.2.1),
    /// in a `specify` block or as a module item.
    pub(super) fn parse_specparam(&mut self, module: &mut Module) -> Result<(), ParseError> {
        self.advance(); // `specparam`
        let packed_range = self.parse_optional_vector_range()?;
        loop {
            let item_start = self.current_span();
            let name = self.expect_identifier("specparam name")?;
            if name.starts_with("PATHPULSE$") {
                self.parse_pulse_limit(module, &name, item_start)?;
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
                continue;
            }
            self.expect(TokenKind::Assign_)?;
            let value = self.parse_mintypmax_expression()?;
            module.localparams.push(ParameterDecl {
                is_given: false,
                param_type: ParamType::Real,
                type_is_explicit: false,
                signedness: None,
                packed_range: packed_range.clone(),
                name: name.into(),
                dimensions: Vec::new(),
                default: Some(value),
                range: None,
                units: None,
                description: None,
                attributes: Vec::new(),
                span: item_start.extend(self.previous_span()),
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    /// One path declaration (section 14.2 and A.7.2 to A.7.4).
    fn parse_specify_path(&mut self, module: &mut Module) -> Result<(), ParseError> {
        let start = self.current_span();
        let mut ifnone = false;
        let mut condition = None;
        if self.match_token(TokenKind::If) {
            self.expect(TokenKind::LParen)?;
            let expression = self.parse_expression()?;
            self.expect(TokenKind::RParen)?;
            condition = Some(expression);
        } else if self.check_identifier_text("ifnone") {
            self.advance();
            ifnone = true;
        }
        self.expect(TokenKind::LParen)?;
        let edge = if self.match_token(TokenKind::Posedge) {
            Some(EdgeKind::Posedge)
        } else if self.match_token(TokenKind::Negedge) {
            Some(EdgeKind::Negedge)
        } else {
            None
        };
        let sources = self.parse_specify_terminals("path source")?;
        // An optional polarity, then `=>` or `*>`.
        if !self.match_token(TokenKind::Plus) {
            self.match_token(TokenKind::Minus);
        }
        let full = !self.match_token(TokenKind::Assign_);
        if full {
            self.expect(TokenKind::Star)?;
        }
        self.expect(TokenKind::Gt)?;
        let destinations = if self.match_token(TokenKind::LParen) {
            // An edge-sensitive path's `(destination [+|-]: data)`.
            let destinations = self.parse_specify_terminals("path destination")?;
            if !self.match_token(TokenKind::Plus) {
                self.match_token(TokenKind::Minus);
            }
            self.expect(TokenKind::Colon)?;
            self.parse_expression()?;
            self.expect(TokenKind::RParen)?;
            destinations
        } else {
            self.parse_specify_terminals("path destination")?
        };
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Assign_)?;
        let mut delays = Vec::new();
        if self.match_token(TokenKind::LParen) {
            loop {
                delays.push(self.parse_mintypmax_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
        } else {
            delays.push(self.parse_mintypmax_expression()?);
        }
        self.expect(TokenKind::Semicolon)?;
        let span = start.extend(self.previous_span());
        if !matches!(delays.len(), 1 | 2 | 3 | 6 | 12) {
            return Err(refusal(
                "module path delay",
                format!(
                    "{} delays; IEEE 1364-2005 section 14.3.1 gives a path one, two, three, \
                     six or twelve",
                    delays.len()
                ),
                span,
            ));
        }
        let condition = condition.map(|expression| {
            let name: SmolStr = format!(
                "{CONDITION_PREFIX}{}",
                module
                    .specify_paths
                    .iter()
                    .filter(|path| path.condition.is_some())
                    .count()
            )
            .into();
            let span = expression.span();
            // IEEE 1364-2005 section 14.2.4.1: a condition that evaluates
            // to several bits decides by its least significant bit, which
            // is what a one-bit net assigned from it keeps.
            module.digital_nets.push(DigitalNetDecl {
                kind: DigitalNetKind::Wire,
                signedness: Signedness::Unsigned,
                range: None,
                items: vec![DigitalDeclItem {
                    name: name.clone(),
                    dimensions: Vec::new(),
                    init: Some(expression),
                    span,
                }],
                span,
            });
            name
        });
        module.specify_paths.push(SpecifyPath {
            edge,
            full,
            sources,
            destinations,
            condition,
            ifnone,
            delays,
            span,
        });
        Ok(())
    }

    /// A comma-separated list of path terminals, each a name with an optional
    /// bit- or part-select, which names the whole port.
    fn parse_specify_terminals(
        &mut self,
        context: &str,
    ) -> Result<Vec<SpecifyTerminal>, ParseError> {
        let mut terminals = Vec::new();
        loop {
            terminals.push(self.parse_specify_terminal(context)?);
            if !self.match_token(TokenKind::Comma) {
                return Ok(terminals);
            }
        }
    }

    /// One terminal: a name with an optional bit- or part-select, which
    /// names the whole port.
    fn parse_specify_terminal(&mut self, context: &str) -> Result<SpecifyTerminal, ParseError> {
        let span = self.current_span();
        let name = self.expect_identifier(context)?;
        let mut select = None;
        if self.match_token(TokenKind::LBracket) {
            let start = self.pos;
            let index = self.parse_expression()?;
            select = Some(if self.check(TokenKind::RBracket) {
                TerminalSelect::Bit {
                    text: self.written_since(start).into(),
                    index: Box::new(index),
                }
            } else {
                TerminalSelect::Part
            });
            let mut depth = 1;
            while depth > 0 {
                if self.at_end() {
                    return Err(self.error(ParseErrorKind::UnexpectedEof));
                }
                match self.advance().kind {
                    TokenKind::LBracket => depth += 1,
                    TokenKind::RBracket => depth -= 1,
                    _ => {}
                }
            }
        }
        Ok(SpecifyTerminal {
            name: name.into(),
            select,
            span: span.extend(self.previous_span()),
        })
    }

    /// A delayed signal (A.7.5.2 `delayed_reference` and `delayed_data`): a
    /// net, or one bit of it by a constant mintypmax expression.
    fn parse_delayed_signal(&mut self, context: &str) -> Result<SpecifyTerminal, ParseError> {
        let span = self.current_span();
        let name = self.expect_identifier(context)?;
        let mut select = None;
        if self.match_token(TokenKind::LBracket) {
            let start = self.pos;
            let index = self.parse_mintypmax_expression()?;
            select = Some(TerminalSelect::Bit {
                text: self.written_since(start).into(),
                index: Box::new(index),
            });
            self.expect(TokenKind::RBracket)?;
        }
        Ok(SpecifyTerminal {
            name: name.into(),
            select,
            span: span.extend(self.previous_span()),
        })
    }

    /// A timing check condition, parsed from the cursor: an event's `&&&`
    /// condition (A.7.5.3 `timing_check_condition`), or with `mintypmax` a
    /// `$setuphold`'s or `$recrem`'s timestamp or timecheck condition
    /// (A.7.5.2).
    ///
    /// Section 15.6 and A.7.5.3: an expression `E`, `~E`, or `E` compared
    /// with a scalar constant by `==`, `!=`, `===` or `!==`, in parentheses
    /// or not. The least significant bit of `E` conditions the check. `E`,
    /// `~E`, `===` and `!==` are deterministic, so `x` or `z` on that bit
    /// disables the check; `==` and `!=` are nondeterministic, so it enables
    /// it (OVI Verilog LRM 1.0 B.9.10). An `E` other than one signal becomes
    /// a one-bit net the module drives from it, which keeps that bit.
    fn parse_timing_check_condition(
        &mut self,
        module: &mut Module,
        mintypmax: bool,
    ) -> Result<TimingCheckCondition, ParseError> {
        fn constant(expression: &Expression) -> Option<bool> {
            let raw = match expression {
                Expression::Number(number) => number.raw.as_str(),
                Expression::Digital(DigitalExpr::FourState(literal)) => literal.value.raw.as_str(),
                _ => return None,
            };
            match raw.to_ascii_lowercase().as_str() {
                "0" | "1'b0" | "'b0" => Some(false),
                "1" | "1'b1" | "'b1" => Some(true),
                _ => None,
            }
        }
        let start = self.pos;
        let expression = if mintypmax {
            self.parse_mintypmax_expression()?
        } else {
            self.parse_expression()?
        };
        let text = self.written_since(start);
        let (operand, enabling, unknown_enables) = match expression {
            Expression::Unary(UnaryExpr {
                op: UnaryOp::BitNot,
                operand,
                ..
            }) => (*operand, false, false),
            Expression::Binary(BinaryExpr {
                op: op @ (BinaryOp::Eq | BinaryOp::Ne),
                left,
                right,
                ..
            }) if constant(&right).is_some() => (
                *left,
                constant(&right) == Some(matches!(op, BinaryOp::Eq)),
                true,
            ),
            Expression::Digital(DigitalExpr::CaseEquality(equality))
                if constant(&equality.right).is_some() =>
            {
                (
                    *equality.left,
                    constant(&equality.right) == Some(!equality.negate),
                    false,
                )
            }
            expression => (expression, true, false),
        };
        // A parameter names no signal, so it is an expression like any other.
        let constant_name = |name: &str| {
            module
                .parameters
                .iter()
                .chain(&module.localparams)
                .any(|parameter| parameter.name == name)
        };
        let signal = match operand {
            Expression::Identifier(identifier) if !constant_name(&identifier.name) => {
                SpecifyTerminal {
                    name: identifier.name,
                    select: None,
                    span: identifier.span,
                }
            }
            operand => {
                let name: SmolStr = format!(
                    "{CHECK_CONDITION_PREFIX}{}",
                    module
                        .digital_nets
                        .iter()
                        .flat_map(|net| &net.items)
                        .filter(|item| item.name.starts_with(CHECK_CONDITION_PREFIX))
                        .count()
                )
                .into();
                let span = operand.span();
                module.digital_nets.push(DigitalNetDecl {
                    kind: DigitalNetKind::Wire,
                    signedness: Signedness::Unsigned,
                    range: None,
                    items: vec![DigitalDeclItem {
                        name: name.clone(),
                        dimensions: Vec::new(),
                        init: Some(operand),
                        span,
                    }],
                    span,
                });
                SpecifyTerminal {
                    name,
                    select: None,
                    span,
                }
            }
        };
        Ok(TimingCheckCondition {
            signal,
            enabling,
            unknown_enables,
            text: text.into(),
        })
    }

    /// The tokens from `start` to the cursor as written: a space between
    /// two, but none after an opening bracket or a unary `~` or `!`, and
    /// none before a closing bracket or a select's `[`.
    fn written_since(&self, start: usize) -> String {
        let mut text = String::new();
        let mut glued = true;
        for token in &self.tokens[start..self.pos] {
            if !glued
                && !matches!(
                    token.kind,
                    TokenKind::RParen | TokenKind::RBracket | TokenKind::LBracket
                )
            {
                text.push(' ');
            }
            match &token.text {
                Some(written) => text.push_str(written),
                None => text.push_str(token.kind.spelling().trim_matches('\'')),
            }
            glued = matches!(
                token.kind,
                TokenKind::LParen | TokenKind::LBracket | TokenKind::BitNot | TokenKind::Not
            );
        }
        text
    }

    /// A timing check (section 15 and A.7.5), at its system task name.
    fn parse_timing_check(&mut self, module: &mut Module) -> Result<(), ParseError> {
        use TimingCheckKind::*;
        let start = self.current_span();
        let name = self.current().text.clone().unwrap_or_default();
        let kind = match name.as_str() {
            "$setup" => Setup,
            "$hold" => Hold,
            "$setuphold" => SetupHold,
            "$recovery" => Recovery,
            "$removal" => Removal,
            "$recrem" => RecRem,
            "$skew" => Skew,
            "$period" => Period,
            "$width" => Width,
            "$nochange" => NoChange,
            _ => {
                return Err(refusal(
                    "`specify` timing check",
                    format!("`{name}`"),
                    start,
                ));
            }
        };
        self.advance();
        self.expect(TokenKind::LParen)?;
        let first = self.parse_timing_check_event(module)?;
        let second = if matches!(kind, Period | Width) {
            None
        } else {
            self.expect(TokenKind::Comma)?;
            Some(self.parse_timing_check_event(module)?)
        };
        // `$setup(data, reference, ...)`; every other names the reference
        // first.
        let (reference, data) = match (kind, second) {
            (Setup, Some(reference)) => (reference, Some(first)),
            (_, data) => (first, data),
        };
        if matches!(kind, Period | Width | NoChange) && reference.edges.is_none() {
            return Err(self.error(ParseErrorKind::UnexpectedToken(format!(
                "`{name}` takes an edge-controlled reference event (IEEE 1364-2005 section 15.3)"
            ))));
        }
        let mut limits = Vec::new();
        for _ in 0..if matches!(kind, SetupHold | RecRem | NoChange) {
            2
        } else {
            1
        } {
            self.expect(TokenKind::Comma)?;
            limits.push(self.parse_mintypmax_expression()?);
        }
        let optional: &[&str] = match kind {
            Width => &["threshold", "notifier"],
            SetupHold | RecRem => &[
                "notifier",
                "timestamp condition",
                "timecheck condition",
                "delayed reference",
                "delayed data",
            ],
            _ => &["notifier"],
        };
        let (mut notifier, mut delayed_reference, mut delayed_data) = (None, None, None);
        let (mut timestamp_condition, mut timecheck_condition) = (None, None);
        for slot in optional {
            if !self.match_token(TokenKind::Comma) {
                break;
            }
            if self.check(TokenKind::Comma) || self.check(TokenKind::RParen) {
                continue; // left empty
            }
            match *slot {
                "threshold" => limits.push(self.parse_mintypmax_expression()?),
                "notifier" => {
                    notifier = Some(self.parse_specify_terminal("timing check notifier")?)
                }
                "delayed reference" => {
                    delayed_reference = Some(self.parse_delayed_signal("delayed reference")?);
                }
                "delayed data" => {
                    delayed_data = Some(self.parse_delayed_signal("delayed data")?);
                }
                // A.7.5.2: `stamptime_condition` and `checktime_condition`
                // are mintypmax expressions.
                condition => {
                    let parsed = self.parse_timing_check_condition(module, true)?;
                    if condition == "timestamp condition" {
                        timestamp_condition = Some(parsed);
                    } else {
                        timecheck_condition = Some(parsed);
                    }
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;
        let span = start.extend(self.previous_span());
        // Each delayed net, or bit of one, is driven by a copy of its signal
        // or of the bit of it the check names (A.7.5.2), which a module path
        // delays (section 15.8). One that several checks name for one signal
        // is driven once; its delay comes from all of them (section 15.5.1).
        for (delayed, source) in [
            (&delayed_reference, Some(&reference)),
            (&delayed_data, data.as_ref()),
        ] {
            let (Some(delayed), Some(source)) = (delayed, source) else {
                continue;
            };
            let source = &source.terminal;
            if matches!(source.select, Some(TerminalSelect::Part)) {
                return Err(refusal(
                    "`specify` timing check",
                    format!(
                        "delayed signal `{}` of a part-select of `{}`",
                        delayed.name, source.name
                    ),
                    source.span,
                ));
            }
            let same = |one: &SpecifyTerminal, other: &SpecifyTerminal| {
                one.name == other.name && bit_text(one) == bit_text(other)
            };
            let copied = module.timing_checks.iter().any(|check| {
                [
                    (&check.delayed_reference, Some(&check.reference)),
                    (&check.delayed_data, check.data.as_ref()),
                ]
                .into_iter()
                .any(|(other, other_source)| {
                    other.as_ref().is_some_and(|other| same(other, delayed))
                        && other_source.is_some_and(|other| same(&other.terminal, source))
                })
            });
            if copied {
                continue;
            }
            let target = match &delayed.select {
                Some(TerminalSelect::Bit { index, .. }) => DigitalLValue::BitSelect {
                    name: delayed.name.clone(),
                    index: index.clone(),
                    span: delayed.span,
                },
                _ => DigitalLValue::Identifier {
                    name: delayed.name.clone(),
                    span: delayed.span,
                },
            };
            let value = match &source.select {
                Some(TerminalSelect::Bit { index, .. }) => {
                    Expression::ArrayAccess(ArrayAccessExpr {
                        normalized: false,
                        packed: None,
                        array: source.name.clone(),
                        discrete_validity: None,
                        index: index.clone(),
                        span: source.span,
                    })
                }
                _ => Expression::Identifier(Identifier {
                    name: source.name.clone(),
                    span: source.span,
                }),
            };
            module.continuous_assigns.push(ContinuousAssign {
                target,
                value,
                delay: None,
                strength: None,
                gate: None,
                span: delayed.span,
            });
        }
        module.timing_checks.push(SpecifyTimingCheck {
            kind,
            reference,
            data,
            limits,
            notifier,
            timestamp_condition,
            timecheck_condition,
            delayed_reference,
            delayed_data,
            span,
        });
        Ok(())
    }

    /// A timing check event (A.7.5.3): an optional edge control, a
    /// terminal and an optional `&&& condition`.
    fn parse_timing_check_event(
        &mut self,
        module: &mut Module,
    ) -> Result<TimingCheckEvent, ParseError> {
        let (edges, edge_text) = if self.match_token(TokenKind::Posedge) {
            (
                Some(TimingCheckEvent::POSEDGE),
                Some(SmolStr::from("posedge")),
            )
        } else if self.match_token(TokenKind::Negedge) {
            (
                Some(TimingCheckEvent::NEGEDGE),
                Some(SmolStr::from("negedge")),
            )
        } else if self.check(TokenKind::AmsDigital)
            && self.current().text.as_deref() == Some("edge")
        {
            let (edges, text) = self.parse_edge_descriptors()?;
            (Some(edges), Some(text))
        } else {
            (None, None)
        };
        let terminal = self.parse_specify_terminal("timing check terminal")?;
        let condition = if self.check(TokenKind::And)
            && self
                .tokens
                .get(self.pos + 1)
                .is_some_and(|token| token.kind == TokenKind::BitAnd)
        {
            self.advance();
            self.advance();
            Some(self.parse_timing_check_condition(module, false)?)
        } else {
            None
        };
        Ok(TimingCheckEvent {
            edges,
            edge_text,
            terminal,
            condition,
        })
    }

    /// `edge [01, x0, ...]` (section 15.4), at `edge`: the transitions it
    /// names, an `x` descriptor naming the same transition with `z`, and
    /// the specifier as written.
    fn parse_edge_descriptors(&mut self) -> Result<(u16, SmolStr), ParseError> {
        self.advance(); // `edge`
        self.expect(TokenKind::LBracket)?;
        let mut edges = 0u16;
        let mut written = Vec::new();
        loop {
            let span = self.current_span();
            let mut descriptor = String::new();
            while !self.check(TokenKind::Comma) && !self.check(TokenKind::RBracket) {
                if self.at_end() {
                    return Err(self.error(ParseErrorKind::UnexpectedEof));
                }
                descriptor.push_str(self.advance().text.as_deref().unwrap_or("?"));
            }
            let descriptor = descriptor.to_ascii_lowercase().replace('z', "x");
            edges |= match descriptor.as_str() {
                "01" => 1 << 0,
                "10" => 1 << 1,
                "0x" => (1 << 6) | (1 << 2),
                "x1" => (1 << 7) | (1 << 3),
                "1x" => (1 << 8) | (1 << 4),
                "x0" => (1 << 9) | (1 << 5),
                _ => {
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedToken(format!(
                            "edge descriptor `{descriptor}`; IEEE 1364-2005 section 15.4 \
                             names 01, 10, 0x, x1, 1x or x0 (z for x)"
                        )),
                        span,
                    ));
                }
            };
            written.push(descriptor);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBracket)?;
        Ok((edges, format!("edge[{}]", written.join(", ")).into()))
    }
}
