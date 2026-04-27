//! Verilog-A/AMS Parser
//!
//! Recursive descent parser for Verilog-A LRM 2.4 syntax.
//! Produces AST from token stream.

use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::{Token, TokenKind};
use crate::source::{SourceMap, Span};
use smol_str::SmolStr;

/// Parser for Verilog-A/AMS
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    #[allow(dead_code)]
    source_map: &'a SourceMap,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(tokens: &'a [Token], source_map: &'a SourceMap) -> Self {
        Self {
            tokens,
            pos: 0,
            source_map,
        }
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

            if self.check(TokenKind::Module) || self.check(TokenKind::Macromodule) {
                items.push(Item::Module(self.parse_module()?));
            } else if self.check(TokenKind::Discipline) {
                items.push(Item::Discipline(self.parse_discipline()?));
            } else if self.check(TokenKind::Nature) {
                items.push(Item::Nature(self.parse_nature()?));
            } else if self.check(TokenKind::Eof) {
                break;
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

    /// Parse a module definition
    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'module' or 'macromodule'

        let name = self.expect_identifier("module name")?;
        let mut module = Module::new(name, start);

        // Optional port list
        if self.check(TokenKind::LParen) {
            module.ports = self.parse_port_list()?;
        }
        self.expect(TokenKind::Semicolon)?;

        // Module body
        while !self.check(TokenKind::Endmodule) && !self.at_end() {
            self.parse_module_item(&mut module)?;
        }

        self.expect(TokenKind::Endmodule)?;
        module.span = start.extend(self.previous_span());
        Ok(module)
    }

    /// Parse port list: (port1, port2, ...)
    fn parse_port_list(&mut self) -> Result<Vec<Port>, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut ports = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                let span = self.current_span();
                let name = self.expect_identifier("port name")?;
                ports.push(Port {
                    name: name.into(),
                    span,
                });

                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        Ok(ports)
    }

    /// Parse a module item (declaration or statement)
    fn parse_module_item(&mut self, module: &mut Module) -> Result<(), ParseError> {
        match self.current().kind {
            TokenKind::Input | TokenKind::Output | TokenKind::Inout => {
                let decl = self.parse_port_declaration()?;
                module.port_declarations.push(decl);
            }
            TokenKind::Parameter => {
                let param = self.parse_parameter()?;
                module.parameters.push(param);
            }
            TokenKind::Localparam => {
                let param = self.parse_parameter()?;
                module.localparams.push(param);
            }
            TokenKind::Real | TokenKind::Integer => {
                let var = self.parse_variable_decl()?;
                module.variables.push(var);
            }
            TokenKind::Electrical
            | TokenKind::Voltage
            | TokenKind::Current
            | TokenKind::Identifier => {
                // Could be discipline or net declaration
                if self.is_discipline_keyword() {
                    let net = self.parse_net_decl()?;
                    module.nets.push(net);
                } else {
                    // Skip unknown declarations
                    self.skip_to_semicolon()?;
                }
            }
            TokenKind::Ground => {
                let net = self.parse_ground_decl()?;
                module.nets.push(net);
            }
            TokenKind::Analog => {
                // Check if this is an analog function declaration
                let next_kind = self.tokens.get(self.pos + 1).map(|t| t.kind);
                if next_kind == Some(TokenKind::Function) {
                    let func = self.parse_analog_function()?;
                    module.functions.push(func);
                } else {
                    let block = self.parse_analog_block()?;
                    if self.previous_was_initial() {
                        module.analog_initial = Some(block);
                    } else {
                        module.analog_block = Some(block);
                    }
                }
            }
            _ => {
                // Skip unknown items
                self.skip_to_semicolon()?;
            }
        }
        Ok(())
    }

    /// Parse port declaration: input/output/inout [discipline] names;
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

        // Optional discipline
        let discipline = if self.is_discipline_keyword() {
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

    /// Parse parameter declaration
    fn parse_parameter(&mut self) -> Result<ParameterDecl, ParseError> {
        let start = self.current_span();
        self.advance(); // consume 'parameter' or 'localparam'

        // Optional type
        let param_type = if self.match_token(TokenKind::Real) {
            ParamType::Real
        } else if self.match_token(TokenKind::Integer) {
            ParamType::Integer
        } else if self.match_token(TokenKind::String) {
            ParamType::String
        } else {
            ParamType::Real // default
        };

        let name = self.expect_identifier("parameter name")?;

        // Optional default value
        let default = if self.match_token(TokenKind::Assign_) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Optional range constraint: from [lb:ub] or from (lb:ub]
        let range = if self.match_token(TokenKind::From) {
            Some(self.parse_parameter_range()?)
        } else {
            None
        };

        self.expect(TokenKind::Semicolon)?;
        Ok(ParameterDecl {
            param_type,
            name: name.into(),
            default,
            range,
            units: None,
            description: None,
            attributes: Vec::new(),
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse parameter range
    fn parse_parameter_range(&mut self) -> Result<ParameterRange, ParseError> {
        let start = self.current_span();
        let mut bounds = Vec::new();

        // Parse bound: [lb:ub], (lb:ub), [lb:ub), (lb:ub]
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

        bounds.push(RangeBound {
            lower,
            lower_inclusive,
            upper,
            upper_inclusive,
            span: start.extend(self.previous_span()),
        });

        Ok(ParameterRange {
            bounds,
            exclude: Vec::new(),
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse variable declaration: real/integer var1, var2;
    fn parse_variable_decl(&mut self) -> Result<VariableDecl, ParseError> {
        let start = self.current_span();
        let var_type = if self.match_token(TokenKind::Real) {
            VarType::Real
        } else if self.match_token(TokenKind::Integer) {
            VarType::Integer
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
            items.push(VariableItem {
                name: name.into(),
                dimensions: Vec::new(),
                init: None,
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
        let statements = match stmt {
            AnalogStatement::Block(block) => block.statements,
            other => vec![other],
        };

        Ok(AnalogBlock {
            statements,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse analog function declaration
    ///
    /// Syntax: analog function [return_type] function_name;
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
                    // Try to parse as a statement or skip
                    if self.check(TokenKind::Semicolon) {
                        self.advance(); // skip empty semicolons
                    } else {
                        // Unknown, skip to next semicolon
                        self.skip_to_semicolon()?;
                    }
                }
            }
        }

        self.expect(TokenKind::Endfunction)?;

        Ok(FunctionDef {
            name: name.into(),
            return_type,
            params,
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
        let start = self.current_span();

        match self.current().kind {
            TokenKind::Begin => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::While => self.parse_while_statement(),
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
            _ => {
                // Keywords might be used as variable names (e.g., voltage, current)
                // Try to parse as assignment/contribution
                self.parse_assignment_or_contribution()
            }
        }
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
        while !self.check(TokenKind::End) && !self.at_end() {
            statements.push(self.parse_analog_statement()?);
        }

        self.expect(TokenKind::End)?;
        Ok(AnalogStatement::Block(BlockStmt {
            name,
            statements,
            variables: Vec::new(),
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
                self.expect(TokenKind::Colon)?;
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

    /// Parse event expression
    fn parse_event_expr(&mut self) -> Result<EventExpr, ParseError> {
        let start = self.current_span();

        // Simple events for now
        if self.match_token(TokenKind::Initial) {
            // initial_step
            return Ok(EventExpr::InitialStep { span: start });
        }

        if self.check(TokenKind::Identifier) {
            let name = self.expect_identifier("event")?;
            if name == "initial_step" {
                return Ok(EventExpr::InitialStep { span: start });
            }
            if name == "final_step" {
                return Ok(EventExpr::FinalStep { span: start });
            }
            // cross, above, timer would go here
        }

        // Default to a simple cross event
        let signal = self.parse_expression()?;
        Ok(EventExpr::Cross {
            signal,
            direction: None,
            tolerance: None,
            span: start.extend(self.previous_span()),
        })
    }

    /// Parse assignment or contribution statement
    fn parse_assignment_or_contribution(&mut self) -> Result<AnalogStatement, ParseError> {
        let start = self.current_span();
        let saved_pos = self.pos; // Save position for backtracking

        // Check if current token is V or I followed by (
        if self.check(TokenKind::Identifier) {
            let name = self.current().text.as_deref().unwrap_or("");
            if (name == "V" || name == "I") && self.peek_is(TokenKind::LParen) {
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
                    } else {
                        // Not a contribution, restore position and parse as normal
                        self.pos = saved_pos;
                    }
                } else {
                    // Failed to parse branch access, restore position
                    self.pos = saved_pos;
                }
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

        // Must be assignment
        let name = self.expect_identifier("variable")?;
        self.expect(TokenKind::Assign_)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(AnalogStatement::Assignment(AssignmentStmt {
            target: LValue::Variable {
                name: name.into(),
                span: start,
            },
            value,
            span: start.extend(self.previous_span()),
        }))
    }

    /// Try to parse a branch access (V(a,b) or I(a,b))
    fn try_parse_branch_access(&mut self) -> Result<BranchAccess, ParseError> {
        let start = self.current_span();
        let access = self.expect_identifier("access function")?;

        // Must be V or I
        if access != "V" && access != "I" {
            return Err(self.error(ParseErrorKind::InvalidBranchAccess));
        }

        self.expect(TokenKind::LParen)?;
        let pos = self.expect_identifier("node")?;
        let neg = if self.match_token(TokenKind::Comma) {
            Some(self.expect_identifier("node")?.into())
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
        let mut left = self.parse_comparison()?;

        while self.match_token(TokenKind::And) {
            let right = self.parse_comparison()?;
            left = Expression::Binary(BinaryExpr {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span: start.extend(self.previous_span()),
            });
        }

        Ok(left)
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        let mut left = self.parse_additive()?;

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
                self.advance();
                let value = parse_number(&text);
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
                        let pos = self.expect_identifier("node")?;
                        let neg = if self.match_token(TokenKind::Comma) {
                            Some(self.expect_identifier("node")?.into())
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

    /// Parse argument list
    fn parse_arg_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut args = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
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
                self.skip_to_semicolon()?;
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

        self.expect(TokenKind::Semicolon)?;

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
                self.skip_to_semicolon()?;
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
        } else {
            Err(ParseError::expected(
                context.to_string(),
                format!("{:?}", self.current().kind),
                self.current_span(),
            ))
        }
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

    fn previous_was_initial(&self) -> bool {
        if self.pos >= 2 {
            matches!(self.tokens[self.pos - 2].kind, TokenKind::Initial)
        } else {
            false
        }
    }

    fn skip_directive(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip directive token
        // Skip until newline (handled by lexer in practice)
        Ok(())
    }

    fn skip_to_semicolon(&mut self) -> Result<(), ParseError> {
        while !self.check(TokenKind::Semicolon) && !self.at_end() {
            self.advance();
        }
        if self.check(TokenKind::Semicolon) {
            self.advance();
        }
        Ok(())
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError::new(kind, self.current_span())
    }
}

/// Parse a number literal with scale factors
fn parse_number(s: &str) -> f64 {
    let s = s.trim();

    // Check for scale factors at the end
    let (num_str, scale) = if let Some(pos) = s.find(|c: char| {
        matches!(
            c,
            'T' | 'G' | 'M' | 'k' | 'K' | 'm' | 'u' | 'n' | 'p' | 'f' | 'a'
        )
    }) {
        let scale_str = &s[pos..];
        let scale = match scale_str {
            "T" => 1e12,
            "G" => 1e9,
            "M" | "meg" | "Meg" | "MEG" => 1e6,
            "k" | "K" => 1e3,
            "m" if !scale_str.starts_with("meg") => 1e-3,
            "u" => 1e-6,
            "n" => 1e-9,
            "p" => 1e-12,
            "f" => 1e-15,
            "a" => 1e-18,
            _ => 1.0,
        };
        (&s[..pos], scale)
    } else {
        (s, 1.0)
    };

    num_str.parse::<f64>().unwrap_or(0.0) * scale
}
