//! Calculator Expression Parser
//!
//! Parses strings like `V(out) * 2 + avg(V(in))` into a `CalculatorExpr` AST.
//! Uses a recursive descent parser with precedence climbing.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, UnaryOp};
use std::iter::Peekable;
use std::str::Chars;

// =============================================================================
// Lexer
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret, // ^ or **
    // Delimiters
    LParen,
    RParen,
    Comma,
    // End of input
    Eof,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
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
                || (c == '+' || c == '-') && (s.ends_with('e') || s.ends_with('E'))
            {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        // Handle SPICE suffixes if needed (e.g. 1k, 1u) - for now simplified to f64
        // In a real implementation we might want to handle suffixes here or in a separate pass
        s.parse().unwrap_or(0.0)
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        // Allow identifiers to contain dots, colons, slashes for node names if quoted?
        // For simple identifiers starting with alpha:
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == ':' {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        s
    }

    fn read_string_literal(&mut self) -> String {
        // Assume starting quote is already consumed or we are about to consume it
        // Simpler approach: if we see ", read until "
        let mut s = String::new();
        if self.chars.peek() == Some(&'"') {
            self.chars.next(); // consume opening "
            while let Some(&c) = self.chars.peek() {
                if c == '"' {
                    self.chars.next(); // consume closing "
                    break;
                }
                s.push(self.chars.next().unwrap());
            }
        }
        s
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Token::Eof,
            Some(&c) => match c {
                '0'..='9' | '.' => Token::Number(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => Token::Ident(self.read_ident()),
                '"' => Token::Ident(self.read_string_literal()), // Treat quoted strings as idents for now
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
                        Token::Caret // Treat ** as ^
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
                _ => {
                    self.chars.next(); // Skip unknown
                    self.next_token()
                }
            },
        }
    }
}

// =============================================================================
// Parser
// =============================================================================

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

    pub fn parse(&mut self) -> CalculatorExpr {
        self.parse_additive()
    }

    // Precedence: (low to high)
    // 1. +, -
    // 2. *, /
    // 3. ^
    // 4. unary -, unary +
    // 5. primary (parens, numbers, idents, calls)

    fn parse_additive(&mut self) -> CalculatorExpr {
        let mut left = self.parse_multiplicative();

        loop {
            let op = match self.current {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = CalculatorExpr::binary(op, left, right);
        }
        left
    }

    fn parse_multiplicative(&mut self) -> CalculatorExpr {
        let mut left = self.parse_power();

        loop {
            let op = match self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_power();
            left = CalculatorExpr::binary(op, left, right);
        }
        left
    }

    fn parse_power(&mut self) -> CalculatorExpr {
        let left = self.parse_unary();

        if matches!(self.current, Token::Caret) {
            self.advance();
            let right = self.parse_power(); // Right associative
            CalculatorExpr::binary(BinaryOp::Pow, left, right)
        } else {
            left
        }
    }

    fn parse_unary(&mut self) -> CalculatorExpr {
        match self.current {
            Token::Minus => {
                self.advance();
                CalculatorExpr::unary(UnaryOp::Neg, self.parse_unary())
            }
            Token::Plus => {
                self.advance(); // Unary plus is no-op
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> CalculatorExpr {
        match self.current.clone() {
            Token::Number(val) => {
                self.advance();
                CalculatorExpr::Number(val)
            }
            Token::Ident(name) => {
                self.advance();
                self.parse_ident_or_call(&name)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_additive(); // Reset to lowest precedence
                if matches!(self.current, Token::RParen) {
                    self.advance();
                }
                expr
            }
            _ => CalculatorExpr::Number(0.0), // Error recovery: return 0
        }
    }

    fn parse_ident_or_call(&mut self, name: &str) -> CalculatorExpr {
        let upper_name = name.to_uppercase();

        // 1. Handle V(node) and I(branch) special forms
        if (upper_name == "V" || upper_name == "I") && matches!(self.current, Token::LParen) {
            self.advance(); // consume (

            // Argument can be Ident (node name) or String (if we had quoted strings)
            // Or Number (node 0, node 1)
            let arg = match self.current.clone() {
                Token::Ident(s) => s,
                Token::Number(n) => n.to_string(), // simple number to string
                _ => String::new(),
            };
            self.advance(); // consume arg

            if matches!(self.current, Token::RParen) {
                self.advance();
            }

            // Construct signal name: e.g. "V(out)" or "I(R1)"
            // In the calculator, we treat "V(node)" as a waveform reference signal name
            let signal_name = format!("{}({})", upper_name, arg);
            return CalculatorExpr::wave(&signal_name);
        }

        // 2. Constants
        if upper_name == "TIME" {
            return CalculatorExpr::Constant(CalculatorConstant::Time);
        }
        if upper_name == "FREQ" {
            return CalculatorExpr::Constant(CalculatorConstant::Frequency);
        }

        // 3. Function Calls
        // If followed by (, it's a function call
        if matches!(self.current, Token::LParen) {
            self.advance(); // consume (
            let mut args = Vec::new();

            if !matches!(self.current, Token::RParen) {
                args.push(self.parse_additive());
                while matches!(self.current, Token::Comma) {
                    self.advance();
                    args.push(self.parse_additive());
                }
            }

            if matches!(self.current, Token::RParen) {
                self.advance();
            }

            return CalculatorExpr::func(name, args);
        }

        // 4. Naked Identifier -> Assume it's a signal reference if nothing else matches?
        // Or maybe a named parameter/variable in future.
        // For now, treat as Waveform Reference (e.g. "out" implies "V(out)"? No, be strict)
        // Actually, maybe it's a variable or alias. Let's treat as WaveformRef for flexibility
        CalculatorExpr::wave(name)
    }
}

/// Helper to parse a string
pub fn parse(input: &str) -> CalculatorExpr {
    Parser::new(input).parse()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expr = parse("42.5");
        assert_eq!(expr, CalculatorExpr::Number(42.5));
    }

    #[test]
    fn test_parse_scientific() {
        let expr = parse("1e-3");
        assert_eq!(expr, CalculatorExpr::Number(0.001));
    }

    #[test]
    fn test_parse_binary_ops() {
        // 1 + 2 * 3 -> 1 + (2 * 3)
        let expr = parse("1 + 2 * 3");
        match expr {
            CalculatorExpr::BinaryOp {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                assert_eq!(*left, CalculatorExpr::Number(1.0));
                match *right {
                    CalculatorExpr::BinaryOp {
                        op: BinaryOp::Mul,
                        left,
                        right,
                    } => {
                        assert_eq!(*left, CalculatorExpr::Number(2.0));
                        assert_eq!(*right, CalculatorExpr::Number(3.0));
                    }
                    _ => panic!("Expected Mul on right"),
                }
            }
            _ => panic!("Expected Add at top"),
        }
    }

    #[test]
    fn test_parse_waveform_ref() {
        let expr = parse("V(out)");
        match expr {
            CalculatorExpr::WaveformRef { signal, .. } => {
                assert_eq!(signal, "V(out)");
            }
            _ => panic!("Expected WaveformRef"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let expr = parse("avg(V(out))");
        match expr {
            CalculatorExpr::FunctionCall { name, args } => {
                assert_eq!(name, "avg");
                assert_eq!(args.len(), 1);
                match &args[0] {
                    CalculatorExpr::WaveformRef { signal, .. } => {
                        assert_eq!(signal, "V(out)");
                    }
                    _ => panic!("Expected V(out) arg"),
                }
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_power() {
        let expr = parse("2^3");
        match expr {
            CalculatorExpr::BinaryOp {
                op: BinaryOp::Pow, ..
            } => (),
            _ => panic!("Expected Pow"),
        }
    }

    #[test]
    fn test_parentheses() {
        let expr = parse("(1 + 2) * 3");
        match expr {
            CalculatorExpr::BinaryOp {
                op: BinaryOp::Mul, ..
            } => (),
            _ => panic!("Expected Mul at top due to parens"),
        }
    }
}
