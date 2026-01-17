//! SPICE netlist lexer/tokenizer using nom
//!
//! Provides robust tokenization for SPICE netlists, handling:
//! - Engineering notation (1k, 1MEG, 1u, 1n, 1p)
//! - Sloppy syntax (commas, missing spaces around `=`)
//! - Parenthesized expressions (PULSE(...), SIN(...))
//! - Parameter expressions ({value+1k})

use crate::Value;

//=============================================================================
// Token Types
//=============================================================================

/// A token with source location information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Source location span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

/// Token variants
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Identifier (R1, VCC, node_name)
    Ident(String),
    /// Numeric value with optional suffix (1k = 1000, 1u = 1e-6)
    Number(Value),
    /// String literal "..."
    StringLit(String),
    /// Expression in braces {expr}
    Expression(String),
    /// Equals sign
    Equals,
    /// Comma (treated as whitespace in SPICE)
    Comma,
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Plus sign (for line continuation or expressions)
    Plus,
    /// Minus sign
    Minus,
    /// Asterisk (multiplication or comment start)
    Star,
    /// Forward slash (division)
    Slash,
    /// Newline (significant for line structure)
    Newline,
    /// End of input
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Number(v) => write!(f, "{}", v),
            TokenKind::StringLit(s) => write!(f, "\"{}\"", s),
            TokenKind::Expression(s) => write!(f, "{{{}}}", s),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

//=============================================================================
// Lexer State
//=============================================================================

/// Lexer for SPICE netlists
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            // Skip horizontal whitespace (not newlines)
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                break;
            }

            let remaining = &self.input[self.pos..];
            let start_pos = self.pos;
            let start_line = self.line;

            // Try to match a token
            let (kind, consumed) = self.next_token(remaining)?;

            let span = Span {
                start: start_pos,
                end: start_pos + consumed,
                line: start_line,
            };

            // Track newlines for line counting
            if matches!(kind, TokenKind::Newline) {
                self.line += 1;
            }

            tokens.push(Token { kind, span });
            self.pos += consumed;
        }

        // Add EOF token
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: self.pos,
                end: self.pos,
                line: self.line,
            },
        });

        Ok(tokens)
    }

    /// Skip horizontal whitespace (spaces and tabs, not newlines)
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c == ' ' || c == '\t' || c == '\r' {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    /// Parse the next token, returning (TokenKind, bytes_consumed)
    fn next_token(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let c = input.chars().next().unwrap();

        match c {
            '\n' => Ok((TokenKind::Newline, 1)),
            '=' => Ok((TokenKind::Equals, 1)),
            ',' => Ok((TokenKind::Comma, 1)),
            '(' => Ok((TokenKind::LParen, 1)),
            ')' => Ok((TokenKind::RParen, 1)),
            '+' => Ok((TokenKind::Plus, 1)),
            '-' => {
                // Could be minus or start of negative number
                if input.len() > 1 {
                    let next = input[1..].chars().next();
                    if matches!(next, Some('0'..='9')) {
                        return self.parse_number_or_ident(input);
                    }
                }
                Ok((TokenKind::Minus, 1))
            }
            '*' => Ok((TokenKind::Star, 1)),
            '/' => Ok((TokenKind::Slash, 1)),
            '{' => self.parse_expression(input),
            '"' => self.parse_string(input),
            '.' => {
                // Could be a dot-command (.PARAM, .OP) or a decimal number
                if input.len() > 1 {
                    let next = input[1..].chars().next();
                    if matches!(next, Some('0'..='9')) {
                        // It's a decimal number like .5
                        self.parse_number(input)
                    } else {
                        // It's a dot-command like .PARAM
                        self.parse_ident(input)
                    }
                } else {
                    Err(LexError::InvalidNumber(".".to_string(), self.line))
                }
            }
            '0'..='9' => self.parse_number_or_ident(input),
            _ if is_ident_start(c) => self.parse_ident(input),
            _ => Err(LexError::UnexpectedChar(c, self.line)),
        }
    }

    /// Parse an identifier
    fn parse_ident(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 0;
        for c in input.chars() {
            if is_ident_char(c) {
                end += c.len_utf8();
            } else {
                break;
            }
        }

        let ident = &input[..end];
        Ok((TokenKind::Ident(ident.to_uppercase()), end))
    }

    /// Try to parse number, but if it looks like a model name (e.g. "1N4148"), parse as identifier
    fn parse_number_or_ident(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        // First, scan ahead to see if this looks like a model name (digit followed by letter then more chars)
        // Examples: 1N4148, 2N2222, 2SA1015
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        // Skip optional leading sign
        if i < chars.len() && (chars[i] == '-' || chars[i] == '+') {
            i += 1;
        }

        // Skip digits
        let digit_start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }

        // If followed by a letter that's NOT a valid SPICE suffix start or E (exponent),
        // it's likely a model name like 1N4148
        if i < chars.len() && i > digit_start {
            let next_char = chars[i].to_ascii_uppercase();
            // Check if it could be a model name pattern (digit + letter + more alphanumeric)
            if next_char.is_ascii_alphabetic() && i + 1 < chars.len() {
                let after = chars[i + 1];
                // If the letter is followed by more alphanumeric, it's likely a model name
                // E.g., "1N4148" - after 'N' we have '4' which makes it a model name
                // vs "1n" which is just 1 * nano
                if after.is_ascii_alphanumeric() {
                    // Check if it's NOT a valid suffix pattern
                    let could_be_suffix = match next_char {
                        'E' => after == '+' || after == '-' || after.is_ascii_digit(), // Scientific notation
                        'M' => {
                            // Could be MEG suffix
                            if chars.len() > i + 2 {
                                let m2 = chars[i + 1].to_ascii_uppercase();
                                let m3 = chars[i + 2].to_ascii_uppercase();
                                m2 == 'E' && m3 == 'G'
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if !could_be_suffix {
                        // It's a model name - parse as identifier
                        return self.parse_ident(input);
                    }
                }
            }
        }

        // Not a model name, parse as number
        self.parse_number(input)
    }

    /// Parse a numeric value with optional SPICE suffix
    fn parse_number(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 0;
        let mut _has_dot = false;
        let mut has_exp = false;
        let chars: Vec<char> = input.chars().collect();

        // Optional leading sign
        if end < chars.len() && (chars[end] == '-' || chars[end] == '+') {
            end += 1;
        }

        // Integer part
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }

        // Decimal part
        if end < chars.len() && chars[end] == '.' {
            _has_dot = true;
            end += 1;
            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }
        }

        // Exponent part (e or E)
        if end < chars.len() && (chars[end] == 'e' || chars[end] == 'E') {
            has_exp = true;
            end += 1;
            // Optional exponent sign
            if end < chars.len() && (chars[end] == '-' || chars[end] == '+') {
                end += 1;
            }
            // Exponent digits
            let exp_start = end;
            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }
            if end == exp_start {
                // No exponent digits after E
                return Err(LexError::InvalidNumber(input[..end].to_string(), self.line));
            }
        }

        // Calculate byte position
        let num_end_bytes: usize = chars[..end].iter().map(|c| c.len_utf8()).sum();
        let num_str = &input[..num_end_bytes];

        // Parse the numeric part
        let base: Value =
            if num_str.is_empty() || num_str == "." || num_str == "-" || num_str == "+" {
                return Err(LexError::InvalidNumber(num_str.to_string(), self.line));
            } else {
                num_str
                    .parse()
                    .map_err(|_| LexError::InvalidNumber(num_str.to_string(), self.line))?
            };

        // Check for SPICE suffix (only if no scientific notation was used)
        let (multiplier, suffix_len) = if !has_exp && end < chars.len() {
            parse_spice_suffix(&chars[end..])
        } else {
            (1.0, 0)
        };

        let suffix_bytes: usize = chars[end..end + suffix_len]
            .iter()
            .map(|c| c.len_utf8())
            .sum();
        let total_bytes = num_end_bytes + suffix_bytes;

        Ok((TokenKind::Number(base * multiplier), total_bytes))
    }

    /// Parse a braced expression {expr}
    fn parse_expression(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut depth = 0;
        let mut end = 0;

        for c in input.chars() {
            end += c.len_utf8();
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }

        if depth != 0 {
            return Err(LexError::UnterminatedExpression(self.line));
        }

        // Extract content without braces
        let content = &input[1..end - 1];
        Ok((TokenKind::Expression(content.to_string()), end))
    }

    /// Parse a quoted string "..."
    fn parse_string(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 1; // Skip opening quote
        let mut content = String::new();
        let chars: Vec<char> = input.chars().collect();

        while end < chars.len() {
            let c = chars[end];
            if c == '"' {
                end += 1;
                break;
            } else if c == '\\' && end + 1 < chars.len() {
                // Escape sequence
                end += 1;
                content.push(chars[end]);
                end += 1;
            } else {
                content.push(c);
                end += 1;
            }
        }

        let byte_len: usize = chars[..end].iter().map(|c| c.len_utf8()).sum();
        Ok((TokenKind::StringLit(content), byte_len))
    }
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Check if character can start an identifier
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Check if character can be part of an identifier
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':'
}

/// Parse SPICE engineering suffix and return (multiplier, chars_consumed)
fn parse_spice_suffix(chars: &[char]) -> (Value, usize) {
    if chars.is_empty() {
        return (1.0, 0);
    }

    // Try multi-char suffixes first
    if chars.len() >= 3 {
        let three: String = chars[..3].iter().collect();
        let three_upper = three.to_uppercase();
        if three_upper == "MEG" {
            return (1e6, 3);
        }
        if three_upper == "MIL" {
            return (25.4e-6, 3); // mil = 1/1000 inch
        }
    }

    // Single char suffixes
    let c = chars[0].to_ascii_uppercase();
    match c {
        'T' => (1e12, 1),
        'G' => (1e9, 1),
        'K' => (1e3, 1),
        'M' => (1e-3, 1), // milli (MEG already handled above)
        'U' => (1e-6, 1),
        'N' => (1e-9, 1),
        'P' => (1e-12, 1),
        'F' => (1e-15, 1),
        'A' => (1e-18, 1), // atto
        _ => (1.0, 0),
    }
}

//=============================================================================
// Lexer Errors
//=============================================================================

/// Errors that can occur during lexing
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedChar(char, usize),
    InvalidNumber(String, usize),
    UnterminatedExpression(usize),
    UnterminatedString(usize),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedChar(c, line) => {
                write!(f, "Unexpected character '{}' at line {}", c, line)
            }
            LexError::InvalidNumber(s, line) => {
                write!(f, "Invalid number '{}' at line {}", s, line)
            }
            LexError::UnterminatedExpression(line) => {
                write!(f, "Unterminated expression at line {}", line)
            }
            LexError::UnterminatedString(line) => {
                write!(f, "Unterminated string at line {}", line)
            }
        }
    }
}

impl std::error::Error for LexError {}

//=============================================================================
// Convenience Functions
//=============================================================================

/// Tokenize a SPICE netlist string
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

/// Parse a single SPICE value with engineering suffix
pub fn parse_spice_value(s: &str) -> Result<Value, LexError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(LexError::InvalidNumber("empty".to_string(), 0));
    }

    let lexer = Lexer::new(s);
    match lexer.parse_number(s) {
        Ok((TokenKind::Number(v), _)) => Ok(v),
        Ok(_) => Err(LexError::InvalidNumber(s.to_string(), 0)),
        Err(e) => Err(e),
    }
}

//=============================================================================
// Token Stream for Parser
//=============================================================================

/// Token stream for parsing - provides lookahead and consumption
#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    /// Create a new token stream from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Peek at the current token without consuming
    #[inline]
    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    /// Peek at token N positions ahead
    #[inline]
    pub fn peek_n(&self, n: usize) -> &Token {
        let idx = (self.pos + n).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    /// Check if current token matches a kind
    #[inline]
    pub fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    /// Consume and return the current token
    pub fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    /// Consume if current token matches, return true if consumed
    pub fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Expect a specific token kind, error if not found
    pub fn expect(&mut self, kind: &TokenKind) -> Result<&Token, String> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at line {}",
                kind,
                self.peek().kind,
                self.peek().span.line
            ))
        }
    }

    /// Get current line number
    #[inline]
    pub fn line(&self) -> usize {
        self.peek().span.line
    }

    /// Check if at end of input
    #[inline]
    pub fn is_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    /// Skip newlines, return count skipped
    pub fn skip_newlines(&mut self) -> usize {
        let mut count = 0;
        while matches!(self.peek().kind, TokenKind::Newline) {
            self.advance();
            count += 1;
        }
        count
    }

    /// Skip to end of line (for error recovery)
    pub fn skip_to_eol(&mut self) {
        while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            self.advance();
        }
    }

    /// Collect tokens until end of line as a vec
    pub fn collect_line(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            tokens.push(self.peek().clone());
            self.advance();
        }
        tokens
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let input = "R1 1 0 1k";
        let tokens = tokenize(input).unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Ident(ref s) if s == "R1"));
        assert!(matches!(tokens[1].kind, TokenKind::Number(v) if (v - 1.0).abs() < 1e-10));
        assert!(matches!(tokens[2].kind, TokenKind::Number(v) if v.abs() < 1e-10));
        assert!(matches!(tokens[3].kind, TokenKind::Number(v) if (v - 1000.0).abs() < 1e-10));
    }

    #[test]
    fn test_spice_suffixes() {
        assert!((parse_spice_value("1k").unwrap() - 1e3).abs() < 1e-10);
        assert!((parse_spice_value("1K").unwrap() - 1e3).abs() < 1e-10);
        assert!((parse_spice_value("1MEG").unwrap() - 1e6).abs() < 1e-10);
        assert!((parse_spice_value("1meg").unwrap() - 1e6).abs() < 1e-10);
        assert!((parse_spice_value("1u").unwrap() - 1e-6).abs() < 1e-20);
        assert!((parse_spice_value("1n").unwrap() - 1e-9).abs() < 1e-20);
        assert!((parse_spice_value("1p").unwrap() - 1e-12).abs() < 1e-20);
        assert!((parse_spice_value("1f").unwrap() - 1e-15).abs() < 1e-20);
        assert!((parse_spice_value("1T").unwrap() - 1e12).abs() < 1e-10);
        assert!((parse_spice_value("1G").unwrap() - 1e9).abs() < 1e-10);
        assert!((parse_spice_value("4.7k").unwrap() - 4700.0).abs() < 1e-10);
        assert!((parse_spice_value("2.2u").unwrap() - 2.2e-6).abs() < 1e-15);
    }

    #[test]
    fn test_scientific_notation() {
        assert!((parse_spice_value("1e3").unwrap() - 1000.0).abs() < 1e-10);
        assert!((parse_spice_value("1E-6").unwrap() - 1e-6).abs() < 1e-20);
        assert!((parse_spice_value("3.3e-9").unwrap() - 3.3e-9).abs() < 1e-20);
    }

    #[test]
    fn test_negative_numbers() {
        assert!((parse_spice_value("-5").unwrap() - (-5.0)).abs() < 1e-10);
        assert!((parse_spice_value("-1.5k").unwrap() - (-1500.0)).abs() < 1e-10);
    }

    #[test]
    fn test_tokenize_with_equals() {
        let input = "R1 1 0 R=1k temp=27";
        let tokens = tokenize(input).unwrap();

        // R1 1 0 R = 1k temp = 27
        assert!(matches!(tokens[0].kind, TokenKind::Ident(ref s) if s == "R1"));
        assert!(matches!(tokens[1].kind, TokenKind::Number(_)));
        assert!(matches!(tokens[2].kind, TokenKind::Number(_)));
        assert!(matches!(tokens[3].kind, TokenKind::Ident(ref s) if s == "R"));
        assert!(matches!(tokens[4].kind, TokenKind::Equals));
        assert!(matches!(tokens[5].kind, TokenKind::Number(v) if (v - 1000.0).abs() < 1e-10));
    }

    #[test]
    fn test_tokenize_commas() {
        let input = "R1 1 0 1k, temp=27";
        let tokens = tokenize(input).unwrap();

        // Comma is tokenized, parser can skip it
        let has_comma = tokens.iter().any(|t| matches!(t.kind, TokenKind::Comma));
        assert!(has_comma);
    }

    #[test]
    fn test_tokenize_expression() {
        let input = "R1 1 0 {1k+500}";
        let tokens = tokenize(input).unwrap();

        assert!(matches!(tokens[3].kind, TokenKind::Expression(ref s) if s == "1k+500"));
    }

    #[test]
    fn test_tokenize_parentheses() {
        let input = "PULSE(0 5 0 1n 1n)";
        let tokens = tokenize(input).unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Ident(ref s) if s == "PULSE"));
        assert!(matches!(tokens[1].kind, TokenKind::LParen));
        assert!(matches!(tokens[2].kind, TokenKind::Number(v) if v.abs() < 1e-10));
    }

    #[test]
    fn test_token_stream() {
        let input = "R1 1 0 1k";
        let tokens = tokenize(input).unwrap();
        let mut stream = TokenStream::new(tokens);

        assert!(matches!(stream.peek().kind, TokenKind::Ident(_)));
        stream.advance();
        assert!(matches!(stream.peek().kind, TokenKind::Number(_)));
    }

    #[test]
    fn test_multiline() {
        let input = "R1 1 0 1k\nR2 2 0 2k";
        let tokens = tokenize(input).unwrap();

        let newline_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::Newline))
            .count();
        assert_eq!(newline_count, 1);
    }
}
