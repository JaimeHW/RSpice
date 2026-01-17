//! Expression parser for behavioral sources
//!
//! Parses expressions like `V(2)*I(L1)+sin(TIME)` into AST.
//! Uses a simple recursive descent parser with operator precedence.

use super::ast::{BinaryOp, Expr, Function, UnaryOp};
use std::iter::Peekable;
use std::str::Chars;

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
            if c.is_alphanumeric() || c == '_' {
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
                    self.next_token()
                } // Skip unknown
            },
        }
    }
}

/// Expression parser using recursive descent with precedence climbing
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> bool {
        if std::mem::discriminant(&self.current) == std::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Parse a complete expression
    pub fn parse(&mut self) -> Expr {
        self.parse_or()
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
                let expr = self.parse();
                self.expect(Token::RParen);
                expr
            }
            _ => Expr::Const(0.0), // Error recovery
        }
    }

    fn parse_ident_or_call(&mut self, name: &str) -> Expr {
        let upper = name.to_uppercase();

        // Check for V(node) or I(element)
        if (upper == "V" || upper == "I") && self.current == Token::LParen {
            self.advance(); // consume (
            if let Token::Ident(arg) = self.current.clone() {
                self.advance();
                self.expect(Token::RParen);
                return if upper == "V" {
                    Expr::NodeVoltage(arg)
                } else {
                    Expr::BranchCurrent(arg)
                };
            } else if let Token::Number(n) = self.current {
                self.advance();
                self.expect(Token::RParen);
                return if upper == "V" {
                    Expr::NodeVoltage(n.to_string())
                } else {
                    Expr::BranchCurrent(n.to_string())
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
                args.push(self.parse());
                while self.current == Token::Comma {
                    self.advance();
                    args.push(self.parse());
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
                "FLOOR" | "INT" => Some(Function::Floor),
                "CEIL" | "CEILING" => Some(Function::Ceil),
                "ROUND" | "NINT" => Some(Function::Round),
                "MIN" => Some(Function::Min),
                "MAX" => Some(Function::Max),
                "PWR" => Some(Function::Pwr),
                "PWRS" => Some(Function::Pwrs),
                "LIMIT" => Some(Function::Limit),
                "SIGN" | "SGN" => Some(Function::Sign),
                "URAMP" => Some(Function::Uramp),
                "STP" | "STEP" | "U" => Some(Function::Stp),
                "MOD" | "FMOD" => Some(Function::Mod),
                "IF" => Some(Function::If),
                _ => None,
            };

            if let Some(f) = func {
                return Expr::Function { func: f, args };
            }
        }

        // Unknown identifier - treat as parameter (constant 0 for now)
        Expr::Const(0.0)
    }
}

/// Parse an expression string into AST
pub fn parse_expression(input: &str) -> Expr {
    Parser::new(input).parse()
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
}
