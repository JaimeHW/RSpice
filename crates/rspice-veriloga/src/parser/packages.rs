//! SystemVerilog packages, imports and nettypes: IEEE 1800-2017 clause 26
//! and section 6.6.7, in the subset this compiler reads.
//!
//! What the names refer to is decided after the whole unit is parsed, by
//! `crate::semantic::packages`: a package may follow the module that imports
//! it, so nothing here looks a name up.

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;

impl Parser<'_> {
    /// `package name ; { package_item } endpackage [ : name ]` (section 26.2).
    pub(super) fn parse_package(&mut self) -> Result<PackageDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // `package`
        let name: SmolStr = self.expect_identifier("package name")?.into();
        self.expect(TokenKind::Semicolon)?;
        let mut package = PackageDecl {
            name: name.clone(),
            imports: Vec::new(),
            nettypes: Vec::new(),
            parameters: Vec::new(),
            span: start,
        };
        loop {
            match self.current().kind {
                TokenKind::Endpackage => break,
                TokenKind::Eof => {
                    return Err(ParseError::new(
                        ParseErrorKind::Unclosed {
                            construct: format!("package '{name}'"),
                            terminator: "'endpackage'",
                            found: self.current().describe(),
                        },
                        start,
                    ));
                }
                TokenKind::Import => package.imports.extend(self.parse_import_declaration()?),
                TokenKind::Nettype => package.nettypes.push(self.parse_nettype_declaration()?),
                TokenKind::Parameter | TokenKind::Localparam => {
                    package.parameters.extend(self.parse_parameter(&[])?);
                }
                TokenKind::Semicolon => {
                    self.advance();
                }
                _ if self.check_identifier_text("__rspice_timescale") => self.parse_time_scale()?,
                _ => return Err(self.package_item_refusal(&name)),
            }
        }
        self.advance(); // `endpackage`
        self.parse_end_label("package", &name)?;
        package.span = start.extend(self.previous_span());
        Ok(package)
    }

    /// The refusal of a package item outside the subset, by what it is.
    fn package_item_refusal(&self, package: &str) -> ParseError {
        let text = self.current().text.as_deref().unwrap_or_default();
        let message = match (self.current().kind, text) {
            (TokenKind::SvReserved, "export") => format!(
                "`export` in package `{package}`: this compiler makes a package's own \
                 declarations visible by `import` and re-exports nothing (IEEE 1800-2017 \
                 section 26.6)"
            ),
            (
                TokenKind::Real
                | TokenKind::Integer
                | TokenKind::Reg
                | TokenKind::String
                | TokenKind::Wire
                | TokenKind::Wreal,
                _,
            )
            | (TokenKind::AmsDigital, "realtime" | "time" | "event")
            | (
                TokenKind::SvReserved,
                "var" | "logic" | "bit" | "byte" | "int" | "shortint" | "longint" | "shortreal"
                | "const" | "static" | "automatic",
            ) => format!(
                "a variable in package `{package}`: package variables are not supported; a \
                 package here holds nettypes, parameters and imports (IEEE 1800-2017 section \
                 26.2)"
            ),
            (TokenKind::Function | TokenKind::Task, _) => format!(
                "a {text} in package `{package}`: package routines are not supported yet; a \
                 package here holds nettypes, parameters and imports"
            ),
            _ => return self.unsupported_current("package item"),
        };
        self.error(ParseErrorKind::SvRefused(message))
    }

    /// `: name` after an `end...` keyword, which must repeat the name.
    fn parse_end_label(&mut self, construct: &str, name: &str) -> Result<(), ParseError> {
        if !self.match_token(TokenKind::Colon) {
            return Ok(());
        }
        let label = self.expect_identifier("end label")?;
        if label != name {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken(format!(
                    "the end label `{label}` of {construct} `{name}` names another {construct} \
                     (IEEE 1800-2017 section 9.3.5)"
                )),
                self.previous_span(),
            ));
        }
        Ok(())
    }

    /// `import p::x, q::* ;` (section 26.3).
    pub(super) fn parse_import_declaration(&mut self) -> Result<Vec<ImportDecl>, ParseError> {
        self.advance(); // `import`
        let mut items = Vec::new();
        loop {
            let start = self.current_span();
            let package: SmolStr = self.expect_identifier("package name")?.into();
            self.expect(TokenKind::ColonColon)?;
            let name = if self.match_token(TokenKind::Star) {
                None
            } else {
                Some(self.expect_identifier("imported name")?.into())
            };
            items.push(ImportDecl {
                package,
                name,
                span: start.extend(self.previous_span()),
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(items)
    }

    /// `nettype data_type name ;` or `nettype nettype_name name ;` (section
    /// 6.6.7). Of the data types, `real` is read; a resolution function and
    /// struct or integral data are refused by name.
    pub(super) fn parse_nettype_declaration(&mut self) -> Result<NettypeDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // `nettype`
        let text = self.current().text.clone().unwrap_or_default();
        let data = match self.current().kind {
            TokenKind::Real => {
                self.advance();
                NettypeData::Real
            }
            TokenKind::Identifier | TokenKind::EscapedIdentifier => {
                NettypeData::Alias(self.parse_type_ref()?)
            }
            TokenKind::SystemIdentifier if text == "$unit" => {
                NettypeData::Alias(self.parse_type_ref()?)
            }
            TokenKind::SvReserved if matches!(text.as_str(), "struct" | "union") => {
                return Err(self.error(ParseErrorKind::SvRefused(format!(
                    "a nettype of `{text}` data: nettypes of struct data are not supported yet; \
                     this compiler reads `nettype real` (IEEE 1800-2017 section 6.6.7)"
                ))));
            }
            TokenKind::SvReserved if matches!(text.as_str(), "shortreal") => {
                return Err(self.error(ParseErrorKind::SvRefused(
                    "a nettype of `shortreal` data is not supported yet; this compiler reads \
                     `nettype real` (IEEE 1800-2017 section 6.6.7)"
                        .into(),
                )));
            }
            TokenKind::Integer
            | TokenKind::Reg
            | TokenKind::Wire
            | TokenKind::Signed
            | TokenKind::Unsigned
            | TokenKind::LBracket => {
                return Err(self.integral_nettype_refusal(&text));
            }
            TokenKind::AmsDigital if text == "time" => {
                return Err(self.integral_nettype_refusal(&text));
            }
            TokenKind::SvReserved
                if matches!(
                    text.as_str(),
                    "logic" | "bit" | "byte" | "shortint" | "int" | "longint" | "enum"
                ) =>
            {
                return Err(self.integral_nettype_refusal(&text));
            }
            _ => {
                return Err(ParseError::expected(
                    "a nettype's data type: `real`, or the name of another nettype (IEEE \
                     1800-2017 section 6.6.7)",
                    self.current().describe(),
                    self.current_span(),
                ));
            }
        };
        let name: SmolStr = self.expect_identifier("nettype name")?.into();
        if self.check(TokenKind::SvReserved) && self.current().text.as_deref() == Some("with") {
            return Err(self.error(ParseErrorKind::SvRefused(format!(
                "`nettype ... {name} with ...`: resolution functions are not supported yet; a \
                 nettype without one takes a single driver (IEEE 1800-2017 section 6.6.7)"
            ))));
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(NettypeDecl {
            name,
            data,
            span: start.extend(self.previous_span()),
        })
    }

    fn integral_nettype_refusal(&self, text: &str) -> ParseError {
        let written = if text.is_empty() {
            self.current().kind.spelling().to_owned()
        } else {
            format!("`{text}`")
        };
        self.error(ParseErrorKind::SvRefused(format!(
            "a nettype of integral data ({written}): nettypes of integral data are not supported; \
             this compiler reads `nettype real` (IEEE 1800-2017 section 6.6.7)"
        )))
    }

    /// `name`, `p::name` or `$unit::name`.
    pub(super) fn parse_type_ref(&mut self) -> Result<TypeRef, ParseError> {
        let start = self.current_span();
        let first: SmolStr = if self.at_unit_scope() {
            self.advance();
            "$unit".into()
        } else {
            self.expect_identifier("type name")?.into()
        };
        if self.match_token(TokenKind::ColonColon) {
            let name = self.expect_identifier("type name")?.into();
            return Ok(TypeRef {
                package: Some(first),
                name,
                span: start.extend(self.previous_span()),
            });
        }
        if first == "$unit" {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "'::' after `$unit`".into(),
                found: self.current().describe(),
            }));
        }
        Ok(TypeRef {
            package: None,
            name: first,
            span: start,
        })
    }

    /// Whether the cursor is at `$unit`.
    pub(super) fn at_unit_scope(&self) -> bool {
        self.check(TokenKind::SystemIdentifier) && self.current().text.as_deref() == Some("$unit")
    }

    /// Whether `name ::` starts at the cursor: a scoped name (section 26.3).
    pub(super) fn scoped_name_follows(&self) -> bool {
        (matches!(
            self.current().kind,
            TokenKind::Identifier | TokenKind::EscapedIdentifier
        ) || self.at_unit_scope())
            && self.peek_is(TokenKind::ColonColon)
    }

    /// `p::name` in an expression: the name, as one identifier spelled
    /// `p::name`, which no declaration can spell, for the package pass to
    /// bind.
    pub(super) fn parse_scoped_identifier(&mut self) -> Result<Expression, ParseError> {
        let type_ref = self.parse_type_ref()?;
        Ok(Expression::Identifier(Identifier {
            name: type_ref.written().into(),
            span: type_ref.span,
        }))
    }

    /// Whether the cursor starts a net declaration whose type is a nettype
    /// name: `rnet a, b;`, `p::rnet y = x;`. A module instance has the same
    /// opening, and its parentheses tell it apart.
    pub(super) fn at_typed_net_declaration(&self) -> bool {
        let kind = |offset: usize| self.tokens.get(self.pos + offset).map(|token| token.kind);
        let name = |offset: usize| {
            matches!(
                kind(offset),
                Some(TokenKind::Identifier | TokenKind::EscapedIdentifier)
            )
        };
        let declared = if self.scoped_name_follows() {
            3
        } else if name(0) {
            1
        } else {
            return false;
        };
        if !name(declared) {
            return false;
        }
        let mut at = declared + 1;
        // Unpacked dimensions follow a declared name; an instance array's
        // range is followed by its connections.
        while kind(at) == Some(TokenKind::LBracket) {
            let mut depth = 0_usize;
            loop {
                match kind(at) {
                    Some(TokenKind::LBracket) => depth += 1,
                    Some(TokenKind::RBracket) => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    Some(_) => {}
                    None => return false,
                }
                at += 1;
            }
            at += 1;
        }
        matches!(
            kind(at),
            Some(TokenKind::Semicolon | TokenKind::Comma | TokenKind::Assign_)
        )
    }

    /// `nettype_name name [= expression] {, ...} ;` (section 6.7.2).
    pub(super) fn parse_typed_net_declaration(&mut self) -> Result<DigitalNetDecl, ParseError> {
        let start = self.current_span();
        let nettype = self.parse_type_ref()?;
        let items = self.parse_digital_decl_items()?;
        Ok(DigitalNetDecl {
            kind: DigitalNetKind::Nettype,
            signedness: Signedness::Unsigned,
            range: None,
            items,
            nettype: Some(nettype),
            span: start.extend(self.previous_span()),
        })
    }

    /// A port's nettype in a SystemVerilog module (section 23.2.2): the
    /// type name, when one stands between the direction and the port name.
    pub(super) fn parse_port_nettype(&mut self) -> Result<Option<TypeRef>, ParseError> {
        if self.language != SourceLanguage::SystemVerilog {
            return Ok(None);
        }
        let kind = |offset: usize| self.tokens.get(self.pos + offset).map(|token| token.kind);
        let declared = if self.scoped_name_follows() {
            3
        } else if matches!(
            kind(0),
            Some(TokenKind::Identifier | TokenKind::EscapedIdentifier)
        ) {
            1
        } else {
            return Ok(None);
        };
        if !matches!(
            kind(declared),
            Some(TokenKind::Identifier | TokenKind::EscapedIdentifier)
        ) {
            return Ok(None);
        }
        self.parse_type_ref().map(Some)
    }

    /// The identifiers a SystemVerilog module's tokens name, from `first` to
    /// the cursor, a scoped name as one identifier `p::name`; see
    /// [`Module::identifier_uses`].
    pub(super) fn identifier_uses(&self, first: usize) -> Vec<(SmolStr, Span)> {
        let tokens = &self.tokens[first..self.pos];
        let name = |index: usize| -> Option<SmolStr> {
            let token = tokens.get(index)?;
            match token.kind {
                TokenKind::Identifier | TokenKind::EscapedIdentifier => {
                    let text = token.text.as_deref()?;
                    Some(text.strip_prefix('\\').unwrap_or(text).into())
                }
                TokenKind::SystemIdentifier if token.text.as_deref() == Some("$unit") => {
                    Some("$unit".into())
                }
                _ => None,
            }
        };
        let kind = |index: usize| tokens.get(index).map(|token| token.kind);
        let mut uses = Vec::new();
        let mut index = 0;
        while index < tokens.len() {
            let Some(first) = name(index) else {
                index += 1;
                continue;
            };
            let after_dot = index > 0 && kind(index - 1) == Some(TokenKind::Dot);
            if kind(index + 1) == Some(TokenKind::ColonColon)
                && let Some(member) = name(index + 2)
            {
                uses.push((
                    format!("{first}::{member}").into(),
                    tokens[index].span.extend(tokens[index + 2].span),
                ));
                index += 3;
                continue;
            }
            if !after_dot && first != "$unit" {
                uses.push((first, tokens[index].span));
            }
            index += 1;
        }
        uses
    }
}
