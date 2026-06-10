use super::*;

//=============================================================================
// Expression Parser
//=============================================================================

/// Parser for SPICE expressions
pub(in crate::netlist::expr) struct ExprParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> ExprParser<'a> {
    pub(in crate::netlist::expr) fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// Skip whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_whitespace() {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    /// Peek at current character
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Consume current character
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Check if current char matches
    fn check(&self, c: char) -> bool {
        self.peek() == Some(c)
    }

    /// Consume if current char matches
    fn consume(&mut self, c: char) -> bool {
        if self.check(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Parse an expression (entry point)
    pub(in crate::netlist::expr) fn parse(&mut self) -> Result<Expr, ExprError> {
        self.skip_ws();
        let expr = self.parse_ternary()?;
        self.skip_ws();
        if self.pos < self.input.len() {
            Err(ExprError::TrailingInput(self.input[self.pos..].to_string()))
        } else {
            Ok(expr)
        }
    }

    /// Parse ternary expressions (cond ? then : else) - lowest precedence
    fn parse_ternary(&mut self) -> Result<Expr, ExprError> {
        let cond = self.parse_or()?;

        self.skip_ws();
        if self.consume('?') {
            self.skip_ws();
            let then_expr = self.parse_ternary()?;
            self.skip_ws();
            if !self.consume(':') {
                return Err(ExprError::UnexpectedChar(self.peek().unwrap_or('\0')));
            }
            self.skip_ws();
            let else_expr = self.parse_ternary()?;
            // Convert to IF function call
            Ok(Expr::FnCall {
                name: "IF".to_string(),
                args: vec![cond, then_expr, else_expr],
            })
        } else {
            Ok(cond)
        }
    }

    /// Parse boolean OR expressions (||)
    fn parse_or(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_and()?;

        loop {
            self.skip_ws();
            let start_pos = self.pos;
            if self.consume('|') {
                if self.consume('|') {
                    self.skip_ws();
                    let right = self.parse_and()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Or,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.pos = start_pos;
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse boolean AND expressions (&&)
    fn parse_and(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_comparison()?;

        loop {
            self.skip_ws();
            let start_pos = self.pos;
            if self.consume('&') {
                if self.consume('&') {
                    self.skip_ws();
                    let right = self.parse_comparison()?;
                    left = Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.pos = start_pos;
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse comparison expressions (>, <, >=, <=, ==, !=)
    fn parse_comparison(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_additive()?;

        loop {
            self.skip_ws();
            let start_pos = self.pos;

            if self.consume('>') {
                if self.consume('=') {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Ge,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Gt,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
            } else if self.consume('<') {
                if self.consume('=') {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Le,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else if self.consume('>') {
                    // ngspice numparam inequality spelling: `a <> b`.
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Ne,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Lt,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
            } else if self.consume('=') {
                // Both `==` and the ngspice numparam single `=` compare for
                // equality; assignment never occurs inside an expression.
                self.consume('=');
                self.skip_ws();
                let right = self.parse_additive()?;
                left = Expr::BinOp {
                    op: BinOpKind::Eq,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.consume('!') {
                if self.consume('=') {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Ne,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.pos = start_pos;
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse additive expressions (+, -)
    fn parse_additive(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_ws();
            if self.consume('+') {
                self.skip_ws();
                let right = self.parse_multiplicative()?;
                left = Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.consume('-') {
                self.skip_ws();
                let right = self.parse_multiplicative()?;
                left = Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse multiplicative expressions (*, /)
    fn parse_multiplicative(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_unary()?;

        loop {
            self.skip_ws();
            if self.consume('*') {
                if self.consume('*') {
                    self.pos -= 2;
                    break;
                }
                self.skip_ws();
                let right = self.parse_unary()?;
                left = Expr::BinOp {
                    op: BinOpKind::Mul,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.consume('/') {
                self.skip_ws();
                let right = self.parse_unary()?;
                left = Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse power expressions (^)
    fn parse_power(&mut self) -> Result<Expr, ExprError> {
        let base = self.parse_primary()?;

        self.skip_ws();
        let op_start = self.pos;
        let is_power = if self.consume('^') {
            true
        } else if self.consume('*') {
            if self.consume('*') {
                true
            } else {
                self.pos = op_start;
                false
            }
        } else {
            false
        };

        if is_power {
            self.skip_ws();
            let exp = self.parse_unary()?;
            Ok(Expr::BinOp {
                op: BinOpKind::Pow,
                left: Box::new(base),
                right: Box::new(exp),
            })
        } else {
            Ok(base)
        }
    }

    /// Parse unary expressions (+, -, !)
    fn parse_unary(&mut self) -> Result<Expr, ExprError> {
        self.skip_ws();

        if self.consume('-') {
            self.skip_ws();
            let operand = self.parse_unary()?;
            Ok(Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                operand: Box::new(operand),
            })
        } else if self.consume('+') {
            self.skip_ws();
            self.parse_unary()
        } else if self.consume('!') {
            // Check this isn't != operator (already consumed by parse_comparison)
            if !self.check('=') {
                self.skip_ws();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOpKind::Not,
                    operand: Box::new(operand),
                })
            } else {
                // Put back the '!' - this shouldn't happen in well-formed input
                self.pos -= 1;
                self.parse_power()
            }
        } else {
            self.parse_power()
        }
    }

    /// Parse primary expressions (numbers, params, functions, parens)
    fn parse_primary(&mut self) -> Result<Expr, ExprError> {
        self.skip_ws();

        // Parenthesized expression
        if self.consume('(') {
            let expr = self.parse_ternary()?; // Full expression inside parens
            self.skip_ws();
            if !self.consume(')') {
                return Err(ExprError::MissingCloseParen);
            }
            return Ok(expr);
        }

        // Number
        if let Some(c) = self.peek()
            && (c.is_ascii_digit() || c == '.')
        {
            return self.parse_number();
        }

        // Identifier (parameter or function)
        if let Some(c) = self.peek()
            && (c.is_ascii_alphabetic() || c == '_')
        {
            return self.parse_ident_or_fn();
        }

        Err(ExprError::UnexpectedChar(self.peek().unwrap_or('\0')))
    }

    /// Parse a number with optional SPICE suffix
    fn parse_number(&mut self) -> Result<Expr, ExprError> {
        let start = self.pos;
        let chars: Vec<char> = self.input[start..].chars().collect();
        let mut i = 0;

        // Consume integer part
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }

        // Consume decimal part
        if i < chars.len() && chars[i] == '.' {
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
        }

        // Consume exponent part
        if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
            i += 1;
            if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                i += 1;
            }
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
        } else {
            // Only consume SPICE suffix if NOT scientific notation
            // Check for MEG first (3 chars)
            let mut consumed_suffix = false;
            if i + 3 <= chars.len() {
                let suffix3: String = chars[i..i + 3].iter().collect();
                if suffix3.eq_ignore_ascii_case("meg") {
                    i += 3;
                    consumed_suffix = true;
                }
            }
            // Then check for single-char suffixes (k, u, n, p, f, m, g, t)
            if !consumed_suffix && i < chars.len() {
                let c = chars[i].to_ascii_uppercase();
                if matches!(c, 'K' | 'M' | 'U' | 'N' | 'P' | 'F' | 'G' | 'T' | 'A') {
                    i += 1;
                }
            }
        }

        // Calculate actual position
        let byte_len: usize = chars[..i].iter().map(|c| c.len_utf8()).sum();
        self.pos = start + byte_len;

        let num_str = &self.input[start..self.pos];
        match parse_spice_value(num_str) {
            Ok(v) => Ok(Expr::Number(v)),
            Err(_) => Err(ExprError::InvalidNumber(num_str.to_string())),
        }
    }

    /// Parse identifier or function call
    fn parse_ident_or_fn(&mut self) -> Result<Expr, ExprError> {
        let start = self.pos;

        // Consume identifier
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let name = self.input[start..self.pos].to_uppercase();

        self.skip_ws();

        // Check for function call
        if self.consume('(') {
            let mut args = Vec::new();

            self.skip_ws();
            if !self.check(')') {
                args.push(self.parse_additive()?);

                loop {
                    self.skip_ws();
                    if self.consume(',') {
                        self.skip_ws();
                        args.push(self.parse_additive()?);
                    } else {
                        break;
                    }
                }
            }

            self.skip_ws();
            if !self.consume(')') {
                return Err(ExprError::MissingCloseParen);
            }

            Ok(Expr::FnCall { name, args })
        } else {
            // Parameter reference
            Ok(Expr::Param(name))
        }
    }
}
