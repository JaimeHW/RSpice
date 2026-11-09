//! `paramset` declarations (Verilog-AMS LRM 2.4 section 6.4).
//!
//! A paramset is a named module plus parameter overrides that a deck selects
//! as an instance master. It is read here as the module it stands for:
//!
//! ```text
//! paramset smallres res;              module smallres(p, n);
//!   parameter real rr = 1k;             inout p; electrical p; ...
//!   .r = rr * 2;               ==>      parameter real rr = 1k;
//! endparamset                           res #(.r(rr * 2)) paramset_master (p, n);
//!                                     endmodule
//! ```
//!
//! so its parameters, ranges and defaults are an ordinary module's, and an
//! instance may set exactly the paramset's parameters. Several paramsets may
//! share a name (section 6.4.2): each becomes a module of its own, named by
//! [`overload_name`], and the simulator picks one per instance from what the
//! instance sets. A master whose ports are not scalar continuous nets is
//! refused by name.
use super::*;

/// One paramset, read but not yet bound to its module.
pub(super) struct PendingParamset {
    name: SmolStr,
    module: SmolStr,
    attributes: Vec<Attribute>,
    /// The `paramset name module;` header, where synthesized tokens point.
    span: Span,
    /// Declarations copied verbatim into the module body.
    declarations: Vec<Token>,
    /// `.name = expression;` statements: the parameter and its value tokens.
    overrides: Vec<(SmolStr, Vec<Token>)>,
    /// The timescale in effect where the paramset is declared.
    time_scale: crate::time_scale::ModuleTimeScale,
}

/// The instance name the master takes inside the synthesized module.
const MASTER_INSTANCE: &str = "paramset_master";

/// The name of the `ordinal`th (from 1) of several same-named paramsets. The
/// mark cannot occur in a Verilog identifier, so it collides with nothing.
fn overload_name(name: &str, ordinal: usize) -> SmolStr {
    format!("{name}{PARAMSET_OVERLOAD_MARK}{ordinal}").into()
}

/// What separates a paramset's name from its ordinal when several share it.
pub(crate) const PARAMSET_OVERLOAD_MARK: char = '#';

impl Parser<'_> {
    pub(super) fn at_paramset(&self) -> bool {
        self.check_identifier_text("paramset")
    }

    /// Read `paramset name module; ... endparamset`.
    pub(super) fn parse_paramset(
        &mut self,
        attributes: Vec<Attribute>,
    ) -> Result<PendingParamset, ParseError> {
        let start = self.current_span();
        self.advance(); // `paramset`
        let name: SmolStr = self.expect_identifier("paramset name")?.into();
        let module: SmolStr = self.expect_identifier("paramset module name")?.into();
        self.expect(TokenKind::Semicolon)?;
        let span = start.extend(self.previous_span());
        let mut declarations = Vec::new();
        let mut overrides = Vec::new();
        while !self.check_identifier_text("endparamset") {
            if self.at_end() {
                return Err(ParseError::new(
                    ParseErrorKind::Unclosed {
                        construct: format!("paramset '{name}'"),
                        terminator: "'endparamset'",
                        found: self.current().describe(),
                    },
                    start,
                ));
            }
            if self.match_token(TokenKind::Dot) {
                let parameter: SmolStr = self.expect_identifier("module parameter name")?.into();
                self.expect(TokenKind::Assign_)?;
                let value = self.statement_tokens()?;
                if value.is_empty() {
                    return Err(self.error(ParseErrorKind::Expected {
                        expected: format!("a value for module parameter '{parameter}'"),
                        found: self.current().describe(),
                    }));
                }
                self.expect(TokenKind::Semicolon)?;
                overrides.push((parameter, value));
                continue;
            }
            let first = self.attributed_item_start();
            if !matches!(
                first.kind,
                TokenKind::Parameter
                    | TokenKind::Localparam
                    | TokenKind::Aliasparam
                    | TokenKind::Real
                    | TokenKind::Integer
            ) {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: format!("paramset '{name}' item"),
                        found: format!(
                            "{}; a paramset declares parameters and assigns module parameters with `.name = value;`",
                            first.describe()
                        ),
                    },
                    first.span,
                ));
            }
            declarations.extend(self.statement_tokens()?);
            declarations.push(self.current().clone());
            self.expect(TokenKind::Semicolon)?;
        }
        self.advance(); // `endparamset`
        Ok(PendingParamset {
            name,
            module,
            attributes,
            span,
            declarations,
            overrides,
            time_scale: self.time_scale,
        })
    }

    /// The first token after any `(* ... *)` attribute instances here.
    fn attributed_item_start(&self) -> Token {
        let kind = |position: usize| self.tokens.get(position).map(|token| token.kind);
        let mut position = self.pos;
        while kind(position) == Some(TokenKind::LParen)
            && kind(position + 1) == Some(TokenKind::Star)
        {
            position += 2;
            while !(kind(position) == Some(TokenKind::Star)
                && kind(position + 1) == Some(TokenKind::RParen))
            {
                if matches!(kind(position), None | Some(TokenKind::Eof)) {
                    return self.current().clone();
                }
                position += 1;
            }
            position += 2;
        }
        self.tokens
            .get(position)
            .cloned()
            .unwrap_or_else(|| self.current().clone())
    }

    /// The tokens up to (not including) the `;` that ends this statement.
    fn statement_tokens(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        let mut depth = 0usize;
        loop {
            let token = self.current().clone();
            match token.kind {
                TokenKind::Eof => {
                    return Err(self.error(ParseErrorKind::UnexpectedEof));
                }
                TokenKind::Semicolon if depth == 0 => return Ok(tokens),
                TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::LBrace
                | TokenKind::AssignmentPatternStart => depth += 1,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            tokens.push(token);
            self.advance();
        }
    }

    /// Bind each paramset to its module and append the module it stands for.
    pub(super) fn bind_paramsets(
        &self,
        paramsets: Vec<PendingParamset>,
        items: &mut Vec<Item>,
    ) -> Result<(), ParseError> {
        for (index, paramset) in paramsets.iter().enumerate() {
            let same_named = |other: &&PendingParamset| other.name == paramset.name;
            let overloads = paramsets.iter().filter(same_named).count();
            let module_named = |name: &str| {
                items.iter().find_map(|item| match item {
                    Item::Module(module) if module.name == name => Some(module),
                    _ => None,
                })
            };
            if module_named(&paramset.name).is_some() {
                return Err(ParseError::new(
                    ParseErrorKind::Duplicate {
                        kind: "module or paramset".into(),
                        name: paramset.name.to_string(),
                    },
                    paramset.span,
                ));
            }
            let Some(master) = module_named(&paramset.module) else {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: format!("paramset '{}'", paramset.name),
                        found: format!(
                            "its module '{}' is not declared in this source",
                            paramset.module
                        ),
                    },
                    paramset.span,
                ));
            };
            let module_name = if overloads > 1 {
                overload_name(&paramset.name, paramsets[..=index].iter().filter(same_named).count())
            } else {
                paramset.name.clone()
            };
            let tokens = synthesized_module(paramset, &module_name, master)?;
            let mut parser = Parser::new(&tokens);
            parser.time_scale = paramset.time_scale;
            parser.library = self.library.clone();
            let mut module = parser.parse_module()?;
            let mut attributes = paramset.attributes.clone();
            attributes.append(&mut module.attributes);
            module.attributes = attributes;
            module.library = self.library.clone();
            items.push(Item::Module(module));
        }
        Ok(())
    }
}

/// The token stream of the module `paramset` stands for.
fn synthesized_module(
    paramset: &PendingParamset,
    module_name: &str,
    master: &Module,
) -> Result<Vec<Token>, ParseError> {
    let span = paramset.span;
    let refuse = |found: String| {
        ParseError::new(
            ParseErrorKind::UnsupportedConstruct {
                context: format!("paramset '{}'", paramset.name),
                found,
            },
            span,
        )
    };
    let token = |kind: TokenKind| Token::new(kind, span);
    let identifier = |text: &str| Token::with_text(TokenKind::Identifier, span, text);

    let mut ports = Vec::with_capacity(master.ports.len());
    for port in &master.ports {
        let declaration = master
            .port_declarations
            .iter()
            .find(|declaration| declaration.names.contains(&port.name));
        let Some(declaration) = declaration
            .filter(|declaration| declaration.range.is_none() && declaration.net_type.is_none())
        else {
            return Err(refuse(format!(
                "port '{}' of module '{}' is not a scalar continuous port with a direction",
                port.name, master.name
            )));
        };
        let discipline = declaration.discipline.clone().or_else(|| {
            master
                .nets
                .iter()
                .find(|net| net.names.contains(&port.name) && net.range.is_none() && !net.is_ground)
                .and_then(|net| net.discipline.clone())
        });
        let Some(discipline) = discipline else {
            return Err(refuse(format!(
                "port '{}' of module '{}' has no discipline",
                port.name, master.name
            )));
        };
        let direction = match declaration.direction {
            PortDirection::Input => TokenKind::Input,
            PortDirection::Output => TokenKind::Output,
            PortDirection::Inout => TokenKind::Inout,
        };
        ports.push((port.name.clone(), direction, discipline));
    }

    let port_list = |tokens: &mut Vec<Token>| {
        tokens.push(token(TokenKind::LParen));
        for (index, (name, _, _)) in ports.iter().enumerate() {
            if index > 0 {
                tokens.push(token(TokenKind::Comma));
            }
            tokens.push(identifier(name));
        }
        tokens.push(token(TokenKind::RParen));
    };
    let mut tokens = vec![token(TokenKind::Module), identifier(module_name)];
    port_list(&mut tokens);
    tokens.push(token(TokenKind::Semicolon));
    for (name, direction, discipline) in &ports {
        tokens.extend([
            token(*direction),
            identifier(name),
            token(TokenKind::Semicolon),
            identifier(discipline),
            identifier(name),
            token(TokenKind::Semicolon),
        ]);
    }
    tokens.extend(paramset.declarations.iter().cloned());
    tokens.push(identifier(&master.name));
    if !paramset.overrides.is_empty() {
        tokens.extend([token(TokenKind::Hash), token(TokenKind::LParen)]);
        for (index, (parameter, value)) in paramset.overrides.iter().enumerate() {
            if index > 0 {
                tokens.push(token(TokenKind::Comma));
            }
            tokens.extend([
                token(TokenKind::Dot),
                identifier(parameter),
                token(TokenKind::LParen),
            ]);
            tokens.extend(value.iter().cloned());
            tokens.push(token(TokenKind::RParen));
        }
        tokens.push(token(TokenKind::RParen));
    }
    tokens.push(identifier(MASTER_INSTANCE));
    port_list(&mut tokens);
    tokens.extend([
        token(TokenKind::Semicolon),
        token(TokenKind::Endmodule),
        token(TokenKind::Eof),
    ]);
    Ok(tokens)
}
