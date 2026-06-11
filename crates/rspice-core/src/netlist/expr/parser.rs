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

    /// Parse power expressions (^ / **), left-associative like ngspice
    /// numparam: `2^3^2` = `(2^3)^2`. A sign in exponent position applies
    /// to the immediate operand only and the chain keeps folding left, so
    /// `2^-3^2` = `(2^-3)^2`.
    fn parse_power(&mut self) -> Result<Expr, ExprError> {
        let mut base = self.parse_primary()?;

        loop {
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

            if !is_power {
                break;
            }

            self.skip_ws();
            let exp = self.parse_power_exponent()?;
            base = Expr::BinOp {
                op: BinOpKind::Pow,
                left: Box::new(base),
                right: Box::new(exp),
            };
        }

        Ok(base)
    }

    /// Exponent operand: optional signs ahead of a primary, nothing more.
    fn parse_power_exponent(&mut self) -> Result<Expr, ExprError> {
        self.skip_ws();
        if self.consume('-') {
            let operand = self.parse_power_exponent()?;
            Ok(Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                operand: Box::new(operand),
            })
        } else if self.consume('+') {
            self.parse_power_exponent()
        } else {
            self.parse_primary()
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
    ///
    /// Mirrors ngspice numparam (xpressn.c `fetchnumber`): a scale suffix
    /// may follow even after a scientific exponent (`1e3k` = 1e6) and any
    /// remaining letters are swallowed (`10kOhm`, `1MegHz`). Expressions
    /// have no `mil` scale, unlike netlist value positions: `1mil` is
    /// milli with `il` swallowed.
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

        // A scientific exponent needs at least one digit; otherwise the
        // `e`/`E` is a unit letter (`2e` = 2.0, like `2x`).
        if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
            let mut j = i + 1;
            if j < chars.len() && (chars[j] == '+' || chars[j] == '-') {
                j += 1;
            }
            let digits_start = j;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j > digits_start {
                i = j;
            }
        }

        let numeric_end = i;

        // ngspice numparam scale factors (xpressn.c `parseunit`).
        let mut multiplier = 1.0;
        if i < chars.len() && chars[i].is_ascii_alphabetic() {
            let is_meg = i + 3 <= chars.len()
                && chars[i..i + 3]
                    .iter()
                    .collect::<String>()
                    .eq_ignore_ascii_case("meg");
            multiplier = if is_meg {
                1e6
            } else {
                match chars[i].to_ascii_uppercase() {
                    'T' => 1e12,
                    'G' => 1e9,
                    'K' => 1e3,
                    'M' => 1e-3,
                    'U' => 1e-6,
                    'N' => 1e-9,
                    'P' => 1e-12,
                    'F' => 1e-15,
                    'A' => 1e-18,
                    _ => 1.0,
                }
            };
            // Swallow the unit and any trailing letters (`10kOhm`).
            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                i += 1;
            }
        }

        let byte_len: usize = chars[..i].iter().map(|c| c.len_utf8()).sum();
        self.pos = start + byte_len;

        let num_str: String = chars[..numeric_end].iter().collect();
        match num_str.parse::<f64>() {
            Ok(v) => Ok(Expr::Number(v * multiplier)),
            Err(_) => Err(ExprError::InvalidNumber(num_str)),
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
