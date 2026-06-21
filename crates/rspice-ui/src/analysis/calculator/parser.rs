//! Calculator Expression Parser
//!
//! Parses strings like `V(out) * 2 + avg(V(in))` into a `CalculatorExpr` AST.
//! Uses a recursive descent parser with precedence climbing.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, UnaryOp};
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    position: usize,
    message: String,
}

impl ParseError {
    fn new(position: usize, message: impl Into<String>) -> Self {
        Self {
            position,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at character {}", self.message, self.position + 1)
    }
}

impl std::error::Error for ParseError {}

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
    chars: Peekable<CharIndices<'a>>,
    len: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices().peekable(),
            len: input.len(),
        }
    }

    fn position(&mut self) -> usize {
        self.chars.peek().map(|(idx, _)| *idx).unwrap_or(self.len)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, ParseError> {
        let start = self.position();
        let mut s = String::new();
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit()
                || c == '.'
                || c == 'e'
                || c == 'E'
                || (c == '+' || c == '-') && (s.ends_with('e') || s.ends_with('E'))
                || c.is_ascii_alphabetic()
                || c == '\u{00b5}'
                || c == '\u{03bc}'
            {
                if let Some((_, consumed)) = self.chars.next() {
                    s.push(consumed);
                }
            } else {
                break;
            }
        }
        let normalized = s.replace(['\u{00b5}', '\u{03bc}'], "u");
        crate::simulation::controller::spice_value::parse_spice_value_checked(&normalized)
            .map_err(|err| ParseError::new(start, err))
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        // Allow identifiers to contain dots, colons, slashes for node names if quoted?
        // For simple identifiers starting with alpha:
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == ':' {
                if let Some((_, consumed)) = self.chars.next() {
                    s.push(consumed);
                }
            } else {
                break;
            }
        }
        s
    }

    fn read_string_literal(&mut self) -> Result<String, ParseError> {
        let start = self.position();
        let mut s = String::new();
        if self.chars.peek().is_some_and(|(_, c)| *c == '"') {
            self.chars.next(); // consume opening "
            while let Some((_, c)) = self.chars.next() {
                if c == '"' {
                    return Ok(s);
                }
                if c == '\\' {
                    let Some((_, escaped)) = self.chars.next() else {
                        return Err(ParseError::new(start, "unterminated string literal"));
                    };
                    s.push(escaped);
                } else {
                    s.push(c);
                }
            }
        }
        Err(ParseError::new(start, "unterminated string literal"))
    }

    fn next_token(&mut self) -> Result<SpannedToken, ParseError> {
        self.skip_whitespace();

        let position = self.position();
        let token = match self.chars.peek() {
            None => Token::Eof,
            Some(&(_, c)) => match c {
                '0'..='9' | '.' => Token::Number(self.read_number()?),
                'a'..='z' | 'A'..='Z' | '_' => Token::Ident(self.read_ident()),
                '"' => Token::Ident(self.read_string_literal()?),
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
                    if self.chars.peek().is_some_and(|(_, c)| *c == '*') {
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
                    return Err(ParseError::new(
                        position,
                        format!("unexpected character '{}'", c),
                    ));
                }
            },
        };

        Ok(SpannedToken { token, position })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SpannedToken {
    token: Token,
    position: usize,
}

// =============================================================================
// Parser
// =============================================================================

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: SpannedToken,
    initial_error: Option<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let (current, initial_error) = match lexer.next_token() {
            Ok(token) => (token, None),
            Err(error) => (
                SpannedToken {
                    position: error.position,
                    token: Token::Eof,
                },
                Some(error),
            ),
        };
        Self {
            lexer,
            current,
            initial_error,
        }
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse(&mut self) -> CalculatorExpr {
        self.try_parse().unwrap_or(CalculatorExpr::Number(0.0))
    }

    pub fn try_parse(&mut self) -> Result<CalculatorExpr, ParseError> {
        if let Some(error) = self.initial_error.take() {
            return Err(error);
        }
        if matches!(self.current.token, Token::Eof) {
            return Err(ParseError::new(0, "empty expression"));
        }
        let expr = self.parse_additive()?;
        if !matches!(self.current.token, Token::Eof) {
            return Err(ParseError::new(
                self.current.position,
                format!(
                    "unexpected {} after expression",
                    describe_token(&self.current.token)
                ),
            ));
        }
        Ok(expr)
    }

    // Precedence: (low to high)
    // 1. +, -
    // 2. *, /
    // 3. ^
    // 4. unary -, unary +
    // 5. primary (parens, numbers, idents, calls)

    fn parse_additive(&mut self) -> Result<CalculatorExpr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.current.token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_multiplicative()?;
            left = CalculatorExpr::binary(op, left, right);
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<CalculatorExpr, ParseError> {
        let mut left = self.parse_power()?;

        loop {
            let op = match self.current.token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_power()?;
            left = CalculatorExpr::binary(op, left, right);
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<CalculatorExpr, ParseError> {
        let left = self.parse_unary()?;

        if matches!(self.current.token, Token::Caret) {
            self.advance()?;
            let right = self.parse_power()?; // Right associative
            Ok(CalculatorExpr::binary(BinaryOp::Pow, left, right))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<CalculatorExpr, ParseError> {
        match self.current.token {
            Token::Minus => {
                self.advance()?;
                Ok(CalculatorExpr::unary(UnaryOp::Neg, self.parse_unary()?))
            }
            Token::Plus => {
                self.advance()?; // Unary plus is no-op
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<CalculatorExpr, ParseError> {
        match self.current.token.clone() {
            Token::Number(val) => {
                self.advance()?;
                Ok(CalculatorExpr::Number(val))
            }
            Token::Ident(name) => {
                self.advance()?;
                self.parse_ident_or_call(&name)
            }
            Token::LParen => {
                let start = self.current.position;
                self.advance()?;
                let expr = self.parse_additive()?; // Reset to lowest precedence
                self.expect_rparen(start)?;
                Ok(expr)
            }
            _ => Err(ParseError::new(
                self.current.position,
                format!(
                    "expected expression, found {}",
                    describe_token(&self.current.token)
                ),
            )),
        }
    }

    fn expect_rparen(&mut self, start_position: usize) -> Result<(), ParseError> {
        if matches!(self.current.token, Token::RParen) {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::new(
                start_position,
                format!("missing ')' before {}", describe_token(&self.current.token)),
            ))
        }
    }

    fn parse_ident_or_call(&mut self, name: &str) -> Result<CalculatorExpr, ParseError> {
        let upper_name = name.to_uppercase();

        // 1. Handle V(node) and I(branch) special forms
        if (upper_name == "V" || upper_name == "I") && matches!(self.current.token, Token::LParen) {
            let start = self.current.position;
            self.advance()?; // consume (

            let arg = match self.current.token.clone() {
                Token::Ident(s) if !s.trim().is_empty() => s,
                Token::Number(n) => n.to_string(),
                _ => {
                    return Err(ParseError::new(
                        self.current.position,
                        format!("expected signal name inside {}(...)", upper_name),
                    ));
                }
            };
            self.advance()?; // consume arg
            self.expect_rparen(start)?;

            // Construct signal name: e.g. "V(out)" or "I(R1)"
            let signal_name = format!("{}({})", upper_name, arg);
            return Ok(CalculatorExpr::wave(&signal_name));
        }

        // 2. Constants
        if upper_name == "TIME" {
            return Ok(CalculatorExpr::Constant(CalculatorConstant::Time));
        }
        if upper_name == "FREQ" {
            return Ok(CalculatorExpr::Constant(CalculatorConstant::Frequency));
        }

        // 3. Function Calls
        if matches!(self.current.token, Token::LParen) {
            let start = self.current.position;
            self.advance()?; // consume (
            let mut args = Vec::new();

            if !matches!(self.current.token, Token::RParen) {
                args.push(self.parse_additive()?);
                while matches!(self.current.token, Token::Comma) {
                    self.advance()?;
                    args.push(self.parse_additive()?);
                }
            }

            self.expect_rparen(start)?;
            return Ok(CalculatorExpr::func(name, args));
        }

        // 4. Naked Identifier -> Assume it's a signal reference if nothing else matches?
        // Or maybe a named parameter/variable in future.
        // For now, treat as Waveform Reference (e.g. "out" implies "V(out)"? No, be strict)
        // Actually, maybe it's a variable or alias. Let's treat as WaveformRef for flexibility
        Ok(CalculatorExpr::wave(name))
    }
}

fn describe_token(token: &Token) -> String {
    match token {
        Token::Number(_) => "number".to_string(),
        Token::Ident(name) => format!("identifier '{name}'"),
        Token::Plus => "'+'".to_string(),
        Token::Minus => "'-'".to_string(),
        Token::Star => "'*'".to_string(),
        Token::Slash => "'/'".to_string(),
        Token::Caret => "'^'".to_string(),
        Token::LParen => "'('".to_string(),
        Token::RParen => "')'".to_string(),
        Token::Comma => "','".to_string(),
        Token::Eof => "end of expression".to_string(),
    }
}

/// Helper to parse a string and report syntax errors.
pub fn try_parse(input: &str) -> Result<CalculatorExpr, ParseError> {
    Parser::new(input).try_parse()
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
    fn parses_spice_suffix_numeric_literals() {
        assert_eq!(try_parse("1k").unwrap(), CalculatorExpr::Number(1.0e3));
        match try_parse("2.5u").unwrap() {
            CalculatorExpr::Number(value) => assert!((value - 2.5e-6).abs() < 1.0e-18),
            other => panic!("expected numeric literal, got {other:?}"),
        }
        assert_eq!(try_parse("3meg").unwrap(), CalculatorExpr::Number(3.0e6));
    }

    #[test]
    fn parses_quoted_hierarchical_signal_names() {
        let expr = try_parse(r#"V("/top/out")"#).unwrap();
        assert_eq!(expr, CalculatorExpr::wave("V(/top/out)"));
    }

    #[test]
    fn rejects_malformed_expressions_instead_of_zero_recovery() {
        for text in ["", "1+", "1 2", "1e", "V()", "avg(V(out)", "1$2"] {
            assert!(
                try_parse(text).is_err(),
                "expression {text:?} should be rejected"
            );
        }
    }
}
