//! Expression parser for behavioral sources
//!
//! Parses expressions like `V(2)*I(L1)+sin(TIME)` into AST.
//! Uses a simple recursive descent parser with operator precedence.

use super::ast::{BinaryOp, Expr, Function, UnaryOp};
use std::iter::Peekable;
use std::str::Chars;

/// Expression parse error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}

/// Token types for expression parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number { value: f64, lexeme: String },
    InvalidNumber(String),
    Ident(String),
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret, // ^
    // Comparison
    Lt,
    Le,
    Gt,
    Ge,
    Eq, // ==
    Ne, // !=
    // Logical
    And, // &&
    Or,  // ||
    Not, // !
    // Delimiters
    LParen,
    RParen,
    Comma,
    Invalid(char),
    // End of input
    Eof,
}

/// Tokenizer for expressions
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        // A scientific exponent counts only when at least one digit
        // follows; otherwise the `e` is a unit letter (`2e` = 2.0).
        if matches!(self.chars.peek(), Some(&c) if c == 'e' || c == 'E') {
            let mut probe = self.chars.clone();
            probe.next();
            let mut span = 1usize;
            if matches!(probe.peek(), Some(&c) if c == '+' || c == '-') {
                probe.next();
                span += 1;
            }
            let mut digits = 0usize;
            while matches!(probe.peek(), Some(c) if c.is_ascii_digit()) {
                probe.next();
                digits += 1;
            }
            if digits > 0 {
                for _ in 0..span + digits {
                    s.push(self.chars.next().unwrap());
                }
            }
        }

        let base: f64 = match s.parse() {
            Ok(value) => value,
            Err(_) => return Token::InvalidNumber(s),
        };

        // ngspice B-source numbers go through INPevaluate: an engineering
        // suffix may follow the digits (even after an exponent, `1e3k` =
        // 1e6) and any remaining letters are swallowed (`10kOhm`). Unlike
        // parameter expressions, `mil` = 25.4e-6 exists here.
        let mut multiplier = 1.0;
        let mut tail = String::new();
        if matches!(self.chars.peek(), Some(c) if c.is_ascii_alphabetic()) {
            while matches!(self.chars.peek(), Some(c) if c.is_ascii_alphabetic()) {
                tail.push(self.chars.next().unwrap());
            }
            let upper = tail.to_ascii_uppercase();
            multiplier = if upper.starts_with("MEG") {
                1e6
            } else if upper.starts_with("MIL") {
                25.4e-6
            } else {
                match upper.as_bytes()[0] {
                    b'T' => 1e12,
                    b'G' => 1e9,
                    b'K' => 1e3,
                    b'M' => 1e-3,
                    b'U' => 1e-6,
                    b'N' => 1e-9,
                    b'P' => 1e-12,
                    b'F' => 1e-15,
                    b'A' => 1e-18,
                    _ => 1.0,
                }
            };
        }

        let lexeme = if tail.is_empty() {
            s
        } else {
            format!("{s}{tail}")
        };

        Token::Number {
            value: base * multiplier,
            lexeme,
        }
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            let is_bang_name_char = c == '!' && {
                let mut probe = self.chars.clone();
                probe.next();
                probe.peek() != Some(&'=')
            };
            if c.is_alphanumeric()
                || c == '_'
                || c == '.'
                || c == '#'
                || c == ':'
                || is_bang_name_char
            {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Token::Eof,
            Some(&c) => match c {
                '0'..='9' | '.' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => Token::Ident(self.read_ident()),
                '+' => {
                    self.chars.next();
                    Token::Plus
                }
                '-' => {
                    self.chars.next();
                    Token::Minus
                }
                '*' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'*') {
                        self.chars.next();
                        Token::Caret // ngspice accepts ** as power
                    } else {
                        Token::Star
                    }
                }
                '/' => {
                    self.chars.next();
                    Token::Slash
                }
                '^' => {
                    self.chars.next();
                    Token::Caret
                }
                '(' => {
                    self.chars.next();
                    Token::LParen
                }
                ')' => {
                    self.chars.next();
                    Token::RParen
                }
                ',' => {
                    self.chars.next();
                    Token::Comma
                }
                '<' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        Token::Le
                    } else {
                        Token::Lt
                    }
                }
                '>' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        Token::Ge
                    } else {
                        Token::Gt
                    }
                }
                '=' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        Token::Eq
                    } else {
                        Token::Eq // Single = also means equality in SPICE
                    }
                }
                '!' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        Token::Ne
                    } else {
                        Token::Not
                    }
                }
                '&' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'&') {
                        self.chars.next();
                    }
                    Token::And
                }
                '|' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'|') {
                        self.chars.next();
                    }
                    Token::Or
                }
                _ => {
                    self.chars.next();
                    Token::Invalid(c)
                }
            },
        }
    }
}

/// Expression parser using recursive descent with precedence climbing
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Self {
            lexer,
            current,
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> bool {
        if std::mem::discriminant(&self.current) == std::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            self.errors.push(format!(
                "Expected token {:?}, found {:?}",
                expected, self.current
            ));
            false
        }
    }

    /// Parse a complete expression
    pub fn parse(&mut self) -> Expr {
        let expr = self.parse_or();
        while self.current != Token::Eof {
            match self.current {
                Token::Invalid(c) => self
                    .errors
                    .push(format!("Invalid character '{}' in expression", c)),
                Token::InvalidNumber(ref s) => self
                    .errors
                    .push(format!("Invalid number '{}' in expression", s)),
                _ => self.errors.push(format!(
                    "Unexpected trailing token in expression: {:?}",
                    self.current
                )),
            }
            self.advance();
        }
        expr
    }

    // Precedence levels (lowest to highest):
    // 1. || (or)
    // 2. && (and)
    // 3. ==, != (equality)
    // 4. <, <=, >, >= (comparison)
    // 5. +, - (additive)
    // 6. *, / (multiplicative)
    // 7. ^ (power)
    // 8. unary -, !
    // 9. function calls, atoms

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while self.current == Token::Or {
            self.advance();
            let right = self.parse_and();
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();
        while self.current == Token::And {
            self.advance();
            let right = self.parse_equality();
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        loop {
            let op = match &self.current {
                Token::Eq => BinaryOp::Eq,
                Token::Ne => BinaryOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();
        loop {
            let op = match &self.current {
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();
        loop {
            let op = match &self.current {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_unary();
        loop {
            let op = match &self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_power(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while self.current == Token::Caret {
            self.advance();
            // ngspice's B-source parser folds power chains left
            // (2^3^2 = (2^3)^2) but lets a signed exponent capture the
            // whole following unary/power chain (2^-3^2 = 2^(-(3^2))).
            let right = match self.current {
                Token::Minus | Token::Plus | Token::Not => self.parse_unary(),
                _ => self.parse_primary(),
            };
            left = Expr::Binary {
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match &self.current {
            Token::Minus => {
                self.advance();
                Expr::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(self.parse_unary()),
                }
            }
            Token::Plus => {
                self.advance();
                self.parse_unary()
            }
            Token::Not => {
                self.advance();
                Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(self.parse_unary()),
                }
            }
            _ => self.parse_power(),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current.clone() {
            Token::Number { value, .. } => {
                self.advance();
                Expr::Const(value)
            }
            Token::Ident(name) => {
                self.advance();
                self.parse_ident_or_call(&name)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_or();
                self.expect(Token::RParen);
                expr
            }
            Token::Invalid(c) => {
                self.errors
                    .push(format!("Invalid character '{}' in expression", c));
                self.advance();
                Expr::Const(0.0)
            }
            Token::InvalidNumber(s) => {
                self.errors
                    .push(format!("Invalid number '{}' in expression", s));
                self.advance();
                Expr::Const(0.0)
            }
            other => {
                self.errors
                    .push(format!("Unexpected token {:?} in expression", other));
                self.advance();
                Expr::Const(0.0)
            }
        }
    }

    fn parse_ident_or_call(&mut self, name: &str) -> Expr {
        let upper = name.to_uppercase();

        // Check for V(node) or I(element)
        if (upper == "V" || upper == "I") && self.current == Token::LParen {
            self.advance(); // consume (
            let parse_ref_arg = |token: &Token| match token {
                Token::Ident(arg) => Some(arg.clone()),
                Token::Number { lexeme, .. } => Some(lexeme.clone()),
                _ => None,
            };

            if let Some(first_arg) = parse_ref_arg(&self.current) {
                self.advance();

                // Differential voltage probe form: V(node_pos, node_neg)
                if upper == "V" && self.current == Token::Comma {
                    self.advance();
                    if let Some(second_arg) = parse_ref_arg(&self.current) {
                        self.advance();
                        self.expect(Token::RParen);
                        return Expr::sub(
                            Expr::NodeVoltage(first_arg),
                            Expr::NodeVoltage(second_arg),
                        );
                    }
                    self.errors.push(
                        "Expected node name after comma in differential voltage probe".to_string(),
                    );
                    self.expect(Token::RParen);
                    return Expr::Const(0.0);
                }

                self.expect(Token::RParen);
                return if upper == "V" {
                    Expr::NodeVoltage(first_arg)
                } else {
                    Expr::BranchCurrent(first_arg)
                };
            }
        }

        // Check for constants and analysis variables.
        if upper == "PI" {
            return Expr::Const(std::f64::consts::PI);
        }

        if upper == "TIME" || upper == "T" {
            return Expr::Time;
        }
        // `hertz` is ngspice's AC-frequency variable. The behavioral AC
        // linearization is evaluated at the operating point, so in AC
        // sweeps it reads as the context frequency where one is supplied
        // and 0 elsewhere (DC/transient), matching ngspice's DC behavior.
        if upper == "FREQ" || upper == "FREQUENCY" || upper == "F" || upper == "HERTZ" {
            return Expr::Frequency;
        }
        if upper == "TEMPER" {
            return Expr::Temperature;
        }

        // Check for function call
        if self.current == Token::LParen {
            self.advance();
            let mut args = Vec::new();

            if self.current != Token::RParen {
                args.push(self.parse_or());
                while self.current == Token::Comma {
                    self.advance();
                    args.push(self.parse_or());
                }
            }
            self.expect(Token::RParen);

            let func = match upper.as_str() {
                "ABS" | "M" => Some(Function::Abs),
                "SQRT" => Some(Function::Sqrt),
                "EXP" => Some(Function::Exp),
                "LOG" | "LN" => Some(Function::Log),
                "LOG10" => Some(Function::Log10),
                "SIN" => Some(Function::Sin),
                "COS" => Some(Function::Cos),
                "TAN" => Some(Function::Tan),
                "ASIN" | "ARCSIN" => Some(Function::Asin),
                "ACOS" | "ARCCOS" => Some(Function::Acos),
                "ATAN" | "ARCTAN" => Some(Function::Atan),
                "ATAN2" => Some(Function::Atan2),
                "SINH" => Some(Function::Sinh),
                "COSH" => Some(Function::Cosh),
                "TANH" => Some(Function::Tanh),
                "ASINH" => Some(Function::Asinh),
                "ACOSH" => Some(Function::Acosh),
                "ATANH" => Some(Function::Atanh),
                "FLOOR" | "INT" => Some(Function::Floor),
                "CEIL" | "CEILING" => Some(Function::Ceil),
                "ROUND" | "NINT" => Some(Function::Round),
                "SQR" => Some(Function::Sqr),
                "MIN" => Some(Function::Min),
                "MAX" => Some(Function::Max),
                "POW" => Some(Function::Pow),
                "PWR" => Some(Function::Pwr),
                "PWRS" => Some(Function::Pwrs),
                "LIMIT" => Some(Function::Limit),
                "SIGN" | "SGN" => Some(Function::Sign),
                "URAMP" => Some(Function::Uramp),
                "STP" | "STEP" | "U" => Some(Function::Stp),
                "U2" => Some(Function::U2),
                "EQ0" => Some(Function::Eq0),
                "NE0" => Some(Function::Ne0),
                "GT0" => Some(Function::Gt0),
                "LT0" => Some(Function::Lt0),
                "GE0" => Some(Function::Ge0),
                "LE0" => Some(Function::Le0),
                "TABLE" => Some(Function::Table),
                "PWL" => Some(Function::Pwl),
                "MOD" | "FMOD" => Some(Function::Mod),
                "IF" => Some(Function::If),
                _ => None,
            };

            if let Some(f) = func {
                return Expr::Function { func: f, args };
            }

            self.errors
                .push(format!("Unknown function '{}' in expression", name));
            return Expr::Const(0.0);
        }

        // Unknown identifier is an error in strict behavioral expressions.
        self.errors
            .push(format!("Unknown identifier '{}' in expression", name));
        Expr::Const(0.0)
    }
}

/// Parse an expression string into AST with strict error reporting.
pub fn parse_expression_strict(input: &str) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(input);
    let expr = parser.parse();
    if parser.errors.is_empty() {
        Ok(expr)
    } else {
        Err(ParseError::new(parser.errors.join("; ")))
    }
}

/// Parse an expression string into AST
pub fn parse_expression(input: &str) -> Expr {
    match parse_expression_strict(input) {
        Ok(expr) => expr,
        Err(err) => {
            log::warn!(
                "Behavioral expression parse fallback for '{}': {}",
                input,
                err
            );
            Expr::Const(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Context, Vm, compile};

    fn eval_const(input: &str) -> f64 {
        let ast = parse_expression_strict(input)
            .unwrap_or_else(|e| panic!("parse `{input}` failed: {e}"));
        let program = compile(&ast);
        let mut vm = Vm::new();
        vm.execute(&program, &Context::dc(&[], &[]))
    }

    #[test]
    fn power_operator_matches_ngspice_b_sources() {
        // Pinned against ngspice-46 B-source oracle runs: chains fold
        // left, but a signed exponent captures the rest of the chain
        // (inpptree gives 2^-3^2 = 2^(-(3^2)), unlike numparam).
        assert_eq!(eval_const("2^3^2"), 64.0);
        assert_eq!(eval_const("-2^2"), -4.0);
        assert_eq!(eval_const("3-2^2"), -1.0);
        assert_eq!(eval_const("2^-3^2"), 0.001953125);
        assert_eq!(eval_const("2^-3^2*4"), 0.0078125);
        assert_eq!(eval_const("3*-2^2"), -12.0);
        assert_eq!(eval_const("2**-2"), 0.25);
        assert_eq!(eval_const("-2**2"), -4.0);
        assert_eq!(eval_const("2**3**2"), 64.0);
    }

    #[test]
    fn pi_constant_matches_xyce_expression_semantics() {
        assert!((eval_const("sin(pi/2)") - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn m_function_alias_matches_xyce_magnitude_semantics() {
        assert_eq!(eval_const("m(-5)"), 5.0);
        assert_eq!(eval_const("M(3)"), 3.0);
    }

    #[test]
    fn pow_pwr_and_pwrs_keep_distinct_spice_sign_semantics() {
        assert_eq!(eval_const("pow(-2,2)"), 4.0);
        assert_eq!(eval_const("pow(-2,3)"), -8.0);
        assert_eq!(eval_const("pwr(-2,3)"), 8.0);
        assert_eq!(eval_const("pwrs(-2,3)"), -8.0);
    }

    #[test]
    fn xyce_unary_step_and_sign_zero_semantics() {
        assert_eq!(eval_const("stp(-1)"), 0.0);
        assert_eq!(eval_const("stp(-1e-15)"), 0.0);
        assert_eq!(eval_const("stp(0)"), 0.0);
        assert_eq!(eval_const("stp(1e-15)"), 0.0);
        assert_eq!(eval_const("stp(1e-9)"), 1.0);
        assert_eq!(eval_const("stp(1)"), 1.0);
        assert_eq!(eval_const("sgn(-1)"), -1.0);
        assert_eq!(eval_const("sgn(-1e-15)"), 0.0);
        assert_eq!(eval_const("sgn(0)"), 0.0);
        assert_eq!(eval_const("sgn(1e-15)"), 0.0);
        assert_eq!(eval_const("sgn(1e-9)"), 1.0);
        assert_eq!(eval_const("sgn(1)"), 1.0);
    }

    #[test]
    fn probe_references_preserve_numeric_looking_node_lexemes() {
        assert_eq!(
            parse_expression_strict("V(2a)").expect("probe parses"),
            Expr::NodeVoltage("2a".to_string())
        );
        assert_eq!(
            parse_expression_strict("V(2e3,0)").expect("differential probe parses"),
            Expr::sub(
                Expr::NodeVoltage("2e3".to_string()),
                Expr::NodeVoltage("0".to_string())
            )
        );
        assert_eq!(
            parse_expression_strict("V(2D,gnd!)").expect("global-ground probe parses"),
            Expr::sub(
                Expr::NodeVoltage("2D".to_string()),
                Expr::NodeVoltage("gnd!".to_string())
            )
        );
        assert!(
            parse_expression_strict("1!=0").is_ok(),
            "allowing bang in node names must not consume the != operator"
        );
    }

    #[test]
    fn numbers_accept_inpevaluate_suffixes() {
        // B-source numbers go through INPevaluate in ngspice: engineering
        // suffixes work (`2k`), `mil` is 25.4e-6 (unlike numparam), the
        // scale applies after an exponent, and unit letters are swallowed.
        assert_eq!(eval_const("2k"), 2000.0);
        assert_eq!(eval_const("1mil*1e6"), 25.4e-6 * 1e6);
        assert_eq!(eval_const("10kohm"), 10_000.0);
        assert_eq!(eval_const("1e3k/1e6"), 1.0);
        assert_eq!(eval_const("3meg"), 3e6);
        assert_eq!(eval_const("100n"), 100.0 * 1e-9);
        assert_eq!(eval_const("1a"), 1e-18);
        assert_eq!(eval_const("5xyz"), 5.0);
    }

    #[test]
    fn invalid_numeric_lexemes_are_rejected_in_strict_parse() {
        for input in [".", "1..2"] {
            let err = parse_expression_strict(input)
                .expect_err("invalid numeric expression must not be coerced to zero");
            assert!(
                err.to_string().contains("Invalid number"),
                "unexpected error for {input}: {err}"
            );
        }
    }
}
