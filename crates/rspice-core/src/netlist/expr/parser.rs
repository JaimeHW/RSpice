//! Recursive-descent parser from expression text to [`Expr`].
//!
//! Handles operator precedence and associativity, parenthesized grouping,
//! quoted expressions, SPICE engineering-suffix numbers, and function-call
//! syntax. Function *names* are not validated here — an unknown name parses
//! successfully and fails at evaluation with
//! [`ExprError::UnknownFunction`], so a deck that never evaluates a branch
//! never trips over it.

use super::*;

//=============================================================================
// Expression Parser
//=============================================================================

/// Parser for SPICE expressions
pub(in crate::netlist::expr) struct ExprParser<'a> {
    input: &'a str,
    pos: usize,
    abort: Option<&'a dyn crate::abort_signal::AbortSignal>,
    aborted: bool,
}

impl<'a> ExprParser<'a> {
    pub(in crate::netlist::expr) fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            abort: None,
            aborted: false,
        }
    }

    pub(in crate::netlist::expr) fn with_abort(
        input: &'a str,
        abort: &'a dyn crate::abort_signal::AbortSignal,
    ) -> Self {
        Self {
            input,
            pos: 0,
            abort: Some(abort),
            aborted: false,
        }
    }

    pub(in crate::netlist::expr) fn was_aborted(&self) -> bool {
        self.aborted
    }

    fn poll_abort(&mut self) -> bool {
        if self.aborted
            || self
                .abort
                .is_some_and(crate::abort_signal::AbortSignal::is_aborted)
        {
            self.aborted = true;
            true
        } else {
            false
        }
    }

    /// Skip whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.input.len() {
            if self.poll_abort() {
                return;
            }
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
        if self.poll_abort() {
            return None;
        }
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
            self.advance().is_some()
        } else {
            false
        }
    }

    /// Parse an expression (entry point)
    pub(in crate::netlist::expr) fn parse(&mut self) -> Result<Expr, ExprError> {
        if self.poll_abort() {
            return Err(ExprError::UnexpectedChar('\0'));
        }
        self.skip_ws();
        let expr = self.parse_ternary()?;
        self.skip_ws();
        if self.aborted {
            Err(ExprError::UnexpectedChar('\0'))
        } else if self.pos < self.input.len() {
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

    /// Parse boolean OR expressions (`|` in Xyce, `||` in HSPICE).
    fn parse_or(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_and()?;

        loop {
            self.skip_ws();
            if self.consume('|') {
                self.consume('|');
                self.skip_ws();
                let right = self.parse_and()?;
                left = Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse boolean AND expressions (`&` in Xyce, `&&` in HSPICE).
    fn parse_and(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_comparison()?;

        loop {
            self.skip_ws();
            if self.consume('&') {
                self.consume('&');
                self.skip_ws();
                let right = self.parse_comparison()?;
                left = Expr::BinOp {
                    op: BinOpKind::And,
                    left: Box::new(left),
                    right: Box::new(right),
                };
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
        let mut left = self.parse_modulo()?;

        loop {
            self.skip_ws();
            if self.consume('+') {
                self.skip_ws();
                let right = self.parse_modulo()?;
                left = Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.consume('-') {
                self.skip_ws();
                let right = self.parse_modulo()?;
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

    /// Parse modulo expressions (%), lower precedence than * and / like Xyce.
    fn parse_modulo(&mut self) -> Result<Expr, ExprError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_ws();
            if self.consume('%') {
                self.skip_ws();
                let right = self.parse_multiplicative()?;
                left = Expr::BinOp {
                    op: BinOpKind::Mod,
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

        // Xyce treats braces as ordinary expression-grouping delimiters at
        // every nesting depth, alongside parentheses.
        if self.check('(') || self.check('{') {
            let open = self.advance().expect("checked grouping delimiter");
            let close = if open == '(' { ')' } else { '}' };
            let expr = self.parse_ternary()?; // Full expression inside parens
            self.skip_ws();
            if !self.consume(close) {
                return Err(if open == '(' {
                    ExprError::MissingCloseParen
                } else {
                    ExprError::TrailingInput("missing closing brace".to_string())
                });
            }
            return Ok(expr);
        }

        // SPICE and Xyce use single quotes as expression delimiters, with
        // the same grouping semantics as braces or parentheses. Double
        // quotes remain reserved for string values such as table filenames.
        if self.consume('\'') {
            let expr = self.parse_ternary()?;
            self.skip_ws();
            if !self.consume('\'') {
                return Err(ExprError::MissingCloseQuote);
            }
            return Ok(expr);
        }

        // Double quotes denote string literals in file-backed expression
        // functions such as TABLE("wave.dat").  Preserve the unquoted value
        // in the AST; the runtime behavioral compiler performs the actual
        // file lookup after parameter expansion.
        if self.consume('"') {
            let mut value = String::new();
            loop {
                let Some(character) = self.advance() else {
                    return Err(ExprError::MissingCloseQuote);
                };
                match character {
                    '"' => break,
                    '\\' => {
                        let Some(escaped) = self.advance() else {
                            return Err(ExprError::MissingCloseQuote);
                        };
                        value.push(escaped);
                    }
                    _ => value.push(character),
                }
            }
            return Ok(Expr::StringLiteral(value));
        }

        // Number
        if let Some(c) = self.peek()
            && (c.is_ascii_digit() || c == '.')
        {
            return self.parse_number();
        }

        // Identifier (parameter or function)
        if let Some(c) = self.peek()
            && is_expr_ident_start(c)
        {
            return self.parse_ident_or_fn();
        }

        Err(ExprError::UnexpectedChar(self.peek().unwrap_or('\0')))
    }

    /// Parse a number with optional SPICE suffix
    ///
    /// Shapes follow ngspice numparam (xpressn.c `fetchnumber`): a scale
    /// suffix may follow even after a scientific exponent (`1e3k` = 1e6) and
    /// any remaining letters are swallowed (`10kOhm`, `1MegHz`). The scale
    /// table is [`crate::netlist::lexer::spice_suffix_scale`], the same one
    /// every netlist value position reads.
    fn parse_number(&mut self) -> Result<Expr, ExprError> {
        let start = self.pos;
        while self.peek().is_some_and(|ch| ch.is_ascii_digit()) {
            if self.advance().is_none() {
                break;
            }
        }

        if self.check('.') {
            self.advance();
            while self.peek().is_some_and(|ch| ch.is_ascii_digit()) {
                if self.advance().is_none() {
                    break;
                }
            }
        }

        // A scientific exponent needs at least one digit; otherwise the
        // `e`/`E` is a unit letter (`2e` = 2.0, like `2x`).
        if self.peek().is_some_and(|ch| matches!(ch, 'e' | 'E')) {
            let exponent_start = self.pos;
            self.advance();
            if self.peek().is_some_and(|ch| matches!(ch, '+' | '-')) {
                self.advance();
            }
            let digits_start = self.pos;
            while self.peek().is_some_and(|ch| ch.is_ascii_digit()) {
                if self.advance().is_none() {
                    break;
                }
            }
            if self.pos == digits_start && !self.aborted {
                self.pos = exponent_start;
            }
        }

        let numeric_end = self.pos;

        // Like ngspice numparam (xpressn.c `parseunit`), the unit and any
        // trailing letters are swallowed (`10kOhm`). The scale itself comes
        // from the deck lexer's one suffix table, so a number means the same
        // thing here as in a value position: `1a` is a unit ampere rather
        // than numparam's atto, and `1mil` is 25.4e-6 rather than milli.
        // Xyce's engineering-expression imaginary suffix (`2.0J`) is the one
        // suffix a value position has no use for.
        let mut multiplier = 1.0;
        let mut imaginary_literal = false;
        if self.peek().is_some_and(|ch| ch.is_ascii_alphabetic()) {
            let suffix_start = self.pos;
            while self.peek().is_some_and(|ch| ch.is_ascii_alphabetic()) {
                if self.advance().is_none() {
                    break;
                }
            }
            let suffix = &self.input[suffix_start..self.pos];
            let first = suffix
                .chars()
                .next()
                .expect("alphabetic suffix was just observed");
            imaginary_literal = first.eq_ignore_ascii_case(&'j');
            if !imaginary_literal {
                multiplier = crate::netlist::lexer::spice_suffix_scale(suffix).0;
            }
        }

        let num_str = &self.input[start..numeric_end];
        match num_str.parse::<f64>() {
            Ok(v) => {
                let value = v * multiplier;
                if imaginary_literal {
                    Ok(Expr::ComplexNumber(ComplexValue::new(0.0, value)))
                } else {
                    Ok(Expr::Number(value))
                }
            }
            Err(_) => Err(ExprError::InvalidNumber(num_str.to_string())),
        }
    }

    /// Parse identifier or function call
    fn parse_ident_or_fn(&mut self) -> Result<Expr, ExprError> {
        let start = self.pos;

        self.advance();
        while let Some(c) = self.peek() {
            if is_expr_ident_continue(c) {
                if self.advance().is_none() {
                    break;
                }
            } else {
                break;
            }
        }

        let name = self.input[start..self.pos].to_uppercase();

        self.skip_ws();

        // Check for function call
        if self.consume('(') {
            if is_raw_probe_accessor(&name) {
                let mut args = Vec::new();
                loop {
                    self.skip_ws();
                    let argument_start = self.pos;
                    while let Some(ch) = self.peek() {
                        if matches!(ch, ',' | ')') {
                            break;
                        }
                        if self.advance().is_none() {
                            break;
                        }
                    }
                    let argument = self.input[argument_start..self.pos].trim();
                    if argument.is_empty() {
                        if args.is_empty() && self.check(')') {
                            break;
                        }
                        return Err(ExprError::UnexpectedChar(self.peek().unwrap_or('\0')));
                    }
                    args.push(Expr::Param(argument.to_uppercase()));
                    self.skip_ws();
                    if self.consume(',') {
                        continue;
                    }
                    break;
                }
                if !self.consume(')') {
                    return Err(ExprError::MissingCloseParen);
                }
                return Ok(Expr::FnCall { name, args });
            }
            let mut args = Vec::new();

            self.skip_ws();
            if !self.check(')') {
                args.push(self.parse_ternary()?);

                loop {
                    self.skip_ws();
                    if self.consume(',') {
                        self.skip_ws();
                        args.push(self.parse_ternary()?);
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
            match name.as_str() {
                "PI" => Ok(Expr::Number(std::f64::consts::PI)),
                "EXP" => Ok(Expr::Number(std::f64::consts::E)),
                "TRUE" => Ok(Expr::Number(1.0)),
                "FALSE" => Ok(Expr::Number(0.0)),
                _ => Ok(Expr::Param(name)),
            }
        }
    }
}

fn is_expr_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c, '_' | '`' | '@' | '#' | '$')
}

fn is_expr_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '`' | '@' | '#' | '.' | '$')
}

fn is_raw_probe_accessor(name: &str) -> bool {
    matches!(
        name,
        "V" | "VM"
            | "VR"
            | "VI"
            | "VP"
            | "VDB"
            | "I"
            | "IM"
            | "IR"
            | "II"
            | "IP"
            | "IDB"
            | "DNO"
            | "DNI"
    )
}
