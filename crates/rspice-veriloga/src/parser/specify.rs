//! `specify` blocks and `specparam` declarations (IEEE 1364-2005 section 14).
//!
//! A specparam is a constant of its module and joins its localparams. A
//! module path joins [`Module::specify_paths`]; a state-dependent path's
//! condition becomes a one-bit net the module drives from it, so the path
//! reads one net's value when its destination changes.

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

fn refusal(context: &str, found: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::UnsupportedConstruct {
            context: context.to_string(),
            found: found.into(),
        },
        span,
    )
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
            match self.current().kind {
                TokenKind::LParen | TokenKind::If => self.parse_specify_path(module)?,
                TokenKind::Identifier if self.check_identifier_text("ifnone") => {
                    self.parse_specify_path(module)?;
                }
                TokenKind::SystemIdentifier => {
                    return Err(refusal(
                        "`specify` timing check",
                        format!("`{}`", self.current().text.as_deref().unwrap_or("")),
                        self.current_span(),
                    ));
                }
                _ => return Err(self.unsupported_current("`specify` item")),
            }
        }
        self.advance(); // `endspecify`
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
                return Err(refusal(
                    "`specify` pulse limit",
                    format!("`{name}`"),
                    item_start,
                ));
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
            let span = self.current_span();
            let name = self.expect_identifier(context)?;
            if self.match_token(TokenKind::LBracket) {
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
            terminals.push(SpecifyTerminal {
                name: name.into(),
                span: span.extend(self.previous_span()),
            });
            if !self.match_token(TokenKind::Comma) {
                return Ok(terminals);
            }
        }
    }
}
