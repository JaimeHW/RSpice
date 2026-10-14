//! `config` declarations and the library markers of a library map
//! (IEEE 1364-2005 clause 13).
//!
//! None of a configuration's words is reserved here: `config` opens one only
//! at file scope, and `design`, `instance`, `cell`, `liblist`, `use` and
//! `endconfig` are read by their text inside it. A Verilog-A source that names
//! a net `cell` keeps compiling.

use super::Parser;
use crate::ast::{ConfigDecl, ConfigExpansion, ConfigRule, ConfigSelection, LibraryCell};
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use smol_str::SmolStr;

/// The library a source with no library marker is read into.
pub(crate) const WORK_LIBRARY: &str = "work";

impl Parser<'_> {
    /// `__rspice_library (name);`: the sources that follow were read into
    /// library `name`. A multi-file preparation writes one ahead of each file
    /// a library map placed.
    pub(super) fn parse_library_marker(&mut self) -> Result<(), ParseError> {
        self.advance();
        self.expect(TokenKind::LParen)?;
        let library = self.expect_identifier("library name")?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;
        self.library = Some(library.into());
        Ok(())
    }

    /// `__rspice_libraries (a, b, ...);`: the libraries a library map
    /// declares, in declaration order, which is the search order of an
    /// instance no configuration rule selects (IEEE 1364-2005 section 13.5.1).
    pub(super) fn parse_library_order_marker(&mut self) -> Result<(), ParseError> {
        self.advance();
        self.expect(TokenKind::LParen)?;
        let mut order = Vec::new();
        loop {
            order.push(SmolStr::from(self.expect_identifier("library name")?));
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;
        self.library_order = Some(order);
        Ok(())
    }

    /// Whether the cursor opens a `config` declaration: `config name ;`.
    pub(super) fn at_config_declaration(&self) -> bool {
        self.check_identifier_text("config")
            && self.tokens.get(self.pos + 1).is_some_and(|token| {
                matches!(
                    token.kind,
                    TokenKind::Identifier | TokenKind::EscapedIdentifier
                )
            })
            && self
                .tokens
                .get(self.pos + 2)
                .is_some_and(|token| token.kind == TokenKind::Semicolon)
    }

    /// `config_declaration` (IEEE 1364-2005 section 13.3.1, Syntax 13-4).
    pub(super) fn parse_config(&mut self) -> Result<ConfigDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // `config`
        let name: SmolStr = self.expect_identifier("configuration name")?.into();
        self.expect(TokenKind::Semicolon)?;
        if self.check(TokenKind::Localparam) {
            // IEEE 1800-2017 section 33.4.1 adds parameters to a configuration;
            // IEEE 1364-2005's grammar has none.
            return Err(self.error(ParseErrorKind::UnsupportedConstruct {
                context: "configuration".into(),
                found: "a `localparam` in a configuration (a SystemVerilog form)".into(),
            }));
        }
        if !self.check_identifier_text("design") {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "`design` statement".into(),
                found: format!("{:?}", self.current().kind),
            }));
        }
        self.advance();
        let mut design = Vec::new();
        while !self.check(TokenKind::Semicolon) && !self.at_end() {
            design.push(self.parse_library_cell("design cell")?);
        }
        self.expect(TokenKind::Semicolon)?;
        let mut rules = Vec::new();
        while !self.check_identifier_text("endconfig") {
            if self.at_end() {
                return Err(self.error(ParseErrorKind::UnexpectedEof));
            }
            rules.push(self.parse_config_rule()?);
        }
        self.advance(); // `endconfig`
        if self.match_token(TokenKind::Colon) {
            // IEEE 1800-2017's `endconfig : name`.
            let label = self.expect_identifier("configuration name")?;
            if label != name {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken(format!(
                        "`endconfig : {label}` closes configuration `{name}`"
                    )),
                    self.previous_span(),
                ));
            }
        }
        Ok(ConfigDecl {
            name,
            library: self
                .library
                .clone()
                .unwrap_or_else(|| SmolStr::new_static(WORK_LIBRARY)),
            design,
            rules,
            span: start.extend(self.previous_span()),
        })
    }

    /// `[library.]cell`.
    fn parse_library_cell(&mut self, context: &str) -> Result<LibraryCell, ParseError> {
        let start = self.current_span();
        let first: SmolStr = self.expect_identifier(context)?.into();
        let (library, cell) = if self.match_token(TokenKind::Dot) {
            (Some(first), self.expect_identifier(context)?.into())
        } else {
            (None, first)
        };
        Ok(LibraryCell {
            library,
            cell,
            span: start.extend(self.previous_span()),
        })
    }

    /// `config_rule_statement` (IEEE 1364-2005 section 13.3.1).
    fn parse_config_rule(&mut self) -> Result<ConfigRule, ParseError> {
        let start = self.current_span();
        let selection = if self.match_token(TokenKind::Default) {
            ConfigSelection::Default
        } else if self.check_identifier_text("instance") {
            self.advance();
            let mut path = vec![SmolStr::from(self.expect_identifier("instance name")?)];
            while self.match_token(TokenKind::Dot) {
                path.push(self.expect_identifier("instance name")?.into());
            }
            ConfigSelection::Instance(path)
        } else if self.check_identifier_text("cell") {
            self.advance();
            ConfigSelection::Cell(self.parse_library_cell("cell name")?)
        } else {
            return Err(self.error(ParseErrorKind::UnexpectedToken(format!(
                "{:?} in a configuration, whose rules begin with `default`, `instance` or `cell`",
                self.current().kind
            ))));
        };
        let expansion = if self.check_identifier_text("liblist") {
            self.advance();
            let mut libraries = Vec::new();
            while !self.check(TokenKind::Semicolon) && !self.at_end() {
                libraries.push(SmolStr::from(self.expect_identifier("library name")?));
            }
            ConfigExpansion::Liblist(libraries)
        } else if self.check_identifier_text("use") {
            if selection == ConfigSelection::Default {
                // Section 13.3.1.2: the default clause's expansion is a liblist.
                return Err(self.error(ParseErrorKind::UnexpectedToken(
                    "`use` in a `default` clause, which takes a `liblist`".into(),
                )));
            }
            self.advance();
            let cell = self.parse_library_cell("cell name")?;
            let configuration = if self.match_token(TokenKind::Colon) {
                if !self.check_identifier_text("config") {
                    return Err(self.error(ParseErrorKind::Expected {
                        expected: "`config`".into(),
                        found: format!("{:?}", self.current().kind),
                    }));
                }
                self.advance();
                true
            } else {
                false
            };
            ConfigExpansion::Use {
                cell,
                configuration,
            }
        } else {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "`liblist` or `use`".into(),
                found: format!("{:?}", self.current().kind),
            }));
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(ConfigRule {
            selection,
            expansion,
            span: start.extend(self.previous_span()),
        })
    }
}
