//! Verilog-A/AMS Parser
//!
//! Recursive descent parser for Verilog-A LRM 2.4 syntax.
//! Produces AST from token stream.

use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::{Token, TokenKind};
use crate::numeric_literal::parse_numeric_literal;
use crate::source::Span;
use smol_str::SmolStr;

/// Parser for Verilog-A/AMS
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse a complete source file
    pub fn parse(&mut self) -> Result<SourceFile, ParseError> {
        let start_span = self.current_span();
        let mut items = Vec::new();

        while !self.at_end() {
            // Skip directives for now (handled by preprocessor)
            if self.check(TokenKind::Directive) {
                self.skip_directive()?;
                continue;
            }

            // Attribute instances may precede any top-level item
            let attributes = self.parse_attributes()?;

            if self.check_identifier_text("__rspice_default_transition") {
                if !attributes.is_empty() {
                    return Err(self.error(ParseErrorKind::UnexpectedToken(
                        "attributes before `default_transition`".to_string(),
                    )));
                }
                items.push(Item::DefaultTransition(
                    self.parse_default_transition_directive()?,
                ));
            } else if self.check(TokenKind::Module) || self.check(TokenKind::Macromodule) {
                let mut module = self.parse_module()?;
                let mut combined_attributes = attributes;
                combined_attributes.append(&mut module.attributes);
                module.attributes = combined_attributes;
                items.push(Item::Module(module));
            } else if self.check(TokenKind::Discipline) {
                items.push(Item::Discipline(self.parse_discipline()?));
            } else if self.check(TokenKind::Nature) {
                items.push(Item::Nature(self.parse_nature()?));
            } else if self.check(TokenKind::Eof) {
                break;
            } else if Self::is_digital_declaration_keyword(self.current().kind) {
                // `connectmodule`, `primitive`, `always`, ... at file scope.
                return Err(self.unsupported_ams_construct());
            } else {
                return Err(self.error(ParseErrorKind::UnexpectedToken(format!(
                    "{:?}",
                    self.current().kind
                ))));
            }
        }

        let end_span = self.previous_span();
        Ok(SourceFile {
            items,
            span: start_span.extend(end_span),
        })
    }

    fn check_identifier_text(&self, expected: &str) -> bool {
        self.check(TokenKind::Identifier) && self.current().text.as_deref() == Some(expected)
    }

    fn parse_default_transition_directive(
        &mut self,
    ) -> Result<DefaultTransitionDirective, ParseError> {
        let start = self.current_span();
        self.advance();
        self.expect(TokenKind::LParen)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(DefaultTransitionDirective {
            value,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse a module definition
    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'module' or 'macromodule'

        let name = self.expect_identifier("module name")?;
        let mut module = Module::new(name, start);

        // Optional port list
        if self.check(TokenKind::LParen) {
            self.parse_port_list(&mut module)?;
        }
        module.attributes = self.parse_attributes()?;
        self.expect(TokenKind::Semicolon)?;

        // Module body
        while !self.check(TokenKind::Endmodule) && !self.at_end() {
            self.parse_module_item(&mut module)?;
        }

        self.expect(TokenKind::Endmodule)?;
        module.span = start.extend(self.previous_span());
        Ok(module)
    }

    /// Parse port list: (port1, port2, ...) or ANSI style
    /// (inout electrical p, inout electrical n)
    fn parse_port_list(&mut self, module: &mut Module) -> Result<(), ParseError> {
        self.expect(TokenKind::LParen)?;

        // Direction/discipline carried forward for ANSI-style bare names
        let mut ansi_context: Option<(PortDirection, Option<SmolStr>)> = None;

        if !self.check(TokenKind::RParen) {
            loop {
                let span = self.current_span();

                let direction = match self.current().kind {
                    TokenKind::Input => {
                        self.advance();
                        Some(PortDirection::Input)
                    }
                    TokenKind::Output => {
                        self.advance();
                        Some(PortDirection::Output)
                    }
                    TokenKind::Inout => {
                        self.advance();
                        Some(PortDirection::Inout)
                    }
                    _ => None,
                };

                if let Some(direction) = direction {
                    // ANSI style: direction [discipline] name
                    let discipline: Option<SmolStr> = if self.is_discipline_keyword()
                        || (self.check(TokenKind::Identifier)
                            && self.peek_is(TokenKind::Identifier))
                    {
                        Some(self.expect_identifier("discipline")?.into())
                    } else {
                        None
                    };

                    let name: SmolStr = self.expect_identifier("port name")?.into();
                    module.ports.push(Port {
                        name: name.clone(),
                        span,
                    });
                    module.port_declarations.push(PortDeclaration {
                        direction,
                        discipline: discipline.clone(),
                        names: vec![name],
                        span: span.extend(self.previous_span()),
                    });
                    ansi_context = Some((direction, discipline));
                } else {
                    let name: SmolStr = self.expect_identifier("port name")?.into();
                    module.ports.push(Port {
                        name: name.clone(),
                        span,
                    });
                    // In an ANSI list, bare names inherit the direction and
                    // discipline of the preceding declared port.
                    if let Some((direction, discipline)) = &ansi_context {
                        module.port_declarations.push(PortDeclaration {
                            direction: *direction,
                            discipline: discipline.clone(),
                            names: vec![name],
                            span: span.extend(self.previous_span()),
                        });
                    }
                }

                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        Ok(())
    }

    /// Parse one or more attribute instances: (* name = value, ... *)
    ///
    /// Attribute values are restricted to simple constants (string, number,
    /// identifier, optionally negated or parenthesized). A full expression
    /// parser would consume the closing `*)` as a multiplication.
    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, ParseError> {
        let mut attrs = Vec::new();

        while self.check(TokenKind::LParen) && self.peek_is(TokenKind::Star) {
            self.advance(); // consume '('
            self.advance(); // consume '*'

            loop {
                // Allow an empty instance or trailing comma: (* *)
                if self.check(TokenKind::Star) && self.peek_is(TokenKind::RParen) {
                    break;
                }

                let span = self.current_span();
                let name = self.expect_identifier("attribute name")?;
                let value = if self.match_token(TokenKind::Assign_) {
                    Some(self.parse_attribute_value()?)
                } else {
                    None
                };

                attrs.push(Attribute {
                    name: name.into(),
                    value,
                    span,
                });

                if self.match_token(TokenKind::Comma) {
                    continue;
                }
                // Xyce's canonical compact-model sources also use the
                // Verilog-AMS attribute form with whitespace-separated
                // entries: `(* name=value other=value *)`.
                if self.check(TokenKind::Identifier) {
                    continue;
                }
                break;
            }

            self.expect(TokenKind::Star)?;
            self.expect(TokenKind::RParen)?;
        }

        Ok(attrs)
    }

    /// Parse a restricted attribute value: constant primary with optional sign
    fn parse_attribute_value(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        if self.match_token(TokenKind::Minus) {
            let operand = self.parse_attribute_value()?;
            return Ok(Expression::Unary(UnaryExpr {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
                span: start.extend(self.previous_span()),
            }));
        }
        match self.current().kind {
            TokenKind::IntegerLiteral
            | TokenKind::RealLiteral
            | TokenKind::StringLiteral
            | TokenKind::Identifier => self.parse_primary(),
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(self.error(ParseErrorKind::InvalidExpression)),
        }
    }

    /// Parse a module item (declaration or statement)
    fn parse_module_item(&mut self, module: &mut Module) -> Result<(), ParseError> {
        // Attribute instances may precede any declaration
        let attributes = self.parse_attributes()?;

        match self.current().kind {
            TokenKind::Input | TokenKind::Output | TokenKind::Inout => {
                let decl = self.parse_port_declaration()?;
                module.port_declarations.push(decl);
            }
            TokenKind::Parameter => {
                let params = self.parse_parameter(&attributes)?;
                module.parameters.extend(params);
            }
            TokenKind::Localparam => {
                let params = self.parse_parameter(&attributes)?;
                module.localparams.extend(params);
            }
            TokenKind::Aliasparam => {
                let alias = self.parse_aliasparam()?;
                module.aliasparams.push(alias);
            }
            TokenKind::Real | TokenKind::Integer | TokenKind::String => {
                let var = self.parse_variable_decl()?;
                module.variables.push(var);
            }
            TokenKind::Electrical | TokenKind::Voltage | TokenKind::Current => {
                let net = self.parse_net_decl()?;
                module.nets.push(net);
            }
            TokenKind::Identifier => {
                self.parse_identifier_module_item(module)?;
            }
            TokenKind::Ground => {
                let net = self.parse_ground_decl()?;
                module.nets.push(net);
            }
            TokenKind::Analog => {
                let next_kind = self.tokens.get(self.pos + 1).map(|t| t.kind);
                match next_kind {
                    Some(TokenKind::Function) => {
                        let func = self.parse_analog_function()?;
                        module.functions.push(func);
                    }
                    Some(TokenKind::Initial) => {
                        self.advance(); // consume 'analog'
                        self.advance(); // consume 'initial'
                        let block = self.parse_analog_statement_as_block()?;
                        Self::merge_analog_block(&mut module.analog_initial, block);
                    }
                    Some(TokenKind::Final) => {
                        self.advance(); // consume 'analog'
                        self.advance(); // consume 'final'
                        let block = self.parse_analog_statement_as_block()?;
                        Self::merge_analog_block(&mut module.analog_final, block);
                    }
                    _ => {
                        let block = self.parse_analog_block()?;
                        Self::merge_analog_block(&mut module.analog_block, block);
                    }
                }
            }
            // Generate regions lex, so they must be refused by name. Falling
            // through to the unrecognized-module-item error would blame the
            // keyword without saying that the construct itself is the gap.
            TokenKind::Genvar | TokenKind::Generate | TokenKind::Endgenerate => {
                return Err(self.unsupported_generate());
            }
            // The digital half of Verilog-AMS. `analog initial` and
            // `analog final` are handled by the `analog` arm above, so a bare
            // `initial` reaching here is the 1364 procedural block.
            kind if Self::is_digital_declaration_keyword(kind) => {
                return Err(self.unsupported_ams_construct());
            }
            _ => {
                return Err(self.unsupported_current("module item"));
            }
        }
        Ok(())
    }

    /// Merge an analog block into an optional slot (the LRM allows multiple
    /// analog blocks; they execute in declaration order).
    fn merge_analog_block(slot: &mut Option<AnalogBlock>, block: AnalogBlock) {
        match slot {
            Some(existing) => {
                existing.span = existing.span.extend(block.span);
                existing.statements.extend(block.statements);
            }
            None => *slot = Some(block),
        }
    }

    /// Parse a single analog statement and wrap it in a block
    fn parse_analog_statement_as_block(&mut self) -> Result<AnalogBlock, ParseError> {
        let start = self.current_span();
        let stmt = self.parse_analog_statement()?;
        let statements = Self::flatten_block_statement(stmt);
        Ok(AnalogBlock {
            statements,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse a module item that starts with a plain identifier.
    ///
    /// Distinguishes user-defined discipline net declarations
    /// (`thermal t1, t2;`) and named branch declarations
    /// (`branch (a, b) br;`) from other constructs.
    fn parse_identifier_module_item(&mut self, module: &mut Module) -> Result<(), ParseError> {
        let first_text = self.current().text.clone().unwrap_or_default();

        // Named branch declaration: branch (a [, b]) name1, name2 ;
        if first_text == "branch" && self.peek_is(TokenKind::LParen) {
            let branches = self.parse_branch_decl()?;
            module.branches.extend(branches);
            return Ok(());
        }

        // Net declaration with a user-defined discipline: `ident ident [, ident]* ;`
        // An instance declaration would have parentheses before the semicolon.
        if self.peek_is(TokenKind::Identifier) {
            let mut lookahead = self.pos + 1;
            let mut is_net_decl = false;
            while let Some(tok) = self.tokens.get(lookahead) {
                match tok.kind {
                    TokenKind::Identifier => lookahead += 1,
                    TokenKind::Comma => lookahead += 1,
                    TokenKind::Semicolon => {
                        is_net_decl = true;
                        break;
                    }
                    _ => break,
                }
            }
            if is_net_decl {
                let net = self.parse_net_decl()?;
                module.nets.push(net);
                return Ok(());
            }
        }

        // Module instance declaration:
        // `model [#(...)] instance (...)[, instance (...)];`
        // The parameter override belongs to every instance in the declaration.
        if self.peek_is(TokenKind::Identifier) || self.peek_is(TokenKind::Hash) {
            let instances = self.parse_module_instances()?;
            module.instances.extend(instances);
            return Ok(());
        }

        Err(self.unsupported_current("module item"))
    }

    fn parse_module_instances(&mut self) -> Result<Vec<ModuleInstance>, ParseError> {
        let declaration_start = self.current_span();
        let module: SmolStr = self.expect_identifier("instantiated module name")?.into();
        let parameters = if self.match_token(TokenKind::Hash) {
            self.parse_instance_parameter_overrides()?
        } else {
            Vec::new()
        };
        let mut instances = Vec::new();
        loop {
            let instance_start = self.current_span();
            let name: SmolStr = self.expect_identifier("module instance name")?.into();
            let connections = self.parse_instance_connections()?;
            instances.push(ModuleInstance {
                module: module.clone(),
                name,
                connections,
                parameters: parameters.clone(),
                span: instance_start.extend(self.previous_span()),
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        let declaration_end = self.previous_span();
        for instance in &mut instances {
            instance.span = declaration_start.extend(declaration_end);
        }
        Ok(instances)
    }

    fn parse_instance_parameter_overrides(&mut self) -> Result<Vec<ParameterOverride>, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut parameters = Vec::new();
        if !self.check(TokenKind::RParen) {
            let named = self.check(TokenKind::Dot);
            loop {
                let start = self.current_span();
                let (name, value) = if named {
                    self.expect(TokenKind::Dot)?;
                    let name: SmolStr = self.expect_identifier("parameter override name")?.into();
                    self.expect(TokenKind::LParen)?;
                    let value = self.parse_expression()?;
                    self.expect(TokenKind::RParen)?;
                    (Some(name), value)
                } else {
                    (None, self.parse_expression()?)
                };
                parameters.push(ParameterOverride {
                    name,
                    value,
                    span: start.extend(self.previous_span()),
                });
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
                if self.check(TokenKind::Dot) != named {
                    return Err(self.unsupported_current(
                        "mixed named and ordered module parameter overrides",
                    ));
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        Ok(parameters)
    }

    fn parse_instance_connections(&mut self) -> Result<Vec<Connection>, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut connections = Vec::new();
        if !self.check(TokenKind::RParen) {
            let named = self.check(TokenKind::Dot);
            loop {
                let start = self.current_span();
                let connection = if named {
                    self.expect(TokenKind::Dot)?;
                    let port: SmolStr = self.expect_identifier("module port name")?.into();
                    self.expect(TokenKind::LParen)?;
                    let signal = if self.check(TokenKind::RParen) {
                        None
                    } else {
                        Some(self.parse_expression()?)
                    };
                    self.expect(TokenKind::RParen)?;
                    Connection::Named {
                        port,
                        signal,
                        span: start.extend(self.previous_span()),
                    }
                } else {
                    let signal = if self.check(TokenKind::Comma) || self.check(TokenKind::RParen) {
                        None
                    } else {
                        Some(self.parse_expression()?)
                    };
                    Connection::Ordered {
                        signal,
                        span: start.extend(self.previous_span()),
                    }
                };
                connections.push(connection);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
                if self.check(TokenKind::Dot) != named {
                    return Err(
                        self.unsupported_current("mixed named and ordered module port connections")
                    );
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        Ok(connections)
    }

    /// Parse a named branch declaration: branch (a [, b]) name1 [, name2] ;
    fn parse_branch_decl(&mut self) -> Result<Vec<BranchDecl>, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'branch'
        self.expect(TokenKind::LParen)?;

        let pos = self.expect_identifier("branch terminal")?;
        let neg = if self.match_token(TokenKind::Comma) {
            Some(self.expect_identifier("branch terminal")?)
        } else {
            None
        };
        self.expect(TokenKind::RParen)?;

        let mut branches = Vec::new();
        loop {
            let name = self.expect_identifier("branch name")?;
            branches.push(BranchDecl {
                name: name.into(),
                pos: pos.clone().into(),
                // Single-terminal branch references the global reference node
                neg: neg.clone().map(SmolStr::from).unwrap_or_default(),
                span: start.extend(self.previous_span()),
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(branches)
    }

    /// Parse port declaration: `input/output/inout [discipline] names;`
    fn parse_port_declaration(&mut self) -> Result<PortDeclaration, ParseError> {
        let start = self.current_span();
        let direction = match self.current().kind {
            TokenKind::Input => {
                self.advance();
                PortDirection::Input
            }
            TokenKind::Output => {
                self.advance();
                PortDirection::Output
            }
            TokenKind::Inout => {
                self.advance();
                PortDirection::Inout
            }
            _ => return Err(self.error(ParseErrorKind::InvalidPort)),
        };

        // Optional discipline. User-defined disciplines are identifiers, so
        // distinguish `inout foo bar;` from `inout foo, bar;` by requiring
        // two adjacent identifiers, matching ANSI port-list parsing.
        let discipline = if self.is_discipline_keyword()
            || (self.check(TokenKind::Identifier) && self.peek_is(TokenKind::Identifier))
        {
            Some(self.expect_identifier("discipline")?)
        } else {
            None
        };

        // Port names
        let mut names = Vec::new();
        loop {
            names.push(self.expect_identifier("port name")?.into());
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(PortDeclaration {
            direction,
            discipline: discipline.map(|s| s.into()),
            names,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse a parameter declaration (one or more comma-separated assignments)
    fn parse_parameter(
        &mut self,
        attributes: &[Attribute],
    ) -> Result<Vec<ParameterDecl>, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'parameter' or 'localparam'

        // Optional type
        let (param_type, type_is_explicit) = if self.match_token(TokenKind::Real) {
            (ParamType::Real, true)
        } else if self.match_token(TokenKind::Integer) {
            (ParamType::Integer, true)
        } else if self.match_token(TokenKind::String) {
            (ParamType::String, true)
        } else {
            (ParamType::Real, false) // language default for scalar parameters
        };

        // Pull desc/units out of any preceding attribute instance
        let (description, units) = Self::extract_param_attributes(attributes);

        let mut decls = Vec::new();
        loop {
            let item_start = if decls.is_empty() {
                start
            } else {
                self.current_span()
            };
            let name = self.expect_identifier("parameter name")?;

            let mut dimensions = Vec::new();
            while self.match_token(TokenKind::LBracket) {
                let dim_start = self.previous_span();
                let dim_lo = self.parse_expression()?;
                self.expect(TokenKind::Colon)?;
                let dim_hi = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                dimensions.push(ArrayDimension {
                    start: dim_lo,
                    end: dim_hi,
                    span: dim_start.extend(self.previous_span()),
                });
            }

            // Optional default value
            let default = if self.match_token(TokenKind::Assign_) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            // Range constraints: any sequence of `from <bound>` and
            // `exclude <value>` clauses
            let mut bounds = Vec::new();
            let mut exclude = Vec::new();
            let range_start = self.current_span();
            loop {
                if self.match_token(TokenKind::From) {
                    bounds.push(self.parse_range_bound()?);
                } else if self.match_token(TokenKind::Exclude) {
                    exclude.push(self.parse_expression()?);
                } else {
                    break;
                }
            }

            let range = if bounds.is_empty() && exclude.is_empty() {
                None
            } else {
                Some(ParameterRange {
                    bounds,
                    exclude,
                    span: range_start.extend(self.previous_span()),
                })
            };

            decls.push(ParameterDecl {
                param_type,
                type_is_explicit,
                name: name.into(),
                dimensions,
                default,
                range,
                units: units.clone(),
                description: description.clone(),
                attributes: attributes.to_vec(),
                span: item_start.extend(self.previous_span()),
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(decls)
    }

    /// Parse a parameter alias declaration: aliasparam alias = target ;
    ///
    /// The right-hand side is strictly a parameter identifier (not an
    /// expression); the semantic phase validates that the target exists.
    fn parse_aliasparam(&mut self) -> Result<AliasParamDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'aliasparam'

        let alias = self.expect_identifier("alias name")?;
        self.expect(TokenKind::Assign_)?;
        let target = self.expect_identifier("target parameter name")?;
        self.expect(TokenKind::Semicolon)?;

        Ok(AliasParamDecl {
            alias: alias.into(),
            target: target.into(),
            span: start.extend(self.previous_span()),
        })
    }

    /// Extract `desc`/`units` strings from an attribute list
    fn extract_param_attributes(attributes: &[Attribute]) -> (Option<SmolStr>, Option<SmolStr>) {
        let mut description = None;
        let mut units = None;
        for attr in attributes {
            let value = match &attr.value {
                Some(Expression::StringLit(s)) => Some(s.value.clone()),
                _ => None,
            };
            match attr.name.as_str() {
                "desc" | "info" => description = value,
                "units" | "unit" => units = value,
                _ => {}
            }
        }
        (description, units)
    }

    /// Parse a single range bound: `[lb:ub]`, `(lb:ub)`, `[lb:ub)`, `(lb:ub]`
    fn parse_range_bound(&mut self) -> Result<RangeBound, ParseError> {
        let start = self.current_span();

        let lower_inclusive = if self.match_token(TokenKind::LBracket) {
            true
        } else if self.match_token(TokenKind::LParen) {
            false
        } else {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "'[' or '('".to_string(),
                found: format!("{:?}", self.current().kind),
            }));
        };

        let lower = if self.check(TokenKind::Minus) && self.peek_is(TokenKind::Inf) {
            self.advance();
            self.advance();
            None
        } else if self.match_token(TokenKind::Inf) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect(TokenKind::Colon)?;

        let upper = if self.match_token(TokenKind::Inf) {
            None
        } else if self.check(TokenKind::Plus) && self.peek_is(TokenKind::Inf) {
            self.advance();
            self.advance();
            None
        } else {
            Some(self.parse_expression()?)
        };

        let upper_inclusive = if self.match_token(TokenKind::RBracket) {
            true
        } else if self.match_token(TokenKind::RParen) {
            false
        } else {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "']' or ')'".to_string(),
                found: format!("{:?}", self.current().kind),
            }));
        };

        Ok(RangeBound {
            lower,
            lower_inclusive,
            upper,
            upper_inclusive,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse variable declaration: real/integer var1[dims][= init], var2, ...;
    fn parse_variable_decl(&mut self) -> Result<VariableDecl, ParseError> {
        let start = self.current_span();
        let var_type = if self.match_token(TokenKind::Real) {
            VarType::Real
        } else if self.match_token(TokenKind::Integer) {
            VarType::Integer
        } else if self.match_token(TokenKind::String) {
            VarType::String
        } else {
            return Err(self.error(ParseErrorKind::Expected {
                expected: "type".to_string(),
                found: format!("{:?}", self.current().kind),
            }));
        };

        let mut items = Vec::new();
        loop {
            let item_span = self.current_span();
            let name = self.expect_identifier("variable name")?;

            // Optional array dimensions: name[lo:hi] (possibly several)
            let mut dimensions = Vec::new();
            while self.match_token(TokenKind::LBracket) {
                let dim_start = self.previous_span();
                let dim_lo = self.parse_expression()?;
                self.expect(TokenKind::Colon)?;
                let dim_hi = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                dimensions.push(ArrayDimension {
                    start: dim_lo,
                    end: dim_hi,
                    span: dim_start.extend(self.previous_span()),
                });
            }

            // Optional initializer
            let init = if self.match_token(TokenKind::Assign_) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            items.push(VariableItem {
                name: name.into(),
                dimensions,
                init,
                span: item_span.extend(self.previous_span()),
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(VariableDecl {
            var_type,
            items,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse net declaration: discipline node1, node2;
    fn parse_net_decl(&mut self) -> Result<NetDecl, ParseError> {
        let start = self.current_span();
        let discipline = self.expect_identifier("discipline")?;

        let mut names = Vec::new();
        loop {
            names.push(self.expect_identifier("node name")?.into());
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(NetDecl {
            discipline: discipline.into(),
            names,
            is_ground: false,
            is_internal: false, // Will be determined after parsing completes
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse ground declaration
    fn parse_ground_decl(&mut self) -> Result<NetDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'ground'

        let mut names = Vec::new();
        loop {
            names.push(self.expect_identifier("ground name")?.into());
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(NetDecl {
            discipline: "electrical".into(),
            names,
            is_ground: true,
            is_internal: false, // Ground nodes are never internal
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse analog block
    fn parse_analog_block(&mut self) -> Result<AnalogBlock, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'analog'

        let stmt = self.parse_analog_statement()?;
        let statements = Self::flatten_block_statement(stmt);

        Ok(AnalogBlock {
            statements,
            span: start.extend(self.previous_span()),
        })
    }

    /// Flatten a trivial begin/end wrapper; named blocks and blocks with
    /// local declarations must keep their structure.
    fn flatten_block_statement(stmt: AnalogStatement) -> Vec<AnalogStatement> {
        match stmt {
            AnalogStatement::Block(block) if block.name.is_none() && block.variables.is_empty() => {
                block.statements
            }
            other => vec![other],
        }
    }

    /// Parse analog function declaration
    ///
    /// Syntax: `analog function [return_type] function_name;`
    ///            input/output declarations;
    ///            variable declarations;
    ///            begin ... end
    ///         endfunction
    fn parse_analog_function(&mut self) -> Result<FunctionDef, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'analog'
        self.expect(TokenKind::Function)?; // consume 'function'

        // Parse optional return type (real, integer) - defaults to real
        let return_type = if self.check(TokenKind::Real) {
            self.advance();
            VarType::Real
        } else if self.check(TokenKind::Integer) {
            self.advance();
            VarType::Integer
        } else {
            VarType::Real // default
        };

        // Parse function name
        let name = self.expect_identifier("function name")?;
        self.expect(TokenKind::Semicolon)?;

        // Parse function body: input/output declarations, variable declarations, statements
        let mut params: Vec<FunctionParam> = Vec::new();
        let mut statements: Vec<AnalogStatement> = Vec::new();
        let mut local_vars: Vec<VariableDecl> = Vec::new();

        while !self.check(TokenKind::Endfunction) && !self.at_end() {
            match self.current().kind {
                TokenKind::Input => {
                    let param_start = self.current_span();
                    self.advance();
                    // Parse type if present
                    let param_type = self.parse_optional_var_type();
                    // Parse names
                    let names = self.parse_identifier_list()?;
                    self.expect(TokenKind::Semicolon)?;
                    for pname in names {
                        params.push(FunctionParam {
                            name: pname.into(),
                            param_type,
                            direction: ParamDirection::Input,
                            span: param_start,
                        });
                    }
                }
                TokenKind::Output => {
                    let param_start = self.current_span();
                    self.advance();
                    let param_type = self.parse_optional_var_type();
                    let names = self.parse_identifier_list()?;
                    self.expect(TokenKind::Semicolon)?;
                    for pname in names {
                        params.push(FunctionParam {
                            name: pname.into(),
                            param_type,
                            direction: ParamDirection::Output,
                            span: param_start,
                        });
                    }
                }
                TokenKind::Inout => {
                    let param_start = self.current_span();
                    self.advance();
                    let param_type = self.parse_optional_var_type();
                    let names = self.parse_identifier_list()?;
                    self.expect(TokenKind::Semicolon)?;
                    for pname in names {
                        params.push(FunctionParam {
                            name: pname.into(),
                            param_type,
                            direction: ParamDirection::Inout,
                            span: param_start,
                        });
                    }
                }
                TokenKind::Real | TokenKind::Integer => {
                    // Variable declaration inside function
                    let var = self.parse_variable_decl()?;
                    local_vars.push(var);
                }
                TokenKind::Begin => {
                    // Function body
                    let block = self.parse_block_statement()?;
                    statements.push(block);
                }
                _ => {
                    if self.check(TokenKind::Semicolon) {
                        self.advance();
                    } else {
                        statements.push(self.parse_analog_statement()?);
                    }
                }
            }
        }

        self.expect(TokenKind::Endfunction)?;

        Ok(FunctionDef {
            name: name.into(),
            return_type,
            params,
            locals: local_vars,
            body: AnalogBlock {
                statements,
                span: start.extend(self.previous_span()),
            },
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse optional variable type (real, integer)
    fn parse_optional_var_type(&mut self) -> VarType {
        if self.check(TokenKind::Real) {
            self.advance();
            VarType::Real
        } else if self.check(TokenKind::Integer) {
            self.advance();
            VarType::Integer
        } else {
            VarType::Real // default for analog functions
        }
    }

    /// Parse a list of identifiers separated by commas
    fn parse_identifier_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut names = Vec::new();
        names.push(self.expect_identifier("identifier")?);
        while self.match_token(TokenKind::Comma) {
            names.push(self.expect_identifier("identifier")?);
        }
        Ok(names)
    }

    /// Parse an analog statement
    fn parse_analog_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        // Attribute instances may precede statements; parse and discard
        if self.check(TokenKind::LParen) && self.peek_is(TokenKind::Star) {
            self.parse_attributes()?;
        }

        let start = self.current_span();

        match self.current().kind {
            TokenKind::Begin => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::Repeat => self.parse_repeat_statement(),
            TokenKind::Case => self.parse_case_statement(),
            TokenKind::At => self.parse_event_control(),
            TokenKind::Semicolon => {
                self.advance();
                Ok(AnalogStatement::Null(start))
            }
            // Handle both identifiers and keywords that can be used as variable names
            TokenKind::Identifier | TokenKind::SystemIdentifier => {
                self.parse_assignment_or_contribution()
            }
            _ if self.is_digital_statement_start() => Err(self.unsupported_ams_construct()),
            _ => {
                // Keywords might be used as variable names (e.g., voltage, current)
                // Try to parse as assignment/contribution
                self.parse_assignment_or_contribution()
            }
        }
    }

    /// Parse repeat loop: repeat (count) statement
    fn parse_repeat_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'repeat'

        self.expect(TokenKind::LParen)?;
        let count = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let body = Box::new(self.parse_analog_statement()?);

        Ok(AnalogStatement::Repeat(RepeatStmt {
            count,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse begin...end block
    fn parse_block_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'begin'

        // Optional block name: begin : block_name
        let name = if self.match_token(TokenKind::Colon) {
            Some(self.expect_identifier("block name")?.into())
        } else {
            None
        };

        let mut statements = Vec::new();
        let mut variables = Vec::new();
        while !self.check(TokenKind::End) && !self.at_end() {
            // Attributes may decorate block-local declarations (for example
            // operating-point metadata emitted by CMC model macros). They do
            // not change local-variable semantics, but must be consumed before
            // deciding whether the next item is a declaration or statement.
            self.parse_attributes()?;
            // Block-local variable declarations (LRM allows them in named
            // blocks; accept them in unnamed blocks for robustness)
            if matches!(
                self.current().kind,
                TokenKind::Real | TokenKind::Integer | TokenKind::String
            ) {
                variables.push(self.parse_variable_decl()?);
            } else {
                statements.push(self.parse_analog_statement()?);
            }
        }

        self.expect(TokenKind::End)?;
        Ok(AnalogStatement::Block(BlockStmt {
            name,
            statements,
            variables,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'if'

        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let then_branch = Box::new(self.parse_analog_statement()?);
        let else_branch = if self.match_token(TokenKind::Else) {
            Some(Box::new(self.parse_analog_statement()?))
        } else {
            None
        };

        Ok(AnalogStatement::Conditional(ConditionalStmt {
            condition,
            then_branch,
            else_branch,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse for loop
    fn parse_for_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'for'

        self.expect(TokenKind::LParen)?;

        // Initialization: var = expr
        let var = self.expect_identifier("loop variable")?;
        self.expect(TokenKind::Assign_)?;
        let init = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        // Condition
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        // Update: var = expr
        let update_var = self.expect_identifier("loop variable")?;
        self.expect(TokenKind::Assign_)?;
        let update_val = self.parse_expression()?;
        let update = Box::new(AssignmentStmt {
            target: LValue::Variable {
                name: update_var.into(),
                span: self.previous_span(),
            },
            value: update_val,
            span: self.previous_span(),
        });

        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_analog_statement()?);

        Ok(AnalogStatement::For(ForStmt {
            var: var.into(),
            init,
            condition,
            update,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse while loop
    fn parse_while_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'while'

        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let body = Box::new(self.parse_analog_statement()?);

        Ok(AnalogStatement::While(WhileStmt {
            condition,
            body,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse case statement
    fn parse_case_statement(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'case'

        self.expect(TokenKind::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let mut items = Vec::new();
        let mut default = None;

        while !self.check(TokenKind::Endcase) && !self.at_end() {
            if self.match_token(TokenKind::Default) {
                // The colon after 'default' is optional per the LRM
                self.match_token(TokenKind::Colon);
                default = Some(Box::new(self.parse_analog_statement()?));
            } else {
                let item = self.parse_case_item()?;
                items.push(item);
            }
        }

        self.expect(TokenKind::Endcase)?;
        Ok(AnalogStatement::Case(CaseStmt {
            expr,
            items,
            default,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse a single case item
    fn parse_case_item(&mut self) -> Result<CaseItem, ParseError> {
        let start = self.current_span();
        let mut matches = Vec::new();

        loop {
            matches.push(self.parse_expression()?);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Colon)?;
        let statement = Box::new(self.parse_analog_statement()?);

        Ok(CaseItem {
            matches,
            statement,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse event control: @(event) statement
    fn parse_event_control(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        self.advance(); // consume '@'

        self.expect(TokenKind::LParen)?;
        let event = self.parse_event_expr()?;
        self.expect(TokenKind::RParen)?;

        let statement = Box::new(self.parse_analog_statement()?);

        Ok(AnalogStatement::EventControl(EventControlStmt {
            event,
            statement,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Parse event expression (with `or` lists)
    fn parse_event_expr(&mut self) -> Result<EventExpr, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_event_primary()?;

        // `or` is not a reserved word; it arrives as an identifier
        while self.check(TokenKind::Identifier) && self.current().text.as_deref() == Some("or") {
            self.advance();
            let right = self.parse_event_primary()?;
            left = EventExpr::Or {
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            };
        }

        Ok(left)
    }

    /// Parse a single event term
    fn parse_event_primary(&mut self) -> Result<EventExpr, ParseError> {
        let start = self.current_span();

        match self.current().kind {
            // initial_step arrives as the `initial` keyword followed by
            // `_step`? No - `initial_step` lexes as one identifier; the
            // `initial` keyword appears for bare @(initial) robustness.
            TokenKind::Initial => {
                self.advance();
                let analyses = self.parse_step_event_analysis_args("initial_step")?;
                Ok(EventExpr::InitialStep {
                    analyses,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Final => {
                self.advance();
                let analyses = self.parse_step_event_analysis_args("final_step")?;
                Ok(EventExpr::FinalStep {
                    analyses,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Cross => {
                self.advance();
                let args = self.parse_arg_list()?;
                if args.is_empty() {
                    return Err(self.error(ParseErrorKind::InvalidAnalogStatement));
                }
                let mut args = args.into_iter();
                let signal = args.next().unwrap();
                if matches!(signal, Expression::NullArgument(_)) {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "cross expression may not be null".into(),
                    )));
                }
                let direction = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                let time_tol = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                let expr_tol = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                let enable = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                if args.next().is_some() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "cross event accepts at most 5 arguments".into(),
                    )));
                }
                if (time_tol.is_some() || expr_tol.is_some()) && direction.is_none() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "cross tolerances require a defined direction".into(),
                    )));
                }
                if expr_tol.is_some() && time_tol.is_none() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "cross expr_tol requires time_tol".into(),
                    )));
                }
                Ok(EventExpr::Cross {
                    signal,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Above => {
                self.advance();
                let args = self.parse_arg_list()?;
                if args.is_empty() {
                    return Err(self.error(ParseErrorKind::InvalidAnalogStatement));
                }
                let mut args = args.into_iter();
                let signal = args.next().unwrap();
                if matches!(signal, Expression::NullArgument(_)) {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "above expression may not be null".into(),
                    )));
                }
                let time_tol = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                let expr_tol = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                let enable = args
                    .next()
                    .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                    .map(Box::new);
                if args.next().is_some() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "above event accepts at most 4 arguments".into(),
                    )));
                }
                if expr_tol.is_some() && time_tol.is_none() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "above expr_tol requires time_tol".into(),
                    )));
                }
                Ok(EventExpr::Above {
                    signal,
                    time_tol,
                    expr_tol,
                    enable,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Timer => {
                self.advance();
                let args = self.parse_arg_list()?;
                if args.is_empty() {
                    return Err(self.error(ParseErrorKind::InvalidAnalogStatement));
                }
                let mut args = args.into_iter();
                let start_time = args.next().unwrap();
                let period = args.next().map(Box::new);
                let time_tol = args.next().map(Box::new);
                let enable = args.next().map(Box::new);
                if args.next().is_some() {
                    return Err(self.error(ParseErrorKind::InvalidEventExpression(
                        "timer event accepts at most 4 arguments".into(),
                    )));
                }
                Ok(EventExpr::Timer {
                    start: start_time,
                    period,
                    time_tol,
                    enable,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Posedge => {
                self.advance();
                let signal = self.parse_expression()?;
                Ok(EventExpr::Posedge {
                    signal,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Negedge => {
                self.advance();
                let signal = self.parse_expression()?;
                Ok(EventExpr::Negedge {
                    signal,
                    span: start.extend(self.previous_span()),
                })
            }
            TokenKind::Identifier => {
                let name = self.current().text.clone().unwrap_or_default();
                match name.as_str() {
                    "initial_step" => {
                        self.advance();
                        let analyses = self.parse_step_event_analysis_args("initial_step")?;
                        Ok(EventExpr::InitialStep {
                            analyses,
                            span: start.extend(self.previous_span()),
                        })
                    }
                    "final_step" => {
                        self.advance();
                        let analyses = self.parse_step_event_analysis_args("final_step")?;
                        Ok(EventExpr::FinalStep {
                            analyses,
                            span: start.extend(self.previous_span()),
                        })
                    }
                    _ => {
                        let signal = self.parse_expression()?;
                        Ok(EventExpr::Cross {
                            signal,
                            direction: None,
                            time_tol: None,
                            expr_tol: None,
                            enable: None,
                            span: start.extend(self.previous_span()),
                        })
                    }
                }
            }
            _ => {
                let signal = self.parse_expression()?;
                Ok(EventExpr::Cross {
                    signal,
                    direction: None,
                    time_tol: None,
                    expr_tol: None,
                    enable: None,
                    span: start.extend(self.previous_span()),
                })
            }
        }
    }

    /// Parse the optional constant-string analysis filters accepted by
    /// `initial_step` and `final_step`.
    fn parse_step_event_analysis_args(
        &mut self,
        event_name: &str,
    ) -> Result<Vec<StringLit>, ParseError> {
        if !self.check(TokenKind::LParen) {
            return Ok(Vec::new());
        }

        self.parse_arg_list()?
            .into_iter()
            .map(|arg| match arg {
                Expression::StringLit(value) => Ok(value),
                _ => Err(self.error(ParseErrorKind::InvalidEventExpression(format!(
                    "{event_name} analysis filters must be string literals"
                )))),
            })
            .collect()
    }

    /// Parse assignment or contribution statement
    fn parse_assignment_or_contribution(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        let saved_pos = self.pos; // Save position for backtracking

        // Any access function (V, I, Pwr, Temp, ...) followed by a node list
        // and `<+` is a contribution; `:` introduces an indirect
        // contribution `V(x): lhs == rhs;`
        if self.check(TokenKind::Identifier) && self.peek_is(TokenKind::LParen) {
            // Try to parse as branch access
            if let Ok(access) = self.try_parse_branch_access() {
                if self.match_token(TokenKind::Contribute) {
                    let value = self.parse_expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    return Ok(AnalogStatement::Contribution(ContributionStmt {
                        target: access,
                        value,
                        span: start.extend(self.previous_span()),
                    }));
                } else if self.match_token(TokenKind::Colon) {
                    // Indirect branch assignment: the equation parses as a
                    // single expression whose top-level operator must be ==
                    let equation = self.parse_expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    let Expression::Binary(BinaryExpr {
                        op: BinaryOp::Eq,
                        left,
                        right,
                        ..
                    }) = equation
                    else {
                        return Err(ParseError::new(
                            ParseErrorKind::Expected {
                                expected: "lhs == rhs equation in indirect contribution"
                                    .to_string(),
                                found: "expression without a top-level ==".to_string(),
                            },
                            start.extend(self.previous_span()),
                        ));
                    };
                    return Ok(AnalogStatement::IndirectContribution(
                        IndirectContributionStmt {
                            branch: access,
                            lhs: *left,
                            rhs: *right,
                            span: start.extend(self.previous_span()),
                        },
                    ));
                } else {
                    // Not a contribution, restore position and parse as normal
                    self.pos = saved_pos;
                }
            } else {
                // Failed to parse branch access, restore position
                self.pos = saved_pos;
            }
        }

        // Check for system task call ($strobe, $display, $write, etc.)
        if self.check(TokenKind::SystemIdentifier) {
            let sys_name = self.current().text.clone().unwrap_or_default();
            self.advance();

            // Parse arguments if present
            let mut args = Vec::new();
            if self.match_token(TokenKind::LParen) {
                if !self.check(TokenKind::RParen) {
                    args.push(self.parse_expression()?);
                    while self.match_token(TokenKind::Comma) {
                        args.push(self.parse_expression()?);
                    }
                }
                self.expect(TokenKind::RParen)?;
            }
            self.expect(TokenKind::Semicolon)?;

            return Ok(AnalogStatement::Call(CallStmt {
                name: sys_name.into(),
                args,
                span: start.extend(self.previous_span()),
            }));
        }

        // Must be assignment (to a scalar variable or an array element)
        let name = self.expect_identifier("variable")?;

        let target = if self.match_token(TokenKind::LBracket) {
            let index = self.parse_expression()?;
            self.expect(TokenKind::RBracket)?;
            LValue::ArrayAccess {
                name: name.into(),
                index: Box::new(index),
                span: start,
            }
        } else {
            LValue::Variable {
                name: name.into(),
                span: start,
            }
        };

        self.expect(TokenKind::Assign_)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(AnalogStatement::Assignment(AssignmentStmt {
            target,
            value,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Try to parse a branch access (V(a,b), I(a,b), Pwr(t), ...)
    fn try_parse_branch_access(&mut self) -> Result<BranchAccess, ParseError> {
        let start = self.current_span();
        let access = self.expect_identifier("access function")?;

        self.expect(TokenKind::LParen)?;
        if self.match_token(TokenKind::Lt) {
            let name = self.expect_identifier("branch")?;
            self.expect(TokenKind::Gt)?;
            self.expect(TokenKind::RParen)?;
            return Ok(BranchAccess::Branch {
                access: access.into(),
                name: name.into(),
                span: start.extend(self.previous_span()),
            });
        }

        let pos = self.expect_branch_endpoint("node")?;
        let neg = if self.match_token(TokenKind::Comma) {
            Some(self.expect_branch_endpoint("node")?.into())
        } else {
            None
        };
        self.expect(TokenKind::RParen)?;

        Ok(BranchAccess::Nodes {
            access: access.into(),
            pos: pos.into(),
            neg,
            span: start.extend(self.previous_span()),
        })
    }

    // Expression parsing (Pratt parser / precedence climbing)

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_ternary()
    }

    /// Parse ternary conditional
    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut expr = self.parse_or()?;

        if self.match_token(TokenKind::Question) {
            let then_expr = Box::new(self.parse_expression()?);
            self.expect(TokenKind::Colon)?;
            let else_expr = Box::new(self.parse_expression()?);
            expr = Expression::Conditional(ConditionalExpr {
                condition: Box::new(expr),
                then_expr,
                else_expr,
                span: start.extend(self.previous_span()),
            });
        }

        Ok(expr)
    }

    /// Parse logical or
    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_and()?;

        while self.match_token(TokenKind::Or) {
            let right = self.parse_and()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse logical and
    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_bit_or()?;

        while self.match_token(TokenKind::And) {
            let right = self.parse_bit_or()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse bitwise or (|)
    fn parse_bit_or(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_bit_xor()?;

        while self.match_token(TokenKind::BitOr) {
            let right = self.parse_bit_xor()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse bitwise xor (^)
    fn parse_bit_xor(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_bit_and()?;

        while self.match_token(TokenKind::BitXor) {
            let right = self.parse_bit_and()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::BitXor,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse bitwise and (&)
    fn parse_bit_and(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_equality()?;

        while self.match_token(TokenKind::BitAnd) {
            let right = self.parse_equality()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse equality (`==`, `!=`) below relational comparisons.
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_relational()?;

        loop {
            let op = match self.current().kind {
                TokenKind::Eq => {
                    self.advance();
                    BinaryOp::Eq
                }
                TokenKind::Ne => {
                    self.advance();
                    BinaryOp::Ne
                }
                _ => break,
            };

            let right = self.parse_relational()?;
            left = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse relational comparisons (`<`, `<=`, `>`, `>=`) above equality.
    fn parse_relational(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_shift()?;

        loop {
            let op = match self.current().kind {
                TokenKind::Lt => {
                    self.advance();
                    BinaryOp::Lt
                }
                TokenKind::Le => {
                    self.advance();
                    BinaryOp::Le
                }
                TokenKind::Gt => {
                    self.advance();
                    BinaryOp::Gt
                }
                TokenKind::Ge => {
                    self.advance();
                    BinaryOp::Ge
                }
                _ => break,
            };

            let right = self.parse_shift()?;
            left = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse shifts (<< >>)
    fn parse_shift(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_additive()?;

        loop {
            let op = match self.current().kind {
                TokenKind::Shl => {
                    self.advance();
                    BinaryOp::Shl
                }
                TokenKind::Shr => {
                    self.advance();
                    BinaryOp::Shr
                }
                _ => break,
            };

            let right = self.parse_additive()?;
            left = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse additive (+ -)
    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    BinaryOp::Add
                }
                TokenKind::Minus => {
                    self.advance();
                    BinaryOp::Sub
                }
                _ => break,
            };

            let right = self.parse_multiplicative()?;
            left = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse multiplicative (* / %)
    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_power()?;

        loop {
            let op = match self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    BinaryOp::Mul
                }
                TokenKind::Slash => {
                    self.advance();
                    BinaryOp::Div
                }
                TokenKind::Percent => {
                    self.advance();
                    BinaryOp::Mod
                }
                _ => break,
            };

            let right = self.parse_power()?;
            left = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse power (**)
    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let left = self.parse_unary()?;

        if self.match_token(TokenKind::StarStar) {
            let right = self.parse_power()?; // Right associative
            return Ok(Expression::Binary(BinaryExpr {
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            }));
        }

        Ok(left)
    }

    /// Parse unary (- ! ~)
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();

        let op = match self.current().kind {
            TokenKind::Minus => {
                self.advance();
                Some(UnaryOp::Neg)
            }
            TokenKind::Plus => {
                self.advance();
                Some(UnaryOp::Pos)
            }
            TokenKind::Not => {
                self.advance();
                Some(UnaryOp::Not)
            }
            TokenKind::BitNot => {
                self.advance();
                Some(UnaryOp::BitNot)
            }
            _ => None,
        };

        if let Some(op) = op {
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary(UnaryExpr {
                op,
                operand: Box::new(operand),
                span: start.extend(self.previous_span()),
            }));
        }

        self.parse_primary()
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();

        match self.current().kind {
            TokenKind::IntegerLiteral | TokenKind::RealLiteral => {
                let text = self.current().text.clone().unwrap_or_default();
                let span = start.extend(self.current_span());
                let value = parse_number(&text).map_err(|message| {
                    ParseError::new(ParseErrorKind::InvalidNumber(message), span)
                })?;
                self.advance();
                Ok(Expression::Number(NumberLit {
                    value,
                    raw: text.into(),
                    span: start.extend(self.previous_span()),
                }))
            }
            TokenKind::StringLiteral => {
                let text = self.current().text.clone().unwrap_or_default();
                self.advance();
                Ok(Expression::StringLit(StringLit {
                    value: text.into(),
                    span: start.extend(self.previous_span()),
                }))
            }
            TokenKind::SystemIdentifier => {
                let name: SmolStr = self.current().text.clone().unwrap_or_default().into();
                self.advance();

                let args = if self.check(TokenKind::LParen) {
                    self.parse_arg_list()?
                } else {
                    Vec::new()
                };

                Ok(Expression::SystemFunction(SystemFunction {
                    name,
                    args,
                    span: start.extend(self.previous_span()),
                }))
            }
            TokenKind::Identifier => {
                let name = self.expect_identifier("identifier")?;

                // Check if it's a function call or branch access
                if self.check(TokenKind::LParen) {
                    if name == "V" || name == "I" {
                        // Branch access
                        self.expect(TokenKind::LParen)?;
                        if self.match_token(TokenKind::Lt) {
                            let branch = self.expect_identifier("branch")?;
                            self.expect(TokenKind::Gt)?;
                            self.expect(TokenKind::RParen)?;
                            return Ok(Expression::BranchAccess(BranchAccess::Branch {
                                access: name.into(),
                                name: branch.into(),
                                span: start.extend(self.previous_span()),
                            }));
                        }

                        let pos = self.expect_branch_endpoint("node")?;
                        let neg = if self.match_token(TokenKind::Comma) {
                            Some(self.expect_branch_endpoint("node")?.into())
                        } else {
                            None
                        };
                        self.expect(TokenKind::RParen)?;
                        return Ok(Expression::BranchAccess(BranchAccess::Nodes {
                            access: name.into(),
                            pos: pos.into(),
                            neg,
                            span: start.extend(self.previous_span()),
                        }));
                    }

                    // Regular function call
                    let args = self.parse_arg_list()?;
                    return Ok(Expression::Call(CallExpr {
                        name: name.into(),
                        args,
                        span: start.extend(self.previous_span()),
                    }));
                }

                // Array element access: arr[index]
                if self.match_token(TokenKind::LBracket) {
                    let index = self.parse_expression()?;
                    self.expect(TokenKind::RBracket)?;
                    return Ok(Expression::ArrayAccess(ArrayAccessExpr {
                        array: name.into(),
                        index: Box::new(index),
                        span: start.extend(self.previous_span()),
                    }));
                }

                Ok(Expression::Identifier(Identifier {
                    name: name.into(),
                    span: start.extend(self.previous_span()),
                }))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBrace | TokenKind::AssignmentPatternStart => {
                let assignment_pattern = self.check(TokenKind::AssignmentPatternStart);
                self.advance();
                let elements = self.parse_array_literal_elements()?;
                Ok(Expression::ArrayLiteral(ArrayLiteralExpr {
                    elements,
                    assignment_pattern,
                    span: start.extend(self.previous_span()),
                }))
            }
            // A reserved digital keyword is never an identifier, so an
            // expression that reaches for one is naming a construct this
            // compiler does not have (`x = time;`), not a variable.
            TokenKind::AmsDigital => Err(self.unsupported_ams_construct()),
            // Handle keywords used as identifiers (e.g., 'voltage' as a variable name)
            _ if self.current().kind.is_keyword() => {
                let name = self
                    .current()
                    .text
                    .clone()
                    .unwrap_or_else(|| format!("{:?}", self.current().kind).to_lowercase());
                self.advance();

                // Check if it's a function call
                if self.check(TokenKind::LParen) {
                    let args = self.parse_arg_list()?;
                    return Ok(Expression::Call(CallExpr {
                        name: name.into(),
                        args,
                        span: start.extend(self.previous_span()),
                    }));
                }

                Ok(Expression::Identifier(Identifier {
                    name: name.into(),
                    span: start.extend(self.previous_span()),
                }))
            }
            _ => Err(self.error(ParseErrorKind::InvalidExpression)),
        }
    }

    /// Parse the non-empty contents of a concatenation, assignment pattern, or
    /// replication body.  Replication is retained recursively and is never
    /// expanded here.
    fn parse_array_literal_elements(&mut self) -> Result<Vec<ArrayLiteralElement>, ParseError> {
        if self.check(TokenKind::RBrace) {
            return Err(ParseError::expected(
                "one or more brace elements",
                "RBrace",
                self.current_span(),
            ));
        }

        let mut elements = Vec::new();
        loop {
            let start = self.current_span();
            let expression = self.parse_expression()?;
            let element = if self.match_token(TokenKind::LBrace) {
                let repeated = self.parse_array_literal_elements()?;
                ArrayLiteralElement::Replication(ReplicationExpr {
                    count: Box::new(expression),
                    elements: repeated,
                    span: start.extend(self.previous_span()),
                })
            } else {
                ArrayLiteralElement::Value(expression)
            };
            elements.push(element);

            if !self.match_token(TokenKind::Comma) {
                break;
            }
            if self.check(TokenKind::RBrace) {
                return Err(ParseError::expected(
                    "brace element after comma",
                    "RBrace",
                    self.current_span(),
                ));
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(elements)
    }

    /// Parse argument list
    fn parse_arg_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut args = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                if self.check(TokenKind::Comma) {
                    args.push(Expression::NullArgument(self.current_span()));
                } else {
                    args.push(self.parse_expression()?);
                }
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
                if self.check(TokenKind::RParen) {
                    args.push(Expression::NullArgument(self.current_span()));
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        Ok(args)
    }

    // Discipline and nature parsing

    /// Parse discipline definition
    fn parse_discipline(&mut self) -> Result<DisciplineDef, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'discipline'

        let name = self.expect_identifier("discipline name")?;
        // Standard headers terminate the discipline name with a semicolon;
        // retain compatibility with older headers that omit it.
        self.match_token(TokenKind::Semicolon);
        let mut potential = None;
        let mut flow = None;
        let mut domain = None;

        while !self.check(TokenKind::Enddiscipline) && !self.at_end() {
            if self.match_token(TokenKind::Potential) {
                potential = Some(self.expect_identifier("potential nature")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Flow) {
                flow = Some(self.expect_identifier("flow nature")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Domain) {
                if self.match_token(TokenKind::Continuous) {
                    domain = Some(DomainKind::Continuous);
                } else if self.match_token(TokenKind::Discrete) {
                    domain = Some(DomainKind::Discrete);
                }
                self.expect(TokenKind::Semicolon)?;
            } else {
                return Err(self.unsupported_current("discipline property"));
            }
        }

        self.expect(TokenKind::Enddiscipline)?;
        Ok(DisciplineDef {
            name: name.into(),
            potential,
            flow,
            domain,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse nature definition
    fn parse_nature(&mut self) -> Result<NatureDef, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'nature'

        let name = self.expect_identifier("nature name")?;
        let mut base = None;
        let mut access_fn = None;
        let mut units = None;
        let mut abstol = None;
        let mut idt_nature = None;
        let mut ddt_nature = None;

        // Optional base nature: nature Voltage : base_nature
        if self.match_token(TokenKind::Colon) {
            base = Some(self.expect_identifier("base nature")?.into());
        }

        // The semicolon after the nature name is optional (pre-LRM-2.2
        // headers like the EKV2.6 distribution omit it)
        self.match_token(TokenKind::Semicolon);

        while !self.check(TokenKind::Endnature) && !self.at_end() {
            if self.match_token(TokenKind::Access) {
                self.expect(TokenKind::Assign_)?;
                access_fn = Some(self.expect_identifier("access function")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Units) {
                self.expect(TokenKind::Assign_)?;
                if let TokenKind::StringLiteral = self.current().kind {
                    units = self.current().text.clone().map(|s| s.into());
                    self.advance();
                }
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Abstol) {
                self.expect(TokenKind::Assign_)?;
                abstol = Some(self.parse_expression()?);
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Idt_Nature) {
                self.expect(TokenKind::Assign_)?;
                idt_nature = Some(self.expect_identifier("idt_nature")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Ddt_Nature) {
                self.expect(TokenKind::Assign_)?;
                ddt_nature = Some(self.expect_identifier("ddt_nature")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else {
                return Err(self.unsupported_current("nature property"));
            }
        }

        self.expect(TokenKind::Endnature)?;
        Ok(NatureDef {
            name: name.into(),
            base,
            access: access_fn,
            units,
            abstol,
            idt_nature,
            ddt_nature,
            span: start.extend(self.previous_span()),
        })
    }

    // Helper methods

    fn current(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn current_span(&self) -> Span {
        self.current().span
    }

    fn previous_span(&self) -> Span {
        if self.pos > 0 {
            self.tokens[self.pos - 1].span
        } else {
            self.current_span()
        }
    }

    fn at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(ParseError::expected(
                format!("{:?}", kind),
                format!("{:?}", self.current().kind),
                self.current_span(),
            ))
        }
    }

    fn expect_identifier(&mut self, context: &str) -> Result<String, ParseError> {
        if let TokenKind::Identifier = self.current().kind {
            let text = self.current().text.clone().unwrap_or_default();
            self.advance();
            Ok(text)
        } else if self.current().kind.is_keyword() {
            // Allow keywords as identifiers - use preserved text from lexer
            let text = self.current().text.clone().unwrap_or_else(|| {
                // Fallback to formatting kind name if text not preserved
                format!("{:?}", self.current().kind).to_lowercase()
            });
            self.advance();
            Ok(text)
        } else if self.current().kind == TokenKind::AmsDigital {
            // A reserved digital keyword is not available as a name, so
            // `real time;` names the 1364 type rather than declaring a
            // variable. Say which keyword instead of reporting the token kind.
            Err(self.unsupported_ams_construct())
        } else {
            Err(ParseError::expected(
                context.to_string(),
                format!("{:?}", self.current().kind),
                self.current_span(),
            ))
        }
    }

    fn unsupported_current(&self, context: &str) -> ParseError {
        let found = self
            .current()
            .text
            .clone()
            .unwrap_or_else(|| format!("{:?}", self.current().kind));
        ParseError::new(
            ParseErrorKind::UnsupportedConstruct {
                context: context.to_string(),
                found,
            },
            self.current_span(),
        )
    }

    /// Refuse the generate keyword under the cursor without parsing its body.
    ///
    /// The body of an unsupported region carries no meaning for this compiler,
    /// so consuming it would only move the reported position away from the
    /// construct the author has to remove.
    fn unsupported_generate(&self) -> ParseError {
        let keyword = self
            .current()
            .text
            .clone()
            .unwrap_or_else(|| format!("{:?}", self.current().kind).to_lowercase());
        ParseError::new(
            ParseErrorKind::UnsupportedGenerate { keyword },
            self.current_span(),
        )
    }

    /// Refuse the Verilog-AMS digital keyword under the cursor by name.
    ///
    /// Like [`Self::unsupported_generate`], the construct's body is left
    /// unconsumed: it carries no meaning for this compiler, so skipping it
    /// would only move the reported position away from the keyword the author
    /// has to remove.
    fn unsupported_ams_construct(&self) -> ParseError {
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

    /// Whether the token introduces a digital construct in a *declaration*
    /// position (top level or module item).
    ///
    /// [`TokenKind::AmsDigital`] covers the keywords reserved solely to be
    /// refused. The rest already had token kinds but no production, so they
    /// reached the generic "unsupported module item" error; naming the
    /// construct is strictly better and changes no accepted source.
    const fn is_digital_declaration_keyword(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::AmsDigital
                | TokenKind::Assign
                | TokenKind::Casex
                | TokenKind::Casez
                | TokenKind::Connectmodule
                | TokenKind::Deassign
                | TokenKind::Endspecify
                | TokenKind::Endtask
                | TokenKind::Force
                | TokenKind::Fork
                | TokenKind::Initial
                | TokenKind::Join
                | TokenKind::Release
                | TokenKind::Specify
                | TokenKind::Task
                | TokenKind::Wire
        )
    }

    /// Whether the token under the cursor introduces a digital construct in
    /// *statement* position.
    ///
    /// The pre-existing keyword kinds double as identifiers here — an analog
    /// block may legitimately assign to a variable named `force` or `assign`,
    /// which [`Self::parse_assignment_or_contribution`] has always accepted.
    /// So a keyword is read as its construct only when what follows it cannot
    /// begin an assignment or a contribution. `force x = 1;` is the digital
    /// statement; `force = 1;` is a variable named `force`.
    ///
    /// Newly reserved [`TokenKind::AmsDigital`] keywords have no such history
    /// and are always the construct.
    fn is_digital_statement_start(&self) -> bool {
        let kind = self.current().kind;
        if kind == TokenKind::AmsDigital {
            return true;
        }
        if !Self::is_digital_declaration_keyword(kind) {
            return false;
        }
        !matches!(
            self.tokens.get(self.pos + 1).map(|t| t.kind),
            Some(
                TokenKind::Assign_
                    | TokenKind::LBracket
                    | TokenKind::Dot
                    | TokenKind::Contribute
                    | TokenKind::Semicolon
            )
        )
    }

    fn expect_branch_endpoint(&mut self, context: &str) -> Result<String, ParseError> {
        if self.check(TokenKind::IntegerLiteral) {
            let text = self.current().text.clone().unwrap_or_default();
            if text == "0" {
                self.advance();
                return Ok(text);
            }
        }
        self.expect_identifier(context)
    }

    fn peek_is(&self, kind: TokenKind) -> bool {
        self.tokens
            .get(self.pos + 1)
            .is_some_and(|t| t.kind == kind)
    }

    fn is_discipline_keyword(&self) -> bool {
        // Only match known discipline keywords - NOT arbitrary identifiers
        // Arbitrary identifiers would be port names, not disciplines
        matches!(
            self.current().kind,
            TokenKind::Electrical | TokenKind::Voltage | TokenKind::Current
        )
    }

    fn skip_directive(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip directive token
        // Skip until newline (handled by lexer in practice)
        Ok(())
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError::new(kind, self.current_span())
    }
}

/// Parse a number literal with scale factors
fn parse_number(s: &str) -> Result<f64, String> {
    parse_numeric_literal(s)?.as_exact_f64(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::source::SourceId;

    fn parse(source: &str) -> SourceFile {
        let tokens = Lexer::new(source, SourceId::new(0))
            .collect_tokens()
            .expect("lex failed");
        Parser::new(&tokens).parse().expect("parse failed")
    }

    fn parse_module(source: &str) -> Module {
        let file = parse(source);
        for item in file.items {
            if let Item::Module(m) = item {
                return m;
            }
        }
        panic!("no module in source");
    }

    fn parse_error(source: &str) -> ParseError {
        let tokens = Lexer::new(source, SourceId::new(0))
            .collect_tokens()
            .expect("lex failed");
        Parser::new(&tokens).parse().expect_err("parse succeeded")
    }

    #[test]
    fn attributes_do_not_swallow_declarations() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                (* desc="Resistance", units="Ohm" *) parameter real r = 1.0;
            endmodule"#,
        );
        assert_eq!(m.parameters.len(), 1);
        assert_eq!(m.parameters[0].name.as_str(), "r");
        assert_eq!(m.parameters[0].description.as_deref(), Some("Resistance"));
        assert_eq!(m.parameters[0].units.as_deref(), Some("Ohm"));
    }

    #[test]
    fn module_attributes_are_accepted_before_and_after_the_port_list() {
        let m = parse_module(
            r#"(* source="canonical" *)
            module a(p, n) (* model_group="BJT" level=504 *);
                inout p, n;
                electrical p, n;
            endmodule"#,
        );

        assert_eq!(m.attributes.len(), 3);
        assert_eq!(m.attributes[0].name.as_str(), "source");
        assert_eq!(m.attributes[1].name.as_str(), "model_group");
        assert_eq!(m.attributes[2].name.as_str(), "level");
    }

    #[test]
    fn multi_name_parameter_declarations() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real a = 1.0, b = 2.0 from (0:inf);
            endmodule"#,
        );
        assert_eq!(m.parameters.len(), 2);
        assert_eq!(m.parameters[0].name.as_str(), "a");
        assert!(m.parameters[0].range.is_none());
        assert_eq!(m.parameters[1].name.as_str(), "b");
        assert!(m.parameters[1].range.is_some());
    }

    #[test]
    fn parameter_array_declarations_preserve_explicit_type_and_ordered_dimensions() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter count = 2;
                parameter real taps[count:0][1:count] = '{'{1.0, 2.0}, '{3.0, 4.0}};
            endmodule"#,
        );

        assert!(!m.parameters[0].type_is_explicit);
        assert!(m.parameters[1].type_is_explicit);
        assert_eq!(m.parameters[1].param_type, ParamType::Real);
        assert_eq!(m.parameters[1].dimensions.len(), 2);
        assert!(matches!(
            m.parameters[1].dimensions[0].start,
            Expression::Identifier(ref identifier) if identifier.name == "count"
        ));
        assert!(matches!(
            m.parameters[1].dimensions[0].end,
            Expression::Number(ref number) if number.value == 0.0
        ));
        assert!(matches!(
            m.parameters[1].dimensions[1].start,
            Expression::Number(ref number) if number.value == 1.0
        ));
        assert!(matches!(
            m.parameters[1].dimensions[1].end,
            Expression::Identifier(ref identifier) if identifier.name == "count"
        ));
        assert!(matches!(
            m.parameters[1].default,
            Some(Expression::ArrayLiteral(ref literal)) if literal.assignment_pattern
        ));
    }

    #[test]
    fn assignment_patterns_remain_distinct_from_concatenations() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real pattern[0:0] = '{1.0};
                parameter real concatenation[0:0] = {1.0};
            endmodule"#,
        );

        assert!(matches!(
            m.parameters[0].default,
            Some(Expression::ArrayLiteral(ref literal)) if literal.assignment_pattern
        ));
        assert!(matches!(
            m.parameters[1].default,
            Some(Expression::ArrayLiteral(ref literal)) if !literal.assignment_pattern
        ));
    }

    #[test]
    fn concatenation_replication_is_retained_without_expansion() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real w = 1.0;
                parameter real repeated[0:3] = {4{w}};
            endmodule"#,
        );

        let Some(Expression::ArrayLiteral(literal)) = &m.parameters[1].default else {
            panic!("expected concatenation");
        };
        assert!(!literal.assignment_pattern);
        assert_eq!(literal.elements.len(), 1);
        let ArrayLiteralElement::Replication(replication) = &literal.elements[0] else {
            panic!("expected replication item");
        };
        assert!(matches!(
            replication.count.as_ref(),
            Expression::Number(number) if number.raw == "4"
        ));
        assert_eq!(replication.elements.len(), 1);
        assert!(matches!(
            &replication.elements[0],
            ArrayLiteralElement::Value(Expression::Identifier(identifier))
                if identifier.name == "w"
        ));
    }

    #[test]
    fn nested_replication_is_retained_recursively() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real w = 1.0;
                parameter real repeated[0:5] = {2{3{w}}};
            endmodule"#,
        );

        let Some(Expression::ArrayLiteral(literal)) = &m.parameters[1].default else {
            panic!("expected concatenation");
        };
        let ArrayLiteralElement::Replication(outer) = &literal.elements[0] else {
            panic!("expected outer replication");
        };
        assert!(matches!(
            outer.count.as_ref(),
            Expression::Number(number) if number.raw == "2"
        ));
        let ArrayLiteralElement::Replication(inner) = &outer.elements[0] else {
            panic!("expected nested replication");
        };
        assert!(matches!(
            inner.count.as_ref(),
            Expression::Number(number) if number.raw == "3"
        ));
        assert!(matches!(
            &inner.elements[0],
            ArrayLiteralElement::Value(Expression::Identifier(identifier))
                if identifier.name == "w"
        ));
    }

    #[test]
    fn assignment_pattern_replication_preserves_pattern_identity() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real repeated[0:4] = '{5{0.0}};
            endmodule"#,
        );

        let Some(Expression::ArrayLiteral(literal)) = &m.parameters[0].default else {
            panic!("expected assignment pattern");
        };
        assert!(literal.assignment_pattern);
        let ArrayLiteralElement::Replication(replication) = &literal.elements[0] else {
            panic!("expected replication item");
        };
        assert!(matches!(
            replication.count.as_ref(),
            Expression::Number(number) if number.raw == "5"
        ));
        assert!(matches!(
            &replication.elements[0],
            ArrayLiteralElement::Value(Expression::Number(number))
                if number.raw == "0.0"
        ));
    }

    #[test]
    fn empty_brace_forms_are_rejected() {
        for initializer in ["{}", "'{}", "{4{}}", "'{4{}}"] {
            let source = format!(
                "module a(p); inout p; electrical p; parameter real x[0:0] = {initializer}; endmodule"
            );
            let error = parse_error(&source);
            assert!(
                matches!(
                    error.kind,
                    ParseErrorKind::Expected { ref expected, .. }
                        if expected == "one or more brace elements"
                ),
                "unexpected error for {initializer}: {error}"
            );
        }
    }

    #[test]
    fn trailing_brace_commas_are_rejected() {
        for initializer in ["{1,}", "'{1,}", "{2{1,}}", "'{2{1,}}"] {
            let source = format!(
                "module a(p); inout p; electrical p; parameter real x[0:0] = {initializer}; endmodule"
            );
            let error = parse_error(&source);
            assert!(
                matches!(
                    error.kind,
                    ParseErrorKind::Expected { ref expected, .. }
                        if expected == "brace element after comma"
                ),
                "unexpected error for {initializer}: {error}"
            );
        }
    }

    #[test]
    fn parameter_exclude_and_multiple_ranges() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                parameter real c = 1.0 from [0:1) from (2:3] exclude 0.5;
            endmodule"#,
        );
        let range = m.parameters[0].range.as_ref().expect("range");
        assert_eq!(range.bounds.len(), 2);
        assert!(range.bounds[0].lower_inclusive);
        assert!(!range.bounds[0].upper_inclusive);
        assert!(!range.bounds[1].lower_inclusive);
        assert!(range.bounds[1].upper_inclusive);
        assert_eq!(range.exclude.len(), 1);
    }

    #[test]
    fn variable_initializers_and_arrays() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                real x = 1.0, y;
                real z[0:4];
                integer i = 3;
            endmodule"#,
        );
        assert_eq!(m.variables.len(), 3);
        let items: Vec<_> = m.variables.iter().flat_map(|d| &d.items).collect();
        assert_eq!(items.len(), 4);
        assert!(items[0].init.is_some());
        assert!(items[1].init.is_none());
        assert_eq!(items[2].dimensions.len(), 1);
        assert!(items[3].init.is_some());
    }

    #[test]
    fn analog_initial_block_parses() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                real x;
                analog initial begin
                    x = 1.0;
                end
                analog I(p, n) <+ x * V(p, n);
            endmodule"#,
        );
        assert!(m.analog_initial.is_some());
        assert_eq!(m.analog_initial.as_ref().unwrap().statements.len(), 1);
        assert!(m.analog_block.is_some());
    }

    #[test]
    fn multiple_analog_blocks_merge() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                analog I(p, n) <+ V(p, n);
                analog I(p, n) <+ 2.0 * V(p, n);
            endmodule"#,
        );
        assert_eq!(m.analog_block.as_ref().unwrap().statements.len(), 2);
    }

    #[test]
    fn module_instances_retain_named_bindings_and_unconnected_ports() {
        let m = parse_module(
            r#"module parent(p, n);
                inout p, n;
                electrical p, n;
                child #(.gain(2.0)) first (.p(p), .n(n)), second (.p(p), .n());
            endmodule"#,
        );

        assert_eq!(m.instances.len(), 2);
        assert_eq!(m.instances[0].module.as_str(), "child");
        assert_eq!(m.instances[0].name.as_str(), "first");
        assert_eq!(m.instances[0].parameters.len(), 1);
        assert_eq!(m.instances[0].parameters[0].name.as_deref(), Some("gain"));
        assert!(matches!(
            m.instances[1].connections[1],
            Connection::Named {
                ref port,
                signal: None,
                ..
            } if port.as_str() == "n"
        ));
    }

    #[test]
    fn module_instances_retain_ordered_parameters_connections_and_holes() {
        let m = parse_module(
            r#"module parent(p, n);
                inout p, n;
                electrical p, n;
                child #(2.0, 3.0) device (p, , n);
            endmodule"#,
        );

        let instance = &m.instances[0];
        assert!(
            instance
                .parameters
                .iter()
                .all(|parameter| parameter.name.is_none())
        );
        assert_eq!(instance.connections.len(), 3);
        assert!(matches!(
            instance.connections[1],
            Connection::Ordered { signal: None, .. }
        ));
    }

    #[test]
    fn named_branch_declarations() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                branch (p, n) res, cap;
                analog I(res) <+ V(res);
            endmodule"#,
        );
        assert_eq!(m.branches.len(), 2);
        assert_eq!(m.branches[0].name.as_str(), "res");
        assert_eq!(m.branches[0].pos.as_str(), "p");
        assert_eq!(m.branches[0].neg.as_str(), "n");
        assert_eq!(m.branches[1].name.as_str(), "cap");
    }

    #[test]
    fn angle_bracket_named_branch_access_parses() {
        let m = parse_module(
            r#"module a(c, e);
                inout c, e;
                electrical c, e;
                branch (c, e) coll;
                real ic;
                analog begin
                    ic = I(<coll>);
                    I(<coll>) <+ ic;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Assignment(assign) = &stmts[0] else {
            panic!("expected branch-current assignment");
        };
        assert!(matches!(
            assign.value,
            Expression::BranchAccess(BranchAccess::Branch { .. })
        ));
        let AnalogStatement::Contribution(contrib) = &stmts[1] else {
            panic!("expected named branch contribution");
        };
        assert!(matches!(
            contrib.target,
            BranchAccess::Branch { ref name, .. } if name.as_str() == "coll"
        ));
    }

    #[test]
    fn numeric_zero_branch_endpoint_parses_as_ground() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                real sensed;
                analog begin
                    sensed = I(p, 0);
                    I(p, 0) <+ sensed;
                end
            endmodule"#,
        );

        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Assignment(assign) = &stmts[0] else {
            panic!("expected branch-current assignment");
        };
        assert!(matches!(
            assign.value,
            Expression::BranchAccess(BranchAccess::Nodes {
                ref pos,
                neg: Some(ref neg),
                ..
            }) if pos.as_str() == "p" && neg.as_str() == "0"
        ));

        let AnalogStatement::Contribution(contrib) = &stmts[1] else {
            panic!("expected branch-current contribution");
        };
        assert!(matches!(
            contrib.target,
            BranchAccess::Nodes {
                ref pos,
                neg: Some(ref neg),
                ..
            } if pos.as_str() == "p" && neg.as_str() == "0"
        ));
    }

    #[test]
    fn user_discipline_net_declaration() {
        let m = parse_module(
            r#"module a(p, n, t);
                inout p, n, t;
                electrical p, n;
                thermal t;
                electrical mid;
            endmodule"#,
        );
        assert_eq!(m.nets.len(), 3);
        assert_eq!(m.nets[1].discipline.as_str(), "thermal");
        assert_eq!(m.nets[1].names, vec![SmolStr::from("t")]);
    }

    #[test]
    fn ansi_port_list() {
        let m = parse_module(
            r#"module a(inout electrical p, n, input electrical g);
            endmodule"#,
        );
        assert_eq!(m.ports.len(), 3);
        assert_eq!(m.port_declarations.len(), 3);
        assert_eq!(m.port_declarations[0].direction, PortDirection::Inout);
        assert_eq!(m.port_declarations[1].direction, PortDirection::Inout);
        assert_eq!(
            m.port_declarations[1].discipline.as_deref(),
            Some("electrical")
        );
        assert_eq!(m.port_declarations[2].direction, PortDirection::Input);
    }

    #[test]
    fn repeat_statement_parses() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                real x;
                analog begin
                    repeat (3) x = x + 1.0;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        assert!(matches!(stmts[0], AnalogStatement::Repeat(_)));
    }

    #[test]
    fn block_local_variable_declarations() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                analog begin : calc
                    real tmp;
                    tmp = V(p, n);
                    I(p, n) <+ tmp;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Block(block) = &stmts[0] else {
            panic!("expected block");
        };
        assert_eq!(block.variables.len(), 1);
        assert_eq!(block.statements.len(), 2);
    }

    #[test]
    fn attributes_may_decorate_block_local_variable_declarations() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                analog begin : calc
                    (* desc="Effective width", units="m" *) real width;
                    width = V(p, n);
                    I(p, n) <+ width;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Block(block) = &stmts[0] else {
            panic!("expected block");
        };
        assert_eq!(block.variables.len(), 1);
        assert_eq!(block.variables[0].items[0].name.as_str(), "width");
        assert_eq!(block.statements.len(), 2);
    }

    #[test]
    fn event_expressions() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                real x;
                analog begin
                    @(initial_step) x = 1.0;
                    @(cross(V(p, n), 1)) x = 2.0;
                    @(initial_step or final_step) x = 3.0;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::EventControl(e1) = &stmts[0] else {
            panic!("expected event control");
        };
        assert!(matches!(e1.event, EventExpr::InitialStep { .. }));
        let AnalogStatement::EventControl(e2) = &stmts[1] else {
            panic!("expected event control");
        };
        let EventExpr::Cross {
            direction: Some(direction),
            ..
        } = &e2.event
        else {
            panic!("expected directed cross event");
        };
        assert!(matches!(
            direction.as_ref(),
            Expression::Number(number) if number.value == 1.0
        ));
        let AnalogStatement::EventControl(e3) = &stmts[2] else {
            panic!("expected event control");
        };
        assert!(matches!(e3.event, EventExpr::Or { .. }));
    }

    #[test]
    fn cross_event_direction_accepts_an_analog_expression() {
        let module = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                real direction, x;
                analog begin
                    direction = V(p, n) >= 0.0 ? 1.0 : -1.0;
                    @(cross(V(p, n), direction)) x = x + 1.0;
                end
            endmodule"#,
        );
        let statements = &module.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::EventControl(event) = &statements[1] else {
            panic!("expected event control");
        };
        let EventExpr::Cross {
            direction: Some(direction),
            ..
        } = &event.event
        else {
            panic!("expected directed cross event");
        };
        assert!(matches!(
            direction.as_ref(),
            Expression::Identifier(identifier) if identifier.name == "direction"
        ));
    }

    #[test]
    fn event_controls_reject_invalid_null_argument_dependencies() {
        for (event, expected) in [
            ("cross(V(p, n), , 1.0e-9)", "tolerances require"),
            ("cross(V(p, n), 1, , 1.0e-3)", "expr_tol requires"),
            ("above(V(p, n), , 1.0e-3)", "expr_tol requires"),
            ("cross(, 1)", "expression may not be null"),
        ] {
            let source = format!(
                "module a(p, n); inout p, n; electrical p, n; real x; analog @({event}) x = 1.0; endmodule"
            );
            let diagnostic = parse_error(&source).to_string();
            assert!(
                diagnostic.contains(expected),
                "{event} produced the wrong diagnostic: {diagnostic}"
            );
        }
    }

    #[test]
    fn step_events_preserve_analysis_filters() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                real x;
                analog begin
                    @(initial_step("dc", "tran")) x = 1.0;
                    @(final_step("ac")) x = 2.0;
                end
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::EventControl(initial) = &stmts[0] else {
            panic!("expected initial-step event control");
        };
        let EventExpr::InitialStep { analyses, .. } = &initial.event else {
            panic!("expected initial-step event");
        };
        assert_eq!(
            analyses
                .iter()
                .map(|analysis| analysis.value.as_str())
                .collect::<Vec<_>>(),
            ["dc", "tran"]
        );

        let AnalogStatement::EventControl(final_step) = &stmts[1] else {
            panic!("expected final-step event control");
        };
        let EventExpr::FinalStep { analyses, .. } = &final_step.event else {
            panic!("expected final-step event");
        };
        assert_eq!(analyses[0].value.as_str(), "ac");
    }

    #[test]
    fn array_literal_expression() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                analog V(p, n) <+ laplace_nd(I(p, n), '{1.0, 2.0}, '{0.5});
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Contribution(c) = &stmts[0] else {
            panic!("expected contribution");
        };
        let Expression::Call(call) = &c.value else {
            panic!("expected call, got {:?}", c.value);
        };
        assert_eq!(call.name.as_str(), "laplace_nd");
        assert!(matches!(
            call.args[1],
            Expression::ArrayLiteral(ref array) if array.assignment_pattern
        ));
    }

    #[test]
    fn bitwise_and_shift_expressions_parse_with_precedence() {
        let m = parse_module(
            r#"module a(p, n);
                inout p, n;
                electrical p, n;
                real a, b, c, d;
                real y1, y2, y3, y4, y5, y6;
                analog begin
                    y1 = a << b + c;
                    y2 = a + b << c;
                    y3 = a & b ^ c | d;
                    y4 = a == b & c != d;
                    y5 = a == b < c;
                    y6 = a < b == c < d;
                end
            endmodule"#,
        );
        let statements = &m.analog_block.as_ref().unwrap().statements;

        let assignment_value = |index: usize| {
            let AnalogStatement::Assignment(assign) = &statements[index] else {
                panic!("expected assignment {index}");
            };
            &assign.value
        };

        let Expression::Binary(y1) = assignment_value(0) else {
            panic!("expected y1 binary expression");
        };
        assert_eq!(y1.op, BinaryOp::Shl);
        assert!(matches!(*y1.right, Expression::Binary(ref b) if b.op == BinaryOp::Add));

        let Expression::Binary(y2) = assignment_value(1) else {
            panic!("expected y2 binary expression");
        };
        assert_eq!(y2.op, BinaryOp::Shl);
        assert!(matches!(*y2.left, Expression::Binary(ref b) if b.op == BinaryOp::Add));

        let Expression::Binary(y3) = assignment_value(2) else {
            panic!("expected y3 binary expression");
        };
        assert_eq!(y3.op, BinaryOp::BitOr);
        assert!(matches!(*y3.left, Expression::Binary(ref b) if b.op == BinaryOp::BitXor));

        let Expression::Binary(y4) = assignment_value(3) else {
            panic!("expected y4 binary expression");
        };
        assert_eq!(y4.op, BinaryOp::BitAnd);
        assert!(matches!(*y4.left, Expression::Binary(ref b) if b.op == BinaryOp::Eq));
        assert!(matches!(*y4.right, Expression::Binary(ref b) if b.op == BinaryOp::Ne));

        let Expression::Binary(y5) = assignment_value(4) else {
            panic!("expected y5 binary expression");
        };
        assert_eq!(y5.op, BinaryOp::Eq);
        assert!(matches!(*y5.right, Expression::Binary(ref b) if b.op == BinaryOp::Lt));

        let Expression::Binary(y6) = assignment_value(5) else {
            panic!("expected y6 binary expression");
        };
        assert_eq!(y6.op, BinaryOp::Eq);
        assert!(matches!(*y6.left, Expression::Binary(ref b) if b.op == BinaryOp::Lt));
        assert!(matches!(*y6.right, Expression::Binary(ref b) if b.op == BinaryOp::Lt));
    }

    #[test]
    fn case_default_without_colon() {
        let m = parse_module(
            r#"module a(p);
                inout p;
                electrical p;
                integer s;
                real x;
                analog case (s)
                    0: x = 1.0;
                    1, 2: x = 2.0;
                    default x = 3.0;
                endcase
            endmodule"#,
        );
        let stmts = &m.analog_block.as_ref().unwrap().statements;
        let AnalogStatement::Case(case) = &stmts[0] else {
            panic!("expected case");
        };
        assert_eq!(case.items.len(), 2);
        assert!(case.default.is_some());
    }

    #[test]
    fn scale_factor_values() {
        assert_eq!(parse_number("1k").unwrap(), 1e3);
        assert_eq!(parse_number("1K").unwrap(), 1e3);
        assert_eq!(parse_number("2.5M").unwrap(), 2.5e6);
        assert_eq!(parse_number("1meg").unwrap(), 1e6);
        assert_eq!(parse_number("1Meg").unwrap(), 1e6);
        assert_eq!(parse_number("1MEG").unwrap(), 1e6);
        assert_eq!(parse_number("3m").unwrap(), 3e-3);
        assert_eq!(parse_number("4u").unwrap(), 4e-6);
        assert_eq!(parse_number("5n").unwrap(), 5e-9);
        assert_eq!(parse_number("6p").unwrap(), 6e-12);
        assert_eq!(parse_number("7f").unwrap(), 7e-15);
        assert_eq!(parse_number("8a").unwrap(), 8e-18);
        assert_eq!(parse_number("9T").unwrap(), 9e12);
        assert_eq!(parse_number("10G").unwrap(), 10e9);
    }

    #[test]
    fn based_literals_preserve_raw_spelling_and_apply_width_and_signedness() {
        let module = parse_module(
            r#"module based;
                parameter integer neg_one = 8'shFF;
                parameter integer truncated = 4 'h 1F;
                parameter integer exact_large = 54'h20_0000_0000_0000;
            endmodule"#,
        );
        for (index, raw, expected) in [
            (0, "8'shFF", -1.0),
            (1, "4 'h 1F", 15.0),
            (2, "54'h20_0000_0000_0000", 9_007_199_254_740_992.0),
        ] {
            let Some(Expression::Number(number)) = &module.parameters[index].default else {
                panic!("expected numeric default");
            };
            assert_eq!(number.raw.as_str(), raw);
            assert_eq!(number.value, expected);
        }
    }

    #[test]
    fn numeric_underscores_do_not_change_values() {
        assert_eq!(parse_number("1_000_000").unwrap(), 1_000_000.0);
        assert_eq!(parse_number("1_2.3_4").unwrap(), 12.34);
        assert_eq!(parse_number("1.0e1_2").unwrap(), 1.0e12);
        assert_eq!(parse_number("2_5k").unwrap(), 25_000.0);
        assert_eq!(parse_number("1__000__").unwrap(), 1_000.0);
        assert_eq!(parse_number("1__2.3__4__").unwrap(), 12.34);
        assert_eq!(parse_number("1e__3__").unwrap(), 1_000.0);
        assert_eq!(parse_number("1.0__").unwrap(), 1.0);
        assert_eq!(parse_number("8'h1234_5678_9ABC_DEFF").unwrap(), 255.0);
        assert_eq!(
            parse_number("9_007_199_254_740_992").unwrap(),
            9_007_199_254_740_992.0
        );
        assert!(parse_number("9_007_199_254_740_993").is_err());
    }

    #[test]
    fn integer_not_exactly_representable_by_scalar_ir_is_rejected() {
        let error = parse_error(
            r#"module rounded;
                parameter integer bad = 54'h20_0000_0000_0001;
            endmodule"#,
        );
        let ParseErrorKind::InvalidNumber(message) = error.kind else {
            panic!("unexpected parse error: {error}");
        };
        assert!(
            message.contains("cannot be represented exactly"),
            "{message}"
        );
    }
}
