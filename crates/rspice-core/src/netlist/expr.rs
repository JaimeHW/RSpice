//! Expression evaluator for SPICE .PARAM statements
//!
//! Supports:
//! - Basic arithmetic: +, -, *, /, ** (power)
//! - Parentheses for grouping
//! - Built-in functions: sqrt, sin, cos, tan, exp, log, log10, abs, min, max
//! - Parameter substitution from context

use super::lexer::parse_spice_value;
use crate::Value;
use std::collections::HashMap;

//=============================================================================
// Expression AST
//=============================================================================

/// Expression node in the AST
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal
    Number(Value),
    /// Parameter reference
    Param(String),
    /// Binary operation
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation
    UnaryOp { op: UnaryOpKind, operand: Box<Expr> },
    /// Function call
    FnCall { name: String, args: Vec<Expr> },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    // Comparison
    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=
    Eq, // ==
    Ne, // !=
    // Boolean
    And, // &&
    Or,  // ||
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Neg, // -
    Pos, // +
    Not, // !
}

//=============================================================================
// Expression Parser
//=============================================================================

/// Parser for SPICE expressions
struct ExprParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> ExprParser<'a> {
    fn new(input: &'a str) -> Self {
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
    fn parse(&mut self) -> Result<Expr, ExprError> {
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
                if self.consume('=') {
                    self.skip_ws();
                    let right = self.parse_additive()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Eq,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    self.pos = start_pos;
                    break;
                }
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

//=============================================================================
// Expression Evaluator
//=============================================================================

/// User-defined function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Argument names
    pub args: Vec<String>,
    /// Function body expression string
    pub body: String,
}

impl FunctionDef {
    /// Create a new function definition
    pub fn new(name: &str, args: Vec<String>, body: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            args: args.into_iter().map(|a| a.to_uppercase()).collect(),
            body: body.to_string(),
        }
    }
}

/// Context for parameter substitution during evaluation
#[derive(Debug, Clone, Default)]
pub struct ParamContext {
    params: HashMap<String, Value>,
    /// User-defined functions (.FUNC)
    functions: HashMap<String, FunctionDef>,
}

impl ParamContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a parameter value
    pub fn set(&mut self, name: &str, value: Value) {
        self.params.insert(name.to_uppercase(), value);
    }

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<Value> {
        self.params.get(&name.to_uppercase()).copied()
    }

    /// Merge another context into this one
    pub fn merge(&mut self, other: &ParamContext) {
        for (k, v) in &other.params {
            self.params.insert(k.clone(), *v);
        }
        for (k, v) in &other.functions {
            self.functions.insert(k.clone(), v.clone());
        }
    }

    /// Get all parameters as a vector of (name, value) tuples
    pub fn all_params(&self) -> Vec<(String, Value)> {
        self.params.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    /// Define a user function
    ///
    /// # Example
    /// ```ignore
    /// ctx.define_function("SQUARE", vec!["X".to_string()], "X*X");
    /// ctx.define_function("RPAR", vec!["R1".to_string(), "R2".to_string()], "R1*R2/(R1+R2)");
    /// ```
    pub fn define_function(&mut self, name: &str, args: Vec<String>, body: &str) {
        let func = FunctionDef::new(name, args, body);
        self.functions.insert(func.name.clone(), func);
    }

    /// Check if a user function is defined
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_uppercase())
    }

    /// Get a user function definition
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(&name.to_uppercase())
    }

    /// Call a user-defined function with the given argument values
    ///
    /// Creates a temporary context with the arguments bound to their values
    /// and evaluates the function body.
    pub fn call_function(&self, name: &str, arg_values: &[Value]) -> Result<Value, ExprError> {
        let func = self
            .get_function(name)
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;

        if arg_values.len() != func.args.len() {
            return Err(ExprError::WrongArgCount(name.to_string()));
        }

        // Create temporary context with arguments bound
        let mut temp_ctx = self.clone();
        for (arg_name, &arg_value) in func.args.iter().zip(arg_values.iter()) {
            temp_ctx.set(arg_name, arg_value);
        }

        // Parse and evaluate the function body
        let expr = parse_expression(&func.body)?;
        evaluate(&expr, &temp_ctx)
    }
}

/// Evaluate an expression with the given context
pub fn evaluate(expr: &Expr, ctx: &ParamContext) -> Result<Value, ExprError> {
    match expr {
        Expr::Number(v) => Ok(*v),

        Expr::Param(name) => ctx
            .get(name)
            .ok_or_else(|| ExprError::UndefinedParam(name.clone())),

        Expr::BinOp { op, left, right } => {
            let l = evaluate(left, ctx)?;
            let r = evaluate(right, ctx)?;

            Ok(match op {
                // Arithmetic
                BinOpKind::Add => l + r,
                BinOpKind::Sub => l - r,
                BinOpKind::Mul => l * r,
                BinOpKind::Div => {
                    if r.abs() < 1e-300 {
                        return Err(ExprError::DivisionByZero);
                    }
                    l / r
                }
                BinOpKind::Pow => l.powf(r),
                // Comparison (return 1.0 for true, 0.0 for false)
                BinOpKind::Gt => {
                    if l > r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Lt => {
                    if l < r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Ge => {
                    if l >= r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Le => {
                    if l <= r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Eq => {
                    if (l - r).abs() < 1e-12 {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Ne => {
                    if (l - r).abs() >= 1e-12 {
                        1.0
                    } else {
                        0.0
                    }
                }
                // Boolean (0.0 is false, anything else is true)
                BinOpKind::And => {
                    if l != 0.0 && r != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Or => {
                    if l != 0.0 || r != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            })
        }

        Expr::UnaryOp { op, operand } => {
            let v = evaluate(operand, ctx)?;
            Ok(match op {
                UnaryOpKind::Neg => -v,
                UnaryOpKind::Pos => v,
                UnaryOpKind::Not => {
                    if v == 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            })
        }

        Expr::FnCall { name, args } => eval_function(name, args, ctx),
    }
}

/// Evaluate a built-in or user-defined function
fn eval_function(name: &str, args: &[Expr], ctx: &ParamContext) -> Result<Value, ExprError> {
    // First check for user-defined functions
    if ctx.has_function(name) {
        let arg_values: Vec<Value> = args
            .iter()
            .map(|e| evaluate(e, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        return ctx.call_function(name, &arg_values);
    }

    let get_arg = |idx: usize| -> Result<Value, ExprError> {
        args.get(idx)
            .ok_or_else(|| ExprError::WrongArgCount(name.to_string()))
            .and_then(|e| evaluate(e, ctx))
    };

    match name {
        "SQRT" => Ok(get_arg(0)?.sqrt()),
        "SIN" => Ok(get_arg(0)?.sin()),
        "COS" => Ok(get_arg(0)?.cos()),
        "TAN" => Ok(get_arg(0)?.tan()),
        "ASIN" => Ok(get_arg(0)?.asin()),
        "ACOS" => Ok(get_arg(0)?.acos()),
        "ATAN" => Ok(get_arg(0)?.atan()),
        "ATAN2" => Ok(get_arg(0)?.atan2(get_arg(1)?)),
        "EXP" => Ok(get_arg(0)?.exp()),
        "LOG" | "LN" => Ok(get_arg(0)?.ln()),
        "LOG10" => Ok(get_arg(0)?.log10()),
        "ABS" => Ok(get_arg(0)?.abs()),
        "FLOOR" => Ok(get_arg(0)?.floor()),
        "CEIL" => Ok(get_arg(0)?.ceil()),
        "ROUND" => Ok(get_arg(0)?.round()),
        "MIN" => Ok(get_arg(0)?.min(get_arg(1)?)),
        "MAX" => Ok(get_arg(0)?.max(get_arg(1)?)),
        "POW" | "PWR" => Ok(get_arg(0)?.powf(get_arg(1)?)),
        "LIMIT" => {
            // LIMIT(x, min, max)
            let x = get_arg(0)?;
            let min = get_arg(1)?;
            let max = get_arg(2)?;
            Ok(x.clamp(min, max))
        }
        "IF" => {
            // IF(cond, then, else)
            let cond = get_arg(0)?;
            if cond != 0.0 { get_arg(1) } else { get_arg(2) }
        }
        "URAMP" => {
            // URAMP(x) = x if x > 0, else 0
            let x = get_arg(0)?;
            Ok(if x > 0.0 { x } else { 0.0 })
        }
        "SGN" | "SIGN" => {
            let x = get_arg(0)?;
            Ok(if x > 0.0 {
                1.0
            } else if x < 0.0 {
                -1.0
            } else {
                0.0
            })
        }
        "TABLE" => {
            // TABLE(x, x1, y1, x2, y2, ...) - piecewise linear interpolation
            // Used extensively in vendor models for nonlinear behavior
            if args.len() < 3 {
                return Err(ExprError::WrongArgCount("TABLE".to_string()));
            }

            let x = get_arg(0)?;

            // Extract (x, y) pairs from remaining arguments
            let mut points: Vec<(Value, Value)> = Vec::new();
            let mut i = 1;
            while i + 1 < args.len() {
                let xi = get_arg(i)?;
                let yi = get_arg(i + 1)?;
                points.push((xi, yi));
                i += 2;
            }

            Ok(table_interpolate(x, &points))
        }
        "PWL" => {
            // PWL is an alias for TABLE (both do piecewise linear)
            if args.len() < 3 {
                return Err(ExprError::WrongArgCount("PWL".to_string()));
            }

            let x = get_arg(0)?;
            let mut points: Vec<(Value, Value)> = Vec::new();
            let mut i = 1;
            while i + 1 < args.len() {
                let xi = get_arg(i)?;
                let yi = get_arg(i + 1)?;
                points.push((xi, yi));
                i += 2;
            }

            Ok(table_interpolate(x, &points))
        }
        // Hyperbolic trigonometric functions
        "SINH" => Ok(get_arg(0)?.sinh()),
        "COSH" => Ok(get_arg(0)?.cosh()),
        "TANH" => Ok(get_arg(0)?.tanh()),
        "ASINH" => Ok(get_arg(0)?.asinh()),
        "ACOSH" => Ok(get_arg(0)?.acosh()),
        "ATANH" => Ok(get_arg(0)?.atanh()),
        "ARCTAN" => Ok(get_arg(0)?.atan()), // Alias for atan
        // Integer/rounding functions
        "INT" | "TRUNC" => Ok(get_arg(0)?.trunc()),
        "NINT" => {
            // Nearest integer (round half towards even - banker's rounding)
            let x = get_arg(0)?;
            Ok(x.round_ties_even())
        }
        "SQR" => {
            // Square of x
            let x = get_arg(0)?;
            Ok(x * x)
        }
        // Step functions for behavioral modeling
        "U" | "USTEP" => {
            // Unit step: 1 if x > 0, 0.5 if x == 0, 0 if x < 0
            let x = get_arg(0)?;
            Ok(if x > 0.0 {
                1.0
            } else if x == 0.0 {
                0.5
            } else {
                0.0
            })
        }
        "U2" => {
            // Smooth step: 0 for x<=0, x for 0<x<1, 1 for x>=1
            let x = get_arg(0)?;
            Ok(x.clamp(0.0, 1.0))
        }
        // Comparison functions (return 0 or 1)
        "EQ0" => {
            let x = get_arg(0)?;
            Ok(if x.abs() < 1e-12 { 1.0 } else { 0.0 })
        }
        "NE0" => {
            let x = get_arg(0)?;
            Ok(if x.abs() >= 1e-12 { 1.0 } else { 0.0 })
        }
        "GT0" => {
            let x = get_arg(0)?;
            Ok(if x > 0.0 { 1.0 } else { 0.0 })
        }
        "LT0" => {
            let x = get_arg(0)?;
            Ok(if x < 0.0 { 1.0 } else { 0.0 })
        }
        "GE0" => {
            let x = get_arg(0)?;
            Ok(if x >= 0.0 { 1.0 } else { 0.0 })
        }
        "LE0" => {
            let x = get_arg(0)?;
            Ok(if x <= 0.0 { 1.0 } else { 0.0 })
        }
        // Pseudo-random (constant for DC, deterministic)
        "RAND" | "RANDOM" => {
            // Return a pseudo-random value (for testing, use a hash)
            Ok(0.5) // Constant for DC evaluation
        }
        _ => Err(ExprError::UnknownFunction(name.to_string())),
    }
}

/// Piecewise linear interpolation for TABLE function
///
/// Linearly interpolates between defined points.
/// For x outside the defined range, extrapolates from the nearest segment.
#[inline]
fn table_interpolate(x: Value, points: &[(Value, Value)]) -> Value {
    if points.is_empty() {
        return 0.0;
    }

    if points.len() == 1 {
        return points[0].1;
    }

    // Sort points by x (should already be sorted in valid input)
    // For performance, we assume input is sorted

    // Handle x below first point - extrapolate
    if x <= points[0].0 {
        if points.len() >= 2 {
            let (x0, y0) = points[0];
            let (x1, y1) = points[1];
            if (x1 - x0).abs() > 1e-18 {
                let slope = (y1 - y0) / (x1 - x0);
                return y0 + slope * (x - x0);
            }
        }
        return points[0].1;
    }

    // Handle x above last point - extrapolate
    let last = points.len() - 1;
    if x >= points[last].0 {
        if points.len() >= 2 {
            let (x0, y0) = points[last - 1];
            let (x1, y1) = points[last];
            if (x1 - x0).abs() > 1e-18 {
                let slope = (y1 - y0) / (x1 - x0);
                return y1 + slope * (x - x1);
            }
        }
        return points[last].1;
    }

    // Find bracketing points and interpolate
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];

        if x >= x0 && x <= x1 {
            if (x1 - x0).abs() < 1e-18 {
                return y0;
            }
            let t = (x - x0) / (x1 - x0);
            return y0 + t * (y1 - y0);
        }
    }

    // Fallback (shouldn't reach here)
    points[last].1
}

//=============================================================================
// Public API
//=============================================================================

/// Parse a SPICE expression string into an AST
pub fn parse_expression(input: &str) -> Result<Expr, ExprError> {
    let mut parser = ExprParser::new(input);
    parser.parse()
}

/// Parse and evaluate a SPICE expression with the given context
pub fn eval_expression(input: &str, ctx: &ParamContext) -> Result<Value, ExprError> {
    let expr = parse_expression(input)?;
    evaluate(&expr, ctx)
}

/// Evaluate a simple expression without parameters
pub fn eval_simple(input: &str) -> Result<Value, ExprError> {
    eval_expression(input, &ParamContext::new())
}

//=============================================================================
// Errors
//=============================================================================

/// Errors that can occur during expression parsing/evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum ExprError {
    InvalidNumber(String),
    UnexpectedChar(char),
    MissingCloseParen,
    TrailingInput(String),
    UndefinedParam(String),
    UnknownFunction(String),
    WrongArgCount(String),
    DivisionByZero,
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ExprError::UnexpectedChar(c) => write!(f, "Unexpected character: '{}'", c),
            ExprError::MissingCloseParen => write!(f, "Missing closing parenthesis"),
            ExprError::TrailingInput(s) => write!(f, "Trailing input: {}", s),
            ExprError::UndefinedParam(s) => write!(f, "Undefined parameter: {}", s),
            ExprError::UnknownFunction(s) => write!(f, "Unknown function: {}", s),
            ExprError::WrongArgCount(s) => write!(f, "Wrong argument count for function: {}", s),
            ExprError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl std::error::Error for ExprError {}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: Value, b: Value) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn test_simple_arithmetic() {
        assert!(approx_eq(eval_simple("1+2").unwrap(), 3.0));
        assert!(approx_eq(eval_simple("5-3").unwrap(), 2.0));
        assert!(approx_eq(eval_simple("2*3").unwrap(), 6.0));
        assert!(approx_eq(eval_simple("6/2").unwrap(), 3.0));
    }

    #[test]
    fn test_operator_precedence() {
        assert!(approx_eq(eval_simple("2+3*4").unwrap(), 14.0));
        assert!(approx_eq(eval_simple("(2+3)*4").unwrap(), 20.0));
        assert!(approx_eq(eval_simple("2*3+4").unwrap(), 10.0));
    }

    #[test]
    fn test_power() {
        assert!(approx_eq(eval_simple("2**3").unwrap(), 8.0));
        assert!(approx_eq(eval_simple("2^3").unwrap(), 8.0));
        assert!(approx_eq(eval_simple("4**0.5").unwrap(), 2.0));
    }

    #[test]
    fn test_unary() {
        assert!(approx_eq(eval_simple("-5").unwrap(), -5.0));
        assert!(approx_eq(eval_simple("--5").unwrap(), 5.0));
        assert!(approx_eq(eval_simple("2*-3").unwrap(), -6.0));
        assert!(approx_eq(eval_simple("-2**2").unwrap(), -4.0));
        assert!(approx_eq(eval_simple("-2^2").unwrap(), -4.0));
    }

    #[test]
    fn test_spice_values() {
        assert!(approx_eq(eval_simple("1k+500").unwrap(), 1500.0));
        assert!(approx_eq(eval_simple("1MEG/1k").unwrap(), 1000.0));
        assert!(approx_eq(eval_simple("2.2u*1k").unwrap(), 2.2e-3));
    }

    #[test]
    fn test_functions() {
        assert!(approx_eq(eval_simple("sqrt(4)").unwrap(), 2.0));
        assert!(approx_eq(eval_simple("abs(-5)").unwrap(), 5.0));
        assert!(approx_eq(eval_simple("max(3, 5)").unwrap(), 5.0));
        assert!(approx_eq(eval_simple("min(3, 5)").unwrap(), 3.0));
        assert!(approx_eq(eval_simple("pow(2, 3)").unwrap(), 8.0));
    }

    #[test]
    fn test_if_uses_nonzero_truthiness() {
        assert!(approx_eq(eval_simple("IF(0.1, 42, 43)").unwrap(), 42.0));
        assert!(approx_eq(eval_simple("IF(-0.1, 42, 43)").unwrap(), 42.0));
        assert!(approx_eq(eval_simple("IF(0.0, 42, 43)").unwrap(), 43.0));
    }

    #[test]
    fn test_params() {
        let mut ctx = ParamContext::new();
        ctx.set("R", 1000.0);
        ctx.set("C", 1e-6);

        assert!(approx_eq(eval_expression("R*C", &ctx).unwrap(), 1e-3));
        assert!(approx_eq(eval_expression("R+1k", &ctx).unwrap(), 2000.0));
    }

    #[test]
    fn test_nested_functions() {
        assert!(approx_eq(
            eval_simple("sqrt(pow(3, 2) + pow(4, 2))").unwrap(),
            5.0
        ));
    }

    #[test]
    fn test_complex_expression() {
        let mut ctx = ParamContext::new();
        ctx.set("VCC", 5.0);
        ctx.set("BETA", 100.0);

        // RC = VCC / 10mA
        assert!(approx_eq(eval_expression("VCC/10m", &ctx).unwrap(), 500.0));
    }

    #[test]
    fn test_undefined_param() {
        let result = eval_simple("UNDEFINED+1");
        assert!(matches!(result, Err(ExprError::UndefinedParam(_))));
    }

    #[test]
    fn test_division_by_zero() {
        let result = eval_simple("1/0");
        assert!(matches!(result, Err(ExprError::DivisionByZero)));
    }

    #[test]
    fn test_user_defined_function_simple() {
        let mut ctx = ParamContext::new();
        ctx.define_function("square", vec!["x".to_string()], "x*x");

        ctx.set("v", 3.0);
        let result = eval_expression("square(v)", &ctx).unwrap();
        assert!(approx_eq(result, 9.0));
    }

    #[test]
    fn test_user_defined_function_multi_arg() {
        let mut ctx = ParamContext::new();
        // Parallel resistance function: R1||R2 = R1*R2/(R1+R2)
        ctx.define_function(
            "rpar",
            vec!["r1".to_string(), "r2".to_string()],
            "r1*r2/(r1+r2)",
        );

        let result = eval_expression("rpar(1000, 1000)", &ctx).unwrap();
        assert!(approx_eq(result, 500.0));

        // 1k || 2k = 1000*2000/3000 = 2000000/3000 = 666.666...
        let result = eval_expression("rpar(1k, 2k)", &ctx).unwrap();
        let expected = 2000.0 / 3.0; // Exact value = 666.666...
        assert!(
            (result - expected).abs() < 1e-6,
            "Expected {}, got {}",
            expected,
            result
        );
    }

    #[test]
    fn test_user_defined_function_nested() {
        let mut ctx = ParamContext::new();
        ctx.define_function("double", vec!["x".to_string()], "2*x");
        ctx.define_function("quad", vec!["x".to_string()], "double(double(x))");

        let result = eval_expression("quad(5)", &ctx).unwrap();
        assert!(approx_eq(result, 20.0));
    }

    #[test]
    fn test_user_defined_function_with_builtin() {
        let mut ctx = ParamContext::new();
        ctx.define_function(
            "cabs",
            vec!["r".to_string(), "i".to_string()],
            "sqrt(r*r+i*i)",
        );

        let result = eval_expression("cabs(3, 4)", &ctx).unwrap();
        assert!(approx_eq(result, 5.0));
    }

    #[test]
    fn test_table_interpolation() {
        let ctx = ParamContext::new();

        // TABLE(x, x1,y1, x2,y2, ...) - simple linear interpolation
        // Points: (0,0), (1,10), (2,20)
        let result = eval_expression("TABLE(0.5, 0,0, 1,10, 2,20)", &ctx).unwrap();
        assert!(approx_eq(result, 5.0)); // Midpoint between (0,0) and (1,10)

        let result = eval_expression("TABLE(1.5, 0,0, 1,10, 2,20)", &ctx).unwrap();
        assert!(approx_eq(result, 15.0)); // Midpoint between (1,10) and (2,20)
    }

    #[test]
    fn test_table_extrapolation() {
        let ctx = ParamContext::new();

        // Extrapolate below first point
        let result = eval_expression("TABLE(-0.5, 0,0, 1,10)", &ctx).unwrap();
        assert!(approx_eq(result, -5.0)); // Extrapolate with slope 10

        // Extrapolate above last point
        let result = eval_expression("TABLE(2.0, 0,0, 1,10)", &ctx).unwrap();
        assert!(approx_eq(result, 20.0)); // Extrapolate with slope 10
    }

    #[test]
    fn test_table_at_points() {
        let ctx = ParamContext::new();

        // Value exactly at defined points
        let result = eval_expression("TABLE(0, 0,5, 1,10)", &ctx).unwrap();
        assert!((result - 5.0).abs() < 1e-10);

        let result = eval_expression("TABLE(1, 0,5, 1,10)", &ctx).unwrap();
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_table_nonlinear_curve() {
        let ctx = ParamContext::new();

        // Model a diode-like I-V curve with TABLE
        // Points: (0,0) -> (0.6,0.001) -> (0.7,0.1) -> (0.8,1.0)
        // At x=0.5, we're in the first segment (0,0) to (0.6,0.001)
        // Interpolation: t = 0.5/0.6 = 0.833..., y = 0 + 0.833*(0.001-0) ≈ 0.00083
        let result = eval_expression("TABLE(0.5, 0,0, 0.6,0.001, 0.7,0.1, 0.8,1.0)", &ctx).unwrap();
        // x=0.5 is between first point (0,0) and (0.6,0.001)
        assert!(
            (0.0..0.001).contains(&result),
            "Expected ~0.00083, got {}",
            result
        );
    }
}
