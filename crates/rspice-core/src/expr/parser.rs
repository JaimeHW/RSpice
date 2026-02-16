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
    Number(f64),
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

    fn read_number(&mut self) -> f64 {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit()
                || c == '.'
                || c == 'e'
                || c == 'E'
                || c == '-' && s.ends_with('e')
                || c == '-' && s.ends_with('E')
                || c == '+' && s.ends_with('e')
                || c == '+' && s.ends_with('E')
            {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        s.parse().unwrap_or(0.0)
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':' {
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
                '0'..='9' | '.' => Token::Number(self.read_number()),
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
                    Token::Star
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
        let mut left = self.parse_power();
        loop {
            let op = match &self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_power();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_power(&mut self) -> Expr {
        let left = self.parse_unary();
        if self.current == Token::Caret {
            self.advance();
            let right = self.parse_power(); // Right associative
            Expr::Binary {
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
            }
        } else {
            left
        }
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
            Token::Not => {
                self.advance();
                Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(self.parse_unary()),
                }
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current.clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Const(n)
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
                Token::Number(n) => Some(n.to_string()),
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

        // Check for TIME or FREQ
        if upper == "TIME" || upper == "T" {
            return Expr::Time;
        }
        if upper == "FREQ" || upper == "FREQUENCY" || upper == "F" {
            return Expr::Frequency;
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
                "ABS" => Some(Function::Abs),
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
                "POW" | "PWR" => Some(Function::Pwr),
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

    #[test]
    fn test_parse_number() {
        let expr = parse_expression("42.5");
        assert_eq!(expr, Expr::Const(42.5));
    }

    #[test]
    fn test_parse_voltage() {
        let expr = parse_expression("V(2)");
        assert_eq!(expr, Expr::NodeVoltage("2".to_string()));
    }

    #[test]
    fn test_parse_current() {
        let expr = parse_expression("I(L1)");
        assert_eq!(expr, Expr::BranchCurrent("L1".to_string()));
    }

    #[test]
    fn test_parse_differential_voltage() {
        let expr = parse_expression("V(10,40)");
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Sub,
                left: Box::new(Expr::NodeVoltage("10".to_string())),
                right: Box::new(Expr::NodeVoltage("40".to_string())),
            }
        );
    }

    #[test]
    fn test_parse_binary() {
        let expr = parse_expression("V(1) + V(2)");
        match expr {
            Expr::Binary {
                op: BinaryOp::Add, ..
            } => (),
            _ => panic!("Expected addition"),
        }
    }

    #[test]
    fn test_parse_multiply() {
        let expr = parse_expression("2 * V(1)");
        match expr {
            Expr::Binary {
                op: BinaryOp::Mul, ..
            } => (),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_parse_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let expr = parse_expression("2 + 3 * 4");
        match expr {
            Expr::Binary {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                assert_eq!(*left, Expr::Const(2.0));
                match *right {
                    Expr::Binary {
                        op: BinaryOp::Mul, ..
                    } => (),
                    _ => panic!("Expected multiplication on right"),
                }
            }
            _ => panic!("Expected addition at top"),
        }
    }

    #[test]
    fn test_parse_function() {
        let expr = parse_expression("sin(V(1))");
        match expr {
            Expr::Function {
                func: Function::Sin,
                args,
            } => {
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_time() {
        let expr = parse_expression("TIME");
        assert_eq!(expr, Expr::Time);
    }

    #[test]
    fn test_parse_complex() {
        // V(2) * 2 + sin(TIME)
        let expr = parse_expression("V(2) * 2 + sin(TIME)");
        match expr {
            Expr::Binary {
                op: BinaryOp::Add, ..
            } => (),
            _ => panic!("Expected addition at top"),
        }
    }

    #[test]
    fn test_parse_expression_strict_rejects_unknown_identifier() {
        let err = parse_expression_strict("FOO + 1").expect_err("strict parser should reject FOO");
        assert!(err.to_string().contains("Unknown identifier"));
    }

    #[test]
    fn test_parse_expression_strict_rejects_unknown_function() {
        let err = parse_expression_strict("MYSTERY(1)")
            .expect_err("strict parser should reject function");
        assert!(err.to_string().contains("Unknown function"));
    }

    #[test]
    fn test_parse_expression_strict_rejects_invalid_character() {
        let err = parse_expression_strict("V(1) @ 2")
            .expect_err("strict parser should reject invalid character");
        assert!(err.to_string().contains("Invalid character"));
    }

    #[test]
    fn test_parse_expression_strict_rejects_missing_rparen() {
        let err = parse_expression_strict("SIN(V(1)")
            .expect_err("strict parser should reject missing parenthesis");
        assert!(err.to_string().contains("Expected token RParen"));
    }
}
